// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_core::{
    GUID, WString,
    runtime::{IStatelessServiceFactory, IStatelessServiceInstance},
    types::{ServicePartitionInformation, Uri},
};

use crate::mock::StatelessServicePartitionMock;

/// Test driver for a single stateless service instance.
pub struct StatelessServiceInstanceDriver {
    service_factory: Box<dyn IStatelessServiceFactory>,
    instance: Option<Box<dyn IStatelessServiceInstance>>,
}

impl StatelessServiceInstanceDriver {
    pub fn new(service_factory: Box<dyn IStatelessServiceFactory>) -> Self {
        Self {
            service_factory,
            instance: None,
        }
    }
}

pub struct CreateStatelessServiceArg {
    pub init_data: Vec<u8>,
    pub partition_id: GUID,
    pub instance_id: i64, // Currently partition id and instance id should be globally unique
    pub service_name: Uri,
    pub service_type_name: WString,
}

// Driver code.

impl StatelessServiceInstanceDriver {
    pub async fn create_service_instance(
        &mut self,
        desc: &CreateStatelessServiceArg,
    ) -> mssf_core::Result<()> {
        let service_instance = self
            .service_factory
            .create_instance(
                desc.service_type_name.clone(),
                desc.service_name.clone(),
                &desc.init_data,
                desc.partition_id,
                desc.instance_id,
            )
            .inspect_err(|e| {
                tracing::error!("Failed to create stateless service instance: {:?}", e)
            })?;
        let prev = self.instance.replace(service_instance);
        assert!(prev.is_none(), "Service instance already exists");
        let cancellation_token = mssf_core::sync::SimpleCancelToken::new_boxed();

        let instance_ref = self.instance.as_ref().unwrap();
        let partition =
            StatelessServicePartitionMock::new_arc(ServicePartitionInformation::Singleton(
                mssf_core::types::SingletonPartitionInformation {
                    id: desc.partition_id,
                },
            ));

        // start the service instance
        instance_ref.open(partition, cancellation_token).await?;
        Ok(())
    }

    pub async fn delete_service_instance(&mut self) -> mssf_core::Result<()> {
        if let Some(instance) = self.instance.take() {
            let cancellation_token = mssf_core::sync::SimpleCancelToken::new_boxed();
            instance.close(cancellation_token).await?;
        }
        Ok(())
    }

    pub fn delete_service_instance_force(&mut self) {
        if let Some(instance) = self.instance.take() {
            instance.abort();
        }
    }
}
