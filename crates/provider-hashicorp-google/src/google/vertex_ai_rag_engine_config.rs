use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct VertexAiRagEngineConfigData {
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
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rag_managed_db_config: Option<Vec<VertexAiRagEngineConfigRagManagedDbConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VertexAiRagEngineConfigTimeoutsEl>,
    dynamic: VertexAiRagEngineConfigDynamic,
}
struct VertexAiRagEngineConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VertexAiRagEngineConfigData>,
}
#[derive(Clone)]
pub struct VertexAiRagEngineConfig(Rc<VertexAiRagEngineConfig_>);
impl VertexAiRagEngineConfig {
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
    #[doc = "Set the field `region`.\nThe region of the RagEngineConfig. eg us-central1"]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `rag_managed_db_config`.\n"]
    pub fn set_rag_managed_db_config(
        self,
        v: impl Into<BlockAssignable<VertexAiRagEngineConfigRagManagedDbConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rag_managed_db_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rag_managed_db_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VertexAiRagEngineConfigTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Dataset. This value is set by Google."]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the RagEngineConfig. eg us-central1"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rag_managed_db_config` after provisioning.\n"]
    pub fn rag_managed_db_config(&self) -> ListRef<VertexAiRagEngineConfigRagManagedDbConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rag_managed_db_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiRagEngineConfigTimeoutsElRef {
        VertexAiRagEngineConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for VertexAiRagEngineConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for VertexAiRagEngineConfig {}
impl ToListMappable for VertexAiRagEngineConfig {
    type O = ListRef<VertexAiRagEngineConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for VertexAiRagEngineConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_vertex_ai_rag_engine_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildVertexAiRagEngineConfig {
    pub tf_id: String,
}
impl BuildVertexAiRagEngineConfig {
    pub fn build(self, stack: &mut Stack) -> VertexAiRagEngineConfig {
        let out = VertexAiRagEngineConfig(Rc::new(VertexAiRagEngineConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VertexAiRagEngineConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                rag_managed_db_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct VertexAiRagEngineConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiRagEngineConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl VertexAiRagEngineConfigRef {
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Dataset. This value is set by Google."]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the RagEngineConfig. eg us-central1"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rag_managed_db_config` after provisioning.\n"]
    pub fn rag_managed_db_config(&self) -> ListRef<VertexAiRagEngineConfigRagManagedDbConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rag_managed_db_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiRagEngineConfigTimeoutsElRef {
        VertexAiRagEngineConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiRagEngineConfigRagManagedDbConfigElBasicEl {}
impl VertexAiRagEngineConfigRagManagedDbConfigElBasicEl {}
impl ToListMappable for VertexAiRagEngineConfigRagManagedDbConfigElBasicEl {
    type O = BlockAssignable<VertexAiRagEngineConfigRagManagedDbConfigElBasicEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiRagEngineConfigRagManagedDbConfigElBasicEl {}
impl BuildVertexAiRagEngineConfigRagManagedDbConfigElBasicEl {
    pub fn build(self) -> VertexAiRagEngineConfigRagManagedDbConfigElBasicEl {
        VertexAiRagEngineConfigRagManagedDbConfigElBasicEl {}
    }
}
pub struct VertexAiRagEngineConfigRagManagedDbConfigElBasicElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiRagEngineConfigRagManagedDbConfigElBasicElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiRagEngineConfigRagManagedDbConfigElBasicElRef {
        VertexAiRagEngineConfigRagManagedDbConfigElBasicElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiRagEngineConfigRagManagedDbConfigElBasicElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct VertexAiRagEngineConfigRagManagedDbConfigElScaledEl {}
impl VertexAiRagEngineConfigRagManagedDbConfigElScaledEl {}
impl ToListMappable for VertexAiRagEngineConfigRagManagedDbConfigElScaledEl {
    type O = BlockAssignable<VertexAiRagEngineConfigRagManagedDbConfigElScaledEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiRagEngineConfigRagManagedDbConfigElScaledEl {}
impl BuildVertexAiRagEngineConfigRagManagedDbConfigElScaledEl {
    pub fn build(self) -> VertexAiRagEngineConfigRagManagedDbConfigElScaledEl {
        VertexAiRagEngineConfigRagManagedDbConfigElScaledEl {}
    }
}
pub struct VertexAiRagEngineConfigRagManagedDbConfigElScaledElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiRagEngineConfigRagManagedDbConfigElScaledElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiRagEngineConfigRagManagedDbConfigElScaledElRef {
        VertexAiRagEngineConfigRagManagedDbConfigElScaledElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiRagEngineConfigRagManagedDbConfigElScaledElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct VertexAiRagEngineConfigRagManagedDbConfigElUnprovisionedEl {}
impl VertexAiRagEngineConfigRagManagedDbConfigElUnprovisionedEl {}
impl ToListMappable for VertexAiRagEngineConfigRagManagedDbConfigElUnprovisionedEl {
    type O = BlockAssignable<VertexAiRagEngineConfigRagManagedDbConfigElUnprovisionedEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiRagEngineConfigRagManagedDbConfigElUnprovisionedEl {}
impl BuildVertexAiRagEngineConfigRagManagedDbConfigElUnprovisionedEl {
    pub fn build(self) -> VertexAiRagEngineConfigRagManagedDbConfigElUnprovisionedEl {
        VertexAiRagEngineConfigRagManagedDbConfigElUnprovisionedEl {}
    }
}
pub struct VertexAiRagEngineConfigRagManagedDbConfigElUnprovisionedElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiRagEngineConfigRagManagedDbConfigElUnprovisionedElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiRagEngineConfigRagManagedDbConfigElUnprovisionedElRef {
        VertexAiRagEngineConfigRagManagedDbConfigElUnprovisionedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiRagEngineConfigRagManagedDbConfigElUnprovisionedElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct VertexAiRagEngineConfigRagManagedDbConfigElDynamic {
    basic: Option<DynamicBlock<VertexAiRagEngineConfigRagManagedDbConfigElBasicEl>>,
    scaled: Option<DynamicBlock<VertexAiRagEngineConfigRagManagedDbConfigElScaledEl>>,
    unprovisioned: Option<DynamicBlock<VertexAiRagEngineConfigRagManagedDbConfigElUnprovisionedEl>>,
}
#[derive(Serialize)]
pub struct VertexAiRagEngineConfigRagManagedDbConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    basic: Option<Vec<VertexAiRagEngineConfigRagManagedDbConfigElBasicEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scaled: Option<Vec<VertexAiRagEngineConfigRagManagedDbConfigElScaledEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unprovisioned: Option<Vec<VertexAiRagEngineConfigRagManagedDbConfigElUnprovisionedEl>>,
    dynamic: VertexAiRagEngineConfigRagManagedDbConfigElDynamic,
}
impl VertexAiRagEngineConfigRagManagedDbConfigEl {
    #[doc = "Set the field `basic`.\n"]
    pub fn set_basic(
        mut self,
        v: impl Into<BlockAssignable<VertexAiRagEngineConfigRagManagedDbConfigElBasicEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.basic = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.basic = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `scaled`.\n"]
    pub fn set_scaled(
        mut self,
        v: impl Into<BlockAssignable<VertexAiRagEngineConfigRagManagedDbConfigElScaledEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.scaled = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.scaled = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `unprovisioned`.\n"]
    pub fn set_unprovisioned(
        mut self,
        v: impl Into<BlockAssignable<VertexAiRagEngineConfigRagManagedDbConfigElUnprovisionedEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.unprovisioned = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.unprovisioned = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VertexAiRagEngineConfigRagManagedDbConfigEl {
    type O = BlockAssignable<VertexAiRagEngineConfigRagManagedDbConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiRagEngineConfigRagManagedDbConfigEl {}
impl BuildVertexAiRagEngineConfigRagManagedDbConfigEl {
    pub fn build(self) -> VertexAiRagEngineConfigRagManagedDbConfigEl {
        VertexAiRagEngineConfigRagManagedDbConfigEl {
            basic: core::default::Default::default(),
            scaled: core::default::Default::default(),
            unprovisioned: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiRagEngineConfigRagManagedDbConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiRagEngineConfigRagManagedDbConfigElRef {
    fn new(shared: StackShared, base: String) -> VertexAiRagEngineConfigRagManagedDbConfigElRef {
        VertexAiRagEngineConfigRagManagedDbConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiRagEngineConfigRagManagedDbConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `basic` after provisioning.\n"]
    pub fn basic(&self) -> ListRef<VertexAiRagEngineConfigRagManagedDbConfigElBasicElRef> {
        ListRef::new(self.shared().clone(), format!("{}.basic", self.base))
    }
    #[doc = "Get a reference to the value of field `scaled` after provisioning.\n"]
    pub fn scaled(&self) -> ListRef<VertexAiRagEngineConfigRagManagedDbConfigElScaledElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scaled", self.base))
    }
    #[doc = "Get a reference to the value of field `unprovisioned` after provisioning.\n"]
    pub fn unprovisioned(
        &self,
    ) -> ListRef<VertexAiRagEngineConfigRagManagedDbConfigElUnprovisionedElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.unprovisioned", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiRagEngineConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl VertexAiRagEngineConfigTimeoutsEl {
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
impl ToListMappable for VertexAiRagEngineConfigTimeoutsEl {
    type O = BlockAssignable<VertexAiRagEngineConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiRagEngineConfigTimeoutsEl {}
impl BuildVertexAiRagEngineConfigTimeoutsEl {
    pub fn build(self) -> VertexAiRagEngineConfigTimeoutsEl {
        VertexAiRagEngineConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct VertexAiRagEngineConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiRagEngineConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VertexAiRagEngineConfigTimeoutsElRef {
        VertexAiRagEngineConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiRagEngineConfigTimeoutsElRef {
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
struct VertexAiRagEngineConfigDynamic {
    rag_managed_db_config: Option<DynamicBlock<VertexAiRagEngineConfigRagManagedDbConfigEl>>,
}
