use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeRegionInstanceGroupManagerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_link: Option<PrimField<String>>,
}
struct DataComputeRegionInstanceGroupManager_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeRegionInstanceGroupManagerData>,
}
#[derive(Clone)]
pub struct DataComputeRegionInstanceGroupManager(Rc<DataComputeRegionInstanceGroupManager_>);
impl DataComputeRegionInstanceGroupManager {
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
    #[doc = "Set the field `name`.\nThe name of the instance group manager. Must be 1-63 characters long and comply with RFC1035. Supported characters include lowercase letters, numbers, and hyphens."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nThe region where the managed instance group resides."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `self_link`.\nThe URL of the created resource."]
    pub fn set_self_link(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().self_link = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `all_instances_config` after provisioning.\nSpecifies configuration that overrides the instance template configuration for the group."]
    pub fn all_instances_config(
        &self,
    ) -> ListRef<DataComputeRegionInstanceGroupManagerAllInstancesConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.all_instances_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auto_healing_policies` after provisioning.\nThe autohealing policies for this managed instance group. You can specify only one value."]
    pub fn auto_healing_policies(
        &self,
    ) -> ListRef<DataComputeRegionInstanceGroupManagerAutoHealingPoliciesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_healing_policies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `base_instance_name` after provisioning.\nThe base instance name to use for instances in this group. The value must be a valid RFC1035 name. Supported characters are lowercase letters, numbers, and hyphens (-). Instances are named by appending a hyphen and a random four-character string to the base instance name."]
    pub fn base_instance_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_instance_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional textual description of the instance group manager."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `distribution_policy_target_shape` after provisioning.\nThe shape to which the group converges either proactively or on resize events (depending on the value set in updatePolicy.instanceRedistributionType)."]
    pub fn distribution_policy_target_shape(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.distribution_policy_target_shape", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `distribution_policy_zones` after provisioning.\nThe distribution policy for this managed instance group. You can specify one or more values."]
    pub fn distribution_policy_zones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.distribution_policy_zones", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nThe fingerprint of the instance group manager."]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_flexibility_policy` after provisioning.\nThe flexibility policy for this managed instance group. Instance flexibility allowing MIG to create VMs from multiple types of machines. Instance flexibility configuration on MIG overrides instance template configuration."]
    pub fn instance_flexibility_policy(
        &self,
    ) -> ListRef<DataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_flexibility_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_group` after provisioning.\nThe full URL of the instance group created by the manager."]
    pub fn instance_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_group_manager_id` after provisioning.\nThe unique identifier number for the resource. This identifier is defined by the server."]
    pub fn instance_group_manager_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_group_manager_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_lifecycle_policy` after provisioning.\nThe instance lifecycle policy for this managed instance group."]
    pub fn instance_lifecycle_policy(
        &self,
    ) -> ListRef<DataComputeRegionInstanceGroupManagerInstanceLifecyclePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_lifecycle_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `list_managed_instances_results` after provisioning.\nPagination behavior of the listManagedInstances API method for this managed instance group. Valid values are: \"PAGELESS\", \"PAGINATED\". If PAGELESS (default), Pagination is disabled for the group's listManagedInstances API method. maxResults and pageToken query parameters are ignored and all instances are returned in a single response. If PAGINATED, pagination is enabled, maxResults and pageToken query parameters are respected."]
    pub fn list_managed_instances_results(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.list_managed_instances_results", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the instance group manager. Must be 1-63 characters long and comply with RFC1035. Supported characters include lowercase letters, numbers, and hyphens."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `named_port` after provisioning.\nThe named port configuration."]
    pub fn named_port(&self) -> SetRef<DataComputeRegionInstanceGroupManagerNamedPortElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.named_port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region where the managed instance group resides."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_policies` after provisioning.\nResource policies for this managed instance group."]
    pub fn resource_policies(
        &self,
    ) -> ListRef<DataComputeRegionInstanceGroupManagerResourcePoliciesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_policies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URL of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `standby_policy` after provisioning.\nStandby policy for stopped and suspended instances."]
    pub fn standby_policy(
        &self,
    ) -> ListRef<DataComputeRegionInstanceGroupManagerStandbyPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.standby_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stateful_disk` after provisioning.\nDisks created on the instances that will be preserved on instance delete, update, etc. Structure is documented below. For more information see the official documentation. Proactive cross zone instance redistribution must be disabled before you can update stateful disks on existing instance group managers. This can be controlled via the update_policy."]
    pub fn stateful_disk(&self) -> SetRef<DataComputeRegionInstanceGroupManagerStatefulDiskElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.stateful_disk", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stateful_external_ip` after provisioning.\nExternal IPs considered stateful by the instance group. "]
    pub fn stateful_external_ip(
        &self,
    ) -> ListRef<DataComputeRegionInstanceGroupManagerStatefulExternalIpElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stateful_external_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stateful_internal_ip` after provisioning.\nExternal IPs considered stateful by the instance group. "]
    pub fn stateful_internal_ip(
        &self,
    ) -> ListRef<DataComputeRegionInstanceGroupManagerStatefulInternalIpElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stateful_internal_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of this managed instance group."]
    pub fn status(&self) -> ListRef<DataComputeRegionInstanceGroupManagerStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_pools` after provisioning.\nThe full URL of all target pools to which new instances in the group are added. Updating the target pools attribute does not affect existing instances."]
    pub fn target_pools(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.target_pools", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_size` after provisioning.\nThe target number of running instances for this managed instance group. This value should always be explicitly set unless this resource is attached to an autoscaler, in which case it should never be set. Defaults to 0."]
    pub fn target_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_size_policy` after provisioning.\nThe policy that specifies how the MIG creates its VMs to achieve the target size."]
    pub fn target_size_policy(
        &self,
    ) -> ListRef<DataComputeRegionInstanceGroupManagerTargetSizePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_size_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_stopped_size` after provisioning.\nThe target number of stopped instances for this managed instance group."]
    pub fn target_stopped_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_stopped_size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_suspended_size` after provisioning.\nThe target number of suspended instances for this managed instance group."]
    pub fn target_suspended_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_suspended_size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_policy` after provisioning.\nThe update policy for this managed instance group."]
    pub fn update_policy(&self) -> ListRef<DataComputeRegionInstanceGroupManagerUpdatePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.update_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nApplication versions managed by this instance group. Each version deals with a specific instance template, allowing canary release scenarios."]
    pub fn version(&self) -> ListRef<DataComputeRegionInstanceGroupManagerVersionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `wait_for_instances` after provisioning.\nWhether to wait for all instances to be created/updated before returning. Note that if this is set to true and the operation does not succeed, Terraform will continue trying until it times out."]
    pub fn wait_for_instances(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_for_instances", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `wait_for_instances_status` after provisioning.\nWhen used with wait_for_instances specifies the status to wait for. When STABLE is specified this resource will wait until the instances are stable before returning. When UPDATED is set, it will wait for the version target to be reached and any per instance configs to be effective and all instances configs to be effective as well as all instances to be stable before returning."]
    pub fn wait_for_instances_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_for_instances_status", self.extract_ref()),
        )
    }
}
impl Referable for DataComputeRegionInstanceGroupManager {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeRegionInstanceGroupManager {}
impl ToListMappable for DataComputeRegionInstanceGroupManager {
    type O = ListRef<DataComputeRegionInstanceGroupManagerRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeRegionInstanceGroupManager_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_region_instance_group_manager".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeRegionInstanceGroupManager {
    pub tf_id: String,
}
impl BuildDataComputeRegionInstanceGroupManager {
    pub fn build(self, stack: &mut Stack) -> DataComputeRegionInstanceGroupManager {
        let out = DataComputeRegionInstanceGroupManager(Rc::new(
            DataComputeRegionInstanceGroupManager_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataComputeRegionInstanceGroupManagerData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    id: core::default::Default::default(),
                    name: core::default::Default::default(),
                    project: core::default::Default::default(),
                    region: core::default::Default::default(),
                    self_link: core::default::Default::default(),
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeRegionInstanceGroupManagerRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionInstanceGroupManagerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeRegionInstanceGroupManagerRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `all_instances_config` after provisioning.\nSpecifies configuration that overrides the instance template configuration for the group."]
    pub fn all_instances_config(
        &self,
    ) -> ListRef<DataComputeRegionInstanceGroupManagerAllInstancesConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.all_instances_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auto_healing_policies` after provisioning.\nThe autohealing policies for this managed instance group. You can specify only one value."]
    pub fn auto_healing_policies(
        &self,
    ) -> ListRef<DataComputeRegionInstanceGroupManagerAutoHealingPoliciesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_healing_policies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `base_instance_name` after provisioning.\nThe base instance name to use for instances in this group. The value must be a valid RFC1035 name. Supported characters are lowercase letters, numbers, and hyphens (-). Instances are named by appending a hyphen and a random four-character string to the base instance name."]
    pub fn base_instance_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_instance_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional textual description of the instance group manager."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `distribution_policy_target_shape` after provisioning.\nThe shape to which the group converges either proactively or on resize events (depending on the value set in updatePolicy.instanceRedistributionType)."]
    pub fn distribution_policy_target_shape(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.distribution_policy_target_shape", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `distribution_policy_zones` after provisioning.\nThe distribution policy for this managed instance group. You can specify one or more values."]
    pub fn distribution_policy_zones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.distribution_policy_zones", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nThe fingerprint of the instance group manager."]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_flexibility_policy` after provisioning.\nThe flexibility policy for this managed instance group. Instance flexibility allowing MIG to create VMs from multiple types of machines. Instance flexibility configuration on MIG overrides instance template configuration."]
    pub fn instance_flexibility_policy(
        &self,
    ) -> ListRef<DataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_flexibility_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_group` after provisioning.\nThe full URL of the instance group created by the manager."]
    pub fn instance_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_group_manager_id` after provisioning.\nThe unique identifier number for the resource. This identifier is defined by the server."]
    pub fn instance_group_manager_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_group_manager_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_lifecycle_policy` after provisioning.\nThe instance lifecycle policy for this managed instance group."]
    pub fn instance_lifecycle_policy(
        &self,
    ) -> ListRef<DataComputeRegionInstanceGroupManagerInstanceLifecyclePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_lifecycle_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `list_managed_instances_results` after provisioning.\nPagination behavior of the listManagedInstances API method for this managed instance group. Valid values are: \"PAGELESS\", \"PAGINATED\". If PAGELESS (default), Pagination is disabled for the group's listManagedInstances API method. maxResults and pageToken query parameters are ignored and all instances are returned in a single response. If PAGINATED, pagination is enabled, maxResults and pageToken query parameters are respected."]
    pub fn list_managed_instances_results(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.list_managed_instances_results", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the instance group manager. Must be 1-63 characters long and comply with RFC1035. Supported characters include lowercase letters, numbers, and hyphens."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `named_port` after provisioning.\nThe named port configuration."]
    pub fn named_port(&self) -> SetRef<DataComputeRegionInstanceGroupManagerNamedPortElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.named_port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region where the managed instance group resides."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_policies` after provisioning.\nResource policies for this managed instance group."]
    pub fn resource_policies(
        &self,
    ) -> ListRef<DataComputeRegionInstanceGroupManagerResourcePoliciesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_policies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URL of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `standby_policy` after provisioning.\nStandby policy for stopped and suspended instances."]
    pub fn standby_policy(
        &self,
    ) -> ListRef<DataComputeRegionInstanceGroupManagerStandbyPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.standby_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stateful_disk` after provisioning.\nDisks created on the instances that will be preserved on instance delete, update, etc. Structure is documented below. For more information see the official documentation. Proactive cross zone instance redistribution must be disabled before you can update stateful disks on existing instance group managers. This can be controlled via the update_policy."]
    pub fn stateful_disk(&self) -> SetRef<DataComputeRegionInstanceGroupManagerStatefulDiskElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.stateful_disk", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stateful_external_ip` after provisioning.\nExternal IPs considered stateful by the instance group. "]
    pub fn stateful_external_ip(
        &self,
    ) -> ListRef<DataComputeRegionInstanceGroupManagerStatefulExternalIpElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stateful_external_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stateful_internal_ip` after provisioning.\nExternal IPs considered stateful by the instance group. "]
    pub fn stateful_internal_ip(
        &self,
    ) -> ListRef<DataComputeRegionInstanceGroupManagerStatefulInternalIpElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stateful_internal_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of this managed instance group."]
    pub fn status(&self) -> ListRef<DataComputeRegionInstanceGroupManagerStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_pools` after provisioning.\nThe full URL of all target pools to which new instances in the group are added. Updating the target pools attribute does not affect existing instances."]
    pub fn target_pools(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.target_pools", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_size` after provisioning.\nThe target number of running instances for this managed instance group. This value should always be explicitly set unless this resource is attached to an autoscaler, in which case it should never be set. Defaults to 0."]
    pub fn target_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_size_policy` after provisioning.\nThe policy that specifies how the MIG creates its VMs to achieve the target size."]
    pub fn target_size_policy(
        &self,
    ) -> ListRef<DataComputeRegionInstanceGroupManagerTargetSizePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_size_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_stopped_size` after provisioning.\nThe target number of stopped instances for this managed instance group."]
    pub fn target_stopped_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_stopped_size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_suspended_size` after provisioning.\nThe target number of suspended instances for this managed instance group."]
    pub fn target_suspended_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_suspended_size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_policy` after provisioning.\nThe update policy for this managed instance group."]
    pub fn update_policy(&self) -> ListRef<DataComputeRegionInstanceGroupManagerUpdatePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.update_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nApplication versions managed by this instance group. Each version deals with a specific instance template, allowing canary release scenarios."]
    pub fn version(&self) -> ListRef<DataComputeRegionInstanceGroupManagerVersionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `wait_for_instances` after provisioning.\nWhether to wait for all instances to be created/updated before returning. Note that if this is set to true and the operation does not succeed, Terraform will continue trying until it times out."]
    pub fn wait_for_instances(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_for_instances", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `wait_for_instances_status` after provisioning.\nWhen used with wait_for_instances specifies the status to wait for. When STABLE is specified this resource will wait until the instances are stable before returning. When UPDATED is set, it will wait for the version target to be reached and any per instance configs to be effective and all instances configs to be effective as well as all instances to be stable before returning."]
    pub fn wait_for_instances_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_for_instances_status", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionInstanceGroupManagerAllInstancesConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RecField<PrimField<String>>>,
}
impl DataComputeRegionInstanceGroupManagerAllInstancesConfigEl {
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.metadata = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionInstanceGroupManagerAllInstancesConfigEl {
    type O = BlockAssignable<DataComputeRegionInstanceGroupManagerAllInstancesConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionInstanceGroupManagerAllInstancesConfigEl {}
impl BuildDataComputeRegionInstanceGroupManagerAllInstancesConfigEl {
    pub fn build(self) -> DataComputeRegionInstanceGroupManagerAllInstancesConfigEl {
        DataComputeRegionInstanceGroupManagerAllInstancesConfigEl {
            labels: core::default::Default::default(),
            metadata: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionInstanceGroupManagerAllInstancesConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionInstanceGroupManagerAllInstancesConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionInstanceGroupManagerAllInstancesConfigElRef {
        DataComputeRegionInstanceGroupManagerAllInstancesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionInstanceGroupManagerAllInstancesConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionInstanceGroupManagerAutoHealingPoliciesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_delay_sec: Option<PrimField<f64>>,
}
impl DataComputeRegionInstanceGroupManagerAutoHealingPoliciesEl {
    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.health_check = Some(v.into());
        self
    }
    #[doc = "Set the field `initial_delay_sec`.\n"]
    pub fn set_initial_delay_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.initial_delay_sec = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionInstanceGroupManagerAutoHealingPoliciesEl {
    type O = BlockAssignable<DataComputeRegionInstanceGroupManagerAutoHealingPoliciesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionInstanceGroupManagerAutoHealingPoliciesEl {}
impl BuildDataComputeRegionInstanceGroupManagerAutoHealingPoliciesEl {
    pub fn build(self) -> DataComputeRegionInstanceGroupManagerAutoHealingPoliciesEl {
        DataComputeRegionInstanceGroupManagerAutoHealingPoliciesEl {
            health_check: core::default::Default::default(),
            initial_delay_sec: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionInstanceGroupManagerAutoHealingPoliciesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionInstanceGroupManagerAutoHealingPoliciesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionInstanceGroupManagerAutoHealingPoliciesElRef {
        DataComputeRegionInstanceGroupManagerAutoHealingPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionInstanceGroupManagerAutoHealingPoliciesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.health_check", self.base))
    }
    #[doc = "Get a reference to the value of field `initial_delay_sec` after provisioning.\n"]
    pub fn initial_delay_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_delay_sec", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyElInstanceSelectionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rank: Option<PrimField<f64>>,
}
impl DataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyElInstanceSelectionsEl {
    #[doc = "Set the field `machine_types`.\n"]
    pub fn set_machine_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.machine_types = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `rank`.\n"]
    pub fn set_rank(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.rank = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyElInstanceSelectionsEl
{
    type O = BlockAssignable<
        DataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyElInstanceSelectionsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyElInstanceSelectionsEl
{}
impl BuildDataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyElInstanceSelectionsEl {
    pub fn build(
        self,
    ) -> DataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyElInstanceSelectionsEl {
        DataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyElInstanceSelectionsEl {
            machine_types: core::default::Default::default(),
            name: core::default::Default::default(),
            rank: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyElInstanceSelectionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyElInstanceSelectionsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyElInstanceSelectionsElRef
    {
        DataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyElInstanceSelectionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyElInstanceSelectionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `machine_types` after provisioning.\n"]
    pub fn machine_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.machine_types", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `rank` after provisioning.\n"]
    pub fn rank(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.rank", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_selections: Option<
        SetField<
            DataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyElInstanceSelectionsEl,
        >,
    >,
}
impl DataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyEl {
    #[doc = "Set the field `instance_selections`.\n"]
    pub fn set_instance_selections(
        mut self,
        v : impl Into < SetField < DataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyElInstanceSelectionsEl > >,
    ) -> Self {
        self.instance_selections = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyEl {
    type O = BlockAssignable<DataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyEl {}
impl BuildDataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyEl {
    pub fn build(self) -> DataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyEl {
        DataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyEl {
            instance_selections: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyElRef {
        DataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_selections` after provisioning.\n"]
    pub fn instance_selections(
        &self,
    ) -> SetRef<
        DataComputeRegionInstanceGroupManagerInstanceFlexibilityPolicyElInstanceSelectionsElRef,
    > {
        SetRef::new(
            self.shared().clone(),
            format!("{}.instance_selections", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionInstanceGroupManagerInstanceLifecyclePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_action_on_failure: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_update_on_repair: Option<PrimField<String>>,
}
impl DataComputeRegionInstanceGroupManagerInstanceLifecyclePolicyEl {
    #[doc = "Set the field `default_action_on_failure`.\n"]
    pub fn set_default_action_on_failure(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_action_on_failure = Some(v.into());
        self
    }
    #[doc = "Set the field `force_update_on_repair`.\n"]
    pub fn set_force_update_on_repair(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.force_update_on_repair = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionInstanceGroupManagerInstanceLifecyclePolicyEl {
    type O = BlockAssignable<DataComputeRegionInstanceGroupManagerInstanceLifecyclePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionInstanceGroupManagerInstanceLifecyclePolicyEl {}
impl BuildDataComputeRegionInstanceGroupManagerInstanceLifecyclePolicyEl {
    pub fn build(self) -> DataComputeRegionInstanceGroupManagerInstanceLifecyclePolicyEl {
        DataComputeRegionInstanceGroupManagerInstanceLifecyclePolicyEl {
            default_action_on_failure: core::default::Default::default(),
            force_update_on_repair: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionInstanceGroupManagerInstanceLifecyclePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionInstanceGroupManagerInstanceLifecyclePolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionInstanceGroupManagerInstanceLifecyclePolicyElRef {
        DataComputeRegionInstanceGroupManagerInstanceLifecyclePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionInstanceGroupManagerInstanceLifecyclePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_action_on_failure` after provisioning.\n"]
    pub fn default_action_on_failure(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_action_on_failure", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `force_update_on_repair` after provisioning.\n"]
    pub fn force_update_on_repair(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_update_on_repair", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionInstanceGroupManagerNamedPortEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl DataComputeRegionInstanceGroupManagerNamedPortEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionInstanceGroupManagerNamedPortEl {
    type O = BlockAssignable<DataComputeRegionInstanceGroupManagerNamedPortEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionInstanceGroupManagerNamedPortEl {}
impl BuildDataComputeRegionInstanceGroupManagerNamedPortEl {
    pub fn build(self) -> DataComputeRegionInstanceGroupManagerNamedPortEl {
        DataComputeRegionInstanceGroupManagerNamedPortEl {
            name: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionInstanceGroupManagerNamedPortElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionInstanceGroupManagerNamedPortElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionInstanceGroupManagerNamedPortElRef {
        DataComputeRegionInstanceGroupManagerNamedPortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionInstanceGroupManagerNamedPortElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionInstanceGroupManagerResourcePoliciesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    workload_policy: Option<PrimField<String>>,
}
impl DataComputeRegionInstanceGroupManagerResourcePoliciesEl {
    #[doc = "Set the field `workload_policy`.\n"]
    pub fn set_workload_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.workload_policy = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionInstanceGroupManagerResourcePoliciesEl {
    type O = BlockAssignable<DataComputeRegionInstanceGroupManagerResourcePoliciesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionInstanceGroupManagerResourcePoliciesEl {}
impl BuildDataComputeRegionInstanceGroupManagerResourcePoliciesEl {
    pub fn build(self) -> DataComputeRegionInstanceGroupManagerResourcePoliciesEl {
        DataComputeRegionInstanceGroupManagerResourcePoliciesEl {
            workload_policy: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionInstanceGroupManagerResourcePoliciesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionInstanceGroupManagerResourcePoliciesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionInstanceGroupManagerResourcePoliciesElRef {
        DataComputeRegionInstanceGroupManagerResourcePoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionInstanceGroupManagerResourcePoliciesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `workload_policy` after provisioning.\n"]
    pub fn workload_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workload_policy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionInstanceGroupManagerStandbyPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_delay_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}
impl DataComputeRegionInstanceGroupManagerStandbyPolicyEl {
    #[doc = "Set the field `initial_delay_sec`.\n"]
    pub fn set_initial_delay_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.initial_delay_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionInstanceGroupManagerStandbyPolicyEl {
    type O = BlockAssignable<DataComputeRegionInstanceGroupManagerStandbyPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionInstanceGroupManagerStandbyPolicyEl {}
impl BuildDataComputeRegionInstanceGroupManagerStandbyPolicyEl {
    pub fn build(self) -> DataComputeRegionInstanceGroupManagerStandbyPolicyEl {
        DataComputeRegionInstanceGroupManagerStandbyPolicyEl {
            initial_delay_sec: core::default::Default::default(),
            mode: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionInstanceGroupManagerStandbyPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionInstanceGroupManagerStandbyPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionInstanceGroupManagerStandbyPolicyElRef {
        DataComputeRegionInstanceGroupManagerStandbyPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionInstanceGroupManagerStandbyPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `initial_delay_sec` after provisioning.\n"]
    pub fn initial_delay_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_delay_sec", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionInstanceGroupManagerStatefulDiskEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_rule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_name: Option<PrimField<String>>,
}
impl DataComputeRegionInstanceGroupManagerStatefulDiskEl {
    #[doc = "Set the field `delete_rule`.\n"]
    pub fn set_delete_rule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete_rule = Some(v.into());
        self
    }
    #[doc = "Set the field `device_name`.\n"]
    pub fn set_device_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionInstanceGroupManagerStatefulDiskEl {
    type O = BlockAssignable<DataComputeRegionInstanceGroupManagerStatefulDiskEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionInstanceGroupManagerStatefulDiskEl {}
impl BuildDataComputeRegionInstanceGroupManagerStatefulDiskEl {
    pub fn build(self) -> DataComputeRegionInstanceGroupManagerStatefulDiskEl {
        DataComputeRegionInstanceGroupManagerStatefulDiskEl {
            delete_rule: core::default::Default::default(),
            device_name: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionInstanceGroupManagerStatefulDiskElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionInstanceGroupManagerStatefulDiskElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionInstanceGroupManagerStatefulDiskElRef {
        DataComputeRegionInstanceGroupManagerStatefulDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionInstanceGroupManagerStatefulDiskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `delete_rule` after provisioning.\n"]
    pub fn delete_rule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_rule", self.base))
    }
    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionInstanceGroupManagerStatefulExternalIpEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_rule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interface_name: Option<PrimField<String>>,
}
impl DataComputeRegionInstanceGroupManagerStatefulExternalIpEl {
    #[doc = "Set the field `delete_rule`.\n"]
    pub fn set_delete_rule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete_rule = Some(v.into());
        self
    }
    #[doc = "Set the field `interface_name`.\n"]
    pub fn set_interface_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interface_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionInstanceGroupManagerStatefulExternalIpEl {
    type O = BlockAssignable<DataComputeRegionInstanceGroupManagerStatefulExternalIpEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionInstanceGroupManagerStatefulExternalIpEl {}
impl BuildDataComputeRegionInstanceGroupManagerStatefulExternalIpEl {
    pub fn build(self) -> DataComputeRegionInstanceGroupManagerStatefulExternalIpEl {
        DataComputeRegionInstanceGroupManagerStatefulExternalIpEl {
            delete_rule: core::default::Default::default(),
            interface_name: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionInstanceGroupManagerStatefulExternalIpElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionInstanceGroupManagerStatefulExternalIpElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionInstanceGroupManagerStatefulExternalIpElRef {
        DataComputeRegionInstanceGroupManagerStatefulExternalIpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionInstanceGroupManagerStatefulExternalIpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `delete_rule` after provisioning.\n"]
    pub fn delete_rule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_rule", self.base))
    }
    #[doc = "Get a reference to the value of field `interface_name` after provisioning.\n"]
    pub fn interface_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.interface_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionInstanceGroupManagerStatefulInternalIpEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_rule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interface_name: Option<PrimField<String>>,
}
impl DataComputeRegionInstanceGroupManagerStatefulInternalIpEl {
    #[doc = "Set the field `delete_rule`.\n"]
    pub fn set_delete_rule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete_rule = Some(v.into());
        self
    }
    #[doc = "Set the field `interface_name`.\n"]
    pub fn set_interface_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interface_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionInstanceGroupManagerStatefulInternalIpEl {
    type O = BlockAssignable<DataComputeRegionInstanceGroupManagerStatefulInternalIpEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionInstanceGroupManagerStatefulInternalIpEl {}
impl BuildDataComputeRegionInstanceGroupManagerStatefulInternalIpEl {
    pub fn build(self) -> DataComputeRegionInstanceGroupManagerStatefulInternalIpEl {
        DataComputeRegionInstanceGroupManagerStatefulInternalIpEl {
            delete_rule: core::default::Default::default(),
            interface_name: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionInstanceGroupManagerStatefulInternalIpElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionInstanceGroupManagerStatefulInternalIpElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionInstanceGroupManagerStatefulInternalIpElRef {
        DataComputeRegionInstanceGroupManagerStatefulInternalIpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionInstanceGroupManagerStatefulInternalIpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `delete_rule` after provisioning.\n"]
    pub fn delete_rule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_rule", self.base))
    }
    #[doc = "Get a reference to the value of field `interface_name` after provisioning.\n"]
    pub fn interface_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.interface_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionInstanceGroupManagerStatusElAllInstancesConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    current_revision: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective: Option<PrimField<bool>>,
}
impl DataComputeRegionInstanceGroupManagerStatusElAllInstancesConfigEl {
    #[doc = "Set the field `current_revision`.\n"]
    pub fn set_current_revision(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.current_revision = Some(v.into());
        self
    }
    #[doc = "Set the field `effective`.\n"]
    pub fn set_effective(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.effective = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionInstanceGroupManagerStatusElAllInstancesConfigEl {
    type O = BlockAssignable<DataComputeRegionInstanceGroupManagerStatusElAllInstancesConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionInstanceGroupManagerStatusElAllInstancesConfigEl {}
impl BuildDataComputeRegionInstanceGroupManagerStatusElAllInstancesConfigEl {
    pub fn build(self) -> DataComputeRegionInstanceGroupManagerStatusElAllInstancesConfigEl {
        DataComputeRegionInstanceGroupManagerStatusElAllInstancesConfigEl {
            current_revision: core::default::Default::default(),
            effective: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionInstanceGroupManagerStatusElAllInstancesConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionInstanceGroupManagerStatusElAllInstancesConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionInstanceGroupManagerStatusElAllInstancesConfigElRef {
        DataComputeRegionInstanceGroupManagerStatusElAllInstancesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionInstanceGroupManagerStatusElAllInstancesConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `current_revision` after provisioning.\n"]
    pub fn current_revision(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.current_revision", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `effective` after provisioning.\n"]
    pub fn effective(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.effective", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all_effective: Option<PrimField<bool>>,
}
impl DataComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
    #[doc = "Set the field `all_effective`.\n"]
    pub fn set_all_effective(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.all_effective = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl
{
    type O = BlockAssignable<
        DataComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {}
impl BuildDataComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
    pub fn build(
        self,
    ) -> DataComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
        DataComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
            all_effective: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef {
        DataComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `all_effective` after provisioning.\n"]
    pub fn all_effective(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.all_effective", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionInstanceGroupManagerStatusElStatefulEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    has_stateful_config: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_instance_configs: Option<
        ListField<DataComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl>,
    >,
}
impl DataComputeRegionInstanceGroupManagerStatusElStatefulEl {
    #[doc = "Set the field `has_stateful_config`.\n"]
    pub fn set_has_stateful_config(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.has_stateful_config = Some(v.into());
        self
    }
    #[doc = "Set the field `per_instance_configs`.\n"]
    pub fn set_per_instance_configs(
        mut self,
        v: impl Into<
            ListField<DataComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl>,
        >,
    ) -> Self {
        self.per_instance_configs = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionInstanceGroupManagerStatusElStatefulEl {
    type O = BlockAssignable<DataComputeRegionInstanceGroupManagerStatusElStatefulEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionInstanceGroupManagerStatusElStatefulEl {}
impl BuildDataComputeRegionInstanceGroupManagerStatusElStatefulEl {
    pub fn build(self) -> DataComputeRegionInstanceGroupManagerStatusElStatefulEl {
        DataComputeRegionInstanceGroupManagerStatusElStatefulEl {
            has_stateful_config: core::default::Default::default(),
            per_instance_configs: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionInstanceGroupManagerStatusElStatefulElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionInstanceGroupManagerStatusElStatefulElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionInstanceGroupManagerStatusElStatefulElRef {
        DataComputeRegionInstanceGroupManagerStatusElStatefulElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionInstanceGroupManagerStatusElStatefulElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `has_stateful_config` after provisioning.\n"]
    pub fn has_stateful_config(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.has_stateful_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `per_instance_configs` after provisioning.\n"]
    pub fn per_instance_configs(
        &self,
    ) -> ListRef<DataComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.per_instance_configs", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionInstanceGroupManagerStatusElVersionTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    is_reached: Option<PrimField<bool>>,
}
impl DataComputeRegionInstanceGroupManagerStatusElVersionTargetEl {
    #[doc = "Set the field `is_reached`.\n"]
    pub fn set_is_reached(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_reached = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionInstanceGroupManagerStatusElVersionTargetEl {
    type O = BlockAssignable<DataComputeRegionInstanceGroupManagerStatusElVersionTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionInstanceGroupManagerStatusElVersionTargetEl {}
impl BuildDataComputeRegionInstanceGroupManagerStatusElVersionTargetEl {
    pub fn build(self) -> DataComputeRegionInstanceGroupManagerStatusElVersionTargetEl {
        DataComputeRegionInstanceGroupManagerStatusElVersionTargetEl {
            is_reached: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionInstanceGroupManagerStatusElVersionTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionInstanceGroupManagerStatusElVersionTargetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionInstanceGroupManagerStatusElVersionTargetElRef {
        DataComputeRegionInstanceGroupManagerStatusElVersionTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionInstanceGroupManagerStatusElVersionTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `is_reached` after provisioning.\n"]
    pub fn is_reached(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_reached", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionInstanceGroupManagerStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all_instances_config:
        Option<ListField<DataComputeRegionInstanceGroupManagerStatusElAllInstancesConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_stable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stateful: Option<ListField<DataComputeRegionInstanceGroupManagerStatusElStatefulEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_target: Option<ListField<DataComputeRegionInstanceGroupManagerStatusElVersionTargetEl>>,
}
impl DataComputeRegionInstanceGroupManagerStatusEl {
    #[doc = "Set the field `all_instances_config`.\n"]
    pub fn set_all_instances_config(
        mut self,
        v: impl Into<ListField<DataComputeRegionInstanceGroupManagerStatusElAllInstancesConfigEl>>,
    ) -> Self {
        self.all_instances_config = Some(v.into());
        self
    }
    #[doc = "Set the field `is_stable`.\n"]
    pub fn set_is_stable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_stable = Some(v.into());
        self
    }
    #[doc = "Set the field `stateful`.\n"]
    pub fn set_stateful(
        mut self,
        v: impl Into<ListField<DataComputeRegionInstanceGroupManagerStatusElStatefulEl>>,
    ) -> Self {
        self.stateful = Some(v.into());
        self
    }
    #[doc = "Set the field `version_target`.\n"]
    pub fn set_version_target(
        mut self,
        v: impl Into<ListField<DataComputeRegionInstanceGroupManagerStatusElVersionTargetEl>>,
    ) -> Self {
        self.version_target = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionInstanceGroupManagerStatusEl {
    type O = BlockAssignable<DataComputeRegionInstanceGroupManagerStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionInstanceGroupManagerStatusEl {}
impl BuildDataComputeRegionInstanceGroupManagerStatusEl {
    pub fn build(self) -> DataComputeRegionInstanceGroupManagerStatusEl {
        DataComputeRegionInstanceGroupManagerStatusEl {
            all_instances_config: core::default::Default::default(),
            is_stable: core::default::Default::default(),
            stateful: core::default::Default::default(),
            version_target: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionInstanceGroupManagerStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionInstanceGroupManagerStatusElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRegionInstanceGroupManagerStatusElRef {
        DataComputeRegionInstanceGroupManagerStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionInstanceGroupManagerStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `all_instances_config` after provisioning.\n"]
    pub fn all_instances_config(
        &self,
    ) -> ListRef<DataComputeRegionInstanceGroupManagerStatusElAllInstancesConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.all_instances_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `is_stable` after provisioning.\n"]
    pub fn is_stable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_stable", self.base))
    }
    #[doc = "Get a reference to the value of field `stateful` after provisioning.\n"]
    pub fn stateful(&self) -> ListRef<DataComputeRegionInstanceGroupManagerStatusElStatefulElRef> {
        ListRef::new(self.shared().clone(), format!("{}.stateful", self.base))
    }
    #[doc = "Get a reference to the value of field `version_target` after provisioning.\n"]
    pub fn version_target(
        &self,
    ) -> ListRef<DataComputeRegionInstanceGroupManagerStatusElVersionTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.version_target", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionInstanceGroupManagerTargetSizePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}
impl DataComputeRegionInstanceGroupManagerTargetSizePolicyEl {
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionInstanceGroupManagerTargetSizePolicyEl {
    type O = BlockAssignable<DataComputeRegionInstanceGroupManagerTargetSizePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionInstanceGroupManagerTargetSizePolicyEl {}
impl BuildDataComputeRegionInstanceGroupManagerTargetSizePolicyEl {
    pub fn build(self) -> DataComputeRegionInstanceGroupManagerTargetSizePolicyEl {
        DataComputeRegionInstanceGroupManagerTargetSizePolicyEl {
            mode: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionInstanceGroupManagerTargetSizePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionInstanceGroupManagerTargetSizePolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionInstanceGroupManagerTargetSizePolicyElRef {
        DataComputeRegionInstanceGroupManagerTargetSizePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionInstanceGroupManagerTargetSizePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionInstanceGroupManagerUpdatePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_redistribution_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_surge_fixed: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_surge_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_unavailable_fixed: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_unavailable_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimal_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    most_disruptive_allowed_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replacement_method: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataComputeRegionInstanceGroupManagerUpdatePolicyEl {
    #[doc = "Set the field `instance_redistribution_type`.\n"]
    pub fn set_instance_redistribution_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_redistribution_type = Some(v.into());
        self
    }
    #[doc = "Set the field `max_surge_fixed`.\n"]
    pub fn set_max_surge_fixed(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_surge_fixed = Some(v.into());
        self
    }
    #[doc = "Set the field `max_surge_percent`.\n"]
    pub fn set_max_surge_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_surge_percent = Some(v.into());
        self
    }
    #[doc = "Set the field `max_unavailable_fixed`.\n"]
    pub fn set_max_unavailable_fixed(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_unavailable_fixed = Some(v.into());
        self
    }
    #[doc = "Set the field `max_unavailable_percent`.\n"]
    pub fn set_max_unavailable_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_unavailable_percent = Some(v.into());
        self
    }
    #[doc = "Set the field `minimal_action`.\n"]
    pub fn set_minimal_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.minimal_action = Some(v.into());
        self
    }
    #[doc = "Set the field `most_disruptive_allowed_action`.\n"]
    pub fn set_most_disruptive_allowed_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.most_disruptive_allowed_action = Some(v.into());
        self
    }
    #[doc = "Set the field `replacement_method`.\n"]
    pub fn set_replacement_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.replacement_method = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionInstanceGroupManagerUpdatePolicyEl {
    type O = BlockAssignable<DataComputeRegionInstanceGroupManagerUpdatePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionInstanceGroupManagerUpdatePolicyEl {}
impl BuildDataComputeRegionInstanceGroupManagerUpdatePolicyEl {
    pub fn build(self) -> DataComputeRegionInstanceGroupManagerUpdatePolicyEl {
        DataComputeRegionInstanceGroupManagerUpdatePolicyEl {
            instance_redistribution_type: core::default::Default::default(),
            max_surge_fixed: core::default::Default::default(),
            max_surge_percent: core::default::Default::default(),
            max_unavailable_fixed: core::default::Default::default(),
            max_unavailable_percent: core::default::Default::default(),
            minimal_action: core::default::Default::default(),
            most_disruptive_allowed_action: core::default::Default::default(),
            replacement_method: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionInstanceGroupManagerUpdatePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionInstanceGroupManagerUpdatePolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionInstanceGroupManagerUpdatePolicyElRef {
        DataComputeRegionInstanceGroupManagerUpdatePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionInstanceGroupManagerUpdatePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_redistribution_type` after provisioning.\n"]
    pub fn instance_redistribution_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_redistribution_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_surge_fixed` after provisioning.\n"]
    pub fn max_surge_fixed(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_surge_fixed", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_surge_percent` after provisioning.\n"]
    pub fn max_surge_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_surge_percent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_unavailable_fixed` after provisioning.\n"]
    pub fn max_unavailable_fixed(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_unavailable_fixed", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_unavailable_percent` after provisioning.\n"]
    pub fn max_unavailable_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_unavailable_percent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `minimal_action` after provisioning.\n"]
    pub fn minimal_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minimal_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `most_disruptive_allowed_action` after provisioning.\n"]
    pub fn most_disruptive_allowed_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.most_disruptive_allowed_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `replacement_method` after provisioning.\n"]
    pub fn replacement_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replacement_method", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionInstanceGroupManagerVersionElTargetSizeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percent: Option<PrimField<f64>>,
}
impl DataComputeRegionInstanceGroupManagerVersionElTargetSizeEl {
    #[doc = "Set the field `fixed`.\n"]
    pub fn set_fixed(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.fixed = Some(v.into());
        self
    }
    #[doc = "Set the field `percent`.\n"]
    pub fn set_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percent = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionInstanceGroupManagerVersionElTargetSizeEl {
    type O = BlockAssignable<DataComputeRegionInstanceGroupManagerVersionElTargetSizeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionInstanceGroupManagerVersionElTargetSizeEl {}
impl BuildDataComputeRegionInstanceGroupManagerVersionElTargetSizeEl {
    pub fn build(self) -> DataComputeRegionInstanceGroupManagerVersionElTargetSizeEl {
        DataComputeRegionInstanceGroupManagerVersionElTargetSizeEl {
            fixed: core::default::Default::default(),
            percent: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionInstanceGroupManagerVersionElTargetSizeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionInstanceGroupManagerVersionElTargetSizeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionInstanceGroupManagerVersionElTargetSizeElRef {
        DataComputeRegionInstanceGroupManagerVersionElTargetSizeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionInstanceGroupManagerVersionElTargetSizeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `fixed` after provisioning.\n"]
    pub fn fixed(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.fixed", self.base))
    }
    #[doc = "Get a reference to the value of field `percent` after provisioning.\n"]
    pub fn percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionInstanceGroupManagerVersionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_template: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_size: Option<ListField<DataComputeRegionInstanceGroupManagerVersionElTargetSizeEl>>,
}
impl DataComputeRegionInstanceGroupManagerVersionEl {
    #[doc = "Set the field `instance_template`.\n"]
    pub fn set_instance_template(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_template = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `target_size`.\n"]
    pub fn set_target_size(
        mut self,
        v: impl Into<ListField<DataComputeRegionInstanceGroupManagerVersionElTargetSizeEl>>,
    ) -> Self {
        self.target_size = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionInstanceGroupManagerVersionEl {
    type O = BlockAssignable<DataComputeRegionInstanceGroupManagerVersionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionInstanceGroupManagerVersionEl {}
impl BuildDataComputeRegionInstanceGroupManagerVersionEl {
    pub fn build(self) -> DataComputeRegionInstanceGroupManagerVersionEl {
        DataComputeRegionInstanceGroupManagerVersionEl {
            instance_template: core::default::Default::default(),
            name: core::default::Default::default(),
            target_size: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionInstanceGroupManagerVersionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionInstanceGroupManagerVersionElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRegionInstanceGroupManagerVersionElRef {
        DataComputeRegionInstanceGroupManagerVersionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionInstanceGroupManagerVersionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_template` after provisioning.\n"]
    pub fn instance_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_template", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `target_size` after provisioning.\n"]
    pub fn target_size(
        &self,
    ) -> ListRef<DataComputeRegionInstanceGroupManagerVersionElTargetSizeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target_size", self.base))
    }
}
