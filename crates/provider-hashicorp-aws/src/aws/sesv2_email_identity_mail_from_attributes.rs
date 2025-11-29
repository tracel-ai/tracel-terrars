use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Sesv2EmailIdentityMailFromAttributesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    behavior_on_mx_failure: Option<PrimField<String>>,
    email_identity: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mail_from_domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct Sesv2EmailIdentityMailFromAttributes_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Sesv2EmailIdentityMailFromAttributesData>,
}

#[derive(Clone)]
pub struct Sesv2EmailIdentityMailFromAttributes(Rc<Sesv2EmailIdentityMailFromAttributes_>);

impl Sesv2EmailIdentityMailFromAttributes {
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

    #[doc = "Set the field `behavior_on_mx_failure`.\n"]
    pub fn set_behavior_on_mx_failure(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().behavior_on_mx_failure = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `mail_from_domain`.\n"]
    pub fn set_mail_from_domain(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().mail_from_domain = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `behavior_on_mx_failure` after provisioning.\n"]
    pub fn behavior_on_mx_failure(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.behavior_on_mx_failure", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_identity` after provisioning.\n"]
    pub fn email_identity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mail_from_domain` after provisioning.\n"]
    pub fn mail_from_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mail_from_domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl Referable for Sesv2EmailIdentityMailFromAttributes {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Sesv2EmailIdentityMailFromAttributes { }

impl ToListMappable for Sesv2EmailIdentityMailFromAttributes {
    type O = ListRef<Sesv2EmailIdentityMailFromAttributesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Sesv2EmailIdentityMailFromAttributes_ {
    fn extract_resource_type(&self) -> String {
        "aws_sesv2_email_identity_mail_from_attributes".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSesv2EmailIdentityMailFromAttributes {
    pub tf_id: String,
    #[doc = ""]
    pub email_identity: PrimField<String>,
}

impl BuildSesv2EmailIdentityMailFromAttributes {
    pub fn build(self, stack: &mut Stack) -> Sesv2EmailIdentityMailFromAttributes {
        let out = Sesv2EmailIdentityMailFromAttributes(Rc::new(Sesv2EmailIdentityMailFromAttributes_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Sesv2EmailIdentityMailFromAttributesData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                behavior_on_mx_failure: core::default::Default::default(),
                email_identity: self.email_identity,
                id: core::default::Default::default(),
                mail_from_domain: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Sesv2EmailIdentityMailFromAttributesRef {
    shared: StackShared,
    base: String,
}

impl Ref for Sesv2EmailIdentityMailFromAttributesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl Sesv2EmailIdentityMailFromAttributesRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `behavior_on_mx_failure` after provisioning.\n"]
    pub fn behavior_on_mx_failure(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.behavior_on_mx_failure", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_identity` after provisioning.\n"]
    pub fn email_identity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mail_from_domain` after provisioning.\n"]
    pub fn mail_from_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mail_from_domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}
