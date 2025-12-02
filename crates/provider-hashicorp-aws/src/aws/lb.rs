use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct LbData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_keep_alive: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_owned_ipv4_pool: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desync_mitigation_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_record_client_routing_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drop_invalid_header_fields: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_cross_zone_load_balancing: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_deletion_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_http2: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_tls_version_and_cipher_suite_headers: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_waf_fail_open: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_xff_client_port: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_zonal_shift: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_security_group_inbound_rules_on_private_link_traffic: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancer_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preserve_host_header: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_ips_auto_assigned_per_subnet: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_groups: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnets: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    xff_header_processing_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_logs: Option<Vec<LbAccessLogsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_logs: Option<Vec<LbConnectionLogsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipam_pools: Option<Vec<LbIpamPoolsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_load_balancer_capacity: Option<Vec<LbMinimumLoadBalancerCapacityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_mapping: Option<Vec<LbSubnetMappingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<LbTimeoutsEl>,
    dynamic: LbDynamic,
}
struct Lb_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LbData>,
}
#[derive(Clone)]
pub struct Lb(Rc<Lb_>);
impl Lb {
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
    #[doc = "Set the field `client_keep_alive`.\n"]
    pub fn set_client_keep_alive(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().client_keep_alive = Some(v.into());
        self
    }
    #[doc = "Set the field `customer_owned_ipv4_pool`.\n"]
    pub fn set_customer_owned_ipv4_pool(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().customer_owned_ipv4_pool = Some(v.into());
        self
    }
    #[doc = "Set the field `desync_mitigation_mode`.\n"]
    pub fn set_desync_mitigation_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().desync_mitigation_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `dns_record_client_routing_policy`.\n"]
    pub fn set_dns_record_client_routing_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().dns_record_client_routing_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `drop_invalid_header_fields`.\n"]
    pub fn set_drop_invalid_header_fields(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().drop_invalid_header_fields = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_cross_zone_load_balancing`.\n"]
    pub fn set_enable_cross_zone_load_balancing(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_cross_zone_load_balancing = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_deletion_protection`.\n"]
    pub fn set_enable_deletion_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_deletion_protection = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_http2`.\n"]
    pub fn set_enable_http2(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_http2 = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_tls_version_and_cipher_suite_headers`.\n"]
    pub fn set_enable_tls_version_and_cipher_suite_headers(
        self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .enable_tls_version_and_cipher_suite_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_waf_fail_open`.\n"]
    pub fn set_enable_waf_fail_open(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_waf_fail_open = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_xff_client_port`.\n"]
    pub fn set_enable_xff_client_port(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_xff_client_port = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_zonal_shift`.\n"]
    pub fn set_enable_zonal_shift(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_zonal_shift = Some(v.into());
        self
    }
    #[doc = "Set the field `enforce_security_group_inbound_rules_on_private_link_traffic`.\n"]
    pub fn set_enforce_security_group_inbound_rules_on_private_link_traffic(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .enforce_security_group_inbound_rules_on_private_link_traffic = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `idle_timeout`.\n"]
    pub fn set_idle_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().idle_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `internal`.\n"]
    pub fn set_internal(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().internal = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_address_type`.\n"]
    pub fn set_ip_address_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ip_address_type = Some(v.into());
        self
    }
    #[doc = "Set the field `load_balancer_type`.\n"]
    pub fn set_load_balancer_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().load_balancer_type = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `preserve_host_header`.\n"]
    pub fn set_preserve_host_header(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().preserve_host_header = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `secondary_ips_auto_assigned_per_subnet`.\n"]
    pub fn set_secondary_ips_auto_assigned_per_subnet(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0
            .data
            .borrow_mut()
            .secondary_ips_auto_assigned_per_subnet = Some(v.into());
        self
    }
    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().security_groups = Some(v.into());
        self
    }
    #[doc = "Set the field `subnets`.\n"]
    pub fn set_subnets(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().subnets = Some(v.into());
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
    #[doc = "Set the field `xff_header_processing_mode`.\n"]
    pub fn set_xff_header_processing_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().xff_header_processing_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `access_logs`.\n"]
    pub fn set_access_logs(self, v: impl Into<BlockAssignable<LbAccessLogsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_logs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_logs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `connection_logs`.\n"]
    pub fn set_connection_logs(self, v: impl Into<BlockAssignable<LbConnectionLogsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().connection_logs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.connection_logs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ipam_pools`.\n"]
    pub fn set_ipam_pools(self, v: impl Into<BlockAssignable<LbIpamPoolsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ipam_pools = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ipam_pools = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `minimum_load_balancer_capacity`.\n"]
    pub fn set_minimum_load_balancer_capacity(
        self,
        v: impl Into<BlockAssignable<LbMinimumLoadBalancerCapacityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().minimum_load_balancer_capacity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .minimum_load_balancer_capacity = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `subnet_mapping`.\n"]
    pub fn set_subnet_mapping(self, v: impl Into<BlockAssignable<LbSubnetMappingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().subnet_mapping = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.subnet_mapping = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<LbTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `arn_suffix` after provisioning.\n"]
    pub fn arn_suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.arn_suffix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_keep_alive` after provisioning.\n"]
    pub fn client_keep_alive(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_keep_alive", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_owned_ipv4_pool` after provisioning.\n"]
    pub fn customer_owned_ipv4_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_owned_ipv4_pool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desync_mitigation_mode` after provisioning.\n"]
    pub fn desync_mitigation_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desync_mitigation_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_record_client_routing_policy` after provisioning.\n"]
    pub fn dns_record_client_routing_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_record_client_routing_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `drop_invalid_header_fields` after provisioning.\n"]
    pub fn drop_invalid_header_fields(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.drop_invalid_header_fields", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_cross_zone_load_balancing` after provisioning.\n"]
    pub fn enable_cross_zone_load_balancing(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_cross_zone_load_balancing", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_deletion_protection` after provisioning.\n"]
    pub fn enable_deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_http2` after provisioning.\n"]
    pub fn enable_http2(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_http2", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_tls_version_and_cipher_suite_headers` after provisioning.\n"]
    pub fn enable_tls_version_and_cipher_suite_headers(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.enable_tls_version_and_cipher_suite_headers",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `enable_waf_fail_open` after provisioning.\n"]
    pub fn enable_waf_fail_open(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_waf_fail_open", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_xff_client_port` after provisioning.\n"]
    pub fn enable_xff_client_port(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_xff_client_port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_zonal_shift` after provisioning.\n"]
    pub fn enable_zonal_shift(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_zonal_shift", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enforce_security_group_inbound_rules_on_private_link_traffic` after provisioning.\n"]
    pub fn enforce_security_group_inbound_rules_on_private_link_traffic(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.enforce_security_group_inbound_rules_on_private_link_traffic",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `idle_timeout` after provisioning.\n"]
    pub fn idle_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.idle_timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `internal` after provisioning.\n"]
    pub fn internal(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.internal", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancer_type` after provisioning.\n"]
    pub fn load_balancer_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancer_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `preserve_host_header` after provisioning.\n"]
    pub fn preserve_host_header(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preserve_host_header", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_ips_auto_assigned_per_subnet` after provisioning.\n"]
    pub fn secondary_ips_auto_assigned_per_subnet(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.secondary_ips_auto_assigned_per_subnet",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subnets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `xff_header_processing_mode` after provisioning.\n"]
    pub fn xff_header_processing_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.xff_header_processing_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `access_logs` after provisioning.\n"]
    pub fn access_logs(&self) -> ListRef<LbAccessLogsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_logs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_logs` after provisioning.\n"]
    pub fn connection_logs(&self) -> ListRef<LbConnectionLogsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connection_logs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipam_pools` after provisioning.\n"]
    pub fn ipam_pools(&self) -> ListRef<LbIpamPoolsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ipam_pools", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `minimum_load_balancer_capacity` after provisioning.\n"]
    pub fn minimum_load_balancer_capacity(&self) -> ListRef<LbMinimumLoadBalancerCapacityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.minimum_load_balancer_capacity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LbTimeoutsElRef {
        LbTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for Lb {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for Lb {}
impl ToListMappable for Lb {
    type O = ListRef<LbRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for Lb_ {
    fn extract_resource_type(&self) -> String {
        "aws_lb".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildLb {
    pub tf_id: String,
}
impl BuildLb {
    pub fn build(self, stack: &mut Stack) -> Lb {
        let out = Lb(Rc::new(Lb_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LbData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                client_keep_alive: core::default::Default::default(),
                customer_owned_ipv4_pool: core::default::Default::default(),
                desync_mitigation_mode: core::default::Default::default(),
                dns_record_client_routing_policy: core::default::Default::default(),
                drop_invalid_header_fields: core::default::Default::default(),
                enable_cross_zone_load_balancing: core::default::Default::default(),
                enable_deletion_protection: core::default::Default::default(),
                enable_http2: core::default::Default::default(),
                enable_tls_version_and_cipher_suite_headers: core::default::Default::default(),
                enable_waf_fail_open: core::default::Default::default(),
                enable_xff_client_port: core::default::Default::default(),
                enable_zonal_shift: core::default::Default::default(),
                enforce_security_group_inbound_rules_on_private_link_traffic:
                    core::default::Default::default(),
                id: core::default::Default::default(),
                idle_timeout: core::default::Default::default(),
                internal: core::default::Default::default(),
                ip_address_type: core::default::Default::default(),
                load_balancer_type: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                preserve_host_header: core::default::Default::default(),
                region: core::default::Default::default(),
                secondary_ips_auto_assigned_per_subnet: core::default::Default::default(),
                security_groups: core::default::Default::default(),
                subnets: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                xff_header_processing_mode: core::default::Default::default(),
                access_logs: core::default::Default::default(),
                connection_logs: core::default::Default::default(),
                ipam_pools: core::default::Default::default(),
                minimum_load_balancer_capacity: core::default::Default::default(),
                subnet_mapping: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct LbRef {
    shared: StackShared,
    base: String,
}
impl Ref for LbRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl LbRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `arn_suffix` after provisioning.\n"]
    pub fn arn_suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.arn_suffix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_keep_alive` after provisioning.\n"]
    pub fn client_keep_alive(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_keep_alive", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_owned_ipv4_pool` after provisioning.\n"]
    pub fn customer_owned_ipv4_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_owned_ipv4_pool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desync_mitigation_mode` after provisioning.\n"]
    pub fn desync_mitigation_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desync_mitigation_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_record_client_routing_policy` after provisioning.\n"]
    pub fn dns_record_client_routing_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_record_client_routing_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `drop_invalid_header_fields` after provisioning.\n"]
    pub fn drop_invalid_header_fields(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.drop_invalid_header_fields", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_cross_zone_load_balancing` after provisioning.\n"]
    pub fn enable_cross_zone_load_balancing(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_cross_zone_load_balancing", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_deletion_protection` after provisioning.\n"]
    pub fn enable_deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_http2` after provisioning.\n"]
    pub fn enable_http2(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_http2", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_tls_version_and_cipher_suite_headers` after provisioning.\n"]
    pub fn enable_tls_version_and_cipher_suite_headers(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.enable_tls_version_and_cipher_suite_headers",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `enable_waf_fail_open` after provisioning.\n"]
    pub fn enable_waf_fail_open(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_waf_fail_open", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_xff_client_port` after provisioning.\n"]
    pub fn enable_xff_client_port(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_xff_client_port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_zonal_shift` after provisioning.\n"]
    pub fn enable_zonal_shift(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_zonal_shift", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enforce_security_group_inbound_rules_on_private_link_traffic` after provisioning.\n"]
    pub fn enforce_security_group_inbound_rules_on_private_link_traffic(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.enforce_security_group_inbound_rules_on_private_link_traffic",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `idle_timeout` after provisioning.\n"]
    pub fn idle_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.idle_timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `internal` after provisioning.\n"]
    pub fn internal(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.internal", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancer_type` after provisioning.\n"]
    pub fn load_balancer_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancer_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `preserve_host_header` after provisioning.\n"]
    pub fn preserve_host_header(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preserve_host_header", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_ips_auto_assigned_per_subnet` after provisioning.\n"]
    pub fn secondary_ips_auto_assigned_per_subnet(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.secondary_ips_auto_assigned_per_subnet",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subnets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `xff_header_processing_mode` after provisioning.\n"]
    pub fn xff_header_processing_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.xff_header_processing_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `access_logs` after provisioning.\n"]
    pub fn access_logs(&self) -> ListRef<LbAccessLogsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_logs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_logs` after provisioning.\n"]
    pub fn connection_logs(&self) -> ListRef<LbConnectionLogsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connection_logs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipam_pools` after provisioning.\n"]
    pub fn ipam_pools(&self) -> ListRef<LbIpamPoolsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ipam_pools", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `minimum_load_balancer_capacity` after provisioning.\n"]
    pub fn minimum_load_balancer_capacity(&self) -> ListRef<LbMinimumLoadBalancerCapacityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.minimum_load_balancer_capacity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LbTimeoutsElRef {
        LbTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct LbAccessLogsEl {
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
}
impl LbAccessLogsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
}
impl ToListMappable for LbAccessLogsEl {
    type O = BlockAssignable<LbAccessLogsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLbAccessLogsEl {
    #[doc = ""]
    pub bucket: PrimField<String>,
}
impl BuildLbAccessLogsEl {
    pub fn build(self) -> LbAccessLogsEl {
        LbAccessLogsEl {
            bucket: self.bucket,
            enabled: core::default::Default::default(),
            prefix: core::default::Default::default(),
        }
    }
}
pub struct LbAccessLogsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LbAccessLogsElRef {
    fn new(shared: StackShared, base: String) -> LbAccessLogsElRef {
        LbAccessLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LbAccessLogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}
#[derive(Serialize)]
pub struct LbConnectionLogsEl {
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
}
impl LbConnectionLogsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
}
impl ToListMappable for LbConnectionLogsEl {
    type O = BlockAssignable<LbConnectionLogsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLbConnectionLogsEl {
    #[doc = ""]
    pub bucket: PrimField<String>,
}
impl BuildLbConnectionLogsEl {
    pub fn build(self) -> LbConnectionLogsEl {
        LbConnectionLogsEl {
            bucket: self.bucket,
            enabled: core::default::Default::default(),
            prefix: core::default::Default::default(),
        }
    }
}
pub struct LbConnectionLogsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LbConnectionLogsElRef {
    fn new(shared: StackShared, base: String) -> LbConnectionLogsElRef {
        LbConnectionLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LbConnectionLogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}
#[derive(Serialize)]
pub struct LbIpamPoolsEl {
    ipv4_ipam_pool_id: PrimField<String>,
}
impl LbIpamPoolsEl {}
impl ToListMappable for LbIpamPoolsEl {
    type O = BlockAssignable<LbIpamPoolsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLbIpamPoolsEl {
    #[doc = ""]
    pub ipv4_ipam_pool_id: PrimField<String>,
}
impl BuildLbIpamPoolsEl {
    pub fn build(self) -> LbIpamPoolsEl {
        LbIpamPoolsEl {
            ipv4_ipam_pool_id: self.ipv4_ipam_pool_id,
        }
    }
}
pub struct LbIpamPoolsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LbIpamPoolsElRef {
    fn new(shared: StackShared, base: String) -> LbIpamPoolsElRef {
        LbIpamPoolsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LbIpamPoolsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ipv4_ipam_pool_id` after provisioning.\n"]
    pub fn ipv4_ipam_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv4_ipam_pool_id", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct LbMinimumLoadBalancerCapacityEl {
    capacity_units: PrimField<f64>,
}
impl LbMinimumLoadBalancerCapacityEl {}
impl ToListMappable for LbMinimumLoadBalancerCapacityEl {
    type O = BlockAssignable<LbMinimumLoadBalancerCapacityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLbMinimumLoadBalancerCapacityEl {
    #[doc = ""]
    pub capacity_units: PrimField<f64>,
}
impl BuildLbMinimumLoadBalancerCapacityEl {
    pub fn build(self) -> LbMinimumLoadBalancerCapacityEl {
        LbMinimumLoadBalancerCapacityEl {
            capacity_units: self.capacity_units,
        }
    }
}
pub struct LbMinimumLoadBalancerCapacityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LbMinimumLoadBalancerCapacityElRef {
    fn new(shared: StackShared, base: String) -> LbMinimumLoadBalancerCapacityElRef {
        LbMinimumLoadBalancerCapacityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LbMinimumLoadBalancerCapacityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `capacity_units` after provisioning.\n"]
    pub fn capacity_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_units", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct LbSubnetMappingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allocation_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_ipv4_address: Option<PrimField<String>>,
    subnet_id: PrimField<String>,
}
impl LbSubnetMappingEl {
    #[doc = "Set the field `allocation_id`.\n"]
    pub fn set_allocation_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.allocation_id = Some(v.into());
        self
    }
    #[doc = "Set the field `ipv6_address`.\n"]
    pub fn set_ipv6_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv6_address = Some(v.into());
        self
    }
    #[doc = "Set the field `private_ipv4_address`.\n"]
    pub fn set_private_ipv4_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_ipv4_address = Some(v.into());
        self
    }
}
impl ToListMappable for LbSubnetMappingEl {
    type O = BlockAssignable<LbSubnetMappingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLbSubnetMappingEl {
    #[doc = ""]
    pub subnet_id: PrimField<String>,
}
impl BuildLbSubnetMappingEl {
    pub fn build(self) -> LbSubnetMappingEl {
        LbSubnetMappingEl {
            allocation_id: core::default::Default::default(),
            ipv6_address: core::default::Default::default(),
            private_ipv4_address: core::default::Default::default(),
            subnet_id: self.subnet_id,
        }
    }
}
pub struct LbSubnetMappingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LbSubnetMappingElRef {
    fn new(shared: StackShared, base: String) -> LbSubnetMappingElRef {
        LbSubnetMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LbSubnetMappingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allocation_id` after provisioning.\n"]
    pub fn allocation_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocation_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_address` after provisioning.\n"]
    pub fn ipv6_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address", self.base))
    }
    #[doc = "Get a reference to the value of field `outpost_id` after provisioning.\n"]
    pub fn outpost_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.outpost_id", self.base))
    }
    #[doc = "Get a reference to the value of field `private_ipv4_address` after provisioning.\n"]
    pub fn private_ipv4_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_ipv4_address", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
}
#[derive(Serialize)]
pub struct LbTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl LbTimeoutsEl {
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
impl ToListMappable for LbTimeoutsEl {
    type O = BlockAssignable<LbTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLbTimeoutsEl {}
impl BuildLbTimeoutsEl {
    pub fn build(self) -> LbTimeoutsEl {
        LbTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct LbTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LbTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> LbTimeoutsElRef {
        LbTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LbTimeoutsElRef {
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
struct LbDynamic {
    access_logs: Option<DynamicBlock<LbAccessLogsEl>>,
    connection_logs: Option<DynamicBlock<LbConnectionLogsEl>>,
    ipam_pools: Option<DynamicBlock<LbIpamPoolsEl>>,
    minimum_load_balancer_capacity: Option<DynamicBlock<LbMinimumLoadBalancerCapacityEl>>,
    subnet_mapping: Option<DynamicBlock<LbSubnetMappingEl>>,
}
