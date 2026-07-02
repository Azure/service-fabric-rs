// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Queries SF for application upgrade progress and produces upgrade events.

use ::tokio::sync::mpsc;
use mssf_core::{
    client::FabricClient,
    runtime::executor::BoxedCancelToken,
    types::{ApplicationQueryDescription, ApplicationUpgradeProgress},
};
use std::time::Duration;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

pub enum Action {
    Stop,
}

/// Events produced by [`UpgradeDataProducer`].
#[derive(Debug, Clone)]
pub enum UpgradeProducerEvent {
    /// An application that is actively going through an upgrade.
    Upgrade(ApplicationUpgradeProgress),
    /// Marker emitted at the end of a producer loop iteration. Allows a consumer
    /// to detect that the producer has finished a full pass over the
    /// applications for the current iteration.
    IterationComplete,
}

/// Queries SF and produces application upgrade data.
/// The user is responsible for implementing a consumer to receive the data.
pub struct UpgradeDataProducer {
    fc: FabricClient,
    interval: Duration,
    sender: mpsc::UnboundedSender<UpgradeProducerEvent>,
}

impl UpgradeDataProducer {
    pub fn new(
        fc: FabricClient,
        interval: Duration,
        sender: mpsc::UnboundedSender<UpgradeProducerEvent>,
    ) -> Self {
        UpgradeDataProducer {
            fc,
            interval,
            sender,
        }
    }

    fn send_event(&self, event: UpgradeProducerEvent) -> Result<(), Action> {
        self.sender.send(event).map_err(|_| {
            tracing::error!("Receiver dropped, cannot send more data.");
            Action::Stop
        })
    }

    /// Run once to produce application upgrade data.
    ///
    /// Enumerates all applications and, for each one that is actively upgrading,
    /// emits an [`UpgradeProducerEvent::Upgrade`]. Applications that are not
    /// upgrading are skipped. An [`UpgradeProducerEvent::IterationComplete`]
    /// marker is emitted once all applications have been processed.
    pub(crate) async fn run_once(&self, token: BoxedCancelToken) -> Result<(), Action> {
        if let Ok(apps) = self.get_all_applications(token.clone()).await {
            for app in apps {
                match self
                    .fc
                    .get_application_manager()
                    .get_application_upgrade_progress(
                        &app.application_name,
                        DEFAULT_TIMEOUT,
                        Some(token.clone()),
                    )
                    .await
                {
                    Ok(progress) => {
                        // Only surface applications that are actively upgrading.
                        if progress.is_active() {
                            self.send_event(UpgradeProducerEvent::Upgrade(progress))?;
                        }
                    }
                    Err(err) => {
                        tracing::error!(
                            "Failed to get upgrade progress for {}: {err}",
                            app.application_name
                        );
                    }
                }
            }
        }
        self.send_event(UpgradeProducerEvent::IterationComplete)?;
        Ok(())
    }

    /// Run a loop to produce application upgrade data, honoring cancellation.
    pub async fn run_loop(&self, token: BoxedCancelToken) {
        loop {
            let start_time = ::tokio::time::Instant::now();
            if let Err(Action::Stop) = self.run_once(token.clone()).await {
                tracing::info!("Upgrade data producer stopped.");
                break;
            }

            // remaining time
            let elapsed = start_time.elapsed();
            // wait for more time if necessary.
            if elapsed < self.interval {
                let wait_duration = self.interval - elapsed;

                tokio::select! {
                    _ = token.wait() => {
                        tracing::info!("Cancellation requested, exiting upgrade data producer loop.");
                        break;
                    }
                    _ = tokio::time::sleep(wait_duration) => {}
                }
            }

            if token.is_cancelled() {
                tracing::info!("Cancellation requested, exiting upgrade data producer loop.");
                break;
            }
        }
        tracing::info!("Upgrade data producer loop exited.");
    }
}

// Get lists of entities.
impl UpgradeDataProducer {
    /// This does not include the system application.
    async fn get_all_applications(
        &self,
        token: BoxedCancelToken,
    ) -> mssf_core::Result<Vec<mssf_core::types::ApplicationQueryResultItem>> {
        let desc = ApplicationQueryDescription::default();
        let apps = self
            .fc
            .get_query_manager()
            .get_application_list(&desc, DEFAULT_TIMEOUT, Some(token.clone()))
            .await
            .inspect_err(|err| {
                tracing::error!("Failed to get application list: {}", err);
            })?
            .items;
        Ok(apps)
    }
}
