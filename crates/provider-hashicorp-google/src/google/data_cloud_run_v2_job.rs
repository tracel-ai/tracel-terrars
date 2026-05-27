use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataCloudRunV2JobData {
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
struct DataCloudRunV2Job_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudRunV2JobData>,
}
#[derive(Clone)]
pub struct DataCloudRunV2Job(Rc<DataCloudRunV2Job_>);
impl DataCloudRunV2Job {
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
    #[doc = "Set the field `location`.\nThe location of the cloud run job"]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected on new resources.\nAll system annotations in v1 now have a corresponding field in v2 Job.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\nSettings for the Binary Authorization feature."]
    pub fn binary_authorization(&self) -> ListRef<DataCloudRunV2JobBinaryAuthorizationElRef> {
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
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\nThe Conditions of all other associated sub-resources. They contain additional diagnostics information in case the Job does not reach its desired state. See comments in reconciling for additional information on 'reconciliation' process in Cloud Run."]
    pub fn conditions(&self) -> ListRef<DataCloudRunV2JobConditionsElRef> {
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
    pub fn latest_created_execution(
        &self,
    ) -> ListRef<DataCloudRunV2JobLatestCreatedExecutionElRef> {
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
    #[doc = "Get a reference to the value of field `template` after provisioning.\nThe template used to create executions for this Job."]
    pub fn template(&self) -> ListRef<DataCloudRunV2JobTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.template", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terminal_condition` after provisioning.\nThe Condition of this Job, containing its readiness status, and detailed error information in case it did not reach the desired state"]
    pub fn terminal_condition(&self) -> ListRef<DataCloudRunV2JobTerminalConditionElRef> {
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
}
impl Referable for DataCloudRunV2Job {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataCloudRunV2Job {}
impl ToListMappable for DataCloudRunV2Job {
    type O = ListRef<DataCloudRunV2JobRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataCloudRunV2Job_ {
    fn extract_datasource_type(&self) -> String {
        "google_cloud_run_v2_job".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataCloudRunV2Job {
    pub tf_id: String,
    #[doc = "Name of the Job."]
    pub name: PrimField<String>,
}
impl BuildDataCloudRunV2Job {
    pub fn build(self, stack: &mut Stack) -> DataCloudRunV2Job {
        let out = DataCloudRunV2Job(Rc::new(DataCloudRunV2Job_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCloudRunV2JobData {
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
pub struct DataCloudRunV2JobRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataCloudRunV2JobRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected on new resources.\nAll system annotations in v1 now have a corresponding field in v2 Job.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\nSettings for the Binary Authorization feature."]
    pub fn binary_authorization(&self) -> ListRef<DataCloudRunV2JobBinaryAuthorizationElRef> {
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
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\nThe Conditions of all other associated sub-resources. They contain additional diagnostics information in case the Job does not reach its desired state. See comments in reconciling for additional information on 'reconciliation' process in Cloud Run."]
    pub fn conditions(&self) -> ListRef<DataCloudRunV2JobConditionsElRef> {
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
    pub fn latest_created_execution(
        &self,
    ) -> ListRef<DataCloudRunV2JobLatestCreatedExecutionElRef> {
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
    #[doc = "Get a reference to the value of field `template` after provisioning.\nThe template used to create executions for this Job."]
    pub fn template(&self) -> ListRef<DataCloudRunV2JobTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.template", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terminal_condition` after provisioning.\nThe Condition of this Job, containing its readiness status, and detailed error information in case it did not reach the desired state"]
    pub fn terminal_condition(&self) -> ListRef<DataCloudRunV2JobTerminalConditionElRef> {
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
}
#[derive(Serialize)]
pub struct DataCloudRunV2JobBinaryAuthorizationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    breakglass_justification: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_default: Option<PrimField<bool>>,
}
impl DataCloudRunV2JobBinaryAuthorizationEl {
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
impl ToListMappable for DataCloudRunV2JobBinaryAuthorizationEl {
    type O = BlockAssignable<DataCloudRunV2JobBinaryAuthorizationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobBinaryAuthorizationEl {}
impl BuildDataCloudRunV2JobBinaryAuthorizationEl {
    pub fn build(self) -> DataCloudRunV2JobBinaryAuthorizationEl {
        DataCloudRunV2JobBinaryAuthorizationEl {
            breakglass_justification: core::default::Default::default(),
            policy: core::default::Default::default(),
            use_default: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2JobBinaryAuthorizationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobBinaryAuthorizationElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2JobBinaryAuthorizationElRef {
        DataCloudRunV2JobBinaryAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobBinaryAuthorizationElRef {
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
pub struct DataCloudRunV2JobConditionsEl {
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
impl DataCloudRunV2JobConditionsEl {
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
impl ToListMappable for DataCloudRunV2JobConditionsEl {
    type O = BlockAssignable<DataCloudRunV2JobConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobConditionsEl {}
impl BuildDataCloudRunV2JobConditionsEl {
    pub fn build(self) -> DataCloudRunV2JobConditionsEl {
        DataCloudRunV2JobConditionsEl {
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
pub struct DataCloudRunV2JobConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobConditionsElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2JobConditionsElRef {
        DataCloudRunV2JobConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobConditionsElRef {
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
pub struct DataCloudRunV2JobLatestCreatedExecutionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    completion_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataCloudRunV2JobLatestCreatedExecutionEl {
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
impl ToListMappable for DataCloudRunV2JobLatestCreatedExecutionEl {
    type O = BlockAssignable<DataCloudRunV2JobLatestCreatedExecutionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobLatestCreatedExecutionEl {}
impl BuildDataCloudRunV2JobLatestCreatedExecutionEl {
    pub fn build(self) -> DataCloudRunV2JobLatestCreatedExecutionEl {
        DataCloudRunV2JobLatestCreatedExecutionEl {
            completion_time: core::default::Default::default(),
            create_time: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2JobLatestCreatedExecutionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobLatestCreatedExecutionElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2JobLatestCreatedExecutionElRef {
        DataCloudRunV2JobLatestCreatedExecutionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobLatestCreatedExecutionElRef {
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
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
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
    for DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl
{
    type O = BlockAssignable<
        DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
}
impl BuildDataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    pub fn build(
        self,
    ) -> DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
        DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
            secret: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
        DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
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
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_key_ref: Option<
        ListField<
            DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl,
        >,
    >,
}
impl DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {
    #[doc = "Set the field `secret_key_ref`.\n"]
    pub fn set_secret_key_ref(
        mut self,
        v: impl Into<
            ListField<
                DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl,
            >,
        >,
    ) -> Self {
        self.secret_key_ref = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {
    type O = BlockAssignable<DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {}
impl BuildDataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {
        DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {
            secret_key_ref: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElRef {
        DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `secret_key_ref` after provisioning.\n"]
    pub fn secret_key_ref(
        &self,
    ) -> ListRef<DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secret_key_ref", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElEnvEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_source:
        Option<ListField<DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl>>,
}
impl DataCloudRunV2JobTemplateElTemplateElContainersElEnvEl {
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
        v: impl Into<ListField<DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl>>,
    ) -> Self {
        self.value_source = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2JobTemplateElTemplateElContainersElEnvEl {
    type O = BlockAssignable<DataCloudRunV2JobTemplateElTemplateElContainersElEnvEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobTemplateElTemplateElContainersElEnvEl {}
impl BuildDataCloudRunV2JobTemplateElTemplateElContainersElEnvEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElContainersElEnvEl {
        DataCloudRunV2JobTemplateElTemplateElContainersElEnvEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
            value_source: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElEnvElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobTemplateElTemplateElContainersElEnvElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2JobTemplateElTemplateElContainersElEnvElRef {
        DataCloudRunV2JobTemplateElTemplateElContainersElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobTemplateElTemplateElContainersElEnvElRef {
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
    ) -> ListRef<DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.value_source", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElPortsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataCloudRunV2JobTemplateElTemplateElContainersElPortsEl {
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
impl ToListMappable for DataCloudRunV2JobTemplateElTemplateElContainersElPortsEl {
    type O = BlockAssignable<DataCloudRunV2JobTemplateElTemplateElContainersElPortsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobTemplateElTemplateElContainersElPortsEl {}
impl BuildDataCloudRunV2JobTemplateElTemplateElContainersElPortsEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElContainersElPortsEl {
        DataCloudRunV2JobTemplateElTemplateElContainersElPortsEl {
            container_port: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElPortsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobTemplateElTemplateElContainersElPortsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2JobTemplateElTemplateElContainersElPortsElRef {
        DataCloudRunV2JobTemplateElTemplateElContainersElPortsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobTemplateElTemplateElContainersElPortsElRef {
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
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    limits: Option<RecField<PrimField<String>>>,
}
impl DataCloudRunV2JobTemplateElTemplateElContainersElResourcesEl {
    #[doc = "Set the field `limits`.\n"]
    pub fn set_limits(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.limits = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2JobTemplateElTemplateElContainersElResourcesEl {
    type O = BlockAssignable<DataCloudRunV2JobTemplateElTemplateElContainersElResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobTemplateElTemplateElContainersElResourcesEl {}
impl BuildDataCloudRunV2JobTemplateElTemplateElContainersElResourcesEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElContainersElResourcesEl {
        DataCloudRunV2JobTemplateElTemplateElContainersElResourcesEl {
            limits: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobTemplateElTemplateElContainersElResourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2JobTemplateElTemplateElContainersElResourcesElRef {
        DataCloudRunV2JobTemplateElTemplateElContainersElResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobTemplateElTemplateElContainersElResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
}
impl DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcEl {
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
impl ToListMappable for DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcEl {
    type O = BlockAssignable<DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcEl {}
impl BuildDataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcEl {
        DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcEl {
            port: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcElRef {
        DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcElRef {
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
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
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
    for DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl
{
    type O = BlockAssignable<
        DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl
{}
impl BuildDataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    pub fn build(
        self,
    ) -> DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
        DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef
    {
        DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
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
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_headers: Option<
        ListField<
            DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetEl {
    #[doc = "Set the field `http_headers`.\n"]
    pub fn set_http_headers(
        mut self,
        v : impl Into < ListField < DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl > >,
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
impl ToListMappable for DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetEl {
    type O =
        BlockAssignable<DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetEl {}
impl BuildDataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetEl {
        DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetEl {
            http_headers: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElRef {
        DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `http_headers` after provisioning.\n"]
    pub fn http_headers(
        &self,
    ) -> ListRef<
        DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef,
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
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketEl {
    type O =
        BlockAssignable<DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketEl {}
impl BuildDataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketEl {
    pub fn build(
        self,
    ) -> DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketEl {
        DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketEl {
            port: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketElRef {
        DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc: Option<ListField<DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_get:
        Option<ListField<DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_delay_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_socket: Option<
        ListField<DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_seconds: Option<PrimField<f64>>,
}
impl DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeEl {
    #[doc = "Set the field `failure_threshold`.\n"]
    pub fn set_failure_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.failure_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v: impl Into<ListField<DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcEl>>,
    ) -> Self {
        self.grpc = Some(v.into());
        self
    }
    #[doc = "Set the field `http_get`.\n"]
    pub fn set_http_get(
        mut self,
        v: impl Into<
            ListField<DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetEl>,
        >,
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
            ListField<DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketEl>,
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
impl ToListMappable for DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeEl {
    type O = BlockAssignable<DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeEl {}
impl BuildDataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeEl {
        DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeEl {
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
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElRef {
        DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElRef {
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
    ) -> ListRef<DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElGrpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }
    #[doc = "Get a reference to the value of field `http_get` after provisioning.\n"]
    pub fn http_get(
        &self,
    ) -> ListRef<DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElHttpGetElRef> {
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
    ) -> ListRef<DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElTcpSocketElRef>
    {
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
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mount_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_path: Option<PrimField<String>>,
}
impl DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl {
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
impl ToListMappable for DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl {
    type O = BlockAssignable<DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl {}
impl BuildDataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl {
        DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl {
            mount_path: core::default::Default::default(),
            name: core::default::Default::default(),
            sub_path: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsElRef {
        DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsElRef {
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
pub struct DataCloudRunV2JobTemplateElTemplateElContainersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    depends_on: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<SetField<DataCloudRunV2JobTemplateElTemplateElContainersElEnvEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<ListField<DataCloudRunV2JobTemplateElTemplateElContainersElPortsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<ListField<DataCloudRunV2JobTemplateElTemplateElContainersElResourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    startup_probe:
        Option<ListField<DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_mounts:
        Option<ListField<DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    working_dir: Option<PrimField<String>>,
}
impl DataCloudRunV2JobTemplateElTemplateElContainersEl {
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
        v: impl Into<SetField<DataCloudRunV2JobTemplateElTemplateElContainersElEnvEl>>,
    ) -> Self {
        self.env = Some(v.into());
        self
    }
    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image = Some(v.into());
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
        v: impl Into<ListField<DataCloudRunV2JobTemplateElTemplateElContainersElPortsEl>>,
    ) -> Self {
        self.ports = Some(v.into());
        self
    }
    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v: impl Into<ListField<DataCloudRunV2JobTemplateElTemplateElContainersElResourcesEl>>,
    ) -> Self {
        self.resources = Some(v.into());
        self
    }
    #[doc = "Set the field `startup_probe`.\n"]
    pub fn set_startup_probe(
        mut self,
        v: impl Into<ListField<DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeEl>>,
    ) -> Self {
        self.startup_probe = Some(v.into());
        self
    }
    #[doc = "Set the field `volume_mounts`.\n"]
    pub fn set_volume_mounts(
        mut self,
        v: impl Into<ListField<DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl>>,
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
impl ToListMappable for DataCloudRunV2JobTemplateElTemplateElContainersEl {
    type O = BlockAssignable<DataCloudRunV2JobTemplateElTemplateElContainersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobTemplateElTemplateElContainersEl {}
impl BuildDataCloudRunV2JobTemplateElTemplateElContainersEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElContainersEl {
        DataCloudRunV2JobTemplateElTemplateElContainersEl {
            args: core::default::Default::default(),
            command: core::default::Default::default(),
            depends_on: core::default::Default::default(),
            env: core::default::Default::default(),
            image: core::default::Default::default(),
            name: core::default::Default::default(),
            ports: core::default::Default::default(),
            resources: core::default::Default::default(),
            startup_probe: core::default::Default::default(),
            volume_mounts: core::default::Default::default(),
            working_dir: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobTemplateElTemplateElContainersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2JobTemplateElTemplateElContainersElRef {
        DataCloudRunV2JobTemplateElTemplateElContainersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobTemplateElTemplateElContainersElRef {
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
    pub fn env(&self) -> SetRef<DataCloudRunV2JobTemplateElTemplateElContainersElEnvElRef> {
        SetRef::new(self.shared().clone(), format!("{}.env", self.base))
    }
    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> ListRef<DataCloudRunV2JobTemplateElTemplateElContainersElPortsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(
        &self,
    ) -> ListRef<DataCloudRunV2JobTemplateElTemplateElContainersElResourcesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
    #[doc = "Get a reference to the value of field `startup_probe` after provisioning.\n"]
    pub fn startup_probe(
        &self,
    ) -> ListRef<DataCloudRunV2JobTemplateElTemplateElContainersElStartupProbeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.startup_probe", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `volume_mounts` after provisioning.\n"]
    pub fn volume_mounts(
        &self,
    ) -> ListRef<DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsElRef> {
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
pub struct DataCloudRunV2JobTemplateElTemplateElNodeSelectorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator: Option<PrimField<String>>,
}
impl DataCloudRunV2JobTemplateElTemplateElNodeSelectorEl {
    #[doc = "Set the field `accelerator`.\n"]
    pub fn set_accelerator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.accelerator = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2JobTemplateElTemplateElNodeSelectorEl {
    type O = BlockAssignable<DataCloudRunV2JobTemplateElTemplateElNodeSelectorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobTemplateElTemplateElNodeSelectorEl {}
impl BuildDataCloudRunV2JobTemplateElTemplateElNodeSelectorEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElNodeSelectorEl {
        DataCloudRunV2JobTemplateElTemplateElNodeSelectorEl {
            accelerator: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2JobTemplateElTemplateElNodeSelectorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobTemplateElTemplateElNodeSelectorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2JobTemplateElTemplateElNodeSelectorElRef {
        DataCloudRunV2JobTemplateElTemplateElNodeSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobTemplateElTemplateElNodeSelectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accelerator` after provisioning.\n"]
    pub fn accelerator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.accelerator", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instances: Option<ListField<PrimField<String>>>,
}
impl DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl {
    #[doc = "Set the field `instances`.\n"]
    pub fn set_instances(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.instances = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl {
    type O = BlockAssignable<DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl {}
impl BuildDataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl {
        DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl {
            instances: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceElRef {
        DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.instances", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2JobTemplateElTemplateElVolumesElEmptyDirEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    medium: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size_limit: Option<PrimField<String>>,
}
impl DataCloudRunV2JobTemplateElTemplateElVolumesElEmptyDirEl {
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
impl ToListMappable for DataCloudRunV2JobTemplateElTemplateElVolumesElEmptyDirEl {
    type O = BlockAssignable<DataCloudRunV2JobTemplateElTemplateElVolumesElEmptyDirEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobTemplateElTemplateElVolumesElEmptyDirEl {}
impl BuildDataCloudRunV2JobTemplateElTemplateElVolumesElEmptyDirEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElVolumesElEmptyDirEl {
        DataCloudRunV2JobTemplateElTemplateElVolumesElEmptyDirEl {
            medium: core::default::Default::default(),
            size_limit: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2JobTemplateElTemplateElVolumesElEmptyDirElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobTemplateElTemplateElVolumesElEmptyDirElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2JobTemplateElTemplateElVolumesElEmptyDirElRef {
        DataCloudRunV2JobTemplateElTemplateElVolumesElEmptyDirElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobTemplateElTemplateElVolumesElEmptyDirElRef {
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
pub struct DataCloudRunV2JobTemplateElTemplateElVolumesElGcsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mount_options: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<PrimField<bool>>,
}
impl DataCloudRunV2JobTemplateElTemplateElVolumesElGcsEl {
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
impl ToListMappable for DataCloudRunV2JobTemplateElTemplateElVolumesElGcsEl {
    type O = BlockAssignable<DataCloudRunV2JobTemplateElTemplateElVolumesElGcsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobTemplateElTemplateElVolumesElGcsEl {}
impl BuildDataCloudRunV2JobTemplateElTemplateElVolumesElGcsEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElVolumesElGcsEl {
        DataCloudRunV2JobTemplateElTemplateElVolumesElGcsEl {
            bucket: core::default::Default::default(),
            mount_options: core::default::Default::default(),
            read_only: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2JobTemplateElTemplateElVolumesElGcsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobTemplateElTemplateElVolumesElGcsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2JobTemplateElTemplateElVolumesElGcsElRef {
        DataCloudRunV2JobTemplateElTemplateElVolumesElGcsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobTemplateElTemplateElVolumesElGcsElRef {
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
pub struct DataCloudRunV2JobTemplateElTemplateElVolumesElNfsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server: Option<PrimField<String>>,
}
impl DataCloudRunV2JobTemplateElTemplateElVolumesElNfsEl {
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
impl ToListMappable for DataCloudRunV2JobTemplateElTemplateElVolumesElNfsEl {
    type O = BlockAssignable<DataCloudRunV2JobTemplateElTemplateElVolumesElNfsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobTemplateElTemplateElVolumesElNfsEl {}
impl BuildDataCloudRunV2JobTemplateElTemplateElVolumesElNfsEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElVolumesElNfsEl {
        DataCloudRunV2JobTemplateElTemplateElVolumesElNfsEl {
            path: core::default::Default::default(),
            read_only: core::default::Default::default(),
            server: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2JobTemplateElTemplateElVolumesElNfsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobTemplateElTemplateElVolumesElNfsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2JobTemplateElTemplateElVolumesElNfsElRef {
        DataCloudRunV2JobTemplateElTemplateElVolumesElNfsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobTemplateElTemplateElVolumesElNfsElRef {
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
pub struct DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {
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
impl ToListMappable for DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {
    type O = BlockAssignable<DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {}
impl BuildDataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {
        DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {
            mode: core::default::Default::default(),
            path: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsElRef {
        DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsElRef {
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
pub struct DataCloudRunV2JobTemplateElTemplateElVolumesElSecretEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_mode: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<ListField<DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<PrimField<String>>,
}
impl DataCloudRunV2JobTemplateElTemplateElVolumesElSecretEl {
    #[doc = "Set the field `default_mode`.\n"]
    pub fn set_default_mode(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl Into<ListField<DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl>>,
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
impl ToListMappable for DataCloudRunV2JobTemplateElTemplateElVolumesElSecretEl {
    type O = BlockAssignable<DataCloudRunV2JobTemplateElTemplateElVolumesElSecretEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobTemplateElTemplateElVolumesElSecretEl {}
impl BuildDataCloudRunV2JobTemplateElTemplateElVolumesElSecretEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElVolumesElSecretEl {
        DataCloudRunV2JobTemplateElTemplateElVolumesElSecretEl {
            default_mode: core::default::Default::default(),
            items: core::default::Default::default(),
            secret: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElRef {
        DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_mode` after provisioning.\n"]
    pub fn default_mode(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(
        &self,
    ) -> ListRef<DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2JobTemplateElTemplateElVolumesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_sql_instance:
        Option<ListField<DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    empty_dir: Option<ListField<DataCloudRunV2JobTemplateElTemplateElVolumesElEmptyDirEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs: Option<ListField<DataCloudRunV2JobTemplateElTemplateElVolumesElGcsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nfs: Option<ListField<DataCloudRunV2JobTemplateElTemplateElVolumesElNfsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<ListField<DataCloudRunV2JobTemplateElTemplateElVolumesElSecretEl>>,
}
impl DataCloudRunV2JobTemplateElTemplateElVolumesEl {
    #[doc = "Set the field `cloud_sql_instance`.\n"]
    pub fn set_cloud_sql_instance(
        mut self,
        v: impl Into<ListField<DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl>>,
    ) -> Self {
        self.cloud_sql_instance = Some(v.into());
        self
    }
    #[doc = "Set the field `empty_dir`.\n"]
    pub fn set_empty_dir(
        mut self,
        v: impl Into<ListField<DataCloudRunV2JobTemplateElTemplateElVolumesElEmptyDirEl>>,
    ) -> Self {
        self.empty_dir = Some(v.into());
        self
    }
    #[doc = "Set the field `gcs`.\n"]
    pub fn set_gcs(
        mut self,
        v: impl Into<ListField<DataCloudRunV2JobTemplateElTemplateElVolumesElGcsEl>>,
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
        v: impl Into<ListField<DataCloudRunV2JobTemplateElTemplateElVolumesElNfsEl>>,
    ) -> Self {
        self.nfs = Some(v.into());
        self
    }
    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(
        mut self,
        v: impl Into<ListField<DataCloudRunV2JobTemplateElTemplateElVolumesElSecretEl>>,
    ) -> Self {
        self.secret = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2JobTemplateElTemplateElVolumesEl {
    type O = BlockAssignable<DataCloudRunV2JobTemplateElTemplateElVolumesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobTemplateElTemplateElVolumesEl {}
impl BuildDataCloudRunV2JobTemplateElTemplateElVolumesEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElVolumesEl {
        DataCloudRunV2JobTemplateElTemplateElVolumesEl {
            cloud_sql_instance: core::default::Default::default(),
            empty_dir: core::default::Default::default(),
            gcs: core::default::Default::default(),
            name: core::default::Default::default(),
            nfs: core::default::Default::default(),
            secret: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2JobTemplateElTemplateElVolumesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobTemplateElTemplateElVolumesElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2JobTemplateElTemplateElVolumesElRef {
        DataCloudRunV2JobTemplateElTemplateElVolumesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobTemplateElTemplateElVolumesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_sql_instance` after provisioning.\n"]
    pub fn cloud_sql_instance(
        &self,
    ) -> ListRef<DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_sql_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `empty_dir` after provisioning.\n"]
    pub fn empty_dir(
        &self,
    ) -> ListRef<DataCloudRunV2JobTemplateElTemplateElVolumesElEmptyDirElRef> {
        ListRef::new(self.shared().clone(), format!("{}.empty_dir", self.base))
    }
    #[doc = "Get a reference to the value of field `gcs` after provisioning.\n"]
    pub fn gcs(&self) -> ListRef<DataCloudRunV2JobTemplateElTemplateElVolumesElGcsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gcs", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `nfs` after provisioning.\n"]
    pub fn nfs(&self) -> ListRef<DataCloudRunV2JobTemplateElTemplateElVolumesElNfsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.nfs", self.base))
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> ListRef<DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElRef> {
        ListRef::new(self.shared().clone(), format!("{}.secret", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<PrimField<String>>>,
}
impl DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {
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
impl ToListMappable for DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {
    type O = BlockAssignable<DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {}
impl BuildDataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {
        DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {
            network: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesElRef {
        DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesElRef {
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
pub struct DataCloudRunV2JobTemplateElTemplateElVpcAccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connector: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interfaces:
        Option<ListField<DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl>>,
}
impl DataCloudRunV2JobTemplateElTemplateElVpcAccessEl {
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
        v: impl Into<ListField<DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl>>,
    ) -> Self {
        self.network_interfaces = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2JobTemplateElTemplateElVpcAccessEl {
    type O = BlockAssignable<DataCloudRunV2JobTemplateElTemplateElVpcAccessEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobTemplateElTemplateElVpcAccessEl {}
impl BuildDataCloudRunV2JobTemplateElTemplateElVpcAccessEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElVpcAccessEl {
        DataCloudRunV2JobTemplateElTemplateElVpcAccessEl {
            connector: core::default::Default::default(),
            egress: core::default::Default::default(),
            network_interfaces: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2JobTemplateElTemplateElVpcAccessElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobTemplateElTemplateElVpcAccessElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudRunV2JobTemplateElTemplateElVpcAccessElRef {
        DataCloudRunV2JobTemplateElTemplateElVpcAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobTemplateElTemplateElVpcAccessElRef {
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
    ) -> ListRef<DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_interfaces", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2JobTemplateElTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    containers: Option<ListField<DataCloudRunV2JobTemplateElTemplateElContainersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_environment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_zonal_redundancy_disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_retries: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_selector: Option<ListField<DataCloudRunV2JobTemplateElTemplateElNodeSelectorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volumes: Option<ListField<DataCloudRunV2JobTemplateElTemplateElVolumesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_access: Option<ListField<DataCloudRunV2JobTemplateElTemplateElVpcAccessEl>>,
}
impl DataCloudRunV2JobTemplateElTemplateEl {
    #[doc = "Set the field `containers`.\n"]
    pub fn set_containers(
        mut self,
        v: impl Into<ListField<DataCloudRunV2JobTemplateElTemplateElContainersEl>>,
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
    #[doc = "Set the field `max_retries`.\n"]
    pub fn set_max_retries(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_retries = Some(v.into());
        self
    }
    #[doc = "Set the field `node_selector`.\n"]
    pub fn set_node_selector(
        mut self,
        v: impl Into<ListField<DataCloudRunV2JobTemplateElTemplateElNodeSelectorEl>>,
    ) -> Self {
        self.node_selector = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
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
        v: impl Into<ListField<DataCloudRunV2JobTemplateElTemplateElVolumesEl>>,
    ) -> Self {
        self.volumes = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_access`.\n"]
    pub fn set_vpc_access(
        mut self,
        v: impl Into<ListField<DataCloudRunV2JobTemplateElTemplateElVpcAccessEl>>,
    ) -> Self {
        self.vpc_access = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2JobTemplateElTemplateEl {
    type O = BlockAssignable<DataCloudRunV2JobTemplateElTemplateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobTemplateElTemplateEl {}
impl BuildDataCloudRunV2JobTemplateElTemplateEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateEl {
        DataCloudRunV2JobTemplateElTemplateEl {
            containers: core::default::Default::default(),
            encryption_key: core::default::Default::default(),
            execution_environment: core::default::Default::default(),
            gpu_zonal_redundancy_disabled: core::default::Default::default(),
            max_retries: core::default::Default::default(),
            node_selector: core::default::Default::default(),
            service_account: core::default::Default::default(),
            timeout: core::default::Default::default(),
            volumes: core::default::Default::default(),
            vpc_access: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2JobTemplateElTemplateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobTemplateElTemplateElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2JobTemplateElTemplateElRef {
        DataCloudRunV2JobTemplateElTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobTemplateElTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `containers` after provisioning.\n"]
    pub fn containers(&self) -> ListRef<DataCloudRunV2JobTemplateElTemplateElContainersElRef> {
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
    #[doc = "Get a reference to the value of field `max_retries` after provisioning.\n"]
    pub fn max_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.base))
    }
    #[doc = "Get a reference to the value of field `node_selector` after provisioning.\n"]
    pub fn node_selector(&self) -> ListRef<DataCloudRunV2JobTemplateElTemplateElNodeSelectorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_selector", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(&self) -> ListRef<DataCloudRunV2JobTemplateElTemplateElVolumesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }
    #[doc = "Get a reference to the value of field `vpc_access` after provisioning.\n"]
    pub fn vpc_access(&self) -> ListRef<DataCloudRunV2JobTemplateElTemplateElVpcAccessElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_access", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2JobTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parallelism: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template: Option<ListField<DataCloudRunV2JobTemplateElTemplateEl>>,
}
impl DataCloudRunV2JobTemplateEl {
    #[doc = "Set the field `annotations`.\n"]
    pub fn set_annotations(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `parallelism`.\n"]
    pub fn set_parallelism(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.parallelism = Some(v.into());
        self
    }
    #[doc = "Set the field `task_count`.\n"]
    pub fn set_task_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.task_count = Some(v.into());
        self
    }
    #[doc = "Set the field `template`.\n"]
    pub fn set_template(
        mut self,
        v: impl Into<ListField<DataCloudRunV2JobTemplateElTemplateEl>>,
    ) -> Self {
        self.template = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudRunV2JobTemplateEl {
    type O = BlockAssignable<DataCloudRunV2JobTemplateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobTemplateEl {}
impl BuildDataCloudRunV2JobTemplateEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateEl {
        DataCloudRunV2JobTemplateEl {
            annotations: core::default::Default::default(),
            labels: core::default::Default::default(),
            parallelism: core::default::Default::default(),
            task_count: core::default::Default::default(),
            template: core::default::Default::default(),
        }
    }
}
pub struct DataCloudRunV2JobTemplateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobTemplateElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2JobTemplateElRef {
        DataCloudRunV2JobTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\n"]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.annotations", self.base))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `parallelism` after provisioning.\n"]
    pub fn parallelism(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.parallelism", self.base))
    }
    #[doc = "Get a reference to the value of field `task_count` after provisioning.\n"]
    pub fn task_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_count", self.base))
    }
    #[doc = "Get a reference to the value of field `template` after provisioning.\n"]
    pub fn template(&self) -> ListRef<DataCloudRunV2JobTemplateElTemplateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.template", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudRunV2JobTerminalConditionEl {
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
impl DataCloudRunV2JobTerminalConditionEl {
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
impl ToListMappable for DataCloudRunV2JobTerminalConditionEl {
    type O = BlockAssignable<DataCloudRunV2JobTerminalConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudRunV2JobTerminalConditionEl {}
impl BuildDataCloudRunV2JobTerminalConditionEl {
    pub fn build(self) -> DataCloudRunV2JobTerminalConditionEl {
        DataCloudRunV2JobTerminalConditionEl {
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
pub struct DataCloudRunV2JobTerminalConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudRunV2JobTerminalConditionElRef {
    fn new(shared: StackShared, base: String) -> DataCloudRunV2JobTerminalConditionElRef {
        DataCloudRunV2JobTerminalConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudRunV2JobTerminalConditionElRef {
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
