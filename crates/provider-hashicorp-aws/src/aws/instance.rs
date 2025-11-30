use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct InstanceData {
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
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_address_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_name: Option<PrimField<String>>,
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
    volume_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_reservation_specification: Option<Vec<InstanceCapacityReservationSpecificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_options: Option<Vec<InstanceCpuOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credit_specification: Option<Vec<InstanceCreditSpecificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs_block_device: Option<Vec<InstanceEbsBlockDeviceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enclave_options: Option<Vec<InstanceEnclaveOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ephemeral_block_device: Option<Vec<InstanceEphemeralBlockDeviceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_market_options: Option<Vec<InstanceInstanceMarketOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template: Option<Vec<InstanceLaunchTemplateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_options: Option<Vec<InstanceMaintenanceOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_options: Option<Vec<InstanceMetadataOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interface: Option<Vec<InstanceNetworkInterfaceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_network_interface: Option<Vec<InstancePrimaryNetworkInterfaceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_dns_name_options: Option<Vec<InstancePrivateDnsNameOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_block_device: Option<Vec<InstanceRootBlockDeviceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<InstanceTimeoutsEl>,
    dynamic: InstanceDynamic,
}

struct Instance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<InstanceData>,
}

#[derive(Clone)]
pub struct Instance(Rc<Instance_>);

impl Instance {
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
        self.0
            .data
            .borrow_mut()
            .instance_initiated_shutdown_behavior = Some(v.into());
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

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
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

