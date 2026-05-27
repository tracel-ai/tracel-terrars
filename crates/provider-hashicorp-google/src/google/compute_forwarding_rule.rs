use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeForwardingRuleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    all_ports: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_global_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_psc_global_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backend_service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_collection: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_mirroring_collector: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancing_scheme: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_tier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_automate_dns_zone: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recreate_closed_psc: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_label: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_ip_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_directory_registrations:
        Option<Vec<ComputeForwardingRuleServiceDirectoryRegistrationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeForwardingRuleTimeoutsEl>,
    dynamic: ComputeForwardingRuleDynamic,
}
struct ComputeForwardingRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeForwardingRuleData>,
}
#[derive(Clone)]
pub struct ComputeForwardingRule(Rc<ComputeForwardingRule_>);
impl ComputeForwardingRule {
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
    #[doc = "Set the field `all_ports`.\nThe 'ports', 'portRange', and 'allPorts' fields are mutually exclusive.\nOnly packets addressed to ports in the specified range will be forwarded\nto the backends configured with this forwarding rule.\n\nThe 'allPorts' field has the following limitations:\n* It requires that the forwarding rule 'IPProtocol' be TCP, UDP, SCTP, or\nL3_DEFAULT.\n* It's applicable only to the following products: internal passthrough\nNetwork Load Balancers, backend service-based external passthrough Network\nLoad Balancers, and internal and external protocol forwarding.\n* Set this field to true to allow packets addressed to any port or packets\nlacking destination port information (for example, UDP fragments after the\nfirst fragment) to be forwarded to the backends configured with this\nforwarding rule. The L3_DEFAULT protocol requires 'allPorts' be set to\ntrue."]
    pub fn set_all_ports(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().all_ports = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_global_access`.\nThis field is used along with the 'backend_service' field for\ninternal load balancing or with the 'target' field for internal\nTargetInstance.\n\nIf the field is set to 'TRUE', clients can access ILB from all\nregions.\n\nOtherwise only allows access from clients in the same region as the\ninternal load balancer."]
    pub fn set_allow_global_access(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_global_access = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_psc_global_access`.\nThis is used in PSC consumer ForwardingRule to control whether the PSC endpoint can be accessed from another region."]
    pub fn set_allow_psc_global_access(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_psc_global_access = Some(v.into());
        self
    }
    #[doc = "Set the field `backend_service`.\nIdentifies the backend service to which the forwarding rule sends traffic.\n\nRequired for Internal TCP/UDP Load Balancing and Network Load Balancing;\nmust be omitted for all other load balancer types."]
    pub fn set_backend_service(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().backend_service = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_address`.\nIP address for which this forwarding rule accepts traffic. When a client\nsends traffic to this IP address, the forwarding rule directs the traffic\nto the referenced 'target' or 'backendService'.\n\nWhile creating a forwarding rule, specifying an 'IPAddress' is\nrequired under the following circumstances:\n\n* When the 'target' is set to 'targetGrpcProxy' and\n'validateForProxyless' is set to 'true', the\n'IPAddress' should be set to '0.0.0.0'.\n* When the 'target' is a Private Service Connect Google APIs\nbundle, you must specify an 'IPAddress'.\n\nOtherwise, you can optionally specify an IP address that references an\nexisting static (reserved) IP address resource. When omitted, Google Cloud\nassigns an ephemeral IP address.\n\nUse one of the following formats to specify an IP address while creating a\nforwarding rule:\n\n* IP address number, as in '100.1.2.3'\n* IPv6 address range, as in '2600:1234::/96'\n* Full resource URL, as in\n'https://www.googleapis.com/compute/v1/projects/project_id/regions/region/addresses/address-name'\n* Partial URL or by name, as in:\n  * 'projects/project_id/regions/region/addresses/address-name'\n  * 'regions/region/addresses/address-name'\n  * 'global/addresses/address-name'\n  * 'address-name'\n\nThe forwarding rule's 'target' or 'backendService',\nand in most cases, also the 'loadBalancingScheme', determine the\ntype of IP address that you can use. For detailed information, see\n[IP address\nspecifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#ip_address_specifications).\n\nWhen reading an 'IPAddress', the API always returns the IP\naddress number."]
    pub fn set_ip_address(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ip_address = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_collection`.\nResource reference of a PublicDelegatedPrefix. The PDP must be a sub-PDP\nin EXTERNAL_IPV6_FORWARDING_RULE_CREATION mode.\nUse one of the following formats to specify a sub-PDP when creating an\nIPv6 NetLB forwarding rule using BYOIP:\nFull resource URL, as in:\n  * 'https://www.googleapis.com/compute/v1/projects/{{projectId}}/regions/{{region}}/publicDelegatedPrefixes/{{sub-pdp-name}}'\nPartial URL, as in:\n  * 'projects/{{projectId}}/regions/region/publicDelegatedPrefixes/{{sub-pdp-name}}'\n  * 'regions/{{region}}/publicDelegatedPrefixes/{{sub-pdp-name}}'"]
    pub fn set_ip_collection(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ip_collection = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_protocol`.\nThe IP protocol to which this rule applies.\n\nFor protocol forwarding, valid\noptions are 'TCP', 'UDP', 'ESP',\n'AH', 'SCTP', 'ICMP' and\n'L3_DEFAULT'.\n\nThe valid IP protocols are different for different load balancing products\nas described in [Load balancing\nfeatures](https://cloud.google.com/load-balancing/docs/features#protocols_from_the_load_balancer_to_the_backends).\n\nA Forwarding Rule with protocol L3_DEFAULT can attach with target instance or\nbackend service with UNSPECIFIED protocol.\nA forwarding rule with \"L3_DEFAULT\" IPProtocal cannot be attached to a backend service with TCP or UDP. Possible values: [\"TCP\", \"UDP\", \"ESP\", \"AH\", \"SCTP\", \"ICMP\", \"L3_DEFAULT\"]"]
    pub fn set_ip_protocol(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ip_protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_version`.\nThe IP address version that will be used by this forwarding rule.\nValid options are IPV4 and IPV6.\n\nIf not set, the IPv4 address will be used by default. Possible values: [\"IPV4\", \"IPV6\"]"]
    pub fn set_ip_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ip_version = Some(v.into());
        self
    }
    #[doc = "Set the field `is_mirroring_collector`.\nIndicates whether or not this load balancer can be used as a collector for\npacket mirroring. To prevent mirroring loops, instances behind this\nload balancer will not have their traffic mirrored even if a\n'PacketMirroring' rule applies to them.\n\nThis can only be set to true for load balancers that have their\n'loadBalancingScheme' set to 'INTERNAL'."]
    pub fn set_is_mirroring_collector(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_mirroring_collector = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels to apply to this forwarding rule.  A list of key->value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `load_balancing_scheme`.\nSpecifies the forwarding rule type.\n\nNote that an empty string value ('\"\"') is also supported for some use\ncases, for example PSC (private service connection) regional forwarding\nrules.\n\nFor more information about forwarding rules, refer to\n[Forwarding rule concepts](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts). Default value: \"EXTERNAL\" Possible values: [\"EXTERNAL\", \"EXTERNAL_MANAGED\", \"INTERNAL\", \"INTERNAL_MANAGED\"]"]
    pub fn set_load_balancing_scheme(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().load_balancing_scheme = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nThis field is not used for external load balancing.\n\nFor Internal TCP/UDP Load Balancing, this field identifies the network that\nthe load balanced IP should belong to for this Forwarding Rule.\nIf the subnetwork is specified, the network of the subnetwork will be used.\nIf neither subnetwork nor this field is specified, the default network will\nbe used.\n\nFor Private Service Connect forwarding rules that forward traffic to Google\nAPIs, a network must be provided."]
    pub fn set_network(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network = Some(v.into());
        self
    }
    #[doc = "Set the field `network_tier`.\nThis signifies the networking tier used for configuring\nthis load balancer and can only take the following values:\n'PREMIUM', 'STANDARD'.\n\nFor regional ForwardingRule, the valid values are 'PREMIUM' and\n'STANDARD'. For GlobalForwardingRule, the valid value is\n'PREMIUM'.\n\nIf this field is not specified, it is assumed to be 'PREMIUM'.\nIf 'IPAddress' is specified, this value must be equal to the\nnetworkTier of the Address. Possible values: [\"PREMIUM\", \"STANDARD\"]"]
    pub fn set_network_tier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network_tier = Some(v.into());
        self
    }
    #[doc = "Set the field `no_automate_dns_zone`.\nThis is used in PSC consumer ForwardingRule to control whether it should try to auto-generate a DNS zone or not. Non-PSC forwarding rules do not use this field."]
    pub fn set_no_automate_dns_zone(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().no_automate_dns_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `port_range`.\nThe 'ports', 'portRange', and 'allPorts' fields are mutually exclusive.\nOnly packets addressed to ports in the specified range will be forwarded\nto the backends configured with this forwarding rule.\n\nThe 'portRange' field has the following limitations:\n* It requires that the forwarding rule 'IPProtocol' be TCP, UDP, or SCTP,\nand\n* It's applicable only to the following products: external passthrough\nNetwork Load Balancers, internal and external proxy Network Load\nBalancers, internal and external Application Load Balancers, external\nprotocol forwarding, and Classic VPN.\n* Some products have restrictions on what ports can be used. See\n[port specifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#port_specifications)\nfor details.\n\nFor external forwarding rules, two or more forwarding rules cannot use the\nsame '[IPAddress, IPProtocol]' pair, and cannot have overlapping\n'portRange's.\n\nFor internal forwarding rules within the same VPC network, two or more\nforwarding rules cannot use the same '[IPAddress, IPProtocol]' pair, and\ncannot have overlapping 'portRange's.\n\n@pattern: \\d+(?:-\\d+)?"]
    pub fn set_port_range(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().port_range = Some(v.into());
        self
    }
    #[doc = "Set the field `ports`.\nThe 'ports', 'portRange', and 'allPorts' fields are mutually exclusive.\nOnly packets addressed to ports in the specified range will be forwarded\nto the backends configured with this forwarding rule.\n\nThe 'ports' field has the following limitations:\n* It requires that the forwarding rule 'IPProtocol' be TCP, UDP, or SCTP,\nand\n* It's applicable only to the following products: internal passthrough\nNetwork Load Balancers, backend service-based external passthrough Network\nLoad Balancers, and internal protocol forwarding.\n* You can specify a list of up to five ports by number, separated by\ncommas. The ports can be contiguous or discontiguous.\n\nFor external forwarding rules, two or more forwarding rules cannot use the\nsame '[IPAddress, IPProtocol]' pair if they share at least one port\nnumber.\n\nFor internal forwarding rules within the same VPC network, two or more\nforwarding rules cannot use the same '[IPAddress, IPProtocol]' pair if\nthey share at least one port number.\n\n@pattern: \\d+(?:-\\d+)?"]
    pub fn set_ports(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().ports = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `recreate_closed_psc`.\nThis is used in PSC consumer ForwardingRule to make terraform recreate the ForwardingRule when the status is closed"]
    pub fn set_recreate_closed_psc(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().recreate_closed_psc = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nA reference to the region where the regional forwarding rule resides.\n\nThis field is not applicable to global forwarding rules."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `service_label`.\nAn optional prefix to the service name for this Forwarding Rule.\nIf specified, will be the first label of the fully qualified service\nname.\n\nThe label must be 1-63 characters long, and comply with RFC1035.\nSpecifically, the label must be 1-63 characters long and match the\nregular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first\ncharacter must be a lowercase letter, and all following characters\nmust be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash.\n\nThis field is only used for INTERNAL load balancing."]
    pub fn set_service_label(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_label = Some(v.into());
        self
    }
    #[doc = "Set the field `source_ip_ranges`.\nIf not empty, this Forwarding Rule will only forward the traffic when the source IP address matches one of the IP addresses or CIDR ranges set here. Note that a Forwarding Rule can only have up to 64 source IP ranges, and this field can only be used with a regional Forwarding Rule whose scheme is EXTERNAL. Each sourceIpRange entry should be either an IP address (for example, 1.2.3.4) or a CIDR range (for example, 1.2.3.0/24)."]
    pub fn set_source_ip_ranges(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().source_ip_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\nThis field identifies the subnetwork that the load balanced IP should\nbelong to for this Forwarding Rule, used in internal load balancing and\nnetwork load balancing with IPv6.\n\nIf the network specified is in auto subnet mode, this field is optional.\nHowever, a subnetwork must be specified if the network is in custom subnet\nmode or when creating external forwarding rule with IPv6."]
    pub fn set_subnetwork(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().subnetwork = Some(v.into());
        self
    }
    #[doc = "Set the field `target`.\nThe URL of the target resource to receive the matched traffic.  For\nregional forwarding rules, this target must be in the same region as the\nforwarding rule. For global forwarding rules, this target must be a global\nload balancing resource.\n\nThe forwarded traffic must be of a type appropriate to the target object.\n*  For load balancers, see the \"Target\" column in [Port specifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#ip_address_specifications).\n\nFor Private Service Connect forwarding rules that forward traffic to managed services, the target must be a service attachment."]
    pub fn set_target(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().target = Some(v.into());
        self
    }
    #[doc = "Set the field `service_directory_registrations`.\n"]
    pub fn set_service_directory_registrations(
        self,
        v: impl Into<BlockAssignable<ComputeForwardingRuleServiceDirectoryRegistrationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_directory_registrations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .service_directory_registrations = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeForwardingRuleTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `all_ports` after provisioning.\nThe 'ports', 'portRange', and 'allPorts' fields are mutually exclusive.\nOnly packets addressed to ports in the specified range will be forwarded\nto the backends configured with this forwarding rule.\n\nThe 'allPorts' field has the following limitations:\n* It requires that the forwarding rule 'IPProtocol' be TCP, UDP, SCTP, or\nL3_DEFAULT.\n* It's applicable only to the following products: internal passthrough\nNetwork Load Balancers, backend service-based external passthrough Network\nLoad Balancers, and internal and external protocol forwarding.\n* Set this field to true to allow packets addressed to any port or packets\nlacking destination port information (for example, UDP fragments after the\nfirst fragment) to be forwarded to the backends configured with this\nforwarding rule. The L3_DEFAULT protocol requires 'allPorts' be set to\ntrue."]
    pub fn all_ports(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.all_ports", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allow_global_access` after provisioning.\nThis field is used along with the 'backend_service' field for\ninternal load balancing or with the 'target' field for internal\nTargetInstance.\n\nIf the field is set to 'TRUE', clients can access ILB from all\nregions.\n\nOtherwise only allows access from clients in the same region as the\ninternal load balancer."]
    pub fn allow_global_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_global_access", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allow_psc_global_access` after provisioning.\nThis is used in PSC consumer ForwardingRule to control whether the PSC endpoint can be accessed from another region."]
    pub fn allow_psc_global_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_psc_global_access", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backend_service` after provisioning.\nIdentifies the backend service to which the forwarding rule sends traffic.\n\nRequired for Internal TCP/UDP Load Balancing and Network Load Balancing;\nmust be omitted for all other load balancer types."]
    pub fn backend_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backend_service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `base_forwarding_rule` after provisioning.\n[Output Only] The URL for the corresponding base Forwarding Rule. By base Forwarding Rule, we mean the Forwarding Rule that has the same IP address, protocol, and port settings with the current Forwarding Rule, but without sourceIPRanges specified. Always empty if the current Forwarding Rule does not have sourceIPRanges specified."]
    pub fn base_forwarding_rule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_forwarding_rule", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `forwarding_rule_id` after provisioning.\nThe unique identifier number for the resource. This identifier is defined by the server."]
    pub fn forwarding_rule_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.forwarding_rule_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\nIP address for which this forwarding rule accepts traffic. When a client\nsends traffic to this IP address, the forwarding rule directs the traffic\nto the referenced 'target' or 'backendService'.\n\nWhile creating a forwarding rule, specifying an 'IPAddress' is\nrequired under the following circumstances:\n\n* When the 'target' is set to 'targetGrpcProxy' and\n'validateForProxyless' is set to 'true', the\n'IPAddress' should be set to '0.0.0.0'.\n* When the 'target' is a Private Service Connect Google APIs\nbundle, you must specify an 'IPAddress'.\n\nOtherwise, you can optionally specify an IP address that references an\nexisting static (reserved) IP address resource. When omitted, Google Cloud\nassigns an ephemeral IP address.\n\nUse one of the following formats to specify an IP address while creating a\nforwarding rule:\n\n* IP address number, as in '100.1.2.3'\n* IPv6 address range, as in '2600:1234::/96'\n* Full resource URL, as in\n'https://www.googleapis.com/compute/v1/projects/project_id/regions/region/addresses/address-name'\n* Partial URL or by name, as in:\n  * 'projects/project_id/regions/region/addresses/address-name'\n  * 'regions/region/addresses/address-name'\n  * 'global/addresses/address-name'\n  * 'address-name'\n\nThe forwarding rule's 'target' or 'backendService',\nand in most cases, also the 'loadBalancingScheme', determine the\ntype of IP address that you can use. For detailed information, see\n[IP address\nspecifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#ip_address_specifications).\n\nWhen reading an 'IPAddress', the API always returns the IP\naddress number."]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ip_collection` after provisioning.\nResource reference of a PublicDelegatedPrefix. The PDP must be a sub-PDP\nin EXTERNAL_IPV6_FORWARDING_RULE_CREATION mode.\nUse one of the following formats to specify a sub-PDP when creating an\nIPv6 NetLB forwarding rule using BYOIP:\nFull resource URL, as in:\n  * 'https://www.googleapis.com/compute/v1/projects/{{projectId}}/regions/{{region}}/publicDelegatedPrefixes/{{sub-pdp-name}}'\nPartial URL, as in:\n  * 'projects/{{projectId}}/regions/region/publicDelegatedPrefixes/{{sub-pdp-name}}'\n  * 'regions/{{region}}/publicDelegatedPrefixes/{{sub-pdp-name}}'"]
    pub fn ip_collection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_collection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ip_protocol` after provisioning.\nThe IP protocol to which this rule applies.\n\nFor protocol forwarding, valid\noptions are 'TCP', 'UDP', 'ESP',\n'AH', 'SCTP', 'ICMP' and\n'L3_DEFAULT'.\n\nThe valid IP protocols are different for different load balancing products\nas described in [Load balancing\nfeatures](https://cloud.google.com/load-balancing/docs/features#protocols_from_the_load_balancer_to_the_backends).\n\nA Forwarding Rule with protocol L3_DEFAULT can attach with target instance or\nbackend service with UNSPECIFIED protocol.\nA forwarding rule with \"L3_DEFAULT\" IPProtocal cannot be attached to a backend service with TCP or UDP. Possible values: [\"TCP\", \"UDP\", \"ESP\", \"AH\", \"SCTP\", \"ICMP\", \"L3_DEFAULT\"]"]
    pub fn ip_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_protocol", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ip_version` after provisioning.\nThe IP address version that will be used by this forwarding rule.\nValid options are IPV4 and IPV6.\n\nIf not set, the IPv4 address will be used by default. Possible values: [\"IPV4\", \"IPV6\"]"]
    pub fn ip_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `is_mirroring_collector` after provisioning.\nIndicates whether or not this load balancer can be used as a collector for\npacket mirroring. To prevent mirroring loops, instances behind this\nload balancer will not have their traffic mirrored even if a\n'PacketMirroring' rule applies to them.\n\nThis can only be set to true for load balancers that have their\n'loadBalancingScheme' set to 'INTERNAL'."]
    pub fn is_mirroring_collector(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_mirroring_collector", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe fingerprint used for optimistic locking of this resource.  Used\ninternally during updates."]
    pub fn label_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label_fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this forwarding rule.  A list of key->value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancing_scheme` after provisioning.\nSpecifies the forwarding rule type.\n\nNote that an empty string value ('\"\"') is also supported for some use\ncases, for example PSC (private service connection) regional forwarding\nrules.\n\nFor more information about forwarding rules, refer to\n[Forwarding rule concepts](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts). Default value: \"EXTERNAL\" Possible values: [\"EXTERNAL\", \"EXTERNAL_MANAGED\", \"INTERNAL\", \"INTERNAL_MANAGED\"]"]
    pub fn load_balancing_scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancing_scheme", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is created.\nThe name must be 1-63 characters long, and comply with\n[RFC1035](https://www.ietf.org/rfc/rfc1035.txt).\n\nSpecifically, the name must be 1-63 characters long and match the regular\nexpression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first\ncharacter must be a lowercase letter, and all following characters must\nbe a dash, lowercase letter, or digit, except the last character, which\ncannot be a dash.\n\nFor Private Service Connect forwarding rules that forward traffic to Google\nAPIs, the forwarding rule name must be a 1-20 characters string with\nlowercase letters and numbers and must start with a letter."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThis field is not used for external load balancing.\n\nFor Internal TCP/UDP Load Balancing, this field identifies the network that\nthe load balanced IP should belong to for this Forwarding Rule.\nIf the subnetwork is specified, the network of the subnetwork will be used.\nIf neither subnetwork nor this field is specified, the default network will\nbe used.\n\nFor Private Service Connect forwarding rules that forward traffic to Google\nAPIs, a network must be provided."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_tier` after provisioning.\nThis signifies the networking tier used for configuring\nthis load balancer and can only take the following values:\n'PREMIUM', 'STANDARD'.\n\nFor regional ForwardingRule, the valid values are 'PREMIUM' and\n'STANDARD'. For GlobalForwardingRule, the valid value is\n'PREMIUM'.\n\nIf this field is not specified, it is assumed to be 'PREMIUM'.\nIf 'IPAddress' is specified, this value must be equal to the\nnetworkTier of the Address. Possible values: [\"PREMIUM\", \"STANDARD\"]"]
    pub fn network_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_tier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `no_automate_dns_zone` after provisioning.\nThis is used in PSC consumer ForwardingRule to control whether it should try to auto-generate a DNS zone or not. Non-PSC forwarding rules do not use this field."]
    pub fn no_automate_dns_zone(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.no_automate_dns_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `port_range` after provisioning.\nThe 'ports', 'portRange', and 'allPorts' fields are mutually exclusive.\nOnly packets addressed to ports in the specified range will be forwarded\nto the backends configured with this forwarding rule.\n\nThe 'portRange' field has the following limitations:\n* It requires that the forwarding rule 'IPProtocol' be TCP, UDP, or SCTP,\nand\n* It's applicable only to the following products: external passthrough\nNetwork Load Balancers, internal and external proxy Network Load\nBalancers, internal and external Application Load Balancers, external\nprotocol forwarding, and Classic VPN.\n* Some products have restrictions on what ports can be used. See\n[port specifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#port_specifications)\nfor details.\n\nFor external forwarding rules, two or more forwarding rules cannot use the\nsame '[IPAddress, IPProtocol]' pair, and cannot have overlapping\n'portRange's.\n\nFor internal forwarding rules within the same VPC network, two or more\nforwarding rules cannot use the same '[IPAddress, IPProtocol]' pair, and\ncannot have overlapping 'portRange's.\n\n@pattern: \\d+(?:-\\d+)?"]
    pub fn port_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ports` after provisioning.\nThe 'ports', 'portRange', and 'allPorts' fields are mutually exclusive.\nOnly packets addressed to ports in the specified range will be forwarded\nto the backends configured with this forwarding rule.\n\nThe 'ports' field has the following limitations:\n* It requires that the forwarding rule 'IPProtocol' be TCP, UDP, or SCTP,\nand\n* It's applicable only to the following products: internal passthrough\nNetwork Load Balancers, backend service-based external passthrough Network\nLoad Balancers, and internal protocol forwarding.\n* You can specify a list of up to five ports by number, separated by\ncommas. The ports can be contiguous or discontiguous.\n\nFor external forwarding rules, two or more forwarding rules cannot use the\nsame '[IPAddress, IPProtocol]' pair if they share at least one port\nnumber.\n\nFor internal forwarding rules within the same VPC network, two or more\nforwarding rules cannot use the same '[IPAddress, IPProtocol]' pair if\nthey share at least one port number.\n\n@pattern: \\d+(?:-\\d+)?"]
    pub fn ports(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.ports", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_connection_id` after provisioning.\nThe PSC connection id of the PSC Forwarding Rule."]
    pub fn psc_connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.psc_connection_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_connection_status` after provisioning.\nThe PSC connection status of the PSC Forwarding Rule. Possible values: 'STATUS_UNSPECIFIED', 'PENDING', 'ACCEPTED', 'REJECTED', 'CLOSED'"]
    pub fn psc_connection_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.psc_connection_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `recreate_closed_psc` after provisioning.\nThis is used in PSC consumer ForwardingRule to make terraform recreate the ForwardingRule when the status is closed"]
    pub fn recreate_closed_psc(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.recreate_closed_psc", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nA reference to the region where the regional forwarding rule resides.\n\nThis field is not applicable to global forwarding rules."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_label` after provisioning.\nAn optional prefix to the service name for this Forwarding Rule.\nIf specified, will be the first label of the fully qualified service\nname.\n\nThe label must be 1-63 characters long, and comply with RFC1035.\nSpecifically, the label must be 1-63 characters long and match the\nregular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first\ncharacter must be a lowercase letter, and all following characters\nmust be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash.\n\nThis field is only used for INTERNAL load balancing."]
    pub fn service_label(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_label", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_name` after provisioning.\nThe internal fully qualified service name for this Forwarding Rule.\n\nThis field is only used for INTERNAL load balancing."]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_ip_ranges` after provisioning.\nIf not empty, this Forwarding Rule will only forward the traffic when the source IP address matches one of the IP addresses or CIDR ranges set here. Note that a Forwarding Rule can only have up to 64 source IP ranges, and this field can only be used with a regional Forwarding Rule whose scheme is EXTERNAL. Each sourceIpRange entry should be either an IP address (for example, 1.2.3.4) or a CIDR range (for example, 1.2.3.0/24)."]
    pub fn source_ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_ip_ranges", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nThis field identifies the subnetwork that the load balanced IP should\nbelong to for this Forwarding Rule, used in internal load balancing and\nnetwork load balancing with IPv6.\n\nIf the network specified is in auto subnet mode, this field is optional.\nHowever, a subnetwork must be specified if the network is in custom subnet\nmode or when creating external forwarding rule with IPv6."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnetwork", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\nThe URL of the target resource to receive the matched traffic.  For\nregional forwarding rules, this target must be in the same region as the\nforwarding rule. For global forwarding rules, this target must be a global\nload balancing resource.\n\nThe forwarded traffic must be of a type appropriate to the target object.\n*  For load balancers, see the \"Target\" column in [Port specifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#ip_address_specifications).\n\nFor Private Service Connect forwarding rules that forward traffic to managed services, the target must be a service attachment."]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_directory_registrations` after provisioning.\n"]
    pub fn service_directory_registrations(
        &self,
    ) -> ListRef<ComputeForwardingRuleServiceDirectoryRegistrationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_directory_registrations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeForwardingRuleTimeoutsElRef {
        ComputeForwardingRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeForwardingRule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeForwardingRule {}
impl ToListMappable for ComputeForwardingRule {
    type O = ListRef<ComputeForwardingRuleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeForwardingRule_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_forwarding_rule".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeForwardingRule {
    pub tf_id: String,
    #[doc = "Name of the resource; provided by the client when the resource is created.\nThe name must be 1-63 characters long, and comply with\n[RFC1035](https://www.ietf.org/rfc/rfc1035.txt).\n\nSpecifically, the name must be 1-63 characters long and match the regular\nexpression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first\ncharacter must be a lowercase letter, and all following characters must\nbe a dash, lowercase letter, or digit, except the last character, which\ncannot be a dash.\n\nFor Private Service Connect forwarding rules that forward traffic to Google\nAPIs, the forwarding rule name must be a 1-20 characters string with\nlowercase letters and numbers and must start with a letter."]
    pub name: PrimField<String>,
}
impl BuildComputeForwardingRule {
    pub fn build(self, stack: &mut Stack) -> ComputeForwardingRule {
        let out = ComputeForwardingRule(Rc::new(ComputeForwardingRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeForwardingRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                all_ports: core::default::Default::default(),
                allow_global_access: core::default::Default::default(),
                allow_psc_global_access: core::default::Default::default(),
                backend_service: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                ip_address: core::default::Default::default(),
                ip_collection: core::default::Default::default(),
                ip_protocol: core::default::Default::default(),
                ip_version: core::default::Default::default(),
                is_mirroring_collector: core::default::Default::default(),
                labels: core::default::Default::default(),
                load_balancing_scheme: core::default::Default::default(),
                name: self.name,
                network: core::default::Default::default(),
                network_tier: core::default::Default::default(),
                no_automate_dns_zone: core::default::Default::default(),
                port_range: core::default::Default::default(),
                ports: core::default::Default::default(),
                project: core::default::Default::default(),
                recreate_closed_psc: core::default::Default::default(),
                region: core::default::Default::default(),
                service_label: core::default::Default::default(),
                source_ip_ranges: core::default::Default::default(),
                subnetwork: core::default::Default::default(),
                target: core::default::Default::default(),
                service_directory_registrations: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeForwardingRuleRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeForwardingRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeForwardingRuleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `all_ports` after provisioning.\nThe 'ports', 'portRange', and 'allPorts' fields are mutually exclusive.\nOnly packets addressed to ports in the specified range will be forwarded\nto the backends configured with this forwarding rule.\n\nThe 'allPorts' field has the following limitations:\n* It requires that the forwarding rule 'IPProtocol' be TCP, UDP, SCTP, or\nL3_DEFAULT.\n* It's applicable only to the following products: internal passthrough\nNetwork Load Balancers, backend service-based external passthrough Network\nLoad Balancers, and internal and external protocol forwarding.\n* Set this field to true to allow packets addressed to any port or packets\nlacking destination port information (for example, UDP fragments after the\nfirst fragment) to be forwarded to the backends configured with this\nforwarding rule. The L3_DEFAULT protocol requires 'allPorts' be set to\ntrue."]
    pub fn all_ports(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.all_ports", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allow_global_access` after provisioning.\nThis field is used along with the 'backend_service' field for\ninternal load balancing or with the 'target' field for internal\nTargetInstance.\n\nIf the field is set to 'TRUE', clients can access ILB from all\nregions.\n\nOtherwise only allows access from clients in the same region as the\ninternal load balancer."]
    pub fn allow_global_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_global_access", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allow_psc_global_access` after provisioning.\nThis is used in PSC consumer ForwardingRule to control whether the PSC endpoint can be accessed from another region."]
    pub fn allow_psc_global_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_psc_global_access", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backend_service` after provisioning.\nIdentifies the backend service to which the forwarding rule sends traffic.\n\nRequired for Internal TCP/UDP Load Balancing and Network Load Balancing;\nmust be omitted for all other load balancer types."]
    pub fn backend_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backend_service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `base_forwarding_rule` after provisioning.\n[Output Only] The URL for the corresponding base Forwarding Rule. By base Forwarding Rule, we mean the Forwarding Rule that has the same IP address, protocol, and port settings with the current Forwarding Rule, but without sourceIPRanges specified. Always empty if the current Forwarding Rule does not have sourceIPRanges specified."]
    pub fn base_forwarding_rule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_forwarding_rule", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `forwarding_rule_id` after provisioning.\nThe unique identifier number for the resource. This identifier is defined by the server."]
    pub fn forwarding_rule_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.forwarding_rule_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\nIP address for which this forwarding rule accepts traffic. When a client\nsends traffic to this IP address, the forwarding rule directs the traffic\nto the referenced 'target' or 'backendService'.\n\nWhile creating a forwarding rule, specifying an 'IPAddress' is\nrequired under the following circumstances:\n\n* When the 'target' is set to 'targetGrpcProxy' and\n'validateForProxyless' is set to 'true', the\n'IPAddress' should be set to '0.0.0.0'.\n* When the 'target' is a Private Service Connect Google APIs\nbundle, you must specify an 'IPAddress'.\n\nOtherwise, you can optionally specify an IP address that references an\nexisting static (reserved) IP address resource. When omitted, Google Cloud\nassigns an ephemeral IP address.\n\nUse one of the following formats to specify an IP address while creating a\nforwarding rule:\n\n* IP address number, as in '100.1.2.3'\n* IPv6 address range, as in '2600:1234::/96'\n* Full resource URL, as in\n'https://www.googleapis.com/compute/v1/projects/project_id/regions/region/addresses/address-name'\n* Partial URL or by name, as in:\n  * 'projects/project_id/regions/region/addresses/address-name'\n  * 'regions/region/addresses/address-name'\n  * 'global/addresses/address-name'\n  * 'address-name'\n\nThe forwarding rule's 'target' or 'backendService',\nand in most cases, also the 'loadBalancingScheme', determine the\ntype of IP address that you can use. For detailed information, see\n[IP address\nspecifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#ip_address_specifications).\n\nWhen reading an 'IPAddress', the API always returns the IP\naddress number."]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ip_collection` after provisioning.\nResource reference of a PublicDelegatedPrefix. The PDP must be a sub-PDP\nin EXTERNAL_IPV6_FORWARDING_RULE_CREATION mode.\nUse one of the following formats to specify a sub-PDP when creating an\nIPv6 NetLB forwarding rule using BYOIP:\nFull resource URL, as in:\n  * 'https://www.googleapis.com/compute/v1/projects/{{projectId}}/regions/{{region}}/publicDelegatedPrefixes/{{sub-pdp-name}}'\nPartial URL, as in:\n  * 'projects/{{projectId}}/regions/region/publicDelegatedPrefixes/{{sub-pdp-name}}'\n  * 'regions/{{region}}/publicDelegatedPrefixes/{{sub-pdp-name}}'"]
    pub fn ip_collection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_collection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ip_protocol` after provisioning.\nThe IP protocol to which this rule applies.\n\nFor protocol forwarding, valid\noptions are 'TCP', 'UDP', 'ESP',\n'AH', 'SCTP', 'ICMP' and\n'L3_DEFAULT'.\n\nThe valid IP protocols are different for different load balancing products\nas described in [Load balancing\nfeatures](https://cloud.google.com/load-balancing/docs/features#protocols_from_the_load_balancer_to_the_backends).\n\nA Forwarding Rule with protocol L3_DEFAULT can attach with target instance or\nbackend service with UNSPECIFIED protocol.\nA forwarding rule with \"L3_DEFAULT\" IPProtocal cannot be attached to a backend service with TCP or UDP. Possible values: [\"TCP\", \"UDP\", \"ESP\", \"AH\", \"SCTP\", \"ICMP\", \"L3_DEFAULT\"]"]
    pub fn ip_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_protocol", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ip_version` after provisioning.\nThe IP address version that will be used by this forwarding rule.\nValid options are IPV4 and IPV6.\n\nIf not set, the IPv4 address will be used by default. Possible values: [\"IPV4\", \"IPV6\"]"]
    pub fn ip_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `is_mirroring_collector` after provisioning.\nIndicates whether or not this load balancer can be used as a collector for\npacket mirroring. To prevent mirroring loops, instances behind this\nload balancer will not have their traffic mirrored even if a\n'PacketMirroring' rule applies to them.\n\nThis can only be set to true for load balancers that have their\n'loadBalancingScheme' set to 'INTERNAL'."]
    pub fn is_mirroring_collector(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_mirroring_collector", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe fingerprint used for optimistic locking of this resource.  Used\ninternally during updates."]
    pub fn label_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label_fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this forwarding rule.  A list of key->value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancing_scheme` after provisioning.\nSpecifies the forwarding rule type.\n\nNote that an empty string value ('\"\"') is also supported for some use\ncases, for example PSC (private service connection) regional forwarding\nrules.\n\nFor more information about forwarding rules, refer to\n[Forwarding rule concepts](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts). Default value: \"EXTERNAL\" Possible values: [\"EXTERNAL\", \"EXTERNAL_MANAGED\", \"INTERNAL\", \"INTERNAL_MANAGED\"]"]
    pub fn load_balancing_scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancing_scheme", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is created.\nThe name must be 1-63 characters long, and comply with\n[RFC1035](https://www.ietf.org/rfc/rfc1035.txt).\n\nSpecifically, the name must be 1-63 characters long and match the regular\nexpression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first\ncharacter must be a lowercase letter, and all following characters must\nbe a dash, lowercase letter, or digit, except the last character, which\ncannot be a dash.\n\nFor Private Service Connect forwarding rules that forward traffic to Google\nAPIs, the forwarding rule name must be a 1-20 characters string with\nlowercase letters and numbers and must start with a letter."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThis field is not used for external load balancing.\n\nFor Internal TCP/UDP Load Balancing, this field identifies the network that\nthe load balanced IP should belong to for this Forwarding Rule.\nIf the subnetwork is specified, the network of the subnetwork will be used.\nIf neither subnetwork nor this field is specified, the default network will\nbe used.\n\nFor Private Service Connect forwarding rules that forward traffic to Google\nAPIs, a network must be provided."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_tier` after provisioning.\nThis signifies the networking tier used for configuring\nthis load balancer and can only take the following values:\n'PREMIUM', 'STANDARD'.\n\nFor regional ForwardingRule, the valid values are 'PREMIUM' and\n'STANDARD'. For GlobalForwardingRule, the valid value is\n'PREMIUM'.\n\nIf this field is not specified, it is assumed to be 'PREMIUM'.\nIf 'IPAddress' is specified, this value must be equal to the\nnetworkTier of the Address. Possible values: [\"PREMIUM\", \"STANDARD\"]"]
    pub fn network_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_tier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `no_automate_dns_zone` after provisioning.\nThis is used in PSC consumer ForwardingRule to control whether it should try to auto-generate a DNS zone or not. Non-PSC forwarding rules do not use this field."]
    pub fn no_automate_dns_zone(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.no_automate_dns_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `port_range` after provisioning.\nThe 'ports', 'portRange', and 'allPorts' fields are mutually exclusive.\nOnly packets addressed to ports in the specified range will be forwarded\nto the backends configured with this forwarding rule.\n\nThe 'portRange' field has the following limitations:\n* It requires that the forwarding rule 'IPProtocol' be TCP, UDP, or SCTP,\nand\n* It's applicable only to the following products: external passthrough\nNetwork Load Balancers, internal and external proxy Network Load\nBalancers, internal and external Application Load Balancers, external\nprotocol forwarding, and Classic VPN.\n* Some products have restrictions on what ports can be used. See\n[port specifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#port_specifications)\nfor details.\n\nFor external forwarding rules, two or more forwarding rules cannot use the\nsame '[IPAddress, IPProtocol]' pair, and cannot have overlapping\n'portRange's.\n\nFor internal forwarding rules within the same VPC network, two or more\nforwarding rules cannot use the same '[IPAddress, IPProtocol]' pair, and\ncannot have overlapping 'portRange's.\n\n@pattern: \\d+(?:-\\d+)?"]
    pub fn port_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ports` after provisioning.\nThe 'ports', 'portRange', and 'allPorts' fields are mutually exclusive.\nOnly packets addressed to ports in the specified range will be forwarded\nto the backends configured with this forwarding rule.\n\nThe 'ports' field has the following limitations:\n* It requires that the forwarding rule 'IPProtocol' be TCP, UDP, or SCTP,\nand\n* It's applicable only to the following products: internal passthrough\nNetwork Load Balancers, backend service-based external passthrough Network\nLoad Balancers, and internal protocol forwarding.\n* You can specify a list of up to five ports by number, separated by\ncommas. The ports can be contiguous or discontiguous.\n\nFor external forwarding rules, two or more forwarding rules cannot use the\nsame '[IPAddress, IPProtocol]' pair if they share at least one port\nnumber.\n\nFor internal forwarding rules within the same VPC network, two or more\nforwarding rules cannot use the same '[IPAddress, IPProtocol]' pair if\nthey share at least one port number.\n\n@pattern: \\d+(?:-\\d+)?"]
    pub fn ports(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.ports", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_connection_id` after provisioning.\nThe PSC connection id of the PSC Forwarding Rule."]
    pub fn psc_connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.psc_connection_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_connection_status` after provisioning.\nThe PSC connection status of the PSC Forwarding Rule. Possible values: 'STATUS_UNSPECIFIED', 'PENDING', 'ACCEPTED', 'REJECTED', 'CLOSED'"]
    pub fn psc_connection_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.psc_connection_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `recreate_closed_psc` after provisioning.\nThis is used in PSC consumer ForwardingRule to make terraform recreate the ForwardingRule when the status is closed"]
    pub fn recreate_closed_psc(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.recreate_closed_psc", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nA reference to the region where the regional forwarding rule resides.\n\nThis field is not applicable to global forwarding rules."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_label` after provisioning.\nAn optional prefix to the service name for this Forwarding Rule.\nIf specified, will be the first label of the fully qualified service\nname.\n\nThe label must be 1-63 characters long, and comply with RFC1035.\nSpecifically, the label must be 1-63 characters long and match the\nregular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first\ncharacter must be a lowercase letter, and all following characters\nmust be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash.\n\nThis field is only used for INTERNAL load balancing."]
    pub fn service_label(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_label", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_name` after provisioning.\nThe internal fully qualified service name for this Forwarding Rule.\n\nThis field is only used for INTERNAL load balancing."]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_ip_ranges` after provisioning.\nIf not empty, this Forwarding Rule will only forward the traffic when the source IP address matches one of the IP addresses or CIDR ranges set here. Note that a Forwarding Rule can only have up to 64 source IP ranges, and this field can only be used with a regional Forwarding Rule whose scheme is EXTERNAL. Each sourceIpRange entry should be either an IP address (for example, 1.2.3.4) or a CIDR range (for example, 1.2.3.0/24)."]
    pub fn source_ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_ip_ranges", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nThis field identifies the subnetwork that the load balanced IP should\nbelong to for this Forwarding Rule, used in internal load balancing and\nnetwork load balancing with IPv6.\n\nIf the network specified is in auto subnet mode, this field is optional.\nHowever, a subnetwork must be specified if the network is in custom subnet\nmode or when creating external forwarding rule with IPv6."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnetwork", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\nThe URL of the target resource to receive the matched traffic.  For\nregional forwarding rules, this target must be in the same region as the\nforwarding rule. For global forwarding rules, this target must be a global\nload balancing resource.\n\nThe forwarded traffic must be of a type appropriate to the target object.\n*  For load balancers, see the \"Target\" column in [Port specifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#ip_address_specifications).\n\nFor Private Service Connect forwarding rules that forward traffic to managed services, the target must be a service attachment."]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_directory_registrations` after provisioning.\n"]
    pub fn service_directory_registrations(
        &self,
    ) -> ListRef<ComputeForwardingRuleServiceDirectoryRegistrationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_directory_registrations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeForwardingRuleTimeoutsElRef {
        ComputeForwardingRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeForwardingRuleServiceDirectoryRegistrationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
}
impl ComputeForwardingRuleServiceDirectoryRegistrationsEl {
    #[doc = "Set the field `namespace`.\nService Directory namespace to register the forwarding rule under."]
    pub fn set_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespace = Some(v.into());
        self
    }
    #[doc = "Set the field `service`.\nService Directory service to register the forwarding rule under."]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeForwardingRuleServiceDirectoryRegistrationsEl {
    type O = BlockAssignable<ComputeForwardingRuleServiceDirectoryRegistrationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeForwardingRuleServiceDirectoryRegistrationsEl {}
impl BuildComputeForwardingRuleServiceDirectoryRegistrationsEl {
    pub fn build(self) -> ComputeForwardingRuleServiceDirectoryRegistrationsEl {
        ComputeForwardingRuleServiceDirectoryRegistrationsEl {
            namespace: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}
pub struct ComputeForwardingRuleServiceDirectoryRegistrationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeForwardingRuleServiceDirectoryRegistrationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeForwardingRuleServiceDirectoryRegistrationsElRef {
        ComputeForwardingRuleServiceDirectoryRegistrationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeForwardingRuleServiceDirectoryRegistrationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\nService Directory namespace to register the forwarding rule under."]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nService Directory service to register the forwarding rule under."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeForwardingRuleTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeForwardingRuleTimeoutsEl {
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
impl ToListMappable for ComputeForwardingRuleTimeoutsEl {
    type O = BlockAssignable<ComputeForwardingRuleTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeForwardingRuleTimeoutsEl {}
impl BuildComputeForwardingRuleTimeoutsEl {
    pub fn build(self) -> ComputeForwardingRuleTimeoutsEl {
        ComputeForwardingRuleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeForwardingRuleTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeForwardingRuleTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeForwardingRuleTimeoutsElRef {
        ComputeForwardingRuleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeForwardingRuleTimeoutsElRef {
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
struct ComputeForwardingRuleDynamic {
    service_directory_registrations:
        Option<DynamicBlock<ComputeForwardingRuleServiceDirectoryRegistrationsEl>>,
}
