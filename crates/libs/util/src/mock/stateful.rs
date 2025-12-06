// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use mssf_core::{
    GUID, WString,
    runtime::IStatefulServicePartition,
    sync::SimpleCancelToken,
    types::{Epoch, ServicePartitionInformation, Uri},
};

#[derive(Clone)]
pub struct StatefulServicePartitionMock {
    info: mssf_core::types::ServicePartitionInformation,
    read_status: Arc<Mutex<mssf_core::types::ServicePartitionAccessStatus>>,
    write_status: Arc<Mutex<mssf_core::types::ServicePartitionAccessStatus>>,
}

impl StatefulServicePartitionMock {
    pub fn new(info: mssf_core::types::ServicePartitionInformation) -> Self {
        Self {
            info,
            read_status: Arc::new(Mutex::new(
                mssf_core::types::ServicePartitionAccessStatus::ReconfigurationPending,
            )),
            write_status: Arc::new(Mutex::new(
                mssf_core::types::ServicePartitionAccessStatus::ReconfigurationPending,
            )),
        }
    }
    pub fn new_boxed(
        info: mssf_core::types::ServicePartitionInformation,
    ) -> Box<dyn IStatefulServicePartition> {
        Box::new(Self::new(info))
    }
    pub fn set_read_status(&self, status: mssf_core::types::ServicePartitionAccessStatus) {
        *self.read_status.lock().unwrap() = status;
    }
    pub fn set_write_status(&self, status: mssf_core::types::ServicePartitionAccessStatus) {
        *self.write_status.lock().unwrap() = status;
    }
}

impl IStatefulServicePartition for StatefulServicePartitionMock {
    fn create_replicator(
        &self,
    ) -> mssf_core::Result<Box<dyn mssf_core::runtime::IPrimaryReplicator>> {
        unimplemented!("Not implemented")
    }

    fn get_partition_information(
        &self,
    ) -> mssf_core::Result<mssf_core::types::ServicePartitionInformation> {
        Ok(self.info.clone())
    }

    fn get_read_status(&self) -> mssf_core::Result<mssf_core::types::ServicePartitionAccessStatus> {
        Ok(self.read_status.lock().unwrap().clone())
    }

    fn get_write_status(
        &self,
    ) -> mssf_core::Result<mssf_core::types::ServicePartitionAccessStatus> {
        Ok(self.write_status.lock().unwrap().clone())
    }

    fn report_load(&self, _metrics: &[mssf_core::types::LoadMetric]) -> mssf_core::Result<()> {
        Ok(())
    }

    fn report_fault(&self, _fault_type: mssf_core::types::FaultType) -> mssf_core::Result<()> {
        Ok(())
    }

    fn report_move_cost(&self, _move_cost: mssf_core::types::MoveCost) -> mssf_core::Result<()> {
        Ok(())
    }

    fn report_partition_health(
        &self,
        _healthinfo: &mssf_core::types::HealthInformation,
    ) -> mssf_core::Result<()> {
        Ok(())
    }

    fn report_replica_health(
        &self,
        _healthinfo: &mssf_core::types::HealthInformation,
    ) -> mssf_core::Result<()> {
        Ok(())
    }

    fn try_get_com(
        &self,
    ) -> mssf_core::Result<&mssf_com::FabricRuntime::IFabricStatefulServicePartition> {
        Err(mssf_core::ErrorCode::FABRIC_E_OPERATION_NOT_SUPPORTED.into())
    }

    fn clone_box(&self) -> Box<dyn IStatefulServicePartition> {
        Box::new(self.clone())
    }
}

pub struct CreateStatefulServicePartitionArg {
    pub partition_id: GUID,
    pub replica_count: usize,
    pub init_data: Vec<u8>,
    pub service_name: Uri,
    pub service_type_name: WString,
}

/// Test driver for a single stateful service replica.
pub struct StatefulServicePartitionDriver {
    service_factory: Box<dyn mssf_core::runtime::IStatefulServiceFactory>,
    replica_index: i64,
    epoch_index: Epoch,
    partition_state: PartitionState,
}

struct PartitionState {
    pub replica_states: HashMap<i64, StatefulServiceReplicaState>,
    pub primary_index: i64,
    pub epoch: Epoch,
}

struct StatefulServiceReplicaState {
    pub replica: Box<dyn mssf_core::runtime::IStatefulServiceReplica>,
    pub replicator: Box<dyn mssf_core::runtime::IPrimaryReplicator>,
    pub partition: StatefulServicePartitionMock,
    pub _replica_address: WString,
    pub _replicator_address: WString,
}

impl StatefulServicePartitionDriver {
    pub fn new(service_factory: Box<dyn mssf_core::runtime::IStatefulServiceFactory>) -> Self {
        Self {
            service_factory,
            replica_index: 1,
            epoch_index: Epoch {
                data_loss_number: 0,
                configuration_number: 1,
            },
            partition_state: PartitionState {
                replica_states: HashMap::new(),
                primary_index: 1, // First replica is the primary.
                epoch: Epoch {
                    data_loss_number: 0,
                    configuration_number: 1,
                },
            },
        }
    }