    #[doc = "Set the field `capacity_reservation_specification`.\n"]
    pub fn set_capacity_reservation_specification(
        self,
        v: impl Into<BlockAssignable<InstanceCapacityReservationSpecificationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().capacity_reservation_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .capacity_reservation_specification = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `cpu_options`.\n"]
    pub fn set_cpu_options(self, v: impl Into<BlockAssignable<InstanceCpuOptionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cpu_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cpu_options = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `credit_specification`.\n"]
    pub fn set_credit_specification(
        self,
        v: impl Into<BlockAssignable<InstanceCreditSpecificationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().credit_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.credit_specification = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `ebs_block_device`.\n"]
    pub fn set_ebs_block_device(
        self,
        v: impl Into<BlockAssignable<InstanceEbsBlockDeviceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ebs_block_device = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ebs_block_device = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `enclave_options`.\n"]
    pub fn set_enclave_options(
        self,
        v: impl Into<BlockAssignable<InstanceEnclaveOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().enclave_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.enclave_options = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `ephemeral_block_device`.\n"]
    pub fn set_ephemeral_block_device(
        self,
        v: impl Into<BlockAssignable<InstanceEphemeralBlockDeviceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ephemeral_block_device = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ephemeral_block_device = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `instance_market_options`.\n"]
    pub fn set_instance_market_options(
        self,
        v: impl Into<BlockAssignable<InstanceInstanceMarketOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_market_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_market_options = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `launch_template`.\n"]
    pub fn set_launch_template(
        self,
        v: impl Into<BlockAssignable<InstanceLaunchTemplateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().launch_template = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.launch_template = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `maintenance_options`.\n"]
    pub fn set_maintenance_options(
        self,
        v: impl Into<BlockAssignable<InstanceMaintenanceOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_options = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `metadata_options`.\n"]
    pub fn set_metadata_options(
        self,
        v: impl Into<BlockAssignable<InstanceMetadataOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metadata_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metadata_options = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `network_interface`.\n"]
    pub fn set_network_interface(
        self,
        v: impl Into<BlockAssignable<InstanceNetworkInterfaceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_interface = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_interface = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `primary_network_interface`.\n"]
    pub fn set_primary_network_interface(
        self,
        v: impl Into<BlockAssignable<InstancePrimaryNetworkInterfaceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().primary_network_interface = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.primary_network_interface = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `private_dns_name_options`.\n"]
    pub fn set_private_dns_name_options(
        self,
        v: impl Into<BlockAssignable<InstancePrivateDnsNameOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().private_dns_name_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.private_dns_name_options = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `root_block_device`.\n"]
    pub fn set_root_block_device(
        self,
        v: impl Into<BlockAssignable<InstanceRootBlockDeviceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().root_block_device = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.root_block_device = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<InstanceTimeoutsEl>) -> Self {
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
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.associate_public_ip_address", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `disable_api_stop` after provisioning.\n"]
    pub fn disable_api_stop(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_api_stop", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `disable_api_termination` after provisioning.\n"]
    pub fn disable_api_termination(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_api_termination", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ebs_optimized` after provisioning.\n"]
    pub fn ebs_optimized(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ebs_optimized", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_primary_ipv6` after provisioning.\n"]
    pub fn enable_primary_ipv6(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_primary_ipv6", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_destroy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `get_password_data` after provisioning.\n"]
    pub fn get_password_data(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.get_password_data", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `hibernation` after provisioning.\n"]
    pub fn hibernation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hibernation", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `host_id` after provisioning.\n"]
    pub fn host_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `host_resource_group_arn` after provisioning.\n"]
    pub fn host_resource_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host_resource_group_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `iam_instance_profile` after provisioning.\n"]
    pub fn iam_instance_profile(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_instance_profile", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_initiated_shutdown_behavior` after provisioning.\n"]
    pub fn instance_initiated_shutdown_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.instance_initiated_shutdown_behavior",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `instance_lifecycle` after provisioning.\n"]
    pub fn instance_lifecycle(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_lifecycle", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_state` after provisioning.\n"]
    pub fn instance_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_state", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ipv6_address_count` after provisioning.\n"]
    pub fn ipv6_address_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_address_count", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ipv6_addresses` after provisioning.\n"]
    pub fn ipv6_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ipv6_addresses", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monitoring", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.outpost_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `password_data` after provisioning.\n"]
    pub fn password_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password_data", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `placement_group` after provisioning.\n"]
    pub fn placement_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.placement_group", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `placement_group_id` after provisioning.\n"]
    pub fn placement_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.placement_group_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `placement_partition_number` after provisioning.\n"]
    pub fn placement_partition_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.placement_partition_number", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `primary_network_interface_id` after provisioning.\n"]
    pub fn primary_network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.primary_network_interface_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `private_dns` after provisioning.\n"]
    pub fn private_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_dns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `private_ip` after provisioning.\n"]
    pub fn private_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_ip", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `public_dns` after provisioning.\n"]
    pub fn public_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_dns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `public_ip` after provisioning.\n"]
    pub fn public_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_ip", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `secondary_private_ips` after provisioning.\n"]
    pub fn secondary_private_ips(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.secondary_private_ips", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_groups", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `source_dest_check` after provisioning.\n"]
    pub fn source_dest_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_dest_check", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `spot_instance_request_id` after provisioning.\n"]
    pub fn spot_instance_request_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.spot_instance_request_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnet_id", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `tenancy` after provisioning.\n"]
    pub fn tenancy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tenancy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_data` after provisioning.\n"]
    pub fn user_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_data", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_data_base64` after provisioning.\n"]
    pub fn user_data_base64(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_data_base64", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_data_replace_on_change` after provisioning.\n"]
    pub fn user_data_replace_on_change(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_data_replace_on_change", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `volume_tags` after provisioning.\n"]
    pub fn volume_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.volume_tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.vpc_security_group_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_specification` after provisioning.\n"]
    pub fn capacity_reservation_specification(
        &self,
    ) -> ListRef<InstanceCapacityReservationSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_specification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cpu_options` after provisioning.\n"]
    pub fn cpu_options(&self) -> ListRef<InstanceCpuOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cpu_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `credit_specification` after provisioning.\n"]
    pub fn credit_specification(&self) -> ListRef<InstanceCreditSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.credit_specification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enclave_options` after provisioning.\n"]
    pub fn enclave_options(&self) -> ListRef<InstanceEnclaveOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.enclave_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_market_options` after provisioning.\n"]
    pub fn instance_market_options(&self) -> ListRef<InstanceInstanceMarketOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_market_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> ListRef<InstanceLaunchTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.launch_template", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `maintenance_options` after provisioning.\n"]
    pub fn maintenance_options(&self) -> ListRef<InstanceMaintenanceOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `metadata_options` after provisioning.\n"]
    pub fn metadata_options(&self) -> ListRef<InstanceMetadataOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `primary_network_interface` after provisioning.\n"]
    pub fn primary_network_interface(&self) -> ListRef<InstancePrimaryNetworkInterfaceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.primary_network_interface", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `private_dns_name_options` after provisioning.\n"]
    pub fn private_dns_name_options(&self) -> ListRef<InstancePrivateDnsNameOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_dns_name_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `root_block_device` after provisioning.\n"]
    pub fn root_block_device(&self) -> ListRef<InstanceRootBlockDeviceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.root_block_device", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> InstanceTimeoutsElRef {
        InstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for Instance {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for Instance {}

impl ToListMappable for Instance {
    type O = ListRef<InstanceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Instance_ {
    fn extract_resource_type(&self) -> String {
        "aws_instance".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildInstance {
    pub tf_id: String,
}

impl BuildInstance {
    pub fn build(self, stack: &mut Stack) -> Instance {
        let out = Instance(Rc::new(Instance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(InstanceData {
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
                instance_type: core::default::Default::default(),
                ipv6_address_count: core::default::Default::default(),
                ipv6_addresses: core::default::Default::default(),
                key_name: core::default::Default::default(),
                monitoring: core::default::Default::default(),
                placement_group: core::default::Default::default(),
                placement_group_id: core::default::Default::default(),
                placement_partition_number: core::default::Default::default(),
                private_ip: core::default::Default::default(),
                region: core::default::Default::default(),
                secondary_private_ips: core::default::Default::default(),
                security_groups: core::default::Default::default(),
                source_dest_check: core::default::Default::default(),
                subnet_id: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                tenancy: core::default::Default::default(),
                user_data: core::default::Default::default(),
                user_data_base64: core::default::Default::default(),
                user_data_replace_on_change: core::default::Default::default(),
                volume_tags: core::default::Default::default(),
                vpc_security_group_ids: core::default::Default::default(),
                capacity_reservation_specification: core::default::Default::default(),
                cpu_options: core::default::Default::default(),
                credit_specification: core::default::Default::default(),
                ebs_block_device: core::default::Default::default(),
                enclave_options: core::default::Default::default(),
                ephemeral_block_device: core::default::Default::default(),
                instance_market_options: core::default::Default::default(),
                launch_template: core::default::Default::default(),
                maintenance_options: core::default::Default::default(),
                metadata_options: core::default::Default::default(),
                network_interface: core::default::Default::default(),
                primary_network_interface: core::default::Default::default(),
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

pub struct InstanceRef {
    shared: StackShared,
    base: String,
}

impl Ref for InstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl InstanceRef {
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
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.associate_public_ip_address", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `disable_api_stop` after provisioning.\n"]
    pub fn disable_api_stop(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_api_stop", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `disable_api_termination` after provisioning.\n"]
    pub fn disable_api_termination(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_api_termination", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ebs_optimized` after provisioning.\n"]
    pub fn ebs_optimized(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ebs_optimized", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_primary_ipv6` after provisioning.\n"]
    pub fn enable_primary_ipv6(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_primary_ipv6", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_destroy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `get_password_data` after provisioning.\n"]
    pub fn get_password_data(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.get_password_data", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `hibernation` after provisioning.\n"]
    pub fn hibernation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hibernation", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `host_id` after provisioning.\n"]
    pub fn host_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `host_resource_group_arn` after provisioning.\n"]
    pub fn host_resource_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host_resource_group_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `iam_instance_profile` after provisioning.\n"]
    pub fn iam_instance_profile(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_instance_profile", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_initiated_shutdown_behavior` after provisioning.\n"]
    pub fn instance_initiated_shutdown_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.instance_initiated_shutdown_behavior",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `instance_lifecycle` after provisioning.\n"]
    pub fn instance_lifecycle(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_lifecycle", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_state` after provisioning.\n"]
    pub fn instance_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_state", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ipv6_address_count` after provisioning.\n"]
    pub fn ipv6_address_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_address_count", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ipv6_addresses` after provisioning.\n"]
    pub fn ipv6_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ipv6_addresses", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monitoring", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.outpost_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `password_data` after provisioning.\n"]
    pub fn password_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password_data", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `placement_group` after provisioning.\n"]
    pub fn placement_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.placement_group", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `placement_group_id` after provisioning.\n"]
    pub fn placement_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.placement_group_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `placement_partition_number` after provisioning.\n"]
    pub fn placement_partition_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.placement_partition_number", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `primary_network_interface_id` after provisioning.\n"]
    pub fn primary_network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.primary_network_interface_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `private_dns` after provisioning.\n"]
    pub fn private_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_dns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `private_ip` after provisioning.\n"]
    pub fn private_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_ip", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `public_dns` after provisioning.\n"]
    pub fn public_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_dns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `public_ip` after provisioning.\n"]
    pub fn public_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_ip", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `secondary_private_ips` after provisioning.\n"]
    pub fn secondary_private_ips(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.secondary_private_ips", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_groups", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `source_dest_check` after provisioning.\n"]
    pub fn source_dest_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_dest_check", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `spot_instance_request_id` after provisioning.\n"]
    pub fn spot_instance_request_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.spot_instance_request_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnet_id", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `tenancy` after provisioning.\n"]
    pub fn tenancy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tenancy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_data` after provisioning.\n"]
    pub fn user_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_data", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_data_base64` after provisioning.\n"]
    pub fn user_data_base64(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_data_base64", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_data_replace_on_change` after provisioning.\n"]
    pub fn user_data_replace_on_change(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_data_replace_on_change", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `volume_tags` after provisioning.\n"]
    pub fn volume_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.volume_tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.vpc_security_group_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_specification` after provisioning.\n"]
    pub fn capacity_reservation_specification(
        &self,
    ) -> ListRef<InstanceCapacityReservationSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_specification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cpu_options` after provisioning.\n"]
    pub fn cpu_options(&self) -> ListRef<InstanceCpuOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cpu_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `credit_specification` after provisioning.\n"]
    pub fn credit_specification(&self) -> ListRef<InstanceCreditSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.credit_specification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enclave_options` after provisioning.\n"]
    pub fn enclave_options(&self) -> ListRef<InstanceEnclaveOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.enclave_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_market_options` after provisioning.\n"]
    pub fn instance_market_options(&self) -> ListRef<InstanceInstanceMarketOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_market_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> ListRef<InstanceLaunchTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.launch_template", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `maintenance_options` after provisioning.\n"]
    pub fn maintenance_options(&self) -> ListRef<InstanceMaintenanceOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `metadata_options` after provisioning.\n"]
    pub fn metadata_options(&self) -> ListRef<InstanceMetadataOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `primary_network_interface` after provisioning.\n"]
    pub fn primary_network_interface(&self) -> ListRef<InstancePrimaryNetworkInterfaceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.primary_network_interface", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `private_dns_name_options` after provisioning.\n"]
    pub fn private_dns_name_options(&self) -> ListRef<InstancePrivateDnsNameOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_dns_name_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `root_block_device` after provisioning.\n"]
    pub fn root_block_device(&self) -> ListRef<InstanceRootBlockDeviceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.root_block_device", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> InstanceTimeoutsElRef {
        InstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct InstanceCapacityReservationSpecificationElCapacityReservationTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_reservation_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_reservation_resource_group_arn: Option<PrimField<String>>,
}

