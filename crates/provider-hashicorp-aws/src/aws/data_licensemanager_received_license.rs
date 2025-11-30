use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataLicensemanagerReceivedLicenseData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    license_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataLicensemanagerReceivedLicense_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataLicensemanagerReceivedLicenseData>,
}

#[derive(Clone)]
pub struct DataLicensemanagerReceivedLicense(Rc<DataLicensemanagerReceivedLicense_>);

impl DataLicensemanagerReceivedLicense {
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

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `beneficiary` after provisioning.\n"]
    pub fn beneficiary(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.beneficiary", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `consumption_configuration` after provisioning.\n"]
    pub fn consumption_configuration(
        &self,
    ) -> ListRef<DataLicensemanagerReceivedLicenseConsumptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.consumption_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `entitlements` after provisioning.\n"]
    pub fn entitlements(&self) -> SetRef<DataLicensemanagerReceivedLicenseEntitlementsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.entitlements", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `home_region` after provisioning.\n"]
    pub fn home_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.home_region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> ListRef<DataLicensemanagerReceivedLicenseIssuerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.issuer", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `license_arn` after provisioning.\n"]
    pub fn license_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.license_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `license_metadata` after provisioning.\n"]
    pub fn license_metadata(
        &self,
    ) -> SetRef<DataLicensemanagerReceivedLicenseLicenseMetadataElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.license_metadata", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `license_name` after provisioning.\n"]
    pub fn license_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.license_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `product_name` after provisioning.\n"]
    pub fn product_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.product_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `product_sku` after provisioning.\n"]
    pub fn product_sku(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.product_sku", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `received_metadata` after provisioning.\n"]
    pub fn received_metadata(
        &self,
    ) -> ListRef<DataLicensemanagerReceivedLicenseReceivedMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.received_metadata", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `validity` after provisioning.\n"]
    pub fn validity(&self) -> ListRef<DataLicensemanagerReceivedLicenseValidityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.validity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
}

