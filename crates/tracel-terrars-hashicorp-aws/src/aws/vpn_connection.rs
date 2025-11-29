use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct VpnConnectionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    customer_gateway_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_acceleration: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ipv4_network_cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ipv6_network_cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outside_ip_address_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preshared_key_storage: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remote_ipv4_network_cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remote_ipv6_network_cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    static_routes_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transport_transit_gateway_attachment_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel1_dpd_timeout_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel1_dpd_timeout_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel1_enable_tunnel_lifecycle_control: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel1_ike_versions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel1_inside_cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel1_inside_ipv6_cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel1_phase1_dh_group_numbers: Option<SetField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel1_phase1_encryption_algorithms: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel1_phase1_integrity_algorithms: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel1_phase1_lifetime_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel1_phase2_dh_group_numbers: Option<SetField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel1_phase2_encryption_algorithms: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel1_phase2_integrity_algorithms: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel1_phase2_lifetime_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel1_preshared_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel1_rekey_fuzz_percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel1_rekey_margin_time_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel1_replay_window_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel1_startup_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel2_dpd_timeout_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel2_dpd_timeout_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel2_enable_tunnel_lifecycle_control: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel2_ike_versions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel2_inside_cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel2_inside_ipv6_cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel2_phase1_dh_group_numbers: Option<SetField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel2_phase1_encryption_algorithms: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel2_phase1_integrity_algorithms: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel2_phase1_lifetime_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel2_phase2_dh_group_numbers: Option<SetField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel2_phase2_encryption_algorithms: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel2_phase2_integrity_algorithms: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel2_phase2_lifetime_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel2_preshared_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel2_rekey_fuzz_percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel2_rekey_margin_time_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel2_replay_window_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel2_startup_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel_bandwidth: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel_inside_ip_version: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpn_gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel1_log_options: Option<Vec<VpnConnectionTunnel1LogOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnel2_log_options: Option<Vec<VpnConnectionTunnel2LogOptionsEl>>,
    dynamic: VpnConnectionDynamic,
}

struct VpnConnection_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VpnConnectionData>,
}

#[derive(Clone)]
pub struct VpnConnection(Rc<VpnConnection_>);

impl VpnConnection {
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

    #[doc = "Set the field `enable_acceleration`.\n"]
    pub fn set_enable_acceleration(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_acceleration = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `local_ipv4_network_cidr`.\n"]
    pub fn set_local_ipv4_network_cidr(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().local_ipv4_network_cidr = Some(v.into());
        self
    }

    #[doc = "Set the field `local_ipv6_network_cidr`.\n"]
    pub fn set_local_ipv6_network_cidr(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().local_ipv6_network_cidr = Some(v.into());
        self
    }

    #[doc = "Set the field `outside_ip_address_type`.\n"]
    pub fn set_outside_ip_address_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().outside_ip_address_type = Some(v.into());
        self
    }

