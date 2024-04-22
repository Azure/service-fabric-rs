pub struct IFabricHealthClient4Wrap {
    com: ::mssf_com::Microsoft::ServiceFabric::FabricCommon::FabricClient::IFabricHealthClient4,
}
impl Default for IFabricHealthClient4Wrap {
    fn default() -> Self {
        Self::new()
    }
}
impl IFabricHealthClient4Wrap {
    pub fn new() -> IFabricHealthClient4Wrap {
        IFabricHealthClient4Wrap { com : crate :: sync :: CreateLocalClient :: < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricHealthClient4 > () , }
    }
    pub fn from_com(
        com: ::mssf_com::Microsoft::ServiceFabric::FabricCommon::FabricClient::IFabricHealthClient4,
    ) -> IFabricHealthClient4Wrap {
        IFabricHealthClient4Wrap { com }
    }    pub fn GetApplicationHealth (& self , applicationName : & u16 , healthPolicy : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_HEALTH_POLICY , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricApplicationHealthResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetApplicationHealth(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginGetApplicationHealth(
                applicationName,
                healthPolicy,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetApplicationHealth2 (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_HEALTH_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricApplicationHealthResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetApplicationHealth2(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetApplicationHealth2(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetClusterHealth (& self , healthPolicy : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_CLUSTER_HEALTH_POLICY , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricClusterHealthResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetClusterHealth(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetClusterHealth(healthPolicy, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetClusterHealth2 (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_CLUSTER_HEALTH_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricClusterHealthResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetClusterHealth2(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetClusterHealth2(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetClusterHealthChunk (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_CLUSTER_HEALTH_CHUNK_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetClusterHealthChunkResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetClusterHealthChunk(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetClusterHealthChunk(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetDeployedApplicationHealth (& self , applicationName : & u16 , nodeName : :: windows_core :: PCWSTR , healthPolicy : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_HEALTH_POLICY , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricDeployedApplicationHealthResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetDeployedApplicationHealth(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginGetDeployedApplicationHealth(
                applicationName,
                nodeName,
                healthPolicy,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetDeployedApplicationHealth2 (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_DEPLOYED_APPLICATION_HEALTH_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricDeployedApplicationHealthResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetDeployedApplicationHealth2(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginGetDeployedApplicationHealth2(
                queryDescription,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetDeployedServicePackageHealth (& self , applicationName : & u16 , serviceManifestName : :: windows_core :: PCWSTR , nodeName : :: windows_core :: PCWSTR , healthPolicy : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_HEALTH_POLICY , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricDeployedServicePackageHealthResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetDeployedServicePackageHealth(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginGetDeployedServicePackageHealth(
                applicationName,
                serviceManifestName,
                nodeName,
                healthPolicy,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetDeployedServicePackageHealth2 (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricDeployedServicePackageHealthResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetDeployedServicePackageHealth2(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginGetDeployedServicePackageHealth2(
                queryDescription,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetNodeHealth (& self , nodeName : :: windows_core :: PCWSTR , healthPolicy : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_CLUSTER_HEALTH_POLICY , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricNodeHealthResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetNodeHealth(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetNodeHealth(nodeName, healthPolicy, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetNodeHealth2 (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_NODE_HEALTH_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricNodeHealthResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetNodeHealth2(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetNodeHealth2(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetPartitionHealth (& self , partitionId : :: windows_core :: GUID , healthPolicy : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_HEALTH_POLICY , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricPartitionHealthResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetPartitionHealth(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginGetPartitionHealth(
                partitionId,
                healthPolicy,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetPartitionHealth2 (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_PARTITION_HEALTH_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricPartitionHealthResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetPartitionHealth2(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetPartitionHealth2(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetReplicaHealth (& self , partitionId : :: windows_core :: GUID , replicaId : i64 , healthPolicy : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_HEALTH_POLICY , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricReplicaHealthResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetReplicaHealth(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginGetReplicaHealth(
                partitionId,
                replicaId,
                healthPolicy,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetReplicaHealth2 (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_REPLICA_HEALTH_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricReplicaHealthResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetReplicaHealth2(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetReplicaHealth2(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetServiceHealth (& self , serviceName : & u16 , healthPolicy : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_HEALTH_POLICY , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricServiceHealthResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetServiceHealth(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginGetServiceHealth(
                serviceName,
                healthPolicy,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetServiceHealth2 (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_SERVICE_HEALTH_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricServiceHealthResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetServiceHealth2(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetServiceHealth2(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
}
