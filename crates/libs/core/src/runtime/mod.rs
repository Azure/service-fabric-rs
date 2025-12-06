// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::Interface;

use mssf_com::FabricCommon::{IFabricAsyncOperationCallback, IFabricAsyncOperationContext};
use mssf_com::FabricRuntime::IFabricRuntime;

pub use self::runtime_wrapper::Runtime;

pub mod config;
pub mod error;

pub mod executor;
pub mod node_context;

pub mod package_change;

pub mod runtime_wrapper;

mod stateful_traits;
pub use stateful_traits::{
    IPrimaryReplicator, IReplicator, IStatefulServiceFactory, IStatefulServicePartition,
    IStatefulServiceReplica,
};

pub mod stateful_bridge;

pub mod stateful_proxy;

mod stateless_traits;
pub use stateless_traits::{
    IStatelessServiceFactory, IStatelessServiceInstance, IStatelessServicePartition,
};

pub mod stateless_bridge;
mod stateless_proxy;
pub use stateless_proxy::StatelessServicePartition;
pub mod store;

pub mod store_proxy;

mod activation_context;
pub use activation_context::{CodePackageActivationContext, CodePackageInfo};

// creates fabric runtime
pub fn create_com_runtime() -> crate::Result<IFabricRuntime> {
    crate::API_TABLE
        .fabric_create_runtime()
        .map_err(crate::Error::from)
}

pub fn get_com_activation_context<T: Interface>() -> crate::Result<T> {
    crate::API_TABLE
        .fabric_get_activation_context::<T>()
        .map_err(crate::Error::from)
}
