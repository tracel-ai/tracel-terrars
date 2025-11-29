use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NatGatewayData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allocation_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connectivity_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_allocation_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_private_ip_address_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_private_ip_addresses: Option<SetField<PrimField<String>>>,
    subnet_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NatGatewayTimeoutsEl>,
}

struct NatGateway_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NatGatewayData>,
}

#[derive(Clone)]
pub struct NatGateway(Rc<NatGateway_>);

impl NatGateway {
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

    #[doc = "Set the field `allocation_id`.\n"]
    pub fn set_allocation_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().allocation_id = Some(v.into());
        self
    }

    #[doc = "Set the field `connectivity_type`.\n"]
    pub fn set_connectivity_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().connectivity_type = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `private_ip`.\n"]
    pub fn set_private_ip(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().private_ip = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `secondary_allocation_ids`.\n"]
    pub fn set_secondary_allocation_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().secondary_allocation_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `secondary_private_ip_address_count`.\n"]
    pub fn set_secondary_private_ip_address_count(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().secondary_private_ip_address_count = Some(v.into());
        self
    }

    #[doc = "Set the field `secondary_private_ip_addresses`.\n"]
    pub fn set_secondary_private_ip_addresses(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().secondary_private_ip_addresses = Some(v.into());
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
    pub fn set_timeouts(self, v: impl Into<NatGatewayTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `allocation_id` after provisioning.\n"]
    pub fn allocation_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.allocation_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `association_id` after provisioning.\n"]
    pub fn association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.association_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connectivity_type` after provisioning.\n"]
    pub fn connectivity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connectivity_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_interface_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_ip` after provisioning.\n"]
    pub fn private_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_ip` after provisioning.\n"]
    pub fn public_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secondary_allocation_ids` after provisioning.\n"]
    pub fn secondary_allocation_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.secondary_allocation_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secondary_private_ip_address_count` after provisioning.\n"]
    pub fn secondary_private_ip_address_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.secondary_private_ip_address_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secondary_private_ip_addresses` after provisioning.\n"]
    pub fn secondary_private_ip_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.secondary_private_ip_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NatGatewayTimeoutsElRef {
        NatGatewayTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for NatGateway {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for NatGateway { }

impl ToListMappable for NatGateway {
    type O = ListRef<NatGatewayRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for NatGateway_ {
    fn extract_resource_type(&self) -> String {
        "aws_nat_gateway".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNatGateway {
    pub tf_id: String,
    #[doc = ""]
    pub subnet_id: PrimField<String>,
}

impl BuildNatGateway {
    pub fn build(self, stack: &mut Stack) -> NatGateway {
        let out = NatGateway(Rc::new(NatGateway_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NatGatewayData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allocation_id: core::default::Default::default(),
                connectivity_type: core::default::Default::default(),
                id: core::default::Default::default(),
                private_ip: core::default::Default::default(),
                region: core::default::Default::default(),
                secondary_allocation_ids: core::default::Default::default(),
                secondary_private_ip_address_count: core::default::Default::default(),
                secondary_private_ip_addresses: core::default::Default::default(),
                subnet_id: self.subnet_id,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NatGatewayRef {
    shared: StackShared,
    base: String,
}

impl Ref for NatGatewayRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl NatGatewayRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocation_id` after provisioning.\n"]
    pub fn allocation_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.allocation_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `association_id` after provisioning.\n"]
    pub fn association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.association_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connectivity_type` after provisioning.\n"]
    pub fn connectivity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connectivity_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_interface_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_ip` after provisioning.\n"]
    pub fn private_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_ip` after provisioning.\n"]
    pub fn public_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secondary_allocation_ids` after provisioning.\n"]
    pub fn secondary_allocation_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.secondary_allocation_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secondary_private_ip_address_count` after provisioning.\n"]
    pub fn secondary_private_ip_address_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.secondary_private_ip_address_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secondary_private_ip_addresses` after provisioning.\n"]
    pub fn secondary_private_ip_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.secondary_private_ip_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NatGatewayTimeoutsElRef {
        NatGatewayTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct NatGatewayTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl NatGatewayTimeoutsEl {
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

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for NatGatewayTimeoutsEl {
    type O = BlockAssignable<NatGatewayTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNatGatewayTimeoutsEl {}

impl BuildNatGatewayTimeoutsEl {
    pub fn build(self) -> NatGatewayTimeoutsEl {
        NatGatewayTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct NatGatewayTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NatGatewayTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NatGatewayTimeoutsElRef {
        NatGatewayTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NatGatewayTimeoutsElRef {
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

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
