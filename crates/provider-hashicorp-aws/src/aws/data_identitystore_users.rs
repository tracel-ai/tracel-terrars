use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataIdentitystoreUsersData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    identity_store_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataIdentitystoreUsers_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataIdentitystoreUsersData>,
}
#[derive(Clone)]
pub struct DataIdentitystoreUsers(Rc<DataIdentitystoreUsers_>);
impl DataIdentitystoreUsers {
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `identity_store_id` after provisioning.\n"]
    pub fn identity_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_store_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `users` after provisioning.\n"]
    pub fn users(&self) -> ListRef<DataIdentitystoreUsersUsersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.users", self.extract_ref()),
        )
    }
}
impl Referable for DataIdentitystoreUsers {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataIdentitystoreUsers {}
impl ToListMappable for DataIdentitystoreUsers {
    type O = ListRef<DataIdentitystoreUsersRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataIdentitystoreUsers_ {
    fn extract_datasource_type(&self) -> String {
        "aws_identitystore_users".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataIdentitystoreUsers {
    pub tf_id: String,
    #[doc = ""]
    pub identity_store_id: PrimField<String>,
}
impl BuildDataIdentitystoreUsers {
    pub fn build(self, stack: &mut Stack) -> DataIdentitystoreUsers {
        let out = DataIdentitystoreUsers(Rc::new(DataIdentitystoreUsers_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataIdentitystoreUsersData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                identity_store_id: self.identity_store_id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataIdentitystoreUsersRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIdentitystoreUsersRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataIdentitystoreUsersRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `identity_store_id` after provisioning.\n"]
    pub fn identity_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_store_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `users` after provisioning.\n"]
    pub fn users(&self) -> ListRef<DataIdentitystoreUsersUsersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.users", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataIdentitystoreUsersUsersElAddressesEl {
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
impl DataIdentitystoreUsersUsersElAddressesEl {
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
impl ToListMappable for DataIdentitystoreUsersUsersElAddressesEl {
    type O = BlockAssignable<DataIdentitystoreUsersUsersElAddressesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIdentitystoreUsersUsersElAddressesEl {}
impl BuildDataIdentitystoreUsersUsersElAddressesEl {
    pub fn build(self) -> DataIdentitystoreUsersUsersElAddressesEl {
        DataIdentitystoreUsersUsersElAddressesEl {
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
pub struct DataIdentitystoreUsersUsersElAddressesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIdentitystoreUsersUsersElAddressesElRef {
    fn new(shared: StackShared, base: String) -> DataIdentitystoreUsersUsersElAddressesElRef {
        DataIdentitystoreUsersUsersElAddressesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIdentitystoreUsersUsersElAddressesElRef {
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
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.street_address", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataIdentitystoreUsersUsersElEmailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    primary: Option<PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataIdentitystoreUsersUsersElEmailsEl {
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
impl ToListMappable for DataIdentitystoreUsersUsersElEmailsEl {
    type O = BlockAssignable<DataIdentitystoreUsersUsersElEmailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIdentitystoreUsersUsersElEmailsEl {}
impl BuildDataIdentitystoreUsersUsersElEmailsEl {
    pub fn build(self) -> DataIdentitystoreUsersUsersElEmailsEl {
        DataIdentitystoreUsersUsersElEmailsEl {
            primary: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataIdentitystoreUsersUsersElEmailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIdentitystoreUsersUsersElEmailsElRef {
    fn new(shared: StackShared, base: String) -> DataIdentitystoreUsersUsersElEmailsElRef {
        DataIdentitystoreUsersUsersElEmailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIdentitystoreUsersUsersElEmailsElRef {
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
pub struct DataIdentitystoreUsersUsersElExternalIdsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<PrimField<String>>,
}
impl DataIdentitystoreUsersUsersElExternalIdsEl {
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
impl ToListMappable for DataIdentitystoreUsersUsersElExternalIdsEl {
    type O = BlockAssignable<DataIdentitystoreUsersUsersElExternalIdsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIdentitystoreUsersUsersElExternalIdsEl {}
impl BuildDataIdentitystoreUsersUsersElExternalIdsEl {
    pub fn build(self) -> DataIdentitystoreUsersUsersElExternalIdsEl {
        DataIdentitystoreUsersUsersElExternalIdsEl {
            id: core::default::Default::default(),
            issuer: core::default::Default::default(),
        }
    }
}
pub struct DataIdentitystoreUsersUsersElExternalIdsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIdentitystoreUsersUsersElExternalIdsElRef {
    fn new(shared: StackShared, base: String) -> DataIdentitystoreUsersUsersElExternalIdsElRef {
        DataIdentitystoreUsersUsersElExternalIdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIdentitystoreUsersUsersElExternalIdsElRef {
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
pub struct DataIdentitystoreUsersUsersElNameEl {
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
impl DataIdentitystoreUsersUsersElNameEl {
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
impl ToListMappable for DataIdentitystoreUsersUsersElNameEl {
    type O = BlockAssignable<DataIdentitystoreUsersUsersElNameEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIdentitystoreUsersUsersElNameEl {}
impl BuildDataIdentitystoreUsersUsersElNameEl {
    pub fn build(self) -> DataIdentitystoreUsersUsersElNameEl {
        DataIdentitystoreUsersUsersElNameEl {
            family_name: core::default::Default::default(),
            formatted: core::default::Default::default(),
            given_name: core::default::Default::default(),
            honorific_prefix: core::default::Default::default(),
            honorific_suffix: core::default::Default::default(),
            middle_name: core::default::Default::default(),
        }
    }
}
pub struct DataIdentitystoreUsersUsersElNameElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIdentitystoreUsersUsersElNameElRef {
    fn new(shared: StackShared, base: String) -> DataIdentitystoreUsersUsersElNameElRef {
        DataIdentitystoreUsersUsersElNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIdentitystoreUsersUsersElNameElRef {
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
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.honorific_prefix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `honorific_suffix` after provisioning.\n"]
    pub fn honorific_suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.honorific_suffix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `middle_name` after provisioning.\n"]
    pub fn middle_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.middle_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataIdentitystoreUsersUsersElPhoneNumbersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    primary: Option<PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataIdentitystoreUsersUsersElPhoneNumbersEl {
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
impl ToListMappable for DataIdentitystoreUsersUsersElPhoneNumbersEl {
    type O = BlockAssignable<DataIdentitystoreUsersUsersElPhoneNumbersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIdentitystoreUsersUsersElPhoneNumbersEl {}
impl BuildDataIdentitystoreUsersUsersElPhoneNumbersEl {
    pub fn build(self) -> DataIdentitystoreUsersUsersElPhoneNumbersEl {
        DataIdentitystoreUsersUsersElPhoneNumbersEl {
            primary: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataIdentitystoreUsersUsersElPhoneNumbersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIdentitystoreUsersUsersElPhoneNumbersElRef {
    fn new(shared: StackShared, base: String) -> DataIdentitystoreUsersUsersElPhoneNumbersElRef {
        DataIdentitystoreUsersUsersElPhoneNumbersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIdentitystoreUsersUsersElPhoneNumbersElRef {
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
pub struct DataIdentitystoreUsersUsersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    addresses: Option<ListField<DataIdentitystoreUsersUsersElAddressesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emails: Option<ListField<DataIdentitystoreUsersUsersElEmailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_ids: Option<ListField<DataIdentitystoreUsersUsersElExternalIdsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_store_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locale: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<ListField<DataIdentitystoreUsersUsersElNameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_numbers: Option<ListField<DataIdentitystoreUsersUsersElPhoneNumbersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_language: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timezone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_type: Option<PrimField<String>>,
}
impl DataIdentitystoreUsersUsersEl {
    #[doc = "Set the field `addresses`.\n"]
    pub fn set_addresses(
        mut self,
        v: impl Into<ListField<DataIdentitystoreUsersUsersElAddressesEl>>,
    ) -> Self {
        self.addresses = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `emails`.\n"]
    pub fn set_emails(
        mut self,
        v: impl Into<ListField<DataIdentitystoreUsersUsersElEmailsEl>>,
    ) -> Self {
        self.emails = Some(v.into());
        self
    }
    #[doc = "Set the field `external_ids`.\n"]
    pub fn set_external_ids(
        mut self,
        v: impl Into<ListField<DataIdentitystoreUsersUsersElExternalIdsEl>>,
    ) -> Self {
        self.external_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `identity_store_id`.\n"]
    pub fn set_identity_store_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_store_id = Some(v.into());
        self
    }
    #[doc = "Set the field `locale`.\n"]
    pub fn set_locale(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.locale = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(
        mut self,
        v: impl Into<ListField<DataIdentitystoreUsersUsersElNameEl>>,
    ) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `nickname`.\n"]
    pub fn set_nickname(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nickname = Some(v.into());
        self
    }
    #[doc = "Set the field `phone_numbers`.\n"]
    pub fn set_phone_numbers(
        mut self,
        v: impl Into<ListField<DataIdentitystoreUsersUsersElPhoneNumbersEl>>,
    ) -> Self {
        self.phone_numbers = Some(v.into());
        self
    }
    #[doc = "Set the field `preferred_language`.\n"]
    pub fn set_preferred_language(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.preferred_language = Some(v.into());
        self
    }
    #[doc = "Set the field `profile_url`.\n"]
    pub fn set_profile_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.profile_url = Some(v.into());
        self
    }
    #[doc = "Set the field `timezone`.\n"]
    pub fn set_timezone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timezone = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\n"]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
    #[doc = "Set the field `user_id`.\n"]
    pub fn set_user_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_id = Some(v.into());
        self
    }
    #[doc = "Set the field `user_name`.\n"]
    pub fn set_user_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_name = Some(v.into());
        self
    }
    #[doc = "Set the field `user_type`.\n"]
    pub fn set_user_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_type = Some(v.into());
        self
    }
}
impl ToListMappable for DataIdentitystoreUsersUsersEl {
    type O = BlockAssignable<DataIdentitystoreUsersUsersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIdentitystoreUsersUsersEl {}
impl BuildDataIdentitystoreUsersUsersEl {
    pub fn build(self) -> DataIdentitystoreUsersUsersEl {
        DataIdentitystoreUsersUsersEl {
            addresses: core::default::Default::default(),
            display_name: core::default::Default::default(),
            emails: core::default::Default::default(),
            external_ids: core::default::Default::default(),
            identity_store_id: core::default::Default::default(),
            locale: core::default::Default::default(),
            name: core::default::Default::default(),
            nickname: core::default::Default::default(),
            phone_numbers: core::default::Default::default(),
            preferred_language: core::default::Default::default(),
            profile_url: core::default::Default::default(),
            timezone: core::default::Default::default(),
            title: core::default::Default::default(),
            user_id: core::default::Default::default(),
            user_name: core::default::Default::default(),
            user_type: core::default::Default::default(),
        }
    }
}
pub struct DataIdentitystoreUsersUsersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIdentitystoreUsersUsersElRef {
    fn new(shared: StackShared, base: String) -> DataIdentitystoreUsersUsersElRef {
        DataIdentitystoreUsersUsersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIdentitystoreUsersUsersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `addresses` after provisioning.\n"]
    pub fn addresses(&self) -> ListRef<DataIdentitystoreUsersUsersElAddressesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.addresses", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `emails` after provisioning.\n"]
    pub fn emails(&self) -> ListRef<DataIdentitystoreUsersUsersElEmailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.emails", self.base))
    }
    #[doc = "Get a reference to the value of field `external_ids` after provisioning.\n"]
    pub fn external_ids(&self) -> ListRef<DataIdentitystoreUsersUsersElExternalIdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.external_ids", self.base))
    }
    #[doc = "Get a reference to the value of field `identity_store_id` after provisioning.\n"]
    pub fn identity_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_store_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `locale` after provisioning.\n"]
    pub fn locale(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.locale", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> ListRef<DataIdentitystoreUsersUsersElNameElRef> {
        ListRef::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `nickname` after provisioning.\n"]
    pub fn nickname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nickname", self.base))
    }
    #[doc = "Get a reference to the value of field `phone_numbers` after provisioning.\n"]
    pub fn phone_numbers(&self) -> ListRef<DataIdentitystoreUsersUsersElPhoneNumbersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.phone_numbers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `preferred_language` after provisioning.\n"]
    pub fn preferred_language(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preferred_language", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `profile_url` after provisioning.\n"]
    pub fn profile_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.profile_url", self.base))
    }
    #[doc = "Get a reference to the value of field `timezone` after provisioning.\n"]
    pub fn timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
    #[doc = "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }
    #[doc = "Get a reference to the value of field `user_name` after provisioning.\n"]
    pub fn user_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_name", self.base))
    }
    #[doc = "Get a reference to the value of field `user_type` after provisioning.\n"]
    pub fn user_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_type", self.base))
    }
}
