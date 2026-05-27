use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputePreviewFeatureData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    activation_status: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rollout_operation: Option<Vec<ComputePreviewFeatureRolloutOperationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputePreviewFeatureTimeoutsEl>,
    dynamic: ComputePreviewFeatureDynamic,
}
struct ComputePreviewFeature_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputePreviewFeatureData>,
}
#[derive(Clone)]
pub struct ComputePreviewFeature(Rc<ComputePreviewFeature_>);
impl ComputePreviewFeature {
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
    #[doc = "Set the field `rollout_operation`.\n"]
    pub fn set_rollout_operation(
        self,
        v: impl Into<BlockAssignable<ComputePreviewFeatureRolloutOperationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rollout_operation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rollout_operation = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputePreviewFeatureTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `activation_status` after provisioning.\nThe activation status of the preview feature. Possible values: [\"ENABLED\", \"ACTIVATION_STATE_UNSPECIFIED\"]"]
    pub fn activation_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.activation_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the preview feature."]
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
    #[doc = "Get a reference to the value of field `rollout_operation` after provisioning.\n"]
    pub fn rollout_operation(&self) -> ListRef<ComputePreviewFeatureRolloutOperationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rollout_operation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputePreviewFeatureTimeoutsElRef {
        ComputePreviewFeatureTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputePreviewFeature {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputePreviewFeature {}
impl ToListMappable for ComputePreviewFeature {
    type O = ListRef<ComputePreviewFeatureRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputePreviewFeature_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_preview_feature".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputePreviewFeature {
    pub tf_id: String,
    #[doc = "The activation status of the preview feature. Possible values: [\"ENABLED\", \"ACTIVATION_STATE_UNSPECIFIED\"]"]
    pub activation_status: PrimField<String>,
    #[doc = "The name of the preview feature."]
    pub name: PrimField<String>,
}
impl BuildComputePreviewFeature {
    pub fn build(self, stack: &mut Stack) -> ComputePreviewFeature {
        let out = ComputePreviewFeature(Rc::new(ComputePreviewFeature_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputePreviewFeatureData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                activation_status: self.activation_status,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                rollout_operation: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputePreviewFeatureRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputePreviewFeatureRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputePreviewFeatureRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `activation_status` after provisioning.\nThe activation status of the preview feature. Possible values: [\"ENABLED\", \"ACTIVATION_STATE_UNSPECIFIED\"]"]
    pub fn activation_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.activation_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the preview feature."]
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
    #[doc = "Get a reference to the value of field `rollout_operation` after provisioning.\n"]
    pub fn rollout_operation(&self) -> ListRef<ComputePreviewFeatureRolloutOperationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rollout_operation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputePreviewFeatureTimeoutsElRef {
        ComputePreviewFeatureTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputePreviewFeatureRolloutOperationElRolloutInputEl {
    predefined_rollout_plan: PrimField<String>,
}
impl ComputePreviewFeatureRolloutOperationElRolloutInputEl {}
impl ToListMappable for ComputePreviewFeatureRolloutOperationElRolloutInputEl {
    type O = BlockAssignable<ComputePreviewFeatureRolloutOperationElRolloutInputEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputePreviewFeatureRolloutOperationElRolloutInputEl {
    #[doc = "Predefined rollout plans. Possible values: [\"ROLLOUT_PLAN_FAST_ROLLOUT\"]"]
    pub predefined_rollout_plan: PrimField<String>,
}
impl BuildComputePreviewFeatureRolloutOperationElRolloutInputEl {
    pub fn build(self) -> ComputePreviewFeatureRolloutOperationElRolloutInputEl {
        ComputePreviewFeatureRolloutOperationElRolloutInputEl {
            predefined_rollout_plan: self.predefined_rollout_plan,
        }
    }
}
pub struct ComputePreviewFeatureRolloutOperationElRolloutInputElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputePreviewFeatureRolloutOperationElRolloutInputElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputePreviewFeatureRolloutOperationElRolloutInputElRef {
        ComputePreviewFeatureRolloutOperationElRolloutInputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputePreviewFeatureRolloutOperationElRolloutInputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `predefined_rollout_plan` after provisioning.\nPredefined rollout plans. Possible values: [\"ROLLOUT_PLAN_FAST_ROLLOUT\"]"]
    pub fn predefined_rollout_plan(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.predefined_rollout_plan", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ComputePreviewFeatureRolloutOperationElDynamic {
    rollout_input: Option<DynamicBlock<ComputePreviewFeatureRolloutOperationElRolloutInputEl>>,
}
#[derive(Serialize)]
pub struct ComputePreviewFeatureRolloutOperationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rollout_input: Option<Vec<ComputePreviewFeatureRolloutOperationElRolloutInputEl>>,
    dynamic: ComputePreviewFeatureRolloutOperationElDynamic,
}
impl ComputePreviewFeatureRolloutOperationEl {
    #[doc = "Set the field `rollout_input`.\n"]
    pub fn set_rollout_input(
        mut self,
        v: impl Into<BlockAssignable<ComputePreviewFeatureRolloutOperationElRolloutInputEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rollout_input = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rollout_input = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputePreviewFeatureRolloutOperationEl {
    type O = BlockAssignable<ComputePreviewFeatureRolloutOperationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputePreviewFeatureRolloutOperationEl {}
impl BuildComputePreviewFeatureRolloutOperationEl {
    pub fn build(self) -> ComputePreviewFeatureRolloutOperationEl {
        ComputePreviewFeatureRolloutOperationEl {
            rollout_input: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputePreviewFeatureRolloutOperationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputePreviewFeatureRolloutOperationElRef {
    fn new(shared: StackShared, base: String) -> ComputePreviewFeatureRolloutOperationElRef {
        ComputePreviewFeatureRolloutOperationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputePreviewFeatureRolloutOperationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `rollout_input` after provisioning.\n"]
    pub fn rollout_input(
        &self,
    ) -> ListRef<ComputePreviewFeatureRolloutOperationElRolloutInputElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rollout_input", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputePreviewFeatureTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputePreviewFeatureTimeoutsEl {
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
impl ToListMappable for ComputePreviewFeatureTimeoutsEl {
    type O = BlockAssignable<ComputePreviewFeatureTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputePreviewFeatureTimeoutsEl {}
impl BuildComputePreviewFeatureTimeoutsEl {
    pub fn build(self) -> ComputePreviewFeatureTimeoutsEl {
        ComputePreviewFeatureTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputePreviewFeatureTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputePreviewFeatureTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputePreviewFeatureTimeoutsElRef {
        ComputePreviewFeatureTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputePreviewFeatureTimeoutsElRef {
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
struct ComputePreviewFeatureDynamic {
    rollout_operation: Option<DynamicBlock<ComputePreviewFeatureRolloutOperationEl>>,
}
