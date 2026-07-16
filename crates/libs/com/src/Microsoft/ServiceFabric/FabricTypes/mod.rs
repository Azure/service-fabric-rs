#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_AAD_CLAIMS_RETRIEVAL_METADATA {
    pub Authority: LPCWSTR,
    pub TenantId: LPCWSTR,
    pub ClusterApplication: LPCWSTR,
    pub ClientApplication: LPCWSTR,
    pub ClientRedirect: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_AAD_CLAIMS_RETRIEVAL_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_AAD_CLAIMS_RETRIEVAL_METADATA_EX1 {
    pub LoginEndpoint: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_AAD_CLAIMS_RETRIEVAL_METADATA_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATIONS_HEALTH_EVALUATION {
    pub Description: LPCWSTR,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub TotalCount: u32,
    pub MaxPercentUnhealthyApplications: u8,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATIONS_HEALTH_EVALUATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_ARM_METADATA_UPDATE_DESCRIPTION {
    pub ArmMetadata: *mut FABRIC_COMMON_ARM_METADATA,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_ARM_METADATA_UPDATE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_CAPACITY_DESCRIPTION {
    pub MaximumNodes: u32,
    pub MinimumNodes: u32,
    pub Metrics: *const FABRIC_APPLICATION_METRIC_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_CAPACITY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_DEFINITION_KIND(pub i32);
impl FABRIC_APPLICATION_DEFINITION_KIND {
    pub const FABRIC_APPLICATION_DEFINITION_KIND_INVALID: Self = Self(65535);
    pub const FABRIC_APPLICATION_DEFINITION_KIND_SERVICE_FABRIC_APPLICATION_DESCRIPTION: Self =
        Self(0);
    pub const FABRIC_APPLICATION_DEFINITION_KIND_COMPOSE: Self = Self(1);
    pub const FABRIC_APPLICATION_DEFINITION_KIND_MESH_APPLICATION_DESCRIPTION: Self = Self(2);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_DEFINITION_KIND_FILTER(pub i32);
impl FABRIC_APPLICATION_DEFINITION_KIND_FILTER {
    pub const FABRIC_APPLICATION_DEFINITION_KIND_FILTER_DEFAULT: Self = Self(0);
    pub const FABRIC_APPLICATION_DEFINITION_KIND_FILTER_ALL: Self = Self(65535);
    pub const FABRIC_APPLICATION_DEFINITION_KIND_FILTER_SERVICE_FABRIC_APPLICATION_DESCRIPTION:
        Self = Self(1);
    pub const FABRIC_APPLICATION_DEFINITION_KIND_FILTER_COMPOSE: Self = Self(2);
    pub const FABRIC_APPLICATION_DEFINITION_KIND_FILTER_MESH_APPLICATION_DESCRIPTION: Self =
        Self(4);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_DESCRIPTION {
    pub ApplicationName: FABRIC_URI,
    pub ApplicationTypeName: LPCWSTR,
    pub ApplicationTypeVersion: LPCWSTR,
    pub ApplicationParameters: *const FABRIC_APPLICATION_PARAMETER_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_DESCRIPTION_EX1 {
    pub ApplicationCapacity: *const FABRIC_APPLICATION_CAPACITY_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_DESCRIPTION_EX2 {
    pub ManagedApplicationIdentity: *const FABRIC_MANAGED_APPLICATION_IDENTITY_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_DESCRIPTION_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_HEALTH {
    pub ApplicationName: FABRIC_URI,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub HealthEvents: *const FABRIC_HEALTH_EVENT_LIST,
    pub DeployedApplicationHealthStates: *const FABRIC_DEPLOYED_APPLICATION_HEALTH_STATE_LIST,
    pub ServiceHealthStates: *const FABRIC_SERVICE_HEALTH_STATE_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_HEALTH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_HEALTH_EVALUATION {
    pub Description: LPCWSTR,
    pub ApplicationName: FABRIC_URI,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_HEALTH_EVALUATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_HEALTH_EX1 {
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_HEALTH_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_HEALTH_EX2 {
    pub HealthStatistics: *const FABRIC_HEALTH_STATISTICS,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_HEALTH_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_HEALTH_POLICY {
    pub ConsiderWarningAsError: bool,
    pub MaxPercentUnhealthyDeployedApplications: u8,
    pub DefaultServiceTypeHealthPolicy: *const FABRIC_SERVICE_TYPE_HEALTH_POLICY,
    pub ServiceTypeHealthPolicyMap: *const FABRIC_SERVICE_TYPE_HEALTH_POLICY_MAP,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_HEALTH_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_HEALTH_POLICY_MAP {
    pub Count: u32,
    pub Items: *mut FABRIC_APPLICATION_HEALTH_POLICY_MAP_ITEM,
}
impl Default for FABRIC_APPLICATION_HEALTH_POLICY_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_HEALTH_POLICY_MAP_ITEM {
    pub ApplicationName: FABRIC_URI,
    pub HealthPolicy: *const FABRIC_APPLICATION_HEALTH_POLICY,
}
impl Default for FABRIC_APPLICATION_HEALTH_POLICY_MAP_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_HEALTH_QUERY_DESCRIPTION {
    pub ApplicationName: FABRIC_URI,
    pub HealthPolicy: *const FABRIC_APPLICATION_HEALTH_POLICY,
    pub EventsFilter: *const FABRIC_HEALTH_EVENTS_FILTER,
    pub ServicesFilter: *const FABRIC_SERVICE_HEALTH_STATES_FILTER,
    pub DeployedApplicationsFilter: *const FABRIC_DEPLOYED_APPLICATION_HEALTH_STATES_FILTER,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_HEALTH_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_HEALTH_QUERY_DESCRIPTION_EX1 {
    pub HealthStatisticsFilter: *const FABRIC_APPLICATION_HEALTH_STATISTICS_FILTER,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_HEALTH_QUERY_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_HEALTH_REPORT {
    pub ApplicationName: FABRIC_URI,
    pub HealthInformation: *const FABRIC_HEALTH_INFORMATION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_HEALTH_REPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_HEALTH_STATE {
    pub ApplicationName: FABRIC_URI,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_HEALTH_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_HEALTH_STATES_FILTER {
    pub HealthStateFilter: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_HEALTH_STATES_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_HEALTH_STATE_CHUNK {
    pub ApplicationName: FABRIC_URI,
    pub HealthState: FABRIC_HEALTH_STATE,
    pub ServiceHealthStateChunks: *const FABRIC_SERVICE_HEALTH_STATE_CHUNK_LIST,
    pub DeployedApplicationHealthStateChunks:
        *const FABRIC_DEPLOYED_APPLICATION_HEALTH_STATE_CHUNK_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_HEALTH_STATE_CHUNK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_HEALTH_STATE_CHUNK_EX1 {
    pub ApplicationTypeName: FABRIC_URI,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_HEALTH_STATE_CHUNK_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_HEALTH_STATE_CHUNK_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_APPLICATION_HEALTH_STATE_CHUNK,
    pub TotalCount: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_HEALTH_STATE_CHUNK_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_HEALTH_STATE_FILTER {
    pub HealthStateFilter: u32,
    pub ApplicationNameFilter: FABRIC_URI,
    pub ServiceFilters: *const FABRIC_SERVICE_HEALTH_STATE_FILTER_LIST,
    pub DeployedApplicationFilters: *const FABRIC_DEPLOYED_APPLICATION_HEALTH_STATE_FILTER_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_HEALTH_STATE_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_HEALTH_STATE_FILTER_EX1 {
    pub ApplicationTypeNameFilter: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_HEALTH_STATE_FILTER_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_HEALTH_STATE_FILTER_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_APPLICATION_HEALTH_STATE_FILTER,
}
impl Default for FABRIC_APPLICATION_HEALTH_STATE_FILTER_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_HEALTH_STATE_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_APPLICATION_HEALTH_STATE,
}
impl Default for FABRIC_APPLICATION_HEALTH_STATE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_HEALTH_STATISTICS_FILTER {
    pub ExcludeHealthStatistics: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_HEALTH_STATISTICS_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_LOAD_INFORMATION {
    pub Name: LPCWSTR,
    pub MinimumNodes: u32,
    pub MaximumNodes: u32,
    pub NodeCount: u32,
    pub ApplicationLoadMetricInformation: *const FABRIC_APPLICATION_LOAD_METRIC_INFORMATION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_LOAD_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_LOAD_INFORMATION_QUERY_DESCRIPTION {
    pub ApplicationName: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_LOAD_INFORMATION_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_LOAD_METRIC_INFORMATION {
    pub Name: LPCWSTR,
    pub ReservationCapacity: i64,
    pub ApplicationCapacity: i64,
    pub ApplicationLoad: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_LOAD_METRIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_LOAD_METRIC_INFORMATION_LIST {
    pub Count: u32,
    pub LoadMetrics: *mut FABRIC_APPLICATION_LOAD_METRIC_INFORMATION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_LOAD_METRIC_INFORMATION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_METADATA {
    pub ArmMetadata: *mut FABRIC_COMMON_ARM_METADATA,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_METRIC_DESCRIPTION {
    pub Name: LPCWSTR,
    pub NodeReservationCapacity: u32,
    pub MaximumNodeCapacity: u32,
    pub TotalApplicationCapacity: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_METRIC_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_METRIC_LIST {
    pub Count: u32,
    pub Capacities: *mut FABRIC_APPLICATION_METRIC_DESCRIPTION,
}
impl Default for FABRIC_APPLICATION_METRIC_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_NAME_QUERY_DESCRIPTION {
    pub ServiceName: FABRIC_URI,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_NAME_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_NAME_QUERY_RESULT {
    pub ApplicationName: FABRIC_URI,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_NAME_QUERY_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_NETWORK_QUERY_DESCRIPTION {
    pub ApplicationName: FABRIC_URI,
    pub PagingDescription: *const FABRIC_QUERY_PAGING_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_NETWORK_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_NETWORK_QUERY_RESULT_ITEM {
    pub NetworkName: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_NETWORK_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_NETWORK_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_APPLICATION_NETWORK_QUERY_RESULT_ITEM,
}
impl Default for FABRIC_APPLICATION_NETWORK_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_PACKAGE_CLEANUP_POLICY(pub i32);
impl FABRIC_APPLICATION_PACKAGE_CLEANUP_POLICY {
    pub const FABRIC_APPLICATION_PACKAGE_CLEANUP_POLICY_INVALID: Self = Self(0);
    pub const FABRIC_APPLICATION_PACKAGE_CLEANUP_POLICY_DEFAULT: Self = Self(1);
    pub const FABRIC_APPLICATION_PACKAGE_CLEANUP_POLICY_AUTOMATIC: Self = Self(2);
    pub const FABRIC_APPLICATION_PACKAGE_CLEANUP_POLICY_MANUAL: Self = Self(3);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_PARAMETER {
    pub Name: LPCWSTR,
    pub Value: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_PARAMETER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_PARAMETER_LIST {
    pub Count: u32,
    pub Items: *mut FABRIC_APPLICATION_PARAMETER,
}
impl Default for FABRIC_APPLICATION_PARAMETER_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_PRINCIPALS_DESCRIPTION {
    pub Users: *const FABRIC_SECURITY_USER_DESCRIPTION_LIST,
    pub Groups: *const FABRIC_SECURITY_GROUP_DESCRIPTION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_PRINCIPALS_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_PRINCIPALS_DESCRIPTION_EX1 {
    pub ManagedIdentities: *const FABRIC_MANAGED_IDENTITY_DESCRIPTION_LIST,
    pub TokenServiceEndpoint: LPCWSTR,
    pub DefaultIdentity: LPCWSTR,
    pub UseServiceLevelIdentities: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_PRINCIPALS_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_QUERY_DESCRIPTION {
    pub ApplicationNameFilter: FABRIC_URI,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_QUERY_DESCRIPTION_EX1 {
    pub ContinuationToken: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_QUERY_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_QUERY_DESCRIPTION_EX2 {
    pub ApplicationTypeNameFilter: LPCWSTR,
    pub ExcludeApplicationParameters: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_QUERY_DESCRIPTION_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_QUERY_DESCRIPTION_EX3 {
    pub ApplicationDefinitionKindFilter: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_QUERY_DESCRIPTION_EX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_QUERY_DESCRIPTION_EX4 {
    pub MaxResults: i32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_QUERY_DESCRIPTION_EX4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_QUERY_DESCRIPTION_EX5 {
    pub ExcludeManagedApplicationIdentity: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_QUERY_DESCRIPTION_EX5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_QUERY_RESULT_ITEM {
    pub ApplicationName: FABRIC_URI,
    pub ApplicationTypeName: LPCWSTR,
    pub ApplicationTypeVersion: LPCWSTR,
    pub Status: FABRIC_APPLICATION_STATUS,
    pub HealthState: FABRIC_HEALTH_STATE,
    pub ApplicationParameters: *mut FABRIC_APPLICATION_PARAMETER_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_QUERY_RESULT_ITEM_EX1 {
    pub UpgradeTypeVersion: LPCWSTR,
    pub UpgradeParameters: *mut FABRIC_APPLICATION_PARAMETER_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_QUERY_RESULT_ITEM_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_QUERY_RESULT_ITEM_EX2 {
    pub ApplicationDefinitionKind: FABRIC_APPLICATION_DEFINITION_KIND,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_QUERY_RESULT_ITEM_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_QUERY_RESULT_ITEM_EX3 {
    pub ManagedApplicationIdentity: *const FABRIC_MANAGED_APPLICATION_IDENTITY_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_QUERY_RESULT_ITEM_EX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_QUERY_RESULT_ITEM_EX4 {
    pub Metadata: *mut FABRIC_APPLICATION_METADATA,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_QUERY_RESULT_ITEM_EX4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_APPLICATION_QUERY_RESULT_ITEM,
}
impl Default for FABRIC_APPLICATION_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_STATUS(pub i32);
impl FABRIC_APPLICATION_STATUS {
    pub const FABRIC_APPLICATION_STATUS_INVALID: Self = Self(0);
    pub const FABRIC_APPLICATION_STATUS_READY: Self = Self(1);
    pub const FABRIC_APPLICATION_STATUS_UPGRADING: Self = Self(2);
    pub const FABRIC_APPLICATION_STATUS_CREATING: Self = Self(3);
    pub const FABRIC_APPLICATION_STATUS_DELETING: Self = Self(4);
    pub const FABRIC_APPLICATION_STATUS_FAILED: Self = Self(5);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_TYPE_APPLICATIONS_HEALTH_EVALUATION {
    pub Description: LPCWSTR,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub ApplicationTypeName: LPCWSTR,
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub TotalCount: u32,
    pub MaxPercentUnhealthyApplications: u8,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_TYPE_APPLICATIONS_HEALTH_EVALUATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_TYPE_ARM_METADATA_UPDATE_DESCRIPTION {
    pub ArmMetadata: *mut FABRIC_COMMON_ARM_METADATA,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_TYPE_ARM_METADATA_UPDATE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_TYPE_DEFINITION_KIND(pub i32);
impl FABRIC_APPLICATION_TYPE_DEFINITION_KIND {
    pub const FABRIC_APPLICATION_TYPE_DEFINITION_KIND_INVALID: Self = Self(0);
    pub const FABRIC_APPLICATION_TYPE_DEFINITION_KIND_SERVICE_FABRIC_APPLICATION_PACKAGE: Self =
        Self(1);
    pub const FABRIC_APPLICATION_TYPE_DEFINITION_KIND_COMPOSE: Self = Self(2);
    pub const FABRIC_APPLICATION_TYPE_DEFINITION_KIND_MESH_APPLICATION_DESCRIPTION: Self = Self(3);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_TYPE_DEFINITION_KIND_FILTER(pub i32);
impl FABRIC_APPLICATION_TYPE_DEFINITION_KIND_FILTER {
    pub const FABRIC_APPLICATION_TYPE_DEFINITION_KIND_FILTER_DEFAULT: Self = Self(0);
    pub const FABRIC_APPLICATION_TYPE_DEFINITION_KIND_FILTER_ALL: Self = Self(65535);
    pub const FABRIC_APPLICATION_TYPE_DEFINITION_KIND_FILTER_SERVICE_FABRIC_APPLICATION_PACKAGE:
        Self = Self(1);
    pub const FABRIC_APPLICATION_TYPE_DEFINITION_KIND_FILTER_COMPOSE: Self = Self(2);
    pub const FABRIC_APPLICATION_TYPE_DEFINITION_KIND_FILTER_MESH_APPLICATION_DESCRIPTION: Self =
        Self(4);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_TYPE_HEALTH_POLICY_MAP {
    pub Count: u32,
    pub Items: *const FABRIC_APPLICATION_TYPE_HEALTH_POLICY_MAP_ITEM,
}
impl Default for FABRIC_APPLICATION_TYPE_HEALTH_POLICY_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_TYPE_HEALTH_POLICY_MAP_ITEM {
    pub ApplicationTypeName: LPCWSTR,
    pub MaxPercentUnhealthyApplications: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_TYPE_METADATA {
    pub ProvisionTimestamp: FILETIME,
    pub ArmMetadata: *mut FABRIC_COMMON_ARM_METADATA,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_TYPE_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION {
    pub ApplicationTypeNameFilter: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_TYPE_QUERY_RESULT_ITEM {
    pub ApplicationTypeName: LPCWSTR,
    pub ApplicationTypeVersion: LPCWSTR,
    pub DefaultParameters: *mut FABRIC_APPLICATION_PARAMETER_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_TYPE_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_TYPE_QUERY_RESULT_ITEM_EX1 {
    pub Status: FABRIC_APPLICATION_TYPE_STATUS,
    pub StatusDetails: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_TYPE_QUERY_RESULT_ITEM_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_TYPE_QUERY_RESULT_ITEM_EX2 {
    pub ApplicationTypeDefinitionKind: FABRIC_APPLICATION_TYPE_DEFINITION_KIND,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_TYPE_QUERY_RESULT_ITEM_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_TYPE_QUERY_RESULT_ITEM_EX3 {
    pub Metadata: *mut FABRIC_APPLICATION_TYPE_METADATA,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_TYPE_QUERY_RESULT_ITEM_EX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_TYPE_QUERY_RESULT_ITEM_EX4 {
    pub ManagedKeyVaultReferenceParameterList:
        *mut FABRIC_MANAGED_KEY_VAULT_REFERENCE_PARAMETER_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_TYPE_QUERY_RESULT_ITEM_EX4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_TYPE_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_APPLICATION_TYPE_QUERY_RESULT_ITEM,
}
impl Default for FABRIC_APPLICATION_TYPE_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_TYPE_STATUS(pub i32);
impl FABRIC_APPLICATION_TYPE_STATUS {
    pub const FABRIC_APPLICATION_TYPE_STATUS_INVALID: Self = Self(0);
    pub const FABRIC_APPLICATION_TYPE_STATUS_PROVISIONING: Self = Self(1);
    pub const FABRIC_APPLICATION_TYPE_STATUS_AVAILABLE: Self = Self(2);
    pub const FABRIC_APPLICATION_TYPE_STATUS_UNPROVISIONING: Self = Self(3);
    pub const FABRIC_APPLICATION_TYPE_STATUS_FAILED: Self = Self(4);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_UPDATE_DESCRIPTION {
    pub Flags: u32,
    pub ApplicationName: FABRIC_URI,
    pub RemoveApplicationCapacity: bool,
    pub MaximumNodes: u32,
    pub MinimumNodes: u32,
    pub Metrics: *const FABRIC_APPLICATION_METRIC_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_UPDATE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_UPDATE_DESCRIPTION_FLAGS(pub i32);
impl FABRIC_APPLICATION_UPDATE_DESCRIPTION_FLAGS {
    pub const FABRIC_APPLICATION_UPDATE_DESCRIPTION_FLAGS_NONE: Self = Self(0);
    pub const FABRIC_APPLICATION_UPDATE_DESCRIPTION_FLAGS_MINNODES: Self = Self(1);
    pub const FABRIC_APPLICATION_UPDATE_DESCRIPTION_FLAGS_MAXNODES: Self = Self(2);
    pub const FABRIC_APPLICATION_UPDATE_DESCRIPTION_FLAGS_METRICS: Self = Self(4);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_UPGRADE_DESCRIPTION {
    pub ApplicationName: FABRIC_URI,
    pub TargetApplicationTypeVersion: LPCWSTR,
    pub ApplicationParameters: *mut FABRIC_APPLICATION_PARAMETER_LIST,
    pub UpgradeKind: FABRIC_APPLICATION_UPGRADE_KIND,
    pub UpgradePolicyDescription: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_UPGRADE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_UPGRADE_DESCRIPTION_EX1 {
    pub ManagedApplicationIdentity: *mut FABRIC_MANAGED_APPLICATION_IDENTITY_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_UPGRADE_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_UPGRADE_KIND(pub i32);
impl FABRIC_APPLICATION_UPGRADE_KIND {
    pub const FABRIC_APPLICATION_UPGRADE_KIND_INVALID: Self = Self(0);
    pub const FABRIC_APPLICATION_UPGRADE_KIND_ROLLING: Self = Self(1);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_UPGRADE_PROGRESS {
    pub UpgradeDescription: *const FABRIC_APPLICATION_UPGRADE_DESCRIPTION,
    pub UpgradeState: FABRIC_APPLICATION_UPGRADE_STATE,
    pub UpgradeMode: FABRIC_ROLLING_UPGRADE_MODE,
    pub NextUpgradeDomain: LPCWSTR,
    pub UpgradeDomains: *const FABRIC_UPGRADE_DOMAIN_STATUS_DESCRIPTION_LIST,
    pub UpgradeDurationInSeconds: u32,
    pub CurrentUpgradeDomainDurationInSeconds: u32,
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub CurrentUpgradeDomainProgress: *const FABRIC_UPGRADE_DOMAIN_PROGRESS,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_UPGRADE_PROGRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_UPGRADE_PROGRESS_EX1 {
    pub StartTimestampUtc: FILETIME,
    pub FailureTimestampUtc: FILETIME,
    pub FailureReason: FABRIC_UPGRADE_FAILURE_REASON,
    pub UpgradeDomainProgressAtFailure: *const FABRIC_UPGRADE_DOMAIN_PROGRESS,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_UPGRADE_PROGRESS_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_UPGRADE_PROGRESS_EX2 {
    pub UpgradeStatusDetails: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_UPGRADE_PROGRESS_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_UPGRADE_PROGRESS_EX3 {
    pub HealthCheckElapsedTime: u32,
    pub HealthCheckPhase: FABRIC_MONITORED_UPGRADE_HEALTH_CHECK_PHASE,
    pub HealthCheckFlips: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_UPGRADE_PROGRESS_EX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_UPGRADE_STATE(pub i32);
impl FABRIC_APPLICATION_UPGRADE_STATE {
    pub const FABRIC_APPLICATION_UPGRADE_STATE_INVALID: Self = Self(0);
    pub const FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_BACK_IN_PROGRESS: Self = Self(1);
    pub const FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_BACK_COMPLETED: Self = Self(2);
    pub const FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_FORWARD_PENDING: Self = Self(3);
    pub const FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_FORWARD_IN_PROGRESS: Self = Self(4);
    pub const FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_FORWARD_COMPLETED: Self = Self(5);
    pub const FABRIC_APPLICATION_UPGRADE_STATE_FAILED: Self = Self(6);
    pub const FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_BACK_PENDING: Self = Self(7);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_APPLICATION_UPGRADE_UPDATE_DESCRIPTION {
    pub ApplicationName: FABRIC_URI,
    pub UpgradeKind: FABRIC_APPLICATION_UPGRADE_KIND,
    pub UpdateFlags: u32,
    pub UpgradePolicyDescription: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_APPLICATION_UPGRADE_UPDATE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FABRIC_ATOMIC_GROUP_ID(pub i64);
pub const FABRIC_AUTO_SEQUENCE_NUMBER: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_BLOCK_LIST_QUERY_DESCRIPTION {
    pub ServiceName: FABRIC_URI,
    pub r#type: FABRIC_BLOCK_LIST_TYPE,
    pub PagingDescription: *mut FABRIC_QUERY_PAGING_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_BLOCK_LIST_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_BLOCK_LIST_QUERY_RESULT_ITEM {
    pub NodeName: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_BLOCK_LIST_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_BLOCK_LIST_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_BLOCK_LIST_QUERY_RESULT_ITEM,
}
impl Default for FABRIC_BLOCK_LIST_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_BLOCK_LIST_TYPE(pub i32);
impl FABRIC_BLOCK_LIST_TYPE {
    pub const FABRIC_BLOCK_LIST_TYPE_SERVICE: Self = Self(0);
    pub const FABRIC_BLOCK_LIST_TYPE_OVERALL: Self = Self(1);
    pub const FABRIC_BLOCK_LIST_TYPE_PREFERRED_PRIMARY: Self = Self(2);
    pub const FABRIC_BLOCK_LIST_TYPE_PLACEMENT_TAGS: Self = Self(3);
    pub const FABRIC_BLOCK_LIST_TYPE_RUNNING_TAGS: Self = Self(4);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CANCEL_TEST_COMMAND_DESCRIPTION {
    pub OperationId: FABRIC_TEST_COMMAND_OPERATION_ID,
    pub Force: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CANCEL_TEST_COMMAND_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CHAOS_DESCRIPTION {
    pub ChaosParameters: *const FABRIC_CHAOS_PARAMETERS,
    pub Status: FABRIC_CHAOS_STATUS,
    pub ScheduleStatus: FABRIC_CHAOS_SCHEDULE_STATUS,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CHAOS_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CHAOS_EVENT {
    pub Kind: FABRIC_CHAOS_EVENT_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_CHAOS_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CHAOS_EVENTS_SEGMENT {
    pub ContinuationToken: LPCWSTR,
    pub History: *const FABRIC_CHAOS_EVENT_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CHAOS_EVENTS_SEGMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CHAOS_EVENTS_SEGMENT_DESCRIPTION {
    pub Filter: *const FABRIC_CHAOS_EVENTS_SEGMENT_FILTER,
    pub PagingDescription: *const FABRIC_QUERY_PAGING_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CHAOS_EVENTS_SEGMENT_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CHAOS_EVENTS_SEGMENT_FILTER {
    pub StartTimeUtc: FILETIME,
    pub EndTimeUtc: FILETIME,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CHAOS_EVENTS_SEGMENT_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_CHAOS_EVENT_KIND(pub i32);
impl FABRIC_CHAOS_EVENT_KIND {
    pub const FABRIC_CHAOS_EVENT_KIND_INVALID: Self = Self(0);
    pub const FABRIC_CHAOS_EVENT_KIND_STARTED: Self = Self(1);
    pub const FABRIC_CHAOS_EVENT_KIND_EXECUTING_FAULTS: Self = Self(2);
    pub const FABRIC_CHAOS_EVENT_KIND_WAITING: Self = Self(3);
    pub const FABRIC_CHAOS_EVENT_KIND_VALIDATION_FAILED: Self = Self(4);
    pub const FABRIC_CHAOS_EVENT_KIND_TEST_ERROR: Self = Self(5);
    pub const FABRIC_CHAOS_EVENT_KIND_STOPPED: Self = Self(6);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CHAOS_EVENT_LIST {
    pub Count: u32,
    pub Items: *mut FABRIC_CHAOS_EVENT,
}
impl Default for FABRIC_CHAOS_EVENT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CHAOS_PARAMETERS {
    pub MaxClusterStabilizationTimeoutInSeconds: u32,
    pub MaxConcurrentFaults: u32,
    pub EnableMoveReplicaFaults: bool,
    pub TimeToRunInSeconds: u64,
    pub WaitTimeBetweenIterationsInSeconds: u32,
    pub WaitTimeBetweenFaultsInSeconds: u32,
    pub Context: *const FABRIC_EVENT_CONTEXT_MAP,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CHAOS_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CHAOS_PARAMETERS_EX1 {
    pub ClusterHealthPolicy: *const FABRIC_CLUSTER_HEALTH_POLICY,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CHAOS_PARAMETERS_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CHAOS_PARAMETERS_EX2 {
    pub ChaosTargetFilter: *const FABRIC_CHAOS_TARGET_FILTER,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CHAOS_PARAMETERS_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CHAOS_REPORT {
    pub ChaosParameters: *mut FABRIC_CHAOS_PARAMETERS,
    pub Status: FABRIC_CHAOS_STATUS,
    pub ContinuationToken: LPCWSTR,
    pub History: *const FABRIC_CHAOS_EVENT_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CHAOS_REPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CHAOS_REPORT_FILTER {
    pub StartTimeUtc: FILETIME,
    pub EndTimeUtc: FILETIME,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CHAOS_REPORT_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CHAOS_SCHEDULE {
    pub StartDate: FILETIME,
    pub ExpiryDate: FILETIME,
    pub ChaosParametersMap: *const FABRIC_CHAOS_SCHEDULE_CHAOS_PARAMETERS_MAP,
    pub Jobs: *const FABRIC_CHAOS_SCHEDULE_JOB_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CHAOS_SCHEDULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CHAOS_SCHEDULE_CHAOS_PARAMETERS_MAP {
    pub Count: u32,
    pub Items: *mut FABRIC_CHAOS_SCHEDULE_CHAOS_PARAMETERS_MAP_ITEM,
}
impl Default for FABRIC_CHAOS_SCHEDULE_CHAOS_PARAMETERS_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CHAOS_SCHEDULE_CHAOS_PARAMETERS_MAP_ITEM {
    pub Name: LPCWSTR,
    pub Parameters: *const FABRIC_CHAOS_PARAMETERS,
}
impl Default for FABRIC_CHAOS_SCHEDULE_CHAOS_PARAMETERS_MAP_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CHAOS_SCHEDULE_DESCRIPTION {
    pub Version: u32,
    pub Schedule: *const FABRIC_CHAOS_SCHEDULE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CHAOS_SCHEDULE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CHAOS_SCHEDULE_JOB {
    pub ChaosParameters: LPCWSTR,
    pub Days: *const FABRIC_CHAOS_SCHEDULE_JOB_ACTIVE_DAYS,
    pub Times: *const FABRIC_CHAOS_SCHEDULE_TIME_RANGE_UTC_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CHAOS_SCHEDULE_JOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CHAOS_SCHEDULE_JOB_ACTIVE_DAYS {
    pub Sunday: bool,
    pub Monday: bool,
    pub Tuesday: bool,
    pub Wednesday: bool,
    pub Thursday: bool,
    pub Friday: bool,
    pub Saturday: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CHAOS_SCHEDULE_JOB_ACTIVE_DAYS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CHAOS_SCHEDULE_JOB_LIST {
    pub Count: u32,
    pub Items: *mut FABRIC_CHAOS_SCHEDULE_JOB,
}
impl Default for FABRIC_CHAOS_SCHEDULE_JOB_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_CHAOS_SCHEDULE_STATUS(pub i32);
impl FABRIC_CHAOS_SCHEDULE_STATUS {
    pub const FABRIC_CHAOS_SCHEDULE_STATUS_INVALID: Self = Self(0);
    pub const FABRIC_CHAOS_SCHEDULE_STATUS_ACTIVE: Self = Self(1);
    pub const FABRIC_CHAOS_SCHEDULE_STATUS_EXPIRED: Self = Self(2);
    pub const FABRIC_CHAOS_SCHEDULE_STATUS_PENDING: Self = Self(3);
    pub const FABRIC_CHAOS_SCHEDULE_STATUS_STOPPED: Self = Self(4);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CHAOS_SCHEDULE_TIME_RANGE_UTC {
    pub StartTime: *const FABRIC_CHAOS_SCHEDULE_TIME_UTC,
    pub EndTime: *const FABRIC_CHAOS_SCHEDULE_TIME_UTC,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CHAOS_SCHEDULE_TIME_RANGE_UTC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CHAOS_SCHEDULE_TIME_RANGE_UTC_LIST {
    pub Count: u32,
    pub Items: *mut FABRIC_CHAOS_SCHEDULE_TIME_RANGE_UTC,
}
impl Default for FABRIC_CHAOS_SCHEDULE_TIME_RANGE_UTC_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CHAOS_SCHEDULE_TIME_UTC {
    pub Hour: u32,
    pub Minute: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CHAOS_SCHEDULE_TIME_UTC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CHAOS_SERVICE_SCHEDULE_DESCRIPTION {
    pub ChaosScheduleDescription: *const FABRIC_CHAOS_SCHEDULE_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CHAOS_SERVICE_SCHEDULE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_CHAOS_STATUS(pub i32);
impl FABRIC_CHAOS_STATUS {
    pub const FABRIC_CHAOS_STATUS_INVALID: Self = Self(0);
    pub const FABRIC_CHAOS_STATUS_RUNNING: Self = Self(1);
    pub const FABRIC_CHAOS_STATUS_STOPPED: Self = Self(2);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CHAOS_TARGET_FILTER {
    pub NodeTypeInclusionList: *const FABRIC_STRING_LIST,
    pub ApplicationInclusionList: *const FABRIC_STRING_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CHAOS_TARGET_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CHECK_EXISTS_PROPERTY_OPERATION {
    pub PropertyName: LPCWSTR,
    pub ExistenceCheck: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CHECK_EXISTS_PROPERTY_OPERATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CHECK_SEQUENCE_PROPERTY_OPERATION {
    pub PropertyName: LPCWSTR,
    pub SequenceNumber: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CHECK_SEQUENCE_PROPERTY_OPERATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CHECK_VALUE_PROPERTY_OPERATION {
    pub PropertyName: LPCWSTR,
    pub PropertyTypeId: FABRIC_PROPERTY_TYPE_ID,
    pub PropertyValue: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CHECK_VALUE_PROPERTY_OPERATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CLAIMS_CREDENTIALS {
    pub ServerCommonNameCount: u32,
    pub ServerCommonNames: *mut LPCWSTR,
    pub IssuerThumbprintCount: u32,
    pub IssuerThumbprints: *mut LPCWSTR,
    pub LocalClaims: LPCWSTR,
    pub ProtectionLevel: FABRIC_PROTECTION_LEVEL,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CLAIMS_CREDENTIALS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CLAIMS_CREDENTIALS_EX1 {
    pub ServerThumbprintCount: u32,
    pub ServerThumbprints: *mut LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CLAIMS_CREDENTIALS_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CLAIMS_RETRIEVAL_METADATA {
    pub Kind: FABRIC_CLAIMS_RETRIEVAL_METADATA_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_CLAIMS_RETRIEVAL_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_CLAIMS_RETRIEVAL_METADATA_KIND(pub i32);
impl FABRIC_CLAIMS_RETRIEVAL_METADATA_KIND {
    pub const FABRIC_CLAIMS_RETRIEVAL_METADATA_KIND_NONE: Self = Self(0);
    pub const FABRIC_CLAIMS_RETRIEVAL_METADATA_KIND_AAD: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_CLIENT_ROLE(pub i32);
impl FABRIC_CLIENT_ROLE {
    pub const FABRIC_CLIENT_ROLE_UNKNOWN: Self = Self(0);
    pub const FABRIC_CLIENT_ROLE_USER: Self = Self(1);
    pub const FABRIC_CLIENT_ROLE_ADMIN: Self = Self(2);
    pub const FABRIC_CLIENT_ROLE_ELEVATED_ADMIN: Self = Self(4);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CLIENT_SETTINGS {
    pub PartitionLocationCacheLimit: u32,
    pub ServiceChangePollIntervalInSeconds: u32,
    pub ConnectionInitializationTimeoutInSeconds: u32,
    pub KeepAliveIntervalInSeconds: u32,
    pub HealthOperationTimeoutInSeconds: u32,
    pub HealthReportSendIntervalInSeconds: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CLIENT_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CLIENT_SETTINGS_EX1 {
    pub ClientFriendlyName: LPCWSTR,
    pub PartitionLocationCacheBucketCount: u32,
    pub HealthReportRetrySendIntervalInSeconds: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CLIENT_SETTINGS_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CLIENT_SETTINGS_EX2 {
    pub NotificationGatewayConnectionTimeoutInSeconds: u32,
    pub NotificationCacheUpdateTimeoutInSeconds: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CLIENT_SETTINGS_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CLIENT_SETTINGS_EX3 {
    pub AuthTokenBufferSize: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CLIENT_SETTINGS_EX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CLIENT_SETTINGS_EX4 {
    pub ConnectionIdleTimeoutInSeconds: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CLIENT_SETTINGS_EX4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CLIENT_SETTINGS_EX5 {
    pub AllowHealthReportCleanup: bool,
    pub HealthReportDropTransientReportTtlThresholdInSeconds: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CLIENT_SETTINGS_EX5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CLUSTER_HEALTH {
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CLUSTER_HEALTH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CLUSTER_HEALTH_CHUNK {
    pub HealthState: FABRIC_HEALTH_STATE,
    pub NodeHealthStateChunks: *const FABRIC_NODE_HEALTH_STATE_CHUNK_LIST,
    pub ApplicationHealthStateChunks: *const FABRIC_APPLICATION_HEALTH_STATE_CHUNK_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CLUSTER_HEALTH_CHUNK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CLUSTER_HEALTH_CHUNK_QUERY_DESCRIPTION {
    pub ClusterHealthPolicy: *const FABRIC_CLUSTER_HEALTH_POLICY,
    pub ApplicationHealthPolicyMap: *const FABRIC_APPLICATION_HEALTH_POLICY_MAP,
    pub ApplicationFilters: *const FABRIC_APPLICATION_HEALTH_STATE_FILTER_LIST,
    pub NodeFilters: *const FABRIC_NODE_HEALTH_STATE_FILTER_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CLUSTER_HEALTH_CHUNK_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CLUSTER_HEALTH_EX1 {
    pub NodeHealthStates: *const FABRIC_NODE_HEALTH_STATE_LIST,
    pub ApplicationHealthStates: *const FABRIC_APPLICATION_HEALTH_STATE_LIST,
    pub HealthEvents: *const FABRIC_HEALTH_EVENT_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CLUSTER_HEALTH_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CLUSTER_HEALTH_EX2 {
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CLUSTER_HEALTH_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CLUSTER_HEALTH_EX3 {
    pub HealthStatistics: *const FABRIC_HEALTH_STATISTICS,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CLUSTER_HEALTH_EX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CLUSTER_HEALTH_POLICY {
    pub ConsiderWarningAsError: bool,
    pub MaxPercentUnhealthyNodes: u8,
    pub MaxPercentUnhealthyApplications: u8,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CLUSTER_HEALTH_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CLUSTER_HEALTH_POLICY_EX1 {
    pub ApplicationTypeHealthPolicyMap: *const FABRIC_APPLICATION_TYPE_HEALTH_POLICY_MAP,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CLUSTER_HEALTH_POLICY_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CLUSTER_HEALTH_POLICY_EX2 {
    pub NodeTypeHealthPolicyMap: *const FABRIC_NODE_TYPE_HEALTH_POLICY_MAP,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CLUSTER_HEALTH_POLICY_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CLUSTER_HEALTH_QUERY_DESCRIPTION {
    pub HealthPolicy: *const FABRIC_CLUSTER_HEALTH_POLICY,
    pub ApplicationHealthPolicyMap: *const FABRIC_APPLICATION_HEALTH_POLICY_MAP,
    pub EventsFilter: *const FABRIC_HEALTH_EVENTS_FILTER,
    pub NodesFilter: *const FABRIC_NODE_HEALTH_STATES_FILTER,
    pub ApplicationsFilter: *const FABRIC_APPLICATION_HEALTH_STATES_FILTER,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CLUSTER_HEALTH_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CLUSTER_HEALTH_QUERY_DESCRIPTION_EX1 {
    pub HealthStatisticsFilter: *const FABRIC_CLUSTER_HEALTH_STATISTICS_FILTER,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CLUSTER_HEALTH_QUERY_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CLUSTER_HEALTH_REPORT {
    pub HealthInformation: *const FABRIC_HEALTH_INFORMATION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CLUSTER_HEALTH_REPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CLUSTER_HEALTH_STATISTICS_FILTER {
    pub ExcludeHealthStatistics: bool,
    pub IncludeSystemApplicationHealthStatistics: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CLUSTER_HEALTH_STATISTICS_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CLUSTER_LOAD_INFORMATION {
    pub LastBalancingStartTimeUtc: FILETIME,
    pub LastBalancingEndTimeUtc: FILETIME,
    pub LoadMetricInformation: *const FABRIC_LOAD_METRIC_INFORMATION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CLUSTER_LOAD_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CLUSTER_MANIFEST_QUERY_DESCRIPTION {
    pub ClusterManifestVersion: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CLUSTER_MANIFEST_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CLUSTER_UPGRADE_HEALTH_POLICY {
    pub MaxPercentDeltaUnhealthyNodes: u8,
    pub MaxPercentUpgradeDomainDeltaUnhealthyNodes: u8,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CLUSTER_UPGRADE_HEALTH_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CODE_PACKAGE_ACTIVATION_INFORMATION {
    pub CodePackageName: LPCWSTR,
    pub ExecutionPolicy: *mut FABRIC_EXECUTION_POLICY_DESCRIPTION,
}
impl Default for FABRIC_CODE_PACKAGE_ACTIVATION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CODE_PACKAGE_ACTIVATION_INFORMATION_LIST {
    pub Count: u32,
    pub Items: *mut FABRIC_CODE_PACKAGE_ACTIVATION_INFORMATION,
}
impl Default for FABRIC_CODE_PACKAGE_ACTIVATION_INFORMATION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CODE_PACKAGE_DESCRIPTION {
    pub Name: LPCWSTR,
    pub Version: LPCWSTR,
    pub ServiceManifestName: LPCWSTR,
    pub ServiceManifestVersion: LPCWSTR,
    pub IsShared: bool,
    pub SetupEntryPoint: *mut FABRIC_EXEHOST_ENTRY_POINT_DESCRIPTION,
    pub EntryPoint: *mut FABRIC_CODE_PACKAGE_ENTRY_POINT_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CODE_PACKAGE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CODE_PACKAGE_DESCRIPTION_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_CODE_PACKAGE_DESCRIPTION,
}
impl Default for FABRIC_CODE_PACKAGE_DESCRIPTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CODE_PACKAGE_ENTRY_POINT {
    pub EntryPointLocation: LPCWSTR,
    pub ProcessId: i64,
    pub RunAsUserName: LPCWSTR,
    pub EntryPointStatus: FABRIC_ENTRY_POINT_STATUS,
    pub NextActivationUtc: FILETIME,
    pub Statistics: *const FABRIC_CODE_PACKAGE_ENTRY_POINT_STATISTICS,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CODE_PACKAGE_ENTRY_POINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CODE_PACKAGE_ENTRY_POINT_DESCRIPTION {
    pub Kind: FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_CODE_PACKAGE_ENTRY_POINT_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CODE_PACKAGE_ENTRY_POINT_EX1 {
    pub CodePackageInstanceId: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CODE_PACKAGE_ENTRY_POINT_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CODE_PACKAGE_ENTRY_POINT_EX2 {
    pub ContainerId: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CODE_PACKAGE_ENTRY_POINT_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND(pub i32);
impl FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND {
    pub const FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND_INVALID: Self = Self(0);
    pub const FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND_NONE: Self = Self(1);
    pub const FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND_EXEHOST: Self = Self(2);
    pub const FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND_DLLHOST: Self = Self(3);
    pub const FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND_CONTAINERHOST: Self = Self(4);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CODE_PACKAGE_ENTRY_POINT_STATISTICS {
    pub LastExitCode: u32,
    pub LastActivationUtc: FILETIME,
    pub LastExitUtc: FILETIME,
    pub LastSuccessfulActivationUtc: FILETIME,
    pub LastSuccessfulExitUtc: FILETIME,
    pub ActivationCount: u32,
    pub ActivationFailureCount: u32,
    pub ContinuousActivationFailureCount: u32,
    pub ExitCount: u32,
    pub ExitFailureCount: u32,
    pub ContinuousExitFailureCount: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CODE_PACKAGE_ENTRY_POINT_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CODE_PACKAGE_EVENT_DESCRIPTION {
    pub CodePackageName: LPCWSTR,
    pub IsSetupEntryPoint: windows_core::BOOL,
    pub IsContainerHost: windows_core::BOOL,
    pub EventType: FABRIC_CODE_PACKAGE_EVENT_TYPE,
    pub TimeStampInTicks: i64,
    pub SequenceNumber: i64,
    pub Properties: *mut FABRIC_STRING_MAP,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CODE_PACKAGE_EVENT_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_CODE_PACKAGE_EVENT_TYPE(pub i32);
impl FABRIC_CODE_PACKAGE_EVENT_TYPE {
    pub const FABRIC_CODE_PACKAGE_EVENT_TYPE_INVALID: Self = Self(0);
    pub const FABRIC_CODE_PACKAGE_EVENT_TYPE_START_FAILED: Self = Self(1);
    pub const FABRIC_CODE_PACKAGE_EVENT_TYPE_STARTED: Self = Self(2);
    pub const FABRIC_CODE_PACKAGE_EVENT_TYPE_READY: Self = Self(3);
    pub const FABRIC_CODE_PACKAGE_EVENT_TYPE_HEALTH: Self = Self(4);
    pub const FABRIC_CODE_PACKAGE_EVENT_TYPE_STOPPED: Self = Self(5);
    pub const FABRIC_CODE_PACKAGE_EVENT_TYPE_TERMINATED: Self = Self(6);
    pub const FABRIC_CODE_PACKAGE_EVENT_TYPE_RAN_TO_COMPLETION: Self = Self(7);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CODE_PACKAGE_USAGE_STATISTICS {
    pub CodePackageStats: *mut FABRIC_STRING_MAP,
    pub TimeRead: FILETIME,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CODE_PACKAGE_USAGE_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_COMMON_ARM_METADATA {
    pub ArmResourceId: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_COMMON_ARM_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_COMPLETE_REPLICA_DESCRIPTION {
    pub NodeName: LPCWSTR,
    pub PartitionId: FABRIC_PARTITION_ID,
    pub ReplicaOrInstanceId: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_COMPLETE_REPLICA_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CONFIGURATION_PACKAGE_DESCRIPTION {
    pub Name: LPCWSTR,
    pub Version: LPCWSTR,
    pub ServiceManifestName: LPCWSTR,
    pub ServiceManifestVersion: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CONFIGURATION_PACKAGE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CONFIGURATION_PACKAGE_DESCRIPTION_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_CONFIGURATION_PACKAGE_DESCRIPTION,
}
impl Default for FABRIC_CONFIGURATION_PACKAGE_DESCRIPTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CONFIGURATION_PARAMETER {
    pub Name: LPCWSTR,
    pub Value: LPCWSTR,
    pub MustOverride: bool,
    pub IsEncrypted: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CONFIGURATION_PARAMETER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CONFIGURATION_PARAMETER_EX1 {
    pub Type: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CONFIGURATION_PARAMETER_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CONFIGURATION_PARAMETER_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_CONFIGURATION_PARAMETER,
}
impl Default for FABRIC_CONFIGURATION_PARAMETER_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CONFIGURATION_PARAMETER_OVERRIDE {
    pub SectionName: LPCWSTR,
    pub ParameterName: LPCWSTR,
    pub ParameterValue: LPCWSTR,
    pub Timeout: u32,
    pub PersistAcrossUpgrade: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CONFIGURATION_PARAMETER_OVERRIDE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CONFIGURATION_PARAMETER_OVERRIDE_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_CONFIGURATION_PARAMETER_OVERRIDE,
}
impl Default for FABRIC_CONFIGURATION_PARAMETER_OVERRIDE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CONFIGURATION_SECTION {
    pub Name: LPCWSTR,
    pub Parameters: *const FABRIC_CONFIGURATION_PARAMETER_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CONFIGURATION_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CONFIGURATION_SECTION_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_CONFIGURATION_SECTION,
}
impl Default for FABRIC_CONFIGURATION_SECTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CONFIGURATION_SETTINGS {
    pub Sections: *const FABRIC_CONFIGURATION_SECTION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CONFIGURATION_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_CONTAINERHOST_ENTRY_POINT_DESCRIPTION {
    pub ImageName: LPCWSTR,
    pub Commands: LPCWSTR,
    pub EntryPoint: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_CONTAINERHOST_ENTRY_POINT_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_DATA_LOSS_MODE(pub i32);
impl FABRIC_DATA_LOSS_MODE {
    pub const FABRIC_DATA_LOSS_MODE_INVALID: Self = Self(0);
    pub const FABRIC_DATA_LOSS_MODE_PARTIAL: Self = Self(1);
    pub const FABRIC_DATA_LOSS_MODE_FULL: Self = Self(2);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DATA_PACKAGE_DESCRIPTION {
    pub Name: LPCWSTR,
    pub Version: LPCWSTR,
    pub ServiceManifestName: LPCWSTR,
    pub ServiceManifestVersion: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DATA_PACKAGE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DATA_PACKAGE_DESCRIPTION_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_DATA_PACKAGE_DESCRIPTION,
}
impl Default for FABRIC_DATA_PACKAGE_DESCRIPTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DELETE_APPLICATION_DESCRIPTION {
    pub ApplicationName: FABRIC_URI,
    pub ForceDelete: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DELETE_APPLICATION_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DELETE_NETWORK_DESCRIPTION {
    pub NetworkName: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DELETE_NETWORK_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DELETE_PROPERTY_OPERATION {
    pub PropertyName: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DELETE_PROPERTY_OPERATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DELETE_SERVICE_DESCRIPTION {
    pub ServiceName: FABRIC_URI,
    pub ForceDelete: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DELETE_SERVICE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DELTA_NODES_CHECK_HEALTH_EVALUATION {
    pub Description: LPCWSTR,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub BaselineErrorCount: u32,
    pub BaselineTotalCount: u32,
    pub TotalCount: u32,
    pub MaxPercentDeltaUnhealthyNodes: u8,
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DELTA_NODES_CHECK_HEALTH_EVALUATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_APPLICATIONS_HEALTH_EVALUATION {
    pub Description: LPCWSTR,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub TotalCount: u32,
    pub MaxPercentUnhealthyDeployedApplications: u8,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_APPLICATIONS_HEALTH_EVALUATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_APPLICATION_HEALTH {
    pub ApplicationName: FABRIC_URI,
    pub NodeName: LPCWSTR,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub HealthEvents: *const FABRIC_HEALTH_EVENT_LIST,
    pub DeployedServicePackageHealthStates:
        *const FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_APPLICATION_HEALTH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_APPLICATION_HEALTH_EVALUATION {
    pub Description: LPCWSTR,
    pub ApplicationName: FABRIC_URI,
    pub NodeName: LPCWSTR,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_APPLICATION_HEALTH_EVALUATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_APPLICATION_HEALTH_EX1 {
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_APPLICATION_HEALTH_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_APPLICATION_HEALTH_EX2 {
    pub HealthStatistics: *const FABRIC_HEALTH_STATISTICS,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_APPLICATION_HEALTH_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_APPLICATION_HEALTH_QUERY_DESCRIPTION {
    pub ApplicationName: FABRIC_URI,
    pub NodeName: LPCWSTR,
    pub HealthPolicy: *const FABRIC_APPLICATION_HEALTH_POLICY,
    pub EventsFilter: *const FABRIC_HEALTH_EVENTS_FILTER,
    pub DeployedServicePackagesFilter: *const FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATES_FILTER,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_APPLICATION_HEALTH_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_APPLICATION_HEALTH_QUERY_DESCRIPTION_EX1 {
    pub HealthStatisticsFilter: *const FABRIC_DEPLOYED_APPLICATION_HEALTH_STATISTICS_FILTER,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_APPLICATION_HEALTH_QUERY_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT {
    pub ApplicationName: FABRIC_URI,
    pub NodeName: LPCWSTR,
    pub HealthInformation: *const FABRIC_HEALTH_INFORMATION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_APPLICATION_HEALTH_STATE {
    pub ApplicationName: FABRIC_URI,
    pub NodeName: LPCWSTR,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_APPLICATION_HEALTH_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_APPLICATION_HEALTH_STATES_FILTER {
    pub HealthStateFilter: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_APPLICATION_HEALTH_STATES_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_APPLICATION_HEALTH_STATE_CHUNK {
    pub NodeName: LPCWSTR,
    pub HealthState: FABRIC_HEALTH_STATE,
    pub DeployedServicePackageHealthStateChunks:
        *const FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE_CHUNK_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_APPLICATION_HEALTH_STATE_CHUNK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_APPLICATION_HEALTH_STATE_CHUNK_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_DEPLOYED_APPLICATION_HEALTH_STATE_CHUNK,
    pub TotalCount: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_APPLICATION_HEALTH_STATE_CHUNK_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_APPLICATION_HEALTH_STATE_FILTER {
    pub HealthStateFilter: u32,
    pub NodeNameFilter: LPCWSTR,
    pub DeployedServicePackageFilters:
        *const FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE_FILTER_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_APPLICATION_HEALTH_STATE_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_APPLICATION_HEALTH_STATE_FILTER_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_DEPLOYED_APPLICATION_HEALTH_STATE_FILTER,
}
impl Default for FABRIC_DEPLOYED_APPLICATION_HEALTH_STATE_FILTER_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_APPLICATION_HEALTH_STATE_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_DEPLOYED_APPLICATION_HEALTH_STATE,
}
impl Default for FABRIC_DEPLOYED_APPLICATION_HEALTH_STATE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_APPLICATION_HEALTH_STATISTICS_FILTER {
    pub ExcludeHealthStatistics: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_APPLICATION_HEALTH_STATISTICS_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_APPLICATION_QUERY_DESCRIPTION {
    pub NodeName: LPCWSTR,
    pub ApplicationNameFilter: FABRIC_URI,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_APPLICATION_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_APPLICATION_QUERY_RESULT_ITEM {
    pub ApplicationName: FABRIC_URI,
    pub ApplicationTypeName: LPCWSTR,
    pub DeployedApplicationStatus: FABRIC_DEPLOYMENT_STATUS,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_APPLICATION_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_APPLICATION_QUERY_RESULT_ITEM_EX {
    pub WorkDirectory: LPCWSTR,
    pub LogDirectory: LPCWSTR,
    pub TempDirectory: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_APPLICATION_QUERY_RESULT_ITEM_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_APPLICATION_QUERY_RESULT_ITEM_EX2 {
    pub HealthState: FABRIC_HEALTH_STATE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_APPLICATION_QUERY_RESULT_ITEM_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_APPLICATION_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_DEPLOYED_APPLICATION_QUERY_RESULT_ITEM,
}
impl Default for FABRIC_DEPLOYED_APPLICATION_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_CODE_PACKAGE_QUERY_DESCRIPTION {
    pub NodeName: LPCWSTR,
    pub ApplicationName: FABRIC_URI,
    pub ServiceManifestNameFilter: LPCWSTR,
    pub CodePackageNameFilter: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_CODE_PACKAGE_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_CODE_PACKAGE_QUERY_DESCRIPTION_EX1 {
    pub IncludeCodePackageUsageStats: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_CODE_PACKAGE_QUERY_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_CODE_PACKAGE_QUERY_RESULT_ITEM {
    pub CodePackageName: LPCWSTR,
    pub CodePackageVersion: LPCWSTR,
    pub ServiceManifestName: LPCWSTR,
    pub RunFrequencyInterval: u32,
    pub DeployedCodePackageStatus: FABRIC_DEPLOYMENT_STATUS,
    pub SetupEntryPoint: *const FABRIC_CODE_PACKAGE_ENTRY_POINT,
    pub EntryPoint: *const FABRIC_CODE_PACKAGE_ENTRY_POINT,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_CODE_PACKAGE_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_CODE_PACKAGE_QUERY_RESULT_ITEM_EX1 {
    pub ServicePackageActivationId: LPCWSTR,
    pub HostType: FABRIC_HOST_TYPE,
    pub HostIsolationMode: FABRIC_HOST_ISOLATION_MODE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_CODE_PACKAGE_QUERY_RESULT_ITEM_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_CODE_PACKAGE_QUERY_RESULT_ITEM_EX2 {
    pub CodePackageStats: *mut FABRIC_CODE_PACKAGE_USAGE_STATISTICS,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_CODE_PACKAGE_QUERY_RESULT_ITEM_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_CODE_PACKAGE_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_DEPLOYED_CODE_PACKAGE_QUERY_RESULT_ITEM,
}
impl Default for FABRIC_DEPLOYED_CODE_PACKAGE_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_CODE_PACKAGE_RESULT {
    pub NodeName: LPCWSTR,
    pub ApplicationName: FABRIC_URI,
    pub ServiceManifestName: LPCWSTR,
    pub CodePackageName: LPCWSTR,
    pub CodePackageInstanceId: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_CODE_PACKAGE_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_CODE_PACKAGE_RESULT_EX1 {
    pub ServicePackageActivationId: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_CODE_PACKAGE_RESULT_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_NETWORK_CODE_PACKAGE_QUERY_DESCRIPTION {
    pub NodeName: LPCWSTR,
    pub NetworkName: LPCWSTR,
    pub ApplicationNameFilter: FABRIC_URI,
    pub ServiceManifestNameFilter: LPCWSTR,
    pub CodePackageNameFilter: LPCWSTR,
    pub PagingDescription: *const FABRIC_QUERY_PAGING_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_NETWORK_CODE_PACKAGE_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_NETWORK_CODE_PACKAGE_QUERY_RESULT_ITEM {
    pub ApplicationName: FABRIC_URI,
    pub NetworkName: LPCWSTR,
    pub CodePackageName: LPCWSTR,
    pub CodePackageVersion: LPCWSTR,
    pub ServiceManifestName: LPCWSTR,
    pub ServicePackageActivationId: LPCWSTR,
    pub ContainerAddress: LPCWSTR,
    pub ContainerId: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_NETWORK_CODE_PACKAGE_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_NETWORK_CODE_PACKAGE_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_DEPLOYED_NETWORK_CODE_PACKAGE_QUERY_RESULT_ITEM,
}
impl Default for FABRIC_DEPLOYED_NETWORK_CODE_PACKAGE_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_NETWORK_QUERY_DESCRIPTION {
    pub NodeName: LPCWSTR,
    pub PagingDescription: *const FABRIC_QUERY_PAGING_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_NETWORK_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_NETWORK_QUERY_RESULT_ITEM {
    pub NetworkName: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_NETWORK_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_NETWORK_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_DEPLOYED_NETWORK_QUERY_RESULT_ITEM,
}
impl Default for FABRIC_DEPLOYED_NETWORK_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SELF_RECONFIGURING_SERVICE_INSTANCE_DETAIL_QUERY_RESULT_ITEM {
    pub ServiceName: FABRIC_URI,
    pub PartitionId: FABRIC_PARTITION_ID,
    pub InstanceId: i64,
    pub CurrentServiceOperation: FABRIC_QUERY_SERVICE_OPERATION_NAME,
    pub CurrentServiceOperationStartTimeUtc: FILETIME,
    pub ReportedLoad: *mut FABRIC_LOAD_METRIC_REPORT_LIST,
    pub DeployedServiceInstance:
        *mut FABRIC_DEPLOYED_SELF_RECONFIGURING_SERVICE_INSTANCE_QUERY_RESULT_ITEM,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SELF_RECONFIGURING_SERVICE_INSTANCE_DETAIL_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SELF_RECONFIGURING_SERVICE_INSTANCE_QUERY_RESULT_ITEM {
    pub ServiceName: FABRIC_URI,
    pub ServiceTypeName: LPCWSTR,
    pub ServiceManifestVersion: LPCWSTR,
    pub CodePackageName: LPCWSTR,
    pub PartitionId: FABRIC_PARTITION_ID,
    pub InstanceId: i64,
    pub InstanceRole: FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE,
    pub ReplicaStatus: FABRIC_QUERY_SERVICE_REPLICA_STATUS,
    pub Address: LPCWSTR,
    pub ServiceManifestName: LPCWSTR,
    pub ServicePackageActivationId: LPCWSTR,
    pub HostProcessId: i64,
    pub ReconfigurationInformation: *mut FABRIC_RECONFIGURATION_INFORMATION_QUERY_RESULT,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SELF_RECONFIGURING_SERVICE_INSTANCE_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_PACKAGES_HEALTH_EVALUATION {
    pub Description: LPCWSTR,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub TotalCount: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_PACKAGES_HEALTH_EVALUATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH {
    pub ApplicationName: FABRIC_URI,
    pub ServiceManifestName: LPCWSTR,
    pub NodeName: LPCWSTR,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub HealthEvents: *const FABRIC_HEALTH_EVENT_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_EVALUATION {
    pub Description: LPCWSTR,
    pub ApplicationName: FABRIC_URI,
    pub ServiceManifestName: LPCWSTR,
    pub NodeName: LPCWSTR,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_EVALUATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_EVALUATION_EX1 {
    pub ServicePackageActivationId: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_EVALUATION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_EX1 {
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_EX2 {
    pub ServicePackageActivationId: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_QUERY_DESCRIPTION {
    pub ApplicationName: FABRIC_URI,
    pub NodeName: LPCWSTR,
    pub ServiceManifestName: LPCWSTR,
    pub HealthPolicy: *const FABRIC_APPLICATION_HEALTH_POLICY,
    pub EventsFilter: *const FABRIC_HEALTH_EVENTS_FILTER,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_QUERY_DESCRIPTION_EX1 {
    pub ServicePackageActivationId: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_QUERY_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT {
    pub ApplicationName: FABRIC_URI,
    pub ServiceManifestName: LPCWSTR,
    pub NodeName: LPCWSTR,
    pub HealthInformation: *const FABRIC_HEALTH_INFORMATION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT_EX1 {
    pub ServicePackageActivationId: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE {
    pub ApplicationName: FABRIC_URI,
    pub ServiceManifestName: LPCWSTR,
    pub NodeName: LPCWSTR,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATES_FILTER {
    pub HealthStateFilter: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATES_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE_CHUNK {
    pub ServiceManifestName: LPCWSTR,
    pub HealthState: FABRIC_HEALTH_STATE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE_CHUNK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE_CHUNK_EX1 {
    pub ServicePackageActivationId: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE_CHUNK_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE_CHUNK_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE_CHUNK,
    pub TotalCount: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE_CHUNK_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE_EX1 {
    pub ServicePackageActivationId: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE_FILTER {
    pub HealthStateFilter: u32,
    pub ServiceManifestNameFilter: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE_FILTER_EX1 {
    pub ServicePackageActivationIdFilter: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE_FILTER_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE_FILTER_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE_FILTER,
}
impl Default for FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE_FILTER_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE,
}
impl Default for FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_DESCRIPTION {
    pub NodeName: LPCWSTR,
    pub ApplicationName: FABRIC_URI,
    pub ServiceManifestNameFilter: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_RESULT_ITEM {
    pub ServiceManifestName: LPCWSTR,
    pub ServiceManifestVersion: LPCWSTR,
    pub DeployedServicePackageStatus: FABRIC_DEPLOYMENT_STATUS,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_RESULT_ITEM_EX1 {
    pub ServicePackageActivationId: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_RESULT_ITEM_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_RESULT_ITEM_EX2 {
    pub HealthState: FABRIC_HEALTH_STATE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_RESULT_ITEM_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_RESULT_ITEM,
}
impl Default for FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_REPLICA_DETAIL_QUERY_DESCRIPTION {
    pub NodeName: LPCWSTR,
    pub PartitionId: FABRIC_PARTITION_ID,
    pub ReplicaId: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_REPLICA_DETAIL_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_REPLICA_DETAIL_QUERY_RESULT_ITEM {
    pub Kind: FABRIC_SERVICE_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_REPLICA_DETAIL_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_REPLICA_QUERY_DESCRIPTION {
    pub NodeName: LPCWSTR,
    pub ApplicationName: FABRIC_URI,
    pub ServiceManifestNameFilter: LPCWSTR,
    pub PartitionIdFilter: FABRIC_PARTITION_ID,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_REPLICA_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_REPLICA_QUERY_RESULT_ITEM {
    pub Kind: FABRIC_SERVICE_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_REPLICA_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_REPLICA_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_DEPLOYED_SERVICE_REPLICA_QUERY_RESULT_ITEM,
}
impl Default for FABRIC_DEPLOYED_SERVICE_REPLICA_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_TYPE_QUERY_DESCRIPTION {
    pub NodeName: LPCWSTR,
    pub ApplicationName: FABRIC_URI,
    pub ServiceManifestNameFilter: LPCWSTR,
    pub ServiceTypeNameFilter: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_TYPE_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_TYPE_QUERY_RESULT_ITEM {
    pub ServiceTypeName: LPCWSTR,
    pub CodePackageName: LPCWSTR,
    pub ServiceManifestName: LPCWSTR,
    pub Status: FABRIC_SERVICE_TYPE_REGISTRATION_STATUS,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_TYPE_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_TYPE_QUERY_RESULT_ITEM_EX1 {
    pub ServicePackageActivationId: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_SERVICE_TYPE_QUERY_RESULT_ITEM_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_SERVICE_TYPE_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_DEPLOYED_SERVICE_TYPE_QUERY_RESULT_ITEM,
}
impl Default for FABRIC_DEPLOYED_SERVICE_TYPE_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_STATEFUL_SERVICE_REPLICA_DETAIL_QUERY_RESULT_ITEM {
    pub ServiceName: FABRIC_URI,
    pub PartitionId: FABRIC_PARTITION_ID,
    pub ReplicaId: i64,
    pub CurrentServiceOperation: FABRIC_QUERY_SERVICE_OPERATION_NAME,
    pub CurrentServiceOperationStartTimeUtc: FILETIME,
    pub CurrentReplicatorOperation: FABRIC_QUERY_REPLICATOR_OPERATION_NAME,
    pub ReadStatus: FABRIC_SERVICE_PARTITION_ACCESS_STATUS,
    pub WriteStatus: FABRIC_SERVICE_PARTITION_ACCESS_STATUS,
    pub ReportedLoad: *mut FABRIC_LOAD_METRIC_REPORT_LIST,
    pub ReplicatorStatus: *mut FABRIC_REPLICATOR_STATUS_QUERY_RESULT,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_STATEFUL_SERVICE_REPLICA_DETAIL_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_STATEFUL_SERVICE_REPLICA_DETAIL_QUERY_RESULT_ITEM_EX1 {
    pub ReplicaStatus: *mut FABRIC_REPLICA_STATUS_QUERY_RESULT,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_STATEFUL_SERVICE_REPLICA_DETAIL_QUERY_RESULT_ITEM_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_STATEFUL_SERVICE_REPLICA_DETAIL_QUERY_RESULT_ITEM_EX2 {
    pub DeployedServiceReplica: *mut FABRIC_DEPLOYED_STATEFUL_SERVICE_REPLICA_QUERY_RESULT_ITEM,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_STATEFUL_SERVICE_REPLICA_DETAIL_QUERY_RESULT_ITEM_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_STATEFUL_SERVICE_REPLICA_QUERY_RESULT_ITEM {
    pub ServiceName: FABRIC_URI,
    pub ServiceTypeName: LPCWSTR,
    pub ServiceManifestVersion: LPCWSTR,
    pub CodePackageName: LPCWSTR,
    pub PartitionId: FABRIC_PARTITION_ID,
    pub ReplicaId: i64,
    pub ReplicaRole: FABRIC_REPLICA_ROLE,
    pub ReplicaStatus: FABRIC_QUERY_SERVICE_REPLICA_STATUS,
    pub Address: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_STATEFUL_SERVICE_REPLICA_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_STATEFUL_SERVICE_REPLICA_QUERY_RESULT_ITEM_EX1 {
    pub ServiceManifestName: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_STATEFUL_SERVICE_REPLICA_QUERY_RESULT_ITEM_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_STATEFUL_SERVICE_REPLICA_QUERY_RESULT_ITEM_EX2 {
    pub ServicePackageActivationId: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_STATEFUL_SERVICE_REPLICA_QUERY_RESULT_ITEM_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_STATEFUL_SERVICE_REPLICA_QUERY_RESULT_ITEM_EX3 {
    pub HostProcessId: i64,
    pub ReconfigurationInformation: *mut FABRIC_RECONFIGURATION_INFORMATION_QUERY_RESULT,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_STATEFUL_SERVICE_REPLICA_QUERY_RESULT_ITEM_EX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_STATELESS_SERVICE_INSTANCE_DETAIL_QUERY_RESULT_ITEM {
    pub ServiceName: FABRIC_URI,
    pub PartitionId: FABRIC_PARTITION_ID,
    pub InstanceId: i64,
    pub CurrentServiceOperation: FABRIC_QUERY_SERVICE_OPERATION_NAME,
    pub CurrentServiceOperationStartTimeUtc: FILETIME,
    pub ReportedLoad: *mut FABRIC_LOAD_METRIC_REPORT_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_STATELESS_SERVICE_INSTANCE_DETAIL_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_STATELESS_SERVICE_INSTANCE_DETAIL_QUERY_RESULT_ITEM_EX1 {
    pub DeployedServiceReplica: *mut FABRIC_DEPLOYED_STATELESS_SERVICE_INSTANCE_QUERY_RESULT_ITEM,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_STATELESS_SERVICE_INSTANCE_DETAIL_QUERY_RESULT_ITEM_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_STATELESS_SERVICE_INSTANCE_QUERY_RESULT_ITEM {
    pub ServiceName: FABRIC_URI,
    pub ServiceTypeName: LPCWSTR,
    pub ServiceManifestVersion: LPCWSTR,
    pub CodePackageName: LPCWSTR,
    pub PartitionId: FABRIC_PARTITION_ID,
    pub InstanceId: i64,
    pub ReplicaStatus: FABRIC_QUERY_SERVICE_REPLICA_STATUS,
    pub Address: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_STATELESS_SERVICE_INSTANCE_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_STATELESS_SERVICE_INSTANCE_QUERY_RESULT_ITEM_EX1 {
    pub ServiceManifestName: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_STATELESS_SERVICE_INSTANCE_QUERY_RESULT_ITEM_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_STATELESS_SERVICE_INSTANCE_QUERY_RESULT_ITEM_EX2 {
    pub ServicePackageActivationId: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_STATELESS_SERVICE_INSTANCE_QUERY_RESULT_ITEM_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DEPLOYED_STATELESS_SERVICE_INSTANCE_QUERY_RESULT_ITEM_EX3 {
    pub HostProcessId: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DEPLOYED_STATELESS_SERVICE_INSTANCE_QUERY_RESULT_ITEM_EX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_DEPLOYMENT_STATUS(pub i32);
impl FABRIC_DEPLOYMENT_STATUS {
    pub const FABRIC_DEPLOYMENT_STATUS_INVALID: Self = Self(0);
    pub const FABRIC_DEPLOYMENT_STATUS_DOWNLOADING: Self = Self(1);
    pub const FABRIC_DEPLOYMENT_STATUS_ACTIVATING: Self = Self(2);
    pub const FABRIC_DEPLOYMENT_STATUS_ACTIVE: Self = Self(3);
    pub const FABRIC_DEPLOYMENT_STATUS_UPGRADING: Self = Self(4);
    pub const FABRIC_DEPLOYMENT_STATUS_DEACTIVATING: Self = Self(5);
    pub const FABRIC_DEPLOYMENT_STATUS_RAN_TO_COMPLETION: Self = Self(6);
    pub const FABRIC_DEPLOYMENT_STATUS_FAILED: Self = Self(7);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_DIAGNOSTICS_SINKS_KIND(pub i32);
impl FABRIC_DIAGNOSTICS_SINKS_KIND {
    pub const FABRIC_DIAGNOSTICS_SINKS_KIND_INVALID: Self = Self(0);
    pub const FABRIC_DIAGNOSTICS_SINKS_KIND_AZUREINTERNAL: Self = Self(1);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DIGESTED_MANAGED_IDENTITY {
    pub Name: LPCWSTR,
    pub Identity: *const FABRIC_MANAGED_IDENTITY_DESCRIPTION,
    pub IdentityBindingPolicy: *const FABRIC_IDENTITY_BINDING_POLICY_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DIGESTED_MANAGED_IDENTITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DIGESTED_MANAGED_IDENTITY_DESCRIPTION {
    pub TokenServiceEndpoint: LPCWSTR,
    pub DefaultIdentity: *const FABRIC_MANAGED_IDENTITY_DESCRIPTION,
    pub WhitelistedIdentities: *const FABRIC_MANAGED_IDENTITY_DESCRIPTION_LIST,
    pub DigestedManagedIdentities: *const FABRIC_DIGESTED_MANAGED_IDENTITY_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DIGESTED_MANAGED_IDENTITY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DIGESTED_MANAGED_IDENTITY_ITEM {
    pub ApplicationIdentityName: LPCWSTR,
    pub DigestedManagedIdentity: *const FABRIC_DIGESTED_MANAGED_IDENTITY,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DIGESTED_MANAGED_IDENTITY_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DIGESTED_MANAGED_IDENTITY_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_DIGESTED_MANAGED_IDENTITY_ITEM,
}
impl Default for FABRIC_DIGESTED_MANAGED_IDENTITY_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DISABLE_SERVICE_DESCRIPTION {
    pub ServiceName: FABRIC_URI,
    pub DisableServiceFlag: FABRIC_SERVICE_DISABLE_FLAG,
    pub ForceDisable: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DISABLE_SERVICE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DLLHOST_ENTRY_POINT_DESCRIPTION {
    pub IsolationPolicyType: FABRIC_DLLHOST_ISOLATION_POLICY,
    pub HostedDlls: *mut FABRIC_DLLHOST_HOSTED_DLL_DESCRIPTION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DLLHOST_ENTRY_POINT_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DLLHOST_HOSTED_DLL_DESCRIPTION {
    pub Kind: FABRIC_DLLHOST_HOSTED_DLL_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_DLLHOST_HOSTED_DLL_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DLLHOST_HOSTED_DLL_DESCRIPTION_LIST {
    pub Count: u32,
    pub Items: *mut FABRIC_DLLHOST_HOSTED_DLL_DESCRIPTION,
}
impl Default for FABRIC_DLLHOST_HOSTED_DLL_DESCRIPTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_DLLHOST_HOSTED_DLL_KIND(pub i32);
impl FABRIC_DLLHOST_HOSTED_DLL_KIND {
    pub const FABRIC_DLLHOST_HOSTED_DLL_KIND_INVALID: Self = Self(0);
    pub const FABRIC_DLLHOST_HOSTED_DLL_KIND_UNMANAGED: Self = Self(1);
    pub const FABRIC_DLLHOST_HOSTED_DLL_KIND_MANAGED: Self = Self(2);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DLLHOST_HOSTED_MANAGED_DLL_DESCRIPTION {
    pub AssemblyName: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DLLHOST_HOSTED_MANAGED_DLL_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_DLLHOST_HOSTED_UNMANAGED_DLL_DESCRIPTION {
    pub DllName: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_DLLHOST_HOSTED_UNMANAGED_DLL_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_DLLHOST_ISOLATION_POLICY(pub i32);
impl FABRIC_DLLHOST_ISOLATION_POLICY {
    pub const FABRIC_DLLHOST_ISOLATION_POLICY_INVALID: Self = Self(0);
    pub const FABRIC_DLLHOST_ISOLATION_POLICY_SHARED_DOMAIN: Self = Self(1);
    pub const FABRIC_DLLHOST_ISOLATION_POLICY_DEDICATED_DOMAIN: Self = Self(2);
    pub const FABRIC_DLLHOST_ISOLATION_POLICY_DEDICATED_PROCESS: Self = Self(3);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_ENDPOINT_RESOURCE_DESCRIPTION {
    pub Name: LPCWSTR,
    pub Protocol: LPCWSTR,
    pub Type: LPCWSTR,
    pub Port: u32,
    pub CertificateName: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_ENDPOINT_RESOURCE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_ENDPOINT_RESOURCE_DESCRIPTION_EX1 {
    pub UriScheme: LPCWSTR,
    pub PathSuffix: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_ENDPOINT_RESOURCE_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_ENDPOINT_RESOURCE_DESCRIPTION_EX2 {
    pub CodePackageName: LPCWSTR,
    pub IpAddressOrFqdn: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_ENDPOINT_RESOURCE_DESCRIPTION_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_ENDPOINT_RESOURCE_DESCRIPTION_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_ENDPOINT_RESOURCE_DESCRIPTION,
}
impl Default for FABRIC_ENDPOINT_RESOURCE_DESCRIPTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_ENTITY_KIND_HEALTH_STATE_COUNT {
    pub EntityKind: FABRIC_HEALTH_ENTITY_KIND,
    pub HealthStateCount: *const FABRIC_HEALTH_STATE_COUNT,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_ENTITY_KIND_HEALTH_STATE_COUNT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_ENTRY_POINT_STATUS(pub i32);
impl FABRIC_ENTRY_POINT_STATUS {
    pub const FABRIC_ENTRY_POINT_STATUS_INVALID: Self = Self(0);
    pub const FABRIC_ENTRY_POINT_STATUS_PENDING: Self = Self(1);
    pub const FABRIC_ENTRY_POINT_STATUS_STARTING: Self = Self(2);
    pub const FABRIC_ENTRY_POINT_STATUS_STARTED: Self = Self(3);
    pub const FABRIC_ENTRY_POINT_STATUS_STOPPING: Self = Self(4);
    pub const FABRIC_ENTRY_POINT_STATUS_STOPPED: Self = Self(5);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_ENUMERATION_STATUS(pub i32);
impl FABRIC_ENUMERATION_STATUS {
    pub const FABRIC_ENUMERATION_INVALID: Self = Self(0);
    pub const FABRIC_ENUMERATION_BEST_EFFORT_MORE_DATA: Self = Self(1);
    pub const FABRIC_ENUMERATION_CONSISTENT_MORE_DATA: Self = Self(2);
    pub const FABRIC_ENUMERATION_BEST_EFFORT_FINISHED: Self = Self(4);
    pub const FABRIC_ENUMERATION_CONSISTENT_FINISHED: Self = Self(8);
    pub const FABRIC_ENUMERATION_VALID_MASK: Self = Self(15);
    pub const FABRIC_ENUMERATION_BEST_EFFORT_MASK: Self = Self(5);
    pub const FABRIC_ENUMERATION_CONSISTENT_MASK: Self = Self(10);
    pub const FABRIC_ENUMERATION_MORE_DATA_MASK: Self = Self(3);
    pub const FABRIC_ENUMERATION_FINISHED_MASK: Self = Self(12);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_EPOCH {
    pub DataLossNumber: i64,
    pub ConfigurationNumber: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_EPOCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_ERROR_CODE(pub i32);
impl FABRIC_ERROR_CODE {
    pub const FABRIC_E_FIRST_RESERVED_HRESULT: Self = Self(-2147017796);
    pub const FABRIC_E_LAST_RESERVED_HRESULT: Self = Self(-2147017397);
    pub const FABRIC_E_COMMUNICATION_ERROR: Self = Self(-2147017796);
    pub const FABRIC_E_INVALID_ADDRESS: Self = Self(-2147017795);
    pub const FABRIC_E_INVALID_NAME_URI: Self = Self(-2147017794);
    pub const FABRIC_E_INVALID_PARTITION_KEY: Self = Self(-2147017793);
    pub const FABRIC_E_NAME_ALREADY_EXISTS: Self = Self(-2147017792);
    pub const FABRIC_E_NAME_DOES_NOT_EXIST: Self = Self(-2147017791);
    pub const FABRIC_E_NAME_NOT_EMPTY: Self = Self(-2147017790);
    pub const FABRIC_E_NODE_NOT_FOUND: Self = Self(-2147017789);
    pub const FABRIC_E_NODE_IS_UP: Self = Self(-2147017788);
    pub const FABRIC_E_NO_WRITE_QUORUM: Self = Self(-2147017787);
    pub const FABRIC_E_NOT_PRIMARY: Self = Self(-2147017786);
    pub const FABRIC_E_NOT_READY: Self = Self(-2147017785);
    pub const FABRIC_E_OPERATION_NOT_COMPLETE: Self = Self(-2147017784);
    pub const FABRIC_E_PROPERTY_DOES_NOT_EXIST: Self = Self(-2147017783);
    pub const FABRIC_E_RECONFIGURATION_PENDING: Self = Self(-2147017782);
    pub const FABRIC_E_REPLICATION_QUEUE_FULL: Self = Self(-2147017781);
    pub const FABRIC_E_SERVICE_ALREADY_EXISTS: Self = Self(-2147017780);
    pub const FABRIC_E_SERVICE_DOES_NOT_EXIST: Self = Self(-2147017779);
    pub const FABRIC_E_SERVICE_OFFLINE: Self = Self(-2147017778);
    pub const FABRIC_E_SERVICE_METADATA_MISMATCH: Self = Self(-2147017777);
    pub const FABRIC_E_SERVICE_AFFINITY_CHAIN_NOT_SUPPORTED: Self = Self(-2147017776);
    pub const FABRIC_E_SERVICE_TYPE_ALREADY_REGISTERED: Self = Self(-2147017775);
    pub const FABRIC_E_SERVICE_TYPE_NOT_REGISTERED: Self = Self(-2147017774);
    pub const FABRIC_E_VALUE_TOO_LARGE: Self = Self(-2147017773);
    pub const FABRIC_E_VALUE_EMPTY: Self = Self(-2147017772);
    pub const FABRIC_E_PROPERTY_CHECK_FAILED: Self = Self(-2147017771);
    pub const FABRIC_E_WRITE_CONFLICT: Self = Self(-2147017770);
    pub const FABRIC_E_ENUMERATION_COMPLETED: Self = Self(-2147017769);
    pub const FABRIC_E_APPLICATION_TYPE_PROVISION_IN_PROGRESS: Self = Self(-2147017768);
    pub const FABRIC_E_APPLICATION_TYPE_ALREADY_EXISTS: Self = Self(-2147017767);
    pub const FABRIC_E_APPLICATION_TYPE_NOT_FOUND: Self = Self(-2147017766);
    pub const FABRIC_E_APPLICATION_TYPE_IN_USE: Self = Self(-2147017765);
    pub const FABRIC_E_APPLICATION_ALREADY_EXISTS: Self = Self(-2147017764);
    pub const FABRIC_E_APPLICATION_NOT_FOUND: Self = Self(-2147017763);
    pub const FABRIC_E_APPLICATION_UPGRADE_IN_PROGRESS: Self = Self(-2147017762);
    pub const FABRIC_E_APPLICATION_UPGRADE_VALIDATION_ERROR: Self = Self(-2147017761);
    pub const FABRIC_E_SERVICE_TYPE_NOT_FOUND: Self = Self(-2147017760);
    pub const FABRIC_E_SERVICE_TYPE_MISMATCH: Self = Self(-2147017759);
    pub const FABRIC_E_SERVICE_TYPE_TEMPLATE_NOT_FOUND: Self = Self(-2147017758);
    pub const FABRIC_E_CONFIGURATION_SECTION_NOT_FOUND: Self = Self(-2147017757);
    pub const FABRIC_E_CONFIGURATION_PARAMETER_NOT_FOUND: Self = Self(-2147017756);
    pub const FABRIC_E_INVALID_CONFIGURATION: Self = Self(-2147017755);
    pub const FABRIC_E_IMAGEBUILDER_VALIDATION_ERROR: Self = Self(-2147017754);
    pub const FABRIC_E_PARTITION_NOT_FOUND: Self = Self(-2147017753);
    pub const FABRIC_E_REPLICA_DOES_NOT_EXIST: Self = Self(-2147017752);
    pub const FABRIC_E_SERVICE_GROUP_ALREADY_EXISTS: Self = Self(-2147017751);
    pub const FABRIC_E_SERVICE_GROUP_DOES_NOT_EXIST: Self = Self(-2147017750);
    pub const FABRIC_E_PROCESS_DEACTIVATED: Self = Self(-2147017749);
    pub const FABRIC_E_PROCESS_ABORTED: Self = Self(-2147017748);
    pub const FABRIC_E_UPGRADE_FAILED: Self = Self(-2147017747);
    pub const FABRIC_E_INVALID_CREDENTIAL_TYPE: Self = Self(-2147017746);
    pub const FABRIC_E_INVALID_X509_FIND_TYPE: Self = Self(-2147017745);
    pub const FABRIC_E_INVALID_X509_STORE_LOCATION: Self = Self(-2147017744);
    pub const FABRIC_E_INVALID_X509_STORE_NAME: Self = Self(-2147017743);
    pub const FABRIC_E_INVALID_X509_THUMBPRINT: Self = Self(-2147017742);
    pub const FABRIC_E_INVALID_PROTECTION_LEVEL: Self = Self(-2147017741);
    pub const FABRIC_E_INVALID_X509_STORE: Self = Self(-2147017740);
    pub const FABRIC_E_INVALID_SUBJECT_NAME: Self = Self(-2147017739);
    pub const FABRIC_E_INVALID_ALLOWED_COMMON_NAME_LIST: Self = Self(-2147017738);
    pub const FABRIC_E_INVALID_CREDENTIALS: Self = Self(-2147017737);
    pub const FABRIC_E_DECRYPTION_FAILED: Self = Self(-2147017736);
    pub const FABRIC_E_CONFIGURATION_PACKAGE_NOT_FOUND: Self = Self(-2147017735);
    pub const FABRIC_E_DATA_PACKAGE_NOT_FOUND: Self = Self(-2147017734);
    pub const FABRIC_E_CODE_PACKAGE_NOT_FOUND: Self = Self(-2147017733);
    pub const FABRIC_E_SERVICE_ENDPOINT_RESOURCE_NOT_FOUND: Self = Self(-2147017732);
    pub const FABRIC_E_INVALID_OPERATION: Self = Self(-2147017731);
    pub const FABRIC_E_OBJECT_CLOSED: Self = Self(-2147017730);
    pub const FABRIC_E_TIMEOUT: Self = Self(-2147017729);
    pub const FABRIC_E_FILE_NOT_FOUND: Self = Self(-2147017728);
    pub const FABRIC_E_DIRECTORY_NOT_FOUND: Self = Self(-2147017727);
    pub const FABRIC_E_INVALID_DIRECTORY: Self = Self(-2147017726);
    pub const FABRIC_E_PATH_TOO_LONG: Self = Self(-2147017725);
    pub const FABRIC_E_IMAGESTORE_IOERROR: Self = Self(-2147017724);
    pub const FABRIC_E_CORRUPTED_IMAGE_STORE_OBJECT_FOUND: Self = Self(-2147017723);
    pub const FABRIC_E_APPLICATION_NOT_UPGRADING: Self = Self(-2147017722);
    pub const FABRIC_E_APPLICATION_ALREADY_IN_TARGET_VERSION: Self = Self(-2147017721);
    pub const FABRIC_E_IMAGEBUILDER_UNEXPECTED_ERROR: Self = Self(-2147017720);
    pub const FABRIC_E_FABRIC_VERSION_NOT_FOUND: Self = Self(-2147017719);
    pub const FABRIC_E_FABRIC_VERSION_IN_USE: Self = Self(-2147017718);
    pub const FABRIC_E_FABRIC_VERSION_ALREADY_EXISTS: Self = Self(-2147017717);
    pub const FABRIC_E_FABRIC_ALREADY_IN_TARGET_VERSION: Self = Self(-2147017716);
    pub const FABRIC_E_FABRIC_NOT_UPGRADING: Self = Self(-2147017715);
    pub const FABRIC_E_FABRIC_UPGRADE_IN_PROGRESS: Self = Self(-2147017714);
    pub const FABRIC_E_FABRIC_UPGRADE_VALIDATION_ERROR: Self = Self(-2147017713);
    pub const FABRIC_E_HEALTH_MAX_REPORTS_REACHED: Self = Self(-2147017712);
    pub const FABRIC_E_HEALTH_STALE_REPORT: Self = Self(-2147017711);
    pub const FABRIC_E_KEY_TOO_LARGE: Self = Self(-2147017710);
    pub const FABRIC_E_KEY_NOT_FOUND: Self = Self(-2147017709);
    pub const FABRIC_E_SEQUENCE_NUMBER_CHECK_FAILED: Self = Self(-2147017708);
    pub const FABRIC_E_ENCRYPTION_FAILED: Self = Self(-2147017707);
    pub const FABRIC_E_INVALID_ATOMIC_GROUP: Self = Self(-2147017706);
    pub const FABRIC_E_HEALTH_ENTITY_NOT_FOUND: Self = Self(-2147017705);
    pub const FABRIC_E_SERVICE_MANIFEST_NOT_FOUND: Self = Self(-2147017704);
    pub const FABRIC_E_RELIABLE_SESSION_TRANSPORT_STARTUP_FAILURE: Self = Self(-2147017703);
    pub const FABRIC_E_RELIABLE_SESSION_ALREADY_EXISTS: Self = Self(-2147017702);
    pub const FABRIC_E_RELIABLE_SESSION_CANNOT_CONNECT: Self = Self(-2147017701);
    pub const FABRIC_E_RELIABLE_SESSION_MANAGER_EXISTS: Self = Self(-2147017700);
    pub const FABRIC_E_RELIABLE_SESSION_REJECTED: Self = Self(-2147017699);
    pub const FABRIC_E_RELIABLE_SESSION_MANAGER_ALREADY_LISTENING: Self = Self(-2147017698);
    pub const FABRIC_E_RELIABLE_SESSION_MANAGER_NOT_FOUND: Self = Self(-2147017697);
    pub const FABRIC_E_RELIABLE_SESSION_MANAGER_NOT_LISTENING: Self = Self(-2147017696);
    pub const FABRIC_E_INVALID_SERVICE_TYPE: Self = Self(-2147017695);
    pub const FABRIC_E_IMAGEBUILDER_TIMEOUT: Self = Self(-2147017694);
    pub const FABRIC_E_IMAGEBUILDER_ACCESS_DENIED: Self = Self(-2147017693);
    pub const FABRIC_E_IMAGEBUILDER_INVALID_MSI_FILE: Self = Self(-2147017692);
    pub const FABRIC_E_SERVICE_TOO_BUSY: Self = Self(-2147017691);
    pub const FABRIC_E_TRANSACTION_NOT_ACTIVE: Self = Self(-2147017690);
    pub const FABRIC_E_REPAIR_TASK_ALREADY_EXISTS: Self = Self(-2147017689);
    pub const FABRIC_E_REPAIR_TASK_NOT_FOUND: Self = Self(-2147017688);
    pub const FABRIC_E_RELIABLE_SESSION_NOT_FOUND: Self = Self(-2147017687);
    pub const FABRIC_E_RELIABLE_SESSION_QUEUE_EMPTY: Self = Self(-2147017686);
    pub const FABRIC_E_RELIABLE_SESSION_QUOTA_EXCEEDED: Self = Self(-2147017685);
    pub const FABRIC_E_RELIABLE_SESSION_SERVICE_FAULTED: Self = Self(-2147017684);
    pub const FABRIC_E_RELIABLE_SESSION_INVALID_TARGET_PARTITION: Self = Self(-2147017683);
    pub const FABRIC_E_TRANSACTION_TOO_LARGE: Self = Self(-2147017682);
    pub const FABRIC_E_REPLICATION_OPERATION_TOO_LARGE: Self = Self(-2147017681);
    pub const FABRIC_E_INSTANCE_ID_MISMATCH: Self = Self(-2147017680);
    pub const FABRIC_E_UPGRADE_DOMAIN_ALREADY_COMPLETED: Self = Self(-2147017679);
    pub const FABRIC_E_NODE_HAS_NOT_STOPPED_YET: Self = Self(-2147017678);
    pub const FABRIC_E_INSUFFICIENT_CLUSTER_CAPACITY: Self = Self(-2147017677);
    pub const FABRIC_E_INVALID_PACKAGE_SHARING_POLICY: Self = Self(-2147017676);
    pub const FABRIC_E_PREDEPLOYMENT_NOT_ALLOWED: Self = Self(-2147017675);
    pub const FABRIC_E_INVALID_BACKUP_SETTING: Self = Self(-2147017674);
    pub const FABRIC_E_MISSING_FULL_BACKUP: Self = Self(-2147017673);
    pub const FABRIC_E_BACKUP_IN_PROGRESS: Self = Self(-2147017672);
    pub const FABRIC_E_DUPLICATE_SERVICE_NOTIFICATION_FILTER_NAME: Self = Self(-2147017671);
    pub const FABRIC_E_INVALID_REPLICA_OPERATION: Self = Self(-2147017670);
    pub const FABRIC_E_INVALID_REPLICA_STATE: Self = Self(-2147017669);
    pub const FABRIC_E_LOADBALANCER_NOT_READY: Self = Self(-2147017668);
    pub const FABRIC_E_INVALID_PARTITION_OPERATION: Self = Self(-2147017667);
    pub const FABRIC_E_PRIMARY_ALREADY_EXISTS: Self = Self(-2147017666);
    pub const FABRIC_E_SECONDARY_ALREADY_EXISTS: Self = Self(-2147017665);
    pub const FABRIC_E_BACKUP_DIRECTORY_NOT_EMPTY: Self = Self(-2147017664);
    pub const FABRIC_E_FORCE_NOT_SUPPORTED_FOR_REPLICA_OPERATION: Self = Self(-2147017663);
    pub const FABRIC_E_ACQUIRE_FILE_LOCK_FAILED: Self = Self(-2147017662);
    pub const FABRIC_E_CONNECTION_DENIED: Self = Self(-2147017661);
    pub const FABRIC_E_SERVER_AUTHENTICATION_FAILED: Self = Self(-2147017660);
    pub const FABRIC_E_CONSTRAINT_KEY_UNDEFINED: Self = Self(-2147017659);
    pub const FABRIC_E_MULTITHREADED_TRANSACTIONS_NOT_ALLOWED: Self = Self(-2147017658);
    pub const FABRIC_E_INVALID_X509_NAME_LIST: Self = Self(-2147017657);
    pub const FABRIC_E_VERBOSE_FM_PLACEMENT_HEALTH_REPORTING_REQUIRED: Self = Self(-2147017656);
    pub const FABRIC_E_GATEWAY_NOT_REACHABLE: Self = Self(-2147017655);
    pub const FABRIC_E_USER_ROLE_CLIENT_CERTIFICATE_NOT_CONFIGURED: Self = Self(-2147017654);
    pub const FABRIC_E_TRANSACTION_ABORTED: Self = Self(-2147017653);
    pub const FABRIC_E_CANNOT_CONNECT: Self = Self(-2147017652);
    pub const FABRIC_E_MESSAGE_TOO_LARGE: Self = Self(-2147017651);
    pub const FABRIC_E_CONSTRAINT_NOT_SATISFIED: Self = Self(-2147017650);
    pub const FABRIC_E_ENDPOINT_NOT_FOUND: Self = Self(-2147017649);
    pub const FABRIC_E_APPLICATION_UPDATE_IN_PROGRESS: Self = Self(-2147017648);
    pub const FABRIC_E_DELETE_BACKUP_FILE_FAILED: Self = Self(-2147017647);
    pub const FABRIC_E_CONNECTION_CLOSED_BY_REMOTE_END: Self = Self(-2147017646);
    pub const FABRIC_E_INVALID_TEST_COMMAND_STATE: Self = Self(-2147017645);
    pub const FABRIC_E_TEST_COMMAND_OPERATION_ID_ALREADY_EXISTS: Self = Self(-2147017644);
    pub const FABRIC_E_CM_OPERATION_FAILED: Self = Self(-2147017643);
    pub const FABRIC_E_IMAGEBUILDER_RESERVED_DIRECTORY_ERROR: Self = Self(-2147017642);
    pub const FABRIC_E_CERTIFICATE_NOT_FOUND: Self = Self(-2147017641);
    pub const FABRIC_E_CHAOS_ALREADY_RUNNING: Self = Self(-2147017640);
    pub const FABRIC_E_FABRIC_DATA_ROOT_NOT_FOUND: Self = Self(-2147017639);
    pub const FABRIC_E_INVALID_RESTORE_DATA: Self = Self(-2147017638);
    pub const FABRIC_E_DUPLICATE_BACKUPS: Self = Self(-2147017637);
    pub const FABRIC_E_INVALID_BACKUP_CHAIN: Self = Self(-2147017636);
    pub const FABRIC_E_STOP_IN_PROGRESS: Self = Self(-2147017635);
    pub const FABRIC_E_ALREADY_STOPPED: Self = Self(-2147017634);
    pub const FABRIC_E_NODE_IS_DOWN: Self = Self(-2147017633);
    pub const FABRIC_E_NODE_TRANSITION_IN_PROGRESS: Self = Self(-2147017632);
    pub const FABRIC_E_INVALID_BACKUP: Self = Self(-2147017631);
    pub const FABRIC_E_INVALID_INSTANCE_ID: Self = Self(-2147017630);
    pub const FABRIC_E_INVALID_DURATION: Self = Self(-2147017629);
    pub const FABRIC_E_RESTORE_SAFE_CHECK_FAILED: Self = Self(-2147017628);
    pub const FABRIC_E_CONFIG_UPGRADE_FAILED: Self = Self(-2147017627);
    pub const FABRIC_E_UPLOAD_SESSION_RANGE_NOT_SATISFIABLE: Self = Self(-2147017626);
    pub const FABRIC_E_UPLOAD_SESSION_ID_CONFLICT: Self = Self(-2147017625);
    pub const FABRIC_E_INVALID_PARTITION_SELECTOR: Self = Self(-2147017624);
    pub const FABRIC_E_INVALID_REPLICA_SELECTOR: Self = Self(-2147017623);
    pub const FABRIC_E_DNS_SERVICE_NOT_FOUND: Self = Self(-2147017622);
    pub const FABRIC_E_INVALID_DNS_NAME: Self = Self(-2147017621);
    pub const FABRIC_E_DNS_NAME_IN_USE: Self = Self(-2147017620);
    pub const FABRIC_E_COMPOSE_DEPLOYMENT_ALREADY_EXISTS: Self = Self(-2147017619);
    pub const FABRIC_E_COMPOSE_DEPLOYMENT_NOT_FOUND: Self = Self(-2147017618);
    pub const FABRIC_E_INVALID_FOR_STATEFUL_SERVICES: Self = Self(-2147017617);
    pub const FABRIC_E_INVALID_FOR_STATELESS_SERVICES: Self = Self(-2147017616);
    pub const FABRIC_E_ONLY_VALID_FOR_STATEFUL_PERSISTENT_SERVICES: Self = Self(-2147017615);
    pub const FABRIC_E_INVALID_UPLOAD_SESSION_ID: Self = Self(-2147017614);
    pub const FABRIC_E_BACKUP_NOT_ENABLED: Self = Self(-2147017613);
    pub const FABRIC_E_BACKUP_IS_ENABLED: Self = Self(-2147017612);
    pub const FABRIC_E_BACKUP_POLICY_DOES_NOT_EXIST: Self = Self(-2147017611);
    pub const FABRIC_E_BACKUP_POLICY_ALREADY_EXISTS: Self = Self(-2147017610);
    pub const FABRIC_E_RESTORE_IN_PROGRESS: Self = Self(-2147017609);
    pub const FABRIC_E_RESTORE_SOURCE_TARGET_PARTITION_MISMATCH: Self = Self(-2147017608);
    pub const FABRIC_E_FAULT_ANALYSIS_SERVICE_NOT_ENABLED: Self = Self(-2147017607);
    pub const FABRIC_E_CONTAINER_NOT_FOUND: Self = Self(-2147017606);
    pub const FABRIC_E_OBJECT_DISPOSED: Self = Self(-2147017605);
    pub const FABRIC_E_NOT_READABLE: Self = Self(-2147017604);
    pub const FABRIC_E_BACKUPCOPIER_UNEXPECTED_ERROR: Self = Self(-2147017603);
    pub const FABRIC_E_BACKUPCOPIER_TIMEOUT: Self = Self(-2147017602);
    pub const FABRIC_E_BACKUPCOPIER_ACCESS_DENIED: Self = Self(-2147017601);
    pub const FABRIC_E_INVALID_SERVICE_SCALING_POLICY: Self = Self(-2147017600);
    pub const FABRIC_E_SINGLE_INSTANCE_APPLICATION_ALREADY_EXISTS: Self = Self(-2147017599);
    pub const FABRIC_E_SINGLE_INSTANCE_APPLICATION_NOT_FOUND: Self = Self(-2147017598);
    pub const FABRIC_E_VOLUME_ALREADY_EXISTS: Self = Self(-2147017597);
    pub const FABRIC_E_VOLUME_NOT_FOUND: Self = Self(-2147017596);
    pub const FABRIC_E_DATABASE_MIGRATION_IN_PROGRESS: Self = Self(-2147017595);
    pub const FABRIC_E_CENTRAL_SECRET_SERVICE_GENERIC: Self = Self(-2147017594);
    pub const FABRIC_E_SECRET_INVALID: Self = Self(-2147017593);
    pub const FABRIC_E_SECRET_VERSION_ALREADY_EXISTS: Self = Self(-2147017592);
    pub const FABRIC_E_SINGLE_INSTANCE_APPLICATION_UPGRADE_IN_PROGRESS: Self = Self(-2147017591);
    pub const FABRIC_E_OPERATION_NOT_SUPPORTED: Self = Self(-2147017590);
    pub const FABRIC_E_COMPOSE_DEPLOYMENT_NOT_UPGRADING: Self = Self(-2147017589);
    pub const FABRIC_E_SECRET_TYPE_CANNOT_BE_CHANGED: Self = Self(-2147017588);
    pub const FABRIC_E_NETWORK_NOT_FOUND: Self = Self(-2147017587);
    pub const FABRIC_E_NETWORK_IN_USE: Self = Self(-2147017586);
    pub const FABRIC_E_ENDPOINT_NOT_REFERENCED: Self = Self(-2147017585);
    pub const FABRIC_E_LAST_USED_HRESULT: Self = Self(-2147017585);
    pub const FABRIC_E_FACILITY_SF_FIRST_HRESULT: Self = Self(-2018508800);
    pub const FABRIC_E_INSTANCE_ALREADY_EXISTS: Self = Self(-2018508800);
    pub const FABRIC_E_NODE_TYPE_NOT_FOUND: Self = Self(-2018508799);
    pub const FABRIC_E_INSTANCE_COUNT_UPDATE_NOT_ALLOWED: Self = Self(-2018508798);
    pub const FABRIC_E_COPY_ABORTED: Self = Self(-2018508797);
    pub const FABRIC_E_AUXILIARY_ALREADY_EXISTS: Self = Self(-2018508796);
    pub const FABRIC_E_AUXILIARY_FEATURE_DISABLED: Self = Self(-2018508795);
    pub const FABRIC_E_RUN_TO_COMPLETION_INCOMPATIBLE_WITH_SHARED_PROCESS: Self = Self(-2018508794);
    pub const FABRIC_E_VERSION_STORE_OUT_OF_MEMORY: Self = Self(-2018508793);
    pub const FABRIC_E_BACKUP_NOT_FOUND: Self = Self(-2018508792);
    pub const FABRIC_E_SKIP_RESTORE_OPERATION: Self = Self(-2018508791);
    pub const FABRIC_E_STORE_OUT_OF_SESSIONS: Self = Self(-2018508790);
    pub const FABRIC_E_RESTORE_WAITING_FOR_USER_INTERVENTION: Self = Self(-2018508789);
    pub const FABRIC_E_DATABASE_FILES_CORRUPTED: Self = Self(-2018508788);
    pub const FABRIC_E_INSUFFICIENT_MAX_LOAD_CAPACITY: Self = Self(-2018508787);
    pub const FABRIC_E_STORE_DISK_ERROR: Self = Self(-2018508786);
    pub const FABRIC_E_SERVICE_ALREADY_IN_REQUESTED_STATE: Self = Self(-2018508785);
    pub const FABRIC_E_DISABLE_ENABLE_SERVICE_FEATURE_DISABLED: Self = Self(-2018508784);
    pub const FABRIC_E_MAX_ALLOWED_DISABLED_SERVICES_REACHED: Self = Self(-2018508783);
    pub const FABRIC_E_SERVICE_DISABLED: Self = Self(-2018508782);
    pub const FABRIC_E_SERVICE_DISABLE_IN_PROGRESS: Self = Self(-2018508781);
    pub const FABRIC_E_STORE_OUT_OF_LONG_VALUE_IDS: Self = Self(-2018508780);
    pub const FABRIC_E_STORE_OUT_OF_INSTANCES: Self = Self(-2018508779);
    pub const FABRIC_E_STORE_SERIALIZED_STREAM_NULL: Self = Self(-2018508778);
    pub const FABRIC_E_STORE_SERIALIZATION_ERROR: Self = Self(-2018508777);
    pub const FABRIC_E_INCOMPATIBLE_EXCLUSIVE_SELF_RECONFIGURING: Self = Self(-2018508776);
    pub const FABRIC_E_LAST_USED_FACILITY_SF_HRESULT: Self = Self(-2018508776);
    pub const FABRIC_E_FACILITY_SF_LAST_HRESULT: Self = Self(-2018443265);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_ESE_LOCAL_STORE_SETTINGS {
    pub DbFolderPath: LPCWSTR,
    pub LogFileSizeInKB: i32,
    pub LogBufferSizeInKB: i32,
    pub MaxCursors: i32,
    pub MaxVerPages: i32,
    pub MaxAsyncCommitDelayInMilliseconds: i32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_ESE_LOCAL_STORE_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_ESE_LOCAL_STORE_SETTINGS_EX1 {
    pub EnableIncrementalBackup: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_ESE_LOCAL_STORE_SETTINGS_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_ESE_LOCAL_STORE_SETTINGS_EX2 {
    pub MaxCacheSizeInMB: i32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_ESE_LOCAL_STORE_SETTINGS_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_ESE_LOCAL_STORE_SETTINGS_EX3 {
    pub MaxDefragFrequencyInMinutes: i32,
    pub DefragThresholdInMB: i32,
    pub DatabasePageSizeInKB: i32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_ESE_LOCAL_STORE_SETTINGS_EX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_ESE_LOCAL_STORE_SETTINGS_EX4 {
    pub CompactionThresholdInMB: i32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_ESE_LOCAL_STORE_SETTINGS_EX4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_ESE_LOCAL_STORE_SETTINGS_EX5 {
    pub IntrinsicValueThresholdInBytes: i32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_ESE_LOCAL_STORE_SETTINGS_EX5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_ESE_LOCAL_STORE_SETTINGS_EX6 {
    pub EnableOverwriteOnUpdate: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_ESE_LOCAL_STORE_SETTINGS_EX6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_ESE_LOCAL_STORE_SETTINGS_EX7 {
    pub PoolMinSize: i32,
    pub PoolAdjustmentSize: i32,
    pub PoolCachedReadCursorsPerSessionSize: i32,
    pub PoolEvictionPeriodInSeconds: i32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_ESE_LOCAL_STORE_SETTINGS_EX7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_ESE_LOCAL_STORE_SETTINGS_EX8 {
    pub FreePageSizeThresholdInMB: i32,
    pub CompactionProbabilityInPercent: i32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_ESE_LOCAL_STORE_SETTINGS_EX8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_ESE_LOCAL_STORE_SETTINGS_EX9 {
    pub FreePageSizeThresholdInPercent: i32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_ESE_LOCAL_STORE_SETTINGS_EX9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_EVENT_CONTEXT_MAP {
    pub Count: u32,
    pub Items: *mut FABRIC_EVENT_CONTEXT_MAP_ITEM,
}
impl Default for FABRIC_EVENT_CONTEXT_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_EVENT_CONTEXT_MAP_ITEM {
    pub Key: LPCWSTR,
    pub Value: LPCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_EVENT_HEALTH_EVALUATION {
    pub Description: LPCWSTR,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub UnhealthyEvent: *const FABRIC_HEALTH_EVENT,
    pub ConsiderWarningAsError: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_EVENT_HEALTH_EVALUATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_EXECUTING_FAULTS_EVENT {
    pub TimeStampUtc: FILETIME,
    pub Faults: *const FABRIC_STRING_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_EXECUTING_FAULTS_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_EXECUTION_POLICY_DESCRIPTION {
    pub ExecutionType: FABRIC_EXECUTION_POLICY_EXECUTION_TYPE,
    pub RestartPolicy: FABRIC_EXECUTION_POLICY_RESTART_POLICY,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_EXECUTION_POLICY_EXECUTION_TYPE(pub i32);
impl FABRIC_EXECUTION_POLICY_EXECUTION_TYPE {
    pub const FABRIC_EXECUTION_POLICY_EXECUTION_TYPE_RUN_ALWAYS: Self = Self(0);
    pub const FABRIC_EXECUTION_POLICY_EXECUTION_TYPE_RUN_TO_COMPLETION: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_EXECUTION_POLICY_RESTART_POLICY(pub i32);
impl FABRIC_EXECUTION_POLICY_RESTART_POLICY {
    pub const FABRIC_EXECUTION_POLICY_RESTART_POLICY_ALWAYS: Self = Self(0);
    pub const FABRIC_EXECUTION_POLICY_RESTART_POLICY_ON_FAILURE: Self = Self(1);
    pub const FABRIC_EXECUTION_POLICY_RESTART_POLICY_NEVER: Self = Self(2);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_EXEHOST_ENTRY_POINT_DESCRIPTION {
    pub Program: LPCWSTR,
    pub Arguments: LPCWSTR,
    pub WorkingFolder: FABRIC_EXEHOST_WORKING_FOLDER,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_EXEHOST_ENTRY_POINT_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_EXEHOST_ENTRY_POINT_DESCRIPTION_EX1 {
    pub PeriodicIntervalInSeconds: u32,
    pub ConsoleRedirectionEnabled: bool,
    pub ConsoleRedirectionFileRetentionCount: u32,
    pub ConsoleRedirectionFileMaxSizeInKb: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_EXEHOST_ENTRY_POINT_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_EXEHOST_ENTRY_POINT_DESCRIPTION_EX2 {
    pub IsExternalExecutable: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_EXEHOST_ENTRY_POINT_DESCRIPTION_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_EXEHOST_WORKING_FOLDER(pub i32);
impl FABRIC_EXEHOST_WORKING_FOLDER {
    pub const FABRIC_EXEHOST_WORKING_FOLDER_INVALID: Self = Self(0);
    pub const FABRIC_EXEHOST_WORKING_FOLDER_WORK: Self = Self(1);
    pub const FABRIC_EXEHOST_WORKING_FOLDER_CODE_PACKAGE: Self = Self(2);
    pub const FABRIC_EXEHOST_WORKING_FOLDER_CODE_BASE: Self = Self(3);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_EXTERNAL_STORE_PROVISION_APPLICATION_TYPE_DESCRIPTION {
    pub ApplicationTypeName: LPCWSTR,
    pub ApplicationTypeVersion: LPCWSTR,
    pub ApplicationPackageDownloadUri: LPCWSTR,
    pub Async: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_EXTERNAL_STORE_PROVISION_APPLICATION_TYPE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_FAULT_TYPE(pub i32);
impl FABRIC_FAULT_TYPE {
    pub const FABRIC_FAULT_TYPE_INVALID: Self = Self(0);
    pub const FABRIC_FAULT_TYPE_PERMANENT: Self = Self(1);
    pub const FABRIC_FAULT_TYPE_TRANSIENT: Self = Self(2);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_GATEWAY_INFORMATION {
    pub NodeAddress: LPCWSTR,
    pub NodeId: FABRIC_NODE_ID,
    pub NodeInstanceId: u64,
    pub NodeName: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_GATEWAY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_GET_CHAOS_REPORT_DESCRIPTION {
    pub Filter: *mut FABRIC_CHAOS_REPORT_FILTER,
    pub ContinuationToken: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_GET_CHAOS_REPORT_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_GET_PROPERTY_OPERATION {
    pub PropertyName: LPCWSTR,
    pub IncludeValue: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_GET_PROPERTY_OPERATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_HEALTH_ENTITY_KIND(pub i32);
impl FABRIC_HEALTH_ENTITY_KIND {
    pub const FABRIC_HEALTH_ENTITY_KIND_INVALID: Self = Self(0);
    pub const FABRIC_HEALTH_ENTITY_KIND_NODE: Self = Self(1);
    pub const FABRIC_HEALTH_ENTITY_KIND_PARTITION: Self = Self(2);
    pub const FABRIC_HEALTH_ENTITY_KIND_SERVICE: Self = Self(3);
    pub const FABRIC_HEALTH_ENTITY_KIND_APPLICATION: Self = Self(4);
    pub const FABRIC_HEALTH_ENTITY_KIND_REPLICA: Self = Self(5);
    pub const FABRIC_HEALTH_ENTITY_KIND_DEPLOYED_APPLICATION: Self = Self(6);
    pub const FABRIC_HEALTH_ENTITY_KIND_DEPLOYED_SERVICE_PACKAGE: Self = Self(7);
    pub const FABRIC_HEALTH_ENTITY_KIND_CLUSTER: Self = Self(8);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_HEALTH_EVALUATION {
    pub Kind: FABRIC_HEALTH_EVALUATION_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_HEALTH_EVALUATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_HEALTH_EVALUATION_KIND(pub i32);
impl FABRIC_HEALTH_EVALUATION_KIND {
    pub const FABRIC_HEALTH_EVALUATION_KIND_INVALID: Self = Self(0);
    pub const FABRIC_HEALTH_EVALUATION_KIND_EVENT: Self = Self(1);
    pub const FABRIC_HEALTH_EVALUATION_KIND_REPLICAS: Self = Self(2);
    pub const FABRIC_HEALTH_EVALUATION_KIND_PARTITIONS: Self = Self(3);
    pub const FABRIC_HEALTH_EVALUATION_KIND_DEPLOYED_SERVICE_PACKAGES: Self = Self(4);
    pub const FABRIC_HEALTH_EVALUATION_KIND_DEPLOYED_APPLICATIONS: Self = Self(5);
    pub const FABRIC_HEALTH_EVALUATION_KIND_SERVICES: Self = Self(6);
    pub const FABRIC_HEALTH_EVALUATION_KIND_NODES: Self = Self(7);
    pub const FABRIC_HEALTH_EVALUATION_KIND_APPLICATIONS: Self = Self(8);
    pub const FABRIC_HEALTH_EVALUATION_KIND_SYSTEM_APPLICATION: Self = Self(9);
    pub const FABRIC_HEALTH_EVALUATION_KIND_UPGRADE_DOMAIN_DEPLOYED_APPLICATIONS: Self = Self(10);
    pub const FABRIC_HEALTH_EVALUATION_KIND_UPGRADE_DOMAIN_NODES: Self = Self(11);
    pub const FABRIC_HEALTH_EVALUATION_KIND_NODE: Self = Self(12);
    pub const FABRIC_HEALTH_EVALUATION_KIND_REPLICA: Self = Self(13);
    pub const FABRIC_HEALTH_EVALUATION_KIND_PARTITION: Self = Self(14);
    pub const FABRIC_HEALTH_EVALUATION_KIND_SERVICE: Self = Self(15);
    pub const FABRIC_HEALTH_EVALUATION_KIND_DEPLOYED_SERVICE_PACKAGE: Self = Self(16);
    pub const FABRIC_HEALTH_EVALUATION_KIND_DEPLOYED_APPLICATION: Self = Self(17);
    pub const FABRIC_HEALTH_EVALUATION_KIND_APPLICATION: Self = Self(18);
    pub const FABRIC_HEALTH_EVALUATION_KIND_DELTA_NODES_CHECK: Self = Self(19);
    pub const FABRIC_HEALTH_EVALUATION_KIND_UPGRADE_DOMAIN_DELTA_NODES_CHECK: Self = Self(20);
    pub const FABRIC_HEALTH_EVALUATION_KIND_APPLICATION_TYPE_APPLICATIONS: Self = Self(21);
    pub const FABRIC_HEALTH_EVALUATION_KIND_NODE_TYPE_NODES: Self = Self(22);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_HEALTH_EVALUATION_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_HEALTH_EVALUATION,
}
impl Default for FABRIC_HEALTH_EVALUATION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_HEALTH_EVENT {
    pub HealthInformation: *const FABRIC_HEALTH_INFORMATION,
    pub SourceUtcTimestamp: FILETIME,
    pub LastModifiedUtcTimestamp: FILETIME,
    pub IsExpired: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_HEALTH_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_HEALTH_EVENTS_FILTER {
    pub HealthStateFilter: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_HEALTH_EVENTS_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_HEALTH_EVENT_EX1 {
    pub LastOkTransitionAt: FILETIME,
    pub LastWarningTransitionAt: FILETIME,
    pub LastErrorTransitionAt: FILETIME,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_HEALTH_EVENT_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_HEALTH_EVENT_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_HEALTH_EVENT,
}
impl Default for FABRIC_HEALTH_EVENT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_HEALTH_INFORMATION {
    pub SourceId: LPCWSTR,
    pub Property: LPCWSTR,
    pub TimeToLiveSeconds: u32,
    pub State: FABRIC_HEALTH_STATE,
    pub Description: LPCWSTR,
    pub SequenceNumber: i64,
    pub RemoveWhenExpired: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_HEALTH_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_HEALTH_INFORMATION_EX1 {
    pub HealthReportId: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_HEALTH_INFORMATION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_HEALTH_REPORT {
    pub Kind: FABRIC_HEALTH_REPORT_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_HEALTH_REPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FABRIC_HEALTH_REPORT_INFINITE_TTL: u32 = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_HEALTH_REPORT_KIND(pub i32);
impl FABRIC_HEALTH_REPORT_KIND {
    pub const FABRIC_HEALTH_REPORT_KIND_INVALID: Self = Self(0);
    pub const FABRIC_HEALTH_REPORT_KIND_STATEFUL_SERVICE_REPLICA: Self = Self(1);
    pub const FABRIC_HEALTH_REPORT_KIND_STATELESS_SERVICE_INSTANCE: Self = Self(2);
    pub const FABRIC_HEALTH_REPORT_KIND_PARTITION: Self = Self(3);
    pub const FABRIC_HEALTH_REPORT_KIND_NODE: Self = Self(4);
    pub const FABRIC_HEALTH_REPORT_KIND_SERVICE: Self = Self(5);
    pub const FABRIC_HEALTH_REPORT_KIND_APPLICATION: Self = Self(6);
    pub const FABRIC_HEALTH_REPORT_KIND_DEPLOYED_APPLICATION: Self = Self(7);
    pub const FABRIC_HEALTH_REPORT_KIND_DEPLOYED_SERVICE_PACKAGE: Self = Self(8);
    pub const FABRIC_HEALTH_REPORT_KIND_CLUSTER: Self = Self(9);
    pub const FABRIC_HEALTH_REPORT_KIND_SELF_RECONFIGURING_SERVICE_INSTANCE: Self = Self(10);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_HEALTH_REPORT_SEND_OPTIONS {
    pub Immediate: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_HEALTH_REPORT_SEND_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_HEALTH_STATE(pub i32);
impl FABRIC_HEALTH_STATE {
    pub const FABRIC_HEALTH_STATE_INVALID: Self = Self(0);
    pub const FABRIC_HEALTH_STATE_OK: Self = Self(1);
    pub const FABRIC_HEALTH_STATE_WARNING: Self = Self(2);
    pub const FABRIC_HEALTH_STATE_ERROR: Self = Self(3);
    pub const FABRIC_HEALTH_STATE_UNKNOWN: Self = Self(65535);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_HEALTH_STATE_COUNT {
    pub OkCount: u32,
    pub WarningCount: u32,
    pub ErrorCount: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_HEALTH_STATE_COUNT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_HEALTH_STATE_FILTER(pub i32);
impl FABRIC_HEALTH_STATE_FILTER {
    pub const FABRIC_HEALTH_STATE_FILTER_DEFAULT: Self = Self(0);
    pub const FABRIC_HEALTH_STATE_FILTER_NONE: Self = Self(1);
    pub const FABRIC_HEALTH_STATE_FILTER_OK: Self = Self(2);
    pub const FABRIC_HEALTH_STATE_FILTER_WARNING: Self = Self(4);
    pub const FABRIC_HEALTH_STATE_FILTER_ERROR: Self = Self(8);
    pub const FABRIC_HEALTH_STATE_FILTER_ALL: Self = Self(65535);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_HEALTH_STATISTICS {
    pub Count: u32,
    pub Items: *const FABRIC_ENTITY_KIND_HEALTH_STATE_COUNT,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_HEALTH_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_HOST_ISOLATION_MODE(pub i32);
impl FABRIC_HOST_ISOLATION_MODE {
    pub const FABRIC_HOST_ISOLATION_MODE_NONE: Self = Self(0);
    pub const FABRIC_HOST_ISOLATION_MODE_PROCESS: Self = Self(1);
    pub const FABRIC_HOST_ISOLATION_MODE_HYPER_V: Self = Self(2);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FABRIC_HOST_PROCESS_ID(pub i64);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_HOST_TYPE(pub i32);
impl FABRIC_HOST_TYPE {
    pub const FABRIC_HOST_TYPE_INVALID: Self = Self(0);
    pub const FABRIC_HOST_TYPE_EXE_HOST: Self = Self(1);
    pub const FABRIC_HOST_TYPE_CONTAINER_HOST: Self = Self(2);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_IDENTITY_BINDING_POLICY_DESCRIPTION {
    pub ServiceIdentityRef: LPCWSTR,
    pub ApplicationIdentityRef: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_IDENTITY_BINDING_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FABRIC_IGNORE_SEQUENCE_NUMBER_CHECK: u32 = 0;
pub const FABRIC_INFINITE_DURATION: u32 = 4294967295;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FABRIC_INSTANCE_ID(pub i64);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_INT64_RANGE_PARTITION_INFORMATION {
    pub Id: FABRIC_PARTITION_ID,
    pub LowKey: i64,
    pub HighKey: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_INT64_RANGE_PARTITION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FABRIC_INVALID_ATOMIC_GROUP_ID: i32 = -1;
pub const FABRIC_INVALID_INSTANCE_ID: i32 = -1;
pub const FABRIC_INVALID_NODE_INSTANCE_ID: u32 = 0;
pub const FABRIC_INVALID_OPERATION_INDEX: u32 = 4294967295;
pub const FABRIC_INVALID_REPLICA_ID: i32 = -1;
pub const FABRIC_INVALID_SEQUENCE_NUMBER: i32 = -1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_KEY_VALUE_STORE_FULL_COPY_MODE(pub i32);
impl FABRIC_KEY_VALUE_STORE_FULL_COPY_MODE {
    pub const FABRIC_KEY_VALUE_STORE_FULL_COPY_MODE_DEFAULT: Self = Self(0);
    pub const FABRIC_KEY_VALUE_STORE_FULL_COPY_MODE_PHYSICAL: Self = Self(1);
    pub const FABRIC_KEY_VALUE_STORE_FULL_COPY_MODE_LOGICAL: Self = Self(2);
    pub const FABRIC_KEY_VALUE_STORE_FULL_COPY_MODE_REBUILD: Self = Self(3);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_KEY_VALUE_STORE_ITEM {
    pub Metadata: *const FABRIC_KEY_VALUE_STORE_ITEM_METADATA,
    pub Value: *mut u8,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_KEY_VALUE_STORE_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_KEY_VALUE_STORE_ITEM_METADATA {
    pub Key: LPCWSTR,
    pub ValueSizeInBytes: i32,
    pub SequenceNumber: i64,
    pub LastModifiedUtc: FILETIME,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_KEY_VALUE_STORE_ITEM_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_KEY_VALUE_STORE_ITEM_METADATA_EX1 {
    pub LastModifiedOnPrimaryUtc: FILETIME,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_KEY_VALUE_STORE_ITEM_METADATA_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE(pub i32);
impl FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE {
    pub const FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE_INACTIVE: Self = Self(0);
    pub const FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE_MIGRATION: Self = Self(1);
    pub const FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE_TARGET_DATABASE_SWAP: Self = Self(2);
    pub const FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE_TARGET_DATABASE_CLEANUP: Self = Self(3);
    pub const FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE_SOURCE_DATABASE_CLEANUP: Self = Self(4);
    pub const FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE_TARGET_DATABASE_ACTIVE: Self = Self(5);
    pub const FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE_RESTORE_SOURCE_BACKUP: Self = Self(6);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_KEY_VALUE_STORE_MIGRATION_QUERY_RESULT {
    pub CurrentPhase: FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE,
    pub State: FABRIC_KEY_VALUE_STORE_MIGRATION_STATE,
    pub NextPhase: FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_KEY_VALUE_STORE_MIGRATION_QUERY_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_KEY_VALUE_STORE_MIGRATION_STATE(pub i32);
impl FABRIC_KEY_VALUE_STORE_MIGRATION_STATE {
    pub const FABRIC_KEY_VALUE_STORE_MIGRATION_STATE_INACTIVE: Self = Self(0);
    pub const FABRIC_KEY_VALUE_STORE_MIGRATION_STATE_PROCESSING: Self = Self(1);
    pub const FABRIC_KEY_VALUE_STORE_MIGRATION_STATE_COMPLETED: Self = Self(2);
    pub const FABRIC_KEY_VALUE_STORE_MIGRATION_STATE_CANCELED: Self = Self(3);
    pub const FABRIC_KEY_VALUE_STORE_MIGRATION_STATE_FAILED: Self = Self(4);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE(pub i32);
impl FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE {
    pub const FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE_INVALID: Self = Self(0);
    pub const FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE_NONE: Self = Self(1);
    pub const FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE_NON_BLOCKING_QUORUM_ACKED: Self = Self(2);
    pub const FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE_BLOCK_SECONDARY_ACK: Self = Self(3);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_KEY_VALUE_STORE_PROVIDER_KIND(pub i32);
impl FABRIC_KEY_VALUE_STORE_PROVIDER_KIND {
    pub const FABRIC_KEY_VALUE_STORE_PROVIDER_KIND_UNKNOWN: Self = Self(0);
    pub const FABRIC_KEY_VALUE_STORE_PROVIDER_KIND_ESE: Self = Self(1);
    pub const FABRIC_KEY_VALUE_STORE_PROVIDER_KIND_TSTORE: Self = Self(2);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_KEY_VALUE_STORE_REPLICA_SETTINGS {
    pub TransactionDrainTimeoutInSeconds: u32,
    pub SecondaryNotificationMode: FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_KEY_VALUE_STORE_REPLICA_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_KEY_VALUE_STORE_REPLICA_SETTINGS_EX1 {
    pub EnableCopyNotificationPrefetch: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_KEY_VALUE_STORE_REPLICA_SETTINGS_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_KEY_VALUE_STORE_REPLICA_SETTINGS_EX2 {
    pub FullCopyMode: FABRIC_KEY_VALUE_STORE_FULL_COPY_MODE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_KEY_VALUE_STORE_REPLICA_SETTINGS_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_KEY_VALUE_STORE_REPLICA_SETTINGS_EX3 {
    pub LogTruncationIntervalInMinutes: i32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_KEY_VALUE_STORE_REPLICA_SETTINGS_EX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_KEY_VALUE_STORE_REPLICA_SETTINGS_EX4 {
    pub DisableTombstoneCleanup: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_KEY_VALUE_STORE_REPLICA_SETTINGS_EX4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_KEY_VALUE_STORE_REPLICA_SETTINGS_EX5 {
    pub LogicalCopyProbabilityInPercent: i32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_KEY_VALUE_STORE_REPLICA_SETTINGS_EX5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_KEY_VALUE_STORE_RESTORE_SETTINGS {
    pub InlineReopen: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_KEY_VALUE_STORE_RESTORE_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_KEY_VALUE_STORE_RESTORE_SETTINGS_EX1 {
    pub EnableLsnCheck: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_KEY_VALUE_STORE_RESTORE_SETTINGS_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_KEY_VALUE_STORE_STATUS_QUERY_RESULT {
    pub DatabaseRowCountEstimate: i64,
    pub DatabaseLogicalSizeEstimate: i64,
    pub CopyNotificationCurrentKeyFilter: LPCWSTR,
    pub CopyNotificationCurrentProgress: i64,
    pub StatusDetails: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_KEY_VALUE_STORE_STATUS_QUERY_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_KEY_VALUE_STORE_STATUS_QUERY_RESULT_EX1 {
    pub ProviderKind: FABRIC_KEY_VALUE_STORE_PROVIDER_KIND,
    pub MigrationStatus: *const FABRIC_KEY_VALUE_STORE_MIGRATION_QUERY_RESULT,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_KEY_VALUE_STORE_STATUS_QUERY_RESULT_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_KEY_VALUE_STORE_STATUS_QUERY_RESULT_EX2 {
    pub DatabasePhysicalSizeEstimateInBytes: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_KEY_VALUE_STORE_STATUS_QUERY_RESULT_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_KEY_VALUE_STORE_TRANSACTION_SETTINGS {
    pub SerializationBlockSize: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_KEY_VALUE_STORE_TRANSACTION_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_LOADED_PARTITION_INFORMATION_QUERY_DESCRIPTION {
    pub MetricName: LPCWSTR,
    pub ServiceName: FABRIC_URI,
    pub Ordering: FABRIC_ORDERING,
    pub PagingDescription: *mut FABRIC_QUERY_PAGING_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_LOADED_PARTITION_INFORMATION_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_LOADED_PARTITION_INFORMATION_QUERY_RESULT_ITEM {
    pub ServiceName: FABRIC_URI,
    pub PartitionId: FABRIC_PARTITION_ID,
    pub MetricName: LPCWSTR,
    pub Load: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_LOADED_PARTITION_INFORMATION_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_LOADED_PARTITION_INFORMATION_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_LOADED_PARTITION_INFORMATION_QUERY_RESULT_ITEM,
}
impl Default for FABRIC_LOADED_PARTITION_INFORMATION_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_LOAD_METRIC {
    pub Name: LPCWSTR,
    pub Value: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_LOAD_METRIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FABRIC_LOAD_METRIC_INFORMATION {
    pub Name: LPCWSTR,
    pub IsBalancedBefore: bool,
    pub IsBalancedAfter: bool,
    pub DeviationBefore: f64,
    pub DeviationAfter: f64,
    pub BalancingThreshold: f64,
    pub Action: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_LOAD_METRIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_LOAD_METRIC_INFORMATION_EX1 {
    pub ActivityThreshold: u32,
    pub ClusterCapacity: i64,
    pub ClusterLoad: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_LOAD_METRIC_INFORMATION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FABRIC_LOAD_METRIC_INFORMATION_EX2 {
    pub RemainingUnbufferedCapacity: i64,
    pub NodeBufferPercentage: f64,
    pub BufferedCapacity: i64,
    pub RemainingBufferedCapacity: i64,
    pub IsClusterCapacityViolation: bool,
    pub MinNodeLoadValue: i64,
    pub MinNodeLoadNodeId: FABRIC_NODE_ID,
    pub MaxNodeLoadValue: i64,
    pub MaxNodeLoadNodeId: FABRIC_NODE_ID,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_LOAD_METRIC_INFORMATION_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FABRIC_LOAD_METRIC_INFORMATION_EX3 {
    pub CurrentClusterLoad: f64,
    pub BufferedClusterCapacityRemaining: f64,
    pub ClusterCapacityRemaining: f64,
    pub MaximumNodeLoad: f64,
    pub MinimumNodeLoad: f64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_LOAD_METRIC_INFORMATION_EX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FABRIC_LOAD_METRIC_INFORMATION_EX4 {
    pub PlannedLoadRemoval: f64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_LOAD_METRIC_INFORMATION_EX4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_LOAD_METRIC_INFORMATION_LIST {
    pub Count: u32,
    pub Items: *mut FABRIC_LOAD_METRIC_INFORMATION,
}
impl Default for FABRIC_LOAD_METRIC_INFORMATION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_LOAD_METRIC_REPORT {
    pub Name: LPCWSTR,
    pub Value: u32,
    pub LastReportedUtc: FILETIME,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_LOAD_METRIC_REPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FABRIC_LOAD_METRIC_REPORT_EX1 {
    pub CurrentValue: f64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_LOAD_METRIC_REPORT_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_LOAD_METRIC_REPORT_LIST {
    pub Count: u32,
    pub Items: *mut FABRIC_LOAD_METRIC_REPORT,
}
impl Default for FABRIC_LOAD_METRIC_REPORT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_LOCAL_NETWORK_CONFIGURATION_DESCRIPTION {
    pub NetworkAddressPrefix: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_LOCAL_NETWORK_CONFIGURATION_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_LOCAL_NETWORK_DESCRIPTION {
    pub NetworkConfiguration: *const FABRIC_LOCAL_NETWORK_CONFIGURATION_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_LOCAL_NETWORK_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_LOCAL_NETWORK_INFORMATION {
    pub NetworkName: LPCWSTR,
    pub NetworkConfiguration: *const FABRIC_LOCAL_NETWORK_CONFIGURATION_DESCRIPTION,
    pub NetworkStatus: FABRIC_NETWORK_STATUS,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_LOCAL_NETWORK_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_LOCAL_STORE_KIND(pub i32);
impl FABRIC_LOCAL_STORE_KIND {
    pub const FABRIC_LOCAL_STORE_KIND_INVALID: Self = Self(0);
    pub const FABRIC_LOCAL_STORE_KIND_ESE: Self = Self(1);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_MANAGED_APPLICATION_IDENTITY_DESCRIPTION {
    pub ManagedIdentities: *const FABRIC_MANAGED_IDENTITY_DESCRIPTION_LIST,
    pub TokenServiceEndpoint: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_MANAGED_APPLICATION_IDENTITY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_MANAGED_IDENTITY_DESCRIPTION {
    pub Name: LPCWSTR,
    pub PrincipalId: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_MANAGED_IDENTITY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_MANAGED_IDENTITY_DESCRIPTION_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_MANAGED_IDENTITY_DESCRIPTION,
}
impl Default for FABRIC_MANAGED_IDENTITY_DESCRIPTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_MANAGED_IDENTITY_POLICY_BINDING {
    pub ServiceIdentityRef: LPCWSTR,
    pub ApplicationIdentityRef: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_MANAGED_IDENTITY_POLICY_BINDING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_MANAGED_IDENTITY_TOKEN_SERVICE_DESCRIPTION {
    pub IsTokenServiceEnabled: bool,
    pub IsClusterIdentityEnabled: bool,
    pub ApplicationTokenEndpoint: LPCWSTR,
    pub SystemTokenEndpoint: LPCWSTR,
    pub ClusterIdentityObjectId: LPCWSTR,
    pub RemoteTokenEndpoint: LPCWSTR,
    pub ServerCertificateThumbprint: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_MANAGED_IDENTITY_TOKEN_SERVICE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_MANAGED_KEY_VAULT_REFERENCE_PARAMETER {
    pub Name: LPCWSTR,
    pub Identity: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_MANAGED_KEY_VAULT_REFERENCE_PARAMETER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_MANAGED_KEY_VAULT_REFERENCE_PARAMETER_LIST {
    pub Count: u32,
    pub Items: *mut FABRIC_MANAGED_KEY_VAULT_REFERENCE_PARAMETER,
}
impl Default for FABRIC_MANAGED_KEY_VAULT_REFERENCE_PARAMETER_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_METRIC_LOAD_DESCRIPTION {
    pub MetricName: LPCWSTR,
    pub CurrentLoad: u32,
    pub IsCurrentLoadSpecified: bool,
    pub PredictedLoad: u32,
    pub IsPredictedLoadSpecified: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_METRIC_LOAD_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_METRIC_LOAD_DESCRIPTION_LIST {
    pub Count: u32,
    pub Items: *mut FABRIC_METRIC_LOAD_DESCRIPTION,
}
impl Default for FABRIC_METRIC_LOAD_DESCRIPTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_MONITORED_UPGRADE_FAILURE_ACTION(pub i32);
impl FABRIC_MONITORED_UPGRADE_FAILURE_ACTION {
    pub const FABRIC_MONITORED_UPGRADE_FAILURE_ACTION_INVALID: Self = Self(0);
    pub const FABRIC_MONITORED_UPGRADE_FAILURE_ACTION_ROLLBACK: Self = Self(1);
    pub const FABRIC_MONITORED_UPGRADE_FAILURE_ACTION_MANUAL: Self = Self(2);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_MONITORED_UPGRADE_HEALTH_CHECK_PHASE(pub i32);
impl FABRIC_MONITORED_UPGRADE_HEALTH_CHECK_PHASE {
    pub const FABRIC_MONITORED_UPGRADE_HEALTH_CHECK_PHASE_INVALID: Self = Self(0);
    pub const FABRIC_MONITORED_UPGRADE_HEALTH_CHECK_PHASE_WAIT_DURATION: Self = Self(1);
    pub const FABRIC_MONITORED_UPGRADE_HEALTH_CHECK_PHASE_STABLE_DURATION: Self = Self(2);
    pub const FABRIC_MONITORED_UPGRADE_HEALTH_CHECK_PHASE_RETRY: Self = Self(3);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_MOVE_AUXILIARY_DESCRIPTION {
    pub Kind: FABRIC_MOVE_AUXILIARY_DESCRIPTION_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_MOVE_AUXILIARY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_MOVE_AUXILIARY_DESCRIPTION_KIND(pub i32);
impl FABRIC_MOVE_AUXILIARY_DESCRIPTION_KIND {
    pub const FABRIC_MOVE_AUXILIARY_DESCRIPTION_KIND_INVALID: Self = Self(0);
    pub const FABRIC_MOVE_AUXILIARY_DESCRIPTION_KIND_USING_NODE_NAME: Self = Self(1);
    pub const FABRIC_MOVE_AUXILIARY_DESCRIPTION_KIND_USING_REPLICA_SELECTOR: Self = Self(2);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_MOVE_AUXILIARY_DESCRIPTION_USING_NODE_NAME {
    pub CurrentNodeName: LPCWSTR,
    pub NewNodeName: LPCWSTR,
    pub ServiceName: FABRIC_URI,
    pub PartitionId: FABRIC_PARTITION_ID,
    pub IgnoreConstraints: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_MOVE_AUXILIARY_DESCRIPTION_USING_NODE_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_MOVE_AUXILIARY_RESULT {
    pub CurrentNodeName: LPCWSTR,
    pub NewNodeName: LPCWSTR,
    pub ServiceName: FABRIC_URI,
    pub PartitionId: FABRIC_PARTITION_ID,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_MOVE_AUXILIARY_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_MOVE_COST(pub i32);
impl FABRIC_MOVE_COST {
    pub const FABRIC_MOVE_COST_ZERO: Self = Self(0);
    pub const FABRIC_MOVE_COST_LOW: Self = Self(1);
    pub const FABRIC_MOVE_COST_MEDIUM: Self = Self(2);
    pub const FABRIC_MOVE_COST_HIGH: Self = Self(3);
    pub const FABRIC_MOVE_COST_VERYHIGH: Self = Self(4);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_MOVE_INSTANCE_DESCRIPTION {
    pub Kind: FABRIC_MOVE_INSTANCE_DESCRIPTION_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_MOVE_INSTANCE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_MOVE_INSTANCE_DESCRIPTION_KIND(pub i32);
impl FABRIC_MOVE_INSTANCE_DESCRIPTION_KIND {
    pub const FABRIC_MOVE_INSTANCE_DESCRIPTION_KIND_INVALID: Self = Self(0);
    pub const FABRIC_MOVE_INSTANCE_DESCRIPTION_KIND_USING_NODE_NAME: Self = Self(1);
    pub const FABRIC_MOVE_INSTANCE_DESCRIPTION_KIND_USING_REPLICA_SELECTOR: Self = Self(2);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_MOVE_INSTANCE_DESCRIPTION_USING_NODE_NAME {
    pub CurrentNodeName: LPCWSTR,
    pub NewNodeName: LPCWSTR,
    pub ServiceName: FABRIC_URI,
    pub PartitionId: FABRIC_PARTITION_ID,
    pub IgnoreConstraints: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_MOVE_INSTANCE_DESCRIPTION_USING_NODE_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_MOVE_INSTANCE_RESULT {
    pub CurrentNodeName: LPCWSTR,
    pub NewNodeName: LPCWSTR,
    pub ServiceName: FABRIC_URI,
    pub PartitionId: FABRIC_PARTITION_ID,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_MOVE_INSTANCE_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_MOVE_PRIMARY_DESCRIPTION2 {
    pub Kind: FABRIC_MOVE_PRIMARY_DESCRIPTION_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_MOVE_PRIMARY_DESCRIPTION2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_MOVE_PRIMARY_DESCRIPTION_KIND(pub i32);
impl FABRIC_MOVE_PRIMARY_DESCRIPTION_KIND {
    pub const FABRIC_MOVE_PRIMARY_DESCRIPTION_KIND_INVALID: Self = Self(0);
    pub const FABRIC_MOVE_PRIMARY_DESCRIPTION_KIND_USING_NODE_NAME: Self = Self(1);
    pub const FABRIC_MOVE_PRIMARY_DESCRIPTION_KIND_USING_REPLICA_SELECTOR: Self = Self(2);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_MOVE_PRIMARY_DESCRIPTION_USING_NODE_NAME {
    pub NodeName: LPCWSTR,
    pub ServiceName: FABRIC_URI,
    pub PartitionId: FABRIC_PARTITION_ID,
    pub IgnoreConstraints: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_MOVE_PRIMARY_DESCRIPTION_USING_NODE_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_MOVE_PRIMARY_RESULT {
    pub NodeName: LPCWSTR,
    pub ServiceName: FABRIC_URI,
    pub PartitionId: FABRIC_PARTITION_ID,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_MOVE_PRIMARY_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_MOVE_SECONDARY_DESCRIPTION2 {
    pub Kind: FABRIC_MOVE_SECONDARY_DESCRIPTION_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_MOVE_SECONDARY_DESCRIPTION2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_MOVE_SECONDARY_DESCRIPTION_KIND(pub i32);
impl FABRIC_MOVE_SECONDARY_DESCRIPTION_KIND {
    pub const FABRIC_MOVE_SECONDARY_DESCRIPTION_KIND_INVALID: Self = Self(0);
    pub const FABRIC_MOVE_SECONDARY_DESCRIPTION_KIND_USING_NODE_NAME: Self = Self(1);
    pub const FABRIC_MOVE_SECONDARY_DESCRIPTION_KIND_USING_REPLICA_SELECTOR: Self = Self(2);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_MOVE_SECONDARY_DESCRIPTION_USING_NODE_NAME {
    pub CurrentNodeName: LPCWSTR,
    pub NewNodeName: LPCWSTR,
    pub ServiceName: FABRIC_URI,
    pub PartitionId: FABRIC_PARTITION_ID,
    pub IgnoreConstraints: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_MOVE_SECONDARY_DESCRIPTION_USING_NODE_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_MOVE_SECONDARY_RESULT {
    pub CurrentNodeName: LPCWSTR,
    pub NewNodeName: LPCWSTR,
    pub ServiceName: FABRIC_URI,
    pub PartitionId: FABRIC_PARTITION_ID,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_MOVE_SECONDARY_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NAMED_PARTITION_INFORMATION {
    pub Id: FABRIC_PARTITION_ID,
    pub Name: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NAMED_PARTITION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NAMED_PARTITION_SCHEME_DESCRIPTION {
    pub PartitionCount: i32,
    pub Names: *mut LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NAMED_PARTITION_SCHEME_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NAMED_PROPERTY {
    pub Metadata: *const FABRIC_NAMED_PROPERTY_METADATA,
    pub Value: *mut u8,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NAMED_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NAMED_PROPERTY_METADATA {
    pub PropertyName: LPCWSTR,
    pub TypeId: FABRIC_PROPERTY_TYPE_ID,
    pub ValueSize: i32,
    pub SequenceNumber: i64,
    pub LastModifiedUtc: FILETIME,
    pub Name: FABRIC_URI,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NAMED_PROPERTY_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NAMED_PROPERTY_METADATA_EX1 {
    pub CustomTypeId: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NAMED_PROPERTY_METADATA_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NAMED_REPARTITION_DESCRIPTION {
    pub NamesToAddCount: u32,
    pub NamesToAdd: *mut LPCWSTR,
    pub NamesToRemoveCount: u32,
    pub NamesToRemove: *mut LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NAMED_REPARTITION_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NETWORK_APPLICATION_QUERY_DESCRIPTION {
    pub NetworkName: LPCWSTR,
    pub ApplicationNameFilter: FABRIC_URI,
    pub PagingDescription: *const FABRIC_QUERY_PAGING_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NETWORK_APPLICATION_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NETWORK_APPLICATION_QUERY_RESULT_ITEM {
    pub ApplicationName: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NETWORK_APPLICATION_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NETWORK_APPLICATION_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_NETWORK_APPLICATION_QUERY_RESULT_ITEM,
}
impl Default for FABRIC_NETWORK_APPLICATION_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NETWORK_DESCRIPTION {
    pub NetworkType: FABRIC_NETWORK_TYPE,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_NETWORK_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NETWORK_INFORMATION {
    pub NetworkType: FABRIC_NETWORK_TYPE,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_NETWORK_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NETWORK_NODE_QUERY_DESCRIPTION {
    pub NetworkName: LPCWSTR,
    pub NodeNameFilter: LPCWSTR,
    pub PagingDescription: *const FABRIC_QUERY_PAGING_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NETWORK_NODE_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NETWORK_NODE_QUERY_RESULT_ITEM {
    pub NodeName: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NETWORK_NODE_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NETWORK_NODE_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_NETWORK_NODE_QUERY_RESULT_ITEM,
}
impl Default for FABRIC_NETWORK_NODE_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NETWORK_QUERY_DESCRIPTION {
    pub NetworkNameFilter: LPCWSTR,
    pub NetworkStatusFilter: u32,
    pub PagingDescription: *const FABRIC_QUERY_PAGING_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NETWORK_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NETWORK_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_NETWORK_INFORMATION,
}
impl Default for FABRIC_NETWORK_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_NETWORK_STATUS(pub i32);
impl FABRIC_NETWORK_STATUS {
    pub const FABRIC_NETWORK_STATUS_INVALID: Self = Self(0);
    pub const FABRIC_NETWORK_STATUS_READY: Self = Self(1);
    pub const FABRIC_NETWORK_STATUS_CREATING: Self = Self(2);
    pub const FABRIC_NETWORK_STATUS_DELETING: Self = Self(3);
    pub const FABRIC_NETWORK_STATUS_UPDATING: Self = Self(4);
    pub const FABRIC_NETWORK_STATUS_FAILED: Self = Self(5);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_NETWORK_STATUS_FILTER(pub i32);
impl FABRIC_NETWORK_STATUS_FILTER {
    pub const FABRIC_NETWORK_STATUS_FILTER_DEFAULT: Self = Self(0);
    pub const FABRIC_NETWORK_STATUS_FILTER_ALL: Self = Self(65535);
    pub const FABRIC_NETWORK_STATUS_FILTER_READY: Self = Self(1);
    pub const FABRIC_NETWORK_STATUS_FILTER_CREATING: Self = Self(2);
    pub const FABRIC_NETWORK_STATUS_FILTER_DELETING: Self = Self(4);
    pub const FABRIC_NETWORK_STATUS_FILTER_UPDATING: Self = Self(8);
    pub const FABRIC_NETWORK_STATUS_FILTER_FAILED: Self = Self(16);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_NETWORK_TYPE(pub i32);
impl FABRIC_NETWORK_TYPE {
    pub const FABRIC_NETWORK_TYPE_INVALID: Self = Self(0);
    pub const FABRIC_NETWORK_TYPE_LOCAL: Self = Self(1);
    pub const FABRIC_NETWORK_TYPE_FEDERATED: Self = Self(2);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODES_HEALTH_EVALUATION {
    pub Description: LPCWSTR,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub TotalCount: u32,
    pub MaxPercentUnhealthyNodes: u8,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODES_HEALTH_EVALUATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_CONTEXT {
    pub NodeName: LPCWSTR,
    pub NodeType: LPCWSTR,
    pub IPAddressOrFQDN: LPCWSTR,
    pub NodeInstanceId: u64,
    pub NodeId: FABRIC_NODE_ID,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_NODE_DEACTIVATION_INTENT(pub i32);
impl FABRIC_NODE_DEACTIVATION_INTENT {
    pub const FABRIC_NODE_DEACTIVATION_INTENT_INVALID: Self = Self(0);
    pub const FABRIC_NODE_DEACTIVATION_INTENT_PAUSE: Self = Self(1);
    pub const FABRIC_NODE_DEACTIVATION_INTENT_RESTART: Self = Self(2);
    pub const FABRIC_NODE_DEACTIVATION_INTENT_REMOVE_DATA: Self = Self(3);
    pub const FABRIC_NODE_DEACTIVATION_INTENT_REMOVE_NODE: Self = Self(4);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_DEACTIVATION_QUERY_RESULT_ITEM {
    pub EffectiveIntent: FABRIC_NODE_DEACTIVATION_INTENT,
    pub Status: FABRIC_NODE_DEACTIVATION_STATUS,
    pub Tasks: *const FABRIC_NODE_DEACTIVATION_TASK_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_DEACTIVATION_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_DEACTIVATION_QUERY_RESULT_ITEM_EX1 {
    pub PendingSafetyChecks: *const FABRIC_SAFETY_CHECK_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_DEACTIVATION_QUERY_RESULT_ITEM_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_NODE_DEACTIVATION_STATUS(pub i32);
impl FABRIC_NODE_DEACTIVATION_STATUS {
    pub const FABRIC_NODE_DEACTIVATION_STATUS_NONE: Self = Self(0);
    pub const FABRIC_NODE_DEACTIVATION_STATUS_SAFETY_CHECK_IN_PROGRESS: Self = Self(1);
    pub const FABRIC_NODE_DEACTIVATION_STATUS_SAFETY_CHECK_COMPLETE: Self = Self(2);
    pub const FABRIC_NODE_DEACTIVATION_STATUS_COMPLETED: Self = Self(3);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_DEACTIVATION_TASK {
    pub TaskId: *const FABRIC_NODE_DEACTIVATION_TASK_ID,
    pub Intent: FABRIC_NODE_DEACTIVATION_INTENT,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_DEACTIVATION_TASK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_DEACTIVATION_TASK_EX1 {
    pub Description: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_DEACTIVATION_TASK_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_DEACTIVATION_TASK_ID {
    pub Id: LPCWSTR,
    pub Type: FABRIC_NODE_DEACTIVATION_TASK_TYPE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_DEACTIVATION_TASK_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_DEACTIVATION_TASK_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_NODE_DEACTIVATION_TASK,
}
impl Default for FABRIC_NODE_DEACTIVATION_TASK_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_NODE_DEACTIVATION_TASK_TYPE(pub i32);
impl FABRIC_NODE_DEACTIVATION_TASK_TYPE {
    pub const FABRIC_NODE_DEACTIVATION_TASK_TYPE_INVALID: Self = Self(0);
    pub const FABRIC_NODE_DEACTIVATION_TASK_TYPE_INFRASTRUCTURE: Self = Self(1);
    pub const FABRIC_NODE_DEACTIVATION_TASK_TYPE_REPAIR: Self = Self(2);
    pub const FABRIC_NODE_DEACTIVATION_TASK_TYPE_CLIENT: Self = Self(3);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_HEALTH {
    pub NodeName: LPCWSTR,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub HealthEvents: *const FABRIC_HEALTH_EVENT_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_HEALTH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_HEALTH_EVALUATION {
    pub Description: LPCWSTR,
    pub NodeName: LPCWSTR,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_HEALTH_EVALUATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_HEALTH_EX1 {
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_HEALTH_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_HEALTH_QUERY_DESCRIPTION {
    pub NodeName: LPCWSTR,
    pub HealthPolicy: *const FABRIC_CLUSTER_HEALTH_POLICY,
    pub EventsFilter: *const FABRIC_HEALTH_EVENTS_FILTER,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_HEALTH_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_HEALTH_REPORT {
    pub NodeName: LPCWSTR,
    pub HealthInformation: *const FABRIC_HEALTH_INFORMATION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_HEALTH_REPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_HEALTH_STATE {
    pub NodeName: LPCWSTR,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_HEALTH_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_HEALTH_STATES_FILTER {
    pub HealthStateFilter: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_HEALTH_STATES_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_HEALTH_STATE_CHUNK {
    pub NodeName: LPCWSTR,
    pub HealthState: FABRIC_HEALTH_STATE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_HEALTH_STATE_CHUNK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_HEALTH_STATE_CHUNK_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_NODE_HEALTH_STATE_CHUNK,
    pub TotalCount: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_HEALTH_STATE_CHUNK_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_HEALTH_STATE_FILTER {
    pub HealthStateFilter: u32,
    pub NodeNameFilter: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_HEALTH_STATE_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_HEALTH_STATE_FILTER_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_NODE_HEALTH_STATE_FILTER,
}
impl Default for FABRIC_NODE_HEALTH_STATE_FILTER_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_HEALTH_STATE_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_NODE_HEALTH_STATE,
}
impl Default for FABRIC_NODE_HEALTH_STATE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_ID {
    pub Low: u64,
    pub High: u64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FABRIC_NODE_INSTANCE_ID(pub u64);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_LOAD_INFORMATION {
    pub NodeName: LPCWSTR,
    pub NodeLoadMetricInformation: *const FABRIC_NODE_LOAD_METRIC_INFORMATION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_LOAD_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_LOAD_INFORMATION_QUERY_DESCRIPTION {
    pub NodeName: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_LOAD_INFORMATION_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_LOAD_METRIC_INFORMATION {
    pub Name: LPCWSTR,
    pub NodeCapacity: i64,
    pub NodeLoad: i64,
    pub NodeRemainingCapacity: i64,
    pub IsCapacityViolation: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_LOAD_METRIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_LOAD_METRIC_INFORMATION_EX1 {
    pub NodeBufferedCapacity: i64,
    pub NodeRemainingBufferedCapacity: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_LOAD_METRIC_INFORMATION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FABRIC_NODE_LOAD_METRIC_INFORMATION_EX2 {
    pub CurrentNodeLoad: f64,
    pub NodeCapacityRemaining: f64,
    pub BufferedNodeCapacityRemaining: f64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_LOAD_METRIC_INFORMATION_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FABRIC_NODE_LOAD_METRIC_INFORMATION_EX3 {
    pub PlannedNodeLoadRemoval: f64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_LOAD_METRIC_INFORMATION_EX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_LOAD_METRIC_INFORMATION_LIST {
    pub Count: u32,
    pub Items: *mut FABRIC_NODE_LOAD_METRIC_INFORMATION,
}
impl Default for FABRIC_NODE_LOAD_METRIC_INFORMATION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_QUERY_DESCRIPTION {
    pub NodeNameFilter: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_QUERY_DESCRIPTION_EX1 {
    pub ContinuationToken: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_QUERY_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_QUERY_DESCRIPTION_EX2 {
    pub NodeStatusFilter: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_QUERY_DESCRIPTION_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_QUERY_DESCRIPTION_EX3 {
    pub MaxResults: i32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_QUERY_DESCRIPTION_EX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_QUERY_RESULT_ITEM {
    pub NodeName: LPCWSTR,
    pub IpAddressOrFQDN: LPCWSTR,
    pub NodeType: LPCWSTR,
    pub CodeVersion: LPCWSTR,
    pub ConfigVersion: LPCWSTR,
    pub NodeStatus: FABRIC_QUERY_NODE_STATUS,
    pub NodeUpTimeInSeconds: i64,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub IsSeedNode: bool,
    pub UpgradeDomain: LPCWSTR,
    pub FaultDomain: FABRIC_URI,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_QUERY_RESULT_ITEM_EX1 {
    pub NodeId: FABRIC_NODE_ID,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_QUERY_RESULT_ITEM_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_QUERY_RESULT_ITEM_EX2 {
    pub NodeInstanceId: u64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_QUERY_RESULT_ITEM_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_QUERY_RESULT_ITEM_EX3 {
    pub NodeDeactivationInfo: *const FABRIC_NODE_DEACTIVATION_QUERY_RESULT_ITEM,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_QUERY_RESULT_ITEM_EX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_QUERY_RESULT_ITEM_EX4 {
    pub IsStopped: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_QUERY_RESULT_ITEM_EX4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_QUERY_RESULT_ITEM_EX5 {
    pub NodeDownTimeInSeconds: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_QUERY_RESULT_ITEM_EX5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_QUERY_RESULT_ITEM_EX6 {
    pub NodeUpAt: FILETIME,
    pub NodeDownAt: FILETIME,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_QUERY_RESULT_ITEM_EX6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_QUERY_RESULT_ITEM_EX7 {
    pub InfrastructurePlacementID: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_QUERY_RESULT_ITEM_EX7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_QUERY_RESULT_ITEM_EX8 {
    pub NodeTags: *mut FABRIC_STRING_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_QUERY_RESULT_ITEM_EX8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_QUERY_RESULT_ITEM_EX9 {
    pub IsNodeByNodeUpgradeInProgress: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_QUERY_RESULT_ITEM_EX9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_NODE_QUERY_RESULT_ITEM,
}
impl Default for FABRIC_NODE_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_RESULT {
    pub NodeName: LPCWSTR,
    pub NodeInstance: u64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_START_DESCRIPTION {
    pub OperationId: FABRIC_TEST_COMMAND_OPERATION_ID,
    pub NodeName: LPCWSTR,
    pub NodeInstanceId: u64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_START_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_STOP_DESCRIPTION {
    pub OperationId: FABRIC_TEST_COMMAND_OPERATION_ID,
    pub NodeName: LPCWSTR,
    pub NodeInstanceId: u64,
    pub StopDurationInSeconds: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_STOP_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_TRANSITION_DESCRIPTION {
    pub NodeTransitionType: FABRIC_NODE_TRANSITION_TYPE,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_TRANSITION_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_TRANSITION_PROGRESS {
    pub State: FABRIC_TEST_COMMAND_PROGRESS_STATE,
    pub Result: *mut FABRIC_NODE_TRANSITION_RESULT,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_TRANSITION_PROGRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_TRANSITION_RESULT {
    pub ErrorCode: windows_core::HRESULT,
    pub NodeResult: *mut FABRIC_NODE_RESULT,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_TRANSITION_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_NODE_TRANSITION_TYPE(pub i32);
impl FABRIC_NODE_TRANSITION_TYPE {
    pub const FABRIC_NODE_TRANSITION_TYPE_INVALID: Self = Self(0);
    pub const FABRIC_NODE_TRANSITION_TYPE_START: Self = Self(1);
    pub const FABRIC_NODE_TRANSITION_TYPE_STOP: Self = Self(2);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_TYPE_HEALTH_POLICY_MAP {
    pub Count: u32,
    pub Items: *const FABRIC_NODE_TYPE_HEALTH_POLICY_MAP_ITEM,
}
impl Default for FABRIC_NODE_TYPE_HEALTH_POLICY_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_NODE_TYPE_HEALTH_POLICY_MAP_ITEM {
    pub NodeTypeName: LPCWSTR,
    pub MaxPercentUnhealthyNodes: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_TYPE_NODEs_HEALTH_EVALUATION {
    pub Description: LPCWSTR,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub NodeTypeName: LPCWSTR,
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub TotalCount: u32,
    pub MaxPercentUnhealthyNodes: u8,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_TYPE_NODEs_HEALTH_EVALUATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_NODE_UPGRADE_PHASE(pub i32);
impl FABRIC_NODE_UPGRADE_PHASE {
    pub const FABRIC_NODE_UPGRADE_PHASE_INVALID: Self = Self(0);
    pub const FABRIC_NODE_UPGRADE_PHASE_PRE_UPGRADE_SAFETY_CHECK: Self = Self(1);
    pub const FABRIC_NODE_UPGRADE_PHASE_UPGRADING: Self = Self(2);
    pub const FABRIC_NODE_UPGRADE_PHASE_POST_UPGRADE_SAFETY_CHECK: Self = Self(3);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_UPGRADE_PROGRESS {
    pub NodeName: LPCWSTR,
    pub UpgradePhase: FABRIC_NODE_UPGRADE_PHASE,
    pub PendingSafetyChecks: *const FABRIC_UPGRADE_SAFETY_CHECK_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_UPGRADE_PROGRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_UPGRADE_PROGRESS_EX1 {
    pub UpgradeDurationInSeconds: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_NODE_UPGRADE_PROGRESS_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_NODE_UPGRADE_PROGRESS_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_NODE_UPGRADE_PROGRESS,
}
impl Default for FABRIC_NODE_UPGRADE_PROGRESS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_OPERATION_DATA_BUFFER {
    pub BufferSize: u32,
    pub Buffer: *mut u8,
}
impl Default for FABRIC_OPERATION_DATA_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_OPERATION_ID {
    pub PartitionId: windows_core::GUID,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_OPERATION_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_OPERATION_METADATA {
    pub Type: FABRIC_OPERATION_TYPE,
    pub SequenceNumber: i64,
    pub AtomicGroupId: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_OPERATION_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_OPERATION_TYPE(pub i32);
impl FABRIC_OPERATION_TYPE {
    pub const FABRIC_OPERATION_TYPE_INVALID: Self = Self(0);
    pub const FABRIC_OPERATION_TYPE_NORMAL: Self = Self(1);
    pub const FABRIC_OPERATION_TYPE_END_OF_STREAM: Self = Self(2);
    pub const FABRIC_OPERATION_TYPE_CREATE_ATOMIC_GROUP: Self = Self(16);
    pub const FABRIC_OPERATION_TYPE_ATOMIC_GROUP_OPERATION: Self = Self(32);
    pub const FABRIC_OPERATION_TYPE_COMMIT_ATOMIC_GROUP: Self = Self(64);
    pub const FABRIC_OPERATION_TYPE_ROLLBACK_ATOMIC_GROUP: Self = Self(128);
    pub const FABRIC_OPERATION_TYPE_HAS_ATOMIC_GROUP_MASK: Self = Self(240);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_ORCHESTRATION_UPGRADE_PROGRESS {
    pub UpgradeState: FABRIC_UPGRADE_STATE,
    pub ProgressStatus: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_ORCHESTRATION_UPGRADE_PROGRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_ORCHESTRATION_UPGRADE_PROGRESS_EX1 {
    pub ConfigVersion: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_ORCHESTRATION_UPGRADE_PROGRESS_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_ORCHESTRATION_UPGRADE_PROGRESS_EX2 {
    pub Details: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_ORCHESTRATION_UPGRADE_PROGRESS_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_ORDERING(pub i32);
impl FABRIC_ORDERING {
    pub const FABRIC_ORDERING_DESC: Self = Self(0);
    pub const FABRIC_ORDERING_ASC: Self = Self(1);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PACKAGE_SHARING_POLICY {
    pub PackageName: LPCWSTR,
    pub Scope: FABRIC_PACKAGE_SHARING_POLICY_SCOPE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PACKAGE_SHARING_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PACKAGE_SHARING_POLICY_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_PACKAGE_SHARING_POLICY,
}
impl Default for FABRIC_PACKAGE_SHARING_POLICY_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_PACKAGE_SHARING_POLICY_SCOPE(pub i32);
impl FABRIC_PACKAGE_SHARING_POLICY_SCOPE {
    pub const FABRIC_PACKAGE_SHARING_POLICY_SCOPE_NONE: Self = Self(0);
    pub const FABRIC_PACKAGE_SHARING_POLICY_SCOPE_ALL: Self = Self(1);
    pub const FABRIC_PACKAGE_SHARING_POLICY_SCOPE_CODE: Self = Self(2);
    pub const FABRIC_PACKAGE_SHARING_POLICY_SCOPE_CONFIG: Self = Self(3);
    pub const FABRIC_PACKAGE_SHARING_POLICY_SCOPE_DATA: Self = Self(4);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PAGED_DEPLOYED_APPLICATION_QUERY_DESCRIPTION {
    pub NodeName: LPCWSTR,
    pub ApplicationNameFilter: FABRIC_URI,
    pub IncludeHealthState: bool,
    pub PagingDescription: *const FABRIC_QUERY_PAGING_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PAGED_DEPLOYED_APPLICATION_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PAGED_DEPLOYED_SERVICE_PACKAGE_QUERY_DESCRIPTION {
    pub NodeName: LPCWSTR,
    pub ApplicationName: FABRIC_URI,
    pub ServiceManifestNameFilter: LPCWSTR,
    pub IncludeHealthState: bool,
    pub PagingDescription: *const FABRIC_QUERY_PAGING_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PAGED_DEPLOYED_SERVICE_PACKAGE_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PAGING_STATUS {
    pub ContinuationToken: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PAGING_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITIONS_HEALTH_EVALUATION {
    pub Description: LPCWSTR,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub TotalCount: u32,
    pub MaxPercentUnhealthyPartitionsPerService: u8,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITIONS_HEALTH_EVALUATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_DATA_LOSS_PROGRESS {
    pub State: FABRIC_TEST_COMMAND_PROGRESS_STATE,
    pub Result: *mut FABRIC_PARTITION_DATA_LOSS_RESULT,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_DATA_LOSS_PROGRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_DATA_LOSS_RESULT {
    pub SelectedPartition: *mut FABRIC_SELECTED_PARTITION,
    pub ErrorCode: windows_core::HRESULT,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_DATA_LOSS_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_HEALTH {
    pub PartitionId: FABRIC_PARTITION_ID,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub HealthEvents: *const FABRIC_HEALTH_EVENT_LIST,
    pub ReplicaHealthStates: *const FABRIC_REPLICA_HEALTH_STATE_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_HEALTH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_HEALTH_EVALUATION {
    pub Description: LPCWSTR,
    pub PartitionId: FABRIC_PARTITION_ID,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_HEALTH_EVALUATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_HEALTH_EX1 {
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_HEALTH_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_HEALTH_EX2 {
    pub HealthStatistics: *const FABRIC_HEALTH_STATISTICS,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_HEALTH_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_HEALTH_QUERY_DESCRIPTION {
    pub PartitionId: FABRIC_PARTITION_ID,
    pub HealthPolicy: *const FABRIC_APPLICATION_HEALTH_POLICY,
    pub EventsFilter: *const FABRIC_HEALTH_EVENTS_FILTER,
    pub ReplicasFilter: *const FABRIC_REPLICA_HEALTH_STATES_FILTER,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_HEALTH_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_HEALTH_QUERY_DESCRIPTION_EX1 {
    pub HealthStatisticsFilter: *const FABRIC_PARTITION_HEALTH_STATISTICS_FILTER,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_HEALTH_QUERY_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_HEALTH_REPORT {
    pub PartitionId: FABRIC_PARTITION_ID,
    pub HealthInformation: *const FABRIC_HEALTH_INFORMATION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_HEALTH_REPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_HEALTH_STATE {
    pub PartitionId: FABRIC_PARTITION_ID,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_HEALTH_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_HEALTH_STATES_FILTER {
    pub HealthStateFilter: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_HEALTH_STATES_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_HEALTH_STATE_CHUNK {
    pub PartitionId: FABRIC_PARTITION_ID,
    pub HealthState: FABRIC_HEALTH_STATE,
    pub ReplicaHealthStateChunks: *const FABRIC_REPLICA_HEALTH_STATE_CHUNK_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_HEALTH_STATE_CHUNK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_HEALTH_STATE_CHUNK_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_PARTITION_HEALTH_STATE_CHUNK,
    pub TotalCount: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_HEALTH_STATE_CHUNK_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_HEALTH_STATE_FILTER {
    pub HealthStateFilter: u32,
    pub PartitionIdFilter: FABRIC_PARTITION_ID,
    pub ReplicaFilters: *const FABRIC_REPLICA_HEALTH_STATE_FILTER_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_HEALTH_STATE_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_HEALTH_STATE_FILTER_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_PARTITION_HEALTH_STATE_FILTER,
}
impl Default for FABRIC_PARTITION_HEALTH_STATE_FILTER_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_HEALTH_STATE_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_PARTITION_HEALTH_STATE,
}
impl Default for FABRIC_PARTITION_HEALTH_STATE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_HEALTH_STATISTICS_FILTER {
    pub ExcludeHealthStatistics: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_HEALTH_STATISTICS_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FABRIC_PARTITION_ID = windows_core::GUID;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_PARTITION_KEY_TYPE(pub i32);
impl FABRIC_PARTITION_KEY_TYPE {
    pub const FABRIC_PARTITION_KEY_TYPE_INVALID: Self = Self(0);
    pub const FABRIC_PARTITION_KEY_TYPE_NONE: Self = Self(1);
    pub const FABRIC_PARTITION_KEY_TYPE_INT64: Self = Self(2);
    pub const FABRIC_PARTITION_KEY_TYPE_STRING: Self = Self(3);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_LOAD_INFORMATION {
    pub PartitionId: FABRIC_PARTITION_ID,
    pub PrimaryLoadMetricReports: *mut FABRIC_LOAD_METRIC_REPORT_LIST,
    pub SecondaryLoadMetricReports: *mut FABRIC_LOAD_METRIC_REPORT_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_LOAD_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_LOAD_INFORMATION_EX1 {
    pub AuxiliaryLoadMetricReports: *mut FABRIC_LOAD_METRIC_REPORT_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_LOAD_INFORMATION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_LOAD_INFORMATION_EX2 {
    pub MaximumLoadMetricReports: *mut FABRIC_LOAD_METRIC_REPORT_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_LOAD_INFORMATION_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_LOAD_INFORMATION_QUERY_DESCRIPTION {
    pub PartitionId: FABRIC_PARTITION_ID,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_LOAD_INFORMATION_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_METRIC_LOAD_DESCRIPTION {
    pub PartitionId: FABRIC_PARTITION_ID,
    pub PrimaryReplicaLoadEntries: *mut FABRIC_METRIC_LOAD_DESCRIPTION_LIST,
    pub SecondaryReplicasOrInstancesLoadEntries: *mut FABRIC_METRIC_LOAD_DESCRIPTION_LIST,
    pub SecondaryReplicaOrInstanceLoadEntriesPerNode:
        *mut FABRIC_REPLICA_METRIC_LOAD_DESCRIPTION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_METRIC_LOAD_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_METRIC_LOAD_DESCRIPTION_EX1 {
    pub AuxiliaryReplicasLoadEntries: *mut FABRIC_METRIC_LOAD_DESCRIPTION_LIST,
    pub AuxiliaryReplicaLoadEntriesPerNode: *mut FABRIC_REPLICA_METRIC_LOAD_DESCRIPTION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_METRIC_LOAD_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_METRIC_LOAD_DESCRIPTION_EX2 {
    pub MaximumReplicaLoadEntries: *mut FABRIC_METRIC_LOAD_DESCRIPTION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_METRIC_LOAD_DESCRIPTION_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_METRIC_LOAD_DESCRIPTION_LIST {
    pub Count: u32,
    pub Items: *mut FABRIC_PARTITION_METRIC_LOAD_DESCRIPTION,
}
impl Default for FABRIC_PARTITION_METRIC_LOAD_DESCRIPTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_MOVE_COST_DESCRIPTION {
    pub PartitionId: FABRIC_PARTITION_ID,
    pub PrimaryReplicaMoveCostEntry: FABRIC_MOVE_COST,
    pub IsPrimaryReplicaMoveCostEntrySpecified: bool,
    pub SecondaryReplicasOrInstancesMoveCostEntry: FABRIC_MOVE_COST,
    pub IsSecondaryReplicasOrInstancesMoveCostEntrySpecified: bool,
    pub SecondaryReplicaOrInstanceMoveCostEntriesPerNode:
        *mut FABRIC_REPLICA_MOVE_COST_DESCRIPTION_LIST,
    pub AuxiliaryReplicasMoveCostEntry: FABRIC_MOVE_COST,
    pub IsAuxiliaryReplicasMoveCostEntrySpecified: bool,
    pub AuxiliaryReplicaMoveCostEntriesPerNode: *mut FABRIC_REPLICA_MOVE_COST_DESCRIPTION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_MOVE_COST_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_MOVE_COST_DESCRIPTION_LIST {
    pub Count: u32,
    pub Items: *mut FABRIC_PARTITION_MOVE_COST_DESCRIPTION,
}
impl Default for FABRIC_PARTITION_MOVE_COST_DESCRIPTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_QUORUM_LOSS_PROGRESS {
    pub State: FABRIC_TEST_COMMAND_PROGRESS_STATE,
    pub Result: *mut FABRIC_PARTITION_QUORUM_LOSS_RESULT,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_QUORUM_LOSS_PROGRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_QUORUM_LOSS_RESULT {
    pub SelectedPartition: *mut FABRIC_SELECTED_PARTITION,
    pub ErrorCode: windows_core::HRESULT,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_QUORUM_LOSS_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_RESTART_PROGRESS {
    pub State: FABRIC_TEST_COMMAND_PROGRESS_STATE,
    pub Result: *mut FABRIC_PARTITION_RESTART_RESULT,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_RESTART_PROGRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_RESTART_RESULT {
    pub SelectedPartition: *mut FABRIC_SELECTED_PARTITION,
    pub ErrorCode: windows_core::HRESULT,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_RESTART_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_SAFETY_CHECK {
    pub PartitionId: FABRIC_PARTITION_ID,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_SAFETY_CHECK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_PARTITION_SCHEME(pub i32);
impl FABRIC_PARTITION_SCHEME {
    pub const FABRIC_PARTITION_SCHEME_INVALID: Self = Self(0);
    pub const FABRIC_PARTITION_SCHEME_SINGLETON: Self = Self(1);
    pub const FABRIC_PARTITION_SCHEME_UNIFORM_INT64_RANGE: Self = Self(2);
    pub const FABRIC_PARTITION_SCHEME_NAMED: Self = Self(3);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PARTITION_SELECTOR {
    pub ServiceName: FABRIC_URI,
    pub PartitionSelectorType: FABRIC_PARTITION_SELECTOR_TYPE,
    pub PartitionKey: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PARTITION_SELECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_PARTITION_SELECTOR_TYPE(pub i32);
impl FABRIC_PARTITION_SELECTOR_TYPE {
    pub const FABRIC_PARTITION_SELECTOR_TYPE_NONE: Self = Self(0);
    pub const FABRIC_PARTITION_SELECTOR_TYPE_SINGLETON: Self = Self(1);
    pub const FABRIC_PARTITION_SELECTOR_TYPE_NAMED: Self = Self(2);
    pub const FABRIC_PARTITION_SELECTOR_TYPE_UNIFORM_INT64: Self = Self(3);
    pub const FABRIC_PARTITION_SELECTOR_TYPE_PARTITION_ID: Self = Self(4);
    pub const FABRIC_PARTITION_SELECTOR_TYPE_RANDOM: Self = Self(5);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PLACEMENT_POLICY_ALLOW_MULTIPLE_STATELESS_INSTANCES_ON_NODE_DESCRIPTION {
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PLACEMENT_POLICY_ALLOW_MULTIPLE_STATELESS_INSTANCES_ON_NODE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PLACEMENT_POLICY_INVALID_DOMAIN_DESCRIPTION {
    pub InvalidFaultDomain: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PLACEMENT_POLICY_INVALID_DOMAIN_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PLACEMENT_POLICY_NONPARTIALLY_PLACE_SERVICE_DESCRIPTION {
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PLACEMENT_POLICY_NONPARTIALLY_PLACE_SERVICE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PLACEMENT_POLICY_PREFERRED_PRIMARY_DOMAIN_DESCRIPTION {
    pub PreferredPrimaryFaultDomain: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PLACEMENT_POLICY_PREFERRED_PRIMARY_DOMAIN_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PLACEMENT_POLICY_REQUIRED_DOMAIN_DESCRIPTION {
    pub RequiredFaultDomain: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PLACEMENT_POLICY_REQUIRED_DOMAIN_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PLACEMENT_POLICY_REQUIRED_DOMAIN_DISTRIBUTION_DESCRIPTION {
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PLACEMENT_POLICY_REQUIRED_DOMAIN_DISTRIBUTION_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_PLACEMENT_POLICY_TYPE(pub i32);
impl FABRIC_PLACEMENT_POLICY_TYPE {
    pub const FABRIC_PLACEMENT_POLICY_INVALID: Self = Self(0);
    pub const FABRIC_PLACEMENT_POLICY_INVALID_DOMAIN: Self = Self(1);
    pub const FABRIC_PLACEMENT_POLICY_REQUIRED_DOMAIN: Self = Self(2);
    pub const FABRIC_PLACEMENT_POLICY_PREFERRED_PRIMARY_DOMAIN: Self = Self(3);
    pub const FABRIC_PLACEMENT_POLICY_REQUIRED_DOMAIN_DISTRIBUTION: Self = Self(4);
    pub const FABRIC_PLACEMENT_POLICY_NONPARTIALLY_PLACE_SERVICE: Self = Self(5);
    pub const FABRIC_PLACEMENT_POLICY_ALLOW_MULTIPLE_STATELESS_INSTANCES_ON_NODE: Self = Self(6);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PRIMARY_REPLICATOR_STATUS_QUERY_RESULT {
    pub ReplicationQueueStatus: *mut FABRIC_REPLICATOR_QUEUE_STATUS,
    pub RemoteReplicators: *mut FABRIC_REMOTE_REPLICATOR_STATUS_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PRIMARY_REPLICATOR_STATUS_QUERY_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PROPERTY_BATCH_OPERATION {
    pub Kind: FABRIC_PROPERTY_BATCH_OPERATION_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_PROPERTY_BATCH_OPERATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_PROPERTY_BATCH_OPERATION_KIND(pub i32);
impl FABRIC_PROPERTY_BATCH_OPERATION_KIND {
    pub const FABRIC_PROPERTY_BATCH_OPERATION_KIND_INVALID: Self = Self(0);
    pub const FABRIC_PROPERTY_BATCH_OPERATION_KIND_PUT: Self = Self(1);
    pub const FABRIC_PROPERTY_BATCH_OPERATION_KIND_GET: Self = Self(2);
    pub const FABRIC_PROPERTY_BATCH_OPERATION_KIND_CHECK_EXISTS: Self = Self(3);
    pub const FABRIC_PROPERTY_BATCH_OPERATION_KIND_CHECK_SEQUENCE: Self = Self(4);
    pub const FABRIC_PROPERTY_BATCH_OPERATION_KIND_DELETE: Self = Self(5);
    pub const FABRIC_PROPERTY_BATCH_OPERATION_KIND_PUT_CUSTOM: Self = Self(6);
    pub const FABRIC_PROPERTY_BATCH_OPERATION_KIND_CHECK_VALUE: Self = Self(7);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_PROPERTY_TYPE_ID(pub i32);
impl FABRIC_PROPERTY_TYPE_ID {
    pub const FABRIC_PROPERTY_TYPE_INVALID: Self = Self(0);
    pub const FABRIC_PROPERTY_TYPE_BINARY: Self = Self(1);
    pub const FABRIC_PROPERTY_TYPE_INT64: Self = Self(2);
    pub const FABRIC_PROPERTY_TYPE_DOUBLE: Self = Self(3);
    pub const FABRIC_PROPERTY_TYPE_WSTRING: Self = Self(4);
    pub const FABRIC_PROPERTY_TYPE_GUID: Self = Self(5);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_PROTECTION_LEVEL(pub i32);
impl FABRIC_PROTECTION_LEVEL {
    pub const FABRIC_PROTECTION_LEVEL_NONE: Self = Self(0);
    pub const FABRIC_PROTECTION_LEVEL_SIGN: Self = Self(1);
    pub const FABRIC_PROTECTION_LEVEL_ENCRYPTANDSIGN: Self = Self(2);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PROVISIONED_CODE_VERSION_QUERY_DESCRIPTION {
    pub CodeVersionFilter: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PROVISIONED_CODE_VERSION_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PROVISIONED_CODE_VERSION_QUERY_RESULT_ITEM {
    pub CodeVersion: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PROVISIONED_CODE_VERSION_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PROVISIONED_CODE_VERSION_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_PROVISIONED_CODE_VERSION_QUERY_RESULT_ITEM,
}
impl Default for FABRIC_PROVISIONED_CODE_VERSION_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PROVISIONED_CONFIG_VERSION_QUERY_DESCRIPTION {
    pub ConfigVersionFilter: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PROVISIONED_CONFIG_VERSION_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PROVISIONED_CONFIG_VERSION_QUERY_RESULT_ITEM {
    pub ConfigVersion: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PROVISIONED_CONFIG_VERSION_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PROVISIONED_CONFIG_VERSION_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_PROVISIONED_CONFIG_VERSION_QUERY_RESULT_ITEM,
}
impl Default for FABRIC_PROVISIONED_CONFIG_VERSION_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PROVISION_APPLICATION_TYPE_DESCRIPTION {
    pub BuildPath: LPCWSTR,
    pub Async: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PROVISION_APPLICATION_TYPE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PROVISION_APPLICATION_TYPE_DESCRIPTION_BASE {
    pub Kind: FABRIC_PROVISION_APPLICATION_TYPE_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_PROVISION_APPLICATION_TYPE_DESCRIPTION_BASE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PROVISION_APPLICATION_TYPE_DESCRIPTION_EX1 {
    pub ApplicationPackageCleanupPolicy: FABRIC_APPLICATION_PACKAGE_CLEANUP_POLICY,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PROVISION_APPLICATION_TYPE_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_PROVISION_APPLICATION_TYPE_KIND(pub i32);
impl FABRIC_PROVISION_APPLICATION_TYPE_KIND {
    pub const FABRIC_PROVISION_APPLICATION_TYPE_KIND_INVALID: Self = Self(0);
    pub const FABRIC_PROVISION_APPLICATION_TYPE_KIND_IMAGE_STORE_PATH: Self = Self(1);
    pub const FABRIC_PROVISION_APPLICATION_TYPE_KIND_EXTERNAL_STORE: Self = Self(2);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PUT_CUSTOM_PROPERTY_OPERATION {
    pub PropertyName: LPCWSTR,
    pub PropertyTypeId: FABRIC_PROPERTY_TYPE_ID,
    pub PropertyValue: *mut core::ffi::c_void,
    pub PropertyCustomTypeId: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PUT_CUSTOM_PROPERTY_OPERATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_PUT_PROPERTY_OPERATION {
    pub PropertyName: LPCWSTR,
    pub PropertyTypeId: FABRIC_PROPERTY_TYPE_ID,
    pub PropertyValue: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_PUT_PROPERTY_OPERATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_QUERY_NODE_STATUS(pub i32);
impl FABRIC_QUERY_NODE_STATUS {
    pub const FABRIC_QUERY_NODE_STATUS_INVALID: Self = Self(0);
    pub const FABRIC_QUERY_NODE_STATUS_UP: Self = Self(1);
    pub const FABRIC_QUERY_NODE_STATUS_DOWN: Self = Self(2);
    pub const FABRIC_QUERY_NODE_STATUS_ENABLING: Self = Self(3);
    pub const FABRIC_QUERY_NODE_STATUS_DISABLING: Self = Self(4);
    pub const FABRIC_QUERY_NODE_STATUS_DISABLED: Self = Self(5);
    pub const FABRIC_QUERY_NODE_STATUS_UNKNOWN: Self = Self(6);
    pub const FABRIC_QUERY_NODE_STATUS_REMOVED: Self = Self(7);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_QUERY_NODE_STATUS_FILTER(pub i32);
impl FABRIC_QUERY_NODE_STATUS_FILTER {
    pub const FABRIC_QUERY_NODE_STATUS_FILTER_DEFAULT: Self = Self(0);
    pub const FABRIC_QUERY_NODE_STATUS_FILTER_ALL: Self = Self(65535);
    pub const FABRIC_QUERY_NODE_STATUS_FILTER_UP: Self = Self(1);
    pub const FABRIC_QUERY_NODE_STATUS_FILTER_DOWN: Self = Self(2);
    pub const FABRIC_QUERY_NODE_STATUS_FILTER_ENABLING: Self = Self(4);
    pub const FABRIC_QUERY_NODE_STATUS_FILTER_DISABLING: Self = Self(8);
    pub const FABRIC_QUERY_NODE_STATUS_FILTER_DISABLED: Self = Self(16);
    pub const FABRIC_QUERY_NODE_STATUS_FILTER_UNKNOWN: Self = Self(32);
    pub const FABRIC_QUERY_NODE_STATUS_FILTER_REMOVED: Self = Self(64);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_QUERY_PAGING_DESCRIPTION {
    pub ContinuationToken: LPCWSTR,
    pub MaxResults: i32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_QUERY_PAGING_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_QUERY_REPLICATOR_OPERATION_NAME(pub i32);
impl FABRIC_QUERY_REPLICATOR_OPERATION_NAME {
    pub const FABRIC_QUERY_REPLICATOR_OPERATION_NAME_INVALID: Self = Self(0);
    pub const FABRIC_QUERY_REPLICATOR_OPERATION_NAME_NONE: Self = Self(1);
    pub const FABRIC_QUERY_REPLICATOR_OPERATION_NAME_OPEN: Self = Self(2);
    pub const FABRIC_QUERY_REPLICATOR_OPERATION_NAME_CHANGEROLE: Self = Self(4);
    pub const FABRIC_QUERY_REPLICATOR_OPERATION_NAME_UPDATEEPOCH: Self = Self(8);
    pub const FABRIC_QUERY_REPLICATOR_OPERATION_NAME_CLOSE: Self = Self(16);
    pub const FABRIC_QUERY_REPLICATOR_OPERATION_NAME_ABORT: Self = Self(32);
    pub const FABRIC_QUERY_REPLICATOR_OPERATION_NAME_ONDATALOSS: Self = Self(64);
    pub const FABRIC_QUERY_REPLICATOR_OPERATION_NAME_WAITFORCATCHUP: Self = Self(128);
    pub const FABRIC_QUERY_REPLICATOR_OPERATION_NAME_BUILD: Self = Self(256);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_QUERY_SERVICE_OPERATION_NAME(pub i32);
impl FABRIC_QUERY_SERVICE_OPERATION_NAME {
    pub const FABRIC_QUERY_SERVICE_OPERATION_NAME_INVALID: Self = Self(0);
    pub const FABRIC_QUERY_SERVICE_OPERATION_NAME_NONE: Self = Self(1);
    pub const FABRIC_QUERY_SERVICE_OPERATION_NAME_OPEN: Self = Self(2);
    pub const FABRIC_QUERY_SERVICE_OPERATION_NAME_CHANGEROLE: Self = Self(4);
    pub const FABRIC_QUERY_SERVICE_OPERATION_NAME_CLOSE: Self = Self(8);
    pub const FABRIC_QUERY_SERVICE_OPERATION_NAME_ABORT: Self = Self(16);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_QUERY_SERVICE_PARTITION_STATUS(pub i32);
impl FABRIC_QUERY_SERVICE_PARTITION_STATUS {
    pub const FABRIC_QUERY_SERVICE_PARTITION_STATUS_INVALID: Self = Self(0);
    pub const FABRIC_QUERY_SERVICE_PARTITION_STATUS_READY: Self = Self(1);
    pub const FABRIC_QUERY_SERVICE_PARTITION_STATUS_NOT_READY: Self = Self(2);
    pub const FABRIC_QUERY_SERVICE_PARTITION_STATUS_IN_QUORUM_LOSS: Self = Self(3);
    pub const FABRIC_QUERY_SERVICE_PARTITION_STATUS_RECONFIGURING: Self = Self(4);
    pub const FABRIC_QUERY_SERVICE_PARTITION_STATUS_DELETING: Self = Self(5);
    pub const FABRIC_QUERY_SERVICE_PARTITION_STATUS_DISABLING: Self = Self(6);
    pub const FABRIC_QUERY_SERVICE_PARTITION_STATUS_DISABLED: Self = Self(7);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_QUERY_SERVICE_REPLICA_STATUS(pub i32);
impl FABRIC_QUERY_SERVICE_REPLICA_STATUS {
    pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_INVALID: Self = Self(0);
    pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_INBUILD: Self = Self(1);
    pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_STANDBY: Self = Self(2);
    pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_READY: Self = Self(3);
    pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_DOWN: Self = Self(4);
    pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_DROPPED: Self = Self(5);
    pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_COMPLETED: Self = Self(6);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER(pub i32);
impl FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER {
    pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER_DEFAULT: Self = Self(0);
    pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER_ALL: Self = Self(65535);
    pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER_INBUILD: Self = Self(1);
    pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER_STANDBY: Self = Self(2);
    pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER_READY: Self = Self(4);
    pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER_DOWN: Self = Self(8);
    pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER_DROPPED: Self = Self(16);
    pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER_COMPLETED: Self = Self(32);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_QUERY_SERVICE_STATUS(pub i32);
impl FABRIC_QUERY_SERVICE_STATUS {
    pub const FABRIC_QUERY_SERVICE_STATUS_UNKNOWN: Self = Self(0);
    pub const FABRIC_QUERY_SERVICE_STATUS_ACTIVE: Self = Self(1);
    pub const FABRIC_QUERY_SERVICE_STATUS_UPGRADING: Self = Self(2);
    pub const FABRIC_QUERY_SERVICE_STATUS_DELETING: Self = Self(3);
    pub const FABRIC_QUERY_SERVICE_STATUS_CREATING: Self = Self(4);
    pub const FABRIC_QUERY_SERVICE_STATUS_FAILED: Self = Self(5);
    pub const FABRIC_QUERY_SERVICE_STATUS_DISABLING: Self = Self(6);
    pub const FABRIC_QUERY_SERVICE_STATUS_DISABLED: Self = Self(7);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_QUORUM_LOSS_MODE(pub i32);
impl FABRIC_QUORUM_LOSS_MODE {
    pub const FABRIC_QUORUM_LOSS_MODE_INVALID: Self = Self(0);
    pub const FABRIC_QUORUM_LOSS_MODE_QUORUM_REPLICAS: Self = Self(1);
    pub const FABRIC_QUORUM_LOSS_MODE_ALL_REPLICAS: Self = Self(2);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_RECONFIGURATION_INFORMATION_QUERY_RESULT {
    pub PreviousConfigurationRole: FABRIC_REPLICA_ROLE,
    pub ReconfigurationPhase: FABRIC_RECONFIGURATION_PHASE,
    pub ReconfigurationType: FABRIC_RECONFIGURATION_TYPE,
    pub ReconfigurationStartTimeUtc: FILETIME,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_RECONFIGURATION_INFORMATION_QUERY_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_RECONFIGURATION_PHASE(pub i32);
impl FABRIC_RECONFIGURATION_PHASE {
    pub const FABRIC_RECONFIGURATION_PHASE_INVALID: Self = Self(0);
    pub const FABRIC_RECONFIGURATION_PHASE_NONE: Self = Self(1);
    pub const FABRIC_RECONFIGURATION_PHASE_ZERO: Self = Self(2);
    pub const FABRIC_RECONFIGURATION_PHASE_ONE: Self = Self(3);
    pub const FABRIC_RECONFIGURATION_PHASE_TWO: Self = Self(4);
    pub const FABRIC_RECONFIGURATION_PHASE_THREE: Self = Self(5);
    pub const FABRIC_RECONFIGURATION_PHASE_FOUR: Self = Self(6);
    pub const FABRIC_RECONFIGURATION_ABORT_PHASE_ZERO: Self = Self(7);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_RECONFIGURATION_TYPE(pub i32);
impl FABRIC_RECONFIGURATION_TYPE {
    pub const FABRIC_RECONFIGURATION_TYPE_INVALID: Self = Self(0);
    pub const FABRIC_RECONFIGURATION_TYPE_SWAPPRIMARY: Self = Self(1);
    pub const FABRIC_RECONFIGURATION_TYPE_FAILOVER: Self = Self(2);
    pub const FABRIC_RECONFIGURATION_TYPE_OTHER: Self = Self(3);
    pub const FABRIC_RECONFIGURATION_TYPE_NONE: Self = Self(4);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REMOTE_REPLICATOR_ACKNOWLEDGEMENT_DETAIL {
    pub AverageReceiveDurationMilliseconds: i64,
    pub AverageApplyDurationMilliseconds: i64,
    pub NotReceivedCount: i64,
    pub ReceivedAndNotAppliedCount: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REMOTE_REPLICATOR_ACKNOWLEDGEMENT_DETAIL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REMOTE_REPLICATOR_ACKNOWLEDGEMENT_STATUS {
    pub CopyStreamAcknowledgementDetails: *mut FABRIC_REMOTE_REPLICATOR_ACKNOWLEDGEMENT_DETAIL,
    pub ReplicationStreamAcknowledgementDetails:
        *mut FABRIC_REMOTE_REPLICATOR_ACKNOWLEDGEMENT_DETAIL,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REMOTE_REPLICATOR_ACKNOWLEDGEMENT_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REMOTE_REPLICATOR_STATUS {
    pub ReplicaId: i64,
    pub LastAcknowledgementProcessedTimeUtc: FILETIME,
    pub LastReceivedReplicationSequenceNumber: i64,
    pub LastAppliedReplicationSequenceNumber: i64,
    pub IsInBuild: bool,
    pub LastReceivedCopySequenceNumber: i64,
    pub LastAppliedCopySequenceNumber: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REMOTE_REPLICATOR_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REMOTE_REPLICATOR_STATUS_LIST {
    pub Count: u32,
    pub Items: *mut FABRIC_REMOTE_REPLICATOR_STATUS,
}
impl Default for FABRIC_REMOTE_REPLICATOR_STATUS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REMOVE_REPLICA_DESCRIPTION {
    pub NodeName: LPCWSTR,
    pub PartitionId: FABRIC_PARTITION_ID,
    pub ReplicaOrInstanceId: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REMOVE_REPLICA_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REMOVE_REPLICA_DESCRIPTION_EX1 {
    pub ForceRemove: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REMOVE_REPLICA_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPAIR_APPROVE_DESCRIPTION {
    pub Scope: *mut FABRIC_REPAIR_SCOPE_IDENTIFIER,
    pub RepairTaskId: LPCWSTR,
    pub Version: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPAIR_APPROVE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPAIR_CANCEL_DESCRIPTION {
    pub Scope: *mut FABRIC_REPAIR_SCOPE_IDENTIFIER,
    pub RepairTaskId: LPCWSTR,
    pub Version: i64,
    pub RequestAbort: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPAIR_CANCEL_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPAIR_DELETE_DESCRIPTION {
    pub Scope: *mut FABRIC_REPAIR_SCOPE_IDENTIFIER,
    pub RepairTaskId: LPCWSTR,
    pub Version: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPAIR_DELETE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPAIR_EXECUTOR_STATE {
    pub Executor: LPCWSTR,
    pub ExecutorData: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPAIR_EXECUTOR_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPAIR_IMPACT_DESCRIPTION {
    pub Kind: FABRIC_REPAIR_IMPACT_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPAIR_IMPACT_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_REPAIR_IMPACT_KIND(pub i32);
impl FABRIC_REPAIR_IMPACT_KIND {
    pub const FABRIC_REPAIR_IMPACT_KIND_INVALID: Self = Self(0);
    pub const FABRIC_REPAIR_IMPACT_KIND_NODE: Self = Self(1);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPAIR_NODE_IMPACT {
    pub NodeName: LPCWSTR,
    pub ImpactLevel: FABRIC_REPAIR_NODE_IMPACT_LEVEL,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPAIR_NODE_IMPACT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_REPAIR_NODE_IMPACT_LEVEL(pub i32);
impl FABRIC_REPAIR_NODE_IMPACT_LEVEL {
    pub const FABRIC_REPAIR_NODE_IMPACT_LEVEL_INVALID: Self = Self(0);
    pub const FABRIC_REPAIR_NODE_IMPACT_LEVEL_NONE: Self = Self(1);
    pub const FABRIC_REPAIR_NODE_IMPACT_LEVEL_RESTART: Self = Self(2);
    pub const FABRIC_REPAIR_NODE_IMPACT_LEVEL_REMOVE_DATA: Self = Self(3);
    pub const FABRIC_REPAIR_NODE_IMPACT_LEVEL_REMOVE_NODE: Self = Self(4);
    pub const FABRIC_REPAIR_NODE_IMPACT_LEVEL_PAUSE: Self = Self(5);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPAIR_NODE_IMPACT_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_REPAIR_NODE_IMPACT,
}
impl Default for FABRIC_REPAIR_NODE_IMPACT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPAIR_RESULT_DESCRIPTION {
    pub ResultStatus: FABRIC_REPAIR_TASK_RESULT,
    pub ResultCode: windows_core::HRESULT,
    pub ResultDetails: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPAIR_RESULT_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPAIR_SCOPE_IDENTIFIER {
    pub Kind: FABRIC_REPAIR_SCOPE_IDENTIFIER_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPAIR_SCOPE_IDENTIFIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_REPAIR_SCOPE_IDENTIFIER_KIND(pub i32);
impl FABRIC_REPAIR_SCOPE_IDENTIFIER_KIND {
    pub const FABRIC_REPAIR_SCOPE_IDENTIFIER_KIND_INVALID: Self = Self(0);
    pub const FABRIC_REPAIR_SCOPE_IDENTIFIER_KIND_CLUSTER: Self = Self(1);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPAIR_TARGET_DESCRIPTION {
    pub Kind: FABRIC_REPAIR_TARGET_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPAIR_TARGET_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_REPAIR_TARGET_KIND(pub i32);
impl FABRIC_REPAIR_TARGET_KIND {
    pub const FABRIC_REPAIR_TARGET_KIND_INVALID: Self = Self(0);
    pub const FABRIC_REPAIR_TARGET_KIND_NODE: Self = Self(1);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPAIR_TASK {
    pub Scope: *const FABRIC_REPAIR_SCOPE_IDENTIFIER,
    pub TaskId: LPCWSTR,
    pub Version: i64,
    pub Description: LPCWSTR,
    pub State: FABRIC_REPAIR_TASK_STATE,
    pub Flags: u32,
    pub Action: LPCWSTR,
    pub Target: *const FABRIC_REPAIR_TARGET_DESCRIPTION,
    pub ExecutorState: *mut FABRIC_REPAIR_EXECUTOR_STATE,
    pub Impact: *mut FABRIC_REPAIR_IMPACT_DESCRIPTION,
    pub Result: *mut FABRIC_REPAIR_RESULT_DESCRIPTION,
    pub History: *const FABRIC_REPAIR_TASK_HISTORY,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPAIR_TASK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPAIR_TASK_EX1 {
    pub PerformPreparingHealthCheck: bool,
    pub PerformRestoringHealthCheck: bool,
    pub PreparingHealthCheckState: FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE,
    pub RestoringHealthCheckState: FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPAIR_TASK_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_REPAIR_TASK_FLAGS(pub i32);
impl FABRIC_REPAIR_TASK_FLAGS {
    pub const FABRIC_REPAIR_TASK_FLAGS_NONE: Self = Self(0);
    pub const FABRIC_REPAIR_TASK_FLAGS_CANCEL_REQUESTED: Self = Self(1);
    pub const FABRIC_REPAIR_TASK_FLAGS_ABORT_REQUESTED: Self = Self(2);
    pub const FABRIC_REPAIR_TASK_FLAGS_FORCED_APPROVAL: Self = Self(4);
    pub const FABRIC_REPAIR_TASK_FLAGS_VALID_MASK: Self = Self(7);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE(pub i32);
impl FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE {
    pub const FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE_NOT_STARTED: Self = Self(0);
    pub const FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE_IN_PROGRESS: Self = Self(1);
    pub const FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE_SUCCEEDED: Self = Self(2);
    pub const FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE_SKIPPED: Self = Self(3);
    pub const FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE_TIMEDOUT: Self = Self(4);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPAIR_TASK_HEALTH_POLICY_UPDATE_DESCRIPTION {
    pub Scope: *mut FABRIC_REPAIR_SCOPE_IDENTIFIER,
    pub RepairTaskId: LPCWSTR,
    pub Version: i64,
    pub Flags: u32,
    pub PerformPreparingHealthCheck: bool,
    pub PerformRestoringHealthCheck: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPAIR_TASK_HEALTH_POLICY_UPDATE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_REPAIR_TASK_HEALTH_POLICY_UPDATE_SETTINGS_FLAGS(pub i32);
impl FABRIC_REPAIR_TASK_HEALTH_POLICY_UPDATE_SETTINGS_FLAGS {
    pub const FABRIC_REPAIR_TASK_HEALTH_POLICY_UPDATE_SETTINGS_NONE: Self = Self(0);
    pub const FABRIC_REPAIR_TASK_HEALTH_POLICY_UPDATE_SETTINGS_HONOR_PERFORM_PREPARING_HEALTH_CHECK : Self = Self (1) ;
    pub const FABRIC_REPAIR_TASK_HEALTH_POLICY_UPDATE_SETTINGS_HONOR_PERFORM_RESTORING_HEALTH_CHECK : Self = Self (2) ;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPAIR_TASK_HISTORY {
    pub CreatedUtcTimestamp: FILETIME,
    pub ClaimedUtcTimestamp: FILETIME,
    pub PreparingUtcTimestamp: FILETIME,
    pub ApprovedUtcTimestamp: FILETIME,
    pub ExecutingUtcTimestamp: FILETIME,
    pub RestoringUtcTimestamp: FILETIME,
    pub CompletedUtcTimestamp: FILETIME,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPAIR_TASK_HISTORY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPAIR_TASK_HISTORY_EX1 {
    pub PreparingHealthCheckStartUtcTimestamp: FILETIME,
    pub PreparingHealthCheckEndUtcTimestamp: FILETIME,
    pub RestoringHealthCheckStartUtcTimestamp: FILETIME,
    pub RestoringHealthCheckEndUtcTimestamp: FILETIME,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPAIR_TASK_HISTORY_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPAIR_TASK_LIST {
    pub Count: u32,
    pub Items: *mut FABRIC_REPAIR_TASK,
}
impl Default for FABRIC_REPAIR_TASK_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPAIR_TASK_QUERY_DESCRIPTION {
    pub Scope: *mut FABRIC_REPAIR_SCOPE_IDENTIFIER,
    pub TaskIdFilter: LPCWSTR,
    pub StateFilter: u32,
    pub ExecutorFilter: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPAIR_TASK_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_REPAIR_TASK_RESULT(pub i32);
impl FABRIC_REPAIR_TASK_RESULT {
    pub const FABRIC_REPAIR_TASK_RESULT_INVALID: Self = Self(0);
    pub const FABRIC_REPAIR_TASK_RESULT_SUCCEEDED: Self = Self(1);
    pub const FABRIC_REPAIR_TASK_RESULT_CANCELLED: Self = Self(2);
    pub const FABRIC_REPAIR_TASK_RESULT_INTERRUPTED: Self = Self(4);
    pub const FABRIC_REPAIR_TASK_RESULT_FAILED: Self = Self(8);
    pub const FABRIC_REPAIR_TASK_RESULT_PENDING: Self = Self(16);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_REPAIR_TASK_STATE(pub i32);
impl FABRIC_REPAIR_TASK_STATE {
    pub const FABRIC_REPAIR_TASK_STATE_INVALID: Self = Self(0);
    pub const FABRIC_REPAIR_TASK_STATE_CREATED: Self = Self(1);
    pub const FABRIC_REPAIR_TASK_STATE_CLAIMED: Self = Self(2);
    pub const FABRIC_REPAIR_TASK_STATE_PREPARING: Self = Self(4);
    pub const FABRIC_REPAIR_TASK_STATE_APPROVED: Self = Self(8);
    pub const FABRIC_REPAIR_TASK_STATE_EXECUTING: Self = Self(16);
    pub const FABRIC_REPAIR_TASK_STATE_RESTORING: Self = Self(32);
    pub const FABRIC_REPAIR_TASK_STATE_COMPLETED: Self = Self(64);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_REPAIR_TASK_STATE_FILTER(pub i32);
impl FABRIC_REPAIR_TASK_STATE_FILTER {
    pub const FABRIC_REPAIR_TASK_STATE_FILTER_DEFAULT: Self = Self(0);
    pub const FABRIC_REPAIR_TASK_STATE_FILTER_CREATED: Self = Self(1);
    pub const FABRIC_REPAIR_TASK_STATE_FILTER_CLAIMED: Self = Self(2);
    pub const FABRIC_REPAIR_TASK_STATE_FILTER_PREPARING: Self = Self(4);
    pub const FABRIC_REPAIR_TASK_STATE_FILTER_APPROVED: Self = Self(8);
    pub const FABRIC_REPAIR_TASK_STATE_FILTER_EXECUTING: Self = Self(16);
    pub const FABRIC_REPAIR_TASK_STATE_FILTER_RESTORING: Self = Self(32);
    pub const FABRIC_REPAIR_TASK_STATE_FILTER_COMPLETED: Self = Self(64);
    pub const FABRIC_REPAIR_TASK_STATE_FILTER_READY_TO_EXECUTE: Self = Self(24);
    pub const FABRIC_REPAIR_TASK_STATE_FILTER_ACTIVE: Self = Self(63);
    pub const FABRIC_REPAIR_TASK_STATE_FILTER_ALL: Self = Self(127);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICAS_HEALTH_EVALUATION {
    pub Description: LPCWSTR,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub TotalCount: u32,
    pub MaxPercentUnhealthyReplicasPerPartition: u8,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPLICAS_HEALTH_EVALUATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICATOR_QUEUE_STATUS {
    pub QueueUtilizationPercentage: u32,
    pub QueueMemorySize: i64,
    pub FirstSequenceNumber: i64,
    pub CompletedSequenceNumber: i64,
    pub CommittedSequenceNumber: i64,
    pub LastSequenceNumber: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPLICATOR_QUEUE_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICATOR_SETTINGS {
    pub Flags: u32,
    pub RetryIntervalMilliseconds: u32,
    pub BatchAcknowledgementIntervalMilliseconds: u32,
    pub ReplicatorAddress: LPCWSTR,
    pub RequireServiceAck: bool,
    pub InitialReplicationQueueSize: u32,
    pub MaxReplicationQueueSize: u32,
    pub InitialCopyQueueSize: u32,
    pub MaxCopyQueueSize: u32,
    pub SecurityCredentials: *const FABRIC_SECURITY_CREDENTIALS,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPLICATOR_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICATOR_SETTINGS_EX1 {
    pub MaxReplicationQueueMemorySize: u32,
    pub SecondaryClearAcknowledgedOperations: bool,
    pub MaxReplicationMessageSize: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPLICATOR_SETTINGS_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICATOR_SETTINGS_EX2 {
    pub UseStreamFaultsAndEndOfStreamOperationAck: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPLICATOR_SETTINGS_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICATOR_SETTINGS_EX3 {
    pub InitialPrimaryReplicationQueueSize: u32,
    pub MaxPrimaryReplicationQueueSize: u32,
    pub MaxPrimaryReplicationQueueMemorySize: u32,
    pub InitialSecondaryReplicationQueueSize: u32,
    pub MaxSecondaryReplicationQueueSize: u32,
    pub MaxSecondaryReplicationQueueMemorySize: u32,
    pub PrimaryWaitForPendingQuorumsTimeoutMilliseconds: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPLICATOR_SETTINGS_EX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICATOR_SETTINGS_EX4 {
    pub ReplicatorListenAddress: LPCWSTR,
    pub ReplicatorPublishAddress: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPLICATOR_SETTINGS_EX4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICATOR_SETTINGS_EX5 {
    pub EnableSendWindowSizeInBytes: bool,
    pub MaxReplicationQueueSendWindowSizeInBytes: u32,
    pub MaxCopyQueueSendWindowSizeInBytes: u32,
    pub UseIndividualHeapPerReplica: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPLICATOR_SETTINGS_EX5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICATOR_SETTINGS_EX6 {
    pub InitialReplicaHeapSizeInKB: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPLICATOR_SETTINGS_EX6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICATOR_SETTINGS_EX7 {
    pub ReplicationBatchSize: u32,
    pub ReplicationBatchSendIntervalMilliseconds: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPLICATOR_SETTINGS_EX7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_REPLICATOR_SETTINGS_FLAGS(pub i32);
impl FABRIC_REPLICATOR_SETTINGS_FLAGS {
    pub const FABRIC_REPLICATOR_SETTINGS_NONE: Self = Self(0);
    pub const FABRIC_REPLICATOR_ADDRESS: Self = Self(1);
    pub const FABRIC_REPLICATOR_SECURITY: Self = Self(2);
    pub const FABRIC_REPLICATOR_RETRY_INTERVAL: Self = Self(4);
    pub const FABRIC_REPLICATOR_BATCH_ACKNOWLEDGEMENT_INTERVAL: Self = Self(8);
    pub const FABRIC_REPLICATOR_REQUIRE_SERVICE_ACK: Self = Self(16);
    pub const FABRIC_REPLICATOR_REPLICATION_QUEUE_INITIAL_SIZE: Self = Self(32);
    pub const FABRIC_REPLICATOR_REPLICATION_QUEUE_MAX_SIZE: Self = Self(64);
    pub const FABRIC_REPLICATOR_COPY_QUEUE_INITIAL_SIZE: Self = Self(128);
    pub const FABRIC_REPLICATOR_COPY_QUEUE_MAX_SIZE: Self = Self(256);
    pub const FABRIC_REPLICATOR_REPLICATION_QUEUE_MAX_MEMORY_SIZE: Self = Self(512);
    pub const FABRIC_REPLICATOR_SECONDARY_CLEAR_ACKNOWLEDGED_OPERATIONS: Self = Self(1024);
    pub const FABRIC_REPLICATOR_REPLICATION_MESSAGE_MAX_SIZE: Self = Self(2048);
    pub const FABRIC_REPLICATOR_USE_STREAMFAULTS_AND_ENDOFSTREAM_OPERATIONACK: Self = Self(4096);
    pub const FABRIC_REPLICATOR_SECONDARY_REPLICATION_QUEUE_INITIAL_SIZE: Self = Self(8192);
    pub const FABRIC_REPLICATOR_SECONDARY_REPLICATION_QUEUE_MAX_SIZE: Self = Self(16384);
    pub const FABRIC_REPLICATOR_SECONDARY_REPLICATION_QUEUE_MAX_MEMORY_SIZE: Self = Self(32768);
    pub const FABRIC_REPLICATOR_PRIMARY_REPLICATION_QUEUE_INITIAL_SIZE: Self = Self(65536);
    pub const FABRIC_REPLICATOR_PRIMARY_REPLICATION_QUEUE_MAX_SIZE: Self = Self(131072);
    pub const FABRIC_REPLICATOR_PRIMARY_REPLICATION_QUEUE_MAX_MEMORY_SIZE: Self = Self(262144);
    pub const FABRIC_REPLICATOR_PRIMARY_WAIT_FOR_PENDING_QUORUMS_TIMEOUT: Self = Self(524288);
    pub const FABRIC_REPLICATOR_LISTEN_ADDRESS: Self = Self(1048576);
    pub const FABRIC_REPLICATOR_PUBLISH_ADDRESS: Self = Self(2097152);
    pub const FABRIC_REPLICATOR_ENABLE_SEND_WINDOW_SIZE_IN_BYTES: Self = Self(4194304);
    pub const FABRIC_REPLICATOR_USE_INDIVIDUAL_HEAP_PER_REPLICA: Self = Self(8388608);
    pub const FABRIC_REPLICATOR_INITIAL_REPLICA_HEAP_SIZE_IN_KB: Self = Self(16777216);
    pub const FABRIC_REPLICATOR_REPLICATION_BATCH_SIZE: Self = Self(33554432);
    pub const FABRIC_REPLICATOR_REPLICATION_BATCH_SEND_INTERVAL: Self = Self(67108864);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICATOR_STATUS_QUERY_RESULT {
    pub Role: FABRIC_REPLICA_ROLE,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPLICATOR_STATUS_QUERY_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICA_HEALTH {
    pub Kind: FABRIC_SERVICE_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPLICA_HEALTH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICA_HEALTH_EVALUATION {
    pub Description: LPCWSTR,
    pub PartitionId: FABRIC_PARTITION_ID,
    pub ReplicaOrInstanceId: i64,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPLICA_HEALTH_EVALUATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICA_HEALTH_QUERY_DESCRIPTION {
    pub PartitionId: FABRIC_PARTITION_ID,
    pub ReplicaOrInstanceId: i64,
    pub HealthPolicy: *const FABRIC_APPLICATION_HEALTH_POLICY,
    pub EventsFilter: *const FABRIC_HEALTH_EVENTS_FILTER,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPLICA_HEALTH_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICA_HEALTH_STATE {
    pub Kind: FABRIC_SERVICE_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPLICA_HEALTH_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICA_HEALTH_STATES_FILTER {
    pub HealthStateFilter: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPLICA_HEALTH_STATES_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICA_HEALTH_STATE_CHUNK {
    pub ReplicaOrInstanceId: i64,
    pub HealthState: FABRIC_HEALTH_STATE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPLICA_HEALTH_STATE_CHUNK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICA_HEALTH_STATE_CHUNK_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_REPLICA_HEALTH_STATE_CHUNK,
    pub TotalCount: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPLICA_HEALTH_STATE_CHUNK_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICA_HEALTH_STATE_FILTER {
    pub HealthStateFilter: u32,
    pub ReplicaOrInstanceIdFilter: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPLICA_HEALTH_STATE_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICA_HEALTH_STATE_FILTER_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_REPLICA_HEALTH_STATE_FILTER,
}
impl Default for FABRIC_REPLICA_HEALTH_STATE_FILTER_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICA_HEALTH_STATE_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_REPLICA_HEALTH_STATE,
}
impl Default for FABRIC_REPLICA_HEALTH_STATE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FABRIC_REPLICA_ID(pub i64);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICA_INFORMATION {
    pub Id: i64,
    pub Role: FABRIC_REPLICA_ROLE,
    pub Status: FABRIC_REPLICA_STATUS,
    pub ReplicatorAddress: LPCWSTR,
    pub CurrentProgress: i64,
    pub CatchUpCapability: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPLICA_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICA_INFORMATION_EX1 {
    pub MustCatchup: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPLICA_INFORMATION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICA_LOAD_INFORMATION {
    pub PartitionId: FABRIC_PARTITION_ID,
    pub ReplicaOrInstanceId: i64,
    pub LoadMetricReports: *mut FABRIC_LOAD_METRIC_REPORT_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPLICA_LOAD_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICA_LOAD_INFORMATION_QUERY_DESCRIPTION {
    pub PartitionId: FABRIC_PARTITION_ID,
    pub ReplicaOrInstanceId: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPLICA_LOAD_INFORMATION_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICA_METRIC_LOAD_DESCRIPTION {
    pub NodeName: LPCWSTR,
    pub ReplicaOrInstanceLoadEntries: *mut FABRIC_METRIC_LOAD_DESCRIPTION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPLICA_METRIC_LOAD_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICA_METRIC_LOAD_DESCRIPTION_LIST {
    pub Count: u32,
    pub Items: *mut FABRIC_REPLICA_METRIC_LOAD_DESCRIPTION,
}
impl Default for FABRIC_REPLICA_METRIC_LOAD_DESCRIPTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICA_MOVE_COST_DESCRIPTION {
    pub NodeName: LPCWSTR,
    pub ReplicaOrInstanceMoveCostEntry: FABRIC_MOVE_COST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPLICA_MOVE_COST_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICA_MOVE_COST_DESCRIPTION_LIST {
    pub Count: u32,
    pub Items: *mut FABRIC_REPLICA_MOVE_COST_DESCRIPTION,
}
impl Default for FABRIC_REPLICA_MOVE_COST_DESCRIPTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_REPLICA_OPEN_MODE(pub i32);
impl FABRIC_REPLICA_OPEN_MODE {
    pub const FABRIC_REPLICA_OPEN_MODE_INVALID: Self = Self(0);
    pub const FABRIC_REPLICA_OPEN_MODE_NEW: Self = Self(1);
    pub const FABRIC_REPLICA_OPEN_MODE_EXISTING: Self = Self(2);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_REPLICA_ROLE(pub i32);
impl FABRIC_REPLICA_ROLE {
    pub const FABRIC_REPLICA_ROLE_UNKNOWN: Self = Self(0);
    pub const FABRIC_REPLICA_ROLE_NONE: Self = Self(1);
    pub const FABRIC_REPLICA_ROLE_PRIMARY: Self = Self(2);
    pub const FABRIC_REPLICA_ROLE_IDLE_SECONDARY: Self = Self(3);
    pub const FABRIC_REPLICA_ROLE_ACTIVE_SECONDARY: Self = Self(4);
    pub const FABRIC_REPLICA_ROLE_IDLE_AUXILIARY: Self = Self(5);
    pub const FABRIC_REPLICA_ROLE_ACTIVE_AUXILIARY: Self = Self(6);
    pub const FABRIC_REPLICA_ROLE_PRIMARY_AUXILIARY: Self = Self(7);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICA_SET_CONFIGURATION {
    pub ReplicaCount: u32,
    pub Replicas: *const FABRIC_REPLICA_INFORMATION,
    pub WriteQuorum: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPLICA_SET_CONFIGURATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_REPLICA_SET_QUORUM_MODE(pub i32);
impl FABRIC_REPLICA_SET_QUORUM_MODE {
    pub const FABRIC_REPLICA_SET_QUORUM_INVALID: Self = Self(0);
    pub const FABRIC_REPLICA_SET_WRITE_QUORUM: Self = Self(1);
    pub const FABRIC_REPLICA_SET_QUORUM_ALL: Self = Self(2);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_REPLICA_STATUS(pub i32);
impl FABRIC_REPLICA_STATUS {
    pub const FABRIC_REPLICA_STATUS_INVALID: Self = Self(0);
    pub const FABRIC_REPLICA_STATUS_DOWN: Self = Self(1);
    pub const FABRIC_REPLICA_STATUS_UP: Self = Self(2);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_REPLICA_STATUS_QUERY_RESULT {
    pub Kind: FABRIC_SERVICE_REPLICA_KIND,
    pub Value: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_REPLICA_STATUS_QUERY_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_RESOLVED_SERVICE_ENDPOINT {
    pub Address: LPCWSTR,
    pub Role: FABRIC_SERVICE_ENDPOINT_ROLE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_RESOLVED_SERVICE_ENDPOINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_RESOLVED_SERVICE_PARTITION {
    pub Info: FABRIC_SERVICE_PARTITION_INFORMATION,
    pub EndpointCount: u32,
    pub Endpoints: *mut FABRIC_RESOLVED_SERVICE_ENDPOINT,
    pub ServiceName: FABRIC_URI,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_RESOLVED_SERVICE_PARTITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION {
    pub NodeName: LPCWSTR,
    pub ApplicationName: FABRIC_URI,
    pub ServiceManifestName: LPCWSTR,
    pub CodePackageName: LPCWSTR,
    pub CodePackageInstanceId: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION2 {
    pub Kind: FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION_KIND(pub i32);
impl FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION_KIND {
    pub const FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION_KIND_INVALID: Self = Self(0);
    pub const FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION_KIND_USING_NODE_NAME: Self = Self(1);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION_USING_NODE_NAME {
    pub NodeName: LPCWSTR,
    pub ApplicationName: FABRIC_URI,
    pub ServiceManifestName: LPCWSTR,
    pub CodePackageName: LPCWSTR,
    pub CodePackageInstanceId: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION_USING_NODE_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION_USING_NODE_NAME_EX1 {
    pub ServicePackageActivationId: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION_USING_NODE_NAME_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_RESTART_NODE_DESCRIPTION {
    pub NodeName: LPCWSTR,
    pub NodeInstanceId: u64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_RESTART_NODE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_RESTART_NODE_DESCRIPTION2 {
    pub Kind: FABRIC_RESTART_NODE_DESCRIPTION_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_RESTART_NODE_DESCRIPTION2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_RESTART_NODE_DESCRIPTION_EX1 {
    pub CreateFabricDump: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_RESTART_NODE_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_RESTART_NODE_DESCRIPTION_KIND(pub i32);
impl FABRIC_RESTART_NODE_DESCRIPTION_KIND {
    pub const FABRIC_RESTART_NODE_DESCRIPTION_KIND_INVALID: Self = Self(0);
    pub const FABRIC_RESTART_NODE_DESCRIPTION_KIND_USING_NODE_NAME: Self = Self(1);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_RESTART_NODE_DESCRIPTION_USING_NODE_NAME {
    pub NodeName: LPCWSTR,
    pub NodeInstanceId: u64,
    pub ShouldCreateFabricDump: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_RESTART_NODE_DESCRIPTION_USING_NODE_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_RESTART_NODE_STATUS {
    pub NodeResult: *mut FABRIC_NODE_RESULT,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_RESTART_NODE_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_RESTART_PARTITION_MODE(pub i32);
impl FABRIC_RESTART_PARTITION_MODE {
    pub const FABRIC_RESTART_PARTITION_MODE_INVALID: Self = Self(0);
    pub const FABRIC_RESTART_PARTITION_MODE_ALL_REPLICAS_OR_INSTANCES: Self = Self(1);
    pub const FABRIC_RESTART_PARTITION_MODE_ONLY_ACTIVE_SECONDARIES: Self = Self(2);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_RESTART_REPLICA_DESCRIPTION {
    pub NodeName: LPCWSTR,
    pub PartitionId: FABRIC_PARTITION_ID,
    pub ReplicaOrInstanceId: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_RESTART_REPLICA_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_ROLLING_UPGRADE_MODE(pub i32);
impl FABRIC_ROLLING_UPGRADE_MODE {
    pub const FABRIC_ROLLING_UPGRADE_MODE_INVALID: Self = Self(0);
    pub const FABRIC_ROLLING_UPGRADE_MODE_UNMONITORED_AUTO: Self = Self(1);
    pub const FABRIC_ROLLING_UPGRADE_MODE_UNMONITORED_MANUAL: Self = Self(2);
    pub const FABRIC_ROLLING_UPGRADE_MODE_MONITORED: Self = Self(3);
    pub const FABRIC_ROLLING_UPGRADE_MODE_UNMONITORED_DEFERRED: Self = Self(4);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_ROLLING_UPGRADE_MONITORING_POLICY {
    pub FailureAction: FABRIC_MONITORED_UPGRADE_FAILURE_ACTION,
    pub HealthCheckWaitDurationInSeconds: u32,
    pub HealthCheckRetryTimeoutInSeconds: u32,
    pub UpgradeTimeoutInSeconds: u32,
    pub UpgradeDomainTimeoutInSeconds: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_ROLLING_UPGRADE_MONITORING_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_ROLLING_UPGRADE_MONITORING_POLICY_EX1 {
    pub HealthCheckStableDurationInSeconds: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_ROLLING_UPGRADE_MONITORING_POLICY_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_ROLLING_UPGRADE_POLICY_DESCRIPTION {
    pub RollingUpgradeMode: FABRIC_ROLLING_UPGRADE_MODE,
    pub ForceRestart: bool,
    pub UpgradeReplicaSetCheckTimeoutInSeconds: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_ROLLING_UPGRADE_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_ROLLING_UPGRADE_POLICY_DESCRIPTION_EX1 {
    pub MonitoringPolicy: *const FABRIC_ROLLING_UPGRADE_MONITORING_POLICY,
    pub HealthPolicy: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_ROLLING_UPGRADE_POLICY_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_ROLLING_UPGRADE_POLICY_DESCRIPTION_EX2 {
    pub EnableDeltaHealthEvaluation: bool,
    pub UpgradeHealthPolicy: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_ROLLING_UPGRADE_POLICY_DESCRIPTION_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_ROLLING_UPGRADE_POLICY_DESCRIPTION_EX3 {
    pub ApplicationHealthPolicyMap: *const FABRIC_APPLICATION_HEALTH_POLICY_MAP,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_ROLLING_UPGRADE_POLICY_DESCRIPTION_EX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_ROLLING_UPGRADE_POLICY_DESCRIPTION_EX4 {
    pub UpgradeSortOrder: FABRIC_UPGRADE_SORT_ORDER,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_ROLLING_UPGRADE_POLICY_DESCRIPTION_EX4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_ROLLING_UPGRADE_POLICY_DESCRIPTION_EX5 {
    pub InstanceCloseDelayDurationInSeconds: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_ROLLING_UPGRADE_POLICY_DESCRIPTION_EX5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS(pub i32);
impl FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS {
    pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_NONE: Self = Self(0);
    pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_UPGRADE_MODE: Self = Self(1);
    pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_FORCE_RESTART: Self = Self(2);
    pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_REPLICA_SET_CHECK_TIMEOUT: Self = Self(4);
    pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_FAILURE_ACTION: Self = Self(8);
    pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_HEALTH_CHECK_WAIT: Self = Self(16);
    pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_HEALTH_CHECK_STABLE: Self = Self(32);
    pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_HEALTH_CHECK_RETRY: Self = Self(64);
    pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_UPGRADE_TIMEOUT: Self = Self(128);
    pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_UPGRADE_DOMAIN_TIMEOUT: Self = Self(256);
    pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_HEALTH_POLICY: Self = Self(512);
    pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_ENABLE_DELTAS: Self = Self(1024);
    pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_UPGRADE_HEALTH_POLICY: Self = Self(2048);
    pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_UPGRADE_APPLICATION_HEALTH_POLICY_MAP: Self =
        Self(4096);
    pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_INSTANCE_CLOSE_DELAY_DURATION: Self = Self(8192);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_RUNAS_POLICY_DESCRIPTION {
    pub UserName: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_RUNAS_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SAFETY_CHECK {
    pub Kind: FABRIC_SAFETY_CHECK_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_SAFETY_CHECK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_SAFETY_CHECK_KIND(pub i32);
impl FABRIC_SAFETY_CHECK_KIND {
    pub const FABRIC_SAFETY_CHECK_KIND_INVALID: Self = Self(0);
    pub const FABRIC_SEED_NODE_SAFETY_CHECK_KIND_ENSURE_QUORUM: Self = Self(1);
    pub const FABRIC_PARTITION_SAFETY_CHECK_KIND_ENSURE_QUORUM: Self = Self(2);
    pub const FABRIC_PARTITION_SAFETY_CHECK_KIND_WAIT_FOR_PRIMARY_PLACEMENT: Self = Self(3);
    pub const FABRIC_PARTITION_SAFETY_CHECK_KIND_WAIT_FOR_PRIMARY_SWAP: Self = Self(4);
    pub const FABRIC_PARTITION_SAFETY_CHECK_KIND_WAIT_FOR_RECONFIGURATION: Self = Self(5);
    pub const FABRIC_PARTITION_SAFETY_CHECK_KIND_WAIT_FOR_INBUILD_REPLICA: Self = Self(6);
    pub const FABRIC_PARTITION_SAFETY_CHECK_KIND_ENSURE_AVAILABILITY: Self = Self(7);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SAFETY_CHECK_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_SAFETY_CHECK,
}
impl Default for FABRIC_SAFETY_CHECK_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SCALING_MECHANISM {
    pub ScalingMechanismKind: FABRIC_SCALING_MECHANISM_KIND,
    pub ScalingMechanismDescription: *mut core::ffi::c_void,
}
impl Default for FABRIC_SCALING_MECHANISM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SCALING_MECHANISM_ADD_REMOVE_INCREMENTAL_NAMED_PARTITION {
    pub MaximumPartitionCount: i32,
    pub MinimumPartitionCount: i32,
    pub ScaleIncrement: i32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SCALING_MECHANISM_ADD_REMOVE_INCREMENTAL_NAMED_PARTITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_SCALING_MECHANISM_KIND(pub i32);
impl FABRIC_SCALING_MECHANISM_KIND {
    pub const FABRIC_SCALING_MECHANISM_INVALID: Self = Self(0);
    pub const FABRIC_SCALING_MECHANISM_KIND_SCALE_PARTITION_INSTANCE_COUNT: Self = Self(1);
    pub const FABRIC_SCALING_MECHANISM_KIND_ADD_REMOVE_INCREMENTAL_NAMED_PARTITION: Self = Self(2);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SCALING_MECHANISM_PARTITION_INSTANCE_COUNT {
    pub MaximumInstanceCount: i32,
    pub MinimumInstanceCount: i32,
    pub ScaleIncrement: i32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SCALING_MECHANISM_PARTITION_INSTANCE_COUNT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SCALING_TRIGGER {
    pub ScalingTriggerKind: FABRIC_SCALING_TRIGGER_KIND,
    pub ScalingTriggerDescription: *mut core::ffi::c_void,
}
impl Default for FABRIC_SCALING_TRIGGER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FABRIC_SCALING_TRIGGER_AVERAGE_PARTITION_LOAD {
    pub MetricName: LPCWSTR,
    pub LowerLoadThreshold: f64,
    pub UpperLoadThreshold: f64,
    pub ScaleIntervalInSeconds: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SCALING_TRIGGER_AVERAGE_PARTITION_LOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FABRIC_SCALING_TRIGGER_AVERAGE_SERVICE_LOAD {
    pub MetricName: LPCWSTR,
    pub LowerLoadThreshold: f64,
    pub UpperLoadThreshold: f64,
    pub ScaleIntervalInSeconds: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SCALING_TRIGGER_AVERAGE_SERVICE_LOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SCALING_TRIGGER_AVERAGE_SERVICE_LOAD_EX1 {
    pub UseOnlyPrimaryLoad: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SCALING_TRIGGER_AVERAGE_SERVICE_LOAD_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_SCALING_TRIGGER_KIND(pub i32);
impl FABRIC_SCALING_TRIGGER_KIND {
    pub const FABRIC_SCALING_TRIGGER_KIND_INVALID: Self = Self(0);
    pub const FABRIC_SCALING_TRIGGER_KIND_AVERAGE_PARTITION_LOAD: Self = Self(1);
    pub const FABRIC_SCALING_TRIGGER_KIND_AVERAGE_SERVICE_LOAD: Self = Self(2);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SECONDARY_REPLICATOR_STATUS_QUERY_RESULT {
    pub ReplicationQueueStatus: *mut FABRIC_REPLICATOR_QUEUE_STATUS,
    pub LastReplicationOperationReceivedTimeUtc: FILETIME,
    pub IsInBuild: bool,
    pub CopyQueueStatus: *mut FABRIC_REPLICATOR_QUEUE_STATUS,
    pub LastCopyOperationReceivedTimeUtc: FILETIME,
    pub LastAcknowledgementSentTimeUtc: FILETIME,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SECONDARY_REPLICATOR_STATUS_QUERY_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_SECRET {
    pub Name: LPCWSTR,
    pub Version: LPCWSTR,
    pub Value: LPCWSTR,
    pub Kind: LPCWSTR,
    pub ContentType: LPCWSTR,
    pub Description: LPCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SECRET_LIST {
    pub Count: u32,
    pub Items: *mut FABRIC_SECRET,
}
impl Default for FABRIC_SECRET_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_SECRET_REFERENCE {
    pub Name: LPCWSTR,
    pub Version: LPCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SECRET_REFERENCE_LIST {
    pub Count: u32,
    pub Items: *mut FABRIC_SECRET_REFERENCE,
}
impl Default for FABRIC_SECRET_REFERENCE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SECURITY_CREDENTIALS {
    pub Kind: FABRIC_SECURITY_CREDENTIAL_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_SECURITY_CREDENTIALS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_SECURITY_CREDENTIAL_KIND(pub i32);
impl FABRIC_SECURITY_CREDENTIAL_KIND {
    pub const FABRIC_SECURITY_CREDENTIAL_KIND_NONE: Self = Self(0);
    pub const FABRIC_SECURITY_CREDENTIAL_KIND_X509: Self = Self(1);
    pub const FABRIC_SECURITY_CREDENTIAL_KIND_WINDOWS: Self = Self(2);
    pub const FABRIC_SECURITY_CREDENTIAL_KIND_CLAIMS: Self = Self(3);
    pub const FABRIC_SECURITY_CREDENTIAL_KIND_X509_2: Self = Self(4);
    pub const FABRIC_SECURITY_CREDENTIAL_KIND_INVALID: Self = Self(255);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SECURITY_GROUP_DESCRIPTION {
    pub Name: LPCWSTR,
    pub Sid: LPCWSTR,
    pub DomainGroupMembers: *const FABRIC_STRING_LIST,
    pub SystemGroupMembers: *const FABRIC_STRING_LIST,
    pub DomainUserMembers: *const FABRIC_STRING_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SECURITY_GROUP_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SECURITY_GROUP_DESCRIPTION_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_SECURITY_GROUP_DESCRIPTION,
}
impl Default for FABRIC_SECURITY_GROUP_DESCRIPTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SECURITY_USER_DESCRIPTION {
    pub Name: LPCWSTR,
    pub Sid: LPCWSTR,
    pub ParentSystemGroups: *const FABRIC_STRING_LIST,
    pub ParentApplicationGroups: *const FABRIC_STRING_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SECURITY_USER_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SECURITY_USER_DESCRIPTION_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_SECURITY_USER_DESCRIPTION,
}
impl Default for FABRIC_SECURITY_USER_DESCRIPTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SEED_NODE_SAFETY_CHECK {
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SEED_NODE_SAFETY_CHECK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SELECTED_PARTITION {
    pub ServiceName: FABRIC_URI,
    pub PartitionId: FABRIC_PARTITION_ID,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SELECTED_PARTITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SELF_RECONFIGURING_CONFIGURATION_CHANGE_REQUEST {
    pub RequestId: FABRIC_SELF_RECONFIGURING_CONFIGURATION_REQUEST_ID,
    pub Instances: *const FABRIC_SELF_RECONFIGURING_INSTANCE_CHANGE_REQUEST_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SELF_RECONFIGURING_CONFIGURATION_CHANGE_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SELF_RECONFIGURING_CONFIGURATION_REPORT {
    pub RequestId: FABRIC_SELF_RECONFIGURING_CONFIGURATION_REQUEST_ID,
    pub ReportId: FABRIC_SELF_RECONFIGURING_CONFIGURATION_REPORT_ID,
    pub Instances: *const FABRIC_SELF_RECONFIGURING_INSTANCE_INFORMATION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SELF_RECONFIGURING_CONFIGURATION_REPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SELF_RECONFIGURING_CONFIGURATION_REPORT_ID {
    pub SequenceNumber: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SELF_RECONFIGURING_CONFIGURATION_REPORT_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SELF_RECONFIGURING_CONFIGURATION_REQUEST {
    pub RequestId: FABRIC_SELF_RECONFIGURING_CONFIGURATION_REQUEST_ID,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SELF_RECONFIGURING_CONFIGURATION_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SELF_RECONFIGURING_CONFIGURATION_REQUEST_ID {
    pub GenerationNumber: i64,
    pub SequenceNumber: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SELF_RECONFIGURING_CONFIGURATION_REQUEST_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_SELF_RECONFIGURING_INSTANCE_ACTIVATION_STATE(pub i32);
impl FABRIC_SELF_RECONFIGURING_INSTANCE_ACTIVATION_STATE {
    pub const FABRIC_SELF_RECONFIGURING_INSTANCE_STATE_INVALID: Self = Self(0);
    pub const FABRIC_SELF_RECONFIGURING_INSTANCE_STATE_ACTIVATED: Self = Self(1);
    pub const FABRIC_SELF_RECONFIGURING_INSTANCE_STATE_DEACTIVATED: Self = Self(2);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SELF_RECONFIGURING_INSTANCE_CHANGE_REQUEST {
    pub InstanceId: i64,
    pub Role: FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE,
    pub RequestedRole: FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE,
    pub ActivationState: FABRIC_SELF_RECONFIGURING_INSTANCE_ACTIVATION_STATE,
    pub RequestedActivationState: FABRIC_SELF_RECONFIGURING_INSTANCE_ACTIVATION_STATE,
    pub Endpoints: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SELF_RECONFIGURING_INSTANCE_CHANGE_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SELF_RECONFIGURING_INSTANCE_CHANGE_REQUEST_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_SELF_RECONFIGURING_INSTANCE_CHANGE_REQUEST,
}
impl Default for FABRIC_SELF_RECONFIGURING_INSTANCE_CHANGE_REQUEST_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SELF_RECONFIGURING_INSTANCE_INFORMATION {
    pub InstanceId: i64,
    pub Role: FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE,
    pub ActivationState: FABRIC_SELF_RECONFIGURING_INSTANCE_ACTIVATION_STATE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SELF_RECONFIGURING_INSTANCE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SELF_RECONFIGURING_INSTANCE_INFORMATION_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_SELF_RECONFIGURING_INSTANCE_INFORMATION,
}
impl Default for FABRIC_SELF_RECONFIGURING_INSTANCE_INFORMATION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE(pub i32);
impl FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE {
    pub const FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE_INVALID: Self = Self(0);
    pub const FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE_NEW: Self = Self(1);
    pub const FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE_EXISTING: Self = Self(2);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE(pub i32);
impl FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE {
    pub const FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE_NONE: Self = Self(0);
    pub const FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE_INITIAL: Self = Self(1);
    pub const FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE_MEMBER: Self = Self(2);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SELF_RECONFIGURING_SERVICE_DESCRIPTION {
    pub ApplicationName: FABRIC_URI,
    pub ServiceName: FABRIC_URI,
    pub ServiceTypeName: LPCWSTR,
    pub InitializationDataSize: u32,
    pub InitializationData: *mut u8,
    pub PartitionScheme: FABRIC_PARTITION_SCHEME,
    pub PartitionSchemeDescription: *mut core::ffi::c_void,
    pub PlacementConstraints: LPCWSTR,
    pub CorrelationCount: u32,
    pub Correlations: *mut FABRIC_SERVICE_CORRELATION_DESCRIPTION,
    pub MetricCount: u32,
    pub Metrics: *mut FABRIC_SERVICE_LOAD_METRIC_DESCRIPTION,
    pub PolicyList: *mut FABRIC_SERVICE_PLACEMENT_POLICY_LIST,
    pub IsDefaultMoveCostSpecified: bool,
    pub DefaultMoveCost: FABRIC_MOVE_COST,
    pub ServicePackageActivationMode: FABRIC_SERVICE_PACKAGE_ACTIVATION_MODE,
    pub ServiceDnsName: LPCWSTR,
    pub ScalingPolicyCount: u32,
    pub ServiceScalingPolicies: *mut FABRIC_SERVICE_SCALING_POLICY,
    pub TagsDescription: *mut FABRIC_SERVICE_TAGS_DESCRIPTION,
    pub FailoverSettings: *mut FABRIC_SELF_RECONFIGURING_SERVICE_FAILOVER_SETTINGS,
    pub MinInstanceCount: i32,
    pub InstanceCount: i32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SELF_RECONFIGURING_SERVICE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SELF_RECONFIGURING_SERVICE_DESCRIPTION_EX1 {
    pub IsCreateAsDisabled: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SELF_RECONFIGURING_SERVICE_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SELF_RECONFIGURING_SERVICE_FAILOVER_SETTINGS {
    pub Flags: u32,
    pub InstanceLifecycleDescription: *mut SELF_RECONFIGURING_INSTANCE_LIFECYCLE_DESCRIPTION,
    pub InstanceRestartWaitDurationSeconds: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SELF_RECONFIGURING_SERVICE_FAILOVER_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SELF_RECONFIGURING_SERVICE_INSTANCE_HEALTH {
    pub PartitionId: FABRIC_PARTITION_ID,
    pub InstanceId: i64,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub HealthEvents: *const FABRIC_HEALTH_EVENT_LIST,
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SELF_RECONFIGURING_SERVICE_INSTANCE_HEALTH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SELF_RECONFIGURING_SERVICE_INSTANCE_HEALTH_REPORT {
    pub PartitionId: FABRIC_PARTITION_ID,
    pub InstanceId: i64,
    pub HealthInformation: *const FABRIC_HEALTH_INFORMATION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SELF_RECONFIGURING_SERVICE_INSTANCE_HEALTH_REPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SELF_RECONFIGURING_SERVICE_INSTANCE_HEALTH_STATE {
    pub PartitionId: FABRIC_PARTITION_ID,
    pub InstanceId: i64,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SELF_RECONFIGURING_SERVICE_INSTANCE_HEALTH_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SELF_RECONFIGURING_SERVICE_INSTANCE_HEALTH_STATE_EX1 {
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SELF_RECONFIGURING_SERVICE_INSTANCE_HEALTH_STATE_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SELF_RECONFIGURING_SERVICE_INSTANCE_QUERY_RESULT_ITEM {
    pub InstanceId: i64,
    pub InstanceRole: FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE,
    pub ReplicaStatus: FABRIC_QUERY_SERVICE_REPLICA_STATUS,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub ReplicaAddress: LPCWSTR,
    pub NodeName: LPCWSTR,
    pub LastInBuildDurationInSeconds: i64,
    pub PreviousSelfReconfiguringInstanceRole: FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SELF_RECONFIGURING_SERVICE_INSTANCE_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SELF_RECONFIGURING_SERVICE_PARTITION_QUERY_RESULT_ITEM {
    pub PartitionInformation: *const FABRIC_SERVICE_PARTITION_INFORMATION,
    pub InstanceCount: u32,
    pub HealthState: FABRIC_HEALTH_STATE,
    pub PartitionStatus: FABRIC_QUERY_SERVICE_PARTITION_STATUS,
    pub MinInstanceCount: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SELF_RECONFIGURING_SERVICE_PARTITION_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SELF_RECONFIGURING_SERVICE_QUERY_RESULT_ITEM {
    pub ServiceName: FABRIC_URI,
    pub ServiceTypeName: LPCWSTR,
    pub ServiceManifestVersion: LPCWSTR,
    pub HealthState: FABRIC_HEALTH_STATE,
    pub ServiceStatus: FABRIC_QUERY_SERVICE_STATUS,
    pub Metadata: *mut FABRIC_SERVICE_METADATA,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SELF_RECONFIGURING_SERVICE_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SELF_RECONFIGURING_SERVICE_TYPE_DESCRIPTION {
    pub ServiceTypeName: LPCWSTR,
    pub PlacementConstraints: LPCWSTR,
    pub LoadMetrics: *const FABRIC_SERVICE_LOAD_METRIC_DESCRIPTION_LIST,
    pub Extensions: *const FABRIC_SERVICE_TYPE_DESCRIPTION_EXTENSION_LIST,
    pub PolicyList: *mut FABRIC_SERVICE_PLACEMENT_POLICY_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SELF_RECONFIGURING_SERVICE_TYPE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION {
    pub Flags: u32,
    pub PlacementConstraints: LPCWSTR,
    pub PolicyList: *mut FABRIC_SERVICE_PLACEMENT_POLICY_LIST,
    pub CorrelationCount: u32,
    pub Correlations: *mut FABRIC_SERVICE_CORRELATION_DESCRIPTION,
    pub MetricCount: u32,
    pub Metrics: *mut FABRIC_SERVICE_LOAD_METRIC_DESCRIPTION,
    pub DefaultMoveCost: FABRIC_MOVE_COST,
    pub RepartitionKind: FABRIC_SERVICE_PARTITION_KIND,
    pub RepartitionDescription: *mut core::ffi::c_void,
    pub ScalingPolicyCount: u32,
    pub ServiceScalingPolicies: *mut FABRIC_SERVICE_SCALING_POLICY,
    pub ServiceDnsName: LPCWSTR,
    pub TagsDescription: *mut FABRIC_SERVICE_TAGS_DESCRIPTION,
    pub InstanceLifecycleDescription: *mut SELF_RECONFIGURING_INSTANCE_LIFECYCLE_DESCRIPTION,
    pub InstanceCount: i32,
    pub MinInstanceCount: i32,
    pub InstanceRestartWaitDurationSeconds: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS(pub i32);
impl FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS {
    pub const FABRIC_SELF_RECONFIGURING_SERVICE_NONE: Self = Self(0);
    pub const FABRIC_SELF_RECONFIGURING_SERVICE_INSTANCE_COUNT: Self = Self(1);
    pub const FABRIC_SELF_RECONFIGURING_SERVICE_MIN_INSTANCE_COUNT: Self = Self(2);
    pub const FABRIC_SELF_RECONFIGURING_SERVICE_PLACEMENT_CONSTRAINTS: Self = Self(4);
    pub const FABRIC_SELF_RECONFIGURING_SERVICE_POLICY_LIST: Self = Self(8);
    pub const FABRIC_SELF_RECONFIGURING_SERVICE_CORRELATIONS: Self = Self(16);
    pub const FABRIC_SELF_RECONFIGURING_SERVICE_METRICS: Self = Self(32);
    pub const FABRIC_SELF_RECONFIGURING_SERVICE_MOVE_COST: Self = Self(64);
    pub const FABRIC_SELF_RECONFIGURING_SERVICE_SCALING_POLICY: Self = Self(128);
    pub const FABRIC_SELF_RECONFIGURING_SERVICE_SERVICE_DNS_NAME: Self = Self(256);
    pub const FABRIC_SELF_RECONFIGURING_SERVICE_RESTORE_REPLICA_LOCATION_AFTER_UPGRADE: Self =
        Self(512);
    pub const FABRIC_SELF_RECONFIGURING_SERVICE_TAGS_REQUIRED_TO_PLACE: Self = Self(1024);
    pub const FABRIC_SELF_RECONFIGURING_SERVICE_TAGS_REQUIRED_TO_RUN: Self = Self(2048);
    pub const FABRIC_SELF_RECONFIGURING_SERVICE_INSTANCE_RESTART_WAIT_DURATION: Self = Self(4096);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FABRIC_SEQUENCE_NUMBER(pub i64);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICES_HEALTH_EVALUATION {
    pub Description: LPCWSTR,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub ServiceTypeName: LPCWSTR,
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub TotalCount: u32,
    pub MaxPercentUnhealthyServices: u8,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICES_HEALTH_EVALUATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_ARM_METADATA_UPDATE_DESCRIPTION {
    pub ArmMetadata: *mut FABRIC_COMMON_ARM_METADATA,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_ARM_METADATA_UPDATE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_CORRELATION_DESCRIPTION {
    pub ServiceName: FABRIC_URI,
    pub Scheme: FABRIC_SERVICE_CORRELATION_SCHEME,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_CORRELATION_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_SERVICE_CORRELATION_SCHEME(pub i32);
impl FABRIC_SERVICE_CORRELATION_SCHEME {
    pub const FABRIC_SERVICE_CORRELATION_SCHEME_INVALID: Self = Self(0);
    pub const FABRIC_SERVICE_CORRELATION_SCHEME_AFFINITY: Self = Self(1);
    pub const FABRIC_SERVICE_CORRELATION_SCHEME_ALIGNED_AFFINITY: Self = Self(2);
    pub const FABRIC_SERVICE_CORRELATION_SCHEME_NONALIGNED_AFFINITY: Self = Self(3);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_DESCRIPTION {
    pub Kind: FABRIC_SERVICE_DESCRIPTION_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_SERVICE_DESCRIPTION_KIND(pub i32);
impl FABRIC_SERVICE_DESCRIPTION_KIND {
    pub const FABRIC_SERVICE_DESCRIPTION_KIND_INVALID: Self = Self(0);
    pub const FABRIC_SERVICE_DESCRIPTION_KIND_STATELESS: Self = Self(1);
    pub const FABRIC_SERVICE_DESCRIPTION_KIND_STATEFUL: Self = Self(2);
    pub const FABRIC_SERVICE_DESCRIPTION_KIND_SELF_RECONFIGURING: Self = Self(3);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_SERVICE_DISABLE_FLAG(pub i32);
impl FABRIC_SERVICE_DISABLE_FLAG {
    pub const FABRIC_SERVICE_DISABLE_FLAG_INVALID: Self = Self(0);
    pub const FABRIC_SERVICE_DISABLE_FLAG_REMOVE_DATA: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_SERVICE_ENDPOINT_ROLE(pub i32);
impl FABRIC_SERVICE_ENDPOINT_ROLE {
    pub const FABRIC_SERVICE_ROLE_INVALID: Self = Self(0);
    pub const FABRIC_SERVICE_ROLE_STATELESS: Self = Self(1);
    pub const FABRIC_SERVICE_ROLE_STATEFUL_PRIMARY: Self = Self(2);
    pub const FABRIC_SERVICE_ROLE_STATEFUL_SECONDARY: Self = Self(3);
    pub const FABRIC_SERVICE_ROLE_STATEFUL_PRIMARY_AUXILIARY: Self = Self(4);
    pub const FABRIC_SERVICE_ROLE_STATEFUL_AUXILIARY: Self = Self(5);
    pub const FABRIC_SERVICE_ROLE_SELF_RECONFIGURING: Self = Self(6);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_FROM_TEMPLATE_DESCRIPTION {
    pub ApplicationName: FABRIC_URI,
    pub ServiceName: FABRIC_URI,
    pub ServiceDnsName: LPCWSTR,
    pub ServiceTypeName: LPCWSTR,
    pub ServicePackageActivationMode: FABRIC_SERVICE_PACKAGE_ACTIVATION_MODE,
    pub InitializationDataSize: u32,
    pub InitializationData: *mut u8,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_FROM_TEMPLATE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_GROUP_DESCRIPTION {
    pub Description: *mut FABRIC_SERVICE_DESCRIPTION,
    pub MemberCount: u32,
    pub MemberDescriptions: *mut FABRIC_SERVICE_GROUP_MEMBER_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_GROUP_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_GROUP_FROM_TEMPLATE_DESCRIPTION {
    pub ApplicationName: FABRIC_URI,
    pub ServiceName: FABRIC_URI,
    pub ServiceTypeName: LPCWSTR,
    pub ServicePackageActivationMode: FABRIC_SERVICE_PACKAGE_ACTIVATION_MODE,
    pub InitializationDataSize: u32,
    pub InitializationData: *mut u8,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_GROUP_FROM_TEMPLATE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_GROUP_MEMBER_DESCRIPTION {
    pub ServiceType: LPCWSTR,
    pub ServiceName: FABRIC_URI,
    pub InitializationDataSize: u32,
    pub InitializationData: *const u8,
    pub MetricCount: u32,
    pub Metrics: *mut FABRIC_SERVICE_LOAD_METRIC_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_GROUP_MEMBER_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_GROUP_MEMBER_MEMBER_QUERY_RESULT_ITEM {
    pub ServiceType: LPCWSTR,
    pub ServiceName: FABRIC_URI,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_GROUP_MEMBER_MEMBER_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_GROUP_MEMBER_MEMBER_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_SERVICE_GROUP_MEMBER_MEMBER_QUERY_RESULT_ITEM,
}
impl Default for FABRIC_SERVICE_GROUP_MEMBER_MEMBER_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_GROUP_MEMBER_QUERY_DESCRIPTION {
    pub ApplicationName: FABRIC_URI,
    pub ServiceNameFilter: FABRIC_URI,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_GROUP_MEMBER_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_GROUP_MEMBER_QUERY_RESULT_ITEM {
    pub ServiceName: FABRIC_URI,
    pub Members: *mut FABRIC_SERVICE_GROUP_MEMBER_MEMBER_QUERY_RESULT_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_GROUP_MEMBER_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_GROUP_MEMBER_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_SERVICE_GROUP_MEMBER_QUERY_RESULT_ITEM,
}
impl Default for FABRIC_SERVICE_GROUP_MEMBER_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_GROUP_MEMBER_TYPE_QUERY_DESCRIPTION {
    pub ApplicationTypeName: LPCWSTR,
    pub ApplicationTypeVersion: LPCWSTR,
    pub ServiceGroupTypeNameFilter: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_GROUP_MEMBER_TYPE_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_GROUP_MEMBER_TYPE_QUERY_RESULT_ITEM {
    pub ServiceGroupMemberTypeDescription: *mut FABRIC_SERVICE_GROUP_TYPE_MEMBER_DESCRIPTION_LIST,
    pub ServiceManifestVersion: LPCWSTR,
    pub ServiceManifestName: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_GROUP_MEMBER_TYPE_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_GROUP_MEMBER_TYPE_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_SERVICE_GROUP_MEMBER_TYPE_QUERY_RESULT_ITEM,
}
impl Default for FABRIC_SERVICE_GROUP_MEMBER_TYPE_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_GROUP_TYPE_DESCRIPTION {
    pub Description: *const FABRIC_SERVICE_TYPE_DESCRIPTION,
    pub Members: *const FABRIC_SERVICE_GROUP_TYPE_MEMBER_DESCRIPTION_LIST,
    pub UseImplicitFactory: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_GROUP_TYPE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_GROUP_TYPE_DESCRIPTION_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_SERVICE_GROUP_TYPE_DESCRIPTION,
}
impl Default for FABRIC_SERVICE_GROUP_TYPE_DESCRIPTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_GROUP_TYPE_MEMBER_DESCRIPTION {
    pub ServiceTypeName: LPCWSTR,
    pub LoadMetrics: *const FABRIC_SERVICE_LOAD_METRIC_DESCRIPTION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_GROUP_TYPE_MEMBER_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_GROUP_TYPE_MEMBER_DESCRIPTION_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_SERVICE_GROUP_TYPE_MEMBER_DESCRIPTION,
}
impl Default for FABRIC_SERVICE_GROUP_TYPE_MEMBER_DESCRIPTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_GROUP_UPDATE_DESCRIPTION {
    pub Description: *mut FABRIC_SERVICE_UPDATE_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_GROUP_UPDATE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_HEALTH {
    pub ServiceName: FABRIC_URI,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub HealthEvents: *const FABRIC_HEALTH_EVENT_LIST,
    pub PartitionHealthStates: *const FABRIC_PARTITION_HEALTH_STATE_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_HEALTH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_HEALTH_EVALUATION {
    pub Description: LPCWSTR,
    pub ServiceName: FABRIC_URI,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_HEALTH_EVALUATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_HEALTH_EX1 {
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_HEALTH_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_HEALTH_EX2 {
    pub HealthStatistics: *const FABRIC_HEALTH_STATISTICS,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_HEALTH_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_HEALTH_QUERY_DESCRIPTION {
    pub ServiceName: FABRIC_URI,
    pub HealthPolicy: *const FABRIC_APPLICATION_HEALTH_POLICY,
    pub EventsFilter: *const FABRIC_HEALTH_EVENTS_FILTER,
    pub PartitionsFilter: *const FABRIC_PARTITION_HEALTH_STATES_FILTER,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_HEALTH_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_HEALTH_QUERY_DESCRIPTION_EX1 {
    pub HealthStatisticsFilter: *const FABRIC_SERVICE_HEALTH_STATISTICS_FILTER,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_HEALTH_QUERY_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_HEALTH_REPORT {
    pub ServiceName: FABRIC_URI,
    pub HealthInformation: *const FABRIC_HEALTH_INFORMATION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_HEALTH_REPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_HEALTH_STATE {
    pub ServiceName: FABRIC_URI,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_HEALTH_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_HEALTH_STATES_FILTER {
    pub HealthStateFilter: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_HEALTH_STATES_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_HEALTH_STATE_CHUNK {
    pub ServiceName: FABRIC_URI,
    pub HealthState: FABRIC_HEALTH_STATE,
    pub PartitionHealthStateChunks: *const FABRIC_PARTITION_HEALTH_STATE_CHUNK_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_HEALTH_STATE_CHUNK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_HEALTH_STATE_CHUNK_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_SERVICE_HEALTH_STATE_CHUNK,
    pub TotalCount: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_HEALTH_STATE_CHUNK_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_HEALTH_STATE_FILTER {
    pub HealthStateFilter: u32,
    pub ServiceNameFilter: FABRIC_URI,
    pub PartitionFilters: *const FABRIC_PARTITION_HEALTH_STATE_FILTER_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_HEALTH_STATE_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_HEALTH_STATE_FILTER_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_SERVICE_HEALTH_STATE_FILTER,
}
impl Default for FABRIC_SERVICE_HEALTH_STATE_FILTER_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_HEALTH_STATE_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_SERVICE_HEALTH_STATE,
}
impl Default for FABRIC_SERVICE_HEALTH_STATE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_HEALTH_STATISTICS_FILTER {
    pub ExcludeHealthStatistics: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_HEALTH_STATISTICS_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_SERVICE_HOST_UPGRADE_IMPACT(pub i32);
impl FABRIC_SERVICE_HOST_UPGRADE_IMPACT {
    pub const FABRIC_SERVICE_HOST_UPGRADE_IMPACT_INVALID: Self = Self(0);
    pub const FABRIC_SERVICE_HOST_UPGRADE_IMPACT_NONE: Self = Self(1);
    pub const FABRIC_SERVICE_HOST_UPGRADE_IMPACT_SERVICE_HOST_RESTART: Self = Self(2);
    pub const FABRIC_SERVICE_HOST_UPGRADE_IMPACT_UNEXPECTED_SERVICE_HOST_RESTART: Self = Self(3);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_SERVICE_KIND(pub i32);
impl FABRIC_SERVICE_KIND {
    pub const FABRIC_SERVICE_KIND_INVALID: Self = Self(0);
    pub const FABRIC_SERVICE_KIND_STATELESS: Self = Self(1);
    pub const FABRIC_SERVICE_KIND_STATEFUL: Self = Self(2);
    pub const FABRIC_SERVICE_KIND_SELF_RECONFIGURING: Self = Self(3);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_LOAD_METRIC_DESCRIPTION {
    pub Name: LPCWSTR,
    pub Weight: FABRIC_SERVICE_LOAD_METRIC_WEIGHT,
    pub PrimaryDefaultLoad: u32,
    pub SecondaryDefaultLoad: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_LOAD_METRIC_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_LOAD_METRIC_DESCRIPTION_EX1 {
    pub AuxiliaryDefaultLoad: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_LOAD_METRIC_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_LOAD_METRIC_DESCRIPTION_EX2 {
    pub MaximumLoad: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_LOAD_METRIC_DESCRIPTION_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_LOAD_METRIC_DESCRIPTION_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_SERVICE_LOAD_METRIC_DESCRIPTION,
}
impl Default for FABRIC_SERVICE_LOAD_METRIC_DESCRIPTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_SERVICE_LOAD_METRIC_WEIGHT(pub i32);
impl FABRIC_SERVICE_LOAD_METRIC_WEIGHT {
    pub const FABRIC_SERVICE_LOAD_METRIC_WEIGHT_ZERO: Self = Self(0);
    pub const FABRIC_SERVICE_LOAD_METRIC_WEIGHT_LOW: Self = Self(1);
    pub const FABRIC_SERVICE_LOAD_METRIC_WEIGHT_MEDIUM: Self = Self(2);
    pub const FABRIC_SERVICE_LOAD_METRIC_WEIGHT_HIGH: Self = Self(3);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_METADATA {
    pub ArmMetadata: *mut FABRIC_COMMON_ARM_METADATA,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_NAME_QUERY_DESCRIPTION {
    pub PartitionId: FABRIC_PARTITION_ID,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_NAME_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_NAME_QUERY_RESULT {
    pub ServiceName: FABRIC_URI,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_NAME_QUERY_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_NOTIFICATION {
    pub ServiceName: FABRIC_URI,
    pub PartitionId: FABRIC_PARTITION_ID,
    pub EndpointCount: u32,
    pub Endpoints: *mut FABRIC_RESOLVED_SERVICE_ENDPOINT,
    pub PartitionInfo: *mut FABRIC_SERVICE_PARTITION_INFORMATION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_NOTIFICATION_FILTER_DESCRIPTION {
    pub Name: FABRIC_URI,
    pub Flags: FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_NOTIFICATION_FILTER_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS(pub i32);
impl FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS {
    pub const FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS_NONE: Self = Self(0);
    pub const FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS_NAME_PREFIX: Self = Self(1);
    pub const FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS_PRIMARY_ONLY: Self = Self(2);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_SERVICE_PACKAGE_ACTIVATION_MODE(pub i32);
impl FABRIC_SERVICE_PACKAGE_ACTIVATION_MODE {
    pub const FABRIC_SERVICE_PACKAGE_ACTIVATION_MODE_SHARED_PROCESS: Self = Self(0);
    pub const FABRIC_SERVICE_PACKAGE_ACTIVATION_MODE_EXCLUSIVE_PROCESS: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_SERVICE_PARTITION_ACCESS_STATUS(pub i32);
impl FABRIC_SERVICE_PARTITION_ACCESS_STATUS {
    pub const FABRIC_SERVICE_PARTITION_ACCESS_STATUS_INVALID: Self = Self(0);
    pub const FABRIC_SERVICE_PARTITION_ACCESS_STATUS_GRANTED: Self = Self(1);
    pub const FABRIC_SERVICE_PARTITION_ACCESS_STATUS_RECONFIGURATION_PENDING: Self = Self(2);
    pub const FABRIC_SERVICE_PARTITION_ACCESS_STATUS_NOT_PRIMARY: Self = Self(3);
    pub const FABRIC_SERVICE_PARTITION_ACCESS_STATUS_NO_WRITE_QUORUM: Self = Self(4);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_PARTITION_INFORMATION {
    pub Kind: FABRIC_SERVICE_PARTITION_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_PARTITION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_SERVICE_PARTITION_KIND(pub i32);
impl FABRIC_SERVICE_PARTITION_KIND {
    pub const FABRIC_SERVICE_PARTITION_KIND_INVALID: Self = Self(0);
    pub const FABRIC_SERVICE_PARTITION_KIND_SINGLETON: Self = Self(1);
    pub const FABRIC_SERVICE_PARTITION_KIND_INT64_RANGE: Self = Self(2);
    pub const FABRIC_SERVICE_PARTITION_KIND_NAMED: Self = Self(3);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION {
    pub ServiceName: FABRIC_URI,
    pub PartitionIdFilter: FABRIC_PARTITION_ID,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION_EX1 {
    pub ContinuationToken: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_PARTITION_QUERY_RESULT_ITEM {
    pub Kind: FABRIC_SERVICE_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_PARTITION_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_PARTITION_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_SERVICE_PARTITION_QUERY_RESULT_ITEM,
}
impl Default for FABRIC_SERVICE_PARTITION_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_PLACEMENT_POLICY_DESCRIPTION {
    pub Type: FABRIC_PLACEMENT_POLICY_TYPE,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_PLACEMENT_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_PLACEMENT_POLICY_LIST {
    pub PolicyCount: u32,
    pub Policies: *mut FABRIC_SERVICE_PLACEMENT_POLICY_DESCRIPTION,
}
impl Default for FABRIC_SERVICE_PLACEMENT_POLICY_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_QUERY_DESCRIPTION {
    pub ApplicationName: FABRIC_URI,
    pub ServiceNameFilter: FABRIC_URI,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_QUERY_DESCRIPTION_EX1 {
    pub ContinuationToken: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_QUERY_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_QUERY_DESCRIPTION_EX2 {
    pub ServiceTypeNameFilter: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_QUERY_DESCRIPTION_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_QUERY_DESCRIPTION_EX3 {
    pub MaxResults: i32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_QUERY_DESCRIPTION_EX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_QUERY_RESULT_ITEM {
    pub Kind: FABRIC_SERVICE_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_SERVICE_QUERY_RESULT_ITEM,
}
impl Default for FABRIC_SERVICE_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_SERVICE_REPLICA_KIND(pub i32);
impl FABRIC_SERVICE_REPLICA_KIND {
    pub const FABRIC_SERVICE_REPLICA_KIND_INVALID: Self = Self(0);
    pub const FABRIC_SERVICE_REPLICA_KIND_KEY_VALUE_STORE: Self = Self(1);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_REPLICA_QUERY_DESCRIPTION {
    pub PartitionId: FABRIC_PARTITION_ID,
    pub ReplicaIdOrInstanceIdFilter: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_REPLICA_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_REPLICA_QUERY_DESCRIPTION_EX1 {
    pub ReplicaStatusFilter: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_REPLICA_QUERY_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_REPLICA_QUERY_DESCRIPTION_EX2 {
    pub ContinuationToken: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_REPLICA_QUERY_DESCRIPTION_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_REPLICA_QUERY_RESULT_ITEM {
    pub Kind: FABRIC_SERVICE_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_REPLICA_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_REPLICA_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_SERVICE_REPLICA_QUERY_RESULT_ITEM,
}
impl Default for FABRIC_SERVICE_REPLICA_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_SCALING_POLICY {
    pub ServiceScalingPolicyTrigger: FABRIC_SCALING_TRIGGER,
    pub ServiceScalingPolicyMechanism: FABRIC_SCALING_MECHANISM,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_SCALING_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_TAGS_DESCRIPTION {
    pub TagsRequiredToPlace: *mut FABRIC_STRING_LIST,
    pub TagsRequiredToRun: *mut FABRIC_STRING_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_TAGS_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_TYPE_DESCRIPTION {
    pub Kind: FABRIC_SERVICE_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_TYPE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_TYPE_DESCRIPTION_EXTENSION {
    pub Name: LPCWSTR,
    pub Value: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_TYPE_DESCRIPTION_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_TYPE_DESCRIPTION_EXTENSION_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_SERVICE_TYPE_DESCRIPTION_EXTENSION,
}
impl Default for FABRIC_SERVICE_TYPE_DESCRIPTION_EXTENSION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_TYPE_DESCRIPTION_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_SERVICE_TYPE_DESCRIPTION,
}
impl Default for FABRIC_SERVICE_TYPE_DESCRIPTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_TYPE_HEALTH_POLICY {
    pub MaxPercentUnhealthyServices: u8,
    pub MaxPercentUnhealthyPartitionsPerService: u8,
    pub MaxPercentUnhealthyReplicasPerPartition: u8,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_TYPE_HEALTH_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_TYPE_HEALTH_POLICY_MAP {
    pub Count: u32,
    pub Items: *mut FABRIC_SERVICE_TYPE_HEALTH_POLICY_MAP_ITEM,
}
impl Default for FABRIC_SERVICE_TYPE_HEALTH_POLICY_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_TYPE_HEALTH_POLICY_MAP_ITEM {
    pub ServiceTypeName: LPCWSTR,
    pub ServiceTypeHealthPolicy: *const FABRIC_SERVICE_TYPE_HEALTH_POLICY,
}
impl Default for FABRIC_SERVICE_TYPE_HEALTH_POLICY_MAP_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_TYPE_QUERY_DESCRIPTION {
    pub ApplicationTypeName: LPCWSTR,
    pub ApplicationTypeVersion: LPCWSTR,
    pub ServiceTypeNameFilter: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_TYPE_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_TYPE_QUERY_RESULT_ITEM {
    pub ServiceTypeDescription: *mut FABRIC_SERVICE_TYPE_DESCRIPTION,
    pub ServiceManifestVersion: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_TYPE_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_TYPE_QUERY_RESULT_ITEM_EX1 {
    pub ServiceManifestName: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_TYPE_QUERY_RESULT_ITEM_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_TYPE_QUERY_RESULT_ITEM_EX2 {
    pub IsServiceGroup: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_TYPE_QUERY_RESULT_ITEM_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_TYPE_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_SERVICE_TYPE_QUERY_RESULT_ITEM,
}
impl Default for FABRIC_SERVICE_TYPE_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_SERVICE_TYPE_REGISTRATION_STATUS(pub i32);
impl FABRIC_SERVICE_TYPE_REGISTRATION_STATUS {
    pub const FABRIC_SERVICE_TYPE_REGISTRATION_STATUS_INVALID: Self = Self(0);
    pub const FABRIC_SERVICE_TYPE_REGISTRATION_STATUS_DISABLED: Self = Self(1);
    pub const FABRIC_SERVICE_TYPE_REGISTRATION_STATUS_NOT_REGISTERED: Self = Self(2);
    pub const FABRIC_SERVICE_TYPE_REGISTRATION_STATUS_REGISTERED: Self = Self(3);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SERVICE_UPDATE_DESCRIPTION {
    pub Kind: FABRIC_SERVICE_DESCRIPTION_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_SERVICE_UPDATE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SINGLETON_PARTITION_INFORMATION {
    pub Id: FABRIC_PARTITION_ID,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SINGLETON_PARTITION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STARTED_EVENT {
    pub TimeStampUtc: FILETIME,
    pub ChaosParameters: *mut FABRIC_CHAOS_PARAMETERS,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STARTED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_START_APPROVED_UPGRADES_DESCRIPTION {
    pub OperationId: FABRIC_UPGRADE_ORCHESTRATION_SERVICE_OPERATION_ID,
    pub ClusterConfigPath: LPCWSTR,
    pub RollbackOnFailure: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_START_APPROVED_UPGRADES_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_START_CHAOS_DESCRIPTION {
    pub ChaosParameters: *mut FABRIC_CHAOS_PARAMETERS,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_START_CHAOS_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_START_NODE_DESCRIPTION {
    pub NodeName: LPCWSTR,
    pub NodeInstanceId: u64,
    pub IPAddressOrFQDN: LPCWSTR,
    pub ClusterConnectionPort: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_START_NODE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_START_NODE_DESCRIPTION2 {
    pub Kind: FABRIC_START_NODE_DESCRIPTION_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_START_NODE_DESCRIPTION2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_START_NODE_DESCRIPTION_KIND(pub i32);
impl FABRIC_START_NODE_DESCRIPTION_KIND {
    pub const FABRIC_START_NODE_DESCRIPTION_KIND_INVALID: Self = Self(0);
    pub const FABRIC_START_NODE_DESCRIPTION_KIND_USING_NODE_NAME: Self = Self(1);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_START_NODE_DESCRIPTION_USING_NODE_NAME {
    pub NodeName: LPCWSTR,
    pub NodeInstanceId: u64,
    pub IPAddressOrFQDN: LPCWSTR,
    pub ClusterConnectionPort: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_START_NODE_DESCRIPTION_USING_NODE_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_START_NODE_POISON_DESCRIPTION {
    pub NodeId: LPCWSTR,
    pub NodeInstanceId: u64,
    pub Neighbors: *mut FABRIC_STRING_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_START_NODE_POISON_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_START_NODE_STATUS {
    pub NodeResult: *mut FABRIC_NODE_RESULT,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_START_NODE_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_START_PARTITION_DATA_LOSS_DESCRIPTION {
    pub OperationId: FABRIC_TEST_COMMAND_OPERATION_ID,
    pub PartitionSelector: *mut FABRIC_PARTITION_SELECTOR,
    pub DataLossMode: FABRIC_DATA_LOSS_MODE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_START_PARTITION_DATA_LOSS_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_START_PARTITION_QUORUM_LOSS_DESCRIPTION {
    pub OperationId: FABRIC_TEST_COMMAND_OPERATION_ID,
    pub PartitionSelector: *mut FABRIC_PARTITION_SELECTOR,
    pub QuorumLossMode: FABRIC_QUORUM_LOSS_MODE,
    pub QuorumLossDurationInMilliSeconds: i32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_START_PARTITION_QUORUM_LOSS_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_START_PARTITION_RESTART_DESCRIPTION {
    pub OperationId: FABRIC_TEST_COMMAND_OPERATION_ID,
    pub PartitionSelector: *mut FABRIC_PARTITION_SELECTOR,
    pub RestartPartitionMode: FABRIC_RESTART_PARTITION_MODE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_START_PARTITION_RESTART_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_START_UPGRADE_DESCRIPTION {
    pub ClusterConfig: LPCWSTR,
    pub HealthCheckRetryTimeoutInSeconds: u32,
    pub HealthCheckWaitDurationInSeconds: u32,
    pub HealthCheckStableDurationInSeconds: u32,
    pub UpgradeDomainTimeoutInSeconds: u32,
    pub UpgradeTimeoutInSeconds: u32,
    pub MaxPercentUnhealthyApplications: u8,
    pub MaxPercentUnhealthyNodes: u8,
    pub MaxPercentDeltaUnhealthyNodes: u8,
    pub MaxPercentUpgradeDomainDeltaUnhealthyNodes: u8,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_START_UPGRADE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_START_UPGRADE_DESCRIPTION_EX1 {
    pub ApplicationHealthPolicyMap: *const FABRIC_APPLICATION_HEALTH_POLICY_MAP,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_START_UPGRADE_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_DESCRIPTION {
    pub ApplicationName: FABRIC_URI,
    pub ServiceName: FABRIC_URI,
    pub ServiceTypeName: LPCWSTR,
    pub InitializationDataSize: u32,
    pub InitializationData: *mut u8,
    pub PartitionScheme: FABRIC_PARTITION_SCHEME,
    pub PartitionSchemeDescription: *mut core::ffi::c_void,
    pub TargetReplicaSetSize: i32,
    pub MinReplicaSetSize: i32,
    pub PlacementConstraints: LPCWSTR,
    pub CorrelationCount: u32,
    pub Correlations: *mut FABRIC_SERVICE_CORRELATION_DESCRIPTION,
    pub MetricCount: u32,
    pub Metrics: *mut FABRIC_SERVICE_LOAD_METRIC_DESCRIPTION,
    pub HasPersistedState: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX1 {
    pub PolicyList: *mut FABRIC_SERVICE_PLACEMENT_POLICY_LIST,
    pub FailoverSettings: *mut FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX2 {
    pub IsDefaultMoveCostSpecified: bool,
    pub DefaultMoveCost: FABRIC_MOVE_COST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX3 {
    pub ServicePackageActivationMode: FABRIC_SERVICE_PACKAGE_ACTIVATION_MODE,
    pub ServiceDnsName: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX4 {
    pub ScalingPolicyCount: u32,
    pub ServiceScalingPolicies: *mut FABRIC_SERVICE_SCALING_POLICY,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX5 {
    pub TagsDescription: *mut FABRIC_SERVICE_TAGS_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX6 {
    pub IsCreateAsDisabled: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS {
    pub Flags: u32,
    pub ReplicaRestartWaitDurationSeconds: u32,
    pub QuorumLossWaitDurationSeconds: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX1 {
    pub StandByReplicaKeepDurationSeconds: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX2 {
    pub ServicePlacementTimeLimitSeconds: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX3 {
    pub DropSourceReplicaOnMove: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX4 {
    pub ReplicaLifecycleDescription: *mut REPLICA_LIFECYCLE_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX5 {
    pub AuxiliaryReplicaCount: i32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX6 {
    pub ServiceSensitivityDescription: *mut SERVICE_SENSITIVITY_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_FLAGS(pub i32);
impl FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_FLAGS {
    pub const FABRIC_STATEFUL_SERVICE_SETTINGS_NONE: Self = Self(0);
    pub const FABRIC_STATEFUL_SERVICE_SETTINGS_REPLICA_RESTART_WAIT_DURATION: Self = Self(1);
    pub const FABRIC_STATEFUL_SERVICE_SETTINGS_QUORUM_LOSS_WAIT_DURATION: Self = Self(2);
    pub const FABRIC_STATEFUL_SERVICE_SETTINGS_STANDBY_REPLICA_KEEP_DURATION: Self = Self(4);
    pub const FABRIC_STATEFUL_SERVICE_SETTINGS_SERVICE_PLACEMENT_TIME_LIMIT: Self = Self(8);
    pub const FABRIC_STATEFUL_SERVICE_SETTINGS_DROP_SOURCE_REPLICA_ON_MOVE: Self = Self(16);
    pub const FABRIC_STATEFUL_SERVICE_SETTINGS_IS_SINGLETON_REPLICA_MOVE_ALLOWED_DURING_UPGRADE:
        Self = Self(32);
    pub const FABRIC_STATEFUL_SERVICE_SETTINGS_RESTORE_REPLICA_LOCATION_AFTER_UPGRADE: Self =
        Self(64);
    pub const FABRIC_STATEFUL_SERVICE_SETTINGS_AUXILIARY_REPLICA_COUNT: Self = Self(128);
    pub const FABRIC_STATEFUL_SERVICE_SETTINGS_SERVICE_SENSITIVITY: Self = Self(256);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_PARTITION_QUERY_RESULT_ITEM {
    pub PartitionInformation: *const FABRIC_SERVICE_PARTITION_INFORMATION,
    pub TargetReplicaSetSize: u32,
    pub MinReplicaSetSize: u32,
    pub HealthState: FABRIC_HEALTH_STATE,
    pub PartitionStatus: FABRIC_QUERY_SERVICE_PARTITION_STATUS,
    pub LastQuorumLossDurationInSeconds: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_PARTITION_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_PARTITION_QUERY_RESULT_ITEM_EX1 {
    pub PrimaryEpoch: FABRIC_EPOCH,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_PARTITION_QUERY_RESULT_ITEM_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_PARTITION_QUERY_RESULT_ITEM_EX2 {
    pub AuxiliaryReplicaCount: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_PARTITION_QUERY_RESULT_ITEM_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_QUERY_RESULT_ITEM {
    pub ServiceName: FABRIC_URI,
    pub ServiceTypeName: LPCWSTR,
    pub ServiceManifestVersion: LPCWSTR,
    pub HasPersistedState: bool,
    pub HealthState: FABRIC_HEALTH_STATE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_QUERY_RESULT_ITEM_EX1 {
    pub ServiceStatus: FABRIC_QUERY_SERVICE_STATUS,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_QUERY_RESULT_ITEM_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_QUERY_RESULT_ITEM_EX2 {
    pub IsServiceGroup: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_QUERY_RESULT_ITEM_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_QUERY_RESULT_ITEM_EX3 {
    pub Metadata: *mut FABRIC_SERVICE_METADATA,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_QUERY_RESULT_ITEM_EX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH {
    pub PartitionId: FABRIC_PARTITION_ID,
    pub ReplicaId: i64,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub HealthEvents: *const FABRIC_HEALTH_EVENT_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_EX1 {
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_REPORT {
    pub PartitionId: FABRIC_PARTITION_ID,
    pub ReplicaId: i64,
    pub HealthInformation: *const FABRIC_HEALTH_INFORMATION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_REPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_STATE {
    pub PartitionId: FABRIC_PARTITION_ID,
    pub ReplicaId: i64,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_STATE_EX1 {
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_STATE_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_REPLICA_QUERY_RESULT_ITEM {
    pub ReplicaId: i64,
    pub ReplicaRole: FABRIC_REPLICA_ROLE,
    pub ReplicaStatus: FABRIC_QUERY_SERVICE_REPLICA_STATUS,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub ReplicaAddress: LPCWSTR,
    pub NodeName: LPCWSTR,
    pub LastInBuildDurationInSeconds: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_REPLICA_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_REPLICA_QUERY_RESULT_ITEM_EX1 {
    pub PreviousReplicaRole: FABRIC_REPLICA_ROLE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_REPLICA_QUERY_RESULT_ITEM_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_TYPE_DESCRIPTION {
    pub ServiceTypeName: LPCWSTR,
    pub PlacementConstraints: LPCWSTR,
    pub LoadMetrics: *const FABRIC_SERVICE_LOAD_METRIC_DESCRIPTION_LIST,
    pub Extensions: *const FABRIC_SERVICE_TYPE_DESCRIPTION_EXTENSION_LIST,
    pub HasPersistedState: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_TYPE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_TYPE_DESCRIPTION_EX1 {
    pub PolicyList: *mut FABRIC_SERVICE_PLACEMENT_POLICY_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_TYPE_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION {
    pub Flags: u32,
    pub TargetReplicaSetSize: i32,
    pub ReplicaRestartWaitDurationSeconds: u32,
    pub QuorumLossWaitDurationSeconds: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX1 {
    pub StandByReplicaKeepDurationSeconds: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX10 {
    pub TagsDescription: *mut FABRIC_SERVICE_TAGS_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX10 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX11 {
    pub AuxiliaryReplicaCount: i32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX11 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX12 {
    pub ServiceSensitivityDescription: *mut SERVICE_SENSITIVITY_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX12 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX2 {
    pub MinReplicaSetSize: i32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX3 {
    pub PlacementConstraints: LPCWSTR,
    pub PolicyList: *mut FABRIC_SERVICE_PLACEMENT_POLICY_LIST,
    pub CorrelationCount: u32,
    pub Correlations: *mut FABRIC_SERVICE_CORRELATION_DESCRIPTION,
    pub MetricCount: u32,
    pub Metrics: *mut FABRIC_SERVICE_LOAD_METRIC_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX4 {
    pub DefaultMoveCost: FABRIC_MOVE_COST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX5 {
    pub RepartitionKind: FABRIC_SERVICE_PARTITION_KIND,
    pub RepartitionDescription: *mut core::ffi::c_void,
    pub ScalingPolicyCount: u32,
    pub ServiceScalingPolicies: *mut FABRIC_SERVICE_SCALING_POLICY,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX6 {
    pub ServicePlacementTimeLimitSeconds: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX7 {
    pub DropSourceReplicaOnMove: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX8 {
    pub ServiceDnsName: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX9 {
    pub ReplicaLifecycleDescription: *mut REPLICA_LIFECYCLE_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS(pub i32);
impl FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS {
    pub const FABRIC_STATEFUL_SERVICE_NONE: Self = Self(0);
    pub const FABRIC_STATEFUL_SERVICE_TARGET_REPLICA_SET_SIZE: Self = Self(1);
    pub const FABRIC_STATEFUL_SERVICE_REPLICA_RESTART_WAIT_DURATION: Self = Self(2);
    pub const FABRIC_STATEFUL_SERVICE_QUORUM_LOSS_WAIT_DURATION: Self = Self(4);
    pub const FABRIC_STATEFUL_SERVICE_STANDBY_REPLICA_KEEP_DURATION: Self = Self(8);
    pub const FABRIC_STATEFUL_SERVICE_MIN_REPLICA_SET_SIZE: Self = Self(16);
    pub const FABRIC_STATEFUL_SERVICE_PLACEMENT_CONSTRAINTS: Self = Self(32);
    pub const FABRIC_STATEFUL_SERVICE_POLICY_LIST: Self = Self(64);
    pub const FABRIC_STATEFUL_SERVICE_CORRELATIONS: Self = Self(128);
    pub const FABRIC_STATEFUL_SERVICE_METRICS: Self = Self(256);
    pub const FABRIC_STATEFUL_SERVICE_MOVE_COST: Self = Self(512);
    pub const FABRIC_STATEFUL_SERVICE_SCALING_POLICY: Self = Self(1024);
    pub const FABRIC_STATEFUL_SERVICE_SERVICE_PLACEMENT_TIME_LIMIT: Self = Self(2048);
    pub const FABRIC_STATEFUL_SERVICE_DROP_SOURCE_REPLICA_ON_MOVE: Self = Self(4096);
    pub const FABRIC_STATEFUL_SERVICE_SERVICE_DNS_NAME: Self = Self(8192);
    pub const FABRIC_STATEFUL_SERVICE_IS_SINGLETON_REPLICA_MOVE_ALLOWED_DURING_UPGRADE: Self =
        Self(16384);
    pub const FABRIC_STATEFUL_SERVICE_RESTORE_REPLICA_LOCATION_AFTER_UPGRADE: Self = Self(32768);
    pub const FABRIC_STATEFUL_SERVICE_TAGS_REQUIRED_TO_PLACE: Self = Self(65536);
    pub const FABRIC_STATEFUL_SERVICE_TAGS_REQUIRED_TO_RUN: Self = Self(131072);
    pub const FABRIC_STATEFUL_SERVICE_AUXILIARY_REPLICA_COUNT: Self = Self(262144);
    pub const FABRIC_STATEFUL_SERVICE_SERVICE_SENSITIVITY: Self = Self(524288);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_DESCRIPTION {
    pub ApplicationName: FABRIC_URI,
    pub ServiceName: FABRIC_URI,
    pub ServiceTypeName: LPCWSTR,
    pub InitializationDataSize: u32,
    pub InitializationData: *mut u8,
    pub PartitionScheme: FABRIC_PARTITION_SCHEME,
    pub PartitionSchemeDescription: *mut core::ffi::c_void,
    pub InstanceCount: i32,
    pub PlacementConstraints: LPCWSTR,
    pub CorrelationCount: u32,
    pub Correlations: *mut FABRIC_SERVICE_CORRELATION_DESCRIPTION,
    pub MetricCount: u32,
    pub Metrics: *mut FABRIC_SERVICE_LOAD_METRIC_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_DESCRIPTION_EX1 {
    pub PolicyList: *mut FABRIC_SERVICE_PLACEMENT_POLICY_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_DESCRIPTION_EX2 {
    pub IsDefaultMoveCostSpecified: bool,
    pub DefaultMoveCost: FABRIC_MOVE_COST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_DESCRIPTION_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_DESCRIPTION_EX3 {
    pub ServicePackageActivationMode: FABRIC_SERVICE_PACKAGE_ACTIVATION_MODE,
    pub ServiceDnsName: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_DESCRIPTION_EX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_DESCRIPTION_EX4 {
    pub ScalingPolicyCount: u32,
    pub ServiceScalingPolicies: *mut FABRIC_SERVICE_SCALING_POLICY,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_DESCRIPTION_EX4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_DESCRIPTION_EX5 {
    pub MinInstanceCount: i32,
    pub MinInstancePercentage: i32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_DESCRIPTION_EX5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_DESCRIPTION_EX6 {
    pub FailoverSettings: *mut FABRIC_STATELESS_SERVICE_FAILOVER_SETTINGS,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_DESCRIPTION_EX6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_DESCRIPTION_EX7 {
    pub TagsDescription: *mut FABRIC_SERVICE_TAGS_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_DESCRIPTION_EX7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_DESCRIPTION_EX8 {
    pub IsCreateAsDisabled: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_DESCRIPTION_EX8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_FAILOVER_SETTINGS {
    pub Flags: u32,
    pub InstanceCloseDelayDurationSeconds: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_FAILOVER_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_FAILOVER_SETTINGS_EX1 {
    pub InstanceRestartWaitDurationSeconds: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_FAILOVER_SETTINGS_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_FAILOVER_SETTINGS_EX2 {
    pub InstanceLifecycleDescription: *mut INSTANCE_LIFECYCLE_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_FAILOVER_SETTINGS_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_FAILOVER_SETTINGS_FLAGS(pub i32);
impl FABRIC_STATELESS_SERVICE_FAILOVER_SETTINGS_FLAGS {
    pub const FABRIC_STATELESS_SERVICE_SETTINGS_NONE: Self = Self(0);
    pub const FABRIC_STATELESS_SERVICE_SETTINGS_INSTANCE_CLOSE_DELAY_DURATION: Self = Self(1);
    pub const FABRIC_STATELESS_SERVICE_SETTINGS_INSTANCE_RESTART_WAIT_DURATION: Self = Self(2);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH {
    pub PartitionId: FABRIC_PARTITION_ID,
    pub InstanceId: i64,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub HealthEvents: *const FABRIC_HEALTH_EVENT_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_EX1 {
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_REPORT {
    pub PartitionId: FABRIC_PARTITION_ID,
    pub InstanceId: i64,
    pub HealthInformation: *const FABRIC_HEALTH_INFORMATION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_REPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_STATE {
    pub PartitionId: FABRIC_PARTITION_ID,
    pub InstanceId: i64,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_STATE_EX1 {
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_STATE_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_INSTANCE_QUERY_RESULT_ITEM {
    pub InstanceId: i64,
    pub ReplicaStatus: FABRIC_QUERY_SERVICE_REPLICA_STATUS,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub ReplicaAddress: LPCWSTR,
    pub NodeName: LPCWSTR,
    pub LastInBuildDurationInSeconds: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_INSTANCE_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_PARTITION_QUERY_RESULT_ITEM {
    pub PartitionInformation: *const FABRIC_SERVICE_PARTITION_INFORMATION,
    pub InstanceCount: u32,
    pub HealthState: FABRIC_HEALTH_STATE,
    pub PartitionStatus: FABRIC_QUERY_SERVICE_PARTITION_STATUS,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_PARTITION_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_PARTITION_QUERY_RESULT_ITEM_EX1 {
    pub MinInstanceCount: i32,
    pub MinInstancePercentage: i32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_PARTITION_QUERY_RESULT_ITEM_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_QUERY_RESULT_ITEM {
    pub ServiceName: FABRIC_URI,
    pub ServiceTypeName: LPCWSTR,
    pub ServiceManifestVersion: LPCWSTR,
    pub HealthState: FABRIC_HEALTH_STATE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_QUERY_RESULT_ITEM_EX1 {
    pub ServiceStatus: FABRIC_QUERY_SERVICE_STATUS,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_QUERY_RESULT_ITEM_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_QUERY_RESULT_ITEM_EX2 {
    pub IsServiceGroup: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_QUERY_RESULT_ITEM_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_QUERY_RESULT_ITEM_EX3 {
    pub Metadata: *mut FABRIC_SERVICE_METADATA,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_QUERY_RESULT_ITEM_EX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_TYPE_DESCRIPTION {
    pub ServiceTypeName: LPCWSTR,
    pub PlacementConstraints: LPCWSTR,
    pub LoadMetrics: *const FABRIC_SERVICE_LOAD_METRIC_DESCRIPTION_LIST,
    pub Extensions: *const FABRIC_SERVICE_TYPE_DESCRIPTION_EXTENSION_LIST,
    pub UseImplicitHost: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_TYPE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_TYPE_DESCRIPTION_EX1 {
    pub PolicyList: *mut FABRIC_SERVICE_PLACEMENT_POLICY_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_TYPE_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION {
    pub Flags: u32,
    pub InstanceCount: i32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_EX1 {
    pub PlacementConstraints: LPCWSTR,
    pub PolicyList: *mut FABRIC_SERVICE_PLACEMENT_POLICY_LIST,
    pub CorrelationCount: u32,
    pub Correlations: *mut FABRIC_SERVICE_CORRELATION_DESCRIPTION,
    pub MetricCount: u32,
    pub Metrics: *mut FABRIC_SERVICE_LOAD_METRIC_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_EX2 {
    pub DefaultMoveCost: FABRIC_MOVE_COST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_EX3 {
    pub RepartitionKind: FABRIC_SERVICE_PARTITION_KIND,
    pub RepartitionDescription: *mut core::ffi::c_void,
    pub ScalingPolicyCount: u32,
    pub ServiceScalingPolicies: *mut FABRIC_SERVICE_SCALING_POLICY,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_EX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_EX4 {
    pub MinInstanceCount: i32,
    pub MinInstancePercentage: i32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_EX4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_EX5 {
    pub InstanceCloseDelayDurationSeconds: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_EX5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_EX6 {
    pub InstanceRestartWaitDurationSeconds: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_EX6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_EX7 {
    pub ServiceDnsName: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_EX7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_EX8 {
    pub InstanceLifecycleDescription: *mut INSTANCE_LIFECYCLE_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_EX8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_EX9 {
    pub TagsDescription: *mut FABRIC_SERVICE_TAGS_DESCRIPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_EX9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS(pub i32);
impl FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS {
    pub const FABRIC_STATELESS_SERVICE_NONE: Self = Self(0);
    pub const FABRIC_STATELESS_SERVICE_INSTANCE_COUNT: Self = Self(1);
    pub const FABRIC_STATELESS_SERVICE_PLACEMENT_CONSTRAINTS: Self = Self(2);
    pub const FABRIC_STATELESS_SERVICE_POLICY_LIST: Self = Self(4);
    pub const FABRIC_STATELESS_SERVICE_CORRELATIONS: Self = Self(8);
    pub const FABRIC_STATELESS_SERVICE_METRICS: Self = Self(16);
    pub const FABRIC_STATELESS_SERVICE_MOVE_COST: Self = Self(32);
    pub const FABRIC_STATELESS_SERVICE_SCALING_POLICY: Self = Self(64);
    pub const FABRIC_STATELESS_SERVICE_MIN_INSTANCE_COUNT: Self = Self(128);
    pub const FABRIC_STATELESS_SERVICE_MIN_INSTANCE_PERCENTAGE: Self = Self(256);
    pub const FABRIC_STATELESS_SERVICE_INSTANCE_CLOSE_DELAY_DURATION: Self = Self(512);
    pub const FABRIC_STATELESS_SERVICE_INSTANCE_RESTART_WAIT_DURATION: Self = Self(1024);
    pub const FABRIC_STATELESS_SERVICE_SERVICE_DNS_NAME: Self = Self(2048);
    pub const FABRIC_STATELESS_SERVICE_RESTORE_REPLICA_LOCATION_AFTER_UPGRADE: Self = Self(4096);
    pub const FABRIC_STATELESS_SERVICE_TAGS_REQUIRED_TO_PLACE: Self = Self(8192);
    pub const FABRIC_STATELESS_SERVICE_TAGS_REQUIRED_TO_RUN: Self = Self(16384);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STOPPED_EVENT {
    pub TimeStampUtc: FILETIME,
    pub Reason: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STOPPED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STOP_NODE_DESCRIPTION {
    pub NodeName: LPCWSTR,
    pub NodeInstanceId: u64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STOP_NODE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STOP_NODE_DESCRIPTION2 {
    pub Kind: FABRIC_STOP_NODE_DESCRIPTION_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_STOP_NODE_DESCRIPTION2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_STOP_NODE_DESCRIPTION_KIND(pub i32);
impl FABRIC_STOP_NODE_DESCRIPTION_KIND {
    pub const FABRIC_STOP_NODE_DESCRIPTION_KIND_INVALID: Self = Self(0);
    pub const FABRIC_STOP_NODE_DESCRIPTION_KIND_USING_NODE_NAME: Self = Self(1);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STOP_NODE_DESCRIPTION_USING_NODE_NAME {
    pub NodeName: LPCWSTR,
    pub NodeInstanceId: u64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STOP_NODE_DESCRIPTION_USING_NODE_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STOP_NODE_STATUS {
    pub NodeResult: *mut FABRIC_NODE_RESULT,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STOP_NODE_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STORE_BACKUP_INFO {
    pub BackupFolder: LPCWSTR,
    pub BackupOption: FABRIC_STORE_BACKUP_OPTION,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STORE_BACKUP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STORE_BACKUP_INFO_EX1 {
    pub BackupChainId: windows_core::GUID,
    pub BackupIndex: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_STORE_BACKUP_INFO_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_STORE_BACKUP_OPTION(pub i32);
impl FABRIC_STORE_BACKUP_OPTION {
    pub const FABRIC_STORE_BACKUP_OPTION_FULL: Self = Self(1);
    pub const FABRIC_STORE_BACKUP_OPTION_INCREMENTAL: Self = Self(2);
    pub const FABRIC_STORE_BACKUP_OPTION_TRUNCATE_LOGS_ONLY: Self = Self(3);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STRING_LIST {
    pub Count: u32,
    pub Items: *const LPCWSTR,
}
impl Default for FABRIC_STRING_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_STRING_MAP {
    pub Count: u32,
    pub Items: *const FABRIC_STRING_PAIR,
}
impl Default for FABRIC_STRING_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FABRIC_STRING_PAIR = FABRIC_APPLICATION_PARAMETER;
pub const FABRIC_SYSTEM_APPLICATION: windows_core::PCWSTR = windows_core::w!("fabric:/System");
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SYSTEM_APPLICATION_HEALTH_EVALUATION {
    pub Description: LPCWSTR,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SYSTEM_APPLICATION_HEALTH_EVALUATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_SYSTEM_SERVICE_QUERY_DESCRIPTION {
    pub SystemServiceNameFilter: FABRIC_URI,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_SYSTEM_SERVICE_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_TEST_COMMAND_LIST_DESCRIPTION {
    pub TestCommandStateFilter: FABRIC_TEST_COMMAND_STATE_FILTER,
    pub TestCommandTypeFilter: FABRIC_TEST_COMMAND_TYPE_FILTER,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_TEST_COMMAND_LIST_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FABRIC_TEST_COMMAND_OPERATION_ID = windows_core::GUID;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_TEST_COMMAND_PROGRESS_STATE(pub i32);
impl FABRIC_TEST_COMMAND_PROGRESS_STATE {
    pub const FABRIC_TEST_COMMAND_PROGRESS_STATE_INVALID: Self = Self(0);
    pub const FABRIC_TEST_COMMAND_PROGRESS_STATE_RUNNING: Self = Self(1);
    pub const FABRIC_TEST_COMMAND_PROGRESS_STATE_ROLLING_BACK: Self = Self(2);
    pub const FABRIC_TEST_COMMAND_PROGRESS_STATE_COMPLETED: Self = Self(3);
    pub const FABRIC_TEST_COMMAND_PROGRESS_STATE_FAULTED: Self = Self(4);
    pub const FABRIC_TEST_COMMAND_PROGRESS_STATE_CANCELLED: Self = Self(5);
    pub const FABRIC_TEST_COMMAND_PROGRESS_STATE_FORCE_CANCELLED: Self = Self(6);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_TEST_COMMAND_STATE_FILTER(pub i32);
impl FABRIC_TEST_COMMAND_STATE_FILTER {
    pub const FABRIC_TEST_COMMAND_STATE_FILTER_DEFAULT: Self = Self(0);
    pub const FABRIC_TEST_COMMAND_STATE_FILTER_ALL: Self = Self(65535);
    pub const FABRIC_TEST_COMMAND_STATE_FILTER_RUNNING: Self = Self(1);
    pub const FABRIC_TEST_COMMAND_STATE_FILTER_ROLLING_BACK: Self = Self(2);
    pub const FABRIC_TEST_COMMAND_STATE_FILTER_COMPLETED_SUCCESSFULLY: Self = Self(8);
    pub const FABRIC_TEST_COMMAND_STATE_FILTER_FAILED: Self = Self(16);
    pub const FABRIC_TEST_COMMAND_STATE_FILTER_CANCELLED: Self = Self(32);
    pub const FABRIC_TEST_COMMAND_STATE_FILTER_FORCE_CANCELLED: Self = Self(64);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_TEST_COMMAND_TYPE(pub i32);
impl FABRIC_TEST_COMMAND_TYPE {
    pub const FABRIC_TEST_COMMAND_TYPE_DEFAULT: Self = Self(0);
    pub const FABRIC_TEST_COMMAND_TYPE_INVOKE_DATA_LOSS: Self = Self(1);
    pub const FABRIC_TEST_COMMAND_TYPE_INVOKE_QUORUM_LOSS: Self = Self(2);
    pub const FABRIC_TEST_COMMAND_TYPE_INVOKE_RESTART_PARTITION: Self = Self(4);
    pub const FABRIC_TEST_COMMAND_TYPE_START_NODE_TRANSITION: Self = Self(8);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_TEST_COMMAND_TYPE_FILTER(pub i32);
impl FABRIC_TEST_COMMAND_TYPE_FILTER {
    pub const FABRIC_TEST_COMMAND_TYPE_FILTER_DEFAULT: Self = Self(0);
    pub const FABRIC_TEST_COMMAND_TYPE_FILTER_ALL: Self = Self(65535);
    pub const FABRIC_TEST_COMMAND_TYPE_FILTER_PARTITION_DATA_LOSS: Self = Self(1);
    pub const FABRIC_TEST_COMMAND_TYPE_FILTER_PARTITION_QUORUM_LOSS: Self = Self(2);
    pub const FABRIC_TEST_COMMAND_TYPE_FILTER_PARTITION_RESTART: Self = Self(4);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_TEST_ERROR_EVENT {
    pub TimeStampUtc: FILETIME,
    pub Reason: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_TEST_ERROR_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FABRIC_TRANSACTION_ID = windows_core::GUID;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_TRANSACTION_ISOLATION_LEVEL(pub i32);
impl FABRIC_TRANSACTION_ISOLATION_LEVEL {
    pub const FABRIC_TRANSACTION_ISOLATION_LEVEL_DEFAULT: Self = Self(0);
    pub const FABRIC_TRANSACTION_ISOLATION_LEVEL_READ_UNCOMMITTED: Self = Self(1);
    pub const FABRIC_TRANSACTION_ISOLATION_LEVEL_READ_COMMITTED: Self = Self(2);
    pub const FABRIC_TRANSACTION_ISOLATION_LEVEL_REPEATABLE_READ: Self = Self(3);
    pub const FABRIC_TRANSACTION_ISOLATION_LEVEL_SNAPSHOT: Self = Self(4);
    pub const FABRIC_TRANSACTION_ISOLATION_LEVEL_SERIALIZABLE: Self = Self(5);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_UNIFORM_INT64_RANGE_PARTITION_SCHEME_DESCRIPTION {
    pub PartitionCount: i32,
    pub LowKey: i64,
    pub HighKey: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_UNIFORM_INT64_RANGE_PARTITION_SCHEME_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_UNPLACED_REPLICA_INFORMATION {
    pub ServiceName: FABRIC_URI,
    pub PartitionId: FABRIC_PARTITION_ID,
    pub UnplacedReplicaReasons: *const FABRIC_STRING_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_UNPLACED_REPLICA_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_UNPLACED_REPLICA_INFORMATION_LIST {
    pub Count: u32,
    pub Items: *mut LPCWSTR,
}
impl Default for FABRIC_UNPLACED_REPLICA_INFORMATION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_UNPLACED_REPLICA_INFORMATION_QUERY_DESCRIPTION {
    pub ServiceName: FABRIC_URI,
    pub PartitionId: FABRIC_PARTITION_ID,
    pub OnlyQueryPrimaries: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_UNPLACED_REPLICA_INFORMATION_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_UNPROVISION_APPLICATION_TYPE_DESCRIPTION {
    pub ApplicationTypeName: LPCWSTR,
    pub ApplicationTypeVersion: LPCWSTR,
    pub Async: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_UNPROVISION_APPLICATION_TYPE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_UPDATE_PARTITION_LOAD_QUERY_DESCRIPTION {
    pub PartitionMetricLoadDescriptionList: *mut FABRIC_PARTITION_METRIC_LOAD_DESCRIPTION_LIST,
    pub ContinuationToken: LPCWSTR,
    pub MaxResults: i32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_UPDATE_PARTITION_LOAD_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_UPDATE_PARTITION_LOAD_QUERY_RESULT_ITEM {
    pub PartitionId: FABRIC_PARTITION_ID,
    pub ErrorCode: windows_core::HRESULT,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_UPDATE_PARTITION_LOAD_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_UPDATE_PARTITION_LOAD_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *mut FABRIC_UPDATE_PARTITION_LOAD_QUERY_RESULT_ITEM,
}
impl Default for FABRIC_UPDATE_PARTITION_LOAD_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_UPDATE_PARTITION_MOVE_COST_QUERY_DESCRIPTION {
    pub PartitionMoveCostDescriptionList: *mut FABRIC_PARTITION_MOVE_COST_DESCRIPTION_LIST,
    pub ContinuationToken: LPCWSTR,
    pub MaxResults: i32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_UPDATE_PARTITION_MOVE_COST_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_UPDATE_PARTITION_MOVE_COST_QUERY_RESULT_ITEM {
    pub PartitionId: FABRIC_PARTITION_ID,
    pub ErrorCode: windows_core::HRESULT,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_UPDATE_PARTITION_MOVE_COST_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_UPDATE_PARTITION_MOVE_COST_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *mut FABRIC_UPDATE_PARTITION_MOVE_COST_QUERY_RESULT_ITEM,
}
impl Default for FABRIC_UPDATE_PARTITION_MOVE_COST_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_UPGRADE_DESCRIPTION {
    pub CodeVersion: LPCWSTR,
    pub ConfigVersion: LPCWSTR,
    pub UpgradeKind: FABRIC_UPGRADE_KIND,
    pub UpgradePolicyDescription: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_UPGRADE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_UPGRADE_DOMAIN_DELTA_NODES_CHECK_HEALTH_EVALUATION {
    pub Description: LPCWSTR,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub UpgradeDomainName: LPCWSTR,
    pub BaselineErrorCount: u32,
    pub BaselineTotalCount: u32,
    pub TotalCount: u32,
    pub MaxPercentUpgradeDomainDeltaUnhealthyNodes: u8,
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_UPGRADE_DOMAIN_DELTA_NODES_CHECK_HEALTH_EVALUATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_UPGRADE_DOMAIN_DEPLOYED_APPLICATIONS_HEALTH_EVALUATION {
    pub Description: LPCWSTR,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub UpgradeDomainName: LPCWSTR,
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub TotalCount: u32,
    pub MaxPercentUnhealthyDeployedApplications: u8,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_UPGRADE_DOMAIN_DEPLOYED_APPLICATIONS_HEALTH_EVALUATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_UPGRADE_DOMAIN_NODES_HEALTH_EVALUATION {
    pub Description: LPCWSTR,
    pub AggregatedHealthState: FABRIC_HEALTH_STATE,
    pub UpgradeDomainName: LPCWSTR,
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub TotalCount: u32,
    pub MaxPercentUnhealthyNodes: u8,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_UPGRADE_DOMAIN_NODES_HEALTH_EVALUATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_UPGRADE_DOMAIN_PROGRESS {
    pub UpgradeDomainName: LPCWSTR,
    pub NodeProgressList: *mut FABRIC_NODE_UPGRADE_PROGRESS_LIST,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_UPGRADE_DOMAIN_PROGRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_UPGRADE_DOMAIN_STATE(pub i32);
impl FABRIC_UPGRADE_DOMAIN_STATE {
    pub const FABRIC_UPGRADE_DOMAIN_STATE_INVALID: Self = Self(0);
    pub const FABRIC_UPGRADE_DOMAIN_STATE_PENDING: Self = Self(1);
    pub const FABRIC_UPGRADE_DOMAIN_STATE_IN_PROGRESS: Self = Self(2);
    pub const FABRIC_UPGRADE_DOMAIN_STATE_COMPLETED: Self = Self(3);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_UPGRADE_DOMAIN_STATUS_DESCRIPTION {
    pub Name: LPCWSTR,
    pub State: FABRIC_UPGRADE_DOMAIN_STATE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_UPGRADE_DOMAIN_STATUS_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_UPGRADE_DOMAIN_STATUS_DESCRIPTION_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_UPGRADE_DOMAIN_STATUS_DESCRIPTION,
}
impl Default for FABRIC_UPGRADE_DOMAIN_STATUS_DESCRIPTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_UPGRADE_FAILURE_REASON(pub i32);
impl FABRIC_UPGRADE_FAILURE_REASON {
    pub const FABRIC_UPGRADE_FAILURE_REASON_NONE: Self = Self(0);
    pub const FABRIC_UPGRADE_FAILURE_REASON_INTERRUPTED: Self = Self(1);
    pub const FABRIC_UPGRADE_FAILURE_REASON_HEALTH_CHECK: Self = Self(2);
    pub const FABRIC_UPGRADE_FAILURE_REASON_UPGRADE_DOMAIN_TIMEOUT: Self = Self(3);
    pub const FABRIC_UPGRADE_FAILURE_REASON_OVERALL_UPGRADE_TIMEOUT: Self = Self(4);
    pub const FABRIC_UPGRADE_FAILURE_REASON_PROCESSING_FAILURE: Self = Self(5);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_UPGRADE_KIND(pub i32);
impl FABRIC_UPGRADE_KIND {
    pub const FABRIC_UPGRADE_KIND_INVALID: Self = Self(0);
    pub const FABRIC_UPGRADE_KIND_ROLLING: Self = Self(1);
}
pub type FABRIC_UPGRADE_ORCHESTRATION_SERVICE_OPERATION_ID = windows_core::GUID;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_UPGRADE_ORCHESTRATION_SERVICE_STATE {
    pub CurrentCodeVersion: LPCWSTR,
    pub CurrentManifestVersion: LPCWSTR,
    pub TargetCodeVersion: LPCWSTR,
    pub TargetManifestVersion: LPCWSTR,
    pub PendingUpgradeType: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_UPGRADE_ORCHESTRATION_SERVICE_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_UPGRADE_PARTITION_SAFETY_CHECK {
    pub PartitionId: FABRIC_PARTITION_ID,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_UPGRADE_PARTITION_SAFETY_CHECK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_UPGRADE_PROGRESS {
    pub UpgradeDescription: *const FABRIC_UPGRADE_DESCRIPTION,
    pub UpgradeState: FABRIC_UPGRADE_STATE,
    pub UpgradeMode: FABRIC_ROLLING_UPGRADE_MODE,
    pub NextUpgradeDomain: LPCWSTR,
    pub UpgradeDomains: *const FABRIC_UPGRADE_DOMAIN_STATUS_DESCRIPTION_LIST,
    pub UpgradeDurationInSeconds: u32,
    pub CurrentUpgradeDomainDurationInSeconds: u32,
    pub UnhealthyEvaluations: *const FABRIC_HEALTH_EVALUATION_LIST,
    pub CurrentUpgradeDomainProgress: *const FABRIC_UPGRADE_DOMAIN_PROGRESS,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_UPGRADE_PROGRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_UPGRADE_PROGRESS_EX1 {
    pub StartTimestampUtc: FILETIME,
    pub FailureTimestampUtc: FILETIME,
    pub FailureReason: FABRIC_UPGRADE_FAILURE_REASON,
    pub UpgradeDomainProgressAtFailure: *const FABRIC_UPGRADE_DOMAIN_PROGRESS,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_UPGRADE_PROGRESS_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_UPGRADE_PROGRESS_EX2 {
    pub HealthCheckElapsedTime: u32,
    pub HealthCheckPhase: FABRIC_MONITORED_UPGRADE_HEALTH_CHECK_PHASE,
    pub HealthCheckFlips: i64,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_UPGRADE_PROGRESS_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_UPGRADE_SAFETY_CHECK {
    pub Kind: FABRIC_UPGRADE_SAFETY_CHECK_KIND,
    pub Value: *mut core::ffi::c_void,
}
impl Default for FABRIC_UPGRADE_SAFETY_CHECK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_UPGRADE_SAFETY_CHECK_KIND(pub i32);
impl FABRIC_UPGRADE_SAFETY_CHECK_KIND {
    pub const FABRIC_UPGRADE_SAFETY_CHECK_KIND_INVALID: Self = Self(0);
    pub const FABRIC_UPGRADE_SEED_NODE_SAFETY_CHECK_KIND_ENSURE_QUORUM: Self = Self(1);
    pub const FABRIC_UPGRADE_PARTITION_SAFETY_CHECK_KIND_ENSURE_QUORUM: Self = Self(2);
    pub const FABRIC_UPGRADE_PARTITION_SAFETY_CHECK_KIND_WAIT_FOR_PRIMARY_PLACEMENT: Self = Self(3);
    pub const FABRIC_UPGRADE_PARTITION_SAFETY_CHECK_KIND_WAIT_FOR_PRIMARY_SWAP: Self = Self(4);
    pub const FABRIC_UPGRADE_PARTITION_SAFETY_CHECK_KIND_WAIT_FOR_RECONFIGURATION: Self = Self(5);
    pub const FABRIC_UPGRADE_PARTITION_SAFETY_CHECK_KIND_WAIT_FOR_INBUILD_REPLICA: Self = Self(6);
    pub const FABRIC_UPGRADE_PARTITION_SAFETY_CHECK_KIND_ENSURE_AVAILABILITY: Self = Self(7);
    pub const FABRIC_UPGRADE_PARTITION_SAFETY_CHECK_KIND_WAIT_FOR_RESOURCE_AVAILABILITY: Self =
        Self(8);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_UPGRADE_SAFETY_CHECK_LIST {
    pub Count: u32,
    pub Items: *const FABRIC_UPGRADE_SAFETY_CHECK,
}
impl Default for FABRIC_UPGRADE_SAFETY_CHECK_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_UPGRADE_SEED_NODE_SAFETY_CHECK {
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_UPGRADE_SEED_NODE_SAFETY_CHECK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_UPGRADE_SORT_ORDER(pub i32);
impl FABRIC_UPGRADE_SORT_ORDER {
    pub const FABRIC_UPGRADE_SORT_ORDER_INVALID: Self = Self(0);
    pub const FABRIC_UPGRADE_SORT_ORDER_DEFAULT: Self = Self(1);
    pub const FABRIC_UPGRADE_SORT_ORDER_NUMERIC: Self = Self(2);
    pub const FABRIC_UPGRADE_SORT_ORDER_LEXICOGRAPHICAL: Self = Self(3);
    pub const FABRIC_UPGRADE_SORT_ORDER_REVERSE_NUMERIC: Self = Self(4);
    pub const FABRIC_UPGRADE_SORT_ORDER_REVERSE_LEXICOGRAPHICAL: Self = Self(5);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_UPGRADE_STATE(pub i32);
impl FABRIC_UPGRADE_STATE {
    pub const FABRIC_UPGRADE_STATE_INVALID: Self = Self(0);
    pub const FABRIC_UPGRADE_STATE_ROLLING_BACK_IN_PROGRESS: Self = Self(1);
    pub const FABRIC_UPGRADE_STATE_ROLLING_BACK_COMPLETED: Self = Self(2);
    pub const FABRIC_UPGRADE_STATE_ROLLING_FORWARD_PENDING: Self = Self(3);
    pub const FABRIC_UPGRADE_STATE_ROLLING_FORWARD_IN_PROGRESS: Self = Self(4);
    pub const FABRIC_UPGRADE_STATE_ROLLING_FORWARD_COMPLETED: Self = Self(5);
    pub const FABRIC_UPGRADE_STATE_FAILED: Self = Self(6);
    pub const FABRIC_UPGRADE_STATE_ROLLING_BACK_PENDING: Self = Self(7);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_UPGRADE_UPDATE_DESCRIPTION {
    pub UpgradeKind: FABRIC_UPGRADE_KIND,
    pub UpdateFlags: u32,
    pub UpgradePolicyDescription: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_UPGRADE_UPDATE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FABRIC_UPLOAD_SESSION_ID = windows_core::GUID;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FABRIC_URI(pub *mut core::ffi::c_void);
impl Default for FABRIC_URI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_VALIDATION_FAILED_EVENT {
    pub TimeStampUtc: FILETIME,
    pub Reason: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_VALIDATION_FAILED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_WAITING_EVENT {
    pub TimeStampUtc: FILETIME,
    pub Reason: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_WAITING_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_WINDOWS_CREDENTIALS {
    pub RemoteSpn: LPCWSTR,
    pub RemoteIdentityCount: u32,
    pub RemoteIdentities: *mut LPCWSTR,
    pub ProtectionLevel: FABRIC_PROTECTION_LEVEL,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_WINDOWS_CREDENTIALS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_X509_CREDENTIALS {
    pub AllowedCommonNameCount: u32,
    pub AllowedCommonNames: *mut LPCWSTR,
    pub FindType: FABRIC_X509_FIND_TYPE,
    pub FindValue: *mut core::ffi::c_void,
    pub StoreLocation: FABRIC_X509_STORE_LOCATION,
    pub StoreName: LPCWSTR,
    pub ProtectionLevel: FABRIC_PROTECTION_LEVEL,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_X509_CREDENTIALS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_X509_CREDENTIALS2 {
    pub CertLoadPath: LPCWSTR,
    pub RemoteCertThumbprintCount: u32,
    pub RemoteCertThumbprints: *mut LPCWSTR,
    pub RemoteX509NameCount: u32,
    pub RemoteX509Names: PCFABRIC_X509_NAME,
    pub ProtectionLevel: FABRIC_PROTECTION_LEVEL,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_X509_CREDENTIALS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_X509_CREDENTIALS_EX1 {
    pub IssuerThumbprintCount: u32,
    pub IssuerThumbprints: *mut LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_X509_CREDENTIALS_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_X509_CREDENTIALS_EX2 {
    pub RemoteCertThumbprintCount: u32,
    pub RemoteCertThumbprints: *mut LPCWSTR,
    pub RemoteX509NameCount: u32,
    pub RemoteX509Names: PCFABRIC_X509_NAME,
    pub FindValueSecondary: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_X509_CREDENTIALS_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_X509_CREDENTIALS_EX3 {
    pub RemoteCertIssuerCount: u32,
    pub RemoteCertIssuers: PCFABRIC_X509_ISSUER_NAME,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_X509_CREDENTIALS_EX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_X509_FIND_TYPE(pub i32);
impl FABRIC_X509_FIND_TYPE {
    pub const FABRIC_X509_FIND_TYPE_FINDBYTHUMBPRINT: Self = Self(0);
    pub const FABRIC_X509_FIND_TYPE_FINDBYSUBJECTNAME: Self = Self(1);
    pub const FABRIC_X509_FIND_TYPE_FINDBYEXTENSION: Self = Self(12);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_X509_ISSUER_NAME {
    pub Name: LPCWSTR,
    pub IssuerStoreCount: u32,
    pub IssuerStores: *mut LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_X509_ISSUER_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FABRIC_X509_NAME {
    pub Name: LPCWSTR,
    pub IssuerCertThumbprint: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for FABRIC_X509_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FABRIC_X509_STORE_LOCATION(pub i32);
impl FABRIC_X509_STORE_LOCATION {
    pub const FABRIC_X509_STORE_LOCATION_INVALID: Self = Self(0);
    pub const FABRIC_X509_STORE_LOCATION_CURRENTUSER: Self = Self(1);
    pub const FABRIC_X509_STORE_LOCATION_LOCALMACHINE: Self = Self(2);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILETIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct INSTANCE_LIFECYCLE_DESCRIPTION {
    pub IsRestoreReplicaLocationAfterUpgradeSpecified: bool,
    pub RestoreReplicaLocationAfterUpgrade: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for INSTANCE_LIFECYCLE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LPCWSTR = windows_core::PCWSTR;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NODE_TYPE_PLACEMENT_CONSTRAINT {
    pub NodePropertyName: LPCWSTR,
    pub NodePropertyValue: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for NODE_TYPE_PLACEMENT_CONSTRAINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NODE_TYPE_PLACEMENT_CONSTRAINT_LIST {
    pub Count: u32,
    pub Items: *const NODE_TYPE_PLACEMENT_CONSTRAINT,
}
impl Default for NODE_TYPE_PLACEMENT_CONSTRAINT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PAGED_FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION {
    pub ApplicationTypeNameFilter: LPCWSTR,
    pub MaxResults: i32,
    pub ContinuationToken: LPCWSTR,
    pub ExcludeApplicationParameters: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for PAGED_FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PAGED_FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION_EX1 {
    pub ApplicationTypeVersionFilter: LPCWSTR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for PAGED_FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PAGED_FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION_EX2 {
    pub ApplicationTypeDefinitionKindFilter: u32,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for PAGED_FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PCFABRIC_X509_ISSUER_NAME = *const FABRIC_X509_ISSUER_NAME;
pub type PCFABRIC_X509_NAME = *const FABRIC_X509_NAME;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REPLICA_LIFECYCLE_DESCRIPTION {
    pub IsIsSingletonReplicaMoveAllowedDuringUpgradeSpecified: bool,
    pub IsSingletonReplicaMoveAllowedDuringUpgrade: bool,
    pub IsRestoreReplicaLocationAfterUpgradeSpecified: bool,
    pub RestoreReplicaLocationAfterUpgrade: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for REPLICA_LIFECYCLE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SELF_RECONFIGURING_INSTANCE_LIFECYCLE_DESCRIPTION {
    pub IsRestoreReplicaLocationAfterUpgradeSpecified: bool,
    pub RestoreReplicaLocationAfterUpgrade: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for SELF_RECONFIGURING_INSTANCE_LIFECYCLE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVICE_SENSITIVITY_DESCRIPTION {
    pub PrimaryDefaultSensitivity: u32,
    pub SecondaryDefaultSensitivity: u32,
    pub AuxiliaryDefaultSensitivity: u32,
    pub IsMaximumSensitivity: bool,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for SERVICE_SENSITIVITY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TEST_COMMAND_QUERY_RESULT_ITEM {
    pub OperationId: windows_core::GUID,
    pub TestCommandState: FABRIC_TEST_COMMAND_PROGRESS_STATE,
    pub TestCommandType: FABRIC_TEST_COMMAND_TYPE,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for TEST_COMMAND_QUERY_RESULT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TEST_COMMAND_QUERY_RESULT_LIST {
    pub Count: u32,
    pub Items: *mut core::ffi::c_void,
}
impl Default for TEST_COMMAND_QUERY_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

// Free module-level aliases for scoped-enum variants (see tools_api). This
// restores the old dotnet-metadata spelling `FabricTypes::FABRIC_X_Y` in
// addition to the newtype's associated const `FABRIC_X::FABRIC_X_Y`.
pub const FABRIC_APPLICATION_DEFINITION_KIND_INVALID: FABRIC_APPLICATION_DEFINITION_KIND = FABRIC_APPLICATION_DEFINITION_KIND::FABRIC_APPLICATION_DEFINITION_KIND_INVALID;
pub const FABRIC_APPLICATION_DEFINITION_KIND_SERVICE_FABRIC_APPLICATION_DESCRIPTION: FABRIC_APPLICATION_DEFINITION_KIND = FABRIC_APPLICATION_DEFINITION_KIND::FABRIC_APPLICATION_DEFINITION_KIND_SERVICE_FABRIC_APPLICATION_DESCRIPTION;
pub const FABRIC_APPLICATION_DEFINITION_KIND_COMPOSE: FABRIC_APPLICATION_DEFINITION_KIND = FABRIC_APPLICATION_DEFINITION_KIND::FABRIC_APPLICATION_DEFINITION_KIND_COMPOSE;
pub const FABRIC_APPLICATION_DEFINITION_KIND_MESH_APPLICATION_DESCRIPTION: FABRIC_APPLICATION_DEFINITION_KIND = FABRIC_APPLICATION_DEFINITION_KIND::FABRIC_APPLICATION_DEFINITION_KIND_MESH_APPLICATION_DESCRIPTION;
pub const FABRIC_APPLICATION_DEFINITION_KIND_FILTER_DEFAULT: FABRIC_APPLICATION_DEFINITION_KIND_FILTER = FABRIC_APPLICATION_DEFINITION_KIND_FILTER::FABRIC_APPLICATION_DEFINITION_KIND_FILTER_DEFAULT;
pub const FABRIC_APPLICATION_DEFINITION_KIND_FILTER_ALL: FABRIC_APPLICATION_DEFINITION_KIND_FILTER = FABRIC_APPLICATION_DEFINITION_KIND_FILTER::FABRIC_APPLICATION_DEFINITION_KIND_FILTER_ALL;
pub const FABRIC_APPLICATION_DEFINITION_KIND_FILTER_SERVICE_FABRIC_APPLICATION_DESCRIPTION: FABRIC_APPLICATION_DEFINITION_KIND_FILTER = FABRIC_APPLICATION_DEFINITION_KIND_FILTER::FABRIC_APPLICATION_DEFINITION_KIND_FILTER_SERVICE_FABRIC_APPLICATION_DESCRIPTION;
pub const FABRIC_APPLICATION_DEFINITION_KIND_FILTER_COMPOSE: FABRIC_APPLICATION_DEFINITION_KIND_FILTER = FABRIC_APPLICATION_DEFINITION_KIND_FILTER::FABRIC_APPLICATION_DEFINITION_KIND_FILTER_COMPOSE;
pub const FABRIC_APPLICATION_DEFINITION_KIND_FILTER_MESH_APPLICATION_DESCRIPTION: FABRIC_APPLICATION_DEFINITION_KIND_FILTER = FABRIC_APPLICATION_DEFINITION_KIND_FILTER::FABRIC_APPLICATION_DEFINITION_KIND_FILTER_MESH_APPLICATION_DESCRIPTION;
pub const FABRIC_APPLICATION_PACKAGE_CLEANUP_POLICY_INVALID: FABRIC_APPLICATION_PACKAGE_CLEANUP_POLICY = FABRIC_APPLICATION_PACKAGE_CLEANUP_POLICY::FABRIC_APPLICATION_PACKAGE_CLEANUP_POLICY_INVALID;
pub const FABRIC_APPLICATION_PACKAGE_CLEANUP_POLICY_DEFAULT: FABRIC_APPLICATION_PACKAGE_CLEANUP_POLICY = FABRIC_APPLICATION_PACKAGE_CLEANUP_POLICY::FABRIC_APPLICATION_PACKAGE_CLEANUP_POLICY_DEFAULT;
pub const FABRIC_APPLICATION_PACKAGE_CLEANUP_POLICY_AUTOMATIC: FABRIC_APPLICATION_PACKAGE_CLEANUP_POLICY = FABRIC_APPLICATION_PACKAGE_CLEANUP_POLICY::FABRIC_APPLICATION_PACKAGE_CLEANUP_POLICY_AUTOMATIC;
pub const FABRIC_APPLICATION_PACKAGE_CLEANUP_POLICY_MANUAL: FABRIC_APPLICATION_PACKAGE_CLEANUP_POLICY = FABRIC_APPLICATION_PACKAGE_CLEANUP_POLICY::FABRIC_APPLICATION_PACKAGE_CLEANUP_POLICY_MANUAL;
pub const FABRIC_APPLICATION_STATUS_INVALID: FABRIC_APPLICATION_STATUS = FABRIC_APPLICATION_STATUS::FABRIC_APPLICATION_STATUS_INVALID;
pub const FABRIC_APPLICATION_STATUS_READY: FABRIC_APPLICATION_STATUS = FABRIC_APPLICATION_STATUS::FABRIC_APPLICATION_STATUS_READY;
pub const FABRIC_APPLICATION_STATUS_UPGRADING: FABRIC_APPLICATION_STATUS = FABRIC_APPLICATION_STATUS::FABRIC_APPLICATION_STATUS_UPGRADING;
pub const FABRIC_APPLICATION_STATUS_CREATING: FABRIC_APPLICATION_STATUS = FABRIC_APPLICATION_STATUS::FABRIC_APPLICATION_STATUS_CREATING;
pub const FABRIC_APPLICATION_STATUS_DELETING: FABRIC_APPLICATION_STATUS = FABRIC_APPLICATION_STATUS::FABRIC_APPLICATION_STATUS_DELETING;
pub const FABRIC_APPLICATION_STATUS_FAILED: FABRIC_APPLICATION_STATUS = FABRIC_APPLICATION_STATUS::FABRIC_APPLICATION_STATUS_FAILED;
pub const FABRIC_APPLICATION_TYPE_DEFINITION_KIND_INVALID: FABRIC_APPLICATION_TYPE_DEFINITION_KIND = FABRIC_APPLICATION_TYPE_DEFINITION_KIND::FABRIC_APPLICATION_TYPE_DEFINITION_KIND_INVALID;
pub const FABRIC_APPLICATION_TYPE_DEFINITION_KIND_SERVICE_FABRIC_APPLICATION_PACKAGE: FABRIC_APPLICATION_TYPE_DEFINITION_KIND = FABRIC_APPLICATION_TYPE_DEFINITION_KIND::FABRIC_APPLICATION_TYPE_DEFINITION_KIND_SERVICE_FABRIC_APPLICATION_PACKAGE;
pub const FABRIC_APPLICATION_TYPE_DEFINITION_KIND_COMPOSE: FABRIC_APPLICATION_TYPE_DEFINITION_KIND = FABRIC_APPLICATION_TYPE_DEFINITION_KIND::FABRIC_APPLICATION_TYPE_DEFINITION_KIND_COMPOSE;
pub const FABRIC_APPLICATION_TYPE_DEFINITION_KIND_MESH_APPLICATION_DESCRIPTION: FABRIC_APPLICATION_TYPE_DEFINITION_KIND = FABRIC_APPLICATION_TYPE_DEFINITION_KIND::FABRIC_APPLICATION_TYPE_DEFINITION_KIND_MESH_APPLICATION_DESCRIPTION;
pub const FABRIC_APPLICATION_TYPE_DEFINITION_KIND_FILTER_DEFAULT: FABRIC_APPLICATION_TYPE_DEFINITION_KIND_FILTER = FABRIC_APPLICATION_TYPE_DEFINITION_KIND_FILTER::FABRIC_APPLICATION_TYPE_DEFINITION_KIND_FILTER_DEFAULT;
pub const FABRIC_APPLICATION_TYPE_DEFINITION_KIND_FILTER_ALL: FABRIC_APPLICATION_TYPE_DEFINITION_KIND_FILTER = FABRIC_APPLICATION_TYPE_DEFINITION_KIND_FILTER::FABRIC_APPLICATION_TYPE_DEFINITION_KIND_FILTER_ALL;
pub const FABRIC_APPLICATION_TYPE_DEFINITION_KIND_FILTER_SERVICE_FABRIC_APPLICATION_PACKAGE: FABRIC_APPLICATION_TYPE_DEFINITION_KIND_FILTER = FABRIC_APPLICATION_TYPE_DEFINITION_KIND_FILTER::FABRIC_APPLICATION_TYPE_DEFINITION_KIND_FILTER_SERVICE_FABRIC_APPLICATION_PACKAGE;
pub const FABRIC_APPLICATION_TYPE_DEFINITION_KIND_FILTER_COMPOSE: FABRIC_APPLICATION_TYPE_DEFINITION_KIND_FILTER = FABRIC_APPLICATION_TYPE_DEFINITION_KIND_FILTER::FABRIC_APPLICATION_TYPE_DEFINITION_KIND_FILTER_COMPOSE;
pub const FABRIC_APPLICATION_TYPE_DEFINITION_KIND_FILTER_MESH_APPLICATION_DESCRIPTION: FABRIC_APPLICATION_TYPE_DEFINITION_KIND_FILTER = FABRIC_APPLICATION_TYPE_DEFINITION_KIND_FILTER::FABRIC_APPLICATION_TYPE_DEFINITION_KIND_FILTER_MESH_APPLICATION_DESCRIPTION;
pub const FABRIC_APPLICATION_TYPE_STATUS_INVALID: FABRIC_APPLICATION_TYPE_STATUS = FABRIC_APPLICATION_TYPE_STATUS::FABRIC_APPLICATION_TYPE_STATUS_INVALID;
pub const FABRIC_APPLICATION_TYPE_STATUS_PROVISIONING: FABRIC_APPLICATION_TYPE_STATUS = FABRIC_APPLICATION_TYPE_STATUS::FABRIC_APPLICATION_TYPE_STATUS_PROVISIONING;
pub const FABRIC_APPLICATION_TYPE_STATUS_AVAILABLE: FABRIC_APPLICATION_TYPE_STATUS = FABRIC_APPLICATION_TYPE_STATUS::FABRIC_APPLICATION_TYPE_STATUS_AVAILABLE;
pub const FABRIC_APPLICATION_TYPE_STATUS_UNPROVISIONING: FABRIC_APPLICATION_TYPE_STATUS = FABRIC_APPLICATION_TYPE_STATUS::FABRIC_APPLICATION_TYPE_STATUS_UNPROVISIONING;
pub const FABRIC_APPLICATION_TYPE_STATUS_FAILED: FABRIC_APPLICATION_TYPE_STATUS = FABRIC_APPLICATION_TYPE_STATUS::FABRIC_APPLICATION_TYPE_STATUS_FAILED;
pub const FABRIC_APPLICATION_UPDATE_DESCRIPTION_FLAGS_NONE: FABRIC_APPLICATION_UPDATE_DESCRIPTION_FLAGS = FABRIC_APPLICATION_UPDATE_DESCRIPTION_FLAGS::FABRIC_APPLICATION_UPDATE_DESCRIPTION_FLAGS_NONE;
pub const FABRIC_APPLICATION_UPDATE_DESCRIPTION_FLAGS_MINNODES: FABRIC_APPLICATION_UPDATE_DESCRIPTION_FLAGS = FABRIC_APPLICATION_UPDATE_DESCRIPTION_FLAGS::FABRIC_APPLICATION_UPDATE_DESCRIPTION_FLAGS_MINNODES;
pub const FABRIC_APPLICATION_UPDATE_DESCRIPTION_FLAGS_MAXNODES: FABRIC_APPLICATION_UPDATE_DESCRIPTION_FLAGS = FABRIC_APPLICATION_UPDATE_DESCRIPTION_FLAGS::FABRIC_APPLICATION_UPDATE_DESCRIPTION_FLAGS_MAXNODES;
pub const FABRIC_APPLICATION_UPDATE_DESCRIPTION_FLAGS_METRICS: FABRIC_APPLICATION_UPDATE_DESCRIPTION_FLAGS = FABRIC_APPLICATION_UPDATE_DESCRIPTION_FLAGS::FABRIC_APPLICATION_UPDATE_DESCRIPTION_FLAGS_METRICS;
pub const FABRIC_APPLICATION_UPGRADE_KIND_INVALID: FABRIC_APPLICATION_UPGRADE_KIND = FABRIC_APPLICATION_UPGRADE_KIND::FABRIC_APPLICATION_UPGRADE_KIND_INVALID;
pub const FABRIC_APPLICATION_UPGRADE_KIND_ROLLING: FABRIC_APPLICATION_UPGRADE_KIND = FABRIC_APPLICATION_UPGRADE_KIND::FABRIC_APPLICATION_UPGRADE_KIND_ROLLING;
pub const FABRIC_APPLICATION_UPGRADE_STATE_INVALID: FABRIC_APPLICATION_UPGRADE_STATE = FABRIC_APPLICATION_UPGRADE_STATE::FABRIC_APPLICATION_UPGRADE_STATE_INVALID;
pub const FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_BACK_IN_PROGRESS: FABRIC_APPLICATION_UPGRADE_STATE = FABRIC_APPLICATION_UPGRADE_STATE::FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_BACK_IN_PROGRESS;
pub const FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_BACK_COMPLETED: FABRIC_APPLICATION_UPGRADE_STATE = FABRIC_APPLICATION_UPGRADE_STATE::FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_BACK_COMPLETED;
pub const FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_FORWARD_PENDING: FABRIC_APPLICATION_UPGRADE_STATE = FABRIC_APPLICATION_UPGRADE_STATE::FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_FORWARD_PENDING;
pub const FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_FORWARD_IN_PROGRESS: FABRIC_APPLICATION_UPGRADE_STATE = FABRIC_APPLICATION_UPGRADE_STATE::FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_FORWARD_IN_PROGRESS;
pub const FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_FORWARD_COMPLETED: FABRIC_APPLICATION_UPGRADE_STATE = FABRIC_APPLICATION_UPGRADE_STATE::FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_FORWARD_COMPLETED;
pub const FABRIC_APPLICATION_UPGRADE_STATE_FAILED: FABRIC_APPLICATION_UPGRADE_STATE = FABRIC_APPLICATION_UPGRADE_STATE::FABRIC_APPLICATION_UPGRADE_STATE_FAILED;
pub const FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_BACK_PENDING: FABRIC_APPLICATION_UPGRADE_STATE = FABRIC_APPLICATION_UPGRADE_STATE::FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_BACK_PENDING;
pub const FABRIC_BLOCK_LIST_TYPE_SERVICE: FABRIC_BLOCK_LIST_TYPE = FABRIC_BLOCK_LIST_TYPE::FABRIC_BLOCK_LIST_TYPE_SERVICE;
pub const FABRIC_BLOCK_LIST_TYPE_OVERALL: FABRIC_BLOCK_LIST_TYPE = FABRIC_BLOCK_LIST_TYPE::FABRIC_BLOCK_LIST_TYPE_OVERALL;
pub const FABRIC_BLOCK_LIST_TYPE_PREFERRED_PRIMARY: FABRIC_BLOCK_LIST_TYPE = FABRIC_BLOCK_LIST_TYPE::FABRIC_BLOCK_LIST_TYPE_PREFERRED_PRIMARY;
pub const FABRIC_BLOCK_LIST_TYPE_PLACEMENT_TAGS: FABRIC_BLOCK_LIST_TYPE = FABRIC_BLOCK_LIST_TYPE::FABRIC_BLOCK_LIST_TYPE_PLACEMENT_TAGS;
pub const FABRIC_BLOCK_LIST_TYPE_RUNNING_TAGS: FABRIC_BLOCK_LIST_TYPE = FABRIC_BLOCK_LIST_TYPE::FABRIC_BLOCK_LIST_TYPE_RUNNING_TAGS;
pub const FABRIC_CHAOS_EVENT_KIND_INVALID: FABRIC_CHAOS_EVENT_KIND = FABRIC_CHAOS_EVENT_KIND::FABRIC_CHAOS_EVENT_KIND_INVALID;
pub const FABRIC_CHAOS_EVENT_KIND_STARTED: FABRIC_CHAOS_EVENT_KIND = FABRIC_CHAOS_EVENT_KIND::FABRIC_CHAOS_EVENT_KIND_STARTED;
pub const FABRIC_CHAOS_EVENT_KIND_EXECUTING_FAULTS: FABRIC_CHAOS_EVENT_KIND = FABRIC_CHAOS_EVENT_KIND::FABRIC_CHAOS_EVENT_KIND_EXECUTING_FAULTS;
pub const FABRIC_CHAOS_EVENT_KIND_WAITING: FABRIC_CHAOS_EVENT_KIND = FABRIC_CHAOS_EVENT_KIND::FABRIC_CHAOS_EVENT_KIND_WAITING;
pub const FABRIC_CHAOS_EVENT_KIND_VALIDATION_FAILED: FABRIC_CHAOS_EVENT_KIND = FABRIC_CHAOS_EVENT_KIND::FABRIC_CHAOS_EVENT_KIND_VALIDATION_FAILED;
pub const FABRIC_CHAOS_EVENT_KIND_TEST_ERROR: FABRIC_CHAOS_EVENT_KIND = FABRIC_CHAOS_EVENT_KIND::FABRIC_CHAOS_EVENT_KIND_TEST_ERROR;
pub const FABRIC_CHAOS_EVENT_KIND_STOPPED: FABRIC_CHAOS_EVENT_KIND = FABRIC_CHAOS_EVENT_KIND::FABRIC_CHAOS_EVENT_KIND_STOPPED;
pub const FABRIC_CHAOS_SCHEDULE_STATUS_INVALID: FABRIC_CHAOS_SCHEDULE_STATUS = FABRIC_CHAOS_SCHEDULE_STATUS::FABRIC_CHAOS_SCHEDULE_STATUS_INVALID;
pub const FABRIC_CHAOS_SCHEDULE_STATUS_ACTIVE: FABRIC_CHAOS_SCHEDULE_STATUS = FABRIC_CHAOS_SCHEDULE_STATUS::FABRIC_CHAOS_SCHEDULE_STATUS_ACTIVE;
pub const FABRIC_CHAOS_SCHEDULE_STATUS_EXPIRED: FABRIC_CHAOS_SCHEDULE_STATUS = FABRIC_CHAOS_SCHEDULE_STATUS::FABRIC_CHAOS_SCHEDULE_STATUS_EXPIRED;
pub const FABRIC_CHAOS_SCHEDULE_STATUS_PENDING: FABRIC_CHAOS_SCHEDULE_STATUS = FABRIC_CHAOS_SCHEDULE_STATUS::FABRIC_CHAOS_SCHEDULE_STATUS_PENDING;
pub const FABRIC_CHAOS_SCHEDULE_STATUS_STOPPED: FABRIC_CHAOS_SCHEDULE_STATUS = FABRIC_CHAOS_SCHEDULE_STATUS::FABRIC_CHAOS_SCHEDULE_STATUS_STOPPED;
pub const FABRIC_CHAOS_STATUS_INVALID: FABRIC_CHAOS_STATUS = FABRIC_CHAOS_STATUS::FABRIC_CHAOS_STATUS_INVALID;
pub const FABRIC_CHAOS_STATUS_RUNNING: FABRIC_CHAOS_STATUS = FABRIC_CHAOS_STATUS::FABRIC_CHAOS_STATUS_RUNNING;
pub const FABRIC_CHAOS_STATUS_STOPPED: FABRIC_CHAOS_STATUS = FABRIC_CHAOS_STATUS::FABRIC_CHAOS_STATUS_STOPPED;
pub const FABRIC_CLAIMS_RETRIEVAL_METADATA_KIND_NONE: FABRIC_CLAIMS_RETRIEVAL_METADATA_KIND = FABRIC_CLAIMS_RETRIEVAL_METADATA_KIND::FABRIC_CLAIMS_RETRIEVAL_METADATA_KIND_NONE;
pub const FABRIC_CLAIMS_RETRIEVAL_METADATA_KIND_AAD: FABRIC_CLAIMS_RETRIEVAL_METADATA_KIND = FABRIC_CLAIMS_RETRIEVAL_METADATA_KIND::FABRIC_CLAIMS_RETRIEVAL_METADATA_KIND_AAD;
pub const FABRIC_CLIENT_ROLE_UNKNOWN: FABRIC_CLIENT_ROLE = FABRIC_CLIENT_ROLE::FABRIC_CLIENT_ROLE_UNKNOWN;
pub const FABRIC_CLIENT_ROLE_USER: FABRIC_CLIENT_ROLE = FABRIC_CLIENT_ROLE::FABRIC_CLIENT_ROLE_USER;
pub const FABRIC_CLIENT_ROLE_ADMIN: FABRIC_CLIENT_ROLE = FABRIC_CLIENT_ROLE::FABRIC_CLIENT_ROLE_ADMIN;
pub const FABRIC_CLIENT_ROLE_ELEVATED_ADMIN: FABRIC_CLIENT_ROLE = FABRIC_CLIENT_ROLE::FABRIC_CLIENT_ROLE_ELEVATED_ADMIN;
pub const FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND_INVALID: FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND = FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND::FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND_INVALID;
pub const FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND_NONE: FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND = FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND::FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND_NONE;
pub const FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND_EXEHOST: FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND = FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND::FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND_EXEHOST;
pub const FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND_DLLHOST: FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND = FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND::FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND_DLLHOST;
pub const FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND_CONTAINERHOST: FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND = FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND::FABRIC_CODE_PACKAGE_ENTRY_POINT_KIND_CONTAINERHOST;
pub const FABRIC_CODE_PACKAGE_EVENT_TYPE_INVALID: FABRIC_CODE_PACKAGE_EVENT_TYPE = FABRIC_CODE_PACKAGE_EVENT_TYPE::FABRIC_CODE_PACKAGE_EVENT_TYPE_INVALID;
pub const FABRIC_CODE_PACKAGE_EVENT_TYPE_START_FAILED: FABRIC_CODE_PACKAGE_EVENT_TYPE = FABRIC_CODE_PACKAGE_EVENT_TYPE::FABRIC_CODE_PACKAGE_EVENT_TYPE_START_FAILED;
pub const FABRIC_CODE_PACKAGE_EVENT_TYPE_STARTED: FABRIC_CODE_PACKAGE_EVENT_TYPE = FABRIC_CODE_PACKAGE_EVENT_TYPE::FABRIC_CODE_PACKAGE_EVENT_TYPE_STARTED;
pub const FABRIC_CODE_PACKAGE_EVENT_TYPE_READY: FABRIC_CODE_PACKAGE_EVENT_TYPE = FABRIC_CODE_PACKAGE_EVENT_TYPE::FABRIC_CODE_PACKAGE_EVENT_TYPE_READY;
pub const FABRIC_CODE_PACKAGE_EVENT_TYPE_HEALTH: FABRIC_CODE_PACKAGE_EVENT_TYPE = FABRIC_CODE_PACKAGE_EVENT_TYPE::FABRIC_CODE_PACKAGE_EVENT_TYPE_HEALTH;
pub const FABRIC_CODE_PACKAGE_EVENT_TYPE_STOPPED: FABRIC_CODE_PACKAGE_EVENT_TYPE = FABRIC_CODE_PACKAGE_EVENT_TYPE::FABRIC_CODE_PACKAGE_EVENT_TYPE_STOPPED;
pub const FABRIC_CODE_PACKAGE_EVENT_TYPE_TERMINATED: FABRIC_CODE_PACKAGE_EVENT_TYPE = FABRIC_CODE_PACKAGE_EVENT_TYPE::FABRIC_CODE_PACKAGE_EVENT_TYPE_TERMINATED;
pub const FABRIC_CODE_PACKAGE_EVENT_TYPE_RAN_TO_COMPLETION: FABRIC_CODE_PACKAGE_EVENT_TYPE = FABRIC_CODE_PACKAGE_EVENT_TYPE::FABRIC_CODE_PACKAGE_EVENT_TYPE_RAN_TO_COMPLETION;
pub const FABRIC_DATA_LOSS_MODE_INVALID: FABRIC_DATA_LOSS_MODE = FABRIC_DATA_LOSS_MODE::FABRIC_DATA_LOSS_MODE_INVALID;
pub const FABRIC_DATA_LOSS_MODE_PARTIAL: FABRIC_DATA_LOSS_MODE = FABRIC_DATA_LOSS_MODE::FABRIC_DATA_LOSS_MODE_PARTIAL;
pub const FABRIC_DATA_LOSS_MODE_FULL: FABRIC_DATA_LOSS_MODE = FABRIC_DATA_LOSS_MODE::FABRIC_DATA_LOSS_MODE_FULL;
pub const FABRIC_DEPLOYMENT_STATUS_INVALID: FABRIC_DEPLOYMENT_STATUS = FABRIC_DEPLOYMENT_STATUS::FABRIC_DEPLOYMENT_STATUS_INVALID;
pub const FABRIC_DEPLOYMENT_STATUS_DOWNLOADING: FABRIC_DEPLOYMENT_STATUS = FABRIC_DEPLOYMENT_STATUS::FABRIC_DEPLOYMENT_STATUS_DOWNLOADING;
pub const FABRIC_DEPLOYMENT_STATUS_ACTIVATING: FABRIC_DEPLOYMENT_STATUS = FABRIC_DEPLOYMENT_STATUS::FABRIC_DEPLOYMENT_STATUS_ACTIVATING;
pub const FABRIC_DEPLOYMENT_STATUS_ACTIVE: FABRIC_DEPLOYMENT_STATUS = FABRIC_DEPLOYMENT_STATUS::FABRIC_DEPLOYMENT_STATUS_ACTIVE;
pub const FABRIC_DEPLOYMENT_STATUS_UPGRADING: FABRIC_DEPLOYMENT_STATUS = FABRIC_DEPLOYMENT_STATUS::FABRIC_DEPLOYMENT_STATUS_UPGRADING;
pub const FABRIC_DEPLOYMENT_STATUS_DEACTIVATING: FABRIC_DEPLOYMENT_STATUS = FABRIC_DEPLOYMENT_STATUS::FABRIC_DEPLOYMENT_STATUS_DEACTIVATING;
pub const FABRIC_DEPLOYMENT_STATUS_RAN_TO_COMPLETION: FABRIC_DEPLOYMENT_STATUS = FABRIC_DEPLOYMENT_STATUS::FABRIC_DEPLOYMENT_STATUS_RAN_TO_COMPLETION;
pub const FABRIC_DEPLOYMENT_STATUS_FAILED: FABRIC_DEPLOYMENT_STATUS = FABRIC_DEPLOYMENT_STATUS::FABRIC_DEPLOYMENT_STATUS_FAILED;
pub const FABRIC_DIAGNOSTICS_SINKS_KIND_INVALID: FABRIC_DIAGNOSTICS_SINKS_KIND = FABRIC_DIAGNOSTICS_SINKS_KIND::FABRIC_DIAGNOSTICS_SINKS_KIND_INVALID;
pub const FABRIC_DIAGNOSTICS_SINKS_KIND_AZUREINTERNAL: FABRIC_DIAGNOSTICS_SINKS_KIND = FABRIC_DIAGNOSTICS_SINKS_KIND::FABRIC_DIAGNOSTICS_SINKS_KIND_AZUREINTERNAL;
pub const FABRIC_DLLHOST_HOSTED_DLL_KIND_INVALID: FABRIC_DLLHOST_HOSTED_DLL_KIND = FABRIC_DLLHOST_HOSTED_DLL_KIND::FABRIC_DLLHOST_HOSTED_DLL_KIND_INVALID;
pub const FABRIC_DLLHOST_HOSTED_DLL_KIND_UNMANAGED: FABRIC_DLLHOST_HOSTED_DLL_KIND = FABRIC_DLLHOST_HOSTED_DLL_KIND::FABRIC_DLLHOST_HOSTED_DLL_KIND_UNMANAGED;
pub const FABRIC_DLLHOST_HOSTED_DLL_KIND_MANAGED: FABRIC_DLLHOST_HOSTED_DLL_KIND = FABRIC_DLLHOST_HOSTED_DLL_KIND::FABRIC_DLLHOST_HOSTED_DLL_KIND_MANAGED;
pub const FABRIC_DLLHOST_ISOLATION_POLICY_INVALID: FABRIC_DLLHOST_ISOLATION_POLICY = FABRIC_DLLHOST_ISOLATION_POLICY::FABRIC_DLLHOST_ISOLATION_POLICY_INVALID;
pub const FABRIC_DLLHOST_ISOLATION_POLICY_SHARED_DOMAIN: FABRIC_DLLHOST_ISOLATION_POLICY = FABRIC_DLLHOST_ISOLATION_POLICY::FABRIC_DLLHOST_ISOLATION_POLICY_SHARED_DOMAIN;
pub const FABRIC_DLLHOST_ISOLATION_POLICY_DEDICATED_DOMAIN: FABRIC_DLLHOST_ISOLATION_POLICY = FABRIC_DLLHOST_ISOLATION_POLICY::FABRIC_DLLHOST_ISOLATION_POLICY_DEDICATED_DOMAIN;
pub const FABRIC_DLLHOST_ISOLATION_POLICY_DEDICATED_PROCESS: FABRIC_DLLHOST_ISOLATION_POLICY = FABRIC_DLLHOST_ISOLATION_POLICY::FABRIC_DLLHOST_ISOLATION_POLICY_DEDICATED_PROCESS;
pub const FABRIC_ENTRY_POINT_STATUS_INVALID: FABRIC_ENTRY_POINT_STATUS = FABRIC_ENTRY_POINT_STATUS::FABRIC_ENTRY_POINT_STATUS_INVALID;
pub const FABRIC_ENTRY_POINT_STATUS_PENDING: FABRIC_ENTRY_POINT_STATUS = FABRIC_ENTRY_POINT_STATUS::FABRIC_ENTRY_POINT_STATUS_PENDING;
pub const FABRIC_ENTRY_POINT_STATUS_STARTING: FABRIC_ENTRY_POINT_STATUS = FABRIC_ENTRY_POINT_STATUS::FABRIC_ENTRY_POINT_STATUS_STARTING;
pub const FABRIC_ENTRY_POINT_STATUS_STARTED: FABRIC_ENTRY_POINT_STATUS = FABRIC_ENTRY_POINT_STATUS::FABRIC_ENTRY_POINT_STATUS_STARTED;
pub const FABRIC_ENTRY_POINT_STATUS_STOPPING: FABRIC_ENTRY_POINT_STATUS = FABRIC_ENTRY_POINT_STATUS::FABRIC_ENTRY_POINT_STATUS_STOPPING;
pub const FABRIC_ENTRY_POINT_STATUS_STOPPED: FABRIC_ENTRY_POINT_STATUS = FABRIC_ENTRY_POINT_STATUS::FABRIC_ENTRY_POINT_STATUS_STOPPED;
pub const FABRIC_ENUMERATION_INVALID: FABRIC_ENUMERATION_STATUS = FABRIC_ENUMERATION_STATUS::FABRIC_ENUMERATION_INVALID;
pub const FABRIC_ENUMERATION_BEST_EFFORT_MORE_DATA: FABRIC_ENUMERATION_STATUS = FABRIC_ENUMERATION_STATUS::FABRIC_ENUMERATION_BEST_EFFORT_MORE_DATA;
pub const FABRIC_ENUMERATION_CONSISTENT_MORE_DATA: FABRIC_ENUMERATION_STATUS = FABRIC_ENUMERATION_STATUS::FABRIC_ENUMERATION_CONSISTENT_MORE_DATA;
pub const FABRIC_ENUMERATION_BEST_EFFORT_FINISHED: FABRIC_ENUMERATION_STATUS = FABRIC_ENUMERATION_STATUS::FABRIC_ENUMERATION_BEST_EFFORT_FINISHED;
pub const FABRIC_ENUMERATION_CONSISTENT_FINISHED: FABRIC_ENUMERATION_STATUS = FABRIC_ENUMERATION_STATUS::FABRIC_ENUMERATION_CONSISTENT_FINISHED;
pub const FABRIC_ENUMERATION_VALID_MASK: FABRIC_ENUMERATION_STATUS = FABRIC_ENUMERATION_STATUS::FABRIC_ENUMERATION_VALID_MASK;
pub const FABRIC_ENUMERATION_BEST_EFFORT_MASK: FABRIC_ENUMERATION_STATUS = FABRIC_ENUMERATION_STATUS::FABRIC_ENUMERATION_BEST_EFFORT_MASK;
pub const FABRIC_ENUMERATION_CONSISTENT_MASK: FABRIC_ENUMERATION_STATUS = FABRIC_ENUMERATION_STATUS::FABRIC_ENUMERATION_CONSISTENT_MASK;
pub const FABRIC_ENUMERATION_MORE_DATA_MASK: FABRIC_ENUMERATION_STATUS = FABRIC_ENUMERATION_STATUS::FABRIC_ENUMERATION_MORE_DATA_MASK;
pub const FABRIC_ENUMERATION_FINISHED_MASK: FABRIC_ENUMERATION_STATUS = FABRIC_ENUMERATION_STATUS::FABRIC_ENUMERATION_FINISHED_MASK;
pub const FABRIC_E_FIRST_RESERVED_HRESULT: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_FIRST_RESERVED_HRESULT;
pub const FABRIC_E_LAST_RESERVED_HRESULT: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_LAST_RESERVED_HRESULT;
pub const FABRIC_E_COMMUNICATION_ERROR: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_COMMUNICATION_ERROR;
pub const FABRIC_E_INVALID_ADDRESS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_ADDRESS;
pub const FABRIC_E_INVALID_NAME_URI: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_NAME_URI;
pub const FABRIC_E_INVALID_PARTITION_KEY: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_PARTITION_KEY;
pub const FABRIC_E_NAME_ALREADY_EXISTS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_NAME_ALREADY_EXISTS;
pub const FABRIC_E_NAME_DOES_NOT_EXIST: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_NAME_DOES_NOT_EXIST;
pub const FABRIC_E_NAME_NOT_EMPTY: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_NAME_NOT_EMPTY;
pub const FABRIC_E_NODE_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_NODE_NOT_FOUND;
pub const FABRIC_E_NODE_IS_UP: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_NODE_IS_UP;
pub const FABRIC_E_NO_WRITE_QUORUM: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_NO_WRITE_QUORUM;
pub const FABRIC_E_NOT_PRIMARY: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_NOT_PRIMARY;
pub const FABRIC_E_NOT_READY: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_NOT_READY;
pub const FABRIC_E_OPERATION_NOT_COMPLETE: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_OPERATION_NOT_COMPLETE;
pub const FABRIC_E_PROPERTY_DOES_NOT_EXIST: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_PROPERTY_DOES_NOT_EXIST;
pub const FABRIC_E_RECONFIGURATION_PENDING: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_RECONFIGURATION_PENDING;
pub const FABRIC_E_REPLICATION_QUEUE_FULL: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_REPLICATION_QUEUE_FULL;
pub const FABRIC_E_SERVICE_ALREADY_EXISTS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SERVICE_ALREADY_EXISTS;
pub const FABRIC_E_SERVICE_DOES_NOT_EXIST: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SERVICE_DOES_NOT_EXIST;
pub const FABRIC_E_SERVICE_OFFLINE: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SERVICE_OFFLINE;
pub const FABRIC_E_SERVICE_METADATA_MISMATCH: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SERVICE_METADATA_MISMATCH;
pub const FABRIC_E_SERVICE_AFFINITY_CHAIN_NOT_SUPPORTED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SERVICE_AFFINITY_CHAIN_NOT_SUPPORTED;
pub const FABRIC_E_SERVICE_TYPE_ALREADY_REGISTERED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SERVICE_TYPE_ALREADY_REGISTERED;
pub const FABRIC_E_SERVICE_TYPE_NOT_REGISTERED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SERVICE_TYPE_NOT_REGISTERED;
pub const FABRIC_E_VALUE_TOO_LARGE: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_VALUE_TOO_LARGE;
pub const FABRIC_E_VALUE_EMPTY: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_VALUE_EMPTY;
pub const FABRIC_E_PROPERTY_CHECK_FAILED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_PROPERTY_CHECK_FAILED;
pub const FABRIC_E_WRITE_CONFLICT: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_WRITE_CONFLICT;
pub const FABRIC_E_ENUMERATION_COMPLETED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_ENUMERATION_COMPLETED;
pub const FABRIC_E_APPLICATION_TYPE_PROVISION_IN_PROGRESS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_APPLICATION_TYPE_PROVISION_IN_PROGRESS;
pub const FABRIC_E_APPLICATION_TYPE_ALREADY_EXISTS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_APPLICATION_TYPE_ALREADY_EXISTS;
pub const FABRIC_E_APPLICATION_TYPE_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_APPLICATION_TYPE_NOT_FOUND;
pub const FABRIC_E_APPLICATION_TYPE_IN_USE: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_APPLICATION_TYPE_IN_USE;
pub const FABRIC_E_APPLICATION_ALREADY_EXISTS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_APPLICATION_ALREADY_EXISTS;
pub const FABRIC_E_APPLICATION_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_APPLICATION_NOT_FOUND;
pub const FABRIC_E_APPLICATION_UPGRADE_IN_PROGRESS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_APPLICATION_UPGRADE_IN_PROGRESS;
pub const FABRIC_E_APPLICATION_UPGRADE_VALIDATION_ERROR: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_APPLICATION_UPGRADE_VALIDATION_ERROR;
pub const FABRIC_E_SERVICE_TYPE_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SERVICE_TYPE_NOT_FOUND;
pub const FABRIC_E_SERVICE_TYPE_MISMATCH: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SERVICE_TYPE_MISMATCH;
pub const FABRIC_E_SERVICE_TYPE_TEMPLATE_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SERVICE_TYPE_TEMPLATE_NOT_FOUND;
pub const FABRIC_E_CONFIGURATION_SECTION_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_CONFIGURATION_SECTION_NOT_FOUND;
pub const FABRIC_E_CONFIGURATION_PARAMETER_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_CONFIGURATION_PARAMETER_NOT_FOUND;
pub const FABRIC_E_INVALID_CONFIGURATION: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_CONFIGURATION;
pub const FABRIC_E_IMAGEBUILDER_VALIDATION_ERROR: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_IMAGEBUILDER_VALIDATION_ERROR;
pub const FABRIC_E_PARTITION_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_PARTITION_NOT_FOUND;
pub const FABRIC_E_REPLICA_DOES_NOT_EXIST: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_REPLICA_DOES_NOT_EXIST;
pub const FABRIC_E_SERVICE_GROUP_ALREADY_EXISTS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SERVICE_GROUP_ALREADY_EXISTS;
pub const FABRIC_E_SERVICE_GROUP_DOES_NOT_EXIST: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SERVICE_GROUP_DOES_NOT_EXIST;
pub const FABRIC_E_PROCESS_DEACTIVATED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_PROCESS_DEACTIVATED;
pub const FABRIC_E_PROCESS_ABORTED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_PROCESS_ABORTED;
pub const FABRIC_E_UPGRADE_FAILED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_UPGRADE_FAILED;
pub const FABRIC_E_INVALID_CREDENTIAL_TYPE: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_CREDENTIAL_TYPE;
pub const FABRIC_E_INVALID_X509_FIND_TYPE: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_X509_FIND_TYPE;
pub const FABRIC_E_INVALID_X509_STORE_LOCATION: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_X509_STORE_LOCATION;
pub const FABRIC_E_INVALID_X509_STORE_NAME: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_X509_STORE_NAME;
pub const FABRIC_E_INVALID_X509_THUMBPRINT: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_X509_THUMBPRINT;
pub const FABRIC_E_INVALID_PROTECTION_LEVEL: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_PROTECTION_LEVEL;
pub const FABRIC_E_INVALID_X509_STORE: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_X509_STORE;
pub const FABRIC_E_INVALID_SUBJECT_NAME: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_SUBJECT_NAME;
pub const FABRIC_E_INVALID_ALLOWED_COMMON_NAME_LIST: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_ALLOWED_COMMON_NAME_LIST;
pub const FABRIC_E_INVALID_CREDENTIALS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_CREDENTIALS;
pub const FABRIC_E_DECRYPTION_FAILED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_DECRYPTION_FAILED;
pub const FABRIC_E_CONFIGURATION_PACKAGE_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_CONFIGURATION_PACKAGE_NOT_FOUND;
pub const FABRIC_E_DATA_PACKAGE_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_DATA_PACKAGE_NOT_FOUND;
pub const FABRIC_E_CODE_PACKAGE_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_CODE_PACKAGE_NOT_FOUND;
pub const FABRIC_E_SERVICE_ENDPOINT_RESOURCE_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SERVICE_ENDPOINT_RESOURCE_NOT_FOUND;
pub const FABRIC_E_INVALID_OPERATION: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_OPERATION;
pub const FABRIC_E_OBJECT_CLOSED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_OBJECT_CLOSED;
pub const FABRIC_E_TIMEOUT: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_TIMEOUT;
pub const FABRIC_E_FILE_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_FILE_NOT_FOUND;
pub const FABRIC_E_DIRECTORY_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_DIRECTORY_NOT_FOUND;
pub const FABRIC_E_INVALID_DIRECTORY: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_DIRECTORY;
pub const FABRIC_E_PATH_TOO_LONG: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_PATH_TOO_LONG;
pub const FABRIC_E_IMAGESTORE_IOERROR: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_IMAGESTORE_IOERROR;
pub const FABRIC_E_CORRUPTED_IMAGE_STORE_OBJECT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_CORRUPTED_IMAGE_STORE_OBJECT_FOUND;
pub const FABRIC_E_APPLICATION_NOT_UPGRADING: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_APPLICATION_NOT_UPGRADING;
pub const FABRIC_E_APPLICATION_ALREADY_IN_TARGET_VERSION: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_APPLICATION_ALREADY_IN_TARGET_VERSION;
pub const FABRIC_E_IMAGEBUILDER_UNEXPECTED_ERROR: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_IMAGEBUILDER_UNEXPECTED_ERROR;
pub const FABRIC_E_FABRIC_VERSION_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_FABRIC_VERSION_NOT_FOUND;
pub const FABRIC_E_FABRIC_VERSION_IN_USE: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_FABRIC_VERSION_IN_USE;
pub const FABRIC_E_FABRIC_VERSION_ALREADY_EXISTS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_FABRIC_VERSION_ALREADY_EXISTS;
pub const FABRIC_E_FABRIC_ALREADY_IN_TARGET_VERSION: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_FABRIC_ALREADY_IN_TARGET_VERSION;
pub const FABRIC_E_FABRIC_NOT_UPGRADING: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_FABRIC_NOT_UPGRADING;
pub const FABRIC_E_FABRIC_UPGRADE_IN_PROGRESS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_FABRIC_UPGRADE_IN_PROGRESS;
pub const FABRIC_E_FABRIC_UPGRADE_VALIDATION_ERROR: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_FABRIC_UPGRADE_VALIDATION_ERROR;
pub const FABRIC_E_HEALTH_MAX_REPORTS_REACHED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_HEALTH_MAX_REPORTS_REACHED;
pub const FABRIC_E_HEALTH_STALE_REPORT: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_HEALTH_STALE_REPORT;
pub const FABRIC_E_KEY_TOO_LARGE: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_KEY_TOO_LARGE;
pub const FABRIC_E_KEY_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_KEY_NOT_FOUND;
pub const FABRIC_E_SEQUENCE_NUMBER_CHECK_FAILED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SEQUENCE_NUMBER_CHECK_FAILED;
pub const FABRIC_E_ENCRYPTION_FAILED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_ENCRYPTION_FAILED;
pub const FABRIC_E_INVALID_ATOMIC_GROUP: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_ATOMIC_GROUP;
pub const FABRIC_E_HEALTH_ENTITY_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_HEALTH_ENTITY_NOT_FOUND;
pub const FABRIC_E_SERVICE_MANIFEST_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SERVICE_MANIFEST_NOT_FOUND;
pub const FABRIC_E_RELIABLE_SESSION_TRANSPORT_STARTUP_FAILURE: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_RELIABLE_SESSION_TRANSPORT_STARTUP_FAILURE;
pub const FABRIC_E_RELIABLE_SESSION_ALREADY_EXISTS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_RELIABLE_SESSION_ALREADY_EXISTS;
pub const FABRIC_E_RELIABLE_SESSION_CANNOT_CONNECT: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_RELIABLE_SESSION_CANNOT_CONNECT;
pub const FABRIC_E_RELIABLE_SESSION_MANAGER_EXISTS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_RELIABLE_SESSION_MANAGER_EXISTS;
pub const FABRIC_E_RELIABLE_SESSION_REJECTED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_RELIABLE_SESSION_REJECTED;
pub const FABRIC_E_RELIABLE_SESSION_MANAGER_ALREADY_LISTENING: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_RELIABLE_SESSION_MANAGER_ALREADY_LISTENING;
pub const FABRIC_E_RELIABLE_SESSION_MANAGER_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_RELIABLE_SESSION_MANAGER_NOT_FOUND;
pub const FABRIC_E_RELIABLE_SESSION_MANAGER_NOT_LISTENING: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_RELIABLE_SESSION_MANAGER_NOT_LISTENING;
pub const FABRIC_E_INVALID_SERVICE_TYPE: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_SERVICE_TYPE;
pub const FABRIC_E_IMAGEBUILDER_TIMEOUT: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_IMAGEBUILDER_TIMEOUT;
pub const FABRIC_E_IMAGEBUILDER_ACCESS_DENIED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_IMAGEBUILDER_ACCESS_DENIED;
pub const FABRIC_E_IMAGEBUILDER_INVALID_MSI_FILE: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_IMAGEBUILDER_INVALID_MSI_FILE;
pub const FABRIC_E_SERVICE_TOO_BUSY: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SERVICE_TOO_BUSY;
pub const FABRIC_E_TRANSACTION_NOT_ACTIVE: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_TRANSACTION_NOT_ACTIVE;
pub const FABRIC_E_REPAIR_TASK_ALREADY_EXISTS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_REPAIR_TASK_ALREADY_EXISTS;
pub const FABRIC_E_REPAIR_TASK_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_REPAIR_TASK_NOT_FOUND;
pub const FABRIC_E_RELIABLE_SESSION_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_RELIABLE_SESSION_NOT_FOUND;
pub const FABRIC_E_RELIABLE_SESSION_QUEUE_EMPTY: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_RELIABLE_SESSION_QUEUE_EMPTY;
pub const FABRIC_E_RELIABLE_SESSION_QUOTA_EXCEEDED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_RELIABLE_SESSION_QUOTA_EXCEEDED;
pub const FABRIC_E_RELIABLE_SESSION_SERVICE_FAULTED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_RELIABLE_SESSION_SERVICE_FAULTED;
pub const FABRIC_E_RELIABLE_SESSION_INVALID_TARGET_PARTITION: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_RELIABLE_SESSION_INVALID_TARGET_PARTITION;
pub const FABRIC_E_TRANSACTION_TOO_LARGE: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_TRANSACTION_TOO_LARGE;
pub const FABRIC_E_REPLICATION_OPERATION_TOO_LARGE: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_REPLICATION_OPERATION_TOO_LARGE;
pub const FABRIC_E_INSTANCE_ID_MISMATCH: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INSTANCE_ID_MISMATCH;
pub const FABRIC_E_UPGRADE_DOMAIN_ALREADY_COMPLETED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_UPGRADE_DOMAIN_ALREADY_COMPLETED;
pub const FABRIC_E_NODE_HAS_NOT_STOPPED_YET: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_NODE_HAS_NOT_STOPPED_YET;
pub const FABRIC_E_INSUFFICIENT_CLUSTER_CAPACITY: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INSUFFICIENT_CLUSTER_CAPACITY;
pub const FABRIC_E_INVALID_PACKAGE_SHARING_POLICY: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_PACKAGE_SHARING_POLICY;
pub const FABRIC_E_PREDEPLOYMENT_NOT_ALLOWED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_PREDEPLOYMENT_NOT_ALLOWED;
pub const FABRIC_E_INVALID_BACKUP_SETTING: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_BACKUP_SETTING;
pub const FABRIC_E_MISSING_FULL_BACKUP: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_MISSING_FULL_BACKUP;
pub const FABRIC_E_BACKUP_IN_PROGRESS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_BACKUP_IN_PROGRESS;
pub const FABRIC_E_DUPLICATE_SERVICE_NOTIFICATION_FILTER_NAME: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_DUPLICATE_SERVICE_NOTIFICATION_FILTER_NAME;
pub const FABRIC_E_INVALID_REPLICA_OPERATION: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_REPLICA_OPERATION;
pub const FABRIC_E_INVALID_REPLICA_STATE: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_REPLICA_STATE;
pub const FABRIC_E_LOADBALANCER_NOT_READY: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_LOADBALANCER_NOT_READY;
pub const FABRIC_E_INVALID_PARTITION_OPERATION: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_PARTITION_OPERATION;
pub const FABRIC_E_PRIMARY_ALREADY_EXISTS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_PRIMARY_ALREADY_EXISTS;
pub const FABRIC_E_SECONDARY_ALREADY_EXISTS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SECONDARY_ALREADY_EXISTS;
pub const FABRIC_E_BACKUP_DIRECTORY_NOT_EMPTY: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_BACKUP_DIRECTORY_NOT_EMPTY;
pub const FABRIC_E_FORCE_NOT_SUPPORTED_FOR_REPLICA_OPERATION: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_FORCE_NOT_SUPPORTED_FOR_REPLICA_OPERATION;
pub const FABRIC_E_ACQUIRE_FILE_LOCK_FAILED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_ACQUIRE_FILE_LOCK_FAILED;
pub const FABRIC_E_CONNECTION_DENIED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_CONNECTION_DENIED;
pub const FABRIC_E_SERVER_AUTHENTICATION_FAILED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SERVER_AUTHENTICATION_FAILED;
pub const FABRIC_E_CONSTRAINT_KEY_UNDEFINED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_CONSTRAINT_KEY_UNDEFINED;
pub const FABRIC_E_MULTITHREADED_TRANSACTIONS_NOT_ALLOWED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_MULTITHREADED_TRANSACTIONS_NOT_ALLOWED;
pub const FABRIC_E_INVALID_X509_NAME_LIST: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_X509_NAME_LIST;
pub const FABRIC_E_VERBOSE_FM_PLACEMENT_HEALTH_REPORTING_REQUIRED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_VERBOSE_FM_PLACEMENT_HEALTH_REPORTING_REQUIRED;
pub const FABRIC_E_GATEWAY_NOT_REACHABLE: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_GATEWAY_NOT_REACHABLE;
pub const FABRIC_E_USER_ROLE_CLIENT_CERTIFICATE_NOT_CONFIGURED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_USER_ROLE_CLIENT_CERTIFICATE_NOT_CONFIGURED;
pub const FABRIC_E_TRANSACTION_ABORTED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_TRANSACTION_ABORTED;
pub const FABRIC_E_CANNOT_CONNECT: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_CANNOT_CONNECT;
pub const FABRIC_E_MESSAGE_TOO_LARGE: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_MESSAGE_TOO_LARGE;
pub const FABRIC_E_CONSTRAINT_NOT_SATISFIED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_CONSTRAINT_NOT_SATISFIED;
pub const FABRIC_E_ENDPOINT_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_ENDPOINT_NOT_FOUND;
pub const FABRIC_E_APPLICATION_UPDATE_IN_PROGRESS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_APPLICATION_UPDATE_IN_PROGRESS;
pub const FABRIC_E_DELETE_BACKUP_FILE_FAILED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_DELETE_BACKUP_FILE_FAILED;
pub const FABRIC_E_CONNECTION_CLOSED_BY_REMOTE_END: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_CONNECTION_CLOSED_BY_REMOTE_END;
pub const FABRIC_E_INVALID_TEST_COMMAND_STATE: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_TEST_COMMAND_STATE;
pub const FABRIC_E_TEST_COMMAND_OPERATION_ID_ALREADY_EXISTS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_TEST_COMMAND_OPERATION_ID_ALREADY_EXISTS;
pub const FABRIC_E_CM_OPERATION_FAILED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_CM_OPERATION_FAILED;
pub const FABRIC_E_IMAGEBUILDER_RESERVED_DIRECTORY_ERROR: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_IMAGEBUILDER_RESERVED_DIRECTORY_ERROR;
pub const FABRIC_E_CERTIFICATE_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_CERTIFICATE_NOT_FOUND;
pub const FABRIC_E_CHAOS_ALREADY_RUNNING: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_CHAOS_ALREADY_RUNNING;
pub const FABRIC_E_FABRIC_DATA_ROOT_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_FABRIC_DATA_ROOT_NOT_FOUND;
pub const FABRIC_E_INVALID_RESTORE_DATA: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_RESTORE_DATA;
pub const FABRIC_E_DUPLICATE_BACKUPS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_DUPLICATE_BACKUPS;
pub const FABRIC_E_INVALID_BACKUP_CHAIN: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_BACKUP_CHAIN;
pub const FABRIC_E_STOP_IN_PROGRESS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_STOP_IN_PROGRESS;
pub const FABRIC_E_ALREADY_STOPPED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_ALREADY_STOPPED;
pub const FABRIC_E_NODE_IS_DOWN: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_NODE_IS_DOWN;
pub const FABRIC_E_NODE_TRANSITION_IN_PROGRESS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_NODE_TRANSITION_IN_PROGRESS;
pub const FABRIC_E_INVALID_BACKUP: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_BACKUP;
pub const FABRIC_E_INVALID_INSTANCE_ID: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_INSTANCE_ID;
pub const FABRIC_E_INVALID_DURATION: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_DURATION;
pub const FABRIC_E_RESTORE_SAFE_CHECK_FAILED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_RESTORE_SAFE_CHECK_FAILED;
pub const FABRIC_E_CONFIG_UPGRADE_FAILED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_CONFIG_UPGRADE_FAILED;
pub const FABRIC_E_UPLOAD_SESSION_RANGE_NOT_SATISFIABLE: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_UPLOAD_SESSION_RANGE_NOT_SATISFIABLE;
pub const FABRIC_E_UPLOAD_SESSION_ID_CONFLICT: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_UPLOAD_SESSION_ID_CONFLICT;
pub const FABRIC_E_INVALID_PARTITION_SELECTOR: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_PARTITION_SELECTOR;
pub const FABRIC_E_INVALID_REPLICA_SELECTOR: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_REPLICA_SELECTOR;
pub const FABRIC_E_DNS_SERVICE_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_DNS_SERVICE_NOT_FOUND;
pub const FABRIC_E_INVALID_DNS_NAME: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_DNS_NAME;
pub const FABRIC_E_DNS_NAME_IN_USE: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_DNS_NAME_IN_USE;
pub const FABRIC_E_COMPOSE_DEPLOYMENT_ALREADY_EXISTS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_COMPOSE_DEPLOYMENT_ALREADY_EXISTS;
pub const FABRIC_E_COMPOSE_DEPLOYMENT_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_COMPOSE_DEPLOYMENT_NOT_FOUND;
pub const FABRIC_E_INVALID_FOR_STATEFUL_SERVICES: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_FOR_STATEFUL_SERVICES;
pub const FABRIC_E_INVALID_FOR_STATELESS_SERVICES: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_FOR_STATELESS_SERVICES;
pub const FABRIC_E_ONLY_VALID_FOR_STATEFUL_PERSISTENT_SERVICES: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_ONLY_VALID_FOR_STATEFUL_PERSISTENT_SERVICES;
pub const FABRIC_E_INVALID_UPLOAD_SESSION_ID: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_UPLOAD_SESSION_ID;
pub const FABRIC_E_BACKUP_NOT_ENABLED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_BACKUP_NOT_ENABLED;
pub const FABRIC_E_BACKUP_IS_ENABLED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_BACKUP_IS_ENABLED;
pub const FABRIC_E_BACKUP_POLICY_DOES_NOT_EXIST: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_BACKUP_POLICY_DOES_NOT_EXIST;
pub const FABRIC_E_BACKUP_POLICY_ALREADY_EXISTS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_BACKUP_POLICY_ALREADY_EXISTS;
pub const FABRIC_E_RESTORE_IN_PROGRESS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_RESTORE_IN_PROGRESS;
pub const FABRIC_E_RESTORE_SOURCE_TARGET_PARTITION_MISMATCH: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_RESTORE_SOURCE_TARGET_PARTITION_MISMATCH;
pub const FABRIC_E_FAULT_ANALYSIS_SERVICE_NOT_ENABLED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_FAULT_ANALYSIS_SERVICE_NOT_ENABLED;
pub const FABRIC_E_CONTAINER_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_CONTAINER_NOT_FOUND;
pub const FABRIC_E_OBJECT_DISPOSED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_OBJECT_DISPOSED;
pub const FABRIC_E_NOT_READABLE: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_NOT_READABLE;
pub const FABRIC_E_BACKUPCOPIER_UNEXPECTED_ERROR: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_BACKUPCOPIER_UNEXPECTED_ERROR;
pub const FABRIC_E_BACKUPCOPIER_TIMEOUT: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_BACKUPCOPIER_TIMEOUT;
pub const FABRIC_E_BACKUPCOPIER_ACCESS_DENIED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_BACKUPCOPIER_ACCESS_DENIED;
pub const FABRIC_E_INVALID_SERVICE_SCALING_POLICY: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INVALID_SERVICE_SCALING_POLICY;
pub const FABRIC_E_SINGLE_INSTANCE_APPLICATION_ALREADY_EXISTS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SINGLE_INSTANCE_APPLICATION_ALREADY_EXISTS;
pub const FABRIC_E_SINGLE_INSTANCE_APPLICATION_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SINGLE_INSTANCE_APPLICATION_NOT_FOUND;
pub const FABRIC_E_VOLUME_ALREADY_EXISTS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_VOLUME_ALREADY_EXISTS;
pub const FABRIC_E_VOLUME_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_VOLUME_NOT_FOUND;
pub const FABRIC_E_DATABASE_MIGRATION_IN_PROGRESS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_DATABASE_MIGRATION_IN_PROGRESS;
pub const FABRIC_E_CENTRAL_SECRET_SERVICE_GENERIC: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_CENTRAL_SECRET_SERVICE_GENERIC;
pub const FABRIC_E_SECRET_INVALID: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SECRET_INVALID;
pub const FABRIC_E_SECRET_VERSION_ALREADY_EXISTS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SECRET_VERSION_ALREADY_EXISTS;
pub const FABRIC_E_SINGLE_INSTANCE_APPLICATION_UPGRADE_IN_PROGRESS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SINGLE_INSTANCE_APPLICATION_UPGRADE_IN_PROGRESS;
pub const FABRIC_E_OPERATION_NOT_SUPPORTED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_OPERATION_NOT_SUPPORTED;
pub const FABRIC_E_COMPOSE_DEPLOYMENT_NOT_UPGRADING: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_COMPOSE_DEPLOYMENT_NOT_UPGRADING;
pub const FABRIC_E_SECRET_TYPE_CANNOT_BE_CHANGED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SECRET_TYPE_CANNOT_BE_CHANGED;
pub const FABRIC_E_NETWORK_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_NETWORK_NOT_FOUND;
pub const FABRIC_E_NETWORK_IN_USE: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_NETWORK_IN_USE;
pub const FABRIC_E_ENDPOINT_NOT_REFERENCED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_ENDPOINT_NOT_REFERENCED;
pub const FABRIC_E_LAST_USED_HRESULT: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_LAST_USED_HRESULT;
pub const FABRIC_E_FACILITY_SF_FIRST_HRESULT: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_FACILITY_SF_FIRST_HRESULT;
pub const FABRIC_E_INSTANCE_ALREADY_EXISTS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INSTANCE_ALREADY_EXISTS;
pub const FABRIC_E_NODE_TYPE_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_NODE_TYPE_NOT_FOUND;
pub const FABRIC_E_INSTANCE_COUNT_UPDATE_NOT_ALLOWED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INSTANCE_COUNT_UPDATE_NOT_ALLOWED;
pub const FABRIC_E_COPY_ABORTED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_COPY_ABORTED;
pub const FABRIC_E_AUXILIARY_ALREADY_EXISTS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_AUXILIARY_ALREADY_EXISTS;
pub const FABRIC_E_AUXILIARY_FEATURE_DISABLED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_AUXILIARY_FEATURE_DISABLED;
pub const FABRIC_E_RUN_TO_COMPLETION_INCOMPATIBLE_WITH_SHARED_PROCESS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_RUN_TO_COMPLETION_INCOMPATIBLE_WITH_SHARED_PROCESS;
pub const FABRIC_E_VERSION_STORE_OUT_OF_MEMORY: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_VERSION_STORE_OUT_OF_MEMORY;
pub const FABRIC_E_BACKUP_NOT_FOUND: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_BACKUP_NOT_FOUND;
pub const FABRIC_E_SKIP_RESTORE_OPERATION: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SKIP_RESTORE_OPERATION;
pub const FABRIC_E_STORE_OUT_OF_SESSIONS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_STORE_OUT_OF_SESSIONS;
pub const FABRIC_E_RESTORE_WAITING_FOR_USER_INTERVENTION: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_RESTORE_WAITING_FOR_USER_INTERVENTION;
pub const FABRIC_E_DATABASE_FILES_CORRUPTED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_DATABASE_FILES_CORRUPTED;
pub const FABRIC_E_INSUFFICIENT_MAX_LOAD_CAPACITY: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INSUFFICIENT_MAX_LOAD_CAPACITY;
pub const FABRIC_E_STORE_DISK_ERROR: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_STORE_DISK_ERROR;
pub const FABRIC_E_SERVICE_ALREADY_IN_REQUESTED_STATE: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SERVICE_ALREADY_IN_REQUESTED_STATE;
pub const FABRIC_E_DISABLE_ENABLE_SERVICE_FEATURE_DISABLED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_DISABLE_ENABLE_SERVICE_FEATURE_DISABLED;
pub const FABRIC_E_MAX_ALLOWED_DISABLED_SERVICES_REACHED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_MAX_ALLOWED_DISABLED_SERVICES_REACHED;
pub const FABRIC_E_SERVICE_DISABLED: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SERVICE_DISABLED;
pub const FABRIC_E_SERVICE_DISABLE_IN_PROGRESS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_SERVICE_DISABLE_IN_PROGRESS;
pub const FABRIC_E_STORE_OUT_OF_LONG_VALUE_IDS: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_STORE_OUT_OF_LONG_VALUE_IDS;
pub const FABRIC_E_STORE_OUT_OF_INSTANCES: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_STORE_OUT_OF_INSTANCES;
pub const FABRIC_E_STORE_SERIALIZED_STREAM_NULL: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_STORE_SERIALIZED_STREAM_NULL;
pub const FABRIC_E_STORE_SERIALIZATION_ERROR: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_STORE_SERIALIZATION_ERROR;
pub const FABRIC_E_INCOMPATIBLE_EXCLUSIVE_SELF_RECONFIGURING: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_INCOMPATIBLE_EXCLUSIVE_SELF_RECONFIGURING;
pub const FABRIC_E_LAST_USED_FACILITY_SF_HRESULT: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_LAST_USED_FACILITY_SF_HRESULT;
pub const FABRIC_E_FACILITY_SF_LAST_HRESULT: FABRIC_ERROR_CODE = FABRIC_ERROR_CODE::FABRIC_E_FACILITY_SF_LAST_HRESULT;
pub const FABRIC_EXECUTION_POLICY_EXECUTION_TYPE_RUN_ALWAYS: FABRIC_EXECUTION_POLICY_EXECUTION_TYPE = FABRIC_EXECUTION_POLICY_EXECUTION_TYPE::FABRIC_EXECUTION_POLICY_EXECUTION_TYPE_RUN_ALWAYS;
pub const FABRIC_EXECUTION_POLICY_EXECUTION_TYPE_RUN_TO_COMPLETION: FABRIC_EXECUTION_POLICY_EXECUTION_TYPE = FABRIC_EXECUTION_POLICY_EXECUTION_TYPE::FABRIC_EXECUTION_POLICY_EXECUTION_TYPE_RUN_TO_COMPLETION;
pub const FABRIC_EXECUTION_POLICY_RESTART_POLICY_ALWAYS: FABRIC_EXECUTION_POLICY_RESTART_POLICY = FABRIC_EXECUTION_POLICY_RESTART_POLICY::FABRIC_EXECUTION_POLICY_RESTART_POLICY_ALWAYS;
pub const FABRIC_EXECUTION_POLICY_RESTART_POLICY_ON_FAILURE: FABRIC_EXECUTION_POLICY_RESTART_POLICY = FABRIC_EXECUTION_POLICY_RESTART_POLICY::FABRIC_EXECUTION_POLICY_RESTART_POLICY_ON_FAILURE;
pub const FABRIC_EXECUTION_POLICY_RESTART_POLICY_NEVER: FABRIC_EXECUTION_POLICY_RESTART_POLICY = FABRIC_EXECUTION_POLICY_RESTART_POLICY::FABRIC_EXECUTION_POLICY_RESTART_POLICY_NEVER;
pub const FABRIC_EXEHOST_WORKING_FOLDER_INVALID: FABRIC_EXEHOST_WORKING_FOLDER = FABRIC_EXEHOST_WORKING_FOLDER::FABRIC_EXEHOST_WORKING_FOLDER_INVALID;
pub const FABRIC_EXEHOST_WORKING_FOLDER_WORK: FABRIC_EXEHOST_WORKING_FOLDER = FABRIC_EXEHOST_WORKING_FOLDER::FABRIC_EXEHOST_WORKING_FOLDER_WORK;
pub const FABRIC_EXEHOST_WORKING_FOLDER_CODE_PACKAGE: FABRIC_EXEHOST_WORKING_FOLDER = FABRIC_EXEHOST_WORKING_FOLDER::FABRIC_EXEHOST_WORKING_FOLDER_CODE_PACKAGE;
pub const FABRIC_EXEHOST_WORKING_FOLDER_CODE_BASE: FABRIC_EXEHOST_WORKING_FOLDER = FABRIC_EXEHOST_WORKING_FOLDER::FABRIC_EXEHOST_WORKING_FOLDER_CODE_BASE;
pub const FABRIC_FAULT_TYPE_INVALID: FABRIC_FAULT_TYPE = FABRIC_FAULT_TYPE::FABRIC_FAULT_TYPE_INVALID;
pub const FABRIC_FAULT_TYPE_PERMANENT: FABRIC_FAULT_TYPE = FABRIC_FAULT_TYPE::FABRIC_FAULT_TYPE_PERMANENT;
pub const FABRIC_FAULT_TYPE_TRANSIENT: FABRIC_FAULT_TYPE = FABRIC_FAULT_TYPE::FABRIC_FAULT_TYPE_TRANSIENT;
pub const FABRIC_HEALTH_ENTITY_KIND_INVALID: FABRIC_HEALTH_ENTITY_KIND = FABRIC_HEALTH_ENTITY_KIND::FABRIC_HEALTH_ENTITY_KIND_INVALID;
pub const FABRIC_HEALTH_ENTITY_KIND_NODE: FABRIC_HEALTH_ENTITY_KIND = FABRIC_HEALTH_ENTITY_KIND::FABRIC_HEALTH_ENTITY_KIND_NODE;
pub const FABRIC_HEALTH_ENTITY_KIND_PARTITION: FABRIC_HEALTH_ENTITY_KIND = FABRIC_HEALTH_ENTITY_KIND::FABRIC_HEALTH_ENTITY_KIND_PARTITION;
pub const FABRIC_HEALTH_ENTITY_KIND_SERVICE: FABRIC_HEALTH_ENTITY_KIND = FABRIC_HEALTH_ENTITY_KIND::FABRIC_HEALTH_ENTITY_KIND_SERVICE;
pub const FABRIC_HEALTH_ENTITY_KIND_APPLICATION: FABRIC_HEALTH_ENTITY_KIND = FABRIC_HEALTH_ENTITY_KIND::FABRIC_HEALTH_ENTITY_KIND_APPLICATION;
pub const FABRIC_HEALTH_ENTITY_KIND_REPLICA: FABRIC_HEALTH_ENTITY_KIND = FABRIC_HEALTH_ENTITY_KIND::FABRIC_HEALTH_ENTITY_KIND_REPLICA;
pub const FABRIC_HEALTH_ENTITY_KIND_DEPLOYED_APPLICATION: FABRIC_HEALTH_ENTITY_KIND = FABRIC_HEALTH_ENTITY_KIND::FABRIC_HEALTH_ENTITY_KIND_DEPLOYED_APPLICATION;
pub const FABRIC_HEALTH_ENTITY_KIND_DEPLOYED_SERVICE_PACKAGE: FABRIC_HEALTH_ENTITY_KIND = FABRIC_HEALTH_ENTITY_KIND::FABRIC_HEALTH_ENTITY_KIND_DEPLOYED_SERVICE_PACKAGE;
pub const FABRIC_HEALTH_ENTITY_KIND_CLUSTER: FABRIC_HEALTH_ENTITY_KIND = FABRIC_HEALTH_ENTITY_KIND::FABRIC_HEALTH_ENTITY_KIND_CLUSTER;
pub const FABRIC_HEALTH_EVALUATION_KIND_INVALID: FABRIC_HEALTH_EVALUATION_KIND = FABRIC_HEALTH_EVALUATION_KIND::FABRIC_HEALTH_EVALUATION_KIND_INVALID;
pub const FABRIC_HEALTH_EVALUATION_KIND_EVENT: FABRIC_HEALTH_EVALUATION_KIND = FABRIC_HEALTH_EVALUATION_KIND::FABRIC_HEALTH_EVALUATION_KIND_EVENT;
pub const FABRIC_HEALTH_EVALUATION_KIND_REPLICAS: FABRIC_HEALTH_EVALUATION_KIND = FABRIC_HEALTH_EVALUATION_KIND::FABRIC_HEALTH_EVALUATION_KIND_REPLICAS;
pub const FABRIC_HEALTH_EVALUATION_KIND_PARTITIONS: FABRIC_HEALTH_EVALUATION_KIND = FABRIC_HEALTH_EVALUATION_KIND::FABRIC_HEALTH_EVALUATION_KIND_PARTITIONS;
pub const FABRIC_HEALTH_EVALUATION_KIND_DEPLOYED_SERVICE_PACKAGES: FABRIC_HEALTH_EVALUATION_KIND = FABRIC_HEALTH_EVALUATION_KIND::FABRIC_HEALTH_EVALUATION_KIND_DEPLOYED_SERVICE_PACKAGES;
pub const FABRIC_HEALTH_EVALUATION_KIND_DEPLOYED_APPLICATIONS: FABRIC_HEALTH_EVALUATION_KIND = FABRIC_HEALTH_EVALUATION_KIND::FABRIC_HEALTH_EVALUATION_KIND_DEPLOYED_APPLICATIONS;
pub const FABRIC_HEALTH_EVALUATION_KIND_SERVICES: FABRIC_HEALTH_EVALUATION_KIND = FABRIC_HEALTH_EVALUATION_KIND::FABRIC_HEALTH_EVALUATION_KIND_SERVICES;
pub const FABRIC_HEALTH_EVALUATION_KIND_NODES: FABRIC_HEALTH_EVALUATION_KIND = FABRIC_HEALTH_EVALUATION_KIND::FABRIC_HEALTH_EVALUATION_KIND_NODES;
pub const FABRIC_HEALTH_EVALUATION_KIND_APPLICATIONS: FABRIC_HEALTH_EVALUATION_KIND = FABRIC_HEALTH_EVALUATION_KIND::FABRIC_HEALTH_EVALUATION_KIND_APPLICATIONS;
pub const FABRIC_HEALTH_EVALUATION_KIND_SYSTEM_APPLICATION: FABRIC_HEALTH_EVALUATION_KIND = FABRIC_HEALTH_EVALUATION_KIND::FABRIC_HEALTH_EVALUATION_KIND_SYSTEM_APPLICATION;
pub const FABRIC_HEALTH_EVALUATION_KIND_UPGRADE_DOMAIN_DEPLOYED_APPLICATIONS: FABRIC_HEALTH_EVALUATION_KIND = FABRIC_HEALTH_EVALUATION_KIND::FABRIC_HEALTH_EVALUATION_KIND_UPGRADE_DOMAIN_DEPLOYED_APPLICATIONS;
pub const FABRIC_HEALTH_EVALUATION_KIND_UPGRADE_DOMAIN_NODES: FABRIC_HEALTH_EVALUATION_KIND = FABRIC_HEALTH_EVALUATION_KIND::FABRIC_HEALTH_EVALUATION_KIND_UPGRADE_DOMAIN_NODES;
pub const FABRIC_HEALTH_EVALUATION_KIND_NODE: FABRIC_HEALTH_EVALUATION_KIND = FABRIC_HEALTH_EVALUATION_KIND::FABRIC_HEALTH_EVALUATION_KIND_NODE;
pub const FABRIC_HEALTH_EVALUATION_KIND_REPLICA: FABRIC_HEALTH_EVALUATION_KIND = FABRIC_HEALTH_EVALUATION_KIND::FABRIC_HEALTH_EVALUATION_KIND_REPLICA;
pub const FABRIC_HEALTH_EVALUATION_KIND_PARTITION: FABRIC_HEALTH_EVALUATION_KIND = FABRIC_HEALTH_EVALUATION_KIND::FABRIC_HEALTH_EVALUATION_KIND_PARTITION;
pub const FABRIC_HEALTH_EVALUATION_KIND_SERVICE: FABRIC_HEALTH_EVALUATION_KIND = FABRIC_HEALTH_EVALUATION_KIND::FABRIC_HEALTH_EVALUATION_KIND_SERVICE;
pub const FABRIC_HEALTH_EVALUATION_KIND_DEPLOYED_SERVICE_PACKAGE: FABRIC_HEALTH_EVALUATION_KIND = FABRIC_HEALTH_EVALUATION_KIND::FABRIC_HEALTH_EVALUATION_KIND_DEPLOYED_SERVICE_PACKAGE;
pub const FABRIC_HEALTH_EVALUATION_KIND_DEPLOYED_APPLICATION: FABRIC_HEALTH_EVALUATION_KIND = FABRIC_HEALTH_EVALUATION_KIND::FABRIC_HEALTH_EVALUATION_KIND_DEPLOYED_APPLICATION;
pub const FABRIC_HEALTH_EVALUATION_KIND_APPLICATION: FABRIC_HEALTH_EVALUATION_KIND = FABRIC_HEALTH_EVALUATION_KIND::FABRIC_HEALTH_EVALUATION_KIND_APPLICATION;
pub const FABRIC_HEALTH_EVALUATION_KIND_DELTA_NODES_CHECK: FABRIC_HEALTH_EVALUATION_KIND = FABRIC_HEALTH_EVALUATION_KIND::FABRIC_HEALTH_EVALUATION_KIND_DELTA_NODES_CHECK;
pub const FABRIC_HEALTH_EVALUATION_KIND_UPGRADE_DOMAIN_DELTA_NODES_CHECK: FABRIC_HEALTH_EVALUATION_KIND = FABRIC_HEALTH_EVALUATION_KIND::FABRIC_HEALTH_EVALUATION_KIND_UPGRADE_DOMAIN_DELTA_NODES_CHECK;
pub const FABRIC_HEALTH_EVALUATION_KIND_APPLICATION_TYPE_APPLICATIONS: FABRIC_HEALTH_EVALUATION_KIND = FABRIC_HEALTH_EVALUATION_KIND::FABRIC_HEALTH_EVALUATION_KIND_APPLICATION_TYPE_APPLICATIONS;
pub const FABRIC_HEALTH_EVALUATION_KIND_NODE_TYPE_NODES: FABRIC_HEALTH_EVALUATION_KIND = FABRIC_HEALTH_EVALUATION_KIND::FABRIC_HEALTH_EVALUATION_KIND_NODE_TYPE_NODES;
pub const FABRIC_HEALTH_REPORT_KIND_INVALID: FABRIC_HEALTH_REPORT_KIND = FABRIC_HEALTH_REPORT_KIND::FABRIC_HEALTH_REPORT_KIND_INVALID;
pub const FABRIC_HEALTH_REPORT_KIND_STATEFUL_SERVICE_REPLICA: FABRIC_HEALTH_REPORT_KIND = FABRIC_HEALTH_REPORT_KIND::FABRIC_HEALTH_REPORT_KIND_STATEFUL_SERVICE_REPLICA;
pub const FABRIC_HEALTH_REPORT_KIND_STATELESS_SERVICE_INSTANCE: FABRIC_HEALTH_REPORT_KIND = FABRIC_HEALTH_REPORT_KIND::FABRIC_HEALTH_REPORT_KIND_STATELESS_SERVICE_INSTANCE;
pub const FABRIC_HEALTH_REPORT_KIND_PARTITION: FABRIC_HEALTH_REPORT_KIND = FABRIC_HEALTH_REPORT_KIND::FABRIC_HEALTH_REPORT_KIND_PARTITION;
pub const FABRIC_HEALTH_REPORT_KIND_NODE: FABRIC_HEALTH_REPORT_KIND = FABRIC_HEALTH_REPORT_KIND::FABRIC_HEALTH_REPORT_KIND_NODE;
pub const FABRIC_HEALTH_REPORT_KIND_SERVICE: FABRIC_HEALTH_REPORT_KIND = FABRIC_HEALTH_REPORT_KIND::FABRIC_HEALTH_REPORT_KIND_SERVICE;
pub const FABRIC_HEALTH_REPORT_KIND_APPLICATION: FABRIC_HEALTH_REPORT_KIND = FABRIC_HEALTH_REPORT_KIND::FABRIC_HEALTH_REPORT_KIND_APPLICATION;
pub const FABRIC_HEALTH_REPORT_KIND_DEPLOYED_APPLICATION: FABRIC_HEALTH_REPORT_KIND = FABRIC_HEALTH_REPORT_KIND::FABRIC_HEALTH_REPORT_KIND_DEPLOYED_APPLICATION;
pub const FABRIC_HEALTH_REPORT_KIND_DEPLOYED_SERVICE_PACKAGE: FABRIC_HEALTH_REPORT_KIND = FABRIC_HEALTH_REPORT_KIND::FABRIC_HEALTH_REPORT_KIND_DEPLOYED_SERVICE_PACKAGE;
pub const FABRIC_HEALTH_REPORT_KIND_CLUSTER: FABRIC_HEALTH_REPORT_KIND = FABRIC_HEALTH_REPORT_KIND::FABRIC_HEALTH_REPORT_KIND_CLUSTER;
pub const FABRIC_HEALTH_REPORT_KIND_SELF_RECONFIGURING_SERVICE_INSTANCE: FABRIC_HEALTH_REPORT_KIND = FABRIC_HEALTH_REPORT_KIND::FABRIC_HEALTH_REPORT_KIND_SELF_RECONFIGURING_SERVICE_INSTANCE;
pub const FABRIC_HEALTH_STATE_INVALID: FABRIC_HEALTH_STATE = FABRIC_HEALTH_STATE::FABRIC_HEALTH_STATE_INVALID;
pub const FABRIC_HEALTH_STATE_OK: FABRIC_HEALTH_STATE = FABRIC_HEALTH_STATE::FABRIC_HEALTH_STATE_OK;
pub const FABRIC_HEALTH_STATE_WARNING: FABRIC_HEALTH_STATE = FABRIC_HEALTH_STATE::FABRIC_HEALTH_STATE_WARNING;
pub const FABRIC_HEALTH_STATE_ERROR: FABRIC_HEALTH_STATE = FABRIC_HEALTH_STATE::FABRIC_HEALTH_STATE_ERROR;
pub const FABRIC_HEALTH_STATE_UNKNOWN: FABRIC_HEALTH_STATE = FABRIC_HEALTH_STATE::FABRIC_HEALTH_STATE_UNKNOWN;
pub const FABRIC_HEALTH_STATE_FILTER_DEFAULT: FABRIC_HEALTH_STATE_FILTER = FABRIC_HEALTH_STATE_FILTER::FABRIC_HEALTH_STATE_FILTER_DEFAULT;
pub const FABRIC_HEALTH_STATE_FILTER_NONE: FABRIC_HEALTH_STATE_FILTER = FABRIC_HEALTH_STATE_FILTER::FABRIC_HEALTH_STATE_FILTER_NONE;
pub const FABRIC_HEALTH_STATE_FILTER_OK: FABRIC_HEALTH_STATE_FILTER = FABRIC_HEALTH_STATE_FILTER::FABRIC_HEALTH_STATE_FILTER_OK;
pub const FABRIC_HEALTH_STATE_FILTER_WARNING: FABRIC_HEALTH_STATE_FILTER = FABRIC_HEALTH_STATE_FILTER::FABRIC_HEALTH_STATE_FILTER_WARNING;
pub const FABRIC_HEALTH_STATE_FILTER_ERROR: FABRIC_HEALTH_STATE_FILTER = FABRIC_HEALTH_STATE_FILTER::FABRIC_HEALTH_STATE_FILTER_ERROR;
pub const FABRIC_HEALTH_STATE_FILTER_ALL: FABRIC_HEALTH_STATE_FILTER = FABRIC_HEALTH_STATE_FILTER::FABRIC_HEALTH_STATE_FILTER_ALL;
pub const FABRIC_HOST_ISOLATION_MODE_NONE: FABRIC_HOST_ISOLATION_MODE = FABRIC_HOST_ISOLATION_MODE::FABRIC_HOST_ISOLATION_MODE_NONE;
pub const FABRIC_HOST_ISOLATION_MODE_PROCESS: FABRIC_HOST_ISOLATION_MODE = FABRIC_HOST_ISOLATION_MODE::FABRIC_HOST_ISOLATION_MODE_PROCESS;
pub const FABRIC_HOST_ISOLATION_MODE_HYPER_V: FABRIC_HOST_ISOLATION_MODE = FABRIC_HOST_ISOLATION_MODE::FABRIC_HOST_ISOLATION_MODE_HYPER_V;
pub const FABRIC_HOST_TYPE_INVALID: FABRIC_HOST_TYPE = FABRIC_HOST_TYPE::FABRIC_HOST_TYPE_INVALID;
pub const FABRIC_HOST_TYPE_EXE_HOST: FABRIC_HOST_TYPE = FABRIC_HOST_TYPE::FABRIC_HOST_TYPE_EXE_HOST;
pub const FABRIC_HOST_TYPE_CONTAINER_HOST: FABRIC_HOST_TYPE = FABRIC_HOST_TYPE::FABRIC_HOST_TYPE_CONTAINER_HOST;
pub const FABRIC_KEY_VALUE_STORE_FULL_COPY_MODE_DEFAULT: FABRIC_KEY_VALUE_STORE_FULL_COPY_MODE = FABRIC_KEY_VALUE_STORE_FULL_COPY_MODE::FABRIC_KEY_VALUE_STORE_FULL_COPY_MODE_DEFAULT;
pub const FABRIC_KEY_VALUE_STORE_FULL_COPY_MODE_PHYSICAL: FABRIC_KEY_VALUE_STORE_FULL_COPY_MODE = FABRIC_KEY_VALUE_STORE_FULL_COPY_MODE::FABRIC_KEY_VALUE_STORE_FULL_COPY_MODE_PHYSICAL;
pub const FABRIC_KEY_VALUE_STORE_FULL_COPY_MODE_LOGICAL: FABRIC_KEY_VALUE_STORE_FULL_COPY_MODE = FABRIC_KEY_VALUE_STORE_FULL_COPY_MODE::FABRIC_KEY_VALUE_STORE_FULL_COPY_MODE_LOGICAL;
pub const FABRIC_KEY_VALUE_STORE_FULL_COPY_MODE_REBUILD: FABRIC_KEY_VALUE_STORE_FULL_COPY_MODE = FABRIC_KEY_VALUE_STORE_FULL_COPY_MODE::FABRIC_KEY_VALUE_STORE_FULL_COPY_MODE_REBUILD;
pub const FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE_INACTIVE: FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE = FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE::FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE_INACTIVE;
pub const FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE_MIGRATION: FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE = FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE::FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE_MIGRATION;
pub const FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE_TARGET_DATABASE_SWAP: FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE = FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE::FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE_TARGET_DATABASE_SWAP;
pub const FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE_TARGET_DATABASE_CLEANUP: FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE = FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE::FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE_TARGET_DATABASE_CLEANUP;
pub const FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE_SOURCE_DATABASE_CLEANUP: FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE = FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE::FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE_SOURCE_DATABASE_CLEANUP;
pub const FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE_TARGET_DATABASE_ACTIVE: FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE = FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE::FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE_TARGET_DATABASE_ACTIVE;
pub const FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE_RESTORE_SOURCE_BACKUP: FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE = FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE::FABRIC_KEY_VALUE_STORE_MIGRATION_PHASE_RESTORE_SOURCE_BACKUP;
pub const FABRIC_KEY_VALUE_STORE_MIGRATION_STATE_INACTIVE: FABRIC_KEY_VALUE_STORE_MIGRATION_STATE = FABRIC_KEY_VALUE_STORE_MIGRATION_STATE::FABRIC_KEY_VALUE_STORE_MIGRATION_STATE_INACTIVE;
pub const FABRIC_KEY_VALUE_STORE_MIGRATION_STATE_PROCESSING: FABRIC_KEY_VALUE_STORE_MIGRATION_STATE = FABRIC_KEY_VALUE_STORE_MIGRATION_STATE::FABRIC_KEY_VALUE_STORE_MIGRATION_STATE_PROCESSING;
pub const FABRIC_KEY_VALUE_STORE_MIGRATION_STATE_COMPLETED: FABRIC_KEY_VALUE_STORE_MIGRATION_STATE = FABRIC_KEY_VALUE_STORE_MIGRATION_STATE::FABRIC_KEY_VALUE_STORE_MIGRATION_STATE_COMPLETED;
pub const FABRIC_KEY_VALUE_STORE_MIGRATION_STATE_CANCELED: FABRIC_KEY_VALUE_STORE_MIGRATION_STATE = FABRIC_KEY_VALUE_STORE_MIGRATION_STATE::FABRIC_KEY_VALUE_STORE_MIGRATION_STATE_CANCELED;
pub const FABRIC_KEY_VALUE_STORE_MIGRATION_STATE_FAILED: FABRIC_KEY_VALUE_STORE_MIGRATION_STATE = FABRIC_KEY_VALUE_STORE_MIGRATION_STATE::FABRIC_KEY_VALUE_STORE_MIGRATION_STATE_FAILED;
pub const FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE_INVALID: FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE = FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE::FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE_INVALID;
pub const FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE_NONE: FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE = FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE::FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE_NONE;
pub const FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE_NON_BLOCKING_QUORUM_ACKED: FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE = FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE::FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE_NON_BLOCKING_QUORUM_ACKED;
pub const FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE_BLOCK_SECONDARY_ACK: FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE = FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE::FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE_BLOCK_SECONDARY_ACK;
pub const FABRIC_KEY_VALUE_STORE_PROVIDER_KIND_UNKNOWN: FABRIC_KEY_VALUE_STORE_PROVIDER_KIND = FABRIC_KEY_VALUE_STORE_PROVIDER_KIND::FABRIC_KEY_VALUE_STORE_PROVIDER_KIND_UNKNOWN;
pub const FABRIC_KEY_VALUE_STORE_PROVIDER_KIND_ESE: FABRIC_KEY_VALUE_STORE_PROVIDER_KIND = FABRIC_KEY_VALUE_STORE_PROVIDER_KIND::FABRIC_KEY_VALUE_STORE_PROVIDER_KIND_ESE;
pub const FABRIC_KEY_VALUE_STORE_PROVIDER_KIND_TSTORE: FABRIC_KEY_VALUE_STORE_PROVIDER_KIND = FABRIC_KEY_VALUE_STORE_PROVIDER_KIND::FABRIC_KEY_VALUE_STORE_PROVIDER_KIND_TSTORE;
pub const FABRIC_LOCAL_STORE_KIND_INVALID: FABRIC_LOCAL_STORE_KIND = FABRIC_LOCAL_STORE_KIND::FABRIC_LOCAL_STORE_KIND_INVALID;
pub const FABRIC_LOCAL_STORE_KIND_ESE: FABRIC_LOCAL_STORE_KIND = FABRIC_LOCAL_STORE_KIND::FABRIC_LOCAL_STORE_KIND_ESE;
pub const FABRIC_MONITORED_UPGRADE_FAILURE_ACTION_INVALID: FABRIC_MONITORED_UPGRADE_FAILURE_ACTION = FABRIC_MONITORED_UPGRADE_FAILURE_ACTION::FABRIC_MONITORED_UPGRADE_FAILURE_ACTION_INVALID;
pub const FABRIC_MONITORED_UPGRADE_FAILURE_ACTION_ROLLBACK: FABRIC_MONITORED_UPGRADE_FAILURE_ACTION = FABRIC_MONITORED_UPGRADE_FAILURE_ACTION::FABRIC_MONITORED_UPGRADE_FAILURE_ACTION_ROLLBACK;
pub const FABRIC_MONITORED_UPGRADE_FAILURE_ACTION_MANUAL: FABRIC_MONITORED_UPGRADE_FAILURE_ACTION = FABRIC_MONITORED_UPGRADE_FAILURE_ACTION::FABRIC_MONITORED_UPGRADE_FAILURE_ACTION_MANUAL;
pub const FABRIC_MONITORED_UPGRADE_HEALTH_CHECK_PHASE_INVALID: FABRIC_MONITORED_UPGRADE_HEALTH_CHECK_PHASE = FABRIC_MONITORED_UPGRADE_HEALTH_CHECK_PHASE::FABRIC_MONITORED_UPGRADE_HEALTH_CHECK_PHASE_INVALID;
pub const FABRIC_MONITORED_UPGRADE_HEALTH_CHECK_PHASE_WAIT_DURATION: FABRIC_MONITORED_UPGRADE_HEALTH_CHECK_PHASE = FABRIC_MONITORED_UPGRADE_HEALTH_CHECK_PHASE::FABRIC_MONITORED_UPGRADE_HEALTH_CHECK_PHASE_WAIT_DURATION;
pub const FABRIC_MONITORED_UPGRADE_HEALTH_CHECK_PHASE_STABLE_DURATION: FABRIC_MONITORED_UPGRADE_HEALTH_CHECK_PHASE = FABRIC_MONITORED_UPGRADE_HEALTH_CHECK_PHASE::FABRIC_MONITORED_UPGRADE_HEALTH_CHECK_PHASE_STABLE_DURATION;
pub const FABRIC_MONITORED_UPGRADE_HEALTH_CHECK_PHASE_RETRY: FABRIC_MONITORED_UPGRADE_HEALTH_CHECK_PHASE = FABRIC_MONITORED_UPGRADE_HEALTH_CHECK_PHASE::FABRIC_MONITORED_UPGRADE_HEALTH_CHECK_PHASE_RETRY;
pub const FABRIC_MOVE_AUXILIARY_DESCRIPTION_KIND_INVALID: FABRIC_MOVE_AUXILIARY_DESCRIPTION_KIND = FABRIC_MOVE_AUXILIARY_DESCRIPTION_KIND::FABRIC_MOVE_AUXILIARY_DESCRIPTION_KIND_INVALID;
pub const FABRIC_MOVE_AUXILIARY_DESCRIPTION_KIND_USING_NODE_NAME: FABRIC_MOVE_AUXILIARY_DESCRIPTION_KIND = FABRIC_MOVE_AUXILIARY_DESCRIPTION_KIND::FABRIC_MOVE_AUXILIARY_DESCRIPTION_KIND_USING_NODE_NAME;
pub const FABRIC_MOVE_AUXILIARY_DESCRIPTION_KIND_USING_REPLICA_SELECTOR: FABRIC_MOVE_AUXILIARY_DESCRIPTION_KIND = FABRIC_MOVE_AUXILIARY_DESCRIPTION_KIND::FABRIC_MOVE_AUXILIARY_DESCRIPTION_KIND_USING_REPLICA_SELECTOR;
pub const FABRIC_MOVE_COST_ZERO: FABRIC_MOVE_COST = FABRIC_MOVE_COST::FABRIC_MOVE_COST_ZERO;
pub const FABRIC_MOVE_COST_LOW: FABRIC_MOVE_COST = FABRIC_MOVE_COST::FABRIC_MOVE_COST_LOW;
pub const FABRIC_MOVE_COST_MEDIUM: FABRIC_MOVE_COST = FABRIC_MOVE_COST::FABRIC_MOVE_COST_MEDIUM;
pub const FABRIC_MOVE_COST_HIGH: FABRIC_MOVE_COST = FABRIC_MOVE_COST::FABRIC_MOVE_COST_HIGH;
pub const FABRIC_MOVE_COST_VERYHIGH: FABRIC_MOVE_COST = FABRIC_MOVE_COST::FABRIC_MOVE_COST_VERYHIGH;
pub const FABRIC_MOVE_INSTANCE_DESCRIPTION_KIND_INVALID: FABRIC_MOVE_INSTANCE_DESCRIPTION_KIND = FABRIC_MOVE_INSTANCE_DESCRIPTION_KIND::FABRIC_MOVE_INSTANCE_DESCRIPTION_KIND_INVALID;
pub const FABRIC_MOVE_INSTANCE_DESCRIPTION_KIND_USING_NODE_NAME: FABRIC_MOVE_INSTANCE_DESCRIPTION_KIND = FABRIC_MOVE_INSTANCE_DESCRIPTION_KIND::FABRIC_MOVE_INSTANCE_DESCRIPTION_KIND_USING_NODE_NAME;
pub const FABRIC_MOVE_INSTANCE_DESCRIPTION_KIND_USING_REPLICA_SELECTOR: FABRIC_MOVE_INSTANCE_DESCRIPTION_KIND = FABRIC_MOVE_INSTANCE_DESCRIPTION_KIND::FABRIC_MOVE_INSTANCE_DESCRIPTION_KIND_USING_REPLICA_SELECTOR;
pub const FABRIC_MOVE_PRIMARY_DESCRIPTION_KIND_INVALID: FABRIC_MOVE_PRIMARY_DESCRIPTION_KIND = FABRIC_MOVE_PRIMARY_DESCRIPTION_KIND::FABRIC_MOVE_PRIMARY_DESCRIPTION_KIND_INVALID;
pub const FABRIC_MOVE_PRIMARY_DESCRIPTION_KIND_USING_NODE_NAME: FABRIC_MOVE_PRIMARY_DESCRIPTION_KIND = FABRIC_MOVE_PRIMARY_DESCRIPTION_KIND::FABRIC_MOVE_PRIMARY_DESCRIPTION_KIND_USING_NODE_NAME;
pub const FABRIC_MOVE_PRIMARY_DESCRIPTION_KIND_USING_REPLICA_SELECTOR: FABRIC_MOVE_PRIMARY_DESCRIPTION_KIND = FABRIC_MOVE_PRIMARY_DESCRIPTION_KIND::FABRIC_MOVE_PRIMARY_DESCRIPTION_KIND_USING_REPLICA_SELECTOR;
pub const FABRIC_MOVE_SECONDARY_DESCRIPTION_KIND_INVALID: FABRIC_MOVE_SECONDARY_DESCRIPTION_KIND = FABRIC_MOVE_SECONDARY_DESCRIPTION_KIND::FABRIC_MOVE_SECONDARY_DESCRIPTION_KIND_INVALID;
pub const FABRIC_MOVE_SECONDARY_DESCRIPTION_KIND_USING_NODE_NAME: FABRIC_MOVE_SECONDARY_DESCRIPTION_KIND = FABRIC_MOVE_SECONDARY_DESCRIPTION_KIND::FABRIC_MOVE_SECONDARY_DESCRIPTION_KIND_USING_NODE_NAME;
pub const FABRIC_MOVE_SECONDARY_DESCRIPTION_KIND_USING_REPLICA_SELECTOR: FABRIC_MOVE_SECONDARY_DESCRIPTION_KIND = FABRIC_MOVE_SECONDARY_DESCRIPTION_KIND::FABRIC_MOVE_SECONDARY_DESCRIPTION_KIND_USING_REPLICA_SELECTOR;
pub const FABRIC_NETWORK_STATUS_INVALID: FABRIC_NETWORK_STATUS = FABRIC_NETWORK_STATUS::FABRIC_NETWORK_STATUS_INVALID;
pub const FABRIC_NETWORK_STATUS_READY: FABRIC_NETWORK_STATUS = FABRIC_NETWORK_STATUS::FABRIC_NETWORK_STATUS_READY;
pub const FABRIC_NETWORK_STATUS_CREATING: FABRIC_NETWORK_STATUS = FABRIC_NETWORK_STATUS::FABRIC_NETWORK_STATUS_CREATING;
pub const FABRIC_NETWORK_STATUS_DELETING: FABRIC_NETWORK_STATUS = FABRIC_NETWORK_STATUS::FABRIC_NETWORK_STATUS_DELETING;
pub const FABRIC_NETWORK_STATUS_UPDATING: FABRIC_NETWORK_STATUS = FABRIC_NETWORK_STATUS::FABRIC_NETWORK_STATUS_UPDATING;
pub const FABRIC_NETWORK_STATUS_FAILED: FABRIC_NETWORK_STATUS = FABRIC_NETWORK_STATUS::FABRIC_NETWORK_STATUS_FAILED;
pub const FABRIC_NETWORK_STATUS_FILTER_DEFAULT: FABRIC_NETWORK_STATUS_FILTER = FABRIC_NETWORK_STATUS_FILTER::FABRIC_NETWORK_STATUS_FILTER_DEFAULT;
pub const FABRIC_NETWORK_STATUS_FILTER_ALL: FABRIC_NETWORK_STATUS_FILTER = FABRIC_NETWORK_STATUS_FILTER::FABRIC_NETWORK_STATUS_FILTER_ALL;
pub const FABRIC_NETWORK_STATUS_FILTER_READY: FABRIC_NETWORK_STATUS_FILTER = FABRIC_NETWORK_STATUS_FILTER::FABRIC_NETWORK_STATUS_FILTER_READY;
pub const FABRIC_NETWORK_STATUS_FILTER_CREATING: FABRIC_NETWORK_STATUS_FILTER = FABRIC_NETWORK_STATUS_FILTER::FABRIC_NETWORK_STATUS_FILTER_CREATING;
pub const FABRIC_NETWORK_STATUS_FILTER_DELETING: FABRIC_NETWORK_STATUS_FILTER = FABRIC_NETWORK_STATUS_FILTER::FABRIC_NETWORK_STATUS_FILTER_DELETING;
pub const FABRIC_NETWORK_STATUS_FILTER_UPDATING: FABRIC_NETWORK_STATUS_FILTER = FABRIC_NETWORK_STATUS_FILTER::FABRIC_NETWORK_STATUS_FILTER_UPDATING;
pub const FABRIC_NETWORK_STATUS_FILTER_FAILED: FABRIC_NETWORK_STATUS_FILTER = FABRIC_NETWORK_STATUS_FILTER::FABRIC_NETWORK_STATUS_FILTER_FAILED;
pub const FABRIC_NETWORK_TYPE_INVALID: FABRIC_NETWORK_TYPE = FABRIC_NETWORK_TYPE::FABRIC_NETWORK_TYPE_INVALID;
pub const FABRIC_NETWORK_TYPE_LOCAL: FABRIC_NETWORK_TYPE = FABRIC_NETWORK_TYPE::FABRIC_NETWORK_TYPE_LOCAL;
pub const FABRIC_NETWORK_TYPE_FEDERATED: FABRIC_NETWORK_TYPE = FABRIC_NETWORK_TYPE::FABRIC_NETWORK_TYPE_FEDERATED;
pub const FABRIC_NODE_DEACTIVATION_INTENT_INVALID: FABRIC_NODE_DEACTIVATION_INTENT = FABRIC_NODE_DEACTIVATION_INTENT::FABRIC_NODE_DEACTIVATION_INTENT_INVALID;
pub const FABRIC_NODE_DEACTIVATION_INTENT_PAUSE: FABRIC_NODE_DEACTIVATION_INTENT = FABRIC_NODE_DEACTIVATION_INTENT::FABRIC_NODE_DEACTIVATION_INTENT_PAUSE;
pub const FABRIC_NODE_DEACTIVATION_INTENT_RESTART: FABRIC_NODE_DEACTIVATION_INTENT = FABRIC_NODE_DEACTIVATION_INTENT::FABRIC_NODE_DEACTIVATION_INTENT_RESTART;
pub const FABRIC_NODE_DEACTIVATION_INTENT_REMOVE_DATA: FABRIC_NODE_DEACTIVATION_INTENT = FABRIC_NODE_DEACTIVATION_INTENT::FABRIC_NODE_DEACTIVATION_INTENT_REMOVE_DATA;
pub const FABRIC_NODE_DEACTIVATION_INTENT_REMOVE_NODE: FABRIC_NODE_DEACTIVATION_INTENT = FABRIC_NODE_DEACTIVATION_INTENT::FABRIC_NODE_DEACTIVATION_INTENT_REMOVE_NODE;
pub const FABRIC_NODE_DEACTIVATION_STATUS_NONE: FABRIC_NODE_DEACTIVATION_STATUS = FABRIC_NODE_DEACTIVATION_STATUS::FABRIC_NODE_DEACTIVATION_STATUS_NONE;
pub const FABRIC_NODE_DEACTIVATION_STATUS_SAFETY_CHECK_IN_PROGRESS: FABRIC_NODE_DEACTIVATION_STATUS = FABRIC_NODE_DEACTIVATION_STATUS::FABRIC_NODE_DEACTIVATION_STATUS_SAFETY_CHECK_IN_PROGRESS;
pub const FABRIC_NODE_DEACTIVATION_STATUS_SAFETY_CHECK_COMPLETE: FABRIC_NODE_DEACTIVATION_STATUS = FABRIC_NODE_DEACTIVATION_STATUS::FABRIC_NODE_DEACTIVATION_STATUS_SAFETY_CHECK_COMPLETE;
pub const FABRIC_NODE_DEACTIVATION_STATUS_COMPLETED: FABRIC_NODE_DEACTIVATION_STATUS = FABRIC_NODE_DEACTIVATION_STATUS::FABRIC_NODE_DEACTIVATION_STATUS_COMPLETED;
pub const FABRIC_NODE_DEACTIVATION_TASK_TYPE_INVALID: FABRIC_NODE_DEACTIVATION_TASK_TYPE = FABRIC_NODE_DEACTIVATION_TASK_TYPE::FABRIC_NODE_DEACTIVATION_TASK_TYPE_INVALID;
pub const FABRIC_NODE_DEACTIVATION_TASK_TYPE_INFRASTRUCTURE: FABRIC_NODE_DEACTIVATION_TASK_TYPE = FABRIC_NODE_DEACTIVATION_TASK_TYPE::FABRIC_NODE_DEACTIVATION_TASK_TYPE_INFRASTRUCTURE;
pub const FABRIC_NODE_DEACTIVATION_TASK_TYPE_REPAIR: FABRIC_NODE_DEACTIVATION_TASK_TYPE = FABRIC_NODE_DEACTIVATION_TASK_TYPE::FABRIC_NODE_DEACTIVATION_TASK_TYPE_REPAIR;
pub const FABRIC_NODE_DEACTIVATION_TASK_TYPE_CLIENT: FABRIC_NODE_DEACTIVATION_TASK_TYPE = FABRIC_NODE_DEACTIVATION_TASK_TYPE::FABRIC_NODE_DEACTIVATION_TASK_TYPE_CLIENT;
pub const FABRIC_NODE_TRANSITION_TYPE_INVALID: FABRIC_NODE_TRANSITION_TYPE = FABRIC_NODE_TRANSITION_TYPE::FABRIC_NODE_TRANSITION_TYPE_INVALID;
pub const FABRIC_NODE_TRANSITION_TYPE_START: FABRIC_NODE_TRANSITION_TYPE = FABRIC_NODE_TRANSITION_TYPE::FABRIC_NODE_TRANSITION_TYPE_START;
pub const FABRIC_NODE_TRANSITION_TYPE_STOP: FABRIC_NODE_TRANSITION_TYPE = FABRIC_NODE_TRANSITION_TYPE::FABRIC_NODE_TRANSITION_TYPE_STOP;
pub const FABRIC_NODE_UPGRADE_PHASE_INVALID: FABRIC_NODE_UPGRADE_PHASE = FABRIC_NODE_UPGRADE_PHASE::FABRIC_NODE_UPGRADE_PHASE_INVALID;
pub const FABRIC_NODE_UPGRADE_PHASE_PRE_UPGRADE_SAFETY_CHECK: FABRIC_NODE_UPGRADE_PHASE = FABRIC_NODE_UPGRADE_PHASE::FABRIC_NODE_UPGRADE_PHASE_PRE_UPGRADE_SAFETY_CHECK;
pub const FABRIC_NODE_UPGRADE_PHASE_UPGRADING: FABRIC_NODE_UPGRADE_PHASE = FABRIC_NODE_UPGRADE_PHASE::FABRIC_NODE_UPGRADE_PHASE_UPGRADING;
pub const FABRIC_NODE_UPGRADE_PHASE_POST_UPGRADE_SAFETY_CHECK: FABRIC_NODE_UPGRADE_PHASE = FABRIC_NODE_UPGRADE_PHASE::FABRIC_NODE_UPGRADE_PHASE_POST_UPGRADE_SAFETY_CHECK;
pub const FABRIC_OPERATION_TYPE_INVALID: FABRIC_OPERATION_TYPE = FABRIC_OPERATION_TYPE::FABRIC_OPERATION_TYPE_INVALID;
pub const FABRIC_OPERATION_TYPE_NORMAL: FABRIC_OPERATION_TYPE = FABRIC_OPERATION_TYPE::FABRIC_OPERATION_TYPE_NORMAL;
pub const FABRIC_OPERATION_TYPE_END_OF_STREAM: FABRIC_OPERATION_TYPE = FABRIC_OPERATION_TYPE::FABRIC_OPERATION_TYPE_END_OF_STREAM;
pub const FABRIC_OPERATION_TYPE_CREATE_ATOMIC_GROUP: FABRIC_OPERATION_TYPE = FABRIC_OPERATION_TYPE::FABRIC_OPERATION_TYPE_CREATE_ATOMIC_GROUP;
pub const FABRIC_OPERATION_TYPE_ATOMIC_GROUP_OPERATION: FABRIC_OPERATION_TYPE = FABRIC_OPERATION_TYPE::FABRIC_OPERATION_TYPE_ATOMIC_GROUP_OPERATION;
pub const FABRIC_OPERATION_TYPE_COMMIT_ATOMIC_GROUP: FABRIC_OPERATION_TYPE = FABRIC_OPERATION_TYPE::FABRIC_OPERATION_TYPE_COMMIT_ATOMIC_GROUP;
pub const FABRIC_OPERATION_TYPE_ROLLBACK_ATOMIC_GROUP: FABRIC_OPERATION_TYPE = FABRIC_OPERATION_TYPE::FABRIC_OPERATION_TYPE_ROLLBACK_ATOMIC_GROUP;
pub const FABRIC_OPERATION_TYPE_HAS_ATOMIC_GROUP_MASK: FABRIC_OPERATION_TYPE = FABRIC_OPERATION_TYPE::FABRIC_OPERATION_TYPE_HAS_ATOMIC_GROUP_MASK;
pub const FABRIC_ORDERING_DESC: FABRIC_ORDERING = FABRIC_ORDERING::FABRIC_ORDERING_DESC;
pub const FABRIC_ORDERING_ASC: FABRIC_ORDERING = FABRIC_ORDERING::FABRIC_ORDERING_ASC;
pub const FABRIC_PACKAGE_SHARING_POLICY_SCOPE_NONE: FABRIC_PACKAGE_SHARING_POLICY_SCOPE = FABRIC_PACKAGE_SHARING_POLICY_SCOPE::FABRIC_PACKAGE_SHARING_POLICY_SCOPE_NONE;
pub const FABRIC_PACKAGE_SHARING_POLICY_SCOPE_ALL: FABRIC_PACKAGE_SHARING_POLICY_SCOPE = FABRIC_PACKAGE_SHARING_POLICY_SCOPE::FABRIC_PACKAGE_SHARING_POLICY_SCOPE_ALL;
pub const FABRIC_PACKAGE_SHARING_POLICY_SCOPE_CODE: FABRIC_PACKAGE_SHARING_POLICY_SCOPE = FABRIC_PACKAGE_SHARING_POLICY_SCOPE::FABRIC_PACKAGE_SHARING_POLICY_SCOPE_CODE;
pub const FABRIC_PACKAGE_SHARING_POLICY_SCOPE_CONFIG: FABRIC_PACKAGE_SHARING_POLICY_SCOPE = FABRIC_PACKAGE_SHARING_POLICY_SCOPE::FABRIC_PACKAGE_SHARING_POLICY_SCOPE_CONFIG;
pub const FABRIC_PACKAGE_SHARING_POLICY_SCOPE_DATA: FABRIC_PACKAGE_SHARING_POLICY_SCOPE = FABRIC_PACKAGE_SHARING_POLICY_SCOPE::FABRIC_PACKAGE_SHARING_POLICY_SCOPE_DATA;
pub const FABRIC_PARTITION_KEY_TYPE_INVALID: FABRIC_PARTITION_KEY_TYPE = FABRIC_PARTITION_KEY_TYPE::FABRIC_PARTITION_KEY_TYPE_INVALID;
pub const FABRIC_PARTITION_KEY_TYPE_NONE: FABRIC_PARTITION_KEY_TYPE = FABRIC_PARTITION_KEY_TYPE::FABRIC_PARTITION_KEY_TYPE_NONE;
pub const FABRIC_PARTITION_KEY_TYPE_INT64: FABRIC_PARTITION_KEY_TYPE = FABRIC_PARTITION_KEY_TYPE::FABRIC_PARTITION_KEY_TYPE_INT64;
pub const FABRIC_PARTITION_KEY_TYPE_STRING: FABRIC_PARTITION_KEY_TYPE = FABRIC_PARTITION_KEY_TYPE::FABRIC_PARTITION_KEY_TYPE_STRING;
pub const FABRIC_PARTITION_SCHEME_INVALID: FABRIC_PARTITION_SCHEME = FABRIC_PARTITION_SCHEME::FABRIC_PARTITION_SCHEME_INVALID;
pub const FABRIC_PARTITION_SCHEME_SINGLETON: FABRIC_PARTITION_SCHEME = FABRIC_PARTITION_SCHEME::FABRIC_PARTITION_SCHEME_SINGLETON;
pub const FABRIC_PARTITION_SCHEME_UNIFORM_INT64_RANGE: FABRIC_PARTITION_SCHEME = FABRIC_PARTITION_SCHEME::FABRIC_PARTITION_SCHEME_UNIFORM_INT64_RANGE;
pub const FABRIC_PARTITION_SCHEME_NAMED: FABRIC_PARTITION_SCHEME = FABRIC_PARTITION_SCHEME::FABRIC_PARTITION_SCHEME_NAMED;
pub const FABRIC_PARTITION_SELECTOR_TYPE_NONE: FABRIC_PARTITION_SELECTOR_TYPE = FABRIC_PARTITION_SELECTOR_TYPE::FABRIC_PARTITION_SELECTOR_TYPE_NONE;
pub const FABRIC_PARTITION_SELECTOR_TYPE_SINGLETON: FABRIC_PARTITION_SELECTOR_TYPE = FABRIC_PARTITION_SELECTOR_TYPE::FABRIC_PARTITION_SELECTOR_TYPE_SINGLETON;
pub const FABRIC_PARTITION_SELECTOR_TYPE_NAMED: FABRIC_PARTITION_SELECTOR_TYPE = FABRIC_PARTITION_SELECTOR_TYPE::FABRIC_PARTITION_SELECTOR_TYPE_NAMED;
pub const FABRIC_PARTITION_SELECTOR_TYPE_UNIFORM_INT64: FABRIC_PARTITION_SELECTOR_TYPE = FABRIC_PARTITION_SELECTOR_TYPE::FABRIC_PARTITION_SELECTOR_TYPE_UNIFORM_INT64;
pub const FABRIC_PARTITION_SELECTOR_TYPE_PARTITION_ID: FABRIC_PARTITION_SELECTOR_TYPE = FABRIC_PARTITION_SELECTOR_TYPE::FABRIC_PARTITION_SELECTOR_TYPE_PARTITION_ID;
pub const FABRIC_PARTITION_SELECTOR_TYPE_RANDOM: FABRIC_PARTITION_SELECTOR_TYPE = FABRIC_PARTITION_SELECTOR_TYPE::FABRIC_PARTITION_SELECTOR_TYPE_RANDOM;
pub const FABRIC_PLACEMENT_POLICY_INVALID: FABRIC_PLACEMENT_POLICY_TYPE = FABRIC_PLACEMENT_POLICY_TYPE::FABRIC_PLACEMENT_POLICY_INVALID;
pub const FABRIC_PLACEMENT_POLICY_INVALID_DOMAIN: FABRIC_PLACEMENT_POLICY_TYPE = FABRIC_PLACEMENT_POLICY_TYPE::FABRIC_PLACEMENT_POLICY_INVALID_DOMAIN;
pub const FABRIC_PLACEMENT_POLICY_REQUIRED_DOMAIN: FABRIC_PLACEMENT_POLICY_TYPE = FABRIC_PLACEMENT_POLICY_TYPE::FABRIC_PLACEMENT_POLICY_REQUIRED_DOMAIN;
pub const FABRIC_PLACEMENT_POLICY_PREFERRED_PRIMARY_DOMAIN: FABRIC_PLACEMENT_POLICY_TYPE = FABRIC_PLACEMENT_POLICY_TYPE::FABRIC_PLACEMENT_POLICY_PREFERRED_PRIMARY_DOMAIN;
pub const FABRIC_PLACEMENT_POLICY_REQUIRED_DOMAIN_DISTRIBUTION: FABRIC_PLACEMENT_POLICY_TYPE = FABRIC_PLACEMENT_POLICY_TYPE::FABRIC_PLACEMENT_POLICY_REQUIRED_DOMAIN_DISTRIBUTION;
pub const FABRIC_PLACEMENT_POLICY_NONPARTIALLY_PLACE_SERVICE: FABRIC_PLACEMENT_POLICY_TYPE = FABRIC_PLACEMENT_POLICY_TYPE::FABRIC_PLACEMENT_POLICY_NONPARTIALLY_PLACE_SERVICE;
pub const FABRIC_PLACEMENT_POLICY_ALLOW_MULTIPLE_STATELESS_INSTANCES_ON_NODE: FABRIC_PLACEMENT_POLICY_TYPE = FABRIC_PLACEMENT_POLICY_TYPE::FABRIC_PLACEMENT_POLICY_ALLOW_MULTIPLE_STATELESS_INSTANCES_ON_NODE;
pub const FABRIC_PROPERTY_BATCH_OPERATION_KIND_INVALID: FABRIC_PROPERTY_BATCH_OPERATION_KIND = FABRIC_PROPERTY_BATCH_OPERATION_KIND::FABRIC_PROPERTY_BATCH_OPERATION_KIND_INVALID;
pub const FABRIC_PROPERTY_BATCH_OPERATION_KIND_PUT: FABRIC_PROPERTY_BATCH_OPERATION_KIND = FABRIC_PROPERTY_BATCH_OPERATION_KIND::FABRIC_PROPERTY_BATCH_OPERATION_KIND_PUT;
pub const FABRIC_PROPERTY_BATCH_OPERATION_KIND_GET: FABRIC_PROPERTY_BATCH_OPERATION_KIND = FABRIC_PROPERTY_BATCH_OPERATION_KIND::FABRIC_PROPERTY_BATCH_OPERATION_KIND_GET;
pub const FABRIC_PROPERTY_BATCH_OPERATION_KIND_CHECK_EXISTS: FABRIC_PROPERTY_BATCH_OPERATION_KIND = FABRIC_PROPERTY_BATCH_OPERATION_KIND::FABRIC_PROPERTY_BATCH_OPERATION_KIND_CHECK_EXISTS;
pub const FABRIC_PROPERTY_BATCH_OPERATION_KIND_CHECK_SEQUENCE: FABRIC_PROPERTY_BATCH_OPERATION_KIND = FABRIC_PROPERTY_BATCH_OPERATION_KIND::FABRIC_PROPERTY_BATCH_OPERATION_KIND_CHECK_SEQUENCE;
pub const FABRIC_PROPERTY_BATCH_OPERATION_KIND_DELETE: FABRIC_PROPERTY_BATCH_OPERATION_KIND = FABRIC_PROPERTY_BATCH_OPERATION_KIND::FABRIC_PROPERTY_BATCH_OPERATION_KIND_DELETE;
pub const FABRIC_PROPERTY_BATCH_OPERATION_KIND_PUT_CUSTOM: FABRIC_PROPERTY_BATCH_OPERATION_KIND = FABRIC_PROPERTY_BATCH_OPERATION_KIND::FABRIC_PROPERTY_BATCH_OPERATION_KIND_PUT_CUSTOM;
pub const FABRIC_PROPERTY_BATCH_OPERATION_KIND_CHECK_VALUE: FABRIC_PROPERTY_BATCH_OPERATION_KIND = FABRIC_PROPERTY_BATCH_OPERATION_KIND::FABRIC_PROPERTY_BATCH_OPERATION_KIND_CHECK_VALUE;
pub const FABRIC_PROPERTY_TYPE_INVALID: FABRIC_PROPERTY_TYPE_ID = FABRIC_PROPERTY_TYPE_ID::FABRIC_PROPERTY_TYPE_INVALID;
pub const FABRIC_PROPERTY_TYPE_BINARY: FABRIC_PROPERTY_TYPE_ID = FABRIC_PROPERTY_TYPE_ID::FABRIC_PROPERTY_TYPE_BINARY;
pub const FABRIC_PROPERTY_TYPE_INT64: FABRIC_PROPERTY_TYPE_ID = FABRIC_PROPERTY_TYPE_ID::FABRIC_PROPERTY_TYPE_INT64;
pub const FABRIC_PROPERTY_TYPE_DOUBLE: FABRIC_PROPERTY_TYPE_ID = FABRIC_PROPERTY_TYPE_ID::FABRIC_PROPERTY_TYPE_DOUBLE;
pub const FABRIC_PROPERTY_TYPE_WSTRING: FABRIC_PROPERTY_TYPE_ID = FABRIC_PROPERTY_TYPE_ID::FABRIC_PROPERTY_TYPE_WSTRING;
pub const FABRIC_PROPERTY_TYPE_GUID: FABRIC_PROPERTY_TYPE_ID = FABRIC_PROPERTY_TYPE_ID::FABRIC_PROPERTY_TYPE_GUID;
pub const FABRIC_PROTECTION_LEVEL_NONE: FABRIC_PROTECTION_LEVEL = FABRIC_PROTECTION_LEVEL::FABRIC_PROTECTION_LEVEL_NONE;
pub const FABRIC_PROTECTION_LEVEL_SIGN: FABRIC_PROTECTION_LEVEL = FABRIC_PROTECTION_LEVEL::FABRIC_PROTECTION_LEVEL_SIGN;
pub const FABRIC_PROTECTION_LEVEL_ENCRYPTANDSIGN: FABRIC_PROTECTION_LEVEL = FABRIC_PROTECTION_LEVEL::FABRIC_PROTECTION_LEVEL_ENCRYPTANDSIGN;
pub const FABRIC_PROVISION_APPLICATION_TYPE_KIND_INVALID: FABRIC_PROVISION_APPLICATION_TYPE_KIND = FABRIC_PROVISION_APPLICATION_TYPE_KIND::FABRIC_PROVISION_APPLICATION_TYPE_KIND_INVALID;
pub const FABRIC_PROVISION_APPLICATION_TYPE_KIND_IMAGE_STORE_PATH: FABRIC_PROVISION_APPLICATION_TYPE_KIND = FABRIC_PROVISION_APPLICATION_TYPE_KIND::FABRIC_PROVISION_APPLICATION_TYPE_KIND_IMAGE_STORE_PATH;
pub const FABRIC_PROVISION_APPLICATION_TYPE_KIND_EXTERNAL_STORE: FABRIC_PROVISION_APPLICATION_TYPE_KIND = FABRIC_PROVISION_APPLICATION_TYPE_KIND::FABRIC_PROVISION_APPLICATION_TYPE_KIND_EXTERNAL_STORE;
pub const FABRIC_QUERY_NODE_STATUS_INVALID: FABRIC_QUERY_NODE_STATUS = FABRIC_QUERY_NODE_STATUS::FABRIC_QUERY_NODE_STATUS_INVALID;
pub const FABRIC_QUERY_NODE_STATUS_UP: FABRIC_QUERY_NODE_STATUS = FABRIC_QUERY_NODE_STATUS::FABRIC_QUERY_NODE_STATUS_UP;
pub const FABRIC_QUERY_NODE_STATUS_DOWN: FABRIC_QUERY_NODE_STATUS = FABRIC_QUERY_NODE_STATUS::FABRIC_QUERY_NODE_STATUS_DOWN;
pub const FABRIC_QUERY_NODE_STATUS_ENABLING: FABRIC_QUERY_NODE_STATUS = FABRIC_QUERY_NODE_STATUS::FABRIC_QUERY_NODE_STATUS_ENABLING;
pub const FABRIC_QUERY_NODE_STATUS_DISABLING: FABRIC_QUERY_NODE_STATUS = FABRIC_QUERY_NODE_STATUS::FABRIC_QUERY_NODE_STATUS_DISABLING;
pub const FABRIC_QUERY_NODE_STATUS_DISABLED: FABRIC_QUERY_NODE_STATUS = FABRIC_QUERY_NODE_STATUS::FABRIC_QUERY_NODE_STATUS_DISABLED;
pub const FABRIC_QUERY_NODE_STATUS_UNKNOWN: FABRIC_QUERY_NODE_STATUS = FABRIC_QUERY_NODE_STATUS::FABRIC_QUERY_NODE_STATUS_UNKNOWN;
pub const FABRIC_QUERY_NODE_STATUS_REMOVED: FABRIC_QUERY_NODE_STATUS = FABRIC_QUERY_NODE_STATUS::FABRIC_QUERY_NODE_STATUS_REMOVED;
pub const FABRIC_QUERY_NODE_STATUS_FILTER_DEFAULT: FABRIC_QUERY_NODE_STATUS_FILTER = FABRIC_QUERY_NODE_STATUS_FILTER::FABRIC_QUERY_NODE_STATUS_FILTER_DEFAULT;
pub const FABRIC_QUERY_NODE_STATUS_FILTER_ALL: FABRIC_QUERY_NODE_STATUS_FILTER = FABRIC_QUERY_NODE_STATUS_FILTER::FABRIC_QUERY_NODE_STATUS_FILTER_ALL;
pub const FABRIC_QUERY_NODE_STATUS_FILTER_UP: FABRIC_QUERY_NODE_STATUS_FILTER = FABRIC_QUERY_NODE_STATUS_FILTER::FABRIC_QUERY_NODE_STATUS_FILTER_UP;
pub const FABRIC_QUERY_NODE_STATUS_FILTER_DOWN: FABRIC_QUERY_NODE_STATUS_FILTER = FABRIC_QUERY_NODE_STATUS_FILTER::FABRIC_QUERY_NODE_STATUS_FILTER_DOWN;
pub const FABRIC_QUERY_NODE_STATUS_FILTER_ENABLING: FABRIC_QUERY_NODE_STATUS_FILTER = FABRIC_QUERY_NODE_STATUS_FILTER::FABRIC_QUERY_NODE_STATUS_FILTER_ENABLING;
pub const FABRIC_QUERY_NODE_STATUS_FILTER_DISABLING: FABRIC_QUERY_NODE_STATUS_FILTER = FABRIC_QUERY_NODE_STATUS_FILTER::FABRIC_QUERY_NODE_STATUS_FILTER_DISABLING;
pub const FABRIC_QUERY_NODE_STATUS_FILTER_DISABLED: FABRIC_QUERY_NODE_STATUS_FILTER = FABRIC_QUERY_NODE_STATUS_FILTER::FABRIC_QUERY_NODE_STATUS_FILTER_DISABLED;
pub const FABRIC_QUERY_NODE_STATUS_FILTER_UNKNOWN: FABRIC_QUERY_NODE_STATUS_FILTER = FABRIC_QUERY_NODE_STATUS_FILTER::FABRIC_QUERY_NODE_STATUS_FILTER_UNKNOWN;
pub const FABRIC_QUERY_NODE_STATUS_FILTER_REMOVED: FABRIC_QUERY_NODE_STATUS_FILTER = FABRIC_QUERY_NODE_STATUS_FILTER::FABRIC_QUERY_NODE_STATUS_FILTER_REMOVED;
pub const FABRIC_QUERY_REPLICATOR_OPERATION_NAME_INVALID: FABRIC_QUERY_REPLICATOR_OPERATION_NAME = FABRIC_QUERY_REPLICATOR_OPERATION_NAME::FABRIC_QUERY_REPLICATOR_OPERATION_NAME_INVALID;
pub const FABRIC_QUERY_REPLICATOR_OPERATION_NAME_NONE: FABRIC_QUERY_REPLICATOR_OPERATION_NAME = FABRIC_QUERY_REPLICATOR_OPERATION_NAME::FABRIC_QUERY_REPLICATOR_OPERATION_NAME_NONE;
pub const FABRIC_QUERY_REPLICATOR_OPERATION_NAME_OPEN: FABRIC_QUERY_REPLICATOR_OPERATION_NAME = FABRIC_QUERY_REPLICATOR_OPERATION_NAME::FABRIC_QUERY_REPLICATOR_OPERATION_NAME_OPEN;
pub const FABRIC_QUERY_REPLICATOR_OPERATION_NAME_CHANGEROLE: FABRIC_QUERY_REPLICATOR_OPERATION_NAME = FABRIC_QUERY_REPLICATOR_OPERATION_NAME::FABRIC_QUERY_REPLICATOR_OPERATION_NAME_CHANGEROLE;
pub const FABRIC_QUERY_REPLICATOR_OPERATION_NAME_UPDATEEPOCH: FABRIC_QUERY_REPLICATOR_OPERATION_NAME = FABRIC_QUERY_REPLICATOR_OPERATION_NAME::FABRIC_QUERY_REPLICATOR_OPERATION_NAME_UPDATEEPOCH;
pub const FABRIC_QUERY_REPLICATOR_OPERATION_NAME_CLOSE: FABRIC_QUERY_REPLICATOR_OPERATION_NAME = FABRIC_QUERY_REPLICATOR_OPERATION_NAME::FABRIC_QUERY_REPLICATOR_OPERATION_NAME_CLOSE;
pub const FABRIC_QUERY_REPLICATOR_OPERATION_NAME_ABORT: FABRIC_QUERY_REPLICATOR_OPERATION_NAME = FABRIC_QUERY_REPLICATOR_OPERATION_NAME::FABRIC_QUERY_REPLICATOR_OPERATION_NAME_ABORT;
pub const FABRIC_QUERY_REPLICATOR_OPERATION_NAME_ONDATALOSS: FABRIC_QUERY_REPLICATOR_OPERATION_NAME = FABRIC_QUERY_REPLICATOR_OPERATION_NAME::FABRIC_QUERY_REPLICATOR_OPERATION_NAME_ONDATALOSS;
pub const FABRIC_QUERY_REPLICATOR_OPERATION_NAME_WAITFORCATCHUP: FABRIC_QUERY_REPLICATOR_OPERATION_NAME = FABRIC_QUERY_REPLICATOR_OPERATION_NAME::FABRIC_QUERY_REPLICATOR_OPERATION_NAME_WAITFORCATCHUP;
pub const FABRIC_QUERY_REPLICATOR_OPERATION_NAME_BUILD: FABRIC_QUERY_REPLICATOR_OPERATION_NAME = FABRIC_QUERY_REPLICATOR_OPERATION_NAME::FABRIC_QUERY_REPLICATOR_OPERATION_NAME_BUILD;
pub const FABRIC_QUERY_SERVICE_OPERATION_NAME_INVALID: FABRIC_QUERY_SERVICE_OPERATION_NAME = FABRIC_QUERY_SERVICE_OPERATION_NAME::FABRIC_QUERY_SERVICE_OPERATION_NAME_INVALID;
pub const FABRIC_QUERY_SERVICE_OPERATION_NAME_NONE: FABRIC_QUERY_SERVICE_OPERATION_NAME = FABRIC_QUERY_SERVICE_OPERATION_NAME::FABRIC_QUERY_SERVICE_OPERATION_NAME_NONE;
pub const FABRIC_QUERY_SERVICE_OPERATION_NAME_OPEN: FABRIC_QUERY_SERVICE_OPERATION_NAME = FABRIC_QUERY_SERVICE_OPERATION_NAME::FABRIC_QUERY_SERVICE_OPERATION_NAME_OPEN;
pub const FABRIC_QUERY_SERVICE_OPERATION_NAME_CHANGEROLE: FABRIC_QUERY_SERVICE_OPERATION_NAME = FABRIC_QUERY_SERVICE_OPERATION_NAME::FABRIC_QUERY_SERVICE_OPERATION_NAME_CHANGEROLE;
pub const FABRIC_QUERY_SERVICE_OPERATION_NAME_CLOSE: FABRIC_QUERY_SERVICE_OPERATION_NAME = FABRIC_QUERY_SERVICE_OPERATION_NAME::FABRIC_QUERY_SERVICE_OPERATION_NAME_CLOSE;
pub const FABRIC_QUERY_SERVICE_OPERATION_NAME_ABORT: FABRIC_QUERY_SERVICE_OPERATION_NAME = FABRIC_QUERY_SERVICE_OPERATION_NAME::FABRIC_QUERY_SERVICE_OPERATION_NAME_ABORT;
pub const FABRIC_QUERY_SERVICE_PARTITION_STATUS_INVALID: FABRIC_QUERY_SERVICE_PARTITION_STATUS = FABRIC_QUERY_SERVICE_PARTITION_STATUS::FABRIC_QUERY_SERVICE_PARTITION_STATUS_INVALID;
pub const FABRIC_QUERY_SERVICE_PARTITION_STATUS_READY: FABRIC_QUERY_SERVICE_PARTITION_STATUS = FABRIC_QUERY_SERVICE_PARTITION_STATUS::FABRIC_QUERY_SERVICE_PARTITION_STATUS_READY;
pub const FABRIC_QUERY_SERVICE_PARTITION_STATUS_NOT_READY: FABRIC_QUERY_SERVICE_PARTITION_STATUS = FABRIC_QUERY_SERVICE_PARTITION_STATUS::FABRIC_QUERY_SERVICE_PARTITION_STATUS_NOT_READY;
pub const FABRIC_QUERY_SERVICE_PARTITION_STATUS_IN_QUORUM_LOSS: FABRIC_QUERY_SERVICE_PARTITION_STATUS = FABRIC_QUERY_SERVICE_PARTITION_STATUS::FABRIC_QUERY_SERVICE_PARTITION_STATUS_IN_QUORUM_LOSS;
pub const FABRIC_QUERY_SERVICE_PARTITION_STATUS_RECONFIGURING: FABRIC_QUERY_SERVICE_PARTITION_STATUS = FABRIC_QUERY_SERVICE_PARTITION_STATUS::FABRIC_QUERY_SERVICE_PARTITION_STATUS_RECONFIGURING;
pub const FABRIC_QUERY_SERVICE_PARTITION_STATUS_DELETING: FABRIC_QUERY_SERVICE_PARTITION_STATUS = FABRIC_QUERY_SERVICE_PARTITION_STATUS::FABRIC_QUERY_SERVICE_PARTITION_STATUS_DELETING;
pub const FABRIC_QUERY_SERVICE_PARTITION_STATUS_DISABLING: FABRIC_QUERY_SERVICE_PARTITION_STATUS = FABRIC_QUERY_SERVICE_PARTITION_STATUS::FABRIC_QUERY_SERVICE_PARTITION_STATUS_DISABLING;
pub const FABRIC_QUERY_SERVICE_PARTITION_STATUS_DISABLED: FABRIC_QUERY_SERVICE_PARTITION_STATUS = FABRIC_QUERY_SERVICE_PARTITION_STATUS::FABRIC_QUERY_SERVICE_PARTITION_STATUS_DISABLED;
pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_INVALID: FABRIC_QUERY_SERVICE_REPLICA_STATUS = FABRIC_QUERY_SERVICE_REPLICA_STATUS::FABRIC_QUERY_SERVICE_REPLICA_STATUS_INVALID;
pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_INBUILD: FABRIC_QUERY_SERVICE_REPLICA_STATUS = FABRIC_QUERY_SERVICE_REPLICA_STATUS::FABRIC_QUERY_SERVICE_REPLICA_STATUS_INBUILD;
pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_STANDBY: FABRIC_QUERY_SERVICE_REPLICA_STATUS = FABRIC_QUERY_SERVICE_REPLICA_STATUS::FABRIC_QUERY_SERVICE_REPLICA_STATUS_STANDBY;
pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_READY: FABRIC_QUERY_SERVICE_REPLICA_STATUS = FABRIC_QUERY_SERVICE_REPLICA_STATUS::FABRIC_QUERY_SERVICE_REPLICA_STATUS_READY;
pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_DOWN: FABRIC_QUERY_SERVICE_REPLICA_STATUS = FABRIC_QUERY_SERVICE_REPLICA_STATUS::FABRIC_QUERY_SERVICE_REPLICA_STATUS_DOWN;
pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_DROPPED: FABRIC_QUERY_SERVICE_REPLICA_STATUS = FABRIC_QUERY_SERVICE_REPLICA_STATUS::FABRIC_QUERY_SERVICE_REPLICA_STATUS_DROPPED;
pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_COMPLETED: FABRIC_QUERY_SERVICE_REPLICA_STATUS = FABRIC_QUERY_SERVICE_REPLICA_STATUS::FABRIC_QUERY_SERVICE_REPLICA_STATUS_COMPLETED;
pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER_DEFAULT: FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER = FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER::FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER_DEFAULT;
pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER_ALL: FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER = FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER::FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER_ALL;
pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER_INBUILD: FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER = FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER::FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER_INBUILD;
pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER_STANDBY: FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER = FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER::FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER_STANDBY;
pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER_READY: FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER = FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER::FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER_READY;
pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER_DOWN: FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER = FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER::FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER_DOWN;
pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER_DROPPED: FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER = FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER::FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER_DROPPED;
pub const FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER_COMPLETED: FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER = FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER::FABRIC_QUERY_SERVICE_REPLICA_STATUS_FILTER_COMPLETED;
pub const FABRIC_QUERY_SERVICE_STATUS_UNKNOWN: FABRIC_QUERY_SERVICE_STATUS = FABRIC_QUERY_SERVICE_STATUS::FABRIC_QUERY_SERVICE_STATUS_UNKNOWN;
pub const FABRIC_QUERY_SERVICE_STATUS_ACTIVE: FABRIC_QUERY_SERVICE_STATUS = FABRIC_QUERY_SERVICE_STATUS::FABRIC_QUERY_SERVICE_STATUS_ACTIVE;
pub const FABRIC_QUERY_SERVICE_STATUS_UPGRADING: FABRIC_QUERY_SERVICE_STATUS = FABRIC_QUERY_SERVICE_STATUS::FABRIC_QUERY_SERVICE_STATUS_UPGRADING;
pub const FABRIC_QUERY_SERVICE_STATUS_DELETING: FABRIC_QUERY_SERVICE_STATUS = FABRIC_QUERY_SERVICE_STATUS::FABRIC_QUERY_SERVICE_STATUS_DELETING;
pub const FABRIC_QUERY_SERVICE_STATUS_CREATING: FABRIC_QUERY_SERVICE_STATUS = FABRIC_QUERY_SERVICE_STATUS::FABRIC_QUERY_SERVICE_STATUS_CREATING;
pub const FABRIC_QUERY_SERVICE_STATUS_FAILED: FABRIC_QUERY_SERVICE_STATUS = FABRIC_QUERY_SERVICE_STATUS::FABRIC_QUERY_SERVICE_STATUS_FAILED;
pub const FABRIC_QUERY_SERVICE_STATUS_DISABLING: FABRIC_QUERY_SERVICE_STATUS = FABRIC_QUERY_SERVICE_STATUS::FABRIC_QUERY_SERVICE_STATUS_DISABLING;
pub const FABRIC_QUERY_SERVICE_STATUS_DISABLED: FABRIC_QUERY_SERVICE_STATUS = FABRIC_QUERY_SERVICE_STATUS::FABRIC_QUERY_SERVICE_STATUS_DISABLED;
pub const FABRIC_QUORUM_LOSS_MODE_INVALID: FABRIC_QUORUM_LOSS_MODE = FABRIC_QUORUM_LOSS_MODE::FABRIC_QUORUM_LOSS_MODE_INVALID;
pub const FABRIC_QUORUM_LOSS_MODE_QUORUM_REPLICAS: FABRIC_QUORUM_LOSS_MODE = FABRIC_QUORUM_LOSS_MODE::FABRIC_QUORUM_LOSS_MODE_QUORUM_REPLICAS;
pub const FABRIC_QUORUM_LOSS_MODE_ALL_REPLICAS: FABRIC_QUORUM_LOSS_MODE = FABRIC_QUORUM_LOSS_MODE::FABRIC_QUORUM_LOSS_MODE_ALL_REPLICAS;
pub const FABRIC_RECONFIGURATION_PHASE_INVALID: FABRIC_RECONFIGURATION_PHASE = FABRIC_RECONFIGURATION_PHASE::FABRIC_RECONFIGURATION_PHASE_INVALID;
pub const FABRIC_RECONFIGURATION_PHASE_NONE: FABRIC_RECONFIGURATION_PHASE = FABRIC_RECONFIGURATION_PHASE::FABRIC_RECONFIGURATION_PHASE_NONE;
pub const FABRIC_RECONFIGURATION_PHASE_ZERO: FABRIC_RECONFIGURATION_PHASE = FABRIC_RECONFIGURATION_PHASE::FABRIC_RECONFIGURATION_PHASE_ZERO;
pub const FABRIC_RECONFIGURATION_PHASE_ONE: FABRIC_RECONFIGURATION_PHASE = FABRIC_RECONFIGURATION_PHASE::FABRIC_RECONFIGURATION_PHASE_ONE;
pub const FABRIC_RECONFIGURATION_PHASE_TWO: FABRIC_RECONFIGURATION_PHASE = FABRIC_RECONFIGURATION_PHASE::FABRIC_RECONFIGURATION_PHASE_TWO;
pub const FABRIC_RECONFIGURATION_PHASE_THREE: FABRIC_RECONFIGURATION_PHASE = FABRIC_RECONFIGURATION_PHASE::FABRIC_RECONFIGURATION_PHASE_THREE;
pub const FABRIC_RECONFIGURATION_PHASE_FOUR: FABRIC_RECONFIGURATION_PHASE = FABRIC_RECONFIGURATION_PHASE::FABRIC_RECONFIGURATION_PHASE_FOUR;
pub const FABRIC_RECONFIGURATION_ABORT_PHASE_ZERO: FABRIC_RECONFIGURATION_PHASE = FABRIC_RECONFIGURATION_PHASE::FABRIC_RECONFIGURATION_ABORT_PHASE_ZERO;
pub const FABRIC_RECONFIGURATION_TYPE_INVALID: FABRIC_RECONFIGURATION_TYPE = FABRIC_RECONFIGURATION_TYPE::FABRIC_RECONFIGURATION_TYPE_INVALID;
pub const FABRIC_RECONFIGURATION_TYPE_SWAPPRIMARY: FABRIC_RECONFIGURATION_TYPE = FABRIC_RECONFIGURATION_TYPE::FABRIC_RECONFIGURATION_TYPE_SWAPPRIMARY;
pub const FABRIC_RECONFIGURATION_TYPE_FAILOVER: FABRIC_RECONFIGURATION_TYPE = FABRIC_RECONFIGURATION_TYPE::FABRIC_RECONFIGURATION_TYPE_FAILOVER;
pub const FABRIC_RECONFIGURATION_TYPE_OTHER: FABRIC_RECONFIGURATION_TYPE = FABRIC_RECONFIGURATION_TYPE::FABRIC_RECONFIGURATION_TYPE_OTHER;
pub const FABRIC_RECONFIGURATION_TYPE_NONE: FABRIC_RECONFIGURATION_TYPE = FABRIC_RECONFIGURATION_TYPE::FABRIC_RECONFIGURATION_TYPE_NONE;
pub const FABRIC_REPAIR_IMPACT_KIND_INVALID: FABRIC_REPAIR_IMPACT_KIND = FABRIC_REPAIR_IMPACT_KIND::FABRIC_REPAIR_IMPACT_KIND_INVALID;
pub const FABRIC_REPAIR_IMPACT_KIND_NODE: FABRIC_REPAIR_IMPACT_KIND = FABRIC_REPAIR_IMPACT_KIND::FABRIC_REPAIR_IMPACT_KIND_NODE;
pub const FABRIC_REPAIR_NODE_IMPACT_LEVEL_INVALID: FABRIC_REPAIR_NODE_IMPACT_LEVEL = FABRIC_REPAIR_NODE_IMPACT_LEVEL::FABRIC_REPAIR_NODE_IMPACT_LEVEL_INVALID;
pub const FABRIC_REPAIR_NODE_IMPACT_LEVEL_NONE: FABRIC_REPAIR_NODE_IMPACT_LEVEL = FABRIC_REPAIR_NODE_IMPACT_LEVEL::FABRIC_REPAIR_NODE_IMPACT_LEVEL_NONE;
pub const FABRIC_REPAIR_NODE_IMPACT_LEVEL_RESTART: FABRIC_REPAIR_NODE_IMPACT_LEVEL = FABRIC_REPAIR_NODE_IMPACT_LEVEL::FABRIC_REPAIR_NODE_IMPACT_LEVEL_RESTART;
pub const FABRIC_REPAIR_NODE_IMPACT_LEVEL_REMOVE_DATA: FABRIC_REPAIR_NODE_IMPACT_LEVEL = FABRIC_REPAIR_NODE_IMPACT_LEVEL::FABRIC_REPAIR_NODE_IMPACT_LEVEL_REMOVE_DATA;
pub const FABRIC_REPAIR_NODE_IMPACT_LEVEL_REMOVE_NODE: FABRIC_REPAIR_NODE_IMPACT_LEVEL = FABRIC_REPAIR_NODE_IMPACT_LEVEL::FABRIC_REPAIR_NODE_IMPACT_LEVEL_REMOVE_NODE;
pub const FABRIC_REPAIR_NODE_IMPACT_LEVEL_PAUSE: FABRIC_REPAIR_NODE_IMPACT_LEVEL = FABRIC_REPAIR_NODE_IMPACT_LEVEL::FABRIC_REPAIR_NODE_IMPACT_LEVEL_PAUSE;
pub const FABRIC_REPAIR_SCOPE_IDENTIFIER_KIND_INVALID: FABRIC_REPAIR_SCOPE_IDENTIFIER_KIND = FABRIC_REPAIR_SCOPE_IDENTIFIER_KIND::FABRIC_REPAIR_SCOPE_IDENTIFIER_KIND_INVALID;
pub const FABRIC_REPAIR_SCOPE_IDENTIFIER_KIND_CLUSTER: FABRIC_REPAIR_SCOPE_IDENTIFIER_KIND = FABRIC_REPAIR_SCOPE_IDENTIFIER_KIND::FABRIC_REPAIR_SCOPE_IDENTIFIER_KIND_CLUSTER;
pub const FABRIC_REPAIR_TARGET_KIND_INVALID: FABRIC_REPAIR_TARGET_KIND = FABRIC_REPAIR_TARGET_KIND::FABRIC_REPAIR_TARGET_KIND_INVALID;
pub const FABRIC_REPAIR_TARGET_KIND_NODE: FABRIC_REPAIR_TARGET_KIND = FABRIC_REPAIR_TARGET_KIND::FABRIC_REPAIR_TARGET_KIND_NODE;
pub const FABRIC_REPAIR_TASK_FLAGS_NONE: FABRIC_REPAIR_TASK_FLAGS = FABRIC_REPAIR_TASK_FLAGS::FABRIC_REPAIR_TASK_FLAGS_NONE;
pub const FABRIC_REPAIR_TASK_FLAGS_CANCEL_REQUESTED: FABRIC_REPAIR_TASK_FLAGS = FABRIC_REPAIR_TASK_FLAGS::FABRIC_REPAIR_TASK_FLAGS_CANCEL_REQUESTED;
pub const FABRIC_REPAIR_TASK_FLAGS_ABORT_REQUESTED: FABRIC_REPAIR_TASK_FLAGS = FABRIC_REPAIR_TASK_FLAGS::FABRIC_REPAIR_TASK_FLAGS_ABORT_REQUESTED;
pub const FABRIC_REPAIR_TASK_FLAGS_FORCED_APPROVAL: FABRIC_REPAIR_TASK_FLAGS = FABRIC_REPAIR_TASK_FLAGS::FABRIC_REPAIR_TASK_FLAGS_FORCED_APPROVAL;
pub const FABRIC_REPAIR_TASK_FLAGS_VALID_MASK: FABRIC_REPAIR_TASK_FLAGS = FABRIC_REPAIR_TASK_FLAGS::FABRIC_REPAIR_TASK_FLAGS_VALID_MASK;
pub const FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE_NOT_STARTED: FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE = FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE::FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE_NOT_STARTED;
pub const FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE_IN_PROGRESS: FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE = FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE::FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE_IN_PROGRESS;
pub const FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE_SUCCEEDED: FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE = FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE::FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE_SUCCEEDED;
pub const FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE_SKIPPED: FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE = FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE::FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE_SKIPPED;
pub const FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE_TIMEDOUT: FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE = FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE::FABRIC_REPAIR_TASK_HEALTH_CHECK_STATE_TIMEDOUT;
pub const FABRIC_REPAIR_TASK_HEALTH_POLICY_UPDATE_SETTINGS_NONE: FABRIC_REPAIR_TASK_HEALTH_POLICY_UPDATE_SETTINGS_FLAGS = FABRIC_REPAIR_TASK_HEALTH_POLICY_UPDATE_SETTINGS_FLAGS::FABRIC_REPAIR_TASK_HEALTH_POLICY_UPDATE_SETTINGS_NONE;
pub const FABRIC_REPAIR_TASK_HEALTH_POLICY_UPDATE_SETTINGS_HONOR_PERFORM_PREPARING_HEALTH_CHECK: FABRIC_REPAIR_TASK_HEALTH_POLICY_UPDATE_SETTINGS_FLAGS = FABRIC_REPAIR_TASK_HEALTH_POLICY_UPDATE_SETTINGS_FLAGS::FABRIC_REPAIR_TASK_HEALTH_POLICY_UPDATE_SETTINGS_HONOR_PERFORM_PREPARING_HEALTH_CHECK;
pub const FABRIC_REPAIR_TASK_HEALTH_POLICY_UPDATE_SETTINGS_HONOR_PERFORM_RESTORING_HEALTH_CHECK: FABRIC_REPAIR_TASK_HEALTH_POLICY_UPDATE_SETTINGS_FLAGS = FABRIC_REPAIR_TASK_HEALTH_POLICY_UPDATE_SETTINGS_FLAGS::FABRIC_REPAIR_TASK_HEALTH_POLICY_UPDATE_SETTINGS_HONOR_PERFORM_RESTORING_HEALTH_CHECK;
pub const FABRIC_REPAIR_TASK_RESULT_INVALID: FABRIC_REPAIR_TASK_RESULT = FABRIC_REPAIR_TASK_RESULT::FABRIC_REPAIR_TASK_RESULT_INVALID;
pub const FABRIC_REPAIR_TASK_RESULT_SUCCEEDED: FABRIC_REPAIR_TASK_RESULT = FABRIC_REPAIR_TASK_RESULT::FABRIC_REPAIR_TASK_RESULT_SUCCEEDED;
pub const FABRIC_REPAIR_TASK_RESULT_CANCELLED: FABRIC_REPAIR_TASK_RESULT = FABRIC_REPAIR_TASK_RESULT::FABRIC_REPAIR_TASK_RESULT_CANCELLED;
pub const FABRIC_REPAIR_TASK_RESULT_INTERRUPTED: FABRIC_REPAIR_TASK_RESULT = FABRIC_REPAIR_TASK_RESULT::FABRIC_REPAIR_TASK_RESULT_INTERRUPTED;
pub const FABRIC_REPAIR_TASK_RESULT_FAILED: FABRIC_REPAIR_TASK_RESULT = FABRIC_REPAIR_TASK_RESULT::FABRIC_REPAIR_TASK_RESULT_FAILED;
pub const FABRIC_REPAIR_TASK_RESULT_PENDING: FABRIC_REPAIR_TASK_RESULT = FABRIC_REPAIR_TASK_RESULT::FABRIC_REPAIR_TASK_RESULT_PENDING;
pub const FABRIC_REPAIR_TASK_STATE_INVALID: FABRIC_REPAIR_TASK_STATE = FABRIC_REPAIR_TASK_STATE::FABRIC_REPAIR_TASK_STATE_INVALID;
pub const FABRIC_REPAIR_TASK_STATE_CREATED: FABRIC_REPAIR_TASK_STATE = FABRIC_REPAIR_TASK_STATE::FABRIC_REPAIR_TASK_STATE_CREATED;
pub const FABRIC_REPAIR_TASK_STATE_CLAIMED: FABRIC_REPAIR_TASK_STATE = FABRIC_REPAIR_TASK_STATE::FABRIC_REPAIR_TASK_STATE_CLAIMED;
pub const FABRIC_REPAIR_TASK_STATE_PREPARING: FABRIC_REPAIR_TASK_STATE = FABRIC_REPAIR_TASK_STATE::FABRIC_REPAIR_TASK_STATE_PREPARING;
pub const FABRIC_REPAIR_TASK_STATE_APPROVED: FABRIC_REPAIR_TASK_STATE = FABRIC_REPAIR_TASK_STATE::FABRIC_REPAIR_TASK_STATE_APPROVED;
pub const FABRIC_REPAIR_TASK_STATE_EXECUTING: FABRIC_REPAIR_TASK_STATE = FABRIC_REPAIR_TASK_STATE::FABRIC_REPAIR_TASK_STATE_EXECUTING;
pub const FABRIC_REPAIR_TASK_STATE_RESTORING: FABRIC_REPAIR_TASK_STATE = FABRIC_REPAIR_TASK_STATE::FABRIC_REPAIR_TASK_STATE_RESTORING;
pub const FABRIC_REPAIR_TASK_STATE_COMPLETED: FABRIC_REPAIR_TASK_STATE = FABRIC_REPAIR_TASK_STATE::FABRIC_REPAIR_TASK_STATE_COMPLETED;
pub const FABRIC_REPAIR_TASK_STATE_FILTER_DEFAULT: FABRIC_REPAIR_TASK_STATE_FILTER = FABRIC_REPAIR_TASK_STATE_FILTER::FABRIC_REPAIR_TASK_STATE_FILTER_DEFAULT;
pub const FABRIC_REPAIR_TASK_STATE_FILTER_CREATED: FABRIC_REPAIR_TASK_STATE_FILTER = FABRIC_REPAIR_TASK_STATE_FILTER::FABRIC_REPAIR_TASK_STATE_FILTER_CREATED;
pub const FABRIC_REPAIR_TASK_STATE_FILTER_CLAIMED: FABRIC_REPAIR_TASK_STATE_FILTER = FABRIC_REPAIR_TASK_STATE_FILTER::FABRIC_REPAIR_TASK_STATE_FILTER_CLAIMED;
pub const FABRIC_REPAIR_TASK_STATE_FILTER_PREPARING: FABRIC_REPAIR_TASK_STATE_FILTER = FABRIC_REPAIR_TASK_STATE_FILTER::FABRIC_REPAIR_TASK_STATE_FILTER_PREPARING;
pub const FABRIC_REPAIR_TASK_STATE_FILTER_APPROVED: FABRIC_REPAIR_TASK_STATE_FILTER = FABRIC_REPAIR_TASK_STATE_FILTER::FABRIC_REPAIR_TASK_STATE_FILTER_APPROVED;
pub const FABRIC_REPAIR_TASK_STATE_FILTER_EXECUTING: FABRIC_REPAIR_TASK_STATE_FILTER = FABRIC_REPAIR_TASK_STATE_FILTER::FABRIC_REPAIR_TASK_STATE_FILTER_EXECUTING;
pub const FABRIC_REPAIR_TASK_STATE_FILTER_RESTORING: FABRIC_REPAIR_TASK_STATE_FILTER = FABRIC_REPAIR_TASK_STATE_FILTER::FABRIC_REPAIR_TASK_STATE_FILTER_RESTORING;
pub const FABRIC_REPAIR_TASK_STATE_FILTER_COMPLETED: FABRIC_REPAIR_TASK_STATE_FILTER = FABRIC_REPAIR_TASK_STATE_FILTER::FABRIC_REPAIR_TASK_STATE_FILTER_COMPLETED;
pub const FABRIC_REPAIR_TASK_STATE_FILTER_READY_TO_EXECUTE: FABRIC_REPAIR_TASK_STATE_FILTER = FABRIC_REPAIR_TASK_STATE_FILTER::FABRIC_REPAIR_TASK_STATE_FILTER_READY_TO_EXECUTE;
pub const FABRIC_REPAIR_TASK_STATE_FILTER_ACTIVE: FABRIC_REPAIR_TASK_STATE_FILTER = FABRIC_REPAIR_TASK_STATE_FILTER::FABRIC_REPAIR_TASK_STATE_FILTER_ACTIVE;
pub const FABRIC_REPAIR_TASK_STATE_FILTER_ALL: FABRIC_REPAIR_TASK_STATE_FILTER = FABRIC_REPAIR_TASK_STATE_FILTER::FABRIC_REPAIR_TASK_STATE_FILTER_ALL;
pub const FABRIC_REPLICATOR_SETTINGS_NONE: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_SETTINGS_NONE;
pub const FABRIC_REPLICATOR_ADDRESS: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_ADDRESS;
pub const FABRIC_REPLICATOR_SECURITY: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_SECURITY;
pub const FABRIC_REPLICATOR_RETRY_INTERVAL: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_RETRY_INTERVAL;
pub const FABRIC_REPLICATOR_BATCH_ACKNOWLEDGEMENT_INTERVAL: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_BATCH_ACKNOWLEDGEMENT_INTERVAL;
pub const FABRIC_REPLICATOR_REQUIRE_SERVICE_ACK: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_REQUIRE_SERVICE_ACK;
pub const FABRIC_REPLICATOR_REPLICATION_QUEUE_INITIAL_SIZE: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_REPLICATION_QUEUE_INITIAL_SIZE;
pub const FABRIC_REPLICATOR_REPLICATION_QUEUE_MAX_SIZE: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_REPLICATION_QUEUE_MAX_SIZE;
pub const FABRIC_REPLICATOR_COPY_QUEUE_INITIAL_SIZE: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_COPY_QUEUE_INITIAL_SIZE;
pub const FABRIC_REPLICATOR_COPY_QUEUE_MAX_SIZE: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_COPY_QUEUE_MAX_SIZE;
pub const FABRIC_REPLICATOR_REPLICATION_QUEUE_MAX_MEMORY_SIZE: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_REPLICATION_QUEUE_MAX_MEMORY_SIZE;
pub const FABRIC_REPLICATOR_SECONDARY_CLEAR_ACKNOWLEDGED_OPERATIONS: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_SECONDARY_CLEAR_ACKNOWLEDGED_OPERATIONS;
pub const FABRIC_REPLICATOR_REPLICATION_MESSAGE_MAX_SIZE: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_REPLICATION_MESSAGE_MAX_SIZE;
pub const FABRIC_REPLICATOR_USE_STREAMFAULTS_AND_ENDOFSTREAM_OPERATIONACK: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_USE_STREAMFAULTS_AND_ENDOFSTREAM_OPERATIONACK;
pub const FABRIC_REPLICATOR_SECONDARY_REPLICATION_QUEUE_INITIAL_SIZE: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_SECONDARY_REPLICATION_QUEUE_INITIAL_SIZE;
pub const FABRIC_REPLICATOR_SECONDARY_REPLICATION_QUEUE_MAX_SIZE: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_SECONDARY_REPLICATION_QUEUE_MAX_SIZE;
pub const FABRIC_REPLICATOR_SECONDARY_REPLICATION_QUEUE_MAX_MEMORY_SIZE: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_SECONDARY_REPLICATION_QUEUE_MAX_MEMORY_SIZE;
pub const FABRIC_REPLICATOR_PRIMARY_REPLICATION_QUEUE_INITIAL_SIZE: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_PRIMARY_REPLICATION_QUEUE_INITIAL_SIZE;
pub const FABRIC_REPLICATOR_PRIMARY_REPLICATION_QUEUE_MAX_SIZE: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_PRIMARY_REPLICATION_QUEUE_MAX_SIZE;
pub const FABRIC_REPLICATOR_PRIMARY_REPLICATION_QUEUE_MAX_MEMORY_SIZE: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_PRIMARY_REPLICATION_QUEUE_MAX_MEMORY_SIZE;
pub const FABRIC_REPLICATOR_PRIMARY_WAIT_FOR_PENDING_QUORUMS_TIMEOUT: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_PRIMARY_WAIT_FOR_PENDING_QUORUMS_TIMEOUT;
pub const FABRIC_REPLICATOR_LISTEN_ADDRESS: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_LISTEN_ADDRESS;
pub const FABRIC_REPLICATOR_PUBLISH_ADDRESS: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_PUBLISH_ADDRESS;
pub const FABRIC_REPLICATOR_ENABLE_SEND_WINDOW_SIZE_IN_BYTES: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_ENABLE_SEND_WINDOW_SIZE_IN_BYTES;
pub const FABRIC_REPLICATOR_USE_INDIVIDUAL_HEAP_PER_REPLICA: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_USE_INDIVIDUAL_HEAP_PER_REPLICA;
pub const FABRIC_REPLICATOR_INITIAL_REPLICA_HEAP_SIZE_IN_KB: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_INITIAL_REPLICA_HEAP_SIZE_IN_KB;
pub const FABRIC_REPLICATOR_REPLICATION_BATCH_SIZE: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_REPLICATION_BATCH_SIZE;
pub const FABRIC_REPLICATOR_REPLICATION_BATCH_SEND_INTERVAL: FABRIC_REPLICATOR_SETTINGS_FLAGS = FABRIC_REPLICATOR_SETTINGS_FLAGS::FABRIC_REPLICATOR_REPLICATION_BATCH_SEND_INTERVAL;
pub const FABRIC_REPLICA_OPEN_MODE_INVALID: FABRIC_REPLICA_OPEN_MODE = FABRIC_REPLICA_OPEN_MODE::FABRIC_REPLICA_OPEN_MODE_INVALID;
pub const FABRIC_REPLICA_OPEN_MODE_NEW: FABRIC_REPLICA_OPEN_MODE = FABRIC_REPLICA_OPEN_MODE::FABRIC_REPLICA_OPEN_MODE_NEW;
pub const FABRIC_REPLICA_OPEN_MODE_EXISTING: FABRIC_REPLICA_OPEN_MODE = FABRIC_REPLICA_OPEN_MODE::FABRIC_REPLICA_OPEN_MODE_EXISTING;
pub const FABRIC_REPLICA_ROLE_UNKNOWN: FABRIC_REPLICA_ROLE = FABRIC_REPLICA_ROLE::FABRIC_REPLICA_ROLE_UNKNOWN;
pub const FABRIC_REPLICA_ROLE_NONE: FABRIC_REPLICA_ROLE = FABRIC_REPLICA_ROLE::FABRIC_REPLICA_ROLE_NONE;
pub const FABRIC_REPLICA_ROLE_PRIMARY: FABRIC_REPLICA_ROLE = FABRIC_REPLICA_ROLE::FABRIC_REPLICA_ROLE_PRIMARY;
pub const FABRIC_REPLICA_ROLE_IDLE_SECONDARY: FABRIC_REPLICA_ROLE = FABRIC_REPLICA_ROLE::FABRIC_REPLICA_ROLE_IDLE_SECONDARY;
pub const FABRIC_REPLICA_ROLE_ACTIVE_SECONDARY: FABRIC_REPLICA_ROLE = FABRIC_REPLICA_ROLE::FABRIC_REPLICA_ROLE_ACTIVE_SECONDARY;
pub const FABRIC_REPLICA_ROLE_IDLE_AUXILIARY: FABRIC_REPLICA_ROLE = FABRIC_REPLICA_ROLE::FABRIC_REPLICA_ROLE_IDLE_AUXILIARY;
pub const FABRIC_REPLICA_ROLE_ACTIVE_AUXILIARY: FABRIC_REPLICA_ROLE = FABRIC_REPLICA_ROLE::FABRIC_REPLICA_ROLE_ACTIVE_AUXILIARY;
pub const FABRIC_REPLICA_ROLE_PRIMARY_AUXILIARY: FABRIC_REPLICA_ROLE = FABRIC_REPLICA_ROLE::FABRIC_REPLICA_ROLE_PRIMARY_AUXILIARY;
pub const FABRIC_REPLICA_SET_QUORUM_INVALID: FABRIC_REPLICA_SET_QUORUM_MODE = FABRIC_REPLICA_SET_QUORUM_MODE::FABRIC_REPLICA_SET_QUORUM_INVALID;
pub const FABRIC_REPLICA_SET_WRITE_QUORUM: FABRIC_REPLICA_SET_QUORUM_MODE = FABRIC_REPLICA_SET_QUORUM_MODE::FABRIC_REPLICA_SET_WRITE_QUORUM;
pub const FABRIC_REPLICA_SET_QUORUM_ALL: FABRIC_REPLICA_SET_QUORUM_MODE = FABRIC_REPLICA_SET_QUORUM_MODE::FABRIC_REPLICA_SET_QUORUM_ALL;
pub const FABRIC_REPLICA_STATUS_INVALID: FABRIC_REPLICA_STATUS = FABRIC_REPLICA_STATUS::FABRIC_REPLICA_STATUS_INVALID;
pub const FABRIC_REPLICA_STATUS_DOWN: FABRIC_REPLICA_STATUS = FABRIC_REPLICA_STATUS::FABRIC_REPLICA_STATUS_DOWN;
pub const FABRIC_REPLICA_STATUS_UP: FABRIC_REPLICA_STATUS = FABRIC_REPLICA_STATUS::FABRIC_REPLICA_STATUS_UP;
pub const FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION_KIND_INVALID: FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION_KIND = FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION_KIND::FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION_KIND_INVALID;
pub const FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION_KIND_USING_NODE_NAME: FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION_KIND = FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION_KIND::FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION_KIND_USING_NODE_NAME;
pub const FABRIC_RESTART_NODE_DESCRIPTION_KIND_INVALID: FABRIC_RESTART_NODE_DESCRIPTION_KIND = FABRIC_RESTART_NODE_DESCRIPTION_KIND::FABRIC_RESTART_NODE_DESCRIPTION_KIND_INVALID;
pub const FABRIC_RESTART_NODE_DESCRIPTION_KIND_USING_NODE_NAME: FABRIC_RESTART_NODE_DESCRIPTION_KIND = FABRIC_RESTART_NODE_DESCRIPTION_KIND::FABRIC_RESTART_NODE_DESCRIPTION_KIND_USING_NODE_NAME;
pub const FABRIC_RESTART_PARTITION_MODE_INVALID: FABRIC_RESTART_PARTITION_MODE = FABRIC_RESTART_PARTITION_MODE::FABRIC_RESTART_PARTITION_MODE_INVALID;
pub const FABRIC_RESTART_PARTITION_MODE_ALL_REPLICAS_OR_INSTANCES: FABRIC_RESTART_PARTITION_MODE = FABRIC_RESTART_PARTITION_MODE::FABRIC_RESTART_PARTITION_MODE_ALL_REPLICAS_OR_INSTANCES;
pub const FABRIC_RESTART_PARTITION_MODE_ONLY_ACTIVE_SECONDARIES: FABRIC_RESTART_PARTITION_MODE = FABRIC_RESTART_PARTITION_MODE::FABRIC_RESTART_PARTITION_MODE_ONLY_ACTIVE_SECONDARIES;
pub const FABRIC_ROLLING_UPGRADE_MODE_INVALID: FABRIC_ROLLING_UPGRADE_MODE = FABRIC_ROLLING_UPGRADE_MODE::FABRIC_ROLLING_UPGRADE_MODE_INVALID;
pub const FABRIC_ROLLING_UPGRADE_MODE_UNMONITORED_AUTO: FABRIC_ROLLING_UPGRADE_MODE = FABRIC_ROLLING_UPGRADE_MODE::FABRIC_ROLLING_UPGRADE_MODE_UNMONITORED_AUTO;
pub const FABRIC_ROLLING_UPGRADE_MODE_UNMONITORED_MANUAL: FABRIC_ROLLING_UPGRADE_MODE = FABRIC_ROLLING_UPGRADE_MODE::FABRIC_ROLLING_UPGRADE_MODE_UNMONITORED_MANUAL;
pub const FABRIC_ROLLING_UPGRADE_MODE_MONITORED: FABRIC_ROLLING_UPGRADE_MODE = FABRIC_ROLLING_UPGRADE_MODE::FABRIC_ROLLING_UPGRADE_MODE_MONITORED;
pub const FABRIC_ROLLING_UPGRADE_MODE_UNMONITORED_DEFERRED: FABRIC_ROLLING_UPGRADE_MODE = FABRIC_ROLLING_UPGRADE_MODE::FABRIC_ROLLING_UPGRADE_MODE_UNMONITORED_DEFERRED;
pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_NONE: FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS = FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS::FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_NONE;
pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_UPGRADE_MODE: FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS = FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS::FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_UPGRADE_MODE;
pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_FORCE_RESTART: FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS = FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS::FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_FORCE_RESTART;
pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_REPLICA_SET_CHECK_TIMEOUT: FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS = FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS::FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_REPLICA_SET_CHECK_TIMEOUT;
pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_FAILURE_ACTION: FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS = FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS::FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_FAILURE_ACTION;
pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_HEALTH_CHECK_WAIT: FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS = FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS::FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_HEALTH_CHECK_WAIT;
pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_HEALTH_CHECK_STABLE: FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS = FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS::FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_HEALTH_CHECK_STABLE;
pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_HEALTH_CHECK_RETRY: FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS = FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS::FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_HEALTH_CHECK_RETRY;
pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_UPGRADE_TIMEOUT: FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS = FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS::FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_UPGRADE_TIMEOUT;
pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_UPGRADE_DOMAIN_TIMEOUT: FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS = FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS::FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_UPGRADE_DOMAIN_TIMEOUT;
pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_HEALTH_POLICY: FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS = FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS::FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_HEALTH_POLICY;
pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_ENABLE_DELTAS: FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS = FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS::FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_ENABLE_DELTAS;
pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_UPGRADE_HEALTH_POLICY: FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS = FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS::FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_UPGRADE_HEALTH_POLICY;
pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_UPGRADE_APPLICATION_HEALTH_POLICY_MAP: FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS = FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS::FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_UPGRADE_APPLICATION_HEALTH_POLICY_MAP;
pub const FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_INSTANCE_CLOSE_DELAY_DURATION: FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS = FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS::FABRIC_ROLLING_UPGRADE_UPDATE_FLAGS_INSTANCE_CLOSE_DELAY_DURATION;
pub const FABRIC_SAFETY_CHECK_KIND_INVALID: FABRIC_SAFETY_CHECK_KIND = FABRIC_SAFETY_CHECK_KIND::FABRIC_SAFETY_CHECK_KIND_INVALID;
pub const FABRIC_SEED_NODE_SAFETY_CHECK_KIND_ENSURE_QUORUM: FABRIC_SAFETY_CHECK_KIND = FABRIC_SAFETY_CHECK_KIND::FABRIC_SEED_NODE_SAFETY_CHECK_KIND_ENSURE_QUORUM;
pub const FABRIC_PARTITION_SAFETY_CHECK_KIND_ENSURE_QUORUM: FABRIC_SAFETY_CHECK_KIND = FABRIC_SAFETY_CHECK_KIND::FABRIC_PARTITION_SAFETY_CHECK_KIND_ENSURE_QUORUM;
pub const FABRIC_PARTITION_SAFETY_CHECK_KIND_WAIT_FOR_PRIMARY_PLACEMENT: FABRIC_SAFETY_CHECK_KIND = FABRIC_SAFETY_CHECK_KIND::FABRIC_PARTITION_SAFETY_CHECK_KIND_WAIT_FOR_PRIMARY_PLACEMENT;
pub const FABRIC_PARTITION_SAFETY_CHECK_KIND_WAIT_FOR_PRIMARY_SWAP: FABRIC_SAFETY_CHECK_KIND = FABRIC_SAFETY_CHECK_KIND::FABRIC_PARTITION_SAFETY_CHECK_KIND_WAIT_FOR_PRIMARY_SWAP;
pub const FABRIC_PARTITION_SAFETY_CHECK_KIND_WAIT_FOR_RECONFIGURATION: FABRIC_SAFETY_CHECK_KIND = FABRIC_SAFETY_CHECK_KIND::FABRIC_PARTITION_SAFETY_CHECK_KIND_WAIT_FOR_RECONFIGURATION;
pub const FABRIC_PARTITION_SAFETY_CHECK_KIND_WAIT_FOR_INBUILD_REPLICA: FABRIC_SAFETY_CHECK_KIND = FABRIC_SAFETY_CHECK_KIND::FABRIC_PARTITION_SAFETY_CHECK_KIND_WAIT_FOR_INBUILD_REPLICA;
pub const FABRIC_PARTITION_SAFETY_CHECK_KIND_ENSURE_AVAILABILITY: FABRIC_SAFETY_CHECK_KIND = FABRIC_SAFETY_CHECK_KIND::FABRIC_PARTITION_SAFETY_CHECK_KIND_ENSURE_AVAILABILITY;
pub const FABRIC_SCALING_MECHANISM_INVALID: FABRIC_SCALING_MECHANISM_KIND = FABRIC_SCALING_MECHANISM_KIND::FABRIC_SCALING_MECHANISM_INVALID;
pub const FABRIC_SCALING_MECHANISM_KIND_SCALE_PARTITION_INSTANCE_COUNT: FABRIC_SCALING_MECHANISM_KIND = FABRIC_SCALING_MECHANISM_KIND::FABRIC_SCALING_MECHANISM_KIND_SCALE_PARTITION_INSTANCE_COUNT;
pub const FABRIC_SCALING_MECHANISM_KIND_ADD_REMOVE_INCREMENTAL_NAMED_PARTITION: FABRIC_SCALING_MECHANISM_KIND = FABRIC_SCALING_MECHANISM_KIND::FABRIC_SCALING_MECHANISM_KIND_ADD_REMOVE_INCREMENTAL_NAMED_PARTITION;
pub const FABRIC_SCALING_TRIGGER_KIND_INVALID: FABRIC_SCALING_TRIGGER_KIND = FABRIC_SCALING_TRIGGER_KIND::FABRIC_SCALING_TRIGGER_KIND_INVALID;
pub const FABRIC_SCALING_TRIGGER_KIND_AVERAGE_PARTITION_LOAD: FABRIC_SCALING_TRIGGER_KIND = FABRIC_SCALING_TRIGGER_KIND::FABRIC_SCALING_TRIGGER_KIND_AVERAGE_PARTITION_LOAD;
pub const FABRIC_SCALING_TRIGGER_KIND_AVERAGE_SERVICE_LOAD: FABRIC_SCALING_TRIGGER_KIND = FABRIC_SCALING_TRIGGER_KIND::FABRIC_SCALING_TRIGGER_KIND_AVERAGE_SERVICE_LOAD;
pub const FABRIC_SECURITY_CREDENTIAL_KIND_NONE: FABRIC_SECURITY_CREDENTIAL_KIND = FABRIC_SECURITY_CREDENTIAL_KIND::FABRIC_SECURITY_CREDENTIAL_KIND_NONE;
pub const FABRIC_SECURITY_CREDENTIAL_KIND_X509: FABRIC_SECURITY_CREDENTIAL_KIND = FABRIC_SECURITY_CREDENTIAL_KIND::FABRIC_SECURITY_CREDENTIAL_KIND_X509;
pub const FABRIC_SECURITY_CREDENTIAL_KIND_WINDOWS: FABRIC_SECURITY_CREDENTIAL_KIND = FABRIC_SECURITY_CREDENTIAL_KIND::FABRIC_SECURITY_CREDENTIAL_KIND_WINDOWS;
pub const FABRIC_SECURITY_CREDENTIAL_KIND_CLAIMS: FABRIC_SECURITY_CREDENTIAL_KIND = FABRIC_SECURITY_CREDENTIAL_KIND::FABRIC_SECURITY_CREDENTIAL_KIND_CLAIMS;
pub const FABRIC_SECURITY_CREDENTIAL_KIND_X509_2: FABRIC_SECURITY_CREDENTIAL_KIND = FABRIC_SECURITY_CREDENTIAL_KIND::FABRIC_SECURITY_CREDENTIAL_KIND_X509_2;
pub const FABRIC_SECURITY_CREDENTIAL_KIND_INVALID: FABRIC_SECURITY_CREDENTIAL_KIND = FABRIC_SECURITY_CREDENTIAL_KIND::FABRIC_SECURITY_CREDENTIAL_KIND_INVALID;
pub const FABRIC_SELF_RECONFIGURING_INSTANCE_STATE_INVALID: FABRIC_SELF_RECONFIGURING_INSTANCE_ACTIVATION_STATE = FABRIC_SELF_RECONFIGURING_INSTANCE_ACTIVATION_STATE::FABRIC_SELF_RECONFIGURING_INSTANCE_STATE_INVALID;
pub const FABRIC_SELF_RECONFIGURING_INSTANCE_STATE_ACTIVATED: FABRIC_SELF_RECONFIGURING_INSTANCE_ACTIVATION_STATE = FABRIC_SELF_RECONFIGURING_INSTANCE_ACTIVATION_STATE::FABRIC_SELF_RECONFIGURING_INSTANCE_STATE_ACTIVATED;
pub const FABRIC_SELF_RECONFIGURING_INSTANCE_STATE_DEACTIVATED: FABRIC_SELF_RECONFIGURING_INSTANCE_ACTIVATION_STATE = FABRIC_SELF_RECONFIGURING_INSTANCE_ACTIVATION_STATE::FABRIC_SELF_RECONFIGURING_INSTANCE_STATE_DEACTIVATED;
pub const FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE_INVALID: FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE = FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE::FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE_INVALID;
pub const FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE_NEW: FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE = FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE::FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE_NEW;
pub const FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE_EXISTING: FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE = FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE::FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE_EXISTING;
pub const FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE_NONE: FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE = FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE::FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE_NONE;
pub const FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE_INITIAL: FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE = FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE::FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE_INITIAL;
pub const FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE_MEMBER: FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE = FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE::FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE_MEMBER;
pub const FABRIC_SELF_RECONFIGURING_SERVICE_NONE: FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_SELF_RECONFIGURING_SERVICE_NONE;
pub const FABRIC_SELF_RECONFIGURING_SERVICE_INSTANCE_COUNT: FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_SELF_RECONFIGURING_SERVICE_INSTANCE_COUNT;
pub const FABRIC_SELF_RECONFIGURING_SERVICE_MIN_INSTANCE_COUNT: FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_SELF_RECONFIGURING_SERVICE_MIN_INSTANCE_COUNT;
pub const FABRIC_SELF_RECONFIGURING_SERVICE_PLACEMENT_CONSTRAINTS: FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_SELF_RECONFIGURING_SERVICE_PLACEMENT_CONSTRAINTS;
pub const FABRIC_SELF_RECONFIGURING_SERVICE_POLICY_LIST: FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_SELF_RECONFIGURING_SERVICE_POLICY_LIST;
pub const FABRIC_SELF_RECONFIGURING_SERVICE_CORRELATIONS: FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_SELF_RECONFIGURING_SERVICE_CORRELATIONS;
pub const FABRIC_SELF_RECONFIGURING_SERVICE_METRICS: FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_SELF_RECONFIGURING_SERVICE_METRICS;
pub const FABRIC_SELF_RECONFIGURING_SERVICE_MOVE_COST: FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_SELF_RECONFIGURING_SERVICE_MOVE_COST;
pub const FABRIC_SELF_RECONFIGURING_SERVICE_SCALING_POLICY: FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_SELF_RECONFIGURING_SERVICE_SCALING_POLICY;
pub const FABRIC_SELF_RECONFIGURING_SERVICE_SERVICE_DNS_NAME: FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_SELF_RECONFIGURING_SERVICE_SERVICE_DNS_NAME;
pub const FABRIC_SELF_RECONFIGURING_SERVICE_RESTORE_REPLICA_LOCATION_AFTER_UPGRADE: FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_SELF_RECONFIGURING_SERVICE_RESTORE_REPLICA_LOCATION_AFTER_UPGRADE;
pub const FABRIC_SELF_RECONFIGURING_SERVICE_TAGS_REQUIRED_TO_PLACE: FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_SELF_RECONFIGURING_SERVICE_TAGS_REQUIRED_TO_PLACE;
pub const FABRIC_SELF_RECONFIGURING_SERVICE_TAGS_REQUIRED_TO_RUN: FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_SELF_RECONFIGURING_SERVICE_TAGS_REQUIRED_TO_RUN;
pub const FABRIC_SELF_RECONFIGURING_SERVICE_INSTANCE_RESTART_WAIT_DURATION: FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_SELF_RECONFIGURING_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_SELF_RECONFIGURING_SERVICE_INSTANCE_RESTART_WAIT_DURATION;
pub const FABRIC_SERVICE_CORRELATION_SCHEME_INVALID: FABRIC_SERVICE_CORRELATION_SCHEME = FABRIC_SERVICE_CORRELATION_SCHEME::FABRIC_SERVICE_CORRELATION_SCHEME_INVALID;
pub const FABRIC_SERVICE_CORRELATION_SCHEME_AFFINITY: FABRIC_SERVICE_CORRELATION_SCHEME = FABRIC_SERVICE_CORRELATION_SCHEME::FABRIC_SERVICE_CORRELATION_SCHEME_AFFINITY;
pub const FABRIC_SERVICE_CORRELATION_SCHEME_ALIGNED_AFFINITY: FABRIC_SERVICE_CORRELATION_SCHEME = FABRIC_SERVICE_CORRELATION_SCHEME::FABRIC_SERVICE_CORRELATION_SCHEME_ALIGNED_AFFINITY;
pub const FABRIC_SERVICE_CORRELATION_SCHEME_NONALIGNED_AFFINITY: FABRIC_SERVICE_CORRELATION_SCHEME = FABRIC_SERVICE_CORRELATION_SCHEME::FABRIC_SERVICE_CORRELATION_SCHEME_NONALIGNED_AFFINITY;
pub const FABRIC_SERVICE_DESCRIPTION_KIND_INVALID: FABRIC_SERVICE_DESCRIPTION_KIND = FABRIC_SERVICE_DESCRIPTION_KIND::FABRIC_SERVICE_DESCRIPTION_KIND_INVALID;
pub const FABRIC_SERVICE_DESCRIPTION_KIND_STATELESS: FABRIC_SERVICE_DESCRIPTION_KIND = FABRIC_SERVICE_DESCRIPTION_KIND::FABRIC_SERVICE_DESCRIPTION_KIND_STATELESS;
pub const FABRIC_SERVICE_DESCRIPTION_KIND_STATEFUL: FABRIC_SERVICE_DESCRIPTION_KIND = FABRIC_SERVICE_DESCRIPTION_KIND::FABRIC_SERVICE_DESCRIPTION_KIND_STATEFUL;
pub const FABRIC_SERVICE_DESCRIPTION_KIND_SELF_RECONFIGURING: FABRIC_SERVICE_DESCRIPTION_KIND = FABRIC_SERVICE_DESCRIPTION_KIND::FABRIC_SERVICE_DESCRIPTION_KIND_SELF_RECONFIGURING;
pub const FABRIC_SERVICE_DISABLE_FLAG_INVALID: FABRIC_SERVICE_DISABLE_FLAG = FABRIC_SERVICE_DISABLE_FLAG::FABRIC_SERVICE_DISABLE_FLAG_INVALID;
pub const FABRIC_SERVICE_DISABLE_FLAG_REMOVE_DATA: FABRIC_SERVICE_DISABLE_FLAG = FABRIC_SERVICE_DISABLE_FLAG::FABRIC_SERVICE_DISABLE_FLAG_REMOVE_DATA;
pub const FABRIC_SERVICE_ROLE_INVALID: FABRIC_SERVICE_ENDPOINT_ROLE = FABRIC_SERVICE_ENDPOINT_ROLE::FABRIC_SERVICE_ROLE_INVALID;
pub const FABRIC_SERVICE_ROLE_STATELESS: FABRIC_SERVICE_ENDPOINT_ROLE = FABRIC_SERVICE_ENDPOINT_ROLE::FABRIC_SERVICE_ROLE_STATELESS;
pub const FABRIC_SERVICE_ROLE_STATEFUL_PRIMARY: FABRIC_SERVICE_ENDPOINT_ROLE = FABRIC_SERVICE_ENDPOINT_ROLE::FABRIC_SERVICE_ROLE_STATEFUL_PRIMARY;
pub const FABRIC_SERVICE_ROLE_STATEFUL_SECONDARY: FABRIC_SERVICE_ENDPOINT_ROLE = FABRIC_SERVICE_ENDPOINT_ROLE::FABRIC_SERVICE_ROLE_STATEFUL_SECONDARY;
pub const FABRIC_SERVICE_ROLE_STATEFUL_PRIMARY_AUXILIARY: FABRIC_SERVICE_ENDPOINT_ROLE = FABRIC_SERVICE_ENDPOINT_ROLE::FABRIC_SERVICE_ROLE_STATEFUL_PRIMARY_AUXILIARY;
pub const FABRIC_SERVICE_ROLE_STATEFUL_AUXILIARY: FABRIC_SERVICE_ENDPOINT_ROLE = FABRIC_SERVICE_ENDPOINT_ROLE::FABRIC_SERVICE_ROLE_STATEFUL_AUXILIARY;
pub const FABRIC_SERVICE_ROLE_SELF_RECONFIGURING: FABRIC_SERVICE_ENDPOINT_ROLE = FABRIC_SERVICE_ENDPOINT_ROLE::FABRIC_SERVICE_ROLE_SELF_RECONFIGURING;
pub const FABRIC_SERVICE_HOST_UPGRADE_IMPACT_INVALID: FABRIC_SERVICE_HOST_UPGRADE_IMPACT = FABRIC_SERVICE_HOST_UPGRADE_IMPACT::FABRIC_SERVICE_HOST_UPGRADE_IMPACT_INVALID;
pub const FABRIC_SERVICE_HOST_UPGRADE_IMPACT_NONE: FABRIC_SERVICE_HOST_UPGRADE_IMPACT = FABRIC_SERVICE_HOST_UPGRADE_IMPACT::FABRIC_SERVICE_HOST_UPGRADE_IMPACT_NONE;
pub const FABRIC_SERVICE_HOST_UPGRADE_IMPACT_SERVICE_HOST_RESTART: FABRIC_SERVICE_HOST_UPGRADE_IMPACT = FABRIC_SERVICE_HOST_UPGRADE_IMPACT::FABRIC_SERVICE_HOST_UPGRADE_IMPACT_SERVICE_HOST_RESTART;
pub const FABRIC_SERVICE_HOST_UPGRADE_IMPACT_UNEXPECTED_SERVICE_HOST_RESTART: FABRIC_SERVICE_HOST_UPGRADE_IMPACT = FABRIC_SERVICE_HOST_UPGRADE_IMPACT::FABRIC_SERVICE_HOST_UPGRADE_IMPACT_UNEXPECTED_SERVICE_HOST_RESTART;
pub const FABRIC_SERVICE_KIND_INVALID: FABRIC_SERVICE_KIND = FABRIC_SERVICE_KIND::FABRIC_SERVICE_KIND_INVALID;
pub const FABRIC_SERVICE_KIND_STATELESS: FABRIC_SERVICE_KIND = FABRIC_SERVICE_KIND::FABRIC_SERVICE_KIND_STATELESS;
pub const FABRIC_SERVICE_KIND_STATEFUL: FABRIC_SERVICE_KIND = FABRIC_SERVICE_KIND::FABRIC_SERVICE_KIND_STATEFUL;
pub const FABRIC_SERVICE_KIND_SELF_RECONFIGURING: FABRIC_SERVICE_KIND = FABRIC_SERVICE_KIND::FABRIC_SERVICE_KIND_SELF_RECONFIGURING;
pub const FABRIC_SERVICE_LOAD_METRIC_WEIGHT_ZERO: FABRIC_SERVICE_LOAD_METRIC_WEIGHT = FABRIC_SERVICE_LOAD_METRIC_WEIGHT::FABRIC_SERVICE_LOAD_METRIC_WEIGHT_ZERO;
pub const FABRIC_SERVICE_LOAD_METRIC_WEIGHT_LOW: FABRIC_SERVICE_LOAD_METRIC_WEIGHT = FABRIC_SERVICE_LOAD_METRIC_WEIGHT::FABRIC_SERVICE_LOAD_METRIC_WEIGHT_LOW;
pub const FABRIC_SERVICE_LOAD_METRIC_WEIGHT_MEDIUM: FABRIC_SERVICE_LOAD_METRIC_WEIGHT = FABRIC_SERVICE_LOAD_METRIC_WEIGHT::FABRIC_SERVICE_LOAD_METRIC_WEIGHT_MEDIUM;
pub const FABRIC_SERVICE_LOAD_METRIC_WEIGHT_HIGH: FABRIC_SERVICE_LOAD_METRIC_WEIGHT = FABRIC_SERVICE_LOAD_METRIC_WEIGHT::FABRIC_SERVICE_LOAD_METRIC_WEIGHT_HIGH;
pub const FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS_NONE: FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS = FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS::FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS_NONE;
pub const FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS_NAME_PREFIX: FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS = FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS::FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS_NAME_PREFIX;
pub const FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS_PRIMARY_ONLY: FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS = FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS::FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS_PRIMARY_ONLY;
pub const FABRIC_SERVICE_PACKAGE_ACTIVATION_MODE_SHARED_PROCESS: FABRIC_SERVICE_PACKAGE_ACTIVATION_MODE = FABRIC_SERVICE_PACKAGE_ACTIVATION_MODE::FABRIC_SERVICE_PACKAGE_ACTIVATION_MODE_SHARED_PROCESS;
pub const FABRIC_SERVICE_PACKAGE_ACTIVATION_MODE_EXCLUSIVE_PROCESS: FABRIC_SERVICE_PACKAGE_ACTIVATION_MODE = FABRIC_SERVICE_PACKAGE_ACTIVATION_MODE::FABRIC_SERVICE_PACKAGE_ACTIVATION_MODE_EXCLUSIVE_PROCESS;
pub const FABRIC_SERVICE_PARTITION_ACCESS_STATUS_INVALID: FABRIC_SERVICE_PARTITION_ACCESS_STATUS = FABRIC_SERVICE_PARTITION_ACCESS_STATUS::FABRIC_SERVICE_PARTITION_ACCESS_STATUS_INVALID;
pub const FABRIC_SERVICE_PARTITION_ACCESS_STATUS_GRANTED: FABRIC_SERVICE_PARTITION_ACCESS_STATUS = FABRIC_SERVICE_PARTITION_ACCESS_STATUS::FABRIC_SERVICE_PARTITION_ACCESS_STATUS_GRANTED;
pub const FABRIC_SERVICE_PARTITION_ACCESS_STATUS_RECONFIGURATION_PENDING: FABRIC_SERVICE_PARTITION_ACCESS_STATUS = FABRIC_SERVICE_PARTITION_ACCESS_STATUS::FABRIC_SERVICE_PARTITION_ACCESS_STATUS_RECONFIGURATION_PENDING;
pub const FABRIC_SERVICE_PARTITION_ACCESS_STATUS_NOT_PRIMARY: FABRIC_SERVICE_PARTITION_ACCESS_STATUS = FABRIC_SERVICE_PARTITION_ACCESS_STATUS::FABRIC_SERVICE_PARTITION_ACCESS_STATUS_NOT_PRIMARY;
pub const FABRIC_SERVICE_PARTITION_ACCESS_STATUS_NO_WRITE_QUORUM: FABRIC_SERVICE_PARTITION_ACCESS_STATUS = FABRIC_SERVICE_PARTITION_ACCESS_STATUS::FABRIC_SERVICE_PARTITION_ACCESS_STATUS_NO_WRITE_QUORUM;
pub const FABRIC_SERVICE_PARTITION_KIND_INVALID: FABRIC_SERVICE_PARTITION_KIND = FABRIC_SERVICE_PARTITION_KIND::FABRIC_SERVICE_PARTITION_KIND_INVALID;
pub const FABRIC_SERVICE_PARTITION_KIND_SINGLETON: FABRIC_SERVICE_PARTITION_KIND = FABRIC_SERVICE_PARTITION_KIND::FABRIC_SERVICE_PARTITION_KIND_SINGLETON;
pub const FABRIC_SERVICE_PARTITION_KIND_INT64_RANGE: FABRIC_SERVICE_PARTITION_KIND = FABRIC_SERVICE_PARTITION_KIND::FABRIC_SERVICE_PARTITION_KIND_INT64_RANGE;
pub const FABRIC_SERVICE_PARTITION_KIND_NAMED: FABRIC_SERVICE_PARTITION_KIND = FABRIC_SERVICE_PARTITION_KIND::FABRIC_SERVICE_PARTITION_KIND_NAMED;
pub const FABRIC_SERVICE_REPLICA_KIND_INVALID: FABRIC_SERVICE_REPLICA_KIND = FABRIC_SERVICE_REPLICA_KIND::FABRIC_SERVICE_REPLICA_KIND_INVALID;
pub const FABRIC_SERVICE_REPLICA_KIND_KEY_VALUE_STORE: FABRIC_SERVICE_REPLICA_KIND = FABRIC_SERVICE_REPLICA_KIND::FABRIC_SERVICE_REPLICA_KIND_KEY_VALUE_STORE;
pub const FABRIC_SERVICE_TYPE_REGISTRATION_STATUS_INVALID: FABRIC_SERVICE_TYPE_REGISTRATION_STATUS = FABRIC_SERVICE_TYPE_REGISTRATION_STATUS::FABRIC_SERVICE_TYPE_REGISTRATION_STATUS_INVALID;
pub const FABRIC_SERVICE_TYPE_REGISTRATION_STATUS_DISABLED: FABRIC_SERVICE_TYPE_REGISTRATION_STATUS = FABRIC_SERVICE_TYPE_REGISTRATION_STATUS::FABRIC_SERVICE_TYPE_REGISTRATION_STATUS_DISABLED;
pub const FABRIC_SERVICE_TYPE_REGISTRATION_STATUS_NOT_REGISTERED: FABRIC_SERVICE_TYPE_REGISTRATION_STATUS = FABRIC_SERVICE_TYPE_REGISTRATION_STATUS::FABRIC_SERVICE_TYPE_REGISTRATION_STATUS_NOT_REGISTERED;
pub const FABRIC_SERVICE_TYPE_REGISTRATION_STATUS_REGISTERED: FABRIC_SERVICE_TYPE_REGISTRATION_STATUS = FABRIC_SERVICE_TYPE_REGISTRATION_STATUS::FABRIC_SERVICE_TYPE_REGISTRATION_STATUS_REGISTERED;
pub const FABRIC_START_NODE_DESCRIPTION_KIND_INVALID: FABRIC_START_NODE_DESCRIPTION_KIND = FABRIC_START_NODE_DESCRIPTION_KIND::FABRIC_START_NODE_DESCRIPTION_KIND_INVALID;
pub const FABRIC_START_NODE_DESCRIPTION_KIND_USING_NODE_NAME: FABRIC_START_NODE_DESCRIPTION_KIND = FABRIC_START_NODE_DESCRIPTION_KIND::FABRIC_START_NODE_DESCRIPTION_KIND_USING_NODE_NAME;
pub const FABRIC_STATEFUL_SERVICE_SETTINGS_NONE: FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_FLAGS = FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_FLAGS::FABRIC_STATEFUL_SERVICE_SETTINGS_NONE;
pub const FABRIC_STATEFUL_SERVICE_SETTINGS_REPLICA_RESTART_WAIT_DURATION: FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_FLAGS = FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_FLAGS::FABRIC_STATEFUL_SERVICE_SETTINGS_REPLICA_RESTART_WAIT_DURATION;
pub const FABRIC_STATEFUL_SERVICE_SETTINGS_QUORUM_LOSS_WAIT_DURATION: FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_FLAGS = FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_FLAGS::FABRIC_STATEFUL_SERVICE_SETTINGS_QUORUM_LOSS_WAIT_DURATION;
pub const FABRIC_STATEFUL_SERVICE_SETTINGS_STANDBY_REPLICA_KEEP_DURATION: FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_FLAGS = FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_FLAGS::FABRIC_STATEFUL_SERVICE_SETTINGS_STANDBY_REPLICA_KEEP_DURATION;
pub const FABRIC_STATEFUL_SERVICE_SETTINGS_SERVICE_PLACEMENT_TIME_LIMIT: FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_FLAGS = FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_FLAGS::FABRIC_STATEFUL_SERVICE_SETTINGS_SERVICE_PLACEMENT_TIME_LIMIT;
pub const FABRIC_STATEFUL_SERVICE_SETTINGS_DROP_SOURCE_REPLICA_ON_MOVE: FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_FLAGS = FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_FLAGS::FABRIC_STATEFUL_SERVICE_SETTINGS_DROP_SOURCE_REPLICA_ON_MOVE;
pub const FABRIC_STATEFUL_SERVICE_SETTINGS_IS_SINGLETON_REPLICA_MOVE_ALLOWED_DURING_UPGRADE: FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_FLAGS = FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_FLAGS::FABRIC_STATEFUL_SERVICE_SETTINGS_IS_SINGLETON_REPLICA_MOVE_ALLOWED_DURING_UPGRADE;
pub const FABRIC_STATEFUL_SERVICE_SETTINGS_RESTORE_REPLICA_LOCATION_AFTER_UPGRADE: FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_FLAGS = FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_FLAGS::FABRIC_STATEFUL_SERVICE_SETTINGS_RESTORE_REPLICA_LOCATION_AFTER_UPGRADE;
pub const FABRIC_STATEFUL_SERVICE_SETTINGS_AUXILIARY_REPLICA_COUNT: FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_FLAGS = FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_FLAGS::FABRIC_STATEFUL_SERVICE_SETTINGS_AUXILIARY_REPLICA_COUNT;
pub const FABRIC_STATEFUL_SERVICE_SETTINGS_SERVICE_SENSITIVITY: FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_FLAGS = FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_FLAGS::FABRIC_STATEFUL_SERVICE_SETTINGS_SERVICE_SENSITIVITY;
pub const FABRIC_STATEFUL_SERVICE_NONE: FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATEFUL_SERVICE_NONE;
pub const FABRIC_STATEFUL_SERVICE_TARGET_REPLICA_SET_SIZE: FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATEFUL_SERVICE_TARGET_REPLICA_SET_SIZE;
pub const FABRIC_STATEFUL_SERVICE_REPLICA_RESTART_WAIT_DURATION: FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATEFUL_SERVICE_REPLICA_RESTART_WAIT_DURATION;
pub const FABRIC_STATEFUL_SERVICE_QUORUM_LOSS_WAIT_DURATION: FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATEFUL_SERVICE_QUORUM_LOSS_WAIT_DURATION;
pub const FABRIC_STATEFUL_SERVICE_STANDBY_REPLICA_KEEP_DURATION: FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATEFUL_SERVICE_STANDBY_REPLICA_KEEP_DURATION;
pub const FABRIC_STATEFUL_SERVICE_MIN_REPLICA_SET_SIZE: FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATEFUL_SERVICE_MIN_REPLICA_SET_SIZE;
pub const FABRIC_STATEFUL_SERVICE_PLACEMENT_CONSTRAINTS: FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATEFUL_SERVICE_PLACEMENT_CONSTRAINTS;
pub const FABRIC_STATEFUL_SERVICE_POLICY_LIST: FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATEFUL_SERVICE_POLICY_LIST;
pub const FABRIC_STATEFUL_SERVICE_CORRELATIONS: FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATEFUL_SERVICE_CORRELATIONS;
pub const FABRIC_STATEFUL_SERVICE_METRICS: FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATEFUL_SERVICE_METRICS;
pub const FABRIC_STATEFUL_SERVICE_MOVE_COST: FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATEFUL_SERVICE_MOVE_COST;
pub const FABRIC_STATEFUL_SERVICE_SCALING_POLICY: FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATEFUL_SERVICE_SCALING_POLICY;
pub const FABRIC_STATEFUL_SERVICE_SERVICE_PLACEMENT_TIME_LIMIT: FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATEFUL_SERVICE_SERVICE_PLACEMENT_TIME_LIMIT;
pub const FABRIC_STATEFUL_SERVICE_DROP_SOURCE_REPLICA_ON_MOVE: FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATEFUL_SERVICE_DROP_SOURCE_REPLICA_ON_MOVE;
pub const FABRIC_STATEFUL_SERVICE_SERVICE_DNS_NAME: FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATEFUL_SERVICE_SERVICE_DNS_NAME;
pub const FABRIC_STATEFUL_SERVICE_IS_SINGLETON_REPLICA_MOVE_ALLOWED_DURING_UPGRADE: FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATEFUL_SERVICE_IS_SINGLETON_REPLICA_MOVE_ALLOWED_DURING_UPGRADE;
pub const FABRIC_STATEFUL_SERVICE_RESTORE_REPLICA_LOCATION_AFTER_UPGRADE: FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATEFUL_SERVICE_RESTORE_REPLICA_LOCATION_AFTER_UPGRADE;
pub const FABRIC_STATEFUL_SERVICE_TAGS_REQUIRED_TO_PLACE: FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATEFUL_SERVICE_TAGS_REQUIRED_TO_PLACE;
pub const FABRIC_STATEFUL_SERVICE_TAGS_REQUIRED_TO_RUN: FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATEFUL_SERVICE_TAGS_REQUIRED_TO_RUN;
pub const FABRIC_STATEFUL_SERVICE_AUXILIARY_REPLICA_COUNT: FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATEFUL_SERVICE_AUXILIARY_REPLICA_COUNT;
pub const FABRIC_STATEFUL_SERVICE_SERVICE_SENSITIVITY: FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATEFUL_SERVICE_SERVICE_SENSITIVITY;
pub const FABRIC_STATELESS_SERVICE_SETTINGS_NONE: FABRIC_STATELESS_SERVICE_FAILOVER_SETTINGS_FLAGS = FABRIC_STATELESS_SERVICE_FAILOVER_SETTINGS_FLAGS::FABRIC_STATELESS_SERVICE_SETTINGS_NONE;
pub const FABRIC_STATELESS_SERVICE_SETTINGS_INSTANCE_CLOSE_DELAY_DURATION: FABRIC_STATELESS_SERVICE_FAILOVER_SETTINGS_FLAGS = FABRIC_STATELESS_SERVICE_FAILOVER_SETTINGS_FLAGS::FABRIC_STATELESS_SERVICE_SETTINGS_INSTANCE_CLOSE_DELAY_DURATION;
pub const FABRIC_STATELESS_SERVICE_SETTINGS_INSTANCE_RESTART_WAIT_DURATION: FABRIC_STATELESS_SERVICE_FAILOVER_SETTINGS_FLAGS = FABRIC_STATELESS_SERVICE_FAILOVER_SETTINGS_FLAGS::FABRIC_STATELESS_SERVICE_SETTINGS_INSTANCE_RESTART_WAIT_DURATION;
pub const FABRIC_STATELESS_SERVICE_NONE: FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATELESS_SERVICE_NONE;
pub const FABRIC_STATELESS_SERVICE_INSTANCE_COUNT: FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATELESS_SERVICE_INSTANCE_COUNT;
pub const FABRIC_STATELESS_SERVICE_PLACEMENT_CONSTRAINTS: FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATELESS_SERVICE_PLACEMENT_CONSTRAINTS;
pub const FABRIC_STATELESS_SERVICE_POLICY_LIST: FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATELESS_SERVICE_POLICY_LIST;
pub const FABRIC_STATELESS_SERVICE_CORRELATIONS: FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATELESS_SERVICE_CORRELATIONS;
pub const FABRIC_STATELESS_SERVICE_METRICS: FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATELESS_SERVICE_METRICS;
pub const FABRIC_STATELESS_SERVICE_MOVE_COST: FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATELESS_SERVICE_MOVE_COST;
pub const FABRIC_STATELESS_SERVICE_SCALING_POLICY: FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATELESS_SERVICE_SCALING_POLICY;
pub const FABRIC_STATELESS_SERVICE_MIN_INSTANCE_COUNT: FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATELESS_SERVICE_MIN_INSTANCE_COUNT;
pub const FABRIC_STATELESS_SERVICE_MIN_INSTANCE_PERCENTAGE: FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATELESS_SERVICE_MIN_INSTANCE_PERCENTAGE;
pub const FABRIC_STATELESS_SERVICE_INSTANCE_CLOSE_DELAY_DURATION: FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATELESS_SERVICE_INSTANCE_CLOSE_DELAY_DURATION;
pub const FABRIC_STATELESS_SERVICE_INSTANCE_RESTART_WAIT_DURATION: FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATELESS_SERVICE_INSTANCE_RESTART_WAIT_DURATION;
pub const FABRIC_STATELESS_SERVICE_SERVICE_DNS_NAME: FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATELESS_SERVICE_SERVICE_DNS_NAME;
pub const FABRIC_STATELESS_SERVICE_RESTORE_REPLICA_LOCATION_AFTER_UPGRADE: FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATELESS_SERVICE_RESTORE_REPLICA_LOCATION_AFTER_UPGRADE;
pub const FABRIC_STATELESS_SERVICE_TAGS_REQUIRED_TO_PLACE: FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATELESS_SERVICE_TAGS_REQUIRED_TO_PLACE;
pub const FABRIC_STATELESS_SERVICE_TAGS_REQUIRED_TO_RUN: FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS = FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_FLAGS::FABRIC_STATELESS_SERVICE_TAGS_REQUIRED_TO_RUN;
pub const FABRIC_STOP_NODE_DESCRIPTION_KIND_INVALID: FABRIC_STOP_NODE_DESCRIPTION_KIND = FABRIC_STOP_NODE_DESCRIPTION_KIND::FABRIC_STOP_NODE_DESCRIPTION_KIND_INVALID;
pub const FABRIC_STOP_NODE_DESCRIPTION_KIND_USING_NODE_NAME: FABRIC_STOP_NODE_DESCRIPTION_KIND = FABRIC_STOP_NODE_DESCRIPTION_KIND::FABRIC_STOP_NODE_DESCRIPTION_KIND_USING_NODE_NAME;
pub const FABRIC_STORE_BACKUP_OPTION_FULL: FABRIC_STORE_BACKUP_OPTION = FABRIC_STORE_BACKUP_OPTION::FABRIC_STORE_BACKUP_OPTION_FULL;
pub const FABRIC_STORE_BACKUP_OPTION_INCREMENTAL: FABRIC_STORE_BACKUP_OPTION = FABRIC_STORE_BACKUP_OPTION::FABRIC_STORE_BACKUP_OPTION_INCREMENTAL;
pub const FABRIC_STORE_BACKUP_OPTION_TRUNCATE_LOGS_ONLY: FABRIC_STORE_BACKUP_OPTION = FABRIC_STORE_BACKUP_OPTION::FABRIC_STORE_BACKUP_OPTION_TRUNCATE_LOGS_ONLY;
pub const FABRIC_TEST_COMMAND_PROGRESS_STATE_INVALID: FABRIC_TEST_COMMAND_PROGRESS_STATE = FABRIC_TEST_COMMAND_PROGRESS_STATE::FABRIC_TEST_COMMAND_PROGRESS_STATE_INVALID;
pub const FABRIC_TEST_COMMAND_PROGRESS_STATE_RUNNING: FABRIC_TEST_COMMAND_PROGRESS_STATE = FABRIC_TEST_COMMAND_PROGRESS_STATE::FABRIC_TEST_COMMAND_PROGRESS_STATE_RUNNING;
pub const FABRIC_TEST_COMMAND_PROGRESS_STATE_ROLLING_BACK: FABRIC_TEST_COMMAND_PROGRESS_STATE = FABRIC_TEST_COMMAND_PROGRESS_STATE::FABRIC_TEST_COMMAND_PROGRESS_STATE_ROLLING_BACK;
pub const FABRIC_TEST_COMMAND_PROGRESS_STATE_COMPLETED: FABRIC_TEST_COMMAND_PROGRESS_STATE = FABRIC_TEST_COMMAND_PROGRESS_STATE::FABRIC_TEST_COMMAND_PROGRESS_STATE_COMPLETED;
pub const FABRIC_TEST_COMMAND_PROGRESS_STATE_FAULTED: FABRIC_TEST_COMMAND_PROGRESS_STATE = FABRIC_TEST_COMMAND_PROGRESS_STATE::FABRIC_TEST_COMMAND_PROGRESS_STATE_FAULTED;
pub const FABRIC_TEST_COMMAND_PROGRESS_STATE_CANCELLED: FABRIC_TEST_COMMAND_PROGRESS_STATE = FABRIC_TEST_COMMAND_PROGRESS_STATE::FABRIC_TEST_COMMAND_PROGRESS_STATE_CANCELLED;
pub const FABRIC_TEST_COMMAND_PROGRESS_STATE_FORCE_CANCELLED: FABRIC_TEST_COMMAND_PROGRESS_STATE = FABRIC_TEST_COMMAND_PROGRESS_STATE::FABRIC_TEST_COMMAND_PROGRESS_STATE_FORCE_CANCELLED;
pub const FABRIC_TEST_COMMAND_STATE_FILTER_DEFAULT: FABRIC_TEST_COMMAND_STATE_FILTER = FABRIC_TEST_COMMAND_STATE_FILTER::FABRIC_TEST_COMMAND_STATE_FILTER_DEFAULT;
pub const FABRIC_TEST_COMMAND_STATE_FILTER_ALL: FABRIC_TEST_COMMAND_STATE_FILTER = FABRIC_TEST_COMMAND_STATE_FILTER::FABRIC_TEST_COMMAND_STATE_FILTER_ALL;
pub const FABRIC_TEST_COMMAND_STATE_FILTER_RUNNING: FABRIC_TEST_COMMAND_STATE_FILTER = FABRIC_TEST_COMMAND_STATE_FILTER::FABRIC_TEST_COMMAND_STATE_FILTER_RUNNING;
pub const FABRIC_TEST_COMMAND_STATE_FILTER_ROLLING_BACK: FABRIC_TEST_COMMAND_STATE_FILTER = FABRIC_TEST_COMMAND_STATE_FILTER::FABRIC_TEST_COMMAND_STATE_FILTER_ROLLING_BACK;
pub const FABRIC_TEST_COMMAND_STATE_FILTER_COMPLETED_SUCCESSFULLY: FABRIC_TEST_COMMAND_STATE_FILTER = FABRIC_TEST_COMMAND_STATE_FILTER::FABRIC_TEST_COMMAND_STATE_FILTER_COMPLETED_SUCCESSFULLY;
pub const FABRIC_TEST_COMMAND_STATE_FILTER_FAILED: FABRIC_TEST_COMMAND_STATE_FILTER = FABRIC_TEST_COMMAND_STATE_FILTER::FABRIC_TEST_COMMAND_STATE_FILTER_FAILED;
pub const FABRIC_TEST_COMMAND_STATE_FILTER_CANCELLED: FABRIC_TEST_COMMAND_STATE_FILTER = FABRIC_TEST_COMMAND_STATE_FILTER::FABRIC_TEST_COMMAND_STATE_FILTER_CANCELLED;
pub const FABRIC_TEST_COMMAND_STATE_FILTER_FORCE_CANCELLED: FABRIC_TEST_COMMAND_STATE_FILTER = FABRIC_TEST_COMMAND_STATE_FILTER::FABRIC_TEST_COMMAND_STATE_FILTER_FORCE_CANCELLED;
pub const FABRIC_TEST_COMMAND_TYPE_DEFAULT: FABRIC_TEST_COMMAND_TYPE = FABRIC_TEST_COMMAND_TYPE::FABRIC_TEST_COMMAND_TYPE_DEFAULT;
pub const FABRIC_TEST_COMMAND_TYPE_INVOKE_DATA_LOSS: FABRIC_TEST_COMMAND_TYPE = FABRIC_TEST_COMMAND_TYPE::FABRIC_TEST_COMMAND_TYPE_INVOKE_DATA_LOSS;
pub const FABRIC_TEST_COMMAND_TYPE_INVOKE_QUORUM_LOSS: FABRIC_TEST_COMMAND_TYPE = FABRIC_TEST_COMMAND_TYPE::FABRIC_TEST_COMMAND_TYPE_INVOKE_QUORUM_LOSS;
pub const FABRIC_TEST_COMMAND_TYPE_INVOKE_RESTART_PARTITION: FABRIC_TEST_COMMAND_TYPE = FABRIC_TEST_COMMAND_TYPE::FABRIC_TEST_COMMAND_TYPE_INVOKE_RESTART_PARTITION;
pub const FABRIC_TEST_COMMAND_TYPE_START_NODE_TRANSITION: FABRIC_TEST_COMMAND_TYPE = FABRIC_TEST_COMMAND_TYPE::FABRIC_TEST_COMMAND_TYPE_START_NODE_TRANSITION;
pub const FABRIC_TEST_COMMAND_TYPE_FILTER_DEFAULT: FABRIC_TEST_COMMAND_TYPE_FILTER = FABRIC_TEST_COMMAND_TYPE_FILTER::FABRIC_TEST_COMMAND_TYPE_FILTER_DEFAULT;
pub const FABRIC_TEST_COMMAND_TYPE_FILTER_ALL: FABRIC_TEST_COMMAND_TYPE_FILTER = FABRIC_TEST_COMMAND_TYPE_FILTER::FABRIC_TEST_COMMAND_TYPE_FILTER_ALL;
pub const FABRIC_TEST_COMMAND_TYPE_FILTER_PARTITION_DATA_LOSS: FABRIC_TEST_COMMAND_TYPE_FILTER = FABRIC_TEST_COMMAND_TYPE_FILTER::FABRIC_TEST_COMMAND_TYPE_FILTER_PARTITION_DATA_LOSS;
pub const FABRIC_TEST_COMMAND_TYPE_FILTER_PARTITION_QUORUM_LOSS: FABRIC_TEST_COMMAND_TYPE_FILTER = FABRIC_TEST_COMMAND_TYPE_FILTER::FABRIC_TEST_COMMAND_TYPE_FILTER_PARTITION_QUORUM_LOSS;
pub const FABRIC_TEST_COMMAND_TYPE_FILTER_PARTITION_RESTART: FABRIC_TEST_COMMAND_TYPE_FILTER = FABRIC_TEST_COMMAND_TYPE_FILTER::FABRIC_TEST_COMMAND_TYPE_FILTER_PARTITION_RESTART;
pub const FABRIC_TRANSACTION_ISOLATION_LEVEL_DEFAULT: FABRIC_TRANSACTION_ISOLATION_LEVEL = FABRIC_TRANSACTION_ISOLATION_LEVEL::FABRIC_TRANSACTION_ISOLATION_LEVEL_DEFAULT;
pub const FABRIC_TRANSACTION_ISOLATION_LEVEL_READ_UNCOMMITTED: FABRIC_TRANSACTION_ISOLATION_LEVEL = FABRIC_TRANSACTION_ISOLATION_LEVEL::FABRIC_TRANSACTION_ISOLATION_LEVEL_READ_UNCOMMITTED;
pub const FABRIC_TRANSACTION_ISOLATION_LEVEL_READ_COMMITTED: FABRIC_TRANSACTION_ISOLATION_LEVEL = FABRIC_TRANSACTION_ISOLATION_LEVEL::FABRIC_TRANSACTION_ISOLATION_LEVEL_READ_COMMITTED;
pub const FABRIC_TRANSACTION_ISOLATION_LEVEL_REPEATABLE_READ: FABRIC_TRANSACTION_ISOLATION_LEVEL = FABRIC_TRANSACTION_ISOLATION_LEVEL::FABRIC_TRANSACTION_ISOLATION_LEVEL_REPEATABLE_READ;
pub const FABRIC_TRANSACTION_ISOLATION_LEVEL_SNAPSHOT: FABRIC_TRANSACTION_ISOLATION_LEVEL = FABRIC_TRANSACTION_ISOLATION_LEVEL::FABRIC_TRANSACTION_ISOLATION_LEVEL_SNAPSHOT;
pub const FABRIC_TRANSACTION_ISOLATION_LEVEL_SERIALIZABLE: FABRIC_TRANSACTION_ISOLATION_LEVEL = FABRIC_TRANSACTION_ISOLATION_LEVEL::FABRIC_TRANSACTION_ISOLATION_LEVEL_SERIALIZABLE;
pub const FABRIC_UPGRADE_DOMAIN_STATE_INVALID: FABRIC_UPGRADE_DOMAIN_STATE = FABRIC_UPGRADE_DOMAIN_STATE::FABRIC_UPGRADE_DOMAIN_STATE_INVALID;
pub const FABRIC_UPGRADE_DOMAIN_STATE_PENDING: FABRIC_UPGRADE_DOMAIN_STATE = FABRIC_UPGRADE_DOMAIN_STATE::FABRIC_UPGRADE_DOMAIN_STATE_PENDING;
pub const FABRIC_UPGRADE_DOMAIN_STATE_IN_PROGRESS: FABRIC_UPGRADE_DOMAIN_STATE = FABRIC_UPGRADE_DOMAIN_STATE::FABRIC_UPGRADE_DOMAIN_STATE_IN_PROGRESS;
pub const FABRIC_UPGRADE_DOMAIN_STATE_COMPLETED: FABRIC_UPGRADE_DOMAIN_STATE = FABRIC_UPGRADE_DOMAIN_STATE::FABRIC_UPGRADE_DOMAIN_STATE_COMPLETED;
pub const FABRIC_UPGRADE_FAILURE_REASON_NONE: FABRIC_UPGRADE_FAILURE_REASON = FABRIC_UPGRADE_FAILURE_REASON::FABRIC_UPGRADE_FAILURE_REASON_NONE;
pub const FABRIC_UPGRADE_FAILURE_REASON_INTERRUPTED: FABRIC_UPGRADE_FAILURE_REASON = FABRIC_UPGRADE_FAILURE_REASON::FABRIC_UPGRADE_FAILURE_REASON_INTERRUPTED;
pub const FABRIC_UPGRADE_FAILURE_REASON_HEALTH_CHECK: FABRIC_UPGRADE_FAILURE_REASON = FABRIC_UPGRADE_FAILURE_REASON::FABRIC_UPGRADE_FAILURE_REASON_HEALTH_CHECK;
pub const FABRIC_UPGRADE_FAILURE_REASON_UPGRADE_DOMAIN_TIMEOUT: FABRIC_UPGRADE_FAILURE_REASON = FABRIC_UPGRADE_FAILURE_REASON::FABRIC_UPGRADE_FAILURE_REASON_UPGRADE_DOMAIN_TIMEOUT;
pub const FABRIC_UPGRADE_FAILURE_REASON_OVERALL_UPGRADE_TIMEOUT: FABRIC_UPGRADE_FAILURE_REASON = FABRIC_UPGRADE_FAILURE_REASON::FABRIC_UPGRADE_FAILURE_REASON_OVERALL_UPGRADE_TIMEOUT;
pub const FABRIC_UPGRADE_FAILURE_REASON_PROCESSING_FAILURE: FABRIC_UPGRADE_FAILURE_REASON = FABRIC_UPGRADE_FAILURE_REASON::FABRIC_UPGRADE_FAILURE_REASON_PROCESSING_FAILURE;
pub const FABRIC_UPGRADE_KIND_INVALID: FABRIC_UPGRADE_KIND = FABRIC_UPGRADE_KIND::FABRIC_UPGRADE_KIND_INVALID;
pub const FABRIC_UPGRADE_KIND_ROLLING: FABRIC_UPGRADE_KIND = FABRIC_UPGRADE_KIND::FABRIC_UPGRADE_KIND_ROLLING;
pub const FABRIC_UPGRADE_SAFETY_CHECK_KIND_INVALID: FABRIC_UPGRADE_SAFETY_CHECK_KIND = FABRIC_UPGRADE_SAFETY_CHECK_KIND::FABRIC_UPGRADE_SAFETY_CHECK_KIND_INVALID;
pub const FABRIC_UPGRADE_SEED_NODE_SAFETY_CHECK_KIND_ENSURE_QUORUM: FABRIC_UPGRADE_SAFETY_CHECK_KIND = FABRIC_UPGRADE_SAFETY_CHECK_KIND::FABRIC_UPGRADE_SEED_NODE_SAFETY_CHECK_KIND_ENSURE_QUORUM;
pub const FABRIC_UPGRADE_PARTITION_SAFETY_CHECK_KIND_ENSURE_QUORUM: FABRIC_UPGRADE_SAFETY_CHECK_KIND = FABRIC_UPGRADE_SAFETY_CHECK_KIND::FABRIC_UPGRADE_PARTITION_SAFETY_CHECK_KIND_ENSURE_QUORUM;
pub const FABRIC_UPGRADE_PARTITION_SAFETY_CHECK_KIND_WAIT_FOR_PRIMARY_PLACEMENT: FABRIC_UPGRADE_SAFETY_CHECK_KIND = FABRIC_UPGRADE_SAFETY_CHECK_KIND::FABRIC_UPGRADE_PARTITION_SAFETY_CHECK_KIND_WAIT_FOR_PRIMARY_PLACEMENT;
pub const FABRIC_UPGRADE_PARTITION_SAFETY_CHECK_KIND_WAIT_FOR_PRIMARY_SWAP: FABRIC_UPGRADE_SAFETY_CHECK_KIND = FABRIC_UPGRADE_SAFETY_CHECK_KIND::FABRIC_UPGRADE_PARTITION_SAFETY_CHECK_KIND_WAIT_FOR_PRIMARY_SWAP;
pub const FABRIC_UPGRADE_PARTITION_SAFETY_CHECK_KIND_WAIT_FOR_RECONFIGURATION: FABRIC_UPGRADE_SAFETY_CHECK_KIND = FABRIC_UPGRADE_SAFETY_CHECK_KIND::FABRIC_UPGRADE_PARTITION_SAFETY_CHECK_KIND_WAIT_FOR_RECONFIGURATION;
pub const FABRIC_UPGRADE_PARTITION_SAFETY_CHECK_KIND_WAIT_FOR_INBUILD_REPLICA: FABRIC_UPGRADE_SAFETY_CHECK_KIND = FABRIC_UPGRADE_SAFETY_CHECK_KIND::FABRIC_UPGRADE_PARTITION_SAFETY_CHECK_KIND_WAIT_FOR_INBUILD_REPLICA;
pub const FABRIC_UPGRADE_PARTITION_SAFETY_CHECK_KIND_ENSURE_AVAILABILITY: FABRIC_UPGRADE_SAFETY_CHECK_KIND = FABRIC_UPGRADE_SAFETY_CHECK_KIND::FABRIC_UPGRADE_PARTITION_SAFETY_CHECK_KIND_ENSURE_AVAILABILITY;
pub const FABRIC_UPGRADE_PARTITION_SAFETY_CHECK_KIND_WAIT_FOR_RESOURCE_AVAILABILITY: FABRIC_UPGRADE_SAFETY_CHECK_KIND = FABRIC_UPGRADE_SAFETY_CHECK_KIND::FABRIC_UPGRADE_PARTITION_SAFETY_CHECK_KIND_WAIT_FOR_RESOURCE_AVAILABILITY;
pub const FABRIC_UPGRADE_SORT_ORDER_INVALID: FABRIC_UPGRADE_SORT_ORDER = FABRIC_UPGRADE_SORT_ORDER::FABRIC_UPGRADE_SORT_ORDER_INVALID;
pub const FABRIC_UPGRADE_SORT_ORDER_DEFAULT: FABRIC_UPGRADE_SORT_ORDER = FABRIC_UPGRADE_SORT_ORDER::FABRIC_UPGRADE_SORT_ORDER_DEFAULT;
pub const FABRIC_UPGRADE_SORT_ORDER_NUMERIC: FABRIC_UPGRADE_SORT_ORDER = FABRIC_UPGRADE_SORT_ORDER::FABRIC_UPGRADE_SORT_ORDER_NUMERIC;
pub const FABRIC_UPGRADE_SORT_ORDER_LEXICOGRAPHICAL: FABRIC_UPGRADE_SORT_ORDER = FABRIC_UPGRADE_SORT_ORDER::FABRIC_UPGRADE_SORT_ORDER_LEXICOGRAPHICAL;
pub const FABRIC_UPGRADE_SORT_ORDER_REVERSE_NUMERIC: FABRIC_UPGRADE_SORT_ORDER = FABRIC_UPGRADE_SORT_ORDER::FABRIC_UPGRADE_SORT_ORDER_REVERSE_NUMERIC;
pub const FABRIC_UPGRADE_SORT_ORDER_REVERSE_LEXICOGRAPHICAL: FABRIC_UPGRADE_SORT_ORDER = FABRIC_UPGRADE_SORT_ORDER::FABRIC_UPGRADE_SORT_ORDER_REVERSE_LEXICOGRAPHICAL;
pub const FABRIC_UPGRADE_STATE_INVALID: FABRIC_UPGRADE_STATE = FABRIC_UPGRADE_STATE::FABRIC_UPGRADE_STATE_INVALID;
pub const FABRIC_UPGRADE_STATE_ROLLING_BACK_IN_PROGRESS: FABRIC_UPGRADE_STATE = FABRIC_UPGRADE_STATE::FABRIC_UPGRADE_STATE_ROLLING_BACK_IN_PROGRESS;
pub const FABRIC_UPGRADE_STATE_ROLLING_BACK_COMPLETED: FABRIC_UPGRADE_STATE = FABRIC_UPGRADE_STATE::FABRIC_UPGRADE_STATE_ROLLING_BACK_COMPLETED;
pub const FABRIC_UPGRADE_STATE_ROLLING_FORWARD_PENDING: FABRIC_UPGRADE_STATE = FABRIC_UPGRADE_STATE::FABRIC_UPGRADE_STATE_ROLLING_FORWARD_PENDING;
pub const FABRIC_UPGRADE_STATE_ROLLING_FORWARD_IN_PROGRESS: FABRIC_UPGRADE_STATE = FABRIC_UPGRADE_STATE::FABRIC_UPGRADE_STATE_ROLLING_FORWARD_IN_PROGRESS;
pub const FABRIC_UPGRADE_STATE_ROLLING_FORWARD_COMPLETED: FABRIC_UPGRADE_STATE = FABRIC_UPGRADE_STATE::FABRIC_UPGRADE_STATE_ROLLING_FORWARD_COMPLETED;
pub const FABRIC_UPGRADE_STATE_FAILED: FABRIC_UPGRADE_STATE = FABRIC_UPGRADE_STATE::FABRIC_UPGRADE_STATE_FAILED;
pub const FABRIC_UPGRADE_STATE_ROLLING_BACK_PENDING: FABRIC_UPGRADE_STATE = FABRIC_UPGRADE_STATE::FABRIC_UPGRADE_STATE_ROLLING_BACK_PENDING;
pub const FABRIC_X509_FIND_TYPE_FINDBYTHUMBPRINT: FABRIC_X509_FIND_TYPE = FABRIC_X509_FIND_TYPE::FABRIC_X509_FIND_TYPE_FINDBYTHUMBPRINT;
pub const FABRIC_X509_FIND_TYPE_FINDBYSUBJECTNAME: FABRIC_X509_FIND_TYPE = FABRIC_X509_FIND_TYPE::FABRIC_X509_FIND_TYPE_FINDBYSUBJECTNAME;
pub const FABRIC_X509_FIND_TYPE_FINDBYEXTENSION: FABRIC_X509_FIND_TYPE = FABRIC_X509_FIND_TYPE::FABRIC_X509_FIND_TYPE_FINDBYEXTENSION;
pub const FABRIC_X509_STORE_LOCATION_INVALID: FABRIC_X509_STORE_LOCATION = FABRIC_X509_STORE_LOCATION::FABRIC_X509_STORE_LOCATION_INVALID;
pub const FABRIC_X509_STORE_LOCATION_CURRENTUSER: FABRIC_X509_STORE_LOCATION = FABRIC_X509_STORE_LOCATION::FABRIC_X509_STORE_LOCATION_CURRENTUSER;
pub const FABRIC_X509_STORE_LOCATION_LOCALMACHINE: FABRIC_X509_STORE_LOCATION = FABRIC_X509_STORE_LOCATION::FABRIC_X509_STORE_LOCATION_LOCALMACHINE;
