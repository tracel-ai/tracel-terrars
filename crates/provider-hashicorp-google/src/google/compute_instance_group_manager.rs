use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeInstanceGroupManagerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    base_instance_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    list_managed_instances_results: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_pools: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_stopped_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_suspended_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_for_instances: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_for_instances_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    all_instances_config: Option<Vec<ComputeInstanceGroupManagerAllInstancesConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_healing_policies: Option<Vec<ComputeInstanceGroupManagerAutoHealingPoliciesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_lifecycle_policy: Option<Vec<ComputeInstanceGroupManagerInstanceLifecyclePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    named_port: Option<Vec<ComputeInstanceGroupManagerNamedPortEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_policies: Option<Vec<ComputeInstanceGroupManagerResourcePoliciesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    standby_policy: Option<Vec<ComputeInstanceGroupManagerStandbyPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stateful_disk: Option<Vec<ComputeInstanceGroupManagerStatefulDiskEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stateful_external_ip: Option<Vec<ComputeInstanceGroupManagerStatefulExternalIpEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stateful_internal_ip: Option<Vec<ComputeInstanceGroupManagerStatefulInternalIpEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_size_policy: Option<Vec<ComputeInstanceGroupManagerTargetSizePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeInstanceGroupManagerTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_policy: Option<Vec<ComputeInstanceGroupManagerUpdatePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<Vec<ComputeInstanceGroupManagerVersionEl>>,
    dynamic: ComputeInstanceGroupManagerDynamic,
}
struct ComputeInstanceGroupManager_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeInstanceGroupManagerData>,
}
#[derive(Clone)]
pub struct ComputeInstanceGroupManager(Rc<ComputeInstanceGroupManager_>);
impl ComputeInstanceGroupManager {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nAn optional textual description of the instance group manager."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `list_managed_instances_results`.\nPagination behavior of the listManagedInstances API method for this managed instance group. Valid values are: \"PAGELESS\", \"PAGINATED\". If PAGELESS (default), Pagination is disabled for the group's listManagedInstances API method. maxResults and pageToken query parameters are ignored and all instances are returned in a single response. If PAGINATED, pagination is enabled, maxResults and pageToken query parameters are respected."]
    pub fn set_list_managed_instances_results(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().list_managed_instances_results = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `target_pools`.\nThe full URL of all target pools to which new instances in the group are added. Updating the target pools attribute does not affect existing instances."]
    pub fn set_target_pools(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().target_pools = Some(v.into());
        self
    }
    #[doc = "Set the field `target_size`.\nThe target number of running instances for this managed instance group. This value should always be explicitly set unless this resource is attached to an autoscaler, in which case it should never be set. Defaults to 0."]
    pub fn set_target_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().target_size = Some(v.into());
        self
    }
    #[doc = "Set the field `target_stopped_size`.\nThe target number of stopped instances for this managed instance group."]
    pub fn set_target_stopped_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().target_stopped_size = Some(v.into());
        self
    }
    #[doc = "Set the field `target_suspended_size`.\nThe target number of suspended instances for this managed instance group."]
    pub fn set_target_suspended_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().target_suspended_size = Some(v.into());
        self
    }
    #[doc = "Set the field `wait_for_instances`.\nWhether to wait for all instances to be created/updated before returning. Note that if this is set to true and the operation does not succeed, Terraform will continue trying until it times out."]
    pub fn set_wait_for_instances(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_for_instances = Some(v.into());
        self
    }
    #[doc = "Set the field `wait_for_instances_status`.\nWhen used with wait_for_instances specifies the status to wait for. When STABLE is specified this resource will wait until the instances are stable before returning. When UPDATED is set, it will wait for the version target to be reached and any per instance configs to be effective and all instances configs to be effective as well as all instances to be stable before returning."]
    pub fn set_wait_for_instances_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().wait_for_instances_status = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\nThe zone that instances in this group should be created in."]
    pub fn set_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone = Some(v.into());
        self
    }
    #[doc = "Set the field `all_instances_config`.\n"]
    pub fn set_all_instances_config(
        self,
        v: impl Into<BlockAssignable<ComputeInstanceGroupManagerAllInstancesConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().all_instances_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.all_instances_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `auto_healing_policies`.\n"]
    pub fn set_auto_healing_policies(
        self,
        v: impl Into<BlockAssignable<ComputeInstanceGroupManagerAutoHealingPoliciesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_healing_policies = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_healing_policies = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `instance_lifecycle_policy`.\n"]
    pub fn set_instance_lifecycle_policy(
        self,
        v: impl Into<BlockAssignable<ComputeInstanceGroupManagerInstanceLifecyclePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_lifecycle_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_lifecycle_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `named_port`.\n"]
    pub fn set_named_port(
        self,
        v: impl Into<BlockAssignable<ComputeInstanceGroupManagerNamedPortEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().named_port = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.named_port = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_policies`.\n"]
    pub fn set_resource_policies(
        self,
        v: impl Into<BlockAssignable<ComputeInstanceGroupManagerResourcePoliciesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_policies = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resource_policies = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `standby_policy`.\n"]
    pub fn set_standby_policy(
        self,
        v: impl Into<BlockAssignable<ComputeInstanceGroupManagerStandbyPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().standby_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.standby_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `stateful_disk`.\n"]
    pub fn set_stateful_disk(
        self,
        v: impl Into<BlockAssignable<ComputeInstanceGroupManagerStatefulDiskEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stateful_disk = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stateful_disk = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `stateful_external_ip`.\n"]
    pub fn set_stateful_external_ip(
        self,
        v: impl Into<BlockAssignable<ComputeInstanceGroupManagerStatefulExternalIpEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stateful_external_ip = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stateful_external_ip = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `stateful_internal_ip`.\n"]
    pub fn set_stateful_internal_ip(
        self,
        v: impl Into<BlockAssignable<ComputeInstanceGroupManagerStatefulInternalIpEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stateful_internal_ip = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stateful_internal_ip = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `target_size_policy`.\n"]
    pub fn set_target_size_policy(
        self,
        v: impl Into<BlockAssignable<ComputeInstanceGroupManagerTargetSizePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_size_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_size_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeInstanceGroupManagerTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `update_policy`.\n"]
    pub fn set_update_policy(
        self,
        v: impl Into<BlockAssignable<ComputeInstanceGroupManagerUpdatePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().update_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.update_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(
        self,
        v: impl Into<BlockAssignable<ComputeInstanceGroupManagerVersionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().version = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.version = Some(d);
            }
        }
        self
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
    #[doc = "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.operation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URL of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of this managed instance group."]
    pub fn status(&self) -> ListRef<ComputeInstanceGroupManagerStatusElRef> {
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
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe zone that instances in this group should be created in."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `all_instances_config` after provisioning.\n"]
    pub fn all_instances_config(
        &self,
    ) -> ListRef<ComputeInstanceGroupManagerAllInstancesConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.all_instances_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auto_healing_policies` after provisioning.\n"]
    pub fn auto_healing_policies(
        &self,
    ) -> ListRef<ComputeInstanceGroupManagerAutoHealingPoliciesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_healing_policies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_lifecycle_policy` after provisioning.\n"]
    pub fn instance_lifecycle_policy(
        &self,
    ) -> ListRef<ComputeInstanceGroupManagerInstanceLifecyclePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_lifecycle_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_policies` after provisioning.\n"]
    pub fn resource_policies(&self) -> ListRef<ComputeInstanceGroupManagerResourcePoliciesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_policies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `standby_policy` after provisioning.\n"]
    pub fn standby_policy(&self) -> ListRef<ComputeInstanceGroupManagerStandbyPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.standby_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stateful_external_ip` after provisioning.\n"]
    pub fn stateful_external_ip(
        &self,
    ) -> ListRef<ComputeInstanceGroupManagerStatefulExternalIpElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stateful_external_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stateful_internal_ip` after provisioning.\n"]
    pub fn stateful_internal_ip(
        &self,
    ) -> ListRef<ComputeInstanceGroupManagerStatefulInternalIpElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stateful_internal_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_size_policy` after provisioning.\n"]
    pub fn target_size_policy(&self) -> ListRef<ComputeInstanceGroupManagerTargetSizePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_size_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeInstanceGroupManagerTimeoutsElRef {
        ComputeInstanceGroupManagerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_policy` after provisioning.\n"]
    pub fn update_policy(&self) -> ListRef<ComputeInstanceGroupManagerUpdatePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.update_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> ListRef<ComputeInstanceGroupManagerVersionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
}
impl Referable for ComputeInstanceGroupManager {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeInstanceGroupManager {}
impl ToListMappable for ComputeInstanceGroupManager {
    type O = ListRef<ComputeInstanceGroupManagerRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeInstanceGroupManager_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_instance_group_manager".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeInstanceGroupManager {
    pub tf_id: String,
    #[doc = "The base instance name to use for instances in this group. The value must be a valid RFC1035 name. Supported characters are lowercase letters, numbers, and hyphens (-). Instances are named by appending a hyphen and a random four-character string to the base instance name."]
    pub base_instance_name: PrimField<String>,
    #[doc = "The name of the instance group manager. Must be 1-63 characters long and comply with RFC1035. Supported characters include lowercase letters, numbers, and hyphens."]
    pub name: PrimField<String>,
}
impl BuildComputeInstanceGroupManager {
    pub fn build(self, stack: &mut Stack) -> ComputeInstanceGroupManager {
        let out = ComputeInstanceGroupManager(Rc::new(ComputeInstanceGroupManager_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeInstanceGroupManagerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                base_instance_name: self.base_instance_name,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                list_managed_instances_results: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                target_pools: core::default::Default::default(),
                target_size: core::default::Default::default(),
                target_stopped_size: core::default::Default::default(),
                target_suspended_size: core::default::Default::default(),
                wait_for_instances: core::default::Default::default(),
                wait_for_instances_status: core::default::Default::default(),
                zone: core::default::Default::default(),
                all_instances_config: core::default::Default::default(),
                auto_healing_policies: core::default::Default::default(),
                instance_lifecycle_policy: core::default::Default::default(),
                named_port: core::default::Default::default(),
                resource_policies: core::default::Default::default(),
                standby_policy: core::default::Default::default(),
                stateful_disk: core::default::Default::default(),
                stateful_external_ip: core::default::Default::default(),
                stateful_internal_ip: core::default::Default::default(),
                target_size_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                update_policy: core::default::Default::default(),
                version: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeInstanceGroupManagerRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceGroupManagerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeInstanceGroupManagerRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.operation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URL of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of this managed instance group."]
    pub fn status(&self) -> ListRef<ComputeInstanceGroupManagerStatusElRef> {
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
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe zone that instances in this group should be created in."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `all_instances_config` after provisioning.\n"]
    pub fn all_instances_config(
        &self,
    ) -> ListRef<ComputeInstanceGroupManagerAllInstancesConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.all_instances_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auto_healing_policies` after provisioning.\n"]
    pub fn auto_healing_policies(
        &self,
    ) -> ListRef<ComputeInstanceGroupManagerAutoHealingPoliciesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_healing_policies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_lifecycle_policy` after provisioning.\n"]
    pub fn instance_lifecycle_policy(
        &self,
    ) -> ListRef<ComputeInstanceGroupManagerInstanceLifecyclePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_lifecycle_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_policies` after provisioning.\n"]
    pub fn resource_policies(&self) -> ListRef<ComputeInstanceGroupManagerResourcePoliciesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_policies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `standby_policy` after provisioning.\n"]
    pub fn standby_policy(&self) -> ListRef<ComputeInstanceGroupManagerStandbyPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.standby_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stateful_external_ip` after provisioning.\n"]
    pub fn stateful_external_ip(
        &self,
    ) -> ListRef<ComputeInstanceGroupManagerStatefulExternalIpElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stateful_external_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stateful_internal_ip` after provisioning.\n"]
    pub fn stateful_internal_ip(
        &self,
    ) -> ListRef<ComputeInstanceGroupManagerStatefulInternalIpElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stateful_internal_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_size_policy` after provisioning.\n"]
    pub fn target_size_policy(&self) -> ListRef<ComputeInstanceGroupManagerTargetSizePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_size_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeInstanceGroupManagerTimeoutsElRef {
        ComputeInstanceGroupManagerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_policy` after provisioning.\n"]
    pub fn update_policy(&self) -> ListRef<ComputeInstanceGroupManagerUpdatePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.update_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> ListRef<ComputeInstanceGroupManagerVersionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceGroupManagerStatusElAllInstancesConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    current_revision: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective: Option<PrimField<bool>>,
}
impl ComputeInstanceGroupManagerStatusElAllInstancesConfigEl {
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
impl ToListMappable for ComputeInstanceGroupManagerStatusElAllInstancesConfigEl {
    type O = BlockAssignable<ComputeInstanceGroupManagerStatusElAllInstancesConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceGroupManagerStatusElAllInstancesConfigEl {}
impl BuildComputeInstanceGroupManagerStatusElAllInstancesConfigEl {
    pub fn build(self) -> ComputeInstanceGroupManagerStatusElAllInstancesConfigEl {
        ComputeInstanceGroupManagerStatusElAllInstancesConfigEl {
            current_revision: core::default::Default::default(),
            effective: core::default::Default::default(),
        }
    }
}
pub struct ComputeInstanceGroupManagerStatusElAllInstancesConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceGroupManagerStatusElAllInstancesConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInstanceGroupManagerStatusElAllInstancesConfigElRef {
        ComputeInstanceGroupManagerStatusElAllInstancesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceGroupManagerStatusElAllInstancesConfigElRef {
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
pub struct ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all_effective: Option<PrimField<bool>>,
}
impl ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
    #[doc = "Set the field `all_effective`.\n"]
    pub fn set_all_effective(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.all_effective = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
    type O = BlockAssignable<ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {}
impl BuildComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
    pub fn build(self) -> ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
        ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
            all_effective: core::default::Default::default(),
        }
    }
}
pub struct ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef {
        ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef {
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
pub struct ComputeInstanceGroupManagerStatusElStatefulEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    has_stateful_config: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_instance_configs:
        Option<ListField<ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl>>,
}
impl ComputeInstanceGroupManagerStatusElStatefulEl {
    #[doc = "Set the field `has_stateful_config`.\n"]
    pub fn set_has_stateful_config(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.has_stateful_config = Some(v.into());
        self
    }
    #[doc = "Set the field `per_instance_configs`.\n"]
    pub fn set_per_instance_configs(
        mut self,
        v: impl Into<ListField<ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl>>,
    ) -> Self {
        self.per_instance_configs = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInstanceGroupManagerStatusElStatefulEl {
    type O = BlockAssignable<ComputeInstanceGroupManagerStatusElStatefulEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceGroupManagerStatusElStatefulEl {}
impl BuildComputeInstanceGroupManagerStatusElStatefulEl {
    pub fn build(self) -> ComputeInstanceGroupManagerStatusElStatefulEl {
        ComputeInstanceGroupManagerStatusElStatefulEl {
            has_stateful_config: core::default::Default::default(),
            per_instance_configs: core::default::Default::default(),
        }
    }
}
pub struct ComputeInstanceGroupManagerStatusElStatefulElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceGroupManagerStatusElStatefulElRef {
    fn new(shared: StackShared, base: String) -> ComputeInstanceGroupManagerStatusElStatefulElRef {
        ComputeInstanceGroupManagerStatusElStatefulElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceGroupManagerStatusElStatefulElRef {
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
    ) -> ListRef<ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.per_instance_configs", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceGroupManagerStatusElVersionTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    is_reached: Option<PrimField<bool>>,
}
impl ComputeInstanceGroupManagerStatusElVersionTargetEl {
    #[doc = "Set the field `is_reached`.\n"]
    pub fn set_is_reached(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_reached = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInstanceGroupManagerStatusElVersionTargetEl {
    type O = BlockAssignable<ComputeInstanceGroupManagerStatusElVersionTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceGroupManagerStatusElVersionTargetEl {}
impl BuildComputeInstanceGroupManagerStatusElVersionTargetEl {
    pub fn build(self) -> ComputeInstanceGroupManagerStatusElVersionTargetEl {
        ComputeInstanceGroupManagerStatusElVersionTargetEl {
            is_reached: core::default::Default::default(),
        }
    }
}
pub struct ComputeInstanceGroupManagerStatusElVersionTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceGroupManagerStatusElVersionTargetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInstanceGroupManagerStatusElVersionTargetElRef {
        ComputeInstanceGroupManagerStatusElVersionTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceGroupManagerStatusElVersionTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `is_reached` after provisioning.\n"]
    pub fn is_reached(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_reached", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceGroupManagerStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all_instances_config:
        Option<ListField<ComputeInstanceGroupManagerStatusElAllInstancesConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_stable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stateful: Option<ListField<ComputeInstanceGroupManagerStatusElStatefulEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_target: Option<ListField<ComputeInstanceGroupManagerStatusElVersionTargetEl>>,
}
impl ComputeInstanceGroupManagerStatusEl {
    #[doc = "Set the field `all_instances_config`.\n"]
    pub fn set_all_instances_config(
        mut self,
        v: impl Into<ListField<ComputeInstanceGroupManagerStatusElAllInstancesConfigEl>>,
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
        v: impl Into<ListField<ComputeInstanceGroupManagerStatusElStatefulEl>>,
    ) -> Self {
        self.stateful = Some(v.into());
        self
    }
    #[doc = "Set the field `version_target`.\n"]
    pub fn set_version_target(
        mut self,
        v: impl Into<ListField<ComputeInstanceGroupManagerStatusElVersionTargetEl>>,
    ) -> Self {
        self.version_target = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInstanceGroupManagerStatusEl {
    type O = BlockAssignable<ComputeInstanceGroupManagerStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceGroupManagerStatusEl {}
impl BuildComputeInstanceGroupManagerStatusEl {
    pub fn build(self) -> ComputeInstanceGroupManagerStatusEl {
        ComputeInstanceGroupManagerStatusEl {
            all_instances_config: core::default::Default::default(),
            is_stable: core::default::Default::default(),
            stateful: core::default::Default::default(),
            version_target: core::default::Default::default(),
        }
    }
}
pub struct ComputeInstanceGroupManagerStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceGroupManagerStatusElRef {
    fn new(shared: StackShared, base: String) -> ComputeInstanceGroupManagerStatusElRef {
        ComputeInstanceGroupManagerStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceGroupManagerStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `all_instances_config` after provisioning.\n"]
    pub fn all_instances_config(
        &self,
    ) -> ListRef<ComputeInstanceGroupManagerStatusElAllInstancesConfigElRef> {
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
    pub fn stateful(&self) -> ListRef<ComputeInstanceGroupManagerStatusElStatefulElRef> {
        ListRef::new(self.shared().clone(), format!("{}.stateful", self.base))
    }
    #[doc = "Get a reference to the value of field `version_target` after provisioning.\n"]
    pub fn version_target(&self) -> ListRef<ComputeInstanceGroupManagerStatusElVersionTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.version_target", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceGroupManagerAllInstancesConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RecField<PrimField<String>>>,
}
impl ComputeInstanceGroupManagerAllInstancesConfigEl {
    #[doc = "Set the field `labels`.\nThe label key-value pairs that you want to patch onto the instance,"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata`.\nThe metadata key-value pairs that you want to patch onto the instance. For more information, see Project and instance metadata,"]
    pub fn set_metadata(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.metadata = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInstanceGroupManagerAllInstancesConfigEl {
    type O = BlockAssignable<ComputeInstanceGroupManagerAllInstancesConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceGroupManagerAllInstancesConfigEl {}
impl BuildComputeInstanceGroupManagerAllInstancesConfigEl {
    pub fn build(self) -> ComputeInstanceGroupManagerAllInstancesConfigEl {
        ComputeInstanceGroupManagerAllInstancesConfigEl {
            labels: core::default::Default::default(),
            metadata: core::default::Default::default(),
        }
    }
}
pub struct ComputeInstanceGroupManagerAllInstancesConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceGroupManagerAllInstancesConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInstanceGroupManagerAllInstancesConfigElRef {
        ComputeInstanceGroupManagerAllInstancesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceGroupManagerAllInstancesConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe label key-value pairs that you want to patch onto the instance,"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nThe metadata key-value pairs that you want to patch onto the instance. For more information, see Project and instance metadata,"]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceGroupManagerAutoHealingPoliciesEl {
    health_check: PrimField<String>,
    initial_delay_sec: PrimField<f64>,
}
impl ComputeInstanceGroupManagerAutoHealingPoliciesEl {}
impl ToListMappable for ComputeInstanceGroupManagerAutoHealingPoliciesEl {
    type O = BlockAssignable<ComputeInstanceGroupManagerAutoHealingPoliciesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceGroupManagerAutoHealingPoliciesEl {
    #[doc = "The health check resource that signals autohealing."]
    pub health_check: PrimField<String>,
    #[doc = "The number of seconds that the managed instance group waits before it applies autohealing policies to new instances or recently recreated instances. Between 0 and 3600."]
    pub initial_delay_sec: PrimField<f64>,
}
impl BuildComputeInstanceGroupManagerAutoHealingPoliciesEl {
    pub fn build(self) -> ComputeInstanceGroupManagerAutoHealingPoliciesEl {
        ComputeInstanceGroupManagerAutoHealingPoliciesEl {
            health_check: self.health_check,
            initial_delay_sec: self.initial_delay_sec,
        }
    }
}
pub struct ComputeInstanceGroupManagerAutoHealingPoliciesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceGroupManagerAutoHealingPoliciesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInstanceGroupManagerAutoHealingPoliciesElRef {
        ComputeInstanceGroupManagerAutoHealingPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceGroupManagerAutoHealingPoliciesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `health_check` after provisioning.\nThe health check resource that signals autohealing."]
    pub fn health_check(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.health_check", self.base))
    }
    #[doc = "Get a reference to the value of field `initial_delay_sec` after provisioning.\nThe number of seconds that the managed instance group waits before it applies autohealing policies to new instances or recently recreated instances. Between 0 and 3600."]
    pub fn initial_delay_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_delay_sec", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceGroupManagerInstanceLifecyclePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_action_on_failure: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_update_on_repair: Option<PrimField<String>>,
}
impl ComputeInstanceGroupManagerInstanceLifecyclePolicyEl {
    #[doc = "Set the field `default_action_on_failure`.\nDefault behavior for all instance or health check failures."]
    pub fn set_default_action_on_failure(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_action_on_failure = Some(v.into());
        self
    }
    #[doc = "Set the field `force_update_on_repair`.\nSpecifies whether to apply the group's latest configuration when repairing a VM. Valid options are: YES, NO. If YES and you updated the group's instance template or per-instance configurations after the VM was created, then these changes are applied when VM is repaired. If NO (default), then updates are applied in accordance with the group's update policy type."]
    pub fn set_force_update_on_repair(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.force_update_on_repair = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInstanceGroupManagerInstanceLifecyclePolicyEl {
    type O = BlockAssignable<ComputeInstanceGroupManagerInstanceLifecyclePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceGroupManagerInstanceLifecyclePolicyEl {}
impl BuildComputeInstanceGroupManagerInstanceLifecyclePolicyEl {
    pub fn build(self) -> ComputeInstanceGroupManagerInstanceLifecyclePolicyEl {
        ComputeInstanceGroupManagerInstanceLifecyclePolicyEl {
            default_action_on_failure: core::default::Default::default(),
            force_update_on_repair: core::default::Default::default(),
        }
    }
}
pub struct ComputeInstanceGroupManagerInstanceLifecyclePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceGroupManagerInstanceLifecyclePolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInstanceGroupManagerInstanceLifecyclePolicyElRef {
        ComputeInstanceGroupManagerInstanceLifecyclePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceGroupManagerInstanceLifecyclePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_action_on_failure` after provisioning.\nDefault behavior for all instance or health check failures."]
    pub fn default_action_on_failure(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_action_on_failure", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `force_update_on_repair` after provisioning.\nSpecifies whether to apply the group's latest configuration when repairing a VM. Valid options are: YES, NO. If YES and you updated the group's instance template or per-instance configurations after the VM was created, then these changes are applied when VM is repaired. If NO (default), then updates are applied in accordance with the group's update policy type."]
    pub fn force_update_on_repair(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_update_on_repair", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceGroupManagerNamedPortEl {
    name: PrimField<String>,
    port: PrimField<f64>,
}
impl ComputeInstanceGroupManagerNamedPortEl {}
impl ToListMappable for ComputeInstanceGroupManagerNamedPortEl {
    type O = BlockAssignable<ComputeInstanceGroupManagerNamedPortEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceGroupManagerNamedPortEl {
    #[doc = "The name of the port."]
    pub name: PrimField<String>,
    #[doc = "The port number."]
    pub port: PrimField<f64>,
}
impl BuildComputeInstanceGroupManagerNamedPortEl {
    pub fn build(self) -> ComputeInstanceGroupManagerNamedPortEl {
        ComputeInstanceGroupManagerNamedPortEl {
            name: self.name,
            port: self.port,
        }
    }
}
pub struct ComputeInstanceGroupManagerNamedPortElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceGroupManagerNamedPortElRef {
    fn new(shared: StackShared, base: String) -> ComputeInstanceGroupManagerNamedPortElRef {
        ComputeInstanceGroupManagerNamedPortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceGroupManagerNamedPortElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the port."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe port number."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceGroupManagerResourcePoliciesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    workload_policy: Option<PrimField<String>>,
}
impl ComputeInstanceGroupManagerResourcePoliciesEl {
    #[doc = "Set the field `workload_policy`.\nThe URL of the workload policy that is specified for this managed instance group. It can be a full or partial URL."]
    pub fn set_workload_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.workload_policy = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInstanceGroupManagerResourcePoliciesEl {
    type O = BlockAssignable<ComputeInstanceGroupManagerResourcePoliciesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceGroupManagerResourcePoliciesEl {}
impl BuildComputeInstanceGroupManagerResourcePoliciesEl {
    pub fn build(self) -> ComputeInstanceGroupManagerResourcePoliciesEl {
        ComputeInstanceGroupManagerResourcePoliciesEl {
            workload_policy: core::default::Default::default(),
        }
    }
}
pub struct ComputeInstanceGroupManagerResourcePoliciesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceGroupManagerResourcePoliciesElRef {
    fn new(shared: StackShared, base: String) -> ComputeInstanceGroupManagerResourcePoliciesElRef {
        ComputeInstanceGroupManagerResourcePoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceGroupManagerResourcePoliciesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `workload_policy` after provisioning.\nThe URL of the workload policy that is specified for this managed instance group. It can be a full or partial URL."]
    pub fn workload_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workload_policy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceGroupManagerStandbyPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_delay_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}
impl ComputeInstanceGroupManagerStandbyPolicyEl {
    #[doc = "Set the field `initial_delay_sec`.\nSpecifies the number of seconds that the MIG should wait to suspend or stop a VM after that VM was created. The initial delay gives the initialization script the time to prepare your VM for a quick scale out. The value of initial delay must be between 0 and 3600 seconds. The default value is 0."]
    pub fn set_initial_delay_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.initial_delay_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `mode`.\nDefines how a MIG resumes or starts VMs from a standby pool when the group scales out. The default mode is \"MANUAL\"."]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInstanceGroupManagerStandbyPolicyEl {
    type O = BlockAssignable<ComputeInstanceGroupManagerStandbyPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceGroupManagerStandbyPolicyEl {}
impl BuildComputeInstanceGroupManagerStandbyPolicyEl {
    pub fn build(self) -> ComputeInstanceGroupManagerStandbyPolicyEl {
        ComputeInstanceGroupManagerStandbyPolicyEl {
            initial_delay_sec: core::default::Default::default(),
            mode: core::default::Default::default(),
        }
    }
}
pub struct ComputeInstanceGroupManagerStandbyPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceGroupManagerStandbyPolicyElRef {
    fn new(shared: StackShared, base: String) -> ComputeInstanceGroupManagerStandbyPolicyElRef {
        ComputeInstanceGroupManagerStandbyPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceGroupManagerStandbyPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `initial_delay_sec` after provisioning.\nSpecifies the number of seconds that the MIG should wait to suspend or stop a VM after that VM was created. The initial delay gives the initialization script the time to prepare your VM for a quick scale out. The value of initial delay must be between 0 and 3600 seconds. The default value is 0."]
    pub fn initial_delay_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_delay_sec", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nDefines how a MIG resumes or starts VMs from a standby pool when the group scales out. The default mode is \"MANUAL\"."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceGroupManagerStatefulDiskEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_rule: Option<PrimField<String>>,
    device_name: PrimField<String>,
}
impl ComputeInstanceGroupManagerStatefulDiskEl {
    #[doc = "Set the field `delete_rule`.\nA value that prescribes what should happen to the stateful disk when the VM instance is deleted. The available options are NEVER and ON_PERMANENT_INSTANCE_DELETION. NEVER - detach the disk when the VM is deleted, but do not delete the disk. ON_PERMANENT_INSTANCE_DELETION will delete the stateful disk when the VM is permanently deleted from the instance group. The default is NEVER."]
    pub fn set_delete_rule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete_rule = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInstanceGroupManagerStatefulDiskEl {
    type O = BlockAssignable<ComputeInstanceGroupManagerStatefulDiskEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceGroupManagerStatefulDiskEl {
    #[doc = "The device name of the disk to be attached."]
    pub device_name: PrimField<String>,
}
impl BuildComputeInstanceGroupManagerStatefulDiskEl {
    pub fn build(self) -> ComputeInstanceGroupManagerStatefulDiskEl {
        ComputeInstanceGroupManagerStatefulDiskEl {
            delete_rule: core::default::Default::default(),
            device_name: self.device_name,
        }
    }
}
pub struct ComputeInstanceGroupManagerStatefulDiskElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceGroupManagerStatefulDiskElRef {
    fn new(shared: StackShared, base: String) -> ComputeInstanceGroupManagerStatefulDiskElRef {
        ComputeInstanceGroupManagerStatefulDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceGroupManagerStatefulDiskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `delete_rule` after provisioning.\nA value that prescribes what should happen to the stateful disk when the VM instance is deleted. The available options are NEVER and ON_PERMANENT_INSTANCE_DELETION. NEVER - detach the disk when the VM is deleted, but do not delete the disk. ON_PERMANENT_INSTANCE_DELETION will delete the stateful disk when the VM is permanently deleted from the instance group. The default is NEVER."]
    pub fn delete_rule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_rule", self.base))
    }
    #[doc = "Get a reference to the value of field `device_name` after provisioning.\nThe device name of the disk to be attached."]
    pub fn device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceGroupManagerStatefulExternalIpEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_rule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interface_name: Option<PrimField<String>>,
}
impl ComputeInstanceGroupManagerStatefulExternalIpEl {
    #[doc = "Set the field `delete_rule`.\nA value that prescribes what should happen to an associated static Address resource when a VM instance is permanently deleted. The available options are NEVER and ON_PERMANENT_INSTANCE_DELETION. NEVER - detach the IP when the VM is deleted, but do not delete the address resource. ON_PERMANENT_INSTANCE_DELETION will delete the stateful address when the VM is permanently deleted from the instance group. The default is NEVER."]
    pub fn set_delete_rule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete_rule = Some(v.into());
        self
    }
    #[doc = "Set the field `interface_name`.\nThe network interface name"]
    pub fn set_interface_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interface_name = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInstanceGroupManagerStatefulExternalIpEl {
    type O = BlockAssignable<ComputeInstanceGroupManagerStatefulExternalIpEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceGroupManagerStatefulExternalIpEl {}
impl BuildComputeInstanceGroupManagerStatefulExternalIpEl {
    pub fn build(self) -> ComputeInstanceGroupManagerStatefulExternalIpEl {
        ComputeInstanceGroupManagerStatefulExternalIpEl {
            delete_rule: core::default::Default::default(),
            interface_name: core::default::Default::default(),
        }
    }
}
pub struct ComputeInstanceGroupManagerStatefulExternalIpElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceGroupManagerStatefulExternalIpElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInstanceGroupManagerStatefulExternalIpElRef {
        ComputeInstanceGroupManagerStatefulExternalIpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceGroupManagerStatefulExternalIpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `delete_rule` after provisioning.\nA value that prescribes what should happen to an associated static Address resource when a VM instance is permanently deleted. The available options are NEVER and ON_PERMANENT_INSTANCE_DELETION. NEVER - detach the IP when the VM is deleted, but do not delete the address resource. ON_PERMANENT_INSTANCE_DELETION will delete the stateful address when the VM is permanently deleted from the instance group. The default is NEVER."]
    pub fn delete_rule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_rule", self.base))
    }
    #[doc = "Get a reference to the value of field `interface_name` after provisioning.\nThe network interface name"]
    pub fn interface_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.interface_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceGroupManagerStatefulInternalIpEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_rule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interface_name: Option<PrimField<String>>,
}
impl ComputeInstanceGroupManagerStatefulInternalIpEl {
    #[doc = "Set the field `delete_rule`.\nA value that prescribes what should happen to an associated static Address resource when a VM instance is permanently deleted. The available options are NEVER and ON_PERMANENT_INSTANCE_DELETION. NEVER - detach the IP when the VM is deleted, but do not delete the address resource. ON_PERMANENT_INSTANCE_DELETION will delete the stateful address when the VM is permanently deleted from the instance group. The default is NEVER."]
    pub fn set_delete_rule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete_rule = Some(v.into());
        self
    }
    #[doc = "Set the field `interface_name`.\nThe network interface name"]
    pub fn set_interface_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interface_name = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInstanceGroupManagerStatefulInternalIpEl {
    type O = BlockAssignable<ComputeInstanceGroupManagerStatefulInternalIpEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceGroupManagerStatefulInternalIpEl {}
impl BuildComputeInstanceGroupManagerStatefulInternalIpEl {
    pub fn build(self) -> ComputeInstanceGroupManagerStatefulInternalIpEl {
        ComputeInstanceGroupManagerStatefulInternalIpEl {
            delete_rule: core::default::Default::default(),
            interface_name: core::default::Default::default(),
        }
    }
}
pub struct ComputeInstanceGroupManagerStatefulInternalIpElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceGroupManagerStatefulInternalIpElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInstanceGroupManagerStatefulInternalIpElRef {
        ComputeInstanceGroupManagerStatefulInternalIpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceGroupManagerStatefulInternalIpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `delete_rule` after provisioning.\nA value that prescribes what should happen to an associated static Address resource when a VM instance is permanently deleted. The available options are NEVER and ON_PERMANENT_INSTANCE_DELETION. NEVER - detach the IP when the VM is deleted, but do not delete the address resource. ON_PERMANENT_INSTANCE_DELETION will delete the stateful address when the VM is permanently deleted from the instance group. The default is NEVER."]
    pub fn delete_rule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_rule", self.base))
    }
    #[doc = "Get a reference to the value of field `interface_name` after provisioning.\nThe network interface name"]
    pub fn interface_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.interface_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceGroupManagerTargetSizePolicyEl {
    mode: PrimField<String>,
}
impl ComputeInstanceGroupManagerTargetSizePolicyEl {}
impl ToListMappable for ComputeInstanceGroupManagerTargetSizePolicyEl {
    type O = BlockAssignable<ComputeInstanceGroupManagerTargetSizePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceGroupManagerTargetSizePolicyEl {
    #[doc = "The mode of target size policy based on which the MIG creates its VMs individually or all at once."]
    pub mode: PrimField<String>,
}
impl BuildComputeInstanceGroupManagerTargetSizePolicyEl {
    pub fn build(self) -> ComputeInstanceGroupManagerTargetSizePolicyEl {
        ComputeInstanceGroupManagerTargetSizePolicyEl { mode: self.mode }
    }
}
pub struct ComputeInstanceGroupManagerTargetSizePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceGroupManagerTargetSizePolicyElRef {
    fn new(shared: StackShared, base: String) -> ComputeInstanceGroupManagerTargetSizePolicyElRef {
        ComputeInstanceGroupManagerTargetSizePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceGroupManagerTargetSizePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nThe mode of target size policy based on which the MIG creates its VMs individually or all at once."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceGroupManagerTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeInstanceGroupManagerTimeoutsEl {
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
impl ToListMappable for ComputeInstanceGroupManagerTimeoutsEl {
    type O = BlockAssignable<ComputeInstanceGroupManagerTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceGroupManagerTimeoutsEl {}
impl BuildComputeInstanceGroupManagerTimeoutsEl {
    pub fn build(self) -> ComputeInstanceGroupManagerTimeoutsEl {
        ComputeInstanceGroupManagerTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeInstanceGroupManagerTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceGroupManagerTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeInstanceGroupManagerTimeoutsElRef {
        ComputeInstanceGroupManagerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceGroupManagerTimeoutsElRef {
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
#[derive(Serialize)]
pub struct ComputeInstanceGroupManagerUpdatePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_surge_fixed: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_surge_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_unavailable_fixed: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_unavailable_percent: Option<PrimField<f64>>,
    minimal_action: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    most_disruptive_allowed_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replacement_method: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl ComputeInstanceGroupManagerUpdatePolicyEl {
    #[doc = "Set the field `max_surge_fixed`.\nSpecifies a fixed number of VM instances. This must be a positive integer. Conflicts with max_surge_percent. Both cannot be 0"]
    pub fn set_max_surge_fixed(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_surge_fixed = Some(v.into());
        self
    }
    #[doc = "Set the field `max_surge_percent`.\nSpecifies a percentage of instances between 0 to 100%, inclusive. For example, specify 80 for 80%. Conflicts with max_surge_fixed."]
    pub fn set_max_surge_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_surge_percent = Some(v.into());
        self
    }
    #[doc = "Set the field `max_unavailable_fixed`.\nSpecifies a fixed number of VM instances. This must be a positive integer."]
    pub fn set_max_unavailable_fixed(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_unavailable_fixed = Some(v.into());
        self
    }
    #[doc = "Set the field `max_unavailable_percent`.\nSpecifies a percentage of instances between 0 to 100%, inclusive. For example, specify 80 for 80%."]
    pub fn set_max_unavailable_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_unavailable_percent = Some(v.into());
        self
    }
    #[doc = "Set the field `most_disruptive_allowed_action`.\nMost disruptive action that is allowed to be taken on an instance. You can specify either NONE to forbid any actions, REFRESH to allow actions that do not need instance restart, RESTART to allow actions that can be applied without instance replacing or REPLACE to allow all possible actions. If the Updater determines that the minimal update action needed is more disruptive than most disruptive allowed action you specify it will not perform the update at all."]
    pub fn set_most_disruptive_allowed_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.most_disruptive_allowed_action = Some(v.into());
        self
    }
    #[doc = "Set the field `replacement_method`.\nThe instance replacement method for managed instance groups. Valid values are: \"RECREATE\", \"SUBSTITUTE\". If SUBSTITUTE (default), the group replaces VM instances with new instances that have randomly generated names. If RECREATE, instance names are preserved.  You must also set max_unavailable_fixed or max_unavailable_percent to be greater than 0."]
    pub fn set_replacement_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.replacement_method = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInstanceGroupManagerUpdatePolicyEl {
    type O = BlockAssignable<ComputeInstanceGroupManagerUpdatePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceGroupManagerUpdatePolicyEl {
    #[doc = "Minimal action to be taken on an instance. You can specify either NONE to forbid any actions, REFRESH to update without stopping instances, RESTART to restart existing instances or REPLACE to delete and create new instances from the target template. If you specify a REFRESH, the Updater will attempt to perform that action only. However, if the Updater determines that the minimal action you specify is not enough to perform the update, it might perform a more disruptive action."]
    pub minimal_action: PrimField<String>,
    #[doc = "The type of update process. You can specify either PROACTIVE so that the instance group manager proactively executes actions in order to bring instances to their target versions or OPPORTUNISTIC so that no action is proactively executed but the update will be performed as part of other actions (for example, resizes or recreateInstances calls)."]
    pub type_: PrimField<String>,
}
impl BuildComputeInstanceGroupManagerUpdatePolicyEl {
    pub fn build(self) -> ComputeInstanceGroupManagerUpdatePolicyEl {
        ComputeInstanceGroupManagerUpdatePolicyEl {
            max_surge_fixed: core::default::Default::default(),
            max_surge_percent: core::default::Default::default(),
            max_unavailable_fixed: core::default::Default::default(),
            max_unavailable_percent: core::default::Default::default(),
            minimal_action: self.minimal_action,
            most_disruptive_allowed_action: core::default::Default::default(),
            replacement_method: core::default::Default::default(),
            type_: self.type_,
        }
    }
}
pub struct ComputeInstanceGroupManagerUpdatePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceGroupManagerUpdatePolicyElRef {
    fn new(shared: StackShared, base: String) -> ComputeInstanceGroupManagerUpdatePolicyElRef {
        ComputeInstanceGroupManagerUpdatePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceGroupManagerUpdatePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_surge_fixed` after provisioning.\nSpecifies a fixed number of VM instances. This must be a positive integer. Conflicts with max_surge_percent. Both cannot be 0"]
    pub fn max_surge_fixed(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_surge_fixed", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_surge_percent` after provisioning.\nSpecifies a percentage of instances between 0 to 100%, inclusive. For example, specify 80 for 80%. Conflicts with max_surge_fixed."]
    pub fn max_surge_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_surge_percent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_unavailable_fixed` after provisioning.\nSpecifies a fixed number of VM instances. This must be a positive integer."]
    pub fn max_unavailable_fixed(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_unavailable_fixed", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_unavailable_percent` after provisioning.\nSpecifies a percentage of instances between 0 to 100%, inclusive. For example, specify 80 for 80%."]
    pub fn max_unavailable_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_unavailable_percent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `minimal_action` after provisioning.\nMinimal action to be taken on an instance. You can specify either NONE to forbid any actions, REFRESH to update without stopping instances, RESTART to restart existing instances or REPLACE to delete and create new instances from the target template. If you specify a REFRESH, the Updater will attempt to perform that action only. However, if the Updater determines that the minimal action you specify is not enough to perform the update, it might perform a more disruptive action."]
    pub fn minimal_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minimal_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `most_disruptive_allowed_action` after provisioning.\nMost disruptive action that is allowed to be taken on an instance. You can specify either NONE to forbid any actions, REFRESH to allow actions that do not need instance restart, RESTART to allow actions that can be applied without instance replacing or REPLACE to allow all possible actions. If the Updater determines that the minimal update action needed is more disruptive than most disruptive allowed action you specify it will not perform the update at all."]
    pub fn most_disruptive_allowed_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.most_disruptive_allowed_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `replacement_method` after provisioning.\nThe instance replacement method for managed instance groups. Valid values are: \"RECREATE\", \"SUBSTITUTE\". If SUBSTITUTE (default), the group replaces VM instances with new instances that have randomly generated names. If RECREATE, instance names are preserved.  You must also set max_unavailable_fixed or max_unavailable_percent to be greater than 0."]
    pub fn replacement_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replacement_method", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of update process. You can specify either PROACTIVE so that the instance group manager proactively executes actions in order to bring instances to their target versions or OPPORTUNISTIC so that no action is proactively executed but the update will be performed as part of other actions (for example, resizes or recreateInstances calls)."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeInstanceGroupManagerVersionElTargetSizeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percent: Option<PrimField<f64>>,
}
impl ComputeInstanceGroupManagerVersionElTargetSizeEl {
    #[doc = "Set the field `fixed`.\nThe number of instances which are managed for this version. Conflicts with percent."]
    pub fn set_fixed(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.fixed = Some(v.into());
        self
    }
    #[doc = "Set the field `percent`.\nThe number of instances (calculated as percentage) which are managed for this version. Conflicts with fixed. Note that when using percent, rounding will be in favor of explicitly set target_size values; a managed instance group with 2 instances and 2 versions, one of which has a target_size.percent of 60 will create 2 instances of that version."]
    pub fn set_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percent = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInstanceGroupManagerVersionElTargetSizeEl {
    type O = BlockAssignable<ComputeInstanceGroupManagerVersionElTargetSizeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceGroupManagerVersionElTargetSizeEl {}
impl BuildComputeInstanceGroupManagerVersionElTargetSizeEl {
    pub fn build(self) -> ComputeInstanceGroupManagerVersionElTargetSizeEl {
        ComputeInstanceGroupManagerVersionElTargetSizeEl {
            fixed: core::default::Default::default(),
            percent: core::default::Default::default(),
        }
    }
}
pub struct ComputeInstanceGroupManagerVersionElTargetSizeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceGroupManagerVersionElTargetSizeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInstanceGroupManagerVersionElTargetSizeElRef {
        ComputeInstanceGroupManagerVersionElTargetSizeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceGroupManagerVersionElTargetSizeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `fixed` after provisioning.\nThe number of instances which are managed for this version. Conflicts with percent."]
    pub fn fixed(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.fixed", self.base))
    }
    #[doc = "Get a reference to the value of field `percent` after provisioning.\nThe number of instances (calculated as percentage) which are managed for this version. Conflicts with fixed. Note that when using percent, rounding will be in favor of explicitly set target_size values; a managed instance group with 2 instances and 2 versions, one of which has a target_size.percent of 60 will create 2 instances of that version."]
    pub fn percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeInstanceGroupManagerVersionElDynamic {
    target_size: Option<DynamicBlock<ComputeInstanceGroupManagerVersionElTargetSizeEl>>,
}
#[derive(Serialize)]
pub struct ComputeInstanceGroupManagerVersionEl {
    instance_template: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_size: Option<Vec<ComputeInstanceGroupManagerVersionElTargetSizeEl>>,
    dynamic: ComputeInstanceGroupManagerVersionElDynamic,
}
impl ComputeInstanceGroupManagerVersionEl {
    #[doc = "Set the field `name`.\nVersion name."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `target_size`.\n"]
    pub fn set_target_size(
        mut self,
        v: impl Into<BlockAssignable<ComputeInstanceGroupManagerVersionElTargetSizeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target_size = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target_size = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeInstanceGroupManagerVersionEl {
    type O = BlockAssignable<ComputeInstanceGroupManagerVersionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInstanceGroupManagerVersionEl {
    #[doc = "The full URL to an instance template from which all new instances of this version will be created."]
    pub instance_template: PrimField<String>,
}
impl BuildComputeInstanceGroupManagerVersionEl {
    pub fn build(self) -> ComputeInstanceGroupManagerVersionEl {
        ComputeInstanceGroupManagerVersionEl {
            instance_template: self.instance_template,
            name: core::default::Default::default(),
            target_size: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeInstanceGroupManagerVersionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInstanceGroupManagerVersionElRef {
    fn new(shared: StackShared, base: String) -> ComputeInstanceGroupManagerVersionElRef {
        ComputeInstanceGroupManagerVersionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInstanceGroupManagerVersionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_template` after provisioning.\nThe full URL to an instance template from which all new instances of this version will be created."]
    pub fn instance_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_template", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nVersion name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `target_size` after provisioning.\n"]
    pub fn target_size(&self) -> ListRef<ComputeInstanceGroupManagerVersionElTargetSizeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target_size", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeInstanceGroupManagerDynamic {
    all_instances_config: Option<DynamicBlock<ComputeInstanceGroupManagerAllInstancesConfigEl>>,
    auto_healing_policies: Option<DynamicBlock<ComputeInstanceGroupManagerAutoHealingPoliciesEl>>,
    instance_lifecycle_policy:
        Option<DynamicBlock<ComputeInstanceGroupManagerInstanceLifecyclePolicyEl>>,
    named_port: Option<DynamicBlock<ComputeInstanceGroupManagerNamedPortEl>>,
    resource_policies: Option<DynamicBlock<ComputeInstanceGroupManagerResourcePoliciesEl>>,
    standby_policy: Option<DynamicBlock<ComputeInstanceGroupManagerStandbyPolicyEl>>,
    stateful_disk: Option<DynamicBlock<ComputeInstanceGroupManagerStatefulDiskEl>>,
    stateful_external_ip: Option<DynamicBlock<ComputeInstanceGroupManagerStatefulExternalIpEl>>,
    stateful_internal_ip: Option<DynamicBlock<ComputeInstanceGroupManagerStatefulInternalIpEl>>,
    target_size_policy: Option<DynamicBlock<ComputeInstanceGroupManagerTargetSizePolicyEl>>,
    update_policy: Option<DynamicBlock<ComputeInstanceGroupManagerUpdatePolicyEl>>,
    version: Option<DynamicBlock<ComputeInstanceGroupManagerVersionEl>>,
}
