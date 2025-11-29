use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct IotCaCertificateData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    active: PrimField<bool>,
    allow_auto_registration: PrimField<bool>,
    ca_certificate_pem: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification_certificate_pem: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registration_config: Option<Vec<IotCaCertificateRegistrationConfigEl>>,
    dynamic: IotCaCertificateDynamic,
}

struct IotCaCertificate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IotCaCertificateData>,
}

#[derive(Clone)]
pub struct IotCaCertificate(Rc<IotCaCertificate_>);

impl IotCaCertificate {
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

    #[doc = "Set the field `certificate_mode`.\n"]
    pub fn set_certificate_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().certificate_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `verification_certificate_pem`.\n"]
    pub fn set_verification_certificate_pem(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().verification_certificate_pem = Some(v.into());
        self
    }

    #[doc = "Set the field `registration_config`.\n"]
    pub fn set_registration_config(self, v: impl Into<BlockAssignable<IotCaCertificateRegistrationConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().registration_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.registration_config = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `active` after provisioning.\n"]
    pub fn active(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.active", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allow_auto_registration` after provisioning.\n"]
    pub fn allow_auto_registration(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_auto_registration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ca_certificate_pem` after provisioning.\n"]
    pub fn ca_certificate_pem(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ca_certificate_pem", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_mode` after provisioning.\n"]
    pub fn certificate_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_version` after provisioning.\n"]
    pub fn customer_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `generation_id` after provisioning.\n"]
    pub fn generation_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.generation_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validity` after provisioning.\n"]
    pub fn validity(&self) -> ListRef<IotCaCertificateValidityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.validity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `verification_certificate_pem` after provisioning.\n"]
    pub fn verification_certificate_pem(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.verification_certificate_pem", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registration_config` after provisioning.\n"]
    pub fn registration_config(&self) -> ListRef<IotCaCertificateRegistrationConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.registration_config", self.extract_ref()))
    }
}

impl Referable for IotCaCertificate {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for IotCaCertificate { }

impl ToListMappable for IotCaCertificate {
    type O = ListRef<IotCaCertificateRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for IotCaCertificate_ {
    fn extract_resource_type(&self) -> String {
        "aws_iot_ca_certificate".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIotCaCertificate {
    pub tf_id: String,
    #[doc = ""]
    pub active: PrimField<bool>,
    #[doc = ""]
    pub allow_auto_registration: PrimField<bool>,
    #[doc = ""]
    pub ca_certificate_pem: PrimField<String>,
}

impl BuildIotCaCertificate {
    pub fn build(self, stack: &mut Stack) -> IotCaCertificate {
        let out = IotCaCertificate(Rc::new(IotCaCertificate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IotCaCertificateData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                active: self.active,
                allow_auto_registration: self.allow_auto_registration,
                ca_certificate_pem: self.ca_certificate_pem,
                certificate_mode: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                verification_certificate_pem: core::default::Default::default(),
                registration_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IotCaCertificateRef {
    shared: StackShared,
    base: String,
}

impl Ref for IotCaCertificateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl IotCaCertificateRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active` after provisioning.\n"]
    pub fn active(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.active", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allow_auto_registration` after provisioning.\n"]
    pub fn allow_auto_registration(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_auto_registration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ca_certificate_pem` after provisioning.\n"]
    pub fn ca_certificate_pem(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ca_certificate_pem", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_mode` after provisioning.\n"]
    pub fn certificate_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_version` after provisioning.\n"]
    pub fn customer_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `generation_id` after provisioning.\n"]
    pub fn generation_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.generation_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validity` after provisioning.\n"]
    pub fn validity(&self) -> ListRef<IotCaCertificateValidityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.validity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `verification_certificate_pem` after provisioning.\n"]
    pub fn verification_certificate_pem(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.verification_certificate_pem", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registration_config` after provisioning.\n"]
    pub fn registration_config(&self) -> ListRef<IotCaCertificateRegistrationConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.registration_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct IotCaCertificateValidityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    not_after: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not_before: Option<PrimField<String>>,
}

impl IotCaCertificateValidityEl {
    #[doc = "Set the field `not_after`.\n"]
    pub fn set_not_after(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.not_after = Some(v.into());
        self
    }

    #[doc = "Set the field `not_before`.\n"]
    pub fn set_not_before(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.not_before = Some(v.into());
        self
    }
}

impl ToListMappable for IotCaCertificateValidityEl {
    type O = BlockAssignable<IotCaCertificateValidityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotCaCertificateValidityEl {}

impl BuildIotCaCertificateValidityEl {
    pub fn build(self) -> IotCaCertificateValidityEl {
        IotCaCertificateValidityEl {
            not_after: core::default::Default::default(),
            not_before: core::default::Default::default(),
        }
    }
}

pub struct IotCaCertificateValidityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IotCaCertificateValidityElRef {
    fn new(shared: StackShared, base: String) -> IotCaCertificateValidityElRef {
        IotCaCertificateValidityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotCaCertificateValidityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `not_after` after provisioning.\n"]
    pub fn not_after(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.not_after", self.base))
    }

    #[doc = "Get a reference to the value of field `not_before` after provisioning.\n"]
    pub fn not_before(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.not_before", self.base))
    }
}

#[derive(Serialize)]
pub struct IotCaCertificateRegistrationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_body: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_name: Option<PrimField<String>>,
}

impl IotCaCertificateRegistrationConfigEl {
    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `template_body`.\n"]
    pub fn set_template_body(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.template_body = Some(v.into());
        self
    }

    #[doc = "Set the field `template_name`.\n"]
    pub fn set_template_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.template_name = Some(v.into());
        self
    }
}

impl ToListMappable for IotCaCertificateRegistrationConfigEl {
    type O = BlockAssignable<IotCaCertificateRegistrationConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotCaCertificateRegistrationConfigEl {}

impl BuildIotCaCertificateRegistrationConfigEl {
    pub fn build(self) -> IotCaCertificateRegistrationConfigEl {
        IotCaCertificateRegistrationConfigEl {
            role_arn: core::default::Default::default(),
            template_body: core::default::Default::default(),
            template_name: core::default::Default::default(),
        }
    }
}

pub struct IotCaCertificateRegistrationConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IotCaCertificateRegistrationConfigElRef {
    fn new(shared: StackShared, base: String) -> IotCaCertificateRegistrationConfigElRef {
        IotCaCertificateRegistrationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotCaCertificateRegistrationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `template_body` after provisioning.\n"]
    pub fn template_body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.template_body", self.base))
    }

    #[doc = "Get a reference to the value of field `template_name` after provisioning.\n"]
    pub fn template_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.template_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct IotCaCertificateDynamic {
    registration_config: Option<DynamicBlock<IotCaCertificateRegistrationConfigEl>>,
}