impl Referable for DataLicensemanagerReceivedLicense {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataLicensemanagerReceivedLicense {}

impl ToListMappable for DataLicensemanagerReceivedLicense {
    type O = ListRef<DataLicensemanagerReceivedLicenseRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataLicensemanagerReceivedLicense_ {
    fn extract_datasource_type(&self) -> String {
        "aws_licensemanager_received_license".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLicensemanagerReceivedLicense {
    pub tf_id: String,
    #[doc = ""]
    pub license_arn: PrimField<String>,
}

impl BuildDataLicensemanagerReceivedLicense {
    pub fn build(self, stack: &mut Stack) -> DataLicensemanagerReceivedLicense {
        let out = DataLicensemanagerReceivedLicense(Rc::new(DataLicensemanagerReceivedLicense_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataLicensemanagerReceivedLicenseData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                license_arn: self.license_arn,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataLicensemanagerReceivedLicenseRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataLicensemanagerReceivedLicenseRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataLicensemanagerReceivedLicenseRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `beneficiary` after provisioning.\n"]
    pub fn beneficiary(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.beneficiary", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `consumption_configuration` after provisioning.\n"]
    pub fn consumption_configuration(
        &self,
    ) -> ListRef<DataLicensemanagerReceivedLicenseConsumptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.consumption_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `entitlements` after provisioning.\n"]
    pub fn entitlements(&self) -> SetRef<DataLicensemanagerReceivedLicenseEntitlementsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.entitlements", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `home_region` after provisioning.\n"]
    pub fn home_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.home_region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> ListRef<DataLicensemanagerReceivedLicenseIssuerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.issuer", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `license_arn` after provisioning.\n"]
    pub fn license_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.license_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `license_metadata` after provisioning.\n"]
    pub fn license_metadata(
        &self,
    ) -> SetRef<DataLicensemanagerReceivedLicenseLicenseMetadataElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.license_metadata", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `license_name` after provisioning.\n"]
    pub fn license_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.license_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `product_name` after provisioning.\n"]
    pub fn product_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.product_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `product_sku` after provisioning.\n"]
    pub fn product_sku(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.product_sku", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `received_metadata` after provisioning.\n"]
    pub fn received_metadata(
        &self,
    ) -> ListRef<DataLicensemanagerReceivedLicenseReceivedMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.received_metadata", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `validity` after provisioning.\n"]
    pub fn validity(&self) -> ListRef<DataLicensemanagerReceivedLicenseValidityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.validity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataLicensemanagerReceivedLicenseConsumptionConfigurationElBorrowConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_early_check_in: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_time_to_live_in_minutes: Option<PrimField<f64>>,
}

impl DataLicensemanagerReceivedLicenseConsumptionConfigurationElBorrowConfigurationEl {
    #[doc = "Set the field `allow_early_check_in`.\n"]
    pub fn set_allow_early_check_in(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_early_check_in = Some(v.into());
        self
    }

    #[doc = "Set the field `max_time_to_live_in_minutes`.\n"]
    pub fn set_max_time_to_live_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_time_to_live_in_minutes = Some(v.into());
        self
    }
}

impl ToListMappable
    for DataLicensemanagerReceivedLicenseConsumptionConfigurationElBorrowConfigurationEl
{
    type O = BlockAssignable<
        DataLicensemanagerReceivedLicenseConsumptionConfigurationElBorrowConfigurationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLicensemanagerReceivedLicenseConsumptionConfigurationElBorrowConfigurationEl {
}

impl BuildDataLicensemanagerReceivedLicenseConsumptionConfigurationElBorrowConfigurationEl {
    pub fn build(
        self,
    ) -> DataLicensemanagerReceivedLicenseConsumptionConfigurationElBorrowConfigurationEl {
        DataLicensemanagerReceivedLicenseConsumptionConfigurationElBorrowConfigurationEl {
            allow_early_check_in: core::default::Default::default(),
            max_time_to_live_in_minutes: core::default::Default::default(),
        }
    }
}

pub struct DataLicensemanagerReceivedLicenseConsumptionConfigurationElBorrowConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataLicensemanagerReceivedLicenseConsumptionConfigurationElBorrowConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLicensemanagerReceivedLicenseConsumptionConfigurationElBorrowConfigurationElRef {
        DataLicensemanagerReceivedLicenseConsumptionConfigurationElBorrowConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLicensemanagerReceivedLicenseConsumptionConfigurationElBorrowConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_early_check_in` after provisioning.\n"]
    pub fn allow_early_check_in(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_early_check_in", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `max_time_to_live_in_minutes` after provisioning.\n"]
    pub fn max_time_to_live_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_time_to_live_in_minutes", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataLicensemanagerReceivedLicenseConsumptionConfigurationElProvisionalConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_time_to_live_in_minutes: Option<PrimField<f64>>,
}

impl DataLicensemanagerReceivedLicenseConsumptionConfigurationElProvisionalConfigurationEl {
    #[doc = "Set the field `max_time_to_live_in_minutes`.\n"]
    pub fn set_max_time_to_live_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_time_to_live_in_minutes = Some(v.into());
        self
    }
}

impl ToListMappable
    for DataLicensemanagerReceivedLicenseConsumptionConfigurationElProvisionalConfigurationEl
{
    type O = BlockAssignable<
        DataLicensemanagerReceivedLicenseConsumptionConfigurationElProvisionalConfigurationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLicensemanagerReceivedLicenseConsumptionConfigurationElProvisionalConfigurationEl
{}

impl BuildDataLicensemanagerReceivedLicenseConsumptionConfigurationElProvisionalConfigurationEl {
    pub fn build(
        self,
    ) -> DataLicensemanagerReceivedLicenseConsumptionConfigurationElProvisionalConfigurationEl {
        DataLicensemanagerReceivedLicenseConsumptionConfigurationElProvisionalConfigurationEl {
            max_time_to_live_in_minutes: core::default::Default::default(),
        }
    }
}

pub struct DataLicensemanagerReceivedLicenseConsumptionConfigurationElProvisionalConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref
    for DataLicensemanagerReceivedLicenseConsumptionConfigurationElProvisionalConfigurationElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLicensemanagerReceivedLicenseConsumptionConfigurationElProvisionalConfigurationElRef
    {
        DataLicensemanagerReceivedLicenseConsumptionConfigurationElProvisionalConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLicensemanagerReceivedLicenseConsumptionConfigurationElProvisionalConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_time_to_live_in_minutes` after provisioning.\n"]
    pub fn max_time_to_live_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_time_to_live_in_minutes", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataLicensemanagerReceivedLicenseConsumptionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    borrow_configuration: Option<
        ListField<DataLicensemanagerReceivedLicenseConsumptionConfigurationElBorrowConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisional_configuration: Option<
        ListField<
            DataLicensemanagerReceivedLicenseConsumptionConfigurationElProvisionalConfigurationEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    renew_type: Option<PrimField<String>>,
}

impl DataLicensemanagerReceivedLicenseConsumptionConfigurationEl {
    #[doc = "Set the field `borrow_configuration`.\n"]
    pub fn set_borrow_configuration(
        mut self,
        v: impl Into<
            ListField<
                DataLicensemanagerReceivedLicenseConsumptionConfigurationElBorrowConfigurationEl,
            >,
        >,
    ) -> Self {
        self.borrow_configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `provisional_configuration`.\n"]
    pub fn set_provisional_configuration(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataLicensemanagerReceivedLicenseConsumptionConfigurationElProvisionalConfigurationEl,
                        >,
                    >,
    ) -> Self {
        self.provisional_configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `renew_type`.\n"]
    pub fn set_renew_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.renew_type = Some(v.into());
        self
    }
}

impl ToListMappable for DataLicensemanagerReceivedLicenseConsumptionConfigurationEl {
    type O = BlockAssignable<DataLicensemanagerReceivedLicenseConsumptionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLicensemanagerReceivedLicenseConsumptionConfigurationEl {}

impl BuildDataLicensemanagerReceivedLicenseConsumptionConfigurationEl {
    pub fn build(self) -> DataLicensemanagerReceivedLicenseConsumptionConfigurationEl {
        DataLicensemanagerReceivedLicenseConsumptionConfigurationEl {
            borrow_configuration: core::default::Default::default(),
            provisional_configuration: core::default::Default::default(),
            renew_type: core::default::Default::default(),
        }
    }
}

pub struct DataLicensemanagerReceivedLicenseConsumptionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataLicensemanagerReceivedLicenseConsumptionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLicensemanagerReceivedLicenseConsumptionConfigurationElRef {
        DataLicensemanagerReceivedLicenseConsumptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLicensemanagerReceivedLicenseConsumptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `borrow_configuration` after provisioning.\n"]
    pub fn borrow_configuration(
        &self,
    ) -> ListRef<DataLicensemanagerReceivedLicenseConsumptionConfigurationElBorrowConfigurationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.borrow_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `provisional_configuration` after provisioning.\n"]
    pub fn provisional_configuration(
        &self,
    ) -> ListRef<
        DataLicensemanagerReceivedLicenseConsumptionConfigurationElProvisionalConfigurationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.provisional_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `renew_type` after provisioning.\n"]
    pub fn renew_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.renew_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLicensemanagerReceivedLicenseEntitlementsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_check_in: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overage: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataLicensemanagerReceivedLicenseEntitlementsEl {
    #[doc = "Set the field `allow_check_in`.\n"]
    pub fn set_allow_check_in(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_check_in = Some(v.into());
        self
    }

    #[doc = "Set the field `max_count`.\n"]
    pub fn set_max_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_count = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `overage`.\n"]
    pub fn set_overage(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.overage = Some(v.into());
        self
    }

    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataLicensemanagerReceivedLicenseEntitlementsEl {
    type O = BlockAssignable<DataLicensemanagerReceivedLicenseEntitlementsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLicensemanagerReceivedLicenseEntitlementsEl {}

impl BuildDataLicensemanagerReceivedLicenseEntitlementsEl {
    pub fn build(self) -> DataLicensemanagerReceivedLicenseEntitlementsEl {
        DataLicensemanagerReceivedLicenseEntitlementsEl {
            allow_check_in: core::default::Default::default(),
            max_count: core::default::Default::default(),
            name: core::default::Default::default(),
            overage: core::default::Default::default(),
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataLicensemanagerReceivedLicenseEntitlementsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataLicensemanagerReceivedLicenseEntitlementsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLicensemanagerReceivedLicenseEntitlementsElRef {
        DataLicensemanagerReceivedLicenseEntitlementsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLicensemanagerReceivedLicenseEntitlementsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_check_in` after provisioning.\n"]
    pub fn allow_check_in(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_check_in", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `max_count` after provisioning.\n"]
    pub fn max_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_count", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `overage` after provisioning.\n"]
    pub fn overage(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.overage", self.base))
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLicensemanagerReceivedLicenseIssuerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key_fingerprint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sign_key: Option<PrimField<String>>,
}

impl DataLicensemanagerReceivedLicenseIssuerEl {
    #[doc = "Set the field `key_fingerprint`.\n"]
    pub fn set_key_fingerprint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_fingerprint = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `sign_key`.\n"]
    pub fn set_sign_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sign_key = Some(v.into());
        self
    }
}

impl ToListMappable for DataLicensemanagerReceivedLicenseIssuerEl {
    type O = BlockAssignable<DataLicensemanagerReceivedLicenseIssuerEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLicensemanagerReceivedLicenseIssuerEl {}

impl BuildDataLicensemanagerReceivedLicenseIssuerEl {
    pub fn build(self) -> DataLicensemanagerReceivedLicenseIssuerEl {
        DataLicensemanagerReceivedLicenseIssuerEl {
            key_fingerprint: core::default::Default::default(),
            name: core::default::Default::default(),
            sign_key: core::default::Default::default(),
        }
    }
}

pub struct DataLicensemanagerReceivedLicenseIssuerElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataLicensemanagerReceivedLicenseIssuerElRef {
    fn new(shared: StackShared, base: String) -> DataLicensemanagerReceivedLicenseIssuerElRef {
        DataLicensemanagerReceivedLicenseIssuerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLicensemanagerReceivedLicenseIssuerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_fingerprint` after provisioning.\n"]
    pub fn key_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_fingerprint", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `sign_key` after provisioning.\n"]
    pub fn sign_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sign_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLicensemanagerReceivedLicenseLicenseMetadataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataLicensemanagerReceivedLicenseLicenseMetadataEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataLicensemanagerReceivedLicenseLicenseMetadataEl {
    type O = BlockAssignable<DataLicensemanagerReceivedLicenseLicenseMetadataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLicensemanagerReceivedLicenseLicenseMetadataEl {}

impl BuildDataLicensemanagerReceivedLicenseLicenseMetadataEl {
    pub fn build(self) -> DataLicensemanagerReceivedLicenseLicenseMetadataEl {
        DataLicensemanagerReceivedLicenseLicenseMetadataEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataLicensemanagerReceivedLicenseLicenseMetadataElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataLicensemanagerReceivedLicenseLicenseMetadataElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLicensemanagerReceivedLicenseLicenseMetadataElRef {
        DataLicensemanagerReceivedLicenseLicenseMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLicensemanagerReceivedLicenseLicenseMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLicensemanagerReceivedLicenseReceivedMetadataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_operations: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    received_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    received_status_reason: Option<PrimField<String>>,
}

impl DataLicensemanagerReceivedLicenseReceivedMetadataEl {
    #[doc = "Set the field `allowed_operations`.\n"]
    pub fn set_allowed_operations(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allowed_operations = Some(v.into());
        self
    }

    #[doc = "Set the field `received_status`.\n"]
    pub fn set_received_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.received_status = Some(v.into());
        self
    }

    #[doc = "Set the field `received_status_reason`.\n"]
    pub fn set_received_status_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.received_status_reason = Some(v.into());
        self
    }
}

impl ToListMappable for DataLicensemanagerReceivedLicenseReceivedMetadataEl {
    type O = BlockAssignable<DataLicensemanagerReceivedLicenseReceivedMetadataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLicensemanagerReceivedLicenseReceivedMetadataEl {}

impl BuildDataLicensemanagerReceivedLicenseReceivedMetadataEl {
    pub fn build(self) -> DataLicensemanagerReceivedLicenseReceivedMetadataEl {
        DataLicensemanagerReceivedLicenseReceivedMetadataEl {
            allowed_operations: core::default::Default::default(),
            received_status: core::default::Default::default(),
            received_status_reason: core::default::Default::default(),
        }
    }
}

pub struct DataLicensemanagerReceivedLicenseReceivedMetadataElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataLicensemanagerReceivedLicenseReceivedMetadataElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLicensemanagerReceivedLicenseReceivedMetadataElRef {
        DataLicensemanagerReceivedLicenseReceivedMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLicensemanagerReceivedLicenseReceivedMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_operations` after provisioning.\n"]
    pub fn allowed_operations(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.allowed_operations", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `received_status` after provisioning.\n"]
    pub fn received_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.received_status", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `received_status_reason` after provisioning.\n"]
    pub fn received_status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.received_status_reason", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataLicensemanagerReceivedLicenseValidityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    begin: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<PrimField<String>>,
}

impl DataLicensemanagerReceivedLicenseValidityEl {
    #[doc = "Set the field `begin`.\n"]
    pub fn set_begin(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.begin = Some(v.into());
        self
    }

    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end = Some(v.into());
        self
    }
}

impl ToListMappable for DataLicensemanagerReceivedLicenseValidityEl {
    type O = BlockAssignable<DataLicensemanagerReceivedLicenseValidityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLicensemanagerReceivedLicenseValidityEl {}

impl BuildDataLicensemanagerReceivedLicenseValidityEl {
    pub fn build(self) -> DataLicensemanagerReceivedLicenseValidityEl {
        DataLicensemanagerReceivedLicenseValidityEl {
            begin: core::default::Default::default(),
            end: core::default::Default::default(),
        }
    }
}

pub struct DataLicensemanagerReceivedLicenseValidityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataLicensemanagerReceivedLicenseValidityElRef {
    fn new(shared: StackShared, base: String) -> DataLicensemanagerReceivedLicenseValidityElRef {
        DataLicensemanagerReceivedLicenseValidityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLicensemanagerReceivedLicenseValidityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `begin` after provisioning.\n"]
    pub fn begin(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.begin", self.base))
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }
}
