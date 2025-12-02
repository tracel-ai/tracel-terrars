use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct VpcData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assign_generated_ipv6_cidr_block: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_dns_hostnames: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_dns_support: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_network_address_usage_metrics: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_tenancy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv4_ipam_pool_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv4_netmask_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_cidr_block_network_border_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_ipam_pool_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_netmask_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
}
struct Vpc_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VpcData>,
}
#[derive(Clone)]
pub struct Vpc(Rc<Vpc_>);
impl Vpc {
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
    #[doc = "Set the field `assign_generated_ipv6_cidr_block`.\n"]
    pub fn set_assign_generated_ipv6_cidr_block(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().assign_generated_ipv6_cidr_block = Some(v.into());
        self
    }
    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cidr_block = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_dns_hostnames`.\n"]
    pub fn set_enable_dns_hostnames(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_dns_hostnames = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_dns_support`.\n"]
    pub fn set_enable_dns_support(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_dns_support = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_network_address_usage_metrics`.\n"]
    pub fn set_enable_network_address_usage_metrics(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0
            .data
            .borrow_mut()
            .enable_network_address_usage_metrics = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_tenancy`.\n"]
    pub fn set_instance_tenancy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().instance_tenancy = Some(v.into());
        self
    }
    #[doc = "Set the field `ipv4_ipam_pool_id`.\n"]
    pub fn set_ipv4_ipam_pool_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ipv4_ipam_pool_id = Some(v.into());
        self
    }
    #[doc = "Set the field `ipv4_netmask_length`.\n"]
    pub fn set_ipv4_netmask_length(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ipv4_netmask_length = Some(v.into());
        self
    }
    #[doc = "Set the field `ipv6_cidr_block`.\n"]
    pub fn set_ipv6_cidr_block(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ipv6_cidr_block = Some(v.into());
        self
    }
    #[doc = "Set the field `ipv6_cidr_block_network_border_group`.\n"]
    pub fn set_ipv6_cidr_block_network_border_group(self, v: impl Into<PrimField<String>>) -> Self {
        self.0
            .data
            .borrow_mut()
            .ipv6_cidr_block_network_border_group = Some(v.into());
        self
    }
    #[doc = "Set the field `ipv6_ipam_pool_id`.\n"]
    pub fn set_ipv6_ipam_pool_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ipv6_ipam_pool_id = Some(v.into());
        self
    }
    #[doc = "Set the field `ipv6_netmask_length`.\n"]
    pub fn set_ipv6_netmask_length(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ipv6_netmask_length = Some(v.into());
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
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `assign_generated_ipv6_cidr_block` after provisioning.\n"]
    pub fn assign_generated_ipv6_cidr_block(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.assign_generated_ipv6_cidr_block", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cidr_block", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_network_acl_id` after provisioning.\n"]
    pub fn default_network_acl_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_network_acl_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_route_table_id` after provisioning.\n"]
    pub fn default_route_table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_route_table_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_security_group_id` after provisioning.\n"]
    pub fn default_security_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_security_group_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dhcp_options_id` after provisioning.\n"]
    pub fn dhcp_options_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dhcp_options_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_dns_hostnames` after provisioning.\n"]
    pub fn enable_dns_hostnames(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_dns_hostnames", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_dns_support` after provisioning.\n"]
    pub fn enable_dns_support(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_dns_support", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_network_address_usage_metrics` after provisioning.\n"]
    pub fn enable_network_address_usage_metrics(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.enable_network_address_usage_metrics",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_tenancy` after provisioning.\n"]
    pub fn instance_tenancy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_tenancy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv4_ipam_pool_id` after provisioning.\n"]
    pub fn ipv4_ipam_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv4_ipam_pool_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv4_netmask_length` after provisioning.\n"]
    pub fn ipv4_netmask_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv4_netmask_length", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_association_id` after provisioning.\n"]
    pub fn ipv6_association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_association_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_cidr_block", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_cidr_block_network_border_group` after provisioning.\n"]
    pub fn ipv6_cidr_block_network_border_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.ipv6_cidr_block_network_border_group",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_ipam_pool_id` after provisioning.\n"]
    pub fn ipv6_ipam_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_ipam_pool_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_netmask_length` after provisioning.\n"]
    pub fn ipv6_netmask_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_netmask_length", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `main_route_table_id` after provisioning.\n"]
    pub fn main_route_table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.main_route_table_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_id", self.extract_ref()),
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
}
impl Referable for Vpc {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for Vpc {}
impl ToListMappable for Vpc {
    type O = ListRef<VpcRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for Vpc_ {
    fn extract_resource_type(&self) -> String {
        "aws_vpc".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildVpc {
    pub tf_id: String,
}
impl BuildVpc {
    pub fn build(self, stack: &mut Stack) -> Vpc {
        let out = Vpc(Rc::new(Vpc_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VpcData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                assign_generated_ipv6_cidr_block: core::default::Default::default(),
                cidr_block: core::default::Default::default(),
                enable_dns_hostnames: core::default::Default::default(),
                enable_dns_support: core::default::Default::default(),
                enable_network_address_usage_metrics: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_tenancy: core::default::Default::default(),
                ipv4_ipam_pool_id: core::default::Default::default(),
                ipv4_netmask_length: core::default::Default::default(),
                ipv6_cidr_block: core::default::Default::default(),
                ipv6_cidr_block_network_border_group: core::default::Default::default(),
                ipv6_ipam_pool_id: core::default::Default::default(),
                ipv6_netmask_length: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct VpcRef {
    shared: StackShared,
    base: String,
}
impl Ref for VpcRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl VpcRef {
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
    #[doc = "Get a reference to the value of field `assign_generated_ipv6_cidr_block` after provisioning.\n"]
    pub fn assign_generated_ipv6_cidr_block(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.assign_generated_ipv6_cidr_block", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cidr_block", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_network_acl_id` after provisioning.\n"]
    pub fn default_network_acl_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_network_acl_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_route_table_id` after provisioning.\n"]
    pub fn default_route_table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_route_table_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_security_group_id` after provisioning.\n"]
    pub fn default_security_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_security_group_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dhcp_options_id` after provisioning.\n"]
    pub fn dhcp_options_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dhcp_options_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_dns_hostnames` after provisioning.\n"]
    pub fn enable_dns_hostnames(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_dns_hostnames", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_dns_support` after provisioning.\n"]
    pub fn enable_dns_support(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_dns_support", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_network_address_usage_metrics` after provisioning.\n"]
    pub fn enable_network_address_usage_metrics(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.enable_network_address_usage_metrics",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_tenancy` after provisioning.\n"]
    pub fn instance_tenancy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_tenancy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv4_ipam_pool_id` after provisioning.\n"]
    pub fn ipv4_ipam_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv4_ipam_pool_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv4_netmask_length` after provisioning.\n"]
    pub fn ipv4_netmask_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv4_netmask_length", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_association_id` after provisioning.\n"]
    pub fn ipv6_association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_association_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_cidr_block", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_cidr_block_network_border_group` after provisioning.\n"]
    pub fn ipv6_cidr_block_network_border_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.ipv6_cidr_block_network_border_group",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_ipam_pool_id` after provisioning.\n"]
    pub fn ipv6_ipam_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_ipam_pool_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_netmask_length` after provisioning.\n"]
    pub fn ipv6_netmask_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_netmask_length", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `main_route_table_id` after provisioning.\n"]
    pub fn main_route_table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.main_route_table_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_id", self.extract_ref()),
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
}
