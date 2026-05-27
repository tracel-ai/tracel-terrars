use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeRouterNatData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_network_tier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drain_nat_ips: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_dynamic_port_allocation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_endpoint_independent_mapping: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_types: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icmp_idle_timeout_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_nat_ips: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_ports_per_vm: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_ports_per_vm: Option<PrimField<f64>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nat_ip_allocate_option: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nat_ips: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    router: PrimField<String>,
    source_subnetwork_ip_ranges_to_nat: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_subnetwork_ip_ranges_to_nat64: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_established_idle_timeout_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_time_wait_timeout_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_transitory_idle_timeout_sec: Option<PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    udp_idle_timeout_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_config: Option<Vec<ComputeRouterNatLogConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nat64_subnetwork: Option<Vec<ComputeRouterNatNat64SubnetworkEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<Vec<ComputeRouterNatRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<Vec<ComputeRouterNatSubnetworkEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeRouterNatTimeoutsEl>,
    dynamic: ComputeRouterNatDynamic,
}
struct ComputeRouterNat_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeRouterNatData>,
}
#[derive(Clone)]
pub struct ComputeRouterNat(Rc<ComputeRouterNat_>);
impl ComputeRouterNat {
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
    #[doc = "Set the field `auto_network_tier`.\nThe network tier to use when automatically reserving NAT IP addresses.\nMust be one of: PREMIUM, STANDARD. If not specified, then the current\nproject-level default tier is used. Possible values: [\"PREMIUM\", \"STANDARD\"]"]
    pub fn set_auto_network_tier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().auto_network_tier = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `drain_nat_ips`.\nA list of URLs of the IP resources to be drained. These IPs must be\nvalid static external IPs that have been assigned to the NAT."]
    pub fn set_drain_nat_ips(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().drain_nat_ips = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_dynamic_port_allocation`.\nEnable Dynamic Port Allocation.\nIf minPortsPerVm is set, minPortsPerVm must be set to a power of two greater than or equal to 32.\nIf minPortsPerVm is not set, a minimum of 32 ports will be allocated to a VM from this NAT config.\nIf maxPortsPerVm is set, maxPortsPerVm must be set to a power of two greater than minPortsPerVm.\nIf maxPortsPerVm is not set, a maximum of 65536 ports will be allocated to a VM from this NAT config.\n\nMutually exclusive with enableEndpointIndependentMapping."]
    pub fn set_enable_dynamic_port_allocation(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_dynamic_port_allocation = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_endpoint_independent_mapping`.\nEnable endpoint independent mapping.\nFor more information see the [official documentation](https://docs.cloud.google.com/nat/docs/public-nat#specs-rfcs)."]
    pub fn set_enable_endpoint_independent_mapping(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_endpoint_independent_mapping = Some(v.into());
        self
    }
    #[doc = "Set the field `endpoint_types`.\nSpecifies the endpoint Types supported by the NAT Gateway.\nSupported values include:\n      'ENDPOINT_TYPE_VM', 'ENDPOINT_TYPE_SWG',\n      'ENDPOINT_TYPE_MANAGED_PROXY_LB'."]
    pub fn set_endpoint_types(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().endpoint_types = Some(v.into());
        self
    }
    #[doc = "Set the field `icmp_idle_timeout_sec`.\nTimeout (in seconds) for ICMP connections. Defaults to 30s if not set."]
    pub fn set_icmp_idle_timeout_sec(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().icmp_idle_timeout_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `initial_nat_ips`.\nSelf-links of NAT IPs to be used as initial value for creation alongside a RouterNatAddress resource.\nConflicts with natIps and drainNatIps. Only valid if natIpAllocateOption is set to MANUAL_ONLY."]
    pub fn set_initial_nat_ips(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().initial_nat_ips = Some(v.into());
        self
    }
    #[doc = "Set the field `max_ports_per_vm`.\nMaximum number of ports allocated to a VM from this NAT.\nThis field can only be set when enableDynamicPortAllocation is enabled."]
    pub fn set_max_ports_per_vm(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_ports_per_vm = Some(v.into());
        self
    }
    #[doc = "Set the field `min_ports_per_vm`.\nMinimum number of ports allocated to a VM from this NAT. Defaults to 64 for static port allocation and 32 dynamic port allocation if not set."]
    pub fn set_min_ports_per_vm(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().min_ports_per_vm = Some(v.into());
        self
    }
    #[doc = "Set the field `nat_ip_allocate_option`.\nHow external IPs should be allocated for this NAT. Valid values are\n'AUTO_ONLY' for only allowing NAT IPs allocated by Google Cloud\nPlatform, or 'MANUAL_ONLY' for only user-allocated NAT IP addresses. Possible values: [\"MANUAL_ONLY\", \"AUTO_ONLY\"]"]
    pub fn set_nat_ip_allocate_option(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().nat_ip_allocate_option = Some(v.into());
        self
    }
    #[doc = "Set the field `nat_ips`.\nSelf-links of NAT IPs. Only valid if natIpAllocateOption\nis set to MANUAL_ONLY.\nIf this field is used alongside with a count created list of address resources 'google_compute_address.foobar.*.self_link',\nthe access level resource for the address resource must have a 'lifecycle' block with 'create_before_destroy = true' so\nthe number of resources can be increased/decreased without triggering the 'resourceInUseByAnotherResource' error."]
    pub fn set_nat_ips(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().nat_ips = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where the router and NAT reside."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `source_subnetwork_ip_ranges_to_nat64`.\nSpecify the Nat option for NAT64, which can take one of the following values:\nALL_IPV6_SUBNETWORKS: All of the IP ranges in every Subnetwork are allowed to Nat.\nLIST_OF_IPV6_SUBNETWORKS: A list of Subnetworks are allowed to Nat (specified in the field nat64Subnetwork below).\nNote that if this field contains NAT64_ALL_V6_SUBNETWORKS no other Router.Nat section in this region can also enable NAT64 for any Subnetworks in this network.\nOther Router.Nat sections can still be present to enable NAT44 only. Possible values: [\"ALL_IPV6_SUBNETWORKS\", \"LIST_OF_IPV6_SUBNETWORKS\"]"]
    pub fn set_source_subnetwork_ip_ranges_to_nat64(self, v: impl Into<PrimField<String>>) -> Self {
        self.0
            .data
            .borrow_mut()
            .source_subnetwork_ip_ranges_to_nat64 = Some(v.into());
        self
    }
    #[doc = "Set the field `tcp_established_idle_timeout_sec`.\nTimeout (in seconds) for TCP established connections.\nDefaults to 1200s if not set."]
    pub fn set_tcp_established_idle_timeout_sec(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tcp_established_idle_timeout_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `tcp_time_wait_timeout_sec`.\nTimeout (in seconds) for TCP connections that are in TIME_WAIT state.\nDefaults to 120s if not set."]
    pub fn set_tcp_time_wait_timeout_sec(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tcp_time_wait_timeout_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `tcp_transitory_idle_timeout_sec`.\nTimeout (in seconds) for TCP transitory connections.\nDefaults to 30s if not set."]
    pub fn set_tcp_transitory_idle_timeout_sec(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tcp_transitory_idle_timeout_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nIndicates whether this NAT is used for public or private IP translation.\nIf unspecified, it defaults to PUBLIC.\nIf 'PUBLIC' NAT used for public IP translation.\nIf 'PRIVATE' NAT used for private IP translation. Default value: \"PUBLIC\" Possible values: [\"PUBLIC\", \"PRIVATE\"]"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `udp_idle_timeout_sec`.\nTimeout (in seconds) for UDP connections. Defaults to 30s if not set."]
    pub fn set_udp_idle_timeout_sec(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().udp_idle_timeout_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `log_config`.\n"]
    pub fn set_log_config(
        self,
        v: impl Into<BlockAssignable<ComputeRouterNatLogConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `nat64_subnetwork`.\n"]
    pub fn set_nat64_subnetwork(
        self,
        v: impl Into<BlockAssignable<ComputeRouterNatNat64SubnetworkEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().nat64_subnetwork = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.nat64_subnetwork = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(self, v: impl Into<BlockAssignable<ComputeRouterNatRulesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rules = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `subnetwork`.\n"]
    pub fn set_subnetwork(
        self,
        v: impl Into<BlockAssignable<ComputeRouterNatSubnetworkEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().subnetwork = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.subnetwork = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeRouterNatTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `auto_network_tier` after provisioning.\nThe network tier to use when automatically reserving NAT IP addresses.\nMust be one of: PREMIUM, STANDARD. If not specified, then the current\nproject-level default tier is used. Possible values: [\"PREMIUM\", \"STANDARD\"]"]
    pub fn auto_network_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_network_tier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `drain_nat_ips` after provisioning.\nA list of URLs of the IP resources to be drained. These IPs must be\nvalid static external IPs that have been assigned to the NAT."]
    pub fn drain_nat_ips(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.drain_nat_ips", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_dynamic_port_allocation` after provisioning.\nEnable Dynamic Port Allocation.\nIf minPortsPerVm is set, minPortsPerVm must be set to a power of two greater than or equal to 32.\nIf minPortsPerVm is not set, a minimum of 32 ports will be allocated to a VM from this NAT config.\nIf maxPortsPerVm is set, maxPortsPerVm must be set to a power of two greater than minPortsPerVm.\nIf maxPortsPerVm is not set, a maximum of 65536 ports will be allocated to a VM from this NAT config.\n\nMutually exclusive with enableEndpointIndependentMapping."]
    pub fn enable_dynamic_port_allocation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_dynamic_port_allocation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_endpoint_independent_mapping` after provisioning.\nEnable endpoint independent mapping.\nFor more information see the [official documentation](https://docs.cloud.google.com/nat/docs/public-nat#specs-rfcs)."]
    pub fn enable_endpoint_independent_mapping(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_endpoint_independent_mapping", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_types` after provisioning.\nSpecifies the endpoint Types supported by the NAT Gateway.\nSupported values include:\n      'ENDPOINT_TYPE_VM', 'ENDPOINT_TYPE_SWG',\n      'ENDPOINT_TYPE_MANAGED_PROXY_LB'."]
    pub fn endpoint_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `icmp_idle_timeout_sec` after provisioning.\nTimeout (in seconds) for ICMP connections. Defaults to 30s if not set."]
    pub fn icmp_idle_timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.icmp_idle_timeout_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `initial_nat_ips` after provisioning.\nSelf-links of NAT IPs to be used as initial value for creation alongside a RouterNatAddress resource.\nConflicts with natIps and drainNatIps. Only valid if natIpAllocateOption is set to MANUAL_ONLY."]
    pub fn initial_nat_ips(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.initial_nat_ips", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_ports_per_vm` after provisioning.\nMaximum number of ports allocated to a VM from this NAT.\nThis field can only be set when enableDynamicPortAllocation is enabled."]
    pub fn max_ports_per_vm(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_ports_per_vm", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `min_ports_per_vm` after provisioning.\nMinimum number of ports allocated to a VM from this NAT. Defaults to 64 for static port allocation and 32 dynamic port allocation if not set."]
    pub fn min_ports_per_vm(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_ports_per_vm", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the NAT service. The name must be 1-63 characters long and\ncomply with RFC1035."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `nat_ip_allocate_option` after provisioning.\nHow external IPs should be allocated for this NAT. Valid values are\n'AUTO_ONLY' for only allowing NAT IPs allocated by Google Cloud\nPlatform, or 'MANUAL_ONLY' for only user-allocated NAT IP addresses. Possible values: [\"MANUAL_ONLY\", \"AUTO_ONLY\"]"]
    pub fn nat_ip_allocate_option(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nat_ip_allocate_option", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `nat_ips` after provisioning.\nSelf-links of NAT IPs. Only valid if natIpAllocateOption\nis set to MANUAL_ONLY.\nIf this field is used alongside with a count created list of address resources 'google_compute_address.foobar.*.self_link',\nthe access level resource for the address resource must have a 'lifecycle' block with 'create_before_destroy = true' so\nthe number of resources can be increased/decreased without triggering the 'resourceInUseByAnotherResource' error."]
    pub fn nat_ips(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.nat_ips", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where the router and NAT reside."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `router` after provisioning.\nThe name of the Cloud Router in which this NAT will be configured."]
    pub fn router(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.router", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_subnetwork_ip_ranges_to_nat` after provisioning.\nHow NAT should be configured per Subnetwork.\nIf 'ALL_SUBNETWORKS_ALL_IP_RANGES', all of the\nIP ranges in every Subnetwork are allowed to Nat.\nIf 'ALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES', all of the primary IP\nranges in every Subnetwork are allowed to Nat.\n'LIST_OF_SUBNETWORKS': A list of Subnetworks are allowed to Nat\n(specified in the field subnetwork below). Note that if this field\ncontains ALL_SUBNETWORKS_ALL_IP_RANGES or\nALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES, then there should not be any\nother RouterNat section in any Router for this network in this region. Possible values: [\"ALL_SUBNETWORKS_ALL_IP_RANGES\", \"ALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES\", \"LIST_OF_SUBNETWORKS\"]"]
    pub fn source_subnetwork_ip_ranges_to_nat(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_subnetwork_ip_ranges_to_nat", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_subnetwork_ip_ranges_to_nat64` after provisioning.\nSpecify the Nat option for NAT64, which can take one of the following values:\nALL_IPV6_SUBNETWORKS: All of the IP ranges in every Subnetwork are allowed to Nat.\nLIST_OF_IPV6_SUBNETWORKS: A list of Subnetworks are allowed to Nat (specified in the field nat64Subnetwork below).\nNote that if this field contains NAT64_ALL_V6_SUBNETWORKS no other Router.Nat section in this region can also enable NAT64 for any Subnetworks in this network.\nOther Router.Nat sections can still be present to enable NAT44 only. Possible values: [\"ALL_IPV6_SUBNETWORKS\", \"LIST_OF_IPV6_SUBNETWORKS\"]"]
    pub fn source_subnetwork_ip_ranges_to_nat64(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.source_subnetwork_ip_ranges_to_nat64",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `tcp_established_idle_timeout_sec` after provisioning.\nTimeout (in seconds) for TCP established connections.\nDefaults to 1200s if not set."]
    pub fn tcp_established_idle_timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tcp_established_idle_timeout_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tcp_time_wait_timeout_sec` after provisioning.\nTimeout (in seconds) for TCP connections that are in TIME_WAIT state.\nDefaults to 120s if not set."]
    pub fn tcp_time_wait_timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tcp_time_wait_timeout_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tcp_transitory_idle_timeout_sec` after provisioning.\nTimeout (in seconds) for TCP transitory connections.\nDefaults to 30s if not set."]
    pub fn tcp_transitory_idle_timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tcp_transitory_idle_timeout_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nIndicates whether this NAT is used for public or private IP translation.\nIf unspecified, it defaults to PUBLIC.\nIf 'PUBLIC' NAT used for public IP translation.\nIf 'PRIVATE' NAT used for private IP translation. Default value: \"PUBLIC\" Possible values: [\"PUBLIC\", \"PRIVATE\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `udp_idle_timeout_sec` after provisioning.\nTimeout (in seconds) for UDP connections. Defaults to 30s if not set."]
    pub fn udp_idle_timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.udp_idle_timeout_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> ListRef<ComputeRouterNatLogConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRouterNatTimeoutsElRef {
        ComputeRouterNatTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeRouterNat {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeRouterNat {}
impl ToListMappable for ComputeRouterNat {
    type O = ListRef<ComputeRouterNatRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeRouterNat_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_router_nat".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeRouterNat {
    pub tf_id: String,
    #[doc = "Name of the NAT service. The name must be 1-63 characters long and\ncomply with RFC1035."]
    pub name: PrimField<String>,
    #[doc = "The name of the Cloud Router in which this NAT will be configured."]
    pub router: PrimField<String>,
    #[doc = "How NAT should be configured per Subnetwork.\nIf 'ALL_SUBNETWORKS_ALL_IP_RANGES', all of the\nIP ranges in every Subnetwork are allowed to Nat.\nIf 'ALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES', all of the primary IP\nranges in every Subnetwork are allowed to Nat.\n'LIST_OF_SUBNETWORKS': A list of Subnetworks are allowed to Nat\n(specified in the field subnetwork below). Note that if this field\ncontains ALL_SUBNETWORKS_ALL_IP_RANGES or\nALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES, then there should not be any\nother RouterNat section in any Router for this network in this region. Possible values: [\"ALL_SUBNETWORKS_ALL_IP_RANGES\", \"ALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES\", \"LIST_OF_SUBNETWORKS\"]"]
    pub source_subnetwork_ip_ranges_to_nat: PrimField<String>,
}
impl BuildComputeRouterNat {
    pub fn build(self, stack: &mut Stack) -> ComputeRouterNat {
        let out = ComputeRouterNat(Rc::new(ComputeRouterNat_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeRouterNatData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                auto_network_tier: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                drain_nat_ips: core::default::Default::default(),
                enable_dynamic_port_allocation: core::default::Default::default(),
                enable_endpoint_independent_mapping: core::default::Default::default(),
                endpoint_types: core::default::Default::default(),
                icmp_idle_timeout_sec: core::default::Default::default(),
                id: core::default::Default::default(),
                initial_nat_ips: core::default::Default::default(),
                max_ports_per_vm: core::default::Default::default(),
                min_ports_per_vm: core::default::Default::default(),
                name: self.name,
                nat_ip_allocate_option: core::default::Default::default(),
                nat_ips: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                router: self.router,
                source_subnetwork_ip_ranges_to_nat: self.source_subnetwork_ip_ranges_to_nat,
                source_subnetwork_ip_ranges_to_nat64: core::default::Default::default(),
                tcp_established_idle_timeout_sec: core::default::Default::default(),
                tcp_time_wait_timeout_sec: core::default::Default::default(),
                tcp_transitory_idle_timeout_sec: core::default::Default::default(),
                type_: core::default::Default::default(),
                udp_idle_timeout_sec: core::default::Default::default(),
                log_config: core::default::Default::default(),
                nat64_subnetwork: core::default::Default::default(),
                rules: core::default::Default::default(),
                subnetwork: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeRouterNatRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouterNatRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeRouterNatRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto_network_tier` after provisioning.\nThe network tier to use when automatically reserving NAT IP addresses.\nMust be one of: PREMIUM, STANDARD. If not specified, then the current\nproject-level default tier is used. Possible values: [\"PREMIUM\", \"STANDARD\"]"]
    pub fn auto_network_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_network_tier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `drain_nat_ips` after provisioning.\nA list of URLs of the IP resources to be drained. These IPs must be\nvalid static external IPs that have been assigned to the NAT."]
    pub fn drain_nat_ips(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.drain_nat_ips", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_dynamic_port_allocation` after provisioning.\nEnable Dynamic Port Allocation.\nIf minPortsPerVm is set, minPortsPerVm must be set to a power of two greater than or equal to 32.\nIf minPortsPerVm is not set, a minimum of 32 ports will be allocated to a VM from this NAT config.\nIf maxPortsPerVm is set, maxPortsPerVm must be set to a power of two greater than minPortsPerVm.\nIf maxPortsPerVm is not set, a maximum of 65536 ports will be allocated to a VM from this NAT config.\n\nMutually exclusive with enableEndpointIndependentMapping."]
    pub fn enable_dynamic_port_allocation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_dynamic_port_allocation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_endpoint_independent_mapping` after provisioning.\nEnable endpoint independent mapping.\nFor more information see the [official documentation](https://docs.cloud.google.com/nat/docs/public-nat#specs-rfcs)."]
    pub fn enable_endpoint_independent_mapping(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_endpoint_independent_mapping", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_types` after provisioning.\nSpecifies the endpoint Types supported by the NAT Gateway.\nSupported values include:\n      'ENDPOINT_TYPE_VM', 'ENDPOINT_TYPE_SWG',\n      'ENDPOINT_TYPE_MANAGED_PROXY_LB'."]
    pub fn endpoint_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `icmp_idle_timeout_sec` after provisioning.\nTimeout (in seconds) for ICMP connections. Defaults to 30s if not set."]
    pub fn icmp_idle_timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.icmp_idle_timeout_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `initial_nat_ips` after provisioning.\nSelf-links of NAT IPs to be used as initial value for creation alongside a RouterNatAddress resource.\nConflicts with natIps and drainNatIps. Only valid if natIpAllocateOption is set to MANUAL_ONLY."]
    pub fn initial_nat_ips(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.initial_nat_ips", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_ports_per_vm` after provisioning.\nMaximum number of ports allocated to a VM from this NAT.\nThis field can only be set when enableDynamicPortAllocation is enabled."]
    pub fn max_ports_per_vm(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_ports_per_vm", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `min_ports_per_vm` after provisioning.\nMinimum number of ports allocated to a VM from this NAT. Defaults to 64 for static port allocation and 32 dynamic port allocation if not set."]
    pub fn min_ports_per_vm(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_ports_per_vm", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the NAT service. The name must be 1-63 characters long and\ncomply with RFC1035."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `nat_ip_allocate_option` after provisioning.\nHow external IPs should be allocated for this NAT. Valid values are\n'AUTO_ONLY' for only allowing NAT IPs allocated by Google Cloud\nPlatform, or 'MANUAL_ONLY' for only user-allocated NAT IP addresses. Possible values: [\"MANUAL_ONLY\", \"AUTO_ONLY\"]"]
    pub fn nat_ip_allocate_option(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nat_ip_allocate_option", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `nat_ips` after provisioning.\nSelf-links of NAT IPs. Only valid if natIpAllocateOption\nis set to MANUAL_ONLY.\nIf this field is used alongside with a count created list of address resources 'google_compute_address.foobar.*.self_link',\nthe access level resource for the address resource must have a 'lifecycle' block with 'create_before_destroy = true' so\nthe number of resources can be increased/decreased without triggering the 'resourceInUseByAnotherResource' error."]
    pub fn nat_ips(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.nat_ips", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where the router and NAT reside."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `router` after provisioning.\nThe name of the Cloud Router in which this NAT will be configured."]
    pub fn router(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.router", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_subnetwork_ip_ranges_to_nat` after provisioning.\nHow NAT should be configured per Subnetwork.\nIf 'ALL_SUBNETWORKS_ALL_IP_RANGES', all of the\nIP ranges in every Subnetwork are allowed to Nat.\nIf 'ALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES', all of the primary IP\nranges in every Subnetwork are allowed to Nat.\n'LIST_OF_SUBNETWORKS': A list of Subnetworks are allowed to Nat\n(specified in the field subnetwork below). Note that if this field\ncontains ALL_SUBNETWORKS_ALL_IP_RANGES or\nALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES, then there should not be any\nother RouterNat section in any Router for this network in this region. Possible values: [\"ALL_SUBNETWORKS_ALL_IP_RANGES\", \"ALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES\", \"LIST_OF_SUBNETWORKS\"]"]
    pub fn source_subnetwork_ip_ranges_to_nat(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_subnetwork_ip_ranges_to_nat", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_subnetwork_ip_ranges_to_nat64` after provisioning.\nSpecify the Nat option for NAT64, which can take one of the following values:\nALL_IPV6_SUBNETWORKS: All of the IP ranges in every Subnetwork are allowed to Nat.\nLIST_OF_IPV6_SUBNETWORKS: A list of Subnetworks are allowed to Nat (specified in the field nat64Subnetwork below).\nNote that if this field contains NAT64_ALL_V6_SUBNETWORKS no other Router.Nat section in this region can also enable NAT64 for any Subnetworks in this network.\nOther Router.Nat sections can still be present to enable NAT44 only. Possible values: [\"ALL_IPV6_SUBNETWORKS\", \"LIST_OF_IPV6_SUBNETWORKS\"]"]
    pub fn source_subnetwork_ip_ranges_to_nat64(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.source_subnetwork_ip_ranges_to_nat64",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `tcp_established_idle_timeout_sec` after provisioning.\nTimeout (in seconds) for TCP established connections.\nDefaults to 1200s if not set."]
    pub fn tcp_established_idle_timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tcp_established_idle_timeout_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tcp_time_wait_timeout_sec` after provisioning.\nTimeout (in seconds) for TCP connections that are in TIME_WAIT state.\nDefaults to 120s if not set."]
    pub fn tcp_time_wait_timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tcp_time_wait_timeout_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tcp_transitory_idle_timeout_sec` after provisioning.\nTimeout (in seconds) for TCP transitory connections.\nDefaults to 30s if not set."]
    pub fn tcp_transitory_idle_timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tcp_transitory_idle_timeout_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nIndicates whether this NAT is used for public or private IP translation.\nIf unspecified, it defaults to PUBLIC.\nIf 'PUBLIC' NAT used for public IP translation.\nIf 'PRIVATE' NAT used for private IP translation. Default value: \"PUBLIC\" Possible values: [\"PUBLIC\", \"PRIVATE\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `udp_idle_timeout_sec` after provisioning.\nTimeout (in seconds) for UDP connections. Defaults to 30s if not set."]
    pub fn udp_idle_timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.udp_idle_timeout_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> ListRef<ComputeRouterNatLogConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRouterNatTimeoutsElRef {
        ComputeRouterNatTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRouterNatLogConfigEl {
    enable: PrimField<bool>,
    filter: PrimField<String>,
}
impl ComputeRouterNatLogConfigEl {}
impl ToListMappable for ComputeRouterNatLogConfigEl {
    type O = BlockAssignable<ComputeRouterNatLogConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRouterNatLogConfigEl {
    #[doc = "Indicates whether or not to export logs."]
    pub enable: PrimField<bool>,
    #[doc = "Specifies the desired filtering of logs on this NAT. Possible values: [\"ERRORS_ONLY\", \"TRANSLATIONS_ONLY\", \"ALL\"]"]
    pub filter: PrimField<String>,
}
impl BuildComputeRouterNatLogConfigEl {
    pub fn build(self) -> ComputeRouterNatLogConfigEl {
        ComputeRouterNatLogConfigEl {
            enable: self.enable,
            filter: self.filter,
        }
    }
}
pub struct ComputeRouterNatLogConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouterNatLogConfigElRef {
    fn new(shared: StackShared, base: String) -> ComputeRouterNatLogConfigElRef {
        ComputeRouterNatLogConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRouterNatLogConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable` after provisioning.\nIndicates whether or not to export logs."]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nSpecifies the desired filtering of logs on this NAT. Possible values: [\"ERRORS_ONLY\", \"TRANSLATIONS_ONLY\", \"ALL\"]"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRouterNatNat64SubnetworkEl {
    name: PrimField<String>,
}
impl ComputeRouterNatNat64SubnetworkEl {}
impl ToListMappable for ComputeRouterNatNat64SubnetworkEl {
    type O = BlockAssignable<ComputeRouterNatNat64SubnetworkEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRouterNatNat64SubnetworkEl {
    #[doc = "Self-link of the subnetwork resource that will use NAT64"]
    pub name: PrimField<String>,
}
impl BuildComputeRouterNatNat64SubnetworkEl {
    pub fn build(self) -> ComputeRouterNatNat64SubnetworkEl {
        ComputeRouterNatNat64SubnetworkEl { name: self.name }
    }
}
pub struct ComputeRouterNatNat64SubnetworkElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouterNatNat64SubnetworkElRef {
    fn new(shared: StackShared, base: String) -> ComputeRouterNatNat64SubnetworkElRef {
        ComputeRouterNatNat64SubnetworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRouterNatNat64SubnetworkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nSelf-link of the subnetwork resource that will use NAT64"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRouterNatRulesElActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    source_nat_active_ips: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_nat_active_ranges: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_nat_drain_ips: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_nat_drain_ranges: Option<SetField<PrimField<String>>>,
}
impl ComputeRouterNatRulesElActionEl {
    #[doc = "Set the field `source_nat_active_ips`.\nA list of URLs of the IP resources used for this NAT rule.\nThese IP addresses must be valid static external IP addresses assigned to the project.\nThis field is used for public NAT."]
    pub fn set_source_nat_active_ips(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.source_nat_active_ips = Some(v.into());
        self
    }
    #[doc = "Set the field `source_nat_active_ranges`.\nA list of URLs of the subnetworks used as source ranges for this NAT Rule.\nThese subnetworks must have purpose set to PRIVATE_NAT.\nThis field is used for private NAT."]
    pub fn set_source_nat_active_ranges(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.source_nat_active_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `source_nat_drain_ips`.\nA list of URLs of the IP resources to be drained.\nThese IPs must be valid static external IPs that have been assigned to the NAT.\nThese IPs should be used for updating/patching a NAT rule only.\nThis field is used for public NAT."]
    pub fn set_source_nat_drain_ips(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.source_nat_drain_ips = Some(v.into());
        self
    }
    #[doc = "Set the field `source_nat_drain_ranges`.\nA list of URLs of subnetworks representing source ranges to be drained.\nThis is only supported on patch/update, and these subnetworks must have previously been used as active ranges in this NAT Rule.\nThis field is used for private NAT."]
    pub fn set_source_nat_drain_ranges(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.source_nat_drain_ranges = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRouterNatRulesElActionEl {
    type O = BlockAssignable<ComputeRouterNatRulesElActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRouterNatRulesElActionEl {}
impl BuildComputeRouterNatRulesElActionEl {
    pub fn build(self) -> ComputeRouterNatRulesElActionEl {
        ComputeRouterNatRulesElActionEl {
            source_nat_active_ips: core::default::Default::default(),
            source_nat_active_ranges: core::default::Default::default(),
            source_nat_drain_ips: core::default::Default::default(),
            source_nat_drain_ranges: core::default::Default::default(),
        }
    }
}
pub struct ComputeRouterNatRulesElActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouterNatRulesElActionElRef {
    fn new(shared: StackShared, base: String) -> ComputeRouterNatRulesElActionElRef {
        ComputeRouterNatRulesElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRouterNatRulesElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `source_nat_active_ips` after provisioning.\nA list of URLs of the IP resources used for this NAT rule.\nThese IP addresses must be valid static external IP addresses assigned to the project.\nThis field is used for public NAT."]
    pub fn source_nat_active_ips(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.source_nat_active_ips", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_nat_active_ranges` after provisioning.\nA list of URLs of the subnetworks used as source ranges for this NAT Rule.\nThese subnetworks must have purpose set to PRIVATE_NAT.\nThis field is used for private NAT."]
    pub fn source_nat_active_ranges(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.source_nat_active_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_nat_drain_ips` after provisioning.\nA list of URLs of the IP resources to be drained.\nThese IPs must be valid static external IPs that have been assigned to the NAT.\nThese IPs should be used for updating/patching a NAT rule only.\nThis field is used for public NAT."]
    pub fn source_nat_drain_ips(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.source_nat_drain_ips", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_nat_drain_ranges` after provisioning.\nA list of URLs of subnetworks representing source ranges to be drained.\nThis is only supported on patch/update, and these subnetworks must have previously been used as active ranges in this NAT Rule.\nThis field is used for private NAT."]
    pub fn source_nat_drain_ranges(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.source_nat_drain_ranges", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ComputeRouterNatRulesElDynamic {
    action: Option<DynamicBlock<ComputeRouterNatRulesElActionEl>>,
}
#[derive(Serialize)]
pub struct ComputeRouterNatRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(rename = "match")]
    match_: PrimField<String>,
    rule_number: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<ComputeRouterNatRulesElActionEl>>,
    dynamic: ComputeRouterNatRulesElDynamic,
}
impl ComputeRouterNatRulesEl {
    #[doc = "Set the field `description`.\nAn optional description of this rule."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl Into<BlockAssignable<ComputeRouterNatRulesElActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.action = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRouterNatRulesEl {
    type O = BlockAssignable<ComputeRouterNatRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRouterNatRulesEl {
    #[doc = "CEL expression that specifies the match condition that egress traffic from a VM is evaluated against.\nIf it evaluates to true, the corresponding action is enforced.\n\nThe following examples are valid match expressions for public NAT:\n\n\"inIpRange(destination.ip, '1.1.0.0/16') || inIpRange(destination.ip, '2.2.0.0/16')\"\n\n\"destination.ip == '1.1.0.1' || destination.ip == '8.8.8.8'\"\n\nThe following example is a valid match expression for private NAT:\n\n\"nexthop.hub == 'https://networkconnectivity.googleapis.com/v1alpha1/projects/my-project/global/hub/hub-1'\""]
    pub match_: PrimField<String>,
    #[doc = "An integer uniquely identifying a rule in the list.\nThe rule number must be a positive value between 0 and 65000, and must be unique among rules within a NAT."]
    pub rule_number: PrimField<f64>,
}
impl BuildComputeRouterNatRulesEl {
    pub fn build(self) -> ComputeRouterNatRulesEl {
        ComputeRouterNatRulesEl {
            description: core::default::Default::default(),
            match_: self.match_,
            rule_number: self.rule_number,
            action: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRouterNatRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouterNatRulesElRef {
    fn new(shared: StackShared, base: String) -> ComputeRouterNatRulesElRef {
        ComputeRouterNatRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRouterNatRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this rule."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `match_` after provisioning.\nCEL expression that specifies the match condition that egress traffic from a VM is evaluated against.\nIf it evaluates to true, the corresponding action is enforced.\n\nThe following examples are valid match expressions for public NAT:\n\n\"inIpRange(destination.ip, '1.1.0.0/16') || inIpRange(destination.ip, '2.2.0.0/16')\"\n\n\"destination.ip == '1.1.0.1' || destination.ip == '8.8.8.8'\"\n\nThe following example is a valid match expression for private NAT:\n\n\"nexthop.hub == 'https://networkconnectivity.googleapis.com/v1alpha1/projects/my-project/global/hub/hub-1'\""]
    pub fn match_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.match", self.base))
    }
    #[doc = "Get a reference to the value of field `rule_number` after provisioning.\nAn integer uniquely identifying a rule in the list.\nThe rule number must be a positive value between 0 and 65000, and must be unique among rules within a NAT."]
    pub fn rule_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_number", self.base))
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<ComputeRouterNatRulesElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRouterNatSubnetworkEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_ip_range_names: Option<SetField<PrimField<String>>>,
    source_ip_ranges_to_nat: SetField<PrimField<String>>,
}
impl ComputeRouterNatSubnetworkEl {
    #[doc = "Set the field `secondary_ip_range_names`.\nList of the secondary ranges of the subnetwork that are allowed\nto use NAT. This can be populated only if\n'LIST_OF_SECONDARY_IP_RANGES' is one of the values in\nsourceIpRangesToNat"]
    pub fn set_secondary_ip_range_names(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.secondary_ip_range_names = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRouterNatSubnetworkEl {
    type O = BlockAssignable<ComputeRouterNatSubnetworkEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRouterNatSubnetworkEl {
    #[doc = "Self-link of subnetwork to NAT"]
    pub name: PrimField<String>,
    #[doc = "List of options for which source IPs in the subnetwork\nshould have NAT enabled. Supported values include:\n'ALL_IP_RANGES', 'LIST_OF_SECONDARY_IP_RANGES',\n'PRIMARY_IP_RANGE'."]
    pub source_ip_ranges_to_nat: SetField<PrimField<String>>,
}
impl BuildComputeRouterNatSubnetworkEl {
    pub fn build(self) -> ComputeRouterNatSubnetworkEl {
        ComputeRouterNatSubnetworkEl {
            name: self.name,
            secondary_ip_range_names: core::default::Default::default(),
            source_ip_ranges_to_nat: self.source_ip_ranges_to_nat,
        }
    }
}
pub struct ComputeRouterNatSubnetworkElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouterNatSubnetworkElRef {
    fn new(shared: StackShared, base: String) -> ComputeRouterNatSubnetworkElRef {
        ComputeRouterNatSubnetworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRouterNatSubnetworkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nSelf-link of subnetwork to NAT"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `secondary_ip_range_names` after provisioning.\nList of the secondary ranges of the subnetwork that are allowed\nto use NAT. This can be populated only if\n'LIST_OF_SECONDARY_IP_RANGES' is one of the values in\nsourceIpRangesToNat"]
    pub fn secondary_ip_range_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.secondary_ip_range_names", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_ip_ranges_to_nat` after provisioning.\nList of options for which source IPs in the subnetwork\nshould have NAT enabled. Supported values include:\n'ALL_IP_RANGES', 'LIST_OF_SECONDARY_IP_RANGES',\n'PRIMARY_IP_RANGE'."]
    pub fn source_ip_ranges_to_nat(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.source_ip_ranges_to_nat", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRouterNatTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeRouterNatTimeoutsEl {
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
impl ToListMappable for ComputeRouterNatTimeoutsEl {
    type O = BlockAssignable<ComputeRouterNatTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRouterNatTimeoutsEl {}
impl BuildComputeRouterNatTimeoutsEl {
    pub fn build(self) -> ComputeRouterNatTimeoutsEl {
        ComputeRouterNatTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeRouterNatTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouterNatTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeRouterNatTimeoutsElRef {
        ComputeRouterNatTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRouterNatTimeoutsElRef {
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
struct ComputeRouterNatDynamic {
    log_config: Option<DynamicBlock<ComputeRouterNatLogConfigEl>>,
    nat64_subnetwork: Option<DynamicBlock<ComputeRouterNatNat64SubnetworkEl>>,
    rules: Option<DynamicBlock<ComputeRouterNatRulesEl>>,
    subnetwork: Option<DynamicBlock<ComputeRouterNatSubnetworkEl>>,
}
