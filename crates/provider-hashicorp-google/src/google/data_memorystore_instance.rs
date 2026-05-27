use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataMemorystoreInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataMemorystoreInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataMemorystoreInstanceData>,
}
#[derive(Clone)]
pub struct DataMemorystoreInstance(Rc<DataMemorystoreInstance_>);
impl DataMemorystoreInstance {
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
    #[doc = "Set the field `location`.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122. See documentation for resource type 'memorystore.googleapis.com/CertificateAuthority'."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `authorization_mode` after provisioning.\nOptional. Immutable. Authorization mode of the instance. Possible values:\n AUTH_DISABLED\nIAM_AUTH"]
    pub fn authorization_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorization_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `automated_backup_config` after provisioning.\nThe automated backup config for a instance."]
    pub fn automated_backup_config(
        &self,
    ) -> ListRef<DataMemorystoreInstanceAutomatedBackupConfigElRef> {
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
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Creation timestamp of the instance."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cross_instance_replication_config` after provisioning.\nCross instance replication config"]
    pub fn cross_instance_replication_config(
        &self,
    ) -> ListRef<DataMemorystoreInstanceCrossInstanceReplicationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cross_instance_replication_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\nOptional. If set to true deletion of the instance will fail."]
    pub fn deletion_protection_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desired_auto_created_endpoints` after provisioning.\nImmutable. User inputs for the auto-created endpoints connections."]
    pub fn desired_auto_created_endpoints(
        &self,
    ) -> ListRef<DataMemorystoreInstanceDesiredAutoCreatedEndpointsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.desired_auto_created_endpoints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desired_psc_auto_connections` after provisioning.\n'desired_psc_auto_connections' is deprecated  Use 'desired_auto_created_endpoints' instead 'terraform import' will only work with desired_auto_created_endpoints'."]
    pub fn desired_psc_auto_connections(
        &self,
    ) -> ListRef<DataMemorystoreInstanceDesiredPscAutoConnectionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.desired_psc_auto_connections", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `discovery_endpoints` after provisioning.\nDeprecated. Output only. Endpoints clients can connect to the instance through."]
    pub fn discovery_endpoints(&self) -> ListRef<DataMemorystoreInstanceDiscoveryEndpointsElRef> {
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
    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\nEndpoints for the instance."]
    pub fn endpoints(&self) -> ListRef<DataMemorystoreInstanceEndpointsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_configs` after provisioning.\nOptional. User-provided engine configurations for the instance."]
    pub fn engine_configs(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.engine_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\nOptional. Engine version of the instance."]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_source` after provisioning.\nGCS source for the instance."]
    pub fn gcs_source(&self) -> ListRef<DataMemorystoreInstanceGcsSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcs_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\nRequired. The ID to use for the instance, which will become the final component of\nthe instance's resource name.\n\nThis value is subject to the following restrictions:\n\n* Must be 4-63 characters in length\n* Must begin with a letter or digit\n* Must contain only lowercase letters, digits, and hyphens\n* Must not end with a hyphen\n* Must be unique within a location"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nThe KMS key used to encrypt the at-rest data of the cluster"]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels to represent user-provided metadata. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122. See documentation for resource type 'memorystore.googleapis.com/CertificateAuthority'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\nMaintenance policy for a cluster"]
    pub fn maintenance_policy(&self) -> ListRef<DataMemorystoreInstanceMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_schedule` after provisioning.\nUpcoming maintenance schedule."]
    pub fn maintenance_schedule(&self) -> ListRef<DataMemorystoreInstanceMaintenanceScheduleElRef> {
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
    #[doc = "Get a reference to the value of field `managed_backup_source` after provisioning.\nManaged backup source for the instance."]
    pub fn managed_backup_source(
        &self,
    ) -> ListRef<DataMemorystoreInstanceManagedBackupSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_backup_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed_server_ca` after provisioning.\nInstance's Certificate Authority. This field will only be populated if instance's transit_encryption_mode is SERVER_AUTHENTICATION"]
    pub fn managed_server_ca(&self) -> ListRef<DataMemorystoreInstanceManagedServerCaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_server_ca", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nOptional. cluster or cluster-disabled. \n Possible values:\n CLUSTER\n CLUSTER_DISABLED Possible values: [\"CLUSTER\", \"CLUSTER_DISABLED\"]"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Unique name of the instance.\nFormat: projects/{project}/locations/{location}/instances/{instance}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_config` after provisioning.\nRepresents configuration for nodes of the instance."]
    pub fn node_config(&self) -> ListRef<DataMemorystoreInstanceNodeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_type` after provisioning.\nOptional. Machine type for individual nodes of the instance. \n Possible values:\n SHARED_CORE_NANO\nCUSTOM_PICO\nCUSTOM_MICRO\nCUSTOM_MINI\nHIGHMEM_MEDIUM\nHIGHCPU_MEDIUM\nHIGHMEM_XLARGE\nSTANDARD_SMALL\nSTANDARD_LARGE\nHIGHMEM_2XLARGE"]
    pub fn node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `persistence_config` after provisioning.\nRepresents persistence configuration for a instance."]
    pub fn persistence_config(&self) -> ListRef<DataMemorystoreInstancePersistenceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.persistence_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_attachment_details` after provisioning.\nConfiguration of a service attachment of the cluster, for creating PSC connections."]
    pub fn psc_attachment_details(
        &self,
    ) -> ListRef<DataMemorystoreInstancePscAttachmentDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_attachment_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_auto_connections` after provisioning.\nOutput only. User inputs and resource details of the auto-created PSC connections."]
    pub fn psc_auto_connections(&self) -> ListRef<DataMemorystoreInstancePscAutoConnectionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_auto_connections", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replica_count` after provisioning.\nOptional. Number of replica nodes per shard. If omitted the default is 0 replicas."]
    pub fn replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replica_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_ca_mode` after provisioning.\nThe serverCaMode for the TLS enabled Memorystore instance.\nIf not provided, GOOGLE_MANAGED_PER_INSTANCE_CA will be used as default Possible values: [\"GOOGLE_MANAGED_PER_INSTANCE_CA\", \"GOOGLE_MANAGED_SHARED_CA\", \"CUSTOMER_MANAGED_CAS_CA\", \"SERVER_CA_MODE_UNSPECIFIED\"]"]
    pub fn server_ca_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_ca_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_ca_pool` after provisioning.\nThe resource name of the server CA pool for an instance with CUSTOMER_MANAGED_CAS_CA\nas the server_ca_mode.\nFormat: projects/{project}/locations/{region}/caPools/{caPoolId}"]
    pub fn server_ca_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_ca_pool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `shard_count` after provisioning.\nRequired. Number of shards for the instance."]
    pub fn shard_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.shard_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. Current state of the instance. \n Possible values:\n CREATING\nACTIVE\nUPDATING\nDELETING"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_info` after provisioning.\nAdditional information about the state of the instance."]
    pub fn state_info(&self) -> ListRef<DataMemorystoreInstanceStateInfoElRef> {
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
    #[doc = "Get a reference to the value of field `transit_encryption_mode` after provisioning.\nOptional. Immutable. In-transit encryption mode of the instance. \n Possible values:\n TRANSIT_ENCRYPTION_DISABLED\nSERVER_AUTHENTICATION"]
    pub fn transit_encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_encryption_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. System assigned, unique identifier for the instance."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Latest update timestamp of the instance."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone_distribution_config` after provisioning.\nZone distribution configuration for allocation of instance resources."]
    pub fn zone_distribution_config(
        &self,
    ) -> ListRef<DataMemorystoreInstanceZoneDistributionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.zone_distribution_config", self.extract_ref()),
        )
    }
}
impl Referable for DataMemorystoreInstance {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataMemorystoreInstance {}
impl ToListMappable for DataMemorystoreInstance {
    type O = ListRef<DataMemorystoreInstanceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataMemorystoreInstance_ {
    fn extract_datasource_type(&self) -> String {
        "google_memorystore_instance".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataMemorystoreInstance {
    pub tf_id: String,
    #[doc = "Required. The ID to use for the instance, which will become the final component of\nthe instance's resource name.\n\nThis value is subject to the following restrictions:\n\n* Must be 4-63 characters in length\n* Must begin with a letter or digit\n* Must contain only lowercase letters, digits, and hyphens\n* Must not end with a hyphen\n* Must be unique within a location"]
    pub instance_id: PrimField<String>,
}
impl BuildDataMemorystoreInstance {
    pub fn build(self, stack: &mut Stack) -> DataMemorystoreInstance {
        let out = DataMemorystoreInstance(Rc::new(DataMemorystoreInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataMemorystoreInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                location: core::default::Default::default(),
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataMemorystoreInstanceRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataMemorystoreInstanceRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `authorization_mode` after provisioning.\nOptional. Immutable. Authorization mode of the instance. Possible values:\n AUTH_DISABLED\nIAM_AUTH"]
    pub fn authorization_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorization_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `automated_backup_config` after provisioning.\nThe automated backup config for a instance."]
    pub fn automated_backup_config(
        &self,
    ) -> ListRef<DataMemorystoreInstanceAutomatedBackupConfigElRef> {
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
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Creation timestamp of the instance."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cross_instance_replication_config` after provisioning.\nCross instance replication config"]
    pub fn cross_instance_replication_config(
        &self,
    ) -> ListRef<DataMemorystoreInstanceCrossInstanceReplicationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cross_instance_replication_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\nOptional. If set to true deletion of the instance will fail."]
    pub fn deletion_protection_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desired_auto_created_endpoints` after provisioning.\nImmutable. User inputs for the auto-created endpoints connections."]
    pub fn desired_auto_created_endpoints(
        &self,
    ) -> ListRef<DataMemorystoreInstanceDesiredAutoCreatedEndpointsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.desired_auto_created_endpoints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desired_psc_auto_connections` after provisioning.\n'desired_psc_auto_connections' is deprecated  Use 'desired_auto_created_endpoints' instead 'terraform import' will only work with desired_auto_created_endpoints'."]
    pub fn desired_psc_auto_connections(
        &self,
    ) -> ListRef<DataMemorystoreInstanceDesiredPscAutoConnectionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.desired_psc_auto_connections", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `discovery_endpoints` after provisioning.\nDeprecated. Output only. Endpoints clients can connect to the instance through."]
    pub fn discovery_endpoints(&self) -> ListRef<DataMemorystoreInstanceDiscoveryEndpointsElRef> {
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
    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\nEndpoints for the instance."]
    pub fn endpoints(&self) -> ListRef<DataMemorystoreInstanceEndpointsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_configs` after provisioning.\nOptional. User-provided engine configurations for the instance."]
    pub fn engine_configs(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.engine_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\nOptional. Engine version of the instance."]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_source` after provisioning.\nGCS source for the instance."]
    pub fn gcs_source(&self) -> ListRef<DataMemorystoreInstanceGcsSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcs_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\nRequired. The ID to use for the instance, which will become the final component of\nthe instance's resource name.\n\nThis value is subject to the following restrictions:\n\n* Must be 4-63 characters in length\n* Must begin with a letter or digit\n* Must contain only lowercase letters, digits, and hyphens\n* Must not end with a hyphen\n* Must be unique within a location"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nThe KMS key used to encrypt the at-rest data of the cluster"]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels to represent user-provided metadata. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122. See documentation for resource type 'memorystore.googleapis.com/CertificateAuthority'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\nMaintenance policy for a cluster"]
    pub fn maintenance_policy(&self) -> ListRef<DataMemorystoreInstanceMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_schedule` after provisioning.\nUpcoming maintenance schedule."]
    pub fn maintenance_schedule(&self) -> ListRef<DataMemorystoreInstanceMaintenanceScheduleElRef> {
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
    #[doc = "Get a reference to the value of field `managed_backup_source` after provisioning.\nManaged backup source for the instance."]
    pub fn managed_backup_source(
        &self,
    ) -> ListRef<DataMemorystoreInstanceManagedBackupSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_backup_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed_server_ca` after provisioning.\nInstance's Certificate Authority. This field will only be populated if instance's transit_encryption_mode is SERVER_AUTHENTICATION"]
    pub fn managed_server_ca(&self) -> ListRef<DataMemorystoreInstanceManagedServerCaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_server_ca", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nOptional. cluster or cluster-disabled. \n Possible values:\n CLUSTER\n CLUSTER_DISABLED Possible values: [\"CLUSTER\", \"CLUSTER_DISABLED\"]"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Unique name of the instance.\nFormat: projects/{project}/locations/{location}/instances/{instance}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_config` after provisioning.\nRepresents configuration for nodes of the instance."]
    pub fn node_config(&self) -> ListRef<DataMemorystoreInstanceNodeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_type` after provisioning.\nOptional. Machine type for individual nodes of the instance. \n Possible values:\n SHARED_CORE_NANO\nCUSTOM_PICO\nCUSTOM_MICRO\nCUSTOM_MINI\nHIGHMEM_MEDIUM\nHIGHCPU_MEDIUM\nHIGHMEM_XLARGE\nSTANDARD_SMALL\nSTANDARD_LARGE\nHIGHMEM_2XLARGE"]
    pub fn node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `persistence_config` after provisioning.\nRepresents persistence configuration for a instance."]
    pub fn persistence_config(&self) -> ListRef<DataMemorystoreInstancePersistenceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.persistence_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_attachment_details` after provisioning.\nConfiguration of a service attachment of the cluster, for creating PSC connections."]
    pub fn psc_attachment_details(
        &self,
    ) -> ListRef<DataMemorystoreInstancePscAttachmentDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_attachment_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_auto_connections` after provisioning.\nOutput only. User inputs and resource details of the auto-created PSC connections."]
    pub fn psc_auto_connections(&self) -> ListRef<DataMemorystoreInstancePscAutoConnectionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_auto_connections", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replica_count` after provisioning.\nOptional. Number of replica nodes per shard. If omitted the default is 0 replicas."]
    pub fn replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replica_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_ca_mode` after provisioning.\nThe serverCaMode for the TLS enabled Memorystore instance.\nIf not provided, GOOGLE_MANAGED_PER_INSTANCE_CA will be used as default Possible values: [\"GOOGLE_MANAGED_PER_INSTANCE_CA\", \"GOOGLE_MANAGED_SHARED_CA\", \"CUSTOMER_MANAGED_CAS_CA\", \"SERVER_CA_MODE_UNSPECIFIED\"]"]
    pub fn server_ca_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_ca_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_ca_pool` after provisioning.\nThe resource name of the server CA pool for an instance with CUSTOMER_MANAGED_CAS_CA\nas the server_ca_mode.\nFormat: projects/{project}/locations/{region}/caPools/{caPoolId}"]
    pub fn server_ca_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_ca_pool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `shard_count` after provisioning.\nRequired. Number of shards for the instance."]
    pub fn shard_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.shard_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. Current state of the instance. \n Possible values:\n CREATING\nACTIVE\nUPDATING\nDELETING"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_info` after provisioning.\nAdditional information about the state of the instance."]
    pub fn state_info(&self) -> ListRef<DataMemorystoreInstanceStateInfoElRef> {
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
    #[doc = "Get a reference to the value of field `transit_encryption_mode` after provisioning.\nOptional. Immutable. In-transit encryption mode of the instance. \n Possible values:\n TRANSIT_ENCRYPTION_DISABLED\nSERVER_AUTHENTICATION"]
    pub fn transit_encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_encryption_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. System assigned, unique identifier for the instance."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Latest update timestamp of the instance."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone_distribution_config` after provisioning.\nZone distribution configuration for allocation of instance resources."]
    pub fn zone_distribution_config(
        &self,
    ) -> ListRef<DataMemorystoreInstanceZoneDistributionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.zone_distribution_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
}
impl DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl {
    #[doc = "Set the field `hours`.\n"]
    pub fn set_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hours = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl
{
    type O = BlockAssignable<
        DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl {
}
impl BuildDataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl {
    pub fn build(
        self,
    ) -> DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl {
        DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl {
            hours: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeElRef {
        DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hours` after provisioning.\n"]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<
        ListField<
            DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl,
        >,
    >,
}
impl DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleEl {
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v: impl Into<
            ListField<
                DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl,
            >,
        >,
    ) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleEl {
    type O =
        BlockAssignable<DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleEl {}
impl BuildDataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleEl {
    pub fn build(self) -> DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleEl {
        DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleEl {
            start_time: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElRef {
        DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(
        &self,
    ) -> ListRef<DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMemorystoreInstanceAutomatedBackupConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_frequency_schedule:
        Option<ListField<DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention: Option<PrimField<String>>,
}
impl DataMemorystoreInstanceAutomatedBackupConfigEl {
    #[doc = "Set the field `fixed_frequency_schedule`.\n"]
    pub fn set_fixed_frequency_schedule(
        mut self,
        v: impl Into<ListField<DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleEl>>,
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
impl ToListMappable for DataMemorystoreInstanceAutomatedBackupConfigEl {
    type O = BlockAssignable<DataMemorystoreInstanceAutomatedBackupConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstanceAutomatedBackupConfigEl {}
impl BuildDataMemorystoreInstanceAutomatedBackupConfigEl {
    pub fn build(self) -> DataMemorystoreInstanceAutomatedBackupConfigEl {
        DataMemorystoreInstanceAutomatedBackupConfigEl {
            fixed_frequency_schedule: core::default::Default::default(),
            retention: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstanceAutomatedBackupConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstanceAutomatedBackupConfigElRef {
    fn new(shared: StackShared, base: String) -> DataMemorystoreInstanceAutomatedBackupConfigElRef {
        DataMemorystoreInstanceAutomatedBackupConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstanceAutomatedBackupConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `fixed_frequency_schedule` after provisioning.\n"]
    pub fn fixed_frequency_schedule(
        &self,
    ) -> ListRef<DataMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElRef> {
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
pub struct DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<PrimField<String>>,
}
impl DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceEl {
    #[doc = "Set the field `instance`.\n"]
    pub fn set_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance = Some(v.into());
        self
    }
    #[doc = "Set the field `uid`.\n"]
    pub fn set_uid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uid = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceEl
{
    type O = BlockAssignable<
        DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceEl
{}
impl BuildDataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceEl {
    pub fn build(
        self,
    ) -> DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceEl {
        DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceEl {
            instance: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceElRef
    {
        DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\n"]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<PrimField<String>>,
}
impl DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceEl {
    #[doc = "Set the field `instance`.\n"]
    pub fn set_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance = Some(v.into());
        self
    }
    #[doc = "Set the field `uid`.\n"]
    pub fn set_uid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uid = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceEl
{
    type O = BlockAssignable<
        DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceEl
{}
impl BuildDataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceEl {
    pub fn build(
        self,
    ) -> DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceEl
    {
        DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceEl {
            instance: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceElRef
    {
        DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\n"]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_instance: Option<
        ListField<
            DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_instance: Option<
        ListField<
            DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceEl,
        >,
    >,
}
impl DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipEl {
    #[doc = "Set the field `primary_instance`.\n"]
    pub fn set_primary_instance(
        mut self,
        v : impl Into < ListField < DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceEl > >,
    ) -> Self {
        self.primary_instance = Some(v.into());
        self
    }
    #[doc = "Set the field `secondary_instance`.\n"]
    pub fn set_secondary_instance(
        mut self,
        v : impl Into < ListField < DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceEl > >,
    ) -> Self {
        self.secondary_instance = Some(v.into());
        self
    }
}
impl ToListMappable for DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipEl {
    type O = BlockAssignable<DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipEl {}
impl BuildDataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipEl {
    pub fn build(self) -> DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipEl {
        DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipEl {
            primary_instance: core::default::Default::default(),
            secondary_instance: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElRef {
        DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `primary_instance` after provisioning.\n"]
    pub fn primary_instance(
        &self,
    ) -> ListRef<
        DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.primary_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_instance` after provisioning.\n"]
    pub fn secondary_instance(
        &self,
    ) -> ListRef<
        DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secondary_instance", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataMemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<PrimField<String>>,
}
impl DataMemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceEl {
    #[doc = "Set the field `instance`.\n"]
    pub fn set_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance = Some(v.into());
        self
    }
    #[doc = "Set the field `uid`.\n"]
    pub fn set_uid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uid = Some(v.into());
        self
    }
}
impl ToListMappable for DataMemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceEl {
    type O =
        BlockAssignable<DataMemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceEl {}
impl BuildDataMemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceEl {
    pub fn build(self) -> DataMemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceEl {
        DataMemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceEl {
            instance: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataMemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceElRef {
        DataMemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\n"]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<PrimField<String>>,
}
impl DataMemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesEl {
    #[doc = "Set the field `instance`.\n"]
    pub fn set_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance = Some(v.into());
        self
    }
    #[doc = "Set the field `uid`.\n"]
    pub fn set_uid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uid = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataMemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesEl
{
    type O = BlockAssignable<
        DataMemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesEl {}
impl BuildDataMemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesEl {
    pub fn build(
        self,
    ) -> DataMemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesEl {
        DataMemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesEl {
            instance: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataMemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesElRef {
        DataMemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\n"]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMemorystoreInstanceCrossInstanceReplicationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    membership:
        Option<ListField<DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_instance:
        Option<ListField<DataMemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_instances: Option<
        ListField<DataMemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
}
impl DataMemorystoreInstanceCrossInstanceReplicationConfigEl {
    #[doc = "Set the field `instance_role`.\n"]
    pub fn set_instance_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_role = Some(v.into());
        self
    }
    #[doc = "Set the field `membership`.\n"]
    pub fn set_membership(
        mut self,
        v: impl Into<ListField<DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipEl>>,
    ) -> Self {
        self.membership = Some(v.into());
        self
    }
    #[doc = "Set the field `primary_instance`.\n"]
    pub fn set_primary_instance(
        mut self,
        v: impl Into<
            ListField<DataMemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceEl>,
        >,
    ) -> Self {
        self.primary_instance = Some(v.into());
        self
    }
    #[doc = "Set the field `secondary_instances`.\n"]
    pub fn set_secondary_instances(
        mut self,
        v: impl Into<
            ListField<DataMemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesEl>,
        >,
    ) -> Self {
        self.secondary_instances = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataMemorystoreInstanceCrossInstanceReplicationConfigEl {
    type O = BlockAssignable<DataMemorystoreInstanceCrossInstanceReplicationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstanceCrossInstanceReplicationConfigEl {}
impl BuildDataMemorystoreInstanceCrossInstanceReplicationConfigEl {
    pub fn build(self) -> DataMemorystoreInstanceCrossInstanceReplicationConfigEl {
        DataMemorystoreInstanceCrossInstanceReplicationConfigEl {
            instance_role: core::default::Default::default(),
            membership: core::default::Default::default(),
            primary_instance: core::default::Default::default(),
            secondary_instances: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstanceCrossInstanceReplicationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstanceCrossInstanceReplicationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataMemorystoreInstanceCrossInstanceReplicationConfigElRef {
        DataMemorystoreInstanceCrossInstanceReplicationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstanceCrossInstanceReplicationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_role` after provisioning.\n"]
    pub fn instance_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_role", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `membership` after provisioning.\n"]
    pub fn membership(
        &self,
    ) -> ListRef<DataMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElRef> {
        ListRef::new(self.shared().clone(), format!("{}.membership", self.base))
    }
    #[doc = "Get a reference to the value of field `primary_instance` after provisioning.\n"]
    pub fn primary_instance(
        &self,
    ) -> ListRef<DataMemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.primary_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_instances` after provisioning.\n"]
    pub fn secondary_instances(
        &self,
    ) -> ListRef<DataMemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secondary_instances", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMemorystoreInstanceDesiredAutoCreatedEndpointsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
}
impl DataMemorystoreInstanceDesiredAutoCreatedEndpointsEl {
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
}
impl ToListMappable for DataMemorystoreInstanceDesiredAutoCreatedEndpointsEl {
    type O = BlockAssignable<DataMemorystoreInstanceDesiredAutoCreatedEndpointsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstanceDesiredAutoCreatedEndpointsEl {}
impl BuildDataMemorystoreInstanceDesiredAutoCreatedEndpointsEl {
    pub fn build(self) -> DataMemorystoreInstanceDesiredAutoCreatedEndpointsEl {
        DataMemorystoreInstanceDesiredAutoCreatedEndpointsEl {
            network: core::default::Default::default(),
            project_id: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstanceDesiredAutoCreatedEndpointsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstanceDesiredAutoCreatedEndpointsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataMemorystoreInstanceDesiredAutoCreatedEndpointsElRef {
        DataMemorystoreInstanceDesiredAutoCreatedEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstanceDesiredAutoCreatedEndpointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMemorystoreInstanceDesiredPscAutoConnectionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
}
impl DataMemorystoreInstanceDesiredPscAutoConnectionsEl {
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
}
impl ToListMappable for DataMemorystoreInstanceDesiredPscAutoConnectionsEl {
    type O = BlockAssignable<DataMemorystoreInstanceDesiredPscAutoConnectionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstanceDesiredPscAutoConnectionsEl {}
impl BuildDataMemorystoreInstanceDesiredPscAutoConnectionsEl {
    pub fn build(self) -> DataMemorystoreInstanceDesiredPscAutoConnectionsEl {
        DataMemorystoreInstanceDesiredPscAutoConnectionsEl {
            network: core::default::Default::default(),
            project_id: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstanceDesiredPscAutoConnectionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstanceDesiredPscAutoConnectionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataMemorystoreInstanceDesiredPscAutoConnectionsElRef {
        DataMemorystoreInstanceDesiredPscAutoConnectionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstanceDesiredPscAutoConnectionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMemorystoreInstanceDiscoveryEndpointsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl DataMemorystoreInstanceDiscoveryEndpointsEl {
    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}
impl ToListMappable for DataMemorystoreInstanceDiscoveryEndpointsEl {
    type O = BlockAssignable<DataMemorystoreInstanceDiscoveryEndpointsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstanceDiscoveryEndpointsEl {}
impl BuildDataMemorystoreInstanceDiscoveryEndpointsEl {
    pub fn build(self) -> DataMemorystoreInstanceDiscoveryEndpointsEl {
        DataMemorystoreInstanceDiscoveryEndpointsEl {
            address: core::default::Default::default(),
            network: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstanceDiscoveryEndpointsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstanceDiscoveryEndpointsElRef {
    fn new(shared: StackShared, base: String) -> DataMemorystoreInstanceDiscoveryEndpointsElRef {
        DataMemorystoreInstanceDiscoveryEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstanceDiscoveryEndpointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forwarding_rule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_connection_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_attachment: Option<PrimField<String>>,
}
impl DataMemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionEl {
    #[doc = "Set the field `connection_type`.\n"]
    pub fn set_connection_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connection_type = Some(v.into());
        self
    }
    #[doc = "Set the field `forwarding_rule`.\n"]
    pub fn set_forwarding_rule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.forwarding_rule = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
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
    #[doc = "Set the field `service_attachment`.\n"]
    pub fn set_service_attachment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_attachment = Some(v.into());
        self
    }
}
impl ToListMappable for DataMemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionEl {
    type O = BlockAssignable<DataMemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionEl {}
impl BuildDataMemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionEl {
    pub fn build(self) -> DataMemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionEl {
        DataMemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionEl {
            connection_type: core::default::Default::default(),
            forwarding_rule: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            network: core::default::Default::default(),
            port: core::default::Default::default(),
            project_id: core::default::Default::default(),
            psc_connection_id: core::default::Default::default(),
            service_attachment: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataMemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionElRef {
        DataMemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionElRef {
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
    #[doc = "Get a reference to the value of field `forwarding_rule` after provisioning.\n"]
    pub fn forwarding_rule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.forwarding_rule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
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
    #[doc = "Get a reference to the value of field `service_attachment` after provisioning.\n"]
    pub fn service_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_attachment", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataMemorystoreInstanceEndpointsElConnectionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_auto_connection:
        Option<ListField<DataMemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionEl>>,
}
impl DataMemorystoreInstanceEndpointsElConnectionsEl {
    #[doc = "Set the field `psc_auto_connection`.\n"]
    pub fn set_psc_auto_connection(
        mut self,
        v: impl Into<ListField<DataMemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionEl>>,
    ) -> Self {
        self.psc_auto_connection = Some(v.into());
        self
    }
}
impl ToListMappable for DataMemorystoreInstanceEndpointsElConnectionsEl {
    type O = BlockAssignable<DataMemorystoreInstanceEndpointsElConnectionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstanceEndpointsElConnectionsEl {}
impl BuildDataMemorystoreInstanceEndpointsElConnectionsEl {
    pub fn build(self) -> DataMemorystoreInstanceEndpointsElConnectionsEl {
        DataMemorystoreInstanceEndpointsElConnectionsEl {
            psc_auto_connection: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstanceEndpointsElConnectionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstanceEndpointsElConnectionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataMemorystoreInstanceEndpointsElConnectionsElRef {
        DataMemorystoreInstanceEndpointsElConnectionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstanceEndpointsElConnectionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `psc_auto_connection` after provisioning.\n"]
    pub fn psc_auto_connection(
        &self,
    ) -> ListRef<DataMemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_auto_connection", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataMemorystoreInstanceEndpointsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connections: Option<ListField<DataMemorystoreInstanceEndpointsElConnectionsEl>>,
}
impl DataMemorystoreInstanceEndpointsEl {
    #[doc = "Set the field `connections`.\n"]
    pub fn set_connections(
        mut self,
        v: impl Into<ListField<DataMemorystoreInstanceEndpointsElConnectionsEl>>,
    ) -> Self {
        self.connections = Some(v.into());
        self
    }
}
impl ToListMappable for DataMemorystoreInstanceEndpointsEl {
    type O = BlockAssignable<DataMemorystoreInstanceEndpointsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstanceEndpointsEl {}
impl BuildDataMemorystoreInstanceEndpointsEl {
    pub fn build(self) -> DataMemorystoreInstanceEndpointsEl {
        DataMemorystoreInstanceEndpointsEl {
            connections: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstanceEndpointsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstanceEndpointsElRef {
    fn new(shared: StackShared, base: String) -> DataMemorystoreInstanceEndpointsElRef {
        DataMemorystoreInstanceEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstanceEndpointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connections` after provisioning.\n"]
    pub fn connections(&self) -> ListRef<DataMemorystoreInstanceEndpointsElConnectionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.connections", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMemorystoreInstanceGcsSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    uris: Option<SetField<PrimField<String>>>,
}
impl DataMemorystoreInstanceGcsSourceEl {
    #[doc = "Set the field `uris`.\n"]
    pub fn set_uris(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.uris = Some(v.into());
        self
    }
}
impl ToListMappable for DataMemorystoreInstanceGcsSourceEl {
    type O = BlockAssignable<DataMemorystoreInstanceGcsSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstanceGcsSourceEl {}
impl BuildDataMemorystoreInstanceGcsSourceEl {
    pub fn build(self) -> DataMemorystoreInstanceGcsSourceEl {
        DataMemorystoreInstanceGcsSourceEl {
            uris: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstanceGcsSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstanceGcsSourceElRef {
    fn new(shared: StackShared, base: String) -> DataMemorystoreInstanceGcsSourceElRef {
        DataMemorystoreInstanceGcsSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstanceGcsSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `uris` after provisioning.\n"]
    pub fn uris(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.uris", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
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
impl ToListMappable
    for DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl
{
    type O = BlockAssignable<
        DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {}
impl BuildDataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    pub fn build(
        self,
    ) -> DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
        DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
        DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
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
pub struct DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<
        ListField<DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>,
    >,
}
impl DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
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
        v: impl Into<
            ListField<
                DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl,
            >,
        >,
    ) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    type O = BlockAssignable<DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {}
impl BuildDataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    pub fn build(self) -> DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
        DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
            day: core::default::Default::default(),
            duration: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
        DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
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
    ) -> ListRef<DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMemorystoreInstanceMaintenancePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_maintenance_window:
        Option<ListField<DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>>,
}
impl DataMemorystoreInstanceMaintenancePolicyEl {
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
        v: impl Into<ListField<DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>>,
    ) -> Self {
        self.weekly_maintenance_window = Some(v.into());
        self
    }
}
impl ToListMappable for DataMemorystoreInstanceMaintenancePolicyEl {
    type O = BlockAssignable<DataMemorystoreInstanceMaintenancePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstanceMaintenancePolicyEl {}
impl BuildDataMemorystoreInstanceMaintenancePolicyEl {
    pub fn build(self) -> DataMemorystoreInstanceMaintenancePolicyEl {
        DataMemorystoreInstanceMaintenancePolicyEl {
            create_time: core::default::Default::default(),
            update_time: core::default::Default::default(),
            weekly_maintenance_window: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstanceMaintenancePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstanceMaintenancePolicyElRef {
    fn new(shared: StackShared, base: String) -> DataMemorystoreInstanceMaintenancePolicyElRef {
        DataMemorystoreInstanceMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstanceMaintenancePolicyElRef {
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
    ) -> ListRef<DataMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.weekly_maintenance_window", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataMemorystoreInstanceMaintenanceScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_deadline_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl DataMemorystoreInstanceMaintenanceScheduleEl {
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
impl ToListMappable for DataMemorystoreInstanceMaintenanceScheduleEl {
    type O = BlockAssignable<DataMemorystoreInstanceMaintenanceScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstanceMaintenanceScheduleEl {}
impl BuildDataMemorystoreInstanceMaintenanceScheduleEl {
    pub fn build(self) -> DataMemorystoreInstanceMaintenanceScheduleEl {
        DataMemorystoreInstanceMaintenanceScheduleEl {
            end_time: core::default::Default::default(),
            schedule_deadline_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstanceMaintenanceScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstanceMaintenanceScheduleElRef {
    fn new(shared: StackShared, base: String) -> DataMemorystoreInstanceMaintenanceScheduleElRef {
        DataMemorystoreInstanceMaintenanceScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstanceMaintenanceScheduleElRef {
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
pub struct DataMemorystoreInstanceManagedBackupSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backup: Option<PrimField<String>>,
}
impl DataMemorystoreInstanceManagedBackupSourceEl {
    #[doc = "Set the field `backup`.\n"]
    pub fn set_backup(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backup = Some(v.into());
        self
    }
}
impl ToListMappable for DataMemorystoreInstanceManagedBackupSourceEl {
    type O = BlockAssignable<DataMemorystoreInstanceManagedBackupSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstanceManagedBackupSourceEl {}
impl BuildDataMemorystoreInstanceManagedBackupSourceEl {
    pub fn build(self) -> DataMemorystoreInstanceManagedBackupSourceEl {
        DataMemorystoreInstanceManagedBackupSourceEl {
            backup: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstanceManagedBackupSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstanceManagedBackupSourceElRef {
    fn new(shared: StackShared, base: String) -> DataMemorystoreInstanceManagedBackupSourceElRef {
        DataMemorystoreInstanceManagedBackupSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstanceManagedBackupSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup` after provisioning.\n"]
    pub fn backup(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMemorystoreInstanceManagedServerCaElCaCertsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificates: Option<ListField<PrimField<String>>>,
}
impl DataMemorystoreInstanceManagedServerCaElCaCertsEl {
    #[doc = "Set the field `certificates`.\n"]
    pub fn set_certificates(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.certificates = Some(v.into());
        self
    }
}
impl ToListMappable for DataMemorystoreInstanceManagedServerCaElCaCertsEl {
    type O = BlockAssignable<DataMemorystoreInstanceManagedServerCaElCaCertsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstanceManagedServerCaElCaCertsEl {}
impl BuildDataMemorystoreInstanceManagedServerCaElCaCertsEl {
    pub fn build(self) -> DataMemorystoreInstanceManagedServerCaElCaCertsEl {
        DataMemorystoreInstanceManagedServerCaElCaCertsEl {
            certificates: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstanceManagedServerCaElCaCertsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstanceManagedServerCaElCaCertsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataMemorystoreInstanceManagedServerCaElCaCertsElRef {
        DataMemorystoreInstanceManagedServerCaElCaCertsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstanceManagedServerCaElCaCertsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificates` after provisioning.\n"]
    pub fn certificates(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.certificates", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMemorystoreInstanceManagedServerCaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_certs: Option<ListField<DataMemorystoreInstanceManagedServerCaElCaCertsEl>>,
}
impl DataMemorystoreInstanceManagedServerCaEl {
    #[doc = "Set the field `ca_certs`.\n"]
    pub fn set_ca_certs(
        mut self,
        v: impl Into<ListField<DataMemorystoreInstanceManagedServerCaElCaCertsEl>>,
    ) -> Self {
        self.ca_certs = Some(v.into());
        self
    }
}
impl ToListMappable for DataMemorystoreInstanceManagedServerCaEl {
    type O = BlockAssignable<DataMemorystoreInstanceManagedServerCaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstanceManagedServerCaEl {}
impl BuildDataMemorystoreInstanceManagedServerCaEl {
    pub fn build(self) -> DataMemorystoreInstanceManagedServerCaEl {
        DataMemorystoreInstanceManagedServerCaEl {
            ca_certs: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstanceManagedServerCaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstanceManagedServerCaElRef {
    fn new(shared: StackShared, base: String) -> DataMemorystoreInstanceManagedServerCaElRef {
        DataMemorystoreInstanceManagedServerCaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstanceManagedServerCaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ca_certs` after provisioning.\n"]
    pub fn ca_certs(&self) -> ListRef<DataMemorystoreInstanceManagedServerCaElCaCertsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ca_certs", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMemorystoreInstanceNodeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    size_gb: Option<PrimField<f64>>,
}
impl DataMemorystoreInstanceNodeConfigEl {
    #[doc = "Set the field `size_gb`.\n"]
    pub fn set_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size_gb = Some(v.into());
        self
    }
}
impl ToListMappable for DataMemorystoreInstanceNodeConfigEl {
    type O = BlockAssignable<DataMemorystoreInstanceNodeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstanceNodeConfigEl {}
impl BuildDataMemorystoreInstanceNodeConfigEl {
    pub fn build(self) -> DataMemorystoreInstanceNodeConfigEl {
        DataMemorystoreInstanceNodeConfigEl {
            size_gb: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstanceNodeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstanceNodeConfigElRef {
    fn new(shared: StackShared, base: String) -> DataMemorystoreInstanceNodeConfigElRef {
        DataMemorystoreInstanceNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstanceNodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `size_gb` after provisioning.\n"]
    pub fn size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_gb", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMemorystoreInstancePersistenceConfigElAofConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    append_fsync: Option<PrimField<String>>,
}
impl DataMemorystoreInstancePersistenceConfigElAofConfigEl {
    #[doc = "Set the field `append_fsync`.\n"]
    pub fn set_append_fsync(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.append_fsync = Some(v.into());
        self
    }
}
impl ToListMappable for DataMemorystoreInstancePersistenceConfigElAofConfigEl {
    type O = BlockAssignable<DataMemorystoreInstancePersistenceConfigElAofConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstancePersistenceConfigElAofConfigEl {}
impl BuildDataMemorystoreInstancePersistenceConfigElAofConfigEl {
    pub fn build(self) -> DataMemorystoreInstancePersistenceConfigElAofConfigEl {
        DataMemorystoreInstancePersistenceConfigElAofConfigEl {
            append_fsync: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstancePersistenceConfigElAofConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstancePersistenceConfigElAofConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataMemorystoreInstancePersistenceConfigElAofConfigElRef {
        DataMemorystoreInstancePersistenceConfigElAofConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstancePersistenceConfigElAofConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `append_fsync` after provisioning.\n"]
    pub fn append_fsync(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.append_fsync", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMemorystoreInstancePersistenceConfigElRdbConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rdb_snapshot_period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rdb_snapshot_start_time: Option<PrimField<String>>,
}
impl DataMemorystoreInstancePersistenceConfigElRdbConfigEl {
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
impl ToListMappable for DataMemorystoreInstancePersistenceConfigElRdbConfigEl {
    type O = BlockAssignable<DataMemorystoreInstancePersistenceConfigElRdbConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstancePersistenceConfigElRdbConfigEl {}
impl BuildDataMemorystoreInstancePersistenceConfigElRdbConfigEl {
    pub fn build(self) -> DataMemorystoreInstancePersistenceConfigElRdbConfigEl {
        DataMemorystoreInstancePersistenceConfigElRdbConfigEl {
            rdb_snapshot_period: core::default::Default::default(),
            rdb_snapshot_start_time: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstancePersistenceConfigElRdbConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstancePersistenceConfigElRdbConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataMemorystoreInstancePersistenceConfigElRdbConfigElRef {
        DataMemorystoreInstancePersistenceConfigElRdbConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstancePersistenceConfigElRdbConfigElRef {
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
pub struct DataMemorystoreInstancePersistenceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aof_config: Option<ListField<DataMemorystoreInstancePersistenceConfigElAofConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rdb_config: Option<ListField<DataMemorystoreInstancePersistenceConfigElRdbConfigEl>>,
}
impl DataMemorystoreInstancePersistenceConfigEl {
    #[doc = "Set the field `aof_config`.\n"]
    pub fn set_aof_config(
        mut self,
        v: impl Into<ListField<DataMemorystoreInstancePersistenceConfigElAofConfigEl>>,
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
        v: impl Into<ListField<DataMemorystoreInstancePersistenceConfigElRdbConfigEl>>,
    ) -> Self {
        self.rdb_config = Some(v.into());
        self
    }
}
impl ToListMappable for DataMemorystoreInstancePersistenceConfigEl {
    type O = BlockAssignable<DataMemorystoreInstancePersistenceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstancePersistenceConfigEl {}
impl BuildDataMemorystoreInstancePersistenceConfigEl {
    pub fn build(self) -> DataMemorystoreInstancePersistenceConfigEl {
        DataMemorystoreInstancePersistenceConfigEl {
            aof_config: core::default::Default::default(),
            mode: core::default::Default::default(),
            rdb_config: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstancePersistenceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstancePersistenceConfigElRef {
    fn new(shared: StackShared, base: String) -> DataMemorystoreInstancePersistenceConfigElRef {
        DataMemorystoreInstancePersistenceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstancePersistenceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aof_config` after provisioning.\n"]
    pub fn aof_config(&self) -> ListRef<DataMemorystoreInstancePersistenceConfigElAofConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.aof_config", self.base))
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `rdb_config` after provisioning.\n"]
    pub fn rdb_config(&self) -> ListRef<DataMemorystoreInstancePersistenceConfigElRdbConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rdb_config", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMemorystoreInstancePscAttachmentDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_attachment: Option<PrimField<String>>,
}
impl DataMemorystoreInstancePscAttachmentDetailsEl {
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
impl ToListMappable for DataMemorystoreInstancePscAttachmentDetailsEl {
    type O = BlockAssignable<DataMemorystoreInstancePscAttachmentDetailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstancePscAttachmentDetailsEl {}
impl BuildDataMemorystoreInstancePscAttachmentDetailsEl {
    pub fn build(self) -> DataMemorystoreInstancePscAttachmentDetailsEl {
        DataMemorystoreInstancePscAttachmentDetailsEl {
            connection_type: core::default::Default::default(),
            service_attachment: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstancePscAttachmentDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstancePscAttachmentDetailsElRef {
    fn new(shared: StackShared, base: String) -> DataMemorystoreInstancePscAttachmentDetailsElRef {
        DataMemorystoreInstancePscAttachmentDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstancePscAttachmentDetailsElRef {
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
pub struct DataMemorystoreInstancePscAutoConnectionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forwarding_rule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_connection_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_connection_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_attachment: Option<PrimField<String>>,
}
impl DataMemorystoreInstancePscAutoConnectionsEl {
    #[doc = "Set the field `connection_type`.\n"]
    pub fn set_connection_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connection_type = Some(v.into());
        self
    }
    #[doc = "Set the field `forwarding_rule`.\n"]
    pub fn set_forwarding_rule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.forwarding_rule = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
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
    #[doc = "Set the field `psc_connection_status`.\n"]
    pub fn set_psc_connection_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.psc_connection_status = Some(v.into());
        self
    }
    #[doc = "Set the field `service_attachment`.\n"]
    pub fn set_service_attachment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_attachment = Some(v.into());
        self
    }
}
impl ToListMappable for DataMemorystoreInstancePscAutoConnectionsEl {
    type O = BlockAssignable<DataMemorystoreInstancePscAutoConnectionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstancePscAutoConnectionsEl {}
impl BuildDataMemorystoreInstancePscAutoConnectionsEl {
    pub fn build(self) -> DataMemorystoreInstancePscAutoConnectionsEl {
        DataMemorystoreInstancePscAutoConnectionsEl {
            connection_type: core::default::Default::default(),
            forwarding_rule: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            network: core::default::Default::default(),
            port: core::default::Default::default(),
            project_id: core::default::Default::default(),
            psc_connection_id: core::default::Default::default(),
            psc_connection_status: core::default::Default::default(),
            service_attachment: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstancePscAutoConnectionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstancePscAutoConnectionsElRef {
    fn new(shared: StackShared, base: String) -> DataMemorystoreInstancePscAutoConnectionsElRef {
        DataMemorystoreInstancePscAutoConnectionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstancePscAutoConnectionsElRef {
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
    #[doc = "Get a reference to the value of field `forwarding_rule` after provisioning.\n"]
    pub fn forwarding_rule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.forwarding_rule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
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
    #[doc = "Get a reference to the value of field `psc_connection_status` after provisioning.\n"]
    pub fn psc_connection_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.psc_connection_status", self.base),
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
pub struct DataMemorystoreInstanceStateInfoElUpdateInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target_engine_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_node_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_replica_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_shard_count: Option<PrimField<f64>>,
}
impl DataMemorystoreInstanceStateInfoElUpdateInfoEl {
    #[doc = "Set the field `target_engine_version`.\n"]
    pub fn set_target_engine_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_engine_version = Some(v.into());
        self
    }
    #[doc = "Set the field `target_node_type`.\n"]
    pub fn set_target_node_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_node_type = Some(v.into());
        self
    }
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
impl ToListMappable for DataMemorystoreInstanceStateInfoElUpdateInfoEl {
    type O = BlockAssignable<DataMemorystoreInstanceStateInfoElUpdateInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstanceStateInfoElUpdateInfoEl {}
impl BuildDataMemorystoreInstanceStateInfoElUpdateInfoEl {
    pub fn build(self) -> DataMemorystoreInstanceStateInfoElUpdateInfoEl {
        DataMemorystoreInstanceStateInfoElUpdateInfoEl {
            target_engine_version: core::default::Default::default(),
            target_node_type: core::default::Default::default(),
            target_replica_count: core::default::Default::default(),
            target_shard_count: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstanceStateInfoElUpdateInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstanceStateInfoElUpdateInfoElRef {
    fn new(shared: StackShared, base: String) -> DataMemorystoreInstanceStateInfoElUpdateInfoElRef {
        DataMemorystoreInstanceStateInfoElUpdateInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstanceStateInfoElUpdateInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target_engine_version` after provisioning.\n"]
    pub fn target_engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_engine_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_node_type` after provisioning.\n"]
    pub fn target_node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_node_type", self.base),
        )
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
pub struct DataMemorystoreInstanceStateInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    update_info: Option<ListField<DataMemorystoreInstanceStateInfoElUpdateInfoEl>>,
}
impl DataMemorystoreInstanceStateInfoEl {
    #[doc = "Set the field `update_info`.\n"]
    pub fn set_update_info(
        mut self,
        v: impl Into<ListField<DataMemorystoreInstanceStateInfoElUpdateInfoEl>>,
    ) -> Self {
        self.update_info = Some(v.into());
        self
    }
}
impl ToListMappable for DataMemorystoreInstanceStateInfoEl {
    type O = BlockAssignable<DataMemorystoreInstanceStateInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstanceStateInfoEl {}
impl BuildDataMemorystoreInstanceStateInfoEl {
    pub fn build(self) -> DataMemorystoreInstanceStateInfoEl {
        DataMemorystoreInstanceStateInfoEl {
            update_info: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstanceStateInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstanceStateInfoElRef {
    fn new(shared: StackShared, base: String) -> DataMemorystoreInstanceStateInfoElRef {
        DataMemorystoreInstanceStateInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstanceStateInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `update_info` after provisioning.\n"]
    pub fn update_info(&self) -> ListRef<DataMemorystoreInstanceStateInfoElUpdateInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.update_info", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMemorystoreInstanceZoneDistributionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}
impl DataMemorystoreInstanceZoneDistributionConfigEl {
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
impl ToListMappable for DataMemorystoreInstanceZoneDistributionConfigEl {
    type O = BlockAssignable<DataMemorystoreInstanceZoneDistributionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemorystoreInstanceZoneDistributionConfigEl {}
impl BuildDataMemorystoreInstanceZoneDistributionConfigEl {
    pub fn build(self) -> DataMemorystoreInstanceZoneDistributionConfigEl {
        DataMemorystoreInstanceZoneDistributionConfigEl {
            mode: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}
pub struct DataMemorystoreInstanceZoneDistributionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemorystoreInstanceZoneDistributionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataMemorystoreInstanceZoneDistributionConfigElRef {
        DataMemorystoreInstanceZoneDistributionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemorystoreInstanceZoneDistributionConfigElRef {
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
