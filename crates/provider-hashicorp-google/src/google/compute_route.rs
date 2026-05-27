use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeRouteData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    dest_range: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_gateway: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_ilb: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_instance_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_vpn_tunnel: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Vec<ComputeRouteParamsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeRouteTimeoutsEl>,
    dynamic: ComputeRouteDynamic,
}
struct ComputeRoute_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeRouteData>,
}
#[derive(Clone)]
pub struct ComputeRoute(Rc<ComputeRoute_>);
impl ComputeRoute {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nAn optional description of this resource. Provide this property\nwhen you create the resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_gateway`.\nURL to a gateway that should handle matching packets.\nCurrently, you can only specify the internet gateway, using a full or\npartial valid URL:\n* 'https://www.googleapis.com/compute/v1/projects/project/global/gateways/default-internet-gateway'\n* 'projects/project/global/gateways/default-internet-gateway'\n* 'global/gateways/default-internet-gateway'\n* The string 'default-internet-gateway'."]
    pub fn set_next_hop_gateway(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().next_hop_gateway = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_ilb`.\nThe IP address or URL to a forwarding rule of type\nloadBalancingScheme=INTERNAL that should handle matching\npackets.\n\nWith the GA provider you can only specify the forwarding\nrule as a partial or full URL. For example, the following\nare all valid values:\n* 10.128.0.56\n* https://www.googleapis.com/compute/v1/projects/project/regions/region/forwardingRules/forwardingRule\n* regions/region/forwardingRules/forwardingRule\n\nWhen the beta provider, you can also specify the IP address\nof a forwarding rule from the same VPC or any peered VPC.\n\nNote that this can only be used when the destinationRange is\na public (non-RFC 1918) IP CIDR range."]
    pub fn set_next_hop_ilb(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().next_hop_ilb = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_instance`.\nURL to an instance that should handle matching packets.\nYou can specify this as a full or partial URL. For example:\n* 'https://www.googleapis.com/compute/v1/projects/project/zones/zone/instances/instance'\n* 'projects/project/zones/zone/instances/instance'\n* 'zones/zone/instances/instance'\n* Just the instance name, with the zone in 'next_hop_instance_zone'."]
    pub fn set_next_hop_instance(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().next_hop_instance = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_instance_zone`.\nThe zone of the instance specified in next_hop_instance. Omit if next_hop_instance is specified as a URL."]
    pub fn set_next_hop_instance_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().next_hop_instance_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_ip`.\nNetwork IP address of an instance that should handle matching packets."]
    pub fn set_next_hop_ip(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().next_hop_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_vpn_tunnel`.\nURL to a VpnTunnel that should handle matching packets."]
    pub fn set_next_hop_vpn_tunnel(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().next_hop_vpn_tunnel = Some(v.into());
        self
    }
    #[doc = "Set the field `priority`.\nThe priority of this route. Priority is used to break ties in cases\nwhere there is more than one matching route of equal prefix length.\n\nIn the case of two routes with equal prefix length, the one with the\nlowest-numbered priority value wins.\n\nDefault value is 1000. Valid range is 0 through 65535."]
    pub fn set_priority(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().priority = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\nA list of instance tags to which this route applies."]
    pub fn set_tags(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `params`.\n"]
    pub fn set_params(self, v: impl Into<BlockAssignable<ComputeRouteParamsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().params = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.params = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeRouteTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `as_paths` after provisioning.\n"]
    pub fn as_paths(&self) -> ListRef<ComputeRouteAsPathsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.as_paths", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property\nwhen you create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dest_range` after provisioning.\nThe destination range of outgoing packets that this route applies to.\nOnly IPv4 is supported."]
    pub fn dest_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dest_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035.  Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe network that this route applies to."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_gateway` after provisioning.\nURL to a gateway that should handle matching packets.\nCurrently, you can only specify the internet gateway, using a full or\npartial valid URL:\n* 'https://www.googleapis.com/compute/v1/projects/project/global/gateways/default-internet-gateway'\n* 'projects/project/global/gateways/default-internet-gateway'\n* 'global/gateways/default-internet-gateway'\n* The string 'default-internet-gateway'."]
    pub fn next_hop_gateway(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_gateway", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_hub` after provisioning.\nThe hub network that should handle matching packets, which should conform to RFC1035."]
    pub fn next_hop_hub(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_hub", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_ilb` after provisioning.\nThe IP address or URL to a forwarding rule of type\nloadBalancingScheme=INTERNAL that should handle matching\npackets.\n\nWith the GA provider you can only specify the forwarding\nrule as a partial or full URL. For example, the following\nare all valid values:\n* 10.128.0.56\n* https://www.googleapis.com/compute/v1/projects/project/regions/region/forwardingRules/forwardingRule\n* regions/region/forwardingRules/forwardingRule\n\nWhen the beta provider, you can also specify the IP address\nof a forwarding rule from the same VPC or any peered VPC.\n\nNote that this can only be used when the destinationRange is\na public (non-RFC 1918) IP CIDR range."]
    pub fn next_hop_ilb(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_ilb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_instance` after provisioning.\nURL to an instance that should handle matching packets.\nYou can specify this as a full or partial URL. For example:\n* 'https://www.googleapis.com/compute/v1/projects/project/zones/zone/instances/instance'\n* 'projects/project/zones/zone/instances/instance'\n* 'zones/zone/instances/instance'\n* Just the instance name, with the zone in 'next_hop_instance_zone'."]
    pub fn next_hop_instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_instance_zone` after provisioning.\nThe zone of the instance specified in next_hop_instance. Omit if next_hop_instance is specified as a URL."]
    pub fn next_hop_instance_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_instance_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_inter_region_cost` after provisioning.\nInternal fixed region-to-region cost that Google Cloud calculates based on factors such as network performance, distance, and available bandwidth between regions."]
    pub fn next_hop_inter_region_cost(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_inter_region_cost", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_ip` after provisioning.\nNetwork IP address of an instance that should handle matching packets."]
    pub fn next_hop_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_med` after provisioning.\nMulti-Exit Discriminator, a BGP route metric that indicates the desirability of a particular route in a network."]
    pub fn next_hop_med(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_med", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_network` after provisioning.\nURL to a Network that should handle matching packets."]
    pub fn next_hop_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_origin` after provisioning.\nIndicates the origin of the route. Can be IGP (Interior Gateway Protocol), EGP (Exterior Gateway Protocol), or INCOMPLETE."]
    pub fn next_hop_origin(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_origin", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_peering` after provisioning.\nThe network peering name that should handle matching packets, which should conform to RFC1035."]
    pub fn next_hop_peering(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_peering", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_vpn_tunnel` after provisioning.\nURL to a VpnTunnel that should handle matching packets."]
    pub fn next_hop_vpn_tunnel(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_vpn_tunnel", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nThe priority of this route. Priority is used to break ties in cases\nwhere there is more than one matching route of equal prefix length.\n\nIn the case of two routes with equal prefix length, the one with the\nlowest-numbered priority value wins.\n\nDefault value is 1000. Valid range is 0 through 65535."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `route_status` after provisioning.\nThe status of the route, which can be one of the following values:\n- 'ACTIVE' for an active route\n- 'INACTIVE' for an inactive route"]
    pub fn route_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.route_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `route_type` after provisioning.\nThe type of this route, which can be one of the following values:\n- 'TRANSIT' for a transit route that this router learned from another Cloud Router and will readvertise to one of its BGP peers\n- 'SUBNET' for a route from a subnet of the VPC\n- 'BGP' for a route learned from a BGP peer of this router\n- 'STATIC' for a static route"]
    pub fn route_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.route_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nA list of instance tags to which this route applies."]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `warnings` after provisioning.\nIf potential misconfigurations are detected for this route, this field will be populated with warning messages."]
    pub fn warnings(&self) -> ListRef<ComputeRouteWarningsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.warnings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> ListRef<ComputeRouteParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRouteTimeoutsElRef {
        ComputeRouteTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeRoute {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeRoute {}
impl ToListMappable for ComputeRoute {
    type O = ListRef<ComputeRouteRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeRoute_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_route".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeRoute {
    pub tf_id: String,
    #[doc = "The destination range of outgoing packets that this route applies to.\nOnly IPv4 is supported."]
    pub dest_range: PrimField<String>,
    #[doc = "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035.  Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
    pub name: PrimField<String>,
    #[doc = "The network that this route applies to."]
    pub network: PrimField<String>,
}
impl BuildComputeRoute {
    pub fn build(self, stack: &mut Stack) -> ComputeRoute {
        let out = ComputeRoute(Rc::new(ComputeRoute_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeRouteData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                dest_range: self.dest_range,
                id: core::default::Default::default(),
                name: self.name,
                network: self.network,
                next_hop_gateway: core::default::Default::default(),
                next_hop_ilb: core::default::Default::default(),
                next_hop_instance: core::default::Default::default(),
                next_hop_instance_zone: core::default::Default::default(),
                next_hop_ip: core::default::Default::default(),
                next_hop_vpn_tunnel: core::default::Default::default(),
                priority: core::default::Default::default(),
                project: core::default::Default::default(),
                tags: core::default::Default::default(),
                params: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeRouteRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouteRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeRouteRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `as_paths` after provisioning.\n"]
    pub fn as_paths(&self) -> ListRef<ComputeRouteAsPathsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.as_paths", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property\nwhen you create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dest_range` after provisioning.\nThe destination range of outgoing packets that this route applies to.\nOnly IPv4 is supported."]
    pub fn dest_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dest_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035.  Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe network that this route applies to."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_gateway` after provisioning.\nURL to a gateway that should handle matching packets.\nCurrently, you can only specify the internet gateway, using a full or\npartial valid URL:\n* 'https://www.googleapis.com/compute/v1/projects/project/global/gateways/default-internet-gateway'\n* 'projects/project/global/gateways/default-internet-gateway'\n* 'global/gateways/default-internet-gateway'\n* The string 'default-internet-gateway'."]
    pub fn next_hop_gateway(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_gateway", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_hub` after provisioning.\nThe hub network that should handle matching packets, which should conform to RFC1035."]
    pub fn next_hop_hub(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_hub", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_ilb` after provisioning.\nThe IP address or URL to a forwarding rule of type\nloadBalancingScheme=INTERNAL that should handle matching\npackets.\n\nWith the GA provider you can only specify the forwarding\nrule as a partial or full URL. For example, the following\nare all valid values:\n* 10.128.0.56\n* https://www.googleapis.com/compute/v1/projects/project/regions/region/forwardingRules/forwardingRule\n* regions/region/forwardingRules/forwardingRule\n\nWhen the beta provider, you can also specify the IP address\nof a forwarding rule from the same VPC or any peered VPC.\n\nNote that this can only be used when the destinationRange is\na public (non-RFC 1918) IP CIDR range."]
    pub fn next_hop_ilb(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_ilb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_instance` after provisioning.\nURL to an instance that should handle matching packets.\nYou can specify this as a full or partial URL. For example:\n* 'https://www.googleapis.com/compute/v1/projects/project/zones/zone/instances/instance'\n* 'projects/project/zones/zone/instances/instance'\n* 'zones/zone/instances/instance'\n* Just the instance name, with the zone in 'next_hop_instance_zone'."]
    pub fn next_hop_instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_instance_zone` after provisioning.\nThe zone of the instance specified in next_hop_instance. Omit if next_hop_instance is specified as a URL."]
    pub fn next_hop_instance_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_instance_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_inter_region_cost` after provisioning.\nInternal fixed region-to-region cost that Google Cloud calculates based on factors such as network performance, distance, and available bandwidth between regions."]
    pub fn next_hop_inter_region_cost(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_inter_region_cost", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_ip` after provisioning.\nNetwork IP address of an instance that should handle matching packets."]
    pub fn next_hop_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_med` after provisioning.\nMulti-Exit Discriminator, a BGP route metric that indicates the desirability of a particular route in a network."]
    pub fn next_hop_med(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_med", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_network` after provisioning.\nURL to a Network that should handle matching packets."]
    pub fn next_hop_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_origin` after provisioning.\nIndicates the origin of the route. Can be IGP (Interior Gateway Protocol), EGP (Exterior Gateway Protocol), or INCOMPLETE."]
    pub fn next_hop_origin(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_origin", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_peering` after provisioning.\nThe network peering name that should handle matching packets, which should conform to RFC1035."]
    pub fn next_hop_peering(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_peering", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_vpn_tunnel` after provisioning.\nURL to a VpnTunnel that should handle matching packets."]
    pub fn next_hop_vpn_tunnel(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_vpn_tunnel", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nThe priority of this route. Priority is used to break ties in cases\nwhere there is more than one matching route of equal prefix length.\n\nIn the case of two routes with equal prefix length, the one with the\nlowest-numbered priority value wins.\n\nDefault value is 1000. Valid range is 0 through 65535."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `route_status` after provisioning.\nThe status of the route, which can be one of the following values:\n- 'ACTIVE' for an active route\n- 'INACTIVE' for an inactive route"]
    pub fn route_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.route_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `route_type` after provisioning.\nThe type of this route, which can be one of the following values:\n- 'TRANSIT' for a transit route that this router learned from another Cloud Router and will readvertise to one of its BGP peers\n- 'SUBNET' for a route from a subnet of the VPC\n- 'BGP' for a route learned from a BGP peer of this router\n- 'STATIC' for a static route"]
    pub fn route_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.route_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nA list of instance tags to which this route applies."]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `warnings` after provisioning.\nIf potential misconfigurations are detected for this route, this field will be populated with warning messages."]
    pub fn warnings(&self) -> ListRef<ComputeRouteWarningsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.warnings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> ListRef<ComputeRouteParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRouteTimeoutsElRef {
        ComputeRouteTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRouteAsPathsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    as_lists: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_segment_type: Option<PrimField<String>>,
}
impl ComputeRouteAsPathsEl {
    #[doc = "Set the field `as_lists`.\n"]
    pub fn set_as_lists(mut self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.as_lists = Some(v.into());
        self
    }
    #[doc = "Set the field `path_segment_type`.\n"]
    pub fn set_path_segment_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_segment_type = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRouteAsPathsEl {
    type O = BlockAssignable<ComputeRouteAsPathsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRouteAsPathsEl {}
impl BuildComputeRouteAsPathsEl {
    pub fn build(self) -> ComputeRouteAsPathsEl {
        ComputeRouteAsPathsEl {
            as_lists: core::default::Default::default(),
            path_segment_type: core::default::Default::default(),
        }
    }
}
pub struct ComputeRouteAsPathsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouteAsPathsElRef {
    fn new(shared: StackShared, base: String) -> ComputeRouteAsPathsElRef {
        ComputeRouteAsPathsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRouteAsPathsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `as_lists` after provisioning.\n"]
    pub fn as_lists(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.as_lists", self.base))
    }
    #[doc = "Get a reference to the value of field `path_segment_type` after provisioning.\n"]
    pub fn path_segment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path_segment_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRouteWarningsElDataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ComputeRouteWarningsElDataEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRouteWarningsElDataEl {
    type O = BlockAssignable<ComputeRouteWarningsElDataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRouteWarningsElDataEl {}
impl BuildComputeRouteWarningsElDataEl {
    pub fn build(self) -> ComputeRouteWarningsElDataEl {
        ComputeRouteWarningsElDataEl {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct ComputeRouteWarningsElDataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouteWarningsElDataElRef {
    fn new(shared: StackShared, base: String) -> ComputeRouteWarningsElDataElRef {
        ComputeRouteWarningsElDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRouteWarningsElDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRouteWarningsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<ListField<ComputeRouteWarningsElDataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl ComputeRouteWarningsEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl Into<ListField<ComputeRouteWarningsElDataEl>>) -> Self {
        self.data = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRouteWarningsEl {
    type O = BlockAssignable<ComputeRouteWarningsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRouteWarningsEl {}
impl BuildComputeRouteWarningsEl {
    pub fn build(self) -> ComputeRouteWarningsEl {
        ComputeRouteWarningsEl {
            code: core::default::Default::default(),
            data: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct ComputeRouteWarningsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouteWarningsElRef {
    fn new(shared: StackShared, base: String) -> ComputeRouteWarningsElRef {
        ComputeRouteWarningsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRouteWarningsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> ListRef<ComputeRouteWarningsElDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRouteParamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<RecField<PrimField<String>>>,
}
impl ComputeRouteParamsEl {
    #[doc = "Set the field `resource_manager_tags`.\nResource manager tags to be bound to the route. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456. The field is ignored when empty.\nThe field is immutable and causes resource replacement when mutated. This field is only\nset at create time and modifying this field after creation will trigger recreation.\nTo apply tags to an existing resource, see the google_tags_tag_binding resource."]
    pub fn set_resource_manager_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_manager_tags = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRouteParamsEl {
    type O = BlockAssignable<ComputeRouteParamsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRouteParamsEl {}
impl BuildComputeRouteParamsEl {
    pub fn build(self) -> ComputeRouteParamsEl {
        ComputeRouteParamsEl {
            resource_manager_tags: core::default::Default::default(),
        }
    }
}
pub struct ComputeRouteParamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouteParamsElRef {
    fn new(shared: StackShared, base: String) -> ComputeRouteParamsElRef {
        ComputeRouteParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRouteParamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\nResource manager tags to be bound to the route. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456. The field is ignored when empty.\nThe field is immutable and causes resource replacement when mutated. This field is only\nset at create time and modifying this field after creation will trigger recreation.\nTo apply tags to an existing resource, see the google_tags_tag_binding resource."]
    pub fn resource_manager_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_manager_tags", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRouteTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl ComputeRouteTimeoutsEl {
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
impl ToListMappable for ComputeRouteTimeoutsEl {
    type O = BlockAssignable<ComputeRouteTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRouteTimeoutsEl {}
impl BuildComputeRouteTimeoutsEl {
    pub fn build(self) -> ComputeRouteTimeoutsEl {
        ComputeRouteTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct ComputeRouteTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouteTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeRouteTimeoutsElRef {
        ComputeRouteTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRouteTimeoutsElRef {
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
#[derive(Serialize, Default)]
struct ComputeRouteDynamic {
    params: Option<DynamicBlock<ComputeRouteParamsEl>>,
}
