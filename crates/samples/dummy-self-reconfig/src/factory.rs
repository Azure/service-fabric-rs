// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Self-reconfiguring service factory.

use mssf_core::runtime::{ISelfReconfiguringServiceFactory, ISelfReconfiguringServiceInstance};
use mssf_core::{GUID, WString};
use tracing::info;

use crate::instance::SelfReconfigInstance;

/// Factory that creates [`SelfReconfigInstance`]s. Registered with the runtime
/// via `Runtime::register_self_reconfiguring_service_factory`.
pub struct SelfReconfigFactory;

impl ISelfReconfiguringServiceFactory for SelfReconfigFactory {
    fn create_instance(
        &self,
        servicetypename: WString,
        servicename: mssf_core::types::Uri,
        _initializationdata: &[u8],
        partitionid: GUID,
        instanceid: i64,
    ) -> mssf_core::Result<Box<dyn ISelfReconfiguringServiceInstance>> {
        info!(
            "CreateInstance: type={servicetypename:?} name={servicename:?} partition={partitionid:?} instance={instanceid}"
        );
        Ok(Box::new(SelfReconfigInstance::new(instanceid)))
    }
}
