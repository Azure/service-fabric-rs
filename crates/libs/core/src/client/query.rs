pub struct IFabricQueryClient10Wrap {
    com: ::mssf_com::Microsoft::ServiceFabric::FabricCommon::FabricClient::IFabricQueryClient10,
}
impl Default for IFabricQueryClient10Wrap {
    fn default() -> Self {
        Self::new()
    }
}
impl IFabricQueryClient10Wrap {
    pub fn new() -> IFabricQueryClient10Wrap {
        IFabricQueryClient10Wrap { com : crate :: sync :: CreateLocalClient :: < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricQueryClient10 > () , }
    }    pub fn GetApplicationList (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetApplicationListResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetApplicationList(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetApplicationList(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetApplicationLoadInformation (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_LOAD_INFORMATION_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetApplicationLoadInformationResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetApplicationLoadInformation(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginGetApplicationLoadInformation(
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
    }    pub fn GetApplicationName (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_NAME_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetApplicationNameResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetApplicationName(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetApplicationName(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetApplicationTypeList (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetApplicationTypeListResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetApplicationTypeList(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetApplicationTypeList(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetApplicationTypePagedList (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: PAGED_FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetApplicationTypePagedListResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetApplicationTypePagedList(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginGetApplicationTypePagedList(
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
    }    pub fn GetClusterLoadInformation (& self , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetClusterLoadInformationResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetClusterLoadInformation(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetClusterLoadInformation(timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetDeployedApplicationList (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_DEPLOYED_APPLICATION_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetDeployedApplicationListResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetDeployedApplicationList(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginGetDeployedApplicationList(
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
    }    pub fn GetDeployedApplicationPagedList (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_PAGED_DEPLOYED_APPLICATION_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetDeployedApplicationPagedListResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetDeployedApplicationPagedList(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginGetDeployedApplicationPagedList(
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
    }    pub fn GetDeployedCodePackageList (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_DEPLOYED_CODE_PACKAGE_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetDeployedCodePackageListResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetDeployedCodePackageList(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginGetDeployedCodePackageList(
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
    }    pub fn GetDeployedReplicaDetail (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_DEPLOYED_SERVICE_REPLICA_DETAIL_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetDeployedServiceReplicaDetailResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetDeployedReplicaDetail(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetDeployedReplicaDetail(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetDeployedReplicaList (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_DEPLOYED_SERVICE_REPLICA_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetDeployedReplicaListResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetDeployedReplicaList(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetDeployedReplicaList(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetDeployedServicePackageList (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetDeployedServicePackageListResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetDeployedServicePackageList(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginGetDeployedServicePackageList(
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
    }    pub fn GetDeployedServiceTypeList (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_DEPLOYED_SERVICE_TYPE_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetDeployedServiceTypeListResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetDeployedServiceTypeList(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginGetDeployedServiceTypeList(
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
    }    pub fn GetNodeList (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_NODE_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetNodeListResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetNodeList(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetNodeList(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetNodeLoadInformation (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_NODE_LOAD_INFORMATION_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetNodeLoadInformationResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetNodeLoadInformation(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetNodeLoadInformation(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetPartitionList (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetPartitionListResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetPartitionList(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetPartitionList(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetPartitionLoadInformation (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_PARTITION_LOAD_INFORMATION_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetPartitionLoadInformationResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetPartitionLoadInformation(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginGetPartitionLoadInformation(
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
    }    pub fn GetProvisionedFabricCodeVersionList (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_PROVISIONED_CODE_VERSION_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetProvisionedCodeVersionListResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetProvisionedFabricCodeVersionList(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginGetProvisionedFabricCodeVersionList(
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
    }    pub fn GetProvisionedFabricConfigVersionList (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_PROVISIONED_CONFIG_VERSION_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetProvisionedConfigVersionListResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetProvisionedFabricConfigVersionList(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginGetProvisionedFabricConfigVersionList(
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
    }    pub fn GetReplicaList (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_SERVICE_REPLICA_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetReplicaListResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetReplicaList(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetReplicaList(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetReplicaLoadInformation (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_REPLICA_LOAD_INFORMATION_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetReplicaLoadInformationResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetReplicaLoadInformation(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginGetReplicaLoadInformation(
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
    }    pub fn GetServiceGroupMemberList (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_SERVICE_GROUP_MEMBER_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetServiceGroupMemberListResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetServiceGroupMemberList(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginGetServiceGroupMemberList(
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
    }    pub fn GetServiceGroupMemberTypeList (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_SERVICE_GROUP_MEMBER_TYPE_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetServiceGroupMemberTypeListResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetServiceGroupMemberTypeList(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginGetServiceGroupMemberTypeList(
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
    }    pub fn GetServiceList (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_SERVICE_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetServiceListResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetServiceList(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetServiceList(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetServiceName (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_SERVICE_NAME_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetServiceNameResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetServiceName(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetServiceName(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetServiceTypeList (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_SERVICE_TYPE_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetServiceTypeListResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetServiceTypeList(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetServiceTypeList(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetUnplacedReplicaInformation (& self , queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_UNPLACED_REPLICA_INFORMATION_QUERY_DESCRIPTION , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricGetUnplacedReplicaInformationResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetUnplacedReplicaInformation(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginGetUnplacedReplicaInformation(
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
    }
}
