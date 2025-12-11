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
}

#[derive(Clone)]
pub struct CreateStatefulServicePartitionArg {
    pub partition_id: GUID,
    pub replica_count: usize,
    pub init_data: Vec<u8>,
    pub service_name: Uri,
    pub service_type_name: WString,
}

/// Test driver for a single stateful service replica.
pub struct StatefulServicePartitionDriver {
    /// This keeps track of which factory to use next.
    factory_index: i64,
    service_factory: Vec<Box<dyn mssf_core::runtime::IStatefulServiceFactory>>,
    replica_index: i64,
    epoch_index: Epoch, // Used to generate new epoch.
    partition_state: PartitionState,
}

struct PartitionState {
    pub replica_states: HashMap<i64, StatefulServiceReplicaState>,
    pub primary_index: i64,
    pub epoch: Epoch,
    pub static_info: Option<CreateStatefulServicePartitionArg>, // Filled when created.
    pub current_configuration: mssf_core::types::ReplicaSetConfig,
}

struct StatefulServiceReplicaState {
    pub replica: Box<dyn mssf_core::runtime::IStatefulServiceReplica>,
    pub replicator: Box<dyn mssf_core::runtime::IPrimaryReplicator>,
    pub partition: StatefulServicePartitionMock,
    pub factory_index: i64, // The index of the factory that created the replica
    pub _replica_address: WString,
    pub _replicator_address: WString,
}

impl Default for StatefulServicePartitionDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl StatefulServicePartitionDriver {
    pub fn new() -> Self {
        Self {
            service_factory: Vec::new(),
            factory_index: 0,
            replica_index: 1, // replica id starting from 1
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
                static_info: None,
                current_configuration: mssf_core::types::ReplicaSetConfig {
                    replicas: vec![],
                    write_quorum: 0,
                },
            },
        }
    }

    /// Register a service factory to be used to create replicas.
    /// One should register multiple factories to simulate multi node scenarios.
    /// Replicas are created in round robin fashion from the registered factories.
    pub fn register_service_factory(
        &mut self,
        factory: Box<dyn mssf_core::runtime::IStatefulServiceFactory>,
    ) {
        self.service_factory.push(factory);
    }

    /// Get the next service factory in round robin fashion.
    /// This ensures that multiple factories can be tested, to simulate
    /// multi node scenarios.
    /// Returns the current index and the factory.
    fn get_round_robin_factory(
        &mut self,
    ) -> (i64, &dyn mssf_core::runtime::IStatefulServiceFactory) {
        assert!(!self.service_factory.is_empty());
        let idx = self.factory_index as usize % self.service_factory.len();
        self.factory_index += 1;
        (idx as i64, &*self.service_factory[idx])
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

    fn get_primary_state(&self) -> mssf_core::Result<&StatefulServiceReplicaState> {
        let state = self
            .partition_state
            .replica_states
            .get(&self.partition_state.primary_index)
            .ok_or_else(|| {
                mssf_core::Error::from(mssf_core::ErrorCode::FABRIC_E_REPLICA_DOES_NOT_EXIST)
            })?;
        Ok(state)
    }

    /// Check the invariants of the partition state.
    /// Panics if any invariant is violated.
    fn check_partition_state(&self) {
        if self.partition_state.replica_states.is_empty() {
            assert!(self.partition_state.static_info.is_none());
            assert_eq!(self.partition_state.current_configuration.replicas.len(), 0);
            assert_eq!(self.partition_state.current_configuration.write_quorum, 0);
            return;
        }
        // check primary exists
        self.get_primary_state().unwrap();
        // check quorum size matches
        let expected_quorum = (self.partition_state.replica_states.len() as u32) / 2 + 1;
        assert_eq!(
            self.partition_state.current_configuration.write_quorum,
            expected_quorum
        );
    }
}

// Public Accessors
impl StatefulServicePartitionDriver {
    /// Get the current primary replica id.
    pub fn get_primary_replica_id(&self) -> i64 {
        self.partition_state.primary_index
    }
    /// Get a replica by id.
    pub fn get_replica(
        &self,
        replica_id: i64,
    ) -> Option<&dyn mssf_core::runtime::IStatefulServiceReplica> {
        let state = self.partition_state.replica_states.get(&replica_id);
        state.map(|s| s.replica.as_ref())
    }
    /// Get a replicator by replica id.
    pub fn get_replicator(
        &self,
        replica_id: i64,
    ) -> Option<&dyn mssf_core::runtime::IPrimaryReplicator> {
        let state = self.partition_state.replica_states.get(&replica_id);
        state.map(|s| s.replicator.as_ref())
    }
    /// List all replica ids.
    pub fn list_replica_ids(&self) -> Vec<i64> {
        self.partition_state
            .replica_states
            .keys()
            .cloned()
            .collect()
    }
}

