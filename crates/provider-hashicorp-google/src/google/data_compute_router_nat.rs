use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeRouterNatData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    router: PrimField<String>,
}
struct DataComputeRouterNat_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeRouterNatData>,
}
#[derive(Clone)]
pub struct DataComputeRouterNat(Rc<DataComputeRouterNat_>);
impl DataComputeRouterNat {
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
    #[doc = "Set the field `region`.\nRegion where the router and NAT reside."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
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
    #[doc = "Get a reference to the value of field `log_config` after provisioning.\nConfiguration for logging on NAT"]
    pub fn log_config(&self) -> ListRef<DataComputeRouterNatLogConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_config", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `nat64_subnetwork` after provisioning.\nOne or more subnetwork NAT configurations whose traffic should be translated by NAT64 Gateway.\nOnly used if 'source_subnetwork_ip_ranges_to_nat64' is set to 'LIST_OF_IPV6_SUBNETWORKS'"]
    pub fn nat64_subnetwork(&self) -> SetRef<DataComputeRouterNatNat64SubnetworkElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.nat64_subnetwork", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `rules` after provisioning.\nA list of rules associated with this NAT."]
    pub fn rules(&self) -> SetRef<DataComputeRouterNatRulesElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nOne or more subnetwork NAT configurations. Only used if\n'source_subnetwork_ip_ranges_to_nat' is set to 'LIST_OF_SUBNETWORKS'"]
    pub fn subnetwork(&self) -> SetRef<DataComputeRouterNatSubnetworkElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subnetwork", self.extract_ref()),
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
}
impl Referable for DataComputeRouterNat {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeRouterNat {}
impl ToListMappable for DataComputeRouterNat {
    type O = ListRef<DataComputeRouterNatRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeRouterNat_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_router_nat".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeRouterNat {
    pub tf_id: String,
    #[doc = "Name of the NAT service. The name must be 1-63 characters long and\ncomply with RFC1035."]
    pub name: PrimField<String>,
    #[doc = "The name of the Cloud Router in which this NAT will be configured."]
    pub router: PrimField<String>,
}
impl BuildDataComputeRouterNat {
    pub fn build(self, stack: &mut Stack) -> DataComputeRouterNat {
        let out = DataComputeRouterNat(Rc::new(DataComputeRouterNat_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeRouterNatData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                router: self.router,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeRouterNatRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRouterNatRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeRouterNatRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
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
    #[doc = "Get a reference to the value of field `log_config` after provisioning.\nConfiguration for logging on NAT"]
    pub fn log_config(&self) -> ListRef<DataComputeRouterNatLogConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_config", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `nat64_subnetwork` after provisioning.\nOne or more subnetwork NAT configurations whose traffic should be translated by NAT64 Gateway.\nOnly used if 'source_subnetwork_ip_ranges_to_nat64' is set to 'LIST_OF_IPV6_SUBNETWORKS'"]
    pub fn nat64_subnetwork(&self) -> SetRef<DataComputeRouterNatNat64SubnetworkElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.nat64_subnetwork", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `rules` after provisioning.\nA list of rules associated with this NAT."]
    pub fn rules(&self) -> SetRef<DataComputeRouterNatRulesElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nOne or more subnetwork NAT configurations. Only used if\n'source_subnetwork_ip_ranges_to_nat' is set to 'LIST_OF_SUBNETWORKS'"]
    pub fn subnetwork(&self) -> SetRef<DataComputeRouterNatSubnetworkElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subnetwork", self.extract_ref()),
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
}
#[derive(Serialize)]
pub struct DataComputeRouterNatLogConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
}
impl DataComputeRouterNatLogConfigEl {
    #[doc = "Set the field `enable`.\n"]
    pub fn set_enable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable = Some(v.into());
        self
    }
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRouterNatLogConfigEl {
    type O = BlockAssignable<DataComputeRouterNatLogConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRouterNatLogConfigEl {}
impl BuildDataComputeRouterNatLogConfigEl {
    pub fn build(self) -> DataComputeRouterNatLogConfigEl {
        DataComputeRouterNatLogConfigEl {
            enable: core::default::Default::default(),
            filter: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRouterNatLogConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRouterNatLogConfigElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRouterNatLogConfigElRef {
        DataComputeRouterNatLogConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRouterNatLogConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRouterNatNat64SubnetworkEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataComputeRouterNatNat64SubnetworkEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRouterNatNat64SubnetworkEl {
    type O = BlockAssignable<DataComputeRouterNatNat64SubnetworkEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRouterNatNat64SubnetworkEl {}
impl BuildDataComputeRouterNatNat64SubnetworkEl {
    pub fn build(self) -> DataComputeRouterNatNat64SubnetworkEl {
        DataComputeRouterNatNat64SubnetworkEl {
            name: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRouterNatNat64SubnetworkElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRouterNatNat64SubnetworkElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRouterNatNat64SubnetworkElRef {
        DataComputeRouterNatNat64SubnetworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRouterNatNat64SubnetworkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRouterNatRulesElActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    source_nat_active_ips: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_nat_active_ranges: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_nat_drain_ips: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_nat_drain_ranges: Option<SetField<PrimField<String>>>,
}
impl DataComputeRouterNatRulesElActionEl {
    #[doc = "Set the field `source_nat_active_ips`.\n"]
    pub fn set_source_nat_active_ips(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.source_nat_active_ips = Some(v.into());
        self
    }
    #[doc = "Set the field `source_nat_active_ranges`.\n"]
    pub fn set_source_nat_active_ranges(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.source_nat_active_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `source_nat_drain_ips`.\n"]
    pub fn set_source_nat_drain_ips(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.source_nat_drain_ips = Some(v.into());
        self
    }
    #[doc = "Set the field `source_nat_drain_ranges`.\n"]
    pub fn set_source_nat_drain_ranges(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.source_nat_drain_ranges = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRouterNatRulesElActionEl {
    type O = BlockAssignable<DataComputeRouterNatRulesElActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRouterNatRulesElActionEl {}
impl BuildDataComputeRouterNatRulesElActionEl {
    pub fn build(self) -> DataComputeRouterNatRulesElActionEl {
        DataComputeRouterNatRulesElActionEl {
            source_nat_active_ips: core::default::Default::default(),
            source_nat_active_ranges: core::default::Default::default(),
            source_nat_drain_ips: core::default::Default::default(),
            source_nat_drain_ranges: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRouterNatRulesElActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRouterNatRulesElActionElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRouterNatRulesElActionElRef {
        DataComputeRouterNatRulesElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRouterNatRulesElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `source_nat_active_ips` after provisioning.\n"]
    pub fn source_nat_active_ips(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.source_nat_active_ips", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_nat_active_ranges` after provisioning.\n"]
    pub fn source_nat_active_ranges(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.source_nat_active_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_nat_drain_ips` after provisioning.\n"]
    pub fn source_nat_drain_ips(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.source_nat_drain_ips", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_nat_drain_ranges` after provisioning.\n"]
    pub fn source_nat_drain_ranges(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.source_nat_drain_ranges", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRouterNatRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<ListField<DataComputeRouterNatRulesElActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_number: Option<PrimField<f64>>,
}
impl DataComputeRouterNatRulesEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl Into<ListField<DataComputeRouterNatRulesElActionEl>>,
    ) -> Self {
        self.action = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.match_ = Some(v.into());
        self
    }
    #[doc = "Set the field `rule_number`.\n"]
    pub fn set_rule_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.rule_number = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRouterNatRulesEl {
    type O = BlockAssignable<DataComputeRouterNatRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRouterNatRulesEl {}
impl BuildDataComputeRouterNatRulesEl {
    pub fn build(self) -> DataComputeRouterNatRulesEl {
        DataComputeRouterNatRulesEl {
            action: core::default::Default::default(),
            description: core::default::Default::default(),
            match_: core::default::Default::default(),
            rule_number: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRouterNatRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRouterNatRulesElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRouterNatRulesElRef {
        DataComputeRouterNatRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRouterNatRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<DataComputeRouterNatRulesElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.match", self.base))
    }
    #[doc = "Get a reference to the value of field `rule_number` after provisioning.\n"]
    pub fn rule_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_number", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRouterNatSubnetworkEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_ip_range_names: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_ip_ranges_to_nat: Option<SetField<PrimField<String>>>,
}
impl DataComputeRouterNatSubnetworkEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `secondary_ip_range_names`.\n"]
    pub fn set_secondary_ip_range_names(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.secondary_ip_range_names = Some(v.into());
        self
    }
    #[doc = "Set the field `source_ip_ranges_to_nat`.\n"]
    pub fn set_source_ip_ranges_to_nat(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.source_ip_ranges_to_nat = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRouterNatSubnetworkEl {
    type O = BlockAssignable<DataComputeRouterNatSubnetworkEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRouterNatSubnetworkEl {}
impl BuildDataComputeRouterNatSubnetworkEl {
    pub fn build(self) -> DataComputeRouterNatSubnetworkEl {
        DataComputeRouterNatSubnetworkEl {
            name: core::default::Default::default(),
            secondary_ip_range_names: core::default::Default::default(),
            source_ip_ranges_to_nat: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRouterNatSubnetworkElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRouterNatSubnetworkElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRouterNatSubnetworkElRef {
        DataComputeRouterNatSubnetworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRouterNatSubnetworkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `secondary_ip_range_names` after provisioning.\n"]
    pub fn secondary_ip_range_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.secondary_ip_range_names", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_ip_ranges_to_nat` after provisioning.\n"]
    pub fn source_ip_ranges_to_nat(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.source_ip_ranges_to_nat", self.base),
        )
    }
}
