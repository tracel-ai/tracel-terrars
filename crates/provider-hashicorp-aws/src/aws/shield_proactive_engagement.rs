use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ShieldProactiveEngagementData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emergency_contact: Option<Vec<ShieldProactiveEngagementEmergencyContactEl>>,
    dynamic: ShieldProactiveEngagementDynamic,
}

struct ShieldProactiveEngagement_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ShieldProactiveEngagementData>,
}

#[derive(Clone)]
pub struct ShieldProactiveEngagement(Rc<ShieldProactiveEngagement_>);

impl ShieldProactiveEngagement {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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

    #[doc = "Set the field `emergency_contact`.\n"]
    pub fn set_emergency_contact(
        self,
        v: impl Into<BlockAssignable<ShieldProactiveEngagementEmergencyContactEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().emergency_contact = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.emergency_contact = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `emergency_contact` after provisioning.\n"]
    pub fn emergency_contact(&self) -> ListRef<ShieldProactiveEngagementEmergencyContactElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.emergency_contact", self.extract_ref()),
        )
    }
}

impl Referable for ShieldProactiveEngagement {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for ShieldProactiveEngagement {}

impl ToListMappable for ShieldProactiveEngagement {
    type O = ListRef<ShieldProactiveEngagementRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ShieldProactiveEngagement_ {
    fn extract_resource_type(&self) -> String {
        "aws_shield_proactive_engagement".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildShieldProactiveEngagement {
    pub tf_id: String,
    #[doc = ""]
    pub enabled: PrimField<bool>,
}

impl BuildShieldProactiveEngagement {
    pub fn build(self, stack: &mut Stack) -> ShieldProactiveEngagement {
        let out = ShieldProactiveEngagement(Rc::new(ShieldProactiveEngagement_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ShieldProactiveEngagementData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                enabled: self.enabled,
                emergency_contact: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ShieldProactiveEngagementRef {
    shared: StackShared,
    base: String,
}

impl Ref for ShieldProactiveEngagementRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl ShieldProactiveEngagementRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `emergency_contact` after provisioning.\n"]
    pub fn emergency_contact(&self) -> ListRef<ShieldProactiveEngagementEmergencyContactElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.emergency_contact", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ShieldProactiveEngagementEmergencyContactEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_notes: Option<PrimField<String>>,
    email_address: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number: Option<PrimField<String>>,
}

impl ShieldProactiveEngagementEmergencyContactEl {
    #[doc = "Set the field `contact_notes`.\n"]
    pub fn set_contact_notes(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contact_notes = Some(v.into());
        self
    }

    #[doc = "Set the field `phone_number`.\n"]
    pub fn set_phone_number(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.phone_number = Some(v.into());
        self
    }
}

impl ToListMappable for ShieldProactiveEngagementEmergencyContactEl {
    type O = BlockAssignable<ShieldProactiveEngagementEmergencyContactEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildShieldProactiveEngagementEmergencyContactEl {
    #[doc = ""]
    pub email_address: PrimField<String>,
}

impl BuildShieldProactiveEngagementEmergencyContactEl {
    pub fn build(self) -> ShieldProactiveEngagementEmergencyContactEl {
        ShieldProactiveEngagementEmergencyContactEl {
            contact_notes: core::default::Default::default(),
            email_address: self.email_address,
            phone_number: core::default::Default::default(),
        }
    }
}

pub struct ShieldProactiveEngagementEmergencyContactElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ShieldProactiveEngagementEmergencyContactElRef {
    fn new(shared: StackShared, base: String) -> ShieldProactiveEngagementEmergencyContactElRef {
        ShieldProactiveEngagementEmergencyContactElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ShieldProactiveEngagementEmergencyContactElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `contact_notes` after provisioning.\n"]
    pub fn contact_notes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.contact_notes", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_address` after provisioning.\n"]
    pub fn email_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.email_address", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
}

#[derive(Serialize, Default)]
struct ShieldProactiveEngagementDynamic {
    emergency_contact: Option<DynamicBlock<ShieldProactiveEngagementEmergencyContactEl>>,
}
