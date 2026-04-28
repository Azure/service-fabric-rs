// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Manual CLI for the reflection sample's `ReplicaControl` gRPC server.
//!
//! Useful for unsticking a cluster after a failed integration test left
//! gates parked, and for ad-hoc inspection ("what is each node currently
//! waiting on?").
//!
//! See `docs/design/ReflectionReplicaTestControl.md` for the design.
//!
//! Examples:
//!   reflection_ctl ping
//!   reflection_ctl list
//!   reflection_ctl approve-all
//!   reflection_ctl approve --partition <GUID> --replica <ID>
//!   reflection_ctl --host 127.0.0.1 list

use std::time::Duration;

use clap::{Parser, Subcommand};
use samples_reflection::grpc_control::REFLECTION_CONTROL_BASE_PORT;
use samples_reflection::grpc_control::proto::{
    ApprovalEvent, ApprovalKind, ApproveRequest, Empty, ListPendingRequest, ReplicaRef,
    approve_request::Decision as ApproveDecisionOneof,
    replica_control_client::ReplicaControlClient,
};
use tonic::transport::{Channel, Endpoint};

const NODE_COUNT: u16 = 5;

#[derive(Parser, Debug)]
#[command(
    name = "reflection_ctl",
    about = "Manual control of the reflection sample's ReplicaControl gRPC server",
    long_about = None,
)]
struct Cli {
    /// Hostname or IP of the cluster (default: $REFLECTION_CLUSTER_HOST or "onebox").
    #[arg(long, global = true)]
    host: Option<String>,

    /// Per-node connect timeout in milliseconds.
    #[arg(long, default_value_t = 500, global = true)]
    connect_timeout_ms: u64,

    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Probe each candidate port and print which nodes are reachable.
    Ping,

    /// Print every pending approval gate across all reachable nodes.
    List {
        /// Filter to a specific partition GUID (matches sfctl's hex format).
        #[arg(long)]
        partition: Option<String>,
    },

    /// Approve every pending gate on every reachable node with `Proceed`.
    /// Use this to clear leftover state from a failed test run.
    ApproveAll {
        /// Skip the y/N prompt before approving.
        #[arg(long)]
        yes: bool,
    },

    /// Approve a specific replica's pending gate. Auto-discovers the
    /// gate_id via ListPending; fails if no gate is pending.
    Approve {
        #[arg(long)]
        partition: String,
        #[arg(long)]
        replica: i64,
        /// If set, send Fail with this message instead of Proceed.
        #[arg(long)]
        fail_message: Option<String>,
    },

    /// Switch one or more controllers into proceed-forever mode. After
    /// detach, every future gate auto-approves and any pending gate is
    /// released. Useful for unblocking a stuck cleanup or for tests
    /// that want SF-driven teardown after a controlled setup.
    Detach {
        /// Detach a single replica (requires --partition and --replica).
        #[arg(long, requires = "replica")]
        partition: Option<String>,
        #[arg(long)]
        replica: Option<i64>,
        /// Detach every controllable replica on every reachable node.
        #[arg(long, conflicts_with = "partition")]
        all: bool,
    },
}

