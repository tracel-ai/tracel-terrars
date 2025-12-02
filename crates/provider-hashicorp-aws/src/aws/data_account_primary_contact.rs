use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataAccountPrimaryContactData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
}
struct DataAccountPrimaryContact_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAccountPrimaryContactData>,
}
#[derive(Clone)]
pub struct DataAccountPrimaryContact(Rc<DataAccountPrimaryContact_>);
impl DataAccountPrimaryContact {
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
    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().account_id = Some(v.into());
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
impl Referable for DataAccountPrimaryContact {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataAccountPrimaryContact {}
impl ToListMappable for DataAccountPrimaryContact {
    type O = ListRef<DataAccountPrimaryContactRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataAccountPrimaryContact_ {
    fn extract_datasource_type(&self) -> String {
        "aws_account_primary_contact".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataAccountPrimaryContact {
    pub tf_id: String,
}
impl BuildDataAccountPrimaryContact {
    pub fn build(self, stack: &mut Stack) -> DataAccountPrimaryContact {
        let out = DataAccountPrimaryContact(Rc::new(DataAccountPrimaryContact_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAccountPrimaryContactData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                account_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataAccountPrimaryContactRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAccountPrimaryContactRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataAccountPrimaryContactRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
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
