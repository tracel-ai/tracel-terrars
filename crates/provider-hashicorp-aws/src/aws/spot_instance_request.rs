use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SpotInstanceRequestData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ami: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    associate_public_ip_address: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_api_stop: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_api_termination: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs_optimized: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_primary_ipv6: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    get_password_data: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hibernation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_resource_group_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_instance_profile: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_initiated_shutdown_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_interruption_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_address_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitoring: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement_group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement_partition_number: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_private_ips: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_groups: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_dest_check: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot_price: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tenancy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_data: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_data_base64: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_data_replace_on_change: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_from: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_until: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_for_fulfillment: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_reservation_specification: Option<Vec<SpotInstanceRequestCapacityReservationSpecificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_options: Option<Vec<SpotInstanceRequestCpuOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credit_specification: Option<Vec<SpotInstanceRequestCreditSpecificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs_block_device: Option<Vec<SpotInstanceRequestEbsBlockDeviceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enclave_options: Option<Vec<SpotInstanceRequestEnclaveOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ephemeral_block_device: Option<Vec<SpotInstanceRequestEphemeralBlockDeviceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template: Option<Vec<SpotInstanceRequestLaunchTemplateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_options: Option<Vec<SpotInstanceRequestMaintenanceOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_options: Option<Vec<SpotInstanceRequestMetadataOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interface: Option<Vec<SpotInstanceRequestNetworkInterfaceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_dns_name_options: Option<Vec<SpotInstanceRequestPrivateDnsNameOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_block_device: Option<Vec<SpotInstanceRequestRootBlockDeviceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SpotInstanceRequestTimeoutsEl>,
    dynamic: SpotInstanceRequestDynamic,
}

struct SpotInstanceRequest_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SpotInstanceRequestData>,
}

#[derive(Clone)]
pub struct SpotInstanceRequest(Rc<SpotInstanceRequest_>);

impl SpotInstanceRequest {
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

    #[doc = "Set the field `ami`.\n"]
    pub fn set_ami(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ami = Some(v.into());
        self
    }

