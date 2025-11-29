use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct IamUserLoginProfileData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_reset_required: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pgp_key: Option<PrimField<String>>,
    user: PrimField<String>,
}

struct IamUserLoginProfile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IamUserLoginProfileData>,
}

#[derive(Clone)]
pub struct IamUserLoginProfile(Rc<IamUserLoginProfile_>);

impl IamUserLoginProfile {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `password_length`.\n"]
    pub fn set_password_length(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().password_length = Some(v.into());
        self
    }

    #[doc = "Set the field `password_reset_required`.\n"]
    pub fn set_password_reset_required(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().password_reset_required = Some(v.into());
        self
    }

    #[doc = "Set the field `pgp_key`.\n"]
    pub fn set_pgp_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().pgp_key = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `encrypted_password` after provisioning.\n"]
    pub fn encrypted_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encrypted_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_fingerprint` after provisioning.\n"]
    pub fn key_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_fingerprint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password_length` after provisioning.\n"]
    pub fn password_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_length", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password_reset_required` after provisioning.\n"]
    pub fn password_reset_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_reset_required", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pgp_key` after provisioning.\n"]
    pub fn pgp_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pgp_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }
}

impl Referable for IamUserLoginProfile {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for IamUserLoginProfile { }

impl ToListMappable for IamUserLoginProfile {
    type O = ListRef<IamUserLoginProfileRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for IamUserLoginProfile_ {
    fn extract_resource_type(&self) -> String {
        "aws_iam_user_login_profile".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIamUserLoginProfile {
    pub tf_id: String,
    #[doc = ""]
    pub user: PrimField<String>,
}

impl BuildIamUserLoginProfile {
    pub fn build(self, stack: &mut Stack) -> IamUserLoginProfile {
        let out = IamUserLoginProfile(Rc::new(IamUserLoginProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IamUserLoginProfileData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                password_length: core::default::Default::default(),
                password_reset_required: core::default::Default::default(),
                pgp_key: core::default::Default::default(),
                user: self.user,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IamUserLoginProfileRef {
    shared: StackShared,
    base: String,
}

impl Ref for IamUserLoginProfileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl IamUserLoginProfileRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encrypted_password` after provisioning.\n"]
    pub fn encrypted_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encrypted_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_fingerprint` after provisioning.\n"]
    pub fn key_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_fingerprint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password_length` after provisioning.\n"]
    pub fn password_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_length", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password_reset_required` after provisioning.\n"]
    pub fn password_reset_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_reset_required", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pgp_key` after provisioning.\n"]
    pub fn pgp_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pgp_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }
}
