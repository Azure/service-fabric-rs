// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::sync::Arc;

use mssf_core::{
    WString,
    runtime::stateless::{StatelessServiceFactory, StatelessServiceInstance},
    types::Uri,
};
use tracing::info;

use crate::app::AppContext;

/// Stateless service factory
pub struct ServiceFactory {
    ctx: Arc<AppContext>,
}

impl ServiceFactory {
    pub fn new(ctx: Arc<AppContext>) -> Self {
        Self { ctx }
    }
}

impl StatelessServiceFactory for ServiceFactory {
    #[tracing::instrument(skip(self))]
    fn create_instance(
        &self,
        servicetypename: WString,
        servicename: Uri,
        initializationdata: &[u8],
        partitionid: mssf_core::GUID,
        instanceid: i64,
    ) -> mssf_core::Result<impl StatelessServiceInstance> {
        info!("create_instance");
        Ok(crate::service_instance::ServiceInstance::new(
            self.ctx.clone(),
        ))
    }
}
