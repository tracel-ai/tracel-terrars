use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudRunV2WorkerPoolData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_audiences: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_stage: Option<PrimField<String>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    binary_authorization: Option<Vec<CloudRunV2WorkerPoolBinaryAuthorizationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_splits: Option<Vec<CloudRunV2WorkerPoolInstanceSplitsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scaling: Option<Vec<CloudRunV2WorkerPoolScalingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template: Option<Vec<CloudRunV2WorkerPoolTemplateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CloudRunV2WorkerPoolTimeoutsEl>,
    dynamic: CloudRunV2WorkerPoolDynamic,
}
struct CloudRunV2WorkerPool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudRunV2WorkerPoolData>,
}
#[derive(Clone)]
pub struct CloudRunV2WorkerPool(Rc<CloudRunV2WorkerPool_>);
impl CloudRunV2WorkerPool {
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
    #[doc = "Set the field `annotations`.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected in new resources.\nAll system annotations in v1 now have a corresponding field in v2 WorkerPool.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `client`.\nArbitrary identifier for the API client."]
    pub fn set_client(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().client = Some(v.into());
        self
    }
    #[doc = "Set the field `client_version`.\nArbitrary version identifier for the API client."]
    pub fn set_client_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().client_version = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_audiences`.\nOne or more custom audiences that you want this worker pool to support. Specify each custom audience as the full URL in a string. The custom audiences are encoded in the token and used to authenticate requests.\nFor more information, see https://cloud.google.com/run/docs/configuring/custom-audiences."]
    pub fn set_custom_audiences(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().custom_audiences = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_protection`.\nWhether Terraform will be prevented from destroying the service. Defaults to true.\nWhen a'terraform destroy' or 'terraform apply' would delete the service,\nthe command will fail if this field is not set to false in Terraform state.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the WorkerPool will fail.\nWhen the field is set to false, deleting the WorkerPool is allowed."]
    pub fn set_deletion_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nUser-provided description of the WorkerPool. This field currently has a 512-character limit."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component,\nenvironment, state, etc. For more information, visit https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.\n\nCloud Run API v2 does not support labels with  'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system labels in v1 now have a corresponding field in v2 WorkerPool.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `launch_stage`.\nThe launch stage as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/products#product-launch-stages). Cloud Run supports ALPHA, BETA, and GA.\nIf no value is specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that stage. On read (or output), describes whether the resource uses preview features.\n\nFor example, if ALPHA is provided as input, but only BETA and GA-level features are used, this field will be BETA on output. Possible values: [\"UNIMPLEMENTED\", \"PRELAUNCH\", \"EARLY_ACCESS\", \"ALPHA\", \"BETA\", \"GA\", \"DEPRECATED\"]"]
    pub fn set_launch_stage(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().launch_stage = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `binary_authorization`.\n"]
    pub fn set_binary_authorization(
        self,
        v: impl Into<BlockAssignable<CloudRunV2WorkerPoolBinaryAuthorizationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().binary_authorization = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.binary_authorization = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `instance_splits`.\n"]
    pub fn set_instance_splits(
        self,
        v: impl Into<BlockAssignable<CloudRunV2WorkerPoolInstanceSplitsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_splits = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_splits = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `scaling`.\n"]
    pub fn set_scaling(self, v: impl Into<BlockAssignable<CloudRunV2WorkerPoolScalingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scaling = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scaling = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `template`.\n"]
    pub fn set_template(
        self,
        v: impl Into<BlockAssignable<CloudRunV2WorkerPoolTemplateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().template = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.template = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CloudRunV2WorkerPoolTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected in new resources.\nAll system annotations in v1 now have a corresponding field in v2 WorkerPool.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
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
    pub fn conditions(&self) -> ListRef<CloudRunV2WorkerPoolConditionsElRef> {
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
    ) -> ListRef<CloudRunV2WorkerPoolInstanceSplitStatusesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_split_statuses", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `terminal_condition` after provisioning.\nThe Condition of this WorkerPool, containing its readiness status, and detailed error information in case it did not reach a serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn terminal_condition(&self) -> ListRef<CloudRunV2WorkerPoolTerminalConditionElRef> {
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
    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\n"]
    pub fn binary_authorization(&self) -> ListRef<CloudRunV2WorkerPoolBinaryAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.binary_authorization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_splits` after provisioning.\n"]
    pub fn instance_splits(&self) -> ListRef<CloudRunV2WorkerPoolInstanceSplitsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_splits", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scaling` after provisioning.\n"]
    pub fn scaling(&self) -> ListRef<CloudRunV2WorkerPoolScalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template` after provisioning.\n"]
    pub fn template(&self) -> ListRef<CloudRunV2WorkerPoolTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.template", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudRunV2WorkerPoolTimeoutsElRef {
        CloudRunV2WorkerPoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for CloudRunV2WorkerPool {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudRunV2WorkerPool {}
impl ToListMappable for CloudRunV2WorkerPool {
    type O = ListRef<CloudRunV2WorkerPoolRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudRunV2WorkerPool_ {
    fn extract_resource_type(&self) -> String {
        "google_cloud_run_v2_worker_pool".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudRunV2WorkerPool {
    pub tf_id: String,
    #[doc = "The location of the cloud run worker pool"]
    pub location: PrimField<String>,
    #[doc = "Name of the WorkerPool."]
    pub name: PrimField<String>,
}
impl BuildCloudRunV2WorkerPool {
    pub fn build(self, stack: &mut Stack) -> CloudRunV2WorkerPool {
        let out = CloudRunV2WorkerPool(Rc::new(CloudRunV2WorkerPool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudRunV2WorkerPoolData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                client: core::default::Default::default(),
                client_version: core::default::Default::default(),
                custom_audiences: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                launch_stage: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                binary_authorization: core::default::Default::default(),
                instance_splits: core::default::Default::default(),
                scaling: core::default::Default::default(),
                template: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudRunV2WorkerPoolRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudRunV2WorkerPoolRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected in new resources.\nAll system annotations in v1 now have a corresponding field in v2 WorkerPool.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
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
    pub fn conditions(&self) -> ListRef<CloudRunV2WorkerPoolConditionsElRef> {
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
    ) -> ListRef<CloudRunV2WorkerPoolInstanceSplitStatusesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_split_statuses", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `terminal_condition` after provisioning.\nThe Condition of this WorkerPool, containing its readiness status, and detailed error information in case it did not reach a serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn terminal_condition(&self) -> ListRef<CloudRunV2WorkerPoolTerminalConditionElRef> {
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
    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\n"]
    pub fn binary_authorization(&self) -> ListRef<CloudRunV2WorkerPoolBinaryAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.binary_authorization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_splits` after provisioning.\n"]
    pub fn instance_splits(&self) -> ListRef<CloudRunV2WorkerPoolInstanceSplitsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_splits", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scaling` after provisioning.\n"]
    pub fn scaling(&self) -> ListRef<CloudRunV2WorkerPoolScalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template` after provisioning.\n"]
    pub fn template(&self) -> ListRef<CloudRunV2WorkerPoolTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.template", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudRunV2WorkerPoolTimeoutsElRef {
        CloudRunV2WorkerPoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolConditionsEl {
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
impl CloudRunV2WorkerPoolConditionsEl {
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
impl ToListMappable for CloudRunV2WorkerPoolConditionsEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolConditionsEl {}
impl BuildCloudRunV2WorkerPoolConditionsEl {
    pub fn build(self) -> CloudRunV2WorkerPoolConditionsEl {
        CloudRunV2WorkerPoolConditionsEl {
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
pub struct CloudRunV2WorkerPoolConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolConditionsElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2WorkerPoolConditionsElRef {
        CloudRunV2WorkerPoolConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolConditionsElRef {
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
pub struct CloudRunV2WorkerPoolInstanceSplitStatusesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl CloudRunV2WorkerPoolInstanceSplitStatusesEl {
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
impl ToListMappable for CloudRunV2WorkerPoolInstanceSplitStatusesEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolInstanceSplitStatusesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolInstanceSplitStatusesEl {}
impl BuildCloudRunV2WorkerPoolInstanceSplitStatusesEl {
    pub fn build(self) -> CloudRunV2WorkerPoolInstanceSplitStatusesEl {
        CloudRunV2WorkerPoolInstanceSplitStatusesEl {
            percent: core::default::Default::default(),
            revision: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolInstanceSplitStatusesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolInstanceSplitStatusesElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2WorkerPoolInstanceSplitStatusesElRef {
        CloudRunV2WorkerPoolInstanceSplitStatusesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolInstanceSplitStatusesElRef {
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
pub struct CloudRunV2WorkerPoolTerminalConditionEl {
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
impl CloudRunV2WorkerPoolTerminalConditionEl {
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
impl ToListMappable for CloudRunV2WorkerPoolTerminalConditionEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolTerminalConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTerminalConditionEl {}
impl BuildCloudRunV2WorkerPoolTerminalConditionEl {
    pub fn build(self) -> CloudRunV2WorkerPoolTerminalConditionEl {
        CloudRunV2WorkerPoolTerminalConditionEl {
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
pub struct CloudRunV2WorkerPoolTerminalConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTerminalConditionElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2WorkerPoolTerminalConditionElRef {
        CloudRunV2WorkerPoolTerminalConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTerminalConditionElRef {
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
pub struct CloudRunV2WorkerPoolBinaryAuthorizationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    breakglass_justification: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_default: Option<PrimField<bool>>,
}
impl CloudRunV2WorkerPoolBinaryAuthorizationEl {
    #[doc = "Set the field `breakglass_justification`.\nIf present, indicates to use Breakglass using this justification. If useDefault is False, then it must be empty. For more information on breakglass, see https://cloud.google.com/binary-authorization/docs/using-breakglass"]
    pub fn set_breakglass_justification(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.breakglass_justification = Some(v.into());
        self
    }
    #[doc = "Set the field `policy`.\nThe path to a binary authorization policy. Format: projects/{project}/platforms/cloudRun/{policy-name}"]
    pub fn set_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy = Some(v.into());
        self
    }
    #[doc = "Set the field `use_default`.\nIf True, indicates to use the default project's binary authorization policy. If False, binary authorization will be disabled."]
    pub fn set_use_default(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_default = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2WorkerPoolBinaryAuthorizationEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolBinaryAuthorizationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolBinaryAuthorizationEl {}
impl BuildCloudRunV2WorkerPoolBinaryAuthorizationEl {
    pub fn build(self) -> CloudRunV2WorkerPoolBinaryAuthorizationEl {
        CloudRunV2WorkerPoolBinaryAuthorizationEl {
            breakglass_justification: core::default::Default::default(),
            policy: core::default::Default::default(),
            use_default: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolBinaryAuthorizationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolBinaryAuthorizationElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2WorkerPoolBinaryAuthorizationElRef {
        CloudRunV2WorkerPoolBinaryAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolBinaryAuthorizationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `breakglass_justification` after provisioning.\nIf present, indicates to use Breakglass using this justification. If useDefault is False, then it must be empty. For more information on breakglass, see https://cloud.google.com/binary-authorization/docs/using-breakglass"]
    pub fn breakglass_justification(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.breakglass_justification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `policy` after provisioning.\nThe path to a binary authorization policy. Format: projects/{project}/platforms/cloudRun/{policy-name}"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
    #[doc = "Get a reference to the value of field `use_default` after provisioning.\nIf True, indicates to use the default project's binary authorization policy. If False, binary authorization will be disabled."]
    pub fn use_default(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_default", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolInstanceSplitsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl CloudRunV2WorkerPoolInstanceSplitsEl {
    #[doc = "Set the field `percent`.\nSpecifies percent of the instance split to this Revision. This defaults to zero if unspecified."]
    pub fn set_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percent = Some(v.into());
        self
    }
    #[doc = "Set the field `revision`.\nRevision to which to assign this portion of instances, if split allocation is by revision."]
    pub fn set_revision(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.revision = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nThe allocation type for this instance split. Possible values: [\"INSTANCE_SPLIT_ALLOCATION_TYPE_LATEST\", \"INSTANCE_SPLIT_ALLOCATION_TYPE_REVISION\"]"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2WorkerPoolInstanceSplitsEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolInstanceSplitsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolInstanceSplitsEl {}
impl BuildCloudRunV2WorkerPoolInstanceSplitsEl {
    pub fn build(self) -> CloudRunV2WorkerPoolInstanceSplitsEl {
        CloudRunV2WorkerPoolInstanceSplitsEl {
            percent: core::default::Default::default(),
            revision: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolInstanceSplitsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolInstanceSplitsElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2WorkerPoolInstanceSplitsElRef {
        CloudRunV2WorkerPoolInstanceSplitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolInstanceSplitsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `percent` after provisioning.\nSpecifies percent of the instance split to this Revision. This defaults to zero if unspecified."]
    pub fn percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }
    #[doc = "Get a reference to the value of field `revision` after provisioning.\nRevision to which to assign this portion of instances, if split allocation is by revision."]
    pub fn revision(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe allocation type for this instance split. Possible values: [\"INSTANCE_SPLIT_ALLOCATION_TYPE_LATEST\", \"INSTANCE_SPLIT_ALLOCATION_TYPE_REVISION\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolScalingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    manual_instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scaling_mode: Option<PrimField<String>>,
}
impl CloudRunV2WorkerPoolScalingEl {
    #[doc = "Set the field `manual_instance_count`.\nThe total number of instances in manual scaling mode."]
    pub fn set_manual_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.manual_instance_count = Some(v.into());
        self
    }
    #[doc = "Set the field `max_instance_count`.\nThe maximum count of instances distributed among revisions based on the specified instance split percentages."]
    pub fn set_max_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_instance_count = Some(v.into());
        self
    }
    #[doc = "Set the field `min_instance_count`.\nThe minimum count of instances distributed among revisions based on the specified instance split percentages."]
    pub fn set_min_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_instance_count = Some(v.into());
        self
    }
    #[doc = "Set the field `scaling_mode`.\nThe scaling mode for the worker pool. It defaults to MANUAL. Possible values: [\"AUTOMATIC\", \"MANUAL\"]"]
    pub fn set_scaling_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scaling_mode = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2WorkerPoolScalingEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolScalingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolScalingEl {}
impl BuildCloudRunV2WorkerPoolScalingEl {
    pub fn build(self) -> CloudRunV2WorkerPoolScalingEl {
        CloudRunV2WorkerPoolScalingEl {
            manual_instance_count: core::default::Default::default(),
            max_instance_count: core::default::Default::default(),
            min_instance_count: core::default::Default::default(),
            scaling_mode: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolScalingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolScalingElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2WorkerPoolScalingElRef {
        CloudRunV2WorkerPoolScalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolScalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `manual_instance_count` after provisioning.\nThe total number of instances in manual scaling mode."]
    pub fn manual_instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.manual_instance_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_instance_count` after provisioning.\nThe maximum count of instances distributed among revisions based on the specified instance split percentages."]
    pub fn max_instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_instance_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_instance_count` after provisioning.\nThe minimum count of instances distributed among revisions based on the specified instance split percentages."]
    pub fn min_instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_instance_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scaling_mode` after provisioning.\nThe scaling mode for the worker pool. It defaults to MANUAL. Possible values: [\"AUTOMATIC\", \"MANUAL\"]"]
    pub fn scaling_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scaling_mode", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    secret: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    #[doc = "Set the field `version`.\nThe Cloud Secret Manager secret version. Can be 'latest' for the latest value or an integer for a specific version."]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    type O =
        BlockAssignable<CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    #[doc = "The name of the secret in Cloud Secret Manager. Format: {secretName} if the secret is in the same project. projects/{project}/secrets/{secretName} if the secret is in a different project."]
    pub secret: PrimField<String>,
}
impl BuildCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    pub fn build(
        self,
    ) -> CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
        CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
            secret: self.secret,
            version: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
        CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\nThe name of the secret in Cloud Secret Manager. Format: {secretName} if the secret is in the same project. projects/{project}/secrets/{secretName} if the secret is in a different project."]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe Cloud Secret Manager secret version. Can be 'latest' for the latest value or an integer for a specific version."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElDynamic {
    secret_key_ref: Option<
        DynamicBlock<CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefEl>,
    >,
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_key_ref:
        Option<Vec<CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefEl>>,
    dynamic: CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElDynamic,
}
impl CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceEl {
    #[doc = "Set the field `secret_key_ref`.\n"]
    pub fn set_secret_key_ref(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.secret_key_ref = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_key_ref = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceEl {}
impl BuildCloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceEl {
    pub fn build(self) -> CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceEl {
        CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceEl {
            secret_key_ref: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElRef {
        CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `secret_key_ref` after provisioning.\n"]
    pub fn secret_key_ref(
        &self,
    ) -> ListRef<CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secret_key_ref", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2WorkerPoolTemplateElContainersElEnvElDynamic {
    value_source:
        Option<DynamicBlock<CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceEl>>,
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTemplateElContainersElEnvEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_source: Option<Vec<CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceEl>>,
    dynamic: CloudRunV2WorkerPoolTemplateElContainersElEnvElDynamic,
}
impl CloudRunV2WorkerPoolTemplateElContainersElEnvEl {
    #[doc = "Set the field `value`.\nLiteral value of the environment variable. Defaults to \"\" and the maximum allowed length is 32768 characters. Variable references are not supported in Cloud Run."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
    #[doc = "Set the field `value_source`.\n"]
    pub fn set_value_source(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.value_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.value_source = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudRunV2WorkerPoolTemplateElContainersElEnvEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolTemplateElContainersElEnvEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTemplateElContainersElEnvEl {
    #[doc = "Name of the environment variable. Must be a C_IDENTIFIER, and may not exceed 32768 characters."]
    pub name: PrimField<String>,
}
impl BuildCloudRunV2WorkerPoolTemplateElContainersElEnvEl {
    pub fn build(self) -> CloudRunV2WorkerPoolTemplateElContainersElEnvEl {
        CloudRunV2WorkerPoolTemplateElContainersElEnvEl {
            name: self.name,
            value: core::default::Default::default(),
            value_source: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolTemplateElContainersElEnvElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTemplateElContainersElEnvElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2WorkerPoolTemplateElContainersElEnvElRef {
        CloudRunV2WorkerPoolTemplateElContainersElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTemplateElContainersElEnvElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the environment variable. Must be a C_IDENTIFIER, and may not exceed 32768 characters."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nLiteral value of the environment variable. Defaults to \"\" and the maximum allowed length is 32768 characters. Variable references are not supported in Cloud Run."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
    #[doc = "Get a reference to the value of field `value_source` after provisioning.\n"]
    pub fn value_source(
        &self,
    ) -> ListRef<CloudRunV2WorkerPoolTemplateElContainersElEnvElValueSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.value_source", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
}
impl CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcEl {
    #[doc = "Set the field `port`.\nOptional. Port number of the gRPC service. Number must be in the range 1 to 65535. If not specified, defaults to the exposed port of the container, which is the value of container.ports[0].containerPort."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `service`.\nOptional. Service is the name of the service to place in the gRPC HealthCheckRequest (see https://github.com/grpc/grpc/blob/master/doc/health-checking.md ). If this is not specified, the default behavior is defined by gRPC"]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcEl {}
impl BuildCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcEl {
    pub fn build(self) -> CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcEl {
        CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcEl {
            port: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcElRef {
        CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nOptional. Port number of the gRPC service. Number must be in the range 1 to 65535. If not specified, defaults to the exposed port of the container, which is the value of container.ports[0].containerPort."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nOptional. Service is the name of the service to place in the gRPC HealthCheckRequest (see https://github.com/grpc/grpc/blob/master/doc/health-checking.md ). If this is not specified, the default behavior is defined by gRPC"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
    port: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
    #[doc = "Set the field `value`.\nOptional. The header field value"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl
{
    type O = BlockAssignable<
        CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
    #[doc = "Required. The header field name"]
    pub port: PrimField<f64>,
}
impl BuildCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
    pub fn build(
        self,
    ) -> CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
        CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
            port: self.port,
            value: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
        CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nRequired. The header field name"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nOptional. The header field value"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElDynamic {
    http_headers: Option<
        DynamicBlock<
            CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_headers: Option<
        Vec<CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl>,
    >,
    dynamic: CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElDynamic,
}
impl CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetEl {
    #[doc = "Set the field `path`.\nOptional. Path to access on the HTTP server. Defaults to '/'."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nOptional. Port number to access on the container. Must be in the range 1 to 65535. If not specified, defaults to the exposed port of the container, which is the value of container.ports[0].containerPort."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `http_headers`.\n"]
    pub fn set_http_headers(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_headers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_headers = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetEl {}
impl BuildCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetEl {
    pub fn build(self) -> CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetEl {
        CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetEl {
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            http_headers: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElRef {
        CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nOptional. Path to access on the HTTP server. Defaults to '/'."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nOptional. Port number to access on the container. Must be in the range 1 to 65535. If not specified, defaults to the exposed port of the container, which is the value of container.ports[0].containerPort."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `http_headers` after provisioning.\n"]
    pub fn http_headers(
        &self,
    ) -> ListRef<CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.http_headers", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketEl {
    #[doc = "Set the field `port`.\nOptional. Port number to access on the container. Must be in the range 1 to 65535. If not specified, defaults to the exposed port of the container, which is the value of container.ports[0].containerPort."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketEl {}
impl BuildCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketEl {
    pub fn build(self) -> CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketEl {
        CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketEl {
            port: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketElRef {
        CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nOptional. Port number to access on the container. Must be in the range 1 to 65535. If not specified, defaults to the exposed port of the container, which is the value of container.ports[0].containerPort."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElDynamic {
    grpc: Option<DynamicBlock<CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcEl>>,
    http_get:
        Option<DynamicBlock<CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetEl>>,
    tcp_socket:
        Option<DynamicBlock<CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketEl>>,
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_delay_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc: Option<Vec<CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_get: Option<Vec<CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_socket: Option<Vec<CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketEl>>,
    dynamic: CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElDynamic,
}
impl CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeEl {
    #[doc = "Set the field `failure_threshold`.\nOptional. Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1."]
    pub fn set_failure_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.failure_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `initial_delay_seconds`.\nOptional. Number of seconds after the container has started before the probe is initiated. Defaults to 0 seconds. Minimum value is 0. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240."]
    pub fn set_initial_delay_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.initial_delay_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `period_seconds`.\nOptional. How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. Must be greater or equal than timeout_seconds."]
    pub fn set_period_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.period_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout_seconds`.\nOptional. Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. Maximum value is 3600. Must be smaller than period_seconds."]
    pub fn set_timeout_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.grpc = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `http_get`.\n"]
    pub fn set_http_get(
        mut self,
        v: impl Into<
            BlockAssignable<CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_get = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_get = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tcp_socket`.\n"]
    pub fn set_tcp_socket(
        mut self,
        v: impl Into<
            BlockAssignable<CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tcp_socket = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tcp_socket = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeEl {}
impl BuildCloudRunV2WorkerPoolTemplateElContainersElLivenessProbeEl {
    pub fn build(self) -> CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeEl {
        CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeEl {
            failure_threshold: core::default::Default::default(),
            initial_delay_seconds: core::default::Default::default(),
            period_seconds: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
            grpc: core::default::Default::default(),
            http_get: core::default::Default::default(),
            tcp_socket: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElRef {
        CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `failure_threshold` after provisioning.\nOptional. Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1."]
    pub fn failure_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failure_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `initial_delay_seconds` after provisioning.\nOptional. Number of seconds after the container has started before the probe is initiated. Defaults to 0 seconds. Minimum value is 0. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240."]
    pub fn initial_delay_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_delay_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `period_seconds` after provisioning.\nOptional. How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. Must be greater or equal than timeout_seconds."]
    pub fn period_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.period_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timeout_seconds` after provisioning.\nOptional. Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. Maximum value is 3600. Must be smaller than period_seconds."]
    pub fn timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(
        &self,
    ) -> ListRef<CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElGrpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }
    #[doc = "Get a reference to the value of field `http_get` after provisioning.\n"]
    pub fn http_get(
        &self,
    ) -> ListRef<CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElHttpGetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_get", self.base))
    }
    #[doc = "Get a reference to the value of field `tcp_socket` after provisioning.\n"]
    pub fn tcp_socket(
        &self,
    ) -> ListRef<CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElTcpSocketElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tcp_socket", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTemplateElContainersElResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    limits: Option<RecField<PrimField<String>>>,
}
impl CloudRunV2WorkerPoolTemplateElContainersElResourcesEl {
    #[doc = "Set the field `limits`.\nOnly memory, CPU, and nvidia.com/gpu are supported. Use key 'cpu' for CPU limit, 'memory' for memory limit, 'nvidia.com/gpu' for gpu limit. Note: The only supported values for CPU are '1', '2', '4', '6', and '8'. Setting 4 CPU requires at least 2Gi of memory, setting 6 or more CPU requires at least 4Gi of memory. The values of the map is string form of the 'quantity' k8s type: https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/apimachinery/pkg/api/resource/quantity.go"]
    pub fn set_limits(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.limits = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2WorkerPoolTemplateElContainersElResourcesEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolTemplateElContainersElResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTemplateElContainersElResourcesEl {}
impl BuildCloudRunV2WorkerPoolTemplateElContainersElResourcesEl {
    pub fn build(self) -> CloudRunV2WorkerPoolTemplateElContainersElResourcesEl {
        CloudRunV2WorkerPoolTemplateElContainersElResourcesEl {
            limits: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolTemplateElContainersElResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTemplateElContainersElResourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2WorkerPoolTemplateElContainersElResourcesElRef {
        CloudRunV2WorkerPoolTemplateElContainersElResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTemplateElContainersElResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `limits` after provisioning.\nOnly memory, CPU, and nvidia.com/gpu are supported. Use key 'cpu' for CPU limit, 'memory' for memory limit, 'nvidia.com/gpu' for gpu limit. Note: The only supported values for CPU are '1', '2', '4', '6', and '8'. Setting 4 CPU requires at least 2Gi of memory, setting 6 or more CPU requires at least 4Gi of memory. The values of the map is string form of the 'quantity' k8s type: https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/apimachinery/pkg/api/resource/quantity.go"]
    pub fn limits(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
}
impl CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcEl {
    #[doc = "Set the field `port`.\nOptional. Port number of the gRPC service. Number must be in the range 1 to 65535. If not specified, defaults to the exposed port of the container, which is the value of container.ports[0].containerPort."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `service`.\nOptional. Service is the name of the service to place in the gRPC HealthCheckRequest (see https://github.com/grpc/grpc/blob/master/doc/health-checking.md ). If this is not specified, the default behavior is defined by gRPC"]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcEl {}
impl BuildCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcEl {
    pub fn build(self) -> CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcEl {
        CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcEl {
            port: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcElRef {
        CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nOptional. Port number of the gRPC service. Number must be in the range 1 to 65535. If not specified, defaults to the exposed port of the container, which is the value of container.ports[0].containerPort."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nOptional. Service is the name of the service to place in the gRPC HealthCheckRequest (see https://github.com/grpc/grpc/blob/master/doc/health-checking.md ). If this is not specified, the default behavior is defined by gRPC"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    port: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    #[doc = "Set the field `value`.\nOptional. The header field value"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl
{
    type O = BlockAssignable<
        CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    #[doc = "Required. The header field name"]
    pub port: PrimField<f64>,
}
impl BuildCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    pub fn build(
        self,
    ) -> CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
        CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
            port: self.port,
            value: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
        CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nRequired. The header field name"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nOptional. The header field value"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElDynamic {
    http_headers: Option<
        DynamicBlock<
            CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_headers:
        Option<Vec<CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl>>,
    dynamic: CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElDynamic,
}
impl CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetEl {
    #[doc = "Set the field `path`.\nOptional. Path to access on the HTTP server. Defaults to '/'."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nOptional. Port number to access on the container. Must be in the range 1 to 65535. If not specified, defaults to the exposed port of the container, which is the value of container.ports[0].containerPort."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `http_headers`.\n"]
    pub fn set_http_headers(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_headers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_headers = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetEl {}
impl BuildCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetEl {
    pub fn build(self) -> CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetEl {
        CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetEl {
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            http_headers: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElRef {
        CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nOptional. Path to access on the HTTP server. Defaults to '/'."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nOptional. Port number to access on the container. Must be in the range 1 to 65535. If not specified, defaults to the exposed port of the container, which is the value of container.ports[0].containerPort."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `http_headers` after provisioning.\n"]
    pub fn http_headers(
        &self,
    ) -> ListRef<CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.http_headers", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketEl {
    #[doc = "Set the field `port`.\nOptional. Port number to access on the container. Must be in the range 1 to 65535. If not specified, defaults to the exposed port of the container, which is the value of container.ports[0].containerPort."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketEl {}
impl BuildCloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketEl {
    pub fn build(self) -> CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketEl {
        CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketEl {
            port: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketElRef {
        CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nOptional. Port number to access on the container. Must be in the range 1 to 65535. If not specified, defaults to the exposed port of the container, which is the value of container.ports[0].containerPort."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElDynamic {
    grpc: Option<DynamicBlock<CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcEl>>,
    http_get:
        Option<DynamicBlock<CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetEl>>,
    tcp_socket:
        Option<DynamicBlock<CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketEl>>,
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTemplateElContainersElStartupProbeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_delay_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc: Option<Vec<CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_get: Option<Vec<CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_socket: Option<Vec<CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketEl>>,
    dynamic: CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElDynamic,
}
impl CloudRunV2WorkerPoolTemplateElContainersElStartupProbeEl {
    #[doc = "Set the field `failure_threshold`.\nOptional. Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1."]
    pub fn set_failure_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.failure_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `initial_delay_seconds`.\nOptional. Number of seconds after the container has started before the probe is initiated. Defaults to 0 seconds. Minimum value is 0. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240."]
    pub fn set_initial_delay_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.initial_delay_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `period_seconds`.\nOptional. How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. Must be greater or equal than timeout_seconds."]
    pub fn set_period_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.period_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout_seconds`.\nOptional. Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. Maximum value is 3600. Must be smaller than period_seconds."]
    pub fn set_timeout_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.grpc = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `http_get`.\n"]
    pub fn set_http_get(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_get = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_get = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tcp_socket`.\n"]
    pub fn set_tcp_socket(
        mut self,
        v: impl Into<
            BlockAssignable<CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tcp_socket = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tcp_socket = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudRunV2WorkerPoolTemplateElContainersElStartupProbeEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolTemplateElContainersElStartupProbeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTemplateElContainersElStartupProbeEl {}
impl BuildCloudRunV2WorkerPoolTemplateElContainersElStartupProbeEl {
    pub fn build(self) -> CloudRunV2WorkerPoolTemplateElContainersElStartupProbeEl {
        CloudRunV2WorkerPoolTemplateElContainersElStartupProbeEl {
            failure_threshold: core::default::Default::default(),
            initial_delay_seconds: core::default::Default::default(),
            period_seconds: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
            grpc: core::default::Default::default(),
            http_get: core::default::Default::default(),
            tcp_socket: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElRef {
        CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `failure_threshold` after provisioning.\nOptional. Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1."]
    pub fn failure_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failure_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `initial_delay_seconds` after provisioning.\nOptional. Number of seconds after the container has started before the probe is initiated. Defaults to 0 seconds. Minimum value is 0. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240."]
    pub fn initial_delay_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_delay_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `period_seconds` after provisioning.\nOptional. How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. Must be greater or equal than timeout_seconds."]
    pub fn period_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.period_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timeout_seconds` after provisioning.\nOptional. Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. Maximum value is 3600. Must be smaller than period_seconds."]
    pub fn timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(
        &self,
    ) -> ListRef<CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElGrpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }
    #[doc = "Get a reference to the value of field `http_get` after provisioning.\n"]
    pub fn http_get(
        &self,
    ) -> ListRef<CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElHttpGetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_get", self.base))
    }
    #[doc = "Get a reference to the value of field `tcp_socket` after provisioning.\n"]
    pub fn tcp_socket(
        &self,
    ) -> ListRef<CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElTcpSocketElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tcp_socket", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTemplateElContainersElVolumeMountsEl {
    mount_path: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_path: Option<PrimField<String>>,
}
impl CloudRunV2WorkerPoolTemplateElContainersElVolumeMountsEl {
    #[doc = "Set the field `sub_path`.\nPath within the volume from which the container's volume should be mounted."]
    pub fn set_sub_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sub_path = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2WorkerPoolTemplateElContainersElVolumeMountsEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolTemplateElContainersElVolumeMountsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTemplateElContainersElVolumeMountsEl {
    #[doc = "Path within the container at which the volume should be mounted. Must not contain ':'. For Cloud SQL volumes, it can be left empty, or must otherwise be /cloudsql. All instances defined in the Volume will be available as /cloudsql/[instance]. For more information on Cloud SQL volumes, visit https://cloud.google.com/sql/docs/mysql/connect-run"]
    pub mount_path: PrimField<String>,
    #[doc = "This must match the Name of a Volume."]
    pub name: PrimField<String>,
}
impl BuildCloudRunV2WorkerPoolTemplateElContainersElVolumeMountsEl {
    pub fn build(self) -> CloudRunV2WorkerPoolTemplateElContainersElVolumeMountsEl {
        CloudRunV2WorkerPoolTemplateElContainersElVolumeMountsEl {
            mount_path: self.mount_path,
            name: self.name,
            sub_path: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolTemplateElContainersElVolumeMountsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTemplateElContainersElVolumeMountsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2WorkerPoolTemplateElContainersElVolumeMountsElRef {
        CloudRunV2WorkerPoolTemplateElContainersElVolumeMountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTemplateElContainersElVolumeMountsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mount_path` after provisioning.\nPath within the container at which the volume should be mounted. Must not contain ':'. For Cloud SQL volumes, it can be left empty, or must otherwise be /cloudsql. All instances defined in the Volume will be available as /cloudsql/[instance]. For more information on Cloud SQL volumes, visit https://cloud.google.com/sql/docs/mysql/connect-run"]
    pub fn mount_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThis must match the Name of a Volume."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `sub_path` after provisioning.\nPath within the volume from which the container's volume should be mounted."]
    pub fn sub_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sub_path", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2WorkerPoolTemplateElContainersElDynamic {
    env: Option<DynamicBlock<CloudRunV2WorkerPoolTemplateElContainersElEnvEl>>,
    liveness_probe: Option<DynamicBlock<CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeEl>>,
    resources: Option<DynamicBlock<CloudRunV2WorkerPoolTemplateElContainersElResourcesEl>>,
    startup_probe: Option<DynamicBlock<CloudRunV2WorkerPoolTemplateElContainersElStartupProbeEl>>,
    volume_mounts: Option<DynamicBlock<CloudRunV2WorkerPoolTemplateElContainersElVolumeMountsEl>>,
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTemplateElContainersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    depends_on: Option<ListField<PrimField<String>>>,
    image: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    working_dir: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<Vec<CloudRunV2WorkerPoolTemplateElContainersElEnvEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    liveness_probe: Option<Vec<CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<Vec<CloudRunV2WorkerPoolTemplateElContainersElResourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    startup_probe: Option<Vec<CloudRunV2WorkerPoolTemplateElContainersElStartupProbeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_mounts: Option<Vec<CloudRunV2WorkerPoolTemplateElContainersElVolumeMountsEl>>,
    dynamic: CloudRunV2WorkerPoolTemplateElContainersElDynamic,
}
impl CloudRunV2WorkerPoolTemplateElContainersEl {
    #[doc = "Set the field `args`.\nArguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references are not supported in Cloud Run."]
    pub fn set_args(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
        self
    }
    #[doc = "Set the field `command`.\nEntrypoint array. Not executed within a shell. The docker image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell"]
    pub fn set_command(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.command = Some(v.into());
        self
    }
    #[doc = "Set the field `depends_on`.\nNames of the containers that must start before this container."]
    pub fn set_depends_on(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.depends_on = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nName of the container specified as a DNS_LABEL."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `working_dir`.\nContainer's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image."]
    pub fn set_working_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.working_dir = Some(v.into());
        self
    }
    #[doc = "Set the field `env`.\n"]
    pub fn set_env(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2WorkerPoolTemplateElContainersElEnvEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.env = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.env = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `liveness_probe`.\n"]
    pub fn set_liveness_probe(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.liveness_probe = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.liveness_probe = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2WorkerPoolTemplateElContainersElResourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resources = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `startup_probe`.\n"]
    pub fn set_startup_probe(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2WorkerPoolTemplateElContainersElStartupProbeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.startup_probe = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.startup_probe = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `volume_mounts`.\n"]
    pub fn set_volume_mounts(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2WorkerPoolTemplateElContainersElVolumeMountsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.volume_mounts = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.volume_mounts = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudRunV2WorkerPoolTemplateElContainersEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolTemplateElContainersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTemplateElContainersEl {
    #[doc = "URL of the Container image in Google Container Registry or Google Artifact Registry. More info: https://kubernetes.io/docs/concepts/containers/images"]
    pub image: PrimField<String>,
}
impl BuildCloudRunV2WorkerPoolTemplateElContainersEl {
    pub fn build(self) -> CloudRunV2WorkerPoolTemplateElContainersEl {
        CloudRunV2WorkerPoolTemplateElContainersEl {
            args: core::default::Default::default(),
            command: core::default::Default::default(),
            depends_on: core::default::Default::default(),
            image: self.image,
            name: core::default::Default::default(),
            working_dir: core::default::Default::default(),
            env: core::default::Default::default(),
            liveness_probe: core::default::Default::default(),
            resources: core::default::Default::default(),
            startup_probe: core::default::Default::default(),
            volume_mounts: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolTemplateElContainersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTemplateElContainersElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2WorkerPoolTemplateElContainersElRef {
        CloudRunV2WorkerPoolTemplateElContainersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTemplateElContainersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `args` after provisioning.\nArguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references are not supported in Cloud Run."]
    pub fn args(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }
    #[doc = "Get a reference to the value of field `command` after provisioning.\nEntrypoint array. Not executed within a shell. The docker image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell"]
    pub fn command(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }
    #[doc = "Get a reference to the value of field `depends_on` after provisioning.\nNames of the containers that must start before this container."]
    pub fn depends_on(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.depends_on", self.base))
    }
    #[doc = "Get a reference to the value of field `image` after provisioning.\nURL of the Container image in Google Container Registry or Google Artifact Registry. More info: https://kubernetes.io/docs/concepts/containers/images"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the container specified as a DNS_LABEL."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `working_dir` after provisioning.\nContainer's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image."]
    pub fn working_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.working_dir", self.base))
    }
    #[doc = "Get a reference to the value of field `liveness_probe` after provisioning.\n"]
    pub fn liveness_probe(
        &self,
    ) -> ListRef<CloudRunV2WorkerPoolTemplateElContainersElLivenessProbeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.liveness_probe", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> ListRef<CloudRunV2WorkerPoolTemplateElContainersElResourcesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
    #[doc = "Get a reference to the value of field `startup_probe` after provisioning.\n"]
    pub fn startup_probe(
        &self,
    ) -> ListRef<CloudRunV2WorkerPoolTemplateElContainersElStartupProbeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.startup_probe", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `volume_mounts` after provisioning.\n"]
    pub fn volume_mounts(
        &self,
    ) -> ListRef<CloudRunV2WorkerPoolTemplateElContainersElVolumeMountsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.volume_mounts", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTemplateElNodeSelectorEl {
    accelerator: PrimField<String>,
}
impl CloudRunV2WorkerPoolTemplateElNodeSelectorEl {}
impl ToListMappable for CloudRunV2WorkerPoolTemplateElNodeSelectorEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolTemplateElNodeSelectorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTemplateElNodeSelectorEl {
    #[doc = "The GPU to attach to an instance. See https://cloud.google.com/run/docs/configuring/services/gpu for configuring GPU."]
    pub accelerator: PrimField<String>,
}
impl BuildCloudRunV2WorkerPoolTemplateElNodeSelectorEl {
    pub fn build(self) -> CloudRunV2WorkerPoolTemplateElNodeSelectorEl {
        CloudRunV2WorkerPoolTemplateElNodeSelectorEl {
            accelerator: self.accelerator,
        }
    }
}
pub struct CloudRunV2WorkerPoolTemplateElNodeSelectorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTemplateElNodeSelectorElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2WorkerPoolTemplateElNodeSelectorElRef {
        CloudRunV2WorkerPoolTemplateElNodeSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTemplateElNodeSelectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accelerator` after provisioning.\nThe GPU to attach to an instance. See https://cloud.google.com/run/docs/configuring/services/gpu for configuring GPU."]
    pub fn accelerator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.accelerator", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instances: Option<SetField<PrimField<String>>>,
}
impl CloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceEl {
    #[doc = "Set the field `instances`.\nThe Cloud SQL instance connection names, as can be found in https://console.cloud.google.com/sql/instances. Visit https://cloud.google.com/sql/docs/mysql/connect-run for more information on how to connect Cloud SQL and Cloud Run. Format: {project}:{location}:{instance}"]
    pub fn set_instances(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.instances = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceEl {}
impl BuildCloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceEl {
    pub fn build(self) -> CloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceEl {
        CloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceEl {
            instances: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceElRef {
        CloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instances` after provisioning.\nThe Cloud SQL instance connection names, as can be found in https://console.cloud.google.com/sql/instances. Visit https://cloud.google.com/sql/docs/mysql/connect-run for more information on how to connect Cloud SQL and Cloud Run. Format: {project}:{location}:{instance}"]
    pub fn instances(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.instances", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTemplateElVolumesElEmptyDirEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    medium: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size_limit: Option<PrimField<String>>,
}
impl CloudRunV2WorkerPoolTemplateElVolumesElEmptyDirEl {
    #[doc = "Set the field `medium`.\nThe different types of medium supported for EmptyDir. Default value: \"MEMORY\" Possible values: [\"MEMORY\", \"DISK\"]"]
    pub fn set_medium(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.medium = Some(v.into());
        self
    }
    #[doc = "Set the field `size_limit`.\nLimit on the storage usable by this EmptyDir volume. The size limit is also applicable for memory medium. The maximum usage on memory medium EmptyDir would be the minimum value between the SizeLimit specified here and the sum of memory limits of all containers in a pod. This field's values are of the 'Quantity' k8s type: https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/. The default is nil which means that the limit is undefined. More info: https://kubernetes.io/docs/concepts/storage/volumes/#emptydir."]
    pub fn set_size_limit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.size_limit = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2WorkerPoolTemplateElVolumesElEmptyDirEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolTemplateElVolumesElEmptyDirEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTemplateElVolumesElEmptyDirEl {}
impl BuildCloudRunV2WorkerPoolTemplateElVolumesElEmptyDirEl {
    pub fn build(self) -> CloudRunV2WorkerPoolTemplateElVolumesElEmptyDirEl {
        CloudRunV2WorkerPoolTemplateElVolumesElEmptyDirEl {
            medium: core::default::Default::default(),
            size_limit: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolTemplateElVolumesElEmptyDirElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTemplateElVolumesElEmptyDirElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2WorkerPoolTemplateElVolumesElEmptyDirElRef {
        CloudRunV2WorkerPoolTemplateElVolumesElEmptyDirElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTemplateElVolumesElEmptyDirElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `medium` after provisioning.\nThe different types of medium supported for EmptyDir. Default value: \"MEMORY\" Possible values: [\"MEMORY\", \"DISK\"]"]
    pub fn medium(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.medium", self.base))
    }
    #[doc = "Get a reference to the value of field `size_limit` after provisioning.\nLimit on the storage usable by this EmptyDir volume. The size limit is also applicable for memory medium. The maximum usage on memory medium EmptyDir would be the minimum value between the SizeLimit specified here and the sum of memory limits of all containers in a pod. This field's values are of the 'Quantity' k8s type: https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/. The default is nil which means that the limit is undefined. More info: https://kubernetes.io/docs/concepts/storage/volumes/#emptydir."]
    pub fn size_limit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_limit", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTemplateElVolumesElGcsEl {
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mount_options: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<PrimField<bool>>,
}
impl CloudRunV2WorkerPoolTemplateElVolumesElGcsEl {
    #[doc = "Set the field `mount_options`.\nA list of flags to pass to the gcsfuse command for configuring this volume.\nFlags should be passed without leading dashes."]
    pub fn set_mount_options(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.mount_options = Some(v.into());
        self
    }
    #[doc = "Set the field `read_only`.\nIf true, mount the GCS bucket as read-only"]
    pub fn set_read_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.read_only = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2WorkerPoolTemplateElVolumesElGcsEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolTemplateElVolumesElGcsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTemplateElVolumesElGcsEl {
    #[doc = "GCS Bucket name"]
    pub bucket: PrimField<String>,
}
impl BuildCloudRunV2WorkerPoolTemplateElVolumesElGcsEl {
    pub fn build(self) -> CloudRunV2WorkerPoolTemplateElVolumesElGcsEl {
        CloudRunV2WorkerPoolTemplateElVolumesElGcsEl {
            bucket: self.bucket,
            mount_options: core::default::Default::default(),
            read_only: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolTemplateElVolumesElGcsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTemplateElVolumesElGcsElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2WorkerPoolTemplateElVolumesElGcsElRef {
        CloudRunV2WorkerPoolTemplateElVolumesElGcsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTemplateElVolumesElGcsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nGCS Bucket name"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `mount_options` after provisioning.\nA list of flags to pass to the gcsfuse command for configuring this volume.\nFlags should be passed without leading dashes."]
    pub fn mount_options(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mount_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `read_only` after provisioning.\nIf true, mount the GCS bucket as read-only"]
    pub fn read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTemplateElVolumesElNfsEl {
    path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<PrimField<bool>>,
    server: PrimField<String>,
}
impl CloudRunV2WorkerPoolTemplateElVolumesElNfsEl {
    #[doc = "Set the field `read_only`.\nIf true, mount the NFS volume as read only"]
    pub fn set_read_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.read_only = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2WorkerPoolTemplateElVolumesElNfsEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolTemplateElVolumesElNfsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTemplateElVolumesElNfsEl {
    #[doc = "Path that is exported by the NFS server."]
    pub path: PrimField<String>,
    #[doc = "Hostname or IP address of the NFS server"]
    pub server: PrimField<String>,
}
impl BuildCloudRunV2WorkerPoolTemplateElVolumesElNfsEl {
    pub fn build(self) -> CloudRunV2WorkerPoolTemplateElVolumesElNfsEl {
        CloudRunV2WorkerPoolTemplateElVolumesElNfsEl {
            path: self.path,
            read_only: core::default::Default::default(),
            server: self.server,
        }
    }
}
pub struct CloudRunV2WorkerPoolTemplateElVolumesElNfsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTemplateElVolumesElNfsElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2WorkerPoolTemplateElVolumesElNfsElRef {
        CloudRunV2WorkerPoolTemplateElVolumesElNfsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTemplateElVolumesElNfsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nPath that is exported by the NFS server."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `read_only` after provisioning.\nIf true, mount the NFS volume as read only"]
    pub fn read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }
    #[doc = "Get a reference to the value of field `server` after provisioning.\nHostname or IP address of the NFS server"]
    pub fn server(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<f64>>,
    path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl CloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsEl {
    #[doc = "Set the field `mode`.\nInteger octal mode bits to use on this file, must be a value between 01 and 0777 (octal). If 0 or not set, the Volume's default mode will be used."]
    pub fn set_mode(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.mode = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\nThe Cloud Secret Manager secret version. Can be 'latest' for the latest value or an integer for a specific version"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsEl {
    #[doc = "The relative path of the secret in the container."]
    pub path: PrimField<String>,
}
impl BuildCloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsEl {
    pub fn build(self) -> CloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsEl {
        CloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsEl {
            mode: core::default::Default::default(),
            path: self.path,
            version: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsElRef {
        CloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nInteger octal mode bits to use on this file, must be a value between 01 and 0777 (octal). If 0 or not set, the Volume's default mode will be used."]
    pub fn mode(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nThe relative path of the secret in the container."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe Cloud Secret Manager secret version. Can be 'latest' for the latest value or an integer for a specific version"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2WorkerPoolTemplateElVolumesElSecretElDynamic {
    items: Option<DynamicBlock<CloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsEl>>,
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTemplateElVolumesElSecretEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_mode: Option<PrimField<f64>>,
    secret: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<Vec<CloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsEl>>,
    dynamic: CloudRunV2WorkerPoolTemplateElVolumesElSecretElDynamic,
}
impl CloudRunV2WorkerPoolTemplateElVolumesElSecretEl {
    #[doc = "Set the field `default_mode`.\nInteger representation of mode bits to use on created files by default. Must be a value between 0000 and 0777 (octal), defaulting to 0444. Directories within the path are not affected by this setting."]
    pub fn set_default_mode(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.items = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.items = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudRunV2WorkerPoolTemplateElVolumesElSecretEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolTemplateElVolumesElSecretEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTemplateElVolumesElSecretEl {
    #[doc = "The name of the secret in Cloud Secret Manager. Format: {secret} if the secret is in the same project. projects/{project}/secrets/{secret} if the secret is in a different project."]
    pub secret: PrimField<String>,
}
impl BuildCloudRunV2WorkerPoolTemplateElVolumesElSecretEl {
    pub fn build(self) -> CloudRunV2WorkerPoolTemplateElVolumesElSecretEl {
        CloudRunV2WorkerPoolTemplateElVolumesElSecretEl {
            default_mode: core::default::Default::default(),
            secret: self.secret,
            items: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolTemplateElVolumesElSecretElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTemplateElVolumesElSecretElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2WorkerPoolTemplateElVolumesElSecretElRef {
        CloudRunV2WorkerPoolTemplateElVolumesElSecretElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTemplateElVolumesElSecretElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_mode` after provisioning.\nInteger representation of mode bits to use on created files by default. Must be a value between 0000 and 0777 (octal), defaulting to 0444. Directories within the path are not affected by this setting."]
    pub fn default_mode(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\nThe name of the secret in Cloud Secret Manager. Format: {secret} if the secret is in the same project. projects/{project}/secrets/{secret} if the secret is in a different project."]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }
    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> ListRef<CloudRunV2WorkerPoolTemplateElVolumesElSecretElItemsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2WorkerPoolTemplateElVolumesElDynamic {
    cloud_sql_instance:
        Option<DynamicBlock<CloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceEl>>,
    empty_dir: Option<DynamicBlock<CloudRunV2WorkerPoolTemplateElVolumesElEmptyDirEl>>,
    gcs: Option<DynamicBlock<CloudRunV2WorkerPoolTemplateElVolumesElGcsEl>>,
    nfs: Option<DynamicBlock<CloudRunV2WorkerPoolTemplateElVolumesElNfsEl>>,
    secret: Option<DynamicBlock<CloudRunV2WorkerPoolTemplateElVolumesElSecretEl>>,
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTemplateElVolumesEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_sql_instance: Option<Vec<CloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    empty_dir: Option<Vec<CloudRunV2WorkerPoolTemplateElVolumesElEmptyDirEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs: Option<Vec<CloudRunV2WorkerPoolTemplateElVolumesElGcsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nfs: Option<Vec<CloudRunV2WorkerPoolTemplateElVolumesElNfsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<Vec<CloudRunV2WorkerPoolTemplateElVolumesElSecretEl>>,
    dynamic: CloudRunV2WorkerPoolTemplateElVolumesElDynamic,
}
impl CloudRunV2WorkerPoolTemplateElVolumesEl {
    #[doc = "Set the field `cloud_sql_instance`.\n"]
    pub fn set_cloud_sql_instance(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloud_sql_instance = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_sql_instance = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `empty_dir`.\n"]
    pub fn set_empty_dir(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2WorkerPoolTemplateElVolumesElEmptyDirEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.empty_dir = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.empty_dir = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `gcs`.\n"]
    pub fn set_gcs(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2WorkerPoolTemplateElVolumesElGcsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gcs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `nfs`.\n"]
    pub fn set_nfs(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2WorkerPoolTemplateElVolumesElNfsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.nfs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.nfs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2WorkerPoolTemplateElVolumesElSecretEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.secret = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.secret = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudRunV2WorkerPoolTemplateElVolumesEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolTemplateElVolumesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTemplateElVolumesEl {
    #[doc = "Volume's name."]
    pub name: PrimField<String>,
}
impl BuildCloudRunV2WorkerPoolTemplateElVolumesEl {
    pub fn build(self) -> CloudRunV2WorkerPoolTemplateElVolumesEl {
        CloudRunV2WorkerPoolTemplateElVolumesEl {
            name: self.name,
            cloud_sql_instance: core::default::Default::default(),
            empty_dir: core::default::Default::default(),
            gcs: core::default::Default::default(),
            nfs: core::default::Default::default(),
            secret: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolTemplateElVolumesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTemplateElVolumesElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2WorkerPoolTemplateElVolumesElRef {
        CloudRunV2WorkerPoolTemplateElVolumesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTemplateElVolumesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nVolume's name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `cloud_sql_instance` after provisioning.\n"]
    pub fn cloud_sql_instance(
        &self,
    ) -> ListRef<CloudRunV2WorkerPoolTemplateElVolumesElCloudSqlInstanceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_sql_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `empty_dir` after provisioning.\n"]
    pub fn empty_dir(&self) -> ListRef<CloudRunV2WorkerPoolTemplateElVolumesElEmptyDirElRef> {
        ListRef::new(self.shared().clone(), format!("{}.empty_dir", self.base))
    }
    #[doc = "Get a reference to the value of field `gcs` after provisioning.\n"]
    pub fn gcs(&self) -> ListRef<CloudRunV2WorkerPoolTemplateElVolumesElGcsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gcs", self.base))
    }
    #[doc = "Get a reference to the value of field `nfs` after provisioning.\n"]
    pub fn nfs(&self) -> ListRef<CloudRunV2WorkerPoolTemplateElVolumesElNfsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.nfs", self.base))
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> ListRef<CloudRunV2WorkerPoolTemplateElVolumesElSecretElRef> {
        ListRef::new(self.shared().clone(), format!("{}.secret", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<PrimField<String>>>,
}
impl CloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesEl {
    #[doc = "Set the field `network`.\nThe VPC network that the Cloud Run resource will be able to send traffic to. At least one of network or subnetwork must be specified. If both\nnetwork and subnetwork are specified, the given VPC subnetwork must belong to the given VPC network. If network is not specified, it will be\nlooked up from the subnetwork."]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\nThe VPC subnetwork that the Cloud Run resource will get IPs from. At least one of network or subnetwork must be specified. If both\nnetwork and subnetwork are specified, the given VPC subnetwork must belong to the given VPC network. If subnetwork is not specified, the\nsubnetwork with the same name with the network will be used."]
    pub fn set_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\nNetwork tags applied to this Cloud Run WorkerPool."]
    pub fn set_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesEl {}
impl BuildCloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesEl {
    pub fn build(self) -> CloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesEl {
        CloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesEl {
            network: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesElRef {
        CloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe VPC network that the Cloud Run resource will be able to send traffic to. At least one of network or subnetwork must be specified. If both\nnetwork and subnetwork are specified, the given VPC subnetwork must belong to the given VPC network. If network is not specified, it will be\nlooked up from the subnetwork."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nThe VPC subnetwork that the Cloud Run resource will get IPs from. At least one of network or subnetwork must be specified. If both\nnetwork and subnetwork are specified, the given VPC subnetwork must belong to the given VPC network. If subnetwork is not specified, the\nsubnetwork with the same name with the network will be used."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nNetwork tags applied to this Cloud Run WorkerPool."]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2WorkerPoolTemplateElVpcAccessElDynamic {
    network_interfaces:
        Option<DynamicBlock<CloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesEl>>,
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTemplateElVpcAccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connector: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interfaces: Option<Vec<CloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesEl>>,
    dynamic: CloudRunV2WorkerPoolTemplateElVpcAccessElDynamic,
}
impl CloudRunV2WorkerPoolTemplateElVpcAccessEl {
    #[doc = "Set the field `connector`.\nVPC Access connector name. Format: projects/{project}/locations/{location}/connectors/{connector}, where {project} can be project id or number."]
    pub fn set_connector(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connector = Some(v.into());
        self
    }
    #[doc = "Set the field `egress`.\nTraffic VPC egress settings. Possible values: [\"ALL_TRAFFIC\", \"PRIVATE_RANGES_ONLY\"]"]
    pub fn set_egress(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.egress = Some(v.into());
        self
    }
    #[doc = "Set the field `network_interfaces`.\n"]
    pub fn set_network_interfaces(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_interfaces = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_interfaces = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudRunV2WorkerPoolTemplateElVpcAccessEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolTemplateElVpcAccessEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTemplateElVpcAccessEl {}
impl BuildCloudRunV2WorkerPoolTemplateElVpcAccessEl {
    pub fn build(self) -> CloudRunV2WorkerPoolTemplateElVpcAccessEl {
        CloudRunV2WorkerPoolTemplateElVpcAccessEl {
            connector: core::default::Default::default(),
            egress: core::default::Default::default(),
            network_interfaces: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolTemplateElVpcAccessElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTemplateElVpcAccessElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2WorkerPoolTemplateElVpcAccessElRef {
        CloudRunV2WorkerPoolTemplateElVpcAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTemplateElVpcAccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connector` after provisioning.\nVPC Access connector name. Format: projects/{project}/locations/{location}/connectors/{connector}, where {project} can be project id or number."]
    pub fn connector(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connector", self.base))
    }
    #[doc = "Get a reference to the value of field `egress` after provisioning.\nTraffic VPC egress settings. Possible values: [\"ALL_TRAFFIC\", \"PRIVATE_RANGES_ONLY\"]"]
    pub fn egress(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.egress", self.base))
    }
    #[doc = "Get a reference to the value of field `network_interfaces` after provisioning.\n"]
    pub fn network_interfaces(
        &self,
    ) -> ListRef<CloudRunV2WorkerPoolTemplateElVpcAccessElNetworkInterfacesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_interfaces", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2WorkerPoolTemplateElDynamic {
    containers: Option<DynamicBlock<CloudRunV2WorkerPoolTemplateElContainersEl>>,
    node_selector: Option<DynamicBlock<CloudRunV2WorkerPoolTemplateElNodeSelectorEl>>,
    volumes: Option<DynamicBlock<CloudRunV2WorkerPoolTemplateElVolumesEl>>,
    vpc_access: Option<DynamicBlock<CloudRunV2WorkerPoolTemplateElVpcAccessEl>>,
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<RecField<PrimField<String>>>,
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
    revision: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    containers: Option<Vec<CloudRunV2WorkerPoolTemplateElContainersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_selector: Option<Vec<CloudRunV2WorkerPoolTemplateElNodeSelectorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volumes: Option<Vec<CloudRunV2WorkerPoolTemplateElVolumesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_access: Option<Vec<CloudRunV2WorkerPoolTemplateElVpcAccessEl>>,
    dynamic: CloudRunV2WorkerPoolTemplateElDynamic,
}
impl CloudRunV2WorkerPoolTemplateEl {
    #[doc = "Set the field `annotations`.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system annotations in v1 now have a corresponding field in v2 WorkerPoolRevisionTemplate.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules."]
    pub fn set_annotations(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_key`.\nA reference to a customer managed encryption key (CMEK) to use to encrypt this container image. For more information, go to https://cloud.google.com/run/docs/securing/using-cmek"]
    pub fn set_encryption_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_key = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_key_revocation_action`.\nThe action to take if the encryption key is revoked. Possible values: [\"PREVENT_NEW\", \"SHUTDOWN\"]"]
    pub fn set_encryption_key_revocation_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_key_revocation_action = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_key_shutdown_duration`.\nIf encryptionKeyRevocationAction is SHUTDOWN, the duration before shutting down all instances. The minimum increment is 1 hour.\n\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn set_encryption_key_shutdown_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_key_shutdown_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `gpu_zonal_redundancy_disabled`.\nTrue if GPU zonal redundancy is disabled on this revision."]
    pub fn set_gpu_zonal_redundancy_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.gpu_zonal_redundancy_disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component, environment, state, etc.\nFor more information, visit https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.\n\nCloud Run API v2 does not support labels with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system labels in v1 now have a corresponding field in v2 WorkerPoolRevisionTemplate."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `revision`.\nThe unique name for the revision. If this field is omitted, it will be automatically generated based on the WorkerPool name."]
    pub fn set_revision(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.revision = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\nEmail address of the IAM service account associated with the revision of the WorkerPool. The service account represents the identity of the running revision, and determines what permissions the revision has. If not provided, the revision will use the project's default service account."]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `containers`.\n"]
    pub fn set_containers(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2WorkerPoolTemplateElContainersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.containers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.containers = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `node_selector`.\n"]
    pub fn set_node_selector(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2WorkerPoolTemplateElNodeSelectorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.node_selector = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.node_selector = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `volumes`.\n"]
    pub fn set_volumes(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2WorkerPoolTemplateElVolumesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.volumes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.volumes = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `vpc_access`.\n"]
    pub fn set_vpc_access(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2WorkerPoolTemplateElVpcAccessEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vpc_access = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_access = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudRunV2WorkerPoolTemplateEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolTemplateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTemplateEl {}
impl BuildCloudRunV2WorkerPoolTemplateEl {
    pub fn build(self) -> CloudRunV2WorkerPoolTemplateEl {
        CloudRunV2WorkerPoolTemplateEl {
            annotations: core::default::Default::default(),
            encryption_key: core::default::Default::default(),
            encryption_key_revocation_action: core::default::Default::default(),
            encryption_key_shutdown_duration: core::default::Default::default(),
            gpu_zonal_redundancy_disabled: core::default::Default::default(),
            labels: core::default::Default::default(),
            revision: core::default::Default::default(),
            service_account: core::default::Default::default(),
            containers: core::default::Default::default(),
            node_selector: core::default::Default::default(),
            volumes: core::default::Default::default(),
            vpc_access: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolTemplateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTemplateElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2WorkerPoolTemplateElRef {
        CloudRunV2WorkerPoolTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system annotations in v1 now have a corresponding field in v2 WorkerPoolRevisionTemplate.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.annotations", self.base))
    }
    #[doc = "Get a reference to the value of field `encryption_key` after provisioning.\nA reference to a customer managed encryption key (CMEK) to use to encrypt this container image. For more information, go to https://cloud.google.com/run/docs/securing/using-cmek"]
    pub fn encryption_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_key_revocation_action` after provisioning.\nThe action to take if the encryption key is revoked. Possible values: [\"PREVENT_NEW\", \"SHUTDOWN\"]"]
    pub fn encryption_key_revocation_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_key_revocation_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_key_shutdown_duration` after provisioning.\nIf encryptionKeyRevocationAction is SHUTDOWN, the duration before shutting down all instances. The minimum increment is 1 hour.\n\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn encryption_key_shutdown_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_key_shutdown_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gpu_zonal_redundancy_disabled` after provisioning.\nTrue if GPU zonal redundancy is disabled on this revision."]
    pub fn gpu_zonal_redundancy_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gpu_zonal_redundancy_disabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component, environment, state, etc.\nFor more information, visit https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.\n\nCloud Run API v2 does not support labels with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system labels in v1 now have a corresponding field in v2 WorkerPoolRevisionTemplate."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `revision` after provisioning.\nThe unique name for the revision. If this field is omitted, it will be automatically generated based on the WorkerPool name."]
    pub fn revision(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nEmail address of the IAM service account associated with the revision of the WorkerPool. The service account represents the identity of the running revision, and determines what permissions the revision has. If not provided, the revision will use the project's default service account."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `containers` after provisioning.\n"]
    pub fn containers(&self) -> ListRef<CloudRunV2WorkerPoolTemplateElContainersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.containers", self.base))
    }
    #[doc = "Get a reference to the value of field `node_selector` after provisioning.\n"]
    pub fn node_selector(&self) -> ListRef<CloudRunV2WorkerPoolTemplateElNodeSelectorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_selector", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(&self) -> ListRef<CloudRunV2WorkerPoolTemplateElVolumesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }
    #[doc = "Get a reference to the value of field `vpc_access` after provisioning.\n"]
    pub fn vpc_access(&self) -> ListRef<CloudRunV2WorkerPoolTemplateElVpcAccessElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_access", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2WorkerPoolTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CloudRunV2WorkerPoolTimeoutsEl {
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
impl ToListMappable for CloudRunV2WorkerPoolTimeoutsEl {
    type O = BlockAssignable<CloudRunV2WorkerPoolTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2WorkerPoolTimeoutsEl {}
impl BuildCloudRunV2WorkerPoolTimeoutsEl {
    pub fn build(self) -> CloudRunV2WorkerPoolTimeoutsEl {
        CloudRunV2WorkerPoolTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2WorkerPoolTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2WorkerPoolTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2WorkerPoolTimeoutsElRef {
        CloudRunV2WorkerPoolTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2WorkerPoolTimeoutsElRef {
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
struct CloudRunV2WorkerPoolDynamic {
    binary_authorization: Option<DynamicBlock<CloudRunV2WorkerPoolBinaryAuthorizationEl>>,
    instance_splits: Option<DynamicBlock<CloudRunV2WorkerPoolInstanceSplitsEl>>,
    scaling: Option<DynamicBlock<CloudRunV2WorkerPoolScalingEl>>,
    template: Option<DynamicBlock<CloudRunV2WorkerPoolTemplateEl>>,
}
