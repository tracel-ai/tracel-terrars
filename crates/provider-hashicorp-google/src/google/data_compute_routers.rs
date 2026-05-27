use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeRoutersData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataComputeRouters_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeRoutersData>,
}
#[derive(Clone)]
pub struct DataComputeRouters(Rc<DataComputeRouters_>);
impl DataComputeRouters {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `routers` after provisioning.\n"]
    pub fn routers(&self) -> ListRef<DataComputeRoutersRoutersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.routers", self.extract_ref()),
        )
    }
}
impl Referable for DataComputeRouters {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeRouters {}
impl ToListMappable for DataComputeRouters {
    type O = ListRef<DataComputeRoutersRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeRouters_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_routers".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeRouters {
    pub tf_id: String,
}
impl BuildDataComputeRouters {
    pub fn build(self, stack: &mut Stack) -> DataComputeRouters {
        let out = DataComputeRouters(Rc::new(DataComputeRouters_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeRoutersData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeRoutersRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRoutersRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeRoutersRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `routers` after provisioning.\n"]
    pub fn routers(&self) -> ListRef<DataComputeRoutersRoutersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.routers", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRoutersRoutersElBgpElAdvertisedIpRangesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<PrimField<String>>,
}
impl DataComputeRoutersRoutersElBgpElAdvertisedIpRangesEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `range`.\n"]
    pub fn set_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.range = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRoutersRoutersElBgpElAdvertisedIpRangesEl {
    type O = BlockAssignable<DataComputeRoutersRoutersElBgpElAdvertisedIpRangesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRoutersRoutersElBgpElAdvertisedIpRangesEl {}
impl BuildDataComputeRoutersRoutersElBgpElAdvertisedIpRangesEl {
    pub fn build(self) -> DataComputeRoutersRoutersElBgpElAdvertisedIpRangesEl {
        DataComputeRoutersRoutersElBgpElAdvertisedIpRangesEl {
            description: core::default::Default::default(),
            range: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRoutersRoutersElBgpElAdvertisedIpRangesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRoutersRoutersElBgpElAdvertisedIpRangesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRoutersRoutersElBgpElAdvertisedIpRangesElRef {
        DataComputeRoutersRoutersElBgpElAdvertisedIpRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRoutersRoutersElBgpElAdvertisedIpRangesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `range` after provisioning.\n"]
    pub fn range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.range", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRoutersRoutersElBgpEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    advertise_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advertised_groups: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advertised_ip_ranges: Option<ListField<DataComputeRoutersRoutersElBgpElAdvertisedIpRangesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    asn: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keepalive_interval: Option<PrimField<f64>>,
}
impl DataComputeRoutersRoutersElBgpEl {
    #[doc = "Set the field `advertise_mode`.\n"]
    pub fn set_advertise_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.advertise_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `advertised_groups`.\n"]
    pub fn set_advertised_groups(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.advertised_groups = Some(v.into());
        self
    }
    #[doc = "Set the field `advertised_ip_ranges`.\n"]
    pub fn set_advertised_ip_ranges(
        mut self,
        v: impl Into<ListField<DataComputeRoutersRoutersElBgpElAdvertisedIpRangesEl>>,
    ) -> Self {
        self.advertised_ip_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `asn`.\n"]
    pub fn set_asn(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.asn = Some(v.into());
        self
    }
    #[doc = "Set the field `keepalive_interval`.\n"]
    pub fn set_keepalive_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.keepalive_interval = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRoutersRoutersElBgpEl {
    type O = BlockAssignable<DataComputeRoutersRoutersElBgpEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRoutersRoutersElBgpEl {}
impl BuildDataComputeRoutersRoutersElBgpEl {
    pub fn build(self) -> DataComputeRoutersRoutersElBgpEl {
        DataComputeRoutersRoutersElBgpEl {
            advertise_mode: core::default::Default::default(),
            advertised_groups: core::default::Default::default(),
            advertised_ip_ranges: core::default::Default::default(),
            asn: core::default::Default::default(),
            keepalive_interval: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRoutersRoutersElBgpElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRoutersRoutersElBgpElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRoutersRoutersElBgpElRef {
        DataComputeRoutersRoutersElBgpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRoutersRoutersElBgpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `advertise_mode` after provisioning.\n"]
    pub fn advertise_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.advertise_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `advertised_groups` after provisioning.\n"]
    pub fn advertised_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advertised_groups", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `advertised_ip_ranges` after provisioning.\n"]
    pub fn advertised_ip_ranges(
        &self,
    ) -> ListRef<DataComputeRoutersRoutersElBgpElAdvertisedIpRangesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advertised_ip_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `asn` after provisioning.\n"]
    pub fn asn(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.asn", self.base))
    }
    #[doc = "Get a reference to the value of field `keepalive_interval` after provisioning.\n"]
    pub fn keepalive_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.keepalive_interval", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRoutersRoutersElBgpPeersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    advertise_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advertised_route_priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_ipv6: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interface_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    management_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_asn: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_ip_address: Option<PrimField<String>>,
}
impl DataComputeRoutersRoutersElBgpPeersEl {
    #[doc = "Set the field `advertise_mode`.\n"]
    pub fn set_advertise_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.advertise_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `advertised_route_priority`.\n"]
    pub fn set_advertised_route_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.advertised_route_priority = Some(v.into());
        self
    }
    #[doc = "Set the field `enable`.\n"]
    pub fn set_enable(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enable = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_ipv6`.\n"]
    pub fn set_enable_ipv6(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_ipv6 = Some(v.into());
        self
    }
    #[doc = "Set the field `interface_name`.\n"]
    pub fn set_interface_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interface_name = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address = Some(v.into());
        self
    }
    #[doc = "Set the field `management_type`.\n"]
    pub fn set_management_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.management_type = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `peer_asn`.\n"]
    pub fn set_peer_asn(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.peer_asn = Some(v.into());
        self
    }
    #[doc = "Set the field `peer_ip_address`.\n"]
    pub fn set_peer_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.peer_ip_address = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRoutersRoutersElBgpPeersEl {
    type O = BlockAssignable<DataComputeRoutersRoutersElBgpPeersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRoutersRoutersElBgpPeersEl {}
impl BuildDataComputeRoutersRoutersElBgpPeersEl {
    pub fn build(self) -> DataComputeRoutersRoutersElBgpPeersEl {
        DataComputeRoutersRoutersElBgpPeersEl {
            advertise_mode: core::default::Default::default(),
            advertised_route_priority: core::default::Default::default(),
            enable: core::default::Default::default(),
            enable_ipv6: core::default::Default::default(),
            interface_name: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            management_type: core::default::Default::default(),
            name: core::default::Default::default(),
            peer_asn: core::default::Default::default(),
            peer_ip_address: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRoutersRoutersElBgpPeersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRoutersRoutersElBgpPeersElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRoutersRoutersElBgpPeersElRef {
        DataComputeRoutersRoutersElBgpPeersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRoutersRoutersElBgpPeersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `advertise_mode` after provisioning.\n"]
    pub fn advertise_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.advertise_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `advertised_route_priority` after provisioning.\n"]
    pub fn advertised_route_priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.advertised_route_priority", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }
    #[doc = "Get a reference to the value of field `enable_ipv6` after provisioning.\n"]
    pub fn enable_ipv6(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_ipv6", self.base))
    }
    #[doc = "Get a reference to the value of field `interface_name` after provisioning.\n"]
    pub fn interface_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.interface_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
    #[doc = "Get a reference to the value of field `management_type` after provisioning.\n"]
    pub fn management_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.management_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `peer_asn` after provisioning.\n"]
    pub fn peer_asn(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_asn", self.base))
    }
    #[doc = "Get a reference to the value of field `peer_ip_address` after provisioning.\n"]
    pub fn peer_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_ip_address", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRoutersRoutersElInterfacesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    linked_interconnect_attachment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    linked_vpn_tunnel: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redundant_interface: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
}
impl DataComputeRoutersRoutersElInterfacesEl {
    #[doc = "Set the field `ip_range`.\n"]
    pub fn set_ip_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_range = Some(v.into());
        self
    }
    #[doc = "Set the field `linked_interconnect_attachment`.\n"]
    pub fn set_linked_interconnect_attachment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.linked_interconnect_attachment = Some(v.into());
        self
    }
    #[doc = "Set the field `linked_vpn_tunnel`.\n"]
    pub fn set_linked_vpn_tunnel(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.linked_vpn_tunnel = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `private_ip_address`.\n"]
    pub fn set_private_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_ip_address = Some(v.into());
        self
    }
    #[doc = "Set the field `redundant_interface`.\n"]
    pub fn set_redundant_interface(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.redundant_interface = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\n"]
    pub fn set_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRoutersRoutersElInterfacesEl {
    type O = BlockAssignable<DataComputeRoutersRoutersElInterfacesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRoutersRoutersElInterfacesEl {}
impl BuildDataComputeRoutersRoutersElInterfacesEl {
    pub fn build(self) -> DataComputeRoutersRoutersElInterfacesEl {
        DataComputeRoutersRoutersElInterfacesEl {
            ip_range: core::default::Default::default(),
            linked_interconnect_attachment: core::default::Default::default(),
            linked_vpn_tunnel: core::default::Default::default(),
            name: core::default::Default::default(),
            private_ip_address: core::default::Default::default(),
            redundant_interface: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRoutersRoutersElInterfacesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRoutersRoutersElInterfacesElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRoutersRoutersElInterfacesElRef {
        DataComputeRoutersRoutersElInterfacesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRoutersRoutersElInterfacesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_range` after provisioning.\n"]
    pub fn ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_range", self.base))
    }
    #[doc = "Get a reference to the value of field `linked_interconnect_attachment` after provisioning.\n"]
    pub fn linked_interconnect_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.linked_interconnect_attachment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `linked_vpn_tunnel` after provisioning.\n"]
    pub fn linked_vpn_tunnel(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.linked_vpn_tunnel", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `private_ip_address` after provisioning.\n"]
    pub fn private_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_ip_address", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `redundant_interface` after provisioning.\n"]
    pub fn redundant_interface(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.redundant_interface", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\n"]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRoutersRoutersElMd5AuthenticationKeysEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataComputeRoutersRoutersElMd5AuthenticationKeysEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRoutersRoutersElMd5AuthenticationKeysEl {
    type O = BlockAssignable<DataComputeRoutersRoutersElMd5AuthenticationKeysEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRoutersRoutersElMd5AuthenticationKeysEl {}
impl BuildDataComputeRoutersRoutersElMd5AuthenticationKeysEl {
    pub fn build(self) -> DataComputeRoutersRoutersElMd5AuthenticationKeysEl {
        DataComputeRoutersRoutersElMd5AuthenticationKeysEl {
            key: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRoutersRoutersElMd5AuthenticationKeysElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRoutersRoutersElMd5AuthenticationKeysElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRoutersRoutersElMd5AuthenticationKeysElRef {
        DataComputeRoutersRoutersElMd5AuthenticationKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRoutersRoutersElMd5AuthenticationKeysElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRoutersRoutersElNatsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_endpoint_independent_mapping: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icmp_idle_timeout_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_ports_per_vm: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nat_ip_allocate_option: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nat_ips: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_subnetwork_ip_ranges_to_nat: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_established_idle_timeout_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_transitory_idle_timeout_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    udp_idle_timeout_sec: Option<PrimField<f64>>,
}
impl DataComputeRoutersRoutersElNatsEl {
    #[doc = "Set the field `enable_endpoint_independent_mapping`.\n"]
    pub fn set_enable_endpoint_independent_mapping(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.enable_endpoint_independent_mapping = Some(v.into());
        self
    }
    #[doc = "Set the field `icmp_idle_timeout_sec`.\n"]
    pub fn set_icmp_idle_timeout_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.icmp_idle_timeout_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `min_ports_per_vm`.\n"]
    pub fn set_min_ports_per_vm(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_ports_per_vm = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `nat_ip_allocate_option`.\n"]
    pub fn set_nat_ip_allocate_option(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nat_ip_allocate_option = Some(v.into());
        self
    }
    #[doc = "Set the field `nat_ips`.\n"]
    pub fn set_nat_ips(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.nat_ips = Some(v.into());
        self
    }
    #[doc = "Set the field `source_subnetwork_ip_ranges_to_nat`.\n"]
    pub fn set_source_subnetwork_ip_ranges_to_nat(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.source_subnetwork_ip_ranges_to_nat = Some(v.into());
        self
    }
    #[doc = "Set the field `tcp_established_idle_timeout_sec`.\n"]
    pub fn set_tcp_established_idle_timeout_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.tcp_established_idle_timeout_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `tcp_transitory_idle_timeout_sec`.\n"]
    pub fn set_tcp_transitory_idle_timeout_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.tcp_transitory_idle_timeout_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `udp_idle_timeout_sec`.\n"]
    pub fn set_udp_idle_timeout_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.udp_idle_timeout_sec = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRoutersRoutersElNatsEl {
    type O = BlockAssignable<DataComputeRoutersRoutersElNatsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRoutersRoutersElNatsEl {}
impl BuildDataComputeRoutersRoutersElNatsEl {
    pub fn build(self) -> DataComputeRoutersRoutersElNatsEl {
        DataComputeRoutersRoutersElNatsEl {
            enable_endpoint_independent_mapping: core::default::Default::default(),
            icmp_idle_timeout_sec: core::default::Default::default(),
            min_ports_per_vm: core::default::Default::default(),
            name: core::default::Default::default(),
            nat_ip_allocate_option: core::default::Default::default(),
            nat_ips: core::default::Default::default(),
            source_subnetwork_ip_ranges_to_nat: core::default::Default::default(),
            tcp_established_idle_timeout_sec: core::default::Default::default(),
            tcp_transitory_idle_timeout_sec: core::default::Default::default(),
            udp_idle_timeout_sec: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRoutersRoutersElNatsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRoutersRoutersElNatsElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRoutersRoutersElNatsElRef {
        DataComputeRoutersRoutersElNatsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRoutersRoutersElNatsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_endpoint_independent_mapping` after provisioning.\n"]
    pub fn enable_endpoint_independent_mapping(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_endpoint_independent_mapping", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `icmp_idle_timeout_sec` after provisioning.\n"]
    pub fn icmp_idle_timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.icmp_idle_timeout_sec", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_ports_per_vm` after provisioning.\n"]
    pub fn min_ports_per_vm(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_ports_per_vm", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `nat_ip_allocate_option` after provisioning.\n"]
    pub fn nat_ip_allocate_option(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nat_ip_allocate_option", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nat_ips` after provisioning.\n"]
    pub fn nat_ips(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.nat_ips", self.base))
    }
    #[doc = "Get a reference to the value of field `source_subnetwork_ip_ranges_to_nat` after provisioning.\n"]
    pub fn source_subnetwork_ip_ranges_to_nat(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_subnetwork_ip_ranges_to_nat", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tcp_established_idle_timeout_sec` after provisioning.\n"]
    pub fn tcp_established_idle_timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tcp_established_idle_timeout_sec", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tcp_transitory_idle_timeout_sec` after provisioning.\n"]
    pub fn tcp_transitory_idle_timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tcp_transitory_idle_timeout_sec", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `udp_idle_timeout_sec` after provisioning.\n"]
    pub fn udp_idle_timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.udp_idle_timeout_sec", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRoutersRoutersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bgp: Option<ListField<DataComputeRoutersRoutersElBgpEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bgp_peers: Option<ListField<DataComputeRoutersRoutersElBgpPeersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creation_timestamp: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encrypted_interconnect_router: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interfaces: Option<ListField<DataComputeRoutersRoutersElInterfacesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    md5_authentication_keys: Option<ListField<DataComputeRoutersRoutersElMd5AuthenticationKeysEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nats: Option<ListField<DataComputeRoutersRoutersElNatsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_link: Option<PrimField<String>>,
}
impl DataComputeRoutersRoutersEl {
    #[doc = "Set the field `bgp`.\n"]
    pub fn set_bgp(mut self, v: impl Into<ListField<DataComputeRoutersRoutersElBgpEl>>) -> Self {
        self.bgp = Some(v.into());
        self
    }
    #[doc = "Set the field `bgp_peers`.\n"]
    pub fn set_bgp_peers(
        mut self,
        v: impl Into<ListField<DataComputeRoutersRoutersElBgpPeersEl>>,
    ) -> Self {
        self.bgp_peers = Some(v.into());
        self
    }
    #[doc = "Set the field `creation_timestamp`.\n"]
    pub fn set_creation_timestamp(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.creation_timestamp = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `encrypted_interconnect_router`.\n"]
    pub fn set_encrypted_interconnect_router(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.encrypted_interconnect_router = Some(v.into());
        self
    }
    #[doc = "Set the field `interfaces`.\n"]
    pub fn set_interfaces(
        mut self,
        v: impl Into<ListField<DataComputeRoutersRoutersElInterfacesEl>>,
    ) -> Self {
        self.interfaces = Some(v.into());
        self
    }
    #[doc = "Set the field `md5_authentication_keys`.\n"]
    pub fn set_md5_authentication_keys(
        mut self,
        v: impl Into<ListField<DataComputeRoutersRoutersElMd5AuthenticationKeysEl>>,
    ) -> Self {
        self.md5_authentication_keys = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `nats`.\n"]
    pub fn set_nats(mut self, v: impl Into<ListField<DataComputeRoutersRoutersElNatsEl>>) -> Self {
        self.nats = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `self_link`.\n"]
    pub fn set_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.self_link = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRoutersRoutersEl {
    type O = BlockAssignable<DataComputeRoutersRoutersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRoutersRoutersEl {}
impl BuildDataComputeRoutersRoutersEl {
    pub fn build(self) -> DataComputeRoutersRoutersEl {
        DataComputeRoutersRoutersEl {
            bgp: core::default::Default::default(),
            bgp_peers: core::default::Default::default(),
            creation_timestamp: core::default::Default::default(),
            description: core::default::Default::default(),
            encrypted_interconnect_router: core::default::Default::default(),
            interfaces: core::default::Default::default(),
            md5_authentication_keys: core::default::Default::default(),
            name: core::default::Default::default(),
            nats: core::default::Default::default(),
            network: core::default::Default::default(),
            self_link: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRoutersRoutersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRoutersRoutersElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRoutersRoutersElRef {
        DataComputeRoutersRoutersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRoutersRoutersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bgp` after provisioning.\n"]
    pub fn bgp(&self) -> ListRef<DataComputeRoutersRoutersElBgpElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bgp", self.base))
    }
    #[doc = "Get a reference to the value of field `bgp_peers` after provisioning.\n"]
    pub fn bgp_peers(&self) -> ListRef<DataComputeRoutersRoutersElBgpPeersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bgp_peers", self.base))
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\n"]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `encrypted_interconnect_router` after provisioning.\n"]
    pub fn encrypted_interconnect_router(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encrypted_interconnect_router", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `interfaces` after provisioning.\n"]
    pub fn interfaces(&self) -> ListRef<DataComputeRoutersRoutersElInterfacesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.interfaces", self.base))
    }
    #[doc = "Get a reference to the value of field `md5_authentication_keys` after provisioning.\n"]
    pub fn md5_authentication_keys(
        &self,
    ) -> ListRef<DataComputeRoutersRoutersElMd5AuthenticationKeysElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.md5_authentication_keys", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `nats` after provisioning.\n"]
    pub fn nats(&self) -> ListRef<DataComputeRoutersRoutersElNatsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.nats", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }
}