    fn next_replica_index(&mut self) -> i64 {
        let idx = self.replica_index;
        self.replica_index += 1;
        idx
    }

    fn next_epoch_index(&mut self) -> Epoch {
        let idx = self.epoch_index.clone();
        self.epoch_index.configuration_number += 1;
        idx
    }

    /// Create a stateful service partition with the specified number of replicas.
    /// The first replica is the primary.
    /// Runs the replica build steps.
    pub async fn create_service_partition(
        &mut self,
        desc: &CreateStatefulServicePartitionArg,
    ) -> mssf_core::Result<()> {
        assert!(desc.replica_count > 0);
        assert!(self.partition_state.replica_states.is_empty());

        let mut replicas = HashMap::new();
        let mut replicators = HashMap::new();
        let mut replica_addresses = HashMap::new();
        let mut replicator_addresses = HashMap::new();
        let mut replica_infos = HashMap::new();
        let mut partitions = HashMap::new();

        for _ in 0..desc.replica_count {
            let id = self.next_replica_index();
            let replica = self
                .service_factory
                .create_replica(
                    desc.service_type_name.clone(),
                    desc.service_name.clone(),
                    &desc.init_data,
                    desc.partition_id,
                    id,
                )
                .inspect_err(|e| {
                    tracing::error!("Failed to create stateful service replica: {:?}", e)
                })?;
            let prev = replicas.insert(id, replica);
            assert!(prev.is_none(), "Service replica already exists");
        }

        // open all replicas
        for (id, replica) in &replicas {
            let cancellation_token = mssf_core::sync::SimpleCancelToken::new_boxed();
            // TODO: support other partition schemes.
            let partition =
                StatefulServicePartitionMock::new(ServicePartitionInformation::Singleton(
                    mssf_core::types::SingletonPartitionInformation {
                        id: desc.partition_id,
                    },
                ));
            let replctr = replica
                .open(
                    mssf_core::types::OpenMode::New,
                    partition.clone_box(),
                    cancellation_token,
                )
                .await?;
            replicators.insert(*id, replctr);
            partitions.insert(*id, partition);
        }

        // open all replicators
        for (id, replctr) in &replicators {
            let cancellation_token = mssf_core::sync::SimpleCancelToken::new_boxed();

            let replctr_addr = replctr.open(cancellation_token).await?;
            replicator_addresses.insert(*id, replctr_addr);
        }

        // assign roles to replicators. for simplicity, we assume the first replica is the primary.
        let primary_index = 1;
        let epoch = self.next_epoch_index();
        for (id, rplctr) in &replicators {
            let epoch_cp = epoch.clone();
            let cancellation_token = mssf_core::sync::SimpleCancelToken::new_boxed();
            if *id == primary_index {
                rplctr
                    .change_role(
                        epoch_cp,
                        mssf_core::types::ReplicaRole::Primary,
                        cancellation_token,
                    )
                    .await?;
                self.partition_state.primary_index = primary_index;
            } else {
                rplctr
                    .change_role(
                        epoch_cp,
                        mssf_core::types::ReplicaRole::IdleSecondary,
                        cancellation_token,
                    )
                    .await?;
            }
        }

        // assign roles to replicas. First one is primary.
        for (id, replica) in &replicas {
            let cancellation_token = mssf_core::sync::SimpleCancelToken::new_boxed();
            let replica_addr = if *id == self.partition_state.primary_index {
                replica
                    .change_role(mssf_core::types::ReplicaRole::Primary, cancellation_token)
                    .await?
            } else {
                replica
                    .change_role(
                        mssf_core::types::ReplicaRole::IdleSecondary,
                        cancellation_token,
                    )
                    .await?
            };
            replica_addresses.insert(*id, replica_addr);
        }

        // build secondaries.
        let primary = replicators
            .get(&self.partition_state.primary_index)
            .unwrap();
        for id in replicas.keys() {
            if *id == self.partition_state.primary_index {
                let replica_info = mssf_core::types::ReplicaInformation {
                    replicator_address: replicator_addresses.get(id).unwrap().clone(),
                    id: *id,
                    role: mssf_core::types::ReplicaRole::Primary,
                    status: mssf_core::types::ReplicaStatus::Up,
                    current_progress: 0,
                    catch_up_capability: 0,
                    must_catch_up: false,
                };
                replica_infos.insert(*id, replica_info);
                continue;
            }
            let cancellation_token = mssf_core::sync::SimpleCancelToken::new_boxed();
            let replica_info = mssf_core::types::ReplicaInformation {
                replicator_address: replicator_addresses.get(id).unwrap().clone(),
                id: *id,
                role: mssf_core::types::ReplicaRole::IdleSecondary,
                status: mssf_core::types::ReplicaStatus::Up,
                current_progress: 0,
                catch_up_capability: 0,
                must_catch_up: false,
            };
            replica_infos.insert(*id, replica_info.clone());
            primary
                .build_replica(replica_info, cancellation_token)
                .await?;
        }

        // update catch up replica set config
        let currentconfiguration = mssf_core::types::ReplicaSetConfig {
            replicas: replica_infos.values().cloned().collect(),
            write_quorum: (replicas.len() / 2 + 1) as u32,
        };
        let priv_config = mssf_core::types::ReplicaSetConfig {
            replicas: vec![],
            write_quorum: 0,
        };
        primary.update_catch_up_replica_set_configuration(currentconfiguration, priv_config)?;

        // wait for catch up
        primary
            .wait_for_catch_up_quorum(
                mssf_core::types::ReplicaSetQuorumMode::All,
                SimpleCancelToken::new_boxed(),
            )
            .await?;

        // Change secondaries to active secondaries.
        for (id, replica) in &replicas {
            if *id == 1 {
                continue;
            }
            let cancellation_token = SimpleCancelToken::new_boxed();
            replica
                .change_role(
                    mssf_core::types::ReplicaRole::ActiveSecondary,
                    cancellation_token,
                )
                .await?;
            // update the replica info
            replica_infos.get_mut(id).unwrap().role =
                mssf_core::types::ReplicaRole::ActiveSecondary;
        }
        // update current configuration
        {
            let currentconfiguration = mssf_core::types::ReplicaSetConfig {
                replicas: replica_infos.values().cloned().collect(),
                write_quorum: (replicas.len() / 2 + 1) as u32,
            };
            primary.update_current_replica_set_configuration(currentconfiguration)?;
        }

        // Update read write status.
        for (id, partition) in &partitions {
            if *id == self.partition_state.primary_index {
                partition.set_read_status(mssf_core::types::ServicePartitionAccessStatus::Granted);
                partition.set_write_status(mssf_core::types::ServicePartitionAccessStatus::Granted);
            } else {
                partition
                    .set_read_status(mssf_core::types::ServicePartitionAccessStatus::NotPrimary);
                partition
                    .set_write_status(mssf_core::types::ServicePartitionAccessStatus::NotPrimary);
            }
        }

        // Save the state.
        for (id, replica) in replicas {
            let state = StatefulServiceReplicaState {
                replica,
                replicator: replicators.remove(&id).unwrap(),
                _replica_address: replica_addresses.remove(&id).unwrap(),
                _replicator_address: replicator_addresses.remove(&id).unwrap(),
                partition: partitions.remove(&id).unwrap(),
            };
            self.partition_state.replica_states.insert(id, state);
        }
        self.partition_state.epoch = epoch;
        Ok(())
    }

