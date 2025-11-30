use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct LaunchTemplateData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_version: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_api_stop: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_api_termination: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs_optimized: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_initiated_shutdown_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kernel_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ram_disk_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_names: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_default_version: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_data: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_device_mappings: Option<Vec<LaunchTemplateBlockDeviceMappingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_reservation_specification:
        Option<Vec<LaunchTemplateCapacityReservationSpecificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_options: Option<Vec<LaunchTemplateCpuOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credit_specification: Option<Vec<LaunchTemplateCreditSpecificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enclave_options: Option<Vec<LaunchTemplateEnclaveOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hibernation_options: Option<Vec<LaunchTemplateHibernationOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_instance_profile: Option<Vec<LaunchTemplateIamInstanceProfileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_market_options: Option<Vec<LaunchTemplateInstanceMarketOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_requirements: Option<Vec<LaunchTemplateInstanceRequirementsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    license_specification: Option<Vec<LaunchTemplateLicenseSpecificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_options: Option<Vec<LaunchTemplateMaintenanceOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_options: Option<Vec<LaunchTemplateMetadataOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitoring: Option<Vec<LaunchTemplateMonitoringEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interfaces: Option<Vec<LaunchTemplateNetworkInterfacesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement: Option<Vec<LaunchTemplatePlacementEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_dns_name_options: Option<Vec<LaunchTemplatePrivateDnsNameOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_specifications: Option<Vec<LaunchTemplateTagSpecificationsEl>>,
    dynamic: LaunchTemplateDynamic,
}

struct LaunchTemplate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LaunchTemplateData>,
}

#[derive(Clone)]
pub struct LaunchTemplate(Rc<LaunchTemplate_>);

impl LaunchTemplate {
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

