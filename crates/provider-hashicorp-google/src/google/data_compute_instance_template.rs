use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeInstanceTemplateData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    most_recent: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_link_unique: Option<PrimField<String>>,
}
struct DataComputeInstanceTemplate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeInstanceTemplateData>,
}
#[derive(Clone)]
pub struct DataComputeInstanceTemplate(Rc<DataComputeInstanceTemplate_>);
impl DataComputeInstanceTemplate {
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
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `most_recent`.\n"]
    pub fn set_most_recent(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().most_recent = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nThe name of the instance template. If you leave this blank, Terraform will auto-generate a unique name."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `self_link_unique`.\n"]
    pub fn set_self_link_unique(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().self_link_unique = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `advanced_machine_features` after provisioning.\nControls for advanced machine-related behavior features."]
    pub fn advanced_machine_features(
        &self,
    ) -> ListRef<DataComputeInstanceTemplateAdvancedMachineFeaturesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_machine_features", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `can_ip_forward` after provisioning.\nWhether to allow sending and receiving of packets with non-matching source or destination IPs. This defaults to false."]
    pub fn can_ip_forward(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.can_ip_forward", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `confidential_instance_config` after provisioning.\nThe Confidential VM config being used by the instance. on_host_maintenance has to be set to TERMINATE or this will fail to create."]
    pub fn confidential_instance_config(
        &self,
    ) -> ListRef<DataComputeInstanceTemplateConfidentialInstanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.confidential_instance_config", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `disk` after provisioning.\nDisks to attach to instances created from this template. This can be specified multiple times for multiple disks."]
    pub fn disk(&self) -> ListRef<DataComputeInstanceTemplateDiskElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `guest_accelerator` after provisioning.\nList of the type and count of accelerator cards attached to the instance."]
    pub fn guest_accelerator(&self) -> ListRef<DataComputeInstanceTemplateGuestAcceleratorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.guest_accelerator", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `most_recent` after provisioning.\n"]
    pub fn most_recent(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.most_recent", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `network_interface` after provisioning.\nNetworks to attach to instances created from this template. This can be specified multiple times for multiple networks."]
    pub fn network_interface(&self) -> ListRef<DataComputeInstanceTemplateNetworkInterfaceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_interface", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_performance_config` after provisioning.\nConfigures network performance settings for the instance. If not specified, the instance will be created with its default network performance configuration."]
    pub fn network_performance_config(
        &self,
    ) -> ListRef<DataComputeInstanceTemplateNetworkPerformanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_performance_config", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\nSpecifies the reservations that this instance can consume from."]
    pub fn reservation_affinity(
        &self,
    ) -> ListRef<DataComputeInstanceTemplateReservationAffinityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reservation_affinity", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `scheduling` after provisioning.\nThe scheduling strategy to use."]
    pub fn scheduling(&self) -> ListRef<DataComputeInstanceTemplateSchedulingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scheduling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link_unique` after provisioning.\n"]
    pub fn self_link_unique(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link_unique", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nService account to attach to the instance."]
    pub fn service_account(&self) -> ListRef<DataComputeInstanceTemplateServiceAccountElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\nEnable Shielded VM on this instance. Shielded VM provides verifiable integrity to prevent against malware and rootkits. Defaults to disabled. Note: shielded_instance_config can only be used with boot images with shielded vm support."]
    pub fn shielded_instance_config(
        &self,
    ) -> ListRef<DataComputeInstanceTemplateShieldedInstanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.shielded_instance_config", self.extract_ref()),
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
}
impl Referable for DataComputeInstanceTemplate {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeInstanceTemplate {}
impl ToListMappable for DataComputeInstanceTemplate {
    type O = ListRef<DataComputeInstanceTemplateRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeInstanceTemplate_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_instance_template".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeInstanceTemplate {
    pub tf_id: String,
}
impl BuildDataComputeInstanceTemplate {
    pub fn build(self, stack: &mut Stack) -> DataComputeInstanceTemplate {
        let out = DataComputeInstanceTemplate(Rc::new(DataComputeInstanceTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeInstanceTemplateData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                most_recent: core::default::Default::default(),
                name: core::default::Default::default(),
                project: core::default::Default::default(),
                self_link_unique: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeInstanceTemplateRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeInstanceTemplateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeInstanceTemplateRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `advanced_machine_features` after provisioning.\nControls for advanced machine-related behavior features."]
    pub fn advanced_machine_features(
        &self,
    ) -> ListRef<DataComputeInstanceTemplateAdvancedMachineFeaturesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_machine_features", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `can_ip_forward` after provisioning.\nWhether to allow sending and receiving of packets with non-matching source or destination IPs. This defaults to false."]
    pub fn can_ip_forward(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.can_ip_forward", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `confidential_instance_config` after provisioning.\nThe Confidential VM config being used by the instance. on_host_maintenance has to be set to TERMINATE or this will fail to create."]
    pub fn confidential_instance_config(
        &self,
    ) -> ListRef<DataComputeInstanceTemplateConfidentialInstanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.confidential_instance_config", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `disk` after provisioning.\nDisks to attach to instances created from this template. This can be specified multiple times for multiple disks."]
    pub fn disk(&self) -> ListRef<DataComputeInstanceTemplateDiskElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `guest_accelerator` after provisioning.\nList of the type and count of accelerator cards attached to the instance."]
    pub fn guest_accelerator(&self) -> ListRef<DataComputeInstanceTemplateGuestAcceleratorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.guest_accelerator", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `most_recent` after provisioning.\n"]
    pub fn most_recent(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.most_recent", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `network_interface` after provisioning.\nNetworks to attach to instances created from this template. This can be specified multiple times for multiple networks."]
    pub fn network_interface(&self) -> ListRef<DataComputeInstanceTemplateNetworkInterfaceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_interface", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_performance_config` after provisioning.\nConfigures network performance settings for the instance. If not specified, the instance will be created with its default network performance configuration."]
    pub fn network_performance_config(
        &self,
    ) -> ListRef<DataComputeInstanceTemplateNetworkPerformanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_performance_config", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\nSpecifies the reservations that this instance can consume from."]
    pub fn reservation_affinity(
        &self,
    ) -> ListRef<DataComputeInstanceTemplateReservationAffinityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reservation_affinity", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `scheduling` after provisioning.\nThe scheduling strategy to use."]
    pub fn scheduling(&self) -> ListRef<DataComputeInstanceTemplateSchedulingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scheduling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link_unique` after provisioning.\n"]
    pub fn self_link_unique(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link_unique", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nService account to attach to the instance."]
    pub fn service_account(&self) -> ListRef<DataComputeInstanceTemplateServiceAccountElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\nEnable Shielded VM on this instance. Shielded VM provides verifiable integrity to prevent against malware and rootkits. Defaults to disabled. Note: shielded_instance_config can only be used with boot images with shielded vm support."]
    pub fn shielded_instance_config(
        &self,
    ) -> ListRef<DataComputeInstanceTemplateShieldedInstanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.shielded_instance_config", self.extract_ref()),
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
}
#[derive(Serialize)]
pub struct DataComputeInstanceTemplateAdvancedMachineFeaturesEl {
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
impl DataComputeInstanceTemplateAdvancedMachineFeaturesEl {
    #[doc = "Set the field `enable_nested_virtualization`.\n"]
    pub fn set_enable_nested_virtualization(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_nested_virtualization = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_uefi_networking`.\n"]
    pub fn set_enable_uefi_networking(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_uefi_networking = Some(v.into());
        self
    }
    #[doc = "Set the field `performance_monitoring_unit`.\n"]
    pub fn set_performance_monitoring_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.performance_monitoring_unit = Some(v.into());
        self
    }
    #[doc = "Set the field `threads_per_core`.\n"]
    pub fn set_threads_per_core(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.threads_per_core = Some(v.into());
        self
    }
    #[doc = "Set the field `turbo_mode`.\n"]
    pub fn set_turbo_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.turbo_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `visible_core_count`.\n"]
    pub fn set_visible_core_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.visible_core_count = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeInstanceTemplateAdvancedMachineFeaturesEl {
    type O = BlockAssignable<DataComputeInstanceTemplateAdvancedMachineFeaturesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeInstanceTemplateAdvancedMachineFeaturesEl {}
impl BuildDataComputeInstanceTemplateAdvancedMachineFeaturesEl {
    pub fn build(self) -> DataComputeInstanceTemplateAdvancedMachineFeaturesEl {
        DataComputeInstanceTemplateAdvancedMachineFeaturesEl {
            enable_nested_virtualization: core::default::Default::default(),
            enable_uefi_networking: core::default::Default::default(),
            performance_monitoring_unit: core::default::Default::default(),
            threads_per_core: core::default::Default::default(),
            turbo_mode: core::default::Default::default(),
            visible_core_count: core::default::Default::default(),
        }
    }
}
pub struct DataComputeInstanceTemplateAdvancedMachineFeaturesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeInstanceTemplateAdvancedMachineFeaturesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeInstanceTemplateAdvancedMachineFeaturesElRef {
        DataComputeInstanceTemplateAdvancedMachineFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeInstanceTemplateAdvancedMachineFeaturesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_nested_virtualization` after provisioning.\n"]
    pub fn enable_nested_virtualization(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_nested_virtualization", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_uefi_networking` after provisioning.\n"]
    pub fn enable_uefi_networking(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_uefi_networking", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `performance_monitoring_unit` after provisioning.\n"]
    pub fn performance_monitoring_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.performance_monitoring_unit", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `threads_per_core` after provisioning.\n"]
    pub fn threads_per_core(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.threads_per_core", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `turbo_mode` after provisioning.\n"]
    pub fn turbo_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.turbo_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `visible_core_count` after provisioning.\n"]
    pub fn visible_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.visible_core_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeInstanceTemplateConfidentialInstanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    confidential_instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_confidential_compute: Option<PrimField<bool>>,
}
impl DataComputeInstanceTemplateConfidentialInstanceConfigEl {
    #[doc = "Set the field `confidential_instance_type`.\n"]
    pub fn set_confidential_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.confidential_instance_type = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_confidential_compute`.\n"]
    pub fn set_enable_confidential_compute(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_confidential_compute = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeInstanceTemplateConfidentialInstanceConfigEl {
    type O = BlockAssignable<DataComputeInstanceTemplateConfidentialInstanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeInstanceTemplateConfidentialInstanceConfigEl {}
impl BuildDataComputeInstanceTemplateConfidentialInstanceConfigEl {
    pub fn build(self) -> DataComputeInstanceTemplateConfidentialInstanceConfigEl {
        DataComputeInstanceTemplateConfidentialInstanceConfigEl {
            confidential_instance_type: core::default::Default::default(),
            enable_confidential_compute: core::default::Default::default(),
        }
    }
}
pub struct DataComputeInstanceTemplateConfidentialInstanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeInstanceTemplateConfidentialInstanceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeInstanceTemplateConfidentialInstanceConfigElRef {
        DataComputeInstanceTemplateConfidentialInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeInstanceTemplateConfidentialInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `confidential_instance_type` after provisioning.\n"]
    pub fn confidential_instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.confidential_instance_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_confidential_compute` after provisioning.\n"]
    pub fn enable_confidential_compute(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_confidential_compute", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeInstanceTemplateDiskElDiskEncryptionKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_service_account: Option<PrimField<String>>,
}
impl DataComputeInstanceTemplateDiskElDiskEncryptionKeyEl {
    #[doc = "Set the field `kms_key_self_link`.\n"]
    pub fn set_kms_key_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_self_link = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_service_account`.\n"]
    pub fn set_kms_key_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_service_account = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeInstanceTemplateDiskElDiskEncryptionKeyEl {
    type O = BlockAssignable<DataComputeInstanceTemplateDiskElDiskEncryptionKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeInstanceTemplateDiskElDiskEncryptionKeyEl {}
impl BuildDataComputeInstanceTemplateDiskElDiskEncryptionKeyEl {
    pub fn build(self) -> DataComputeInstanceTemplateDiskElDiskEncryptionKeyEl {
        DataComputeInstanceTemplateDiskElDiskEncryptionKeyEl {
            kms_key_self_link: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
        }
    }
}
pub struct DataComputeInstanceTemplateDiskElDiskEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeInstanceTemplateDiskElDiskEncryptionKeyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeInstanceTemplateDiskElDiskEncryptionKeyElRef {
        DataComputeInstanceTemplateDiskElDiskEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeInstanceTemplateDiskElDiskEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_self_link` after provisioning.\n"]
    pub fn kms_key_self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_self_link", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_service_account` after provisioning.\n"]
    pub fn kms_key_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_service_account", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rsa_encrypted_key: Option<PrimField<String>>,
}
impl DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl {
    #[doc = "Set the field `kms_key_self_link`.\n"]
    pub fn set_kms_key_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_self_link = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_service_account`.\n"]
    pub fn set_kms_key_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `raw_key`.\n"]
    pub fn set_raw_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.raw_key = Some(v.into());
        self
    }
    #[doc = "Set the field `rsa_encrypted_key`.\n"]
    pub fn set_rsa_encrypted_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rsa_encrypted_key = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl {
    type O = BlockAssignable<DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl {}
impl BuildDataComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl {
    pub fn build(self) -> DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl {
        DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl {
            kms_key_self_link: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
            rsa_encrypted_key: core::default::Default::default(),
        }
    }
}
pub struct DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyElRef {
        DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_self_link` after provisioning.\n"]
    pub fn kms_key_self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_self_link", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_service_account` after provisioning.\n"]
    pub fn kms_key_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `raw_key` after provisioning.\n"]
    pub fn raw_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.raw_key", self.base))
    }
    #[doc = "Get a reference to the value of field `rsa_encrypted_key` after provisioning.\n"]
    pub fn rsa_encrypted_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rsa_encrypted_key", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rsa_encrypted_key: Option<PrimField<String>>,
}
impl DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {
    #[doc = "Set the field `kms_key_self_link`.\n"]
    pub fn set_kms_key_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_self_link = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_service_account`.\n"]
    pub fn set_kms_key_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `raw_key`.\n"]
    pub fn set_raw_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.raw_key = Some(v.into());
        self
    }
    #[doc = "Set the field `rsa_encrypted_key`.\n"]
    pub fn set_rsa_encrypted_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rsa_encrypted_key = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {
    type O = BlockAssignable<DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {}
impl BuildDataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {
    pub fn build(self) -> DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {
        DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {
            kms_key_self_link: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
            rsa_encrypted_key: core::default::Default::default(),
        }
    }
}
pub struct DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyElRef {
        DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_self_link` after provisioning.\n"]
    pub fn kms_key_self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_self_link", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_service_account` after provisioning.\n"]
    pub fn kms_key_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `raw_key` after provisioning.\n"]
    pub fn raw_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.raw_key", self.base))
    }
    #[doc = "Get a reference to the value of field `rsa_encrypted_key` after provisioning.\n"]
    pub fn rsa_encrypted_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rsa_encrypted_key", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeInstanceTemplateDiskEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    architecture: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_delete: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_encryption_key: Option<ListField<DataComputeInstanceTemplateDiskElDiskEncryptionKeyEl>>,
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
    source_image_encryption_key:
        Option<ListField<DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_snapshot: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_snapshot_encryption_key:
        Option<ListField<DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_pool: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataComputeInstanceTemplateDiskEl {
    #[doc = "Set the field `architecture`.\n"]
    pub fn set_architecture(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.architecture = Some(v.into());
        self
    }
    #[doc = "Set the field `auto_delete`.\n"]
    pub fn set_auto_delete(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_delete = Some(v.into());
        self
    }
    #[doc = "Set the field `boot`.\n"]
    pub fn set_boot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.boot = Some(v.into());
        self
    }
    #[doc = "Set the field `device_name`.\n"]
    pub fn set_device_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device_name = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_encryption_key`.\n"]
    pub fn set_disk_encryption_key(
        mut self,
        v: impl Into<ListField<DataComputeInstanceTemplateDiskElDiskEncryptionKeyEl>>,
    ) -> Self {
        self.disk_encryption_key = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_name`.\n"]
    pub fn set_disk_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_name = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_size_gb`.\n"]
    pub fn set_disk_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_type`.\n"]
    pub fn set_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_type = Some(v.into());
        self
    }
    #[doc = "Set the field `guest_os_features`.\n"]
    pub fn set_guest_os_features(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.guest_os_features = Some(v.into());
        self
    }
    #[doc = "Set the field `interface`.\n"]
    pub fn set_interface(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interface = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
    #[doc = "Set the field `provisioned_iops`.\n"]
    pub fn set_provisioned_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.provisioned_iops = Some(v.into());
        self
    }
    #[doc = "Set the field `provisioned_throughput`.\n"]
    pub fn set_provisioned_throughput(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.provisioned_throughput = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_manager_tags`.\n"]
    pub fn set_resource_manager_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_manager_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_policies`.\n"]
    pub fn set_resource_policies(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.resource_policies = Some(v.into());
        self
    }
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source = Some(v.into());
        self
    }
    #[doc = "Set the field `source_image`.\n"]
    pub fn set_source_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_image = Some(v.into());
        self
    }
    #[doc = "Set the field `source_image_encryption_key`.\n"]
    pub fn set_source_image_encryption_key(
        mut self,
        v: impl Into<ListField<DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl>>,
    ) -> Self {
        self.source_image_encryption_key = Some(v.into());
        self
    }
    #[doc = "Set the field `source_snapshot`.\n"]
    pub fn set_source_snapshot(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_snapshot = Some(v.into());
        self
    }
    #[doc = "Set the field `source_snapshot_encryption_key`.\n"]
    pub fn set_source_snapshot_encryption_key(
        mut self,
        v: impl Into<ListField<DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl>>,
    ) -> Self {
        self.source_snapshot_encryption_key = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_pool`.\n"]
    pub fn set_storage_pool(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.storage_pool = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeInstanceTemplateDiskEl {
    type O = BlockAssignable<DataComputeInstanceTemplateDiskEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeInstanceTemplateDiskEl {}
impl BuildDataComputeInstanceTemplateDiskEl {
    pub fn build(self) -> DataComputeInstanceTemplateDiskEl {
        DataComputeInstanceTemplateDiskEl {
            architecture: core::default::Default::default(),
            auto_delete: core::default::Default::default(),
            boot: core::default::Default::default(),
            device_name: core::default::Default::default(),
            disk_encryption_key: core::default::Default::default(),
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
            source_image_encryption_key: core::default::Default::default(),
            source_snapshot: core::default::Default::default(),
            source_snapshot_encryption_key: core::default::Default::default(),
            storage_pool: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataComputeInstanceTemplateDiskElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeInstanceTemplateDiskElRef {
    fn new(shared: StackShared, base: String) -> DataComputeInstanceTemplateDiskElRef {
        DataComputeInstanceTemplateDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeInstanceTemplateDiskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `architecture` after provisioning.\n"]
    pub fn architecture(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.base))
    }
    #[doc = "Get a reference to the value of field `auto_delete` after provisioning.\n"]
    pub fn auto_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_delete", self.base))
    }
    #[doc = "Get a reference to the value of field `boot` after provisioning.\n"]
    pub fn boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.boot", self.base))
    }
    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }
    #[doc = "Get a reference to the value of field `disk_encryption_key` after provisioning.\n"]
    pub fn disk_encryption_key(
        &self,
    ) -> ListRef<DataComputeInstanceTemplateDiskElDiskEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk_encryption_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disk_name` after provisioning.\n"]
    pub fn disk_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_name", self.base))
    }
    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\n"]
    pub fn disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `disk_type` after provisioning.\n"]
    pub fn disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }
    #[doc = "Get a reference to the value of field `guest_os_features` after provisioning.\n"]
    pub fn guest_os_features(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.guest_os_features", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `interface` after provisioning.\n"]
    pub fn interface(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interface", self.base))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `provisioned_iops` after provisioning.\n"]
    pub fn provisioned_iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_iops", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `provisioned_throughput` after provisioning.\n"]
    pub fn provisioned_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_throughput", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\n"]
    pub fn resource_manager_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_manager_tags", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_policies` after provisioning.\n"]
    pub fn resource_policies(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_policies", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
    #[doc = "Get a reference to the value of field `source_image` after provisioning.\n"]
    pub fn source_image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_image", self.base))
    }
    #[doc = "Get a reference to the value of field `source_image_encryption_key` after provisioning.\n"]
    pub fn source_image_encryption_key(
        &self,
    ) -> ListRef<DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_image_encryption_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_snapshot` after provisioning.\n"]
    pub fn source_snapshot(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_snapshot", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_snapshot_encryption_key` after provisioning.\n"]
    pub fn source_snapshot_encryption_key(
        &self,
    ) -> ListRef<DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_snapshot_encryption_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `storage_pool` after provisioning.\n"]
    pub fn storage_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_pool", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeInstanceTemplateGuestAcceleratorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataComputeInstanceTemplateGuestAcceleratorEl {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeInstanceTemplateGuestAcceleratorEl {
    type O = BlockAssignable<DataComputeInstanceTemplateGuestAcceleratorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeInstanceTemplateGuestAcceleratorEl {}
