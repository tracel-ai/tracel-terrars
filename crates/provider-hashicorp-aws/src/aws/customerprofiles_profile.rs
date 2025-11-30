use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct CustomerprofilesProfileData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_number: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_information: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    birth_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_email_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_phone_number: Option<PrimField<String>>,
    domain_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gender_string: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    home_phone_number: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    middle_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mobile_phone_number: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    party_type_string: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    personal_email_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<Vec<CustomerprofilesProfileAddressEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_address: Option<Vec<CustomerprofilesProfileBillingAddressEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mailing_address: Option<Vec<CustomerprofilesProfileMailingAddressEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_address: Option<Vec<CustomerprofilesProfileShippingAddressEl>>,
    dynamic: CustomerprofilesProfileDynamic,
}

struct CustomerprofilesProfile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CustomerprofilesProfileData>,
}

#[derive(Clone)]
pub struct CustomerprofilesProfile(Rc<CustomerprofilesProfile_>);

impl CustomerprofilesProfile {
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

    #[doc = "Set the field `account_number`.\n"]
    pub fn set_account_number(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().account_number = Some(v.into());
        self
    }

    #[doc = "Set the field `additional_information`.\n"]
    pub fn set_additional_information(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().additional_information = Some(v.into());
        self
    }

    #[doc = "Set the field `attributes`.\n"]
    pub fn set_attributes(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().attributes = Some(v.into());
        self
    }

    #[doc = "Set the field `birth_date`.\n"]
    pub fn set_birth_date(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().birth_date = Some(v.into());
        self
    }

    #[doc = "Set the field `business_email_address`.\n"]
    pub fn set_business_email_address(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().business_email_address = Some(v.into());
        self
    }

    #[doc = "Set the field `business_name`.\n"]
    pub fn set_business_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().business_name = Some(v.into());
        self
    }

    #[doc = "Set the field `business_phone_number`.\n"]
    pub fn set_business_phone_number(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().business_phone_number = Some(v.into());
        self
    }

    #[doc = "Set the field `email_address`.\n"]
    pub fn set_email_address(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().email_address = Some(v.into());
        self
    }

    #[doc = "Set the field `first_name`.\n"]
    pub fn set_first_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().first_name = Some(v.into());
        self
    }

    #[doc = "Set the field `gender_string`.\n"]
    pub fn set_gender_string(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().gender_string = Some(v.into());
        self
    }

    #[doc = "Set the field `home_phone_number`.\n"]
    pub fn set_home_phone_number(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().home_phone_number = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `last_name`.\n"]
    pub fn set_last_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().last_name = Some(v.into());
        self
    }

    #[doc = "Set the field `middle_name`.\n"]
    pub fn set_middle_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().middle_name = Some(v.into());
        self
    }

    #[doc = "Set the field `mobile_phone_number`.\n"]
    pub fn set_mobile_phone_number(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().mobile_phone_number = Some(v.into());
        self
    }

    #[doc = "Set the field `party_type_string`.\n"]
    pub fn set_party_type_string(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().party_type_string = Some(v.into());
        self
    }

    #[doc = "Set the field `personal_email_address`.\n"]
    pub fn set_personal_email_address(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().personal_email_address = Some(v.into());
        self
    }

    #[doc = "Set the field `phone_number`.\n"]
    pub fn set_phone_number(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().phone_number = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `address`.\n"]
    pub fn set_address(
        self,
        v: impl Into<BlockAssignable<CustomerprofilesProfileAddressEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().address = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.address = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `billing_address`.\n"]
    pub fn set_billing_address(
        self,
        v: impl Into<BlockAssignable<CustomerprofilesProfileBillingAddressEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().billing_address = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.billing_address = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `mailing_address`.\n"]
    pub fn set_mailing_address(
        self,
        v: impl Into<BlockAssignable<CustomerprofilesProfileMailingAddressEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mailing_address = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mailing_address = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `shipping_address`.\n"]
    pub fn set_shipping_address(
        self,
        v: impl Into<BlockAssignable<CustomerprofilesProfileShippingAddressEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().shipping_address = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.shipping_address = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `account_number` after provisioning.\n"]
    pub fn account_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_number", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `additional_information` after provisioning.\n"]
    pub fn additional_information(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.additional_information", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.attributes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `birth_date` after provisioning.\n"]
    pub fn birth_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.birth_date", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `business_email_address` after provisioning.\n"]
    pub fn business_email_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_email_address", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `business_name` after provisioning.\n"]
    pub fn business_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `business_phone_number` after provisioning.\n"]
    pub fn business_phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_phone_number", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `email_address` after provisioning.\n"]
    pub fn email_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.email_address", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `first_name` after provisioning.\n"]
    pub fn first_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.first_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gender_string` after provisioning.\n"]
    pub fn gender_string(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gender_string", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `home_phone_number` after provisioning.\n"]
    pub fn home_phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.home_phone_number", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_name` after provisioning.\n"]
    pub fn last_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `middle_name` after provisioning.\n"]
    pub fn middle_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.middle_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `mobile_phone_number` after provisioning.\n"]
    pub fn mobile_phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mobile_phone_number", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `party_type_string` after provisioning.\n"]
    pub fn party_type_string(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.party_type_string", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `personal_email_address` after provisioning.\n"]
    pub fn personal_email_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.personal_email_address", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.phone_number", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> ListRef<CustomerprofilesProfileAddressElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.address", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `billing_address` after provisioning.\n"]
    pub fn billing_address(&self) -> ListRef<CustomerprofilesProfileBillingAddressElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.billing_address", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `mailing_address` after provisioning.\n"]
    pub fn mailing_address(&self) -> ListRef<CustomerprofilesProfileMailingAddressElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mailing_address", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `shipping_address` after provisioning.\n"]
    pub fn shipping_address(&self) -> ListRef<CustomerprofilesProfileShippingAddressElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.shipping_address", self.extract_ref()),
        )
    }
}

