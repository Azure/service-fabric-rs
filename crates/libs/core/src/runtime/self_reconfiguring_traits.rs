// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// self_reconfiguring contains the rs definition of the traits that a user needs
// to implement for a self-reconfiguring service.

use std::sync::Arc;

use crate::WString;
use crate::runtime::executor::BoxedCancelToken;
use crate::types::{
    SelfReconfiguringConfigurationChangeRequest, SelfReconfiguringConfigurationReport,
    SelfReconfiguringConfigurationRequest, SelfReconfiguringOpenMode, ServicePartitionInformation,
};

/// Represents a self-reconfiguring service factory that is responsible for
/// creating instances of a specific type of self-reconfiguring service.
/// Self-reconfiguring service factories are registered with the FabricRuntime by
/// service hosts via `Runtime::register_self_reconfiguring_service_factory()`.
pub trait ISelfReconfiguringServiceFactory: Send + Sync + 'static {
    /// Creates a self-reconfiguring service instance for a particular service.
    /// This method is called by Service Fabric.
    fn create_instance(
        &self,
        servicetypename: WString,
        servicename: crate::types::Uri,
        initializationdata: &[u8],
        partitionid: crate::GUID,
        instanceid: i64,
    ) -> crate::Result<Box<dyn ISelfReconfiguringServiceInstance>>;
}

/// Defines behavior that governs the lifecycle and configuration participation of
/// a self-reconfiguring service instance, such as startup, configuration changes,
/// and shutdown.
///
/// Unlike a stateless instance, a self-reconfiguring instance additionally
/// receives configuration requests and configuration-change requests from Service
/// Fabric and reports its resulting configuration through its partition.
#[async_trait::async_trait]
pub trait ISelfReconfiguringServiceInstance: Send + Sync + 'static {
    /// Opens an initialized service instance so that it can be contacted by
    /// clients. The returned string is the address of this service instance,
    /// which is associated with the service name via Service Fabric naming and
    /// returned to clients that resolve the service.
    ///
    /// The `open_mode` indicates whether the instance is being opened as new or
    /// existing.
    async fn open(
        &self,
        partition: Arc<dyn ISelfReconfiguringServicePartition>,
        open_mode: SelfReconfiguringOpenMode,
        cancellation_token: BoxedCancelToken,
    ) -> crate::Result<WString>;

    /// Notifies the instance of a configuration request from Service Fabric.
    ///
    /// This is a synchronous notification; the instance should act on the request
    /// and report the resulting configuration through its partition.
    fn request_configuration(
        &self,
        request: SelfReconfiguringConfigurationRequest,
    ) -> crate::Result<()>;

    /// Notifies the instance of a configuration-change request from Service
    /// Fabric, carrying the set of per-instance changes.
    ///
    /// This is a synchronous notification; the instance should act on the request
    /// and report the resulting configuration through its partition.
    fn request_configuration_change(
        &self,
        change: SelfReconfiguringConfigurationChangeRequest,
    ) -> crate::Result<()>;

    /// Closes this service instance gracefully when the service instance is being
    /// shut down.
    async fn close(&self, cancellation_token: BoxedCancelToken) -> crate::Result<()>;

    /// Terminates this instance ungracefully with this synchronous method call.
    /// When the service instance receives this method, it should immediately
    /// release and clean up all references and return.
    fn abort(&self);
}

/// Abstraction for the `IFabricSelfReconfiguringServicePartition` interface.
pub trait ISelfReconfiguringServicePartition: Send + Sync + 'static {
    /// Provides access to the `ServicePartitionInformation` of the service, which
    /// contains the partition type and ID.
    fn get_partition_info(&self) -> crate::Result<ServicePartitionInformation>;

    /// Reports load for the current instance in the partition.
    fn report_load(&self, metrics: &[crate::types::LoadMetric]) -> crate::Result<()>;

    /// Enables the instance to report a fault to the runtime and indicates that it
    /// has encountered an error from which it cannot recover and must either be
    /// restarted or removed.
    fn report_fault(&self, fault_type: crate::types::FaultType) -> crate::Result<()>;

    /// Reports the move cost for an instance.
    fn report_move_cost(&self, move_cost: crate::types::MoveCost) -> crate::Result<()>;

    /// Reports health on the current self-reconfiguring service instance of the
    /// partition.
    fn report_instance_health(
        &self,
        health_info: &crate::types::HealthInformation,
    ) -> crate::Result<()>;

    /// Reports current partition health.
    fn report_partition_health(
        &self,
        health_info: &crate::types::HealthInformation,
    ) -> crate::Result<()>;

    /// Reports the instance's resulting configuration back to Service Fabric.
    fn report_configuration(
        &self,
        report: &SelfReconfiguringConfigurationReport,
    ) -> crate::Result<()>;
}
