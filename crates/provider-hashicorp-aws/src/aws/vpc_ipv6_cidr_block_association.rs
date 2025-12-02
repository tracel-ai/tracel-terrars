use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct VpcIpv6CidrBlockAssociationData {
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
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_ipam_pool_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_netmask_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_pool: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    vpc_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VpcIpv6CidrBlockAssociationTimeoutsEl>,
}
struct VpcIpv6CidrBlockAssociation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VpcIpv6CidrBlockAssociationData>,
}
#[derive(Clone)]
pub struct VpcIpv6CidrBlockAssociation(Rc<VpcIpv6CidrBlockAssociation_>);
impl VpcIpv6CidrBlockAssociation {
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
    #[doc = "Set the field `ipv6_pool`.\n"]
    pub fn set_ipv6_pool(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ipv6_pool = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VpcIpv6CidrBlockAssociationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `assign_generated_ipv6_cidr_block` after provisioning.\n"]
    pub fn assign_generated_ipv6_cidr_block(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.assign_generated_ipv6_cidr_block", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_source` after provisioning.\n"]
    pub fn ip_source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_address_attribute` after provisioning.\n"]
    pub fn ipv6_address_attribute(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_address_attribute", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_cidr_block", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `ipv6_pool` after provisioning.\n"]
    pub fn ipv6_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_pool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
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
    pub fn timeouts(&self) -> VpcIpv6CidrBlockAssociationTimeoutsElRef {
        VpcIpv6CidrBlockAssociationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for VpcIpv6CidrBlockAssociation {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for VpcIpv6CidrBlockAssociation {}
impl ToListMappable for VpcIpv6CidrBlockAssociation {
    type O = ListRef<VpcIpv6CidrBlockAssociationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for VpcIpv6CidrBlockAssociation_ {
    fn extract_resource_type(&self) -> String {
        "aws_vpc_ipv6_cidr_block_association".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildVpcIpv6CidrBlockAssociation {
    pub tf_id: String,
    #[doc = ""]
    pub vpc_id: PrimField<String>,
}
impl BuildVpcIpv6CidrBlockAssociation {
    pub fn build(self, stack: &mut Stack) -> VpcIpv6CidrBlockAssociation {
        let out = VpcIpv6CidrBlockAssociation(Rc::new(VpcIpv6CidrBlockAssociation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VpcIpv6CidrBlockAssociationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                assign_generated_ipv6_cidr_block: core::default::Default::default(),
                id: core::default::Default::default(),
                ipv6_cidr_block: core::default::Default::default(),
                ipv6_ipam_pool_id: core::default::Default::default(),
                ipv6_netmask_length: core::default::Default::default(),
                ipv6_pool: core::default::Default::default(),
                region: core::default::Default::default(),
                vpc_id: self.vpc_id,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct VpcIpv6CidrBlockAssociationRef {
    shared: StackShared,
    base: String,
}
impl Ref for VpcIpv6CidrBlockAssociationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl VpcIpv6CidrBlockAssociationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `assign_generated_ipv6_cidr_block` after provisioning.\n"]
    pub fn assign_generated_ipv6_cidr_block(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.assign_generated_ipv6_cidr_block", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_source` after provisioning.\n"]
    pub fn ip_source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_address_attribute` after provisioning.\n"]
    pub fn ipv6_address_attribute(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_address_attribute", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_cidr_block", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `ipv6_pool` after provisioning.\n"]
    pub fn ipv6_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_pool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
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
    pub fn timeouts(&self) -> VpcIpv6CidrBlockAssociationTimeoutsElRef {
        VpcIpv6CidrBlockAssociationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct VpcIpv6CidrBlockAssociationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl VpcIpv6CidrBlockAssociationTimeoutsEl {
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
impl ToListMappable for VpcIpv6CidrBlockAssociationTimeoutsEl {
    type O = BlockAssignable<VpcIpv6CidrBlockAssociationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVpcIpv6CidrBlockAssociationTimeoutsEl {}
impl BuildVpcIpv6CidrBlockAssociationTimeoutsEl {
    pub fn build(self) -> VpcIpv6CidrBlockAssociationTimeoutsEl {
        VpcIpv6CidrBlockAssociationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct VpcIpv6CidrBlockAssociationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VpcIpv6CidrBlockAssociationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VpcIpv6CidrBlockAssociationTimeoutsElRef {
        VpcIpv6CidrBlockAssociationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VpcIpv6CidrBlockAssociationTimeoutsElRef {
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
