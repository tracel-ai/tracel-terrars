use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct Sesv2EmailIdentityData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration_set_name: Option<PrimField<String>>,
    email_identity: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dkim_signing_attributes: Option<Vec<Sesv2EmailIdentityDkimSigningAttributesEl>>,
    dynamic: Sesv2EmailIdentityDynamic,
}

struct Sesv2EmailIdentity_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Sesv2EmailIdentityData>,
}

#[derive(Clone)]
pub struct Sesv2EmailIdentity(Rc<Sesv2EmailIdentity_>);

impl Sesv2EmailIdentity {
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

    #[doc = "Set the field `configuration_set_name`.\n"]
    pub fn set_configuration_set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().configuration_set_name = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
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

    #[doc = "Set the field `dkim_signing_attributes`.\n"]
    pub fn set_dkim_signing_attributes(
        self,
        v: impl Into<BlockAssignable<Sesv2EmailIdentityDkimSigningAttributesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dkim_signing_attributes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dkim_signing_attributes = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_set_name` after provisioning.\n"]
    pub fn configuration_set_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.configuration_set_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `email_identity` after provisioning.\n"]
    pub fn email_identity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.email_identity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_type` after provisioning.\n"]
    pub fn identity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `verification_status` after provisioning.\n"]
    pub fn verification_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.verification_status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `verified_for_sending_status` after provisioning.\n"]
    pub fn verified_for_sending_status(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.verified_for_sending_status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dkim_signing_attributes` after provisioning.\n"]
    pub fn dkim_signing_attributes(&self) -> ListRef<Sesv2EmailIdentityDkimSigningAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dkim_signing_attributes", self.extract_ref()),
        )
    }
}

impl Referable for Sesv2EmailIdentity {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for Sesv2EmailIdentity {}

impl ToListMappable for Sesv2EmailIdentity {
    type O = ListRef<Sesv2EmailIdentityRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Sesv2EmailIdentity_ {
    fn extract_resource_type(&self) -> String {
        "aws_sesv2_email_identity".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSesv2EmailIdentity {
    pub tf_id: String,
    #[doc = ""]
    pub email_identity: PrimField<String>,
}

impl BuildSesv2EmailIdentity {
    pub fn build(self, stack: &mut Stack) -> Sesv2EmailIdentity {
        let out = Sesv2EmailIdentity(Rc::new(Sesv2EmailIdentity_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Sesv2EmailIdentityData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                configuration_set_name: core::default::Default::default(),
                email_identity: self.email_identity,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                dkim_signing_attributes: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Sesv2EmailIdentityRef {
    shared: StackShared,
    base: String,
}

impl Ref for Sesv2EmailIdentityRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl Sesv2EmailIdentityRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_set_name` after provisioning.\n"]
    pub fn configuration_set_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.configuration_set_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `email_identity` after provisioning.\n"]
    pub fn email_identity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.email_identity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_type` after provisioning.\n"]
    pub fn identity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `verification_status` after provisioning.\n"]
    pub fn verification_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.verification_status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `verified_for_sending_status` after provisioning.\n"]
    pub fn verified_for_sending_status(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.verified_for_sending_status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dkim_signing_attributes` after provisioning.\n"]
    pub fn dkim_signing_attributes(&self) -> ListRef<Sesv2EmailIdentityDkimSigningAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dkim_signing_attributes", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct Sesv2EmailIdentityDkimSigningAttributesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_signing_private_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_signing_selector: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_signing_key_length: Option<PrimField<String>>,
}

impl Sesv2EmailIdentityDkimSigningAttributesEl {
    #[doc = "Set the field `domain_signing_private_key`.\n"]
    pub fn set_domain_signing_private_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain_signing_private_key = Some(v.into());
        self
    }

    #[doc = "Set the field `domain_signing_selector`.\n"]
    pub fn set_domain_signing_selector(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain_signing_selector = Some(v.into());
        self
    }

    #[doc = "Set the field `next_signing_key_length`.\n"]
    pub fn set_next_signing_key_length(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_signing_key_length = Some(v.into());
        self
    }
}

impl ToListMappable for Sesv2EmailIdentityDkimSigningAttributesEl {
    type O = BlockAssignable<Sesv2EmailIdentityDkimSigningAttributesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2EmailIdentityDkimSigningAttributesEl {}

impl BuildSesv2EmailIdentityDkimSigningAttributesEl {
    pub fn build(self) -> Sesv2EmailIdentityDkimSigningAttributesEl {
        Sesv2EmailIdentityDkimSigningAttributesEl {
            domain_signing_private_key: core::default::Default::default(),
            domain_signing_selector: core::default::Default::default(),
            next_signing_key_length: core::default::Default::default(),
        }
    }
}

pub struct Sesv2EmailIdentityDkimSigningAttributesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Sesv2EmailIdentityDkimSigningAttributesElRef {
    fn new(shared: StackShared, base: String) -> Sesv2EmailIdentityDkimSigningAttributesElRef {
        Sesv2EmailIdentityDkimSigningAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2EmailIdentityDkimSigningAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `current_signing_key_length` after provisioning.\n"]
    pub fn current_signing_key_length(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.current_signing_key_length", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `domain_signing_private_key` after provisioning.\n"]
    pub fn domain_signing_private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_signing_private_key", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `domain_signing_selector` after provisioning.\n"]
    pub fn domain_signing_selector(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_signing_selector", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `last_key_generation_timestamp` after provisioning.\n"]
    pub fn last_key_generation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_key_generation_timestamp", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `next_signing_key_length` after provisioning.\n"]
    pub fn next_signing_key_length(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_signing_key_length", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `signing_attributes_origin` after provisioning.\n"]
    pub fn signing_attributes_origin(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.signing_attributes_origin", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `tokens` after provisioning.\n"]
    pub fn tokens(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tokens", self.base))
    }
}

#[derive(Serialize, Default)]
struct Sesv2EmailIdentityDynamic {
    dkim_signing_attributes: Option<DynamicBlock<Sesv2EmailIdentityDkimSigningAttributesEl>>,
}