impl InstanceCapacityReservationSpecificationElCapacityReservationTargetEl {
    #[doc = "Set the field `capacity_reservation_id`.\n"]
    pub fn set_capacity_reservation_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.capacity_reservation_id = Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_reservation_resource_group_arn`.\n"]
    pub fn set_capacity_reservation_resource_group_arn(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.capacity_reservation_resource_group_arn = Some(v.into());
        self
    }
}

impl ToListMappable for InstanceCapacityReservationSpecificationElCapacityReservationTargetEl {
    type O = BlockAssignable<InstanceCapacityReservationSpecificationElCapacityReservationTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInstanceCapacityReservationSpecificationElCapacityReservationTargetEl {}

impl BuildInstanceCapacityReservationSpecificationElCapacityReservationTargetEl {
    pub fn build(self) -> InstanceCapacityReservationSpecificationElCapacityReservationTargetEl {
        InstanceCapacityReservationSpecificationElCapacityReservationTargetEl {
            capacity_reservation_id: core::default::Default::default(),
            capacity_reservation_resource_group_arn: core::default::Default::default(),
        }
    }
}

pub struct InstanceCapacityReservationSpecificationElCapacityReservationTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for InstanceCapacityReservationSpecificationElCapacityReservationTargetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> InstanceCapacityReservationSpecificationElCapacityReservationTargetElRef {
        InstanceCapacityReservationSpecificationElCapacityReservationTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl InstanceCapacityReservationSpecificationElCapacityReservationTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_id` after provisioning.\n"]
    pub fn capacity_reservation_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_resource_group_arn` after provisioning.\n"]
    pub fn capacity_reservation_resource_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_resource_group_arn", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct InstanceCapacityReservationSpecificationElDynamic {
    capacity_reservation_target:
        Option<DynamicBlock<InstanceCapacityReservationSpecificationElCapacityReservationTargetEl>>,
}

#[derive(Serialize)]
pub struct InstanceCapacityReservationSpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_reservation_preference: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_reservation_target:
        Option<Vec<InstanceCapacityReservationSpecificationElCapacityReservationTargetEl>>,
    dynamic: InstanceCapacityReservationSpecificationElDynamic,
}

impl InstanceCapacityReservationSpecificationEl {
    #[doc = "Set the field `capacity_reservation_preference`.\n"]
    pub fn set_capacity_reservation_preference(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.capacity_reservation_preference = Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_reservation_target`.\n"]
    pub fn set_capacity_reservation_target(
        mut self,
        v: impl Into<
            BlockAssignable<InstanceCapacityReservationSpecificationElCapacityReservationTargetEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.capacity_reservation_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.capacity_reservation_target = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for InstanceCapacityReservationSpecificationEl {
    type O = BlockAssignable<InstanceCapacityReservationSpecificationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInstanceCapacityReservationSpecificationEl {}

impl BuildInstanceCapacityReservationSpecificationEl {
    pub fn build(self) -> InstanceCapacityReservationSpecificationEl {
        InstanceCapacityReservationSpecificationEl {
            capacity_reservation_preference: core::default::Default::default(),
            capacity_reservation_target: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct InstanceCapacityReservationSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for InstanceCapacityReservationSpecificationElRef {
    fn new(shared: StackShared, base: String) -> InstanceCapacityReservationSpecificationElRef {
        InstanceCapacityReservationSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl InstanceCapacityReservationSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_preference` after provisioning.\n"]
    pub fn capacity_reservation_preference(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_preference", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_target` after provisioning.\n"]
    pub fn capacity_reservation_target(
        &self,
    ) -> ListRef<InstanceCapacityReservationSpecificationElCapacityReservationTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_target", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct InstanceCpuOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    amd_sev_snp: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    core_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threads_per_core: Option<PrimField<f64>>,
}

impl InstanceCpuOptionsEl {
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

impl ToListMappable for InstanceCpuOptionsEl {
    type O = BlockAssignable<InstanceCpuOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInstanceCpuOptionsEl {}

impl BuildInstanceCpuOptionsEl {
    pub fn build(self) -> InstanceCpuOptionsEl {
        InstanceCpuOptionsEl {
            amd_sev_snp: core::default::Default::default(),
            core_count: core::default::Default::default(),
            threads_per_core: core::default::Default::default(),
        }
    }
}

pub struct InstanceCpuOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for InstanceCpuOptionsElRef {
    fn new(shared: StackShared, base: String) -> InstanceCpuOptionsElRef {
        InstanceCpuOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl InstanceCpuOptionsElRef {
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
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.threads_per_core", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct InstanceCreditSpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_credits: Option<PrimField<String>>,
}

impl InstanceCreditSpecificationEl {
    #[doc = "Set the field `cpu_credits`.\n"]
    pub fn set_cpu_credits(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cpu_credits = Some(v.into());
        self
    }
}

impl ToListMappable for InstanceCreditSpecificationEl {
    type O = BlockAssignable<InstanceCreditSpecificationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInstanceCreditSpecificationEl {}

impl BuildInstanceCreditSpecificationEl {
    pub fn build(self) -> InstanceCreditSpecificationEl {
        InstanceCreditSpecificationEl {
            cpu_credits: core::default::Default::default(),
        }
    }
}

pub struct InstanceCreditSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for InstanceCreditSpecificationElRef {
    fn new(shared: StackShared, base: String) -> InstanceCreditSpecificationElRef {
        InstanceCreditSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl InstanceCreditSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu_credits` after provisioning.\n"]
    pub fn cpu_credits(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_credits", self.base))
    }
}

#[derive(Serialize)]
pub struct InstanceEbsBlockDeviceEl {
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

impl InstanceEbsBlockDeviceEl {
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

impl ToListMappable for InstanceEbsBlockDeviceEl {
    type O = BlockAssignable<InstanceEbsBlockDeviceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInstanceEbsBlockDeviceEl {
    #[doc = ""]
    pub device_name: PrimField<String>,
}

impl BuildInstanceEbsBlockDeviceEl {
    pub fn build(self) -> InstanceEbsBlockDeviceEl {
        InstanceEbsBlockDeviceEl {
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

pub struct InstanceEbsBlockDeviceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for InstanceEbsBlockDeviceElRef {
    fn new(shared: StackShared, base: String) -> InstanceEbsBlockDeviceElRef {
        InstanceEbsBlockDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl InstanceEbsBlockDeviceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_on_termination", self.base),
        )
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
pub struct InstanceEnclaveOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl InstanceEnclaveOptionsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for InstanceEnclaveOptionsEl {
    type O = BlockAssignable<InstanceEnclaveOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInstanceEnclaveOptionsEl {}

impl BuildInstanceEnclaveOptionsEl {
    pub fn build(self) -> InstanceEnclaveOptionsEl {
        InstanceEnclaveOptionsEl {
            enabled: core::default::Default::default(),
        }
    }
}

pub struct InstanceEnclaveOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for InstanceEnclaveOptionsElRef {
    fn new(shared: StackShared, base: String) -> InstanceEnclaveOptionsElRef {
        InstanceEnclaveOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl InstanceEnclaveOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct InstanceEphemeralBlockDeviceEl {
    device_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_device: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_name: Option<PrimField<String>>,
}

impl InstanceEphemeralBlockDeviceEl {
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

impl ToListMappable for InstanceEphemeralBlockDeviceEl {
    type O = BlockAssignable<InstanceEphemeralBlockDeviceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInstanceEphemeralBlockDeviceEl {
    #[doc = ""]
    pub device_name: PrimField<String>,
}

impl BuildInstanceEphemeralBlockDeviceEl {
    pub fn build(self) -> InstanceEphemeralBlockDeviceEl {
        InstanceEphemeralBlockDeviceEl {
            device_name: self.device_name,
            no_device: core::default::Default::default(),
            virtual_name: core::default::Default::default(),
        }
    }
}

pub struct InstanceEphemeralBlockDeviceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for InstanceEphemeralBlockDeviceElRef {
    fn new(shared: StackShared, base: String) -> InstanceEphemeralBlockDeviceElRef {
        InstanceEphemeralBlockDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl InstanceEphemeralBlockDeviceElRef {
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
pub struct InstanceInstanceMarketOptionsElSpotOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_interruption_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_price: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot_instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_until: Option<PrimField<String>>,
}

impl InstanceInstanceMarketOptionsElSpotOptionsEl {
    #[doc = "Set the field `instance_interruption_behavior`.\n"]
    pub fn set_instance_interruption_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_interruption_behavior = Some(v.into());
        self
    }

    #[doc = "Set the field `max_price`.\n"]
    pub fn set_max_price(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_price = Some(v.into());
        self
    }

    #[doc = "Set the field `spot_instance_type`.\n"]
    pub fn set_spot_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.spot_instance_type = Some(v.into());
        self
    }

    #[doc = "Set the field `valid_until`.\n"]
    pub fn set_valid_until(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.valid_until = Some(v.into());
        self
    }
}

impl ToListMappable for InstanceInstanceMarketOptionsElSpotOptionsEl {
    type O = BlockAssignable<InstanceInstanceMarketOptionsElSpotOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInstanceInstanceMarketOptionsElSpotOptionsEl {}

impl BuildInstanceInstanceMarketOptionsElSpotOptionsEl {
    pub fn build(self) -> InstanceInstanceMarketOptionsElSpotOptionsEl {
        InstanceInstanceMarketOptionsElSpotOptionsEl {
            instance_interruption_behavior: core::default::Default::default(),
            max_price: core::default::Default::default(),
            spot_instance_type: core::default::Default::default(),
            valid_until: core::default::Default::default(),
        }
    }
}

pub struct InstanceInstanceMarketOptionsElSpotOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for InstanceInstanceMarketOptionsElSpotOptionsElRef {
    fn new(shared: StackShared, base: String) -> InstanceInstanceMarketOptionsElSpotOptionsElRef {
        InstanceInstanceMarketOptionsElSpotOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl InstanceInstanceMarketOptionsElSpotOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_interruption_behavior` after provisioning.\n"]
    pub fn instance_interruption_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_interruption_behavior", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `max_price` after provisioning.\n"]
    pub fn max_price(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_price", self.base))
    }

    #[doc = "Get a reference to the value of field `spot_instance_type` after provisioning.\n"]
    pub fn spot_instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.spot_instance_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `valid_until` after provisioning.\n"]
    pub fn valid_until(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.valid_until", self.base))
    }
}

#[derive(Serialize, Default)]
struct InstanceInstanceMarketOptionsElDynamic {
    spot_options: Option<DynamicBlock<InstanceInstanceMarketOptionsElSpotOptionsEl>>,
}

#[derive(Serialize)]
pub struct InstanceInstanceMarketOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    market_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot_options: Option<Vec<InstanceInstanceMarketOptionsElSpotOptionsEl>>,
    dynamic: InstanceInstanceMarketOptionsElDynamic,
}

impl InstanceInstanceMarketOptionsEl {
    #[doc = "Set the field `market_type`.\n"]
    pub fn set_market_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.market_type = Some(v.into());
        self
    }

    #[doc = "Set the field `spot_options`.\n"]
    pub fn set_spot_options(
        mut self,
        v: impl Into<BlockAssignable<InstanceInstanceMarketOptionsElSpotOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.spot_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.spot_options = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for InstanceInstanceMarketOptionsEl {
    type O = BlockAssignable<InstanceInstanceMarketOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInstanceInstanceMarketOptionsEl {}

impl BuildInstanceInstanceMarketOptionsEl {
    pub fn build(self) -> InstanceInstanceMarketOptionsEl {
        InstanceInstanceMarketOptionsEl {
            market_type: core::default::Default::default(),
            spot_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct InstanceInstanceMarketOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for InstanceInstanceMarketOptionsElRef {
    fn new(shared: StackShared, base: String) -> InstanceInstanceMarketOptionsElRef {
        InstanceInstanceMarketOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl InstanceInstanceMarketOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `market_type` after provisioning.\n"]
    pub fn market_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.market_type", self.base))
    }

    #[doc = "Get a reference to the value of field `spot_options` after provisioning.\n"]
    pub fn spot_options(&self) -> ListRef<InstanceInstanceMarketOptionsElSpotOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spot_options", self.base))
    }
}

#[derive(Serialize)]
pub struct InstanceLaunchTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl InstanceLaunchTemplateEl {
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

impl ToListMappable for InstanceLaunchTemplateEl {
    type O = BlockAssignable<InstanceLaunchTemplateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInstanceLaunchTemplateEl {}

impl BuildInstanceLaunchTemplateEl {
    pub fn build(self) -> InstanceLaunchTemplateEl {
        InstanceLaunchTemplateEl {
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct InstanceLaunchTemplateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for InstanceLaunchTemplateElRef {
    fn new(shared: StackShared, base: String) -> InstanceLaunchTemplateElRef {
        InstanceLaunchTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl InstanceLaunchTemplateElRef {
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
pub struct InstanceMaintenanceOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_recovery: Option<PrimField<String>>,
}

impl InstanceMaintenanceOptionsEl {
    #[doc = "Set the field `auto_recovery`.\n"]
    pub fn set_auto_recovery(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auto_recovery = Some(v.into());
        self
    }
}

impl ToListMappable for InstanceMaintenanceOptionsEl {
    type O = BlockAssignable<InstanceMaintenanceOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInstanceMaintenanceOptionsEl {}

impl BuildInstanceMaintenanceOptionsEl {
    pub fn build(self) -> InstanceMaintenanceOptionsEl {
        InstanceMaintenanceOptionsEl {
            auto_recovery: core::default::Default::default(),
        }
    }
}

pub struct InstanceMaintenanceOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for InstanceMaintenanceOptionsElRef {
    fn new(shared: StackShared, base: String) -> InstanceMaintenanceOptionsElRef {
        InstanceMaintenanceOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl InstanceMaintenanceOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_recovery` after provisioning.\n"]
    pub fn auto_recovery(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_recovery", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct InstanceMetadataOptionsEl {
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

impl InstanceMetadataOptionsEl {
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

impl ToListMappable for InstanceMetadataOptionsEl {
    type O = BlockAssignable<InstanceMetadataOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInstanceMetadataOptionsEl {}

impl BuildInstanceMetadataOptionsEl {
    pub fn build(self) -> InstanceMetadataOptionsEl {
        InstanceMetadataOptionsEl {
            http_endpoint: core::default::Default::default(),
            http_protocol_ipv6: core::default::Default::default(),
            http_put_response_hop_limit: core::default::Default::default(),
            http_tokens: core::default::Default::default(),
            instance_metadata_tags: core::default::Default::default(),
        }
    }
}

pub struct InstanceMetadataOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for InstanceMetadataOptionsElRef {
    fn new(shared: StackShared, base: String) -> InstanceMetadataOptionsElRef {
        InstanceMetadataOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl InstanceMetadataOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `http_endpoint` after provisioning.\n"]
    pub fn http_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_endpoint", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `http_protocol_ipv6` after provisioning.\n"]
    pub fn http_protocol_ipv6(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_protocol_ipv6", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `http_put_response_hop_limit` after provisioning.\n"]
    pub fn http_put_response_hop_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_put_response_hop_limit", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `http_tokens` after provisioning.\n"]
    pub fn http_tokens(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_tokens", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_metadata_tags` after provisioning.\n"]
    pub fn instance_metadata_tags(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_metadata_tags", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct InstanceNetworkInterfaceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_on_termination: Option<PrimField<bool>>,
    device_index: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_card_index: Option<PrimField<f64>>,
    network_interface_id: PrimField<String>,
}

impl InstanceNetworkInterfaceEl {
    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.delete_on_termination = Some(v.into());
        self
    }

    #[doc = "Set the field `network_card_index`.\n"]
    pub fn set_network_card_index(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.network_card_index = Some(v.into());
        self
    }
}

impl ToListMappable for InstanceNetworkInterfaceEl {
    type O = BlockAssignable<InstanceNetworkInterfaceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInstanceNetworkInterfaceEl {
    #[doc = ""]
    pub device_index: PrimField<f64>,
    #[doc = ""]
    pub network_interface_id: PrimField<String>,
}

impl BuildInstanceNetworkInterfaceEl {
    pub fn build(self) -> InstanceNetworkInterfaceEl {
        InstanceNetworkInterfaceEl {
            delete_on_termination: core::default::Default::default(),
            device_index: self.device_index,
            network_card_index: core::default::Default::default(),
            network_interface_id: self.network_interface_id,
        }
    }
}

pub struct InstanceNetworkInterfaceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for InstanceNetworkInterfaceElRef {
    fn new(shared: StackShared, base: String) -> InstanceNetworkInterfaceElRef {
        InstanceNetworkInterfaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl InstanceNetworkInterfaceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_on_termination", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `device_index` after provisioning.\n"]
    pub fn device_index(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_index", self.base))
    }

    #[doc = "Get a reference to the value of field `network_card_index` after provisioning.\n"]
    pub fn network_card_index(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_card_index", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_interface_id", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct InstancePrimaryNetworkInterfaceEl {
    network_interface_id: PrimField<String>,
}

impl InstancePrimaryNetworkInterfaceEl {}

impl ToListMappable for InstancePrimaryNetworkInterfaceEl {
    type O = BlockAssignable<InstancePrimaryNetworkInterfaceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInstancePrimaryNetworkInterfaceEl {
    #[doc = ""]
    pub network_interface_id: PrimField<String>,
}

impl BuildInstancePrimaryNetworkInterfaceEl {
    pub fn build(self) -> InstancePrimaryNetworkInterfaceEl {
        InstancePrimaryNetworkInterfaceEl {
            network_interface_id: self.network_interface_id,
        }
    }
}

pub struct InstancePrimaryNetworkInterfaceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for InstancePrimaryNetworkInterfaceElRef {
    fn new(shared: StackShared, base: String) -> InstancePrimaryNetworkInterfaceElRef {
        InstancePrimaryNetworkInterfaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl InstancePrimaryNetworkInterfaceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_on_termination", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_interface_id", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct InstancePrivateDnsNameOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_resource_name_dns_a_record: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_resource_name_dns_aaaa_record: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname_type: Option<PrimField<String>>,
}

impl InstancePrivateDnsNameOptionsEl {
    #[doc = "Set the field `enable_resource_name_dns_a_record`.\n"]
    pub fn set_enable_resource_name_dns_a_record(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_resource_name_dns_a_record = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_resource_name_dns_aaaa_record`.\n"]
    pub fn set_enable_resource_name_dns_aaaa_record(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.enable_resource_name_dns_aaaa_record = Some(v.into());
        self
    }

    #[doc = "Set the field `hostname_type`.\n"]
    pub fn set_hostname_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hostname_type = Some(v.into());
        self
    }
}

impl ToListMappable for InstancePrivateDnsNameOptionsEl {
    type O = BlockAssignable<InstancePrivateDnsNameOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInstancePrivateDnsNameOptionsEl {}

impl BuildInstancePrivateDnsNameOptionsEl {
    pub fn build(self) -> InstancePrivateDnsNameOptionsEl {
        InstancePrivateDnsNameOptionsEl {
            enable_resource_name_dns_a_record: core::default::Default::default(),
            enable_resource_name_dns_aaaa_record: core::default::Default::default(),
            hostname_type: core::default::Default::default(),
        }
    }
}

pub struct InstancePrivateDnsNameOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for InstancePrivateDnsNameOptionsElRef {
    fn new(shared: StackShared, base: String) -> InstancePrivateDnsNameOptionsElRef {
        InstancePrivateDnsNameOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl InstancePrivateDnsNameOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_resource_name_dns_a_record` after provisioning.\n"]
    pub fn enable_resource_name_dns_a_record(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_resource_name_dns_a_record", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `enable_resource_name_dns_aaaa_record` after provisioning.\n"]
    pub fn enable_resource_name_dns_aaaa_record(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_resource_name_dns_aaaa_record", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `hostname_type` after provisioning.\n"]
    pub fn hostname_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hostname_type", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct InstanceRootBlockDeviceEl {
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

impl InstanceRootBlockDeviceEl {
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

impl ToListMappable for InstanceRootBlockDeviceEl {
    type O = BlockAssignable<InstanceRootBlockDeviceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInstanceRootBlockDeviceEl {}

impl BuildInstanceRootBlockDeviceEl {
    pub fn build(self) -> InstanceRootBlockDeviceEl {
        InstanceRootBlockDeviceEl {
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

pub struct InstanceRootBlockDeviceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for InstanceRootBlockDeviceElRef {
    fn new(shared: StackShared, base: String) -> InstanceRootBlockDeviceElRef {
        InstanceRootBlockDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl InstanceRootBlockDeviceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_on_termination", self.base),
        )
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
pub struct InstanceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl InstanceTimeoutsEl {
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

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for InstanceTimeoutsEl {
    type O = BlockAssignable<InstanceTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInstanceTimeoutsEl {}

impl BuildInstanceTimeoutsEl {
    pub fn build(self) -> InstanceTimeoutsEl {
        InstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            read: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct InstanceTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for InstanceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> InstanceTimeoutsElRef {
        InstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl InstanceTimeoutsElRef {
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

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct InstanceDynamic {
    capacity_reservation_specification:
        Option<DynamicBlock<InstanceCapacityReservationSpecificationEl>>,
    cpu_options: Option<DynamicBlock<InstanceCpuOptionsEl>>,
    credit_specification: Option<DynamicBlock<InstanceCreditSpecificationEl>>,
    ebs_block_device: Option<DynamicBlock<InstanceEbsBlockDeviceEl>>,
    enclave_options: Option<DynamicBlock<InstanceEnclaveOptionsEl>>,
    ephemeral_block_device: Option<DynamicBlock<InstanceEphemeralBlockDeviceEl>>,
    instance_market_options: Option<DynamicBlock<InstanceInstanceMarketOptionsEl>>,
    launch_template: Option<DynamicBlock<InstanceLaunchTemplateEl>>,
    maintenance_options: Option<DynamicBlock<InstanceMaintenanceOptionsEl>>,
    metadata_options: Option<DynamicBlock<InstanceMetadataOptionsEl>>,
    network_interface: Option<DynamicBlock<InstanceNetworkInterfaceEl>>,
    primary_network_interface: Option<DynamicBlock<InstancePrimaryNetworkInterfaceEl>>,
    private_dns_name_options: Option<DynamicBlock<InstancePrivateDnsNameOptionsEl>>,
    root_block_device: Option<DynamicBlock<InstanceRootBlockDeviceEl>>,
}
