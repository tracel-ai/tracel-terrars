use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataprocMetastoreServiceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    release_channel: Option<PrimField<String>>,
    service_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_config: Option<Vec<DataprocMetastoreServiceEncryptionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hive_metastore_config: Option<Vec<DataprocMetastoreServiceHiveMetastoreConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_window: Option<Vec<DataprocMetastoreServiceMaintenanceWindowEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_integration: Option<Vec<DataprocMetastoreServiceMetadataIntegrationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_config: Option<Vec<DataprocMetastoreServiceNetworkConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scaling_config: Option<Vec<DataprocMetastoreServiceScalingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduled_backup: Option<Vec<DataprocMetastoreServiceScheduledBackupEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    telemetry_config: Option<Vec<DataprocMetastoreServiceTelemetryConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataprocMetastoreServiceTimeoutsEl>,
    dynamic: DataprocMetastoreServiceDynamic,
}
struct DataprocMetastoreService_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataprocMetastoreServiceData>,
}
#[derive(Clone)]
pub struct DataprocMetastoreService(Rc<DataprocMetastoreService_>);
impl DataprocMetastoreService {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }
    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }
    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }
    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => true,
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    }
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }
    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(r.extract_ref());
        self
    }
    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(attr.to_string());
        self
    }
    #[doc = "Set the field `database_type`.\nThe database type that the Metastore service stores its data. Default value: \"MYSQL\" Possible values: [\"MYSQL\", \"SPANNER\"]"]
    pub fn set_database_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().database_type = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_protection`.\nIndicates if the dataproc metastore should be protected against accidental deletions."]
    pub fn set_deletion_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nUser-defined labels for the metastore service.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe location where the metastore service should reside.\nThe default value is 'global'."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nThe relative resource name of the VPC network on which the instance can be accessed. It is specified in the following form:\n\n\"projects/{projectNumber}/global/networks/{network_id}\"."]
    pub fn set_network(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nThe TCP port at which the metastore service is reached. Default: 9083."]
    pub fn set_port(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `release_channel`.\nThe release channel of the service. If unspecified, defaults to 'STABLE'. Default value: \"STABLE\" Possible values: [\"CANARY\", \"STABLE\"]"]
    pub fn set_release_channel(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().release_channel = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\nA map of resource manager tags.\nResource manager tag keys and values have the same definition as resource manager tags.\nKeys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/{tag_value_id}."]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `tier`.\nThe tier of the service. Possible values: [\"DEVELOPER\", \"ENTERPRISE\"]"]
    pub fn set_tier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tier = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        self,
        v: impl Into<BlockAssignable<DataprocMetastoreServiceEncryptionConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `hive_metastore_config`.\n"]
    pub fn set_hive_metastore_config(
        self,
        v: impl Into<BlockAssignable<DataprocMetastoreServiceHiveMetastoreConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().hive_metastore_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.hive_metastore_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(
        self,
        v: impl Into<BlockAssignable<DataprocMetastoreServiceMaintenanceWindowEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_window = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_window = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `metadata_integration`.\n"]
    pub fn set_metadata_integration(
        self,
        v: impl Into<BlockAssignable<DataprocMetastoreServiceMetadataIntegrationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metadata_integration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metadata_integration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_config`.\n"]
    pub fn set_network_config(
        self,
        v: impl Into<BlockAssignable<DataprocMetastoreServiceNetworkConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `scaling_config`.\n"]
    pub fn set_scaling_config(
        self,
        v: impl Into<BlockAssignable<DataprocMetastoreServiceScalingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scaling_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scaling_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `scheduled_backup`.\n"]
    pub fn set_scheduled_backup(
        self,
        v: impl Into<BlockAssignable<DataprocMetastoreServiceScheduledBackupEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scheduled_backup = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scheduled_backup = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `telemetry_config`.\n"]
    pub fn set_telemetry_config(
        self,
        v: impl Into<BlockAssignable<DataprocMetastoreServiceTelemetryConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().telemetry_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.telemetry_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataprocMetastoreServiceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `artifact_gcs_uri` after provisioning.\nA Cloud Storage URI (starting with gs://) that specifies where artifacts related to the metastore service are stored."]
    pub fn artifact_gcs_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.artifact_gcs_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time when the metastore service was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_type` after provisioning.\nThe database type that the Metastore service stores its data. Default value: \"MYSQL\" Possible values: [\"MYSQL\", \"SPANNER\"]"]
    pub fn database_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nIndicates if the dataproc metastore should be protected against accidental deletions."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_uri` after provisioning.\nThe URI of the endpoint used to access the metastore service."]
    pub fn endpoint_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the metastore service.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location where the metastore service should reside.\nThe default value is 'global'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe relative resource name of the metastore service."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe relative resource name of the VPC network on which the instance can be accessed. It is specified in the following form:\n\n\"projects/{projectNumber}/global/networks/{network_id}\"."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe TCP port at which the metastore service is reached. Default: 9083."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `release_channel` after provisioning.\nThe release channel of the service. If unspecified, defaults to 'STABLE'. Default value: \"STABLE\" Possible values: [\"CANARY\", \"STABLE\"]"]
    pub fn release_channel(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.release_channel", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_id` after provisioning.\nThe ID of the metastore service. The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_),\nand hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between\n3 and 63 characters."]
    pub fn service_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the metastore service."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_message` after provisioning.\nAdditional information about the current state of the metastore service, if available."]
    pub fn state_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_message", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nA map of resource manager tags.\nResource manager tag keys and values have the same definition as resource manager tags.\nKeys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/{tag_value_id}."]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tier` after provisioning.\nThe tier of the service. Possible values: [\"DEVELOPER\", \"ENTERPRISE\"]"]
    pub fn tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe globally unique resource identifier of the metastore service."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time when the metastore service was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> ListRef<DataprocMetastoreServiceEncryptionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hive_metastore_config` after provisioning.\n"]
    pub fn hive_metastore_config(
        &self,
    ) -> ListRef<DataprocMetastoreServiceHiveMetastoreConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hive_metastore_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> ListRef<DataprocMetastoreServiceMaintenanceWindowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_window", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata_integration` after provisioning.\n"]
    pub fn metadata_integration(
        &self,
    ) -> ListRef<DataprocMetastoreServiceMetadataIntegrationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata_integration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<DataprocMetastoreServiceNetworkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scaling_config` after provisioning.\n"]
    pub fn scaling_config(&self) -> ListRef<DataprocMetastoreServiceScalingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scaling_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scheduled_backup` after provisioning.\n"]
    pub fn scheduled_backup(&self) -> ListRef<DataprocMetastoreServiceScheduledBackupElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scheduled_backup", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `telemetry_config` after provisioning.\n"]
    pub fn telemetry_config(&self) -> ListRef<DataprocMetastoreServiceTelemetryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.telemetry_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocMetastoreServiceTimeoutsElRef {
        DataprocMetastoreServiceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataprocMetastoreService {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DataprocMetastoreService {}
impl ToListMappable for DataprocMetastoreService {
    type O = ListRef<DataprocMetastoreServiceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DataprocMetastoreService_ {
    fn extract_resource_type(&self) -> String {
        "google_dataproc_metastore_service".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataprocMetastoreService {
    pub tf_id: String,
    #[doc = "The ID of the metastore service. The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_),\nand hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between\n3 and 63 characters."]
    pub service_id: PrimField<String>,
}
impl BuildDataprocMetastoreService {
    pub fn build(self, stack: &mut Stack) -> DataprocMetastoreService {
        let out = DataprocMetastoreService(Rc::new(DataprocMetastoreService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataprocMetastoreServiceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                database_type: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                network: core::default::Default::default(),
                port: core::default::Default::default(),
                project: core::default::Default::default(),
                release_channel: core::default::Default::default(),
                service_id: self.service_id,
                tags: core::default::Default::default(),
                tier: core::default::Default::default(),
                encryption_config: core::default::Default::default(),
                hive_metastore_config: core::default::Default::default(),
                maintenance_window: core::default::Default::default(),
                metadata_integration: core::default::Default::default(),
                network_config: core::default::Default::default(),
                scaling_config: core::default::Default::default(),
                scheduled_backup: core::default::Default::default(),
                telemetry_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DataprocMetastoreServiceRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocMetastoreServiceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataprocMetastoreServiceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `artifact_gcs_uri` after provisioning.\nA Cloud Storage URI (starting with gs://) that specifies where artifacts related to the metastore service are stored."]
    pub fn artifact_gcs_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.artifact_gcs_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time when the metastore service was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_type` after provisioning.\nThe database type that the Metastore service stores its data. Default value: \"MYSQL\" Possible values: [\"MYSQL\", \"SPANNER\"]"]
    pub fn database_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nIndicates if the dataproc metastore should be protected against accidental deletions."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_uri` after provisioning.\nThe URI of the endpoint used to access the metastore service."]
    pub fn endpoint_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the metastore service.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location where the metastore service should reside.\nThe default value is 'global'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe relative resource name of the metastore service."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe relative resource name of the VPC network on which the instance can be accessed. It is specified in the following form:\n\n\"projects/{projectNumber}/global/networks/{network_id}\"."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe TCP port at which the metastore service is reached. Default: 9083."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `release_channel` after provisioning.\nThe release channel of the service. If unspecified, defaults to 'STABLE'. Default value: \"STABLE\" Possible values: [\"CANARY\", \"STABLE\"]"]
    pub fn release_channel(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.release_channel", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_id` after provisioning.\nThe ID of the metastore service. The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_),\nand hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between\n3 and 63 characters."]
    pub fn service_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the metastore service."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_message` after provisioning.\nAdditional information about the current state of the metastore service, if available."]
    pub fn state_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_message", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nA map of resource manager tags.\nResource manager tag keys and values have the same definition as resource manager tags.\nKeys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/{tag_value_id}."]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tier` after provisioning.\nThe tier of the service. Possible values: [\"DEVELOPER\", \"ENTERPRISE\"]"]
    pub fn tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe globally unique resource identifier of the metastore service."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time when the metastore service was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> ListRef<DataprocMetastoreServiceEncryptionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hive_metastore_config` after provisioning.\n"]
    pub fn hive_metastore_config(
        &self,
    ) -> ListRef<DataprocMetastoreServiceHiveMetastoreConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hive_metastore_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> ListRef<DataprocMetastoreServiceMaintenanceWindowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_window", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata_integration` after provisioning.\n"]
    pub fn metadata_integration(
        &self,
    ) -> ListRef<DataprocMetastoreServiceMetadataIntegrationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata_integration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<DataprocMetastoreServiceNetworkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scaling_config` after provisioning.\n"]
    pub fn scaling_config(&self) -> ListRef<DataprocMetastoreServiceScalingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scaling_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scheduled_backup` after provisioning.\n"]
    pub fn scheduled_backup(&self) -> ListRef<DataprocMetastoreServiceScheduledBackupElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scheduled_backup", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `telemetry_config` after provisioning.\n"]
    pub fn telemetry_config(&self) -> ListRef<DataprocMetastoreServiceTelemetryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.telemetry_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocMetastoreServiceTimeoutsElRef {
        DataprocMetastoreServiceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocMetastoreServiceEncryptionConfigEl {
    kms_key: PrimField<String>,
}
impl DataprocMetastoreServiceEncryptionConfigEl {}
impl ToListMappable for DataprocMetastoreServiceEncryptionConfigEl {
    type O = BlockAssignable<DataprocMetastoreServiceEncryptionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocMetastoreServiceEncryptionConfigEl {
    #[doc = "The fully qualified customer provided Cloud KMS key name to use for customer data encryption.\nUse the following format: 'projects/([^/]+)/locations/([^/]+)/keyRings/([^/]+)/cryptoKeys/([^/]+)'"]
    pub kms_key: PrimField<String>,
}
impl BuildDataprocMetastoreServiceEncryptionConfigEl {
    pub fn build(self) -> DataprocMetastoreServiceEncryptionConfigEl {
        DataprocMetastoreServiceEncryptionConfigEl {
            kms_key: self.kms_key,
        }
    }
}
pub struct DataprocMetastoreServiceEncryptionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocMetastoreServiceEncryptionConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocMetastoreServiceEncryptionConfigElRef {
        DataprocMetastoreServiceEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocMetastoreServiceEncryptionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nThe fully qualified customer provided Cloud KMS key name to use for customer data encryption.\nUse the following format: 'projects/([^/]+)/locations/([^/]+)/keyRings/([^/]+)/cryptoKeys/([^/]+)'"]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
}
#[derive(Serialize)]
pub struct DataprocMetastoreServiceHiveMetastoreConfigElAuxiliaryVersionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    config_overrides: Option<RecField<PrimField<String>>>,
    key: PrimField<String>,
    version: PrimField<String>,
}
impl DataprocMetastoreServiceHiveMetastoreConfigElAuxiliaryVersionsEl {
    #[doc = "Set the field `config_overrides`.\nA mapping of Hive metastore configuration key-value pairs to apply to the auxiliary Hive metastore (configured in hive-site.xml) in addition to the primary version's overrides.\nIf keys are present in both the auxiliary version's overrides and the primary version's overrides, the value from the auxiliary version's overrides takes precedence."]
    pub fn set_config_overrides(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.config_overrides = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocMetastoreServiceHiveMetastoreConfigElAuxiliaryVersionsEl {
    type O = BlockAssignable<DataprocMetastoreServiceHiveMetastoreConfigElAuxiliaryVersionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocMetastoreServiceHiveMetastoreConfigElAuxiliaryVersionsEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = "The Hive metastore version of the auxiliary service. It must be less than the primary Hive metastore service's version."]
    pub version: PrimField<String>,
}
impl BuildDataprocMetastoreServiceHiveMetastoreConfigElAuxiliaryVersionsEl {
    pub fn build(self) -> DataprocMetastoreServiceHiveMetastoreConfigElAuxiliaryVersionsEl {
        DataprocMetastoreServiceHiveMetastoreConfigElAuxiliaryVersionsEl {
            config_overrides: core::default::Default::default(),
            key: self.key,
            version: self.version,
        }
    }
}
pub struct DataprocMetastoreServiceHiveMetastoreConfigElAuxiliaryVersionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocMetastoreServiceHiveMetastoreConfigElAuxiliaryVersionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocMetastoreServiceHiveMetastoreConfigElAuxiliaryVersionsElRef {
        DataprocMetastoreServiceHiveMetastoreConfigElAuxiliaryVersionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocMetastoreServiceHiveMetastoreConfigElAuxiliaryVersionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `config_overrides` after provisioning.\nA mapping of Hive metastore configuration key-value pairs to apply to the auxiliary Hive metastore (configured in hive-site.xml) in addition to the primary version's overrides.\nIf keys are present in both the auxiliary version's overrides and the primary version's overrides, the value from the auxiliary version's overrides takes precedence."]
    pub fn config_overrides(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.config_overrides", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe Hive metastore version of the auxiliary service. It must be less than the primary Hive metastore service's version."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize)]
pub struct DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl {
    cloud_secret: PrimField<String>,
}
impl DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl {}
impl ToListMappable for DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl {
    type O = BlockAssignable<DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl {
    #[doc = "The relative resource name of a Secret Manager secret version, in the following form:\n\n\"projects/{projectNumber}/secrets/{secret_id}/versions/{version_id}\"."]
    pub cloud_secret: PrimField<String>,
}
impl BuildDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl {
    pub fn build(self) -> DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl {
        DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl {
            cloud_secret: self.cloud_secret,
        }
    }
}
pub struct DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabElRef {
        DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_secret` after provisioning.\nThe relative resource name of a Secret Manager secret version, in the following form:\n\n\"projects/{projectNumber}/secrets/{secret_id}/versions/{version_id}\"."]
    pub fn cloud_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloud_secret", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElDynamic {
    keytab:
        Option<DynamicBlock<DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl>>,
}
#[derive(Serialize)]
pub struct DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl {
    krb5_config_gcs_uri: PrimField<String>,
    principal: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keytab: Option<Vec<DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl>>,
    dynamic: DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElDynamic,
}
impl DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl {
    #[doc = "Set the field `keytab`.\n"]
    pub fn set_keytab(
        mut self,
        v: impl Into<
            BlockAssignable<DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.keytab = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.keytab = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl {
    type O = BlockAssignable<DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl {
    #[doc = "A Cloud Storage URI that specifies the path to a krb5.conf file. It is of the form gs://{bucket_name}/path/to/krb5.conf, although the file does not need to be named krb5.conf explicitly."]
    pub krb5_config_gcs_uri: PrimField<String>,
    #[doc = "A Kerberos principal that exists in the both the keytab the KDC to authenticate as. A typical principal is of the form \"primary/instance@REALM\", but there is no exact format."]
    pub principal: PrimField<String>,
}
impl BuildDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl {
    pub fn build(self) -> DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl {
        DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl {
            krb5_config_gcs_uri: self.krb5_config_gcs_uri,
            principal: self.principal,
            keytab: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElRef {
        DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `krb5_config_gcs_uri` after provisioning.\nA Cloud Storage URI that specifies the path to a krb5.conf file. It is of the form gs://{bucket_name}/path/to/krb5.conf, although the file does not need to be named krb5.conf explicitly."]
    pub fn krb5_config_gcs_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.krb5_config_gcs_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `principal` after provisioning.\nA Kerberos principal that exists in the both the keytab the KDC to authenticate as. A typical principal is of the form \"primary/instance@REALM\", but there is no exact format."]
    pub fn principal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.principal", self.base))
    }
    #[doc = "Get a reference to the value of field `keytab` after provisioning.\n"]
    pub fn keytab(
        &self,
    ) -> ListRef<DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabElRef> {
        ListRef::new(self.shared().clone(), format!("{}.keytab", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataprocMetastoreServiceHiveMetastoreConfigElDynamic {
    auxiliary_versions:
        Option<DynamicBlock<DataprocMetastoreServiceHiveMetastoreConfigElAuxiliaryVersionsEl>>,
    kerberos_config:
        Option<DynamicBlock<DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl>>,
}
#[derive(Serialize)]
pub struct DataprocMetastoreServiceHiveMetastoreConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    config_overrides: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_protocol: Option<PrimField<String>>,
    version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auxiliary_versions:
        Option<Vec<DataprocMetastoreServiceHiveMetastoreConfigElAuxiliaryVersionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kerberos_config: Option<Vec<DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl>>,
    dynamic: DataprocMetastoreServiceHiveMetastoreConfigElDynamic,
}
impl DataprocMetastoreServiceHiveMetastoreConfigEl {
    #[doc = "Set the field `config_overrides`.\nA mapping of Hive metastore configuration key-value pairs to apply to the Hive metastore (configured in hive-site.xml).\nThe mappings override system defaults (some keys cannot be overridden)"]
    pub fn set_config_overrides(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.config_overrides = Some(v.into());
        self
    }
    #[doc = "Set the field `endpoint_protocol`.\nThe protocol to use for the metastore service endpoint. If unspecified, defaults to 'THRIFT'. Default value: \"THRIFT\" Possible values: [\"THRIFT\", \"GRPC\"]"]
    pub fn set_endpoint_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint_protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `auxiliary_versions`.\n"]
    pub fn set_auxiliary_versions(
        mut self,
        v: impl Into<BlockAssignable<DataprocMetastoreServiceHiveMetastoreConfigElAuxiliaryVersionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.auxiliary_versions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.auxiliary_versions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `kerberos_config`.\n"]
    pub fn set_kerberos_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kerberos_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kerberos_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocMetastoreServiceHiveMetastoreConfigEl {
    type O = BlockAssignable<DataprocMetastoreServiceHiveMetastoreConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocMetastoreServiceHiveMetastoreConfigEl {
    #[doc = "The Hive metastore schema version."]
    pub version: PrimField<String>,
}
impl BuildDataprocMetastoreServiceHiveMetastoreConfigEl {
    pub fn build(self) -> DataprocMetastoreServiceHiveMetastoreConfigEl {
        DataprocMetastoreServiceHiveMetastoreConfigEl {
            config_overrides: core::default::Default::default(),
            endpoint_protocol: core::default::Default::default(),
            version: self.version,
            auxiliary_versions: core::default::Default::default(),
            kerberos_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocMetastoreServiceHiveMetastoreConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocMetastoreServiceHiveMetastoreConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocMetastoreServiceHiveMetastoreConfigElRef {
        DataprocMetastoreServiceHiveMetastoreConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocMetastoreServiceHiveMetastoreConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `config_overrides` after provisioning.\nA mapping of Hive metastore configuration key-value pairs to apply to the Hive metastore (configured in hive-site.xml).\nThe mappings override system defaults (some keys cannot be overridden)"]
    pub fn config_overrides(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.config_overrides", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_protocol` after provisioning.\nThe protocol to use for the metastore service endpoint. If unspecified, defaults to 'THRIFT'. Default value: \"THRIFT\" Possible values: [\"THRIFT\", \"GRPC\"]"]
    pub fn endpoint_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_protocol", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe Hive metastore schema version."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
    #[doc = "Get a reference to the value of field `kerberos_config` after provisioning.\n"]
    pub fn kerberos_config(
        &self,
    ) -> ListRef<DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kerberos_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocMetastoreServiceMaintenanceWindowEl {
    day_of_week: PrimField<String>,
    hour_of_day: PrimField<f64>,
}
impl DataprocMetastoreServiceMaintenanceWindowEl {}
impl ToListMappable for DataprocMetastoreServiceMaintenanceWindowEl {
    type O = BlockAssignable<DataprocMetastoreServiceMaintenanceWindowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocMetastoreServiceMaintenanceWindowEl {
    #[doc = "The day of week, when the window starts. Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub day_of_week: PrimField<String>,
    #[doc = "The hour of day (0-23) when the window starts."]
    pub hour_of_day: PrimField<f64>,
}
impl BuildDataprocMetastoreServiceMaintenanceWindowEl {
    pub fn build(self) -> DataprocMetastoreServiceMaintenanceWindowEl {
        DataprocMetastoreServiceMaintenanceWindowEl {
            day_of_week: self.day_of_week,
            hour_of_day: self.hour_of_day,
        }
    }
}
pub struct DataprocMetastoreServiceMaintenanceWindowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocMetastoreServiceMaintenanceWindowElRef {
    fn new(shared: StackShared, base: String) -> DataprocMetastoreServiceMaintenanceWindowElRef {
        DataprocMetastoreServiceMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocMetastoreServiceMaintenanceWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day_of_week` after provisioning.\nThe day of week, when the window starts. Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn day_of_week(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }
    #[doc = "Get a reference to the value of field `hour_of_day` after provisioning.\nThe hour of day (0-23) when the window starts."]
    pub fn hour_of_day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hour_of_day", self.base))
    }
}
#[derive(Serialize)]
pub struct DataprocMetastoreServiceMetadataIntegrationElDataCatalogConfigEl {
    enabled: PrimField<bool>,
}
impl DataprocMetastoreServiceMetadataIntegrationElDataCatalogConfigEl {}
impl ToListMappable for DataprocMetastoreServiceMetadataIntegrationElDataCatalogConfigEl {
    type O = BlockAssignable<DataprocMetastoreServiceMetadataIntegrationElDataCatalogConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocMetastoreServiceMetadataIntegrationElDataCatalogConfigEl {
    #[doc = "Defines whether the metastore metadata should be synced to Data Catalog. The default value is to disable syncing metastore metadata to Data Catalog."]
    pub enabled: PrimField<bool>,
}
impl BuildDataprocMetastoreServiceMetadataIntegrationElDataCatalogConfigEl {
    pub fn build(self) -> DataprocMetastoreServiceMetadataIntegrationElDataCatalogConfigEl {
        DataprocMetastoreServiceMetadataIntegrationElDataCatalogConfigEl {
            enabled: self.enabled,
        }
    }
}
pub struct DataprocMetastoreServiceMetadataIntegrationElDataCatalogConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocMetastoreServiceMetadataIntegrationElDataCatalogConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocMetastoreServiceMetadataIntegrationElDataCatalogConfigElRef {
        DataprocMetastoreServiceMetadataIntegrationElDataCatalogConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocMetastoreServiceMetadataIntegrationElDataCatalogConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nDefines whether the metastore metadata should be synced to Data Catalog. The default value is to disable syncing metastore metadata to Data Catalog."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataprocMetastoreServiceMetadataIntegrationElDynamic {
    data_catalog_config:
        Option<DynamicBlock<DataprocMetastoreServiceMetadataIntegrationElDataCatalogConfigEl>>,
}
#[derive(Serialize)]
pub struct DataprocMetastoreServiceMetadataIntegrationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_catalog_config:
        Option<Vec<DataprocMetastoreServiceMetadataIntegrationElDataCatalogConfigEl>>,
    dynamic: DataprocMetastoreServiceMetadataIntegrationElDynamic,
}
impl DataprocMetastoreServiceMetadataIntegrationEl {
    #[doc = "Set the field `data_catalog_config`.\n"]
    pub fn set_data_catalog_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocMetastoreServiceMetadataIntegrationElDataCatalogConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data_catalog_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data_catalog_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocMetastoreServiceMetadataIntegrationEl {
    type O = BlockAssignable<DataprocMetastoreServiceMetadataIntegrationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocMetastoreServiceMetadataIntegrationEl {}
impl BuildDataprocMetastoreServiceMetadataIntegrationEl {
    pub fn build(self) -> DataprocMetastoreServiceMetadataIntegrationEl {
        DataprocMetastoreServiceMetadataIntegrationEl {
            data_catalog_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocMetastoreServiceMetadataIntegrationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocMetastoreServiceMetadataIntegrationElRef {
    fn new(shared: StackShared, base: String) -> DataprocMetastoreServiceMetadataIntegrationElRef {
        DataprocMetastoreServiceMetadataIntegrationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocMetastoreServiceMetadataIntegrationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_catalog_config` after provisioning.\n"]
    pub fn data_catalog_config(
        &self,
    ) -> ListRef<DataprocMetastoreServiceMetadataIntegrationElDataCatalogConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_catalog_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocMetastoreServiceNetworkConfigElConsumersEl {
    subnetwork: PrimField<String>,
}
impl DataprocMetastoreServiceNetworkConfigElConsumersEl {}
impl ToListMappable for DataprocMetastoreServiceNetworkConfigElConsumersEl {
    type O = BlockAssignable<DataprocMetastoreServiceNetworkConfigElConsumersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocMetastoreServiceNetworkConfigElConsumersEl {
    #[doc = "The subnetwork of the customer project from which an IP address is reserved and used as the Dataproc Metastore service's endpoint.\nIt is accessible to hosts in the subnet and to all hosts in a subnet in the same region and same network.\nThere must be at least one IP address available in the subnet's primary range. The subnet is specified in the following form:\n'projects/{projectNumber}/regions/{region_id}/subnetworks/{subnetwork_id}"]
    pub subnetwork: PrimField<String>,
}
impl BuildDataprocMetastoreServiceNetworkConfigElConsumersEl {
    pub fn build(self) -> DataprocMetastoreServiceNetworkConfigElConsumersEl {
        DataprocMetastoreServiceNetworkConfigElConsumersEl {
            subnetwork: self.subnetwork,
        }
    }
}
pub struct DataprocMetastoreServiceNetworkConfigElConsumersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocMetastoreServiceNetworkConfigElConsumersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocMetastoreServiceNetworkConfigElConsumersElRef {
        DataprocMetastoreServiceNetworkConfigElConsumersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocMetastoreServiceNetworkConfigElConsumersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `endpoint_uri` after provisioning.\nThe URI of the endpoint used to access the metastore service."]
    pub fn endpoint_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nThe subnetwork of the customer project from which an IP address is reserved and used as the Dataproc Metastore service's endpoint.\nIt is accessible to hosts in the subnet and to all hosts in a subnet in the same region and same network.\nThere must be at least one IP address available in the subnet's primary range. The subnet is specified in the following form:\n'projects/{projectNumber}/regions/{region_id}/subnetworks/{subnetwork_id}"]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataprocMetastoreServiceNetworkConfigElDynamic {
    consumers: Option<DynamicBlock<DataprocMetastoreServiceNetworkConfigElConsumersEl>>,
}
#[derive(Serialize)]
pub struct DataprocMetastoreServiceNetworkConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    consumers: Option<Vec<DataprocMetastoreServiceNetworkConfigElConsumersEl>>,
    dynamic: DataprocMetastoreServiceNetworkConfigElDynamic,
}
impl DataprocMetastoreServiceNetworkConfigEl {
    #[doc = "Set the field `consumers`.\n"]
    pub fn set_consumers(
        mut self,
        v: impl Into<BlockAssignable<DataprocMetastoreServiceNetworkConfigElConsumersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.consumers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.consumers = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocMetastoreServiceNetworkConfigEl {
    type O = BlockAssignable<DataprocMetastoreServiceNetworkConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocMetastoreServiceNetworkConfigEl {}
impl BuildDataprocMetastoreServiceNetworkConfigEl {
    pub fn build(self) -> DataprocMetastoreServiceNetworkConfigEl {
        DataprocMetastoreServiceNetworkConfigEl {
            consumers: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocMetastoreServiceNetworkConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocMetastoreServiceNetworkConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocMetastoreServiceNetworkConfigElRef {
        DataprocMetastoreServiceNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocMetastoreServiceNetworkConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `consumers` after provisioning.\n"]
    pub fn consumers(&self) -> ListRef<DataprocMetastoreServiceNetworkConfigElConsumersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.consumers", self.base))
    }
}
#[derive(Serialize)]
pub struct DataprocMetastoreServiceScalingConfigElAutoscalingConfigElLimitConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_scaling_factor: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_scaling_factor: Option<PrimField<f64>>,
}
impl DataprocMetastoreServiceScalingConfigElAutoscalingConfigElLimitConfigEl {
    #[doc = "Set the field `max_scaling_factor`.\nThe maximum scaling factor that the service will autoscale to. The default value is 6.0."]
    pub fn set_max_scaling_factor(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_scaling_factor = Some(v.into());
        self
    }
    #[doc = "Set the field `min_scaling_factor`.\nThe minimum scaling factor that the service will autoscale to. The default value is 0.1."]
    pub fn set_min_scaling_factor(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_scaling_factor = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocMetastoreServiceScalingConfigElAutoscalingConfigElLimitConfigEl {
    type O =
        BlockAssignable<DataprocMetastoreServiceScalingConfigElAutoscalingConfigElLimitConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocMetastoreServiceScalingConfigElAutoscalingConfigElLimitConfigEl {}
impl BuildDataprocMetastoreServiceScalingConfigElAutoscalingConfigElLimitConfigEl {
    pub fn build(self) -> DataprocMetastoreServiceScalingConfigElAutoscalingConfigElLimitConfigEl {
        DataprocMetastoreServiceScalingConfigElAutoscalingConfigElLimitConfigEl {
            max_scaling_factor: core::default::Default::default(),
            min_scaling_factor: core::default::Default::default(),
        }
    }
}
pub struct DataprocMetastoreServiceScalingConfigElAutoscalingConfigElLimitConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocMetastoreServiceScalingConfigElAutoscalingConfigElLimitConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocMetastoreServiceScalingConfigElAutoscalingConfigElLimitConfigElRef {
        DataprocMetastoreServiceScalingConfigElAutoscalingConfigElLimitConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocMetastoreServiceScalingConfigElAutoscalingConfigElLimitConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_scaling_factor` after provisioning.\nThe maximum scaling factor that the service will autoscale to. The default value is 6.0."]
    pub fn max_scaling_factor(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_scaling_factor", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_scaling_factor` after provisioning.\nThe minimum scaling factor that the service will autoscale to. The default value is 0.1."]
    pub fn min_scaling_factor(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_scaling_factor", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataprocMetastoreServiceScalingConfigElAutoscalingConfigElDynamic {
    limit_config: Option<
        DynamicBlock<DataprocMetastoreServiceScalingConfigElAutoscalingConfigElLimitConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct DataprocMetastoreServiceScalingConfigElAutoscalingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit_config:
        Option<Vec<DataprocMetastoreServiceScalingConfigElAutoscalingConfigElLimitConfigEl>>,
    dynamic: DataprocMetastoreServiceScalingConfigElAutoscalingConfigElDynamic,
}
impl DataprocMetastoreServiceScalingConfigElAutoscalingConfigEl {
    #[doc = "Set the field `autoscaling_enabled`.\nDefines whether autoscaling is enabled. The default value is false."]
    pub fn set_autoscaling_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.autoscaling_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `limit_config`.\n"]
    pub fn set_limit_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataprocMetastoreServiceScalingConfigElAutoscalingConfigElLimitConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.limit_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.limit_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocMetastoreServiceScalingConfigElAutoscalingConfigEl {
    type O = BlockAssignable<DataprocMetastoreServiceScalingConfigElAutoscalingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocMetastoreServiceScalingConfigElAutoscalingConfigEl {}
impl BuildDataprocMetastoreServiceScalingConfigElAutoscalingConfigEl {
    pub fn build(self) -> DataprocMetastoreServiceScalingConfigElAutoscalingConfigEl {
        DataprocMetastoreServiceScalingConfigElAutoscalingConfigEl {
            autoscaling_enabled: core::default::Default::default(),
            limit_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocMetastoreServiceScalingConfigElAutoscalingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocMetastoreServiceScalingConfigElAutoscalingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocMetastoreServiceScalingConfigElAutoscalingConfigElRef {
        DataprocMetastoreServiceScalingConfigElAutoscalingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocMetastoreServiceScalingConfigElAutoscalingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `autoscaling_enabled` after provisioning.\nDefines whether autoscaling is enabled. The default value is false."]
    pub fn autoscaling_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.autoscaling_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `autoscaling_factor` after provisioning.\nOutput only. The scaling factor of a service with autoscaling enabled."]
    pub fn autoscaling_factor(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.autoscaling_factor", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `limit_config` after provisioning.\n"]
    pub fn limit_config(
        &self,
    ) -> ListRef<DataprocMetastoreServiceScalingConfigElAutoscalingConfigElLimitConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.limit_config", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataprocMetastoreServiceScalingConfigElDynamic {
    autoscaling_config:
        Option<DynamicBlock<DataprocMetastoreServiceScalingConfigElAutoscalingConfigEl>>,
}
#[derive(Serialize)]
pub struct DataprocMetastoreServiceScalingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_size: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scaling_factor: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling_config: Option<Vec<DataprocMetastoreServiceScalingConfigElAutoscalingConfigEl>>,
    dynamic: DataprocMetastoreServiceScalingConfigElDynamic,
}
impl DataprocMetastoreServiceScalingConfigEl {
    #[doc = "Set the field `instance_size`.\nMetastore instance sizes. Possible values: [\"EXTRA_SMALL\", \"SMALL\", \"MEDIUM\", \"LARGE\", \"EXTRA_LARGE\"]"]
    pub fn set_instance_size(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_size = Some(v.into());
        self
    }
    #[doc = "Set the field `scaling_factor`.\nScaling factor, in increments of 0.1 for values less than 1.0, and increments of 1.0 for values greater than 1.0."]
    pub fn set_scaling_factor(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.scaling_factor = Some(v.into());
        self
    }
    #[doc = "Set the field `autoscaling_config`.\n"]
    pub fn set_autoscaling_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocMetastoreServiceScalingConfigElAutoscalingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.autoscaling_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocMetastoreServiceScalingConfigEl {
    type O = BlockAssignable<DataprocMetastoreServiceScalingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocMetastoreServiceScalingConfigEl {}
impl BuildDataprocMetastoreServiceScalingConfigEl {
    pub fn build(self) -> DataprocMetastoreServiceScalingConfigEl {
        DataprocMetastoreServiceScalingConfigEl {
            instance_size: core::default::Default::default(),
            scaling_factor: core::default::Default::default(),
            autoscaling_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocMetastoreServiceScalingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocMetastoreServiceScalingConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocMetastoreServiceScalingConfigElRef {
        DataprocMetastoreServiceScalingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocMetastoreServiceScalingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_size` after provisioning.\nMetastore instance sizes. Possible values: [\"EXTRA_SMALL\", \"SMALL\", \"MEDIUM\", \"LARGE\", \"EXTRA_LARGE\"]"]
    pub fn instance_size(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_size", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scaling_factor` after provisioning.\nScaling factor, in increments of 0.1 for values less than 1.0, and increments of 1.0 for values greater than 1.0."]
    pub fn scaling_factor(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scaling_factor", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `autoscaling_config` after provisioning.\n"]
    pub fn autoscaling_config(
        &self,
    ) -> ListRef<DataprocMetastoreServiceScalingConfigElAutoscalingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocMetastoreServiceScheduledBackupEl {
    backup_location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cron_schedule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<PrimField<String>>,
}
impl DataprocMetastoreServiceScheduledBackupEl {
    #[doc = "Set the field `cron_schedule`.\nThe scheduled interval in Cron format, see https://en.wikipedia.org/wiki/Cron The default is empty: scheduled backup is not enabled. Must be specified to enable scheduled backups."]
    pub fn set_cron_schedule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cron_schedule = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\nDefines whether the scheduled backup is enabled. The default value is false."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `time_zone`.\nSpecifies the time zone to be used when interpreting cronSchedule. Must be a time zone name from the time zone database (https://en.wikipedia.org/wiki/List_of_tz_database_time_zones), e.g. America/Los_Angeles or Africa/Abidjan. If left unspecified, the default is UTC."]
    pub fn set_time_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.time_zone = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocMetastoreServiceScheduledBackupEl {
    type O = BlockAssignable<DataprocMetastoreServiceScheduledBackupEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocMetastoreServiceScheduledBackupEl {
    #[doc = "A Cloud Storage URI of a folder, in the format gs://<bucket_name>/<path_inside_bucket>. A sub-folder <backup_folder> containing backup files will be stored below it."]
    pub backup_location: PrimField<String>,
}
impl BuildDataprocMetastoreServiceScheduledBackupEl {
    pub fn build(self) -> DataprocMetastoreServiceScheduledBackupEl {
        DataprocMetastoreServiceScheduledBackupEl {
            backup_location: self.backup_location,
            cron_schedule: core::default::Default::default(),
            enabled: core::default::Default::default(),
            time_zone: core::default::Default::default(),
        }
    }
}
pub struct DataprocMetastoreServiceScheduledBackupElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocMetastoreServiceScheduledBackupElRef {
    fn new(shared: StackShared, base: String) -> DataprocMetastoreServiceScheduledBackupElRef {
        DataprocMetastoreServiceScheduledBackupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocMetastoreServiceScheduledBackupElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup_location` after provisioning.\nA Cloud Storage URI of a folder, in the format gs://<bucket_name>/<path_inside_bucket>. A sub-folder <backup_folder> containing backup files will be stored below it."]
    pub fn backup_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_location", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cron_schedule` after provisioning.\nThe scheduled interval in Cron format, see https://en.wikipedia.org/wiki/Cron The default is empty: scheduled backup is not enabled. Must be specified to enable scheduled backups."]
    pub fn cron_schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cron_schedule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nDefines whether the scheduled backup is enabled. The default value is false."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\nSpecifies the time zone to be used when interpreting cronSchedule. Must be a time zone name from the time zone database (https://en.wikipedia.org/wiki/List_of_tz_database_time_zones), e.g. America/Los_Angeles or Africa/Abidjan. If left unspecified, the default is UTC."]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}
#[derive(Serialize)]
pub struct DataprocMetastoreServiceTelemetryConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    log_format: Option<PrimField<String>>,
}
impl DataprocMetastoreServiceTelemetryConfigEl {
    #[doc = "Set the field `log_format`.\nThe output format of the Dataproc Metastore service's logs. Default value: \"JSON\" Possible values: [\"LEGACY\", \"JSON\"]"]
    pub fn set_log_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_format = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocMetastoreServiceTelemetryConfigEl {
    type O = BlockAssignable<DataprocMetastoreServiceTelemetryConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocMetastoreServiceTelemetryConfigEl {}
impl BuildDataprocMetastoreServiceTelemetryConfigEl {
    pub fn build(self) -> DataprocMetastoreServiceTelemetryConfigEl {
        DataprocMetastoreServiceTelemetryConfigEl {
            log_format: core::default::Default::default(),
        }
    }
}
pub struct DataprocMetastoreServiceTelemetryConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocMetastoreServiceTelemetryConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocMetastoreServiceTelemetryConfigElRef {
        DataprocMetastoreServiceTelemetryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocMetastoreServiceTelemetryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `log_format` after provisioning.\nThe output format of the Dataproc Metastore service's logs. Default value: \"JSON\" Possible values: [\"LEGACY\", \"JSON\"]"]
    pub fn log_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_format", self.base))
    }
}
#[derive(Serialize)]
pub struct DataprocMetastoreServiceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DataprocMetastoreServiceTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocMetastoreServiceTimeoutsEl {
    type O = BlockAssignable<DataprocMetastoreServiceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocMetastoreServiceTimeoutsEl {}
impl BuildDataprocMetastoreServiceTimeoutsEl {
    pub fn build(self) -> DataprocMetastoreServiceTimeoutsEl {
        DataprocMetastoreServiceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DataprocMetastoreServiceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocMetastoreServiceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataprocMetastoreServiceTimeoutsElRef {
        DataprocMetastoreServiceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocMetastoreServiceTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataprocMetastoreServiceDynamic {
    encryption_config: Option<DynamicBlock<DataprocMetastoreServiceEncryptionConfigEl>>,
    hive_metastore_config: Option<DynamicBlock<DataprocMetastoreServiceHiveMetastoreConfigEl>>,
    maintenance_window: Option<DynamicBlock<DataprocMetastoreServiceMaintenanceWindowEl>>,
    metadata_integration: Option<DynamicBlock<DataprocMetastoreServiceMetadataIntegrationEl>>,
    network_config: Option<DynamicBlock<DataprocMetastoreServiceNetworkConfigEl>>,
    scaling_config: Option<DynamicBlock<DataprocMetastoreServiceScalingConfigEl>>,
    scheduled_backup: Option<DynamicBlock<DataprocMetastoreServiceScheduledBackupEl>>,
    telemetry_config: Option<DynamicBlock<DataprocMetastoreServiceTelemetryConfigEl>>,
}
