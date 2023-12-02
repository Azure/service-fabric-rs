pub struct IFabricQueryClient10Wrap {
    c: ::fabric_base::Microsoft::ServiceFabric::FabricCommon::FabricClient::IFabricQueryClient10,
}
impl IFabricQueryClient10Wrap {
    pub fn GetApplicationList (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetApplicationListResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetApplicationList(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetApplicationList(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetApplicationLoadInformation (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_LOAD_INFORMATION_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetApplicationLoadInformationResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetApplicationLoadInformation(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginGetApplicationLoadInformation(
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
    }    pub fn GetApplicationName (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_NAME_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetApplicationNameResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetApplicationName(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetApplicationName(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetApplicationTypeList (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetApplicationTypeListResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetApplicationTypeList(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetApplicationTypeList(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetApplicationTypePagedList (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: PAGED_FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetApplicationTypePagedListResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetApplicationTypePagedList(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginGetApplicationTypePagedList(
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
    }    pub fn GetClusterLoadInformation (& self , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetClusterLoadInformationResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetClusterLoadInformation(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetClusterLoadInformation(timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetDeployedApplicationList (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_DEPLOYED_APPLICATION_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetDeployedApplicationListResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetDeployedApplicationList(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetDeployedApplicationList(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetDeployedApplicationPagedList (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_PAGED_DEPLOYED_APPLICATION_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetDeployedApplicationPagedListResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetDeployedApplicationPagedList(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginGetDeployedApplicationPagedList(
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
    }    pub fn GetDeployedCodePackageList (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_DEPLOYED_CODE_PACKAGE_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetDeployedCodePackageListResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetDeployedCodePackageList(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetDeployedCodePackageList(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetDeployedReplicaDetail (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_DEPLOYED_SERVICE_REPLICA_DETAIL_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetDeployedServiceReplicaDetailResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetDeployedReplicaDetail(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetDeployedReplicaDetail(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetDeployedReplicaList (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_DEPLOYED_SERVICE_REPLICA_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetDeployedReplicaListResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetDeployedReplicaList(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetDeployedReplicaList(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetDeployedServicePackageList (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetDeployedServicePackageListResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetDeployedServicePackageList(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginGetDeployedServicePackageList(
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
    }    pub fn GetDeployedServiceTypeList (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_DEPLOYED_SERVICE_TYPE_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetDeployedServiceTypeListResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetDeployedServiceTypeList(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetDeployedServiceTypeList(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetNodeList (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_NODE_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetNodeListResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetNodeList(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetNodeList(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetNodeLoadInformation (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_NODE_LOAD_INFORMATION_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetNodeLoadInformationResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetNodeLoadInformation(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetNodeLoadInformation(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetPartitionList (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetPartitionListResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetPartitionList(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetPartitionList(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetPartitionLoadInformation (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_PARTITION_LOAD_INFORMATION_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetPartitionLoadInformationResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetPartitionLoadInformation(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginGetPartitionLoadInformation(
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
    }    pub fn GetProvisionedFabricCodeVersionList (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_PROVISIONED_CODE_VERSION_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetProvisionedCodeVersionListResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetProvisionedFabricCodeVersionList(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginGetProvisionedFabricCodeVersionList(
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
    }    pub fn GetProvisionedFabricConfigVersionList (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_PROVISIONED_CONFIG_VERSION_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetProvisionedConfigVersionListResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetProvisionedFabricConfigVersionList(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginGetProvisionedFabricConfigVersionList(
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
    }    pub fn GetReplicaList (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_SERVICE_REPLICA_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetReplicaListResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetReplicaList(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetReplicaList(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetReplicaLoadInformation (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_REPLICA_LOAD_INFORMATION_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetReplicaLoadInformationResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetReplicaLoadInformation(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetReplicaLoadInformation(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetServiceGroupMemberList (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_SERVICE_GROUP_MEMBER_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetServiceGroupMemberListResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetServiceGroupMemberList(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetServiceGroupMemberList(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetServiceGroupMemberTypeList (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_SERVICE_GROUP_MEMBER_TYPE_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetServiceGroupMemberTypeListResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetServiceGroupMemberTypeList(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginGetServiceGroupMemberTypeList(
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
    }    pub fn GetServiceList (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_SERVICE_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetServiceListResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetServiceList(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetServiceList(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetServiceName (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_SERVICE_NAME_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetServiceNameResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetServiceName(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetServiceName(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetServiceTypeList (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_SERVICE_TYPE_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetServiceTypeListResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetServiceTypeList(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetServiceTypeList(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetUnplacedReplicaInformation (& self , queryDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_UNPLACED_REPLICA_INFORMATION_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetUnplacedReplicaInformationResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetUnplacedReplicaInformation(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginGetUnplacedReplicaInformation(
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
    }
}