    #[doc = "Set the field `associate_public_ip_address`.\n"]
    pub fn set_associate_public_ip_address(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().associate_public_ip_address = Some(v.into());
        self
    }

    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().availability_zone = Some(v.into());
        self
    }

    #[doc = "Set the field `disable_api_stop`.\n"]
    pub fn set_disable_api_stop(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_api_stop = Some(v.into());
        self
    }

    #[doc = "Set the field `disable_api_termination`.\n"]
    pub fn set_disable_api_termination(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_api_termination = Some(v.into());
        self
    }

    #[doc = "Set the field `ebs_optimized`.\n"]
    pub fn set_ebs_optimized(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ebs_optimized = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_primary_ipv6`.\n"]
    pub fn set_enable_primary_ipv6(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_primary_ipv6 = Some(v.into());
        self
    }

    #[doc = "Set the field `force_destroy`.\n"]
    pub fn set_force_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = Some(v.into());
        self
    }

    #[doc = "Set the field `get_password_data`.\n"]
    pub fn set_get_password_data(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().get_password_data = Some(v.into());
        self
    }

    #[doc = "Set the field `hibernation`.\n"]
    pub fn set_hibernation(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().hibernation = Some(v.into());
        self
    }

    #[doc = "Set the field `host_id`.\n"]
    pub fn set_host_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().host_id = Some(v.into());
        self
    }

    #[doc = "Set the field `host_resource_group_arn`.\n"]
    pub fn set_host_resource_group_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().host_resource_group_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `iam_instance_profile`.\n"]
    pub fn set_iam_instance_profile(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().iam_instance_profile = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_initiated_shutdown_behavior`.\n"]
    pub fn set_instance_initiated_shutdown_behavior(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().instance_initiated_shutdown_behavior = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_interruption_behavior`.\n"]
    pub fn set_instance_interruption_behavior(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().instance_interruption_behavior = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().instance_type = Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_address_count`.\n"]
    pub fn set_ipv6_address_count(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ipv6_address_count = Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_addresses`.\n"]
    pub fn set_ipv6_addresses(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().ipv6_addresses = Some(v.into());
        self
    }

    #[doc = "Set the field `key_name`.\n"]
    pub fn set_key_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().key_name = Some(v.into());
        self
    }

    #[doc = "Set the field `launch_group`.\n"]
    pub fn set_launch_group(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().launch_group = Some(v.into());
        self
    }

    #[doc = "Set the field `monitoring`.\n"]
    pub fn set_monitoring(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().monitoring = Some(v.into());
        self
    }

    #[doc = "Set the field `placement_group`.\n"]
    pub fn set_placement_group(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().placement_group = Some(v.into());
        self
    }

    #[doc = "Set the field `placement_group_id`.\n"]
    pub fn set_placement_group_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().placement_group_id = Some(v.into());
        self
    }

    #[doc = "Set the field `placement_partition_number`.\n"]
    pub fn set_placement_partition_number(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().placement_partition_number = Some(v.into());
        self
    }

    #[doc = "Set the field `private_ip`.\n"]
    pub fn set_private_ip(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().private_ip = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `secondary_private_ips`.\n"]
    pub fn set_secondary_private_ips(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().secondary_private_ips = Some(v.into());
        self
    }

    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().security_groups = Some(v.into());
        self
    }

    #[doc = "Set the field `source_dest_check`.\n"]
    pub fn set_source_dest_check(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().source_dest_check = Some(v.into());
        self
    }

    #[doc = "Set the field `spot_price`.\n"]
    pub fn set_spot_price(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().spot_price = Some(v.into());
        self
    }

    #[doc = "Set the field `spot_type`.\n"]
    pub fn set_spot_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().spot_type = Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().subnet_id = Some(v.into());
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

    #[doc = "Set the field `tenancy`.\n"]
    pub fn set_tenancy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tenancy = Some(v.into());
        self
    }

    #[doc = "Set the field `user_data`.\n"]
    pub fn set_user_data(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().user_data = Some(v.into());
        self
    }

    #[doc = "Set the field `user_data_base64`.\n"]
    pub fn set_user_data_base64(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().user_data_base64 = Some(v.into());
        self
    }

    #[doc = "Set the field `user_data_replace_on_change`.\n"]
    pub fn set_user_data_replace_on_change(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().user_data_replace_on_change = Some(v.into());
        self
    }

    #[doc = "Set the field `valid_from`.\n"]
    pub fn set_valid_from(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().valid_from = Some(v.into());
        self
    }

    #[doc = "Set the field `valid_until`.\n"]
    pub fn set_valid_until(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().valid_until = Some(v.into());
        self
    }

    #[doc = "Set the field `volume_tags`.\n"]
    pub fn set_volume_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().volume_tags = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_security_group_ids`.\n"]
    pub fn set_vpc_security_group_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().vpc_security_group_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `wait_for_fulfillment`.\n"]
    pub fn set_wait_for_fulfillment(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_for_fulfillment = Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_reservation_specification`.\n"]
    pub fn set_capacity_reservation_specification(
        self,
        v: impl Into<BlockAssignable<SpotInstanceRequestCapacityReservationSpecificationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().capacity_reservation_specification = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.capacity_reservation_specification = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cpu_options`.\n"]
    pub fn set_cpu_options(self, v: impl Into<BlockAssignable<SpotInstanceRequestCpuOptionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cpu_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cpu_options = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `credit_specification`.\n"]
    pub fn set_credit_specification(
        self,
        v: impl Into<BlockAssignable<SpotInstanceRequestCreditSpecificationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().credit_specification = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.credit_specification = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ebs_block_device`.\n"]
    pub fn set_ebs_block_device(self, v: impl Into<BlockAssignable<SpotInstanceRequestEbsBlockDeviceEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ebs_block_device = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ebs_block_device = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `enclave_options`.\n"]
    pub fn set_enclave_options(self, v: impl Into<BlockAssignable<SpotInstanceRequestEnclaveOptionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().enclave_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.enclave_options = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ephemeral_block_device`.\n"]
    pub fn set_ephemeral_block_device(
        self,
        v: impl Into<BlockAssignable<SpotInstanceRequestEphemeralBlockDeviceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ephemeral_block_device = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ephemeral_block_device = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `launch_template`.\n"]
    pub fn set_launch_template(self, v: impl Into<BlockAssignable<SpotInstanceRequestLaunchTemplateEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().launch_template = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.launch_template = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maintenance_options`.\n"]
    pub fn set_maintenance_options(
        self,
        v: impl Into<BlockAssignable<SpotInstanceRequestMaintenanceOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_options = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metadata_options`.\n"]
    pub fn set_metadata_options(self, v: impl Into<BlockAssignable<SpotInstanceRequestMetadataOptionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metadata_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metadata_options = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_interface`.\n"]
    pub fn set_network_interface(self, v: impl Into<BlockAssignable<SpotInstanceRequestNetworkInterfaceEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_interface = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_interface = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `private_dns_name_options`.\n"]
    pub fn set_private_dns_name_options(
        self,
        v: impl Into<BlockAssignable<SpotInstanceRequestPrivateDnsNameOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().private_dns_name_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.private_dns_name_options = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `root_block_device`.\n"]
    pub fn set_root_block_device(self, v: impl Into<BlockAssignable<SpotInstanceRequestRootBlockDeviceEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().root_block_device = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.root_block_device = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<SpotInstanceRequestTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `ami` after provisioning.\n"]
    pub fn ami(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ami", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associate_public_ip_address` after provisioning.\n"]
    pub fn associate_public_ip_address(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.associate_public_ip_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disable_api_stop` after provisioning.\n"]
    pub fn disable_api_stop(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_api_stop", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disable_api_termination` after provisioning.\n"]
    pub fn disable_api_termination(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_api_termination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_optimized` after provisioning.\n"]
    pub fn ebs_optimized(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_optimized", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_primary_ipv6` after provisioning.\n"]
    pub fn enable_primary_ipv6(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_primary_ipv6", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `get_password_data` after provisioning.\n"]
    pub fn get_password_data(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.get_password_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hibernation` after provisioning.\n"]
    pub fn hibernation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.hibernation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_id` after provisioning.\n"]
    pub fn host_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_resource_group_arn` after provisioning.\n"]
    pub fn host_resource_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_resource_group_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_instance_profile` after provisioning.\n"]
    pub fn iam_instance_profile(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam_instance_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_initiated_shutdown_behavior` after provisioning.\n"]
    pub fn instance_initiated_shutdown_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_initiated_shutdown_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_interruption_behavior` after provisioning.\n"]
    pub fn instance_interruption_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_interruption_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_state` after provisioning.\n"]
    pub fn instance_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_address_count` after provisioning.\n"]
    pub fn ipv6_address_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_addresses` after provisioning.\n"]
    pub fn ipv6_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ipv6_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_group` after provisioning.\n"]
    pub fn launch_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.launch_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.monitoring", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.outpost_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password_data` after provisioning.\n"]
    pub fn password_data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_group` after provisioning.\n"]
    pub fn placement_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.placement_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_group_id` after provisioning.\n"]
    pub fn placement_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.placement_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_partition_number` after provisioning.\n"]
    pub fn placement_partition_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.placement_partition_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `primary_network_interface` after provisioning.\n"]
    pub fn primary_network_interface(&self) -> ListRef<SpotInstanceRequestPrimaryNetworkInterfaceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.primary_network_interface", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `primary_network_interface_id` after provisioning.\n"]
    pub fn primary_network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.primary_network_interface_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns` after provisioning.\n"]
    pub fn private_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_dns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_ip` after provisioning.\n"]
    pub fn private_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_dns` after provisioning.\n"]
    pub fn public_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_dns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_ip` after provisioning.\n"]
    pub fn public_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secondary_private_ips` after provisioning.\n"]
    pub fn secondary_private_ips(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.secondary_private_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_dest_check` after provisioning.\n"]
    pub fn source_dest_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_dest_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_bid_status` after provisioning.\n"]
    pub fn spot_bid_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.spot_bid_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_instance_id` after provisioning.\n"]
    pub fn spot_instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.spot_instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_price` after provisioning.\n"]
    pub fn spot_price(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.spot_price", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_request_state` after provisioning.\n"]
    pub fn spot_request_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.spot_request_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_type` after provisioning.\n"]
    pub fn spot_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.spot_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenancy` after provisioning.\n"]
    pub fn tenancy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tenancy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_data` after provisioning.\n"]
    pub fn user_data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_data_base64` after provisioning.\n"]
    pub fn user_data_base64(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_data_base64", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_data_replace_on_change` after provisioning.\n"]
    pub fn user_data_replace_on_change(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_data_replace_on_change", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `valid_from` after provisioning.\n"]
    pub fn valid_from(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.valid_from", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `valid_until` after provisioning.\n"]
    pub fn valid_until(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.valid_until", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume_tags` after provisioning.\n"]
    pub fn volume_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.volume_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_for_fulfillment` after provisioning.\n"]
    pub fn wait_for_fulfillment(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wait_for_fulfillment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_specification` after provisioning.\n"]
    pub fn capacity_reservation_specification(
        &self,
    ) -> ListRef<SpotInstanceRequestCapacityReservationSpecificationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.capacity_reservation_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpu_options` after provisioning.\n"]
    pub fn cpu_options(&self) -> ListRef<SpotInstanceRequestCpuOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cpu_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credit_specification` after provisioning.\n"]
    pub fn credit_specification(&self) -> ListRef<SpotInstanceRequestCreditSpecificationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.credit_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enclave_options` after provisioning.\n"]
    pub fn enclave_options(&self) -> ListRef<SpotInstanceRequestEnclaveOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.enclave_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> ListRef<SpotInstanceRequestLaunchTemplateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.launch_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_options` after provisioning.\n"]
    pub fn maintenance_options(&self) -> ListRef<SpotInstanceRequestMaintenanceOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.maintenance_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata_options` after provisioning.\n"]
    pub fn metadata_options(&self) -> ListRef<SpotInstanceRequestMetadataOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.metadata_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns_name_options` after provisioning.\n"]
    pub fn private_dns_name_options(&self) -> ListRef<SpotInstanceRequestPrivateDnsNameOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.private_dns_name_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_block_device` after provisioning.\n"]
    pub fn root_block_device(&self) -> ListRef<SpotInstanceRequestRootBlockDeviceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.root_block_device", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SpotInstanceRequestTimeoutsElRef {
        SpotInstanceRequestTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for SpotInstanceRequest {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for SpotInstanceRequest { }

impl ToListMappable for SpotInstanceRequest {
    type O = ListRef<SpotInstanceRequestRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SpotInstanceRequest_ {
    fn extract_resource_type(&self) -> String {
        "aws_spot_instance_request".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSpotInstanceRequest {
    pub tf_id: String,
}

impl BuildSpotInstanceRequest {
    pub fn build(self, stack: &mut Stack) -> SpotInstanceRequest {
        let out = SpotInstanceRequest(Rc::new(SpotInstanceRequest_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SpotInstanceRequestData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                ami: core::default::Default::default(),
                associate_public_ip_address: core::default::Default::default(),
                availability_zone: core::default::Default::default(),
                disable_api_stop: core::default::Default::default(),
                disable_api_termination: core::default::Default::default(),
                ebs_optimized: core::default::Default::default(),
                enable_primary_ipv6: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                get_password_data: core::default::Default::default(),
                hibernation: core::default::Default::default(),
                host_id: core::default::Default::default(),
                host_resource_group_arn: core::default::Default::default(),
                iam_instance_profile: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_initiated_shutdown_behavior: core::default::Default::default(),
                instance_interruption_behavior: core::default::Default::default(),
                instance_type: core::default::Default::default(),
                ipv6_address_count: core::default::Default::default(),
                ipv6_addresses: core::default::Default::default(),
                key_name: core::default::Default::default(),
                launch_group: core::default::Default::default(),
                monitoring: core::default::Default::default(),
                placement_group: core::default::Default::default(),
                placement_group_id: core::default::Default::default(),
                placement_partition_number: core::default::Default::default(),
                private_ip: core::default::Default::default(),
                region: core::default::Default::default(),
                secondary_private_ips: core::default::Default::default(),
                security_groups: core::default::Default::default(),
                source_dest_check: core::default::Default::default(),
                spot_price: core::default::Default::default(),
                spot_type: core::default::Default::default(),
                subnet_id: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                tenancy: core::default::Default::default(),
                user_data: core::default::Default::default(),
                user_data_base64: core::default::Default::default(),
                user_data_replace_on_change: core::default::Default::default(),
                valid_from: core::default::Default::default(),
                valid_until: core::default::Default::default(),
                volume_tags: core::default::Default::default(),
                vpc_security_group_ids: core::default::Default::default(),
                wait_for_fulfillment: core::default::Default::default(),
                capacity_reservation_specification: core::default::Default::default(),
                cpu_options: core::default::Default::default(),
                credit_specification: core::default::Default::default(),
                ebs_block_device: core::default::Default::default(),
                enclave_options: core::default::Default::default(),
                ephemeral_block_device: core::default::Default::default(),
                launch_template: core::default::Default::default(),
                maintenance_options: core::default::Default::default(),
                metadata_options: core::default::Default::default(),
                network_interface: core::default::Default::default(),
                private_dns_name_options: core::default::Default::default(),
                root_block_device: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SpotInstanceRequestRef {
    shared: StackShared,
    base: String,
}

impl Ref for SpotInstanceRequestRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl SpotInstanceRequestRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ami` after provisioning.\n"]
    pub fn ami(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ami", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associate_public_ip_address` after provisioning.\n"]
    pub fn associate_public_ip_address(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.associate_public_ip_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disable_api_stop` after provisioning.\n"]
    pub fn disable_api_stop(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_api_stop", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disable_api_termination` after provisioning.\n"]
    pub fn disable_api_termination(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_api_termination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_optimized` after provisioning.\n"]
    pub fn ebs_optimized(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_optimized", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_primary_ipv6` after provisioning.\n"]
    pub fn enable_primary_ipv6(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_primary_ipv6", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `get_password_data` after provisioning.\n"]
    pub fn get_password_data(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.get_password_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hibernation` after provisioning.\n"]
    pub fn hibernation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.hibernation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_id` after provisioning.\n"]
    pub fn host_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_resource_group_arn` after provisioning.\n"]
    pub fn host_resource_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_resource_group_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_instance_profile` after provisioning.\n"]
    pub fn iam_instance_profile(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam_instance_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_initiated_shutdown_behavior` after provisioning.\n"]
    pub fn instance_initiated_shutdown_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_initiated_shutdown_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_interruption_behavior` after provisioning.\n"]
    pub fn instance_interruption_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_interruption_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_state` after provisioning.\n"]
    pub fn instance_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_address_count` after provisioning.\n"]
    pub fn ipv6_address_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_addresses` after provisioning.\n"]
    pub fn ipv6_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ipv6_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_group` after provisioning.\n"]
    pub fn launch_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.launch_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.monitoring", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.outpost_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password_data` after provisioning.\n"]
    pub fn password_data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_group` after provisioning.\n"]
    pub fn placement_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.placement_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_group_id` after provisioning.\n"]
    pub fn placement_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.placement_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_partition_number` after provisioning.\n"]
    pub fn placement_partition_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.placement_partition_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `primary_network_interface` after provisioning.\n"]
    pub fn primary_network_interface(&self) -> ListRef<SpotInstanceRequestPrimaryNetworkInterfaceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.primary_network_interface", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `primary_network_interface_id` after provisioning.\n"]
    pub fn primary_network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.primary_network_interface_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns` after provisioning.\n"]
    pub fn private_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_dns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_ip` after provisioning.\n"]
    pub fn private_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_dns` after provisioning.\n"]
    pub fn public_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_dns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_ip` after provisioning.\n"]
    pub fn public_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secondary_private_ips` after provisioning.\n"]
    pub fn secondary_private_ips(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.secondary_private_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_dest_check` after provisioning.\n"]
    pub fn source_dest_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_dest_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_bid_status` after provisioning.\n"]
    pub fn spot_bid_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.spot_bid_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_instance_id` after provisioning.\n"]
    pub fn spot_instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.spot_instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_price` after provisioning.\n"]
    pub fn spot_price(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.spot_price", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_request_state` after provisioning.\n"]
    pub fn spot_request_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.spot_request_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_type` after provisioning.\n"]
    pub fn spot_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.spot_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenancy` after provisioning.\n"]
    pub fn tenancy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tenancy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_data` after provisioning.\n"]
    pub fn user_data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_data_base64` after provisioning.\n"]
    pub fn user_data_base64(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_data_base64", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_data_replace_on_change` after provisioning.\n"]
    pub fn user_data_replace_on_change(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_data_replace_on_change", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `valid_from` after provisioning.\n"]
    pub fn valid_from(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.valid_from", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `valid_until` after provisioning.\n"]
    pub fn valid_until(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.valid_until", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume_tags` after provisioning.\n"]
    pub fn volume_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.volume_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_for_fulfillment` after provisioning.\n"]
    pub fn wait_for_fulfillment(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wait_for_fulfillment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_specification` after provisioning.\n"]
    pub fn capacity_reservation_specification(
        &self,
    ) -> ListRef<SpotInstanceRequestCapacityReservationSpecificationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.capacity_reservation_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpu_options` after provisioning.\n"]
    pub fn cpu_options(&self) -> ListRef<SpotInstanceRequestCpuOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cpu_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credit_specification` after provisioning.\n"]
    pub fn credit_specification(&self) -> ListRef<SpotInstanceRequestCreditSpecificationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.credit_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enclave_options` after provisioning.\n"]
    pub fn enclave_options(&self) -> ListRef<SpotInstanceRequestEnclaveOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.enclave_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> ListRef<SpotInstanceRequestLaunchTemplateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.launch_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_options` after provisioning.\n"]
    pub fn maintenance_options(&self) -> ListRef<SpotInstanceRequestMaintenanceOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.maintenance_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata_options` after provisioning.\n"]
    pub fn metadata_options(&self) -> ListRef<SpotInstanceRequestMetadataOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.metadata_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns_name_options` after provisioning.\n"]
    pub fn private_dns_name_options(&self) -> ListRef<SpotInstanceRequestPrivateDnsNameOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.private_dns_name_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_block_device` after provisioning.\n"]
    pub fn root_block_device(&self) -> ListRef<SpotInstanceRequestRootBlockDeviceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.root_block_device", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SpotInstanceRequestTimeoutsElRef {
        SpotInstanceRequestTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SpotInstanceRequestPrimaryNetworkInterfaceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_on_termination: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interface_id: Option<PrimField<String>>,
}

impl SpotInstanceRequestPrimaryNetworkInterfaceEl {
    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.delete_on_termination = Some(v.into());
        self
    }

    #[doc = "Set the field `network_interface_id`.\n"]
    pub fn set_network_interface_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_interface_id = Some(v.into());
        self
    }
}

impl ToListMappable for SpotInstanceRequestPrimaryNetworkInterfaceEl {
    type O = BlockAssignable<SpotInstanceRequestPrimaryNetworkInterfaceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestPrimaryNetworkInterfaceEl {}

impl BuildSpotInstanceRequestPrimaryNetworkInterfaceEl {
    pub fn build(self) -> SpotInstanceRequestPrimaryNetworkInterfaceEl {
        SpotInstanceRequestPrimaryNetworkInterfaceEl {
            delete_on_termination: core::default::Default::default(),
            network_interface_id: core::default::Default::default(),
        }
    }
}

pub struct SpotInstanceRequestPrimaryNetworkInterfaceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SpotInstanceRequestPrimaryNetworkInterfaceElRef {
    fn new(shared: StackShared, base: String) -> SpotInstanceRequestPrimaryNetworkInterfaceElRef {
        SpotInstanceRequestPrimaryNetworkInterfaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestPrimaryNetworkInterfaceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_on_termination", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_interface_id", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_reservation_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_reservation_resource_group_arn: Option<PrimField<String>>,
}

impl SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetEl {
    #[doc = "Set the field `capacity_reservation_id`.\n"]
    pub fn set_capacity_reservation_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.capacity_reservation_id = Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_reservation_resource_group_arn`.\n"]
    pub fn set_capacity_reservation_resource_group_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.capacity_reservation_resource_group_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetEl {
    type O = BlockAssignable<SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetEl {}

impl BuildSpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetEl {
    pub fn build(self) -> SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetEl {
        SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetEl {
            capacity_reservation_id: core::default::Default::default(),
            capacity_reservation_resource_group_arn: core::default::Default::default(),
        }
    }
}

pub struct SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetElRef {
        SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_id` after provisioning.\n"]
    pub fn capacity_reservation_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.capacity_reservation_id", self.base))
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_resource_group_arn` after provisioning.\n"]
    pub fn capacity_reservation_resource_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.capacity_reservation_resource_group_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SpotInstanceRequestCapacityReservationSpecificationElDynamic {
    capacity_reservation_target: Option<
        DynamicBlock<SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetEl>,
    >,
}

#[derive(Serialize)]
pub struct SpotInstanceRequestCapacityReservationSpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_reservation_preference: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_reservation_target: Option<
        Vec<SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetEl>,
    >,
    dynamic: SpotInstanceRequestCapacityReservationSpecificationElDynamic,
}

impl SpotInstanceRequestCapacityReservationSpecificationEl {
    #[doc = "Set the field `capacity_reservation_preference`.\n"]
    pub fn set_capacity_reservation_preference(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.capacity_reservation_preference = Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_reservation_target`.\n"]
    pub fn set_capacity_reservation_target(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.capacity_reservation_target = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.capacity_reservation_target = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SpotInstanceRequestCapacityReservationSpecificationEl {
    type O = BlockAssignable<SpotInstanceRequestCapacityReservationSpecificationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestCapacityReservationSpecificationEl {}

impl BuildSpotInstanceRequestCapacityReservationSpecificationEl {
    pub fn build(self) -> SpotInstanceRequestCapacityReservationSpecificationEl {
        SpotInstanceRequestCapacityReservationSpecificationEl {
            capacity_reservation_preference: core::default::Default::default(),
            capacity_reservation_target: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SpotInstanceRequestCapacityReservationSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SpotInstanceRequestCapacityReservationSpecificationElRef {
    fn new(shared: StackShared, base: String) -> SpotInstanceRequestCapacityReservationSpecificationElRef {
        SpotInstanceRequestCapacityReservationSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestCapacityReservationSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_preference` after provisioning.\n"]
    pub fn capacity_reservation_preference(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.capacity_reservation_preference", self.base))
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_target` after provisioning.\n"]
    pub fn capacity_reservation_target(
        &self,
    ) -> ListRef<SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.capacity_reservation_target", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotInstanceRequestCpuOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    amd_sev_snp: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    core_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threads_per_core: Option<PrimField<f64>>,
}

impl SpotInstanceRequestCpuOptionsEl {
    #[doc = "Set the field `amd_sev_snp`.\n"]
    pub fn set_amd_sev_snp(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.amd_sev_snp = Some(v.into());
        self
    }

    #[doc = "Set the field `core_count`.\n"]
    pub fn set_core_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.core_count = Some(v.into());
        self
    }

    #[doc = "Set the field `threads_per_core`.\n"]
    pub fn set_threads_per_core(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.threads_per_core = Some(v.into());
        self
    }
}

impl ToListMappable for SpotInstanceRequestCpuOptionsEl {
    type O = BlockAssignable<SpotInstanceRequestCpuOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestCpuOptionsEl {}

impl BuildSpotInstanceRequestCpuOptionsEl {
    pub fn build(self) -> SpotInstanceRequestCpuOptionsEl {
        SpotInstanceRequestCpuOptionsEl {
            amd_sev_snp: core::default::Default::default(),
            core_count: core::default::Default::default(),
            threads_per_core: core::default::Default::default(),
        }
    }
}

pub struct SpotInstanceRequestCpuOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SpotInstanceRequestCpuOptionsElRef {
    fn new(shared: StackShared, base: String) -> SpotInstanceRequestCpuOptionsElRef {
        SpotInstanceRequestCpuOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestCpuOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amd_sev_snp` after provisioning.\n"]
    pub fn amd_sev_snp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.amd_sev_snp", self.base))
    }

    #[doc = "Get a reference to the value of field `core_count` after provisioning.\n"]
    pub fn core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.core_count", self.base))
    }

    #[doc = "Get a reference to the value of field `threads_per_core` after provisioning.\n"]
    pub fn threads_per_core(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.threads_per_core", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotInstanceRequestCreditSpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_credits: Option<PrimField<String>>,
}

impl SpotInstanceRequestCreditSpecificationEl {
    #[doc = "Set the field `cpu_credits`.\n"]
    pub fn set_cpu_credits(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cpu_credits = Some(v.into());
        self
    }
}

impl ToListMappable for SpotInstanceRequestCreditSpecificationEl {
    type O = BlockAssignable<SpotInstanceRequestCreditSpecificationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestCreditSpecificationEl {}

impl BuildSpotInstanceRequestCreditSpecificationEl {
    pub fn build(self) -> SpotInstanceRequestCreditSpecificationEl {
        SpotInstanceRequestCreditSpecificationEl { cpu_credits: core::default::Default::default() }
    }
}

pub struct SpotInstanceRequestCreditSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SpotInstanceRequestCreditSpecificationElRef {
    fn new(shared: StackShared, base: String) -> SpotInstanceRequestCreditSpecificationElRef {
        SpotInstanceRequestCreditSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestCreditSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu_credits` after provisioning.\n"]
    pub fn cpu_credits(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_credits", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotInstanceRequestEbsBlockDeviceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_on_termination: Option<PrimField<bool>>,
    device_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encrypted: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throughput: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_type: Option<PrimField<String>>,
}

impl SpotInstanceRequestEbsBlockDeviceEl {
    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.delete_on_termination = Some(v.into());
        self
    }

    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.encrypted = Some(v.into());
        self
    }

    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.iops = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_id`.\n"]
    pub fn set_snapshot_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.snapshot_id = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `throughput`.\n"]
    pub fn set_throughput(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.throughput = Some(v.into());
        self
    }

    #[doc = "Set the field `volume_size`.\n"]
    pub fn set_volume_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volume_size = Some(v.into());
        self
    }

    #[doc = "Set the field `volume_type`.\n"]
    pub fn set_volume_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.volume_type = Some(v.into());
        self
    }
}

impl ToListMappable for SpotInstanceRequestEbsBlockDeviceEl {
    type O = BlockAssignable<SpotInstanceRequestEbsBlockDeviceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestEbsBlockDeviceEl {
    #[doc = ""]
    pub device_name: PrimField<String>,
}

impl BuildSpotInstanceRequestEbsBlockDeviceEl {
    pub fn build(self) -> SpotInstanceRequestEbsBlockDeviceEl {
        SpotInstanceRequestEbsBlockDeviceEl {
            delete_on_termination: core::default::Default::default(),
            device_name: self.device_name,
            encrypted: core::default::Default::default(),
            iops: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            snapshot_id: core::default::Default::default(),
            tags: core::default::Default::default(),
            tags_all: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

pub struct SpotInstanceRequestEbsBlockDeviceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SpotInstanceRequestEbsBlockDeviceElRef {
    fn new(shared: StackShared, base: String) -> SpotInstanceRequestEbsBlockDeviceElRef {
        SpotInstanceRequestEbsBlockDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestEbsBlockDeviceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_on_termination", self.base))
    }

    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.base))
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_id` after provisioning.\n"]
    pub fn snapshot_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_id", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.base))
    }

    #[doc = "Get a reference to the value of field `throughput` after provisioning.\n"]
    pub fn throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_id` after provisioning.\n"]
    pub fn volume_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_id", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_size", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotInstanceRequestEnclaveOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl SpotInstanceRequestEnclaveOptionsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for SpotInstanceRequestEnclaveOptionsEl {
    type O = BlockAssignable<SpotInstanceRequestEnclaveOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestEnclaveOptionsEl {}

impl BuildSpotInstanceRequestEnclaveOptionsEl {
    pub fn build(self) -> SpotInstanceRequestEnclaveOptionsEl {
        SpotInstanceRequestEnclaveOptionsEl { enabled: core::default::Default::default() }
    }
}

pub struct SpotInstanceRequestEnclaveOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SpotInstanceRequestEnclaveOptionsElRef {
    fn new(shared: StackShared, base: String) -> SpotInstanceRequestEnclaveOptionsElRef {
        SpotInstanceRequestEnclaveOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestEnclaveOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotInstanceRequestEphemeralBlockDeviceEl {
    device_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_device: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_name: Option<PrimField<String>>,
}

impl SpotInstanceRequestEphemeralBlockDeviceEl {
    #[doc = "Set the field `no_device`.\n"]
    pub fn set_no_device(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.no_device = Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_name`.\n"]
    pub fn set_virtual_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.virtual_name = Some(v.into());
        self
    }
}

impl ToListMappable for SpotInstanceRequestEphemeralBlockDeviceEl {
    type O = BlockAssignable<SpotInstanceRequestEphemeralBlockDeviceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestEphemeralBlockDeviceEl {
    #[doc = ""]
    pub device_name: PrimField<String>,
}

impl BuildSpotInstanceRequestEphemeralBlockDeviceEl {
    pub fn build(self) -> SpotInstanceRequestEphemeralBlockDeviceEl {
        SpotInstanceRequestEphemeralBlockDeviceEl {
            device_name: self.device_name,
            no_device: core::default::Default::default(),
            virtual_name: core::default::Default::default(),
        }
    }
}

pub struct SpotInstanceRequestEphemeralBlockDeviceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SpotInstanceRequestEphemeralBlockDeviceElRef {
    fn new(shared: StackShared, base: String) -> SpotInstanceRequestEphemeralBlockDeviceElRef {
        SpotInstanceRequestEphemeralBlockDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestEphemeralBlockDeviceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc = "Get a reference to the value of field `no_device` after provisioning.\n"]
    pub fn no_device(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.no_device", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_name` after provisioning.\n"]
    pub fn virtual_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtual_name", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotInstanceRequestLaunchTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl SpotInstanceRequestLaunchTemplateEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}

impl ToListMappable for SpotInstanceRequestLaunchTemplateEl {
    type O = BlockAssignable<SpotInstanceRequestLaunchTemplateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestLaunchTemplateEl {}

impl BuildSpotInstanceRequestLaunchTemplateEl {
    pub fn build(self) -> SpotInstanceRequestLaunchTemplateEl {
        SpotInstanceRequestLaunchTemplateEl {
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct SpotInstanceRequestLaunchTemplateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SpotInstanceRequestLaunchTemplateElRef {
    fn new(shared: StackShared, base: String) -> SpotInstanceRequestLaunchTemplateElRef {
        SpotInstanceRequestLaunchTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestLaunchTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotInstanceRequestMaintenanceOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_recovery: Option<PrimField<String>>,
}

impl SpotInstanceRequestMaintenanceOptionsEl {
    #[doc = "Set the field `auto_recovery`.\n"]
    pub fn set_auto_recovery(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auto_recovery = Some(v.into());
        self
    }
}

impl ToListMappable for SpotInstanceRequestMaintenanceOptionsEl {
    type O = BlockAssignable<SpotInstanceRequestMaintenanceOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestMaintenanceOptionsEl {}

impl BuildSpotInstanceRequestMaintenanceOptionsEl {
    pub fn build(self) -> SpotInstanceRequestMaintenanceOptionsEl {
        SpotInstanceRequestMaintenanceOptionsEl { auto_recovery: core::default::Default::default() }
    }
}

pub struct SpotInstanceRequestMaintenanceOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SpotInstanceRequestMaintenanceOptionsElRef {
    fn new(shared: StackShared, base: String) -> SpotInstanceRequestMaintenanceOptionsElRef {
        SpotInstanceRequestMaintenanceOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestMaintenanceOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_recovery` after provisioning.\n"]
    pub fn auto_recovery(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_recovery", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotInstanceRequestMetadataOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_protocol_ipv6: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_put_response_hop_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_tokens: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_metadata_tags: Option<PrimField<String>>,
}

impl SpotInstanceRequestMetadataOptionsEl {
    #[doc = "Set the field `http_endpoint`.\n"]
    pub fn set_http_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http_endpoint = Some(v.into());
        self
    }

    #[doc = "Set the field `http_protocol_ipv6`.\n"]
    pub fn set_http_protocol_ipv6(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http_protocol_ipv6 = Some(v.into());
        self
    }

    #[doc = "Set the field `http_put_response_hop_limit`.\n"]
    pub fn set_http_put_response_hop_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.http_put_response_hop_limit = Some(v.into());
        self
    }

    #[doc = "Set the field `http_tokens`.\n"]
    pub fn set_http_tokens(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http_tokens = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_metadata_tags`.\n"]
    pub fn set_instance_metadata_tags(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_metadata_tags = Some(v.into());
        self
    }
}

impl ToListMappable for SpotInstanceRequestMetadataOptionsEl {
    type O = BlockAssignable<SpotInstanceRequestMetadataOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestMetadataOptionsEl {}

impl BuildSpotInstanceRequestMetadataOptionsEl {
    pub fn build(self) -> SpotInstanceRequestMetadataOptionsEl {
        SpotInstanceRequestMetadataOptionsEl {
            http_endpoint: core::default::Default::default(),
            http_protocol_ipv6: core::default::Default::default(),
            http_put_response_hop_limit: core::default::Default::default(),
            http_tokens: core::default::Default::default(),
            instance_metadata_tags: core::default::Default::default(),
        }
    }
}

pub struct SpotInstanceRequestMetadataOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SpotInstanceRequestMetadataOptionsElRef {
    fn new(shared: StackShared, base: String) -> SpotInstanceRequestMetadataOptionsElRef {
        SpotInstanceRequestMetadataOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestMetadataOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `http_endpoint` after provisioning.\n"]
    pub fn http_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `http_protocol_ipv6` after provisioning.\n"]
    pub fn http_protocol_ipv6(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_protocol_ipv6", self.base))
    }

    #[doc = "Get a reference to the value of field `http_put_response_hop_limit` after provisioning.\n"]
    pub fn http_put_response_hop_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_put_response_hop_limit", self.base))
    }

    #[doc = "Get a reference to the value of field `http_tokens` after provisioning.\n"]
    pub fn http_tokens(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_tokens", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_metadata_tags` after provisioning.\n"]
    pub fn instance_metadata_tags(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_metadata_tags", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotInstanceRequestNetworkInterfaceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_on_termination: Option<PrimField<bool>>,
    device_index: PrimField<f64>,
    network_interface_id: PrimField<String>,
}

impl SpotInstanceRequestNetworkInterfaceEl {
    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.delete_on_termination = Some(v.into());
        self
    }
}

impl ToListMappable for SpotInstanceRequestNetworkInterfaceEl {
    type O = BlockAssignable<SpotInstanceRequestNetworkInterfaceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestNetworkInterfaceEl {
    #[doc = ""]
    pub device_index: PrimField<f64>,
    #[doc = ""]
    pub network_interface_id: PrimField<String>,
}

impl BuildSpotInstanceRequestNetworkInterfaceEl {
    pub fn build(self) -> SpotInstanceRequestNetworkInterfaceEl {
        SpotInstanceRequestNetworkInterfaceEl {
            delete_on_termination: core::default::Default::default(),
            device_index: self.device_index,
            network_interface_id: self.network_interface_id,
        }
    }
}

pub struct SpotInstanceRequestNetworkInterfaceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SpotInstanceRequestNetworkInterfaceElRef {
    fn new(shared: StackShared, base: String) -> SpotInstanceRequestNetworkInterfaceElRef {
        SpotInstanceRequestNetworkInterfaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestNetworkInterfaceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_on_termination", self.base))
    }

    #[doc = "Get a reference to the value of field `device_index` after provisioning.\n"]
    pub fn device_index(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_index", self.base))
    }

    #[doc = "Get a reference to the value of field `network_card_index` after provisioning.\n"]
    pub fn network_card_index(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_card_index", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_interface_id", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotInstanceRequestPrivateDnsNameOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_resource_name_dns_a_record: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_resource_name_dns_aaaa_record: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname_type: Option<PrimField<String>>,
}

impl SpotInstanceRequestPrivateDnsNameOptionsEl {
    #[doc = "Set the field `enable_resource_name_dns_a_record`.\n"]
    pub fn set_enable_resource_name_dns_a_record(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_resource_name_dns_a_record = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_resource_name_dns_aaaa_record`.\n"]
    pub fn set_enable_resource_name_dns_aaaa_record(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_resource_name_dns_aaaa_record = Some(v.into());
        self
    }

    #[doc = "Set the field `hostname_type`.\n"]
    pub fn set_hostname_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hostname_type = Some(v.into());
        self
    }
}

impl ToListMappable for SpotInstanceRequestPrivateDnsNameOptionsEl {
    type O = BlockAssignable<SpotInstanceRequestPrivateDnsNameOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestPrivateDnsNameOptionsEl {}

impl BuildSpotInstanceRequestPrivateDnsNameOptionsEl {
    pub fn build(self) -> SpotInstanceRequestPrivateDnsNameOptionsEl {
        SpotInstanceRequestPrivateDnsNameOptionsEl {
            enable_resource_name_dns_a_record: core::default::Default::default(),
            enable_resource_name_dns_aaaa_record: core::default::Default::default(),
            hostname_type: core::default::Default::default(),
        }
    }
}

pub struct SpotInstanceRequestPrivateDnsNameOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SpotInstanceRequestPrivateDnsNameOptionsElRef {
    fn new(shared: StackShared, base: String) -> SpotInstanceRequestPrivateDnsNameOptionsElRef {
        SpotInstanceRequestPrivateDnsNameOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestPrivateDnsNameOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_resource_name_dns_a_record` after provisioning.\n"]
    pub fn enable_resource_name_dns_a_record(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_resource_name_dns_a_record", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_resource_name_dns_aaaa_record` after provisioning.\n"]
    pub fn enable_resource_name_dns_aaaa_record(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_resource_name_dns_aaaa_record", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname_type` after provisioning.\n"]
    pub fn hostname_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname_type", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotInstanceRequestRootBlockDeviceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_on_termination: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encrypted: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throughput: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_type: Option<PrimField<String>>,
}

impl SpotInstanceRequestRootBlockDeviceEl {
    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.delete_on_termination = Some(v.into());
        self
    }

    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.encrypted = Some(v.into());
        self
    }

    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.iops = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `throughput`.\n"]
    pub fn set_throughput(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.throughput = Some(v.into());
        self
    }

    #[doc = "Set the field `volume_size`.\n"]
    pub fn set_volume_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volume_size = Some(v.into());
        self
    }

    #[doc = "Set the field `volume_type`.\n"]
    pub fn set_volume_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.volume_type = Some(v.into());
        self
    }
}

impl ToListMappable for SpotInstanceRequestRootBlockDeviceEl {
    type O = BlockAssignable<SpotInstanceRequestRootBlockDeviceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestRootBlockDeviceEl {}

impl BuildSpotInstanceRequestRootBlockDeviceEl {
    pub fn build(self) -> SpotInstanceRequestRootBlockDeviceEl {
        SpotInstanceRequestRootBlockDeviceEl {
            delete_on_termination: core::default::Default::default(),
            encrypted: core::default::Default::default(),
            iops: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            tags: core::default::Default::default(),
            tags_all: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

pub struct SpotInstanceRequestRootBlockDeviceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SpotInstanceRequestRootBlockDeviceElRef {
    fn new(shared: StackShared, base: String) -> SpotInstanceRequestRootBlockDeviceElRef {
        SpotInstanceRequestRootBlockDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestRootBlockDeviceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_on_termination", self.base))
    }

    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.base))
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.base))
    }

    #[doc = "Get a reference to the value of field `throughput` after provisioning.\n"]
    pub fn throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_id` after provisioning.\n"]
    pub fn volume_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_id", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_size", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotInstanceRequestTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}

impl SpotInstanceRequestTimeoutsEl {
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

    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}

impl ToListMappable for SpotInstanceRequestTimeoutsEl {
    type O = BlockAssignable<SpotInstanceRequestTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestTimeoutsEl {}

impl BuildSpotInstanceRequestTimeoutsEl {
    pub fn build(self) -> SpotInstanceRequestTimeoutsEl {
        SpotInstanceRequestTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            read: core::default::Default::default(),
        }
    }
}

pub struct SpotInstanceRequestTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SpotInstanceRequestTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> SpotInstanceRequestTimeoutsElRef {
        SpotInstanceRequestTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestTimeoutsElRef {
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

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct SpotInstanceRequestDynamic {
    capacity_reservation_specification: Option<DynamicBlock<SpotInstanceRequestCapacityReservationSpecificationEl>>,
    cpu_options: Option<DynamicBlock<SpotInstanceRequestCpuOptionsEl>>,
    credit_specification: Option<DynamicBlock<SpotInstanceRequestCreditSpecificationEl>>,
    ebs_block_device: Option<DynamicBlock<SpotInstanceRequestEbsBlockDeviceEl>>,
    enclave_options: Option<DynamicBlock<SpotInstanceRequestEnclaveOptionsEl>>,
    ephemeral_block_device: Option<DynamicBlock<SpotInstanceRequestEphemeralBlockDeviceEl>>,
    launch_template: Option<DynamicBlock<SpotInstanceRequestLaunchTemplateEl>>,
    maintenance_options: Option<DynamicBlock<SpotInstanceRequestMaintenanceOptionsEl>>,
    metadata_options: Option<DynamicBlock<SpotInstanceRequestMetadataOptionsEl>>,
    network_interface: Option<DynamicBlock<SpotInstanceRequestNetworkInterfaceEl>>,
    private_dns_name_options: Option<DynamicBlock<SpotInstanceRequestPrivateDnsNameOptionsEl>>,
    root_block_device: Option<DynamicBlock<SpotInstanceRequestRootBlockDeviceEl>>,
}
