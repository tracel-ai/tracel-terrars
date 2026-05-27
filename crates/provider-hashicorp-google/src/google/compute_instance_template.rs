use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeInstanceTemplateData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_ip_forward: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_revocation_action_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    machine_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_startup_script: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_cpu_platform: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_policies: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_machine_features: Option<Vec<ComputeInstanceTemplateAdvancedMachineFeaturesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confidential_instance_config: Option<Vec<ComputeInstanceTemplateConfidentialInstanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk: Option<Vec<ComputeInstanceTemplateDiskEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_accelerator: Option<Vec<ComputeInstanceTemplateGuestAcceleratorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interface: Option<Vec<ComputeInstanceTemplateNetworkInterfaceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_performance_config: Option<Vec<ComputeInstanceTemplateNetworkPerformanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reservation_affinity: Option<Vec<ComputeInstanceTemplateReservationAffinityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduling: Option<Vec<ComputeInstanceTemplateSchedulingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<Vec<ComputeInstanceTemplateServiceAccountEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shielded_instance_config: Option<Vec<ComputeInstanceTemplateShieldedInstanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeInstanceTemplateTimeoutsEl>,
    dynamic: ComputeInstanceTemplateDynamic,
}
struct ComputeInstanceTemplate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeInstanceTemplateData>,
}
#[derive(Clone)]
pub struct ComputeInstanceTemplate(Rc<ComputeInstanceTemplate_>);
impl ComputeInstanceTemplate {
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
    #[doc = "Set the field `can_ip_forward`.\nWhether to allow sending and receiving of packets with non-matching source or destination IPs. This defaults to false."]
    pub fn set_can_ip_forward(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().can_ip_forward = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nA brief description of this resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_description`.\nA description of the instance."]
    pub fn set_instance_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().instance_description = Some(v.into());
        self
    }
    #[doc = "Set the field `key_revocation_action_type`.\nAction to be taken when a customer's encryption key is revoked. Supports \"STOP\" and \"NONE\", with \"NONE\" being the default."]
    pub fn set_key_revocation_action_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().key_revocation_action_type = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nA set of key/value label pairs to assign to instances created from this template.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata`.\nMetadata key/value pairs to make available from within instances created from this template."]
    pub fn set_metadata(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().metadata = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata_startup_script`.\nAn alternative to using the startup-script metadata key, mostly to match the compute_instance resource. This replaces the startup-script metadata key on the created instance and thus the two mechanisms are not allowed to be used simultaneously."]
    pub fn set_metadata_startup_script(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().metadata_startup_script = Some(v.into());
        self
    }
    #[doc = "Set the field `min_cpu_platform`.\nSpecifies a minimum CPU platform. Applicable values are the friendly names of CPU platforms, such as Intel Haswell or Intel Skylake."]
    pub fn set_min_cpu_platform(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().min_cpu_platform = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nThe name of the instance template. If you leave this blank, Terraform will auto-generate a unique name."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `name_prefix`.\nCreates a unique name beginning with the specified prefix. Conflicts with name. Max length is 54 characters. Prefixes with lengths longer than 37 characters will use a shortened UUID that will be more prone to collisions."]
    pub fn set_name_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nAn instance template is a global resource that is not bound to a zone or a region. However, you can still specify some regional resources in an instance template, which restricts the template to the region where that resource resides. For example, a custom subnetwork resource is tied to a specific region. Defaults to the region of the Provider if no value is given."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_manager_tags`.\nA map of resource manager tags.\n\t\t\t\tResource manager tag keys and values have the same definition as resource manager tags.\n\t\t\t\tKeys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456.\n\t\t\t\tThe field is ignored (both PUT & PATCH) when empty."]
    pub fn set_resource_manager_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().resource_manager_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_policies`.\nA list of self_links of resource policies to attach to the instance. Currently a max of 1 resource policy is supported."]
    pub fn set_resource_policies(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().resource_policies = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\nTags to attach to the instance."]
    pub fn set_tags(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `advanced_machine_features`.\n"]
    pub fn set_advanced_machine_features(
        self,
        v: impl Into<BlockAssignable<ComputeInstanceTemplateAdvancedMachineFeaturesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().advanced_machine_features = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.advanced_machine_features = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `confidential_instance_config`.\n"]
    pub fn set_confidential_instance_config(
        self,
        v: impl Into<BlockAssignable<ComputeInstanceTemplateConfidentialInstanceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().confidential_instance_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .confidential_instance_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `disk`.\n"]
    pub fn set_disk(self, v: impl Into<BlockAssignable<ComputeInstanceTemplateDiskEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().disk = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.disk = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `guest_accelerator`.\n"]
    pub fn set_guest_accelerator(
        self,
        v: impl Into<BlockAssignable<ComputeInstanceTemplateGuestAcceleratorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().guest_accelerator = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.guest_accelerator = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_interface`.\n"]
    pub fn set_network_interface(
        self,
        v: impl Into<BlockAssignable<ComputeInstanceTemplateNetworkInterfaceEl>>,
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
    #[doc = "Set the field `network_performance_config`.\n"]
    pub fn set_network_performance_config(
        self,
        v: impl Into<BlockAssignable<ComputeInstanceTemplateNetworkPerformanceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_performance_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_performance_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `reservation_affinity`.\n"]
    pub fn set_reservation_affinity(
        self,
        v: impl Into<BlockAssignable<ComputeInstanceTemplateReservationAffinityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().reservation_affinity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.reservation_affinity = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `scheduling`.\n"]
    pub fn set_scheduling(
        self,
        v: impl Into<BlockAssignable<ComputeInstanceTemplateSchedulingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scheduling = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scheduling = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(
        self,
        v: impl Into<BlockAssignable<ComputeInstanceTemplateServiceAccountEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_account = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_account = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        self,
        v: impl Into<BlockAssignable<ComputeInstanceTemplateShieldedInstanceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().shielded_instance_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.shielded_instance_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeInstanceTemplateTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `can_ip_forward` after provisioning.\nWhether to allow sending and receiving of packets with non-matching source or destination IPs. This defaults to false."]
    pub fn can_ip_forward(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.can_ip_forward", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA brief description of this resource."]
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_description` after provisioning.\nA description of the instance."]
    pub fn instance_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `key_revocation_action_type` after provisioning.\nAction to be taken when a customer's encryption key is revoked. Supports \"STOP\" and \"NONE\", with \"NONE\" being the default."]
    pub fn key_revocation_action_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_revocation_action_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to instances created from this template.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nThe machine type to create. To create a machine with a custom type (such as extended memory), format the value like custom-VCPUS-MEM_IN_MB like custom-6-20480 for 6 vCPU and 20GB of RAM."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.machine_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nMetadata key/value pairs to make available from within instances created from this template."]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata_fingerprint` after provisioning.\nThe unique fingerprint of the metadata."]
    pub fn metadata_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metadata_fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata_startup_script` after provisioning.\nAn alternative to using the startup-script metadata key, mostly to match the compute_instance resource. This replaces the startup-script metadata key on the created instance and thus the two mechanisms are not allowed to be used simultaneously."]
    pub fn metadata_startup_script(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metadata_startup_script", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `min_cpu_platform` after provisioning.\nSpecifies a minimum CPU platform. Applicable values are the friendly names of CPU platforms, such as Intel Haswell or Intel Skylake."]
    pub fn min_cpu_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_cpu_platform", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the instance template. If you leave this blank, Terraform will auto-generate a unique name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\nCreates a unique name beginning with the specified prefix. Conflicts with name. Max length is 54 characters. Prefixes with lengths longer than 37 characters will use a shortened UUID that will be more prone to collisions."]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `numeric_id` after provisioning.\nThe ID of the template in numeric format."]
    pub fn numeric_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.numeric_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nAn instance template is a global resource that is not bound to a zone or a region. However, you can still specify some regional resources in an instance template, which restricts the template to the region where that resource resides. For example, a custom subnetwork resource is tied to a specific region. Defaults to the region of the Provider if no value is given."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\nA map of resource manager tags.\n\t\t\t\tResource manager tag keys and values have the same definition as resource manager tags.\n\t\t\t\tKeys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456.\n\t\t\t\tThe field is ignored (both PUT & PATCH) when empty."]
    pub fn resource_manager_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_manager_tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_policies` after provisioning.\nA list of self_links of resource policies to attach to the instance. Currently a max of 1 resource policy is supported."]
    pub fn resource_policies(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_policies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link_unique` after provisioning.\nA special URI of the created resource that uniquely identifies this instance template."]
    pub fn self_link_unique(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link_unique", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nTags to attach to the instance."]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_fingerprint` after provisioning.\nThe unique fingerprint of the tags."]
    pub fn tags_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tags_fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `advanced_machine_features` after provisioning.\n"]
    pub fn advanced_machine_features(
        &self,
    ) -> ListRef<ComputeInstanceTemplateAdvancedMachineFeaturesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_machine_features", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `confidential_instance_config` after provisioning.\n"]
    pub fn confidential_instance_config(
        &self,
    ) -> ListRef<ComputeInstanceTemplateConfidentialInstanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.confidential_instance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disk` after provisioning.\n"]
    pub fn disk(&self) -> ListRef<ComputeInstanceTemplateDiskElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `guest_accelerator` after provisioning.\n"]
    pub fn guest_accelerator(&self) -> ListRef<ComputeInstanceTemplateGuestAcceleratorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.guest_accelerator", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_interface` after provisioning.\n"]
    pub fn network_interface(&self) -> ListRef<ComputeInstanceTemplateNetworkInterfaceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_interface", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_performance_config` after provisioning.\n"]
    pub fn network_performance_config(
        &self,
    ) -> ListRef<ComputeInstanceTemplateNetworkPerformanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_performance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(&self) -> ListRef<ComputeInstanceTemplateReservationAffinityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reservation_affinity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scheduling` after provisioning.\n"]
    pub fn scheduling(&self) -> ListRef<ComputeInstanceTemplateSchedulingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scheduling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> ListRef<ComputeInstanceTemplateServiceAccountElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(
        &self,
    ) -> ListRef<ComputeInstanceTemplateShieldedInstanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.shielded_instance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeInstanceTemplateTimeoutsElRef {
        ComputeInstanceTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeInstanceTemplate {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeInstanceTemplate {}
impl ToListMappable for ComputeInstanceTemplate {
    type O = ListRef<ComputeInstanceTemplateRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeInstanceTemplate_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_instance_template".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeInstanceTemplate {
    pub tf_id: String,
    #[doc = "The machine type to create. To create a machine with a custom type (such as extended memory), format the value like custom-VCPUS-MEM_IN_MB like custom-6-20480 for 6 vCPU and 20GB of RAM."]
    pub machine_type: PrimField<String>,
}
impl BuildComputeInstanceTemplate {
    pub fn build(self, stack: &mut Stack) -> ComputeInstanceTemplate {
        let out = ComputeInstanceTemplate(Rc::new(ComputeInstanceTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeInstanceTemplateData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                can_ip_forward: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_description: core::default::Default::default(),
                key_revocation_action_type: core::default::Default::default(),
                labels: core::default::Default::default(),
                machine_type: self.machine_type,
                metadata: core::default::Default::default(),
                metadata_startup_script: core::default::Default::default(),
                min_cpu_platform: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                resource_manager_tags: core::default::Default::default(),
                resource_policies: core::default::Default::default(),
                tags: core::default::Default::default(),
                advanced_machine_features: core::default::Default::default(),
                confidential_instance_config: core::default::Default::default(),
                disk: core::default::Default::default(),
                guest_accelerator: core::default::Default::default(),
                network_interface: core::default::Default::default(),
                network_performance_config: core::default::Default::default(),
                reservation_affinity: core::default::Default::default(),
                scheduling: core::default::Default::default(),
                service_account: core::default::Default::default(),
                shielded_instance_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeInstanceTemplateRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceTemplateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeInstanceTemplateRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `can_ip_forward` after provisioning.\nWhether to allow sending and receiving of packets with non-matching source or destination IPs. This defaults to false."]
    pub fn can_ip_forward(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.can_ip_forward", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA brief description of this resource."]
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_description` after provisioning.\nA description of the instance."]
    pub fn instance_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `key_revocation_action_type` after provisioning.\nAction to be taken when a customer's encryption key is revoked. Supports \"STOP\" and \"NONE\", with \"NONE\" being the default."]
    pub fn key_revocation_action_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_revocation_action_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to instances created from this template.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nThe machine type to create. To create a machine with a custom type (such as extended memory), format the value like custom-VCPUS-MEM_IN_MB like custom-6-20480 for 6 vCPU and 20GB of RAM."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.machine_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nMetadata key/value pairs to make available from within instances created from this template."]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata_fingerprint` after provisioning.\nThe unique fingerprint of the metadata."]
    pub fn metadata_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metadata_fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata_startup_script` after provisioning.\nAn alternative to using the startup-script metadata key, mostly to match the compute_instance resource. This replaces the startup-script metadata key on the created instance and thus the two mechanisms are not allowed to be used simultaneously."]
    pub fn metadata_startup_script(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metadata_startup_script", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `min_cpu_platform` after provisioning.\nSpecifies a minimum CPU platform. Applicable values are the friendly names of CPU platforms, such as Intel Haswell or Intel Skylake."]
    pub fn min_cpu_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_cpu_platform", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the instance template. If you leave this blank, Terraform will auto-generate a unique name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\nCreates a unique name beginning with the specified prefix. Conflicts with name. Max length is 54 characters. Prefixes with lengths longer than 37 characters will use a shortened UUID that will be more prone to collisions."]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `numeric_id` after provisioning.\nThe ID of the template in numeric format."]
    pub fn numeric_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.numeric_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nAn instance template is a global resource that is not bound to a zone or a region. However, you can still specify some regional resources in an instance template, which restricts the template to the region where that resource resides. For example, a custom subnetwork resource is tied to a specific region. Defaults to the region of the Provider if no value is given."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\nA map of resource manager tags.\n\t\t\t\tResource manager tag keys and values have the same definition as resource manager tags.\n\t\t\t\tKeys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456.\n\t\t\t\tThe field is ignored (both PUT & PATCH) when empty."]
    pub fn resource_manager_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_manager_tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_policies` after provisioning.\nA list of self_links of resource policies to attach to the instance. Currently a max of 1 resource policy is supported."]
    pub fn resource_policies(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_policies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link_unique` after provisioning.\nA special URI of the created resource that uniquely identifies this instance template."]
    pub fn self_link_unique(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link_unique", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nTags to attach to the instance."]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_fingerprint` after provisioning.\nThe unique fingerprint of the tags."]
    pub fn tags_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tags_fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `advanced_machine_features` after provisioning.\n"]
    pub fn advanced_machine_features(
        &self,
    ) -> ListRef<ComputeInstanceTemplateAdvancedMachineFeaturesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_machine_features", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `confidential_instance_config` after provisioning.\n"]
    pub fn confidential_instance_config(
        &self,
    ) -> ListRef<ComputeInstanceTemplateConfidentialInstanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.confidential_instance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disk` after provisioning.\n"]
    pub fn disk(&self) -> ListRef<ComputeInstanceTemplateDiskElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `guest_accelerator` after provisioning.\n"]
    pub fn guest_accelerator(&self) -> ListRef<ComputeInstanceTemplateGuestAcceleratorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.guest_accelerator", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_interface` after provisioning.\n"]
    pub fn network_interface(&self) -> ListRef<ComputeInstanceTemplateNetworkInterfaceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_interface", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_performance_config` after provisioning.\n"]
    pub fn network_performance_config(
        &self,
    ) -> ListRef<ComputeInstanceTemplateNetworkPerformanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_performance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(&self) -> ListRef<ComputeInstanceTemplateReservationAffinityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reservation_affinity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scheduling` after provisioning.\n"]
    pub fn scheduling(&self) -> ListRef<ComputeInstanceTemplateSchedulingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scheduling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> ListRef<ComputeInstanceTemplateServiceAccountElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(
        &self,
    ) -> ListRef<ComputeInstanceTemplateShieldedInstanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.shielded_instance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeInstanceTemplateTimeoutsElRef {
        ComputeInstanceTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceTemplateAdvancedMachineFeaturesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_nested_virtualization: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_uefi_networking: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performance_monitoring_unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threads_per_core: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    turbo_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible_core_count: Option<PrimField<f64>>,
}
impl ComputeInstanceTemplateAdvancedMachineFeaturesEl {
    #[doc = "Set the field `enable_nested_virtualization`.\nWhether to enable nested virtualization or not."]
    pub fn set_enable_nested_virtualization(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_nested_virtualization = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_uefi_networking`.\nWhether to enable UEFI networking or not."]
    pub fn set_enable_uefi_networking(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_uefi_networking = Some(v.into());
        self
    }
    #[doc = "Set the field `performance_monitoring_unit`.\nThe PMU is a hardware component within the CPU core that monitors how the processor runs code. Valid values for the level of PMU are \"STANDARD\", \"ENHANCED\", and \"ARCHITECTURAL\"."]
    pub fn set_performance_monitoring_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.performance_monitoring_unit = Some(v.into());
        self
    }
    #[doc = "Set the field `threads_per_core`.\nThe number of threads per physical core. To disable simultaneous multithreading (SMT) set this to 1. If unset, the maximum number of threads supported per core by the underlying processor is assumed."]
    pub fn set_threads_per_core(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.threads_per_core = Some(v.into());
        self
    }
    #[doc = "Set the field `turbo_mode`.\nTurbo frequency mode to use for the instance. Currently supported modes is \"ALL_CORE_MAX\"."]
    pub fn set_turbo_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.turbo_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `visible_core_count`.\nThe number of physical cores to expose to an instance. Multiply by the number of threads per core to compute the total number of virtual CPUs to expose to the instance. If unset, the number of cores is inferred from the instance\\'s nominal CPU count and the underlying platform\\'s SMT width."]
    pub fn set_visible_core_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.visible_core_count = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInstanceTemplateAdvancedMachineFeaturesEl {
    type O = BlockAssignable<ComputeInstanceTemplateAdvancedMachineFeaturesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceTemplateAdvancedMachineFeaturesEl {}
impl BuildComputeInstanceTemplateAdvancedMachineFeaturesEl {
    pub fn build(self) -> ComputeInstanceTemplateAdvancedMachineFeaturesEl {
        ComputeInstanceTemplateAdvancedMachineFeaturesEl {
            enable_nested_virtualization: core::default::Default::default(),
            enable_uefi_networking: core::default::Default::default(),
            performance_monitoring_unit: core::default::Default::default(),
            threads_per_core: core::default::Default::default(),
            turbo_mode: core::default::Default::default(),
            visible_core_count: core::default::Default::default(),
        }
    }
}
pub struct ComputeInstanceTemplateAdvancedMachineFeaturesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceTemplateAdvancedMachineFeaturesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInstanceTemplateAdvancedMachineFeaturesElRef {
        ComputeInstanceTemplateAdvancedMachineFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceTemplateAdvancedMachineFeaturesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_nested_virtualization` after provisioning.\nWhether to enable nested virtualization or not."]
    pub fn enable_nested_virtualization(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_nested_virtualization", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_uefi_networking` after provisioning.\nWhether to enable UEFI networking or not."]
    pub fn enable_uefi_networking(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_uefi_networking", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `performance_monitoring_unit` after provisioning.\nThe PMU is a hardware component within the CPU core that monitors how the processor runs code. Valid values for the level of PMU are \"STANDARD\", \"ENHANCED\", and \"ARCHITECTURAL\"."]
    pub fn performance_monitoring_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.performance_monitoring_unit", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `threads_per_core` after provisioning.\nThe number of threads per physical core. To disable simultaneous multithreading (SMT) set this to 1. If unset, the maximum number of threads supported per core by the underlying processor is assumed."]
    pub fn threads_per_core(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.threads_per_core", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `turbo_mode` after provisioning.\nTurbo frequency mode to use for the instance. Currently supported modes is \"ALL_CORE_MAX\"."]
    pub fn turbo_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.turbo_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `visible_core_count` after provisioning.\nThe number of physical cores to expose to an instance. Multiply by the number of threads per core to compute the total number of virtual CPUs to expose to the instance. If unset, the number of cores is inferred from the instance\\'s nominal CPU count and the underlying platform\\'s SMT width."]
    pub fn visible_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.visible_core_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceTemplateConfidentialInstanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    confidential_instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_confidential_compute: Option<PrimField<bool>>,
}
impl ComputeInstanceTemplateConfidentialInstanceConfigEl {
    #[doc = "Set the field `confidential_instance_type`.\n\n\t\t\t\t\t\t\t\tThe confidential computing technology the instance uses.\n\t\t\t\t\t\t\t\tSEV is an AMD feature. TDX is an Intel feature. One of the following\n\t\t\t\t\t\t\t\tvalues is required: SEV, SEV_SNP, TDX. If SEV_SNP, min_cpu_platform =\n\t\t\t\t\t\t\t\t\"AMD Milan\" is currently required."]
    pub fn set_confidential_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.confidential_instance_type = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_confidential_compute`.\nDefines whether the instance should have confidential compute enabled. Field will be deprecated in a future release."]
    pub fn set_enable_confidential_compute(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_confidential_compute = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInstanceTemplateConfidentialInstanceConfigEl {
    type O = BlockAssignable<ComputeInstanceTemplateConfidentialInstanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceTemplateConfidentialInstanceConfigEl {}
impl BuildComputeInstanceTemplateConfidentialInstanceConfigEl {
    pub fn build(self) -> ComputeInstanceTemplateConfidentialInstanceConfigEl {
        ComputeInstanceTemplateConfidentialInstanceConfigEl {
            confidential_instance_type: core::default::Default::default(),
            enable_confidential_compute: core::default::Default::default(),
        }
    }
}
pub struct ComputeInstanceTemplateConfidentialInstanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceTemplateConfidentialInstanceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInstanceTemplateConfidentialInstanceConfigElRef {
        ComputeInstanceTemplateConfidentialInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceTemplateConfidentialInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `confidential_instance_type` after provisioning.\n\n\t\t\t\t\t\t\t\tThe confidential computing technology the instance uses.\n\t\t\t\t\t\t\t\tSEV is an AMD feature. TDX is an Intel feature. One of the following\n\t\t\t\t\t\t\t\tvalues is required: SEV, SEV_SNP, TDX. If SEV_SNP, min_cpu_platform =\n\t\t\t\t\t\t\t\t\"AMD Milan\" is currently required."]
    pub fn confidential_instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.confidential_instance_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_confidential_compute` after provisioning.\nDefines whether the instance should have confidential compute enabled. Field will be deprecated in a future release."]
    pub fn enable_confidential_compute(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_confidential_compute", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceTemplateDiskElDiskEncryptionKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_service_account: Option<PrimField<String>>,
}
impl ComputeInstanceTemplateDiskElDiskEncryptionKeyEl {
    #[doc = "Set the field `kms_key_self_link`.\nThe self link of the encryption key that is stored in Google Cloud KMS."]
    pub fn set_kms_key_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_self_link = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_service_account`.\nThe service account being used for the encryption request for the given KMS key. If absent, the Compute Engine default service account is used."]
    pub fn set_kms_key_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_service_account = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInstanceTemplateDiskElDiskEncryptionKeyEl {
    type O = BlockAssignable<ComputeInstanceTemplateDiskElDiskEncryptionKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceTemplateDiskElDiskEncryptionKeyEl {}
impl BuildComputeInstanceTemplateDiskElDiskEncryptionKeyEl {
    pub fn build(self) -> ComputeInstanceTemplateDiskElDiskEncryptionKeyEl {
        ComputeInstanceTemplateDiskElDiskEncryptionKeyEl {
            kms_key_self_link: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
        }
    }
}
pub struct ComputeInstanceTemplateDiskElDiskEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceTemplateDiskElDiskEncryptionKeyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInstanceTemplateDiskElDiskEncryptionKeyElRef {
        ComputeInstanceTemplateDiskElDiskEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceTemplateDiskElDiskEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_self_link` after provisioning.\nThe self link of the encryption key that is stored in Google Cloud KMS."]
    pub fn kms_key_self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_self_link", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_service_account` after provisioning.\nThe service account being used for the encryption request for the given KMS key. If absent, the Compute Engine default service account is used."]
    pub fn kms_key_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_service_account", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rsa_encrypted_key: Option<PrimField<String>>,
}
impl ComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl {
    #[doc = "Set the field `kms_key_self_link`.\nThe self link of the encryption key that is stored in\nGoogle Cloud KMS. Only one of kms_key_self_link, rsa_encrypted_key and raw_key may be set."]
    pub fn set_kms_key_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_self_link = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_service_account`.\nThe service account being used for the encryption\nrequest for the given KMS key. If absent, the Compute\nEngine default service account is used."]
    pub fn set_kms_key_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `raw_key`.\nSpecifies a 256-bit customer-supplied encryption key, encoded in RFC 4648 base64 to either encrypt or decrypt this resource. Only one of kms_key_self_link, rsa_encrypted_key and raw_key may be set."]
    pub fn set_raw_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.raw_key = Some(v.into());
        self
    }
    #[doc = "Set the field `rsa_encrypted_key`.\nSpecifies an RFC 4648 base64 encoded, RSA-wrapped 2048-bit customer-supplied encryption key to either encrypt or decrypt this resource. Only one of kms_key_self_link, rsa_encrypted_key and raw_key may be set."]
    pub fn set_rsa_encrypted_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rsa_encrypted_key = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl {
    type O = BlockAssignable<ComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl {}
impl BuildComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl {
    pub fn build(self) -> ComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl {
        ComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl {
            kms_key_self_link: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
            rsa_encrypted_key: core::default::Default::default(),
        }
    }
}
pub struct ComputeInstanceTemplateDiskElSourceImageEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceTemplateDiskElSourceImageEncryptionKeyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInstanceTemplateDiskElSourceImageEncryptionKeyElRef {
        ComputeInstanceTemplateDiskElSourceImageEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceTemplateDiskElSourceImageEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_self_link` after provisioning.\nThe self link of the encryption key that is stored in\nGoogle Cloud KMS. Only one of kms_key_self_link, rsa_encrypted_key and raw_key may be set."]
    pub fn kms_key_self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_self_link", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_service_account` after provisioning.\nThe service account being used for the encryption\nrequest for the given KMS key. If absent, the Compute\nEngine default service account is used."]
    pub fn kms_key_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `raw_key` after provisioning.\nSpecifies a 256-bit customer-supplied encryption key, encoded in RFC 4648 base64 to either encrypt or decrypt this resource. Only one of kms_key_self_link, rsa_encrypted_key and raw_key may be set."]
    pub fn raw_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.raw_key", self.base))
    }
    #[doc = "Get a reference to the value of field `rsa_encrypted_key` after provisioning.\nSpecifies an RFC 4648 base64 encoded, RSA-wrapped 2048-bit customer-supplied encryption key to either encrypt or decrypt this resource. Only one of kms_key_self_link, rsa_encrypted_key and raw_key may be set."]
    pub fn rsa_encrypted_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rsa_encrypted_key", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rsa_encrypted_key: Option<PrimField<String>>,
}
impl ComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {
    #[doc = "Set the field `kms_key_self_link`.\nThe self link of the encryption key that is stored in\nGoogle Cloud KMS. Only one of kms_key_self_link, rsa_encrypted_key and raw_key may be set."]
    pub fn set_kms_key_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_self_link = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_service_account`.\nThe service account being used for the encryption\nrequest for the given KMS key. If absent, the Compute\nEngine default service account is used."]
    pub fn set_kms_key_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `raw_key`.\nSpecifies a 256-bit customer-supplied encryption key, encoded in RFC 4648 base64 to either encrypt or decrypt this resource. Only one of kms_key_self_link, rsa_encrypted_key and raw_key may be set."]
    pub fn set_raw_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.raw_key = Some(v.into());
        self
    }
    #[doc = "Set the field `rsa_encrypted_key`.\nSpecifies an RFC 4648 base64 encoded, RSA-wrapped 2048-bit customer-supplied encryption key to either encrypt or decrypt this resource. Only one of kms_key_self_link, rsa_encrypted_key and raw_key may be set."]
    pub fn set_rsa_encrypted_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rsa_encrypted_key = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {
    type O = BlockAssignable<ComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {}
impl BuildComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {
    pub fn build(self) -> ComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {
        ComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {
            kms_key_self_link: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
            rsa_encrypted_key: core::default::Default::default(),
        }
    }
}
pub struct ComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyElRef {
        ComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_self_link` after provisioning.\nThe self link of the encryption key that is stored in\nGoogle Cloud KMS. Only one of kms_key_self_link, rsa_encrypted_key and raw_key may be set."]
    pub fn kms_key_self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_self_link", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_service_account` after provisioning.\nThe service account being used for the encryption\nrequest for the given KMS key. If absent, the Compute\nEngine default service account is used."]
    pub fn kms_key_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `raw_key` after provisioning.\nSpecifies a 256-bit customer-supplied encryption key, encoded in RFC 4648 base64 to either encrypt or decrypt this resource. Only one of kms_key_self_link, rsa_encrypted_key and raw_key may be set."]
    pub fn raw_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.raw_key", self.base))
    }
    #[doc = "Get a reference to the value of field `rsa_encrypted_key` after provisioning.\nSpecifies an RFC 4648 base64 encoded, RSA-wrapped 2048-bit customer-supplied encryption key to either encrypt or decrypt this resource. Only one of kms_key_self_link, rsa_encrypted_key and raw_key may be set."]
    pub fn rsa_encrypted_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rsa_encrypted_key", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ComputeInstanceTemplateDiskElDynamic {
    disk_encryption_key: Option<DynamicBlock<ComputeInstanceTemplateDiskElDiskEncryptionKeyEl>>,
    source_image_encryption_key:
        Option<DynamicBlock<ComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl>>,
    source_snapshot_encryption_key:
        Option<DynamicBlock<ComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl>>,
}
#[derive(Serialize)]
pub struct ComputeInstanceTemplateDiskEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    architecture: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_delete: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_os_features: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interface: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioned_iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioned_throughput: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_policies: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_snapshot: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_pool: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_encryption_key: Option<Vec<ComputeInstanceTemplateDiskElDiskEncryptionKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_image_encryption_key:
        Option<Vec<ComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_snapshot_encryption_key:
        Option<Vec<ComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl>>,
    dynamic: ComputeInstanceTemplateDiskElDynamic,
}
impl ComputeInstanceTemplateDiskEl {
    #[doc = "Set the field `architecture`.\nThe architecture of the image. Allowed values are ARM64 or X86_64."]
    pub fn set_architecture(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.architecture = Some(v.into());
        self
    }
    #[doc = "Set the field `auto_delete`.\nWhether or not the disk should be auto-deleted. This defaults to true."]
    pub fn set_auto_delete(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_delete = Some(v.into());
        self
    }
    #[doc = "Set the field `boot`.\nIndicates that this is a boot disk."]
    pub fn set_boot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.boot = Some(v.into());
        self
    }
    #[doc = "Set the field `device_name`.\nA unique device name that is reflected into the /dev/ tree of a Linux operating system running within the instance. If not specified, the server chooses a default device name to apply to this disk."]
    pub fn set_device_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device_name = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_name`.\nName of the disk. When not provided, this defaults to the name of the instance."]
    pub fn set_disk_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_name = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_size_gb`.\nThe size of the image in gigabytes. If not specified, it will inherit the size of its base image. For SCRATCH disks, the size must be one of 375 or 3000 GB, with a default of 375 GB."]
    pub fn set_disk_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_type`.\nThe Google Compute Engine disk type. Such as \"pd-ssd\", \"local-ssd\", \"pd-balanced\" or \"pd-standard\"."]
    pub fn set_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_type = Some(v.into());
        self
    }
    #[doc = "Set the field `guest_os_features`.\nA list of features to enable on the guest operating system. Applicable only for bootable images."]
    pub fn set_guest_os_features(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.guest_os_features = Some(v.into());
        self
    }
    #[doc = "Set the field `interface`.\nSpecifies the disk interface to use for attaching this disk."]
    pub fn set_interface(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interface = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nA set of key/value label pairs to assign to disks,"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `mode`.\nThe mode in which to attach this disk, either READ_WRITE or READ_ONLY. If you are attaching or creating a boot disk, this must read-write mode."]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
    #[doc = "Set the field `provisioned_iops`.\nIndicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle. For more details, see the [Extreme persistent disk documentation](https://cloud.google.com/compute/docs/disks/extreme-persistent-disk) or the [Hyperdisk documentation](https://cloud.google.com/compute/docs/disks/hyperdisks) depending on the selected disk_type."]
    pub fn set_provisioned_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.provisioned_iops = Some(v.into());
        self
    }
    #[doc = "Set the field `provisioned_throughput`.\nIndicates how much throughput to provision for the disk, in MB/s. This sets the amount of data that can be read or written from the disk per second. Values must greater than or equal to 1. For more details, see the [Hyperdisk documentation](https://cloud.google.com/compute/docs/disks/hyperdisks)."]
    pub fn set_provisioned_throughput(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.provisioned_throughput = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_manager_tags`.\nA map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored (both PUT & PATCH) when empty."]
    pub fn set_resource_manager_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_manager_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_policies`.\nA list (short name or id) of resource policies to attach to this disk. Currently a max of 1 resource policy is supported."]
    pub fn set_resource_policies(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.resource_policies = Some(v.into());
        self
    }
    #[doc = "Set the field `source`.\nThe name (not self_link) of the disk (such as those managed by google_compute_disk) to attach. ~> Note: Either source or source_image is required when creating a new instance except for when creating a local SSD."]
    pub fn set_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source = Some(v.into());
        self
    }
    #[doc = "Set the field `source_image`.\nThe image from which to initialize this disk. This can be one of: the image's self_link, projects/{project}/global/images/{image}, projects/{project}/global/images/family/{family}, global/images/{image}, global/images/family/{family}, family/{family}, {project}/{family}, {project}/{image}, {family}, or {image}. ~> Note: Either source or source_image is required when creating a new instance except for when creating a local SSD."]
    pub fn set_source_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_image = Some(v.into());
        self
    }
    #[doc = "Set the field `source_snapshot`.\nThe source snapshot to create this disk. When creating\na new instance, one of initializeParams.sourceSnapshot,\ninitializeParams.sourceImage, or disks.source is\nrequired except for local SSD."]
    pub fn set_source_snapshot(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_snapshot = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_pool`.\nThe self_link or ID of the Storage Pool to create this disk in."]
    pub fn set_storage_pool(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.storage_pool = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nThe type of Google Compute Engine disk, can be either \"SCRATCH\" or \"PERSISTENT\"."]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_encryption_key`.\n"]
    pub fn set_disk_encryption_key(
        mut self,
        v: impl Into<BlockAssignable<ComputeInstanceTemplateDiskElDiskEncryptionKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.disk_encryption_key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.disk_encryption_key = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source_image_encryption_key`.\n"]
    pub fn set_source_image_encryption_key(
        mut self,
        v: impl Into<BlockAssignable<ComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_image_encryption_key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_image_encryption_key = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source_snapshot_encryption_key`.\n"]
    pub fn set_source_snapshot_encryption_key(
        mut self,
        v: impl Into<BlockAssignable<ComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_snapshot_encryption_key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_snapshot_encryption_key = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeInstanceTemplateDiskEl {
    type O = BlockAssignable<ComputeInstanceTemplateDiskEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceTemplateDiskEl {}
impl BuildComputeInstanceTemplateDiskEl {
    pub fn build(self) -> ComputeInstanceTemplateDiskEl {
        ComputeInstanceTemplateDiskEl {
            architecture: core::default::Default::default(),
            auto_delete: core::default::Default::default(),
            boot: core::default::Default::default(),
            device_name: core::default::Default::default(),
            disk_name: core::default::Default::default(),
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            guest_os_features: core::default::Default::default(),
            interface: core::default::Default::default(),
            labels: core::default::Default::default(),
            mode: core::default::Default::default(),
            provisioned_iops: core::default::Default::default(),
            provisioned_throughput: core::default::Default::default(),
            resource_manager_tags: core::default::Default::default(),
            resource_policies: core::default::Default::default(),
            source: core::default::Default::default(),
            source_image: core::default::Default::default(),
            source_snapshot: core::default::Default::default(),
            storage_pool: core::default::Default::default(),
            type_: core::default::Default::default(),
            disk_encryption_key: core::default::Default::default(),
            source_image_encryption_key: core::default::Default::default(),
            source_snapshot_encryption_key: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeInstanceTemplateDiskElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceTemplateDiskElRef {
    fn new(shared: StackShared, base: String) -> ComputeInstanceTemplateDiskElRef {
        ComputeInstanceTemplateDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceTemplateDiskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `architecture` after provisioning.\nThe architecture of the image. Allowed values are ARM64 or X86_64."]
    pub fn architecture(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.base))
    }
    #[doc = "Get a reference to the value of field `auto_delete` after provisioning.\nWhether or not the disk should be auto-deleted. This defaults to true."]
    pub fn auto_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_delete", self.base))
    }
    #[doc = "Get a reference to the value of field `boot` after provisioning.\nIndicates that this is a boot disk."]
    pub fn boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.boot", self.base))
    }
    #[doc = "Get a reference to the value of field `device_name` after provisioning.\nA unique device name that is reflected into the /dev/ tree of a Linux operating system running within the instance. If not specified, the server chooses a default device name to apply to this disk."]
    pub fn device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }
    #[doc = "Get a reference to the value of field `disk_name` after provisioning.\nName of the disk. When not provided, this defaults to the name of the instance."]
    pub fn disk_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_name", self.base))
    }
    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\nThe size of the image in gigabytes. If not specified, it will inherit the size of its base image. For SCRATCH disks, the size must be one of 375 or 3000 GB, with a default of 375 GB."]
    pub fn disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `disk_type` after provisioning.\nThe Google Compute Engine disk type. Such as \"pd-ssd\", \"local-ssd\", \"pd-balanced\" or \"pd-standard\"."]
    pub fn disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }
    #[doc = "Get a reference to the value of field `guest_os_features` after provisioning.\nA list of features to enable on the guest operating system. Applicable only for bootable images."]
    pub fn guest_os_features(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.guest_os_features", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `interface` after provisioning.\nSpecifies the disk interface to use for attaching this disk."]
    pub fn interface(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interface", self.base))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to disks,"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nThe mode in which to attach this disk, either READ_WRITE or READ_ONLY. If you are attaching or creating a boot disk, this must read-write mode."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `provisioned_iops` after provisioning.\nIndicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle. For more details, see the [Extreme persistent disk documentation](https://cloud.google.com/compute/docs/disks/extreme-persistent-disk) or the [Hyperdisk documentation](https://cloud.google.com/compute/docs/disks/hyperdisks) depending on the selected disk_type."]
    pub fn provisioned_iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_iops", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `provisioned_throughput` after provisioning.\nIndicates how much throughput to provision for the disk, in MB/s. This sets the amount of data that can be read or written from the disk per second. Values must greater than or equal to 1. For more details, see the [Hyperdisk documentation](https://cloud.google.com/compute/docs/disks/hyperdisks)."]
    pub fn provisioned_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_throughput", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\nA map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored (both PUT & PATCH) when empty."]
    pub fn resource_manager_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_manager_tags", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_policies` after provisioning.\nA list (short name or id) of resource policies to attach to this disk. Currently a max of 1 resource policy is supported."]
    pub fn resource_policies(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_policies", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\nThe name (not self_link) of the disk (such as those managed by google_compute_disk) to attach. ~> Note: Either source or source_image is required when creating a new instance except for when creating a local SSD."]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
    #[doc = "Get a reference to the value of field `source_image` after provisioning.\nThe image from which to initialize this disk. This can be one of: the image's self_link, projects/{project}/global/images/{image}, projects/{project}/global/images/family/{family}, global/images/{image}, global/images/family/{family}, family/{family}, {project}/{family}, {project}/{image}, {family}, or {image}. ~> Note: Either source or source_image is required when creating a new instance except for when creating a local SSD."]
    pub fn source_image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_image", self.base))
    }
    #[doc = "Get a reference to the value of field `source_snapshot` after provisioning.\nThe source snapshot to create this disk. When creating\na new instance, one of initializeParams.sourceSnapshot,\ninitializeParams.sourceImage, or disks.source is\nrequired except for local SSD."]
    pub fn source_snapshot(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_snapshot", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `storage_pool` after provisioning.\nThe self_link or ID of the Storage Pool to create this disk in."]
    pub fn storage_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_pool", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of Google Compute Engine disk, can be either \"SCRATCH\" or \"PERSISTENT\"."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `disk_encryption_key` after provisioning.\n"]
    pub fn disk_encryption_key(
        &self,
    ) -> ListRef<ComputeInstanceTemplateDiskElDiskEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk_encryption_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_image_encryption_key` after provisioning.\n"]
    pub fn source_image_encryption_key(
        &self,
    ) -> ListRef<ComputeInstanceTemplateDiskElSourceImageEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_image_encryption_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_snapshot_encryption_key` after provisioning.\n"]
    pub fn source_snapshot_encryption_key(
        &self,
    ) -> ListRef<ComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_snapshot_encryption_key", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceTemplateGuestAcceleratorEl {
    count: PrimField<f64>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl ComputeInstanceTemplateGuestAcceleratorEl {}
