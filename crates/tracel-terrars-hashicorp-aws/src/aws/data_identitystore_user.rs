use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataIdentitystoreUserData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    identity_store_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alternate_identifier: Option<Vec<DataIdentitystoreUserAlternateIdentifierEl>>,
    dynamic: DataIdentitystoreUserDynamic,
}

struct DataIdentitystoreUser_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataIdentitystoreUserData>,
}

#[derive(Clone)]
pub struct DataIdentitystoreUser(Rc<DataIdentitystoreUser_>);

impl DataIdentitystoreUser {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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

    #[doc = "Set the field `user_id`.\n"]
    pub fn set_user_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().user_id = Some(v.into());
        self
    }

    #[doc = "Set the field `alternate_identifier`.\n"]
    pub fn set_alternate_identifier(
        self,
        v: impl Into<BlockAssignable<DataIdentitystoreUserAlternateIdentifierEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().alternate_identifier = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.alternate_identifier = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `addresses` after provisioning.\n"]
    pub fn addresses(&self) -> ListRef<DataIdentitystoreUserAddressesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `emails` after provisioning.\n"]
    pub fn emails(&self) -> ListRef<DataIdentitystoreUserEmailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.emails", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_ids` after provisioning.\n"]
    pub fn external_ids(&self) -> ListRef<DataIdentitystoreUserExternalIdsElRef> {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> ListRef<DataIdentitystoreUserNameElRef> {
        ListRef::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nickname` after provisioning.\n"]
    pub fn nickname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nickname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `phone_numbers` after provisioning.\n"]
    pub fn phone_numbers(&self) -> ListRef<DataIdentitystoreUserPhoneNumbersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.phone_numbers", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `alternate_identifier` after provisioning.\n"]
    pub fn alternate_identifier(&self) -> ListRef<DataIdentitystoreUserAlternateIdentifierElRef> {
        ListRef::new(self.shared().clone(), format!("{}.alternate_identifier", self.extract_ref()))
    }
}

impl Referable for DataIdentitystoreUser {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataIdentitystoreUser { }

impl ToListMappable for DataIdentitystoreUser {
    type O = ListRef<DataIdentitystoreUserRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataIdentitystoreUser_ {
    fn extract_datasource_type(&self) -> String {
        "aws_identitystore_user".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataIdentitystoreUser {
    pub tf_id: String,
    #[doc = ""]
    pub identity_store_id: PrimField<String>,
}

impl BuildDataIdentitystoreUser {
    pub fn build(self, stack: &mut Stack) -> DataIdentitystoreUser {
        let out = DataIdentitystoreUser(Rc::new(DataIdentitystoreUser_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataIdentitystoreUserData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                identity_store_id: self.identity_store_id,
                region: core::default::Default::default(),
                user_id: core::default::Default::default(),
                alternate_identifier: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataIdentitystoreUserRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataIdentitystoreUserRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataIdentitystoreUserRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `addresses` after provisioning.\n"]
    pub fn addresses(&self) -> ListRef<DataIdentitystoreUserAddressesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `emails` after provisioning.\n"]
    pub fn emails(&self) -> ListRef<DataIdentitystoreUserEmailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.emails", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_ids` after provisioning.\n"]
    pub fn external_ids(&self) -> ListRef<DataIdentitystoreUserExternalIdsElRef> {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> ListRef<DataIdentitystoreUserNameElRef> {
        ListRef::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nickname` after provisioning.\n"]
    pub fn nickname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nickname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `phone_numbers` after provisioning.\n"]
    pub fn phone_numbers(&self) -> ListRef<DataIdentitystoreUserPhoneNumbersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.phone_numbers", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `alternate_identifier` after provisioning.\n"]
    pub fn alternate_identifier(&self) -> ListRef<DataIdentitystoreUserAlternateIdentifierElRef> {
        ListRef::new(self.shared().clone(), format!("{}.alternate_identifier", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataIdentitystoreUserAddressesEl {
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

impl DataIdentitystoreUserAddressesEl {
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

impl ToListMappable for DataIdentitystoreUserAddressesEl {
    type O = BlockAssignable<DataIdentitystoreUserAddressesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreUserAddressesEl {}

impl BuildDataIdentitystoreUserAddressesEl {
    pub fn build(self) -> DataIdentitystoreUserAddressesEl {
        DataIdentitystoreUserAddressesEl {
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

pub struct DataIdentitystoreUserAddressesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataIdentitystoreUserAddressesElRef {
    fn new(shared: StackShared, base: String) -> DataIdentitystoreUserAddressesElRef {
        DataIdentitystoreUserAddressesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreUserAddressesElRef {
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
pub struct DataIdentitystoreUserEmailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    primary: Option<PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataIdentitystoreUserEmailsEl {
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

impl ToListMappable for DataIdentitystoreUserEmailsEl {
    type O = BlockAssignable<DataIdentitystoreUserEmailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreUserEmailsEl {}

impl BuildDataIdentitystoreUserEmailsEl {
    pub fn build(self) -> DataIdentitystoreUserEmailsEl {
        DataIdentitystoreUserEmailsEl {
            primary: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataIdentitystoreUserEmailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataIdentitystoreUserEmailsElRef {
    fn new(shared: StackShared, base: String) -> DataIdentitystoreUserEmailsElRef {
        DataIdentitystoreUserEmailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreUserEmailsElRef {
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
pub struct DataIdentitystoreUserExternalIdsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<PrimField<String>>,
}

impl DataIdentitystoreUserExternalIdsEl {
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

impl ToListMappable for DataIdentitystoreUserExternalIdsEl {
    type O = BlockAssignable<DataIdentitystoreUserExternalIdsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreUserExternalIdsEl {}

impl BuildDataIdentitystoreUserExternalIdsEl {
    pub fn build(self) -> DataIdentitystoreUserExternalIdsEl {
        DataIdentitystoreUserExternalIdsEl {
            id: core::default::Default::default(),
            issuer: core::default::Default::default(),
        }
    }
}

pub struct DataIdentitystoreUserExternalIdsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataIdentitystoreUserExternalIdsElRef {
    fn new(shared: StackShared, base: String) -> DataIdentitystoreUserExternalIdsElRef {
        DataIdentitystoreUserExternalIdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreUserExternalIdsElRef {
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
pub struct DataIdentitystoreUserNameEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    family_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    formatted: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    given_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    honorific_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    honorific_suffix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    middle_name: Option<PrimField<String>>,
}

impl DataIdentitystoreUserNameEl {
    #[doc = "Set the field `family_name`.\n"]
    pub fn set_family_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.family_name = Some(v.into());
        self
    }

    #[doc = "Set the field `formatted`.\n"]
    pub fn set_formatted(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.formatted = Some(v.into());
        self
    }

    #[doc = "Set the field `given_name`.\n"]
    pub fn set_given_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.given_name = Some(v.into());
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

impl ToListMappable for DataIdentitystoreUserNameEl {
    type O = BlockAssignable<DataIdentitystoreUserNameEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreUserNameEl {}

impl BuildDataIdentitystoreUserNameEl {
    pub fn build(self) -> DataIdentitystoreUserNameEl {
        DataIdentitystoreUserNameEl {
            family_name: core::default::Default::default(),
            formatted: core::default::Default::default(),
            given_name: core::default::Default::default(),
            honorific_prefix: core::default::Default::default(),
            honorific_suffix: core::default::Default::default(),
            middle_name: core::default::Default::default(),
        }
    }
}

pub struct DataIdentitystoreUserNameElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataIdentitystoreUserNameElRef {
    fn new(shared: StackShared, base: String) -> DataIdentitystoreUserNameElRef {
        DataIdentitystoreUserNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreUserNameElRef {
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
pub struct DataIdentitystoreUserPhoneNumbersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    primary: Option<PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataIdentitystoreUserPhoneNumbersEl {
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

impl ToListMappable for DataIdentitystoreUserPhoneNumbersEl {
    type O = BlockAssignable<DataIdentitystoreUserPhoneNumbersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreUserPhoneNumbersEl {}

impl BuildDataIdentitystoreUserPhoneNumbersEl {
    pub fn build(self) -> DataIdentitystoreUserPhoneNumbersEl {
        DataIdentitystoreUserPhoneNumbersEl {
            primary: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataIdentitystoreUserPhoneNumbersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataIdentitystoreUserPhoneNumbersElRef {
    fn new(shared: StackShared, base: String) -> DataIdentitystoreUserPhoneNumbersElRef {
        DataIdentitystoreUserPhoneNumbersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreUserPhoneNumbersElRef {
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
pub struct DataIdentitystoreUserAlternateIdentifierElExternalIdEl {
    id: PrimField<String>,
    issuer: PrimField<String>,
}

impl DataIdentitystoreUserAlternateIdentifierElExternalIdEl { }

impl ToListMappable for DataIdentitystoreUserAlternateIdentifierElExternalIdEl {
    type O = BlockAssignable<DataIdentitystoreUserAlternateIdentifierElExternalIdEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreUserAlternateIdentifierElExternalIdEl {
    #[doc = ""]
    pub id: PrimField<String>,
    #[doc = ""]
    pub issuer: PrimField<String>,
}

impl BuildDataIdentitystoreUserAlternateIdentifierElExternalIdEl {
    pub fn build(self) -> DataIdentitystoreUserAlternateIdentifierElExternalIdEl {
        DataIdentitystoreUserAlternateIdentifierElExternalIdEl {
            id: self.id,
            issuer: self.issuer,
        }
    }
}

pub struct DataIdentitystoreUserAlternateIdentifierElExternalIdElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataIdentitystoreUserAlternateIdentifierElExternalIdElRef {
    fn new(shared: StackShared, base: String) -> DataIdentitystoreUserAlternateIdentifierElExternalIdElRef {
        DataIdentitystoreUserAlternateIdentifierElExternalIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreUserAlternateIdentifierElExternalIdElRef {
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
pub struct DataIdentitystoreUserAlternateIdentifierElUniqueAttributeEl {
    attribute_path: PrimField<String>,
    attribute_value: PrimField<String>,
}

impl DataIdentitystoreUserAlternateIdentifierElUniqueAttributeEl { }

impl ToListMappable for DataIdentitystoreUserAlternateIdentifierElUniqueAttributeEl {
    type O = BlockAssignable<DataIdentitystoreUserAlternateIdentifierElUniqueAttributeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreUserAlternateIdentifierElUniqueAttributeEl {
    #[doc = ""]
    pub attribute_path: PrimField<String>,
    #[doc = ""]
    pub attribute_value: PrimField<String>,
}

impl BuildDataIdentitystoreUserAlternateIdentifierElUniqueAttributeEl {
    pub fn build(self) -> DataIdentitystoreUserAlternateIdentifierElUniqueAttributeEl {
        DataIdentitystoreUserAlternateIdentifierElUniqueAttributeEl {
            attribute_path: self.attribute_path,
            attribute_value: self.attribute_value,
        }
    }
}

pub struct DataIdentitystoreUserAlternateIdentifierElUniqueAttributeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataIdentitystoreUserAlternateIdentifierElUniqueAttributeElRef {
    fn new(shared: StackShared, base: String) -> DataIdentitystoreUserAlternateIdentifierElUniqueAttributeElRef {
        DataIdentitystoreUserAlternateIdentifierElUniqueAttributeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreUserAlternateIdentifierElUniqueAttributeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute_path` after provisioning.\n"]
    pub fn attribute_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attribute_path", self.base))
    }

    #[doc = "Get a reference to the value of field `attribute_value` after provisioning.\n"]
    pub fn attribute_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attribute_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataIdentitystoreUserAlternateIdentifierElDynamic {
    external_id: Option<DynamicBlock<DataIdentitystoreUserAlternateIdentifierElExternalIdEl>>,
    unique_attribute: Option<DynamicBlock<DataIdentitystoreUserAlternateIdentifierElUniqueAttributeEl>>,
}

#[derive(Serialize)]
pub struct DataIdentitystoreUserAlternateIdentifierEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    external_id: Option<Vec<DataIdentitystoreUserAlternateIdentifierElExternalIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique_attribute: Option<Vec<DataIdentitystoreUserAlternateIdentifierElUniqueAttributeEl>>,
    dynamic: DataIdentitystoreUserAlternateIdentifierElDynamic,
}

impl DataIdentitystoreUserAlternateIdentifierEl {
    #[doc = "Set the field `external_id`.\n"]
    pub fn set_external_id(
        mut self,
        v: impl Into<BlockAssignable<DataIdentitystoreUserAlternateIdentifierElExternalIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.external_id = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.external_id = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `unique_attribute`.\n"]
    pub fn set_unique_attribute(
        mut self,
        v: impl Into<BlockAssignable<DataIdentitystoreUserAlternateIdentifierElUniqueAttributeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.unique_attribute = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.unique_attribute = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DataIdentitystoreUserAlternateIdentifierEl {
    type O = BlockAssignable<DataIdentitystoreUserAlternateIdentifierEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreUserAlternateIdentifierEl {}

impl BuildDataIdentitystoreUserAlternateIdentifierEl {
    pub fn build(self) -> DataIdentitystoreUserAlternateIdentifierEl {
        DataIdentitystoreUserAlternateIdentifierEl {
            external_id: core::default::Default::default(),
            unique_attribute: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DataIdentitystoreUserAlternateIdentifierElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataIdentitystoreUserAlternateIdentifierElRef {
    fn new(shared: StackShared, base: String) -> DataIdentitystoreUserAlternateIdentifierElRef {
        DataIdentitystoreUserAlternateIdentifierElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreUserAlternateIdentifierElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> ListRef<DataIdentitystoreUserAlternateIdentifierElExternalIdElRef> {
        ListRef::new(self.shared().clone(), format!("{}.external_id", self.base))
    }

    #[doc = "Get a reference to the value of field `unique_attribute` after provisioning.\n"]
    pub fn unique_attribute(&self) -> ListRef<DataIdentitystoreUserAlternateIdentifierElUniqueAttributeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.unique_attribute", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataIdentitystoreUserDynamic {
    alternate_identifier: Option<DynamicBlock<DataIdentitystoreUserAlternateIdentifierEl>>,
}
