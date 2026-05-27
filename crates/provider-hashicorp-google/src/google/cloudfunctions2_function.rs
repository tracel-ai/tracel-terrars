use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct Cloudfunctions2FunctionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_config: Option<Vec<Cloudfunctions2FunctionBuildConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_trigger: Option<Vec<Cloudfunctions2FunctionEventTriggerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_config: Option<Vec<Cloudfunctions2FunctionServiceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<Cloudfunctions2FunctionTimeoutsEl>,
    dynamic: Cloudfunctions2FunctionDynamic,
}
struct Cloudfunctions2Function_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Cloudfunctions2FunctionData>,
}
#[derive(Clone)]
pub struct Cloudfunctions2Function(Rc<Cloudfunctions2Function_>);
impl Cloudfunctions2Function {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nUser-provided description of a function."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_name`.\nResource name of a KMS crypto key (managed by the user) used to encrypt/decrypt function resources.\nIt must match the pattern projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}."]
    pub fn set_kms_key_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nA set of key/value label pairs associated with this Cloud Function.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `build_config`.\n"]
    pub fn set_build_config(
        self,
        v: impl Into<BlockAssignable<Cloudfunctions2FunctionBuildConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().build_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.build_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `event_trigger`.\n"]
    pub fn set_event_trigger(
        self,
        v: impl Into<BlockAssignable<Cloudfunctions2FunctionEventTriggerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event_trigger = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event_trigger = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `service_config`.\n"]
    pub fn set_service_config(
        self,
        v: impl Into<BlockAssignable<Cloudfunctions2FunctionServiceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<Cloudfunctions2FunctionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
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
    #[doc = "Get a reference to the value of field `build_config` after provisioning.\n"]
    pub fn build_config(&self) -> ListRef<Cloudfunctions2FunctionBuildConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.build_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_trigger` after provisioning.\n"]
    pub fn event_trigger(&self) -> ListRef<Cloudfunctions2FunctionEventTriggerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.event_trigger", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_config` after provisioning.\n"]
    pub fn service_config(&self) -> ListRef<Cloudfunctions2FunctionServiceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Cloudfunctions2FunctionTimeoutsElRef {
        Cloudfunctions2FunctionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for Cloudfunctions2Function {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for Cloudfunctions2Function {}
impl ToListMappable for Cloudfunctions2Function {
    type O = ListRef<Cloudfunctions2FunctionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for Cloudfunctions2Function_ {
    fn extract_resource_type(&self) -> String {
        "google_cloudfunctions2_function".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudfunctions2Function {
    pub tf_id: String,
    #[doc = "The location of this cloud function."]
    pub location: PrimField<String>,
    #[doc = "A user-defined name of the function. Function names must\nbe unique globally and match pattern 'projects/*/locations/*/functions/*'."]
    pub name: PrimField<String>,
}
impl BuildCloudfunctions2Function {
    pub fn build(self, stack: &mut Stack) -> Cloudfunctions2Function {
        let out = Cloudfunctions2Function(Rc::new(Cloudfunctions2Function_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Cloudfunctions2FunctionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_name: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                build_config: core::default::Default::default(),
                event_trigger: core::default::Default::default(),
                service_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct Cloudfunctions2FunctionRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudfunctions2FunctionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl Cloudfunctions2FunctionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `build_config` after provisioning.\n"]
    pub fn build_config(&self) -> ListRef<Cloudfunctions2FunctionBuildConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.build_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_trigger` after provisioning.\n"]
    pub fn event_trigger(&self) -> ListRef<Cloudfunctions2FunctionEventTriggerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.event_trigger", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_config` after provisioning.\n"]
    pub fn service_config(&self) -> ListRef<Cloudfunctions2FunctionServiceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Cloudfunctions2FunctionTimeoutsElRef {
        Cloudfunctions2FunctionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct Cloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyEl {}
impl Cloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyEl {}
impl ToListMappable for Cloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyEl {
    type O = BlockAssignable<Cloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyEl {}
impl BuildCloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyEl {
    pub fn build(self) -> Cloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyEl {
        Cloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyEl {}
    }
}
pub struct Cloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Cloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyElRef {
        Cloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct Cloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyEl {}
impl Cloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyEl {}
impl ToListMappable for Cloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyEl {
    type O = BlockAssignable<Cloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyEl {}
impl BuildCloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyEl {
    pub fn build(self) -> Cloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyEl {
        Cloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyEl {}
    }
}
pub struct Cloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Cloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyElRef {
        Cloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `runtime_version` after provisioning.\nThe runtime version which was used during latest function deployment."]
    pub fn runtime_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.runtime_version", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Cloudfunctions2FunctionBuildConfigElSourceElRepoSourceEl {
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
impl Cloudfunctions2FunctionBuildConfigElSourceElRepoSourceEl {
    #[doc = "Set the field `branch_name`.\nRegex matching branches to build."]
    pub fn set_branch_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch_name = Some(v.into());
        self
    }
    #[doc = "Set the field `commit_sha`.\nRegex matching tags to build."]
    pub fn set_commit_sha(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.commit_sha = Some(v.into());
        self
    }
    #[doc = "Set the field `dir`.\nDirectory, relative to the source root, in which to run the build."]
    pub fn set_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dir = Some(v.into());
        self
    }
    #[doc = "Set the field `invert_regex`.\nOnly trigger a build if the revision regex does\nNOT match the revision regex."]
    pub fn set_invert_regex(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert_regex = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id`.\nID of the project that owns the Cloud Source Repository. If omitted, the\nproject ID requesting the build is assumed."]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
    #[doc = "Set the field `repo_name`.\nName of the Cloud Source Repository."]
    pub fn set_repo_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo_name = Some(v.into());
        self
    }
    #[doc = "Set the field `tag_name`.\nRegex matching tags to build."]
    pub fn set_tag_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag_name = Some(v.into());
        self
    }
}
impl ToListMappable for Cloudfunctions2FunctionBuildConfigElSourceElRepoSourceEl {
    type O = BlockAssignable<Cloudfunctions2FunctionBuildConfigElSourceElRepoSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfunctions2FunctionBuildConfigElSourceElRepoSourceEl {}
impl BuildCloudfunctions2FunctionBuildConfigElSourceElRepoSourceEl {
    pub fn build(self) -> Cloudfunctions2FunctionBuildConfigElSourceElRepoSourceEl {
        Cloudfunctions2FunctionBuildConfigElSourceElRepoSourceEl {
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
pub struct Cloudfunctions2FunctionBuildConfigElSourceElRepoSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudfunctions2FunctionBuildConfigElSourceElRepoSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Cloudfunctions2FunctionBuildConfigElSourceElRepoSourceElRef {
        Cloudfunctions2FunctionBuildConfigElSourceElRepoSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudfunctions2FunctionBuildConfigElSourceElRepoSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `branch_name` after provisioning.\nRegex matching branches to build."]
    pub fn branch_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch_name", self.base))
    }
    #[doc = "Get a reference to the value of field `commit_sha` after provisioning.\nRegex matching tags to build."]
    pub fn commit_sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_sha", self.base))
    }
    #[doc = "Get a reference to the value of field `dir` after provisioning.\nDirectory, relative to the source root, in which to run the build."]
    pub fn dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dir", self.base))
    }
    #[doc = "Get a reference to the value of field `invert_regex` after provisioning.\nOnly trigger a build if the revision regex does\nNOT match the revision regex."]
    pub fn invert_regex(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nID of the project that owns the Cloud Source Repository. If omitted, the\nproject ID requesting the build is assumed."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `repo_name` after provisioning.\nName of the Cloud Source Repository."]
    pub fn repo_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_name", self.base))
    }
    #[doc = "Get a reference to the value of field `tag_name` after provisioning.\nRegex matching tags to build."]
    pub fn tag_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_name", self.base))
    }
}
#[derive(Serialize)]
pub struct Cloudfunctions2FunctionBuildConfigElSourceElStorageSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object: Option<PrimField<String>>,
}
impl Cloudfunctions2FunctionBuildConfigElSourceElStorageSourceEl {
    #[doc = "Set the field `bucket`.\nGoogle Cloud Storage bucket containing the source"]
    pub fn set_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket = Some(v.into());
        self
    }
    #[doc = "Set the field `generation`.\nGoogle Cloud Storage generation for the object. If the generation\nis omitted, the latest generation will be used."]
    pub fn set_generation(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.generation = Some(v.into());
        self
    }
    #[doc = "Set the field `object`.\nGoogle Cloud Storage object containing the source."]
    pub fn set_object(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.object = Some(v.into());
        self
    }
}
impl ToListMappable for Cloudfunctions2FunctionBuildConfigElSourceElStorageSourceEl {
    type O = BlockAssignable<Cloudfunctions2FunctionBuildConfigElSourceElStorageSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfunctions2FunctionBuildConfigElSourceElStorageSourceEl {}
impl BuildCloudfunctions2FunctionBuildConfigElSourceElStorageSourceEl {
    pub fn build(self) -> Cloudfunctions2FunctionBuildConfigElSourceElStorageSourceEl {
        Cloudfunctions2FunctionBuildConfigElSourceElStorageSourceEl {
            bucket: core::default::Default::default(),
            generation: core::default::Default::default(),
            object: core::default::Default::default(),
        }
    }
}
pub struct Cloudfunctions2FunctionBuildConfigElSourceElStorageSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudfunctions2FunctionBuildConfigElSourceElStorageSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Cloudfunctions2FunctionBuildConfigElSourceElStorageSourceElRef {
        Cloudfunctions2FunctionBuildConfigElSourceElStorageSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudfunctions2FunctionBuildConfigElSourceElStorageSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nGoogle Cloud Storage bucket containing the source"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `generation` after provisioning.\nGoogle Cloud Storage generation for the object. If the generation\nis omitted, the latest generation will be used."]
    pub fn generation(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.generation", self.base))
    }
    #[doc = "Get a reference to the value of field `object` after provisioning.\nGoogle Cloud Storage object containing the source."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}
#[derive(Serialize, Default)]
struct Cloudfunctions2FunctionBuildConfigElSourceElDynamic {
    repo_source: Option<DynamicBlock<Cloudfunctions2FunctionBuildConfigElSourceElRepoSourceEl>>,
    storage_source:
        Option<DynamicBlock<Cloudfunctions2FunctionBuildConfigElSourceElStorageSourceEl>>,
}
#[derive(Serialize)]
pub struct Cloudfunctions2FunctionBuildConfigElSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_source: Option<Vec<Cloudfunctions2FunctionBuildConfigElSourceElRepoSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_source: Option<Vec<Cloudfunctions2FunctionBuildConfigElSourceElStorageSourceEl>>,
    dynamic: Cloudfunctions2FunctionBuildConfigElSourceElDynamic,
}
impl Cloudfunctions2FunctionBuildConfigElSourceEl {
    #[doc = "Set the field `repo_source`.\n"]
    pub fn set_repo_source(
        mut self,
        v: impl Into<BlockAssignable<Cloudfunctions2FunctionBuildConfigElSourceElRepoSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.repo_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.repo_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `storage_source`.\n"]
    pub fn set_storage_source(
        mut self,
        v: impl Into<BlockAssignable<Cloudfunctions2FunctionBuildConfigElSourceElStorageSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.storage_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_source = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Cloudfunctions2FunctionBuildConfigElSourceEl {
    type O = BlockAssignable<Cloudfunctions2FunctionBuildConfigElSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfunctions2FunctionBuildConfigElSourceEl {}
impl BuildCloudfunctions2FunctionBuildConfigElSourceEl {
    pub fn build(self) -> Cloudfunctions2FunctionBuildConfigElSourceEl {
        Cloudfunctions2FunctionBuildConfigElSourceEl {
            repo_source: core::default::Default::default(),
            storage_source: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Cloudfunctions2FunctionBuildConfigElSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudfunctions2FunctionBuildConfigElSourceElRef {
    fn new(shared: StackShared, base: String) -> Cloudfunctions2FunctionBuildConfigElSourceElRef {
        Cloudfunctions2FunctionBuildConfigElSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudfunctions2FunctionBuildConfigElSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `repo_source` after provisioning.\n"]
    pub fn repo_source(
        &self,
    ) -> ListRef<Cloudfunctions2FunctionBuildConfigElSourceElRepoSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.repo_source", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_source` after provisioning.\n"]
    pub fn storage_source(
        &self,
    ) -> ListRef<Cloudfunctions2FunctionBuildConfigElSourceElStorageSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_source", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct Cloudfunctions2FunctionBuildConfigElDynamic {
    automatic_update_policy:
        Option<DynamicBlock<Cloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyEl>>,
    on_deploy_update_policy:
        Option<DynamicBlock<Cloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyEl>>,
    source: Option<DynamicBlock<Cloudfunctions2FunctionBuildConfigElSourceEl>>,
}
#[derive(Serialize)]
pub struct Cloudfunctions2FunctionBuildConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    docker_repository: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entry_point: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_variables: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_pool: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_update_policy:
        Option<Vec<Cloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_deploy_update_policy:
        Option<Vec<Cloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Vec<Cloudfunctions2FunctionBuildConfigElSourceEl>>,
    dynamic: Cloudfunctions2FunctionBuildConfigElDynamic,
}
impl Cloudfunctions2FunctionBuildConfigEl {
    #[doc = "Set the field `docker_repository`.\nUser managed repository created in Artifact Registry optionally with a customer managed encryption key."]
    pub fn set_docker_repository(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.docker_repository = Some(v.into());
        self
    }
    #[doc = "Set the field `entry_point`.\nThe name of the function (as defined in source code) that will be executed.\nDefaults to the resource name suffix, if not specified. For backward\ncompatibility, if function with given name is not found, then the system\nwill try to use function named \"function\". For Node.js this is name of a\nfunction exported by the module specified in source_location."]
    pub fn set_entry_point(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.entry_point = Some(v.into());
        self
    }
    #[doc = "Set the field `environment_variables`.\nUser-provided build-time environment variables for the function."]
    pub fn set_environment_variables(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.environment_variables = Some(v.into());
        self
    }
    #[doc = "Set the field `runtime`.\nThe runtime in which to run the function. Required when deploying a new\nfunction, optional when updating an existing function."]
    pub fn set_runtime(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.runtime = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\nThe fully-qualified name of the service account to be used for building the container."]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `worker_pool`.\nName of the Cloud Build Custom Worker Pool that should be used to build the function."]
    pub fn set_worker_pool(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.worker_pool = Some(v.into());
        self
    }
    #[doc = "Set the field `automatic_update_policy`.\n"]
    pub fn set_automatic_update_policy(
        mut self,
        v: impl Into<BlockAssignable<Cloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.automatic_update_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.automatic_update_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `on_deploy_update_policy`.\n"]
    pub fn set_on_deploy_update_policy(
        mut self,
        v: impl Into<BlockAssignable<Cloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.on_deploy_update_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.on_deploy_update_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v: impl Into<BlockAssignable<Cloudfunctions2FunctionBuildConfigElSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Cloudfunctions2FunctionBuildConfigEl {
    type O = BlockAssignable<Cloudfunctions2FunctionBuildConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfunctions2FunctionBuildConfigEl {}
impl BuildCloudfunctions2FunctionBuildConfigEl {
    pub fn build(self) -> Cloudfunctions2FunctionBuildConfigEl {
        Cloudfunctions2FunctionBuildConfigEl {
            docker_repository: core::default::Default::default(),
            entry_point: core::default::Default::default(),
            environment_variables: core::default::Default::default(),
            runtime: core::default::Default::default(),
            service_account: core::default::Default::default(),
            worker_pool: core::default::Default::default(),
            automatic_update_policy: core::default::Default::default(),
            on_deploy_update_policy: core::default::Default::default(),
            source: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Cloudfunctions2FunctionBuildConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudfunctions2FunctionBuildConfigElRef {
    fn new(shared: StackShared, base: String) -> Cloudfunctions2FunctionBuildConfigElRef {
        Cloudfunctions2FunctionBuildConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudfunctions2FunctionBuildConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `build` after provisioning.\nThe Cloud Build name of the latest successful\ndeployment of the function."]
    pub fn build(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build", self.base))
    }
    #[doc = "Get a reference to the value of field `docker_repository` after provisioning.\nUser managed repository created in Artifact Registry optionally with a customer managed encryption key."]
    pub fn docker_repository(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.docker_repository", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `entry_point` after provisioning.\nThe name of the function (as defined in source code) that will be executed.\nDefaults to the resource name suffix, if not specified. For backward\ncompatibility, if function with given name is not found, then the system\nwill try to use function named \"function\". For Node.js this is name of a\nfunction exported by the module specified in source_location."]
    pub fn entry_point(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.entry_point", self.base))
    }
    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\nUser-provided build-time environment variables for the function."]
    pub fn environment_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.environment_variables", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `runtime` after provisioning.\nThe runtime in which to run the function. Required when deploying a new\nfunction, optional when updating an existing function."]
    pub fn runtime(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.runtime", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nThe fully-qualified name of the service account to be used for building the container."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `worker_pool` after provisioning.\nName of the Cloud Build Custom Worker Pool that should be used to build the function."]
    pub fn worker_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.worker_pool", self.base))
    }
    #[doc = "Get a reference to the value of field `automatic_update_policy` after provisioning.\n"]
    pub fn automatic_update_policy(
        &self,
    ) -> ListRef<Cloudfunctions2FunctionBuildConfigElAutomaticUpdatePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.automatic_update_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `on_deploy_update_policy` after provisioning.\n"]
    pub fn on_deploy_update_policy(
        &self,
    ) -> ListRef<Cloudfunctions2FunctionBuildConfigElOnDeployUpdatePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.on_deploy_update_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<Cloudfunctions2FunctionBuildConfigElSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
}
#[derive(Serialize)]
pub struct Cloudfunctions2FunctionEventTriggerElEventFiltersEl {
    attribute: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<PrimField<String>>,
    value: PrimField<String>,
}
impl Cloudfunctions2FunctionEventTriggerElEventFiltersEl {
    #[doc = "Set the field `operator`.\nOptional. The operator used for matching the events with the value of\nthe filter. If not specified, only events that have an exact key-value\npair specified in the filter are matched.\nThe only allowed value is 'match-path-pattern'.\n[See documentation on path patterns here](https://cloud.google.com/eventarc/docs/path-patterns)'"]
    pub fn set_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operator = Some(v.into());
        self
    }
}
impl ToListMappable for Cloudfunctions2FunctionEventTriggerElEventFiltersEl {
    type O = BlockAssignable<Cloudfunctions2FunctionEventTriggerElEventFiltersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfunctions2FunctionEventTriggerElEventFiltersEl {
    #[doc = "'Required. The name of a CloudEvents attribute.\nCurrently, only a subset of attributes are supported for filtering. Use the 'gcloud eventarc providers describe' command to learn more about events and their attributes.\nDo not filter for the 'type' attribute here, as this is already achieved by the resource's 'event_type' attribute."]
    pub attribute: PrimField<String>,
    #[doc = "Required. The value for the attribute.\nIf the operator field is set as 'match-path-pattern', this value can be a path pattern instead of an exact value."]
    pub value: PrimField<String>,
}
impl BuildCloudfunctions2FunctionEventTriggerElEventFiltersEl {
    pub fn build(self) -> Cloudfunctions2FunctionEventTriggerElEventFiltersEl {
        Cloudfunctions2FunctionEventTriggerElEventFiltersEl {
            attribute: self.attribute,
            operator: core::default::Default::default(),
            value: self.value,
        }
    }
}
pub struct Cloudfunctions2FunctionEventTriggerElEventFiltersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudfunctions2FunctionEventTriggerElEventFiltersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Cloudfunctions2FunctionEventTriggerElEventFiltersElRef {
        Cloudfunctions2FunctionEventTriggerElEventFiltersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudfunctions2FunctionEventTriggerElEventFiltersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n'Required. The name of a CloudEvents attribute.\nCurrently, only a subset of attributes are supported for filtering. Use the 'gcloud eventarc providers describe' command to learn more about events and their attributes.\nDo not filter for the 'type' attribute here, as this is already achieved by the resource's 'event_type' attribute."]
    pub fn attribute(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\nOptional. The operator used for matching the events with the value of\nthe filter. If not specified, only events that have an exact key-value\npair specified in the filter are matched.\nThe only allowed value is 'match-path-pattern'.\n[See documentation on path patterns here](https://cloud.google.com/eventarc/docs/path-patterns)'"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nRequired. The value for the attribute.\nIf the operator field is set as 'match-path-pattern', this value can be a path pattern instead of an exact value."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct Cloudfunctions2FunctionEventTriggerElDynamic {
    event_filters: Option<DynamicBlock<Cloudfunctions2FunctionEventTriggerElEventFiltersEl>>,
}
#[derive(Serialize)]
pub struct Cloudfunctions2FunctionEventTriggerEl {
    event_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pubsub_topic: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger_region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_filters: Option<Vec<Cloudfunctions2FunctionEventTriggerElEventFiltersEl>>,
    dynamic: Cloudfunctions2FunctionEventTriggerElDynamic,
}
impl Cloudfunctions2FunctionEventTriggerEl {
    #[doc = "Set the field `pubsub_topic`.\nThe name of a Pub/Sub topic in the same project that will be used\nas the transport topic for the event delivery."]
    pub fn set_pubsub_topic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pubsub_topic = Some(v.into());
        self
    }
    #[doc = "Set the field `retry_policy`.\nDescribes the retry policy in case of function's execution failure.\nRetried execution is charged as any other execution. Possible values: [\"RETRY_POLICY_UNSPECIFIED\", \"RETRY_POLICY_DO_NOT_RETRY\", \"RETRY_POLICY_RETRY\"]"]
    pub fn set_retry_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.retry_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account_email`.\nOptional. The email of the trigger's service account. The service account\nmust have permission to invoke Cloud Run services. If empty, defaults to the\nCompute Engine default service account: {project_number}-compute@developer.gserviceaccount.com."]
    pub fn set_service_account_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account_email = Some(v.into());
        self
    }
    #[doc = "Set the field `trigger_region`.\nThe region that the trigger will be in. The trigger will only receive\nevents originating in this region. It can be the same\nregion as the function, a different region or multi-region, or the global\nregion. If not provided, defaults to the same region as the function."]
    pub fn set_trigger_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.trigger_region = Some(v.into());
        self
    }
    #[doc = "Set the field `event_filters`.\n"]
    pub fn set_event_filters(
        mut self,
        v: impl Into<BlockAssignable<Cloudfunctions2FunctionEventTriggerElEventFiltersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.event_filters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.event_filters = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Cloudfunctions2FunctionEventTriggerEl {
    type O = BlockAssignable<Cloudfunctions2FunctionEventTriggerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfunctions2FunctionEventTriggerEl {
    #[doc = "Required. The type of event to observe."]
    pub event_type: PrimField<String>,
}
impl BuildCloudfunctions2FunctionEventTriggerEl {
    pub fn build(self) -> Cloudfunctions2FunctionEventTriggerEl {
        Cloudfunctions2FunctionEventTriggerEl {
            event_type: self.event_type,
            pubsub_topic: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
            trigger_region: core::default::Default::default(),
            event_filters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Cloudfunctions2FunctionEventTriggerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudfunctions2FunctionEventTriggerElRef {
    fn new(shared: StackShared, base: String) -> Cloudfunctions2FunctionEventTriggerElRef {
        Cloudfunctions2FunctionEventTriggerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudfunctions2FunctionEventTriggerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `event_type` after provisioning.\nRequired. The type of event to observe."]
    pub fn event_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_type", self.base))
    }
    #[doc = "Get a reference to the value of field `pubsub_topic` after provisioning.\nThe name of a Pub/Sub topic in the same project that will be used\nas the transport topic for the event delivery."]
    pub fn pubsub_topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pubsub_topic", self.base))
    }
    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\nDescribes the retry policy in case of function's execution failure.\nRetried execution is charged as any other execution. Possible values: [\"RETRY_POLICY_UNSPECIFIED\", \"RETRY_POLICY_DO_NOT_RETRY\", \"RETRY_POLICY_RETRY\"]"]
    pub fn retry_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\nOptional. The email of the trigger's service account. The service account\nmust have permission to invoke Cloud Run services. If empty, defaults to the\nCompute Engine default service account: {project_number}-compute@developer.gserviceaccount.com."]
    pub fn service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_email", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `trigger` after provisioning.\nOutput only. The resource name of the Eventarc trigger."]
    pub fn trigger(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trigger", self.base))
    }
    #[doc = "Get a reference to the value of field `trigger_region` after provisioning.\nThe region that the trigger will be in. The trigger will only receive\nevents originating in this region. It can be the same\nregion as the function, a different region or multi-region, or the global\nregion. If not provided, defaults to the same region as the function."]
    pub fn trigger_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trigger_region", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Cloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<PrimField<String>>>,
}
impl Cloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceEl {
    #[doc = "Set the field `network`.\nThe name of the VPC network to which the function will be connected. Specify either a VPC network or a subnet, or both. If you specify only a network, the subnet uses the same name as the network."]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\nThe name of the VPC subnetwork that the Cloud Function resource will get IPs from. Specify either a VPC network or a subnet, or both. If both network and subnetwork are specified, the given VPC subnetwork must belong to the given VPC network. If subnetwork is not specified, the subnetwork with the same name with the network will be used."]
    pub fn set_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\nNetwork tags applied to this Cloud Function resource."]
    pub fn set_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
}
impl ToListMappable for Cloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceEl {
    type O = BlockAssignable<Cloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceEl {}
impl BuildCloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceEl {
    pub fn build(self) -> Cloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceEl {
        Cloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceEl {
            network: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}
pub struct Cloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Cloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceElRef {
        Cloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe name of the VPC network to which the function will be connected. Specify either a VPC network or a subnet, or both. If you specify only a network, the subnet uses the same name as the network."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nThe name of the VPC subnetwork that the Cloud Function resource will get IPs from. Specify either a VPC network or a subnet, or both. If both network and subnetwork are specified, the given VPC subnetwork must belong to the given VPC network. If subnetwork is not specified, the subnetwork with the same name with the network will be used."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nNetwork tags applied to this Cloud Function resource."]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}
#[derive(Serialize)]
pub struct Cloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesEl {
    key: PrimField<String>,
    project_id: PrimField<String>,
    secret: PrimField<String>,
    version: PrimField<String>,
}
impl Cloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesEl {}
impl ToListMappable for Cloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesEl {
    type O = BlockAssignable<Cloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesEl {
    #[doc = "Name of the environment variable."]
    pub key: PrimField<String>,
    #[doc = "Project identifier (preferably project number but can also be the project ID) of the project that contains the secret. If not set, it will be populated with the function's project assuming that the secret exists in the same project as of the function."]
    pub project_id: PrimField<String>,
    #[doc = "Name of the secret in secret manager (not the full resource name)."]
    pub secret: PrimField<String>,
    #[doc = "Version of the secret (version number or the string 'latest'). It is recommended to use a numeric version for secret environment variables as any updates to the secret value is not reflected until new instances start."]
    pub version: PrimField<String>,
}
impl BuildCloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesEl {
    pub fn build(self) -> Cloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesEl {
        Cloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesEl {
            key: self.key,
            project_id: self.project_id,
            secret: self.secret,
            version: self.version,
        }
    }
}
pub struct Cloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Cloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesElRef {
        Cloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nName of the environment variable."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nProject identifier (preferably project number but can also be the project ID) of the project that contains the secret. If not set, it will be populated with the function's project assuming that the secret exists in the same project as of the function."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\nName of the secret in secret manager (not the full resource name)."]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion of the secret (version number or the string 'latest'). It is recommended to use a numeric version for secret environment variables as any updates to the secret value is not reflected until new instances start."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize)]
pub struct Cloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsEl {
    path: PrimField<String>,
    version: PrimField<String>,
}
impl Cloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsEl {}
impl ToListMappable for Cloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsEl {
    type O = BlockAssignable<Cloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsEl {
    #[doc = "Relative path of the file under the mount path where the secret value for this version will be fetched and made available. For example, setting the mountPath as '/etc/secrets' and path as secret_foo would mount the secret value file at /etc/secrets/secret_foo."]
    pub path: PrimField<String>,
    #[doc = "Version of the secret (version number or the string 'latest'). It is preferable to use latest version with secret volumes as secret value changes are reflected immediately."]
    pub version: PrimField<String>,
}
impl BuildCloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsEl {
    pub fn build(self) -> Cloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsEl {
        Cloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsEl {
            path: self.path,
            version: self.version,
        }
    }
}
pub struct Cloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Cloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsElRef {
        Cloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nRelative path of the file under the mount path where the secret value for this version will be fetched and made available. For example, setting the mountPath as '/etc/secrets' and path as secret_foo would mount the secret value file at /etc/secrets/secret_foo."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion of the secret (version number or the string 'latest'). It is preferable to use latest version with secret volumes as secret value changes are reflected immediately."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize, Default)]
struct Cloudfunctions2FunctionServiceConfigElSecretVolumesElDynamic {
    versions: Option<DynamicBlock<Cloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsEl>>,
}
#[derive(Serialize)]
pub struct Cloudfunctions2FunctionServiceConfigElSecretVolumesEl {
    mount_path: PrimField<String>,
    project_id: PrimField<String>,
    secret: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    versions: Option<Vec<Cloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsEl>>,
    dynamic: Cloudfunctions2FunctionServiceConfigElSecretVolumesElDynamic,
}
impl Cloudfunctions2FunctionServiceConfigElSecretVolumesEl {
    #[doc = "Set the field `versions`.\n"]
    pub fn set_versions(
        mut self,
        v: impl Into<BlockAssignable<Cloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.versions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.versions = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Cloudfunctions2FunctionServiceConfigElSecretVolumesEl {
    type O = BlockAssignable<Cloudfunctions2FunctionServiceConfigElSecretVolumesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfunctions2FunctionServiceConfigElSecretVolumesEl {
    #[doc = "The path within the container to mount the secret volume. For example, setting the mountPath as /etc/secrets would mount the secret value files under the /etc/secrets directory. This directory will also be completely shadowed and unavailable to mount any other secrets. Recommended mount path: /etc/secrets"]
    pub mount_path: PrimField<String>,
    #[doc = "Project identifier (preferably project number but can also be the project ID) of the project that contains the secret. If not set, it will be populated with the function's project assuming that the secret exists in the same project as of the function."]
    pub project_id: PrimField<String>,
    #[doc = "Name of the secret in secret manager (not the full resource name)."]
    pub secret: PrimField<String>,
}
impl BuildCloudfunctions2FunctionServiceConfigElSecretVolumesEl {
    pub fn build(self) -> Cloudfunctions2FunctionServiceConfigElSecretVolumesEl {
        Cloudfunctions2FunctionServiceConfigElSecretVolumesEl {
            mount_path: self.mount_path,
            project_id: self.project_id,
            secret: self.secret,
            versions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Cloudfunctions2FunctionServiceConfigElSecretVolumesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudfunctions2FunctionServiceConfigElSecretVolumesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Cloudfunctions2FunctionServiceConfigElSecretVolumesElRef {
        Cloudfunctions2FunctionServiceConfigElSecretVolumesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudfunctions2FunctionServiceConfigElSecretVolumesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mount_path` after provisioning.\nThe path within the container to mount the secret volume. For example, setting the mountPath as /etc/secrets would mount the secret value files under the /etc/secrets directory. This directory will also be completely shadowed and unavailable to mount any other secrets. Recommended mount path: /etc/secrets"]
    pub fn mount_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nProject identifier (preferably project number but can also be the project ID) of the project that contains the secret. If not set, it will be populated with the function's project assuming that the secret exists in the same project as of the function."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\nName of the secret in secret manager (not the full resource name)."]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }
    #[doc = "Get a reference to the value of field `versions` after provisioning.\n"]
    pub fn versions(
        &self,
    ) -> ListRef<Cloudfunctions2FunctionServiceConfigElSecretVolumesElVersionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.versions", self.base))
    }
}
#[derive(Serialize, Default)]
struct Cloudfunctions2FunctionServiceConfigElDynamic {
    direct_vpc_network_interface:
        Option<DynamicBlock<Cloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceEl>>,
    secret_environment_variables:
        Option<DynamicBlock<Cloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesEl>>,
    secret_volumes: Option<DynamicBlock<Cloudfunctions2FunctionServiceConfigElSecretVolumesEl>>,
}
#[derive(Serialize)]
pub struct Cloudfunctions2FunctionServiceConfigEl {
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
    environment_variables: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress_settings: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_instance_request_concurrency: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_connector: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_connector_egress_settings: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_vpc_network_interface:
        Option<Vec<Cloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_environment_variables:
        Option<Vec<Cloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_volumes: Option<Vec<Cloudfunctions2FunctionServiceConfigElSecretVolumesEl>>,
    dynamic: Cloudfunctions2FunctionServiceConfigElDynamic,
}
impl Cloudfunctions2FunctionServiceConfigEl {
    #[doc = "Set the field `all_traffic_on_latest_revision`.\nWhether 100% of traffic is routed to the latest revision. Defaults to true. When false, GCF honors the existing traffic configuration of the underlying Cloud Run service. If that configuration is set to route to LATEST (the default), the new deployment will become LATEST and intercept the traffic. To prevent traffic from shifting, you must manually pin the existing service to a specific revision name in Cloud Run before deploying."]
    pub fn set_all_traffic_on_latest_revision(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.all_traffic_on_latest_revision = Some(v.into());
        self
    }
    #[doc = "Set the field `available_cpu`.\nThe number of CPUs used in a single container instance. Default value is calculated from available memory."]
    pub fn set_available_cpu(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.available_cpu = Some(v.into());
        self
    }
    #[doc = "Set the field `available_memory`.\nThe amount of memory available for a function.\nDefaults to 256M. Supported units are k, M, G, Mi, Gi. If no unit is\nsupplied the value is interpreted as bytes."]
    pub fn set_available_memory(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.available_memory = Some(v.into());
        self
    }
    #[doc = "Set the field `binary_authorization_policy`.\nThe binary authorization policy to be checked when deploying the Cloud Run service."]
    pub fn set_binary_authorization_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.binary_authorization_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `direct_vpc_egress`.\nEgress settings for direct VPC. If not provided, it defaults to VPC_EGRESS_PRIVATE_RANGES_ONLY. Possible values: [\"VPC_EGRESS_ALL_TRAFFIC\", \"VPC_EGRESS_PRIVATE_RANGES_ONLY\"]"]
    pub fn set_direct_vpc_egress(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.direct_vpc_egress = Some(v.into());
        self
    }
    #[doc = "Set the field `environment_variables`.\nEnvironment variables that shall be available during function execution."]
    pub fn set_environment_variables(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.environment_variables = Some(v.into());
        self
    }
    #[doc = "Set the field `ingress_settings`.\nAvailable ingress settings. Defaults to \"ALLOW_ALL\" if unspecified. Default value: \"ALLOW_ALL\" Possible values: [\"ALLOW_ALL\", \"ALLOW_INTERNAL_ONLY\", \"ALLOW_INTERNAL_AND_GCLB\"]"]
    pub fn set_ingress_settings(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ingress_settings = Some(v.into());
        self
    }
    #[doc = "Set the field `max_instance_count`.\nThe limit on the maximum number of function instances that may coexist at a\ngiven time."]
    pub fn set_max_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_instance_count = Some(v.into());
        self
    }
    #[doc = "Set the field `max_instance_request_concurrency`.\nSets the maximum number of concurrent requests that each instance can receive. Defaults to 1."]
    pub fn set_max_instance_request_concurrency(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_instance_request_concurrency = Some(v.into());
        self
    }
    #[doc = "Set the field `min_instance_count`.\nThe limit on the minimum number of function instances that may coexist at a\ngiven time."]
    pub fn set_min_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_instance_count = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account_email`.\nThe email of the service account for this function."]
    pub fn set_service_account_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account_email = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout_seconds`.\nThe function execution timeout. Execution is considered failed and\ncan be terminated if the function is not completed at the end of the\ntimeout period. Defaults to 60 seconds."]
    pub fn set_timeout_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_connector`.\nThe Serverless VPC Access connector that this cloud function can connect to."]
    pub fn set_vpc_connector(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_connector = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_connector_egress_settings`.\nAvailable egress settings. Possible values: [\"VPC_CONNECTOR_EGRESS_SETTINGS_UNSPECIFIED\", \"PRIVATE_RANGES_ONLY\", \"ALL_TRAFFIC\"]"]
    pub fn set_vpc_connector_egress_settings(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_connector_egress_settings = Some(v.into());
        self
    }
    #[doc = "Set the field `direct_vpc_network_interface`.\n"]
    pub fn set_direct_vpc_network_interface(
        mut self,
        v: impl Into<BlockAssignable<Cloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.direct_vpc_network_interface = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.direct_vpc_network_interface = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `secret_environment_variables`.\n"]
    pub fn set_secret_environment_variables(
        mut self,
        v: impl Into<
            BlockAssignable<Cloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.secret_environment_variables = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_environment_variables = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `secret_volumes`.\n"]
    pub fn set_secret_volumes(
        mut self,
        v: impl Into<BlockAssignable<Cloudfunctions2FunctionServiceConfigElSecretVolumesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.secret_volumes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_volumes = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Cloudfunctions2FunctionServiceConfigEl {
    type O = BlockAssignable<Cloudfunctions2FunctionServiceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfunctions2FunctionServiceConfigEl {}
impl BuildCloudfunctions2FunctionServiceConfigEl {
    pub fn build(self) -> Cloudfunctions2FunctionServiceConfigEl {
        Cloudfunctions2FunctionServiceConfigEl {
            all_traffic_on_latest_revision: core::default::Default::default(),
            available_cpu: core::default::Default::default(),
            available_memory: core::default::Default::default(),
            binary_authorization_policy: core::default::Default::default(),
            direct_vpc_egress: core::default::Default::default(),
            environment_variables: core::default::Default::default(),
            ingress_settings: core::default::Default::default(),
            max_instance_count: core::default::Default::default(),
            max_instance_request_concurrency: core::default::Default::default(),
            min_instance_count: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
            vpc_connector: core::default::Default::default(),
            vpc_connector_egress_settings: core::default::Default::default(),
            direct_vpc_network_interface: core::default::Default::default(),
            secret_environment_variables: core::default::Default::default(),
            secret_volumes: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Cloudfunctions2FunctionServiceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudfunctions2FunctionServiceConfigElRef {
    fn new(shared: StackShared, base: String) -> Cloudfunctions2FunctionServiceConfigElRef {
        Cloudfunctions2FunctionServiceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudfunctions2FunctionServiceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `all_traffic_on_latest_revision` after provisioning.\nWhether 100% of traffic is routed to the latest revision. Defaults to true. When false, GCF honors the existing traffic configuration of the underlying Cloud Run service. If that configuration is set to route to LATEST (the default), the new deployment will become LATEST and intercept the traffic. To prevent traffic from shifting, you must manually pin the existing service to a specific revision name in Cloud Run before deploying."]
    pub fn all_traffic_on_latest_revision(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.all_traffic_on_latest_revision", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `available_cpu` after provisioning.\nThe number of CPUs used in a single container instance. Default value is calculated from available memory."]
    pub fn available_cpu(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_cpu", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `available_memory` after provisioning.\nThe amount of memory available for a function.\nDefaults to 256M. Supported units are k, M, G, Mi, Gi. If no unit is\nsupplied the value is interpreted as bytes."]
    pub fn available_memory(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_memory", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `binary_authorization_policy` after provisioning.\nThe binary authorization policy to be checked when deploying the Cloud Run service."]
    pub fn binary_authorization_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.binary_authorization_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `direct_vpc_egress` after provisioning.\nEgress settings for direct VPC. If not provided, it defaults to VPC_EGRESS_PRIVATE_RANGES_ONLY. Possible values: [\"VPC_EGRESS_ALL_TRAFFIC\", \"VPC_EGRESS_PRIVATE_RANGES_ONLY\"]"]
    pub fn direct_vpc_egress(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.direct_vpc_egress", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\nEnvironment variables that shall be available during function execution."]
    pub fn environment_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.environment_variables", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gcf_uri` after provisioning.\nURIs of the Service deployed"]
    pub fn gcf_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gcf_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `ingress_settings` after provisioning.\nAvailable ingress settings. Defaults to \"ALLOW_ALL\" if unspecified. Default value: \"ALLOW_ALL\" Possible values: [\"ALLOW_ALL\", \"ALLOW_INTERNAL_ONLY\", \"ALLOW_INTERNAL_AND_GCLB\"]"]
    pub fn ingress_settings(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ingress_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_instance_count` after provisioning.\nThe limit on the maximum number of function instances that may coexist at a\ngiven time."]
    pub fn max_instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_instance_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_instance_request_concurrency` after provisioning.\nSets the maximum number of concurrent requests that each instance can receive. Defaults to 1."]
    pub fn max_instance_request_concurrency(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_instance_request_concurrency", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_instance_count` after provisioning.\nThe limit on the minimum number of function instances that may coexist at a\ngiven time."]
    pub fn min_instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_instance_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nName of the service associated with a Function."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\nThe email of the service account for this function."]
    pub fn service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_email", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timeout_seconds` after provisioning.\nThe function execution timeout. Execution is considered failed and\ncan be terminated if the function is not completed at the end of the\ntimeout period. Defaults to 60 seconds."]
    pub fn timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nURI of the Service deployed."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
    #[doc = "Get a reference to the value of field `vpc_connector` after provisioning.\nThe Serverless VPC Access connector that this cloud function can connect to."]
    pub fn vpc_connector(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_connector", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_connector_egress_settings` after provisioning.\nAvailable egress settings. Possible values: [\"VPC_CONNECTOR_EGRESS_SETTINGS_UNSPECIFIED\", \"PRIVATE_RANGES_ONLY\", \"ALL_TRAFFIC\"]"]
    pub fn vpc_connector_egress_settings(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_connector_egress_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `direct_vpc_network_interface` after provisioning.\n"]
    pub fn direct_vpc_network_interface(
        &self,
    ) -> ListRef<Cloudfunctions2FunctionServiceConfigElDirectVpcNetworkInterfaceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.direct_vpc_network_interface", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secret_environment_variables` after provisioning.\n"]
    pub fn secret_environment_variables(
        &self,
    ) -> ListRef<Cloudfunctions2FunctionServiceConfigElSecretEnvironmentVariablesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secret_environment_variables", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secret_volumes` after provisioning.\n"]
    pub fn secret_volumes(
        &self,
    ) -> ListRef<Cloudfunctions2FunctionServiceConfigElSecretVolumesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secret_volumes", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Cloudfunctions2FunctionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl Cloudfunctions2FunctionTimeoutsEl {
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
impl ToListMappable for Cloudfunctions2FunctionTimeoutsEl {
    type O = BlockAssignable<Cloudfunctions2FunctionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfunctions2FunctionTimeoutsEl {}
impl BuildCloudfunctions2FunctionTimeoutsEl {
    pub fn build(self) -> Cloudfunctions2FunctionTimeoutsEl {
        Cloudfunctions2FunctionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct Cloudfunctions2FunctionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudfunctions2FunctionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> Cloudfunctions2FunctionTimeoutsElRef {
        Cloudfunctions2FunctionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudfunctions2FunctionTimeoutsElRef {
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
struct Cloudfunctions2FunctionDynamic {
    build_config: Option<DynamicBlock<Cloudfunctions2FunctionBuildConfigEl>>,
    event_trigger: Option<DynamicBlock<Cloudfunctions2FunctionEventTriggerEl>>,
    service_config: Option<DynamicBlock<Cloudfunctions2FunctionServiceConfigEl>>,
}
