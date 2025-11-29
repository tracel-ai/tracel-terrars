use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct IamAccountPasswordPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_users_to_change_password: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hard_expiry: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_password_age: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_password_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_reuse_prevention: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_lowercase_characters: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_numbers: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_symbols: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_uppercase_characters: Option<PrimField<bool>>,
}

struct IamAccountPasswordPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IamAccountPasswordPolicyData>,
}

#[derive(Clone)]
pub struct IamAccountPasswordPolicy(Rc<IamAccountPasswordPolicy_>);

impl IamAccountPasswordPolicy {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `allow_users_to_change_password`.\n"]
    pub fn set_allow_users_to_change_password(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_users_to_change_password = Some(v.into());
        self
    }

    #[doc = "Set the field `hard_expiry`.\n"]
    pub fn set_hard_expiry(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().hard_expiry = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `max_password_age`.\n"]
    pub fn set_max_password_age(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_password_age = Some(v.into());
        self
    }

    #[doc = "Set the field `minimum_password_length`.\n"]
    pub fn set_minimum_password_length(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().minimum_password_length = Some(v.into());
        self
    }

    #[doc = "Set the field `password_reuse_prevention`.\n"]
    pub fn set_password_reuse_prevention(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().password_reuse_prevention = Some(v.into());
        self
    }

    #[doc = "Set the field `require_lowercase_characters`.\n"]
    pub fn set_require_lowercase_characters(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_lowercase_characters = Some(v.into());
        self
    }

    #[doc = "Set the field `require_numbers`.\n"]
    pub fn set_require_numbers(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_numbers = Some(v.into());
        self
    }

    #[doc = "Set the field `require_symbols`.\n"]
    pub fn set_require_symbols(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_symbols = Some(v.into());
        self
    }

    #[doc = "Set the field `require_uppercase_characters`.\n"]
    pub fn set_require_uppercase_characters(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_uppercase_characters = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `allow_users_to_change_password` after provisioning.\n"]
    pub fn allow_users_to_change_password(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_users_to_change_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expire_passwords` after provisioning.\n"]
    pub fn expire_passwords(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.expire_passwords", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hard_expiry` after provisioning.\n"]
    pub fn hard_expiry(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.hard_expiry", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_password_age` after provisioning.\n"]
    pub fn max_password_age(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_password_age", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `minimum_password_length` after provisioning.\n"]
    pub fn minimum_password_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum_password_length", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password_reuse_prevention` after provisioning.\n"]
    pub fn password_reuse_prevention(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_reuse_prevention", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require_lowercase_characters` after provisioning.\n"]
    pub fn require_lowercase_characters(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_lowercase_characters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require_numbers` after provisioning.\n"]
    pub fn require_numbers(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_numbers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require_symbols` after provisioning.\n"]
    pub fn require_symbols(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_symbols", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require_uppercase_characters` after provisioning.\n"]
    pub fn require_uppercase_characters(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_uppercase_characters", self.extract_ref()))
    }
}

impl Referable for IamAccountPasswordPolicy {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for IamAccountPasswordPolicy { }

impl ToListMappable for IamAccountPasswordPolicy {
    type O = ListRef<IamAccountPasswordPolicyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for IamAccountPasswordPolicy_ {
    fn extract_resource_type(&self) -> String {
        "aws_iam_account_password_policy".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIamAccountPasswordPolicy {
    pub tf_id: String,
}

impl BuildIamAccountPasswordPolicy {
    pub fn build(self, stack: &mut Stack) -> IamAccountPasswordPolicy {
        let out = IamAccountPasswordPolicy(Rc::new(IamAccountPasswordPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IamAccountPasswordPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allow_users_to_change_password: core::default::Default::default(),
                hard_expiry: core::default::Default::default(),
                id: core::default::Default::default(),
                max_password_age: core::default::Default::default(),
                minimum_password_length: core::default::Default::default(),
                password_reuse_prevention: core::default::Default::default(),
                require_lowercase_characters: core::default::Default::default(),
                require_numbers: core::default::Default::default(),
                require_symbols: core::default::Default::default(),
                require_uppercase_characters: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IamAccountPasswordPolicyRef {
    shared: StackShared,
    base: String,
}

impl Ref for IamAccountPasswordPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl IamAccountPasswordPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_users_to_change_password` after provisioning.\n"]
    pub fn allow_users_to_change_password(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_users_to_change_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expire_passwords` after provisioning.\n"]
    pub fn expire_passwords(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.expire_passwords", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hard_expiry` after provisioning.\n"]
    pub fn hard_expiry(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.hard_expiry", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_password_age` after provisioning.\n"]
    pub fn max_password_age(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_password_age", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `minimum_password_length` after provisioning.\n"]
    pub fn minimum_password_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum_password_length", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password_reuse_prevention` after provisioning.\n"]
    pub fn password_reuse_prevention(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_reuse_prevention", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require_lowercase_characters` after provisioning.\n"]
    pub fn require_lowercase_characters(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_lowercase_characters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require_numbers` after provisioning.\n"]
    pub fn require_numbers(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_numbers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require_symbols` after provisioning.\n"]
    pub fn require_symbols(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_symbols", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require_uppercase_characters` after provisioning.\n"]
    pub fn require_uppercase_characters(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_uppercase_characters", self.extract_ref()))
    }
}
