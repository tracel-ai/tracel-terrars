use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct HealthcareHl7V2StoreIamBindingData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    hl7_v2_store_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    members: SetField<PrimField<String>>,
    role: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Vec<HealthcareHl7V2StoreIamBindingConditionEl>>,
    dynamic: HealthcareHl7V2StoreIamBindingDynamic,
}
struct HealthcareHl7V2StoreIamBinding_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<HealthcareHl7V2StoreIamBindingData>,
}
#[derive(Clone)]
pub struct HealthcareHl7V2StoreIamBinding(Rc<HealthcareHl7V2StoreIamBinding_>);
impl HealthcareHl7V2StoreIamBinding {
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
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        self,
        v: impl Into<BlockAssignable<HealthcareHl7V2StoreIamBindingConditionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.condition = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hl7_v2_store_id` after provisioning.\n"]
    pub fn hl7_v2_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hl7_v2_store_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `members` after provisioning.\n"]
    pub fn members(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.members", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> ListRef<HealthcareHl7V2StoreIamBindingConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.condition", self.extract_ref()),
        )
    }
}
impl Referable for HealthcareHl7V2StoreIamBinding {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for HealthcareHl7V2StoreIamBinding {}
impl ToListMappable for HealthcareHl7V2StoreIamBinding {
    type O = ListRef<HealthcareHl7V2StoreIamBindingRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for HealthcareHl7V2StoreIamBinding_ {
    fn extract_resource_type(&self) -> String {
        "google_healthcare_hl7_v2_store_iam_binding".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildHealthcareHl7V2StoreIamBinding {
    pub tf_id: String,
    #[doc = ""]
    pub hl7_v2_store_id: PrimField<String>,
    #[doc = ""]
    pub members: SetField<PrimField<String>>,
    #[doc = ""]
    pub role: PrimField<String>,
}
impl BuildHealthcareHl7V2StoreIamBinding {
    pub fn build(self, stack: &mut Stack) -> HealthcareHl7V2StoreIamBinding {
        let out = HealthcareHl7V2StoreIamBinding(Rc::new(HealthcareHl7V2StoreIamBinding_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(HealthcareHl7V2StoreIamBindingData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                hl7_v2_store_id: self.hl7_v2_store_id,
                id: core::default::Default::default(),
                members: self.members,
                role: self.role,
                condition: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct HealthcareHl7V2StoreIamBindingRef {
    shared: StackShared,
    base: String,
}
impl Ref for HealthcareHl7V2StoreIamBindingRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl HealthcareHl7V2StoreIamBindingRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hl7_v2_store_id` after provisioning.\n"]
    pub fn hl7_v2_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hl7_v2_store_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `members` after provisioning.\n"]
    pub fn members(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.members", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> ListRef<HealthcareHl7V2StoreIamBindingConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.condition", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct HealthcareHl7V2StoreIamBindingConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    expression: PrimField<String>,
    title: PrimField<String>,
}
impl HealthcareHl7V2StoreIamBindingConditionEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
}
impl ToListMappable for HealthcareHl7V2StoreIamBindingConditionEl {
    type O = BlockAssignable<HealthcareHl7V2StoreIamBindingConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHealthcareHl7V2StoreIamBindingConditionEl {
    #[doc = ""]
    pub expression: PrimField<String>,
    #[doc = ""]
    pub title: PrimField<String>,
}
impl BuildHealthcareHl7V2StoreIamBindingConditionEl {
    pub fn build(self) -> HealthcareHl7V2StoreIamBindingConditionEl {
        HealthcareHl7V2StoreIamBindingConditionEl {
            description: core::default::Default::default(),
            expression: self.expression,
            title: self.title,
        }
    }
}
pub struct HealthcareHl7V2StoreIamBindingConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HealthcareHl7V2StoreIamBindingConditionElRef {
    fn new(shared: StackShared, base: String) -> HealthcareHl7V2StoreIamBindingConditionElRef {
        HealthcareHl7V2StoreIamBindingConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HealthcareHl7V2StoreIamBindingConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}
#[derive(Serialize, Default)]
struct HealthcareHl7V2StoreIamBindingDynamic {
    condition: Option<DynamicBlock<HealthcareHl7V2StoreIamBindingConditionEl>>,
}
