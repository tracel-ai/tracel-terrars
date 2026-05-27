use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataCloudRunV2WorkerPoolData {
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
struct DataCloudRunV2WorkerPool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudRunV2WorkerPoolData>,
}
#[derive(Clone)]
pub struct DataCloudRunV2WorkerPool(Rc<DataCloudRunV2WorkerPool_>);
impl DataCloudRunV2WorkerPool {
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
    #[doc = "Set the field `location`.\nThe location of the cloud run worker pool"]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected in new resources.\nAll system annotations in v1 now have a corresponding field in v2 WorkerPool.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\nSettings for the Binary Authorization feature."]
    pub fn binary_authorization(
        &self,
    ) -> ListRef<DataCloudRunV2WorkerPoolBinaryAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.binary_authorization", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\nThe Conditions of all other associated sub-resources. They contain additional diagnostics information in case the WorkerPool does not reach its Serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn conditions(&self) -> ListRef<DataCloudRunV2WorkerPoolConditionsElRef> {
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
    #[doc = "Get a reference to the value of field `custom_audiences` after provisioning.\nOne or more custom audiences that you want this worker pool to support. Specify each custom audience as the full URL in a string. The custom audiences are encoded in the token and used to authenticate requests.\nFor more information, see https://cloud.google.com/run/docs/configuring/custom-audiences."]
    pub fn custom_audiences(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_audiences", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the service. Defaults to true.\nWhen a'terraform destroy' or 'terraform apply' would delete the service,\nthe command will fail if this field is not set to false in Terraform state.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the WorkerPool will fail.\nWhen the field is set to false, deleting the WorkerPool is allowed."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-provided description of the WorkerPool. This field currently has a 512-character limit."]
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_split_statuses` after provisioning.\nDetailed status information for corresponding instance splits. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn instance_split_statuses(
        &self,
    ) -> ListRef<DataCloudRunV2WorkerPoolInstanceSplitStatusesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_split_statuses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_splits` after provisioning.\nSpecifies how to distribute instances over a collection of Revisions belonging to the WorkerPool. If instance split is empty or not provided, defaults to 100% instances assigned to the latest Ready Revision."]
    pub fn instance_splits(&self) -> ListRef<DataCloudRunV2WorkerPoolInstanceSplitsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_splits", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component,\nenvironment, state, etc. For more information, visit https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.\n\nCloud Run API v2 does not support labels with  'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system labels in v1 now have a corresponding field in v2 WorkerPool.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the cloud run worker pool"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the WorkerPool."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `observed_generation` after provisioning.\nThe generation of this WorkerPool currently serving traffic. See comments in reconciling for additional information on reconciliation process in Cloud Run. Please note that unlike v1, this is an int64 value. As with most Google APIs, its JSON representation will be a string instead of an integer."]
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
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nReturns true if the WorkerPool is currently being acted upon by the system to bring it into the desired state.\n\nWhen a new WorkerPool is created, or an existing one is updated, Cloud Run will asynchronously perform all necessary steps to bring the WorkerPool to the desired serving state. This process is called reconciliation. While reconciliation is in process, observedGeneration, latest_ready_revison, trafficStatuses, and uri will have transient values that might mismatch the intended state: Once reconciliation is over (and this field is false), there are two possible outcomes: reconciliation succeeded and the serving state matches the WorkerPool, or there was an error, and reconciliation failed. This state can be found in terminalCondition.state.\n\nIf reconciliation succeeded, the following fields will match: traffic and trafficStatuses, observedGeneration and generation, latestReadyRevision and latestCreatedRevision.\n\nIf reconciliation failed, trafficStatuses, observedGeneration, and latestReadyRevision will have the state of the last serving revision, or empty for newly created WorkerPools. Additional information on the failure can be found in terminalCondition and conditions."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scaling` after provisioning.\nScaling settings that apply to the worker pool."]
    pub fn scaling(&self) -> ListRef<DataCloudRunV2WorkerPoolScalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template` after provisioning.\nThe template used to create revisions for this WorkerPool."]
    pub fn template(&self) -> ListRef<DataCloudRunV2WorkerPoolTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.template", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terminal_condition` after provisioning.\nThe Condition of this WorkerPool, containing its readiness status, and detailed error information in case it did not reach a serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn terminal_condition(&self) -> ListRef<DataCloudRunV2WorkerPoolTerminalConditionElRef> {
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
}
impl Referable for DataCloudRunV2WorkerPool {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataCloudRunV2WorkerPool {}
impl ToListMappable for DataCloudRunV2WorkerPool {
    type O = ListRef<DataCloudRunV2WorkerPoolRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataCloudRunV2WorkerPool_ {
    fn extract_datasource_type(&self) -> String {
        "google_cloud_run_v2_worker_pool".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataCloudRunV2WorkerPool {
    pub tf_id: String,
    #[doc = "Name of the WorkerPool."]
    pub name: PrimField<String>,
}
impl BuildDataCloudRunV2WorkerPool {
    pub fn build(self, stack: &mut Stack) -> DataCloudRunV2WorkerPool {
        let out = DataCloudRunV2WorkerPool(Rc::new(DataCloudRunV2WorkerPool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCloudRunV2WorkerPoolData {
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
pub struct DataCloudRunV2WorkerPoolRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataCloudRunV2WorkerPoolRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected in new resources.\nAll system annotations in v1 now have a corresponding field in v2 WorkerPool.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\nSettings for the Binary Authorization feature."]
    pub fn binary_authorization(
        &self,
    ) -> ListRef<DataCloudRunV2WorkerPoolBinaryAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.binary_authorization", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\nThe Conditions of all other associated sub-resources. They contain additional diagnostics information in case the WorkerPool does not reach its Serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn conditions(&self) -> ListRef<DataCloudRunV2WorkerPoolConditionsElRef> {
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
    #[doc = "Get a reference to the value of field `custom_audiences` after provisioning.\nOne or more custom audiences that you want this worker pool to support. Specify each custom audience as the full URL in a string. The custom audiences are encoded in the token and used to authenticate requests.\nFor more information, see https://cloud.google.com/run/docs/configuring/custom-audiences."]
    pub fn custom_audiences(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_audiences", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the service. Defaults to true.\nWhen a'terraform destroy' or 'terraform apply' would delete the service,\nthe command will fail if this field is not set to false in Terraform state.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the WorkerPool will fail.\nWhen the field is set to false, deleting the WorkerPool is allowed."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-provided description of the WorkerPool. This field currently has a 512-character limit."]
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_split_statuses` after provisioning.\nDetailed status information for corresponding instance splits. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn instance_split_statuses(
        &self,
    ) -> ListRef<DataCloudRunV2WorkerPoolInstanceSplitStatusesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_split_statuses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_splits` after provisioning.\nSpecifies how to distribute instances over a collection of Revisions belonging to the WorkerPool. If instance split is empty or not provided, defaults to 100% instances assigned to the latest Ready Revision."]
    pub fn instance_splits(&self) -> ListRef<DataCloudRunV2WorkerPoolInstanceSplitsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_splits", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component,\nenvironment, state, etc. For more information, visit https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.\n\nCloud Run API v2 does not support labels with  'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system labels in v1 now have a corresponding field in v2 WorkerPool.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the cloud run worker pool"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the WorkerPool."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `observed_generation` after provisioning.\nThe generation of this WorkerPool currently serving traffic. See comments in reconciling for additional information on reconciliation process in Cloud Run. Please note that unlike v1, this is an int64 value. As with most Google APIs, its JSON representation will be a string instead of an integer."]
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
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nReturns true if the WorkerPool is currently being acted upon by the system to bring it into the desired state.\n\nWhen a new WorkerPool is created, or an existing one is updated, Cloud Run will asynchronously perform all necessary steps to bring the WorkerPool to the desired serving state. This process is called reconciliation. While reconciliation is in process, observedGeneration, latest_ready_revison, trafficStatuses, and uri will have transient values that might mismatch the intended state: Once reconciliation is over (and this field is false), there are two possible outcomes: reconciliation succeeded and the serving state matches the WorkerPool, or there was an error, and reconciliation failed. This state can be found in terminalCondition.state.\n\nIf reconciliation succeeded, the following fields will match: traffic and trafficStatuses, observedGeneration and generation, latestReadyRevision and latestCreatedRevision.\n\nIf reconciliation failed, trafficStatuses, observedGeneration, and latestReadyRevision will have the state of the last serving revision, or empty for newly created WorkerPools. Additional information on the failure can be found in terminalCondition and conditions."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scaling` after provisioning.\nScaling settings that apply to the worker pool."]
    pub fn scaling(&self) -> ListRef<DataCloudRunV2WorkerPoolScalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template` after provisioning.\nThe template used to create revisions for this WorkerPool."]
    pub fn template(&self) -> ListRef<DataCloudRunV2WorkerPoolTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.template", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terminal_condition` after provisioning.\nThe Condition of this WorkerPool, containing its readiness status, and detailed error information in case it did not reach a serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn terminal_condition(&self) -> ListRef<DataCloudRunV2WorkerPoolTerminalConditionElRef> {
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
}
#[derive(Serialize)]
pub struct DataCloudRunV2WorkerPoolBinaryAuthorizationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    breakglass_justification: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_default: Option<PrimField<bool>>,
}
impl DataCloudRunV2WorkerPoolBinaryAuthorizationEl {
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
impl ToListMappable for DataCloudRunV2WorkerPoolBinaryAuthorizationEl {
    type O = BlockAssignable<DataCloudRunV2WorkerPoolBinaryAuthorizationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolBinaryAuthorizationEl {}
impl BuildDataCloudRunV2WorkerPoolBinaryAuthorizationEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolBinaryAuthorizationEl {
        DataCloudRunV2WorkerPoolBinaryAuthorizationEl {
            breakglass_justification: core::default::Default::default(),
            policy: core::default::Default::default(),
            use_default: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolBinaryAuthorizationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolBinaryAuthorizationElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2WorkerPoolBinaryAuthorizationElRef {
        DataCloudRunV2WorkerPoolBinaryAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolBinaryAuthorizationElRef {
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
pub struct DataCloudRunV2WorkerPoolConditionsEl {
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
impl DataCloudRunV2WorkerPoolConditionsEl {
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
impl ToListMappable for DataCloudRunV2WorkerPoolConditionsEl {
    type O = BlockAssignable<DataCloudRunV2WorkerPoolConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolConditionsEl {}
impl BuildDataCloudRunV2WorkerPoolConditionsEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolConditionsEl {
        DataCloudRunV2WorkerPoolConditionsEl {
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
pub struct DataCloudRunV2WorkerPoolConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolConditionsElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2WorkerPoolConditionsElRef {
        DataCloudRunV2WorkerPoolConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolConditionsElRef {
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
pub struct DataCloudRunV2WorkerPoolInstanceSplitStatusesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataCloudRunV2WorkerPoolInstanceSplitStatusesEl {
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
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2WorkerPoolInstanceSplitStatusesEl {
    type O = BlockAssignable<DataCloudRunV2WorkerPoolInstanceSplitStatusesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolInstanceSplitStatusesEl {}
impl BuildDataCloudRunV2WorkerPoolInstanceSplitStatusesEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolInstanceSplitStatusesEl {
        DataCloudRunV2WorkerPoolInstanceSplitStatusesEl {
            percent: core::default::Default::default(),
            revision: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolInstanceSplitStatusesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolInstanceSplitStatusesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2WorkerPoolInstanceSplitStatusesElRef {
        DataCloudRunV2WorkerPoolInstanceSplitStatusesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolInstanceSplitStatusesElRef {
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2WorkerPoolInstanceSplitsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataCloudRunV2WorkerPoolInstanceSplitsEl {
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
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2WorkerPoolInstanceSplitsEl {
    type O = BlockAssignable<DataCloudRunV2WorkerPoolInstanceSplitsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolInstanceSplitsEl {}
impl BuildDataCloudRunV2WorkerPoolInstanceSplitsEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolInstanceSplitsEl {
        DataCloudRunV2WorkerPoolInstanceSplitsEl {
            percent: core::default::Default::default(),
            revision: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolInstanceSplitsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolInstanceSplitsElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2WorkerPoolInstanceSplitsElRef {
        DataCloudRunV2WorkerPoolInstanceSplitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolInstanceSplitsElRef {
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2WorkerPoolScalingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    manual_instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scaling_mode: Option<PrimField<String>>,
}
impl DataCloudRunV2WorkerPoolScalingEl {
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
impl ToListMappable for DataCloudRunV2WorkerPoolScalingEl {
    type O = BlockAssignable<DataCloudRunV2WorkerPoolScalingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolScalingEl {}
impl BuildDataCloudRunV2WorkerPoolScalingEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolScalingEl {
        DataCloudRunV2WorkerPoolScalingEl {
            manual_instance_count: core::default::Default::default(),
            max_instance_count: core::default::Default::default(),
            min_instance_count: core::default::Default::default(),
            scaling_mode: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolScalingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolScalingElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2WorkerPoolScalingElRef {
        DataCloudRunV2WorkerPoolScalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolScalingElRef {
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
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
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
    for DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefEl
{
    type O = BlockAssignable<
        DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {}
impl BuildDataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    pub fn build(
        self,
    ) -> DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
        DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
            secret: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
        DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
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
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_key_ref: Option<
        ListField<DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefEl>,
    >,
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceEl {
    #[doc = "Set the field `secret_key_ref`.\n"]
    pub fn set_secret_key_ref(
        mut self,
        v: impl Into<
            ListField<
                DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefEl,
            >,
        >,
    ) -> Self {
        self.secret_key_ref = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceEl {
    type O = BlockAssignable<DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceEl {}
impl BuildDataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceEl {
        DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceEl {
            secret_key_ref: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElRef {
        DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `secret_key_ref` after provisioning.\n"]
    pub fn secret_key_ref(
        &self,
    ) -> ListRef<DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secret_key_ref", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElEnvEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_source:
        Option<ListField<DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceEl>>,
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElEnvEl {
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
        v: impl Into<ListField<DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceEl>>,
    ) -> Self {
        self.value_source = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2WorkerPoolTemplateElContainersElEnvEl {
    type O = BlockAssignable<DataCloudRunV2WorkerPoolTemplateElContainersElEnvEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTemplateElContainersElEnvEl {}
impl BuildDataCloudRunV2WorkerPoolTemplateElContainersElEnvEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolTemplateElContainersElEnvEl {
        DataCloudRunV2WorkerPoolTemplateElContainersElEnvEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
            value_source: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElEnvElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolTemplateElContainersElEnvElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2WorkerPoolTemplateElContainersElEnvElRef {
        DataCloudRunV2WorkerPoolTemplateElContainersElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElEnvElRef {
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
    ) -> ListRef<DataCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.value_source", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcEl {
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
impl ToListMappable for DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcEl {
    type O = BlockAssignable<DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcEl {}
impl BuildDataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcEl {
        DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcEl {
            port: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcElRef {
        DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcElRef {
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
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl
{
    type O = BlockAssignable<
        DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl
{}
impl BuildDataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
    pub fn build(
        self,
    ) -> DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
        DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
            port: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef
    {
        DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_headers: Option<
        ListField<
            DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetEl {
    #[doc = "Set the field `http_headers`.\n"]
    pub fn set_http_headers(
        mut self,
        v: impl Into<
            ListField<
                DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl,
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
impl ToListMappable for DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetEl {
    type O =
        BlockAssignable<DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetEl {}
impl BuildDataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetEl {
        DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetEl {
            http_headers: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElRef {
        DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `http_headers` after provisioning.\n"]
    pub fn http_headers(
        &self,
    ) -> ListRef<
        DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef,
    > {
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
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketEl {
    type O =
        BlockAssignable<DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketEl {}
impl BuildDataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketEl {
        DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketEl {
            port: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketElRef {
        DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc: Option<ListField<DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_get:
        Option<ListField<DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_delay_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_socket:
        Option<ListField<DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_seconds: Option<PrimField<f64>>,
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeEl {
    #[doc = "Set the field `failure_threshold`.\n"]
    pub fn set_failure_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.failure_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v: impl Into<ListField<DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcEl>>,
    ) -> Self {
        self.grpc = Some(v.into());
        self
    }
    #[doc = "Set the field `http_get`.\n"]
    pub fn set_http_get(
        mut self,
        v: impl Into<ListField<DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetEl>>,
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
        v: impl Into<
            ListField<DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketEl>,
        >,
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
impl ToListMappable for DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeEl {
    type O = BlockAssignable<DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeEl {}
impl BuildDataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeEl {
        DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeEl {
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
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElRef {
        DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElRef {
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
    ) -> ListRef<DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }
    #[doc = "Get a reference to the value of field `http_get` after provisioning.\n"]
    pub fn http_get(
        &self,
    ) -> ListRef<DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElRef> {
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
    ) -> ListRef<DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketElRef> {
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
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    limits: Option<RecField<PrimField<String>>>,
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElResourcesEl {
    #[doc = "Set the field `limits`.\n"]
    pub fn set_limits(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.limits = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2WorkerPoolTemplateElContainersElResourcesEl {
    type O = BlockAssignable<DataCloudRunV2WorkerPoolTemplateElContainersElResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTemplateElContainersElResourcesEl {}
impl BuildDataCloudRunV2WorkerPoolTemplateElContainersElResourcesEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolTemplateElContainersElResourcesEl {
        DataCloudRunV2WorkerPoolTemplateElContainersElResourcesEl {
            limits: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolTemplateElContainersElResourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2WorkerPoolTemplateElContainersElResourcesElRef {
        DataCloudRunV2WorkerPoolTemplateElContainersElResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcEl {
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
impl ToListMappable for DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcEl {
    type O = BlockAssignable<DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcEl {}
impl BuildDataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcEl {
        DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcEl {
            port: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcElRef {
        DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcElRef {
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
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl
{
    type O = BlockAssignable<
        DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl
{}
impl BuildDataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    pub fn build(
        self,
    ) -> DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
        DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
            port: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
        DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_headers: Option<
        ListField<
            DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetEl {
    #[doc = "Set the field `http_headers`.\n"]
    pub fn set_http_headers(
        mut self,
        v: impl Into<
            ListField<
                DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl,
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
impl ToListMappable for DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetEl {
    type O = BlockAssignable<DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetEl {}
impl BuildDataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetEl {
        DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetEl {
            http_headers: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElRef {
        DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `http_headers` after provisioning.\n"]
    pub fn http_headers(
        &self,
    ) -> ListRef<
        DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef,
    > {
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
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketEl {
    type O =
        BlockAssignable<DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketEl {}
impl BuildDataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketEl {
        DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketEl {
            port: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketElRef {
        DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc: Option<ListField<DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_get:
        Option<ListField<DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_delay_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_socket:
        Option<ListField<DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_seconds: Option<PrimField<f64>>,
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeEl {
    #[doc = "Set the field `failure_threshold`.\n"]
    pub fn set_failure_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.failure_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v: impl Into<ListField<DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcEl>>,
    ) -> Self {
        self.grpc = Some(v.into());
        self
    }
    #[doc = "Set the field `http_get`.\n"]
    pub fn set_http_get(
        mut self,
        v: impl Into<ListField<DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetEl>>,
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
        v: impl Into<ListField<DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketEl>>,
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
impl ToListMappable for DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeEl {
    type O = BlockAssignable<DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeEl {}
impl BuildDataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeEl {
        DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeEl {
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
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElRef {
        DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElRef {
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
    ) -> ListRef<DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }
    #[doc = "Get a reference to the value of field `http_get` after provisioning.\n"]
    pub fn http_get(
        &self,
    ) -> ListRef<DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElRef> {
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
    ) -> ListRef<DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketElRef> {
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
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElVolumeMountsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mount_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_path: Option<PrimField<String>>,
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElVolumeMountsEl {
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
impl ToListMappable for DataCloudRunV2WorkerPoolTemplateElContainersElVolumeMountsEl {
    type O = BlockAssignable<DataCloudRunV2WorkerPoolTemplateElContainersElVolumeMountsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTemplateElContainersElVolumeMountsEl {}
impl BuildDataCloudRunV2WorkerPoolTemplateElContainersElVolumeMountsEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolTemplateElContainersElVolumeMountsEl {
        DataCloudRunV2WorkerPoolTemplateElContainersElVolumeMountsEl {
            mount_path: core::default::Default::default(),
            name: core::default::Default::default(),
            sub_path: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElVolumeMountsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolTemplateElContainersElVolumeMountsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2WorkerPoolTemplateElContainersElVolumeMountsElRef {
        DataCloudRunV2WorkerPoolTemplateElContainersElVolumeMountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElVolumeMountsElRef {
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
pub struct DataCloudRunV2WorkerPoolTemplateElContainersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    depends_on: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<SetField<DataCloudRunV2WorkerPoolTemplateElContainersElEnvEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    liveness_probe:
        Option<ListField<DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<ListField<DataCloudRunV2WorkerPoolTemplateElContainersElResourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    startup_probe: Option<ListField<DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_mounts: Option<ListField<DataCloudRunV2WorkerPoolTemplateElContainersElVolumeMountsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    working_dir: Option<PrimField<String>>,
}
impl DataCloudRunV2WorkerPoolTemplateElContainersEl {
    #[doc = "Set the field `args`.\n"]
    pub fn set_args(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
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
        v: impl Into<SetField<DataCloudRunV2WorkerPoolTemplateElContainersElEnvEl>>,
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
        v: impl Into<ListField<DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeEl>>,
    ) -> Self {
        self.liveness_probe = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v: impl Into<ListField<DataCloudRunV2WorkerPoolTemplateElContainersElResourcesEl>>,
    ) -> Self {
        self.resources = Some(v.into());
        self
    }
    #[doc = "Set the field `startup_probe`.\n"]
    pub fn set_startup_probe(
        mut self,
        v: impl Into<ListField<DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeEl>>,
    ) -> Self {
        self.startup_probe = Some(v.into());
        self
    }
    #[doc = "Set the field `volume_mounts`.\n"]
    pub fn set_volume_mounts(
        mut self,
        v: impl Into<ListField<DataCloudRunV2WorkerPoolTemplateElContainersElVolumeMountsEl>>,
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
impl ToListMappable for DataCloudRunV2WorkerPoolTemplateElContainersEl {
    type O = BlockAssignable<DataCloudRunV2WorkerPoolTemplateElContainersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTemplateElContainersEl {}
impl BuildDataCloudRunV2WorkerPoolTemplateElContainersEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolTemplateElContainersEl {
        DataCloudRunV2WorkerPoolTemplateElContainersEl {
            args: core::default::Default::default(),
            command: core::default::Default::default(),
            depends_on: core::default::Default::default(),
            env: core::default::Default::default(),
            image: core::default::Default::default(),
            liveness_probe: core::default::Default::default(),
            name: core::default::Default::default(),
            resources: core::default::Default::default(),
            startup_probe: core::default::Default::default(),
            volume_mounts: core::default::Default::default(),
            working_dir: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolTemplateElContainersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolTemplateElContainersElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2WorkerPoolTemplateElContainersElRef {
        DataCloudRunV2WorkerPoolTemplateElContainersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTemplateElContainersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `args` after provisioning.\n"]
    pub fn args(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
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
    pub fn env(&self) -> SetRef<DataCloudRunV2WorkerPoolTemplateElContainersElEnvElRef> {
        SetRef::new(self.shared().clone(), format!("{}.env", self.base))
    }
    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
    #[doc = "Get a reference to the value of field `liveness_probe` after provisioning.\n"]
    pub fn liveness_probe(
        &self,
    ) -> ListRef<DataCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.liveness_probe", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(
        &self,
    ) -> ListRef<DataCloudRunV2WorkerPoolTemplateElContainersElResourcesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
    #[doc = "Get a reference to the value of field `startup_probe` after provisioning.\n"]
    pub fn startup_probe(
        &self,
    ) -> ListRef<DataCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.startup_probe", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `volume_mounts` after provisioning.\n"]
    pub fn volume_mounts(
        &self,
    ) -> ListRef<DataCloudRunV2WorkerPoolTemplateElContainersElVolumeMountsElRef> {
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
pub struct DataCloudRunV2WorkerPoolTemplateElNodeSelectorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator: Option<PrimField<String>>,
}
impl DataCloudRunV2WorkerPoolTemplateElNodeSelectorEl {
    #[doc = "Set the field `accelerator`.\n"]
    pub fn set_accelerator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.accelerator = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2WorkerPoolTemplateElNodeSelectorEl {
    type O = BlockAssignable<DataCloudRunV2WorkerPoolTemplateElNodeSelectorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTemplateElNodeSelectorEl {}
impl BuildDataCloudRunV2WorkerPoolTemplateElNodeSelectorEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolTemplateElNodeSelectorEl {
        DataCloudRunV2WorkerPoolTemplateElNodeSelectorEl {
            accelerator: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolTemplateElNodeSelectorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolTemplateElNodeSelectorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2WorkerPoolTemplateElNodeSelectorElRef {
        DataCloudRunV2WorkerPoolTemplateElNodeSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTemplateElNodeSelectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accelerator` after provisioning.\n"]
    pub fn accelerator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.accelerator", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instances: Option<SetField<PrimField<String>>>,
}
impl DataCloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceEl {
    #[doc = "Set the field `instances`.\n"]
    pub fn set_instances(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.instances = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceEl {
    type O = BlockAssignable<DataCloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceEl {}
impl BuildDataCloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceEl {
        DataCloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceEl {
            instances: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceElRef {
        DataCloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.instances", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2WorkerPoolTemplateElVolumesElEmptyDirEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    medium: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size_limit: Option<PrimField<String>>,
}
impl DataCloudRunV2WorkerPoolTemplateElVolumesElEmptyDirEl {
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
impl ToListMappable for DataCloudRunV2WorkerPoolTemplateElVolumesElEmptyDirEl {
    type O = BlockAssignable<DataCloudRunV2WorkerPoolTemplateElVolumesElEmptyDirEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTemplateElVolumesElEmptyDirEl {}
impl BuildDataCloudRunV2WorkerPoolTemplateElVolumesElEmptyDirEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolTemplateElVolumesElEmptyDirEl {
        DataCloudRunV2WorkerPoolTemplateElVolumesElEmptyDirEl {
            medium: core::default::Default::default(),
            size_limit: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolTemplateElVolumesElEmptyDirElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolTemplateElVolumesElEmptyDirElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2WorkerPoolTemplateElVolumesElEmptyDirElRef {
        DataCloudRunV2WorkerPoolTemplateElVolumesElEmptyDirElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTemplateElVolumesElEmptyDirElRef {
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
pub struct DataCloudRunV2WorkerPoolTemplateElVolumesElGcsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mount_options: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<PrimField<bool>>,
}
impl DataCloudRunV2WorkerPoolTemplateElVolumesElGcsEl {
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
impl ToListMappable for DataCloudRunV2WorkerPoolTemplateElVolumesElGcsEl {
    type O = BlockAssignable<DataCloudRunV2WorkerPoolTemplateElVolumesElGcsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTemplateElVolumesElGcsEl {}
impl BuildDataCloudRunV2WorkerPoolTemplateElVolumesElGcsEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolTemplateElVolumesElGcsEl {
        DataCloudRunV2WorkerPoolTemplateElVolumesElGcsEl {
            bucket: core::default::Default::default(),
            mount_options: core::default::Default::default(),
            read_only: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolTemplateElVolumesElGcsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolTemplateElVolumesElGcsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2WorkerPoolTemplateElVolumesElGcsElRef {
        DataCloudRunV2WorkerPoolTemplateElVolumesElGcsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTemplateElVolumesElGcsElRef {
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
pub struct DataCloudRunV2WorkerPoolTemplateElVolumesElNfsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server: Option<PrimField<String>>,
}
impl DataCloudRunV2WorkerPoolTemplateElVolumesElNfsEl {
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
impl ToListMappable for DataCloudRunV2WorkerPoolTemplateElVolumesElNfsEl {
    type O = BlockAssignable<DataCloudRunV2WorkerPoolTemplateElVolumesElNfsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTemplateElVolumesElNfsEl {}
impl BuildDataCloudRunV2WorkerPoolTemplateElVolumesElNfsEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolTemplateElVolumesElNfsEl {
        DataCloudRunV2WorkerPoolTemplateElVolumesElNfsEl {
            path: core::default::Default::default(),
            read_only: core::default::Default::default(),
            server: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolTemplateElVolumesElNfsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolTemplateElVolumesElNfsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2WorkerPoolTemplateElVolumesElNfsElRef {
        DataCloudRunV2WorkerPoolTemplateElVolumesElNfsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTemplateElVolumesElNfsElRef {
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
pub struct DataCloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl DataCloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsEl {
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
impl ToListMappable for DataCloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsEl {
    type O = BlockAssignable<DataCloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsEl {}
impl BuildDataCloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsEl {
        DataCloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsEl {
            mode: core::default::Default::default(),
            path: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsElRef {
        DataCloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsElRef {
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
pub struct DataCloudRunV2WorkerPoolTemplateElVolumesElSecretEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_mode: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<ListField<DataCloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<PrimField<String>>,
}
impl DataCloudRunV2WorkerPoolTemplateElVolumesElSecretEl {
    #[doc = "Set the field `default_mode`.\n"]
    pub fn set_default_mode(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl Into<ListField<DataCloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsEl>>,
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
impl ToListMappable for DataCloudRunV2WorkerPoolTemplateElVolumesElSecretEl {
    type O = BlockAssignable<DataCloudRunV2WorkerPoolTemplateElVolumesElSecretEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTemplateElVolumesElSecretEl {}
impl BuildDataCloudRunV2WorkerPoolTemplateElVolumesElSecretEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolTemplateElVolumesElSecretEl {
        DataCloudRunV2WorkerPoolTemplateElVolumesElSecretEl {
            default_mode: core::default::Default::default(),
            items: core::default::Default::default(),
            secret: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolTemplateElVolumesElSecretElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolTemplateElVolumesElSecretElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2WorkerPoolTemplateElVolumesElSecretElRef {
        DataCloudRunV2WorkerPoolTemplateElVolumesElSecretElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTemplateElVolumesElSecretElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_mode` after provisioning.\n"]
    pub fn default_mode(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> ListRef<DataCloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2WorkerPoolTemplateElVolumesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_sql_instance:
        Option<ListField<DataCloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    empty_dir: Option<ListField<DataCloudRunV2WorkerPoolTemplateElVolumesElEmptyDirEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs: Option<ListField<DataCloudRunV2WorkerPoolTemplateElVolumesElGcsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nfs: Option<ListField<DataCloudRunV2WorkerPoolTemplateElVolumesElNfsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<ListField<DataCloudRunV2WorkerPoolTemplateElVolumesElSecretEl>>,
}
impl DataCloudRunV2WorkerPoolTemplateElVolumesEl {
    #[doc = "Set the field `cloud_sql_instance`.\n"]
    pub fn set_cloud_sql_instance(
        mut self,
        v: impl Into<ListField<DataCloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceEl>>,
    ) -> Self {
        self.cloud_sql_instance = Some(v.into());
        self
    }
    #[doc = "Set the field `empty_dir`.\n"]
    pub fn set_empty_dir(
        mut self,
        v: impl Into<ListField<DataCloudRunV2WorkerPoolTemplateElVolumesElEmptyDirEl>>,
    ) -> Self {
        self.empty_dir = Some(v.into());
        self
    }
    #[doc = "Set the field `gcs`.\n"]
    pub fn set_gcs(
        mut self,
        v: impl Into<ListField<DataCloudRunV2WorkerPoolTemplateElVolumesElGcsEl>>,
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
        v: impl Into<ListField<DataCloudRunV2WorkerPoolTemplateElVolumesElNfsEl>>,
    ) -> Self {
        self.nfs = Some(v.into());
        self
    }
    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(
        mut self,
        v: impl Into<ListField<DataCloudRunV2WorkerPoolTemplateElVolumesElSecretEl>>,
    ) -> Self {
        self.secret = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2WorkerPoolTemplateElVolumesEl {
    type O = BlockAssignable<DataCloudRunV2WorkerPoolTemplateElVolumesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTemplateElVolumesEl {}
impl BuildDataCloudRunV2WorkerPoolTemplateElVolumesEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolTemplateElVolumesEl {
        DataCloudRunV2WorkerPoolTemplateElVolumesEl {
            cloud_sql_instance: core::default::Default::default(),
            empty_dir: core::default::Default::default(),
            gcs: core::default::Default::default(),
            name: core::default::Default::default(),
            nfs: core::default::Default::default(),
            secret: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolTemplateElVolumesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolTemplateElVolumesElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2WorkerPoolTemplateElVolumesElRef {
        DataCloudRunV2WorkerPoolTemplateElVolumesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTemplateElVolumesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_sql_instance` after provisioning.\n"]
    pub fn cloud_sql_instance(
        &self,
    ) -> ListRef<DataCloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_sql_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `empty_dir` after provisioning.\n"]
    pub fn empty_dir(&self) -> ListRef<DataCloudRunV2WorkerPoolTemplateElVolumesElEmptyDirElRef> {
        ListRef::new(self.shared().clone(), format!("{}.empty_dir", self.base))
    }
    #[doc = "Get a reference to the value of field `gcs` after provisioning.\n"]
    pub fn gcs(&self) -> ListRef<DataCloudRunV2WorkerPoolTemplateElVolumesElGcsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gcs", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `nfs` after provisioning.\n"]
    pub fn nfs(&self) -> ListRef<DataCloudRunV2WorkerPoolTemplateElVolumesElNfsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.nfs", self.base))
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> ListRef<DataCloudRunV2WorkerPoolTemplateElVolumesElSecretElRef> {
        ListRef::new(self.shared().clone(), format!("{}.secret", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<PrimField<String>>>,
}
impl DataCloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesEl {
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
impl ToListMappable for DataCloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesEl {
    type O = BlockAssignable<DataCloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesEl {}
impl BuildDataCloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesEl {
        DataCloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesEl {
            network: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesElRef {
        DataCloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesElRef {
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
pub struct DataCloudRunV2WorkerPoolTemplateElVpcAccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connector: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interfaces:
        Option<ListField<DataCloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesEl>>,
}
impl DataCloudRunV2WorkerPoolTemplateElVpcAccessEl {
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
        v: impl Into<ListField<DataCloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesEl>>,
    ) -> Self {
        self.network_interfaces = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2WorkerPoolTemplateElVpcAccessEl {
    type O = BlockAssignable<DataCloudRunV2WorkerPoolTemplateElVpcAccessEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTemplateElVpcAccessEl {}
impl BuildDataCloudRunV2WorkerPoolTemplateElVpcAccessEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolTemplateElVpcAccessEl {
        DataCloudRunV2WorkerPoolTemplateElVpcAccessEl {
            connector: core::default::Default::default(),
            egress: core::default::Default::default(),
            network_interfaces: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolTemplateElVpcAccessElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolTemplateElVpcAccessElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2WorkerPoolTemplateElVpcAccessElRef {
        DataCloudRunV2WorkerPoolTemplateElVpcAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTemplateElVpcAccessElRef {
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
    ) -> ListRef<DataCloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_interfaces", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2WorkerPoolTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    containers: Option<ListField<DataCloudRunV2WorkerPoolTemplateElContainersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_key_revocation_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_key_shutdown_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_zonal_redundancy_disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_selector: Option<ListField<DataCloudRunV2WorkerPoolTemplateElNodeSelectorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volumes: Option<ListField<DataCloudRunV2WorkerPoolTemplateElVolumesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_access: Option<ListField<DataCloudRunV2WorkerPoolTemplateElVpcAccessEl>>,
}
impl DataCloudRunV2WorkerPoolTemplateEl {
    #[doc = "Set the field `annotations`.\n"]
    pub fn set_annotations(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `containers`.\n"]
    pub fn set_containers(
        mut self,
        v: impl Into<ListField<DataCloudRunV2WorkerPoolTemplateElContainersEl>>,
    ) -> Self {
        self.containers = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_key`.\n"]
    pub fn set_encryption_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_key = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_key_revocation_action`.\n"]
    pub fn set_encryption_key_revocation_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_key_revocation_action = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_key_shutdown_duration`.\n"]
    pub fn set_encryption_key_shutdown_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_key_shutdown_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `gpu_zonal_redundancy_disabled`.\n"]
    pub fn set_gpu_zonal_redundancy_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.gpu_zonal_redundancy_disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `node_selector`.\n"]
    pub fn set_node_selector(
        mut self,
        v: impl Into<ListField<DataCloudRunV2WorkerPoolTemplateElNodeSelectorEl>>,
    ) -> Self {
        self.node_selector = Some(v.into());
        self
    }
    #[doc = "Set the field `revision`.\n"]
    pub fn set_revision(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.revision = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `volumes`.\n"]
    pub fn set_volumes(
        mut self,
        v: impl Into<ListField<DataCloudRunV2WorkerPoolTemplateElVolumesEl>>,
    ) -> Self {
        self.volumes = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_access`.\n"]
    pub fn set_vpc_access(
        mut self,
        v: impl Into<ListField<DataCloudRunV2WorkerPoolTemplateElVpcAccessEl>>,
    ) -> Self {
        self.vpc_access = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2WorkerPoolTemplateEl {
    type O = BlockAssignable<DataCloudRunV2WorkerPoolTemplateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTemplateEl {}
impl BuildDataCloudRunV2WorkerPoolTemplateEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolTemplateEl {
        DataCloudRunV2WorkerPoolTemplateEl {
            annotations: core::default::Default::default(),
            containers: core::default::Default::default(),
            encryption_key: core::default::Default::default(),
            encryption_key_revocation_action: core::default::Default::default(),
            encryption_key_shutdown_duration: core::default::Default::default(),
            gpu_zonal_redundancy_disabled: core::default::Default::default(),
            labels: core::default::Default::default(),
            node_selector: core::default::Default::default(),
            revision: core::default::Default::default(),
            service_account: core::default::Default::default(),
            volumes: core::default::Default::default(),
            vpc_access: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2WorkerPoolTemplateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolTemplateElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2WorkerPoolTemplateElRef {
        DataCloudRunV2WorkerPoolTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\n"]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.annotations", self.base))
    }
    #[doc = "Get a reference to the value of field `containers` after provisioning.\n"]
    pub fn containers(&self) -> ListRef<DataCloudRunV2WorkerPoolTemplateElContainersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.containers", self.base))
    }
    #[doc = "Get a reference to the value of field `encryption_key` after provisioning.\n"]
    pub fn encryption_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_key_revocation_action` after provisioning.\n"]
    pub fn encryption_key_revocation_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_key_revocation_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_key_shutdown_duration` after provisioning.\n"]
    pub fn encryption_key_shutdown_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_key_shutdown_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gpu_zonal_redundancy_disabled` after provisioning.\n"]
    pub fn gpu_zonal_redundancy_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gpu_zonal_redundancy_disabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `node_selector` after provisioning.\n"]
    pub fn node_selector(&self) -> ListRef<DataCloudRunV2WorkerPoolTemplateElNodeSelectorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_selector", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(&self) -> ListRef<DataCloudRunV2WorkerPoolTemplateElVolumesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }
    #[doc = "Get a reference to the value of field `vpc_access` after provisioning.\n"]
    pub fn vpc_access(&self) -> ListRef<DataCloudRunV2WorkerPoolTemplateElVpcAccessElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_access", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2WorkerPoolTerminalConditionEl {
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
impl DataCloudRunV2WorkerPoolTerminalConditionEl {
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
impl ToListMappable for DataCloudRunV2WorkerPoolTerminalConditionEl {
    type O = BlockAssignable<DataCloudRunV2WorkerPoolTerminalConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2WorkerPoolTerminalConditionEl {}
impl BuildDataCloudRunV2WorkerPoolTerminalConditionEl {
    pub fn build(self) -> DataCloudRunV2WorkerPoolTerminalConditionEl {
        DataCloudRunV2WorkerPoolTerminalConditionEl {
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
pub struct DataCloudRunV2WorkerPoolTerminalConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2WorkerPoolTerminalConditionElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2WorkerPoolTerminalConditionElRef {
        DataCloudRunV2WorkerPoolTerminalConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2WorkerPoolTerminalConditionElRef {
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
