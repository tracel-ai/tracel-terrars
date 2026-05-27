use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeRouterPeerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advertise_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advertised_groups: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advertised_route_priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_learned_route_priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_ipv4: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_ipv6: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    export_policies: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    import_policies: Option<ListField<PrimField<String>>>,
    interface: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv4_nexthop_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_nexthop_address: Option<PrimField<String>>,
    name: PrimField<String>,
    peer_asn: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_ipv4_nexthop_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_ipv6_nexthop_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    router: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    router_appliance_instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zero_advertised_route_priority: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zero_custom_learned_route_priority: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advertised_ip_ranges: Option<Vec<ComputeRouterPeerAdvertisedIpRangesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bfd: Option<Vec<ComputeRouterPeerBfdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_learned_ip_ranges: Option<Vec<ComputeRouterPeerCustomLearnedIpRangesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    md5_authentication_key: Option<Vec<ComputeRouterPeerMd5AuthenticationKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeRouterPeerTimeoutsEl>,
    dynamic: ComputeRouterPeerDynamic,
}
struct ComputeRouterPeer_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeRouterPeerData>,
}
#[derive(Clone)]
pub struct ComputeRouterPeer(Rc<ComputeRouterPeer_>);
impl ComputeRouterPeer {
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
    #[doc = "Set the field `advertise_mode`.\nUser-specified flag to indicate which mode to use for advertisement.\nValid values of this enum field are: 'DEFAULT', 'CUSTOM' Default value: \"DEFAULT\" Possible values: [\"DEFAULT\", \"CUSTOM\"]"]
    pub fn set_advertise_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().advertise_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `advertised_groups`.\nUser-specified list of prefix groups to advertise in custom\nmode, which currently supports the following option:\n\n* 'ALL_SUBNETS': Advertises all of the router's own VPC subnets.\nThis excludes any routes learned for subnets that use VPC Network\nPeering.\n\n\nNote that this field can only be populated if advertiseMode is 'CUSTOM'\nand overrides the list defined for the router (in the \"bgp\" message).\nThese groups are advertised in addition to any specified prefixes.\nLeave this field blank to advertise no custom groups."]
    pub fn set_advertised_groups(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().advertised_groups = Some(v.into());
        self
    }
    #[doc = "Set the field `advertised_route_priority`.\nThe priority of routes advertised to this BGP peer.\nWhere there is more than one matching route of maximum\nlength, the routes with the lowest priority value win."]
    pub fn set_advertised_route_priority(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().advertised_route_priority = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_learned_route_priority`.\nThe user-defined custom learned route priority for a BGP session.\nThis value is applied to all custom learned route ranges for the session. You can choose a value\nfrom 0 to 65335. If you don't provide a value, Google Cloud assigns a priority of 100 to the ranges."]
    pub fn set_custom_learned_route_priority(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().custom_learned_route_priority = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `enable`.\nThe status of the BGP peer connection. If set to false, any active session\nwith the peer is terminated and all associated routing information is removed.\nIf set to true, the peer connection can be established with routing information.\nThe default is true."]
    pub fn set_enable(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_ipv4`.\nEnable IPv4 traffic over BGP Peer. It is enabled by default if the peerIpAddress is version 4."]
    pub fn set_enable_ipv4(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_ipv4 = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_ipv6`.\nEnable IPv6 traffic over BGP Peer. If not specified, it is disabled by default."]
    pub fn set_enable_ipv6(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_ipv6 = Some(v.into());
        self
    }
    #[doc = "Set the field `export_policies`.\nrouters.list of export policies applied to this peer, in the order they must be evaluated. \nThe name must correspond to an existing policy that has ROUTE_POLICY_TYPE_EXPORT type."]
    pub fn set_export_policies(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().export_policies = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `import_policies`.\nrouters.list of import policies applied to this peer, in the order they must be evaluated. \nThe name must correspond to an existing policy that has ROUTE_POLICY_TYPE_IMPORT type."]
    pub fn set_import_policies(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().import_policies = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_address`.\nIP address of the interface inside Google Cloud Platform.\nOnly IPv4 is supported."]
    pub fn set_ip_address(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ip_address = Some(v.into());
        self
    }
    #[doc = "Set the field `ipv4_nexthop_address`.\nIPv4 address of the interface inside Google Cloud Platform."]
    pub fn set_ipv4_nexthop_address(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ipv4_nexthop_address = Some(v.into());
        self
    }
    #[doc = "Set the field `ipv6_nexthop_address`.\nIPv6 address of the interface inside Google Cloud Platform.\nThe address must be in the range 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64.\nIf you do not specify the next hop addresses, Google Cloud automatically\nassigns unused addresses from the 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64 range for you."]
    pub fn set_ipv6_nexthop_address(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ipv6_nexthop_address = Some(v.into());
        self
    }
    #[doc = "Set the field `peer_ip_address`.\nIP address of the BGP interface outside Google Cloud Platform.\nOnly IPv4 is supported. Required if 'ip_address' is set."]
    pub fn set_peer_ip_address(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().peer_ip_address = Some(v.into());
        self
    }
    #[doc = "Set the field `peer_ipv4_nexthop_address`.\nIPv4 address of the BGP interface outside Google Cloud Platform."]
    pub fn set_peer_ipv4_nexthop_address(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().peer_ipv4_nexthop_address = Some(v.into());
        self
    }
    #[doc = "Set the field `peer_ipv6_nexthop_address`.\nIPv6 address of the BGP interface outside Google Cloud Platform.\nThe address must be in the range 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64.\nIf you do not specify the next hop addresses, Google Cloud automatically\nassigns unused addresses from the 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64 range for you."]
    pub fn set_peer_ipv6_nexthop_address(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().peer_ipv6_nexthop_address = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where the router and BgpPeer reside.\nIf it is not provided, the provider region is used."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `router_appliance_instance`.\nThe URI of the VM instance that is used as third-party router appliances\nsuch as Next Gen Firewalls, Virtual Routers, or Router Appliances.\nThe VM instance must be located in zones contained in the same region as\nthis Cloud Router. The VM instance is the peer side of the BGP session."]
    pub fn set_router_appliance_instance(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().router_appliance_instance = Some(v.into());
        self
    }
    #[doc = "Set the field `zero_advertised_route_priority`.\nForce the advertised_route_priority to be 0."]
    pub fn set_zero_advertised_route_priority(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().zero_advertised_route_priority = Some(v.into());
        self
    }
    #[doc = "Set the field `zero_custom_learned_route_priority`.\nForce the custom_learned_route_priority to be 0."]
    pub fn set_zero_custom_learned_route_priority(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().zero_custom_learned_route_priority = Some(v.into());
        self
    }
    #[doc = "Set the field `advertised_ip_ranges`.\n"]
    pub fn set_advertised_ip_ranges(
        self,
        v: impl Into<BlockAssignable<ComputeRouterPeerAdvertisedIpRangesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().advertised_ip_ranges = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.advertised_ip_ranges = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `bfd`.\n"]
    pub fn set_bfd(self, v: impl Into<BlockAssignable<ComputeRouterPeerBfdEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bfd = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bfd = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `custom_learned_ip_ranges`.\n"]
    pub fn set_custom_learned_ip_ranges(
        self,
        v: impl Into<BlockAssignable<ComputeRouterPeerCustomLearnedIpRangesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_learned_ip_ranges = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_learned_ip_ranges = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `md5_authentication_key`.\n"]
    pub fn set_md5_authentication_key(
        self,
        v: impl Into<BlockAssignable<ComputeRouterPeerMd5AuthenticationKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().md5_authentication_key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.md5_authentication_key = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeRouterPeerTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `advertise_mode` after provisioning.\nUser-specified flag to indicate which mode to use for advertisement.\nValid values of this enum field are: 'DEFAULT', 'CUSTOM' Default value: \"DEFAULT\" Possible values: [\"DEFAULT\", \"CUSTOM\"]"]
    pub fn advertise_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.advertise_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `advertised_groups` after provisioning.\nUser-specified list of prefix groups to advertise in custom\nmode, which currently supports the following option:\n\n* 'ALL_SUBNETS': Advertises all of the router's own VPC subnets.\nThis excludes any routes learned for subnets that use VPC Network\nPeering.\n\n\nNote that this field can only be populated if advertiseMode is 'CUSTOM'\nand overrides the list defined for the router (in the \"bgp\" message).\nThese groups are advertised in addition to any specified prefixes.\nLeave this field blank to advertise no custom groups."]
    pub fn advertised_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advertised_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `advertised_route_priority` after provisioning.\nThe priority of routes advertised to this BGP peer.\nWhere there is more than one matching route of maximum\nlength, the routes with the lowest priority value win."]
    pub fn advertised_route_priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.advertised_route_priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_learned_route_priority` after provisioning.\nThe user-defined custom learned route priority for a BGP session.\nThis value is applied to all custom learned route ranges for the session. You can choose a value\nfrom 0 to 65335. If you don't provide a value, Google Cloud assigns a priority of 100 to the ranges."]
    pub fn custom_learned_route_priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_learned_route_priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable` after provisioning.\nThe status of the BGP peer connection. If set to false, any active session\nwith the peer is terminated and all associated routing information is removed.\nIf set to true, the peer connection can be established with routing information.\nThe default is true."]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_ipv4` after provisioning.\nEnable IPv4 traffic over BGP Peer. It is enabled by default if the peerIpAddress is version 4."]
    pub fn enable_ipv4(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_ipv4", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_ipv6` after provisioning.\nEnable IPv6 traffic over BGP Peer. If not specified, it is disabled by default."]
    pub fn enable_ipv6(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_ipv6", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `export_policies` after provisioning.\nrouters.list of export policies applied to this peer, in the order they must be evaluated. \nThe name must correspond to an existing policy that has ROUTE_POLICY_TYPE_EXPORT type."]
    pub fn export_policies(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.export_policies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `import_policies` after provisioning.\nrouters.list of import policies applied to this peer, in the order they must be evaluated. \nThe name must correspond to an existing policy that has ROUTE_POLICY_TYPE_IMPORT type."]
    pub fn import_policies(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.import_policies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `interface` after provisioning.\nName of the interface the BGP peer is associated with."]
    pub fn interface(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.interface", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\nIP address of the interface inside Google Cloud Platform.\nOnly IPv4 is supported."]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv4_nexthop_address` after provisioning.\nIPv4 address of the interface inside Google Cloud Platform."]
    pub fn ipv4_nexthop_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv4_nexthop_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_nexthop_address` after provisioning.\nIPv6 address of the interface inside Google Cloud Platform.\nThe address must be in the range 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64.\nIf you do not specify the next hop addresses, Google Cloud automatically\nassigns unused addresses from the 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64 range for you."]
    pub fn ipv6_nexthop_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_nexthop_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `is_advertised_route_priority_set` after provisioning.\nAn internal boolean field for provider use for zero_advertised_route_priority."]
    pub fn is_advertised_route_priority_set(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_advertised_route_priority_set", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `is_custom_learned_priority_set` after provisioning.\nAn internal boolean field for provider use."]
    pub fn is_custom_learned_priority_set(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_custom_learned_priority_set", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `management_type` after provisioning.\nThe resource that configures and manages this BGP peer.\n\n* 'MANAGED_BY_USER' is the default value and can be managed by\nyou or other users\n* 'MANAGED_BY_ATTACHMENT' is a BGP peer that is configured and\nmanaged by Cloud Interconnect, specifically by an\nInterconnectAttachment of type PARTNER. Google automatically\ncreates, updates, and deletes this type of BGP peer when the\nPARTNER InterconnectAttachment is created, updated,\nor deleted."]
    pub fn management_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.management_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of this BGP peer. The name must be 1-63 characters long,\nand comply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which\nmeans the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `peer_asn` after provisioning.\nPeer BGP Autonomous System Number (ASN).\nEach BGP interface may use a different value."]
    pub fn peer_asn(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_asn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `peer_ip_address` after provisioning.\nIP address of the BGP interface outside Google Cloud Platform.\nOnly IPv4 is supported. Required if 'ip_address' is set."]
    pub fn peer_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `peer_ipv4_nexthop_address` after provisioning.\nIPv4 address of the BGP interface outside Google Cloud Platform."]
    pub fn peer_ipv4_nexthop_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_ipv4_nexthop_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `peer_ipv6_nexthop_address` after provisioning.\nIPv6 address of the BGP interface outside Google Cloud Platform.\nThe address must be in the range 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64.\nIf you do not specify the next hop addresses, Google Cloud automatically\nassigns unused addresses from the 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64 range for you."]
    pub fn peer_ipv6_nexthop_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_ipv6_nexthop_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where the router and BgpPeer reside.\nIf it is not provided, the provider region is used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `router` after provisioning.\nThe name of the Cloud Router in which this BgpPeer will be configured."]
    pub fn router(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.router", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `router_appliance_instance` after provisioning.\nThe URI of the VM instance that is used as third-party router appliances\nsuch as Next Gen Firewalls, Virtual Routers, or Router Appliances.\nThe VM instance must be located in zones contained in the same region as\nthis Cloud Router. The VM instance is the peer side of the BGP session."]
    pub fn router_appliance_instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.router_appliance_instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zero_advertised_route_priority` after provisioning.\nForce the advertised_route_priority to be 0."]
    pub fn zero_advertised_route_priority(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zero_advertised_route_priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zero_custom_learned_route_priority` after provisioning.\nForce the custom_learned_route_priority to be 0."]
    pub fn zero_custom_learned_route_priority(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zero_custom_learned_route_priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bfd` after provisioning.\n"]
    pub fn bfd(&self) -> ListRef<ComputeRouterPeerBfdElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bfd", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `custom_learned_ip_ranges` after provisioning.\n"]
    pub fn custom_learned_ip_ranges(&self) -> ListRef<ComputeRouterPeerCustomLearnedIpRangesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_learned_ip_ranges", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `md5_authentication_key` after provisioning.\n"]
    pub fn md5_authentication_key(&self) -> ListRef<ComputeRouterPeerMd5AuthenticationKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.md5_authentication_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRouterPeerTimeoutsElRef {
        ComputeRouterPeerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeRouterPeer {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeRouterPeer {}
impl ToListMappable for ComputeRouterPeer {
    type O = ListRef<ComputeRouterPeerRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeRouterPeer_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_router_peer".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeRouterPeer {
    pub tf_id: String,
    #[doc = "Name of the interface the BGP peer is associated with."]
    pub interface: PrimField<String>,
    #[doc = "Name of this BGP peer. The name must be 1-63 characters long,\nand comply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which\nmeans the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub name: PrimField<String>,
    #[doc = "Peer BGP Autonomous System Number (ASN).\nEach BGP interface may use a different value."]
    pub peer_asn: PrimField<f64>,
    #[doc = "The name of the Cloud Router in which this BgpPeer will be configured."]
    pub router: PrimField<String>,
}
impl BuildComputeRouterPeer {
    pub fn build(self, stack: &mut Stack) -> ComputeRouterPeer {
        let out = ComputeRouterPeer(Rc::new(ComputeRouterPeer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeRouterPeerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                advertise_mode: core::default::Default::default(),
                advertised_groups: core::default::Default::default(),
                advertised_route_priority: core::default::Default::default(),
                custom_learned_route_priority: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                enable: core::default::Default::default(),
                enable_ipv4: core::default::Default::default(),
                enable_ipv6: core::default::Default::default(),
                export_policies: core::default::Default::default(),
                id: core::default::Default::default(),
                import_policies: core::default::Default::default(),
                interface: self.interface,
                ip_address: core::default::Default::default(),
                ipv4_nexthop_address: core::default::Default::default(),
                ipv6_nexthop_address: core::default::Default::default(),
                name: self.name,
                peer_asn: self.peer_asn,
                peer_ip_address: core::default::Default::default(),
                peer_ipv4_nexthop_address: core::default::Default::default(),
                peer_ipv6_nexthop_address: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                router: self.router,
                router_appliance_instance: core::default::Default::default(),
                zero_advertised_route_priority: core::default::Default::default(),
                zero_custom_learned_route_priority: core::default::Default::default(),
                advertised_ip_ranges: core::default::Default::default(),
                bfd: core::default::Default::default(),
                custom_learned_ip_ranges: core::default::Default::default(),
                md5_authentication_key: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeRouterPeerRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouterPeerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeRouterPeerRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `advertise_mode` after provisioning.\nUser-specified flag to indicate which mode to use for advertisement.\nValid values of this enum field are: 'DEFAULT', 'CUSTOM' Default value: \"DEFAULT\" Possible values: [\"DEFAULT\", \"CUSTOM\"]"]
    pub fn advertise_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.advertise_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `advertised_groups` after provisioning.\nUser-specified list of prefix groups to advertise in custom\nmode, which currently supports the following option:\n\n* 'ALL_SUBNETS': Advertises all of the router's own VPC subnets.\nThis excludes any routes learned for subnets that use VPC Network\nPeering.\n\n\nNote that this field can only be populated if advertiseMode is 'CUSTOM'\nand overrides the list defined for the router (in the \"bgp\" message).\nThese groups are advertised in addition to any specified prefixes.\nLeave this field blank to advertise no custom groups."]
    pub fn advertised_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advertised_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `advertised_route_priority` after provisioning.\nThe priority of routes advertised to this BGP peer.\nWhere there is more than one matching route of maximum\nlength, the routes with the lowest priority value win."]
    pub fn advertised_route_priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.advertised_route_priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_learned_route_priority` after provisioning.\nThe user-defined custom learned route priority for a BGP session.\nThis value is applied to all custom learned route ranges for the session. You can choose a value\nfrom 0 to 65335. If you don't provide a value, Google Cloud assigns a priority of 100 to the ranges."]
    pub fn custom_learned_route_priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_learned_route_priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable` after provisioning.\nThe status of the BGP peer connection. If set to false, any active session\nwith the peer is terminated and all associated routing information is removed.\nIf set to true, the peer connection can be established with routing information.\nThe default is true."]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_ipv4` after provisioning.\nEnable IPv4 traffic over BGP Peer. It is enabled by default if the peerIpAddress is version 4."]
    pub fn enable_ipv4(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_ipv4", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_ipv6` after provisioning.\nEnable IPv6 traffic over BGP Peer. If not specified, it is disabled by default."]
    pub fn enable_ipv6(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_ipv6", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `export_policies` after provisioning.\nrouters.list of export policies applied to this peer, in the order they must be evaluated. \nThe name must correspond to an existing policy that has ROUTE_POLICY_TYPE_EXPORT type."]
    pub fn export_policies(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.export_policies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `import_policies` after provisioning.\nrouters.list of import policies applied to this peer, in the order they must be evaluated. \nThe name must correspond to an existing policy that has ROUTE_POLICY_TYPE_IMPORT type."]
    pub fn import_policies(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.import_policies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `interface` after provisioning.\nName of the interface the BGP peer is associated with."]
    pub fn interface(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.interface", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\nIP address of the interface inside Google Cloud Platform.\nOnly IPv4 is supported."]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv4_nexthop_address` after provisioning.\nIPv4 address of the interface inside Google Cloud Platform."]
    pub fn ipv4_nexthop_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv4_nexthop_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_nexthop_address` after provisioning.\nIPv6 address of the interface inside Google Cloud Platform.\nThe address must be in the range 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64.\nIf you do not specify the next hop addresses, Google Cloud automatically\nassigns unused addresses from the 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64 range for you."]
    pub fn ipv6_nexthop_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_nexthop_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `is_advertised_route_priority_set` after provisioning.\nAn internal boolean field for provider use for zero_advertised_route_priority."]
    pub fn is_advertised_route_priority_set(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_advertised_route_priority_set", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `is_custom_learned_priority_set` after provisioning.\nAn internal boolean field for provider use."]
    pub fn is_custom_learned_priority_set(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_custom_learned_priority_set", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `management_type` after provisioning.\nThe resource that configures and manages this BGP peer.\n\n* 'MANAGED_BY_USER' is the default value and can be managed by\nyou or other users\n* 'MANAGED_BY_ATTACHMENT' is a BGP peer that is configured and\nmanaged by Cloud Interconnect, specifically by an\nInterconnectAttachment of type PARTNER. Google automatically\ncreates, updates, and deletes this type of BGP peer when the\nPARTNER InterconnectAttachment is created, updated,\nor deleted."]
    pub fn management_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.management_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of this BGP peer. The name must be 1-63 characters long,\nand comply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which\nmeans the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `peer_asn` after provisioning.\nPeer BGP Autonomous System Number (ASN).\nEach BGP interface may use a different value."]
    pub fn peer_asn(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_asn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `peer_ip_address` after provisioning.\nIP address of the BGP interface outside Google Cloud Platform.\nOnly IPv4 is supported. Required if 'ip_address' is set."]
    pub fn peer_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `peer_ipv4_nexthop_address` after provisioning.\nIPv4 address of the BGP interface outside Google Cloud Platform."]
    pub fn peer_ipv4_nexthop_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_ipv4_nexthop_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `peer_ipv6_nexthop_address` after provisioning.\nIPv6 address of the BGP interface outside Google Cloud Platform.\nThe address must be in the range 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64.\nIf you do not specify the next hop addresses, Google Cloud automatically\nassigns unused addresses from the 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64 range for you."]
    pub fn peer_ipv6_nexthop_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_ipv6_nexthop_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where the router and BgpPeer reside.\nIf it is not provided, the provider region is used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `router` after provisioning.\nThe name of the Cloud Router in which this BgpPeer will be configured."]
    pub fn router(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.router", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `router_appliance_instance` after provisioning.\nThe URI of the VM instance that is used as third-party router appliances\nsuch as Next Gen Firewalls, Virtual Routers, or Router Appliances.\nThe VM instance must be located in zones contained in the same region as\nthis Cloud Router. The VM instance is the peer side of the BGP session."]
    pub fn router_appliance_instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.router_appliance_instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zero_advertised_route_priority` after provisioning.\nForce the advertised_route_priority to be 0."]
    pub fn zero_advertised_route_priority(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zero_advertised_route_priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zero_custom_learned_route_priority` after provisioning.\nForce the custom_learned_route_priority to be 0."]
    pub fn zero_custom_learned_route_priority(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zero_custom_learned_route_priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bfd` after provisioning.\n"]
    pub fn bfd(&self) -> ListRef<ComputeRouterPeerBfdElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bfd", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `custom_learned_ip_ranges` after provisioning.\n"]
    pub fn custom_learned_ip_ranges(&self) -> ListRef<ComputeRouterPeerCustomLearnedIpRangesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_learned_ip_ranges", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `md5_authentication_key` after provisioning.\n"]
    pub fn md5_authentication_key(&self) -> ListRef<ComputeRouterPeerMd5AuthenticationKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.md5_authentication_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRouterPeerTimeoutsElRef {
        ComputeRouterPeerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRouterPeerAdvertisedIpRangesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    range: PrimField<String>,
}
impl ComputeRouterPeerAdvertisedIpRangesEl {
    #[doc = "Set the field `description`.\nUser-specified description for the IP range."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRouterPeerAdvertisedIpRangesEl {
    type O = BlockAssignable<ComputeRouterPeerAdvertisedIpRangesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRouterPeerAdvertisedIpRangesEl {
    #[doc = "The IP range to advertise. The value must be a\nCIDR-formatted string."]
    pub range: PrimField<String>,
}
impl BuildComputeRouterPeerAdvertisedIpRangesEl {
    pub fn build(self) -> ComputeRouterPeerAdvertisedIpRangesEl {
        ComputeRouterPeerAdvertisedIpRangesEl {
            description: core::default::Default::default(),
            range: self.range,
        }
    }
}
pub struct ComputeRouterPeerAdvertisedIpRangesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouterPeerAdvertisedIpRangesElRef {
    fn new(shared: StackShared, base: String) -> ComputeRouterPeerAdvertisedIpRangesElRef {
        ComputeRouterPeerAdvertisedIpRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRouterPeerAdvertisedIpRangesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-specified description for the IP range."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `range` after provisioning.\nThe IP range to advertise. The value must be a\nCIDR-formatted string."]
    pub fn range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.range", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRouterPeerBfdEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    min_receive_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_transmit_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multiplier: Option<PrimField<f64>>,
    session_initialization_mode: PrimField<String>,
}
impl ComputeRouterPeerBfdEl {
    #[doc = "Set the field `min_receive_interval`.\nThe minimum interval, in milliseconds, between BFD control packets\nreceived from the peer router. The actual value is negotiated\nbetween the two routers and is equal to the greater of this value\nand the transmit interval of the other router. If set, this value\nmust be between 1000 and 30000."]
    pub fn set_min_receive_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_receive_interval = Some(v.into());
        self
    }
    #[doc = "Set the field `min_transmit_interval`.\nThe minimum interval, in milliseconds, between BFD control packets\ntransmitted to the peer router. The actual value is negotiated\nbetween the two routers and is equal to the greater of this value\nand the corresponding receive interval of the other router. If set,\nthis value must be between 1000 and 30000."]
    pub fn set_min_transmit_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_transmit_interval = Some(v.into());
        self
    }
    #[doc = "Set the field `multiplier`.\nThe number of consecutive BFD packets that must be missed before\nBFD declares that a peer is unavailable. If set, the value must\nbe a value between 5 and 16."]
    pub fn set_multiplier(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.multiplier = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRouterPeerBfdEl {
    type O = BlockAssignable<ComputeRouterPeerBfdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRouterPeerBfdEl {
    #[doc = "The BFD session initialization mode for this BGP peer.\nIf set to 'ACTIVE', the Cloud Router will initiate the BFD session\nfor this BGP peer. If set to 'PASSIVE', the Cloud Router will wait\nfor the peer router to initiate the BFD session for this BGP peer.\nIf set to 'DISABLED', BFD is disabled for this BGP peer. Possible values: [\"ACTIVE\", \"DISABLED\", \"PASSIVE\"]"]
    pub session_initialization_mode: PrimField<String>,
}
impl BuildComputeRouterPeerBfdEl {
    pub fn build(self) -> ComputeRouterPeerBfdEl {
        ComputeRouterPeerBfdEl {
            min_receive_interval: core::default::Default::default(),
            min_transmit_interval: core::default::Default::default(),
            multiplier: core::default::Default::default(),
            session_initialization_mode: self.session_initialization_mode,
        }
    }
}
pub struct ComputeRouterPeerBfdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouterPeerBfdElRef {
    fn new(shared: StackShared, base: String) -> ComputeRouterPeerBfdElRef {
        ComputeRouterPeerBfdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRouterPeerBfdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `min_receive_interval` after provisioning.\nThe minimum interval, in milliseconds, between BFD control packets\nreceived from the peer router. The actual value is negotiated\nbetween the two routers and is equal to the greater of this value\nand the transmit interval of the other router. If set, this value\nmust be between 1000 and 30000."]
    pub fn min_receive_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_receive_interval", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_transmit_interval` after provisioning.\nThe minimum interval, in milliseconds, between BFD control packets\ntransmitted to the peer router. The actual value is negotiated\nbetween the two routers and is equal to the greater of this value\nand the corresponding receive interval of the other router. If set,\nthis value must be between 1000 and 30000."]
    pub fn min_transmit_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_transmit_interval", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `multiplier` after provisioning.\nThe number of consecutive BFD packets that must be missed before\nBFD declares that a peer is unavailable. If set, the value must\nbe a value between 5 and 16."]
    pub fn multiplier(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.multiplier", self.base))
    }
    #[doc = "Get a reference to the value of field `session_initialization_mode` after provisioning.\nThe BFD session initialization mode for this BGP peer.\nIf set to 'ACTIVE', the Cloud Router will initiate the BFD session\nfor this BGP peer. If set to 'PASSIVE', the Cloud Router will wait\nfor the peer router to initiate the BFD session for this BGP peer.\nIf set to 'DISABLED', BFD is disabled for this BGP peer. Possible values: [\"ACTIVE\", \"DISABLED\", \"PASSIVE\"]"]
    pub fn session_initialization_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_initialization_mode", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRouterPeerCustomLearnedIpRangesEl {
    range: PrimField<String>,
}
impl ComputeRouterPeerCustomLearnedIpRangesEl {}
impl ToListMappable for ComputeRouterPeerCustomLearnedIpRangesEl {
    type O = BlockAssignable<ComputeRouterPeerCustomLearnedIpRangesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRouterPeerCustomLearnedIpRangesEl {
    #[doc = "The IP range to learn. The value must be a\nCIDR-formatted string."]
    pub range: PrimField<String>,
}
impl BuildComputeRouterPeerCustomLearnedIpRangesEl {
    pub fn build(self) -> ComputeRouterPeerCustomLearnedIpRangesEl {
        ComputeRouterPeerCustomLearnedIpRangesEl { range: self.range }
    }
}
pub struct ComputeRouterPeerCustomLearnedIpRangesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouterPeerCustomLearnedIpRangesElRef {
    fn new(shared: StackShared, base: String) -> ComputeRouterPeerCustomLearnedIpRangesElRef {
        ComputeRouterPeerCustomLearnedIpRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRouterPeerCustomLearnedIpRangesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `range` after provisioning.\nThe IP range to learn. The value must be a\nCIDR-formatted string."]
    pub fn range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.range", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRouterPeerMd5AuthenticationKeyEl {
    key: PrimField<String>,
    name: PrimField<String>,
}
impl ComputeRouterPeerMd5AuthenticationKeyEl {}
impl ToListMappable for ComputeRouterPeerMd5AuthenticationKeyEl {
    type O = BlockAssignable<ComputeRouterPeerMd5AuthenticationKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRouterPeerMd5AuthenticationKeyEl {
    #[doc = "Value of the key."]
    pub key: PrimField<String>,
    #[doc = "[REQUIRED] Name used to identify the key.\nMust be unique within a router. Must be referenced by exactly one bgpPeer. Must comply with RFC1035."]
    pub name: PrimField<String>,
}
impl BuildComputeRouterPeerMd5AuthenticationKeyEl {
    pub fn build(self) -> ComputeRouterPeerMd5AuthenticationKeyEl {
        ComputeRouterPeerMd5AuthenticationKeyEl {
            key: self.key,
            name: self.name,
        }
    }
}
pub struct ComputeRouterPeerMd5AuthenticationKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouterPeerMd5AuthenticationKeyElRef {
    fn new(shared: StackShared, base: String) -> ComputeRouterPeerMd5AuthenticationKeyElRef {
        ComputeRouterPeerMd5AuthenticationKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRouterPeerMd5AuthenticationKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nValue of the key."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n[REQUIRED] Name used to identify the key.\nMust be unique within a router. Must be referenced by exactly one bgpPeer. Must comply with RFC1035."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRouterPeerTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeRouterPeerTimeoutsEl {
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
impl ToListMappable for ComputeRouterPeerTimeoutsEl {
    type O = BlockAssignable<ComputeRouterPeerTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRouterPeerTimeoutsEl {}
impl BuildComputeRouterPeerTimeoutsEl {
    pub fn build(self) -> ComputeRouterPeerTimeoutsEl {
        ComputeRouterPeerTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeRouterPeerTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouterPeerTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeRouterPeerTimeoutsElRef {
        ComputeRouterPeerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRouterPeerTimeoutsElRef {
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
#[derive(Serialize, Default)]
struct ComputeRouterPeerDynamic {
    advertised_ip_ranges: Option<DynamicBlock<ComputeRouterPeerAdvertisedIpRangesEl>>,
    bfd: Option<DynamicBlock<ComputeRouterPeerBfdEl>>,
    custom_learned_ip_ranges: Option<DynamicBlock<ComputeRouterPeerCustomLearnedIpRangesEl>>,
    md5_authentication_key: Option<DynamicBlock<ComputeRouterPeerMd5AuthenticationKeyEl>>,
}
