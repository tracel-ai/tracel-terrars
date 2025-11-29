use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSubnetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_for_az: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataSubnetFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataSubnetTimeoutsEl>,
    dynamic: DataSubnetDynamic,
}

struct DataSubnet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSubnetData>,
}

#[derive(Clone)]
pub struct DataSubnet(Rc<DataSubnet_>);

impl DataSubnet {
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

    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().availability_zone = Some(v.into());
        self
    }

    #[doc = "Set the field `availability_zone_id`.\n"]
    pub fn set_availability_zone_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().availability_zone_id = Some(v.into());
        self
    }

    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cidr_block = Some(v.into());
        self
    }

    #[doc = "Set the field `default_for_az`.\n"]
    pub fn set_default_for_az(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().default_for_az = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_cidr_block`.\n"]
    pub fn set_ipv6_cidr_block(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ipv6_cidr_block = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().state = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().vpc_id = Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataSubnetFilterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataSubnetTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `assign_ipv6_address_on_creation` after provisioning.\n"]
    pub fn assign_ipv6_address_on_creation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.assign_ipv6_address_on_creation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone_id` after provisioning.\n"]
    pub fn availability_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `available_ip_address_count` after provisioning.\n"]
    pub fn available_ip_address_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.available_ip_address_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_owned_ipv4_pool` after provisioning.\n"]
    pub fn customer_owned_ipv4_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_owned_ipv4_pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_for_az` after provisioning.\n"]
    pub fn default_for_az(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_for_az", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_dns64` after provisioning.\n"]
    pub fn enable_dns64(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_dns64", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_lni_at_device_index` after provisioning.\n"]
    pub fn enable_lni_at_device_index(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_lni_at_device_index", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_resource_name_dns_a_record_on_launch` after provisioning.\n"]
    pub fn enable_resource_name_dns_a_record_on_launch(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_resource_name_dns_a_record_on_launch", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `enable_resource_name_dns_aaaa_record_on_launch` after provisioning.\n"]
    pub fn enable_resource_name_dns_aaaa_record_on_launch(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_resource_name_dns_aaaa_record_on_launch", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block_association_id` after provisioning.\n"]
    pub fn ipv6_cidr_block_association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_cidr_block_association_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_native` after provisioning.\n"]
    pub fn ipv6_native(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_native", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `map_customer_owned_ip_on_launch` after provisioning.\n"]
    pub fn map_customer_owned_ip_on_launch(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.map_customer_owned_ip_on_launch", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `map_public_ip_on_launch` after provisioning.\n"]
    pub fn map_public_ip_on_launch(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.map_public_ip_on_launch", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.outpost_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns_hostname_type_on_launch` after provisioning.\n"]
    pub fn private_dns_hostname_type_on_launch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_dns_hostname_type_on_launch", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataSubnetTimeoutsElRef {
        DataSubnetTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for DataSubnet {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataSubnet { }

impl ToListMappable for DataSubnet {
    type O = ListRef<DataSubnetRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSubnet_ {
    fn extract_datasource_type(&self) -> String {
        "aws_subnet".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSubnet {
    pub tf_id: String,
}

impl BuildDataSubnet {
    pub fn build(self, stack: &mut Stack) -> DataSubnet {
        let out = DataSubnet(Rc::new(DataSubnet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSubnetData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                availability_zone: core::default::Default::default(),
                availability_zone_id: core::default::Default::default(),
                cidr_block: core::default::Default::default(),
                default_for_az: core::default::Default::default(),
                id: core::default::Default::default(),
                ipv6_cidr_block: core::default::Default::default(),
                region: core::default::Default::default(),
                state: core::default::Default::default(),
                tags: core::default::Default::default(),
                vpc_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSubnetRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSubnetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataSubnetRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `assign_ipv6_address_on_creation` after provisioning.\n"]
    pub fn assign_ipv6_address_on_creation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.assign_ipv6_address_on_creation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone_id` after provisioning.\n"]
    pub fn availability_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `available_ip_address_count` after provisioning.\n"]
    pub fn available_ip_address_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.available_ip_address_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_owned_ipv4_pool` after provisioning.\n"]
    pub fn customer_owned_ipv4_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_owned_ipv4_pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_for_az` after provisioning.\n"]
    pub fn default_for_az(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_for_az", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_dns64` after provisioning.\n"]
    pub fn enable_dns64(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_dns64", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_lni_at_device_index` after provisioning.\n"]
    pub fn enable_lni_at_device_index(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_lni_at_device_index", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_resource_name_dns_a_record_on_launch` after provisioning.\n"]
    pub fn enable_resource_name_dns_a_record_on_launch(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_resource_name_dns_a_record_on_launch", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `enable_resource_name_dns_aaaa_record_on_launch` after provisioning.\n"]
    pub fn enable_resource_name_dns_aaaa_record_on_launch(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_resource_name_dns_aaaa_record_on_launch", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block_association_id` after provisioning.\n"]
    pub fn ipv6_cidr_block_association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_cidr_block_association_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_native` after provisioning.\n"]
    pub fn ipv6_native(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_native", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `map_customer_owned_ip_on_launch` after provisioning.\n"]
    pub fn map_customer_owned_ip_on_launch(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.map_customer_owned_ip_on_launch", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `map_public_ip_on_launch` after provisioning.\n"]
    pub fn map_public_ip_on_launch(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.map_public_ip_on_launch", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.outpost_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns_hostname_type_on_launch` after provisioning.\n"]
    pub fn private_dns_hostname_type_on_launch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_dns_hostname_type_on_launch", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataSubnetTimeoutsElRef {
        DataSubnetTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSubnetFilterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl DataSubnetFilterEl { }

impl ToListMappable for DataSubnetFilterEl {
    type O = BlockAssignable<DataSubnetFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSubnetFilterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildDataSubnetFilterEl {
    pub fn build(self) -> DataSubnetFilterEl {
        DataSubnetFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataSubnetFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSubnetFilterElRef {
    fn new(shared: StackShared, base: String) -> DataSubnetFilterElRef {
        DataSubnetFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSubnetFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSubnetTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}

impl DataSubnetTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}

impl ToListMappable for DataSubnetTimeoutsEl {
    type O = BlockAssignable<DataSubnetTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSubnetTimeoutsEl {}

impl BuildDataSubnetTimeoutsEl {
    pub fn build(self) -> DataSubnetTimeoutsEl {
        DataSubnetTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataSubnetTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSubnetTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataSubnetTimeoutsElRef {
        DataSubnetTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSubnetTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataSubnetDynamic {
    filter: Option<DynamicBlock<DataSubnetFilterEl>>,
}
