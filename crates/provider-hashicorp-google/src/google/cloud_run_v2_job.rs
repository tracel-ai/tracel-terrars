use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudRunV2JobData {
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
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
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
    binary_authorization: Option<Vec<CloudRunV2JobBinaryAuthorizationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template: Option<Vec<CloudRunV2JobTemplateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CloudRunV2JobTimeoutsEl>,
    dynamic: CloudRunV2JobDynamic,
}
struct CloudRunV2Job_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudRunV2JobData>,
}
#[derive(Clone)]
pub struct CloudRunV2Job(Rc<CloudRunV2Job_>);
impl CloudRunV2Job {
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
    #[doc = "Set the field `annotations`.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected on new resources.\nAll system annotations in v1 now have a corresponding field in v2 Job.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_protection`.\nWhether Terraform will be prevented from destroying the job. Defaults to true.\nWhen a'terraform destroy' or 'terraform apply' would delete the job,\nthe command will fail if this field is not set to false in Terraform state.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the job will fail.\nWhen the field is set to false, deleting the job is allowed."]
    pub fn set_deletion_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component,\nenvironment, state, etc. For more information, visit https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.\n\nCloud Run API v2 does not support labels with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system labels in v1 now have a corresponding field in v2 Job.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
        v: impl Into<BlockAssignable<CloudRunV2JobBinaryAuthorizationEl>>,
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
    #[doc = "Set the field `template`.\n"]
    pub fn set_template(self, v: impl Into<BlockAssignable<CloudRunV2JobTemplateEl>>) -> Self {
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
    pub fn set_timeouts(self, v: impl Into<CloudRunV2JobTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected on new resources.\nAll system annotations in v1 now have a corresponding field in v2 Job.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
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
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\nThe Conditions of all other associated sub-resources. They contain additional diagnostics information in case the Job does not reach its desired state. See comments in reconciling for additional information on 'reconciliation' process in Cloud Run."]
    pub fn conditions(&self) -> ListRef<CloudRunV2JobConditionsElRef> {
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
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the job. Defaults to true.\nWhen a'terraform destroy' or 'terraform apply' would delete the job,\nthe command will fail if this field is not set to false in Terraform state.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the job will fail.\nWhen the field is set to false, deleting the job is allowed."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `execution_count` after provisioning.\nNumber of executions created for this job."]
    pub fn execution_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\nFor a deleted resource, the time after which it will be permanently deleted."]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expire_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generation` after provisioning.\nA number that monotonically increases every time the user modifies the desired state."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component,\nenvironment, state, etc. For more information, visit https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.\n\nCloud Run API v2 does not support labels with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system labels in v1 now have a corresponding field in v2 Job.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
    #[doc = "Get a reference to the value of field `latest_created_execution` after provisioning.\nName of the last created execution."]
    pub fn latest_created_execution(&self) -> ListRef<CloudRunV2JobLatestCreatedExecutionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.latest_created_execution", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `launch_stage` after provisioning.\nThe launch stage as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/products#product-launch-stages). Cloud Run supports ALPHA, BETA, and GA.\nIf no value is specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that stage. On read (or output), describes whether the resource uses preview features.\n\nFor example, if ALPHA is provided as input, but only BETA and GA-level features are used, this field will be BETA on output. Possible values: [\"UNIMPLEMENTED\", \"PRELAUNCH\", \"EARLY_ACCESS\", \"ALPHA\", \"BETA\", \"GA\", \"DEPRECATED\"]"]
    pub fn launch_stage(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.launch_stage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the cloud run job"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Job."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `observed_generation` after provisioning.\nThe generation of this Job. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
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
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nReturns true if the Job is currently being acted upon by the system to bring it into the desired state.\n\nWhen a new Job is created, or an existing one is updated, Cloud Run will asynchronously perform all necessary steps to bring the Job to the desired state. This process is called reconciliation. While reconciliation is in process, observedGeneration and latest_succeeded_execution, will have transient values that might mismatch the intended state: Once reconciliation is over (and this field is false), there are two possible outcomes: reconciliation succeeded and the state matches the Job, or there was an error, and reconciliation failed. This state can be found in terminalCondition.state.\n\nIf reconciliation succeeded, the following fields will match: observedGeneration and generation, latest_succeeded_execution and latestCreatedExecution.\n\nIf reconciliation failed, observedGeneration and latest_succeeded_execution will have the state of the last succeeded execution or empty for newly created Job. Additional information on the failure can be found in terminalCondition and conditions"]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terminal_condition` after provisioning.\nThe Condition of this Job, containing its readiness status, and detailed error information in case it did not reach the desired state"]
    pub fn terminal_condition(&self) -> ListRef<CloudRunV2JobTerminalConditionElRef> {
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
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nServer assigned unique identifier for the Execution. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted."]
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
    pub fn binary_authorization(&self) -> ListRef<CloudRunV2JobBinaryAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.binary_authorization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template` after provisioning.\n"]
    pub fn template(&self) -> ListRef<CloudRunV2JobTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.template", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudRunV2JobTimeoutsElRef {
        CloudRunV2JobTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for CloudRunV2Job {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudRunV2Job {}
impl ToListMappable for CloudRunV2Job {
    type O = ListRef<CloudRunV2JobRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudRunV2Job_ {
    fn extract_resource_type(&self) -> String {
        "google_cloud_run_v2_job".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudRunV2Job {
    pub tf_id: String,
    #[doc = "The location of the cloud run job"]
    pub location: PrimField<String>,
    #[doc = "Name of the Job."]
    pub name: PrimField<String>,
}
impl BuildCloudRunV2Job {
    pub fn build(self, stack: &mut Stack) -> CloudRunV2Job {
        let out = CloudRunV2Job(Rc::new(CloudRunV2Job_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudRunV2JobData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                client: core::default::Default::default(),
                client_version: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                launch_stage: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                binary_authorization: core::default::Default::default(),
                template: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudRunV2JobRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudRunV2JobRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected on new resources.\nAll system annotations in v1 now have a corresponding field in v2 Job.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
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
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\nThe Conditions of all other associated sub-resources. They contain additional diagnostics information in case the Job does not reach its desired state. See comments in reconciling for additional information on 'reconciliation' process in Cloud Run."]
    pub fn conditions(&self) -> ListRef<CloudRunV2JobConditionsElRef> {
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
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the job. Defaults to true.\nWhen a'terraform destroy' or 'terraform apply' would delete the job,\nthe command will fail if this field is not set to false in Terraform state.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the job will fail.\nWhen the field is set to false, deleting the job is allowed."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `execution_count` after provisioning.\nNumber of executions created for this job."]
    pub fn execution_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\nFor a deleted resource, the time after which it will be permanently deleted."]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expire_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generation` after provisioning.\nA number that monotonically increases every time the user modifies the desired state."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component,\nenvironment, state, etc. For more information, visit https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.\n\nCloud Run API v2 does not support labels with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system labels in v1 now have a corresponding field in v2 Job.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
    #[doc = "Get a reference to the value of field `latest_created_execution` after provisioning.\nName of the last created execution."]
    pub fn latest_created_execution(&self) -> ListRef<CloudRunV2JobLatestCreatedExecutionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.latest_created_execution", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `launch_stage` after provisioning.\nThe launch stage as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/products#product-launch-stages). Cloud Run supports ALPHA, BETA, and GA.\nIf no value is specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that stage. On read (or output), describes whether the resource uses preview features.\n\nFor example, if ALPHA is provided as input, but only BETA and GA-level features are used, this field will be BETA on output. Possible values: [\"UNIMPLEMENTED\", \"PRELAUNCH\", \"EARLY_ACCESS\", \"ALPHA\", \"BETA\", \"GA\", \"DEPRECATED\"]"]
    pub fn launch_stage(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.launch_stage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the cloud run job"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Job."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `observed_generation` after provisioning.\nThe generation of this Job. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
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
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nReturns true if the Job is currently being acted upon by the system to bring it into the desired state.\n\nWhen a new Job is created, or an existing one is updated, Cloud Run will asynchronously perform all necessary steps to bring the Job to the desired state. This process is called reconciliation. While reconciliation is in process, observedGeneration and latest_succeeded_execution, will have transient values that might mismatch the intended state: Once reconciliation is over (and this field is false), there are two possible outcomes: reconciliation succeeded and the state matches the Job, or there was an error, and reconciliation failed. This state can be found in terminalCondition.state.\n\nIf reconciliation succeeded, the following fields will match: observedGeneration and generation, latest_succeeded_execution and latestCreatedExecution.\n\nIf reconciliation failed, observedGeneration and latest_succeeded_execution will have the state of the last succeeded execution or empty for newly created Job. Additional information on the failure can be found in terminalCondition and conditions"]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terminal_condition` after provisioning.\nThe Condition of this Job, containing its readiness status, and detailed error information in case it did not reach the desired state"]
    pub fn terminal_condition(&self) -> ListRef<CloudRunV2JobTerminalConditionElRef> {
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
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nServer assigned unique identifier for the Execution. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted."]
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
    pub fn binary_authorization(&self) -> ListRef<CloudRunV2JobBinaryAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.binary_authorization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template` after provisioning.\n"]
    pub fn template(&self) -> ListRef<CloudRunV2JobTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.template", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudRunV2JobTimeoutsElRef {
        CloudRunV2JobTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudRunV2JobConditionsEl {
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
impl CloudRunV2JobConditionsEl {
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
impl ToListMappable for CloudRunV2JobConditionsEl {
    type O = BlockAssignable<CloudRunV2JobConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobConditionsEl {}
impl BuildCloudRunV2JobConditionsEl {
    pub fn build(self) -> CloudRunV2JobConditionsEl {
        CloudRunV2JobConditionsEl {
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
pub struct CloudRunV2JobConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobConditionsElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2JobConditionsElRef {
        CloudRunV2JobConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobConditionsElRef {
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
pub struct CloudRunV2JobLatestCreatedExecutionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    completion_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl CloudRunV2JobLatestCreatedExecutionEl {
    #[doc = "Set the field `completion_time`.\n"]
    pub fn set_completion_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.completion_time = Some(v.into());
        self
    }
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2JobLatestCreatedExecutionEl {
    type O = BlockAssignable<CloudRunV2JobLatestCreatedExecutionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobLatestCreatedExecutionEl {}
impl BuildCloudRunV2JobLatestCreatedExecutionEl {
    pub fn build(self) -> CloudRunV2JobLatestCreatedExecutionEl {
        CloudRunV2JobLatestCreatedExecutionEl {
            completion_time: core::default::Default::default(),
            create_time: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2JobLatestCreatedExecutionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobLatestCreatedExecutionElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2JobLatestCreatedExecutionElRef {
        CloudRunV2JobLatestCreatedExecutionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobLatestCreatedExecutionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `completion_time` after provisioning.\n"]
    pub fn completion_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.completion_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2JobTerminalConditionEl {
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
impl CloudRunV2JobTerminalConditionEl {
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
impl ToListMappable for CloudRunV2JobTerminalConditionEl {
    type O = BlockAssignable<CloudRunV2JobTerminalConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobTerminalConditionEl {}
impl BuildCloudRunV2JobTerminalConditionEl {
    pub fn build(self) -> CloudRunV2JobTerminalConditionEl {
        CloudRunV2JobTerminalConditionEl {
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
pub struct CloudRunV2JobTerminalConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobTerminalConditionElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2JobTerminalConditionElRef {
        CloudRunV2JobTerminalConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobTerminalConditionElRef {
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
pub struct CloudRunV2JobBinaryAuthorizationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    breakglass_justification: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_default: Option<PrimField<bool>>,
}
impl CloudRunV2JobBinaryAuthorizationEl {
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
impl ToListMappable for CloudRunV2JobBinaryAuthorizationEl {
    type O = BlockAssignable<CloudRunV2JobBinaryAuthorizationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobBinaryAuthorizationEl {}
impl BuildCloudRunV2JobBinaryAuthorizationEl {
    pub fn build(self) -> CloudRunV2JobBinaryAuthorizationEl {
        CloudRunV2JobBinaryAuthorizationEl {
            breakglass_justification: core::default::Default::default(),
            policy: core::default::Default::default(),
            use_default: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2JobBinaryAuthorizationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobBinaryAuthorizationElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2JobBinaryAuthorizationElRef {
        CloudRunV2JobBinaryAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobBinaryAuthorizationElRef {
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
pub struct CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    secret: PrimField<String>,
    version: PrimField<String>,
}
impl CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {}
impl ToListMappable
    for CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl
{
    type O = BlockAssignable<
        CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    #[doc = "The name of the secret in Cloud Secret Manager. Format: {secretName} if the secret is in the same project. projects/{project}/secrets/{secretName} if the secret is in a different project."]
    pub secret: PrimField<String>,
    #[doc = "The Cloud Secret Manager secret version. Can be 'latest' for the latest value or an integer for a specific version."]
    pub version: PrimField<String>,
}
impl BuildCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    pub fn build(
        self,
    ) -> CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
        CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
            secret: self.secret,
            version: self.version,
        }
    }
}
pub struct CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
        CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
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
struct CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElDynamic {
    secret_key_ref: Option<
        DynamicBlock<CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl>,
    >,
}
#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_key_ref:
        Option<Vec<CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl>>,
    dynamic: CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElDynamic,
}
impl CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {
    #[doc = "Set the field `secret_key_ref`.\n"]
    pub fn set_secret_key_ref(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl,
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
impl ToListMappable for CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {
    type O = BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {}
impl BuildCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {
        CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {
            secret_key_ref: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElRef {
        CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `secret_key_ref` after provisioning.\n"]
    pub fn secret_key_ref(
        &self,
    ) -> ListRef<CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secret_key_ref", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2JobTemplateElTemplateElContainersElEnvElDynamic {
    value_source:
        Option<DynamicBlock<CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl>>,
}
#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElContainersElEnvEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_source: Option<Vec<CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl>>,
    dynamic: CloudRunV2JobTemplateElTemplateElContainersElEnvElDynamic,
}
impl CloudRunV2JobTemplateElTemplateElContainersElEnvEl {
    #[doc = "Set the field `value`.\nLiteral value of the environment variable. Defaults to \"\" and the maximum allowed length is 32768 characters. Variable references are not supported in Cloud Run."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
    #[doc = "Set the field `value_source`.\n"]
    pub fn set_value_source(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl>>,
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
impl ToListMappable for CloudRunV2JobTemplateElTemplateElContainersElEnvEl {
    type O = BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElEnvEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobTemplateElTemplateElContainersElEnvEl {
    #[doc = "Name of the environment variable. Must be a C_IDENTIFIER, and mnay not exceed 32768 characters."]
    pub name: PrimField<String>,
}
impl BuildCloudRunV2JobTemplateElTemplateElContainersElEnvEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElContainersElEnvEl {
        CloudRunV2JobTemplateElTemplateElContainersElEnvEl {
            name: self.name,
            value: core::default::Default::default(),
            value_source: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2JobTemplateElTemplateElContainersElEnvElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobTemplateElTemplateElContainersElEnvElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2JobTemplateElTemplateElContainersElEnvElRef {
        CloudRunV2JobTemplateElTemplateElContainersElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobTemplateElTemplateElContainersElEnvElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the environment variable. Must be a C_IDENTIFIER, and mnay not exceed 32768 characters."]
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
    ) -> ListRef<CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.value_source", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElContainersElPortsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl CloudRunV2JobTemplateElTemplateElContainersElPortsEl {
    #[doc = "Set the field `container_port`.\nPort number the container listens on. This must be a valid TCP port number, 0 < containerPort < 65536."]
    pub fn set_container_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.container_port = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nIf specified, used to specify which protocol to use. Allowed values are \"http1\" and \"h2c\"."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2JobTemplateElTemplateElContainersElPortsEl {
    type O = BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElPortsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobTemplateElTemplateElContainersElPortsEl {}
impl BuildCloudRunV2JobTemplateElTemplateElContainersElPortsEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElContainersElPortsEl {
        CloudRunV2JobTemplateElTemplateElContainersElPortsEl {
            container_port: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2JobTemplateElTemplateElContainersElPortsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobTemplateElTemplateElContainersElPortsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2JobTemplateElTemplateElContainersElPortsElRef {
        CloudRunV2JobTemplateElTemplateElContainersElPortsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobTemplateElTemplateElContainersElPortsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container_port` after provisioning.\nPort number the container listens on. This must be a valid TCP port number, 0 < containerPort < 65536."]
    pub fn container_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_port", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIf specified, used to specify which protocol to use. Allowed values are \"http1\" and \"h2c\"."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElContainersElResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    limits: Option<RecField<PrimField<String>>>,
}
impl CloudRunV2JobTemplateElTemplateElContainersElResourcesEl {
    #[doc = "Set the field `limits`.\nOnly memory, CPU, and nvidia.com/gpu are supported. Use key 'cpu' for CPU limit, 'memory' for memory limit, 'nvidia.com/gpu' for gpu limit. Note: The only supported values for CPU are '1', '2', '4', '6', and '8'. Setting 4 CPU requires at least 2Gi of memory, setting 6 or more CPU requires at least 4Gi of memory. The values of the map is string form of the 'quantity' k8s type: https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/apimachinery/pkg/api/resource/quantity.go"]
    pub fn set_limits(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.limits = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2JobTemplateElTemplateElContainersElResourcesEl {
    type O = BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobTemplateElTemplateElContainersElResourcesEl {}
impl BuildCloudRunV2JobTemplateElTemplateElContainersElResourcesEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElContainersElResourcesEl {
        CloudRunV2JobTemplateElTemplateElContainersElResourcesEl {
            limits: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2JobTemplateElTemplateElContainersElResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobTemplateElTemplateElContainersElResourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2JobTemplateElTemplateElContainersElResourcesElRef {
        CloudRunV2JobTemplateElTemplateElContainersElResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobTemplateElTemplateElContainersElResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `limits` after provisioning.\nOnly memory, CPU, and nvidia.com/gpu are supported. Use key 'cpu' for CPU limit, 'memory' for memory limit, 'nvidia.com/gpu' for gpu limit. Note: The only supported values for CPU are '1', '2', '4', '6', and '8'. Setting 4 CPU requires at least 2Gi of memory, setting 6 or more CPU requires at least 4Gi of memory. The values of the map is string form of the 'quantity' k8s type: https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/apimachinery/pkg/api/resource/quantity.go"]
    pub fn limits(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
}
impl CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcEl {
    #[doc = "Set the field `port`.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `service`.\nThe name of the service to place in the gRPC HealthCheckRequest\n(see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).\nIf this is not specified, the default behavior is defined by gRPC."]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcEl {
    type O = BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcEl {}
impl BuildCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcEl {
        CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcEl {
            port: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcElRef {
        CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nThe name of the service to place in the gRPC HealthCheckRequest\n(see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).\nIf this is not specified, the default behavior is defined by gRPC."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    #[doc = "Set the field `value`.\nThe header field value."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl
{
    type O = BlockAssignable<
        CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    #[doc = "The header field name."]
    pub name: PrimField<String>,
}
impl BuildCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    pub fn build(
        self,
    ) -> CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
        CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
            name: self.name,
            value: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
        CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe header field name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe header field value."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElDynamic {
    http_headers: Option<
        DynamicBlock<
            CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_headers: Option<
        Vec<CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl>,
    >,
    dynamic: CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElDynamic,
}
impl CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetEl {
    #[doc = "Set the field `path`.\nPath to access on the HTTP server. If set, it should not be empty string."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `http_headers`.\n"]
    pub fn set_http_headers(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl,
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
impl ToListMappable for CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetEl {
    type O = BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetEl {}
impl BuildCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetEl {
        CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetEl {
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            http_headers: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElRef {
        CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nPath to access on the HTTP server. If set, it should not be empty string."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `http_headers` after provisioning.\n"]
    pub fn http_headers(
        &self,
    ) -> ListRef<CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.http_headers", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketEl {
    #[doc = "Set the field `port`.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketEl {
    type O =
        BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketEl {}
impl BuildCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketEl {
        CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketEl {
            port: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketElRef {
        CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElDynamic {
    grpc: Option<DynamicBlock<CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcEl>>,
    http_get:
        Option<DynamicBlock<CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetEl>>,
    tcp_socket: Option<
        DynamicBlock<CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketEl>,
    >,
}
#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElContainersElStartupProbeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_delay_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc: Option<Vec<CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_get: Option<Vec<CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_socket: Option<Vec<CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketEl>>,
    dynamic: CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElDynamic,
}
impl CloudRunV2JobTemplateElTemplateElContainersElStartupProbeEl {
    #[doc = "Set the field `failure_threshold`.\nMinimum consecutive failures for the probe to be considered failed after\nhaving succeeded. Defaults to 3. Minimum value is 1."]
    pub fn set_failure_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.failure_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `initial_delay_seconds`.\nNumber of seconds after the container has started before the probe is\ninitiated.\nDefaults to 0 seconds. Minimum value is 0. Maximum value is 240."]
    pub fn set_initial_delay_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.initial_delay_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `period_seconds`.\nHow often (in seconds) to perform the probe.\nDefault to 10 seconds. Minimum value is 1. Maximum value is 240."]
    pub fn set_period_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.period_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout_seconds`.\nNumber of seconds after which the probe times out.\nDefaults to 1 second. Minimum value is 1. Maximum value is 3600.\nMust be smaller than periodSeconds."]
    pub fn set_timeout_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcEl>>,
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
            BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetEl>,
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
            BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketEl>,
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
impl ToListMappable for CloudRunV2JobTemplateElTemplateElContainersElStartupProbeEl {
    type O = BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElStartupProbeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobTemplateElTemplateElContainersElStartupProbeEl {}
impl BuildCloudRunV2JobTemplateElTemplateElContainersElStartupProbeEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElContainersElStartupProbeEl {
        CloudRunV2JobTemplateElTemplateElContainersElStartupProbeEl {
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
pub struct CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElRef {
        CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `failure_threshold` after provisioning.\nMinimum consecutive failures for the probe to be considered failed after\nhaving succeeded. Defaults to 3. Minimum value is 1."]
    pub fn failure_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failure_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `initial_delay_seconds` after provisioning.\nNumber of seconds after the container has started before the probe is\ninitiated.\nDefaults to 0 seconds. Minimum value is 0. Maximum value is 240."]
    pub fn initial_delay_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_delay_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `period_seconds` after provisioning.\nHow often (in seconds) to perform the probe.\nDefault to 10 seconds. Minimum value is 1. Maximum value is 240."]
    pub fn period_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.period_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timeout_seconds` after provisioning.\nNumber of seconds after which the probe times out.\nDefaults to 1 second. Minimum value is 1. Maximum value is 3600.\nMust be smaller than periodSeconds."]
    pub fn timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(
        &self,
    ) -> ListRef<CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }
    #[doc = "Get a reference to the value of field `http_get` after provisioning.\n"]
    pub fn http_get(
        &self,
    ) -> ListRef<CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_get", self.base))
    }
    #[doc = "Get a reference to the value of field `tcp_socket` after provisioning.\n"]
    pub fn tcp_socket(
        &self,
    ) -> ListRef<CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tcp_socket", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl {
    mount_path: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_path: Option<PrimField<String>>,
}
impl CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl {
    #[doc = "Set the field `sub_path`.\nPath within the volume from which the container's volume should be mounted."]
    pub fn set_sub_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sub_path = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl {
    type O = BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl {
    #[doc = "Path within the container at which the volume should be mounted. Must not contain ':'. For Cloud SQL volumes, it can be left empty, or must otherwise be /cloudsql. All instances defined in the Volume will be available as /cloudsql/[instance]. For more information on Cloud SQL volumes, visit https://cloud.google.com/sql/docs/mysql/connect-run"]
    pub mount_path: PrimField<String>,
    #[doc = "This must match the Name of a Volume."]
    pub name: PrimField<String>,
}
impl BuildCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl {
        CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl {
            mount_path: self.mount_path,
            name: self.name,
            sub_path: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsElRef {
        CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsElRef {
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
struct CloudRunV2JobTemplateElTemplateElContainersElDynamic {
    env: Option<DynamicBlock<CloudRunV2JobTemplateElTemplateElContainersElEnvEl>>,
    ports: Option<DynamicBlock<CloudRunV2JobTemplateElTemplateElContainersElPortsEl>>,
    resources: Option<DynamicBlock<CloudRunV2JobTemplateElTemplateElContainersElResourcesEl>>,
    startup_probe:
        Option<DynamicBlock<CloudRunV2JobTemplateElTemplateElContainersElStartupProbeEl>>,
    volume_mounts:
        Option<DynamicBlock<CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl>>,
}
#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElContainersEl {
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
    env: Option<Vec<CloudRunV2JobTemplateElTemplateElContainersElEnvEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<Vec<CloudRunV2JobTemplateElTemplateElContainersElPortsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<Vec<CloudRunV2JobTemplateElTemplateElContainersElResourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    startup_probe: Option<Vec<CloudRunV2JobTemplateElTemplateElContainersElStartupProbeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_mounts: Option<Vec<CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl>>,
    dynamic: CloudRunV2JobTemplateElTemplateElContainersElDynamic,
}
impl CloudRunV2JobTemplateElTemplateElContainersEl {
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
        v: impl Into<BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElEnvEl>>,
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
    #[doc = "Set the field `ports`.\n"]
    pub fn set_ports(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElPortsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ports = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ports = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElResourcesEl>>,
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
        v: impl Into<BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElStartupProbeEl>>,
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
        v: impl Into<BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl>>,
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
impl ToListMappable for CloudRunV2JobTemplateElTemplateElContainersEl {
    type O = BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobTemplateElTemplateElContainersEl {
    #[doc = "URL of the Container image in Google Container Registry or Google Artifact Registry. More info: https://kubernetes.io/docs/concepts/containers/images"]
    pub image: PrimField<String>,
}
impl BuildCloudRunV2JobTemplateElTemplateElContainersEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElContainersEl {
        CloudRunV2JobTemplateElTemplateElContainersEl {
            args: core::default::Default::default(),
            command: core::default::Default::default(),
            depends_on: core::default::Default::default(),
            image: self.image,
            name: core::default::Default::default(),
            working_dir: core::default::Default::default(),
            env: core::default::Default::default(),
            ports: core::default::Default::default(),
            resources: core::default::Default::default(),
            startup_probe: core::default::Default::default(),
            volume_mounts: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2JobTemplateElTemplateElContainersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobTemplateElTemplateElContainersElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2JobTemplateElTemplateElContainersElRef {
        CloudRunV2JobTemplateElTemplateElContainersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobTemplateElTemplateElContainersElRef {
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
    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> ListRef<CloudRunV2JobTemplateElTemplateElContainersElPortsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(
        &self,
    ) -> ListRef<CloudRunV2JobTemplateElTemplateElContainersElResourcesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
    #[doc = "Get a reference to the value of field `startup_probe` after provisioning.\n"]
    pub fn startup_probe(
        &self,
    ) -> ListRef<CloudRunV2JobTemplateElTemplateElContainersElStartupProbeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.startup_probe", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `volume_mounts` after provisioning.\n"]
    pub fn volume_mounts(
        &self,
    ) -> ListRef<CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.volume_mounts", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElNodeSelectorEl {
    accelerator: PrimField<String>,
}
impl CloudRunV2JobTemplateElTemplateElNodeSelectorEl {}
impl ToListMappable for CloudRunV2JobTemplateElTemplateElNodeSelectorEl {
    type O = BlockAssignable<CloudRunV2JobTemplateElTemplateElNodeSelectorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobTemplateElTemplateElNodeSelectorEl {
    #[doc = "The GPU to attach to an instance. See https://cloud.google.com/run/docs/configuring/jobs/gpu for configuring GPU."]
    pub accelerator: PrimField<String>,
}
impl BuildCloudRunV2JobTemplateElTemplateElNodeSelectorEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElNodeSelectorEl {
        CloudRunV2JobTemplateElTemplateElNodeSelectorEl {
            accelerator: self.accelerator,
        }
    }
}
pub struct CloudRunV2JobTemplateElTemplateElNodeSelectorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobTemplateElTemplateElNodeSelectorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2JobTemplateElTemplateElNodeSelectorElRef {
        CloudRunV2JobTemplateElTemplateElNodeSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobTemplateElTemplateElNodeSelectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accelerator` after provisioning.\nThe GPU to attach to an instance. See https://cloud.google.com/run/docs/configuring/jobs/gpu for configuring GPU."]
    pub fn accelerator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.accelerator", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instances: Option<ListField<PrimField<String>>>,
}
impl CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl {
    #[doc = "Set the field `instances`.\nThe Cloud SQL instance connection names, as can be found in https://console.cloud.google.com/sql/instances. Visit https://cloud.google.com/sql/docs/mysql/connect-run for more information on how to connect Cloud SQL and Cloud Run. Format: {project}:{location}:{instance}"]
    pub fn set_instances(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.instances = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl {
    type O = BlockAssignable<CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl {}
impl BuildCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl {
        CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl {
            instances: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceElRef {
        CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instances` after provisioning.\nThe Cloud SQL instance connection names, as can be found in https://console.cloud.google.com/sql/instances. Visit https://cloud.google.com/sql/docs/mysql/connect-run for more information on how to connect Cloud SQL and Cloud Run. Format: {project}:{location}:{instance}"]
    pub fn instances(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.instances", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElVolumesElEmptyDirEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    medium: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size_limit: Option<PrimField<String>>,
}
impl CloudRunV2JobTemplateElTemplateElVolumesElEmptyDirEl {
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
impl ToListMappable for CloudRunV2JobTemplateElTemplateElVolumesElEmptyDirEl {
    type O = BlockAssignable<CloudRunV2JobTemplateElTemplateElVolumesElEmptyDirEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobTemplateElTemplateElVolumesElEmptyDirEl {}
impl BuildCloudRunV2JobTemplateElTemplateElVolumesElEmptyDirEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElVolumesElEmptyDirEl {
        CloudRunV2JobTemplateElTemplateElVolumesElEmptyDirEl {
            medium: core::default::Default::default(),
            size_limit: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2JobTemplateElTemplateElVolumesElEmptyDirElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobTemplateElTemplateElVolumesElEmptyDirElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2JobTemplateElTemplateElVolumesElEmptyDirElRef {
        CloudRunV2JobTemplateElTemplateElVolumesElEmptyDirElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobTemplateElTemplateElVolumesElEmptyDirElRef {
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
pub struct CloudRunV2JobTemplateElTemplateElVolumesElGcsEl {
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mount_options: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<PrimField<bool>>,
}
impl CloudRunV2JobTemplateElTemplateElVolumesElGcsEl {
    #[doc = "Set the field `mount_options`.\nA list of flags to pass to the gcsfuse command for configuring this volume.\nFlags should be passed without leading dashes."]
    pub fn set_mount_options(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.mount_options = Some(v.into());
        self
    }
    #[doc = "Set the field `read_only`.\nIf true, mount this volume as read-only in all mounts. If false, mount this volume as read-write."]
    pub fn set_read_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.read_only = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2JobTemplateElTemplateElVolumesElGcsEl {
    type O = BlockAssignable<CloudRunV2JobTemplateElTemplateElVolumesElGcsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobTemplateElTemplateElVolumesElGcsEl {
    #[doc = "Name of the cloud storage bucket to back the volume. The resource service account must have permission to access the bucket."]
    pub bucket: PrimField<String>,
}
impl BuildCloudRunV2JobTemplateElTemplateElVolumesElGcsEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElVolumesElGcsEl {
        CloudRunV2JobTemplateElTemplateElVolumesElGcsEl {
            bucket: self.bucket,
            mount_options: core::default::Default::default(),
            read_only: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2JobTemplateElTemplateElVolumesElGcsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobTemplateElTemplateElVolumesElGcsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2JobTemplateElTemplateElVolumesElGcsElRef {
        CloudRunV2JobTemplateElTemplateElVolumesElGcsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobTemplateElTemplateElVolumesElGcsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nName of the cloud storage bucket to back the volume. The resource service account must have permission to access the bucket."]
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
    #[doc = "Get a reference to the value of field `read_only` after provisioning.\nIf true, mount this volume as read-only in all mounts. If false, mount this volume as read-write."]
    pub fn read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElVolumesElNfsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<PrimField<bool>>,
    server: PrimField<String>,
}
impl CloudRunV2JobTemplateElTemplateElVolumesElNfsEl {
    #[doc = "Set the field `path`.\nPath that is exported by the NFS server."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `read_only`.\nIf true, mount this volume as read-only in all mounts."]
    pub fn set_read_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.read_only = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2JobTemplateElTemplateElVolumesElNfsEl {
    type O = BlockAssignable<CloudRunV2JobTemplateElTemplateElVolumesElNfsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobTemplateElTemplateElVolumesElNfsEl {
    #[doc = "Hostname or IP address of the NFS server."]
    pub server: PrimField<String>,
}
impl BuildCloudRunV2JobTemplateElTemplateElVolumesElNfsEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElVolumesElNfsEl {
        CloudRunV2JobTemplateElTemplateElVolumesElNfsEl {
            path: core::default::Default::default(),
            read_only: core::default::Default::default(),
            server: self.server,
        }
    }
}
pub struct CloudRunV2JobTemplateElTemplateElVolumesElNfsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobTemplateElTemplateElVolumesElNfsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2JobTemplateElTemplateElVolumesElNfsElRef {
        CloudRunV2JobTemplateElTemplateElVolumesElNfsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobTemplateElTemplateElVolumesElNfsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nPath that is exported by the NFS server."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `read_only` after provisioning.\nIf true, mount this volume as read-only in all mounts."]
    pub fn read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }
    #[doc = "Get a reference to the value of field `server` after provisioning.\nHostname or IP address of the NFS server."]
    pub fn server(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<f64>>,
    path: PrimField<String>,
    version: PrimField<String>,
}
impl CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {
    #[doc = "Set the field `mode`.\nInteger octal mode bits to use on this file, must be a value between 01 and 0777 (octal). If 0 or not set, the Volume's default mode will be used."]
    pub fn set_mode(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {
    type O = BlockAssignable<CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {
    #[doc = "The relative path of the secret in the container."]
    pub path: PrimField<String>,
    #[doc = "The Cloud Secret Manager secret version. Can be 'latest' for the latest value or an integer for a specific version"]
    pub version: PrimField<String>,
}
impl BuildCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {
        CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {
            mode: core::default::Default::default(),
            path: self.path,
            version: self.version,
        }
    }
}
pub struct CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsElRef {
        CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsElRef {
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
struct CloudRunV2JobTemplateElTemplateElVolumesElSecretElDynamic {
    items: Option<DynamicBlock<CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl>>,
}
#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElVolumesElSecretEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_mode: Option<PrimField<f64>>,
    secret: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<Vec<CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl>>,
    dynamic: CloudRunV2JobTemplateElTemplateElVolumesElSecretElDynamic,
}
impl CloudRunV2JobTemplateElTemplateElVolumesElSecretEl {
    #[doc = "Set the field `default_mode`.\nInteger representation of mode bits to use on created files by default. Must be a value between 0000 and 0777 (octal), defaulting to 0444. Directories within the path are not affected by this setting."]
    pub fn set_default_mode(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl>>,
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
impl ToListMappable for CloudRunV2JobTemplateElTemplateElVolumesElSecretEl {
    type O = BlockAssignable<CloudRunV2JobTemplateElTemplateElVolumesElSecretEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobTemplateElTemplateElVolumesElSecretEl {
    #[doc = "The name of the secret in Cloud Secret Manager. Format: {secret} if the secret is in the same project. projects/{project}/secrets/{secret} if the secret is in a different project."]
    pub secret: PrimField<String>,
}
impl BuildCloudRunV2JobTemplateElTemplateElVolumesElSecretEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElVolumesElSecretEl {
        CloudRunV2JobTemplateElTemplateElVolumesElSecretEl {
            default_mode: core::default::Default::default(),
            secret: self.secret,
            items: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2JobTemplateElTemplateElVolumesElSecretElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobTemplateElTemplateElVolumesElSecretElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2JobTemplateElTemplateElVolumesElSecretElRef {
        CloudRunV2JobTemplateElTemplateElVolumesElSecretElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobTemplateElTemplateElVolumesElSecretElRef {
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
    pub fn items(&self) -> ListRef<CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2JobTemplateElTemplateElVolumesElDynamic {
    cloud_sql_instance:
        Option<DynamicBlock<CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl>>,
    empty_dir: Option<DynamicBlock<CloudRunV2JobTemplateElTemplateElVolumesElEmptyDirEl>>,
    gcs: Option<DynamicBlock<CloudRunV2JobTemplateElTemplateElVolumesElGcsEl>>,
    nfs: Option<DynamicBlock<CloudRunV2JobTemplateElTemplateElVolumesElNfsEl>>,
    secret: Option<DynamicBlock<CloudRunV2JobTemplateElTemplateElVolumesElSecretEl>>,
}
#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElVolumesEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_sql_instance: Option<Vec<CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    empty_dir: Option<Vec<CloudRunV2JobTemplateElTemplateElVolumesElEmptyDirEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs: Option<Vec<CloudRunV2JobTemplateElTemplateElVolumesElGcsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nfs: Option<Vec<CloudRunV2JobTemplateElTemplateElVolumesElNfsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<Vec<CloudRunV2JobTemplateElTemplateElVolumesElSecretEl>>,
    dynamic: CloudRunV2JobTemplateElTemplateElVolumesElDynamic,
}
impl CloudRunV2JobTemplateElTemplateElVolumesEl {
    #[doc = "Set the field `cloud_sql_instance`.\n"]
    pub fn set_cloud_sql_instance(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl>>,
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
        v: impl Into<BlockAssignable<CloudRunV2JobTemplateElTemplateElVolumesElEmptyDirEl>>,
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
        v: impl Into<BlockAssignable<CloudRunV2JobTemplateElTemplateElVolumesElGcsEl>>,
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
        v: impl Into<BlockAssignable<CloudRunV2JobTemplateElTemplateElVolumesElNfsEl>>,
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
        v: impl Into<BlockAssignable<CloudRunV2JobTemplateElTemplateElVolumesElSecretEl>>,
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
impl ToListMappable for CloudRunV2JobTemplateElTemplateElVolumesEl {
    type O = BlockAssignable<CloudRunV2JobTemplateElTemplateElVolumesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobTemplateElTemplateElVolumesEl {
    #[doc = "Volume's name."]
    pub name: PrimField<String>,
}
impl BuildCloudRunV2JobTemplateElTemplateElVolumesEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElVolumesEl {
        CloudRunV2JobTemplateElTemplateElVolumesEl {
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
pub struct CloudRunV2JobTemplateElTemplateElVolumesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobTemplateElTemplateElVolumesElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2JobTemplateElTemplateElVolumesElRef {
        CloudRunV2JobTemplateElTemplateElVolumesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobTemplateElTemplateElVolumesElRef {
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
    ) -> ListRef<CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_sql_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `empty_dir` after provisioning.\n"]
    pub fn empty_dir(&self) -> ListRef<CloudRunV2JobTemplateElTemplateElVolumesElEmptyDirElRef> {
        ListRef::new(self.shared().clone(), format!("{}.empty_dir", self.base))
    }
    #[doc = "Get a reference to the value of field `gcs` after provisioning.\n"]
    pub fn gcs(&self) -> ListRef<CloudRunV2JobTemplateElTemplateElVolumesElGcsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gcs", self.base))
    }
    #[doc = "Get a reference to the value of field `nfs` after provisioning.\n"]
    pub fn nfs(&self) -> ListRef<CloudRunV2JobTemplateElTemplateElVolumesElNfsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.nfs", self.base))
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> ListRef<CloudRunV2JobTemplateElTemplateElVolumesElSecretElRef> {
        ListRef::new(self.shared().clone(), format!("{}.secret", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<PrimField<String>>>,
}
impl CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {
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
    #[doc = "Set the field `tags`.\nNetwork tags applied to this Cloud Run job."]
    pub fn set_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {
    type O = BlockAssignable<CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {}
impl BuildCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {
        CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {
            network: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesElRef {
        CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesElRef {
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
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nNetwork tags applied to this Cloud Run job."]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2JobTemplateElTemplateElVpcAccessElDynamic {
    network_interfaces:
        Option<DynamicBlock<CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl>>,
}
#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElVpcAccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connector: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interfaces:
        Option<Vec<CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl>>,
    dynamic: CloudRunV2JobTemplateElTemplateElVpcAccessElDynamic,
}
impl CloudRunV2JobTemplateElTemplateElVpcAccessEl {
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
        v: impl Into<BlockAssignable<CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl>>,
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
impl ToListMappable for CloudRunV2JobTemplateElTemplateElVpcAccessEl {
    type O = BlockAssignable<CloudRunV2JobTemplateElTemplateElVpcAccessEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobTemplateElTemplateElVpcAccessEl {}
impl BuildCloudRunV2JobTemplateElTemplateElVpcAccessEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElVpcAccessEl {
        CloudRunV2JobTemplateElTemplateElVpcAccessEl {
            connector: core::default::Default::default(),
            egress: core::default::Default::default(),
            network_interfaces: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2JobTemplateElTemplateElVpcAccessElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobTemplateElTemplateElVpcAccessElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2JobTemplateElTemplateElVpcAccessElRef {
        CloudRunV2JobTemplateElTemplateElVpcAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobTemplateElTemplateElVpcAccessElRef {
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
    ) -> ListRef<CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_interfaces", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2JobTemplateElTemplateElDynamic {
    containers: Option<DynamicBlock<CloudRunV2JobTemplateElTemplateElContainersEl>>,
    node_selector: Option<DynamicBlock<CloudRunV2JobTemplateElTemplateElNodeSelectorEl>>,
    volumes: Option<DynamicBlock<CloudRunV2JobTemplateElTemplateElVolumesEl>>,
    vpc_access: Option<DynamicBlock<CloudRunV2JobTemplateElTemplateElVpcAccessEl>>,
}
#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_environment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_zonal_redundancy_disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_retries: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    containers: Option<Vec<CloudRunV2JobTemplateElTemplateElContainersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_selector: Option<Vec<CloudRunV2JobTemplateElTemplateElNodeSelectorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volumes: Option<Vec<CloudRunV2JobTemplateElTemplateElVolumesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_access: Option<Vec<CloudRunV2JobTemplateElTemplateElVpcAccessEl>>,
    dynamic: CloudRunV2JobTemplateElTemplateElDynamic,
}
impl CloudRunV2JobTemplateElTemplateEl {
    #[doc = "Set the field `encryption_key`.\nA reference to a customer managed encryption key (CMEK) to use to encrypt this container image. For more information, go to https://cloud.google.com/run/docs/securing/using-cmek"]
    pub fn set_encryption_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_key = Some(v.into());
        self
    }
    #[doc = "Set the field `execution_environment`.\nThe execution environment being used to host this Task. Possible values: [\"EXECUTION_ENVIRONMENT_GEN1\", \"EXECUTION_ENVIRONMENT_GEN2\"]"]
    pub fn set_execution_environment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.execution_environment = Some(v.into());
        self
    }
    #[doc = "Set the field `gpu_zonal_redundancy_disabled`.\nTrue if GPU zonal redundancy is disabled on this execution."]
    pub fn set_gpu_zonal_redundancy_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.gpu_zonal_redundancy_disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `max_retries`.\nNumber of retries allowed per Task, before marking this Task failed. Defaults to 3. Minimum value is 0."]
    pub fn set_max_retries(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_retries = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\nEmail address of the IAM service account associated with the Task of a Job. The service account represents the identity of the running task, and determines what permissions the task has. If not provided, the task will use the project's default service account."]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout`.\nMax allowed time duration the Task may be active before the system will actively try to mark it failed and kill associated containers. This applies per attempt of a task, meaning each retry can run for the full timeout.\n\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn set_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `containers`.\n"]
    pub fn set_containers(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersEl>>,
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
        v: impl Into<BlockAssignable<CloudRunV2JobTemplateElTemplateElNodeSelectorEl>>,
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
        v: impl Into<BlockAssignable<CloudRunV2JobTemplateElTemplateElVolumesEl>>,
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
        v: impl Into<BlockAssignable<CloudRunV2JobTemplateElTemplateElVpcAccessEl>>,
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
impl ToListMappable for CloudRunV2JobTemplateElTemplateEl {
    type O = BlockAssignable<CloudRunV2JobTemplateElTemplateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobTemplateElTemplateEl {}
impl BuildCloudRunV2JobTemplateElTemplateEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateEl {
        CloudRunV2JobTemplateElTemplateEl {
            encryption_key: core::default::Default::default(),
            execution_environment: core::default::Default::default(),
            gpu_zonal_redundancy_disabled: core::default::Default::default(),
            max_retries: core::default::Default::default(),
            service_account: core::default::Default::default(),
            timeout: core::default::Default::default(),
            containers: core::default::Default::default(),
            node_selector: core::default::Default::default(),
            volumes: core::default::Default::default(),
            vpc_access: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2JobTemplateElTemplateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobTemplateElTemplateElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2JobTemplateElTemplateElRef {
        CloudRunV2JobTemplateElTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobTemplateElTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `encryption_key` after provisioning.\nA reference to a customer managed encryption key (CMEK) to use to encrypt this container image. For more information, go to https://cloud.google.com/run/docs/securing/using-cmek"]
    pub fn encryption_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `execution_environment` after provisioning.\nThe execution environment being used to host this Task. Possible values: [\"EXECUTION_ENVIRONMENT_GEN1\", \"EXECUTION_ENVIRONMENT_GEN2\"]"]
    pub fn execution_environment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_environment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gpu_zonal_redundancy_disabled` after provisioning.\nTrue if GPU zonal redundancy is disabled on this execution."]
    pub fn gpu_zonal_redundancy_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gpu_zonal_redundancy_disabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_retries` after provisioning.\nNumber of retries allowed per Task, before marking this Task failed. Defaults to 3. Minimum value is 0."]
    pub fn max_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nEmail address of the IAM service account associated with the Task of a Job. The service account represents the identity of the running task, and determines what permissions the task has. If not provided, the task will use the project's default service account."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\nMax allowed time duration the Task may be active before the system will actively try to mark it failed and kill associated containers. This applies per attempt of a task, meaning each retry can run for the full timeout.\n\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
    #[doc = "Get a reference to the value of field `containers` after provisioning.\n"]
    pub fn containers(&self) -> ListRef<CloudRunV2JobTemplateElTemplateElContainersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.containers", self.base))
    }
    #[doc = "Get a reference to the value of field `node_selector` after provisioning.\n"]
    pub fn node_selector(&self) -> ListRef<CloudRunV2JobTemplateElTemplateElNodeSelectorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_selector", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(&self) -> ListRef<CloudRunV2JobTemplateElTemplateElVolumesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }
    #[doc = "Get a reference to the value of field `vpc_access` after provisioning.\n"]
    pub fn vpc_access(&self) -> ListRef<CloudRunV2JobTemplateElTemplateElVpcAccessElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_access", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2JobTemplateElDynamic {
    template: Option<DynamicBlock<CloudRunV2JobTemplateElTemplateEl>>,
}
#[derive(Serialize)]
pub struct CloudRunV2JobTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parallelism: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template: Option<Vec<CloudRunV2JobTemplateElTemplateEl>>,
    dynamic: CloudRunV2JobTemplateElDynamic,
}
impl CloudRunV2JobTemplateEl {
    #[doc = "Set the field `annotations`.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system annotations in v1 now have a corresponding field in v2 ExecutionTemplate.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules."]
    pub fn set_annotations(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter,\nor break down billing charges by team, component, environment, state, etc. For more information, visit https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels or\nhttps://cloud.google.com/run/docs/configuring/labels.\n\nCloud Run API v2 does not support labels with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system labels in v1 now have a corresponding field in v2 ExecutionTemplate."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `parallelism`.\nSpecifies the maximum desired number of tasks the execution should run at given time. Must be <= taskCount. When the job is run, if this field is 0 or unset, the maximum possible value will be used for that execution. The actual number of tasks running in steady state will be less than this number when there are fewer tasks waiting to be completed remaining, i.e. when the work left to do is less than max parallelism."]
    pub fn set_parallelism(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.parallelism = Some(v.into());
        self
    }
    #[doc = "Set the field `task_count`.\nSpecifies the desired number of tasks the execution should run. Setting to 1 means that parallelism is limited to 1 and the success of that task signals the success of the execution. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/"]
    pub fn set_task_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.task_count = Some(v.into());
        self
    }
    #[doc = "Set the field `template`.\n"]
    pub fn set_template(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2JobTemplateElTemplateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.template = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.template = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudRunV2JobTemplateEl {
    type O = BlockAssignable<CloudRunV2JobTemplateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobTemplateEl {}
impl BuildCloudRunV2JobTemplateEl {
    pub fn build(self) -> CloudRunV2JobTemplateEl {
        CloudRunV2JobTemplateEl {
            annotations: core::default::Default::default(),
            labels: core::default::Default::default(),
            parallelism: core::default::Default::default(),
            task_count: core::default::Default::default(),
            template: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2JobTemplateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobTemplateElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2JobTemplateElRef {
        CloudRunV2JobTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system annotations in v1 now have a corresponding field in v2 ExecutionTemplate.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.annotations", self.base))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter,\nor break down billing charges by team, component, environment, state, etc. For more information, visit https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels or\nhttps://cloud.google.com/run/docs/configuring/labels.\n\nCloud Run API v2 does not support labels with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system labels in v1 now have a corresponding field in v2 ExecutionTemplate."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `parallelism` after provisioning.\nSpecifies the maximum desired number of tasks the execution should run at given time. Must be <= taskCount. When the job is run, if this field is 0 or unset, the maximum possible value will be used for that execution. The actual number of tasks running in steady state will be less than this number when there are fewer tasks waiting to be completed remaining, i.e. when the work left to do is less than max parallelism."]
    pub fn parallelism(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.parallelism", self.base))
    }
    #[doc = "Get a reference to the value of field `task_count` after provisioning.\nSpecifies the desired number of tasks the execution should run. Setting to 1 means that parallelism is limited to 1 and the success of that task signals the success of the execution. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/"]
    pub fn task_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_count", self.base))
    }
    #[doc = "Get a reference to the value of field `template` after provisioning.\n"]
    pub fn template(&self) -> ListRef<CloudRunV2JobTemplateElTemplateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.template", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2JobTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CloudRunV2JobTimeoutsEl {
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
impl ToListMappable for CloudRunV2JobTimeoutsEl {
    type O = BlockAssignable<CloudRunV2JobTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2JobTimeoutsEl {}
impl BuildCloudRunV2JobTimeoutsEl {
    pub fn build(self) -> CloudRunV2JobTimeoutsEl {
        CloudRunV2JobTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2JobTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2JobTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2JobTimeoutsElRef {
        CloudRunV2JobTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2JobTimeoutsElRef {
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
struct CloudRunV2JobDynamic {
    binary_authorization: Option<DynamicBlock<CloudRunV2JobBinaryAuthorizationEl>>,
    template: Option<DynamicBlock<CloudRunV2JobTemplateEl>>,
}
