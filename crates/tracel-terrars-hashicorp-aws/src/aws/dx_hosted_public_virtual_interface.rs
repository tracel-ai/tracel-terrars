use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DxHostedPublicVirtualInterfaceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    address_family: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amazon_address: Option<PrimField<String>>,
    bgp_asn: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bgp_auth_key: Option<PrimField<String>>,
    connection_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    owner_account_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    route_filter_prefixes: SetField<PrimField<String>>,
    vlan: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DxHostedPublicVirtualInterfaceTimeoutsEl>,
}

struct DxHostedPublicVirtualInterface_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DxHostedPublicVirtualInterfaceData>,
}

#[derive(Clone)]
pub struct DxHostedPublicVirtualInterface(Rc<DxHostedPublicVirtualInterface_>);

impl DxHostedPublicVirtualInterface {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `amazon_address`.\n"]
    pub fn set_amazon_address(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().amazon_address = Some(v.into());
        self
    }

    #[doc = "Set the field `bgp_auth_key`.\n"]
    pub fn set_bgp_auth_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().bgp_auth_key = Some(v.into());
        self
    }

    #[doc = "Set the field `customer_address`.\n"]
    pub fn set_customer_address(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().customer_address = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DxHostedPublicVirtualInterfaceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `address_family` after provisioning.\n"]
    pub fn address_family(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `amazon_address` after provisioning.\n"]
    pub fn amazon_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.amazon_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `amazon_side_asn` after provisioning.\n"]
    pub fn amazon_side_asn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.amazon_side_asn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_device` after provisioning.\n"]
    pub fn aws_device(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aws_device", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgp_asn` after provisioning.\n"]
    pub fn bgp_asn(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.bgp_asn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgp_auth_key` after provisioning.\n"]
    pub fn bgp_auth_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bgp_auth_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_id` after provisioning.\n"]
    pub fn connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_address` after provisioning.\n"]
    pub fn customer_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_account_id` after provisioning.\n"]
    pub fn owner_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_filter_prefixes` after provisioning.\n"]
    pub fn route_filter_prefixes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.route_filter_prefixes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vlan` after provisioning.\n"]
    pub fn vlan(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vlan", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DxHostedPublicVirtualInterfaceTimeoutsElRef {
        DxHostedPublicVirtualInterfaceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for DxHostedPublicVirtualInterface {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DxHostedPublicVirtualInterface { }

impl ToListMappable for DxHostedPublicVirtualInterface {
    type O = ListRef<DxHostedPublicVirtualInterfaceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DxHostedPublicVirtualInterface_ {
    fn extract_resource_type(&self) -> String {
        "aws_dx_hosted_public_virtual_interface".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDxHostedPublicVirtualInterface {
    pub tf_id: String,
    #[doc = ""]
    pub address_family: PrimField<String>,
    #[doc = ""]
    pub bgp_asn: PrimField<f64>,
    #[doc = ""]
    pub connection_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub owner_account_id: PrimField<String>,
    #[doc = ""]
    pub route_filter_prefixes: SetField<PrimField<String>>,
    #[doc = ""]
    pub vlan: PrimField<f64>,
}

impl BuildDxHostedPublicVirtualInterface {
    pub fn build(self, stack: &mut Stack) -> DxHostedPublicVirtualInterface {
        let out = DxHostedPublicVirtualInterface(Rc::new(DxHostedPublicVirtualInterface_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DxHostedPublicVirtualInterfaceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                address_family: self.address_family,
                amazon_address: core::default::Default::default(),
                bgp_asn: self.bgp_asn,
                bgp_auth_key: core::default::Default::default(),
                connection_id: self.connection_id,
                customer_address: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                owner_account_id: self.owner_account_id,
                region: core::default::Default::default(),
                route_filter_prefixes: self.route_filter_prefixes,
                vlan: self.vlan,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DxHostedPublicVirtualInterfaceRef {
    shared: StackShared,
    base: String,
}

impl Ref for DxHostedPublicVirtualInterfaceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DxHostedPublicVirtualInterfaceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_family` after provisioning.\n"]
    pub fn address_family(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `amazon_address` after provisioning.\n"]
    pub fn amazon_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.amazon_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `amazon_side_asn` after provisioning.\n"]
    pub fn amazon_side_asn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.amazon_side_asn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_device` after provisioning.\n"]
    pub fn aws_device(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aws_device", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgp_asn` after provisioning.\n"]
    pub fn bgp_asn(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.bgp_asn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgp_auth_key` after provisioning.\n"]
    pub fn bgp_auth_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bgp_auth_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_id` after provisioning.\n"]
    pub fn connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_address` after provisioning.\n"]
    pub fn customer_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_account_id` after provisioning.\n"]
    pub fn owner_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_filter_prefixes` after provisioning.\n"]
    pub fn route_filter_prefixes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.route_filter_prefixes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vlan` after provisioning.\n"]
    pub fn vlan(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vlan", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DxHostedPublicVirtualInterfaceTimeoutsElRef {
        DxHostedPublicVirtualInterfaceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DxHostedPublicVirtualInterfaceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl DxHostedPublicVirtualInterfaceTimeoutsEl {
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

impl ToListMappable for DxHostedPublicVirtualInterfaceTimeoutsEl {
    type O = BlockAssignable<DxHostedPublicVirtualInterfaceTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDxHostedPublicVirtualInterfaceTimeoutsEl {}

impl BuildDxHostedPublicVirtualInterfaceTimeoutsEl {
    pub fn build(self) -> DxHostedPublicVirtualInterfaceTimeoutsEl {
        DxHostedPublicVirtualInterfaceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct DxHostedPublicVirtualInterfaceTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DxHostedPublicVirtualInterfaceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DxHostedPublicVirtualInterfaceTimeoutsElRef {
        DxHostedPublicVirtualInterfaceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DxHostedPublicVirtualInterfaceTimeoutsElRef {
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
