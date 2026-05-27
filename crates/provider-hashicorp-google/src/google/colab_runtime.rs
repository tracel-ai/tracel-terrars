use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ColabRuntimeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_upgrade: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_state: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    runtime_user: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notebook_runtime_template_ref: Option<Vec<ColabRuntimeNotebookRuntimeTemplateRefEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ColabRuntimeTimeoutsEl>,
    dynamic: ColabRuntimeDynamic,
}
struct ColabRuntime_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ColabRuntimeData>,
}
#[derive(Clone)]
pub struct ColabRuntime(Rc<ColabRuntime_>);
impl ColabRuntime {
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
    #[doc = "Set the field `auto_upgrade`.\nTriggers an upgrade anytime the runtime is started if it is upgradable."]
    pub fn set_auto_upgrade(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_upgrade = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nThe description of the Runtime."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `desired_state`.\nDesired state of the Colab Runtime. Set this field to 'RUNNING' to start the runtime, and 'STOPPED' to stop it."]
    pub fn set_desired_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().desired_state = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nThe resource name of the Runtime"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `notebook_runtime_template_ref`.\n"]
    pub fn set_notebook_runtime_template_ref(
        self,
        v: impl Into<BlockAssignable<ColabRuntimeNotebookRuntimeTemplateRefEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notebook_runtime_template_ref = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .notebook_runtime_template_ref = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ColabRuntimeTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `auto_upgrade` after provisioning.\nTriggers an upgrade anytime the runtime is started if it is upgradable."]
    pub fn auto_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_upgrade", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the Runtime."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desired_state` after provisioning.\nDesired state of the Colab Runtime. Set this field to 'RUNNING' to start the runtime, and 'STOPPED' to stop it."]
    pub fn desired_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the Runtime."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expiration_time` after provisioning.\nOutput only. Timestamp when this NotebookRuntime will be expired."]
    pub fn expiration_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expiration_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `is_upgradable` after provisioning.\nOutput only. Checks if the NotebookRuntime is upgradable."]
    pub fn is_upgradable(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_upgradable", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource: https://cloud.google.com/colab/docs/locations"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Runtime"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notebook_runtime_type` after provisioning.\nOutput only. The type of the notebook runtime."]
    pub fn notebook_runtime_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notebook_runtime_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `runtime_user` after provisioning.\nThe user email of the NotebookRuntime."]
    pub fn runtime_user(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.runtime_user", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The state of the runtime."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notebook_runtime_template_ref` after provisioning.\n"]
    pub fn notebook_runtime_template_ref(
        &self,
    ) -> ListRef<ColabRuntimeNotebookRuntimeTemplateRefElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notebook_runtime_template_ref", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ColabRuntimeTimeoutsElRef {
        ColabRuntimeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ColabRuntime {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ColabRuntime {}
impl ToListMappable for ColabRuntime {
    type O = ListRef<ColabRuntimeRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ColabRuntime_ {
    fn extract_resource_type(&self) -> String {
        "google_colab_runtime".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildColabRuntime {
    pub tf_id: String,
    #[doc = "Required. The display name of the Runtime."]
    pub display_name: PrimField<String>,
    #[doc = "The location for the resource: https://cloud.google.com/colab/docs/locations"]
    pub location: PrimField<String>,
    #[doc = "The user email of the NotebookRuntime."]
    pub runtime_user: PrimField<String>,
}
impl BuildColabRuntime {
    pub fn build(self, stack: &mut Stack) -> ColabRuntime {
        let out = ColabRuntime(Rc::new(ColabRuntime_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ColabRuntimeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                auto_upgrade: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                desired_state: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                location: self.location,
                name: core::default::Default::default(),
                project: core::default::Default::default(),
                runtime_user: self.runtime_user,
                notebook_runtime_template_ref: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ColabRuntimeRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabRuntimeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ColabRuntimeRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto_upgrade` after provisioning.\nTriggers an upgrade anytime the runtime is started if it is upgradable."]
    pub fn auto_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_upgrade", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the Runtime."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desired_state` after provisioning.\nDesired state of the Colab Runtime. Set this field to 'RUNNING' to start the runtime, and 'STOPPED' to stop it."]
    pub fn desired_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the Runtime."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expiration_time` after provisioning.\nOutput only. Timestamp when this NotebookRuntime will be expired."]
    pub fn expiration_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expiration_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `is_upgradable` after provisioning.\nOutput only. Checks if the NotebookRuntime is upgradable."]
    pub fn is_upgradable(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_upgradable", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource: https://cloud.google.com/colab/docs/locations"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Runtime"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notebook_runtime_type` after provisioning.\nOutput only. The type of the notebook runtime."]
    pub fn notebook_runtime_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notebook_runtime_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `runtime_user` after provisioning.\nThe user email of the NotebookRuntime."]
    pub fn runtime_user(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.runtime_user", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The state of the runtime."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notebook_runtime_template_ref` after provisioning.\n"]
    pub fn notebook_runtime_template_ref(
        &self,
    ) -> ListRef<ColabRuntimeNotebookRuntimeTemplateRefElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notebook_runtime_template_ref", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ColabRuntimeTimeoutsElRef {
        ColabRuntimeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ColabRuntimeNotebookRuntimeTemplateRefEl {
    notebook_runtime_template: PrimField<String>,
}
impl ColabRuntimeNotebookRuntimeTemplateRefEl {}
impl ToListMappable for ColabRuntimeNotebookRuntimeTemplateRefEl {
    type O = BlockAssignable<ColabRuntimeNotebookRuntimeTemplateRefEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildColabRuntimeNotebookRuntimeTemplateRefEl {
    #[doc = "The resource name of the NotebookRuntimeTemplate based on which a NotebookRuntime will be created."]
    pub notebook_runtime_template: PrimField<String>,
}
impl BuildColabRuntimeNotebookRuntimeTemplateRefEl {
    pub fn build(self) -> ColabRuntimeNotebookRuntimeTemplateRefEl {
        ColabRuntimeNotebookRuntimeTemplateRefEl {
            notebook_runtime_template: self.notebook_runtime_template,
        }
    }
}
pub struct ColabRuntimeNotebookRuntimeTemplateRefElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabRuntimeNotebookRuntimeTemplateRefElRef {
    fn new(shared: StackShared, base: String) -> ColabRuntimeNotebookRuntimeTemplateRefElRef {
        ColabRuntimeNotebookRuntimeTemplateRefElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ColabRuntimeNotebookRuntimeTemplateRefElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `notebook_runtime_template` after provisioning.\nThe resource name of the NotebookRuntimeTemplate based on which a NotebookRuntime will be created."]
    pub fn notebook_runtime_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notebook_runtime_template", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ColabRuntimeTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ColabRuntimeTimeoutsEl {
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
impl ToListMappable for ColabRuntimeTimeoutsEl {
    type O = BlockAssignable<ColabRuntimeTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildColabRuntimeTimeoutsEl {}
impl BuildColabRuntimeTimeoutsEl {
    pub fn build(self) -> ColabRuntimeTimeoutsEl {
        ColabRuntimeTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ColabRuntimeTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabRuntimeTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ColabRuntimeTimeoutsElRef {
        ColabRuntimeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ColabRuntimeTimeoutsElRef {
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
struct ColabRuntimeDynamic {
    notebook_runtime_template_ref: Option<DynamicBlock<ColabRuntimeNotebookRuntimeTemplateRefEl>>,
}