impl ToListMappable for ComputeInstanceTemplateGuestAcceleratorEl {
    type O = BlockAssignable<ComputeInstanceTemplateGuestAcceleratorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceTemplateGuestAcceleratorEl {
    #[doc = "The number of the guest accelerator cards exposed to this instance."]
    pub count: PrimField<f64>,
    #[doc = "The accelerator type resource to expose to this instance. E.g. nvidia-tesla-k80."]
    pub type_: PrimField<String>,
}
impl BuildComputeInstanceTemplateGuestAcceleratorEl {
    pub fn build(self) -> ComputeInstanceTemplateGuestAcceleratorEl {
        ComputeInstanceTemplateGuestAcceleratorEl {
            count: self.count,
            type_: self.type_,
        }
    }
}
pub struct ComputeInstanceTemplateGuestAcceleratorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceTemplateGuestAcceleratorElRef {
    fn new(shared: StackShared, base: String) -> ComputeInstanceTemplateGuestAcceleratorElRef {
        ComputeInstanceTemplateGuestAcceleratorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceTemplateGuestAcceleratorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\nThe number of the guest accelerator cards exposed to this instance."]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe accelerator type resource to expose to this instance. E.g. nvidia-tesla-k80."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceTemplateNetworkInterfaceElAccessConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nat_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_tier: Option<PrimField<String>>,
}
impl ComputeInstanceTemplateNetworkInterfaceElAccessConfigEl {
    #[doc = "Set the field `nat_ip`.\nThe IP address that will be 1:1 mapped to the instance's network ip. If not given, one will be generated."]
    pub fn set_nat_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nat_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `network_tier`.\nThe networking tier used for configuring this instance template. This field can take the following values: PREMIUM, STANDARD, FIXED_STANDARD. If this field is not specified, it is assumed to be PREMIUM."]
    pub fn set_network_tier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_tier = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInstanceTemplateNetworkInterfaceElAccessConfigEl {
    type O = BlockAssignable<ComputeInstanceTemplateNetworkInterfaceElAccessConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceTemplateNetworkInterfaceElAccessConfigEl {}
impl BuildComputeInstanceTemplateNetworkInterfaceElAccessConfigEl {
    pub fn build(self) -> ComputeInstanceTemplateNetworkInterfaceElAccessConfigEl {
        ComputeInstanceTemplateNetworkInterfaceElAccessConfigEl {
            nat_ip: core::default::Default::default(),
            network_tier: core::default::Default::default(),
        }
    }
}
pub struct ComputeInstanceTemplateNetworkInterfaceElAccessConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceTemplateNetworkInterfaceElAccessConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInstanceTemplateNetworkInterfaceElAccessConfigElRef {
        ComputeInstanceTemplateNetworkInterfaceElAccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceTemplateNetworkInterfaceElAccessConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nat_ip` after provisioning.\nThe IP address that will be 1:1 mapped to the instance's network ip. If not given, one will be generated."]
    pub fn nat_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nat_ip", self.base))
    }
    #[doc = "Get a reference to the value of field `network_tier` after provisioning.\nThe networking tier used for configuring this instance template. This field can take the following values: PREMIUM, STANDARD, FIXED_STANDARD. If this field is not specified, it is assumed to be PREMIUM."]
    pub fn network_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_tier", self.base))
    }
    #[doc = "Get a reference to the value of field `public_ptr_domain_name` after provisioning.\nThe DNS domain name for the public PTR record.The DNS domain name for the public PTR record."]
    pub fn public_ptr_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_ptr_domain_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl {
    ip_cidr_range: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork_range_name: Option<PrimField<String>>,
}
impl ComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl {
    #[doc = "Set the field `subnetwork_range_name`.\nThe subnetwork secondary range name specifying the secondary range from which to allocate the IP CIDR range for this alias IP range. If left unspecified, the primary range of the subnetwork will be used."]
    pub fn set_subnetwork_range_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork_range_name = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl {
    type O = BlockAssignable<ComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl {
    #[doc = "The IP CIDR range represented by this alias IP range. This IP CIDR range must belong to the specified subnetwork and cannot contain IP addresses reserved by system or used by other network interfaces. At the time of writing only a netmask (e.g. /24) may be supplied, with a CIDR format resulting in an API error."]
    pub ip_cidr_range: PrimField<String>,
}
impl BuildComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl {
    pub fn build(self) -> ComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl {
        ComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl {
            ip_cidr_range: self.ip_cidr_range,
            subnetwork_range_name: core::default::Default::default(),
        }
    }
}
pub struct ComputeInstanceTemplateNetworkInterfaceElAliasIpRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceTemplateNetworkInterfaceElAliasIpRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInstanceTemplateNetworkInterfaceElAliasIpRangeElRef {
        ComputeInstanceTemplateNetworkInterfaceElAliasIpRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceTemplateNetworkInterfaceElAliasIpRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_cidr_range` after provisioning.\nThe IP CIDR range represented by this alias IP range. This IP CIDR range must belong to the specified subnetwork and cannot contain IP addresses reserved by system or used by other network interfaces. At the time of writing only a netmask (e.g. /24) may be supplied, with a CIDR format resulting in an API error."]
    pub fn ip_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_cidr_range", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnetwork_range_name` after provisioning.\nThe subnetwork secondary range name specifying the secondary range from which to allocate the IP CIDR range for this alias IP range. If left unspecified, the primary range of the subnetwork will be used."]
    pub fn subnetwork_range_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnetwork_range_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl {
    network_tier: PrimField<String>,
}
impl ComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl {}
impl ToListMappable for ComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl {
    type O = BlockAssignable<ComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl {
    #[doc = "The service-level to be provided for IPv6 traffic when the subnet has an external subnet. Only PREMIUM tier is valid for IPv6"]
    pub network_tier: PrimField<String>,
}
impl BuildComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl {
    pub fn build(self) -> ComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl {
        ComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl {
            network_tier: self.network_tier,
        }
    }
}
pub struct ComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigElRef {
        ComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `external_ipv6` after provisioning.\nThe first IPv6 address of the external IPv6 range associated with this instance, prefix length is stored in externalIpv6PrefixLength in ipv6AccessConfig. The field is output only, an IPv6 address from a subnetwork associated with the instance will be allocated dynamically."]
    pub fn external_ipv6(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_ipv6", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `external_ipv6_prefix_length` after provisioning.\nThe prefix length of the external IPv6 range."]
    pub fn external_ipv6_prefix_length(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_ipv6_prefix_length", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of this access configuration."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `network_tier` after provisioning.\nThe service-level to be provided for IPv6 traffic when the subnet has an external subnet. Only PREMIUM tier is valid for IPv6"]
    pub fn network_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_tier", self.base))
    }
    #[doc = "Get a reference to the value of field `public_ptr_domain_name` after provisioning.\nThe domain name to be used when creating DNSv6 records for the external IPv6 ranges."]
    pub fn public_ptr_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_ptr_domain_name", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ComputeInstanceTemplateNetworkInterfaceElDynamic {
    access_config: Option<DynamicBlock<ComputeInstanceTemplateNetworkInterfaceElAccessConfigEl>>,
    alias_ip_range: Option<DynamicBlock<ComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl>>,
    ipv6_access_config:
        Option<DynamicBlock<ComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl>>,
}
#[derive(Serialize)]
pub struct ComputeInstanceTemplateNetworkInterfaceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    igmp_query: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_ipv6_prefix_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_attachment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nic_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queue_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stack_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork_project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vlan: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_config: Option<Vec<ComputeInstanceTemplateNetworkInterfaceElAccessConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alias_ip_range: Option<Vec<ComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_access_config: Option<Vec<ComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl>>,
    dynamic: ComputeInstanceTemplateNetworkInterfaceElDynamic,
}
impl ComputeInstanceTemplateNetworkInterfaceEl {
    #[doc = "Set the field `igmp_query`.\nIndicates whether igmp query is enabled on the network interface or not. If enabled, also indicates the version of IGMP supported."]
    pub fn set_igmp_query(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.igmp_query = Some(v.into());
        self
    }
    #[doc = "Set the field `internal_ipv6_prefix_length`.\nThe prefix length of the primary internal IPv6 range."]
    pub fn set_internal_ipv6_prefix_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.internal_ipv6_prefix_length = Some(v.into());
        self
    }
    #[doc = "Set the field `ipv6_address`.\nAn IPv6 internal network address for this network interface. If not specified, Google Cloud will automatically assign an internal IPv6 address from the instance's subnetwork."]
    pub fn set_ipv6_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv6_address = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nThe name or self_link of the network to attach this interface to. Use network attribute for Legacy or Auto subnetted networks and subnetwork for custom subnetted networks."]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `network_attachment`.\nThe URL of the network attachment that this interface should connect to in the following format: projects/{projectNumber}/regions/{region_name}/networkAttachments/{network_attachment_name}."]
    pub fn set_network_attachment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_attachment = Some(v.into());
        self
    }
    #[doc = "Set the field `network_ip`.\nThe private IP address to assign to the instance. If empty, the address will be automatically assigned."]
    pub fn set_network_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `nic_type`.\nThe type of vNIC to be used on this interface. Possible values:GVNIC, VIRTIO_NET, MRDMA, IRDMA and IDPF"]
    pub fn set_nic_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nic_type = Some(v.into());
        self
    }
    #[doc = "Set the field `queue_count`.\nThe networking queue count that's specified by users for the network interface. Both Rx and Tx queues will be set to this number. It will be empty if not specified."]
    pub fn set_queue_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.queue_count = Some(v.into());
        self
    }
    #[doc = "Set the field `stack_type`.\nThe stack type for this network interface to identify whether the IPv6 feature is enabled or not. If not specified, IPV4_ONLY will be used."]
    pub fn set_stack_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stack_type = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\nThe name of the subnetwork to attach this interface to. The subnetwork must exist in the same region this instance will be created in. Either network or subnetwork must be provided."]
    pub fn set_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork_project`.\nThe ID of the project in which the subnetwork belongs. If it is not provided, the provider project is used."]
    pub fn set_subnetwork_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork_project = Some(v.into());
        self
    }
    #[doc = "Set the field `vlan`.\nVLAN tag of a dynamic network interface, must be an integer in the range from 2 to 255 inclusively."]
    pub fn set_vlan(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.vlan = Some(v.into());
        self
    }
    #[doc = "Set the field `access_config`.\n"]
    pub fn set_access_config(
        mut self,
        v: impl Into<BlockAssignable<ComputeInstanceTemplateNetworkInterfaceElAccessConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.access_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.access_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `alias_ip_range`.\n"]
    pub fn set_alias_ip_range(
        mut self,
        v: impl Into<BlockAssignable<ComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.alias_ip_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.alias_ip_range = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ipv6_access_config`.\n"]
    pub fn set_ipv6_access_config(
        mut self,
        v: impl Into<BlockAssignable<ComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ipv6_access_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ipv6_access_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeInstanceTemplateNetworkInterfaceEl {
    type O = BlockAssignable<ComputeInstanceTemplateNetworkInterfaceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceTemplateNetworkInterfaceEl {}
impl BuildComputeInstanceTemplateNetworkInterfaceEl {
    pub fn build(self) -> ComputeInstanceTemplateNetworkInterfaceEl {
        ComputeInstanceTemplateNetworkInterfaceEl {
            igmp_query: core::default::Default::default(),
            internal_ipv6_prefix_length: core::default::Default::default(),
            ipv6_address: core::default::Default::default(),
            network: core::default::Default::default(),
            network_attachment: core::default::Default::default(),
            network_ip: core::default::Default::default(),
            nic_type: core::default::Default::default(),
            queue_count: core::default::Default::default(),
            stack_type: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            subnetwork_project: core::default::Default::default(),
            vlan: core::default::Default::default(),
            access_config: core::default::Default::default(),
            alias_ip_range: core::default::Default::default(),
            ipv6_access_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeInstanceTemplateNetworkInterfaceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceTemplateNetworkInterfaceElRef {
    fn new(shared: StackShared, base: String) -> ComputeInstanceTemplateNetworkInterfaceElRef {
        ComputeInstanceTemplateNetworkInterfaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceTemplateNetworkInterfaceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `igmp_query` after provisioning.\nIndicates whether igmp query is enabled on the network interface or not. If enabled, also indicates the version of IGMP supported."]
    pub fn igmp_query(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.igmp_query", self.base))
    }
    #[doc = "Get a reference to the value of field `internal_ipv6_prefix_length` after provisioning.\nThe prefix length of the primary internal IPv6 range."]
    pub fn internal_ipv6_prefix_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.internal_ipv6_prefix_length", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_access_type` after provisioning.\nOne of EXTERNAL, INTERNAL to indicate whether the IP can be accessed from the Internet. This field is always inherited from its subnetwork."]
    pub fn ipv6_access_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_access_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_address` after provisioning.\nAn IPv6 internal network address for this network interface. If not specified, Google Cloud will automatically assign an internal IPv6 address from the instance's subnetwork."]
    pub fn ipv6_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the network_interface."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe name or self_link of the network to attach this interface to. Use network attribute for Legacy or Auto subnetted networks and subnetwork for custom subnetted networks."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `network_attachment` after provisioning.\nThe URL of the network attachment that this interface should connect to in the following format: projects/{projectNumber}/regions/{region_name}/networkAttachments/{network_attachment_name}."]
    pub fn network_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_attachment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network_ip` after provisioning.\nThe private IP address to assign to the instance. If empty, the address will be automatically assigned."]
    pub fn network_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_ip", self.base))
    }
    #[doc = "Get a reference to the value of field `nic_type` after provisioning.\nThe type of vNIC to be used on this interface. Possible values:GVNIC, VIRTIO_NET, MRDMA, IRDMA and IDPF"]
    pub fn nic_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nic_type", self.base))
    }
    #[doc = "Get a reference to the value of field `parent_nic_name` after provisioning.\nName of the parent network interface of a dynamic network interface."]
    pub fn parent_nic_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_nic_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `queue_count` after provisioning.\nThe networking queue count that's specified by users for the network interface. Both Rx and Tx queues will be set to this number. It will be empty if not specified."]
    pub fn queue_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.queue_count", self.base))
    }
    #[doc = "Get a reference to the value of field `stack_type` after provisioning.\nThe stack type for this network interface to identify whether the IPv6 feature is enabled or not. If not specified, IPV4_ONLY will be used."]
    pub fn stack_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_type", self.base))
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nThe name of the subnetwork to attach this interface to. The subnetwork must exist in the same region this instance will be created in. Either network or subnetwork must be provided."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
    #[doc = "Get a reference to the value of field `subnetwork_project` after provisioning.\nThe ID of the project in which the subnetwork belongs. If it is not provided, the provider project is used."]
    pub fn subnetwork_project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnetwork_project", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vlan` after provisioning.\nVLAN tag of a dynamic network interface, must be an integer in the range from 2 to 255 inclusively."]
    pub fn vlan(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vlan", self.base))
    }
    #[doc = "Get a reference to the value of field `access_config` after provisioning.\n"]
    pub fn access_config(
        &self,
    ) -> ListRef<ComputeInstanceTemplateNetworkInterfaceElAccessConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `alias_ip_range` after provisioning.\n"]
    pub fn alias_ip_range(
        &self,
    ) -> ListRef<ComputeInstanceTemplateNetworkInterfaceElAliasIpRangeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.alias_ip_range", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_access_config` after provisioning.\n"]
    pub fn ipv6_access_config(
        &self,
    ) -> ListRef<ComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ipv6_access_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceTemplateNetworkPerformanceConfigEl {
    total_egress_bandwidth_tier: PrimField<String>,
}
impl ComputeInstanceTemplateNetworkPerformanceConfigEl {}
impl ToListMappable for ComputeInstanceTemplateNetworkPerformanceConfigEl {
    type O = BlockAssignable<ComputeInstanceTemplateNetworkPerformanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceTemplateNetworkPerformanceConfigEl {
    #[doc = "The egress bandwidth tier to enable. Possible values:TIER_1, DEFAULT"]
    pub total_egress_bandwidth_tier: PrimField<String>,
}
impl BuildComputeInstanceTemplateNetworkPerformanceConfigEl {
    pub fn build(self) -> ComputeInstanceTemplateNetworkPerformanceConfigEl {
        ComputeInstanceTemplateNetworkPerformanceConfigEl {
            total_egress_bandwidth_tier: self.total_egress_bandwidth_tier,
        }
    }
}
pub struct ComputeInstanceTemplateNetworkPerformanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceTemplateNetworkPerformanceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInstanceTemplateNetworkPerformanceConfigElRef {
        ComputeInstanceTemplateNetworkPerformanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceTemplateNetworkPerformanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `total_egress_bandwidth_tier` after provisioning.\nThe egress bandwidth tier to enable. Possible values:TIER_1, DEFAULT"]
    pub fn total_egress_bandwidth_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_egress_bandwidth_tier", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceTemplateReservationAffinityElSpecificReservationEl {
    key: PrimField<String>,
    values: ListField<PrimField<String>>,
}
impl ComputeInstanceTemplateReservationAffinityElSpecificReservationEl {}
impl ToListMappable for ComputeInstanceTemplateReservationAffinityElSpecificReservationEl {
    type O = BlockAssignable<ComputeInstanceTemplateReservationAffinityElSpecificReservationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceTemplateReservationAffinityElSpecificReservationEl {
    #[doc = "Corresponds to the label key of a reservation resource. To target a SPECIFIC_RESERVATION by name, specify compute.googleapis.com/reservation-name as the key and specify the name of your reservation as the only value."]
    pub key: PrimField<String>,
    #[doc = "Corresponds to the label values of a reservation resource."]
    pub values: ListField<PrimField<String>>,
}
impl BuildComputeInstanceTemplateReservationAffinityElSpecificReservationEl {
    pub fn build(self) -> ComputeInstanceTemplateReservationAffinityElSpecificReservationEl {
        ComputeInstanceTemplateReservationAffinityElSpecificReservationEl {
            key: self.key,
            values: self.values,
        }
    }
}
pub struct ComputeInstanceTemplateReservationAffinityElSpecificReservationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceTemplateReservationAffinityElSpecificReservationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInstanceTemplateReservationAffinityElSpecificReservationElRef {
        ComputeInstanceTemplateReservationAffinityElSpecificReservationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceTemplateReservationAffinityElSpecificReservationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nCorresponds to the label key of a reservation resource. To target a SPECIFIC_RESERVATION by name, specify compute.googleapis.com/reservation-name as the key and specify the name of your reservation as the only value."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\nCorresponds to the label values of a reservation resource."]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeInstanceTemplateReservationAffinityElDynamic {
    specific_reservation:
        Option<DynamicBlock<ComputeInstanceTemplateReservationAffinityElSpecificReservationEl>>,
}
#[derive(Serialize)]
pub struct ComputeInstanceTemplateReservationAffinityEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    specific_reservation:
        Option<Vec<ComputeInstanceTemplateReservationAffinityElSpecificReservationEl>>,
    dynamic: ComputeInstanceTemplateReservationAffinityElDynamic,
}
impl ComputeInstanceTemplateReservationAffinityEl {
    #[doc = "Set the field `specific_reservation`.\n"]
    pub fn set_specific_reservation(
        mut self,
        v: impl Into<BlockAssignable<ComputeInstanceTemplateReservationAffinityElSpecificReservationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.specific_reservation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.specific_reservation = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeInstanceTemplateReservationAffinityEl {
    type O = BlockAssignable<ComputeInstanceTemplateReservationAffinityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceTemplateReservationAffinityEl {
    #[doc = "The type of reservation from which this instance can consume resources."]
    pub type_: PrimField<String>,
}
impl BuildComputeInstanceTemplateReservationAffinityEl {
    pub fn build(self) -> ComputeInstanceTemplateReservationAffinityEl {
        ComputeInstanceTemplateReservationAffinityEl {
            type_: self.type_,
            specific_reservation: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeInstanceTemplateReservationAffinityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceTemplateReservationAffinityElRef {
    fn new(shared: StackShared, base: String) -> ComputeInstanceTemplateReservationAffinityElRef {
        ComputeInstanceTemplateReservationAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceTemplateReservationAffinityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of reservation from which this instance can consume resources."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `specific_reservation` after provisioning.\n"]
    pub fn specific_reservation(
        &self,
    ) -> ListRef<ComputeInstanceTemplateReservationAffinityElSpecificReservationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.specific_reservation", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    seconds: PrimField<f64>,
}
impl ComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond\nresolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must\nbe from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
    type O = BlockAssignable<ComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
    #[doc = "Span of time at a resolution of a second.\nMust be from 0 to 315,576,000,000 inclusive."]
    pub seconds: PrimField<f64>,
}
impl BuildComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
    pub fn build(self) -> ComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
        ComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}
pub struct ComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutElRef {
        ComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond\nresolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must\nbe from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second.\nMust be from 0 to 315,576,000,000 inclusive."]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceTemplateSchedulingElMaxRunDurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    seconds: PrimField<f64>,
}
impl ComputeInstanceTemplateSchedulingElMaxRunDurationEl {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond\nresolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must\nbe from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInstanceTemplateSchedulingElMaxRunDurationEl {
    type O = BlockAssignable<ComputeInstanceTemplateSchedulingElMaxRunDurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceTemplateSchedulingElMaxRunDurationEl {
    #[doc = "Span of time at a resolution of a second.\nMust be from 0 to 315,576,000,000 inclusive."]
    pub seconds: PrimField<f64>,
}
impl BuildComputeInstanceTemplateSchedulingElMaxRunDurationEl {
    pub fn build(self) -> ComputeInstanceTemplateSchedulingElMaxRunDurationEl {
        ComputeInstanceTemplateSchedulingElMaxRunDurationEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}
pub struct ComputeInstanceTemplateSchedulingElMaxRunDurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceTemplateSchedulingElMaxRunDurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInstanceTemplateSchedulingElMaxRunDurationElRef {
        ComputeInstanceTemplateSchedulingElMaxRunDurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceTemplateSchedulingElMaxRunDurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond\nresolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must\nbe from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second.\nMust be from 0 to 315,576,000,000 inclusive."]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceTemplateSchedulingElNodeAffinitiesEl {
    key: PrimField<String>,
    operator: PrimField<String>,
    values: SetField<PrimField<String>>,
}
impl ComputeInstanceTemplateSchedulingElNodeAffinitiesEl {}
impl ToListMappable for ComputeInstanceTemplateSchedulingElNodeAffinitiesEl {
    type O = BlockAssignable<ComputeInstanceTemplateSchedulingElNodeAffinitiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceTemplateSchedulingElNodeAffinitiesEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub operator: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}
impl BuildComputeInstanceTemplateSchedulingElNodeAffinitiesEl {
    pub fn build(self) -> ComputeInstanceTemplateSchedulingElNodeAffinitiesEl {
        ComputeInstanceTemplateSchedulingElNodeAffinitiesEl {
            key: self.key,
            operator: self.operator,
            values: self.values,
        }
    }
}
pub struct ComputeInstanceTemplateSchedulingElNodeAffinitiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceTemplateSchedulingElNodeAffinitiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInstanceTemplateSchedulingElNodeAffinitiesElRef {
        ComputeInstanceTemplateSchedulingElNodeAffinitiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceTemplateSchedulingElNodeAffinitiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceTemplateSchedulingElOnInstanceStopActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    discard_local_ssd: Option<PrimField<bool>>,
}
impl ComputeInstanceTemplateSchedulingElOnInstanceStopActionEl {
    #[doc = "Set the field `discard_local_ssd`.\nIf true, the contents of any attached Local SSD disks will be discarded."]
    pub fn set_discard_local_ssd(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.discard_local_ssd = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInstanceTemplateSchedulingElOnInstanceStopActionEl {
    type O = BlockAssignable<ComputeInstanceTemplateSchedulingElOnInstanceStopActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceTemplateSchedulingElOnInstanceStopActionEl {}
impl BuildComputeInstanceTemplateSchedulingElOnInstanceStopActionEl {
    pub fn build(self) -> ComputeInstanceTemplateSchedulingElOnInstanceStopActionEl {
        ComputeInstanceTemplateSchedulingElOnInstanceStopActionEl {
            discard_local_ssd: core::default::Default::default(),
        }
    }
}
pub struct ComputeInstanceTemplateSchedulingElOnInstanceStopActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceTemplateSchedulingElOnInstanceStopActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInstanceTemplateSchedulingElOnInstanceStopActionElRef {
        ComputeInstanceTemplateSchedulingElOnInstanceStopActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceTemplateSchedulingElOnInstanceStopActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `discard_local_ssd` after provisioning.\nIf true, the contents of any attached Local SSD disks will be discarded."]
    pub fn discard_local_ssd(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.discard_local_ssd", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ComputeInstanceTemplateSchedulingElDynamic {
    local_ssd_recovery_timeout:
        Option<DynamicBlock<ComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl>>,
    max_run_duration: Option<DynamicBlock<ComputeInstanceTemplateSchedulingElMaxRunDurationEl>>,
    node_affinities: Option<DynamicBlock<ComputeInstanceTemplateSchedulingElNodeAffinitiesEl>>,
    on_instance_stop_action:
        Option<DynamicBlock<ComputeInstanceTemplateSchedulingElOnInstanceStopActionEl>>,
}
#[derive(Serialize)]
pub struct ComputeInstanceTemplateSchedulingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_restart: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_domain: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_termination_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_node_cpus: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_host_maintenance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preemptible: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioning_model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    termination_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ssd_recovery_timeout:
        Option<Vec<ComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_run_duration: Option<Vec<ComputeInstanceTemplateSchedulingElMaxRunDurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_affinities: Option<Vec<ComputeInstanceTemplateSchedulingElNodeAffinitiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_instance_stop_action: Option<Vec<ComputeInstanceTemplateSchedulingElOnInstanceStopActionEl>>,
    dynamic: ComputeInstanceTemplateSchedulingElDynamic,
}
impl ComputeInstanceTemplateSchedulingEl {
    #[doc = "Set the field `automatic_restart`.\nSpecifies whether the instance should be automatically restarted if it is terminated by Compute Engine (not terminated by a user). This defaults to true."]
    pub fn set_automatic_restart(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.automatic_restart = Some(v.into());
        self
    }
    #[doc = "Set the field `availability_domain`.\nSpecifies the availability domain, which this instance should be scheduled on."]
    pub fn set_availability_domain(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.availability_domain = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_termination_action`.\nSpecifies the action GCE should take when SPOT VM is preempted."]
    pub fn set_instance_termination_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_termination_action = Some(v.into());
        self
    }
    #[doc = "Set the field `min_node_cpus`.\nMinimum number of cpus for the instance."]
    pub fn set_min_node_cpus(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_node_cpus = Some(v.into());
        self
    }
    #[doc = "Set the field `on_host_maintenance`.\nDefines the maintenance behavior for this instance."]
    pub fn set_on_host_maintenance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.on_host_maintenance = Some(v.into());
        self
    }
    #[doc = "Set the field `preemptible`.\nAllows instance to be preempted. This defaults to false."]
    pub fn set_preemptible(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preemptible = Some(v.into());
        self
    }
    #[doc = "Set the field `provisioning_model`.\nWhether the instance is spot. If this is set as SPOT."]
    pub fn set_provisioning_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.provisioning_model = Some(v.into());
        self
    }
    #[doc = "Set the field `termination_time`.\nSpecifies the timestamp, when the instance will be terminated,\nin RFC3339 text format. If specified, the instance termination action\nwill be performed at the termination time."]
    pub fn set_termination_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.termination_time = Some(v.into());
        self
    }
    #[doc = "Set the field `local_ssd_recovery_timeout`.\n"]
    pub fn set_local_ssd_recovery_timeout(
        mut self,
        v: impl Into<BlockAssignable<ComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.local_ssd_recovery_timeout = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.local_ssd_recovery_timeout = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `max_run_duration`.\n"]
    pub fn set_max_run_duration(
        mut self,
        v: impl Into<BlockAssignable<ComputeInstanceTemplateSchedulingElMaxRunDurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.max_run_duration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.max_run_duration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `node_affinities`.\n"]
    pub fn set_node_affinities(
        mut self,
        v: impl Into<BlockAssignable<ComputeInstanceTemplateSchedulingElNodeAffinitiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.node_affinities = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.node_affinities = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `on_instance_stop_action`.\n"]
    pub fn set_on_instance_stop_action(
        mut self,
        v: impl Into<BlockAssignable<ComputeInstanceTemplateSchedulingElOnInstanceStopActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.on_instance_stop_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.on_instance_stop_action = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeInstanceTemplateSchedulingEl {
    type O = BlockAssignable<ComputeInstanceTemplateSchedulingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceTemplateSchedulingEl {}
impl BuildComputeInstanceTemplateSchedulingEl {
    pub fn build(self) -> ComputeInstanceTemplateSchedulingEl {
        ComputeInstanceTemplateSchedulingEl {
            automatic_restart: core::default::Default::default(),
            availability_domain: core::default::Default::default(),
            instance_termination_action: core::default::Default::default(),
            min_node_cpus: core::default::Default::default(),
            on_host_maintenance: core::default::Default::default(),
            preemptible: core::default::Default::default(),
            provisioning_model: core::default::Default::default(),
            termination_time: core::default::Default::default(),
            local_ssd_recovery_timeout: core::default::Default::default(),
            max_run_duration: core::default::Default::default(),
            node_affinities: core::default::Default::default(),
            on_instance_stop_action: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeInstanceTemplateSchedulingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceTemplateSchedulingElRef {
    fn new(shared: StackShared, base: String) -> ComputeInstanceTemplateSchedulingElRef {
        ComputeInstanceTemplateSchedulingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceTemplateSchedulingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `automatic_restart` after provisioning.\nSpecifies whether the instance should be automatically restarted if it is terminated by Compute Engine (not terminated by a user). This defaults to true."]
    pub fn automatic_restart(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.automatic_restart", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `availability_domain` after provisioning.\nSpecifies the availability domain, which this instance should be scheduled on."]
    pub fn availability_domain(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_domain", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance_termination_action` after provisioning.\nSpecifies the action GCE should take when SPOT VM is preempted."]
    pub fn instance_termination_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_termination_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_node_cpus` after provisioning.\nMinimum number of cpus for the instance."]
    pub fn min_node_cpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_node_cpus", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `on_host_maintenance` after provisioning.\nDefines the maintenance behavior for this instance."]
    pub fn on_host_maintenance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_host_maintenance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `preemptible` after provisioning.\nAllows instance to be preempted. This defaults to false."]
    pub fn preemptible(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preemptible", self.base))
    }
    #[doc = "Get a reference to the value of field `provisioning_model` after provisioning.\nWhether the instance is spot. If this is set as SPOT."]
    pub fn provisioning_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioning_model", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `termination_time` after provisioning.\nSpecifies the timestamp, when the instance will be terminated,\nin RFC3339 text format. If specified, the instance termination action\nwill be performed at the termination time."]
    pub fn termination_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.termination_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_ssd_recovery_timeout` after provisioning.\n"]
    pub fn local_ssd_recovery_timeout(
        &self,
    ) -> ListRef<ComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.local_ssd_recovery_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_run_duration` after provisioning.\n"]
    pub fn max_run_duration(
        &self,
    ) -> ListRef<ComputeInstanceTemplateSchedulingElMaxRunDurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.max_run_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `on_instance_stop_action` after provisioning.\n"]
    pub fn on_instance_stop_action(
        &self,
    ) -> ListRef<ComputeInstanceTemplateSchedulingElOnInstanceStopActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.on_instance_stop_action", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceTemplateServiceAccountEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
    scopes: SetField<PrimField<String>>,
}
impl ComputeInstanceTemplateServiceAccountEl {
    #[doc = "Set the field `email`.\nThe service account e-mail address. If not given, the default Google Compute Engine service account is used."]
    pub fn set_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInstanceTemplateServiceAccountEl {
    type O = BlockAssignable<ComputeInstanceTemplateServiceAccountEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceTemplateServiceAccountEl {
    #[doc = "A list of service scopes. Both OAuth2 URLs and gcloud short names are supported. To allow full access to all Cloud APIs, use the cloud-platform scope."]
    pub scopes: SetField<PrimField<String>>,
}
impl BuildComputeInstanceTemplateServiceAccountEl {
    pub fn build(self) -> ComputeInstanceTemplateServiceAccountEl {
        ComputeInstanceTemplateServiceAccountEl {
            email: core::default::Default::default(),
            scopes: self.scopes,
        }
    }
}
pub struct ComputeInstanceTemplateServiceAccountElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceTemplateServiceAccountElRef {
    fn new(shared: StackShared, base: String) -> ComputeInstanceTemplateServiceAccountElRef {
        ComputeInstanceTemplateServiceAccountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceTemplateServiceAccountElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\nThe service account e-mail address. If not given, the default Google Compute Engine service account is used."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
    #[doc = "Get a reference to the value of field `scopes` after provisioning.\nA list of service scopes. Both OAuth2 URLs and gcloud short names are supported. To allow full access to all Cloud APIs, use the cloud-platform scope."]
    pub fn scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceTemplateShieldedInstanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_integrity_monitoring: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_secure_boot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_vtpm: Option<PrimField<bool>>,
}
impl ComputeInstanceTemplateShieldedInstanceConfigEl {
    #[doc = "Set the field `enable_integrity_monitoring`.\nCompare the most recent boot measurements to the integrity policy baseline and return a pair of pass/fail results depending on whether they match or not. Defaults to true."]
    pub fn set_enable_integrity_monitoring(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_secure_boot`.\nVerify the digital signature of all boot components, and halt the boot process if signature verification fails. Defaults to false."]
    pub fn set_enable_secure_boot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_secure_boot = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_vtpm`.\nUse a virtualized trusted platform module, which is a specialized computer chip you can use to encrypt objects like keys and certificates. Defaults to true."]
    pub fn set_enable_vtpm(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_vtpm = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInstanceTemplateShieldedInstanceConfigEl {
    type O = BlockAssignable<ComputeInstanceTemplateShieldedInstanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceTemplateShieldedInstanceConfigEl {}
impl BuildComputeInstanceTemplateShieldedInstanceConfigEl {
    pub fn build(self) -> ComputeInstanceTemplateShieldedInstanceConfigEl {
        ComputeInstanceTemplateShieldedInstanceConfigEl {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
            enable_vtpm: core::default::Default::default(),
        }
    }
}
pub struct ComputeInstanceTemplateShieldedInstanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceTemplateShieldedInstanceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInstanceTemplateShieldedInstanceConfigElRef {
        ComputeInstanceTemplateShieldedInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceTemplateShieldedInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\nCompare the most recent boot measurements to the integrity policy baseline and return a pair of pass/fail results depending on whether they match or not. Defaults to true."]
    pub fn enable_integrity_monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_integrity_monitoring", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_secure_boot` after provisioning.\nVerify the digital signature of all boot components, and halt the boot process if signature verification fails. Defaults to false."]
    pub fn enable_secure_boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_secure_boot", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_vtpm` after provisioning.\nUse a virtualized trusted platform module, which is a specialized computer chip you can use to encrypt objects like keys and certificates. Defaults to true."]
    pub fn enable_vtpm(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_vtpm", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceTemplateTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl ComputeInstanceTemplateTimeoutsEl {
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
impl ToListMappable for ComputeInstanceTemplateTimeoutsEl {
    type O = BlockAssignable<ComputeInstanceTemplateTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceTemplateTimeoutsEl {}
impl BuildComputeInstanceTemplateTimeoutsEl {
    pub fn build(self) -> ComputeInstanceTemplateTimeoutsEl {
        ComputeInstanceTemplateTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct ComputeInstanceTemplateTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceTemplateTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeInstanceTemplateTimeoutsElRef {
        ComputeInstanceTemplateTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceTemplateTimeoutsElRef {
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
struct ComputeInstanceTemplateDynamic {
    advanced_machine_features:
        Option<DynamicBlock<ComputeInstanceTemplateAdvancedMachineFeaturesEl>>,
    confidential_instance_config:
        Option<DynamicBlock<ComputeInstanceTemplateConfidentialInstanceConfigEl>>,
    disk: Option<DynamicBlock<ComputeInstanceTemplateDiskEl>>,
    guest_accelerator: Option<DynamicBlock<ComputeInstanceTemplateGuestAcceleratorEl>>,
    network_interface: Option<DynamicBlock<ComputeInstanceTemplateNetworkInterfaceEl>>,
    network_performance_config:
        Option<DynamicBlock<ComputeInstanceTemplateNetworkPerformanceConfigEl>>,
    reservation_affinity: Option<DynamicBlock<ComputeInstanceTemplateReservationAffinityEl>>,
    scheduling: Option<DynamicBlock<ComputeInstanceTemplateSchedulingEl>>,
    service_account: Option<DynamicBlock<ComputeInstanceTemplateServiceAccountEl>>,
    shielded_instance_config: Option<DynamicBlock<ComputeInstanceTemplateShieldedInstanceConfigEl>>,
}
