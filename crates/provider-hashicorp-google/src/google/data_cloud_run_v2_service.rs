use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataCloudRunV2ServiceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataCloudRunV2Service_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudRunV2ServiceData>,
}
#[derive(Clone)]
pub struct DataCloudRunV2Service(Rc<DataCloudRunV2Service_>);
impl DataCloudRunV2Service {
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
    #[doc = "Set the field `location`.\nThe location of the cloud run service"]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected in new resources.\nAll system annotations in v1 now have a corresponding field in v2 Service.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\nSettings for the Binary Authorization feature."]
    pub fn binary_authorization(&self) -> ListRef<DataCloudRunV2ServiceBinaryAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.binary_authorization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `build_config` after provisioning.\nConfiguration for building a Cloud Run function."]
    pub fn build_config(&self) -> ListRef<DataCloudRunV2ServiceBuildConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.build_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client` after provisioning.\nArbitrary identifier for the API client."]
    pub fn client(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_version` after provisioning.\nArbitrary version identifier for the API client."]
    pub fn client_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\nThe Conditions of all other associated sub-resources. They contain additional diagnostics information in case the Service does not reach its Serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn conditions(&self) -> ListRef<DataCloudRunV2ServiceConditionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conditions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe creation time."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creator` after provisioning.\nEmail address of the authenticated creator."]
    pub fn creator(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creator", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_audiences` after provisioning.\nOne or more custom audiences that you want this service to support. Specify each custom audience as the full URL in a string. The custom audiences are encoded in the token and used to authenticate requests.\nFor more information, see https://cloud.google.com/run/docs/configuring/custom-audiences."]
    pub fn custom_audiences(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_audiences", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_uri_disabled` after provisioning.\nDisables public resolution of the default URI of this service."]
    pub fn default_uri_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_uri_disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nThe deletion time."]
    pub fn delete_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the service. Defaults to true.\nWhen a'terraform destroy' or 'terraform apply' would delete the service,\nthe command will fail if this field is not set to false in Terraform state.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the service will fail.\nWhen the field is set to false, deleting the service is allowed."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-provided description of the Service. This field currently has a 512-character limit."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nA system-generated fingerprint for this version of the resource. May be used to detect modification conflict during updates."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\nFor a deleted resource, the time after which it will be permanently deleted."]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expire_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generation` after provisioning.\nA number that monotonically increases every time the user modifies the desired state. Please note that unlike v1, this is an int64 value. As with most Google APIs, its JSON representation will be a string instead of an integer."]
    pub fn generation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iap_enabled` after provisioning.\nUsed to enable/disable IAP for the cloud-run service."]
    pub fn iap_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iap_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ingress` after provisioning.\nProvides the ingress settings for this Service. On output, returns the currently observed ingress settings, or INGRESS_TRAFFIC_UNSPECIFIED if no revision is active. Possible values: [\"INGRESS_TRAFFIC_ALL\", \"INGRESS_TRAFFIC_INTERNAL_ONLY\", \"INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER\"]"]
    pub fn ingress(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ingress", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `invoker_iam_disabled` after provisioning.\nDisables IAM permission check for run.routes.invoke for callers of this service. For more information, visit https://cloud.google.com/run/docs/securing/managing-access#invoker_check."]
    pub fn invoker_iam_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.invoker_iam_disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component,\nenvironment, state, etc. For more information, visit https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.\n\nCloud Run API v2 does not support labels with  'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system labels in v1 now have a corresponding field in v2 Service.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_modifier` after provisioning.\nEmail address of the last authenticated modifier."]
    pub fn last_modifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `latest_created_revision` after provisioning.\nName of the last created revision. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn latest_created_revision(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest_created_revision", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `latest_ready_revision` after provisioning.\nName of the latest revision that is serving traffic. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn latest_ready_revision(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest_ready_revision", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `launch_stage` after provisioning.\nThe launch stage as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/products#product-launch-stages). Cloud Run supports ALPHA, BETA, and GA.\nIf no value is specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that stage. On read (or output), describes whether the resource uses preview features.\n\nFor example, if ALPHA is provided as input, but only BETA and GA-level features are used, this field will be BETA on output. Possible values: [\"UNIMPLEMENTED\", \"PRELAUNCH\", \"EARLY_ACCESS\", \"ALPHA\", \"BETA\", \"GA\", \"DEPRECATED\"]"]
    pub fn launch_stage(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.launch_stage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the cloud run service"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multi_region_settings` after provisioning.\nSettings for creating a Multi-Region Service. Make sure to use region = 'global' when using them. For more information, visit https://cloud.google.com/run/docs/multiple-regions#deploy"]
    pub fn multi_region_settings(&self) -> ListRef<DataCloudRunV2ServiceMultiRegionSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.multi_region_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Service."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `observed_generation` after provisioning.\nThe generation of this Service currently serving traffic. See comments in reconciling for additional information on reconciliation process in Cloud Run. Please note that unlike v1, this is an int64 value. As with most Google APIs, its JSON representation will be a string instead of an integer."]
    pub fn observed_generation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.observed_generation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nReturns true if the Service is currently being acted upon by the system to bring it into the desired state.\n\nWhen a new Service is created, or an existing one is updated, Cloud Run will asynchronously perform all necessary steps to bring the Service to the desired serving state. This process is called reconciliation. While reconciliation is in process, observedGeneration, latest_ready_revison, trafficStatuses, and uri will have transient values that might mismatch the intended state: Once reconciliation is over (and this field is false), there are two possible outcomes: reconciliation succeeded and the serving state matches the Service, or there was an error, and reconciliation failed. This state can be found in terminalCondition.state.\n\nIf reconciliation succeeded, the following fields will match: traffic and trafficStatuses, observedGeneration and generation, latestReadyRevision and latestCreatedRevision.\n\nIf reconciliation failed, trafficStatuses, observedGeneration, and latestReadyRevision will have the state of the last serving revision, or empty for newly created Services. Additional information on the failure can be found in terminalCondition and conditions."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scaling` after provisioning.\nScaling settings that apply to the whole service"]
    pub fn scaling(&self) -> ListRef<DataCloudRunV2ServiceScalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template` after provisioning.\nThe template used to create revisions for this Service."]
    pub fn template(&self) -> ListRef<DataCloudRunV2ServiceTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.template", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terminal_condition` after provisioning.\nThe Condition of this Service, containing its readiness status, and detailed error information in case it did not reach a serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn terminal_condition(&self) -> ListRef<DataCloudRunV2ServiceTerminalConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.terminal_condition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `traffic` after provisioning.\nSpecifies how to distribute traffic over a collection of Revisions belonging to the Service. If traffic is empty or not provided, defaults to 100% traffic to the latest Ready Revision."]
    pub fn traffic(&self) -> ListRef<DataCloudRunV2ServiceTrafficElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.traffic", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `traffic_statuses` after provisioning.\nDetailed status information for corresponding traffic targets. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn traffic_statuses(&self) -> ListRef<DataCloudRunV2ServiceTrafficStatusesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.traffic_statuses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nServer assigned unique identifier for the trigger. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe last-modified time."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe main URI in which this Service is serving traffic."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `urls` after provisioning.\nAll URLs serving traffic for this Service."]
    pub fn urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.urls", self.extract_ref()),
        )
    }
}
impl Referable for DataCloudRunV2Service {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataCloudRunV2Service {}
impl ToListMappable for DataCloudRunV2Service {
    type O = ListRef<DataCloudRunV2ServiceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataCloudRunV2Service_ {
    fn extract_datasource_type(&self) -> String {
        "google_cloud_run_v2_service".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataCloudRunV2Service {
    pub tf_id: String,
    #[doc = "Name of the Service."]
    pub name: PrimField<String>,
}
impl BuildDataCloudRunV2Service {
    pub fn build(self, stack: &mut Stack) -> DataCloudRunV2Service {
        let out = DataCloudRunV2Service(Rc::new(DataCloudRunV2Service_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCloudRunV2ServiceData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataCloudRunV2ServiceRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataCloudRunV2ServiceRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected in new resources.\nAll system annotations in v1 now have a corresponding field in v2 Service.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\nSettings for the Binary Authorization feature."]
    pub fn binary_authorization(&self) -> ListRef<DataCloudRunV2ServiceBinaryAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.binary_authorization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `build_config` after provisioning.\nConfiguration for building a Cloud Run function."]
    pub fn build_config(&self) -> ListRef<DataCloudRunV2ServiceBuildConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.build_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client` after provisioning.\nArbitrary identifier for the API client."]
    pub fn client(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_version` after provisioning.\nArbitrary version identifier for the API client."]
    pub fn client_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\nThe Conditions of all other associated sub-resources. They contain additional diagnostics information in case the Service does not reach its Serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn conditions(&self) -> ListRef<DataCloudRunV2ServiceConditionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conditions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe creation time."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creator` after provisioning.\nEmail address of the authenticated creator."]
    pub fn creator(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creator", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_audiences` after provisioning.\nOne or more custom audiences that you want this service to support. Specify each custom audience as the full URL in a string. The custom audiences are encoded in the token and used to authenticate requests.\nFor more information, see https://cloud.google.com/run/docs/configuring/custom-audiences."]
    pub fn custom_audiences(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_audiences", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_uri_disabled` after provisioning.\nDisables public resolution of the default URI of this service."]
    pub fn default_uri_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_uri_disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nThe deletion time."]
    pub fn delete_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the service. Defaults to true.\nWhen a'terraform destroy' or 'terraform apply' would delete the service,\nthe command will fail if this field is not set to false in Terraform state.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the service will fail.\nWhen the field is set to false, deleting the service is allowed."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-provided description of the Service. This field currently has a 512-character limit."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nA system-generated fingerprint for this version of the resource. May be used to detect modification conflict during updates."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\nFor a deleted resource, the time after which it will be permanently deleted."]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expire_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generation` after provisioning.\nA number that monotonically increases every time the user modifies the desired state. Please note that unlike v1, this is an int64 value. As with most Google APIs, its JSON representation will be a string instead of an integer."]
    pub fn generation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iap_enabled` after provisioning.\nUsed to enable/disable IAP for the cloud-run service."]
    pub fn iap_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iap_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ingress` after provisioning.\nProvides the ingress settings for this Service. On output, returns the currently observed ingress settings, or INGRESS_TRAFFIC_UNSPECIFIED if no revision is active. Possible values: [\"INGRESS_TRAFFIC_ALL\", \"INGRESS_TRAFFIC_INTERNAL_ONLY\", \"INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER\"]"]
    pub fn ingress(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ingress", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `invoker_iam_disabled` after provisioning.\nDisables IAM permission check for run.routes.invoke for callers of this service. For more information, visit https://cloud.google.com/run/docs/securing/managing-access#invoker_check."]
    pub fn invoker_iam_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.invoker_iam_disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component,\nenvironment, state, etc. For more information, visit https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.\n\nCloud Run API v2 does not support labels with  'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system labels in v1 now have a corresponding field in v2 Service.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_modifier` after provisioning.\nEmail address of the last authenticated modifier."]
    pub fn last_modifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `latest_created_revision` after provisioning.\nName of the last created revision. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn latest_created_revision(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest_created_revision", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `latest_ready_revision` after provisioning.\nName of the latest revision that is serving traffic. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn latest_ready_revision(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest_ready_revision", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `launch_stage` after provisioning.\nThe launch stage as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/products#product-launch-stages). Cloud Run supports ALPHA, BETA, and GA.\nIf no value is specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that stage. On read (or output), describes whether the resource uses preview features.\n\nFor example, if ALPHA is provided as input, but only BETA and GA-level features are used, this field will be BETA on output. Possible values: [\"UNIMPLEMENTED\", \"PRELAUNCH\", \"EARLY_ACCESS\", \"ALPHA\", \"BETA\", \"GA\", \"DEPRECATED\"]"]
    pub fn launch_stage(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.launch_stage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the cloud run service"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multi_region_settings` after provisioning.\nSettings for creating a Multi-Region Service. Make sure to use region = 'global' when using them. For more information, visit https://cloud.google.com/run/docs/multiple-regions#deploy"]
    pub fn multi_region_settings(&self) -> ListRef<DataCloudRunV2ServiceMultiRegionSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.multi_region_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Service."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `observed_generation` after provisioning.\nThe generation of this Service currently serving traffic. See comments in reconciling for additional information on reconciliation process in Cloud Run. Please note that unlike v1, this is an int64 value. As with most Google APIs, its JSON representation will be a string instead of an integer."]
    pub fn observed_generation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.observed_generation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nReturns true if the Service is currently being acted upon by the system to bring it into the desired state.\n\nWhen a new Service is created, or an existing one is updated, Cloud Run will asynchronously perform all necessary steps to bring the Service to the desired serving state. This process is called reconciliation. While reconciliation is in process, observedGeneration, latest_ready_revison, trafficStatuses, and uri will have transient values that might mismatch the intended state: Once reconciliation is over (and this field is false), there are two possible outcomes: reconciliation succeeded and the serving state matches the Service, or there was an error, and reconciliation failed. This state can be found in terminalCondition.state.\n\nIf reconciliation succeeded, the following fields will match: traffic and trafficStatuses, observedGeneration and generation, latestReadyRevision and latestCreatedRevision.\n\nIf reconciliation failed, trafficStatuses, observedGeneration, and latestReadyRevision will have the state of the last serving revision, or empty for newly created Services. Additional information on the failure can be found in terminalCondition and conditions."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scaling` after provisioning.\nScaling settings that apply to the whole service"]
    pub fn scaling(&self) -> ListRef<DataCloudRunV2ServiceScalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template` after provisioning.\nThe template used to create revisions for this Service."]
    pub fn template(&self) -> ListRef<DataCloudRunV2ServiceTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.template", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terminal_condition` after provisioning.\nThe Condition of this Service, containing its readiness status, and detailed error information in case it did not reach a serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn terminal_condition(&self) -> ListRef<DataCloudRunV2ServiceTerminalConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.terminal_condition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `traffic` after provisioning.\nSpecifies how to distribute traffic over a collection of Revisions belonging to the Service. If traffic is empty or not provided, defaults to 100% traffic to the latest Ready Revision."]
    pub fn traffic(&self) -> ListRef<DataCloudRunV2ServiceTrafficElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.traffic", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `traffic_statuses` after provisioning.\nDetailed status information for corresponding traffic targets. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn traffic_statuses(&self) -> ListRef<DataCloudRunV2ServiceTrafficStatusesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.traffic_statuses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nServer assigned unique identifier for the trigger. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe last-modified time."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe main URI in which this Service is serving traffic."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `urls` after provisioning.\nAll URLs serving traffic for this Service."]
    pub fn urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.urls", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceBinaryAuthorizationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    breakglass_justification: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_default: Option<PrimField<bool>>,
}
impl DataCloudRunV2ServiceBinaryAuthorizationEl {
    #[doc = "Set the field `breakglass_justification`.\n"]
    pub fn set_breakglass_justification(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.breakglass_justification = Some(v.into());
        self
    }
    #[doc = "Set the field `policy`.\n"]
    pub fn set_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy = Some(v.into());
        self
    }
    #[doc = "Set the field `use_default`.\n"]
    pub fn set_use_default(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_default = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceBinaryAuthorizationEl {
    type O = BlockAssignable<DataCloudRunV2ServiceBinaryAuthorizationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceBinaryAuthorizationEl {}
impl BuildDataCloudRunV2ServiceBinaryAuthorizationEl {
    pub fn build(self) -> DataCloudRunV2ServiceBinaryAuthorizationEl {
        DataCloudRunV2ServiceBinaryAuthorizationEl {
            breakglass_justification: core::default::Default::default(),
            policy: core::default::Default::default(),
            use_default: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceBinaryAuthorizationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceBinaryAuthorizationElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2ServiceBinaryAuthorizationElRef {
        DataCloudRunV2ServiceBinaryAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceBinaryAuthorizationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `breakglass_justification` after provisioning.\n"]
    pub fn breakglass_justification(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.breakglass_justification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
    #[doc = "Get a reference to the value of field `use_default` after provisioning.\n"]
    pub fn use_default(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_default", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceBuildConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    base_image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_automatic_updates: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_variables: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    function_target: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_pool: Option<PrimField<String>>,
}
impl DataCloudRunV2ServiceBuildConfigEl {
    #[doc = "Set the field `base_image`.\n"]
    pub fn set_base_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.base_image = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_automatic_updates`.\n"]
    pub fn set_enable_automatic_updates(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_automatic_updates = Some(v.into());
        self
    }
    #[doc = "Set the field `environment_variables`.\n"]
    pub fn set_environment_variables(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.environment_variables = Some(v.into());
        self
    }
    #[doc = "Set the field `function_target`.\n"]
    pub fn set_function_target(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.function_target = Some(v.into());
        self
    }
    #[doc = "Set the field `image_uri`.\n"]
    pub fn set_image_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `source_location`.\n"]
    pub fn set_source_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_location = Some(v.into());
        self
    }
    #[doc = "Set the field `worker_pool`.\n"]
    pub fn set_worker_pool(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.worker_pool = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceBuildConfigEl {
    type O = BlockAssignable<DataCloudRunV2ServiceBuildConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceBuildConfigEl {}
impl BuildDataCloudRunV2ServiceBuildConfigEl {
    pub fn build(self) -> DataCloudRunV2ServiceBuildConfigEl {
        DataCloudRunV2ServiceBuildConfigEl {
            base_image: core::default::Default::default(),
            enable_automatic_updates: core::default::Default::default(),
            environment_variables: core::default::Default::default(),
            function_target: core::default::Default::default(),
            image_uri: core::default::Default::default(),
            name: core::default::Default::default(),
            service_account: core::default::Default::default(),
            source_location: core::default::Default::default(),
            worker_pool: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceBuildConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceBuildConfigElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2ServiceBuildConfigElRef {
        DataCloudRunV2ServiceBuildConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceBuildConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `base_image` after provisioning.\n"]
    pub fn base_image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.base_image", self.base))
    }
    #[doc = "Get a reference to the value of field `enable_automatic_updates` after provisioning.\n"]
    pub fn enable_automatic_updates(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_automatic_updates", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\n"]
    pub fn environment_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.environment_variables", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `function_target` after provisioning.\n"]
    pub fn function_target(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.function_target", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_uri` after provisioning.\n"]
    pub fn image_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_location` after provisioning.\n"]
    pub fn source_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_location", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `worker_pool` after provisioning.\n"]
    pub fn worker_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.worker_pool", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceConditionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_transition_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision_reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    severity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataCloudRunV2ServiceConditionsEl {
    #[doc = "Set the field `execution_reason`.\n"]
    pub fn set_execution_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.execution_reason = Some(v.into());
        self
    }
    #[doc = "Set the field `last_transition_time`.\n"]
    pub fn set_last_transition_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_transition_time = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reason = Some(v.into());
        self
    }
    #[doc = "Set the field `revision_reason`.\n"]
    pub fn set_revision_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.revision_reason = Some(v.into());
        self
    }
    #[doc = "Set the field `severity`.\n"]
    pub fn set_severity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.severity = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceConditionsEl {
    type O = BlockAssignable<DataCloudRunV2ServiceConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceConditionsEl {}
impl BuildDataCloudRunV2ServiceConditionsEl {
    pub fn build(self) -> DataCloudRunV2ServiceConditionsEl {
        DataCloudRunV2ServiceConditionsEl {
            execution_reason: core::default::Default::default(),
            last_transition_time: core::default::Default::default(),
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
            revision_reason: core::default::Default::default(),
            severity: core::default::Default::default(),
            state: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceConditionsElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2ServiceConditionsElRef {
        DataCloudRunV2ServiceConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `execution_reason` after provisioning.\n"]
    pub fn execution_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_reason", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `last_transition_time` after provisioning.\n"]
    pub fn last_transition_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_transition_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }
    #[doc = "Get a reference to the value of field `revision_reason` after provisioning.\n"]
    pub fn revision_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revision_reason", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `severity` after provisioning.\n"]
    pub fn severity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.severity", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceMultiRegionSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_region_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regions: Option<ListField<PrimField<String>>>,
}
impl DataCloudRunV2ServiceMultiRegionSettingsEl {
    #[doc = "Set the field `multi_region_id`.\n"]
    pub fn set_multi_region_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.multi_region_id = Some(v.into());
        self
    }
    #[doc = "Set the field `regions`.\n"]
    pub fn set_regions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.regions = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceMultiRegionSettingsEl {
    type O = BlockAssignable<DataCloudRunV2ServiceMultiRegionSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceMultiRegionSettingsEl {}
impl BuildDataCloudRunV2ServiceMultiRegionSettingsEl {
    pub fn build(self) -> DataCloudRunV2ServiceMultiRegionSettingsEl {
        DataCloudRunV2ServiceMultiRegionSettingsEl {
            multi_region_id: core::default::Default::default(),
            regions: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceMultiRegionSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceMultiRegionSettingsElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2ServiceMultiRegionSettingsElRef {
        DataCloudRunV2ServiceMultiRegionSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceMultiRegionSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `multi_region_id` after provisioning.\n"]
    pub fn multi_region_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multi_region_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceScalingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    manual_instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scaling_mode: Option<PrimField<String>>,
}
impl DataCloudRunV2ServiceScalingEl {
    #[doc = "Set the field `manual_instance_count`.\n"]
    pub fn set_manual_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.manual_instance_count = Some(v.into());
        self
    }
    #[doc = "Set the field `max_instance_count`.\n"]
    pub fn set_max_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_instance_count = Some(v.into());
        self
    }
    #[doc = "Set the field `min_instance_count`.\n"]
    pub fn set_min_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_instance_count = Some(v.into());
        self
    }
    #[doc = "Set the field `scaling_mode`.\n"]
    pub fn set_scaling_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scaling_mode = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceScalingEl {
    type O = BlockAssignable<DataCloudRunV2ServiceScalingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceScalingEl {}
impl BuildDataCloudRunV2ServiceScalingEl {
    pub fn build(self) -> DataCloudRunV2ServiceScalingEl {
        DataCloudRunV2ServiceScalingEl {
            manual_instance_count: core::default::Default::default(),
            max_instance_count: core::default::Default::default(),
            min_instance_count: core::default::Default::default(),
            scaling_mode: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceScalingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceScalingElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2ServiceScalingElRef {
        DataCloudRunV2ServiceScalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceScalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `manual_instance_count` after provisioning.\n"]
    pub fn manual_instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.manual_instance_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_instance_count` after provisioning.\n"]
    pub fn max_instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_instance_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_instance_count` after provisioning.\n"]
    pub fn min_instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_instance_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scaling_mode` after provisioning.\n"]
    pub fn scaling_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scaling_mode", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElBuildInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    function_target: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_location: Option<PrimField<String>>,
}
impl DataCloudRunV2ServiceTemplateElContainersElBuildInfoEl {
    #[doc = "Set the field `function_target`.\n"]
    pub fn set_function_target(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.function_target = Some(v.into());
        self
    }
    #[doc = "Set the field `source_location`.\n"]
    pub fn set_source_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_location = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElContainersElBuildInfoEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElBuildInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElContainersElBuildInfoEl {}
impl BuildDataCloudRunV2ServiceTemplateElContainersElBuildInfoEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElBuildInfoEl {
        DataCloudRunV2ServiceTemplateElContainersElBuildInfoEl {
            function_target: core::default::Default::default(),
            source_location: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElContainersElBuildInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElContainersElBuildInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElBuildInfoElRef {
        DataCloudRunV2ServiceTemplateElContainersElBuildInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElContainersElBuildInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `function_target` after provisioning.\n"]
    pub fn function_target(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.function_target", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_location` after provisioning.\n"]
    pub fn source_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_location", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl
{
    type O = BlockAssignable<
        DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {}
impl BuildDataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    pub fn build(
        self,
    ) -> DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
        DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
            secret: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
        DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_key_ref: Option<
        ListField<DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl>,
    >,
}
impl DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {
    #[doc = "Set the field `secret_key_ref`.\n"]
    pub fn set_secret_key_ref(
        mut self,
        v: impl Into<
            ListField<DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl>,
        >,
    ) -> Self {
        self.secret_key_ref = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {}
impl BuildDataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {
        DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {
            secret_key_ref: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElRef {
        DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `secret_key_ref` after provisioning.\n"]
    pub fn secret_key_ref(
        &self,
    ) -> ListRef<DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secret_key_ref", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElEnvEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_source: Option<ListField<DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl>>,
}
impl DataCloudRunV2ServiceTemplateElContainersElEnvEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
    #[doc = "Set the field `value_source`.\n"]
    pub fn set_value_source(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl>>,
    ) -> Self {
        self.value_source = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElContainersElEnvEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElEnvEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElContainersElEnvEl {}
impl BuildDataCloudRunV2ServiceTemplateElContainersElEnvEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElEnvEl {
        DataCloudRunV2ServiceTemplateElContainersElEnvEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
            value_source: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElContainersElEnvElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElContainersElEnvElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElEnvElRef {
        DataCloudRunV2ServiceTemplateElContainersElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElContainersElEnvElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
    #[doc = "Get a reference to the value of field `value_source` after provisioning.\n"]
    pub fn value_source(
        &self,
    ) -> ListRef<DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.value_source", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
}
impl DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {}
impl BuildDataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {
        DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {
            port: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcElRef {
        DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl
{
    type O = BlockAssignable<
        DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
}
impl BuildDataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
    pub fn build(
        self,
    ) -> DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
        DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
        DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_headers: Option<
        ListField<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {
    #[doc = "Set the field `http_headers`.\n"]
    pub fn set_http_headers(
        mut self,
        v: impl Into<
            ListField<
                DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl,
            >,
        >,
    ) -> Self {
        self.http_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {}
impl BuildDataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {
        DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {
            http_headers: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElRef {
        DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `http_headers` after provisioning.\n"]
    pub fn http_headers(
        &self,
    ) -> ListRef<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.http_headers", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketEl {}
impl BuildDataCloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketEl {
        DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketEl {
            port: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketElRef {
        DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElLivenessProbeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc: Option<ListField<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_get:
        Option<ListField<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_delay_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_socket:
        Option<ListField<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_seconds: Option<PrimField<f64>>,
}
impl DataCloudRunV2ServiceTemplateElContainersElLivenessProbeEl {
    #[doc = "Set the field `failure_threshold`.\n"]
    pub fn set_failure_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.failure_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl>>,
    ) -> Self {
        self.grpc = Some(v.into());
        self
    }
    #[doc = "Set the field `http_get`.\n"]
    pub fn set_http_get(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl>>,
    ) -> Self {
        self.http_get = Some(v.into());
        self
    }
    #[doc = "Set the field `initial_delay_seconds`.\n"]
    pub fn set_initial_delay_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.initial_delay_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `period_seconds`.\n"]
    pub fn set_period_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.period_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `tcp_socket`.\n"]
    pub fn set_tcp_socket(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketEl>>,
    ) -> Self {
        self.tcp_socket = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout_seconds`.\n"]
    pub fn set_timeout_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_seconds = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElContainersElLivenessProbeEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElContainersElLivenessProbeEl {}
impl BuildDataCloudRunV2ServiceTemplateElContainersElLivenessProbeEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElLivenessProbeEl {
        DataCloudRunV2ServiceTemplateElContainersElLivenessProbeEl {
            failure_threshold: core::default::Default::default(),
            grpc: core::default::Default::default(),
            http_get: core::default::Default::default(),
            initial_delay_seconds: core::default::Default::default(),
            period_seconds: core::default::Default::default(),
            tcp_socket: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElRef {
        DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `failure_threshold` after provisioning.\n"]
    pub fn failure_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failure_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(
        &self,
    ) -> ListRef<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }
    #[doc = "Get a reference to the value of field `http_get` after provisioning.\n"]
    pub fn http_get(
        &self,
    ) -> ListRef<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_get", self.base))
    }
    #[doc = "Get a reference to the value of field `initial_delay_seconds` after provisioning.\n"]
    pub fn initial_delay_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_delay_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `period_seconds` after provisioning.\n"]
    pub fn period_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.period_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tcp_socket` after provisioning.\n"]
    pub fn tcp_socket(
        &self,
    ) -> ListRef<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tcp_socket", self.base))
    }
    #[doc = "Get a reference to the value of field `timeout_seconds` after provisioning.\n"]
    pub fn timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_seconds", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElPortsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataCloudRunV2ServiceTemplateElContainersElPortsEl {
    #[doc = "Set the field `container_port`.\n"]
    pub fn set_container_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.container_port = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElContainersElPortsEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElPortsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElContainersElPortsEl {}
impl BuildDataCloudRunV2ServiceTemplateElContainersElPortsEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElPortsEl {
        DataCloudRunV2ServiceTemplateElContainersElPortsEl {
            container_port: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElContainersElPortsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElContainersElPortsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElPortsElRef {
        DataCloudRunV2ServiceTemplateElContainersElPortsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElContainersElPortsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container_port` after provisioning.\n"]
    pub fn container_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_port", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
}
impl DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcEl {}
impl BuildDataCloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcEl {
        DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcEl {
            port: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcElRef {
        DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetEl {
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetEl {}
impl BuildDataCloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetEl {
        DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetEl {
            path: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetElRef {
        DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElReadinessProbeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc: Option<ListField<DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_get:
        Option<ListField<DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_seconds: Option<PrimField<f64>>,
}
impl DataCloudRunV2ServiceTemplateElContainersElReadinessProbeEl {
    #[doc = "Set the field `failure_threshold`.\n"]
    pub fn set_failure_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.failure_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcEl>>,
    ) -> Self {
        self.grpc = Some(v.into());
        self
    }
    #[doc = "Set the field `http_get`.\n"]
    pub fn set_http_get(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetEl>>,
    ) -> Self {
        self.http_get = Some(v.into());
        self
    }
    #[doc = "Set the field `period_seconds`.\n"]
    pub fn set_period_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.period_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `success_threshold`.\n"]
    pub fn set_success_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.success_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout_seconds`.\n"]
    pub fn set_timeout_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_seconds = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElContainersElReadinessProbeEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElReadinessProbeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElContainersElReadinessProbeEl {}
impl BuildDataCloudRunV2ServiceTemplateElContainersElReadinessProbeEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElReadinessProbeEl {
        DataCloudRunV2ServiceTemplateElContainersElReadinessProbeEl {
            failure_threshold: core::default::Default::default(),
            grpc: core::default::Default::default(),
            http_get: core::default::Default::default(),
            period_seconds: core::default::Default::default(),
            success_threshold: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElRef {
        DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `failure_threshold` after provisioning.\n"]
    pub fn failure_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failure_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(
        &self,
    ) -> ListRef<DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }
    #[doc = "Get a reference to the value of field `http_get` after provisioning.\n"]
    pub fn http_get(
        &self,
    ) -> ListRef<DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_get", self.base))
    }
    #[doc = "Get a reference to the value of field `period_seconds` after provisioning.\n"]
    pub fn period_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.period_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `success_threshold` after provisioning.\n"]
    pub fn success_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.success_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timeout_seconds` after provisioning.\n"]
    pub fn timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_seconds", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_idle: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limits: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    startup_cpu_boost: Option<PrimField<bool>>,
}
impl DataCloudRunV2ServiceTemplateElContainersElResourcesEl {
    #[doc = "Set the field `cpu_idle`.\n"]
    pub fn set_cpu_idle(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.cpu_idle = Some(v.into());
        self
    }
    #[doc = "Set the field `limits`.\n"]
    pub fn set_limits(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.limits = Some(v.into());
        self
    }
    #[doc = "Set the field `startup_cpu_boost`.\n"]
    pub fn set_startup_cpu_boost(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.startup_cpu_boost = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElContainersElResourcesEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElContainersElResourcesEl {}
impl BuildDataCloudRunV2ServiceTemplateElContainersElResourcesEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElResourcesEl {
        DataCloudRunV2ServiceTemplateElContainersElResourcesEl {
            cpu_idle: core::default::Default::default(),
            limits: core::default::Default::default(),
            startup_cpu_boost: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElContainersElResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElContainersElResourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElResourcesElRef {
        DataCloudRunV2ServiceTemplateElContainersElResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElContainersElResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cpu_idle` after provisioning.\n"]
    pub fn cpu_idle(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_idle", self.base))
    }
    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }
    #[doc = "Get a reference to the value of field `startup_cpu_boost` after provisioning.\n"]
    pub fn startup_cpu_boost(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.startup_cpu_boost", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
}
impl DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {}
impl BuildDataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {
        DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {
            port: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcElRef {
        DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl
{
    type O = BlockAssignable<
        DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {}
impl BuildDataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    pub fn build(
        self,
    ) -> DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
        DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
        DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_headers: Option<
        ListField<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {
    #[doc = "Set the field `http_headers`.\n"]
    pub fn set_http_headers(
        mut self,
        v: impl Into<
            ListField<
                DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl,
            >,
        >,
    ) -> Self {
        self.http_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {}
impl BuildDataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {
        DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {
            http_headers: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElRef {
        DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `http_headers` after provisioning.\n"]
    pub fn http_headers(
        &self,
    ) -> ListRef<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.http_headers", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl {}
impl BuildDataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl {
        DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl {
            port: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketElRef {
        DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElStartupProbeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc: Option<ListField<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_get: Option<ListField<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_delay_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_socket:
        Option<ListField<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_seconds: Option<PrimField<f64>>,
}
impl DataCloudRunV2ServiceTemplateElContainersElStartupProbeEl {
    #[doc = "Set the field `failure_threshold`.\n"]
    pub fn set_failure_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.failure_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl>>,
    ) -> Self {
        self.grpc = Some(v.into());
        self
    }
    #[doc = "Set the field `http_get`.\n"]
    pub fn set_http_get(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl>>,
    ) -> Self {
        self.http_get = Some(v.into());
        self
    }
    #[doc = "Set the field `initial_delay_seconds`.\n"]
    pub fn set_initial_delay_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.initial_delay_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `period_seconds`.\n"]
    pub fn set_period_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.period_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `tcp_socket`.\n"]
    pub fn set_tcp_socket(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl>>,
    ) -> Self {
        self.tcp_socket = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout_seconds`.\n"]
    pub fn set_timeout_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_seconds = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElContainersElStartupProbeEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElStartupProbeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElContainersElStartupProbeEl {}
impl BuildDataCloudRunV2ServiceTemplateElContainersElStartupProbeEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElStartupProbeEl {
        DataCloudRunV2ServiceTemplateElContainersElStartupProbeEl {
            failure_threshold: core::default::Default::default(),
            grpc: core::default::Default::default(),
            http_get: core::default::Default::default(),
            initial_delay_seconds: core::default::Default::default(),
            period_seconds: core::default::Default::default(),
            tcp_socket: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElContainersElStartupProbeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElContainersElStartupProbeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElStartupProbeElRef {
        DataCloudRunV2ServiceTemplateElContainersElStartupProbeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElContainersElStartupProbeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `failure_threshold` after provisioning.\n"]
    pub fn failure_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failure_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(
        &self,
    ) -> ListRef<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }
    #[doc = "Get a reference to the value of field `http_get` after provisioning.\n"]
    pub fn http_get(
        &self,
    ) -> ListRef<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_get", self.base))
    }
    #[doc = "Get a reference to the value of field `initial_delay_seconds` after provisioning.\n"]
    pub fn initial_delay_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_delay_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `period_seconds` after provisioning.\n"]
    pub fn period_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.period_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tcp_socket` after provisioning.\n"]
    pub fn tcp_socket(
        &self,
    ) -> ListRef<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tcp_socket", self.base))
    }
    #[doc = "Get a reference to the value of field `timeout_seconds` after provisioning.\n"]
    pub fn timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_seconds", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElVolumeMountsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mount_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_path: Option<PrimField<String>>,
}
impl DataCloudRunV2ServiceTemplateElContainersElVolumeMountsEl {
    #[doc = "Set the field `mount_path`.\n"]
    pub fn set_mount_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mount_path = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `sub_path`.\n"]
    pub fn set_sub_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sub_path = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElContainersElVolumeMountsEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElVolumeMountsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElContainersElVolumeMountsEl {}
impl BuildDataCloudRunV2ServiceTemplateElContainersElVolumeMountsEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElVolumeMountsEl {
        DataCloudRunV2ServiceTemplateElContainersElVolumeMountsEl {
            mount_path: core::default::Default::default(),
            name: core::default::Default::default(),
            sub_path: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElContainersElVolumeMountsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElContainersElVolumeMountsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElVolumeMountsElRef {
        DataCloudRunV2ServiceTemplateElContainersElVolumeMountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElContainersElVolumeMountsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mount_path` after provisioning.\n"]
    pub fn mount_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `sub_path` after provisioning.\n"]
    pub fn sub_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sub_path", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base_image_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_info: Option<ListField<DataCloudRunV2ServiceTemplateElContainersElBuildInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    depends_on: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<SetField<DataCloudRunV2ServiceTemplateElContainersElEnvEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    liveness_probe: Option<ListField<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<ListField<DataCloudRunV2ServiceTemplateElContainersElPortsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    readiness_probe: Option<ListField<DataCloudRunV2ServiceTemplateElContainersElReadinessProbeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<ListField<DataCloudRunV2ServiceTemplateElContainersElResourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    startup_probe: Option<ListField<DataCloudRunV2ServiceTemplateElContainersElStartupProbeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_mounts: Option<ListField<DataCloudRunV2ServiceTemplateElContainersElVolumeMountsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    working_dir: Option<PrimField<String>>,
}
impl DataCloudRunV2ServiceTemplateElContainersEl {
    #[doc = "Set the field `args`.\n"]
    pub fn set_args(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
        self
    }
    #[doc = "Set the field `base_image_uri`.\n"]
    pub fn set_base_image_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.base_image_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `build_info`.\n"]
    pub fn set_build_info(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElContainersElBuildInfoEl>>,
    ) -> Self {
        self.build_info = Some(v.into());
        self
    }
    #[doc = "Set the field `command`.\n"]
    pub fn set_command(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.command = Some(v.into());
        self
    }
    #[doc = "Set the field `depends_on`.\n"]
    pub fn set_depends_on(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.depends_on = Some(v.into());
        self
    }
    #[doc = "Set the field `env`.\n"]
    pub fn set_env(
        mut self,
        v: impl Into<SetField<DataCloudRunV2ServiceTemplateElContainersElEnvEl>>,
    ) -> Self {
        self.env = Some(v.into());
        self
    }
    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image = Some(v.into());
        self
    }
    #[doc = "Set the field `liveness_probe`.\n"]
    pub fn set_liveness_probe(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeEl>>,
    ) -> Self {
        self.liveness_probe = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `ports`.\n"]
    pub fn set_ports(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElContainersElPortsEl>>,
    ) -> Self {
        self.ports = Some(v.into());
        self
    }
    #[doc = "Set the field `readiness_probe`.\n"]
    pub fn set_readiness_probe(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElContainersElReadinessProbeEl>>,
    ) -> Self {
        self.readiness_probe = Some(v.into());
        self
    }
    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElContainersElResourcesEl>>,
    ) -> Self {
        self.resources = Some(v.into());
        self
    }
    #[doc = "Set the field `startup_probe`.\n"]
    pub fn set_startup_probe(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElContainersElStartupProbeEl>>,
    ) -> Self {
        self.startup_probe = Some(v.into());
        self
    }
    #[doc = "Set the field `volume_mounts`.\n"]
    pub fn set_volume_mounts(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElContainersElVolumeMountsEl>>,
    ) -> Self {
        self.volume_mounts = Some(v.into());
        self
    }
    #[doc = "Set the field `working_dir`.\n"]
    pub fn set_working_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.working_dir = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElContainersEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElContainersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElContainersEl {}
impl BuildDataCloudRunV2ServiceTemplateElContainersEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersEl {
        DataCloudRunV2ServiceTemplateElContainersEl {
            args: core::default::Default::default(),
            base_image_uri: core::default::Default::default(),
            build_info: core::default::Default::default(),
            command: core::default::Default::default(),
            depends_on: core::default::Default::default(),
            env: core::default::Default::default(),
            image: core::default::Default::default(),
            liveness_probe: core::default::Default::default(),
            name: core::default::Default::default(),
            ports: core::default::Default::default(),
            readiness_probe: core::default::Default::default(),
            resources: core::default::Default::default(),
            startup_probe: core::default::Default::default(),
            volume_mounts: core::default::Default::default(),
            working_dir: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElContainersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElContainersElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2ServiceTemplateElContainersElRef {
        DataCloudRunV2ServiceTemplateElContainersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElContainersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `args` after provisioning.\n"]
    pub fn args(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }
    #[doc = "Get a reference to the value of field `base_image_uri` after provisioning.\n"]
    pub fn base_image_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_image_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `build_info` after provisioning.\n"]
    pub fn build_info(&self) -> ListRef<DataCloudRunV2ServiceTemplateElContainersElBuildInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.build_info", self.base))
    }
    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }
    #[doc = "Get a reference to the value of field `depends_on` after provisioning.\n"]
    pub fn depends_on(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.depends_on", self.base))
    }
    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> SetRef<DataCloudRunV2ServiceTemplateElContainersElEnvElRef> {
        SetRef::new(self.shared().clone(), format!("{}.env", self.base))
    }
    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
    #[doc = "Get a reference to the value of field `liveness_probe` after provisioning.\n"]
    pub fn liveness_probe(
        &self,
    ) -> ListRef<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.liveness_probe", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> ListRef<DataCloudRunV2ServiceTemplateElContainersElPortsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }
    #[doc = "Get a reference to the value of field `readiness_probe` after provisioning.\n"]
    pub fn readiness_probe(
        &self,
    ) -> ListRef<DataCloudRunV2ServiceTemplateElContainersElReadinessProbeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.readiness_probe", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> ListRef<DataCloudRunV2ServiceTemplateElContainersElResourcesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
    #[doc = "Get a reference to the value of field `startup_probe` after provisioning.\n"]
    pub fn startup_probe(
        &self,
    ) -> ListRef<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.startup_probe", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `volume_mounts` after provisioning.\n"]
    pub fn volume_mounts(
        &self,
    ) -> ListRef<DataCloudRunV2ServiceTemplateElContainersElVolumeMountsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.volume_mounts", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `working_dir` after provisioning.\n"]
    pub fn working_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.working_dir", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElNodeSelectorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator: Option<PrimField<String>>,
}
impl DataCloudRunV2ServiceTemplateElNodeSelectorEl {
    #[doc = "Set the field `accelerator`.\n"]
    pub fn set_accelerator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.accelerator = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElNodeSelectorEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElNodeSelectorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElNodeSelectorEl {}
impl BuildDataCloudRunV2ServiceTemplateElNodeSelectorEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElNodeSelectorEl {
        DataCloudRunV2ServiceTemplateElNodeSelectorEl {
            accelerator: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElNodeSelectorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElNodeSelectorElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2ServiceTemplateElNodeSelectorElRef {
        DataCloudRunV2ServiceTemplateElNodeSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElNodeSelectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accelerator` after provisioning.\n"]
    pub fn accelerator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.accelerator", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElScalingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_instance_count: Option<PrimField<f64>>,
}
impl DataCloudRunV2ServiceTemplateElScalingEl {
    #[doc = "Set the field `max_instance_count`.\n"]
    pub fn set_max_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_instance_count = Some(v.into());
        self
    }
    #[doc = "Set the field `min_instance_count`.\n"]
    pub fn set_min_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_instance_count = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElScalingEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElScalingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElScalingEl {}
impl BuildDataCloudRunV2ServiceTemplateElScalingEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElScalingEl {
        DataCloudRunV2ServiceTemplateElScalingEl {
            max_instance_count: core::default::Default::default(),
            min_instance_count: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElScalingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElScalingElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2ServiceTemplateElScalingElRef {
        DataCloudRunV2ServiceTemplateElScalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElScalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_instance_count` after provisioning.\n"]
    pub fn max_instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_instance_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_instance_count` after provisioning.\n"]
    pub fn min_instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_instance_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instances: Option<SetField<PrimField<String>>>,
}
impl DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl {
    #[doc = "Set the field `instances`.\n"]
    pub fn set_instances(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.instances = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl {}
impl BuildDataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl {
        DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl {
            instances: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceElRef {
        DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.instances", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElVolumesElEmptyDirEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    medium: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size_limit: Option<PrimField<String>>,
}
impl DataCloudRunV2ServiceTemplateElVolumesElEmptyDirEl {
    #[doc = "Set the field `medium`.\n"]
    pub fn set_medium(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.medium = Some(v.into());
        self
    }
    #[doc = "Set the field `size_limit`.\n"]
    pub fn set_size_limit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.size_limit = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElVolumesElEmptyDirEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElVolumesElEmptyDirEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElVolumesElEmptyDirEl {}
impl BuildDataCloudRunV2ServiceTemplateElVolumesElEmptyDirEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElVolumesElEmptyDirEl {
        DataCloudRunV2ServiceTemplateElVolumesElEmptyDirEl {
            medium: core::default::Default::default(),
            size_limit: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElVolumesElEmptyDirElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElVolumesElEmptyDirElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2ServiceTemplateElVolumesElEmptyDirElRef {
        DataCloudRunV2ServiceTemplateElVolumesElEmptyDirElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElVolumesElEmptyDirElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `medium` after provisioning.\n"]
    pub fn medium(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.medium", self.base))
    }
    #[doc = "Get a reference to the value of field `size_limit` after provisioning.\n"]
    pub fn size_limit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_limit", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElVolumesElGcsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mount_options: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<PrimField<bool>>,
}
impl DataCloudRunV2ServiceTemplateElVolumesElGcsEl {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket = Some(v.into());
        self
    }
    #[doc = "Set the field `mount_options`.\n"]
    pub fn set_mount_options(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.mount_options = Some(v.into());
        self
    }
    #[doc = "Set the field `read_only`.\n"]
    pub fn set_read_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.read_only = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElVolumesElGcsEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElVolumesElGcsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElVolumesElGcsEl {}
impl BuildDataCloudRunV2ServiceTemplateElVolumesElGcsEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElVolumesElGcsEl {
        DataCloudRunV2ServiceTemplateElVolumesElGcsEl {
            bucket: core::default::Default::default(),
            mount_options: core::default::Default::default(),
            read_only: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElVolumesElGcsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElVolumesElGcsElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2ServiceTemplateElVolumesElGcsElRef {
        DataCloudRunV2ServiceTemplateElVolumesElGcsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElVolumesElGcsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `mount_options` after provisioning.\n"]
    pub fn mount_options(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mount_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `read_only` after provisioning.\n"]
    pub fn read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElVolumesElNfsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server: Option<PrimField<String>>,
}
impl DataCloudRunV2ServiceTemplateElVolumesElNfsEl {
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `read_only`.\n"]
    pub fn set_read_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.read_only = Some(v.into());
        self
    }
    #[doc = "Set the field `server`.\n"]
    pub fn set_server(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.server = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElVolumesElNfsEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElVolumesElNfsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElVolumesElNfsEl {}
impl BuildDataCloudRunV2ServiceTemplateElVolumesElNfsEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElVolumesElNfsEl {
        DataCloudRunV2ServiceTemplateElVolumesElNfsEl {
            path: core::default::Default::default(),
            read_only: core::default::Default::default(),
            server: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElVolumesElNfsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElVolumesElNfsElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2ServiceTemplateElVolumesElNfsElRef {
        DataCloudRunV2ServiceTemplateElVolumesElNfsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElVolumesElNfsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `read_only` after provisioning.\n"]
    pub fn read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }
    #[doc = "Get a reference to the value of field `server` after provisioning.\n"]
    pub fn server(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.mode = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {}
impl BuildDataCloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {
        DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {
            mode: core::default::Default::default(),
            path: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsElRef {
        DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElVolumesElSecretEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_mode: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<ListField<DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<PrimField<String>>,
}
impl DataCloudRunV2ServiceTemplateElVolumesElSecretEl {
    #[doc = "Set the field `default_mode`.\n"]
    pub fn set_default_mode(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsEl>>,
    ) -> Self {
        self.items = Some(v.into());
        self
    }
    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElVolumesElSecretEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElVolumesElSecretEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElVolumesElSecretEl {}
impl BuildDataCloudRunV2ServiceTemplateElVolumesElSecretEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElVolumesElSecretEl {
        DataCloudRunV2ServiceTemplateElVolumesElSecretEl {
            default_mode: core::default::Default::default(),
            items: core::default::Default::default(),
            secret: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElVolumesElSecretElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElVolumesElSecretElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2ServiceTemplateElVolumesElSecretElRef {
        DataCloudRunV2ServiceTemplateElVolumesElSecretElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElVolumesElSecretElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_mode` after provisioning.\n"]
    pub fn default_mode(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> ListRef<DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElVolumesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_sql_instance:
        Option<ListField<DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    empty_dir: Option<ListField<DataCloudRunV2ServiceTemplateElVolumesElEmptyDirEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs: Option<ListField<DataCloudRunV2ServiceTemplateElVolumesElGcsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nfs: Option<ListField<DataCloudRunV2ServiceTemplateElVolumesElNfsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<ListField<DataCloudRunV2ServiceTemplateElVolumesElSecretEl>>,
}
impl DataCloudRunV2ServiceTemplateElVolumesEl {
    #[doc = "Set the field `cloud_sql_instance`.\n"]
    pub fn set_cloud_sql_instance(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl>>,
    ) -> Self {
        self.cloud_sql_instance = Some(v.into());
        self
    }
    #[doc = "Set the field `empty_dir`.\n"]
    pub fn set_empty_dir(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElVolumesElEmptyDirEl>>,
    ) -> Self {
        self.empty_dir = Some(v.into());
        self
    }
    #[doc = "Set the field `gcs`.\n"]
    pub fn set_gcs(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElVolumesElGcsEl>>,
    ) -> Self {
        self.gcs = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `nfs`.\n"]
    pub fn set_nfs(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElVolumesElNfsEl>>,
    ) -> Self {
        self.nfs = Some(v.into());
        self
    }
    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElVolumesElSecretEl>>,
    ) -> Self {
        self.secret = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElVolumesEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElVolumesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElVolumesEl {}
impl BuildDataCloudRunV2ServiceTemplateElVolumesEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElVolumesEl {
        DataCloudRunV2ServiceTemplateElVolumesEl {
            cloud_sql_instance: core::default::Default::default(),
            empty_dir: core::default::Default::default(),
            gcs: core::default::Default::default(),
            name: core::default::Default::default(),
            nfs: core::default::Default::default(),
            secret: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElVolumesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElVolumesElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2ServiceTemplateElVolumesElRef {
        DataCloudRunV2ServiceTemplateElVolumesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElVolumesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_sql_instance` after provisioning.\n"]
    pub fn cloud_sql_instance(
        &self,
    ) -> ListRef<DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_sql_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `empty_dir` after provisioning.\n"]
    pub fn empty_dir(&self) -> ListRef<DataCloudRunV2ServiceTemplateElVolumesElEmptyDirElRef> {
        ListRef::new(self.shared().clone(), format!("{}.empty_dir", self.base))
    }
    #[doc = "Get a reference to the value of field `gcs` after provisioning.\n"]
    pub fn gcs(&self) -> ListRef<DataCloudRunV2ServiceTemplateElVolumesElGcsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gcs", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `nfs` after provisioning.\n"]
    pub fn nfs(&self) -> ListRef<DataCloudRunV2ServiceTemplateElVolumesElNfsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.nfs", self.base))
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> ListRef<DataCloudRunV2ServiceTemplateElVolumesElSecretElRef> {
        ListRef::new(self.shared().clone(), format!("{}.secret", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<PrimField<String>>>,
}
impl DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\n"]
    pub fn set_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {}
impl BuildDataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {
        DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {
            network: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesElRef {
        DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\n"]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElVpcAccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connector: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interfaces:
        Option<ListField<DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl>>,
}
impl DataCloudRunV2ServiceTemplateElVpcAccessEl {
    #[doc = "Set the field `connector`.\n"]
    pub fn set_connector(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connector = Some(v.into());
        self
    }
    #[doc = "Set the field `egress`.\n"]
    pub fn set_egress(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.egress = Some(v.into());
        self
    }
    #[doc = "Set the field `network_interfaces`.\n"]
    pub fn set_network_interfaces(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl>>,
    ) -> Self {
        self.network_interfaces = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateElVpcAccessEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateElVpcAccessEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateElVpcAccessEl {}
impl BuildDataCloudRunV2ServiceTemplateElVpcAccessEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElVpcAccessEl {
        DataCloudRunV2ServiceTemplateElVpcAccessEl {
            connector: core::default::Default::default(),
            egress: core::default::Default::default(),
            network_interfaces: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElVpcAccessElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElVpcAccessElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2ServiceTemplateElVpcAccessElRef {
        DataCloudRunV2ServiceTemplateElVpcAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElVpcAccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connector` after provisioning.\n"]
    pub fn connector(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connector", self.base))
    }
    #[doc = "Get a reference to the value of field `egress` after provisioning.\n"]
    pub fn egress(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.egress", self.base))
    }
    #[doc = "Get a reference to the value of field `network_interfaces` after provisioning.\n"]
    pub fn network_interfaces(
        &self,
    ) -> ListRef<DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_interfaces", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    containers: Option<ListField<DataCloudRunV2ServiceTemplateElContainersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_environment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_zonal_redundancy_disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check_disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_instance_request_concurrency: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_selector: Option<ListField<DataCloudRunV2ServiceTemplateElNodeSelectorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scaling: Option<ListField<DataCloudRunV2ServiceTemplateElScalingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_affinity: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volumes: Option<ListField<DataCloudRunV2ServiceTemplateElVolumesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_access: Option<ListField<DataCloudRunV2ServiceTemplateElVpcAccessEl>>,
}
impl DataCloudRunV2ServiceTemplateEl {
    #[doc = "Set the field `annotations`.\n"]
    pub fn set_annotations(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `containers`.\n"]
    pub fn set_containers(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElContainersEl>>,
    ) -> Self {
        self.containers = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_key`.\n"]
    pub fn set_encryption_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_key = Some(v.into());
        self
    }
    #[doc = "Set the field `execution_environment`.\n"]
    pub fn set_execution_environment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.execution_environment = Some(v.into());
        self
    }
    #[doc = "Set the field `gpu_zonal_redundancy_disabled`.\n"]
    pub fn set_gpu_zonal_redundancy_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.gpu_zonal_redundancy_disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `health_check_disabled`.\n"]
    pub fn set_health_check_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.health_check_disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `max_instance_request_concurrency`.\n"]
    pub fn set_max_instance_request_concurrency(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_instance_request_concurrency = Some(v.into());
        self
    }
    #[doc = "Set the field `node_selector`.\n"]
    pub fn set_node_selector(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElNodeSelectorEl>>,
    ) -> Self {
        self.node_selector = Some(v.into());
        self
    }
    #[doc = "Set the field `revision`.\n"]
    pub fn set_revision(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.revision = Some(v.into());
        self
    }
    #[doc = "Set the field `scaling`.\n"]
    pub fn set_scaling(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElScalingEl>>,
    ) -> Self {
        self.scaling = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `session_affinity`.\n"]
    pub fn set_session_affinity(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.session_affinity = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `volumes`.\n"]
    pub fn set_volumes(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElVolumesEl>>,
    ) -> Self {
        self.volumes = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_access`.\n"]
    pub fn set_vpc_access(
        mut self,
        v: impl Into<ListField<DataCloudRunV2ServiceTemplateElVpcAccessEl>>,
    ) -> Self {
        self.vpc_access = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTemplateEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTemplateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTemplateEl {}
impl BuildDataCloudRunV2ServiceTemplateEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateEl {
        DataCloudRunV2ServiceTemplateEl {
            annotations: core::default::Default::default(),
            containers: core::default::Default::default(),
            encryption_key: core::default::Default::default(),
            execution_environment: core::default::Default::default(),
            gpu_zonal_redundancy_disabled: core::default::Default::default(),
            health_check_disabled: core::default::Default::default(),
            labels: core::default::Default::default(),
            max_instance_request_concurrency: core::default::Default::default(),
            node_selector: core::default::Default::default(),
            revision: core::default::Default::default(),
            scaling: core::default::Default::default(),
            service_account: core::default::Default::default(),
            session_affinity: core::default::Default::default(),
            timeout: core::default::Default::default(),
            volumes: core::default::Default::default(),
            vpc_access: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTemplateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTemplateElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2ServiceTemplateElRef {
        DataCloudRunV2ServiceTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\n"]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.annotations", self.base))
    }
    #[doc = "Get a reference to the value of field `containers` after provisioning.\n"]
    pub fn containers(&self) -> ListRef<DataCloudRunV2ServiceTemplateElContainersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.containers", self.base))
    }
    #[doc = "Get a reference to the value of field `encryption_key` after provisioning.\n"]
    pub fn encryption_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `execution_environment` after provisioning.\n"]
    pub fn execution_environment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_environment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gpu_zonal_redundancy_disabled` after provisioning.\n"]
    pub fn gpu_zonal_redundancy_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gpu_zonal_redundancy_disabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `health_check_disabled` after provisioning.\n"]
    pub fn health_check_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_check_disabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `max_instance_request_concurrency` after provisioning.\n"]
    pub fn max_instance_request_concurrency(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_instance_request_concurrency", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_selector` after provisioning.\n"]
    pub fn node_selector(&self) -> ListRef<DataCloudRunV2ServiceTemplateElNodeSelectorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_selector", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }
    #[doc = "Get a reference to the value of field `scaling` after provisioning.\n"]
    pub fn scaling(&self) -> ListRef<DataCloudRunV2ServiceTemplateElScalingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scaling", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `session_affinity` after provisioning.\n"]
    pub fn session_affinity(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_affinity", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(&self) -> ListRef<DataCloudRunV2ServiceTemplateElVolumesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }
    #[doc = "Get a reference to the value of field `vpc_access` after provisioning.\n"]
    pub fn vpc_access(&self) -> ListRef<DataCloudRunV2ServiceTemplateElVpcAccessElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_access", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTerminalConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_transition_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision_reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    severity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataCloudRunV2ServiceTerminalConditionEl {
    #[doc = "Set the field `execution_reason`.\n"]
    pub fn set_execution_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.execution_reason = Some(v.into());
        self
    }
    #[doc = "Set the field `last_transition_time`.\n"]
    pub fn set_last_transition_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_transition_time = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reason = Some(v.into());
        self
    }
    #[doc = "Set the field `revision_reason`.\n"]
    pub fn set_revision_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.revision_reason = Some(v.into());
        self
    }
    #[doc = "Set the field `severity`.\n"]
    pub fn set_severity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.severity = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTerminalConditionEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTerminalConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTerminalConditionEl {}
impl BuildDataCloudRunV2ServiceTerminalConditionEl {
    pub fn build(self) -> DataCloudRunV2ServiceTerminalConditionEl {
        DataCloudRunV2ServiceTerminalConditionEl {
            execution_reason: core::default::Default::default(),
            last_transition_time: core::default::Default::default(),
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
            revision_reason: core::default::Default::default(),
            severity: core::default::Default::default(),
            state: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTerminalConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTerminalConditionElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2ServiceTerminalConditionElRef {
        DataCloudRunV2ServiceTerminalConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTerminalConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `execution_reason` after provisioning.\n"]
    pub fn execution_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_reason", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `last_transition_time` after provisioning.\n"]
    pub fn last_transition_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_transition_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }
    #[doc = "Get a reference to the value of field `revision_reason` after provisioning.\n"]
    pub fn revision_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revision_reason", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `severity` after provisioning.\n"]
    pub fn severity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.severity", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTrafficEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataCloudRunV2ServiceTrafficEl {
    #[doc = "Set the field `percent`.\n"]
    pub fn set_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percent = Some(v.into());
        self
    }
    #[doc = "Set the field `revision`.\n"]
    pub fn set_revision(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.revision = Some(v.into());
        self
    }
    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTrafficEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTrafficEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTrafficEl {}
impl BuildDataCloudRunV2ServiceTrafficEl {
    pub fn build(self) -> DataCloudRunV2ServiceTrafficEl {
        DataCloudRunV2ServiceTrafficEl {
            percent: core::default::Default::default(),
            revision: core::default::Default::default(),
            tag: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTrafficElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTrafficElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2ServiceTrafficElRef {
        DataCloudRunV2ServiceTrafficElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTrafficElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `percent` after provisioning.\n"]
    pub fn percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }
    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTrafficStatusesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl DataCloudRunV2ServiceTrafficStatusesEl {
    #[doc = "Set the field `percent`.\n"]
    pub fn set_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percent = Some(v.into());
        self
    }
    #[doc = "Set the field `revision`.\n"]
    pub fn set_revision(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.revision = Some(v.into());
        self
    }
    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2ServiceTrafficStatusesEl {
    type O = BlockAssignable<DataCloudRunV2ServiceTrafficStatusesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2ServiceTrafficStatusesEl {}
impl BuildDataCloudRunV2ServiceTrafficStatusesEl {
    pub fn build(self) -> DataCloudRunV2ServiceTrafficStatusesEl {
        DataCloudRunV2ServiceTrafficStatusesEl {
            percent: core::default::Default::default(),
            revision: core::default::Default::default(),
            tag: core::default::Default::default(),
            type_: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2ServiceTrafficStatusesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2ServiceTrafficStatusesElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2ServiceTrafficStatusesElRef {
        DataCloudRunV2ServiceTrafficStatusesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2ServiceTrafficStatusesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `percent` after provisioning.\n"]
    pub fn percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }
    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
