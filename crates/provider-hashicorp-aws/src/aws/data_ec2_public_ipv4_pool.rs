use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataEc2PublicIpv4PoolData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    pool_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}
struct DataEc2PublicIpv4Pool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEc2PublicIpv4PoolData>,
}
#[derive(Clone)]
pub struct DataEc2PublicIpv4Pool(Rc<DataEc2PublicIpv4Pool_>);
impl DataEc2PublicIpv4Pool {
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
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `network_border_group` after provisioning.\n"]
    pub fn network_border_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_border_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pool_address_ranges` after provisioning.\n"]
    pub fn pool_address_ranges(&self) -> ListRef<DataEc2PublicIpv4PoolPoolAddressRangesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pool_address_ranges", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pool_id` after provisioning.\n"]
    pub fn pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pool_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `total_address_count` after provisioning.\n"]
    pub fn total_address_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_address_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `total_available_address_count` after provisioning.\n"]
    pub fn total_available_address_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_available_address_count", self.extract_ref()),
        )
    }
}
impl Referable for DataEc2PublicIpv4Pool {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataEc2PublicIpv4Pool {}
impl ToListMappable for DataEc2PublicIpv4Pool {
    type O = ListRef<DataEc2PublicIpv4PoolRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataEc2PublicIpv4Pool_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ec2_public_ipv4_pool".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataEc2PublicIpv4Pool {
    pub tf_id: String,
    #[doc = ""]
    pub pool_id: PrimField<String>,
}
impl BuildDataEc2PublicIpv4Pool {
    pub fn build(self, stack: &mut Stack) -> DataEc2PublicIpv4Pool {
        let out = DataEc2PublicIpv4Pool(Rc::new(DataEc2PublicIpv4Pool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEc2PublicIpv4PoolData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                pool_id: self.pool_id,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataEc2PublicIpv4PoolRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEc2PublicIpv4PoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataEc2PublicIpv4PoolRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `network_border_group` after provisioning.\n"]
    pub fn network_border_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_border_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pool_address_ranges` after provisioning.\n"]
    pub fn pool_address_ranges(&self) -> ListRef<DataEc2PublicIpv4PoolPoolAddressRangesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pool_address_ranges", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pool_id` after provisioning.\n"]
    pub fn pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pool_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `total_address_count` after provisioning.\n"]
    pub fn total_address_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_address_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `total_available_address_count` after provisioning.\n"]
    pub fn total_available_address_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_available_address_count", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataEc2PublicIpv4PoolPoolAddressRangesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    available_address_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_address: Option<PrimField<String>>,
}
impl DataEc2PublicIpv4PoolPoolAddressRangesEl {
    #[doc = "Set the field `address_count`.\n"]
    pub fn set_address_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.address_count = Some(v.into());
        self
    }
    #[doc = "Set the field `available_address_count`.\n"]
    pub fn set_available_address_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.available_address_count = Some(v.into());
        self
    }
    #[doc = "Set the field `first_address`.\n"]
    pub fn set_first_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.first_address = Some(v.into());
        self
    }
    #[doc = "Set the field `last_address`.\n"]
    pub fn set_last_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_address = Some(v.into());
        self
    }
}
impl ToListMappable for DataEc2PublicIpv4PoolPoolAddressRangesEl {
    type O = BlockAssignable<DataEc2PublicIpv4PoolPoolAddressRangesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataEc2PublicIpv4PoolPoolAddressRangesEl {}
impl BuildDataEc2PublicIpv4PoolPoolAddressRangesEl {
    pub fn build(self) -> DataEc2PublicIpv4PoolPoolAddressRangesEl {
        DataEc2PublicIpv4PoolPoolAddressRangesEl {
            address_count: core::default::Default::default(),
            available_address_count: core::default::Default::default(),
            first_address: core::default::Default::default(),
            last_address: core::default::Default::default(),
        }
    }
}
pub struct DataEc2PublicIpv4PoolPoolAddressRangesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEc2PublicIpv4PoolPoolAddressRangesElRef {
    fn new(shared: StackShared, base: String) -> DataEc2PublicIpv4PoolPoolAddressRangesElRef {
        DataEc2PublicIpv4PoolPoolAddressRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataEc2PublicIpv4PoolPoolAddressRangesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address_count` after provisioning.\n"]
    pub fn address_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.address_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `available_address_count` after provisioning.\n"]
    pub fn available_address_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_address_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `first_address` after provisioning.\n"]
    pub fn first_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.first_address", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `last_address` after provisioning.\n"]
    pub fn last_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_address", self.base))
    }
}