fn cluster_host(cli: &Cli) -> String {
    cli.host
        .clone()
        .or_else(|| std::env::var("REFLECTION_CLUSTER_HOST").ok())
        .unwrap_or_else(|| "onebox".to_string())
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::process::ExitCode {
    let cli = Cli::parse();
    let host = cluster_host(&cli);
    let connect_timeout = Duration::from_millis(cli.connect_timeout_ms);

    match cli.cmd {
        Command::Ping => cmd_ping(&host, connect_timeout).await,
        Command::List { partition } => cmd_list(&host, connect_timeout, partition).await,
        Command::ApproveAll { yes } => cmd_approve_all(&host, connect_timeout, yes).await,
        Command::Approve {
            partition,
            replica,
            fail_message,
        } => cmd_approve(&host, connect_timeout, partition, replica, fail_message).await,
        Command::Detach {
            partition,
            replica,
            all,
        } => cmd_detach(&host, connect_timeout, partition, replica, all).await,
    }
}

// ---------------------------------------------------------------------
// Subcommands
// ---------------------------------------------------------------------

async fn cmd_ping(host: &str, connect_timeout: Duration) -> std::process::ExitCode {
    let any = dial_all(host, connect_timeout)
        .await
        .into_iter()
        .filter_map(|(i, r)| match r {
            Ok(_) => {
                println!(
                    "node #{i:>1}  port {}  OK",
                    REFLECTION_CONTROL_BASE_PORT + i as u16
                );
                Some(())
            }
            Err(e) => {
                println!(
                    "node #{i:>1}  port {}  unreachable: {e}",
                    REFLECTION_CONTROL_BASE_PORT + i as u16
                );
                None
            }
        })
        .count();
    if any > 0 {
        std::process::ExitCode::SUCCESS
    } else {
        eprintln!("no ReplicaControl endpoints reachable on {host}");
        std::process::ExitCode::FAILURE
    }
}

async fn cmd_list(
    host: &str,
    connect_timeout: Duration,
    partition: Option<String>,
) -> std::process::ExitCode {
    let mut total = 0;
    for (i, conn) in dial_all(host, connect_timeout).await {
        let mut client = match conn {
            Ok(c) => c,
            Err(_) => continue,
        };
        let req = ListPendingRequest {
            partition_id: partition.clone().unwrap_or_default(),
            replica_filter: None,
        };
        match client.list_pending(req).await {
            Ok(r) => {
                let events = r.into_inner().events;
                if events.is_empty() {
                    continue;
                }
                println!(
                    "=== node #{i} (port {}) ===",
                    REFLECTION_CONTROL_BASE_PORT + i as u16
                );
                for ev in events {
                    print_event(&ev);
                    total += 1;
                }
            }
            Err(e) => eprintln!("node #{i}: ListPending failed: {e}"),
        }
    }
    if total == 0 {
        println!("no pending gates");
    } else {
        println!("\n{total} pending gate(s) total");
    }
    std::process::ExitCode::SUCCESS
}

async fn cmd_approve_all(
    host: &str,
    connect_timeout: Duration,
    yes: bool,
) -> std::process::ExitCode {
    if !yes {
        eprintln!(
            "About to approve(Proceed) every pending gate on every reachable node. \
             Pass --yes to skip this prompt or Ctrl-C to abort."
        );
        eprint!("Continue? [y/N] ");
        let mut buf = String::new();
        if std::io::stdin().read_line(&mut buf).is_err() || !buf.trim().eq_ignore_ascii_case("y") {
            eprintln!("aborted");
            return std::process::ExitCode::FAILURE;
        }
    }
    let mut released = 0u32;
    let mut errors = 0u32;
    for (i, conn) in dial_all(host, connect_timeout).await {
        let mut client = match conn {
            Ok(c) => c,
            Err(_) => continue,
        };
        let req = ListPendingRequest {
            partition_id: String::new(),
            replica_filter: None,
        };
        let events = match client.list_pending(req).await {
            Ok(r) => r.into_inner().events,
            Err(e) => {
                eprintln!("node #{i}: ListPending failed: {e}");
                errors += 1;
                continue;
            }
        };
        for ev in events {
            let target = match ev.target.clone() {
                Some(t) => t,
                None => continue,
            };
            let approve = ApproveRequest {
                target: Some(target.clone()),
                gate_id: ev.gate_id.clone(),
                decision: Some(ApproveDecisionOneof::Proceed(Empty {})),
            };
            match client.approve(approve).await {
                Ok(_) => {
                    println!(
                        "node #{i}: approved {kind:?} (partition={pid}, replica={rid}, gate_id={gid})",
                        kind = ApprovalKind::try_from(ev.kind)
                            .unwrap_or(ApprovalKind::ApprovalUnspecified),
                        pid = target.partition_id,
                        rid = target.replica_id,
                        gid = ev.gate_id,
                    );
                    released += 1;
                }
                Err(e) => {
                    eprintln!(
                        "node #{i}: Approve failed for partition={} replica={}: {e}",
                        target.partition_id, target.replica_id
                    );
                    errors += 1;
                }
            }
        }
    }
    println!("\n{released} gate(s) released, {errors} error(s)");
    if errors > 0 && released == 0 {
        std::process::ExitCode::FAILURE
    } else {
        std::process::ExitCode::SUCCESS
    }
}

async fn cmd_approve(
    host: &str,
    connect_timeout: Duration,
    partition: String,
    replica: i64,
    fail_message: Option<String>,
) -> std::process::ExitCode {
    // Find which node has this replica's pending gate.
    for (i, conn) in dial_all(host, connect_timeout).await {
        let mut client = match conn {
            Ok(c) => c,
            Err(_) => continue,
        };
        let events = client
            .list_pending(ListPendingRequest {
                partition_id: partition.clone(),
                replica_filter: None,
            })
            .await
            .map(|r| r.into_inner().events)
            .unwrap_or_default();
        for ev in events {
            let Some(target) = ev.target.clone() else {
                continue;
            };
            if target.replica_id != replica {
                continue;
            }
            let decision = match fail_message.as_ref() {
                None => ApproveDecisionOneof::Proceed(Empty {}),
                Some(msg) => ApproveDecisionOneof::FailMessage(msg.clone()),
            };
            let req = ApproveRequest {
                target: Some(target.clone()),
                gate_id: ev.gate_id.clone(),
                decision: Some(decision),
            };
            match client.approve(req).await {
                Ok(_) => {
                    println!(
                        "approved on node #{i} (gate_id={}, kind={:?})",
                        ev.gate_id,
                        ApprovalKind::try_from(ev.kind)
                            .unwrap_or(ApprovalKind::ApprovalUnspecified)
                    );
                    return std::process::ExitCode::SUCCESS;
                }
                Err(e) => {
                    eprintln!("Approve failed on node #{i}: {e}");
                    return std::process::ExitCode::FAILURE;
                }
            }
        }
    }
    eprintln!("no pending gate found for partition={partition} replica={replica} on any node");
    std::process::ExitCode::FAILURE
}

async fn cmd_detach(
    host: &str,
    connect_timeout: Duration,
    partition: Option<String>,
    replica: Option<i64>,
    all: bool,
) -> std::process::ExitCode {
    if all {
        let mut total_detached = 0u32;
        let mut errors = 0u32;
        for (i, conn) in dial_all(host, connect_timeout).await {
            let mut client = match conn {
                Ok(c) => c,
                Err(_) => continue,
            };
            match client.detach_all(Empty {}).await {
                Ok(r) => {
                    let n = r.into_inner().detached;
                    println!("node #{i}: detached {n} controller(s)");
                    total_detached += n;
                }
                Err(e) => {
                    eprintln!("node #{i}: DetachAll failed: {e}");
                    errors += 1;
                }
            }
        }
        println!("\n{total_detached} controller(s) detached, {errors} error(s)");
        return if errors > 0 && total_detached == 0 {
            std::process::ExitCode::FAILURE
        } else {
            std::process::ExitCode::SUCCESS
        };
    }

    let (Some(partition), Some(replica)) = (partition, replica) else {
        eprintln!("either --all or both --partition and --replica are required");
        return std::process::ExitCode::FAILURE;
    };
    let target = ReplicaRef {
        partition_id: partition,
        replica_id: replica,
    };
    for (i, conn) in dial_all(host, connect_timeout).await {
        let mut client = match conn {
            Ok(c) => c,
            Err(_) => continue,
        };
        match client.detach(target.clone()).await {
            Ok(_) => {
                println!(
                    "node #{i}: detached partition={} replica={}",
                    target.partition_id, target.replica_id
                );
                return std::process::ExitCode::SUCCESS;
            }
            Err(s) if s.code() == tonic::Code::NotFound => {
                // not on this node; try the next
                continue;
            }
            Err(e) => {
                eprintln!("node #{i}: Detach failed: {e}");
                return std::process::ExitCode::FAILURE;
            }
        }
    }
    eprintln!(
        "replica partition={} replica={} not registered on any node",
        target.partition_id, target.replica_id
    );
    std::process::ExitCode::FAILURE
}

// ---------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------

/// Dial every candidate port. Returns one entry per node with either
/// the connected client or the connection error.
async fn dial_all(
    host: &str,
    connect_timeout: Duration,
) -> Vec<(
    usize,
    Result<ReplicaControlClient<Channel>, tonic::transport::Error>,
)> {
    let mut out = Vec::with_capacity(NODE_COUNT as usize);
    for i in 0..NODE_COUNT {
        let port = REFLECTION_CONTROL_BASE_PORT + i;
        let url = format!("http://{host}:{port}");
        let endpoint = Endpoint::from_shared(url)
            .expect("hard-coded URL is valid")
            .connect_timeout(connect_timeout)
            .timeout(Duration::from_secs(10));
        out.push((
            i as usize,
            endpoint.connect().await.map(ReplicaControlClient::new),
        ));
    }
    out
}

fn print_event(ev: &ApprovalEvent) {
    let kind = ApprovalKind::try_from(ev.kind).unwrap_or(ApprovalKind::ApprovalUnspecified);
    let (pid, rid) = ev
        .target
        .as_ref()
        .map(|t| (t.partition_id.as_str(), t.replica_id))
        .unwrap_or(("?", 0));
    println!(
        "  {kind:?}  partition={pid}  replica={rid}  gate_id={}",
        ev.gate_id
    );
}
