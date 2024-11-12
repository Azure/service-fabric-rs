// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::Interface;
use mssf_com::FabricRuntime::{FabricCreateRuntime, FabricGetActivationContext, IFabricRuntime};

#[cfg(feature = "tokio_async")]
use mssf_com::FabricCommon::{IFabricAsyncOperationCallback, IFabricAsyncOperationContext};

#[cfg(feature = "tokio_async")]
pub use self::runtime_wrapper::Runtime;

pub mod config;
pub mod error;
#[cfg(feature = "tokio_async")]
pub mod executor;
#[cfg(feature = "tokio_async")]
pub mod node_context;
#[cfg(feature = "tokio_async")]
pub mod runtime_wrapper;
#[cfg(feature = "tokio_async")]
pub mod stateful;
#[cfg(feature = "tokio_async")]
pub mod stateful_bridge;
#[cfg(feature = "tokio_async")]
pub mod stateful_proxy;
#[cfg(feature = "tokio_async")]
pub mod stateless;
#[cfg(feature = "tokio_async")]
pub mod stateless_bridge;
pub mod store;
#[cfg(feature = "tokio_async")]
pub mod store_proxy;

mod activation_context;
pub use activation_context::{CodePackageActivationContext, CodePackageInfo};

// creates fabric runtime
pub fn create_com_runtime() -> crate::Result<IFabricRuntime> {
    let rawruntime = unsafe { FabricCreateRuntime(&IFabricRuntime::IID)? };
    let runtime = unsafe { IFabricRuntime::from_raw(rawruntime) };
    Ok(runtime)
}

pub fn get_com_activation_context<T: Interface>() -> crate::Result<T> {
    let raw_activation_ctx = unsafe { FabricGetActivationContext(&T::IID)? };

    let activation_ctx = unsafe { T::from_raw(raw_activation_ctx) };
    Ok(activation_ctx)
}
