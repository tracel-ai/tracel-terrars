use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct VmwareengineSubnetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    ip_cidr_range: PrimField<String>,
    name: PrimField<String>,
    parent: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VmwareengineSubnetTimeoutsEl>,
}
struct VmwareengineSubnet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VmwareengineSubnetData>,
}
#[derive(Clone)]
pub struct VmwareengineSubnet(Rc<VmwareengineSubnet_>);
impl VmwareengineSubnet {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VmwareengineSubnetTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreation time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dhcp_address_ranges` after provisioning.\nDHCP address ranges."]
    pub fn dhcp_address_ranges(&self) -> ListRef<VmwareengineSubnetDhcpAddressRangesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dhcp_address_ranges", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gateway_id` after provisioning.\nThe canonical identifier of the logical router that this subnet is attached to."]
    pub fn gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gateway_ip` after provisioning.\nThe IP address of the gateway of this subnet. Must fall within the IP prefix defined above."]
    pub fn gateway_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_cidr_range` after provisioning.\nThe IP address range of the subnet in CIDR format."]
    pub fn ip_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_cidr_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the subnet. For userDefined subnets, this name should be in the format of \"service-n\",\nwhere n ranges from 1 to 5."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the private cloud to create a new subnet in.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `standard_config` after provisioning.\nWhether the NSX-T configuration in the backend follows the standard configuration supported by Google Cloud.\nIf false, the subnet cannot be modified through Google Cloud, only through NSX-T directly."]
    pub fn standard_config(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.standard_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the subnet."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the subnet."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem-generated unique identifier for the resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nLast updated time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine\nfractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vlan_id` after provisioning.\nVLAN ID of the VLAN on which the subnet is configured."]
    pub fn vlan_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vlan_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VmwareengineSubnetTimeoutsElRef {
        VmwareengineSubnetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for VmwareengineSubnet {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for VmwareengineSubnet {}
impl ToListMappable for VmwareengineSubnet {
    type O = ListRef<VmwareengineSubnetRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for VmwareengineSubnet_ {
    fn extract_resource_type(&self) -> String {
        "google_vmwareengine_subnet".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildVmwareengineSubnet {
    pub tf_id: String,
    #[doc = "The IP address range of the subnet in CIDR format."]
    pub ip_cidr_range: PrimField<String>,
    #[doc = "The ID of the subnet. For userDefined subnets, this name should be in the format of \"service-n\",\nwhere n ranges from 1 to 5."]
    pub name: PrimField<String>,
    #[doc = "The resource name of the private cloud to create a new subnet in.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud"]
    pub parent: PrimField<String>,
}
impl BuildVmwareengineSubnet {
    pub fn build(self, stack: &mut Stack) -> VmwareengineSubnet {
        let out = VmwareengineSubnet(Rc::new(VmwareengineSubnet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VmwareengineSubnetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                ip_cidr_range: self.ip_cidr_range,
                name: self.name,
                parent: self.parent,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct VmwareengineSubnetRef {
    shared: StackShared,
    base: String,
}
impl Ref for VmwareengineSubnetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl VmwareengineSubnetRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreation time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dhcp_address_ranges` after provisioning.\nDHCP address ranges."]
    pub fn dhcp_address_ranges(&self) -> ListRef<VmwareengineSubnetDhcpAddressRangesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dhcp_address_ranges", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gateway_id` after provisioning.\nThe canonical identifier of the logical router that this subnet is attached to."]
    pub fn gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gateway_ip` after provisioning.\nThe IP address of the gateway of this subnet. Must fall within the IP prefix defined above."]
    pub fn gateway_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_cidr_range` after provisioning.\nThe IP address range of the subnet in CIDR format."]
    pub fn ip_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_cidr_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the subnet. For userDefined subnets, this name should be in the format of \"service-n\",\nwhere n ranges from 1 to 5."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the private cloud to create a new subnet in.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `standard_config` after provisioning.\nWhether the NSX-T configuration in the backend follows the standard configuration supported by Google Cloud.\nIf false, the subnet cannot be modified through Google Cloud, only through NSX-T directly."]
    pub fn standard_config(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.standard_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the subnet."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the subnet."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem-generated unique identifier for the resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nLast updated time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine\nfractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vlan_id` after provisioning.\nVLAN ID of the VLAN on which the subnet is configured."]
    pub fn vlan_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vlan_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VmwareengineSubnetTimeoutsElRef {
        VmwareengineSubnetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct VmwareengineSubnetDhcpAddressRangesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    first_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_address: Option<PrimField<String>>,
}
impl VmwareengineSubnetDhcpAddressRangesEl {
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
impl ToListMappable for VmwareengineSubnetDhcpAddressRangesEl {
    type O = BlockAssignable<VmwareengineSubnetDhcpAddressRangesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVmwareengineSubnetDhcpAddressRangesEl {}
impl BuildVmwareengineSubnetDhcpAddressRangesEl {
    pub fn build(self) -> VmwareengineSubnetDhcpAddressRangesEl {
        VmwareengineSubnetDhcpAddressRangesEl {
            first_address: core::default::Default::default(),
            last_address: core::default::Default::default(),
        }
    }
}
pub struct VmwareengineSubnetDhcpAddressRangesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VmwareengineSubnetDhcpAddressRangesElRef {
    fn new(shared: StackShared, base: String) -> VmwareengineSubnetDhcpAddressRangesElRef {
        VmwareengineSubnetDhcpAddressRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VmwareengineSubnetDhcpAddressRangesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
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
#[derive(Serialize)]
pub struct VmwareengineSubnetTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl VmwareengineSubnetTimeoutsEl {
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
impl ToListMappable for VmwareengineSubnetTimeoutsEl {
    type O = BlockAssignable<VmwareengineSubnetTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVmwareengineSubnetTimeoutsEl {}
impl BuildVmwareengineSubnetTimeoutsEl {
    pub fn build(self) -> VmwareengineSubnetTimeoutsEl {
        VmwareengineSubnetTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct VmwareengineSubnetTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VmwareengineSubnetTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VmwareengineSubnetTimeoutsElRef {
        VmwareengineSubnetTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VmwareengineSubnetTimeoutsElRef {
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
