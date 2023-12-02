pub struct IFabricHealthClient4Wrap {
    c: ::fabric_base::Microsoft::ServiceFabric::FabricCommon::FabricClient::IFabricHealthClient4,
}
impl IFabricHealthClient4Wrap {
    pub fn GetApplicationHealth (& self , applicationName : & u16 , healthPolicy : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_HEALTH_POLICY , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricApplicationHealthResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetApplicationHealth(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginGetApplicationHealth(
                applicationName,
                healthPolicy,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetApplicationHealth2 (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_HEALTH_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricApplicationHealthResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetApplicationHealth2(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetApplicationHealth2(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetClusterHealth (& self , healthPolicy : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_CLUSTER_HEALTH_POLICY , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricClusterHealthResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetClusterHealth(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetClusterHealth(healthPolicy, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetClusterHealth2 (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_CLUSTER_HEALTH_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricClusterHealthResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetClusterHealth2(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetClusterHealth2(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetClusterHealthChunk (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_CLUSTER_HEALTH_CHUNK_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetClusterHealthChunkResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetClusterHealthChunk(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetClusterHealthChunk(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetDeployedApplicationHealth (& self , applicationName : & u16 , nodeName : :: windows_core :: PCWSTR , healthPolicy : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_HEALTH_POLICY , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricDeployedApplicationHealthResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetDeployedApplicationHealth(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginGetDeployedApplicationHealth(
                applicationName,
                nodeName,
                healthPolicy,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetDeployedApplicationHealth2 (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_DEPLOYED_APPLICATION_HEALTH_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricDeployedApplicationHealthResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetDeployedApplicationHealth2(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginGetDeployedApplicationHealth2(
                queryDescription,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetDeployedServicePackageHealth (& self , applicationName : & u16 , serviceManifestName : :: windows_core :: PCWSTR , nodeName : :: windows_core :: PCWSTR , healthPolicy : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_HEALTH_POLICY , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricDeployedServicePackageHealthResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetDeployedServicePackageHealth(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginGetDeployedServicePackageHealth(
                applicationName,
                serviceManifestName,
                nodeName,
                healthPolicy,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetDeployedServicePackageHealth2 (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricDeployedServicePackageHealthResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetDeployedServicePackageHealth2(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginGetDeployedServicePackageHealth2(
                queryDescription,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetNodeHealth (& self , nodeName : :: windows_core :: PCWSTR , healthPolicy : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_CLUSTER_HEALTH_POLICY , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricNodeHealthResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetNodeHealth(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetNodeHealth(nodeName, healthPolicy, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetNodeHealth2 (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_NODE_HEALTH_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricNodeHealthResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetNodeHealth2(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetNodeHealth2(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetPartitionHealth (& self , partitionId : :: windows_core :: GUID , healthPolicy : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_HEALTH_POLICY , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricPartitionHealthResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetPartitionHealth(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginGetPartitionHealth(
                partitionId,
                healthPolicy,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetPartitionHealth2 (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_PARTITION_HEALTH_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricPartitionHealthResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetPartitionHealth2(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetPartitionHealth2(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetReplicaHealth (& self , partitionId : :: windows_core :: GUID , replicaId : i64 , healthPolicy : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_HEALTH_POLICY , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricReplicaHealthResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetReplicaHealth(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginGetReplicaHealth(
                partitionId,
                replicaId,
                healthPolicy,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetReplicaHealth2 (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_REPLICA_HEALTH_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricReplicaHealthResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetReplicaHealth2(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetReplicaHealth2(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetServiceHealth (& self , serviceName : & u16 , healthPolicy : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_HEALTH_POLICY , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricServiceHealthResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetServiceHealth(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetServiceHealth(serviceName, healthPolicy, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetServiceHealth2 (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_SERVICE_HEALTH_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricServiceHealthResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetServiceHealth2(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetServiceHealth2(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }
}
