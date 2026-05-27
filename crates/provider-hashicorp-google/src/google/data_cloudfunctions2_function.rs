use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataCloudfunctions2FunctionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataCloudfunctions2Function_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudfunctions2FunctionData>,
}
#[derive(Clone)]
pub struct DataCloudfunctions2Function(Rc<DataCloudfunctions2Function_>);
impl DataCloudfunctions2Function {
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
    #[doc = "Get a reference to the value of field `build_config` after provisioning.\nDescribes the Build step of the function that builds a container\nfrom the given source."]
    pub fn build_config(&self) -> ListRef<DataCloudfunctions2FunctionBuildConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.build_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-provided description of a function."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `environment` after provisioning.\nThe environment the function is hosted on."]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.environment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_trigger` after provisioning.\nAn Eventarc trigger managed by Google Cloud Functions that fires events in\nresponse to a condition in another service."]
    pub fn event_trigger(&self) -> ListRef<DataCloudfunctions2FunctionEventTriggerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.event_trigger", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nResource name of a KMS crypto key (managed by the user) used to encrypt/decrypt function resources.\nIt must match the pattern projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs associated with this Cloud Function.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of this cloud function."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA user-defined name of the function. Function names must\nbe unique globally and match pattern 'projects/*/locations/*/functions/*'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_config` after provisioning.\nDescribes the Service being deployed."]
    pub fn service_config(&self) -> ListRef<DataCloudfunctions2FunctionServiceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nDescribes the current state of the function."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe last update timestamp of a Cloud Function."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nOutput only. The deployed url for the function."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
}
impl Referable for DataCloudfunctions2Function {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataCloudfunctions2Function {}
impl ToListMappable for DataCloudfunctions2Function {
    type O = ListRef<DataCloudfunctions2FunctionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataCloudfunctions2Function_ {
    fn extract_datasource_type(&self) -> String {
        "google_cloudfunctions2_function".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataCloudfunctions2Function {
    pub tf_id: String,
    #[doc = "The location of this cloud function."]
    pub location: PrimField<String>,
    #[doc = "A user-defined name of the function. Function names must\nbe unique globally and match pattern 'projects/*/locations/*/functions/*'."]
    pub name: PrimField<String>,
}
impl BuildDataCloudfunctions2Function {
    pub fn build(self, stack: &mut Stack) -> DataCloudfunctions2Function {
        let out = DataCloudfunctions2Function(Rc::new(DataCloudfunctions2Function_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCloudfunctions2FunctionData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataCloudfunctions2FunctionRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfunctions2FunctionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataCloudfunctions2FunctionRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `build_config` after provisioning.\nDescribes the Build step of the function that builds a container\nfrom the given source."]
    pub fn build_config(&self) -> ListRef<DataCloudfunctions2FunctionBuildConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.build_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-provided description of a function."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `environment` after provisioning.\nThe environment the function is hosted on."]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.environment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_trigger` after provisioning.\nAn Eventarc trigger managed by Google Cloud Functions that fires events in\nresponse to a condition in another service."]
    pub fn event_trigger(&self) -> ListRef<DataCloudfunctions2FunctionEventTriggerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.event_trigger", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nResource name of a KMS crypto key (managed by the user) used to encrypt/decrypt function resources.\nIt must match the pattern projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs associated with this Cloud Function.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of this cloud function."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA user-defined name of the function. Function names must\nbe unique globally and match pattern 'projects/*/locations/*/functions/*'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_config` after provisioning.\nDescribes the Service being deployed."]
    pub fn service_config(&self) -> ListRef<DataCloudfunctions2FunctionServiceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nDescribes the current state of the function."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe last update timestamp of a Cloud Function."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nOutput only. The deployed url for the function."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
}
#[derive(Serialize)]
pub struct DataCloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyEl {}
impl DataCloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyEl {}
impl ToListMappable for DataCloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyEl {
    type O = BlockAssignable<DataCloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyEl {}
impl BuildDataCloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyEl {
    pub fn build(self) -> DataCloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyEl {
        DataCloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyEl {}
    }
}
pub struct DataCloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyElRef {
        DataCloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DataCloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime_version: Option<PrimField<String>>,
}
impl DataCloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyEl {
    #[doc = "Set the field `runtime_version`.\n"]
    pub fn set_runtime_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.runtime_version = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyEl {
    type O = BlockAssignable<DataCloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyEl {}
impl BuildDataCloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyEl {
    pub fn build(self) -> DataCloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyEl {
        DataCloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyEl {
            runtime_version: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyElRef {
        DataCloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `runtime_version` after provisioning.\n"]
    pub fn runtime_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.runtime_version", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudfunctions2FunctionBuildConfigElSourceElRepoSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_sha: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dir: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invert_regex: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_name: Option<PrimField<String>>,
}
impl DataCloudfunctions2FunctionBuildConfigElSourceElRepoSourceEl {
    #[doc = "Set the field `branch_name`.\n"]
    pub fn set_branch_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch_name = Some(v.into());
        self
    }
    #[doc = "Set the field `commit_sha`.\n"]
    pub fn set_commit_sha(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.commit_sha = Some(v.into());
        self
    }
    #[doc = "Set the field `dir`.\n"]
    pub fn set_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dir = Some(v.into());
        self
    }
    #[doc = "Set the field `invert_regex`.\n"]
    pub fn set_invert_regex(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert_regex = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
    #[doc = "Set the field `repo_name`.\n"]
    pub fn set_repo_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo_name = Some(v.into());
        self
    }
    #[doc = "Set the field `tag_name`.\n"]
    pub fn set_tag_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudfunctions2FunctionBuildConfigElSourceElRepoSourceEl {
    type O = BlockAssignable<DataCloudfunctions2FunctionBuildConfigElSourceElRepoSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfunctions2FunctionBuildConfigElSourceElRepoSourceEl {}
impl BuildDataCloudfunctions2FunctionBuildConfigElSourceElRepoSourceEl {
    pub fn build(self) -> DataCloudfunctions2FunctionBuildConfigElSourceElRepoSourceEl {
        DataCloudfunctions2FunctionBuildConfigElSourceElRepoSourceEl {
            branch_name: core::default::Default::default(),
            commit_sha: core::default::Default::default(),
            dir: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
            project_id: core::default::Default::default(),
            repo_name: core::default::Default::default(),
            tag_name: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfunctions2FunctionBuildConfigElSourceElRepoSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfunctions2FunctionBuildConfigElSourceElRepoSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfunctions2FunctionBuildConfigElSourceElRepoSourceElRef {
        DataCloudfunctions2FunctionBuildConfigElSourceElRepoSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfunctions2FunctionBuildConfigElSourceElRepoSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `branch_name` after provisioning.\n"]
    pub fn branch_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch_name", self.base))
    }
    #[doc = "Get a reference to the value of field `commit_sha` after provisioning.\n"]
    pub fn commit_sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_sha", self.base))
    }
    #[doc = "Get a reference to the value of field `dir` after provisioning.\n"]
    pub fn dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dir", self.base))
    }
    #[doc = "Get a reference to the value of field `invert_regex` after provisioning.\n"]
    pub fn invert_regex(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `repo_name` after provisioning.\n"]
    pub fn repo_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_name", self.base))
    }
    #[doc = "Get a reference to the value of field `tag_name` after provisioning.\n"]
    pub fn tag_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudfunctions2FunctionBuildConfigElSourceElStorageSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object: Option<PrimField<String>>,
}
impl DataCloudfunctions2FunctionBuildConfigElSourceElStorageSourceEl {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket = Some(v.into());
        self
    }
    #[doc = "Set the field `generation`.\n"]
    pub fn set_generation(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.generation = Some(v.into());
        self
    }
    #[doc = "Set the field `object`.\n"]
    pub fn set_object(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.object = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudfunctions2FunctionBuildConfigElSourceElStorageSourceEl {
    type O = BlockAssignable<DataCloudfunctions2FunctionBuildConfigElSourceElStorageSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfunctions2FunctionBuildConfigElSourceElStorageSourceEl {}
impl BuildDataCloudfunctions2FunctionBuildConfigElSourceElStorageSourceEl {
    pub fn build(self) -> DataCloudfunctions2FunctionBuildConfigElSourceElStorageSourceEl {
        DataCloudfunctions2FunctionBuildConfigElSourceElStorageSourceEl {
            bucket: core::default::Default::default(),
            generation: core::default::Default::default(),
            object: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfunctions2FunctionBuildConfigElSourceElStorageSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfunctions2FunctionBuildConfigElSourceElStorageSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfunctions2FunctionBuildConfigElSourceElStorageSourceElRef {
        DataCloudfunctions2FunctionBuildConfigElSourceElStorageSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfunctions2FunctionBuildConfigElSourceElStorageSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `generation` after provisioning.\n"]
    pub fn generation(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.generation", self.base))
    }
    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudfunctions2FunctionBuildConfigElSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_source: Option<ListField<DataCloudfunctions2FunctionBuildConfigElSourceElRepoSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_source:
        Option<ListField<DataCloudfunctions2FunctionBuildConfigElSourceElStorageSourceEl>>,
}
impl DataCloudfunctions2FunctionBuildConfigElSourceEl {
    #[doc = "Set the field `repo_source`.\n"]
    pub fn set_repo_source(
        mut self,
        v: impl Into<ListField<DataCloudfunctions2FunctionBuildConfigElSourceElRepoSourceEl>>,
    ) -> Self {
        self.repo_source = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_source`.\n"]
    pub fn set_storage_source(
        mut self,
        v: impl Into<ListField<DataCloudfunctions2FunctionBuildConfigElSourceElStorageSourceEl>>,
    ) -> Self {
        self.storage_source = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudfunctions2FunctionBuildConfigElSourceEl {
    type O = BlockAssignable<DataCloudfunctions2FunctionBuildConfigElSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfunctions2FunctionBuildConfigElSourceEl {}
impl BuildDataCloudfunctions2FunctionBuildConfigElSourceEl {
    pub fn build(self) -> DataCloudfunctions2FunctionBuildConfigElSourceEl {
        DataCloudfunctions2FunctionBuildConfigElSourceEl {
            repo_source: core::default::Default::default(),
            storage_source: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfunctions2FunctionBuildConfigElSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfunctions2FunctionBuildConfigElSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfunctions2FunctionBuildConfigElSourceElRef {
        DataCloudfunctions2FunctionBuildConfigElSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfunctions2FunctionBuildConfigElSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `repo_source` after provisioning.\n"]
    pub fn repo_source(
        &self,
    ) -> ListRef<DataCloudfunctions2FunctionBuildConfigElSourceElRepoSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.repo_source", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_source` after provisioning.\n"]
    pub fn storage_source(
        &self,
    ) -> ListRef<DataCloudfunctions2FunctionBuildConfigElSourceElStorageSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_source", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudfunctions2FunctionBuildConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_update_policy:
        Option<ListField<DataCloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    docker_repository: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entry_point: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_variables: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_deploy_update_policy:
        Option<ListField<DataCloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<ListField<DataCloudfunctions2FunctionBuildConfigElSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_pool: Option<PrimField<String>>,
}
impl DataCloudfunctions2FunctionBuildConfigEl {
    #[doc = "Set the field `automatic_update_policy`.\n"]
    pub fn set_automatic_update_policy(
        mut self,
        v: impl Into<ListField<DataCloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyEl>>,
    ) -> Self {
        self.automatic_update_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `build`.\n"]
    pub fn set_build(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.build = Some(v.into());
        self
    }
    #[doc = "Set the field `docker_repository`.\n"]
    pub fn set_docker_repository(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.docker_repository = Some(v.into());
        self
    }
    #[doc = "Set the field `entry_point`.\n"]
    pub fn set_entry_point(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.entry_point = Some(v.into());
        self
    }
    #[doc = "Set the field `environment_variables`.\n"]
    pub fn set_environment_variables(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.environment_variables = Some(v.into());
        self
    }
    #[doc = "Set the field `on_deploy_update_policy`.\n"]
    pub fn set_on_deploy_update_policy(
        mut self,
        v: impl Into<ListField<DataCloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyEl>>,
    ) -> Self {
        self.on_deploy_update_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `runtime`.\n"]
    pub fn set_runtime(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.runtime = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v: impl Into<ListField<DataCloudfunctions2FunctionBuildConfigElSourceEl>>,
    ) -> Self {
        self.source = Some(v.into());
        self
    }
    #[doc = "Set the field `worker_pool`.\n"]
    pub fn set_worker_pool(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.worker_pool = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudfunctions2FunctionBuildConfigEl {
    type O = BlockAssignable<DataCloudfunctions2FunctionBuildConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfunctions2FunctionBuildConfigEl {}
impl BuildDataCloudfunctions2FunctionBuildConfigEl {
    pub fn build(self) -> DataCloudfunctions2FunctionBuildConfigEl {
        DataCloudfunctions2FunctionBuildConfigEl {
            automatic_update_policy: core::default::Default::default(),
            build: core::default::Default::default(),
            docker_repository: core::default::Default::default(),
            entry_point: core::default::Default::default(),
            environment_variables: core::default::Default::default(),
            on_deploy_update_policy: core::default::Default::default(),
            runtime: core::default::Default::default(),
            service_account: core::default::Default::default(),
            source: core::default::Default::default(),
            worker_pool: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfunctions2FunctionBuildConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfunctions2FunctionBuildConfigElRef {
    fn new(shared: StackShared, base: String) -> DataCloudfunctions2FunctionBuildConfigElRef {
        DataCloudfunctions2FunctionBuildConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfunctions2FunctionBuildConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `automatic_update_policy` after provisioning.\n"]
    pub fn automatic_update_policy(
        &self,
    ) -> ListRef<DataCloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.automatic_update_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `build` after provisioning.\n"]
    pub fn build(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build", self.base))
    }
    #[doc = "Get a reference to the value of field `docker_repository` after provisioning.\n"]
    pub fn docker_repository(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.docker_repository", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `entry_point` after provisioning.\n"]
    pub fn entry_point(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.entry_point", self.base))
    }
    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\n"]
    pub fn environment_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.environment_variables", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `on_deploy_update_policy` after provisioning.\n"]
    pub fn on_deploy_update_policy(
        &self,
    ) -> ListRef<DataCloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.on_deploy_update_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `runtime` after provisioning.\n"]
    pub fn runtime(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.runtime", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<DataCloudfunctions2FunctionBuildConfigElSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
    #[doc = "Get a reference to the value of field `worker_pool` after provisioning.\n"]
    pub fn worker_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.worker_pool", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudfunctions2FunctionEventTriggerElEventFiltersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataCloudfunctions2FunctionEventTriggerElEventFiltersEl {
    #[doc = "Set the field `attribute`.\n"]
    pub fn set_attribute(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attribute = Some(v.into());
        self
    }
    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operator = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudfunctions2FunctionEventTriggerElEventFiltersEl {
    type O = BlockAssignable<DataCloudfunctions2FunctionEventTriggerElEventFiltersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfunctions2FunctionEventTriggerElEventFiltersEl {}
impl BuildDataCloudfunctions2FunctionEventTriggerElEventFiltersEl {
    pub fn build(self) -> DataCloudfunctions2FunctionEventTriggerElEventFiltersEl {
        DataCloudfunctions2FunctionEventTriggerElEventFiltersEl {
            attribute: core::default::Default::default(),
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfunctions2FunctionEventTriggerElEventFiltersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfunctions2FunctionEventTriggerElEventFiltersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfunctions2FunctionEventTriggerElEventFiltersElRef {
        DataCloudfunctions2FunctionEventTriggerElEventFiltersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfunctions2FunctionEventTriggerElEventFiltersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudfunctions2FunctionEventTriggerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    event_filters: Option<SetField<DataCloudfunctions2FunctionEventTriggerElEventFiltersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pubsub_topic: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger_region: Option<PrimField<String>>,
}
impl DataCloudfunctions2FunctionEventTriggerEl {
    #[doc = "Set the field `event_filters`.\n"]
    pub fn set_event_filters(
        mut self,
        v: impl Into<SetField<DataCloudfunctions2FunctionEventTriggerElEventFiltersEl>>,
    ) -> Self {
        self.event_filters = Some(v.into());
        self
    }
    #[doc = "Set the field `event_type`.\n"]
    pub fn set_event_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.event_type = Some(v.into());
        self
    }
    #[doc = "Set the field `pubsub_topic`.\n"]
    pub fn set_pubsub_topic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pubsub_topic = Some(v.into());
        self
    }
    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.retry_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account_email`.\n"]
    pub fn set_service_account_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account_email = Some(v.into());
        self
    }
    #[doc = "Set the field `trigger`.\n"]
    pub fn set_trigger(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.trigger = Some(v.into());
        self
    }
    #[doc = "Set the field `trigger_region`.\n"]
    pub fn set_trigger_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.trigger_region = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudfunctions2FunctionEventTriggerEl {
    type O = BlockAssignable<DataCloudfunctions2FunctionEventTriggerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfunctions2FunctionEventTriggerEl {}
impl BuildDataCloudfunctions2FunctionEventTriggerEl {
    pub fn build(self) -> DataCloudfunctions2FunctionEventTriggerEl {
        DataCloudfunctions2FunctionEventTriggerEl {
            event_filters: core::default::Default::default(),
            event_type: core::default::Default::default(),
            pubsub_topic: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
            trigger: core::default::Default::default(),
            trigger_region: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfunctions2FunctionEventTriggerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfunctions2FunctionEventTriggerElRef {
    fn new(shared: StackShared, base: String) -> DataCloudfunctions2FunctionEventTriggerElRef {
        DataCloudfunctions2FunctionEventTriggerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfunctions2FunctionEventTriggerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `event_filters` after provisioning.\n"]
    pub fn event_filters(
        &self,
    ) -> SetRef<DataCloudfunctions2FunctionEventTriggerElEventFiltersElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.event_filters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `event_type` after provisioning.\n"]
    pub fn event_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_type", self.base))
    }
    #[doc = "Get a reference to the value of field `pubsub_topic` after provisioning.\n"]
    pub fn pubsub_topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pubsub_topic", self.base))
    }
    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\n"]
    pub fn service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_email", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `trigger` after provisioning.\n"]
    pub fn trigger(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trigger", self.base))
    }
    #[doc = "Get a reference to the value of field `trigger_region` after provisioning.\n"]
    pub fn trigger_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trigger_region", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<PrimField<String>>>,
}
impl DataCloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceEl {
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
impl ToListMappable for DataCloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceEl {
    type O = BlockAssignable<DataCloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceEl {}
impl BuildDataCloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceEl {
    pub fn build(self) -> DataCloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceEl {
        DataCloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceEl {
            network: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceElRef {
        DataCloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceElRef {
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
pub struct DataCloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl DataCloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
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
impl ToListMappable for DataCloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesEl {
    type O =
        BlockAssignable<DataCloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesEl {}
impl BuildDataCloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesEl {
    pub fn build(self) -> DataCloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesEl {
        DataCloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesEl {
            key: core::default::Default::default(),
            project_id: core::default::Default::default(),
            secret: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesElRef {
        DataCloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
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
pub struct DataCloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl DataCloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsEl {
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
impl ToListMappable for DataCloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsEl {
    type O = BlockAssignable<DataCloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsEl {}
impl BuildDataCloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsEl {
    pub fn build(self) -> DataCloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsEl {
        DataCloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsEl {
            path: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsElRef {
        DataCloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
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
pub struct DataCloudfunctions2FunctionServiceConfigElSecretVolumesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mount_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    versions:
        Option<ListField<DataCloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsEl>>,
}
impl DataCloudfunctions2FunctionServiceConfigElSecretVolumesEl {
    #[doc = "Set the field `mount_path`.\n"]
    pub fn set_mount_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mount_path = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret = Some(v.into());
        self
    }
    #[doc = "Set the field `versions`.\n"]
    pub fn set_versions(
        mut self,
        v: impl Into<ListField<DataCloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsEl>>,
    ) -> Self {
        self.versions = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudfunctions2FunctionServiceConfigElSecretVolumesEl {
    type O = BlockAssignable<DataCloudfunctions2FunctionServiceConfigElSecretVolumesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfunctions2FunctionServiceConfigElSecretVolumesEl {}
impl BuildDataCloudfunctions2FunctionServiceConfigElSecretVolumesEl {
    pub fn build(self) -> DataCloudfunctions2FunctionServiceConfigElSecretVolumesEl {
        DataCloudfunctions2FunctionServiceConfigElSecretVolumesEl {
            mount_path: core::default::Default::default(),
            project_id: core::default::Default::default(),
            secret: core::default::Default::default(),
            versions: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfunctions2FunctionServiceConfigElSecretVolumesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfunctions2FunctionServiceConfigElSecretVolumesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfunctions2FunctionServiceConfigElSecretVolumesElRef {
        DataCloudfunctions2FunctionServiceConfigElSecretVolumesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfunctions2FunctionServiceConfigElSecretVolumesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mount_path` after provisioning.\n"]
    pub fn mount_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }
    #[doc = "Get a reference to the value of field `versions` after provisioning.\n"]
    pub fn versions(
        &self,
    ) -> ListRef<DataCloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.versions", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudfunctions2FunctionServiceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all_traffic_on_latest_revision: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    available_cpu: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    available_memory: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    binary_authorization_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_vpc_egress: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_vpc_network_interface:
        Option<ListField<DataCloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_variables: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcf_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress_settings: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_instance_request_concurrency: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_environment_variables:
        Option<ListField<DataCloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_volumes: Option<ListField<DataCloudfunctions2FunctionServiceConfigElSecretVolumesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_connector: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_connector_egress_settings: Option<PrimField<String>>,
}
impl DataCloudfunctions2FunctionServiceConfigEl {
    #[doc = "Set the field `all_traffic_on_latest_revision`.\n"]
    pub fn set_all_traffic_on_latest_revision(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.all_traffic_on_latest_revision = Some(v.into());
        self
    }
    #[doc = "Set the field `available_cpu`.\n"]
    pub fn set_available_cpu(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.available_cpu = Some(v.into());
        self
    }
    #[doc = "Set the field `available_memory`.\n"]
    pub fn set_available_memory(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.available_memory = Some(v.into());
        self
    }
    #[doc = "Set the field `binary_authorization_policy`.\n"]
    pub fn set_binary_authorization_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.binary_authorization_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `direct_vpc_egress`.\n"]
    pub fn set_direct_vpc_egress(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.direct_vpc_egress = Some(v.into());
        self
    }
    #[doc = "Set the field `direct_vpc_network_interface`.\n"]
    pub fn set_direct_vpc_network_interface(
        mut self,
        v: impl Into<ListField<DataCloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceEl>>,
    ) -> Self {
        self.direct_vpc_network_interface = Some(v.into());
        self
    }
    #[doc = "Set the field `environment_variables`.\n"]
    pub fn set_environment_variables(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.environment_variables = Some(v.into());
        self
    }
    #[doc = "Set the field `gcf_uri`.\n"]
    pub fn set_gcf_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcf_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `ingress_settings`.\n"]
    pub fn set_ingress_settings(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ingress_settings = Some(v.into());
        self
    }
    #[doc = "Set the field `max_instance_count`.\n"]
    pub fn set_max_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_instance_count = Some(v.into());
        self
    }
    #[doc = "Set the field `max_instance_request_concurrency`.\n"]
    pub fn set_max_instance_request_concurrency(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_instance_request_concurrency = Some(v.into());
        self
    }
    #[doc = "Set the field `min_instance_count`.\n"]
    pub fn set_min_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_instance_count = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_environment_variables`.\n"]
    pub fn set_secret_environment_variables(
        mut self,
        v: impl Into<ListField<DataCloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesEl>>,
    ) -> Self {
        self.secret_environment_variables = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_volumes`.\n"]
    pub fn set_secret_volumes(
        mut self,
        v: impl Into<ListField<DataCloudfunctions2FunctionServiceConfigElSecretVolumesEl>>,
    ) -> Self {
        self.secret_volumes = Some(v.into());
        self
    }
    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account_email`.\n"]
    pub fn set_service_account_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account_email = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout_seconds`.\n"]
    pub fn set_timeout_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_connector`.\n"]
    pub fn set_vpc_connector(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_connector = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_connector_egress_settings`.\n"]
    pub fn set_vpc_connector_egress_settings(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_connector_egress_settings = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudfunctions2FunctionServiceConfigEl {
    type O = BlockAssignable<DataCloudfunctions2FunctionServiceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfunctions2FunctionServiceConfigEl {}
impl BuildDataCloudfunctions2FunctionServiceConfigEl {
    pub fn build(self) -> DataCloudfunctions2FunctionServiceConfigEl {
        DataCloudfunctions2FunctionServiceConfigEl {
            all_traffic_on_latest_revision: core::default::Default::default(),
            available_cpu: core::default::Default::default(),
            available_memory: core::default::Default::default(),
            binary_authorization_policy: core::default::Default::default(),
            direct_vpc_egress: core::default::Default::default(),
            direct_vpc_network_interface: core::default::Default::default(),
            environment_variables: core::default::Default::default(),
            gcf_uri: core::default::Default::default(),
            ingress_settings: core::default::Default::default(),
            max_instance_count: core::default::Default::default(),
            max_instance_request_concurrency: core::default::Default::default(),
            min_instance_count: core::default::Default::default(),
            secret_environment_variables: core::default::Default::default(),
            secret_volumes: core::default::Default::default(),
            service: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
            uri: core::default::Default::default(),
            vpc_connector: core::default::Default::default(),
            vpc_connector_egress_settings: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfunctions2FunctionServiceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfunctions2FunctionServiceConfigElRef {
    fn new(shared: StackShared, base: String) -> DataCloudfunctions2FunctionServiceConfigElRef {
        DataCloudfunctions2FunctionServiceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfunctions2FunctionServiceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `all_traffic_on_latest_revision` after provisioning.\n"]
    pub fn all_traffic_on_latest_revision(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.all_traffic_on_latest_revision", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `available_cpu` after provisioning.\n"]
    pub fn available_cpu(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_cpu", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `available_memory` after provisioning.\n"]
    pub fn available_memory(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_memory", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `binary_authorization_policy` after provisioning.\n"]
    pub fn binary_authorization_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.binary_authorization_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `direct_vpc_egress` after provisioning.\n"]
    pub fn direct_vpc_egress(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.direct_vpc_egress", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `direct_vpc_network_interface` after provisioning.\n"]
    pub fn direct_vpc_network_interface(
        &self,
    ) -> ListRef<DataCloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.direct_vpc_network_interface", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\n"]
    pub fn environment_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.environment_variables", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gcf_uri` after provisioning.\n"]
    pub fn gcf_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gcf_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `ingress_settings` after provisioning.\n"]
    pub fn ingress_settings(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ingress_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_instance_count` after provisioning.\n"]
    pub fn max_instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_instance_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_instance_request_concurrency` after provisioning.\n"]
    pub fn max_instance_request_concurrency(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_instance_request_concurrency", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_instance_count` after provisioning.\n"]
    pub fn min_instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_instance_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secret_environment_variables` after provisioning.\n"]
    pub fn secret_environment_variables(
        &self,
    ) -> ListRef<DataCloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secret_environment_variables", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secret_volumes` after provisioning.\n"]
    pub fn secret_volumes(
        &self,
    ) -> ListRef<DataCloudfunctions2FunctionServiceConfigElSecretVolumesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secret_volumes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\n"]
    pub fn service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_email", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timeout_seconds` after provisioning.\n"]
    pub fn timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
    #[doc = "Get a reference to the value of field `vpc_connector` after provisioning.\n"]
    pub fn vpc_connector(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_connector", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_connector_egress_settings` after provisioning.\n"]
    pub fn vpc_connector_egress_settings(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_connector_egress_settings", self.base),
        )
    }
}