impl BuildDataComputeInstanceTemplateGuestAcceleratorEl {
    pub fn build(self) -> DataComputeInstanceTemplateGuestAcceleratorEl {
        DataComputeInstanceTemplateGuestAcceleratorEl {
            count: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataComputeInstanceTemplateGuestAcceleratorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeInstanceTemplateGuestAcceleratorElRef {
    fn new(shared: StackShared, base: String) -> DataComputeInstanceTemplateGuestAcceleratorElRef {
        DataComputeInstanceTemplateGuestAcceleratorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeInstanceTemplateGuestAcceleratorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeInstanceTemplateNetworkInterfaceElAccessConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nat_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_tier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_ptr_domain_name: Option<PrimField<String>>,
}
impl DataComputeInstanceTemplateNetworkInterfaceElAccessConfigEl {
    #[doc = "Set the field `nat_ip`.\n"]
    pub fn set_nat_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nat_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `network_tier`.\n"]
    pub fn set_network_tier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_tier = Some(v.into());
        self
    }
    #[doc = "Set the field `public_ptr_domain_name`.\n"]
    pub fn set_public_ptr_domain_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.public_ptr_domain_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeInstanceTemplateNetworkInterfaceElAccessConfigEl {
    type O = BlockAssignable<DataComputeInstanceTemplateNetworkInterfaceElAccessConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeInstanceTemplateNetworkInterfaceElAccessConfigEl {}
impl BuildDataComputeInstanceTemplateNetworkInterfaceElAccessConfigEl {
    pub fn build(self) -> DataComputeInstanceTemplateNetworkInterfaceElAccessConfigEl {
        DataComputeInstanceTemplateNetworkInterfaceElAccessConfigEl {
            nat_ip: core::default::Default::default(),
            network_tier: core::default::Default::default(),
            public_ptr_domain_name: core::default::Default::default(),
        }
    }
}
pub struct DataComputeInstanceTemplateNetworkInterfaceElAccessConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeInstanceTemplateNetworkInterfaceElAccessConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeInstanceTemplateNetworkInterfaceElAccessConfigElRef {
        DataComputeInstanceTemplateNetworkInterfaceElAccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeInstanceTemplateNetworkInterfaceElAccessConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nat_ip` after provisioning.\n"]
    pub fn nat_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nat_ip", self.base))
    }
    #[doc = "Get a reference to the value of field `network_tier` after provisioning.\n"]
    pub fn network_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_tier", self.base))
    }
    #[doc = "Get a reference to the value of field `public_ptr_domain_name` after provisioning.\n"]
    pub fn public_ptr_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_ptr_domain_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_cidr_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork_range_name: Option<PrimField<String>>,
}
impl DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl {
    #[doc = "Set the field `ip_cidr_range`.\n"]
    pub fn set_ip_cidr_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_cidr_range = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork_range_name`.\n"]
    pub fn set_subnetwork_range_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork_range_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl {
    type O = BlockAssignable<DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl {}
impl BuildDataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl {
    pub fn build(self) -> DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl {
        DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl {
            ip_cidr_range: core::default::Default::default(),
            subnetwork_range_name: core::default::Default::default(),
        }
    }
}
pub struct DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeElRef {
        DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_cidr_range` after provisioning.\n"]
    pub fn ip_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_cidr_range", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnetwork_range_name` after provisioning.\n"]
    pub fn subnetwork_range_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnetwork_range_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    external_ipv6: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_ipv6_prefix_length: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_tier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_ptr_domain_name: Option<PrimField<String>>,
}
impl DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl {
    #[doc = "Set the field `external_ipv6`.\n"]
    pub fn set_external_ipv6(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.external_ipv6 = Some(v.into());
        self
    }
    #[doc = "Set the field `external_ipv6_prefix_length`.\n"]
    pub fn set_external_ipv6_prefix_length(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.external_ipv6_prefix_length = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `network_tier`.\n"]
    pub fn set_network_tier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_tier = Some(v.into());
        self
    }
    #[doc = "Set the field `public_ptr_domain_name`.\n"]
    pub fn set_public_ptr_domain_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.public_ptr_domain_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl {
    type O = BlockAssignable<DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl {}
impl BuildDataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl {
    pub fn build(self) -> DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl {
        DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl {
            external_ipv6: core::default::Default::default(),
            external_ipv6_prefix_length: core::default::Default::default(),
            name: core::default::Default::default(),
            network_tier: core::default::Default::default(),
            public_ptr_domain_name: core::default::Default::default(),
        }
    }
}
pub struct DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigElRef {
        DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `external_ipv6` after provisioning.\n"]
    pub fn external_ipv6(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_ipv6", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `external_ipv6_prefix_length` after provisioning.\n"]
    pub fn external_ipv6_prefix_length(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_ipv6_prefix_length", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `network_tier` after provisioning.\n"]
    pub fn network_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_tier", self.base))
    }
    #[doc = "Get a reference to the value of field `public_ptr_domain_name` after provisioning.\n"]
    pub fn public_ptr_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_ptr_domain_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeInstanceTemplateNetworkInterfaceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_config: Option<ListField<DataComputeInstanceTemplateNetworkInterfaceElAccessConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alias_ip_range: Option<ListField<DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    igmp_query: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_ipv6_prefix_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_access_config:
        Option<ListField<DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_access_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_attachment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nic_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_nic_name: Option<PrimField<String>>,
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
}
impl DataComputeInstanceTemplateNetworkInterfaceEl {
    #[doc = "Set the field `access_config`.\n"]
    pub fn set_access_config(
        mut self,
        v: impl Into<ListField<DataComputeInstanceTemplateNetworkInterfaceElAccessConfigEl>>,
    ) -> Self {
        self.access_config = Some(v.into());
        self
    }
    #[doc = "Set the field `alias_ip_range`.\n"]
    pub fn set_alias_ip_range(
        mut self,
        v: impl Into<ListField<DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl>>,
    ) -> Self {
        self.alias_ip_range = Some(v.into());
        self
    }
    #[doc = "Set the field `igmp_query`.\n"]
    pub fn set_igmp_query(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.igmp_query = Some(v.into());
        self
    }
    #[doc = "Set the field `internal_ipv6_prefix_length`.\n"]
    pub fn set_internal_ipv6_prefix_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.internal_ipv6_prefix_length = Some(v.into());
        self
    }
    #[doc = "Set the field `ipv6_access_config`.\n"]
    pub fn set_ipv6_access_config(
        mut self,
        v: impl Into<ListField<DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl>>,
    ) -> Self {
        self.ipv6_access_config = Some(v.into());
        self
    }
    #[doc = "Set the field `ipv6_access_type`.\n"]
    pub fn set_ipv6_access_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv6_access_type = Some(v.into());
        self
    }
    #[doc = "Set the field `ipv6_address`.\n"]
    pub fn set_ipv6_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv6_address = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `network_attachment`.\n"]
    pub fn set_network_attachment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_attachment = Some(v.into());
        self
    }
    #[doc = "Set the field `network_ip`.\n"]
    pub fn set_network_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `nic_type`.\n"]
    pub fn set_nic_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nic_type = Some(v.into());
        self
    }
    #[doc = "Set the field `parent_nic_name`.\n"]
    pub fn set_parent_nic_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.parent_nic_name = Some(v.into());
        self
    }
    #[doc = "Set the field `queue_count`.\n"]
    pub fn set_queue_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.queue_count = Some(v.into());
        self
    }
    #[doc = "Set the field `stack_type`.\n"]
    pub fn set_stack_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stack_type = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\n"]
    pub fn set_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork_project`.\n"]
    pub fn set_subnetwork_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork_project = Some(v.into());
        self
    }
    #[doc = "Set the field `vlan`.\n"]
    pub fn set_vlan(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.vlan = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeInstanceTemplateNetworkInterfaceEl {
    type O = BlockAssignable<DataComputeInstanceTemplateNetworkInterfaceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeInstanceTemplateNetworkInterfaceEl {}
impl BuildDataComputeInstanceTemplateNetworkInterfaceEl {
    pub fn build(self) -> DataComputeInstanceTemplateNetworkInterfaceEl {
        DataComputeInstanceTemplateNetworkInterfaceEl {
            access_config: core::default::Default::default(),
            alias_ip_range: core::default::Default::default(),
            igmp_query: core::default::Default::default(),
            internal_ipv6_prefix_length: core::default::Default::default(),
            ipv6_access_config: core::default::Default::default(),
            ipv6_access_type: core::default::Default::default(),
            ipv6_address: core::default::Default::default(),
            name: core::default::Default::default(),
            network: core::default::Default::default(),
            network_attachment: core::default::Default::default(),
            network_ip: core::default::Default::default(),
            nic_type: core::default::Default::default(),
            parent_nic_name: core::default::Default::default(),
            queue_count: core::default::Default::default(),
            stack_type: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            subnetwork_project: core::default::Default::default(),
            vlan: core::default::Default::default(),
        }
    }
}
pub struct DataComputeInstanceTemplateNetworkInterfaceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeInstanceTemplateNetworkInterfaceElRef {
    fn new(shared: StackShared, base: String) -> DataComputeInstanceTemplateNetworkInterfaceElRef {
        DataComputeInstanceTemplateNetworkInterfaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeInstanceTemplateNetworkInterfaceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_config` after provisioning.\n"]
    pub fn access_config(
        &self,
    ) -> ListRef<DataComputeInstanceTemplateNetworkInterfaceElAccessConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `alias_ip_range` after provisioning.\n"]
    pub fn alias_ip_range(
        &self,
    ) -> ListRef<DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.alias_ip_range", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `igmp_query` after provisioning.\n"]
    pub fn igmp_query(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.igmp_query", self.base))
    }
    #[doc = "Get a reference to the value of field `internal_ipv6_prefix_length` after provisioning.\n"]
    pub fn internal_ipv6_prefix_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.internal_ipv6_prefix_length", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_access_config` after provisioning.\n"]
    pub fn ipv6_access_config(
        &self,
    ) -> ListRef<DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ipv6_access_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_access_type` after provisioning.\n"]
    pub fn ipv6_access_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_access_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_address` after provisioning.\n"]
    pub fn ipv6_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `network_attachment` after provisioning.\n"]
    pub fn network_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_attachment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network_ip` after provisioning.\n"]
    pub fn network_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_ip", self.base))
    }
    #[doc = "Get a reference to the value of field `nic_type` after provisioning.\n"]
    pub fn nic_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nic_type", self.base))
    }
    #[doc = "Get a reference to the value of field `parent_nic_name` after provisioning.\n"]
    pub fn parent_nic_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_nic_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `queue_count` after provisioning.\n"]
    pub fn queue_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.queue_count", self.base))
    }
    #[doc = "Get a reference to the value of field `stack_type` after provisioning.\n"]
    pub fn stack_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_type", self.base))
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\n"]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
    #[doc = "Get a reference to the value of field `subnetwork_project` after provisioning.\n"]
    pub fn subnetwork_project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnetwork_project", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vlan` after provisioning.\n"]
    pub fn vlan(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vlan", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeInstanceTemplateNetworkPerformanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    total_egress_bandwidth_tier: Option<PrimField<String>>,
}
impl DataComputeInstanceTemplateNetworkPerformanceConfigEl {
    #[doc = "Set the field `total_egress_bandwidth_tier`.\n"]
    pub fn set_total_egress_bandwidth_tier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.total_egress_bandwidth_tier = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeInstanceTemplateNetworkPerformanceConfigEl {
    type O = BlockAssignable<DataComputeInstanceTemplateNetworkPerformanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeInstanceTemplateNetworkPerformanceConfigEl {}
impl BuildDataComputeInstanceTemplateNetworkPerformanceConfigEl {
    pub fn build(self) -> DataComputeInstanceTemplateNetworkPerformanceConfigEl {
        DataComputeInstanceTemplateNetworkPerformanceConfigEl {
            total_egress_bandwidth_tier: core::default::Default::default(),
        }
    }
}
pub struct DataComputeInstanceTemplateNetworkPerformanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeInstanceTemplateNetworkPerformanceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeInstanceTemplateNetworkPerformanceConfigElRef {
        DataComputeInstanceTemplateNetworkPerformanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeInstanceTemplateNetworkPerformanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `total_egress_bandwidth_tier` after provisioning.\n"]
    pub fn total_egress_bandwidth_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_egress_bandwidth_tier", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeInstanceTemplateReservationAffinityElSpecificReservationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<ListField<PrimField<String>>>,
}
impl DataComputeInstanceTemplateReservationAffinityElSpecificReservationEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeInstanceTemplateReservationAffinityElSpecificReservationEl {
    type O = BlockAssignable<DataComputeInstanceTemplateReservationAffinityElSpecificReservationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeInstanceTemplateReservationAffinityElSpecificReservationEl {}
impl BuildDataComputeInstanceTemplateReservationAffinityElSpecificReservationEl {
    pub fn build(self) -> DataComputeInstanceTemplateReservationAffinityElSpecificReservationEl {
        DataComputeInstanceTemplateReservationAffinityElSpecificReservationEl {
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}
pub struct DataComputeInstanceTemplateReservationAffinityElSpecificReservationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeInstanceTemplateReservationAffinityElSpecificReservationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeInstanceTemplateReservationAffinityElSpecificReservationElRef {
        DataComputeInstanceTemplateReservationAffinityElSpecificReservationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeInstanceTemplateReservationAffinityElSpecificReservationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeInstanceTemplateReservationAffinityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    specific_reservation:
        Option<ListField<DataComputeInstanceTemplateReservationAffinityElSpecificReservationEl>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataComputeInstanceTemplateReservationAffinityEl {
    #[doc = "Set the field `specific_reservation`.\n"]
    pub fn set_specific_reservation(
        mut self,
        v: impl Into<ListField<DataComputeInstanceTemplateReservationAffinityElSpecificReservationEl>>,
    ) -> Self {
        self.specific_reservation = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeInstanceTemplateReservationAffinityEl {
    type O = BlockAssignable<DataComputeInstanceTemplateReservationAffinityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeInstanceTemplateReservationAffinityEl {}
impl BuildDataComputeInstanceTemplateReservationAffinityEl {
    pub fn build(self) -> DataComputeInstanceTemplateReservationAffinityEl {
        DataComputeInstanceTemplateReservationAffinityEl {
            specific_reservation: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataComputeInstanceTemplateReservationAffinityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeInstanceTemplateReservationAffinityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeInstanceTemplateReservationAffinityElRef {
        DataComputeInstanceTemplateReservationAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeInstanceTemplateReservationAffinityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `specific_reservation` after provisioning.\n"]
    pub fn specific_reservation(
        &self,
    ) -> ListRef<DataComputeInstanceTemplateReservationAffinityElSpecificReservationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.specific_reservation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
    type O = BlockAssignable<DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {}
impl BuildDataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
    pub fn build(self) -> DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
        DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutElRef {
        DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeInstanceTemplateSchedulingElMaxRunDurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl DataComputeInstanceTemplateSchedulingElMaxRunDurationEl {
    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeInstanceTemplateSchedulingElMaxRunDurationEl {
    type O = BlockAssignable<DataComputeInstanceTemplateSchedulingElMaxRunDurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeInstanceTemplateSchedulingElMaxRunDurationEl {}
impl BuildDataComputeInstanceTemplateSchedulingElMaxRunDurationEl {
    pub fn build(self) -> DataComputeInstanceTemplateSchedulingElMaxRunDurationEl {
        DataComputeInstanceTemplateSchedulingElMaxRunDurationEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct DataComputeInstanceTemplateSchedulingElMaxRunDurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeInstanceTemplateSchedulingElMaxRunDurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeInstanceTemplateSchedulingElMaxRunDurationElRef {
        DataComputeInstanceTemplateSchedulingElMaxRunDurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeInstanceTemplateSchedulingElMaxRunDurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeInstanceTemplateSchedulingElNodeAffinitiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}
impl DataComputeInstanceTemplateSchedulingElNodeAffinitiesEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operator = Some(v.into());
        self
    }
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeInstanceTemplateSchedulingElNodeAffinitiesEl {
    type O = BlockAssignable<DataComputeInstanceTemplateSchedulingElNodeAffinitiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeInstanceTemplateSchedulingElNodeAffinitiesEl {}
impl BuildDataComputeInstanceTemplateSchedulingElNodeAffinitiesEl {
    pub fn build(self) -> DataComputeInstanceTemplateSchedulingElNodeAffinitiesEl {
        DataComputeInstanceTemplateSchedulingElNodeAffinitiesEl {
            key: core::default::Default::default(),
            operator: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}
pub struct DataComputeInstanceTemplateSchedulingElNodeAffinitiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeInstanceTemplateSchedulingElNodeAffinitiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeInstanceTemplateSchedulingElNodeAffinitiesElRef {
        DataComputeInstanceTemplateSchedulingElNodeAffinitiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeInstanceTemplateSchedulingElNodeAffinitiesElRef {
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
pub struct DataComputeInstanceTemplateSchedulingElOnInstanceStopActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    discard_local_ssd: Option<PrimField<bool>>,
}
impl DataComputeInstanceTemplateSchedulingElOnInstanceStopActionEl {
    #[doc = "Set the field `discard_local_ssd`.\n"]
    pub fn set_discard_local_ssd(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.discard_local_ssd = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeInstanceTemplateSchedulingElOnInstanceStopActionEl {
    type O = BlockAssignable<DataComputeInstanceTemplateSchedulingElOnInstanceStopActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeInstanceTemplateSchedulingElOnInstanceStopActionEl {}
impl BuildDataComputeInstanceTemplateSchedulingElOnInstanceStopActionEl {
    pub fn build(self) -> DataComputeInstanceTemplateSchedulingElOnInstanceStopActionEl {
        DataComputeInstanceTemplateSchedulingElOnInstanceStopActionEl {
            discard_local_ssd: core::default::Default::default(),
        }
    }
}
pub struct DataComputeInstanceTemplateSchedulingElOnInstanceStopActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeInstanceTemplateSchedulingElOnInstanceStopActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeInstanceTemplateSchedulingElOnInstanceStopActionElRef {
        DataComputeInstanceTemplateSchedulingElOnInstanceStopActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeInstanceTemplateSchedulingElOnInstanceStopActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `discard_local_ssd` after provisioning.\n"]
    pub fn discard_local_ssd(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.discard_local_ssd", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeInstanceTemplateSchedulingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_restart: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_domain: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_termination_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ssd_recovery_timeout:
        Option<ListField<DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_run_duration: Option<ListField<DataComputeInstanceTemplateSchedulingElMaxRunDurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_node_cpus: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_affinities: Option<SetField<DataComputeInstanceTemplateSchedulingElNodeAffinitiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_host_maintenance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_instance_stop_action:
        Option<ListField<DataComputeInstanceTemplateSchedulingElOnInstanceStopActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preemptible: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioning_model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    termination_time: Option<PrimField<String>>,
}
impl DataComputeInstanceTemplateSchedulingEl {
    #[doc = "Set the field `automatic_restart`.\n"]
    pub fn set_automatic_restart(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.automatic_restart = Some(v.into());
        self
    }
    #[doc = "Set the field `availability_domain`.\n"]
    pub fn set_availability_domain(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.availability_domain = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_termination_action`.\n"]
    pub fn set_instance_termination_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_termination_action = Some(v.into());
        self
    }
    #[doc = "Set the field `local_ssd_recovery_timeout`.\n"]
    pub fn set_local_ssd_recovery_timeout(
        mut self,
        v: impl Into<ListField<DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl>>,
    ) -> Self {
        self.local_ssd_recovery_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `max_run_duration`.\n"]
    pub fn set_max_run_duration(
        mut self,
        v: impl Into<ListField<DataComputeInstanceTemplateSchedulingElMaxRunDurationEl>>,
    ) -> Self {
        self.max_run_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `min_node_cpus`.\n"]
    pub fn set_min_node_cpus(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_node_cpus = Some(v.into());
        self
    }
    #[doc = "Set the field `node_affinities`.\n"]
    pub fn set_node_affinities(
        mut self,
        v: impl Into<SetField<DataComputeInstanceTemplateSchedulingElNodeAffinitiesEl>>,
    ) -> Self {
        self.node_affinities = Some(v.into());
        self
    }
    #[doc = "Set the field `on_host_maintenance`.\n"]
    pub fn set_on_host_maintenance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.on_host_maintenance = Some(v.into());
        self
    }
    #[doc = "Set the field `on_instance_stop_action`.\n"]
    pub fn set_on_instance_stop_action(
        mut self,
        v: impl Into<ListField<DataComputeInstanceTemplateSchedulingElOnInstanceStopActionEl>>,
    ) -> Self {
        self.on_instance_stop_action = Some(v.into());
        self
    }
    #[doc = "Set the field `preemptible`.\n"]
    pub fn set_preemptible(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preemptible = Some(v.into());
        self
    }
    #[doc = "Set the field `provisioning_model`.\n"]
    pub fn set_provisioning_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.provisioning_model = Some(v.into());
        self
    }
    #[doc = "Set the field `termination_time`.\n"]
    pub fn set_termination_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.termination_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeInstanceTemplateSchedulingEl {
    type O = BlockAssignable<DataComputeInstanceTemplateSchedulingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeInstanceTemplateSchedulingEl {}
impl BuildDataComputeInstanceTemplateSchedulingEl {
    pub fn build(self) -> DataComputeInstanceTemplateSchedulingEl {
        DataComputeInstanceTemplateSchedulingEl {
            automatic_restart: core::default::Default::default(),
            availability_domain: core::default::Default::default(),
            instance_termination_action: core::default::Default::default(),
            local_ssd_recovery_timeout: core::default::Default::default(),
            max_run_duration: core::default::Default::default(),
            min_node_cpus: core::default::Default::default(),
            node_affinities: core::default::Default::default(),
            on_host_maintenance: core::default::Default::default(),
            on_instance_stop_action: core::default::Default::default(),
            preemptible: core::default::Default::default(),
            provisioning_model: core::default::Default::default(),
            termination_time: core::default::Default::default(),
        }
    }
}
pub struct DataComputeInstanceTemplateSchedulingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeInstanceTemplateSchedulingElRef {
    fn new(shared: StackShared, base: String) -> DataComputeInstanceTemplateSchedulingElRef {
        DataComputeInstanceTemplateSchedulingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeInstanceTemplateSchedulingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `automatic_restart` after provisioning.\n"]
    pub fn automatic_restart(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.automatic_restart", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `availability_domain` after provisioning.\n"]
    pub fn availability_domain(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_domain", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance_termination_action` after provisioning.\n"]
    pub fn instance_termination_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_termination_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_ssd_recovery_timeout` after provisioning.\n"]
    pub fn local_ssd_recovery_timeout(
        &self,
    ) -> ListRef<DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.local_ssd_recovery_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_run_duration` after provisioning.\n"]
    pub fn max_run_duration(
        &self,
    ) -> ListRef<DataComputeInstanceTemplateSchedulingElMaxRunDurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.max_run_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_node_cpus` after provisioning.\n"]
    pub fn min_node_cpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_node_cpus", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_affinities` after provisioning.\n"]
    pub fn node_affinities(
        &self,
    ) -> SetRef<DataComputeInstanceTemplateSchedulingElNodeAffinitiesElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.node_affinities", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `on_host_maintenance` after provisioning.\n"]
    pub fn on_host_maintenance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_host_maintenance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `on_instance_stop_action` after provisioning.\n"]
    pub fn on_instance_stop_action(
        &self,
    ) -> ListRef<DataComputeInstanceTemplateSchedulingElOnInstanceStopActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.on_instance_stop_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `preemptible` after provisioning.\n"]
    pub fn preemptible(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preemptible", self.base))
    }
    #[doc = "Get a reference to the value of field `provisioning_model` after provisioning.\n"]
    pub fn provisioning_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioning_model", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `termination_time` after provisioning.\n"]
    pub fn termination_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.termination_time", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeInstanceTemplateServiceAccountEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scopes: Option<SetField<PrimField<String>>>,
}
impl DataComputeInstanceTemplateServiceAccountEl {
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email = Some(v.into());
        self
    }
    #[doc = "Set the field `scopes`.\n"]
    pub fn set_scopes(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.scopes = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeInstanceTemplateServiceAccountEl {
    type O = BlockAssignable<DataComputeInstanceTemplateServiceAccountEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeInstanceTemplateServiceAccountEl {}
impl BuildDataComputeInstanceTemplateServiceAccountEl {
    pub fn build(self) -> DataComputeInstanceTemplateServiceAccountEl {
        DataComputeInstanceTemplateServiceAccountEl {
            email: core::default::Default::default(),
            scopes: core::default::Default::default(),
        }
    }
}
pub struct DataComputeInstanceTemplateServiceAccountElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeInstanceTemplateServiceAccountElRef {
    fn new(shared: StackShared, base: String) -> DataComputeInstanceTemplateServiceAccountElRef {
        DataComputeInstanceTemplateServiceAccountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeInstanceTemplateServiceAccountElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
    #[doc = "Get a reference to the value of field `scopes` after provisioning.\n"]
    pub fn scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeInstanceTemplateShieldedInstanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_integrity_monitoring: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_secure_boot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_vtpm: Option<PrimField<bool>>,
}
impl DataComputeInstanceTemplateShieldedInstanceConfigEl {
    #[doc = "Set the field `enable_integrity_monitoring`.\n"]
    pub fn set_enable_integrity_monitoring(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_secure_boot`.\n"]
    pub fn set_enable_secure_boot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_secure_boot = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_vtpm`.\n"]
    pub fn set_enable_vtpm(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_vtpm = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeInstanceTemplateShieldedInstanceConfigEl {
    type O = BlockAssignable<DataComputeInstanceTemplateShieldedInstanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeInstanceTemplateShieldedInstanceConfigEl {}
impl BuildDataComputeInstanceTemplateShieldedInstanceConfigEl {
    pub fn build(self) -> DataComputeInstanceTemplateShieldedInstanceConfigEl {
        DataComputeInstanceTemplateShieldedInstanceConfigEl {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
            enable_vtpm: core::default::Default::default(),
        }
    }
}
pub struct DataComputeInstanceTemplateShieldedInstanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeInstanceTemplateShieldedInstanceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeInstanceTemplateShieldedInstanceConfigElRef {
        DataComputeInstanceTemplateShieldedInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeInstanceTemplateShieldedInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\n"]
    pub fn enable_integrity_monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_integrity_monitoring", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_secure_boot` after provisioning.\n"]
    pub fn enable_secure_boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_secure_boot", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_vtpm` after provisioning.\n"]
    pub fn enable_vtpm(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_vtpm", self.base))
    }
}
