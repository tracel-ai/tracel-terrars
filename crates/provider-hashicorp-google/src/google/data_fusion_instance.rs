use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataFusionInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dataproc_service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_rbac: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_stackdriver_logging: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_stackdriver_monitoring: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    patch_revision: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_instance: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerators: Option<Vec<DataFusionInstanceAcceleratorsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    crypto_key_config: Option<Vec<DataFusionInstanceCryptoKeyConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_publish_config: Option<Vec<DataFusionInstanceEventPublishConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_config: Option<Vec<DataFusionInstanceNetworkConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataFusionInstanceTimeoutsEl>,
    dynamic: DataFusionInstanceDynamic,
}
struct DataFusionInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataFusionInstanceData>,
}
#[derive(Clone)]
pub struct DataFusionInstance(Rc<DataFusionInstance_>);
impl DataFusionInstance {
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
    #[doc = "Set the field `dataproc_service_account`.\nUser-managed service account to set on Dataproc when Cloud Data Fusion creates Dataproc to run data processing pipelines."]
    pub fn set_dataproc_service_account(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().dataproc_service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nAn optional description of the instance."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nDisplay name for an instance."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_rbac`.\nOption to enable granular role-based access control."]
    pub fn set_enable_rbac(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_rbac = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_stackdriver_logging`.\nOption to enable Stackdriver Logging."]
    pub fn set_enable_stackdriver_logging(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_stackdriver_logging = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_stackdriver_monitoring`.\nOption to enable Stackdriver Monitoring."]
    pub fn set_enable_stackdriver_monitoring(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_stackdriver_monitoring = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nThe resource labels for instance to use to annotate any related underlying resources,\nsuch as Compute Engine VMs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `options`.\nMap of additional options used to configure the behavior of Data Fusion instance."]
    pub fn set_options(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().options = Some(v.into());
        self
    }
    #[doc = "Set the field `patch_revision`.\nCurrent patch revision of the Data Fusion."]
    pub fn set_patch_revision(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().patch_revision = Some(v.into());
        self
    }
    #[doc = "Set the field `private_instance`.\nSpecifies whether the Data Fusion instance should be private. If set to\ntrue, all Data Fusion nodes will have private IP addresses and will not be\nable to access the public internet."]
    pub fn set_private_instance(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().private_instance = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nThe region of the Data Fusion instance."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\nA map of resource manager tags.\nResource manager tag keys and values have the same definition as resource manager tags.\nKeys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/{tag_value_id}.\nThe field is ignored (both PUT & PATCH) when empty."]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\nCurrent version of the Data Fusion."]
    pub fn set_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\nName of the zone in which the Data Fusion instance will be created. Only DEVELOPER instances use this field."]
    pub fn set_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone = Some(v.into());
        self
    }
    #[doc = "Set the field `accelerators`.\n"]
    pub fn set_accelerators(
        self,
        v: impl Into<BlockAssignable<DataFusionInstanceAcceleratorsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().accelerators = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.accelerators = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `crypto_key_config`.\n"]
    pub fn set_crypto_key_config(
        self,
        v: impl Into<BlockAssignable<DataFusionInstanceCryptoKeyConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().crypto_key_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.crypto_key_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `event_publish_config`.\n"]
    pub fn set_event_publish_config(
        self,
        v: impl Into<BlockAssignable<DataFusionInstanceEventPublishConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event_publish_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event_publish_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_config`.\n"]
    pub fn set_network_config(
        self,
        v: impl Into<BlockAssignable<DataFusionInstanceNetworkConfigEl>>,
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
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataFusionInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `api_endpoint` after provisioning.\nEndpoint on which the REST APIs is accessible."]
    pub fn api_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time the instance was created in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataproc_service_account` after provisioning.\nUser-managed service account to set on Dataproc when Cloud Data Fusion creates Dataproc to run data processing pipelines."]
    pub fn dataproc_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataproc_service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of the instance."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name for an instance."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_rbac` after provisioning.\nOption to enable granular role-based access control."]
    pub fn enable_rbac(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_rbac", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_stackdriver_logging` after provisioning.\nOption to enable Stackdriver Logging."]
    pub fn enable_stackdriver_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_stackdriver_logging", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_stackdriver_monitoring` after provisioning.\nOption to enable Stackdriver Monitoring."]
    pub fn enable_stackdriver_monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_stackdriver_monitoring", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_bucket` after provisioning.\nCloud Storage bucket generated by Data Fusion in the customer project."]
    pub fn gcs_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcs_bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe resource labels for instance to use to annotate any related underlying resources,\nsuch as Compute Engine VMs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the instance or a fully qualified identifier for the instance."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `options` after provisioning.\nMap of additional options used to configure the behavior of Data Fusion instance."]
    pub fn options(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `p4_service_account` after provisioning.\nP4 service account for the customer project."]
    pub fn p4_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.p4_service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `patch_revision` after provisioning.\nCurrent patch revision of the Data Fusion."]
    pub fn patch_revision(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.patch_revision", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_instance` after provisioning.\nSpecifies whether the Data Fusion instance should be private. If set to\ntrue, all Data Fusion nodes will have private IP addresses and will not be\nable to access the public internet."]
    pub fn private_instance(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the Data Fusion instance."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_endpoint` after provisioning.\nEndpoint on which the Data Fusion UI and REST APIs are accessible."]
    pub fn service_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of this Data Fusion instance.\n- CREATING: Instance is being created\n- RUNNING: Instance is running and ready for requests\n- FAILED: Instance creation failed\n- DELETING: Instance is being deleted\n- UPGRADING: Instance is being upgraded\n- RESTARTING: Instance is being restarted"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_message` after provisioning.\nAdditional information about the current state of this Data Fusion instance if available."]
    pub fn state_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_message", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nA map of resource manager tags.\nResource manager tag keys and values have the same definition as resource manager tags.\nKeys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/{tag_value_id}.\nThe field is ignored (both PUT & PATCH) when empty."]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tenant_project_id` after provisioning.\nThe name of the tenant project."]
    pub fn tenant_project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tenant_project_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nRepresents the type of Data Fusion instance. Each type is configured with\nthe default settings for processing and memory.\n- BASIC: Basic Data Fusion instance. In Basic type, the user will be able to create data pipelines\nusing point and click UI. However, there are certain limitations, such as fewer number\nof concurrent pipelines, no support for streaming pipelines, etc.\n- ENTERPRISE: Enterprise Data Fusion instance. In Enterprise type, the user will have more features\navailable, such as support for streaming pipelines, higher number of concurrent pipelines, etc.\n- DEVELOPER: Developer Data Fusion instance. In Developer type, the user will have all features available but\nwith restrictive capabilities. This is to help enterprises design and develop their data ingestion and integration\npipelines at low cost. Possible values: [\"BASIC\", \"ENTERPRISE\", \"DEVELOPER\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time the instance was last updated in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nCurrent version of the Data Fusion."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nName of the zone in which the Data Fusion instance will be created. Only DEVELOPER instances use this field."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `accelerators` after provisioning.\n"]
    pub fn accelerators(&self) -> ListRef<DataFusionInstanceAcceleratorsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.accelerators", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `crypto_key_config` after provisioning.\n"]
    pub fn crypto_key_config(&self) -> ListRef<DataFusionInstanceCryptoKeyConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.crypto_key_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_publish_config` after provisioning.\n"]
    pub fn event_publish_config(&self) -> ListRef<DataFusionInstanceEventPublishConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.event_publish_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<DataFusionInstanceNetworkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataFusionInstanceTimeoutsElRef {
        DataFusionInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataFusionInstance {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DataFusionInstance {}
impl ToListMappable for DataFusionInstance {
    type O = ListRef<DataFusionInstanceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DataFusionInstance_ {
    fn extract_resource_type(&self) -> String {
        "google_data_fusion_instance".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataFusionInstance {
    pub tf_id: String,
    #[doc = "The ID of the instance or a fully qualified identifier for the instance."]
    pub name: PrimField<String>,
    #[doc = "Represents the type of Data Fusion instance. Each type is configured with\nthe default settings for processing and memory.\n- BASIC: Basic Data Fusion instance. In Basic type, the user will be able to create data pipelines\nusing point and click UI. However, there are certain limitations, such as fewer number\nof concurrent pipelines, no support for streaming pipelines, etc.\n- ENTERPRISE: Enterprise Data Fusion instance. In Enterprise type, the user will have more features\navailable, such as support for streaming pipelines, higher number of concurrent pipelines, etc.\n- DEVELOPER: Developer Data Fusion instance. In Developer type, the user will have all features available but\nwith restrictive capabilities. This is to help enterprises design and develop their data ingestion and integration\npipelines at low cost. Possible values: [\"BASIC\", \"ENTERPRISE\", \"DEVELOPER\"]"]
    pub type_: PrimField<String>,
}
impl BuildDataFusionInstance {
    pub fn build(self, stack: &mut Stack) -> DataFusionInstance {
        let out = DataFusionInstance(Rc::new(DataFusionInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataFusionInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                dataproc_service_account: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                enable_rbac: core::default::Default::default(),
                enable_stackdriver_logging: core::default::Default::default(),
                enable_stackdriver_monitoring: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                options: core::default::Default::default(),
                patch_revision: core::default::Default::default(),
                private_instance: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                type_: self.type_,
                version: core::default::Default::default(),
                zone: core::default::Default::default(),
                accelerators: core::default::Default::default(),
                crypto_key_config: core::default::Default::default(),
                event_publish_config: core::default::Default::default(),
                network_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DataFusionInstanceRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataFusionInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataFusionInstanceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_endpoint` after provisioning.\nEndpoint on which the REST APIs is accessible."]
    pub fn api_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time the instance was created in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataproc_service_account` after provisioning.\nUser-managed service account to set on Dataproc when Cloud Data Fusion creates Dataproc to run data processing pipelines."]
    pub fn dataproc_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataproc_service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of the instance."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name for an instance."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_rbac` after provisioning.\nOption to enable granular role-based access control."]
    pub fn enable_rbac(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_rbac", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_stackdriver_logging` after provisioning.\nOption to enable Stackdriver Logging."]
    pub fn enable_stackdriver_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_stackdriver_logging", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_stackdriver_monitoring` after provisioning.\nOption to enable Stackdriver Monitoring."]
    pub fn enable_stackdriver_monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_stackdriver_monitoring", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_bucket` after provisioning.\nCloud Storage bucket generated by Data Fusion in the customer project."]
    pub fn gcs_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcs_bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe resource labels for instance to use to annotate any related underlying resources,\nsuch as Compute Engine VMs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the instance or a fully qualified identifier for the instance."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `options` after provisioning.\nMap of additional options used to configure the behavior of Data Fusion instance."]
    pub fn options(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `p4_service_account` after provisioning.\nP4 service account for the customer project."]
    pub fn p4_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.p4_service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `patch_revision` after provisioning.\nCurrent patch revision of the Data Fusion."]
    pub fn patch_revision(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.patch_revision", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_instance` after provisioning.\nSpecifies whether the Data Fusion instance should be private. If set to\ntrue, all Data Fusion nodes will have private IP addresses and will not be\nable to access the public internet."]
    pub fn private_instance(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the Data Fusion instance."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_endpoint` after provisioning.\nEndpoint on which the Data Fusion UI and REST APIs are accessible."]
    pub fn service_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of this Data Fusion instance.\n- CREATING: Instance is being created\n- RUNNING: Instance is running and ready for requests\n- FAILED: Instance creation failed\n- DELETING: Instance is being deleted\n- UPGRADING: Instance is being upgraded\n- RESTARTING: Instance is being restarted"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_message` after provisioning.\nAdditional information about the current state of this Data Fusion instance if available."]
    pub fn state_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_message", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nA map of resource manager tags.\nResource manager tag keys and values have the same definition as resource manager tags.\nKeys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/{tag_value_id}.\nThe field is ignored (both PUT & PATCH) when empty."]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tenant_project_id` after provisioning.\nThe name of the tenant project."]
    pub fn tenant_project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tenant_project_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nRepresents the type of Data Fusion instance. Each type is configured with\nthe default settings for processing and memory.\n- BASIC: Basic Data Fusion instance. In Basic type, the user will be able to create data pipelines\nusing point and click UI. However, there are certain limitations, such as fewer number\nof concurrent pipelines, no support for streaming pipelines, etc.\n- ENTERPRISE: Enterprise Data Fusion instance. In Enterprise type, the user will have more features\navailable, such as support for streaming pipelines, higher number of concurrent pipelines, etc.\n- DEVELOPER: Developer Data Fusion instance. In Developer type, the user will have all features available but\nwith restrictive capabilities. This is to help enterprises design and develop their data ingestion and integration\npipelines at low cost. Possible values: [\"BASIC\", \"ENTERPRISE\", \"DEVELOPER\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time the instance was last updated in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nCurrent version of the Data Fusion."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nName of the zone in which the Data Fusion instance will be created. Only DEVELOPER instances use this field."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `accelerators` after provisioning.\n"]
    pub fn accelerators(&self) -> ListRef<DataFusionInstanceAcceleratorsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.accelerators", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `crypto_key_config` after provisioning.\n"]
    pub fn crypto_key_config(&self) -> ListRef<DataFusionInstanceCryptoKeyConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.crypto_key_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_publish_config` after provisioning.\n"]
    pub fn event_publish_config(&self) -> ListRef<DataFusionInstanceEventPublishConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.event_publish_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<DataFusionInstanceNetworkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataFusionInstanceTimeoutsElRef {
        DataFusionInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataFusionInstanceAcceleratorsEl {
    accelerator_type: PrimField<String>,
    state: PrimField<String>,
}
impl DataFusionInstanceAcceleratorsEl {}
impl ToListMappable for DataFusionInstanceAcceleratorsEl {
    type O = BlockAssignable<DataFusionInstanceAcceleratorsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataFusionInstanceAcceleratorsEl {
    #[doc = "The type of an accelator for a CDF instance. Possible values: [\"CDC\", \"HEALTHCARE\", \"CCAI_INSIGHTS\"]"]
    pub accelerator_type: PrimField<String>,
    #[doc = "The type of an accelator for a CDF instance. Possible values: [\"ENABLED\", \"DISABLED\"]"]
    pub state: PrimField<String>,
}
impl BuildDataFusionInstanceAcceleratorsEl {
    pub fn build(self) -> DataFusionInstanceAcceleratorsEl {
        DataFusionInstanceAcceleratorsEl {
            accelerator_type: self.accelerator_type,
            state: self.state,
        }
    }
}
pub struct DataFusionInstanceAcceleratorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataFusionInstanceAcceleratorsElRef {
    fn new(shared: StackShared, base: String) -> DataFusionInstanceAcceleratorsElRef {
        DataFusionInstanceAcceleratorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataFusionInstanceAcceleratorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accelerator_type` after provisioning.\nThe type of an accelator for a CDF instance. Possible values: [\"CDC\", \"HEALTHCARE\", \"CCAI_INSIGHTS\"]"]
    pub fn accelerator_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe type of an accelator for a CDF instance. Possible values: [\"ENABLED\", \"DISABLED\"]"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct DataFusionInstanceCryptoKeyConfigEl {
    key_reference: PrimField<String>,
}
impl DataFusionInstanceCryptoKeyConfigEl {}
impl ToListMappable for DataFusionInstanceCryptoKeyConfigEl {
    type O = BlockAssignable<DataFusionInstanceCryptoKeyConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataFusionInstanceCryptoKeyConfigEl {
    #[doc = "The name of the key which is used to encrypt/decrypt customer data. For key in Cloud KMS, the key should be in the format of projects/*/locations/*/keyRings/*/cryptoKeys/*."]
    pub key_reference: PrimField<String>,
}
impl BuildDataFusionInstanceCryptoKeyConfigEl {
    pub fn build(self) -> DataFusionInstanceCryptoKeyConfigEl {
        DataFusionInstanceCryptoKeyConfigEl {
            key_reference: self.key_reference,
        }
    }
}
pub struct DataFusionInstanceCryptoKeyConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataFusionInstanceCryptoKeyConfigElRef {
    fn new(shared: StackShared, base: String) -> DataFusionInstanceCryptoKeyConfigElRef {
        DataFusionInstanceCryptoKeyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataFusionInstanceCryptoKeyConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key_reference` after provisioning.\nThe name of the key which is used to encrypt/decrypt customer data. For key in Cloud KMS, the key should be in the format of projects/*/locations/*/keyRings/*/cryptoKeys/*."]
    pub fn key_reference(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_reference", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataFusionInstanceEventPublishConfigEl {
    enabled: PrimField<bool>,
    topic: PrimField<String>,
}
impl DataFusionInstanceEventPublishConfigEl {}
impl ToListMappable for DataFusionInstanceEventPublishConfigEl {
    type O = BlockAssignable<DataFusionInstanceEventPublishConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataFusionInstanceEventPublishConfigEl {
    #[doc = "Option to enable Event Publishing."]
    pub enabled: PrimField<bool>,
    #[doc = "The resource name of the Pub/Sub topic. Format: projects/{projectId}/topics/{topic_id}"]
    pub topic: PrimField<String>,
}
impl BuildDataFusionInstanceEventPublishConfigEl {
    pub fn build(self) -> DataFusionInstanceEventPublishConfigEl {
        DataFusionInstanceEventPublishConfigEl {
            enabled: self.enabled,
            topic: self.topic,
        }
    }
}
pub struct DataFusionInstanceEventPublishConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataFusionInstanceEventPublishConfigElRef {
    fn new(shared: StackShared, base: String) -> DataFusionInstanceEventPublishConfigElRef {
        DataFusionInstanceEventPublishConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataFusionInstanceEventPublishConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nOption to enable Event Publishing."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\nThe resource name of the Pub/Sub topic. Format: projects/{projectId}/topics/{topic_id}"]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}
#[derive(Serialize)]
pub struct DataFusionInstanceNetworkConfigElPrivateServiceConnectConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network_attachment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unreachable_cidr_block: Option<PrimField<String>>,
}
impl DataFusionInstanceNetworkConfigElPrivateServiceConnectConfigEl {
    #[doc = "Set the field `network_attachment`.\nOptional. The reference to the network attachment used to establish private connectivity.\nIt will be of the form projects/{project-id}/regions/{region}/networkAttachments/{network-attachment-id}.\nThis is required only when using connection type PRIVATE_SERVICE_CONNECT_INTERFACES."]
    pub fn set_network_attachment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_attachment = Some(v.into());
        self
    }
    #[doc = "Set the field `unreachable_cidr_block`.\nOptional. Input only. The CIDR block to which the CDF instance can't route traffic to in the consumer project VPC.\nThe size of this block should be at least /25. This range should not overlap with the primary address range of any subnetwork used by the network attachment.\nThis range can be used for other purposes in the consumer VPC as long as there is no requirement for CDF to reach destinations using these addresses.\nIf this value is not provided, the server chooses a non RFC 1918 address range. The format of this field is governed by RFC 4632."]
    pub fn set_unreachable_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unreachable_cidr_block = Some(v.into());
        self
    }
}
impl ToListMappable for DataFusionInstanceNetworkConfigElPrivateServiceConnectConfigEl {
    type O = BlockAssignable<DataFusionInstanceNetworkConfigElPrivateServiceConnectConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataFusionInstanceNetworkConfigElPrivateServiceConnectConfigEl {}
impl BuildDataFusionInstanceNetworkConfigElPrivateServiceConnectConfigEl {
    pub fn build(self) -> DataFusionInstanceNetworkConfigElPrivateServiceConnectConfigEl {
        DataFusionInstanceNetworkConfigElPrivateServiceConnectConfigEl {
            network_attachment: core::default::Default::default(),
            unreachable_cidr_block: core::default::Default::default(),
        }
    }
}
pub struct DataFusionInstanceNetworkConfigElPrivateServiceConnectConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataFusionInstanceNetworkConfigElPrivateServiceConnectConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataFusionInstanceNetworkConfigElPrivateServiceConnectConfigElRef {
        DataFusionInstanceNetworkConfigElPrivateServiceConnectConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataFusionInstanceNetworkConfigElPrivateServiceConnectConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `effective_unreachable_cidr_block` after provisioning.\nOutput only. The CIDR block to which the CDF instance can't route traffic to in the consumer project VPC.\nThe size of this block is /25. The format of this field is governed by RFC 4632."]
    pub fn effective_unreachable_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_unreachable_cidr_block", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network_attachment` after provisioning.\nOptional. The reference to the network attachment used to establish private connectivity.\nIt will be of the form projects/{project-id}/regions/{region}/networkAttachments/{network-attachment-id}.\nThis is required only when using connection type PRIVATE_SERVICE_CONNECT_INTERFACES."]
    pub fn network_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_attachment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `unreachable_cidr_block` after provisioning.\nOptional. Input only. The CIDR block to which the CDF instance can't route traffic to in the consumer project VPC.\nThe size of this block should be at least /25. This range should not overlap with the primary address range of any subnetwork used by the network attachment.\nThis range can be used for other purposes in the consumer VPC as long as there is no requirement for CDF to reach destinations using these addresses.\nIf this value is not provided, the server chooses a non RFC 1918 address range. The format of this field is governed by RFC 4632."]
    pub fn unreachable_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unreachable_cidr_block", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataFusionInstanceNetworkConfigElDynamic {
    private_service_connect_config:
        Option<DynamicBlock<DataFusionInstanceNetworkConfigElPrivateServiceConnectConfigEl>>,
}
#[derive(Serialize)]
pub struct DataFusionInstanceNetworkConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_allocation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_service_connect_config:
        Option<Vec<DataFusionInstanceNetworkConfigElPrivateServiceConnectConfigEl>>,
    dynamic: DataFusionInstanceNetworkConfigElDynamic,
}
impl DataFusionInstanceNetworkConfigEl {
    #[doc = "Set the field `connection_type`.\nOptional. Type of connection for establishing private IP connectivity between the Data Fusion customer project VPC and\nthe corresponding tenant project from a predefined list of available connection modes.\nIf this field is unspecified for a private instance, VPC peering is used. Possible values: [\"VPC_PEERING\", \"PRIVATE_SERVICE_CONNECT_INTERFACES\"]"]
    pub fn set_connection_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connection_type = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_allocation`.\nThe IP range in CIDR notation to use for the managed Data Fusion instance\nnodes. This range must not overlap with any other ranges used in the Data Fusion instance network."]
    pub fn set_ip_allocation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_allocation = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nName of the network in the project with which the tenant project\nwill be peered for executing pipelines. In case of shared VPC where the network resides in another host\nproject the network should specified in the form of projects/{host-project-id}/global/networks/{network}"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `private_service_connect_config`.\n"]
    pub fn set_private_service_connect_config(
        mut self,
        v: impl Into<BlockAssignable<DataFusionInstanceNetworkConfigElPrivateServiceConnectConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.private_service_connect_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.private_service_connect_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataFusionInstanceNetworkConfigEl {
    type O = BlockAssignable<DataFusionInstanceNetworkConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataFusionInstanceNetworkConfigEl {}
impl BuildDataFusionInstanceNetworkConfigEl {
    pub fn build(self) -> DataFusionInstanceNetworkConfigEl {
        DataFusionInstanceNetworkConfigEl {
            connection_type: core::default::Default::default(),
            ip_allocation: core::default::Default::default(),
            network: core::default::Default::default(),
            private_service_connect_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataFusionInstanceNetworkConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataFusionInstanceNetworkConfigElRef {
    fn new(shared: StackShared, base: String) -> DataFusionInstanceNetworkConfigElRef {
        DataFusionInstanceNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataFusionInstanceNetworkConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_type` after provisioning.\nOptional. Type of connection for establishing private IP connectivity between the Data Fusion customer project VPC and\nthe corresponding tenant project from a predefined list of available connection modes.\nIf this field is unspecified for a private instance, VPC peering is used. Possible values: [\"VPC_PEERING\", \"PRIVATE_SERVICE_CONNECT_INTERFACES\"]"]
    pub fn connection_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ip_allocation` after provisioning.\nThe IP range in CIDR notation to use for the managed Data Fusion instance\nnodes. This range must not overlap with any other ranges used in the Data Fusion instance network."]
    pub fn ip_allocation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_allocation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nName of the network in the project with which the tenant project\nwill be peered for executing pipelines. In case of shared VPC where the network resides in another host\nproject the network should specified in the form of projects/{host-project-id}/global/networks/{network}"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `private_service_connect_config` after provisioning.\n"]
    pub fn private_service_connect_config(
        &self,
    ) -> ListRef<DataFusionInstanceNetworkConfigElPrivateServiceConnectConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_service_connect_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataFusionInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DataFusionInstanceTimeoutsEl {
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
impl ToListMappable for DataFusionInstanceTimeoutsEl {
    type O = BlockAssignable<DataFusionInstanceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataFusionInstanceTimeoutsEl {}
impl BuildDataFusionInstanceTimeoutsEl {
    pub fn build(self) -> DataFusionInstanceTimeoutsEl {
        DataFusionInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DataFusionInstanceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataFusionInstanceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataFusionInstanceTimeoutsElRef {
        DataFusionInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataFusionInstanceTimeoutsElRef {
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
struct DataFusionInstanceDynamic {
    accelerators: Option<DynamicBlock<DataFusionInstanceAcceleratorsEl>>,
    crypto_key_config: Option<DynamicBlock<DataFusionInstanceCryptoKeyConfigEl>>,
    event_publish_config: Option<DynamicBlock<DataFusionInstanceEventPublishConfigEl>>,
    network_config: Option<DynamicBlock<DataFusionInstanceNetworkConfigEl>>,
}
