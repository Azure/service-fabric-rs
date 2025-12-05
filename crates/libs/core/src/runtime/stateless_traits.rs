// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

#![deny(non_snake_case)] // this file is safe rust

use crate::WString;
use crate::runtime::executor::BoxedCancelToken;
use crate::types::ServicePartitionInformation;

/// Stateless service factories are registered with the FabricRuntime by service hosts via
/// Runtime::register_stateless_service_factory().
///
pub trait IStatelessServiceFactory: Send + Sync + 'static {
    /// Creates a stateless service instance for a particular service. This method is called by Service Fabric.
    fn create_instance(
        &self,
        servicetypename: WString,
        servicename: crate::types::Uri,
        initializationdata: &[u8],
        partitionid: crate::GUID,
        instanceid: i64,
    ) -> crate::Result<Box<dyn IStatelessServiceInstance>>;
}

/// Defines behavior that governs the lifecycle of a stateless service instance, such as startup, initialization, and shutdown.
#[async_trait::async_trait]
pub trait IStatelessServiceInstance: Send + Sync + 'static {
    /// Opens an initialized service instance so that it can be contacted by clients.
    /// Remarks:
    /// Opening an instance stateless service indicates that the service is now resolvable
    /// and discoverable by service clients. The string that is returned is the address of this service instance.
    /// The address is associated with the service name via Service Fabric naming and returned to
    /// clients that resolve the service via resolve_service_partition(uri).
    async fn open(
        &self,
        partition: Box<dyn IStatelessServicePartition>,
        cancellation_token: BoxedCancelToken,
    ) -> crate::Result<WString>;

    /// Closes this service instance gracefully when the service instance is being shut down.
    async fn close(&self, cancellation_token: BoxedCancelToken) -> crate::Result<()>;

    /// Terminates this instance ungracefully with this synchronous method call.
    /// Remarks:
    /// Examples of ungraceful termination are network issues resulting in Service Fabric process shutdown and the
    /// use of ReportFault(FaultType) to report a Permanent fault. When the service instance receives this method,
    /// it should immediately release and clean up all references and return.
    fn abort(&self);
}

/// Abstrction for IFStatelessServicePartition interface
pub trait IStatelessServicePartition: Send + Sync + 'static {
    fn get_partition_info(&self) -> crate::Result<ServicePartitionInformation>;
    /// Reports load for the current replica in the partition.
    /// Remarks:
    /// The reported metrics should correspond to those that are provided in the ServiceLoadMetricDescription
    /// as a part of the ServiceDescription that is used to create the service. Load metrics that are not
    /// present in the description are ignored. Reporting custom metrics allows Service Fabric to balance
    /// services that are based on additional custom information.
    fn report_load(&self, metrics: &[crate::types::LoadMetric]) -> crate::Result<()>;
    /// Enables the replica to report a fault to the runtime and indicates that it has encountered
    /// an error from which it cannot recover and must either be restarted or removed.
    fn report_fault(&self, fault_type: crate::types::FaultType) -> crate::Result<()>;
    /// Reports the move cost for a replica.
    /// Remarks:
    /// Services can report move cost of a replica using this method.
    /// While the Service Fabric Resource Balances searches for the best balance in the cluster,
    /// it examines both load information and move cost of each replica.
    /// Resource balances will prefer to move replicas with lower cost in order to achieve balance.
    fn report_move_cost(&self, move_cost: crate::types::MoveCost) -> crate::Result<()>;
    /// Reports current partition health.
    fn report_partition_health(
        &self,
        health_info: &crate::types::HealthInformation,
    ) -> crate::Result<()>;
    /// Reports health on the current stateless service instance of the partition.
    fn report_instance_health(
        &self,
        health_info: &crate::types::HealthInformation,
    ) -> crate::Result<()>;
}
