use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct IdentitystoreUserData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    identity_store_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locale: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_language: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timezone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    user_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    addresses: Option<Vec<IdentitystoreUserAddressesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emails: Option<Vec<IdentitystoreUserEmailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Vec<IdentitystoreUserNameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_numbers: Option<Vec<IdentitystoreUserPhoneNumbersEl>>,
    dynamic: IdentitystoreUserDynamic,
}

struct IdentitystoreUser_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IdentitystoreUserData>,
}

#[derive(Clone)]
pub struct IdentitystoreUser(Rc<IdentitystoreUser_>);

impl IdentitystoreUser {
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

    #[doc = "Set the field `locale`.\n"]
    pub fn set_locale(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().locale = Some(v.into());
        self
    }

    #[doc = "Set the field `nickname`.\n"]
    pub fn set_nickname(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().nickname = Some(v.into());
        self
    }

    #[doc = "Set the field `preferred_language`.\n"]
    pub fn set_preferred_language(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().preferred_language = Some(v.into());
        self
    }

    #[doc = "Set the field `profile_url`.\n"]
    pub fn set_profile_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().profile_url = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `timezone`.\n"]
    pub fn set_timezone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().timezone = Some(v.into());
        self
    }

    #[doc = "Set the field `title`.\n"]
    pub fn set_title(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().title = Some(v.into());
        self
    }

    #[doc = "Set the field `user_type`.\n"]
    pub fn set_user_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().user_type = Some(v.into());
        self
    }

    #[doc = "Set the field `addresses`.\n"]
    pub fn set_addresses(self, v: impl Into<BlockAssignable<IdentitystoreUserAddressesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().addresses = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.addresses = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `emails`.\n"]
    pub fn set_emails(self, v: impl Into<BlockAssignable<IdentitystoreUserEmailsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().emails = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.emails = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<BlockAssignable<IdentitystoreUserNameEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().name = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.name = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `phone_numbers`.\n"]
    pub fn set_phone_numbers(self, v: impl Into<BlockAssignable<IdentitystoreUserPhoneNumbersEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().phone_numbers = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.phone_numbers = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_ids` after provisioning.\n"]
    pub fn external_ids(&self) -> ListRef<IdentitystoreUserExternalIdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.external_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_store_id` after provisioning.\n"]
    pub fn identity_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_store_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locale` after provisioning.\n"]
    pub fn locale(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.locale", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nickname` after provisioning.\n"]
    pub fn nickname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nickname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_language` after provisioning.\n"]
    pub fn preferred_language(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preferred_language", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profile_url` after provisioning.\n"]
    pub fn profile_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.profile_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\n"]
    pub fn timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_name` after provisioning.\n"]
    pub fn user_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_type` after provisioning.\n"]
    pub fn user_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `addresses` after provisioning.\n"]
    pub fn addresses(&self) -> ListRef<IdentitystoreUserAddressesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `emails` after provisioning.\n"]
    pub fn emails(&self) -> ListRef<IdentitystoreUserEmailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.emails", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> ListRef<IdentitystoreUserNameElRef> {
        ListRef::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `phone_numbers` after provisioning.\n"]
    pub fn phone_numbers(&self) -> ListRef<IdentitystoreUserPhoneNumbersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.phone_numbers", self.extract_ref()))
    }
}

impl Referable for IdentitystoreUser {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for IdentitystoreUser { }

impl ToListMappable for IdentitystoreUser {
    type O = ListRef<IdentitystoreUserRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for IdentitystoreUser_ {
    fn extract_resource_type(&self) -> String {
        "aws_identitystore_user".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIdentitystoreUser {
    pub tf_id: String,
    #[doc = ""]
    pub display_name: PrimField<String>,
    #[doc = ""]
    pub identity_store_id: PrimField<String>,
    #[doc = ""]
    pub user_name: PrimField<String>,
}

impl BuildIdentitystoreUser {
    pub fn build(self, stack: &mut Stack) -> IdentitystoreUser {
        let out = IdentitystoreUser(Rc::new(IdentitystoreUser_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IdentitystoreUserData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                display_name: self.display_name,
                id: core::default::Default::default(),
                identity_store_id: self.identity_store_id,
                locale: core::default::Default::default(),
                nickname: core::default::Default::default(),
                preferred_language: core::default::Default::default(),
                profile_url: core::default::Default::default(),
                region: core::default::Default::default(),
                timezone: core::default::Default::default(),
                title: core::default::Default::default(),
                user_name: self.user_name,
                user_type: core::default::Default::default(),
                addresses: core::default::Default::default(),
                emails: core::default::Default::default(),
                name: core::default::Default::default(),
                phone_numbers: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IdentitystoreUserRef {
    shared: StackShared,
    base: String,
}

impl Ref for IdentitystoreUserRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl IdentitystoreUserRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_ids` after provisioning.\n"]
    pub fn external_ids(&self) -> ListRef<IdentitystoreUserExternalIdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.external_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_store_id` after provisioning.\n"]
    pub fn identity_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_store_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locale` after provisioning.\n"]
    pub fn locale(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.locale", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nickname` after provisioning.\n"]
    pub fn nickname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nickname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_language` after provisioning.\n"]
    pub fn preferred_language(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preferred_language", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profile_url` after provisioning.\n"]
    pub fn profile_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.profile_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\n"]
    pub fn timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_name` after provisioning.\n"]
    pub fn user_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_type` after provisioning.\n"]
    pub fn user_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `addresses` after provisioning.\n"]
    pub fn addresses(&self) -> ListRef<IdentitystoreUserAddressesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `emails` after provisioning.\n"]
    pub fn emails(&self) -> ListRef<IdentitystoreUserEmailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.emails", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> ListRef<IdentitystoreUserNameElRef> {
        ListRef::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `phone_numbers` after provisioning.\n"]
    pub fn phone_numbers(&self) -> ListRef<IdentitystoreUserPhoneNumbersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.phone_numbers", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct IdentitystoreUserExternalIdsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<PrimField<String>>,
}

impl IdentitystoreUserExternalIdsEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issuer = Some(v.into());
        self
    }
}

impl ToListMappable for IdentitystoreUserExternalIdsEl {
    type O = BlockAssignable<IdentitystoreUserExternalIdsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentitystoreUserExternalIdsEl {}

impl BuildIdentitystoreUserExternalIdsEl {
    pub fn build(self) -> IdentitystoreUserExternalIdsEl {
        IdentitystoreUserExternalIdsEl {
            id: core::default::Default::default(),
            issuer: core::default::Default::default(),
        }
    }
}

pub struct IdentitystoreUserExternalIdsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IdentitystoreUserExternalIdsElRef {
    fn new(shared: StackShared, base: String) -> IdentitystoreUserExternalIdsElRef {
        IdentitystoreUserExternalIdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentitystoreUserExternalIdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentitystoreUserAddressesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    formatted: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locality: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    postal_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    street_address: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl IdentitystoreUserAddressesEl {
    #[doc = "Set the field `country`.\n"]
    pub fn set_country(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.country = Some(v.into());
        self
    }

    #[doc = "Set the field `formatted`.\n"]
    pub fn set_formatted(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.formatted = Some(v.into());
        self
    }

    #[doc = "Set the field `locality`.\n"]
    pub fn set_locality(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.locality = Some(v.into());
        self
    }

    #[doc = "Set the field `postal_code`.\n"]
    pub fn set_postal_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.postal_code = Some(v.into());
        self
    }

    #[doc = "Set the field `primary`.\n"]
    pub fn set_primary(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.primary = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }

    #[doc = "Set the field `street_address`.\n"]
    pub fn set_street_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.street_address = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for IdentitystoreUserAddressesEl {
    type O = BlockAssignable<IdentitystoreUserAddressesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentitystoreUserAddressesEl {}

impl BuildIdentitystoreUserAddressesEl {
    pub fn build(self) -> IdentitystoreUserAddressesEl {
        IdentitystoreUserAddressesEl {
            country: core::default::Default::default(),
            formatted: core::default::Default::default(),
            locality: core::default::Default::default(),
            postal_code: core::default::Default::default(),
            primary: core::default::Default::default(),
            region: core::default::Default::default(),
            street_address: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct IdentitystoreUserAddressesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IdentitystoreUserAddressesElRef {
    fn new(shared: StackShared, base: String) -> IdentitystoreUserAddressesElRef {
        IdentitystoreUserAddressesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentitystoreUserAddressesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `country` after provisioning.\n"]
    pub fn country(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.country", self.base))
    }

    #[doc = "Get a reference to the value of field `formatted` after provisioning.\n"]
    pub fn formatted(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.formatted", self.base))
    }

    #[doc = "Get a reference to the value of field `locality` after provisioning.\n"]
    pub fn locality(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.locality", self.base))
    }

    #[doc = "Get a reference to the value of field `postal_code` after provisioning.\n"]
    pub fn postal_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.postal_code", self.base))
    }

    #[doc = "Get a reference to the value of field `primary` after provisioning.\n"]
    pub fn primary(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.primary", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `street_address` after provisioning.\n"]
    pub fn street_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.street_address", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentitystoreUserEmailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    primary: Option<PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl IdentitystoreUserEmailsEl {
    #[doc = "Set the field `primary`.\n"]
    pub fn set_primary(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.primary = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for IdentitystoreUserEmailsEl {
    type O = BlockAssignable<IdentitystoreUserEmailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentitystoreUserEmailsEl {}

impl BuildIdentitystoreUserEmailsEl {
    pub fn build(self) -> IdentitystoreUserEmailsEl {
        IdentitystoreUserEmailsEl {
            primary: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct IdentitystoreUserEmailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IdentitystoreUserEmailsElRef {
    fn new(shared: StackShared, base: String) -> IdentitystoreUserEmailsElRef {
        IdentitystoreUserEmailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentitystoreUserEmailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `primary` after provisioning.\n"]
    pub fn primary(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.primary", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentitystoreUserNameEl {
    family_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    formatted: Option<PrimField<String>>,
    given_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    honorific_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    honorific_suffix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    middle_name: Option<PrimField<String>>,
}

impl IdentitystoreUserNameEl {
    #[doc = "Set the field `formatted`.\n"]
    pub fn set_formatted(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.formatted = Some(v.into());
        self
    }

    #[doc = "Set the field `honorific_prefix`.\n"]
    pub fn set_honorific_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.honorific_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `honorific_suffix`.\n"]
    pub fn set_honorific_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.honorific_suffix = Some(v.into());
        self
    }

    #[doc = "Set the field `middle_name`.\n"]
    pub fn set_middle_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.middle_name = Some(v.into());
        self
    }
}

impl ToListMappable for IdentitystoreUserNameEl {
    type O = BlockAssignable<IdentitystoreUserNameEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentitystoreUserNameEl {
    #[doc = ""]
    pub family_name: PrimField<String>,
    #[doc = ""]
    pub given_name: PrimField<String>,
}

impl BuildIdentitystoreUserNameEl {
    pub fn build(self) -> IdentitystoreUserNameEl {
        IdentitystoreUserNameEl {
            family_name: self.family_name,
            formatted: core::default::Default::default(),
            given_name: self.given_name,
            honorific_prefix: core::default::Default::default(),
            honorific_suffix: core::default::Default::default(),
            middle_name: core::default::Default::default(),
        }
    }
}

pub struct IdentitystoreUserNameElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IdentitystoreUserNameElRef {
    fn new(shared: StackShared, base: String) -> IdentitystoreUserNameElRef {
        IdentitystoreUserNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentitystoreUserNameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `family_name` after provisioning.\n"]
    pub fn family_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.family_name", self.base))
    }

    #[doc = "Get a reference to the value of field `formatted` after provisioning.\n"]
    pub fn formatted(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.formatted", self.base))
    }

    #[doc = "Get a reference to the value of field `given_name` after provisioning.\n"]
    pub fn given_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.given_name", self.base))
    }

    #[doc = "Get a reference to the value of field `honorific_prefix` after provisioning.\n"]
    pub fn honorific_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.honorific_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `honorific_suffix` after provisioning.\n"]
    pub fn honorific_suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.honorific_suffix", self.base))
    }

    #[doc = "Get a reference to the value of field `middle_name` after provisioning.\n"]
    pub fn middle_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.middle_name", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentitystoreUserPhoneNumbersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    primary: Option<PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl IdentitystoreUserPhoneNumbersEl {
    #[doc = "Set the field `primary`.\n"]
    pub fn set_primary(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.primary = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for IdentitystoreUserPhoneNumbersEl {
    type O = BlockAssignable<IdentitystoreUserPhoneNumbersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentitystoreUserPhoneNumbersEl {}

impl BuildIdentitystoreUserPhoneNumbersEl {
    pub fn build(self) -> IdentitystoreUserPhoneNumbersEl {
        IdentitystoreUserPhoneNumbersEl {
            primary: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct IdentitystoreUserPhoneNumbersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IdentitystoreUserPhoneNumbersElRef {
    fn new(shared: StackShared, base: String) -> IdentitystoreUserPhoneNumbersElRef {
        IdentitystoreUserPhoneNumbersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentitystoreUserPhoneNumbersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `primary` after provisioning.\n"]
    pub fn primary(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.primary", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct IdentitystoreUserDynamic {
    addresses: Option<DynamicBlock<IdentitystoreUserAddressesEl>>,
    emails: Option<DynamicBlock<IdentitystoreUserEmailsEl>>,
    name: Option<DynamicBlock<IdentitystoreUserNameEl>>,
    phone_numbers: Option<DynamicBlock<IdentitystoreUserPhoneNumbersEl>>,
}
