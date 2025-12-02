use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AccountPrimaryContactData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    address_line_1: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_line_2: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_line_3: Option<PrimField<String>>,
    city: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    company_name: Option<PrimField<String>>,
    country_code: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    district_or_county: Option<PrimField<String>>,
    full_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    phone_number: PrimField<String>,
    postal_code: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state_or_region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    website_url: Option<PrimField<String>>,
}
struct AccountPrimaryContact_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AccountPrimaryContactData>,
}
#[derive(Clone)]
pub struct AccountPrimaryContact(Rc<AccountPrimaryContact_>);
impl AccountPrimaryContact {
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
    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().account_id = Some(v.into());
        self
    }
    #[doc = "Set the field `address_line_2`.\n"]
    pub fn set_address_line_2(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().address_line_2 = Some(v.into());
        self
    }
    #[doc = "Set the field `address_line_3`.\n"]
    pub fn set_address_line_3(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().address_line_3 = Some(v.into());
        self
    }
    #[doc = "Set the field `company_name`.\n"]
    pub fn set_company_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().company_name = Some(v.into());
        self
    }
    #[doc = "Set the field `district_or_county`.\n"]
    pub fn set_district_or_county(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().district_or_county = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `state_or_region`.\n"]
    pub fn set_state_or_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().state_or_region = Some(v.into());
        self
    }
    #[doc = "Set the field `website_url`.\n"]
    pub fn set_website_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().website_url = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `address_line_1` after provisioning.\n"]
    pub fn address_line_1(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.address_line_1", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `address_line_2` after provisioning.\n"]
    pub fn address_line_2(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.address_line_2", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `address_line_3` after provisioning.\n"]
    pub fn address_line_3(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.address_line_3", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `city` after provisioning.\n"]
    pub fn city(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.city", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `company_name` after provisioning.\n"]
    pub fn company_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.company_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `country_code` after provisioning.\n"]
    pub fn country_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.country_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `district_or_county` after provisioning.\n"]
    pub fn district_or_county(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.district_or_county", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `full_name` after provisioning.\n"]
    pub fn full_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.full_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.phone_number", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `postal_code` after provisioning.\n"]
    pub fn postal_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.postal_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_or_region` after provisioning.\n"]
    pub fn state_or_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_or_region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `website_url` after provisioning.\n"]
    pub fn website_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.website_url", self.extract_ref()),
        )
    }
}
impl Referable for AccountPrimaryContact {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AccountPrimaryContact {}
impl ToListMappable for AccountPrimaryContact {
    type O = ListRef<AccountPrimaryContactRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AccountPrimaryContact_ {
    fn extract_resource_type(&self) -> String {
        "aws_account_primary_contact".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAccountPrimaryContact {
    pub tf_id: String,
    #[doc = ""]
    pub address_line_1: PrimField<String>,
    #[doc = ""]
    pub city: PrimField<String>,
    #[doc = ""]
    pub country_code: PrimField<String>,
    #[doc = ""]
    pub full_name: PrimField<String>,
    #[doc = ""]
    pub phone_number: PrimField<String>,
    #[doc = ""]
    pub postal_code: PrimField<String>,
}
impl BuildAccountPrimaryContact {
    pub fn build(self, stack: &mut Stack) -> AccountPrimaryContact {
        let out = AccountPrimaryContact(Rc::new(AccountPrimaryContact_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AccountPrimaryContactData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: core::default::Default::default(),
                address_line_1: self.address_line_1,
                address_line_2: core::default::Default::default(),
                address_line_3: core::default::Default::default(),
                city: self.city,
                company_name: core::default::Default::default(),
                country_code: self.country_code,
                district_or_county: core::default::Default::default(),
                full_name: self.full_name,
                id: core::default::Default::default(),
                phone_number: self.phone_number,
                postal_code: self.postal_code,
                state_or_region: core::default::Default::default(),
                website_url: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AccountPrimaryContactRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccountPrimaryContactRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AccountPrimaryContactRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `address_line_1` after provisioning.\n"]
    pub fn address_line_1(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.address_line_1", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `address_line_2` after provisioning.\n"]
    pub fn address_line_2(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.address_line_2", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `address_line_3` after provisioning.\n"]
    pub fn address_line_3(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.address_line_3", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `city` after provisioning.\n"]
    pub fn city(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.city", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `company_name` after provisioning.\n"]
    pub fn company_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.company_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `country_code` after provisioning.\n"]
    pub fn country_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.country_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `district_or_county` after provisioning.\n"]
    pub fn district_or_county(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.district_or_county", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `full_name` after provisioning.\n"]
    pub fn full_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.full_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.phone_number", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `postal_code` after provisioning.\n"]
    pub fn postal_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.postal_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_or_region` after provisioning.\n"]
    pub fn state_or_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_or_region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `website_url` after provisioning.\n"]
    pub fn website_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.website_url", self.extract_ref()),
        )
    }
}
