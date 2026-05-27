use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataRedisClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataRedisCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRedisClusterData>,
}
#[derive(Clone)]
pub struct DataRedisCluster(Rc<DataRedisCluster_>);
impl DataRedisCluster {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nThe name of the region of the Redis cluster."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `authorization_mode` after provisioning.\nOptional. The authorization mode of the Redis cluster. If not provided, auth feature is disabled for the cluster. Default value: \"AUTH_MODE_DISABLED\" Possible values: [\"AUTH_MODE_UNSPECIFIED\", \"AUTH_MODE_IAM_AUTH\", \"AUTH_MODE_DISABLED\"]"]
    pub fn authorization_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorization_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `automated_backup_config` after provisioning.\nThe automated backup config for a instance."]
    pub fn automated_backup_config(&self) -> ListRef<DataRedisClusterAutomatedBackupConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.automated_backup_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `available_maintenance_versions` after provisioning.\nThis field is used to determine the available maintenance versions for the self service update."]
    pub fn available_maintenance_versions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.available_maintenance_versions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_collection` after provisioning.\nThe backup collection full resource name.\nExample: projects/{project}/locations/{location}/backupCollections/{collection}"]
    pub fn backup_collection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_collection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp associated with the cluster creation request. A timestamp in\nRFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional\ndigits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cross_cluster_replication_config` after provisioning.\nCross cluster replication config"]
    pub fn cross_cluster_replication_config(
        &self,
    ) -> ListRef<DataRedisClusterCrossClusterReplicationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cross_cluster_replication_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\nOptional. Indicates if the cluster is deletion protected or not.\nIf the value if set to true, any delete cluster operation will fail.\nDefault value is true."]
    pub fn deletion_protection_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `discovery_endpoints` after provisioning.\nOutput only. Endpoints created on each given network,\nfor Redis clients to connect to the cluster.\nCurrently only one endpoint is supported."]
    pub fn discovery_endpoints(&self) -> ListRef<DataRedisClusterDiscoveryEndpointsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.discovery_endpoints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_maintenance_version` after provisioning.\nThis field represents the actual maintenance version of the cluster."]
    pub fn effective_maintenance_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_maintenance_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_source` after provisioning.\nBackups stored in Cloud Storage buckets. The Cloud Storage buckets need to be the same region as the clusters."]
    pub fn gcs_source(&self) -> ListRef<DataRedisClusterGcsSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcs_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nThe KMS key used to encrypt the at-rest data of the cluster."]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user provided metadata.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\nMaintenance policy for a cluster"]
    pub fn maintenance_policy(&self) -> ListRef<DataRedisClusterMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_schedule` after provisioning.\nUpcoming maintenance schedule."]
    pub fn maintenance_schedule(&self) -> ListRef<DataRedisClusterMaintenanceScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_schedule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_version` after provisioning.\nThis field can be used to trigger self service update to indicate the desired maintenance version. The input to this field can be determined by the available_maintenance_versions field.\n*Note*: This field can only be specified when updating an existing cluster to a newer version. Downgrades are currently not supported!"]
    pub fn maintenance_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed_backup_source` after provisioning.\nBackups that generated and managed by memorystore."]
    pub fn managed_backup_source(&self) -> ListRef<DataRedisClusterManagedBackupSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_backup_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed_server_ca` after provisioning.\nCluster's Certificate Authority. This field will only be populated if Redis Cluster's transit_encryption_mode is TRANSIT_ENCRYPTION_MODE_SERVER_AUTHENTICATION"]
    pub fn managed_server_ca(&self) -> ListRef<DataRedisClusterManagedServerCaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_server_ca", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nUnique name of the resource in this scope including project and location using the form:\nprojects/{projectId}/locations/{locationId}/clusters/{clusterId}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_type` after provisioning.\nThe nodeType for the Redis cluster.\nIf not provided, REDIS_HIGHMEM_MEDIUM will be used as default Possible values: [\"REDIS_SHARED_CORE_NANO\", \"REDIS_HIGHMEM_MEDIUM\", \"REDIS_HIGHCPU_MEDIUM\", \"REDIS_STANDARD_LARGE\", \"REDIS_HIGHMEM_XLARGE\", \"REDIS_HIGHMEM_2XLARGE\", \"REDIS_STANDARD_SMALL\"]"]
    pub fn node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `persistence_config` after provisioning.\nPersistence config (RDB, AOF) for the cluster."]
    pub fn persistence_config(&self) -> ListRef<DataRedisClusterPersistenceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.persistence_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `precise_size_gb` after provisioning.\nOutput only. Redis memory precise size in GB for the entire cluster."]
    pub fn precise_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.precise_size_gb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_configs` after provisioning.\nRequired. Each PscConfig configures the consumer network where two\nnetwork addresses will be designated to the cluster for client access.\nCurrently, only one PscConfig is supported."]
    pub fn psc_configs(&self) -> ListRef<DataRedisClusterPscConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_connections` after provisioning.\nOutput only. PSC connections for discovery of the cluster topology and accessing the cluster."]
    pub fn psc_connections(&self) -> ListRef<DataRedisClusterPscConnectionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_connections", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_service_attachments` after provisioning.\nService attachment details to configure Psc connections."]
    pub fn psc_service_attachments(&self) -> ListRef<DataRedisClusterPscServiceAttachmentsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_service_attachments", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redis_configs` after provisioning.\nConfigure Redis Cluster behavior using a subset of native Redis configuration parameters.\nPlease check Memorystore documentation for the list of supported parameters:\nhttps://cloud.google.com/memorystore/docs/cluster/supported-instance-configurations"]
    pub fn redis_configs(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.redis_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe name of the region of the Redis cluster."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replica_count` after provisioning.\nOptional. The number of replica nodes per shard."]
    pub fn replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replica_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_ca_mode` after provisioning.\nThe serverCaMode for the TLS enabled Redis cluster.\nIf not provided, SERVER_CA_MODE_GOOGLE_MANAGED_PER_INSTANCE_CA will be used as default Possible values: [\"SERVER_CA_MODE_GOOGLE_MANAGED_PER_INSTANCE_CA\", \"SERVER_CA_MODE_GOOGLE_MANAGED_SHARED_CA\", \"SERVER_CA_MODE_CUSTOMER_MANAGED_CAS_CA\", \"SERVER_CA_MODE_UNSPECIFIED\"]"]
    pub fn server_ca_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_ca_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_ca_pool` after provisioning.\nThe resource name of the server CA pool for an instance with SERVER_CA_MODE_CUSTOMER_MANAGED_CAS_CA\nas the server_ca_mode.\nFormat: projects/{project}/locations/{region}/caPools/{caPoolId}"]
    pub fn server_ca_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_ca_pool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `shard_count` after provisioning.\nRequired. Number of shards for the Redis cluster."]
    pub fn shard_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.shard_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `size_gb` after provisioning.\nOutput only. Redis memory size in GB for the entire cluster."]
    pub fn size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.size_gb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of this cluster. Can be CREATING, READY, UPDATING, DELETING and SUSPENDED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_info` after provisioning.\nOutput only. Additional information about the current state of the cluster."]
    pub fn state_info(&self) -> ListRef<DataRedisClusterStateInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.state_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transit_encryption_mode` after provisioning.\nOptional. The in-transit encryption for the Redis cluster.\nIf not provided, encryption is disabled for the cluster. Default value: \"TRANSIT_ENCRYPTION_MODE_DISABLED\" Possible values: [\"TRANSIT_ENCRYPTION_MODE_UNSPECIFIED\", \"TRANSIT_ENCRYPTION_MODE_DISABLED\", \"TRANSIT_ENCRYPTION_MODE_SERVER_AUTHENTICATION\"]"]
    pub fn transit_encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_encryption_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem assigned, unique identifier for the cluster."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `zone_distribution_config` after provisioning.\nImmutable. Zone distribution config for Memorystore Redis cluster."]
    pub fn zone_distribution_config(&self) -> ListRef<DataRedisClusterZoneDistributionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.zone_distribution_config", self.extract_ref()),
        )
    }
}
impl Referable for DataRedisCluster {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataRedisCluster {}
impl ToListMappable for DataRedisCluster {
    type O = ListRef<DataRedisClusterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataRedisCluster_ {
    fn extract_datasource_type(&self) -> String {
        "google_redis_cluster".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataRedisCluster {
    pub tf_id: String,
    #[doc = "Unique name of the resource in this scope including project and location using the form:\nprojects/{projectId}/locations/{locationId}/clusters/{clusterId}"]
    pub name: PrimField<String>,
}
impl BuildDataRedisCluster {
    pub fn build(self, stack: &mut Stack) -> DataRedisCluster {
        let out = DataRedisCluster(Rc::new(DataRedisCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRedisClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataRedisClusterRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataRedisClusterRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `authorization_mode` after provisioning.\nOptional. The authorization mode of the Redis cluster. If not provided, auth feature is disabled for the cluster. Default value: \"AUTH_MODE_DISABLED\" Possible values: [\"AUTH_MODE_UNSPECIFIED\", \"AUTH_MODE_IAM_AUTH\", \"AUTH_MODE_DISABLED\"]"]
    pub fn authorization_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorization_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `automated_backup_config` after provisioning.\nThe automated backup config for a instance."]
    pub fn automated_backup_config(&self) -> ListRef<DataRedisClusterAutomatedBackupConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.automated_backup_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `available_maintenance_versions` after provisioning.\nThis field is used to determine the available maintenance versions for the self service update."]
    pub fn available_maintenance_versions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.available_maintenance_versions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_collection` after provisioning.\nThe backup collection full resource name.\nExample: projects/{project}/locations/{location}/backupCollections/{collection}"]
    pub fn backup_collection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_collection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp associated with the cluster creation request. A timestamp in\nRFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional\ndigits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cross_cluster_replication_config` after provisioning.\nCross cluster replication config"]
    pub fn cross_cluster_replication_config(
        &self,
    ) -> ListRef<DataRedisClusterCrossClusterReplicationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cross_cluster_replication_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\nOptional. Indicates if the cluster is deletion protected or not.\nIf the value if set to true, any delete cluster operation will fail.\nDefault value is true."]
    pub fn deletion_protection_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `discovery_endpoints` after provisioning.\nOutput only. Endpoints created on each given network,\nfor Redis clients to connect to the cluster.\nCurrently only one endpoint is supported."]
    pub fn discovery_endpoints(&self) -> ListRef<DataRedisClusterDiscoveryEndpointsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.discovery_endpoints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_maintenance_version` after provisioning.\nThis field represents the actual maintenance version of the cluster."]
    pub fn effective_maintenance_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_maintenance_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_source` after provisioning.\nBackups stored in Cloud Storage buckets. The Cloud Storage buckets need to be the same region as the clusters."]
    pub fn gcs_source(&self) -> ListRef<DataRedisClusterGcsSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcs_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nThe KMS key used to encrypt the at-rest data of the cluster."]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user provided metadata.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\nMaintenance policy for a cluster"]
    pub fn maintenance_policy(&self) -> ListRef<DataRedisClusterMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_schedule` after provisioning.\nUpcoming maintenance schedule."]
    pub fn maintenance_schedule(&self) -> ListRef<DataRedisClusterMaintenanceScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_schedule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_version` after provisioning.\nThis field can be used to trigger self service update to indicate the desired maintenance version. The input to this field can be determined by the available_maintenance_versions field.\n*Note*: This field can only be specified when updating an existing cluster to a newer version. Downgrades are currently not supported!"]
    pub fn maintenance_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed_backup_source` after provisioning.\nBackups that generated and managed by memorystore."]
    pub fn managed_backup_source(&self) -> ListRef<DataRedisClusterManagedBackupSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_backup_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed_server_ca` after provisioning.\nCluster's Certificate Authority. This field will only be populated if Redis Cluster's transit_encryption_mode is TRANSIT_ENCRYPTION_MODE_SERVER_AUTHENTICATION"]
    pub fn managed_server_ca(&self) -> ListRef<DataRedisClusterManagedServerCaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_server_ca", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nUnique name of the resource in this scope including project and location using the form:\nprojects/{projectId}/locations/{locationId}/clusters/{clusterId}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_type` after provisioning.\nThe nodeType for the Redis cluster.\nIf not provided, REDIS_HIGHMEM_MEDIUM will be used as default Possible values: [\"REDIS_SHARED_CORE_NANO\", \"REDIS_HIGHMEM_MEDIUM\", \"REDIS_HIGHCPU_MEDIUM\", \"REDIS_STANDARD_LARGE\", \"REDIS_HIGHMEM_XLARGE\", \"REDIS_HIGHMEM_2XLARGE\", \"REDIS_STANDARD_SMALL\"]"]
    pub fn node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `persistence_config` after provisioning.\nPersistence config (RDB, AOF) for the cluster."]
    pub fn persistence_config(&self) -> ListRef<DataRedisClusterPersistenceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.persistence_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `precise_size_gb` after provisioning.\nOutput only. Redis memory precise size in GB for the entire cluster."]
    pub fn precise_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.precise_size_gb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_configs` after provisioning.\nRequired. Each PscConfig configures the consumer network where two\nnetwork addresses will be designated to the cluster for client access.\nCurrently, only one PscConfig is supported."]
    pub fn psc_configs(&self) -> ListRef<DataRedisClusterPscConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_connections` after provisioning.\nOutput only. PSC connections for discovery of the cluster topology and accessing the cluster."]
    pub fn psc_connections(&self) -> ListRef<DataRedisClusterPscConnectionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_connections", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_service_attachments` after provisioning.\nService attachment details to configure Psc connections."]
    pub fn psc_service_attachments(&self) -> ListRef<DataRedisClusterPscServiceAttachmentsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_service_attachments", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redis_configs` after provisioning.\nConfigure Redis Cluster behavior using a subset of native Redis configuration parameters.\nPlease check Memorystore documentation for the list of supported parameters:\nhttps://cloud.google.com/memorystore/docs/cluster/supported-instance-configurations"]
    pub fn redis_configs(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.redis_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe name of the region of the Redis cluster."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replica_count` after provisioning.\nOptional. The number of replica nodes per shard."]
    pub fn replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replica_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_ca_mode` after provisioning.\nThe serverCaMode for the TLS enabled Redis cluster.\nIf not provided, SERVER_CA_MODE_GOOGLE_MANAGED_PER_INSTANCE_CA will be used as default Possible values: [\"SERVER_CA_MODE_GOOGLE_MANAGED_PER_INSTANCE_CA\", \"SERVER_CA_MODE_GOOGLE_MANAGED_SHARED_CA\", \"SERVER_CA_MODE_CUSTOMER_MANAGED_CAS_CA\", \"SERVER_CA_MODE_UNSPECIFIED\"]"]
    pub fn server_ca_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_ca_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_ca_pool` after provisioning.\nThe resource name of the server CA pool for an instance with SERVER_CA_MODE_CUSTOMER_MANAGED_CAS_CA\nas the server_ca_mode.\nFormat: projects/{project}/locations/{region}/caPools/{caPoolId}"]
    pub fn server_ca_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_ca_pool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `shard_count` after provisioning.\nRequired. Number of shards for the Redis cluster."]
    pub fn shard_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.shard_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `size_gb` after provisioning.\nOutput only. Redis memory size in GB for the entire cluster."]
    pub fn size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.size_gb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of this cluster. Can be CREATING, READY, UPDATING, DELETING and SUSPENDED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_info` after provisioning.\nOutput only. Additional information about the current state of the cluster."]
    pub fn state_info(&self) -> ListRef<DataRedisClusterStateInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.state_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transit_encryption_mode` after provisioning.\nOptional. The in-transit encryption for the Redis cluster.\nIf not provided, encryption is disabled for the cluster. Default value: \"TRANSIT_ENCRYPTION_MODE_DISABLED\" Possible values: [\"TRANSIT_ENCRYPTION_MODE_UNSPECIFIED\", \"TRANSIT_ENCRYPTION_MODE_DISABLED\", \"TRANSIT_ENCRYPTION_MODE_SERVER_AUTHENTICATION\"]"]
    pub fn transit_encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_encryption_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem assigned, unique identifier for the cluster."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `zone_distribution_config` after provisioning.\nImmutable. Zone distribution config for Memorystore Redis cluster."]
    pub fn zone_distribution_config(&self) -> ListRef<DataRedisClusterZoneDistributionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.zone_distribution_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
}
impl DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl {
    #[doc = "Set the field `hours`.\n"]
    pub fn set_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hours = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl {
    type O =
        BlockAssignable<DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl {}
impl BuildDataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl {
    pub fn build(
        self,
    ) -> DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl {
        DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl {
            hours: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeElRef {
        DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hours` after provisioning.\n"]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<
        ListField<DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl>,
    >,
}
impl DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleEl {
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v: impl Into<
            ListField<DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl>,
        >,
    ) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleEl {
    type O = BlockAssignable<DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleEl {}
impl BuildDataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleEl {
    pub fn build(self) -> DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleEl {
        DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleEl {
            start_time: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElRef {
        DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(
        &self,
    ) -> ListRef<DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterAutomatedBackupConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_frequency_schedule:
        Option<ListField<DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention: Option<PrimField<String>>,
}
impl DataRedisClusterAutomatedBackupConfigEl {
    #[doc = "Set the field `fixed_frequency_schedule`.\n"]
    pub fn set_fixed_frequency_schedule(
        mut self,
        v: impl Into<ListField<DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleEl>>,
    ) -> Self {
        self.fixed_frequency_schedule = Some(v.into());
        self
    }
    #[doc = "Set the field `retention`.\n"]
    pub fn set_retention(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.retention = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisClusterAutomatedBackupConfigEl {
    type O = BlockAssignable<DataRedisClusterAutomatedBackupConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterAutomatedBackupConfigEl {}
impl BuildDataRedisClusterAutomatedBackupConfigEl {
    pub fn build(self) -> DataRedisClusterAutomatedBackupConfigEl {
        DataRedisClusterAutomatedBackupConfigEl {
            fixed_frequency_schedule: core::default::Default::default(),
            retention: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterAutomatedBackupConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterAutomatedBackupConfigElRef {
    fn new(shared: StackShared, base: String) -> DataRedisClusterAutomatedBackupConfigElRef {
        DataRedisClusterAutomatedBackupConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterAutomatedBackupConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `fixed_frequency_schedule` after provisioning.\n"]
    pub fn fixed_frequency_schedule(
        &self,
    ) -> ListRef<DataRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fixed_frequency_schedule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `retention` after provisioning.\n"]
    pub fn retention(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.retention", self.base))
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<PrimField<String>>,
}
impl DataRedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterEl {
    #[doc = "Set the field `cluster`.\n"]
    pub fn set_cluster(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster = Some(v.into());
        self
    }
    #[doc = "Set the field `uid`.\n"]
    pub fn set_uid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uid = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataRedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterEl
{
    type O = BlockAssignable<
        DataRedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterEl {}
impl BuildDataRedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterEl {
    pub fn build(
        self,
    ) -> DataRedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterEl {
        DataRedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterEl {
            cluster: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterElRef {
        DataRedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<PrimField<String>>,
}
impl DataRedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersEl {
    #[doc = "Set the field `cluster`.\n"]
    pub fn set_cluster(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster = Some(v.into());
        self
    }
    #[doc = "Set the field `uid`.\n"]
    pub fn set_uid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uid = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataRedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersEl
{
    type O = BlockAssignable<
        DataRedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersEl {}
impl BuildDataRedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersEl {
    pub fn build(
        self,
    ) -> DataRedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersEl {
        DataRedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersEl {
            cluster: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersElRef {
        DataRedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterCrossClusterReplicationConfigElMembershipEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_cluster: Option<
        ListField<DataRedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_clusters: Option<
        ListField<DataRedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersEl>,
    >,
}
impl DataRedisClusterCrossClusterReplicationConfigElMembershipEl {
    #[doc = "Set the field `primary_cluster`.\n"]
    pub fn set_primary_cluster(
        mut self,
        v: impl Into<
            ListField<DataRedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterEl>,
        >,
    ) -> Self {
        self.primary_cluster = Some(v.into());
        self
    }
    #[doc = "Set the field `secondary_clusters`.\n"]
    pub fn set_secondary_clusters(
        mut self,
        v: impl Into<
            ListField<
                DataRedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersEl,
            >,
        >,
    ) -> Self {
        self.secondary_clusters = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisClusterCrossClusterReplicationConfigElMembershipEl {
    type O = BlockAssignable<DataRedisClusterCrossClusterReplicationConfigElMembershipEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterCrossClusterReplicationConfigElMembershipEl {}
impl BuildDataRedisClusterCrossClusterReplicationConfigElMembershipEl {
    pub fn build(self) -> DataRedisClusterCrossClusterReplicationConfigElMembershipEl {
        DataRedisClusterCrossClusterReplicationConfigElMembershipEl {
            primary_cluster: core::default::Default::default(),
            secondary_clusters: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterCrossClusterReplicationConfigElMembershipElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterCrossClusterReplicationConfigElMembershipElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRedisClusterCrossClusterReplicationConfigElMembershipElRef {
        DataRedisClusterCrossClusterReplicationConfigElMembershipElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterCrossClusterReplicationConfigElMembershipElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `primary_cluster` after provisioning.\n"]
    pub fn primary_cluster(
        &self,
    ) -> ListRef<DataRedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.primary_cluster", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_clusters` after provisioning.\n"]
    pub fn secondary_clusters(
        &self,
    ) -> ListRef<DataRedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secondary_clusters", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterCrossClusterReplicationConfigElPrimaryClusterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<PrimField<String>>,
}
impl DataRedisClusterCrossClusterReplicationConfigElPrimaryClusterEl {
    #[doc = "Set the field `cluster`.\n"]
    pub fn set_cluster(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster = Some(v.into());
        self
    }
    #[doc = "Set the field `uid`.\n"]
    pub fn set_uid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uid = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisClusterCrossClusterReplicationConfigElPrimaryClusterEl {
    type O = BlockAssignable<DataRedisClusterCrossClusterReplicationConfigElPrimaryClusterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterCrossClusterReplicationConfigElPrimaryClusterEl {}
impl BuildDataRedisClusterCrossClusterReplicationConfigElPrimaryClusterEl {
    pub fn build(self) -> DataRedisClusterCrossClusterReplicationConfigElPrimaryClusterEl {
        DataRedisClusterCrossClusterReplicationConfigElPrimaryClusterEl {
            cluster: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterCrossClusterReplicationConfigElPrimaryClusterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterCrossClusterReplicationConfigElPrimaryClusterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRedisClusterCrossClusterReplicationConfigElPrimaryClusterElRef {
        DataRedisClusterCrossClusterReplicationConfigElPrimaryClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterCrossClusterReplicationConfigElPrimaryClusterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterCrossClusterReplicationConfigElSecondaryClustersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<PrimField<String>>,
}
impl DataRedisClusterCrossClusterReplicationConfigElSecondaryClustersEl {
    #[doc = "Set the field `cluster`.\n"]
    pub fn set_cluster(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster = Some(v.into());
        self
    }
    #[doc = "Set the field `uid`.\n"]
    pub fn set_uid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uid = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisClusterCrossClusterReplicationConfigElSecondaryClustersEl {
    type O = BlockAssignable<DataRedisClusterCrossClusterReplicationConfigElSecondaryClustersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterCrossClusterReplicationConfigElSecondaryClustersEl {}
impl BuildDataRedisClusterCrossClusterReplicationConfigElSecondaryClustersEl {
    pub fn build(self) -> DataRedisClusterCrossClusterReplicationConfigElSecondaryClustersEl {
        DataRedisClusterCrossClusterReplicationConfigElSecondaryClustersEl {
            cluster: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterCrossClusterReplicationConfigElSecondaryClustersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterCrossClusterReplicationConfigElSecondaryClustersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRedisClusterCrossClusterReplicationConfigElSecondaryClustersElRef {
        DataRedisClusterCrossClusterReplicationConfigElSecondaryClustersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterCrossClusterReplicationConfigElSecondaryClustersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterCrossClusterReplicationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    membership: Option<ListField<DataRedisClusterCrossClusterReplicationConfigElMembershipEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_cluster:
        Option<ListField<DataRedisClusterCrossClusterReplicationConfigElPrimaryClusterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_clusters:
        Option<ListField<DataRedisClusterCrossClusterReplicationConfigElSecondaryClustersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
}
impl DataRedisClusterCrossClusterReplicationConfigEl {
    #[doc = "Set the field `cluster_role`.\n"]
    pub fn set_cluster_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_role = Some(v.into());
        self
    }
    #[doc = "Set the field `membership`.\n"]
    pub fn set_membership(
        mut self,
        v: impl Into<ListField<DataRedisClusterCrossClusterReplicationConfigElMembershipEl>>,
    ) -> Self {
        self.membership = Some(v.into());
        self
    }
    #[doc = "Set the field `primary_cluster`.\n"]
    pub fn set_primary_cluster(
        mut self,
        v: impl Into<ListField<DataRedisClusterCrossClusterReplicationConfigElPrimaryClusterEl>>,
    ) -> Self {
        self.primary_cluster = Some(v.into());
        self
    }
    #[doc = "Set the field `secondary_clusters`.\n"]
    pub fn set_secondary_clusters(
        mut self,
        v: impl Into<ListField<DataRedisClusterCrossClusterReplicationConfigElSecondaryClustersEl>>,
    ) -> Self {
        self.secondary_clusters = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisClusterCrossClusterReplicationConfigEl {
    type O = BlockAssignable<DataRedisClusterCrossClusterReplicationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterCrossClusterReplicationConfigEl {}
impl BuildDataRedisClusterCrossClusterReplicationConfigEl {
    pub fn build(self) -> DataRedisClusterCrossClusterReplicationConfigEl {
        DataRedisClusterCrossClusterReplicationConfigEl {
            cluster_role: core::default::Default::default(),
            membership: core::default::Default::default(),
            primary_cluster: core::default::Default::default(),
            secondary_clusters: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterCrossClusterReplicationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterCrossClusterReplicationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRedisClusterCrossClusterReplicationConfigElRef {
        DataRedisClusterCrossClusterReplicationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterCrossClusterReplicationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster_role` after provisioning.\n"]
    pub fn cluster_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_role", self.base))
    }
    #[doc = "Get a reference to the value of field `membership` after provisioning.\n"]
    pub fn membership(
        &self,
    ) -> ListRef<DataRedisClusterCrossClusterReplicationConfigElMembershipElRef> {
        ListRef::new(self.shared().clone(), format!("{}.membership", self.base))
    }
    #[doc = "Get a reference to the value of field `primary_cluster` after provisioning.\n"]
    pub fn primary_cluster(
        &self,
    ) -> ListRef<DataRedisClusterCrossClusterReplicationConfigElPrimaryClusterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.primary_cluster", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_clusters` after provisioning.\n"]
    pub fn secondary_clusters(
        &self,
    ) -> ListRef<DataRedisClusterCrossClusterReplicationConfigElSecondaryClustersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secondary_clusters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterDiscoveryEndpointsElPscConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
}
impl DataRedisClusterDiscoveryEndpointsElPscConfigEl {
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisClusterDiscoveryEndpointsElPscConfigEl {
    type O = BlockAssignable<DataRedisClusterDiscoveryEndpointsElPscConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterDiscoveryEndpointsElPscConfigEl {}
impl BuildDataRedisClusterDiscoveryEndpointsElPscConfigEl {
    pub fn build(self) -> DataRedisClusterDiscoveryEndpointsElPscConfigEl {
        DataRedisClusterDiscoveryEndpointsElPscConfigEl {
            network: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterDiscoveryEndpointsElPscConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterDiscoveryEndpointsElPscConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRedisClusterDiscoveryEndpointsElPscConfigElRef {
        DataRedisClusterDiscoveryEndpointsElPscConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterDiscoveryEndpointsElPscConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterDiscoveryEndpointsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_config: Option<ListField<DataRedisClusterDiscoveryEndpointsElPscConfigEl>>,
}
impl DataRedisClusterDiscoveryEndpointsEl {
    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `psc_config`.\n"]
    pub fn set_psc_config(
        mut self,
        v: impl Into<ListField<DataRedisClusterDiscoveryEndpointsElPscConfigEl>>,
    ) -> Self {
        self.psc_config = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisClusterDiscoveryEndpointsEl {
    type O = BlockAssignable<DataRedisClusterDiscoveryEndpointsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterDiscoveryEndpointsEl {}
impl BuildDataRedisClusterDiscoveryEndpointsEl {
    pub fn build(self) -> DataRedisClusterDiscoveryEndpointsEl {
        DataRedisClusterDiscoveryEndpointsEl {
            address: core::default::Default::default(),
            port: core::default::Default::default(),
            psc_config: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterDiscoveryEndpointsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterDiscoveryEndpointsElRef {
    fn new(shared: StackShared, base: String) -> DataRedisClusterDiscoveryEndpointsElRef {
        DataRedisClusterDiscoveryEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterDiscoveryEndpointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `psc_config` after provisioning.\n"]
    pub fn psc_config(&self) -> ListRef<DataRedisClusterDiscoveryEndpointsElPscConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.psc_config", self.base))
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterGcsSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    uris: Option<SetField<PrimField<String>>>,
}
impl DataRedisClusterGcsSourceEl {
    #[doc = "Set the field `uris`.\n"]
    pub fn set_uris(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.uris = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisClusterGcsSourceEl {
    type O = BlockAssignable<DataRedisClusterGcsSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterGcsSourceEl {}
impl BuildDataRedisClusterGcsSourceEl {
    pub fn build(self) -> DataRedisClusterGcsSourceEl {
        DataRedisClusterGcsSourceEl {
            uris: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterGcsSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterGcsSourceElRef {
    fn new(shared: StackShared, base: String) -> DataRedisClusterGcsSourceElRef {
        DataRedisClusterGcsSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterGcsSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `uris` after provisioning.\n"]
    pub fn uris(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.uris", self.base))
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    #[doc = "Set the field `hours`.\n"]
    pub fn set_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hours = Some(v.into());
        self
    }
    #[doc = "Set the field `minutes`.\n"]
    pub fn set_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minutes = Some(v.into());
        self
    }
    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    type O =
        BlockAssignable<DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {}
impl BuildDataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    pub fn build(self) -> DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
        DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
        DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hours` after provisioning.\n"]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }
    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time:
        Option<ListField<DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>>,
}
impl DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowEl {
    #[doc = "Set the field `day`.\n"]
    pub fn set_day(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.day = Some(v.into());
        self
    }
    #[doc = "Set the field `duration`.\n"]
    pub fn set_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.duration = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v: impl Into<ListField<DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>>,
    ) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowEl {
    type O = BlockAssignable<DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowEl {}
impl BuildDataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowEl {
    pub fn build(self) -> DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowEl {
        DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowEl {
            day: core::default::Default::default(),
            duration: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElRef {
        DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day` after provisioning.\n"]
    pub fn day(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }
    #[doc = "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(
        &self,
    ) -> ListRef<DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterMaintenancePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_maintenance_window:
        Option<ListField<DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowEl>>,
}
impl DataRedisClusterMaintenancePolicyEl {
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
    #[doc = "Set the field `weekly_maintenance_window`.\n"]
    pub fn set_weekly_maintenance_window(
        mut self,
        v: impl Into<ListField<DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowEl>>,
    ) -> Self {
        self.weekly_maintenance_window = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisClusterMaintenancePolicyEl {
    type O = BlockAssignable<DataRedisClusterMaintenancePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterMaintenancePolicyEl {}
impl BuildDataRedisClusterMaintenancePolicyEl {
    pub fn build(self) -> DataRedisClusterMaintenancePolicyEl {
        DataRedisClusterMaintenancePolicyEl {
            create_time: core::default::Default::default(),
            update_time: core::default::Default::default(),
            weekly_maintenance_window: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterMaintenancePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterMaintenancePolicyElRef {
    fn new(shared: StackShared, base: String) -> DataRedisClusterMaintenancePolicyElRef {
        DataRedisClusterMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterMaintenancePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
    #[doc = "Get a reference to the value of field `weekly_maintenance_window` after provisioning.\n"]
    pub fn weekly_maintenance_window(
        &self,
    ) -> ListRef<DataRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.weekly_maintenance_window", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterMaintenanceScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_deadline_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl DataRedisClusterMaintenanceScheduleEl {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `schedule_deadline_time`.\n"]
    pub fn set_schedule_deadline_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schedule_deadline_time = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisClusterMaintenanceScheduleEl {
    type O = BlockAssignable<DataRedisClusterMaintenanceScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterMaintenanceScheduleEl {}
impl BuildDataRedisClusterMaintenanceScheduleEl {
    pub fn build(self) -> DataRedisClusterMaintenanceScheduleEl {
        DataRedisClusterMaintenanceScheduleEl {
            end_time: core::default::Default::default(),
            schedule_deadline_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterMaintenanceScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterMaintenanceScheduleElRef {
    fn new(shared: StackShared, base: String) -> DataRedisClusterMaintenanceScheduleElRef {
        DataRedisClusterMaintenanceScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterMaintenanceScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `schedule_deadline_time` after provisioning.\n"]
    pub fn schedule_deadline_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schedule_deadline_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterManagedBackupSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backup: Option<PrimField<String>>,
}
impl DataRedisClusterManagedBackupSourceEl {
    #[doc = "Set the field `backup`.\n"]
    pub fn set_backup(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backup = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisClusterManagedBackupSourceEl {
    type O = BlockAssignable<DataRedisClusterManagedBackupSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterManagedBackupSourceEl {}
impl BuildDataRedisClusterManagedBackupSourceEl {
    pub fn build(self) -> DataRedisClusterManagedBackupSourceEl {
        DataRedisClusterManagedBackupSourceEl {
            backup: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterManagedBackupSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterManagedBackupSourceElRef {
    fn new(shared: StackShared, base: String) -> DataRedisClusterManagedBackupSourceElRef {
        DataRedisClusterManagedBackupSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterManagedBackupSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup` after provisioning.\n"]
    pub fn backup(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup", self.base))
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterManagedServerCaElCaCertsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificates: Option<ListField<PrimField<String>>>,
}
impl DataRedisClusterManagedServerCaElCaCertsEl {
    #[doc = "Set the field `certificates`.\n"]
    pub fn set_certificates(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.certificates = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisClusterManagedServerCaElCaCertsEl {
    type O = BlockAssignable<DataRedisClusterManagedServerCaElCaCertsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterManagedServerCaElCaCertsEl {}
impl BuildDataRedisClusterManagedServerCaElCaCertsEl {
    pub fn build(self) -> DataRedisClusterManagedServerCaElCaCertsEl {
        DataRedisClusterManagedServerCaElCaCertsEl {
            certificates: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterManagedServerCaElCaCertsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterManagedServerCaElCaCertsElRef {
    fn new(shared: StackShared, base: String) -> DataRedisClusterManagedServerCaElCaCertsElRef {
        DataRedisClusterManagedServerCaElCaCertsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterManagedServerCaElCaCertsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificates` after provisioning.\n"]
    pub fn certificates(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.certificates", self.base))
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterManagedServerCaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_certs: Option<ListField<DataRedisClusterManagedServerCaElCaCertsEl>>,
}
impl DataRedisClusterManagedServerCaEl {
    #[doc = "Set the field `ca_certs`.\n"]
    pub fn set_ca_certs(
        mut self,
        v: impl Into<ListField<DataRedisClusterManagedServerCaElCaCertsEl>>,
    ) -> Self {
        self.ca_certs = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisClusterManagedServerCaEl {
    type O = BlockAssignable<DataRedisClusterManagedServerCaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterManagedServerCaEl {}
impl BuildDataRedisClusterManagedServerCaEl {
    pub fn build(self) -> DataRedisClusterManagedServerCaEl {
        DataRedisClusterManagedServerCaEl {
            ca_certs: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterManagedServerCaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterManagedServerCaElRef {
    fn new(shared: StackShared, base: String) -> DataRedisClusterManagedServerCaElRef {
        DataRedisClusterManagedServerCaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterManagedServerCaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ca_certs` after provisioning.\n"]
    pub fn ca_certs(&self) -> ListRef<DataRedisClusterManagedServerCaElCaCertsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ca_certs", self.base))
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterPersistenceConfigElAofConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    append_fsync: Option<PrimField<String>>,
}
impl DataRedisClusterPersistenceConfigElAofConfigEl {
    #[doc = "Set the field `append_fsync`.\n"]
    pub fn set_append_fsync(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.append_fsync = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisClusterPersistenceConfigElAofConfigEl {
    type O = BlockAssignable<DataRedisClusterPersistenceConfigElAofConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterPersistenceConfigElAofConfigEl {}
impl BuildDataRedisClusterPersistenceConfigElAofConfigEl {
    pub fn build(self) -> DataRedisClusterPersistenceConfigElAofConfigEl {
        DataRedisClusterPersistenceConfigElAofConfigEl {
            append_fsync: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterPersistenceConfigElAofConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterPersistenceConfigElAofConfigElRef {
    fn new(shared: StackShared, base: String) -> DataRedisClusterPersistenceConfigElAofConfigElRef {
        DataRedisClusterPersistenceConfigElAofConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterPersistenceConfigElAofConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `append_fsync` after provisioning.\n"]
    pub fn append_fsync(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.append_fsync", self.base))
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterPersistenceConfigElRdbConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rdb_snapshot_period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rdb_snapshot_start_time: Option<PrimField<String>>,
}
impl DataRedisClusterPersistenceConfigElRdbConfigEl {
    #[doc = "Set the field `rdb_snapshot_period`.\n"]
    pub fn set_rdb_snapshot_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rdb_snapshot_period = Some(v.into());
        self
    }
    #[doc = "Set the field `rdb_snapshot_start_time`.\n"]
    pub fn set_rdb_snapshot_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rdb_snapshot_start_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisClusterPersistenceConfigElRdbConfigEl {
    type O = BlockAssignable<DataRedisClusterPersistenceConfigElRdbConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterPersistenceConfigElRdbConfigEl {}
impl BuildDataRedisClusterPersistenceConfigElRdbConfigEl {
    pub fn build(self) -> DataRedisClusterPersistenceConfigElRdbConfigEl {
        DataRedisClusterPersistenceConfigElRdbConfigEl {
            rdb_snapshot_period: core::default::Default::default(),
            rdb_snapshot_start_time: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterPersistenceConfigElRdbConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterPersistenceConfigElRdbConfigElRef {
    fn new(shared: StackShared, base: String) -> DataRedisClusterPersistenceConfigElRdbConfigElRef {
        DataRedisClusterPersistenceConfigElRdbConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterPersistenceConfigElRdbConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `rdb_snapshot_period` after provisioning.\n"]
    pub fn rdb_snapshot_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rdb_snapshot_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rdb_snapshot_start_time` after provisioning.\n"]
    pub fn rdb_snapshot_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rdb_snapshot_start_time", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterPersistenceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aof_config: Option<ListField<DataRedisClusterPersistenceConfigElAofConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rdb_config: Option<ListField<DataRedisClusterPersistenceConfigElRdbConfigEl>>,
}
impl DataRedisClusterPersistenceConfigEl {
    #[doc = "Set the field `aof_config`.\n"]
    pub fn set_aof_config(
        mut self,
        v: impl Into<ListField<DataRedisClusterPersistenceConfigElAofConfigEl>>,
    ) -> Self {
        self.aof_config = Some(v.into());
        self
    }
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
    #[doc = "Set the field `rdb_config`.\n"]
    pub fn set_rdb_config(
        mut self,
        v: impl Into<ListField<DataRedisClusterPersistenceConfigElRdbConfigEl>>,
    ) -> Self {
        self.rdb_config = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisClusterPersistenceConfigEl {
    type O = BlockAssignable<DataRedisClusterPersistenceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterPersistenceConfigEl {}
impl BuildDataRedisClusterPersistenceConfigEl {
    pub fn build(self) -> DataRedisClusterPersistenceConfigEl {
        DataRedisClusterPersistenceConfigEl {
            aof_config: core::default::Default::default(),
            mode: core::default::Default::default(),
            rdb_config: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterPersistenceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterPersistenceConfigElRef {
    fn new(shared: StackShared, base: String) -> DataRedisClusterPersistenceConfigElRef {
        DataRedisClusterPersistenceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterPersistenceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aof_config` after provisioning.\n"]
    pub fn aof_config(&self) -> ListRef<DataRedisClusterPersistenceConfigElAofConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.aof_config", self.base))
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `rdb_config` after provisioning.\n"]
    pub fn rdb_config(&self) -> ListRef<DataRedisClusterPersistenceConfigElRdbConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rdb_config", self.base))
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterPscConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
}
impl DataRedisClusterPscConfigsEl {
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisClusterPscConfigsEl {
    type O = BlockAssignable<DataRedisClusterPscConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterPscConfigsEl {}
impl BuildDataRedisClusterPscConfigsEl {
    pub fn build(self) -> DataRedisClusterPscConfigsEl {
        DataRedisClusterPscConfigsEl {
            network: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterPscConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterPscConfigsElRef {
    fn new(shared: StackShared, base: String) -> DataRedisClusterPscConfigsElRef {
        DataRedisClusterPscConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterPscConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterPscConnectionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forwarding_rule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_connection_id: Option<PrimField<String>>,
}
impl DataRedisClusterPscConnectionsEl {
    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address = Some(v.into());
        self
    }
    #[doc = "Set the field `forwarding_rule`.\n"]
    pub fn set_forwarding_rule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.forwarding_rule = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
    #[doc = "Set the field `psc_connection_id`.\n"]
    pub fn set_psc_connection_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.psc_connection_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisClusterPscConnectionsEl {
    type O = BlockAssignable<DataRedisClusterPscConnectionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterPscConnectionsEl {}
impl BuildDataRedisClusterPscConnectionsEl {
    pub fn build(self) -> DataRedisClusterPscConnectionsEl {
        DataRedisClusterPscConnectionsEl {
            address: core::default::Default::default(),
            forwarding_rule: core::default::Default::default(),
            network: core::default::Default::default(),
            project_id: core::default::Default::default(),
            psc_connection_id: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterPscConnectionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterPscConnectionsElRef {
    fn new(shared: StackShared, base: String) -> DataRedisClusterPscConnectionsElRef {
        DataRedisClusterPscConnectionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterPscConnectionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }
    #[doc = "Get a reference to the value of field `forwarding_rule` after provisioning.\n"]
    pub fn forwarding_rule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.forwarding_rule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `psc_connection_id` after provisioning.\n"]
    pub fn psc_connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.psc_connection_id", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterPscServiceAttachmentsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_attachment: Option<PrimField<String>>,
}
impl DataRedisClusterPscServiceAttachmentsEl {
    #[doc = "Set the field `connection_type`.\n"]
    pub fn set_connection_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connection_type = Some(v.into());
        self
    }
    #[doc = "Set the field `service_attachment`.\n"]
    pub fn set_service_attachment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_attachment = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisClusterPscServiceAttachmentsEl {
    type O = BlockAssignable<DataRedisClusterPscServiceAttachmentsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterPscServiceAttachmentsEl {}
impl BuildDataRedisClusterPscServiceAttachmentsEl {
    pub fn build(self) -> DataRedisClusterPscServiceAttachmentsEl {
        DataRedisClusterPscServiceAttachmentsEl {
            connection_type: core::default::Default::default(),
            service_attachment: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterPscServiceAttachmentsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterPscServiceAttachmentsElRef {
    fn new(shared: StackShared, base: String) -> DataRedisClusterPscServiceAttachmentsElRef {
        DataRedisClusterPscServiceAttachmentsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterPscServiceAttachmentsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_type` after provisioning.\n"]
    pub fn connection_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_attachment` after provisioning.\n"]
    pub fn service_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_attachment", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterStateInfoElUpdateInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target_replica_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_shard_count: Option<PrimField<f64>>,
}
impl DataRedisClusterStateInfoElUpdateInfoEl {
    #[doc = "Set the field `target_replica_count`.\n"]
    pub fn set_target_replica_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.target_replica_count = Some(v.into());
        self
    }
    #[doc = "Set the field `target_shard_count`.\n"]
    pub fn set_target_shard_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.target_shard_count = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisClusterStateInfoElUpdateInfoEl {
    type O = BlockAssignable<DataRedisClusterStateInfoElUpdateInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterStateInfoElUpdateInfoEl {}
impl BuildDataRedisClusterStateInfoElUpdateInfoEl {
    pub fn build(self) -> DataRedisClusterStateInfoElUpdateInfoEl {
        DataRedisClusterStateInfoElUpdateInfoEl {
            target_replica_count: core::default::Default::default(),
            target_shard_count: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterStateInfoElUpdateInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterStateInfoElUpdateInfoElRef {
    fn new(shared: StackShared, base: String) -> DataRedisClusterStateInfoElUpdateInfoElRef {
        DataRedisClusterStateInfoElUpdateInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterStateInfoElUpdateInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target_replica_count` after provisioning.\n"]
    pub fn target_replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_replica_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_shard_count` after provisioning.\n"]
    pub fn target_shard_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_shard_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterStateInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    update_info: Option<ListField<DataRedisClusterStateInfoElUpdateInfoEl>>,
}
impl DataRedisClusterStateInfoEl {
    #[doc = "Set the field `update_info`.\n"]
    pub fn set_update_info(
        mut self,
        v: impl Into<ListField<DataRedisClusterStateInfoElUpdateInfoEl>>,
    ) -> Self {
        self.update_info = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisClusterStateInfoEl {
    type O = BlockAssignable<DataRedisClusterStateInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterStateInfoEl {}
impl BuildDataRedisClusterStateInfoEl {
    pub fn build(self) -> DataRedisClusterStateInfoEl {
        DataRedisClusterStateInfoEl {
            update_info: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterStateInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterStateInfoElRef {
    fn new(shared: StackShared, base: String) -> DataRedisClusterStateInfoElRef {
        DataRedisClusterStateInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterStateInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `update_info` after provisioning.\n"]
    pub fn update_info(&self) -> ListRef<DataRedisClusterStateInfoElUpdateInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.update_info", self.base))
    }
}
#[derive(Serialize)]
pub struct DataRedisClusterZoneDistributionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}
impl DataRedisClusterZoneDistributionConfigEl {
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zone = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisClusterZoneDistributionConfigEl {
    type O = BlockAssignable<DataRedisClusterZoneDistributionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisClusterZoneDistributionConfigEl {}
impl BuildDataRedisClusterZoneDistributionConfigEl {
    pub fn build(self) -> DataRedisClusterZoneDistributionConfigEl {
        DataRedisClusterZoneDistributionConfigEl {
            mode: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}
pub struct DataRedisClusterZoneDistributionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisClusterZoneDistributionConfigElRef {
    fn new(shared: StackShared, base: String) -> DataRedisClusterZoneDistributionConfigElRef {
        DataRedisClusterZoneDistributionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisClusterZoneDistributionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}
