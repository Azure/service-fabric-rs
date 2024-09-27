// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

#![deny(non_snake_case)] // this file is safe rust

use crate::sync::CancellationToken;
use crate::HSTRING;
use mssf_com::FabricRuntime::IFabricStatelessServicePartition;

use crate::types::ServicePartitionInformation;

// wrap of com interface
pub struct StatelessServicePartition {
    com_impl: IFabricStatelessServicePartition,
}

impl StatelessServicePartition {
    pub fn new(com_impl: IFabricStatelessServicePartition) -> StatelessServicePartition {
        StatelessServicePartition { com_impl }
    }

    pub fn get_partition_info(&self) -> crate::Result<ServicePartitionInformation> {
        let raw = unsafe { self.com_impl.GetPartitionInfo() }?;
        let raw_ref = unsafe { raw.as_ref().unwrap() };
        assert!(!raw.is_null());
        Ok(raw_ref.into())
    }
}

/// Stateless service factories are registered with the FabricRuntime by service hosts via
/// Runtime::register_stateless_service_factory().
///
pub trait StatelessServiceFactory {
    /// Creates a stateless service instance for a particular service. This method is called by Service Fabric.
    fn create_instance(
        &self,
        servicetypename: &HSTRING,
        servicename: &HSTRING,
        initializationdata: &[u8],
        partitionid: &::windows::core::GUID,
        instanceid: i64,
    ) -> crate::Result<impl StatelessServiceInstance>;
}

/// Defines behavior that governs the lifecycle of a stateless service instance, such as startup, initialization, and shutdown.
#[trait_variant::make(StatelessServiceInstance: Send)]
pub trait LocalStatelessServiceInstance: Send + Sync + 'static {
    /// Opens an initialized service instance so that it can be contacted by clients.
    /// Remarks:
    /// Opening an instance stateless service indicates that the service is now resolvable
    /// and discoverable by service clients. The string that is returned is the address of this service instance.
    /// The address is associated with the service name via Service Fabric naming and returned to
    /// clients that resolve the service via resolve_service_partition(uri).
    async fn open(
        &self,
        partition: &StatelessServicePartition,
        cancellation_token: CancellationToken,
    ) -> crate::Result<HSTRING>;

    /// Closes this service instance gracefully when the service instance is being shut down.
    async fn close(&self, cancellation_token: CancellationToken) -> crate::Result<()>;

    /// Terminates this instance ungracefully with this synchronous method call.
    /// Remarks:
    /// Examples of ungraceful termination are network issues resulting in Service Fabric process shutdown and the
    /// use of ReportFault(FaultType) to report a Permanent fault. When the service instance receives this method,
    /// it should immediately release and clean up all references and return.
    fn abort(&self);
}