    #[doc = "Set the field `default_version`.\n"]
    pub fn set_default_version(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().default_version = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
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
    pub fn set_ebs_optimized(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ebs_optimized = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `image_id`.\n"]
    pub fn set_image_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().image_id = Some(v.into());
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

    #[doc = "Set the field `kernel_id`.\n"]
    pub fn set_kernel_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kernel_id = Some(v.into());
        self
    }

    #[doc = "Set the field `key_name`.\n"]
    pub fn set_key_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().key_name = Some(v.into());
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

    #[doc = "Set the field `ram_disk_id`.\n"]
    pub fn set_ram_disk_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ram_disk_id = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_names`.\n"]
    pub fn set_security_group_names(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().security_group_names = Some(v.into());
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

    #[doc = "Set the field `update_default_version`.\n"]
    pub fn set_update_default_version(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().update_default_version = Some(v.into());
        self
    }

    #[doc = "Set the field `user_data`.\n"]
    pub fn set_user_data(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().user_data = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_security_group_ids`.\n"]
    pub fn set_vpc_security_group_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().vpc_security_group_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `block_device_mappings`.\n"]
    pub fn set_block_device_mappings(
        self,
        v: impl Into<BlockAssignable<LaunchTemplateBlockDeviceMappingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().block_device_mappings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.block_device_mappings = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `capacity_reservation_specification`.\n"]
    pub fn set_capacity_reservation_specification(
        self,
        v: impl Into<BlockAssignable<LaunchTemplateCapacityReservationSpecificationEl>>,
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
    pub fn set_cpu_options(
        self,
        v: impl Into<BlockAssignable<LaunchTemplateCpuOptionsEl>>,
    ) -> Self {
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
        v: impl Into<BlockAssignable<LaunchTemplateCreditSpecificationEl>>,
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

    #[doc = "Set the field `enclave_options`.\n"]
    pub fn set_enclave_options(
        self,
        v: impl Into<BlockAssignable<LaunchTemplateEnclaveOptionsEl>>,
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

    #[doc = "Set the field `hibernation_options`.\n"]
    pub fn set_hibernation_options(
        self,
        v: impl Into<BlockAssignable<LaunchTemplateHibernationOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().hibernation_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.hibernation_options = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `iam_instance_profile`.\n"]
    pub fn set_iam_instance_profile(
        self,
        v: impl Into<BlockAssignable<LaunchTemplateIamInstanceProfileEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().iam_instance_profile = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.iam_instance_profile = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `instance_market_options`.\n"]
    pub fn set_instance_market_options(
        self,
        v: impl Into<BlockAssignable<LaunchTemplateInstanceMarketOptionsEl>>,
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

    #[doc = "Set the field `instance_requirements`.\n"]
    pub fn set_instance_requirements(
        self,
        v: impl Into<BlockAssignable<LaunchTemplateInstanceRequirementsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_requirements = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_requirements = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `license_specification`.\n"]
    pub fn set_license_specification(
        self,
        v: impl Into<BlockAssignable<LaunchTemplateLicenseSpecificationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().license_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.license_specification = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `maintenance_options`.\n"]
    pub fn set_maintenance_options(
        self,
        v: impl Into<BlockAssignable<LaunchTemplateMaintenanceOptionsEl>>,
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
        v: impl Into<BlockAssignable<LaunchTemplateMetadataOptionsEl>>,
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

    #[doc = "Set the field `monitoring`.\n"]
    pub fn set_monitoring(self, v: impl Into<BlockAssignable<LaunchTemplateMonitoringEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().monitoring = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.monitoring = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `network_interfaces`.\n"]
    pub fn set_network_interfaces(
        self,
        v: impl Into<BlockAssignable<LaunchTemplateNetworkInterfacesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_interfaces = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_interfaces = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `placement`.\n"]
    pub fn set_placement(self, v: impl Into<BlockAssignable<LaunchTemplatePlacementEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().placement = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.placement = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `private_dns_name_options`.\n"]
    pub fn set_private_dns_name_options(
        self,
        v: impl Into<BlockAssignable<LaunchTemplatePrivateDnsNameOptionsEl>>,
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

    #[doc = "Set the field `tag_specifications`.\n"]
    pub fn set_tag_specifications(
        self,
        v: impl Into<BlockAssignable<LaunchTemplateTagSpecificationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tag_specifications = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tag_specifications = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_version` after provisioning.\n"]
    pub fn default_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
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
    pub fn ebs_optimized(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ebs_optimized", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_id` after provisioning.\n"]
    pub fn image_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_id", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kernel_id` after provisioning.\n"]
    pub fn kernel_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kernel_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `latest_version` after provisioning.\n"]
    pub fn latest_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest_version", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `ram_disk_id` after provisioning.\n"]
    pub fn ram_disk_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ram_disk_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `security_group_names` after provisioning.\n"]
    pub fn security_group_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_names", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `update_default_version` after provisioning.\n"]
    pub fn update_default_version(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_default_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_data` after provisioning.\n"]
    pub fn user_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_data", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.vpc_security_group_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `block_device_mappings` after provisioning.\n"]
    pub fn block_device_mappings(&self) -> ListRef<LaunchTemplateBlockDeviceMappingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.block_device_mappings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_specification` after provisioning.\n"]
    pub fn capacity_reservation_specification(
        &self,
    ) -> ListRef<LaunchTemplateCapacityReservationSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_specification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cpu_options` after provisioning.\n"]
    pub fn cpu_options(&self) -> ListRef<LaunchTemplateCpuOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cpu_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `credit_specification` after provisioning.\n"]
    pub fn credit_specification(&self) -> ListRef<LaunchTemplateCreditSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.credit_specification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enclave_options` after provisioning.\n"]
    pub fn enclave_options(&self) -> ListRef<LaunchTemplateEnclaveOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.enclave_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `hibernation_options` after provisioning.\n"]
    pub fn hibernation_options(&self) -> ListRef<LaunchTemplateHibernationOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hibernation_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `iam_instance_profile` after provisioning.\n"]
    pub fn iam_instance_profile(&self) -> ListRef<LaunchTemplateIamInstanceProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.iam_instance_profile", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_market_options` after provisioning.\n"]
    pub fn instance_market_options(&self) -> ListRef<LaunchTemplateInstanceMarketOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_market_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_requirements` after provisioning.\n"]
    pub fn instance_requirements(&self) -> ListRef<LaunchTemplateInstanceRequirementsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_requirements", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `maintenance_options` after provisioning.\n"]
    pub fn maintenance_options(&self) -> ListRef<LaunchTemplateMaintenanceOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `metadata_options` after provisioning.\n"]
    pub fn metadata_options(&self) -> ListRef<LaunchTemplateMetadataOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> ListRef<LaunchTemplateMonitoringElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.monitoring", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `network_interfaces` after provisioning.\n"]
    pub fn network_interfaces(&self) -> ListRef<LaunchTemplateNetworkInterfacesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_interfaces", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `placement` after provisioning.\n"]
    pub fn placement(&self) -> ListRef<LaunchTemplatePlacementElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.placement", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `private_dns_name_options` after provisioning.\n"]
    pub fn private_dns_name_options(&self) -> ListRef<LaunchTemplatePrivateDnsNameOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_dns_name_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tag_specifications` after provisioning.\n"]
    pub fn tag_specifications(&self) -> ListRef<LaunchTemplateTagSpecificationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tag_specifications", self.extract_ref()),
        )
    }
}

impl Referable for LaunchTemplate {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for LaunchTemplate {}

impl ToListMappable for LaunchTemplate {
    type O = ListRef<LaunchTemplateRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for LaunchTemplate_ {
    fn extract_resource_type(&self) -> String {
        "aws_launch_template".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLaunchTemplate {
    pub tf_id: String,
}

impl BuildLaunchTemplate {
    pub fn build(self, stack: &mut Stack) -> LaunchTemplate {
        let out = LaunchTemplate(Rc::new(LaunchTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LaunchTemplateData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                default_version: core::default::Default::default(),
                description: core::default::Default::default(),
                disable_api_stop: core::default::Default::default(),
                disable_api_termination: core::default::Default::default(),
                ebs_optimized: core::default::Default::default(),
                id: core::default::Default::default(),
                image_id: core::default::Default::default(),
                instance_initiated_shutdown_behavior: core::default::Default::default(),
                instance_type: core::default::Default::default(),
                kernel_id: core::default::Default::default(),
                key_name: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                ram_disk_id: core::default::Default::default(),
                region: core::default::Default::default(),
                security_group_names: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                update_default_version: core::default::Default::default(),
                user_data: core::default::Default::default(),
                vpc_security_group_ids: core::default::Default::default(),
                block_device_mappings: core::default::Default::default(),
                capacity_reservation_specification: core::default::Default::default(),
                cpu_options: core::default::Default::default(),
                credit_specification: core::default::Default::default(),
                enclave_options: core::default::Default::default(),
                hibernation_options: core::default::Default::default(),
                iam_instance_profile: core::default::Default::default(),
                instance_market_options: core::default::Default::default(),
                instance_requirements: core::default::Default::default(),
                license_specification: core::default::Default::default(),
                maintenance_options: core::default::Default::default(),
                metadata_options: core::default::Default::default(),
                monitoring: core::default::Default::default(),
                network_interfaces: core::default::Default::default(),
                placement: core::default::Default::default(),
                private_dns_name_options: core::default::Default::default(),
                tag_specifications: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LaunchTemplateRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl LaunchTemplateRef {
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

    #[doc = "Get a reference to the value of field `default_version` after provisioning.\n"]
    pub fn default_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
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
    pub fn ebs_optimized(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ebs_optimized", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_id` after provisioning.\n"]
    pub fn image_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_id", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kernel_id` after provisioning.\n"]
    pub fn kernel_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kernel_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `latest_version` after provisioning.\n"]
    pub fn latest_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest_version", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `ram_disk_id` after provisioning.\n"]
    pub fn ram_disk_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ram_disk_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `security_group_names` after provisioning.\n"]
    pub fn security_group_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_names", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `update_default_version` after provisioning.\n"]
    pub fn update_default_version(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_default_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_data` after provisioning.\n"]
    pub fn user_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_data", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.vpc_security_group_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `block_device_mappings` after provisioning.\n"]
    pub fn block_device_mappings(&self) -> ListRef<LaunchTemplateBlockDeviceMappingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.block_device_mappings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_specification` after provisioning.\n"]
    pub fn capacity_reservation_specification(
        &self,
    ) -> ListRef<LaunchTemplateCapacityReservationSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_specification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cpu_options` after provisioning.\n"]
    pub fn cpu_options(&self) -> ListRef<LaunchTemplateCpuOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cpu_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `credit_specification` after provisioning.\n"]
    pub fn credit_specification(&self) -> ListRef<LaunchTemplateCreditSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.credit_specification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enclave_options` after provisioning.\n"]
    pub fn enclave_options(&self) -> ListRef<LaunchTemplateEnclaveOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.enclave_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `hibernation_options` after provisioning.\n"]
    pub fn hibernation_options(&self) -> ListRef<LaunchTemplateHibernationOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hibernation_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `iam_instance_profile` after provisioning.\n"]
    pub fn iam_instance_profile(&self) -> ListRef<LaunchTemplateIamInstanceProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.iam_instance_profile", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_market_options` after provisioning.\n"]
    pub fn instance_market_options(&self) -> ListRef<LaunchTemplateInstanceMarketOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_market_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_requirements` after provisioning.\n"]
    pub fn instance_requirements(&self) -> ListRef<LaunchTemplateInstanceRequirementsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_requirements", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `maintenance_options` after provisioning.\n"]
    pub fn maintenance_options(&self) -> ListRef<LaunchTemplateMaintenanceOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `metadata_options` after provisioning.\n"]
    pub fn metadata_options(&self) -> ListRef<LaunchTemplateMetadataOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> ListRef<LaunchTemplateMonitoringElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.monitoring", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `network_interfaces` after provisioning.\n"]
    pub fn network_interfaces(&self) -> ListRef<LaunchTemplateNetworkInterfacesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_interfaces", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `placement` after provisioning.\n"]
    pub fn placement(&self) -> ListRef<LaunchTemplatePlacementElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.placement", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `private_dns_name_options` after provisioning.\n"]
    pub fn private_dns_name_options(&self) -> ListRef<LaunchTemplatePrivateDnsNameOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_dns_name_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tag_specifications` after provisioning.\n"]
    pub fn tag_specifications(&self) -> ListRef<LaunchTemplateTagSpecificationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tag_specifications", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateBlockDeviceMappingsElEbsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_on_termination: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encrypted: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throughput: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_initialization_rate: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_type: Option<PrimField<String>>,
}

impl LaunchTemplateBlockDeviceMappingsElEbsEl {
    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete_on_termination = Some(v.into());
        self
    }

    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(mut self, v: impl Into<PrimField<String>>) -> Self {
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

    #[doc = "Set the field `throughput`.\n"]
    pub fn set_throughput(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.throughput = Some(v.into());
        self
    }

    #[doc = "Set the field `volume_initialization_rate`.\n"]
    pub fn set_volume_initialization_rate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volume_initialization_rate = Some(v.into());
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

impl ToListMappable for LaunchTemplateBlockDeviceMappingsElEbsEl {
    type O = BlockAssignable<LaunchTemplateBlockDeviceMappingsElEbsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateBlockDeviceMappingsElEbsEl {}

impl BuildLaunchTemplateBlockDeviceMappingsElEbsEl {
    pub fn build(self) -> LaunchTemplateBlockDeviceMappingsElEbsEl {
        LaunchTemplateBlockDeviceMappingsElEbsEl {
            delete_on_termination: core::default::Default::default(),
            encrypted: core::default::Default::default(),
            iops: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            snapshot_id: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_initialization_rate: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateBlockDeviceMappingsElEbsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateBlockDeviceMappingsElEbsElRef {
    fn new(shared: StackShared, base: String) -> LaunchTemplateBlockDeviceMappingsElEbsElRef {
        LaunchTemplateBlockDeviceMappingsElEbsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateBlockDeviceMappingsElEbsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_on_termination", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> PrimExpr<String> {
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

    #[doc = "Get a reference to the value of field `throughput` after provisioning.\n"]
    pub fn throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_initialization_rate` after provisioning.\n"]
    pub fn volume_initialization_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_initialization_rate", self.base),
        )
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

#[derive(Serialize, Default)]
struct LaunchTemplateBlockDeviceMappingsElDynamic {
    ebs: Option<DynamicBlock<LaunchTemplateBlockDeviceMappingsElEbsEl>>,
}

#[derive(Serialize)]
pub struct LaunchTemplateBlockDeviceMappingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    device_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_device: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs: Option<Vec<LaunchTemplateBlockDeviceMappingsElEbsEl>>,
    dynamic: LaunchTemplateBlockDeviceMappingsElDynamic,
}

impl LaunchTemplateBlockDeviceMappingsEl {
    #[doc = "Set the field `device_name`.\n"]
    pub fn set_device_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device_name = Some(v.into());
        self
    }

    #[doc = "Set the field `no_device`.\n"]
    pub fn set_no_device(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.no_device = Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_name`.\n"]
    pub fn set_virtual_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.virtual_name = Some(v.into());
        self
    }

    #[doc = "Set the field `ebs`.\n"]
    pub fn set_ebs(
        mut self,
        v: impl Into<BlockAssignable<LaunchTemplateBlockDeviceMappingsElEbsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ebs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ebs = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for LaunchTemplateBlockDeviceMappingsEl {
    type O = BlockAssignable<LaunchTemplateBlockDeviceMappingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateBlockDeviceMappingsEl {}

impl BuildLaunchTemplateBlockDeviceMappingsEl {
    pub fn build(self) -> LaunchTemplateBlockDeviceMappingsEl {
        LaunchTemplateBlockDeviceMappingsEl {
            device_name: core::default::Default::default(),
            no_device: core::default::Default::default(),
            virtual_name: core::default::Default::default(),
            ebs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct LaunchTemplateBlockDeviceMappingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateBlockDeviceMappingsElRef {
    fn new(shared: StackShared, base: String) -> LaunchTemplateBlockDeviceMappingsElRef {
        LaunchTemplateBlockDeviceMappingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateBlockDeviceMappingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc = "Get a reference to the value of field `no_device` after provisioning.\n"]
    pub fn no_device(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.no_device", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_name` after provisioning.\n"]
    pub fn virtual_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtual_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ebs` after provisioning.\n"]
    pub fn ebs(&self) -> ListRef<LaunchTemplateBlockDeviceMappingsElEbsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ebs", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_reservation_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_reservation_resource_group_arn: Option<PrimField<String>>,
}

impl LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl {
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

impl ToListMappable
    for LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl
{
    type O = BlockAssignable<
        LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl {}

impl BuildLaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl {
    pub fn build(
        self,
    ) -> LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl {
        LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl {
            capacity_reservation_id: core::default::Default::default(),
            capacity_reservation_resource_group_arn: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetElRef {
        LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetElRef {
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
struct LaunchTemplateCapacityReservationSpecificationElDynamic {
    capacity_reservation_target: Option<
        DynamicBlock<LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl>,
    >,
}

#[derive(Serialize)]
pub struct LaunchTemplateCapacityReservationSpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_reservation_preference: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_reservation_target:
        Option<Vec<LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl>>,
    dynamic: LaunchTemplateCapacityReservationSpecificationElDynamic,
}

impl LaunchTemplateCapacityReservationSpecificationEl {
    #[doc = "Set the field `capacity_reservation_preference`.\n"]
    pub fn set_capacity_reservation_preference(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.capacity_reservation_preference = Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_reservation_target`.\n"]
    pub fn set_capacity_reservation_target(
        mut self,
        v: impl Into<
            BlockAssignable<
                LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl,
            >,
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

impl ToListMappable for LaunchTemplateCapacityReservationSpecificationEl {
    type O = BlockAssignable<LaunchTemplateCapacityReservationSpecificationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateCapacityReservationSpecificationEl {}

impl BuildLaunchTemplateCapacityReservationSpecificationEl {
    pub fn build(self) -> LaunchTemplateCapacityReservationSpecificationEl {
        LaunchTemplateCapacityReservationSpecificationEl {
            capacity_reservation_preference: core::default::Default::default(),
            capacity_reservation_target: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct LaunchTemplateCapacityReservationSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateCapacityReservationSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LaunchTemplateCapacityReservationSpecificationElRef {
        LaunchTemplateCapacityReservationSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateCapacityReservationSpecificationElRef {
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
    ) -> ListRef<LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_target", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateCpuOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    amd_sev_snp: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    core_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threads_per_core: Option<PrimField<f64>>,
}

impl LaunchTemplateCpuOptionsEl {
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

impl ToListMappable for LaunchTemplateCpuOptionsEl {
    type O = BlockAssignable<LaunchTemplateCpuOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateCpuOptionsEl {}

impl BuildLaunchTemplateCpuOptionsEl {
    pub fn build(self) -> LaunchTemplateCpuOptionsEl {
        LaunchTemplateCpuOptionsEl {
            amd_sev_snp: core::default::Default::default(),
            core_count: core::default::Default::default(),
            threads_per_core: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateCpuOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateCpuOptionsElRef {
    fn new(shared: StackShared, base: String) -> LaunchTemplateCpuOptionsElRef {
        LaunchTemplateCpuOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateCpuOptionsElRef {
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
pub struct LaunchTemplateCreditSpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_credits: Option<PrimField<String>>,
}

impl LaunchTemplateCreditSpecificationEl {
    #[doc = "Set the field `cpu_credits`.\n"]
    pub fn set_cpu_credits(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cpu_credits = Some(v.into());
        self
    }
}

impl ToListMappable for LaunchTemplateCreditSpecificationEl {
    type O = BlockAssignable<LaunchTemplateCreditSpecificationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateCreditSpecificationEl {}

impl BuildLaunchTemplateCreditSpecificationEl {
    pub fn build(self) -> LaunchTemplateCreditSpecificationEl {
        LaunchTemplateCreditSpecificationEl {
            cpu_credits: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateCreditSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateCreditSpecificationElRef {
    fn new(shared: StackShared, base: String) -> LaunchTemplateCreditSpecificationElRef {
        LaunchTemplateCreditSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateCreditSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu_credits` after provisioning.\n"]
    pub fn cpu_credits(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_credits", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateEnclaveOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl LaunchTemplateEnclaveOptionsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for LaunchTemplateEnclaveOptionsEl {
    type O = BlockAssignable<LaunchTemplateEnclaveOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateEnclaveOptionsEl {}

impl BuildLaunchTemplateEnclaveOptionsEl {
    pub fn build(self) -> LaunchTemplateEnclaveOptionsEl {
        LaunchTemplateEnclaveOptionsEl {
            enabled: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateEnclaveOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateEnclaveOptionsElRef {
    fn new(shared: StackShared, base: String) -> LaunchTemplateEnclaveOptionsElRef {
        LaunchTemplateEnclaveOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateEnclaveOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateHibernationOptionsEl {
    configured: PrimField<bool>,
}

impl LaunchTemplateHibernationOptionsEl {}

impl ToListMappable for LaunchTemplateHibernationOptionsEl {
    type O = BlockAssignable<LaunchTemplateHibernationOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateHibernationOptionsEl {
    #[doc = ""]
    pub configured: PrimField<bool>,
}

impl BuildLaunchTemplateHibernationOptionsEl {
    pub fn build(self) -> LaunchTemplateHibernationOptionsEl {
        LaunchTemplateHibernationOptionsEl {
            configured: self.configured,
        }
    }
}

pub struct LaunchTemplateHibernationOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateHibernationOptionsElRef {
    fn new(shared: StackShared, base: String) -> LaunchTemplateHibernationOptionsElRef {
        LaunchTemplateHibernationOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateHibernationOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configured` after provisioning.\n"]
    pub fn configured(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.configured", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateIamInstanceProfileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl LaunchTemplateIamInstanceProfileEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for LaunchTemplateIamInstanceProfileEl {
    type O = BlockAssignable<LaunchTemplateIamInstanceProfileEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateIamInstanceProfileEl {}

impl BuildLaunchTemplateIamInstanceProfileEl {
    pub fn build(self) -> LaunchTemplateIamInstanceProfileEl {
        LaunchTemplateIamInstanceProfileEl {
            arn: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateIamInstanceProfileElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateIamInstanceProfileElRef {
    fn new(shared: StackShared, base: String) -> LaunchTemplateIamInstanceProfileElRef {
        LaunchTemplateIamInstanceProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateIamInstanceProfileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateInstanceMarketOptionsElSpotOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    block_duration_minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_interruption_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_price: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot_instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_until: Option<PrimField<String>>,
}

impl LaunchTemplateInstanceMarketOptionsElSpotOptionsEl {
    #[doc = "Set the field `block_duration_minutes`.\n"]
    pub fn set_block_duration_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.block_duration_minutes = Some(v.into());
        self
    }

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

impl ToListMappable for LaunchTemplateInstanceMarketOptionsElSpotOptionsEl {
    type O = BlockAssignable<LaunchTemplateInstanceMarketOptionsElSpotOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateInstanceMarketOptionsElSpotOptionsEl {}

impl BuildLaunchTemplateInstanceMarketOptionsElSpotOptionsEl {
    pub fn build(self) -> LaunchTemplateInstanceMarketOptionsElSpotOptionsEl {
        LaunchTemplateInstanceMarketOptionsElSpotOptionsEl {
            block_duration_minutes: core::default::Default::default(),
            instance_interruption_behavior: core::default::Default::default(),
            max_price: core::default::Default::default(),
            spot_instance_type: core::default::Default::default(),
            valid_until: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateInstanceMarketOptionsElSpotOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateInstanceMarketOptionsElSpotOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LaunchTemplateInstanceMarketOptionsElSpotOptionsElRef {
        LaunchTemplateInstanceMarketOptionsElSpotOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateInstanceMarketOptionsElSpotOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `block_duration_minutes` after provisioning.\n"]
    pub fn block_duration_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.block_duration_minutes", self.base),
        )
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
struct LaunchTemplateInstanceMarketOptionsElDynamic {
    spot_options: Option<DynamicBlock<LaunchTemplateInstanceMarketOptionsElSpotOptionsEl>>,
}

#[derive(Serialize)]
pub struct LaunchTemplateInstanceMarketOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    market_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot_options: Option<Vec<LaunchTemplateInstanceMarketOptionsElSpotOptionsEl>>,
    dynamic: LaunchTemplateInstanceMarketOptionsElDynamic,
}

impl LaunchTemplateInstanceMarketOptionsEl {
    #[doc = "Set the field `market_type`.\n"]
    pub fn set_market_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.market_type = Some(v.into());
        self
    }

    #[doc = "Set the field `spot_options`.\n"]
    pub fn set_spot_options(
        mut self,
        v: impl Into<BlockAssignable<LaunchTemplateInstanceMarketOptionsElSpotOptionsEl>>,
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

impl ToListMappable for LaunchTemplateInstanceMarketOptionsEl {
    type O = BlockAssignable<LaunchTemplateInstanceMarketOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateInstanceMarketOptionsEl {}

impl BuildLaunchTemplateInstanceMarketOptionsEl {
    pub fn build(self) -> LaunchTemplateInstanceMarketOptionsEl {
        LaunchTemplateInstanceMarketOptionsEl {
            market_type: core::default::Default::default(),
            spot_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct LaunchTemplateInstanceMarketOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateInstanceMarketOptionsElRef {
    fn new(shared: StackShared, base: String) -> LaunchTemplateInstanceMarketOptionsElRef {
        LaunchTemplateInstanceMarketOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateInstanceMarketOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `market_type` after provisioning.\n"]
    pub fn market_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.market_type", self.base))
    }

    #[doc = "Get a reference to the value of field `spot_options` after provisioning.\n"]
    pub fn spot_options(&self) -> ListRef<LaunchTemplateInstanceMarketOptionsElSpotOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spot_options", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateInstanceRequirementsElAcceleratorCountEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}

impl LaunchTemplateInstanceRequirementsElAcceleratorCountEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max = Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min = Some(v.into());
        self
    }
}

impl ToListMappable for LaunchTemplateInstanceRequirementsElAcceleratorCountEl {
    type O = BlockAssignable<LaunchTemplateInstanceRequirementsElAcceleratorCountEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateInstanceRequirementsElAcceleratorCountEl {}

impl BuildLaunchTemplateInstanceRequirementsElAcceleratorCountEl {
    pub fn build(self) -> LaunchTemplateInstanceRequirementsElAcceleratorCountEl {
        LaunchTemplateInstanceRequirementsElAcceleratorCountEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateInstanceRequirementsElAcceleratorCountElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateInstanceRequirementsElAcceleratorCountElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LaunchTemplateInstanceRequirementsElAcceleratorCountElRef {
        LaunchTemplateInstanceRequirementsElAcceleratorCountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateInstanceRequirementsElAcceleratorCountElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}

impl LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max = Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min = Some(v.into());
        self
    }
}

impl ToListMappable for LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl {
    type O = BlockAssignable<LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl {}

impl BuildLaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl {
    pub fn build(self) -> LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl {
        LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
        LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}

impl LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max = Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min = Some(v.into());
        self
    }
}

impl ToListMappable for LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
    type O = BlockAssignable<LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl {}

impl BuildLaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
    pub fn build(self) -> LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
        LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
        LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}

impl LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max = Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min = Some(v.into());
        self
    }
}

impl ToListMappable for LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl {
    type O = BlockAssignable<LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl {}

impl BuildLaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl {
    pub fn build(self) -> LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl {
        LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuElRef {
        LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateInstanceRequirementsElMemoryMibEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    min: PrimField<f64>,
}

impl LaunchTemplateInstanceRequirementsElMemoryMibEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max = Some(v.into());
        self
    }
}

impl ToListMappable for LaunchTemplateInstanceRequirementsElMemoryMibEl {
    type O = BlockAssignable<LaunchTemplateInstanceRequirementsElMemoryMibEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateInstanceRequirementsElMemoryMibEl {
    #[doc = ""]
    pub min: PrimField<f64>,
}

impl BuildLaunchTemplateInstanceRequirementsElMemoryMibEl {
    pub fn build(self) -> LaunchTemplateInstanceRequirementsElMemoryMibEl {
        LaunchTemplateInstanceRequirementsElMemoryMibEl {
            max: core::default::Default::default(),
            min: self.min,
        }
    }
}

pub struct LaunchTemplateInstanceRequirementsElMemoryMibElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateInstanceRequirementsElMemoryMibElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LaunchTemplateInstanceRequirementsElMemoryMibElRef {
        LaunchTemplateInstanceRequirementsElMemoryMibElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateInstanceRequirementsElMemoryMibElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateInstanceRequirementsElNetworkBandwidthGbpsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}

impl LaunchTemplateInstanceRequirementsElNetworkBandwidthGbpsEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max = Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min = Some(v.into());
        self
    }
}

impl ToListMappable for LaunchTemplateInstanceRequirementsElNetworkBandwidthGbpsEl {
    type O = BlockAssignable<LaunchTemplateInstanceRequirementsElNetworkBandwidthGbpsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateInstanceRequirementsElNetworkBandwidthGbpsEl {}

impl BuildLaunchTemplateInstanceRequirementsElNetworkBandwidthGbpsEl {
    pub fn build(self) -> LaunchTemplateInstanceRequirementsElNetworkBandwidthGbpsEl {
        LaunchTemplateInstanceRequirementsElNetworkBandwidthGbpsEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateInstanceRequirementsElNetworkBandwidthGbpsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateInstanceRequirementsElNetworkBandwidthGbpsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LaunchTemplateInstanceRequirementsElNetworkBandwidthGbpsElRef {
        LaunchTemplateInstanceRequirementsElNetworkBandwidthGbpsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateInstanceRequirementsElNetworkBandwidthGbpsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}

impl LaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max = Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min = Some(v.into());
        self
    }
}

impl ToListMappable for LaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl {
    type O = BlockAssignable<LaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl {}

impl BuildLaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl {
    pub fn build(self) -> LaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl {
        LaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateInstanceRequirementsElNetworkInterfaceCountElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateInstanceRequirementsElNetworkInterfaceCountElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LaunchTemplateInstanceRequirementsElNetworkInterfaceCountElRef {
        LaunchTemplateInstanceRequirementsElNetworkInterfaceCountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateInstanceRequirementsElNetworkInterfaceCountElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}

impl LaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max = Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min = Some(v.into());
        self
    }
}

impl ToListMappable for LaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl {
    type O = BlockAssignable<LaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl {}

impl BuildLaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl {
    pub fn build(self) -> LaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl {
        LaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateInstanceRequirementsElTotalLocalStorageGbElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateInstanceRequirementsElTotalLocalStorageGbElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LaunchTemplateInstanceRequirementsElTotalLocalStorageGbElRef {
        LaunchTemplateInstanceRequirementsElTotalLocalStorageGbElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateInstanceRequirementsElTotalLocalStorageGbElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateInstanceRequirementsElVcpuCountEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    min: PrimField<f64>,
}

impl LaunchTemplateInstanceRequirementsElVcpuCountEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max = Some(v.into());
        self
    }
}

impl ToListMappable for LaunchTemplateInstanceRequirementsElVcpuCountEl {
    type O = BlockAssignable<LaunchTemplateInstanceRequirementsElVcpuCountEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateInstanceRequirementsElVcpuCountEl {
    #[doc = ""]
    pub min: PrimField<f64>,
}

impl BuildLaunchTemplateInstanceRequirementsElVcpuCountEl {
    pub fn build(self) -> LaunchTemplateInstanceRequirementsElVcpuCountEl {
        LaunchTemplateInstanceRequirementsElVcpuCountEl {
            max: core::default::Default::default(),
            min: self.min,
        }
    }
}

pub struct LaunchTemplateInstanceRequirementsElVcpuCountElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateInstanceRequirementsElVcpuCountElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LaunchTemplateInstanceRequirementsElVcpuCountElRef {
        LaunchTemplateInstanceRequirementsElVcpuCountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateInstanceRequirementsElVcpuCountElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize, Default)]
struct LaunchTemplateInstanceRequirementsElDynamic {
    accelerator_count: Option<DynamicBlock<LaunchTemplateInstanceRequirementsElAcceleratorCountEl>>,
    accelerator_total_memory_mib:
        Option<DynamicBlock<LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl>>,
    baseline_ebs_bandwidth_mbps:
        Option<DynamicBlock<LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl>>,
    memory_gib_per_vcpu:
        Option<DynamicBlock<LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl>>,
    memory_mib: Option<DynamicBlock<LaunchTemplateInstanceRequirementsElMemoryMibEl>>,
    network_bandwidth_gbps:
        Option<DynamicBlock<LaunchTemplateInstanceRequirementsElNetworkBandwidthGbpsEl>>,
    network_interface_count:
        Option<DynamicBlock<LaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl>>,
    total_local_storage_gb:
        Option<DynamicBlock<LaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl>>,
    vcpu_count: Option<DynamicBlock<LaunchTemplateInstanceRequirementsElVcpuCountEl>>,
}

#[derive(Serialize)]
pub struct LaunchTemplateInstanceRequirementsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_manufacturers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_names: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_instance_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bare_metal: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    burstable_performance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_manufacturers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_instance_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_generations: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_storage: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_storage_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_spot_price_as_percentage_of_optimal_on_demand_price: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_demand_max_price_percentage_over_lowest_price: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_hibernate_support: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot_max_price_percentage_over_lowest_price: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_count: Option<Vec<LaunchTemplateInstanceRequirementsElAcceleratorCountEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_total_memory_mib:
        Option<Vec<LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    baseline_ebs_bandwidth_mbps:
        Option<Vec<LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_gib_per_vcpu: Option<Vec<LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_mib: Option<Vec<LaunchTemplateInstanceRequirementsElMemoryMibEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_bandwidth_gbps: Option<Vec<LaunchTemplateInstanceRequirementsElNetworkBandwidthGbpsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interface_count:
        Option<Vec<LaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_local_storage_gb: Option<Vec<LaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcpu_count: Option<Vec<LaunchTemplateInstanceRequirementsElVcpuCountEl>>,
    dynamic: LaunchTemplateInstanceRequirementsElDynamic,
}

impl LaunchTemplateInstanceRequirementsEl {
    #[doc = "Set the field `accelerator_manufacturers`.\n"]
    pub fn set_accelerator_manufacturers(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.accelerator_manufacturers = Some(v.into());
        self
    }

    #[doc = "Set the field `accelerator_names`.\n"]
    pub fn set_accelerator_names(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.accelerator_names = Some(v.into());
        self
    }

    #[doc = "Set the field `accelerator_types`.\n"]
    pub fn set_accelerator_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.accelerator_types = Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_instance_types`.\n"]
    pub fn set_allowed_instance_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allowed_instance_types = Some(v.into());
        self
    }

    #[doc = "Set the field `bare_metal`.\n"]
    pub fn set_bare_metal(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bare_metal = Some(v.into());
        self
    }

    #[doc = "Set the field `burstable_performance`.\n"]
    pub fn set_burstable_performance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.burstable_performance = Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_manufacturers`.\n"]
    pub fn set_cpu_manufacturers(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.cpu_manufacturers = Some(v.into());
        self
    }

    #[doc = "Set the field `excluded_instance_types`.\n"]
    pub fn set_excluded_instance_types(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.excluded_instance_types = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_generations`.\n"]
    pub fn set_instance_generations(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.instance_generations = Some(v.into());
        self
    }

    #[doc = "Set the field `local_storage`.\n"]
    pub fn set_local_storage(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_storage = Some(v.into());
        self
    }

    #[doc = "Set the field `local_storage_types`.\n"]
    pub fn set_local_storage_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.local_storage_types = Some(v.into());
        self
    }

    #[doc = "Set the field `max_spot_price_as_percentage_of_optimal_on_demand_price`.\n"]
    pub fn set_max_spot_price_as_percentage_of_optimal_on_demand_price(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.max_spot_price_as_percentage_of_optimal_on_demand_price = Some(v.into());
        self
    }

    #[doc = "Set the field `on_demand_max_price_percentage_over_lowest_price`.\n"]
    pub fn set_on_demand_max_price_percentage_over_lowest_price(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.on_demand_max_price_percentage_over_lowest_price = Some(v.into());
        self
    }

    #[doc = "Set the field `require_hibernate_support`.\n"]
    pub fn set_require_hibernate_support(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_hibernate_support = Some(v.into());
        self
    }

    #[doc = "Set the field `spot_max_price_percentage_over_lowest_price`.\n"]
    pub fn set_spot_max_price_percentage_over_lowest_price(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.spot_max_price_percentage_over_lowest_price = Some(v.into());
        self
    }

    #[doc = "Set the field `accelerator_count`.\n"]
    pub fn set_accelerator_count(
        mut self,
        v: impl Into<BlockAssignable<LaunchTemplateInstanceRequirementsElAcceleratorCountEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.accelerator_count = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerator_count = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `accelerator_total_memory_mib`.\n"]
    pub fn set_accelerator_total_memory_mib(
        mut self,
        v: impl Into<BlockAssignable<LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.accelerator_total_memory_mib = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerator_total_memory_mib = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `baseline_ebs_bandwidth_mbps`.\n"]
    pub fn set_baseline_ebs_bandwidth_mbps(
        mut self,
        v: impl Into<BlockAssignable<LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.baseline_ebs_bandwidth_mbps = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.baseline_ebs_bandwidth_mbps = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `memory_gib_per_vcpu`.\n"]
    pub fn set_memory_gib_per_vcpu(
        mut self,
        v: impl Into<BlockAssignable<LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.memory_gib_per_vcpu = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.memory_gib_per_vcpu = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `memory_mib`.\n"]
    pub fn set_memory_mib(
        mut self,
        v: impl Into<BlockAssignable<LaunchTemplateInstanceRequirementsElMemoryMibEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.memory_mib = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.memory_mib = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `network_bandwidth_gbps`.\n"]
    pub fn set_network_bandwidth_gbps(
        mut self,
        v: impl Into<BlockAssignable<LaunchTemplateInstanceRequirementsElNetworkBandwidthGbpsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_bandwidth_gbps = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_bandwidth_gbps = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `network_interface_count`.\n"]
    pub fn set_network_interface_count(
        mut self,
        v: impl Into<BlockAssignable<LaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_interface_count = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_interface_count = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `total_local_storage_gb`.\n"]
    pub fn set_total_local_storage_gb(
        mut self,
        v: impl Into<BlockAssignable<LaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.total_local_storage_gb = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.total_local_storage_gb = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `vcpu_count`.\n"]
    pub fn set_vcpu_count(
        mut self,
        v: impl Into<BlockAssignable<LaunchTemplateInstanceRequirementsElVcpuCountEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vcpu_count = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vcpu_count = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for LaunchTemplateInstanceRequirementsEl {
    type O = BlockAssignable<LaunchTemplateInstanceRequirementsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateInstanceRequirementsEl {}

impl BuildLaunchTemplateInstanceRequirementsEl {
    pub fn build(self) -> LaunchTemplateInstanceRequirementsEl {
        LaunchTemplateInstanceRequirementsEl {
            accelerator_manufacturers: core::default::Default::default(),
            accelerator_names: core::default::Default::default(),
            accelerator_types: core::default::Default::default(),
            allowed_instance_types: core::default::Default::default(),
            bare_metal: core::default::Default::default(),
            burstable_performance: core::default::Default::default(),
            cpu_manufacturers: core::default::Default::default(),
            excluded_instance_types: core::default::Default::default(),
            instance_generations: core::default::Default::default(),
            local_storage: core::default::Default::default(),
            local_storage_types: core::default::Default::default(),
            max_spot_price_as_percentage_of_optimal_on_demand_price:
                core::default::Default::default(),
            on_demand_max_price_percentage_over_lowest_price: core::default::Default::default(),
            require_hibernate_support: core::default::Default::default(),
            spot_max_price_percentage_over_lowest_price: core::default::Default::default(),
            accelerator_count: core::default::Default::default(),
            accelerator_total_memory_mib: core::default::Default::default(),
            baseline_ebs_bandwidth_mbps: core::default::Default::default(),
            memory_gib_per_vcpu: core::default::Default::default(),
            memory_mib: core::default::Default::default(),
            network_bandwidth_gbps: core::default::Default::default(),
            network_interface_count: core::default::Default::default(),
            total_local_storage_gb: core::default::Default::default(),
            vcpu_count: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct LaunchTemplateInstanceRequirementsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateInstanceRequirementsElRef {
    fn new(shared: StackShared, base: String) -> LaunchTemplateInstanceRequirementsElRef {
        LaunchTemplateInstanceRequirementsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateInstanceRequirementsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accelerator_manufacturers` after provisioning.\n"]
    pub fn accelerator_manufacturers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.accelerator_manufacturers", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `accelerator_names` after provisioning.\n"]
    pub fn accelerator_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.accelerator_names", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `accelerator_types` after provisioning.\n"]
    pub fn accelerator_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.accelerator_types", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `allowed_instance_types` after provisioning.\n"]
    pub fn allowed_instance_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.allowed_instance_types", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `bare_metal` after provisioning.\n"]
    pub fn bare_metal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bare_metal", self.base))
    }

    #[doc = "Get a reference to the value of field `burstable_performance` after provisioning.\n"]
    pub fn burstable_performance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.burstable_performance", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `cpu_manufacturers` after provisioning.\n"]
    pub fn cpu_manufacturers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.cpu_manufacturers", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `excluded_instance_types` after provisioning.\n"]
    pub fn excluded_instance_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.excluded_instance_types", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `instance_generations` after provisioning.\n"]
    pub fn instance_generations(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.instance_generations", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `local_storage` after provisioning.\n"]
    pub fn local_storage(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_storage", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `local_storage_types` after provisioning.\n"]
    pub fn local_storage_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.local_storage_types", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `max_spot_price_as_percentage_of_optimal_on_demand_price` after provisioning.\n"]
    pub fn max_spot_price_as_percentage_of_optimal_on_demand_price(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.max_spot_price_as_percentage_of_optimal_on_demand_price",
                self.base
            ),
        )
    }

    #[doc = "Get a reference to the value of field `on_demand_max_price_percentage_over_lowest_price` after provisioning.\n"]
    pub fn on_demand_max_price_percentage_over_lowest_price(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.on_demand_max_price_percentage_over_lowest_price",
                self.base
            ),
        )
    }

    #[doc = "Get a reference to the value of field `require_hibernate_support` after provisioning.\n"]
    pub fn require_hibernate_support(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_hibernate_support", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `spot_max_price_percentage_over_lowest_price` after provisioning.\n"]
    pub fn spot_max_price_percentage_over_lowest_price(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.spot_max_price_percentage_over_lowest_price", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `accelerator_count` after provisioning.\n"]
    pub fn accelerator_count(
        &self,
    ) -> ListRef<LaunchTemplateInstanceRequirementsElAcceleratorCountElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.accelerator_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `accelerator_total_memory_mib` after provisioning.\n"]
    pub fn accelerator_total_memory_mib(
        &self,
    ) -> ListRef<LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.accelerator_total_memory_mib", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `baseline_ebs_bandwidth_mbps` after provisioning.\n"]
    pub fn baseline_ebs_bandwidth_mbps(
        &self,
    ) -> ListRef<LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.baseline_ebs_bandwidth_mbps", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `memory_gib_per_vcpu` after provisioning.\n"]
    pub fn memory_gib_per_vcpu(
        &self,
    ) -> ListRef<LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.memory_gib_per_vcpu", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `memory_mib` after provisioning.\n"]
    pub fn memory_mib(&self) -> ListRef<LaunchTemplateInstanceRequirementsElMemoryMibElRef> {
        ListRef::new(self.shared().clone(), format!("{}.memory_mib", self.base))
    }

    #[doc = "Get a reference to the value of field `network_bandwidth_gbps` after provisioning.\n"]
    pub fn network_bandwidth_gbps(
        &self,
    ) -> ListRef<LaunchTemplateInstanceRequirementsElNetworkBandwidthGbpsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_bandwidth_gbps", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `network_interface_count` after provisioning.\n"]
    pub fn network_interface_count(
        &self,
    ) -> ListRef<LaunchTemplateInstanceRequirementsElNetworkInterfaceCountElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_interface_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `total_local_storage_gb` after provisioning.\n"]
    pub fn total_local_storage_gb(
        &self,
    ) -> ListRef<LaunchTemplateInstanceRequirementsElTotalLocalStorageGbElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.total_local_storage_gb", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `vcpu_count` after provisioning.\n"]
    pub fn vcpu_count(&self) -> ListRef<LaunchTemplateInstanceRequirementsElVcpuCountElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vcpu_count", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateLicenseSpecificationEl {
    license_configuration_arn: PrimField<String>,
}

impl LaunchTemplateLicenseSpecificationEl {}

impl ToListMappable for LaunchTemplateLicenseSpecificationEl {
    type O = BlockAssignable<LaunchTemplateLicenseSpecificationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateLicenseSpecificationEl {
    #[doc = ""]
    pub license_configuration_arn: PrimField<String>,
}

impl BuildLaunchTemplateLicenseSpecificationEl {
    pub fn build(self) -> LaunchTemplateLicenseSpecificationEl {
        LaunchTemplateLicenseSpecificationEl {
            license_configuration_arn: self.license_configuration_arn,
        }
    }
}

pub struct LaunchTemplateLicenseSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateLicenseSpecificationElRef {
    fn new(shared: StackShared, base: String) -> LaunchTemplateLicenseSpecificationElRef {
        LaunchTemplateLicenseSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateLicenseSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `license_configuration_arn` after provisioning.\n"]
    pub fn license_configuration_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.license_configuration_arn", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateMaintenanceOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_recovery: Option<PrimField<String>>,
}

impl LaunchTemplateMaintenanceOptionsEl {
    #[doc = "Set the field `auto_recovery`.\n"]
    pub fn set_auto_recovery(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auto_recovery = Some(v.into());
        self
    }
}

impl ToListMappable for LaunchTemplateMaintenanceOptionsEl {
    type O = BlockAssignable<LaunchTemplateMaintenanceOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateMaintenanceOptionsEl {}

impl BuildLaunchTemplateMaintenanceOptionsEl {
    pub fn build(self) -> LaunchTemplateMaintenanceOptionsEl {
        LaunchTemplateMaintenanceOptionsEl {
            auto_recovery: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateMaintenanceOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateMaintenanceOptionsElRef {
    fn new(shared: StackShared, base: String) -> LaunchTemplateMaintenanceOptionsElRef {
        LaunchTemplateMaintenanceOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateMaintenanceOptionsElRef {
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
pub struct LaunchTemplateMetadataOptionsEl {
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

impl LaunchTemplateMetadataOptionsEl {
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

impl ToListMappable for LaunchTemplateMetadataOptionsEl {
    type O = BlockAssignable<LaunchTemplateMetadataOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateMetadataOptionsEl {}

impl BuildLaunchTemplateMetadataOptionsEl {
    pub fn build(self) -> LaunchTemplateMetadataOptionsEl {
        LaunchTemplateMetadataOptionsEl {
            http_endpoint: core::default::Default::default(),
            http_protocol_ipv6: core::default::Default::default(),
            http_put_response_hop_limit: core::default::Default::default(),
            http_tokens: core::default::Default::default(),
            instance_metadata_tags: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateMetadataOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateMetadataOptionsElRef {
    fn new(shared: StackShared, base: String) -> LaunchTemplateMetadataOptionsElRef {
        LaunchTemplateMetadataOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateMetadataOptionsElRef {
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
pub struct LaunchTemplateMonitoringEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl LaunchTemplateMonitoringEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for LaunchTemplateMonitoringEl {
    type O = BlockAssignable<LaunchTemplateMonitoringEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateMonitoringEl {}

impl BuildLaunchTemplateMonitoringEl {
    pub fn build(self) -> LaunchTemplateMonitoringEl {
        LaunchTemplateMonitoringEl {
            enabled: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateMonitoringElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateMonitoringElRef {
    fn new(shared: StackShared, base: String) -> LaunchTemplateMonitoringElRef {
        LaunchTemplateMonitoringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateMonitoringElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateNetworkInterfacesElConnectionTrackingSpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_established_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    udp_stream_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    udp_timeout: Option<PrimField<f64>>,
}

impl LaunchTemplateNetworkInterfacesElConnectionTrackingSpecificationEl {
    #[doc = "Set the field `tcp_established_timeout`.\n"]
    pub fn set_tcp_established_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.tcp_established_timeout = Some(v.into());
        self
    }

    #[doc = "Set the field `udp_stream_timeout`.\n"]
    pub fn set_udp_stream_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.udp_stream_timeout = Some(v.into());
        self
    }

    #[doc = "Set the field `udp_timeout`.\n"]
    pub fn set_udp_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.udp_timeout = Some(v.into());
        self
    }
}

impl ToListMappable for LaunchTemplateNetworkInterfacesElConnectionTrackingSpecificationEl {
    type O = BlockAssignable<LaunchTemplateNetworkInterfacesElConnectionTrackingSpecificationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateNetworkInterfacesElConnectionTrackingSpecificationEl {}

impl BuildLaunchTemplateNetworkInterfacesElConnectionTrackingSpecificationEl {
    pub fn build(self) -> LaunchTemplateNetworkInterfacesElConnectionTrackingSpecificationEl {
        LaunchTemplateNetworkInterfacesElConnectionTrackingSpecificationEl {
            tcp_established_timeout: core::default::Default::default(),
            udp_stream_timeout: core::default::Default::default(),
            udp_timeout: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateNetworkInterfacesElConnectionTrackingSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateNetworkInterfacesElConnectionTrackingSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LaunchTemplateNetworkInterfacesElConnectionTrackingSpecificationElRef {
        LaunchTemplateNetworkInterfacesElConnectionTrackingSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateNetworkInterfacesElConnectionTrackingSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tcp_established_timeout` after provisioning.\n"]
    pub fn tcp_established_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tcp_established_timeout", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `udp_stream_timeout` after provisioning.\n"]
    pub fn udp_stream_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.udp_stream_timeout", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `udp_timeout` after provisioning.\n"]
    pub fn udp_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.udp_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateNetworkInterfacesElEnaSrdSpecificationElEnaSrdUdpSpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ena_srd_udp_enabled: Option<PrimField<bool>>,
}

impl LaunchTemplateNetworkInterfacesElEnaSrdSpecificationElEnaSrdUdpSpecificationEl {
    #[doc = "Set the field `ena_srd_udp_enabled`.\n"]
    pub fn set_ena_srd_udp_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ena_srd_udp_enabled = Some(v.into());
        self
    }
}

impl ToListMappable
    for LaunchTemplateNetworkInterfacesElEnaSrdSpecificationElEnaSrdUdpSpecificationEl
{
    type O = BlockAssignable<
        LaunchTemplateNetworkInterfacesElEnaSrdSpecificationElEnaSrdUdpSpecificationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateNetworkInterfacesElEnaSrdSpecificationElEnaSrdUdpSpecificationEl {}

impl BuildLaunchTemplateNetworkInterfacesElEnaSrdSpecificationElEnaSrdUdpSpecificationEl {
    pub fn build(
        self,
    ) -> LaunchTemplateNetworkInterfacesElEnaSrdSpecificationElEnaSrdUdpSpecificationEl {
        LaunchTemplateNetworkInterfacesElEnaSrdSpecificationElEnaSrdUdpSpecificationEl {
            ena_srd_udp_enabled: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateNetworkInterfacesElEnaSrdSpecificationElEnaSrdUdpSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateNetworkInterfacesElEnaSrdSpecificationElEnaSrdUdpSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LaunchTemplateNetworkInterfacesElEnaSrdSpecificationElEnaSrdUdpSpecificationElRef {
        LaunchTemplateNetworkInterfacesElEnaSrdSpecificationElEnaSrdUdpSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateNetworkInterfacesElEnaSrdSpecificationElEnaSrdUdpSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ena_srd_udp_enabled` after provisioning.\n"]
    pub fn ena_srd_udp_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ena_srd_udp_enabled", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct LaunchTemplateNetworkInterfacesElEnaSrdSpecificationElDynamic {
    ena_srd_udp_specification: Option<
        DynamicBlock<
            LaunchTemplateNetworkInterfacesElEnaSrdSpecificationElEnaSrdUdpSpecificationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct LaunchTemplateNetworkInterfacesElEnaSrdSpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ena_srd_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ena_srd_udp_specification:
        Option<Vec<LaunchTemplateNetworkInterfacesElEnaSrdSpecificationElEnaSrdUdpSpecificationEl>>,
    dynamic: LaunchTemplateNetworkInterfacesElEnaSrdSpecificationElDynamic,
}

impl LaunchTemplateNetworkInterfacesElEnaSrdSpecificationEl {
    #[doc = "Set the field `ena_srd_enabled`.\n"]
    pub fn set_ena_srd_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ena_srd_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `ena_srd_udp_specification`.\n"]
    pub fn set_ena_srd_udp_specification(
        mut self,
        v: impl Into<
            BlockAssignable<
                LaunchTemplateNetworkInterfacesElEnaSrdSpecificationElEnaSrdUdpSpecificationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ena_srd_udp_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ena_srd_udp_specification = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for LaunchTemplateNetworkInterfacesElEnaSrdSpecificationEl {
    type O = BlockAssignable<LaunchTemplateNetworkInterfacesElEnaSrdSpecificationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateNetworkInterfacesElEnaSrdSpecificationEl {}

impl BuildLaunchTemplateNetworkInterfacesElEnaSrdSpecificationEl {
    pub fn build(self) -> LaunchTemplateNetworkInterfacesElEnaSrdSpecificationEl {
        LaunchTemplateNetworkInterfacesElEnaSrdSpecificationEl {
            ena_srd_enabled: core::default::Default::default(),
            ena_srd_udp_specification: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct LaunchTemplateNetworkInterfacesElEnaSrdSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateNetworkInterfacesElEnaSrdSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LaunchTemplateNetworkInterfacesElEnaSrdSpecificationElRef {
        LaunchTemplateNetworkInterfacesElEnaSrdSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateNetworkInterfacesElEnaSrdSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ena_srd_enabled` after provisioning.\n"]
    pub fn ena_srd_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ena_srd_enabled", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ena_srd_udp_specification` after provisioning.\n"]
    pub fn ena_srd_udp_specification(
        &self,
    ) -> ListRef<LaunchTemplateNetworkInterfacesElEnaSrdSpecificationElEnaSrdUdpSpecificationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ena_srd_udp_specification", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct LaunchTemplateNetworkInterfacesElDynamic {
    connection_tracking_specification:
        Option<DynamicBlock<LaunchTemplateNetworkInterfacesElConnectionTrackingSpecificationEl>>,
    ena_srd_specification:
        Option<DynamicBlock<LaunchTemplateNetworkInterfacesElEnaSrdSpecificationEl>>,
}

#[derive(Serialize)]
pub struct LaunchTemplateNetworkInterfacesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    associate_carrier_ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    associate_public_ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_on_termination: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_index: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interface_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv4_address_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv4_addresses: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv4_prefix_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv4_prefixes: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_address_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_addresses: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_prefix_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_prefixes: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_card_index: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interface_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_ipv6: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_groups: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_tracking_specification:
        Option<Vec<LaunchTemplateNetworkInterfacesElConnectionTrackingSpecificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ena_srd_specification: Option<Vec<LaunchTemplateNetworkInterfacesElEnaSrdSpecificationEl>>,
    dynamic: LaunchTemplateNetworkInterfacesElDynamic,
}

impl LaunchTemplateNetworkInterfacesEl {
    #[doc = "Set the field `associate_carrier_ip_address`.\n"]
    pub fn set_associate_carrier_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.associate_carrier_ip_address = Some(v.into());
        self
    }

    #[doc = "Set the field `associate_public_ip_address`.\n"]
    pub fn set_associate_public_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.associate_public_ip_address = Some(v.into());
        self
    }

    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete_on_termination = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc = "Set the field `device_index`.\n"]
    pub fn set_device_index(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.device_index = Some(v.into());
        self
    }

    #[doc = "Set the field `interface_type`.\n"]
    pub fn set_interface_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interface_type = Some(v.into());
        self
    }

    #[doc = "Set the field `ipv4_address_count`.\n"]
    pub fn set_ipv4_address_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ipv4_address_count = Some(v.into());
        self
    }

    #[doc = "Set the field `ipv4_addresses`.\n"]
    pub fn set_ipv4_addresses(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.ipv4_addresses = Some(v.into());
        self
    }

    #[doc = "Set the field `ipv4_prefix_count`.\n"]
    pub fn set_ipv4_prefix_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ipv4_prefix_count = Some(v.into());
        self
    }

    #[doc = "Set the field `ipv4_prefixes`.\n"]
    pub fn set_ipv4_prefixes(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.ipv4_prefixes = Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_address_count`.\n"]
    pub fn set_ipv6_address_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ipv6_address_count = Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_addresses`.\n"]
    pub fn set_ipv6_addresses(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.ipv6_addresses = Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_prefix_count`.\n"]
    pub fn set_ipv6_prefix_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ipv6_prefix_count = Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_prefixes`.\n"]
    pub fn set_ipv6_prefixes(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.ipv6_prefixes = Some(v.into());
        self
    }

    #[doc = "Set the field `network_card_index`.\n"]
    pub fn set_network_card_index(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.network_card_index = Some(v.into());
        self
    }

    #[doc = "Set the field `network_interface_id`.\n"]
    pub fn set_network_interface_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_interface_id = Some(v.into());
        self
    }

    #[doc = "Set the field `primary_ipv6`.\n"]
    pub fn set_primary_ipv6(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.primary_ipv6 = Some(v.into());
        self
    }

    #[doc = "Set the field `private_ip_address`.\n"]
    pub fn set_private_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_ip_address = Some(v.into());
        self
    }

    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_groups = Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnet_id = Some(v.into());
        self
    }

    #[doc = "Set the field `connection_tracking_specification`.\n"]
    pub fn set_connection_tracking_specification(
        mut self,
        v: impl Into<
            BlockAssignable<LaunchTemplateNetworkInterfacesElConnectionTrackingSpecificationEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.connection_tracking_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.connection_tracking_specification = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `ena_srd_specification`.\n"]
    pub fn set_ena_srd_specification(
        mut self,
        v: impl Into<BlockAssignable<LaunchTemplateNetworkInterfacesElEnaSrdSpecificationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ena_srd_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ena_srd_specification = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for LaunchTemplateNetworkInterfacesEl {
    type O = BlockAssignable<LaunchTemplateNetworkInterfacesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateNetworkInterfacesEl {}

impl BuildLaunchTemplateNetworkInterfacesEl {
    pub fn build(self) -> LaunchTemplateNetworkInterfacesEl {
        LaunchTemplateNetworkInterfacesEl {
            associate_carrier_ip_address: core::default::Default::default(),
            associate_public_ip_address: core::default::Default::default(),
            delete_on_termination: core::default::Default::default(),
            description: core::default::Default::default(),
            device_index: core::default::Default::default(),
            interface_type: core::default::Default::default(),
            ipv4_address_count: core::default::Default::default(),
            ipv4_addresses: core::default::Default::default(),
            ipv4_prefix_count: core::default::Default::default(),
            ipv4_prefixes: core::default::Default::default(),
            ipv6_address_count: core::default::Default::default(),
            ipv6_addresses: core::default::Default::default(),
            ipv6_prefix_count: core::default::Default::default(),
            ipv6_prefixes: core::default::Default::default(),
            network_card_index: core::default::Default::default(),
            network_interface_id: core::default::Default::default(),
            primary_ipv6: core::default::Default::default(),
            private_ip_address: core::default::Default::default(),
            security_groups: core::default::Default::default(),
            subnet_id: core::default::Default::default(),
            connection_tracking_specification: core::default::Default::default(),
            ena_srd_specification: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct LaunchTemplateNetworkInterfacesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateNetworkInterfacesElRef {
    fn new(shared: StackShared, base: String) -> LaunchTemplateNetworkInterfacesElRef {
        LaunchTemplateNetworkInterfacesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateNetworkInterfacesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `associate_carrier_ip_address` after provisioning.\n"]
    pub fn associate_carrier_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.associate_carrier_ip_address", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `associate_public_ip_address` after provisioning.\n"]
    pub fn associate_public_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.associate_public_ip_address", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_on_termination", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `device_index` after provisioning.\n"]
    pub fn device_index(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_index", self.base))
    }

    #[doc = "Get a reference to the value of field `interface_type` after provisioning.\n"]
    pub fn interface_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.interface_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ipv4_address_count` after provisioning.\n"]
    pub fn ipv4_address_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv4_address_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ipv4_addresses` after provisioning.\n"]
    pub fn ipv4_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.ipv4_addresses", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ipv4_prefix_count` after provisioning.\n"]
    pub fn ipv4_prefix_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv4_prefix_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ipv4_prefixes` after provisioning.\n"]
    pub fn ipv4_prefixes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.ipv4_prefixes", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ipv6_address_count` after provisioning.\n"]
    pub fn ipv6_address_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_address_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ipv6_addresses` after provisioning.\n"]
    pub fn ipv6_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.ipv6_addresses", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ipv6_prefix_count` after provisioning.\n"]
    pub fn ipv6_prefix_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_prefix_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ipv6_prefixes` after provisioning.\n"]
    pub fn ipv6_prefixes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.ipv6_prefixes", self.base),
        )
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

    #[doc = "Get a reference to the value of field `primary_ipv6` after provisioning.\n"]
    pub fn primary_ipv6(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.primary_ipv6", self.base))
    }

    #[doc = "Get a reference to the value of field `private_ip_address` after provisioning.\n"]
    pub fn private_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_ip_address", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_groups", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }

    #[doc = "Get a reference to the value of field `connection_tracking_specification` after provisioning.\n"]
    pub fn connection_tracking_specification(
        &self,
    ) -> ListRef<LaunchTemplateNetworkInterfacesElConnectionTrackingSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connection_tracking_specification", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ena_srd_specification` after provisioning.\n"]
    pub fn ena_srd_specification(
        &self,
    ) -> ListRef<LaunchTemplateNetworkInterfacesElEnaSrdSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ena_srd_specification", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct LaunchTemplatePlacementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    affinity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_resource_group_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partition_number: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spread_domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tenancy: Option<PrimField<String>>,
}

impl LaunchTemplatePlacementEl {
    #[doc = "Set the field `affinity`.\n"]
    pub fn set_affinity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.affinity = Some(v.into());
        self
    }

    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.availability_zone = Some(v.into());
        self
    }

    #[doc = "Set the field `group_id`.\n"]
    pub fn set_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_id = Some(v.into());
        self
    }

    #[doc = "Set the field `group_name`.\n"]
    pub fn set_group_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_name = Some(v.into());
        self
    }

    #[doc = "Set the field `host_id`.\n"]
    pub fn set_host_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_id = Some(v.into());
        self
    }

    #[doc = "Set the field `host_resource_group_arn`.\n"]
    pub fn set_host_resource_group_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_resource_group_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `partition_number`.\n"]
    pub fn set_partition_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.partition_number = Some(v.into());
        self
    }

    #[doc = "Set the field `spread_domain`.\n"]
    pub fn set_spread_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.spread_domain = Some(v.into());
        self
    }

    #[doc = "Set the field `tenancy`.\n"]
    pub fn set_tenancy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tenancy = Some(v.into());
        self
    }
}

impl ToListMappable for LaunchTemplatePlacementEl {
    type O = BlockAssignable<LaunchTemplatePlacementEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplatePlacementEl {}

impl BuildLaunchTemplatePlacementEl {
    pub fn build(self) -> LaunchTemplatePlacementEl {
        LaunchTemplatePlacementEl {
            affinity: core::default::Default::default(),
            availability_zone: core::default::Default::default(),
            group_id: core::default::Default::default(),
            group_name: core::default::Default::default(),
            host_id: core::default::Default::default(),
            host_resource_group_arn: core::default::Default::default(),
            partition_number: core::default::Default::default(),
            spread_domain: core::default::Default::default(),
            tenancy: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplatePlacementElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplatePlacementElRef {
    fn new(shared: StackShared, base: String) -> LaunchTemplatePlacementElRef {
        LaunchTemplatePlacementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplatePlacementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `affinity` after provisioning.\n"]
    pub fn affinity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.affinity", self.base))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `group_id` after provisioning.\n"]
    pub fn group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `group_name` after provisioning.\n"]
    pub fn group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_name", self.base))
    }

    #[doc = "Get a reference to the value of field `host_id` after provisioning.\n"]
    pub fn host_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_id", self.base))
    }

    #[doc = "Get a reference to the value of field `host_resource_group_arn` after provisioning.\n"]
    pub fn host_resource_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host_resource_group_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `partition_number` after provisioning.\n"]
    pub fn partition_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.partition_number", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `spread_domain` after provisioning.\n"]
    pub fn spread_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.spread_domain", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `tenancy` after provisioning.\n"]
    pub fn tenancy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tenancy", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplatePrivateDnsNameOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_resource_name_dns_a_record: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_resource_name_dns_aaaa_record: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname_type: Option<PrimField<String>>,
}

impl LaunchTemplatePrivateDnsNameOptionsEl {
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

impl ToListMappable for LaunchTemplatePrivateDnsNameOptionsEl {
    type O = BlockAssignable<LaunchTemplatePrivateDnsNameOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplatePrivateDnsNameOptionsEl {}

impl BuildLaunchTemplatePrivateDnsNameOptionsEl {
    pub fn build(self) -> LaunchTemplatePrivateDnsNameOptionsEl {
        LaunchTemplatePrivateDnsNameOptionsEl {
            enable_resource_name_dns_a_record: core::default::Default::default(),
            enable_resource_name_dns_aaaa_record: core::default::Default::default(),
            hostname_type: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplatePrivateDnsNameOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplatePrivateDnsNameOptionsElRef {
    fn new(shared: StackShared, base: String) -> LaunchTemplatePrivateDnsNameOptionsElRef {
        LaunchTemplatePrivateDnsNameOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplatePrivateDnsNameOptionsElRef {
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
pub struct LaunchTemplateTagSpecificationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

impl LaunchTemplateTagSpecificationsEl {
    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_type = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
}

impl ToListMappable for LaunchTemplateTagSpecificationsEl {
    type O = BlockAssignable<LaunchTemplateTagSpecificationsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateTagSpecificationsEl {}

impl BuildLaunchTemplateTagSpecificationsEl {
    pub fn build(self) -> LaunchTemplateTagSpecificationsEl {
        LaunchTemplateTagSpecificationsEl {
            resource_type: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateTagSpecificationsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LaunchTemplateTagSpecificationsElRef {
    fn new(shared: StackShared, base: String) -> LaunchTemplateTagSpecificationsElRef {
        LaunchTemplateTagSpecificationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateTagSpecificationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct LaunchTemplateDynamic {
    block_device_mappings: Option<DynamicBlock<LaunchTemplateBlockDeviceMappingsEl>>,
    capacity_reservation_specification:
        Option<DynamicBlock<LaunchTemplateCapacityReservationSpecificationEl>>,
    cpu_options: Option<DynamicBlock<LaunchTemplateCpuOptionsEl>>,
    credit_specification: Option<DynamicBlock<LaunchTemplateCreditSpecificationEl>>,
    enclave_options: Option<DynamicBlock<LaunchTemplateEnclaveOptionsEl>>,
    hibernation_options: Option<DynamicBlock<LaunchTemplateHibernationOptionsEl>>,
    iam_instance_profile: Option<DynamicBlock<LaunchTemplateIamInstanceProfileEl>>,
    instance_market_options: Option<DynamicBlock<LaunchTemplateInstanceMarketOptionsEl>>,
    instance_requirements: Option<DynamicBlock<LaunchTemplateInstanceRequirementsEl>>,
    license_specification: Option<DynamicBlock<LaunchTemplateLicenseSpecificationEl>>,
    maintenance_options: Option<DynamicBlock<LaunchTemplateMaintenanceOptionsEl>>,
    metadata_options: Option<DynamicBlock<LaunchTemplateMetadataOptionsEl>>,
    monitoring: Option<DynamicBlock<LaunchTemplateMonitoringEl>>,
    network_interfaces: Option<DynamicBlock<LaunchTemplateNetworkInterfacesEl>>,
    placement: Option<DynamicBlock<LaunchTemplatePlacementEl>>,
    private_dns_name_options: Option<DynamicBlock<LaunchTemplatePrivateDnsNameOptionsEl>>,
    tag_specifications: Option<DynamicBlock<LaunchTemplateTagSpecificationsEl>>,
}