impl Referable for CustomerprofilesProfile {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for CustomerprofilesProfile {}

impl ToListMappable for CustomerprofilesProfile {
    type O = ListRef<CustomerprofilesProfileRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CustomerprofilesProfile_ {
    fn extract_resource_type(&self) -> String {
        "aws_customerprofiles_profile".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCustomerprofilesProfile {
    pub tf_id: String,
    #[doc = ""]
    pub domain_name: PrimField<String>,
}

impl BuildCustomerprofilesProfile {
    pub fn build(self, stack: &mut Stack) -> CustomerprofilesProfile {
        let out = CustomerprofilesProfile(Rc::new(CustomerprofilesProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CustomerprofilesProfileData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_number: core::default::Default::default(),
                additional_information: core::default::Default::default(),
                attributes: core::default::Default::default(),
                birth_date: core::default::Default::default(),
                business_email_address: core::default::Default::default(),
                business_name: core::default::Default::default(),
                business_phone_number: core::default::Default::default(),
                domain_name: self.domain_name,
                email_address: core::default::Default::default(),
                first_name: core::default::Default::default(),
                gender_string: core::default::Default::default(),
                home_phone_number: core::default::Default::default(),
                id: core::default::Default::default(),
                last_name: core::default::Default::default(),
                middle_name: core::default::Default::default(),
                mobile_phone_number: core::default::Default::default(),
                party_type_string: core::default::Default::default(),
                personal_email_address: core::default::Default::default(),
                phone_number: core::default::Default::default(),
                region: core::default::Default::default(),
                address: core::default::Default::default(),
                billing_address: core::default::Default::default(),
                mailing_address: core::default::Default::default(),
                shipping_address: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CustomerprofilesProfileRef {
    shared: StackShared,
    base: String,
}

impl Ref for CustomerprofilesProfileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl CustomerprofilesProfileRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_number` after provisioning.\n"]
    pub fn account_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_number", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `additional_information` after provisioning.\n"]
    pub fn additional_information(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.additional_information", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.attributes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `birth_date` after provisioning.\n"]
    pub fn birth_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.birth_date", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `business_email_address` after provisioning.\n"]
    pub fn business_email_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_email_address", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `business_name` after provisioning.\n"]
    pub fn business_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `business_phone_number` after provisioning.\n"]
    pub fn business_phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_phone_number", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `email_address` after provisioning.\n"]
    pub fn email_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.email_address", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `first_name` after provisioning.\n"]
    pub fn first_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.first_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gender_string` after provisioning.\n"]
    pub fn gender_string(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gender_string", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `home_phone_number` after provisioning.\n"]
    pub fn home_phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.home_phone_number", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_name` after provisioning.\n"]
    pub fn last_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `middle_name` after provisioning.\n"]
    pub fn middle_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.middle_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `mobile_phone_number` after provisioning.\n"]
    pub fn mobile_phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mobile_phone_number", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `party_type_string` after provisioning.\n"]
    pub fn party_type_string(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.party_type_string", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `personal_email_address` after provisioning.\n"]
    pub fn personal_email_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.personal_email_address", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.phone_number", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> ListRef<CustomerprofilesProfileAddressElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.address", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `billing_address` after provisioning.\n"]
    pub fn billing_address(&self) -> ListRef<CustomerprofilesProfileBillingAddressElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.billing_address", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `mailing_address` after provisioning.\n"]
    pub fn mailing_address(&self) -> ListRef<CustomerprofilesProfileMailingAddressElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mailing_address", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `shipping_address` after provisioning.\n"]
    pub fn shipping_address(&self) -> ListRef<CustomerprofilesProfileShippingAddressElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.shipping_address", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct CustomerprofilesProfileAddressEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address_1: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_2: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_3: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_4: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    city: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    county: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    postal_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    province: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}

impl CustomerprofilesProfileAddressEl {
    #[doc = "Set the field `address_1`.\n"]
    pub fn set_address_1(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address_1 = Some(v.into());
        self
    }

    #[doc = "Set the field `address_2`.\n"]
    pub fn set_address_2(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address_2 = Some(v.into());
        self
    }

    #[doc = "Set the field `address_3`.\n"]
    pub fn set_address_3(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address_3 = Some(v.into());
        self
    }

    #[doc = "Set the field `address_4`.\n"]
    pub fn set_address_4(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address_4 = Some(v.into());
        self
    }

    #[doc = "Set the field `city`.\n"]
    pub fn set_city(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.city = Some(v.into());
        self
    }

    #[doc = "Set the field `country`.\n"]
    pub fn set_country(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.country = Some(v.into());
        self
    }

    #[doc = "Set the field `county`.\n"]
    pub fn set_county(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.county = Some(v.into());
        self
    }

    #[doc = "Set the field `postal_code`.\n"]
    pub fn set_postal_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.postal_code = Some(v.into());
        self
    }

    #[doc = "Set the field `province`.\n"]
    pub fn set_province(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.province = Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}

impl ToListMappable for CustomerprofilesProfileAddressEl {
    type O = BlockAssignable<CustomerprofilesProfileAddressEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesProfileAddressEl {}

impl BuildCustomerprofilesProfileAddressEl {
    pub fn build(self) -> CustomerprofilesProfileAddressEl {
        CustomerprofilesProfileAddressEl {
            address_1: core::default::Default::default(),
            address_2: core::default::Default::default(),
            address_3: core::default::Default::default(),
            address_4: core::default::Default::default(),
            city: core::default::Default::default(),
            country: core::default::Default::default(),
            county: core::default::Default::default(),
            postal_code: core::default::Default::default(),
            province: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

pub struct CustomerprofilesProfileAddressElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CustomerprofilesProfileAddressElRef {
    fn new(shared: StackShared, base: String) -> CustomerprofilesProfileAddressElRef {
        CustomerprofilesProfileAddressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesProfileAddressElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_1` after provisioning.\n"]
    pub fn address_1(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_1", self.base))
    }

    #[doc = "Get a reference to the value of field `address_2` after provisioning.\n"]
    pub fn address_2(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_2", self.base))
    }

    #[doc = "Get a reference to the value of field `address_3` after provisioning.\n"]
    pub fn address_3(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_3", self.base))
    }

    #[doc = "Get a reference to the value of field `address_4` after provisioning.\n"]
    pub fn address_4(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_4", self.base))
    }

    #[doc = "Get a reference to the value of field `city` after provisioning.\n"]
    pub fn city(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.city", self.base))
    }

    #[doc = "Get a reference to the value of field `country` after provisioning.\n"]
    pub fn country(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.country", self.base))
    }

    #[doc = "Get a reference to the value of field `county` after provisioning.\n"]
    pub fn county(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.county", self.base))
    }

    #[doc = "Get a reference to the value of field `postal_code` after provisioning.\n"]
    pub fn postal_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.postal_code", self.base))
    }

    #[doc = "Get a reference to the value of field `province` after provisioning.\n"]
    pub fn province(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.province", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomerprofilesProfileBillingAddressEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address_1: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_2: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_3: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_4: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    city: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    county: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    postal_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    province: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}

impl CustomerprofilesProfileBillingAddressEl {
    #[doc = "Set the field `address_1`.\n"]
    pub fn set_address_1(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address_1 = Some(v.into());
        self
    }

    #[doc = "Set the field `address_2`.\n"]
    pub fn set_address_2(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address_2 = Some(v.into());
        self
    }

    #[doc = "Set the field `address_3`.\n"]
    pub fn set_address_3(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address_3 = Some(v.into());
        self
    }

    #[doc = "Set the field `address_4`.\n"]
    pub fn set_address_4(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address_4 = Some(v.into());
        self
    }

    #[doc = "Set the field `city`.\n"]
    pub fn set_city(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.city = Some(v.into());
        self
    }

    #[doc = "Set the field `country`.\n"]
    pub fn set_country(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.country = Some(v.into());
        self
    }

    #[doc = "Set the field `county`.\n"]
    pub fn set_county(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.county = Some(v.into());
        self
    }

    #[doc = "Set the field `postal_code`.\n"]
    pub fn set_postal_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.postal_code = Some(v.into());
        self
    }

    #[doc = "Set the field `province`.\n"]
    pub fn set_province(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.province = Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}

impl ToListMappable for CustomerprofilesProfileBillingAddressEl {
    type O = BlockAssignable<CustomerprofilesProfileBillingAddressEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesProfileBillingAddressEl {}

impl BuildCustomerprofilesProfileBillingAddressEl {
    pub fn build(self) -> CustomerprofilesProfileBillingAddressEl {
        CustomerprofilesProfileBillingAddressEl {
            address_1: core::default::Default::default(),
            address_2: core::default::Default::default(),
            address_3: core::default::Default::default(),
            address_4: core::default::Default::default(),
            city: core::default::Default::default(),
            country: core::default::Default::default(),
            county: core::default::Default::default(),
            postal_code: core::default::Default::default(),
            province: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

pub struct CustomerprofilesProfileBillingAddressElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CustomerprofilesProfileBillingAddressElRef {
    fn new(shared: StackShared, base: String) -> CustomerprofilesProfileBillingAddressElRef {
        CustomerprofilesProfileBillingAddressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesProfileBillingAddressElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_1` after provisioning.\n"]
    pub fn address_1(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_1", self.base))
    }

    #[doc = "Get a reference to the value of field `address_2` after provisioning.\n"]
    pub fn address_2(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_2", self.base))
    }

    #[doc = "Get a reference to the value of field `address_3` after provisioning.\n"]
    pub fn address_3(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_3", self.base))
    }

    #[doc = "Get a reference to the value of field `address_4` after provisioning.\n"]
    pub fn address_4(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_4", self.base))
    }

    #[doc = "Get a reference to the value of field `city` after provisioning.\n"]
    pub fn city(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.city", self.base))
    }

    #[doc = "Get a reference to the value of field `country` after provisioning.\n"]
    pub fn country(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.country", self.base))
    }

    #[doc = "Get a reference to the value of field `county` after provisioning.\n"]
    pub fn county(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.county", self.base))
    }

    #[doc = "Get a reference to the value of field `postal_code` after provisioning.\n"]
    pub fn postal_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.postal_code", self.base))
    }

    #[doc = "Get a reference to the value of field `province` after provisioning.\n"]
    pub fn province(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.province", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomerprofilesProfileMailingAddressEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address_1: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_2: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_3: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_4: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    city: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    county: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    postal_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    province: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}

impl CustomerprofilesProfileMailingAddressEl {
    #[doc = "Set the field `address_1`.\n"]
    pub fn set_address_1(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address_1 = Some(v.into());
        self
    }

    #[doc = "Set the field `address_2`.\n"]
    pub fn set_address_2(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address_2 = Some(v.into());
        self
    }

    #[doc = "Set the field `address_3`.\n"]
    pub fn set_address_3(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address_3 = Some(v.into());
        self
    }

    #[doc = "Set the field `address_4`.\n"]
    pub fn set_address_4(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address_4 = Some(v.into());
        self
    }

    #[doc = "Set the field `city`.\n"]
    pub fn set_city(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.city = Some(v.into());
        self
    }

    #[doc = "Set the field `country`.\n"]
    pub fn set_country(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.country = Some(v.into());
        self
    }

    #[doc = "Set the field `county`.\n"]
    pub fn set_county(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.county = Some(v.into());
        self
    }

    #[doc = "Set the field `postal_code`.\n"]
    pub fn set_postal_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.postal_code = Some(v.into());
        self
    }

    #[doc = "Set the field `province`.\n"]
    pub fn set_province(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.province = Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}

impl ToListMappable for CustomerprofilesProfileMailingAddressEl {
    type O = BlockAssignable<CustomerprofilesProfileMailingAddressEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesProfileMailingAddressEl {}

impl BuildCustomerprofilesProfileMailingAddressEl {
    pub fn build(self) -> CustomerprofilesProfileMailingAddressEl {
        CustomerprofilesProfileMailingAddressEl {
            address_1: core::default::Default::default(),
            address_2: core::default::Default::default(),
            address_3: core::default::Default::default(),
            address_4: core::default::Default::default(),
            city: core::default::Default::default(),
            country: core::default::Default::default(),
            county: core::default::Default::default(),
            postal_code: core::default::Default::default(),
            province: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

pub struct CustomerprofilesProfileMailingAddressElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CustomerprofilesProfileMailingAddressElRef {
    fn new(shared: StackShared, base: String) -> CustomerprofilesProfileMailingAddressElRef {
        CustomerprofilesProfileMailingAddressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesProfileMailingAddressElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_1` after provisioning.\n"]
    pub fn address_1(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_1", self.base))
    }

    #[doc = "Get a reference to the value of field `address_2` after provisioning.\n"]
    pub fn address_2(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_2", self.base))
    }

    #[doc = "Get a reference to the value of field `address_3` after provisioning.\n"]
    pub fn address_3(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_3", self.base))
    }

    #[doc = "Get a reference to the value of field `address_4` after provisioning.\n"]
    pub fn address_4(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_4", self.base))
    }

    #[doc = "Get a reference to the value of field `city` after provisioning.\n"]
    pub fn city(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.city", self.base))
    }

    #[doc = "Get a reference to the value of field `country` after provisioning.\n"]
    pub fn country(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.country", self.base))
    }

    #[doc = "Get a reference to the value of field `county` after provisioning.\n"]
    pub fn county(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.county", self.base))
    }

    #[doc = "Get a reference to the value of field `postal_code` after provisioning.\n"]
    pub fn postal_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.postal_code", self.base))
    }

    #[doc = "Get a reference to the value of field `province` after provisioning.\n"]
    pub fn province(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.province", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomerprofilesProfileShippingAddressEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address_1: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_2: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_3: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_4: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    city: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    county: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    postal_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    province: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}

impl CustomerprofilesProfileShippingAddressEl {
    #[doc = "Set the field `address_1`.\n"]
    pub fn set_address_1(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address_1 = Some(v.into());
        self
    }

    #[doc = "Set the field `address_2`.\n"]
    pub fn set_address_2(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address_2 = Some(v.into());
        self
    }

    #[doc = "Set the field `address_3`.\n"]
    pub fn set_address_3(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address_3 = Some(v.into());
        self
    }

    #[doc = "Set the field `address_4`.\n"]
    pub fn set_address_4(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address_4 = Some(v.into());
        self
    }

    #[doc = "Set the field `city`.\n"]
    pub fn set_city(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.city = Some(v.into());
        self
    }

    #[doc = "Set the field `country`.\n"]
    pub fn set_country(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.country = Some(v.into());
        self
    }

    #[doc = "Set the field `county`.\n"]
    pub fn set_county(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.county = Some(v.into());
        self
    }

    #[doc = "Set the field `postal_code`.\n"]
    pub fn set_postal_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.postal_code = Some(v.into());
        self
    }

    #[doc = "Set the field `province`.\n"]
    pub fn set_province(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.province = Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}

impl ToListMappable for CustomerprofilesProfileShippingAddressEl {
    type O = BlockAssignable<CustomerprofilesProfileShippingAddressEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesProfileShippingAddressEl {}

impl BuildCustomerprofilesProfileShippingAddressEl {
    pub fn build(self) -> CustomerprofilesProfileShippingAddressEl {
        CustomerprofilesProfileShippingAddressEl {
            address_1: core::default::Default::default(),
            address_2: core::default::Default::default(),
            address_3: core::default::Default::default(),
            address_4: core::default::Default::default(),
            city: core::default::Default::default(),
            country: core::default::Default::default(),
            county: core::default::Default::default(),
            postal_code: core::default::Default::default(),
            province: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

pub struct CustomerprofilesProfileShippingAddressElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CustomerprofilesProfileShippingAddressElRef {
    fn new(shared: StackShared, base: String) -> CustomerprofilesProfileShippingAddressElRef {
        CustomerprofilesProfileShippingAddressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesProfileShippingAddressElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_1` after provisioning.\n"]
    pub fn address_1(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_1", self.base))
    }

    #[doc = "Get a reference to the value of field `address_2` after provisioning.\n"]
    pub fn address_2(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_2", self.base))
    }

    #[doc = "Get a reference to the value of field `address_3` after provisioning.\n"]
    pub fn address_3(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_3", self.base))
    }

    #[doc = "Get a reference to the value of field `address_4` after provisioning.\n"]
    pub fn address_4(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_4", self.base))
    }

    #[doc = "Get a reference to the value of field `city` after provisioning.\n"]
    pub fn city(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.city", self.base))
    }

    #[doc = "Get a reference to the value of field `country` after provisioning.\n"]
    pub fn country(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.country", self.base))
    }

    #[doc = "Get a reference to the value of field `county` after provisioning.\n"]
    pub fn county(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.county", self.base))
    }

    #[doc = "Get a reference to the value of field `postal_code` after provisioning.\n"]
    pub fn postal_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.postal_code", self.base))
    }

    #[doc = "Get a reference to the value of field `province` after provisioning.\n"]
    pub fn province(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.province", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize, Default)]
struct CustomerprofilesProfileDynamic {
    address: Option<DynamicBlock<CustomerprofilesProfileAddressEl>>,
    billing_address: Option<DynamicBlock<CustomerprofilesProfileBillingAddressEl>>,
    mailing_address: Option<DynamicBlock<CustomerprofilesProfileMailingAddressEl>>,
    shipping_address: Option<DynamicBlock<CustomerprofilesProfileShippingAddressEl>>,
}