    pub async fn delete_service_partition(&mut self) -> mssf_core::Result<()> {
        // Not sure if the sequence is correct.

        // Change read write status to pending
        for state in self.partition_state.replica_states.values_mut() {
            state.partition.set_read_status(
                mssf_core::types::ServicePartitionAccessStatus::ReconfigurationPending,
            );
            state.partition.set_write_status(
                mssf_core::types::ServicePartitionAccessStatus::ReconfigurationPending,
            );
        }

        // Change primary to active secondary
        let primary = self
            .partition_state
            .replica_states
            .get_mut(&1)
            .expect("Primary replica not found");

        let cancellation_token = mssf_core::sync::SimpleCancelToken::new_boxed();
        primary
            .replica
            .change_role(
                mssf_core::types::ReplicaRole::ActiveSecondary,
                cancellation_token,
            )
            .await?;
        primary
            .replicator
            .change_role(
                self.partition_state.epoch.clone(), // Epoch is unchanged.
                mssf_core::types::ReplicaRole::ActiveSecondary,
                SimpleCancelToken::new_boxed(),
            )
            .await?;

        // change role to none for all replicas
        for state in self.partition_state.replica_states.values_mut() {
            let cancellation_token = mssf_core::sync::SimpleCancelToken::new_boxed();
            state
                .replica
                .change_role(mssf_core::types::ReplicaRole::None, cancellation_token)
                .await?;
            state
                .replicator
                .change_role(
                    self.partition_state.epoch.clone(), // Epoch is unchanged.
                    mssf_core::types::ReplicaRole::None,
                    SimpleCancelToken::new_boxed(),
                )
                .await?;
        }

        // close all replicas and replicators
        for state in self.partition_state.replica_states.values_mut() {
            let cancellation_token = mssf_core::sync::SimpleCancelToken::new_boxed();
            state.replica.close(cancellation_token.clone()).await?;
            state.replicator.close(cancellation_token).await?;
        }

        // clear the state
        self.partition_state.replica_states.clear();

        Ok(())
    }
}