    #[doc = "Set the field `preshared_key_storage`.\n"]
    pub fn set_preshared_key_storage(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().preshared_key_storage = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `remote_ipv4_network_cidr`.\n"]
    pub fn set_remote_ipv4_network_cidr(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().remote_ipv4_network_cidr = Some(v.into());
        self
    }

    #[doc = "Set the field `remote_ipv6_network_cidr`.\n"]
    pub fn set_remote_ipv6_network_cidr(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().remote_ipv6_network_cidr = Some(v.into());
        self
    }

    #[doc = "Set the field `static_routes_only`.\n"]
    pub fn set_static_routes_only(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().static_routes_only = Some(v.into());
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

    #[doc = "Set the field `transit_gateway_id`.\n"]
    pub fn set_transit_gateway_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().transit_gateway_id = Some(v.into());
        self
    }

    #[doc = "Set the field `transport_transit_gateway_attachment_id`.\n"]
    pub fn set_transport_transit_gateway_attachment_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().transport_transit_gateway_attachment_id = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_dpd_timeout_action`.\n"]
    pub fn set_tunnel1_dpd_timeout_action(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tunnel1_dpd_timeout_action = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_dpd_timeout_seconds`.\n"]
    pub fn set_tunnel1_dpd_timeout_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tunnel1_dpd_timeout_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_enable_tunnel_lifecycle_control`.\n"]
    pub fn set_tunnel1_enable_tunnel_lifecycle_control(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().tunnel1_enable_tunnel_lifecycle_control = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_ike_versions`.\n"]
    pub fn set_tunnel1_ike_versions(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tunnel1_ike_versions = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_inside_cidr`.\n"]
    pub fn set_tunnel1_inside_cidr(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tunnel1_inside_cidr = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_inside_ipv6_cidr`.\n"]
    pub fn set_tunnel1_inside_ipv6_cidr(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tunnel1_inside_ipv6_cidr = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_phase1_dh_group_numbers`.\n"]
    pub fn set_tunnel1_phase1_dh_group_numbers(self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tunnel1_phase1_dh_group_numbers = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_phase1_encryption_algorithms`.\n"]
    pub fn set_tunnel1_phase1_encryption_algorithms(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tunnel1_phase1_encryption_algorithms = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_phase1_integrity_algorithms`.\n"]
    pub fn set_tunnel1_phase1_integrity_algorithms(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tunnel1_phase1_integrity_algorithms = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_phase1_lifetime_seconds`.\n"]
    pub fn set_tunnel1_phase1_lifetime_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tunnel1_phase1_lifetime_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_phase2_dh_group_numbers`.\n"]
    pub fn set_tunnel1_phase2_dh_group_numbers(self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tunnel1_phase2_dh_group_numbers = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_phase2_encryption_algorithms`.\n"]
    pub fn set_tunnel1_phase2_encryption_algorithms(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tunnel1_phase2_encryption_algorithms = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_phase2_integrity_algorithms`.\n"]
    pub fn set_tunnel1_phase2_integrity_algorithms(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tunnel1_phase2_integrity_algorithms = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_phase2_lifetime_seconds`.\n"]
    pub fn set_tunnel1_phase2_lifetime_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tunnel1_phase2_lifetime_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_preshared_key`.\n"]
    pub fn set_tunnel1_preshared_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tunnel1_preshared_key = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_rekey_fuzz_percentage`.\n"]
    pub fn set_tunnel1_rekey_fuzz_percentage(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tunnel1_rekey_fuzz_percentage = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_rekey_margin_time_seconds`.\n"]
    pub fn set_tunnel1_rekey_margin_time_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tunnel1_rekey_margin_time_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_replay_window_size`.\n"]
    pub fn set_tunnel1_replay_window_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tunnel1_replay_window_size = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_startup_action`.\n"]
    pub fn set_tunnel1_startup_action(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tunnel1_startup_action = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_dpd_timeout_action`.\n"]
    pub fn set_tunnel2_dpd_timeout_action(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tunnel2_dpd_timeout_action = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_dpd_timeout_seconds`.\n"]
    pub fn set_tunnel2_dpd_timeout_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tunnel2_dpd_timeout_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_enable_tunnel_lifecycle_control`.\n"]
    pub fn set_tunnel2_enable_tunnel_lifecycle_control(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().tunnel2_enable_tunnel_lifecycle_control = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_ike_versions`.\n"]
    pub fn set_tunnel2_ike_versions(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tunnel2_ike_versions = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_inside_cidr`.\n"]
    pub fn set_tunnel2_inside_cidr(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tunnel2_inside_cidr = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_inside_ipv6_cidr`.\n"]
    pub fn set_tunnel2_inside_ipv6_cidr(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tunnel2_inside_ipv6_cidr = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_phase1_dh_group_numbers`.\n"]
    pub fn set_tunnel2_phase1_dh_group_numbers(self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tunnel2_phase1_dh_group_numbers = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_phase1_encryption_algorithms`.\n"]
    pub fn set_tunnel2_phase1_encryption_algorithms(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tunnel2_phase1_encryption_algorithms = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_phase1_integrity_algorithms`.\n"]
    pub fn set_tunnel2_phase1_integrity_algorithms(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tunnel2_phase1_integrity_algorithms = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_phase1_lifetime_seconds`.\n"]
    pub fn set_tunnel2_phase1_lifetime_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tunnel2_phase1_lifetime_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_phase2_dh_group_numbers`.\n"]
    pub fn set_tunnel2_phase2_dh_group_numbers(self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tunnel2_phase2_dh_group_numbers = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_phase2_encryption_algorithms`.\n"]
    pub fn set_tunnel2_phase2_encryption_algorithms(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tunnel2_phase2_encryption_algorithms = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_phase2_integrity_algorithms`.\n"]
    pub fn set_tunnel2_phase2_integrity_algorithms(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tunnel2_phase2_integrity_algorithms = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_phase2_lifetime_seconds`.\n"]
    pub fn set_tunnel2_phase2_lifetime_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tunnel2_phase2_lifetime_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_preshared_key`.\n"]
    pub fn set_tunnel2_preshared_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tunnel2_preshared_key = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_rekey_fuzz_percentage`.\n"]
    pub fn set_tunnel2_rekey_fuzz_percentage(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tunnel2_rekey_fuzz_percentage = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_rekey_margin_time_seconds`.\n"]
    pub fn set_tunnel2_rekey_margin_time_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tunnel2_rekey_margin_time_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_replay_window_size`.\n"]
    pub fn set_tunnel2_replay_window_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tunnel2_replay_window_size = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_startup_action`.\n"]
    pub fn set_tunnel2_startup_action(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tunnel2_startup_action = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel_bandwidth`.\n"]
    pub fn set_tunnel_bandwidth(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tunnel_bandwidth = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel_inside_ip_version`.\n"]
    pub fn set_tunnel_inside_ip_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tunnel_inside_ip_version = Some(v.into());
        self
    }

    #[doc = "Set the field `vpn_gateway_id`.\n"]
    pub fn set_vpn_gateway_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().vpn_gateway_id = Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_log_options`.\n"]
    pub fn set_tunnel1_log_options(self, v: impl Into<BlockAssignable<VpnConnectionTunnel1LogOptionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tunnel1_log_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tunnel1_log_options = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tunnel2_log_options`.\n"]
    pub fn set_tunnel2_log_options(self, v: impl Into<BlockAssignable<VpnConnectionTunnel2LogOptionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tunnel2_log_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tunnel2_log_options = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_arn` after provisioning.\n"]
    pub fn core_network_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.core_network_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_attachment_arn` after provisioning.\n"]
    pub fn core_network_attachment_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.core_network_attachment_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_gateway_configuration` after provisioning.\n"]
    pub fn customer_gateway_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_gateway_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_gateway_id` after provisioning.\n"]
    pub fn customer_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_acceleration` after provisioning.\n"]
    pub fn enable_acceleration(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_acceleration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_ipv4_network_cidr` after provisioning.\n"]
    pub fn local_ipv4_network_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_ipv4_network_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_ipv6_network_cidr` after provisioning.\n"]
    pub fn local_ipv6_network_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_ipv6_network_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outside_ip_address_type` after provisioning.\n"]
    pub fn outside_ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.outside_ip_address_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preshared_key_arn` after provisioning.\n"]
    pub fn preshared_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preshared_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preshared_key_storage` after provisioning.\n"]
    pub fn preshared_key_storage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preshared_key_storage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_ipv4_network_cidr` after provisioning.\n"]
    pub fn remote_ipv4_network_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.remote_ipv4_network_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_ipv6_network_cidr` after provisioning.\n"]
    pub fn remote_ipv6_network_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.remote_ipv6_network_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> SetRef<VpnConnectionRoutesElRef> {
        SetRef::new(self.shared().clone(), format!("{}.routes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `static_routes_only` after provisioning.\n"]
    pub fn static_routes_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.static_routes_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_attachment_id` after provisioning.\n"]
    pub fn transit_gateway_attachment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_attachment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transport_transit_gateway_attachment_id` after provisioning.\n"]
    pub fn transport_transit_gateway_attachment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transport_transit_gateway_attachment_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_address` after provisioning.\n"]
    pub fn tunnel1_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_bgp_asn` after provisioning.\n"]
    pub fn tunnel1_bgp_asn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_bgp_asn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_bgp_holdtime` after provisioning.\n"]
    pub fn tunnel1_bgp_holdtime(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_bgp_holdtime", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_cgw_inside_address` after provisioning.\n"]
    pub fn tunnel1_cgw_inside_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_cgw_inside_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_dpd_timeout_action` after provisioning.\n"]
    pub fn tunnel1_dpd_timeout_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_dpd_timeout_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_dpd_timeout_seconds` after provisioning.\n"]
    pub fn tunnel1_dpd_timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_dpd_timeout_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_enable_tunnel_lifecycle_control` after provisioning.\n"]
    pub fn tunnel1_enable_tunnel_lifecycle_control(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tunnel1_enable_tunnel_lifecycle_control", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_ike_versions` after provisioning.\n"]
    pub fn tunnel1_ike_versions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel1_ike_versions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_inside_cidr` after provisioning.\n"]
    pub fn tunnel1_inside_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_inside_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_inside_ipv6_cidr` after provisioning.\n"]
    pub fn tunnel1_inside_ipv6_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_inside_ipv6_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase1_dh_group_numbers` after provisioning.\n"]
    pub fn tunnel1_phase1_dh_group_numbers(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel1_phase1_dh_group_numbers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase1_encryption_algorithms` after provisioning.\n"]
    pub fn tunnel1_phase1_encryption_algorithms(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel1_phase1_encryption_algorithms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase1_integrity_algorithms` after provisioning.\n"]
    pub fn tunnel1_phase1_integrity_algorithms(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel1_phase1_integrity_algorithms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase1_lifetime_seconds` after provisioning.\n"]
    pub fn tunnel1_phase1_lifetime_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_phase1_lifetime_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase2_dh_group_numbers` after provisioning.\n"]
    pub fn tunnel1_phase2_dh_group_numbers(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel1_phase2_dh_group_numbers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase2_encryption_algorithms` after provisioning.\n"]
    pub fn tunnel1_phase2_encryption_algorithms(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel1_phase2_encryption_algorithms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase2_integrity_algorithms` after provisioning.\n"]
    pub fn tunnel1_phase2_integrity_algorithms(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel1_phase2_integrity_algorithms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase2_lifetime_seconds` after provisioning.\n"]
    pub fn tunnel1_phase2_lifetime_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_phase2_lifetime_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_preshared_key` after provisioning.\n"]
    pub fn tunnel1_preshared_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_preshared_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_rekey_fuzz_percentage` after provisioning.\n"]
    pub fn tunnel1_rekey_fuzz_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_rekey_fuzz_percentage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_rekey_margin_time_seconds` after provisioning.\n"]
    pub fn tunnel1_rekey_margin_time_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_rekey_margin_time_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_replay_window_size` after provisioning.\n"]
    pub fn tunnel1_replay_window_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_replay_window_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_startup_action` after provisioning.\n"]
    pub fn tunnel1_startup_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_startup_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_vgw_inside_address` after provisioning.\n"]
    pub fn tunnel1_vgw_inside_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_vgw_inside_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_address` after provisioning.\n"]
    pub fn tunnel2_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_bgp_asn` after provisioning.\n"]
    pub fn tunnel2_bgp_asn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_bgp_asn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_bgp_holdtime` after provisioning.\n"]
    pub fn tunnel2_bgp_holdtime(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_bgp_holdtime", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_cgw_inside_address` after provisioning.\n"]
    pub fn tunnel2_cgw_inside_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_cgw_inside_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_dpd_timeout_action` after provisioning.\n"]
    pub fn tunnel2_dpd_timeout_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_dpd_timeout_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_dpd_timeout_seconds` after provisioning.\n"]
    pub fn tunnel2_dpd_timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_dpd_timeout_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_enable_tunnel_lifecycle_control` after provisioning.\n"]
    pub fn tunnel2_enable_tunnel_lifecycle_control(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tunnel2_enable_tunnel_lifecycle_control", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel2_ike_versions` after provisioning.\n"]
    pub fn tunnel2_ike_versions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel2_ike_versions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_inside_cidr` after provisioning.\n"]
    pub fn tunnel2_inside_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_inside_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_inside_ipv6_cidr` after provisioning.\n"]
    pub fn tunnel2_inside_ipv6_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_inside_ipv6_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase1_dh_group_numbers` after provisioning.\n"]
    pub fn tunnel2_phase1_dh_group_numbers(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel2_phase1_dh_group_numbers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase1_encryption_algorithms` after provisioning.\n"]
    pub fn tunnel2_phase1_encryption_algorithms(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel2_phase1_encryption_algorithms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase1_integrity_algorithms` after provisioning.\n"]
    pub fn tunnel2_phase1_integrity_algorithms(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel2_phase1_integrity_algorithms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase1_lifetime_seconds` after provisioning.\n"]
    pub fn tunnel2_phase1_lifetime_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_phase1_lifetime_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase2_dh_group_numbers` after provisioning.\n"]
    pub fn tunnel2_phase2_dh_group_numbers(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel2_phase2_dh_group_numbers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase2_encryption_algorithms` after provisioning.\n"]
    pub fn tunnel2_phase2_encryption_algorithms(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel2_phase2_encryption_algorithms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase2_integrity_algorithms` after provisioning.\n"]
    pub fn tunnel2_phase2_integrity_algorithms(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel2_phase2_integrity_algorithms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase2_lifetime_seconds` after provisioning.\n"]
    pub fn tunnel2_phase2_lifetime_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_phase2_lifetime_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_preshared_key` after provisioning.\n"]
    pub fn tunnel2_preshared_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_preshared_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_rekey_fuzz_percentage` after provisioning.\n"]
    pub fn tunnel2_rekey_fuzz_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_rekey_fuzz_percentage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_rekey_margin_time_seconds` after provisioning.\n"]
    pub fn tunnel2_rekey_margin_time_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_rekey_margin_time_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_replay_window_size` after provisioning.\n"]
    pub fn tunnel2_replay_window_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_replay_window_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_startup_action` after provisioning.\n"]
    pub fn tunnel2_startup_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_startup_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_vgw_inside_address` after provisioning.\n"]
    pub fn tunnel2_vgw_inside_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_vgw_inside_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel_bandwidth` after provisioning.\n"]
    pub fn tunnel_bandwidth(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel_bandwidth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel_inside_ip_version` after provisioning.\n"]
    pub fn tunnel_inside_ip_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel_inside_ip_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vgw_telemetry` after provisioning.\n"]
    pub fn vgw_telemetry(&self) -> SetRef<VpnConnectionVgwTelemetryElRef> {
        SetRef::new(self.shared().clone(), format!("{}.vgw_telemetry", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpn_gateway_id` after provisioning.\n"]
    pub fn vpn_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpn_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_log_options` after provisioning.\n"]
    pub fn tunnel1_log_options(&self) -> ListRef<VpnConnectionTunnel1LogOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tunnel1_log_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_log_options` after provisioning.\n"]
    pub fn tunnel2_log_options(&self) -> ListRef<VpnConnectionTunnel2LogOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tunnel2_log_options", self.extract_ref()))
    }
}

impl Referable for VpnConnection {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for VpnConnection { }

impl ToListMappable for VpnConnection {
    type O = ListRef<VpnConnectionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for VpnConnection_ {
    fn extract_resource_type(&self) -> String {
        "aws_vpn_connection".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVpnConnection {
    pub tf_id: String,
    #[doc = ""]
    pub customer_gateway_id: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildVpnConnection {
    pub fn build(self, stack: &mut Stack) -> VpnConnection {
        let out = VpnConnection(Rc::new(VpnConnection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VpnConnectionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                customer_gateway_id: self.customer_gateway_id,
                enable_acceleration: core::default::Default::default(),
                id: core::default::Default::default(),
                local_ipv4_network_cidr: core::default::Default::default(),
                local_ipv6_network_cidr: core::default::Default::default(),
                outside_ip_address_type: core::default::Default::default(),
                preshared_key_storage: core::default::Default::default(),
                region: core::default::Default::default(),
                remote_ipv4_network_cidr: core::default::Default::default(),
                remote_ipv6_network_cidr: core::default::Default::default(),
                static_routes_only: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                transit_gateway_id: core::default::Default::default(),
                transport_transit_gateway_attachment_id: core::default::Default::default(),
                tunnel1_dpd_timeout_action: core::default::Default::default(),
                tunnel1_dpd_timeout_seconds: core::default::Default::default(),
                tunnel1_enable_tunnel_lifecycle_control: core::default::Default::default(),
                tunnel1_ike_versions: core::default::Default::default(),
                tunnel1_inside_cidr: core::default::Default::default(),
                tunnel1_inside_ipv6_cidr: core::default::Default::default(),
                tunnel1_phase1_dh_group_numbers: core::default::Default::default(),
                tunnel1_phase1_encryption_algorithms: core::default::Default::default(),
                tunnel1_phase1_integrity_algorithms: core::default::Default::default(),
                tunnel1_phase1_lifetime_seconds: core::default::Default::default(),
                tunnel1_phase2_dh_group_numbers: core::default::Default::default(),
                tunnel1_phase2_encryption_algorithms: core::default::Default::default(),
                tunnel1_phase2_integrity_algorithms: core::default::Default::default(),
                tunnel1_phase2_lifetime_seconds: core::default::Default::default(),
                tunnel1_preshared_key: core::default::Default::default(),
                tunnel1_rekey_fuzz_percentage: core::default::Default::default(),
                tunnel1_rekey_margin_time_seconds: core::default::Default::default(),
                tunnel1_replay_window_size: core::default::Default::default(),
                tunnel1_startup_action: core::default::Default::default(),
                tunnel2_dpd_timeout_action: core::default::Default::default(),
                tunnel2_dpd_timeout_seconds: core::default::Default::default(),
                tunnel2_enable_tunnel_lifecycle_control: core::default::Default::default(),
                tunnel2_ike_versions: core::default::Default::default(),
                tunnel2_inside_cidr: core::default::Default::default(),
                tunnel2_inside_ipv6_cidr: core::default::Default::default(),
                tunnel2_phase1_dh_group_numbers: core::default::Default::default(),
                tunnel2_phase1_encryption_algorithms: core::default::Default::default(),
                tunnel2_phase1_integrity_algorithms: core::default::Default::default(),
                tunnel2_phase1_lifetime_seconds: core::default::Default::default(),
                tunnel2_phase2_dh_group_numbers: core::default::Default::default(),
                tunnel2_phase2_encryption_algorithms: core::default::Default::default(),
                tunnel2_phase2_integrity_algorithms: core::default::Default::default(),
                tunnel2_phase2_lifetime_seconds: core::default::Default::default(),
                tunnel2_preshared_key: core::default::Default::default(),
                tunnel2_rekey_fuzz_percentage: core::default::Default::default(),
                tunnel2_rekey_margin_time_seconds: core::default::Default::default(),
                tunnel2_replay_window_size: core::default::Default::default(),
                tunnel2_startup_action: core::default::Default::default(),
                tunnel_bandwidth: core::default::Default::default(),
                tunnel_inside_ip_version: core::default::Default::default(),
                type_: self.type_,
                vpn_gateway_id: core::default::Default::default(),
                tunnel1_log_options: core::default::Default::default(),
                tunnel2_log_options: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VpnConnectionRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpnConnectionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl VpnConnectionRef {
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

    #[doc = "Get a reference to the value of field `core_network_arn` after provisioning.\n"]
    pub fn core_network_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.core_network_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_attachment_arn` after provisioning.\n"]
    pub fn core_network_attachment_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.core_network_attachment_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_gateway_configuration` after provisioning.\n"]
    pub fn customer_gateway_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_gateway_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_gateway_id` after provisioning.\n"]
    pub fn customer_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_acceleration` after provisioning.\n"]
    pub fn enable_acceleration(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_acceleration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_ipv4_network_cidr` after provisioning.\n"]
    pub fn local_ipv4_network_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_ipv4_network_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_ipv6_network_cidr` after provisioning.\n"]
    pub fn local_ipv6_network_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_ipv6_network_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outside_ip_address_type` after provisioning.\n"]
    pub fn outside_ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.outside_ip_address_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preshared_key_arn` after provisioning.\n"]
    pub fn preshared_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preshared_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preshared_key_storage` after provisioning.\n"]
    pub fn preshared_key_storage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preshared_key_storage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_ipv4_network_cidr` after provisioning.\n"]
    pub fn remote_ipv4_network_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.remote_ipv4_network_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_ipv6_network_cidr` after provisioning.\n"]
    pub fn remote_ipv6_network_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.remote_ipv6_network_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> SetRef<VpnConnectionRoutesElRef> {
        SetRef::new(self.shared().clone(), format!("{}.routes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `static_routes_only` after provisioning.\n"]
    pub fn static_routes_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.static_routes_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_attachment_id` after provisioning.\n"]
    pub fn transit_gateway_attachment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_attachment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transport_transit_gateway_attachment_id` after provisioning.\n"]
    pub fn transport_transit_gateway_attachment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transport_transit_gateway_attachment_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_address` after provisioning.\n"]
    pub fn tunnel1_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_bgp_asn` after provisioning.\n"]
    pub fn tunnel1_bgp_asn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_bgp_asn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_bgp_holdtime` after provisioning.\n"]
    pub fn tunnel1_bgp_holdtime(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_bgp_holdtime", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_cgw_inside_address` after provisioning.\n"]
    pub fn tunnel1_cgw_inside_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_cgw_inside_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_dpd_timeout_action` after provisioning.\n"]
    pub fn tunnel1_dpd_timeout_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_dpd_timeout_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_dpd_timeout_seconds` after provisioning.\n"]
    pub fn tunnel1_dpd_timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_dpd_timeout_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_enable_tunnel_lifecycle_control` after provisioning.\n"]
    pub fn tunnel1_enable_tunnel_lifecycle_control(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tunnel1_enable_tunnel_lifecycle_control", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_ike_versions` after provisioning.\n"]
    pub fn tunnel1_ike_versions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel1_ike_versions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_inside_cidr` after provisioning.\n"]
    pub fn tunnel1_inside_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_inside_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_inside_ipv6_cidr` after provisioning.\n"]
    pub fn tunnel1_inside_ipv6_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_inside_ipv6_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase1_dh_group_numbers` after provisioning.\n"]
    pub fn tunnel1_phase1_dh_group_numbers(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel1_phase1_dh_group_numbers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase1_encryption_algorithms` after provisioning.\n"]
    pub fn tunnel1_phase1_encryption_algorithms(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel1_phase1_encryption_algorithms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase1_integrity_algorithms` after provisioning.\n"]
    pub fn tunnel1_phase1_integrity_algorithms(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel1_phase1_integrity_algorithms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase1_lifetime_seconds` after provisioning.\n"]
    pub fn tunnel1_phase1_lifetime_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_phase1_lifetime_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase2_dh_group_numbers` after provisioning.\n"]
    pub fn tunnel1_phase2_dh_group_numbers(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel1_phase2_dh_group_numbers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase2_encryption_algorithms` after provisioning.\n"]
    pub fn tunnel1_phase2_encryption_algorithms(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel1_phase2_encryption_algorithms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase2_integrity_algorithms` after provisioning.\n"]
    pub fn tunnel1_phase2_integrity_algorithms(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel1_phase2_integrity_algorithms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase2_lifetime_seconds` after provisioning.\n"]
    pub fn tunnel1_phase2_lifetime_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_phase2_lifetime_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_preshared_key` after provisioning.\n"]
    pub fn tunnel1_preshared_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_preshared_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_rekey_fuzz_percentage` after provisioning.\n"]
    pub fn tunnel1_rekey_fuzz_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_rekey_fuzz_percentage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_rekey_margin_time_seconds` after provisioning.\n"]
    pub fn tunnel1_rekey_margin_time_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_rekey_margin_time_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_replay_window_size` after provisioning.\n"]
    pub fn tunnel1_replay_window_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_replay_window_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_startup_action` after provisioning.\n"]
    pub fn tunnel1_startup_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_startup_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_vgw_inside_address` after provisioning.\n"]
    pub fn tunnel1_vgw_inside_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_vgw_inside_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_address` after provisioning.\n"]
    pub fn tunnel2_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_bgp_asn` after provisioning.\n"]
    pub fn tunnel2_bgp_asn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_bgp_asn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_bgp_holdtime` after provisioning.\n"]
    pub fn tunnel2_bgp_holdtime(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_bgp_holdtime", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_cgw_inside_address` after provisioning.\n"]
    pub fn tunnel2_cgw_inside_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_cgw_inside_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_dpd_timeout_action` after provisioning.\n"]
    pub fn tunnel2_dpd_timeout_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_dpd_timeout_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_dpd_timeout_seconds` after provisioning.\n"]
    pub fn tunnel2_dpd_timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_dpd_timeout_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_enable_tunnel_lifecycle_control` after provisioning.\n"]
    pub fn tunnel2_enable_tunnel_lifecycle_control(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tunnel2_enable_tunnel_lifecycle_control", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel2_ike_versions` after provisioning.\n"]
    pub fn tunnel2_ike_versions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel2_ike_versions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_inside_cidr` after provisioning.\n"]
    pub fn tunnel2_inside_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_inside_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_inside_ipv6_cidr` after provisioning.\n"]
    pub fn tunnel2_inside_ipv6_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_inside_ipv6_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase1_dh_group_numbers` after provisioning.\n"]
    pub fn tunnel2_phase1_dh_group_numbers(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel2_phase1_dh_group_numbers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase1_encryption_algorithms` after provisioning.\n"]
    pub fn tunnel2_phase1_encryption_algorithms(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel2_phase1_encryption_algorithms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase1_integrity_algorithms` after provisioning.\n"]
    pub fn tunnel2_phase1_integrity_algorithms(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel2_phase1_integrity_algorithms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase1_lifetime_seconds` after provisioning.\n"]
    pub fn tunnel2_phase1_lifetime_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_phase1_lifetime_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase2_dh_group_numbers` after provisioning.\n"]
    pub fn tunnel2_phase2_dh_group_numbers(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel2_phase2_dh_group_numbers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase2_encryption_algorithms` after provisioning.\n"]
    pub fn tunnel2_phase2_encryption_algorithms(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel2_phase2_encryption_algorithms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase2_integrity_algorithms` after provisioning.\n"]
    pub fn tunnel2_phase2_integrity_algorithms(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tunnel2_phase2_integrity_algorithms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase2_lifetime_seconds` after provisioning.\n"]
    pub fn tunnel2_phase2_lifetime_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_phase2_lifetime_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_preshared_key` after provisioning.\n"]
    pub fn tunnel2_preshared_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_preshared_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_rekey_fuzz_percentage` after provisioning.\n"]
    pub fn tunnel2_rekey_fuzz_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_rekey_fuzz_percentage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_rekey_margin_time_seconds` after provisioning.\n"]
    pub fn tunnel2_rekey_margin_time_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_rekey_margin_time_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_replay_window_size` after provisioning.\n"]
    pub fn tunnel2_replay_window_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_replay_window_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_startup_action` after provisioning.\n"]
    pub fn tunnel2_startup_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_startup_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_vgw_inside_address` after provisioning.\n"]
    pub fn tunnel2_vgw_inside_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_vgw_inside_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel_bandwidth` after provisioning.\n"]
    pub fn tunnel_bandwidth(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel_bandwidth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel_inside_ip_version` after provisioning.\n"]
    pub fn tunnel_inside_ip_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel_inside_ip_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vgw_telemetry` after provisioning.\n"]
    pub fn vgw_telemetry(&self) -> SetRef<VpnConnectionVgwTelemetryElRef> {
        SetRef::new(self.shared().clone(), format!("{}.vgw_telemetry", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpn_gateway_id` after provisioning.\n"]
    pub fn vpn_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpn_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_log_options` after provisioning.\n"]
    pub fn tunnel1_log_options(&self) -> ListRef<VpnConnectionTunnel1LogOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tunnel1_log_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_log_options` after provisioning.\n"]
    pub fn tunnel2_log_options(&self) -> ListRef<VpnConnectionTunnel2LogOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tunnel2_log_options", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct VpnConnectionRoutesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}

impl VpnConnectionRoutesEl {
    #[doc = "Set the field `destination_cidr_block`.\n"]
    pub fn set_destination_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_cidr_block = Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source = Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}

impl ToListMappable for VpnConnectionRoutesEl {
    type O = BlockAssignable<VpnConnectionRoutesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpnConnectionRoutesEl {}

impl BuildVpnConnectionRoutesEl {
    pub fn build(self) -> VpnConnectionRoutesEl {
        VpnConnectionRoutesEl {
            destination_cidr_block: core::default::Default::default(),
            source: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

pub struct VpnConnectionRoutesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpnConnectionRoutesElRef {
    fn new(shared: StackShared, base: String) -> VpnConnectionRoutesElRef {
        VpnConnectionRoutesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpnConnectionRoutesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_cidr_block` after provisioning.\n"]
    pub fn destination_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct VpnConnectionVgwTelemetryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accepted_route_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_status_change: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outside_ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_message: Option<PrimField<String>>,
}

impl VpnConnectionVgwTelemetryEl {
    #[doc = "Set the field `accepted_route_count`.\n"]
    pub fn set_accepted_route_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.accepted_route_count = Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `last_status_change`.\n"]
    pub fn set_last_status_change(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_status_change = Some(v.into());
        self
    }

    #[doc = "Set the field `outside_ip_address`.\n"]
    pub fn set_outside_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.outside_ip_address = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }

    #[doc = "Set the field `status_message`.\n"]
    pub fn set_status_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status_message = Some(v.into());
        self
    }
}

impl ToListMappable for VpnConnectionVgwTelemetryEl {
    type O = BlockAssignable<VpnConnectionVgwTelemetryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpnConnectionVgwTelemetryEl {}

impl BuildVpnConnectionVgwTelemetryEl {
    pub fn build(self) -> VpnConnectionVgwTelemetryEl {
        VpnConnectionVgwTelemetryEl {
            accepted_route_count: core::default::Default::default(),
            certificate_arn: core::default::Default::default(),
            last_status_change: core::default::Default::default(),
            outside_ip_address: core::default::Default::default(),
            status: core::default::Default::default(),
            status_message: core::default::Default::default(),
        }
    }
}

pub struct VpnConnectionVgwTelemetryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpnConnectionVgwTelemetryElRef {
    fn new(shared: StackShared, base: String) -> VpnConnectionVgwTelemetryElRef {
        VpnConnectionVgwTelemetryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpnConnectionVgwTelemetryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accepted_route_count` after provisioning.\n"]
    pub fn accepted_route_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.accepted_route_count", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `last_status_change` after provisioning.\n"]
    pub fn last_status_change(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_status_change", self.base))
    }

    #[doc = "Get a reference to the value of field `outside_ip_address` after provisioning.\n"]
    pub fn outside_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.outside_ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.base))
    }
}

#[derive(Serialize)]
pub struct VpnConnectionTunnel1LogOptionsElCloudwatchLogOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    log_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_group_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_output_format: Option<PrimField<String>>,
}

impl VpnConnectionTunnel1LogOptionsElCloudwatchLogOptionsEl {
    #[doc = "Set the field `log_enabled`.\n"]
    pub fn set_log_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.log_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `log_group_arn`.\n"]
    pub fn set_log_group_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_group_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `log_output_format`.\n"]
    pub fn set_log_output_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_output_format = Some(v.into());
        self
    }
}

impl ToListMappable for VpnConnectionTunnel1LogOptionsElCloudwatchLogOptionsEl {
    type O = BlockAssignable<VpnConnectionTunnel1LogOptionsElCloudwatchLogOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpnConnectionTunnel1LogOptionsElCloudwatchLogOptionsEl {}

impl BuildVpnConnectionTunnel1LogOptionsElCloudwatchLogOptionsEl {
    pub fn build(self) -> VpnConnectionTunnel1LogOptionsElCloudwatchLogOptionsEl {
        VpnConnectionTunnel1LogOptionsElCloudwatchLogOptionsEl {
            log_enabled: core::default::Default::default(),
            log_group_arn: core::default::Default::default(),
            log_output_format: core::default::Default::default(),
        }
    }
}

pub struct VpnConnectionTunnel1LogOptionsElCloudwatchLogOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpnConnectionTunnel1LogOptionsElCloudwatchLogOptionsElRef {
    fn new(shared: StackShared, base: String) -> VpnConnectionTunnel1LogOptionsElCloudwatchLogOptionsElRef {
        VpnConnectionTunnel1LogOptionsElCloudwatchLogOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpnConnectionTunnel1LogOptionsElCloudwatchLogOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_enabled` after provisioning.\n"]
    pub fn log_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `log_group_arn` after provisioning.\n"]
    pub fn log_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `log_output_format` after provisioning.\n"]
    pub fn log_output_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_output_format", self.base))
    }
}

#[derive(Serialize, Default)]
struct VpnConnectionTunnel1LogOptionsElDynamic {
    cloudwatch_log_options: Option<DynamicBlock<VpnConnectionTunnel1LogOptionsElCloudwatchLogOptionsEl>>,
}

#[derive(Serialize)]
pub struct VpnConnectionTunnel1LogOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_log_options: Option<Vec<VpnConnectionTunnel1LogOptionsElCloudwatchLogOptionsEl>>,
    dynamic: VpnConnectionTunnel1LogOptionsElDynamic,
}

impl VpnConnectionTunnel1LogOptionsEl {
    #[doc = "Set the field `cloudwatch_log_options`.\n"]
    pub fn set_cloudwatch_log_options(
        mut self,
        v: impl Into<BlockAssignable<VpnConnectionTunnel1LogOptionsElCloudwatchLogOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_log_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_log_options = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for VpnConnectionTunnel1LogOptionsEl {
    type O = BlockAssignable<VpnConnectionTunnel1LogOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpnConnectionTunnel1LogOptionsEl {}

impl BuildVpnConnectionTunnel1LogOptionsEl {
    pub fn build(self) -> VpnConnectionTunnel1LogOptionsEl {
        VpnConnectionTunnel1LogOptionsEl {
            cloudwatch_log_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct VpnConnectionTunnel1LogOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpnConnectionTunnel1LogOptionsElRef {
    fn new(shared: StackShared, base: String) -> VpnConnectionTunnel1LogOptionsElRef {
        VpnConnectionTunnel1LogOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpnConnectionTunnel1LogOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_log_options` after provisioning.\n"]
    pub fn cloudwatch_log_options(&self) -> ListRef<VpnConnectionTunnel1LogOptionsElCloudwatchLogOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloudwatch_log_options", self.base))
    }
}

#[derive(Serialize)]
pub struct VpnConnectionTunnel2LogOptionsElCloudwatchLogOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    log_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_group_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_output_format: Option<PrimField<String>>,
}

impl VpnConnectionTunnel2LogOptionsElCloudwatchLogOptionsEl {
    #[doc = "Set the field `log_enabled`.\n"]
    pub fn set_log_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.log_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `log_group_arn`.\n"]
    pub fn set_log_group_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_group_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `log_output_format`.\n"]
    pub fn set_log_output_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_output_format = Some(v.into());
        self
    }
}

impl ToListMappable for VpnConnectionTunnel2LogOptionsElCloudwatchLogOptionsEl {
    type O = BlockAssignable<VpnConnectionTunnel2LogOptionsElCloudwatchLogOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpnConnectionTunnel2LogOptionsElCloudwatchLogOptionsEl {}

impl BuildVpnConnectionTunnel2LogOptionsElCloudwatchLogOptionsEl {
    pub fn build(self) -> VpnConnectionTunnel2LogOptionsElCloudwatchLogOptionsEl {
        VpnConnectionTunnel2LogOptionsElCloudwatchLogOptionsEl {
            log_enabled: core::default::Default::default(),
            log_group_arn: core::default::Default::default(),
            log_output_format: core::default::Default::default(),
        }
    }
}

pub struct VpnConnectionTunnel2LogOptionsElCloudwatchLogOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpnConnectionTunnel2LogOptionsElCloudwatchLogOptionsElRef {
    fn new(shared: StackShared, base: String) -> VpnConnectionTunnel2LogOptionsElCloudwatchLogOptionsElRef {
        VpnConnectionTunnel2LogOptionsElCloudwatchLogOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpnConnectionTunnel2LogOptionsElCloudwatchLogOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_enabled` after provisioning.\n"]
    pub fn log_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `log_group_arn` after provisioning.\n"]
    pub fn log_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `log_output_format` after provisioning.\n"]
    pub fn log_output_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_output_format", self.base))
    }
}

#[derive(Serialize, Default)]
struct VpnConnectionTunnel2LogOptionsElDynamic {
    cloudwatch_log_options: Option<DynamicBlock<VpnConnectionTunnel2LogOptionsElCloudwatchLogOptionsEl>>,
}

#[derive(Serialize)]
pub struct VpnConnectionTunnel2LogOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_log_options: Option<Vec<VpnConnectionTunnel2LogOptionsElCloudwatchLogOptionsEl>>,
    dynamic: VpnConnectionTunnel2LogOptionsElDynamic,
}

impl VpnConnectionTunnel2LogOptionsEl {
    #[doc = "Set the field `cloudwatch_log_options`.\n"]
    pub fn set_cloudwatch_log_options(
        mut self,
        v: impl Into<BlockAssignable<VpnConnectionTunnel2LogOptionsElCloudwatchLogOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_log_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_log_options = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for VpnConnectionTunnel2LogOptionsEl {
    type O = BlockAssignable<VpnConnectionTunnel2LogOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpnConnectionTunnel2LogOptionsEl {}

impl BuildVpnConnectionTunnel2LogOptionsEl {
    pub fn build(self) -> VpnConnectionTunnel2LogOptionsEl {
        VpnConnectionTunnel2LogOptionsEl {
            cloudwatch_log_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct VpnConnectionTunnel2LogOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpnConnectionTunnel2LogOptionsElRef {
    fn new(shared: StackShared, base: String) -> VpnConnectionTunnel2LogOptionsElRef {
        VpnConnectionTunnel2LogOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpnConnectionTunnel2LogOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_log_options` after provisioning.\n"]
    pub fn cloudwatch_log_options(&self) -> ListRef<VpnConnectionTunnel2LogOptionsElCloudwatchLogOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloudwatch_log_options", self.base))
    }
}

#[derive(Serialize, Default)]
struct VpnConnectionDynamic {
    tunnel1_log_options: Option<DynamicBlock<VpnConnectionTunnel1LogOptionsEl>>,
    tunnel2_log_options: Option<DynamicBlock<VpnConnectionTunnel2LogOptionsEl>>,
}
