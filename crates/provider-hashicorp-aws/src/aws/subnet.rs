use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct SubnetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assign_ipv6_address_on_creation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_owned_ipv4_pool: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_dns64: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_lni_at_device_index: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_resource_name_dns_a_record_on_launch: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_resource_name_dns_aaaa_record_on_launch: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_native: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    map_customer_owned_ip_on_launch: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    map_public_ip_on_launch: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outpost_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_dns_hostname_type_on_launch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    vpc_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SubnetTimeoutsEl>,
}

struct Subnet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SubnetData>,
}

#[derive(Clone)]
pub struct Subnet(Rc<Subnet_>);

impl Subnet {
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

    #[doc = "Set the field `assign_ipv6_address_on_creation`.\n"]
    pub fn set_assign_ipv6_address_on_creation(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().assign_ipv6_address_on_creation = Some(v.into());
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

    #[doc = "Set the field `customer_owned_ipv4_pool`.\n"]
    pub fn set_customer_owned_ipv4_pool(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().customer_owned_ipv4_pool = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_dns64`.\n"]
    pub fn set_enable_dns64(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_dns64 = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_lni_at_device_index`.\n"]
    pub fn set_enable_lni_at_device_index(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().enable_lni_at_device_index = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_resource_name_dns_a_record_on_launch`.\n"]
    pub fn set_enable_resource_name_dns_a_record_on_launch(
        self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .enable_resource_name_dns_a_record_on_launch = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_resource_name_dns_aaaa_record_on_launch`.\n"]
    pub fn set_enable_resource_name_dns_aaaa_record_on_launch(
        self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .enable_resource_name_dns_aaaa_record_on_launch = Some(v.into());
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

    #[doc = "Set the field `ipv6_native`.\n"]
    pub fn set_ipv6_native(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ipv6_native = Some(v.into());
        self
    }

    #[doc = "Set the field `map_customer_owned_ip_on_launch`.\n"]
    pub fn set_map_customer_owned_ip_on_launch(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().map_customer_owned_ip_on_launch = Some(v.into());
        self
    }

    #[doc = "Set the field `map_public_ip_on_launch`.\n"]
    pub fn set_map_public_ip_on_launch(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().map_public_ip_on_launch = Some(v.into());
        self
    }

    #[doc = "Set the field `outpost_arn`.\n"]
    pub fn set_outpost_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().outpost_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `private_dns_hostname_type_on_launch`.\n"]
    pub fn set_private_dns_hostname_type_on_launch(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().private_dns_hostname_type_on_launch = Some(v.into());
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<SubnetTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `assign_ipv6_address_on_creation` after provisioning.\n"]
    pub fn assign_ipv6_address_on_creation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.assign_ipv6_address_on_creation", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `availability_zone_id` after provisioning.\n"]
    pub fn availability_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cidr_block", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `customer_owned_ipv4_pool` after provisioning.\n"]
    pub fn customer_owned_ipv4_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_owned_ipv4_pool", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_dns64` after provisioning.\n"]
    pub fn enable_dns64(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_dns64", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_lni_at_device_index` after provisioning.\n"]
    pub fn enable_lni_at_device_index(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_lni_at_device_index", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_resource_name_dns_a_record_on_launch` after provisioning.\n"]
    pub fn enable_resource_name_dns_a_record_on_launch(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.enable_resource_name_dns_a_record_on_launch",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `enable_resource_name_dns_aaaa_record_on_launch` after provisioning.\n"]
    pub fn enable_resource_name_dns_aaaa_record_on_launch(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.enable_resource_name_dns_aaaa_record_on_launch",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_cidr_block", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block_association_id` after provisioning.\n"]
    pub fn ipv6_cidr_block_association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_cidr_block_association_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ipv6_native` after provisioning.\n"]
    pub fn ipv6_native(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_native", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `map_customer_owned_ip_on_launch` after provisioning.\n"]
    pub fn map_customer_owned_ip_on_launch(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.map_customer_owned_ip_on_launch", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `map_public_ip_on_launch` after provisioning.\n"]
    pub fn map_public_ip_on_launch(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.map_public_ip_on_launch", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.outpost_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `private_dns_hostname_type_on_launch` after provisioning.\n"]
    pub fn private_dns_hostname_type_on_launch(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_dns_hostname_type_on_launch", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SubnetTimeoutsElRef {
        SubnetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for Subnet {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for Subnet {}

impl ToListMappable for Subnet {
    type O = ListRef<SubnetRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Subnet_ {
    fn extract_resource_type(&self) -> String {
        "aws_subnet".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSubnet {
    pub tf_id: String,
    #[doc = ""]
    pub vpc_id: PrimField<String>,
}

impl BuildSubnet {
    pub fn build(self, stack: &mut Stack) -> Subnet {
        let out = Subnet(Rc::new(Subnet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SubnetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                assign_ipv6_address_on_creation: core::default::Default::default(),
                availability_zone: core::default::Default::default(),
                availability_zone_id: core::default::Default::default(),
                cidr_block: core::default::Default::default(),
                customer_owned_ipv4_pool: core::default::Default::default(),
                enable_dns64: core::default::Default::default(),
                enable_lni_at_device_index: core::default::Default::default(),
                enable_resource_name_dns_a_record_on_launch: core::default::Default::default(),
                enable_resource_name_dns_aaaa_record_on_launch: core::default::Default::default(),
                id: core::default::Default::default(),
                ipv6_cidr_block: core::default::Default::default(),
                ipv6_native: core::default::Default::default(),
                map_customer_owned_ip_on_launch: core::default::Default::default(),
                map_public_ip_on_launch: core::default::Default::default(),
                outpost_arn: core::default::Default::default(),
                private_dns_hostname_type_on_launch: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                vpc_id: self.vpc_id,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SubnetRef {
    shared: StackShared,
    base: String,
}

impl Ref for SubnetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl SubnetRef {
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

    #[doc = "Get a reference to the value of field `assign_ipv6_address_on_creation` after provisioning.\n"]
    pub fn assign_ipv6_address_on_creation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.assign_ipv6_address_on_creation", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `availability_zone_id` after provisioning.\n"]
    pub fn availability_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cidr_block", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `customer_owned_ipv4_pool` after provisioning.\n"]
    pub fn customer_owned_ipv4_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_owned_ipv4_pool", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_dns64` after provisioning.\n"]
    pub fn enable_dns64(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_dns64", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_lni_at_device_index` after provisioning.\n"]
    pub fn enable_lni_at_device_index(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_lni_at_device_index", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_resource_name_dns_a_record_on_launch` after provisioning.\n"]
    pub fn enable_resource_name_dns_a_record_on_launch(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.enable_resource_name_dns_a_record_on_launch",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `enable_resource_name_dns_aaaa_record_on_launch` after provisioning.\n"]
    pub fn enable_resource_name_dns_aaaa_record_on_launch(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.enable_resource_name_dns_aaaa_record_on_launch",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_cidr_block", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block_association_id` after provisioning.\n"]
    pub fn ipv6_cidr_block_association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_cidr_block_association_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ipv6_native` after provisioning.\n"]
    pub fn ipv6_native(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_native", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `map_customer_owned_ip_on_launch` after provisioning.\n"]
    pub fn map_customer_owned_ip_on_launch(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.map_customer_owned_ip_on_launch", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `map_public_ip_on_launch` after provisioning.\n"]
    pub fn map_public_ip_on_launch(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.map_public_ip_on_launch", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.outpost_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `private_dns_hostname_type_on_launch` after provisioning.\n"]
    pub fn private_dns_hostname_type_on_launch(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_dns_hostname_type_on_launch", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SubnetTimeoutsElRef {
        SubnetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct SubnetTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl SubnetTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
}

impl ToListMappable for SubnetTimeoutsEl {
    type O = BlockAssignable<SubnetTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSubnetTimeoutsEl {}

impl BuildSubnetTimeoutsEl {
    pub fn build(self) -> SubnetTimeoutsEl {
        SubnetTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct SubnetTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SubnetTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> SubnetTimeoutsElRef {
        SubnetTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SubnetTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}
