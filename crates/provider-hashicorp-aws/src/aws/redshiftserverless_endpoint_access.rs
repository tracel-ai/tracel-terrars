use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct RedshiftserverlessEndpointAccessData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    endpoint_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    subnet_ids: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_security_group_ids: Option<SetField<PrimField<String>>>,
    workgroup_name: PrimField<String>,
}

struct RedshiftserverlessEndpointAccess_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RedshiftserverlessEndpointAccessData>,
}

#[derive(Clone)]
pub struct RedshiftserverlessEndpointAccess(Rc<RedshiftserverlessEndpointAccess_>);

impl RedshiftserverlessEndpointAccess {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `owner_account`.\n"]
    pub fn set_owner_account(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().owner_account = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_security_group_ids`.\n"]
    pub fn set_vpc_security_group_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().vpc_security_group_ids = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_name` after provisioning.\n"]
    pub fn endpoint_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_account` after provisioning.\n"]
    pub fn owner_account(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_account", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint` after provisioning.\n"]
    pub fn vpc_endpoint(&self) -> ListRef<RedshiftserverlessEndpointAccessVpcEndpointElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workgroup_name` after provisioning.\n"]
    pub fn workgroup_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workgroup_name", self.extract_ref()))
    }
}

impl Referable for RedshiftserverlessEndpointAccess {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for RedshiftserverlessEndpointAccess { }

impl ToListMappable for RedshiftserverlessEndpointAccess {
    type O = ListRef<RedshiftserverlessEndpointAccessRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for RedshiftserverlessEndpointAccess_ {
    fn extract_resource_type(&self) -> String {
        "aws_redshiftserverless_endpoint_access".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRedshiftserverlessEndpointAccess {
    pub tf_id: String,
    #[doc = ""]
    pub endpoint_name: PrimField<String>,
    #[doc = ""]
    pub subnet_ids: SetField<PrimField<String>>,
    #[doc = ""]
    pub workgroup_name: PrimField<String>,
}

impl BuildRedshiftserverlessEndpointAccess {
    pub fn build(self, stack: &mut Stack) -> RedshiftserverlessEndpointAccess {
        let out = RedshiftserverlessEndpointAccess(Rc::new(RedshiftserverlessEndpointAccess_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RedshiftserverlessEndpointAccessData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                endpoint_name: self.endpoint_name,
                id: core::default::Default::default(),
                owner_account: core::default::Default::default(),
                region: core::default::Default::default(),
                subnet_ids: self.subnet_ids,
                vpc_security_group_ids: core::default::Default::default(),
                workgroup_name: self.workgroup_name,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RedshiftserverlessEndpointAccessRef {
    shared: StackShared,
    base: String,
}

impl Ref for RedshiftserverlessEndpointAccessRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl RedshiftserverlessEndpointAccessRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_name` after provisioning.\n"]
    pub fn endpoint_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_account` after provisioning.\n"]
    pub fn owner_account(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_account", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint` after provisioning.\n"]
    pub fn vpc_endpoint(&self) -> ListRef<RedshiftserverlessEndpointAccessVpcEndpointElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workgroup_name` after provisioning.\n"]
    pub fn workgroup_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workgroup_name", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RedshiftserverlessEndpointAccessVpcEndpointElNetworkInterfaceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interface_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_id: Option<PrimField<String>>,
}

impl RedshiftserverlessEndpointAccessVpcEndpointElNetworkInterfaceEl {
    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.availability_zone = Some(v.into());
        self
    }

    #[doc = "Set the field `network_interface_id`.\n"]
    pub fn set_network_interface_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_interface_id = Some(v.into());
        self
    }

    #[doc = "Set the field `private_ip_address`.\n"]
    pub fn set_private_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_ip_address = Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnet_id = Some(v.into());
        self
    }
}

impl ToListMappable for RedshiftserverlessEndpointAccessVpcEndpointElNetworkInterfaceEl {
    type O = BlockAssignable<RedshiftserverlessEndpointAccessVpcEndpointElNetworkInterfaceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftserverlessEndpointAccessVpcEndpointElNetworkInterfaceEl {}

impl BuildRedshiftserverlessEndpointAccessVpcEndpointElNetworkInterfaceEl {
    pub fn build(self) -> RedshiftserverlessEndpointAccessVpcEndpointElNetworkInterfaceEl {
        RedshiftserverlessEndpointAccessVpcEndpointElNetworkInterfaceEl {
            availability_zone: core::default::Default::default(),
            network_interface_id: core::default::Default::default(),
            private_ip_address: core::default::Default::default(),
            subnet_id: core::default::Default::default(),
        }
    }
}

pub struct RedshiftserverlessEndpointAccessVpcEndpointElNetworkInterfaceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RedshiftserverlessEndpointAccessVpcEndpointElNetworkInterfaceElRef {
    fn new(shared: StackShared, base: String) -> RedshiftserverlessEndpointAccessVpcEndpointElNetworkInterfaceElRef {
        RedshiftserverlessEndpointAccessVpcEndpointElNetworkInterfaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftserverlessEndpointAccessVpcEndpointElNetworkInterfaceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_interface_id", self.base))
    }

    #[doc = "Get a reference to the value of field `private_ip_address` after provisioning.\n"]
    pub fn private_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
}

#[derive(Serialize)]
pub struct RedshiftserverlessEndpointAccessVpcEndpointEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interface: Option<ListField<RedshiftserverlessEndpointAccessVpcEndpointElNetworkInterfaceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_endpoint_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_id: Option<PrimField<String>>,
}

impl RedshiftserverlessEndpointAccessVpcEndpointEl {
    #[doc = "Set the field `network_interface`.\n"]
    pub fn set_network_interface(
        mut self,
        v: impl Into<ListField<RedshiftserverlessEndpointAccessVpcEndpointElNetworkInterfaceEl>>,
    ) -> Self {
        self.network_interface = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_endpoint_id`.\n"]
    pub fn set_vpc_endpoint_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_endpoint_id = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_id = Some(v.into());
        self
    }
}

impl ToListMappable for RedshiftserverlessEndpointAccessVpcEndpointEl {
    type O = BlockAssignable<RedshiftserverlessEndpointAccessVpcEndpointEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftserverlessEndpointAccessVpcEndpointEl {}

impl BuildRedshiftserverlessEndpointAccessVpcEndpointEl {
    pub fn build(self) -> RedshiftserverlessEndpointAccessVpcEndpointEl {
        RedshiftserverlessEndpointAccessVpcEndpointEl {
            network_interface: core::default::Default::default(),
            vpc_endpoint_id: core::default::Default::default(),
            vpc_id: core::default::Default::default(),
        }
    }
}

pub struct RedshiftserverlessEndpointAccessVpcEndpointElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RedshiftserverlessEndpointAccessVpcEndpointElRef {
    fn new(shared: StackShared, base: String) -> RedshiftserverlessEndpointAccessVpcEndpointElRef {
        RedshiftserverlessEndpointAccessVpcEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftserverlessEndpointAccessVpcEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_interface` after provisioning.\n"]
    pub fn network_interface(&self) -> ListRef<RedshiftserverlessEndpointAccessVpcEndpointElNetworkInterfaceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_interface", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_id` after provisioning.\n"]
    pub fn vpc_endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_id", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}