// Workflow implementations.
impl StatefulServicePartitionDriver {
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
            let (factory_index, factory) = self.get_round_robin_factory();
            let replica = factory
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
            let prev = replicas.insert(id, (factory_index, replica));
            assert!(prev.is_none(), "Service replica already exists");
        }

        // open all replicas
        for (id, (_, replica)) in &replicas {
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
                    Arc::new(partition.clone()),
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
        for (id, (_, replica)) in &replicas {
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
        for (id, (_, replica)) in &replicas {
            if *id == self.partition_state.primary_index {
                let replica_info = mssf_core::types::ReplicaInformation {
                    replicator_address: replicator_addresses.get(id).unwrap().clone(),
                    id: *id,
                    role: mssf_core::types::ReplicaRole::Primary,
                    status: mssf_core::types::ReplicaStatus::Up,
                    current_progress: -1, // -1 for invalid. observed in sf logs.
                    catch_up_capability: -1,
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
                current_progress: -1,
                catch_up_capability: -1,
                must_catch_up: false,
            };
            replica_infos.insert(*id, replica_info.clone());
            primary
                .build_replica(replica_info, cancellation_token)
                .await?;
            // change role to active secondary after successful build.
            replica
                .change_role(
                    mssf_core::types::ReplicaRole::ActiveSecondary,
                    SimpleCancelToken::new_boxed(),
                )
                .await?;
            // update the replica info
            replica_infos.get_mut(id).unwrap().role =
                mssf_core::types::ReplicaRole::ActiveSecondary;
        }

        // Run update catchup workflow for each secondary replica. Exclude primary.
        let mut new_config = mssf_core::types::ReplicaSetConfig {
            replicas: vec![],
            write_quorum: 1, // for primary
        };
        let mut ready_replicas = 1;
        for id in replicas.keys() {
            if *id == self.partition_state.primary_index {
                continue;
            }
            let prev_config = new_config.clone();
            // construct new config
            let replica_info = replica_infos.get(id).unwrap().clone();
            new_config.replicas.push(replica_info);
            ready_replicas += 1;
            new_config.write_quorum = ready_replicas / 2 + 1_u32;

            primary.update_catch_up_replica_set_configuration(new_config.clone(), prev_config)?;

            // wait for catch up
            primary
                .wait_for_catch_up_quorum(
                    mssf_core::types::ReplicaSetQuorumMode::Write,
                    SimpleCancelToken::new_boxed(),
                )
                .await?;
            // update current configuration
            primary.update_current_replica_set_configuration(new_config.clone())?;
            self.partition_state.current_configuration = new_config.clone();
        }

        // Update read write status.
        // TODO: This might not be accurate.
        // Maybe for primary it is always granted.
        // Since the quorum size is increasing and no replica down during build process.
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
        for (id, (factory_index, replica)) in replicas {
            let state = StatefulServiceReplicaState {
                replica,
                replicator: replicators.remove(&id).unwrap(),
                _replica_address: replica_addresses.remove(&id).unwrap(),
                _replicator_address: replicator_addresses.remove(&id).unwrap(),
                partition: partitions.remove(&id).unwrap(),
                factory_index,
            };
            self.partition_state.replica_states.insert(id, state);
        }
        self.partition_state.epoch = epoch;
        self.partition_state.static_info = Some(desc.clone());

        self.check_partition_state();
        Ok(())
    }

    /// Delete the service partition.
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
        self.partition_state.static_info = None;
        self.partition_state.current_configuration = mssf_core::types::ReplicaSetConfig {
            replicas: vec![],
            write_quorum: 0,
        };
        self.check_partition_state();
        Ok(())
    }

    /// Restart a secondary replica gracefully.
    pub async fn restart_secondary_graceful(&mut self, replica_id: i64) -> mssf_core::Result<()> {
        // check if replica exists
        {
            self.partition_state
                .replica_states
                .get_mut(&replica_id)
                .ok_or_else(|| {
                    mssf_core::Error::from(mssf_core::ErrorCode::FABRIC_E_REPLICA_DOES_NOT_EXIST)
                })?;
            // check if it is not primary
            if replica_id == self.partition_state.primary_index {
                tracing::error!(
                    "Replica {} is primary, cannot restart as secondary",
                    replica_id
                );
                return Err(mssf_core::Error::from(
                    mssf_core::ErrorCode::FABRIC_E_INVALID_OPERATION,
                ));
            }
        }

        // Update primary to remove the replica from the configuration.
        {
            let primary = self.get_primary_state().unwrap();
            let current_config = self.partition_state.current_configuration.clone();
            let replica_count = current_config.replicas.len();
            let new_replicas = current_config
                .replicas
                .iter()
                .filter(|r| r.id != replica_id)
                .cloned()
                .collect::<Vec<_>>();
            let write_quorum = (replica_count as u32) / 2 + 1; // Note that quorum is not changing here during graceful restart.
            let new_config = mssf_core::types::ReplicaSetConfig {
                replicas: new_replicas,
                write_quorum,
            };
            primary
                .replicator
                .update_current_replica_set_configuration(new_config.clone())?;
            self.partition_state.current_configuration = new_config;
        }

        let prev_state = self
            .partition_state
            .replica_states
            .remove(&replica_id)
            .unwrap();
        let factory_index = prev_state.factory_index;
        // Close the Secondary, and cleanup.
        {
            let cancellation_token = mssf_core::sync::SimpleCancelToken::new_boxed();
            prev_state.replica.close(cancellation_token.clone()).await?;
            prev_state.replicator.close(cancellation_token).await?;
            drop(prev_state);
        }

        // Create replica existing from the same factory.
        let factory = &*self.service_factory[factory_index as usize];
        let replica = factory
            .create_replica(
                self.partition_state
                    .static_info
                    .as_ref()
                    .unwrap()
                    .service_type_name
                    .clone(),
                self.partition_state
                    .static_info
                    .as_ref()
                    .unwrap()
                    .service_name
                    .clone(),
                &self.partition_state.static_info.as_ref().unwrap().init_data,
                self.partition_state
                    .static_info
                    .as_ref()
                    .unwrap()
                    .partition_id,
                replica_id,
            )
            .inspect_err(|e| {
                tracing::error!("Failed to create stateful service replica: {:?}", e)
            })?;
        // open the replica
        let partition = StatefulServicePartitionMock::new(ServicePartitionInformation::Singleton(
            mssf_core::types::SingletonPartitionInformation {
                id: self
                    .partition_state
                    .static_info
                    .as_ref()
                    .unwrap()
                    .partition_id,
            },
        ));
        // open existing replicator
        let replctr = replica
            .open(
                mssf_core::types::OpenMode::Existing,
                Arc::new(partition.clone()),
                SimpleCancelToken::new_boxed(),
            )
            .await
            .inspect_err(|e| tracing::error!("Fail to open replica {}", e))?;
        // open the replicator
        let replctr_addr = replctr.open(SimpleCancelToken::new_boxed()).await?;
        // change role to idle secondary
        replctr
            .change_role(
                self.partition_state.epoch.clone(),
                mssf_core::types::ReplicaRole::IdleSecondary,
                SimpleCancelToken::new_boxed(),
            )
            .await?;
        let replica_addr = replica
            .change_role(
                mssf_core::types::ReplicaRole::IdleSecondary,
                SimpleCancelToken::new_boxed(),
            )
            .await?;

        // build the replica again using the same id.
        let primary = self.get_primary_state().unwrap();

        let replica_info = mssf_core::types::ReplicaInformation {
            replicator_address: replctr_addr.clone(),
            id: replica_id,
            role: mssf_core::types::ReplicaRole::IdleSecondary,
            status: mssf_core::types::ReplicaStatus::Up,
            current_progress: -1, // Observed value for restart.
            catch_up_capability: -1,
            must_catch_up: false,
        };
        primary
            .replicator
            .build_replica(replica_info.clone(), SimpleCancelToken::new_boxed())
            .await?;

        // change role to active secondary after successful build.
        replica
            .change_role(
                mssf_core::types::ReplicaRole::ActiveSecondary,
                SimpleCancelToken::new_boxed(),
            )
            .await?;
        // update the replica info
        let mut updated_replica_info = replica_info.clone();
        updated_replica_info.role = mssf_core::types::ReplicaRole::ActiveSecondary;
        // update catch up config again.
        let prev_config = self.partition_state.current_configuration.clone();
        let mut new_replicas = prev_config.replicas.clone();
        new_replicas.push(updated_replica_info.clone());
        let write_quorum = (new_replicas.len() as u32) / 2 + 1;
        let new_config = mssf_core::types::ReplicaSetConfig {
            replicas: new_replicas,
            write_quorum,
        };
        primary
            .replicator
            .update_catch_up_replica_set_configuration(new_config.clone(), prev_config)?;
        // wait for catch up
        primary
            .replicator
            .wait_for_catch_up_quorum(
                mssf_core::types::ReplicaSetQuorumMode::Write,
                SimpleCancelToken::new_boxed(),
            )
            .await?;
        // update current configuration again.
        primary
            .replicator
            .update_current_replica_set_configuration(new_config.clone())?;
        self.partition_state.current_configuration = new_config;
        // save the state
        let state = StatefulServiceReplicaState {
            replica,
            replicator: replctr,
            _replica_address: replica_addr,
            _replicator_address: replctr_addr,
            partition,
            factory_index,
        };
        let prev = self
            .partition_state
            .replica_states
            .insert(replica_id, state);
        assert!(prev.is_none(), "Service replica already exists");
        // done.
        self.check_partition_state();
        Ok(())
    }
}
