use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ContainerNodePoolData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cluster: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_pods_per_node: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_locations: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling: Option<Vec<ContainerNodePoolAutoscalingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    management: Option<Vec<ContainerNodePoolManagementEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_config: Option<Vec<ContainerNodePoolNetworkConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_config: Option<Vec<ContainerNodePoolNodeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_drain_config: Option<Vec<ContainerNodePoolNodeDrainConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement_policy: Option<Vec<ContainerNodePoolPlacementPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queued_provisioning: Option<Vec<ContainerNodePoolQueuedProvisioningEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ContainerNodePoolTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upgrade_settings: Option<Vec<ContainerNodePoolUpgradeSettingsEl>>,
    dynamic: ContainerNodePoolDynamic,
}
struct ContainerNodePool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ContainerNodePoolData>,
}
#[derive(Clone)]
pub struct ContainerNodePool(Rc<ContainerNodePool_>);
impl ContainerNodePool {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `initial_node_count`.\nThe initial number of nodes for the pool. In regional or multi-zonal clusters, this is the number of nodes per zone. Changing this will force recreation of the resource."]
    pub fn set_initial_node_count(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().initial_node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe location (region or zone) of the cluster."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `max_pods_per_node`.\nThe maximum number of pods per node in this node pool. Note that this does not work on node pools which are \"route-based\" - that is, node pools belonging to clusters that do not have IP Aliasing enabled."]
    pub fn set_max_pods_per_node(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_pods_per_node = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nThe name of the node pool. If left blank, Terraform will auto-generate a unique name."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `name_prefix`.\nCreates a unique name for the node pool beginning with the specified prefix. Conflicts with name."]
    pub fn set_name_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `node_count`.\nThe number of nodes per instance group. This field can be used to update the number of nodes per instance group but should not be used alongside autoscaling."]
    pub fn set_node_count(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `node_locations`.\nThe list of zones in which the node pool's nodes should be located. Nodes must be in the region of their regional cluster or in the same region as their cluster's zone for zonal clusters. If unspecified, the cluster-level node_locations will be used."]
    pub fn set_node_locations(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().node_locations = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe ID of the project in which to create the node pool. If blank, the provider-configured project will be used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\nThe Kubernetes version for the nodes in this pool. Note that if this field and auto_upgrade are both specified, they will fight each other for what the node version should be, so setting both is highly discouraged. While a fuzzy version can be specified, it's recommended that you specify explicit versions as Terraform will see spurious diffs when fuzzy versions are used. See the google_container_engine_versions data source's version_prefix field to approximate fuzzy versions in a Terraform-compatible way."]
    pub fn set_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version = Some(v.into());
        self
    }
    #[doc = "Set the field `autoscaling`.\n"]
    pub fn set_autoscaling(
        self,
        v: impl Into<BlockAssignable<ContainerNodePoolAutoscalingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().autoscaling = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.autoscaling = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `management`.\n"]
    pub fn set_management(
        self,
        v: impl Into<BlockAssignable<ContainerNodePoolManagementEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().management = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.management = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_config`.\n"]
    pub fn set_network_config(
        self,
        v: impl Into<BlockAssignable<ContainerNodePoolNetworkConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `node_config`.\n"]
    pub fn set_node_config(
        self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `node_drain_config`.\n"]
    pub fn set_node_drain_config(
        self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeDrainConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_drain_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_drain_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `placement_policy`.\n"]
    pub fn set_placement_policy(
        self,
        v: impl Into<BlockAssignable<ContainerNodePoolPlacementPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().placement_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.placement_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `queued_provisioning`.\n"]
    pub fn set_queued_provisioning(
        self,
        v: impl Into<BlockAssignable<ContainerNodePoolQueuedProvisioningEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().queued_provisioning = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.queued_provisioning = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ContainerNodePoolTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `upgrade_settings`.\n"]
    pub fn set_upgrade_settings(
        self,
        v: impl Into<BlockAssignable<ContainerNodePoolUpgradeSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().upgrade_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.upgrade_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\nThe cluster to create the node pool for. Cluster must be present in location provided for zonal clusters."]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `initial_node_count` after provisioning.\nThe initial number of nodes for the pool. In regional or multi-zonal clusters, this is the number of nodes per zone. Changing this will force recreation of the resource."]
    pub fn initial_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_node_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_group_urls` after provisioning.\nThe resource URLs of the managed instance groups associated with this node pool."]
    pub fn instance_group_urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_group_urls", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location (region or zone) of the cluster."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed_instance_group_urls` after provisioning.\nList of instance group URLs which have been assigned to this node pool."]
    pub fn managed_instance_group_urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_instance_group_urls", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_pods_per_node` after provisioning.\nThe maximum number of pods per node in this node pool. Note that this does not work on node pools which are \"route-based\" - that is, node pools belonging to clusters that do not have IP Aliasing enabled."]
    pub fn max_pods_per_node(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_pods_per_node", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the node pool. If left blank, Terraform will auto-generate a unique name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\nCreates a unique name for the node pool beginning with the specified prefix. Conflicts with name."]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes per instance group. This field can be used to update the number of nodes per instance group but should not be used alongside autoscaling."]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_locations` after provisioning.\nThe list of zones in which the node pool's nodes should be located. Nodes must be in the region of their regional cluster or in the same region as their cluster's zone for zonal clusters. If unspecified, the cluster-level node_locations will be used."]
    pub fn node_locations(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.node_locations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.operation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which to create the node pool. If blank, the provider-configured project will be used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe Kubernetes version for the nodes in this pool. Note that if this field and auto_upgrade are both specified, they will fight each other for what the node version should be, so setting both is highly discouraged. While a fuzzy version can be specified, it's recommended that you specify explicit versions as Terraform will see spurious diffs when fuzzy versions are used. See the google_container_engine_versions data source's version_prefix field to approximate fuzzy versions in a Terraform-compatible way."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `autoscaling` after provisioning.\n"]
    pub fn autoscaling(&self) -> ListRef<ContainerNodePoolAutoscalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> ListRef<ContainerNodePoolManagementElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.management", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<ContainerNodePoolNetworkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> ListRef<ContainerNodePoolNodeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_drain_config` after provisioning.\n"]
    pub fn node_drain_config(&self) -> ListRef<ContainerNodePoolNodeDrainConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_drain_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `placement_policy` after provisioning.\n"]
    pub fn placement_policy(&self) -> ListRef<ContainerNodePoolPlacementPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.placement_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `queued_provisioning` after provisioning.\n"]
    pub fn queued_provisioning(&self) -> ListRef<ContainerNodePoolQueuedProvisioningElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.queued_provisioning", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerNodePoolTimeoutsElRef {
        ContainerNodePoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `upgrade_settings` after provisioning.\n"]
    pub fn upgrade_settings(&self) -> ListRef<ContainerNodePoolUpgradeSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.upgrade_settings", self.extract_ref()),
        )
    }
}
impl Referable for ContainerNodePool {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ContainerNodePool {}
impl ToListMappable for ContainerNodePool {
    type O = ListRef<ContainerNodePoolRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ContainerNodePool_ {
    fn extract_resource_type(&self) -> String {
        "google_container_node_pool".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildContainerNodePool {
    pub tf_id: String,
    #[doc = "The cluster to create the node pool for. Cluster must be present in location provided for zonal clusters."]
    pub cluster: PrimField<String>,
}
impl BuildContainerNodePool {
    pub fn build(self, stack: &mut Stack) -> ContainerNodePool {
        let out = ContainerNodePool(Rc::new(ContainerNodePool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ContainerNodePoolData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cluster: self.cluster,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                initial_node_count: core::default::Default::default(),
                location: core::default::Default::default(),
                max_pods_per_node: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                node_count: core::default::Default::default(),
                node_locations: core::default::Default::default(),
                project: core::default::Default::default(),
                version: core::default::Default::default(),
                autoscaling: core::default::Default::default(),
                management: core::default::Default::default(),
                network_config: core::default::Default::default(),
                node_config: core::default::Default::default(),
                node_drain_config: core::default::Default::default(),
                placement_policy: core::default::Default::default(),
                queued_provisioning: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                upgrade_settings: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ContainerNodePoolRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ContainerNodePoolRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\nThe cluster to create the node pool for. Cluster must be present in location provided for zonal clusters."]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `initial_node_count` after provisioning.\nThe initial number of nodes for the pool. In regional or multi-zonal clusters, this is the number of nodes per zone. Changing this will force recreation of the resource."]
    pub fn initial_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_node_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_group_urls` after provisioning.\nThe resource URLs of the managed instance groups associated with this node pool."]
    pub fn instance_group_urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_group_urls", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location (region or zone) of the cluster."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed_instance_group_urls` after provisioning.\nList of instance group URLs which have been assigned to this node pool."]
    pub fn managed_instance_group_urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_instance_group_urls", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_pods_per_node` after provisioning.\nThe maximum number of pods per node in this node pool. Note that this does not work on node pools which are \"route-based\" - that is, node pools belonging to clusters that do not have IP Aliasing enabled."]
    pub fn max_pods_per_node(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_pods_per_node", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the node pool. If left blank, Terraform will auto-generate a unique name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\nCreates a unique name for the node pool beginning with the specified prefix. Conflicts with name."]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes per instance group. This field can be used to update the number of nodes per instance group but should not be used alongside autoscaling."]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_locations` after provisioning.\nThe list of zones in which the node pool's nodes should be located. Nodes must be in the region of their regional cluster or in the same region as their cluster's zone for zonal clusters. If unspecified, the cluster-level node_locations will be used."]
    pub fn node_locations(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.node_locations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.operation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which to create the node pool. If blank, the provider-configured project will be used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe Kubernetes version for the nodes in this pool. Note that if this field and auto_upgrade are both specified, they will fight each other for what the node version should be, so setting both is highly discouraged. While a fuzzy version can be specified, it's recommended that you specify explicit versions as Terraform will see spurious diffs when fuzzy versions are used. See the google_container_engine_versions data source's version_prefix field to approximate fuzzy versions in a Terraform-compatible way."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `autoscaling` after provisioning.\n"]
    pub fn autoscaling(&self) -> ListRef<ContainerNodePoolAutoscalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> ListRef<ContainerNodePoolManagementElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.management", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<ContainerNodePoolNetworkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> ListRef<ContainerNodePoolNodeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_drain_config` after provisioning.\n"]
    pub fn node_drain_config(&self) -> ListRef<ContainerNodePoolNodeDrainConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_drain_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `placement_policy` after provisioning.\n"]
    pub fn placement_policy(&self) -> ListRef<ContainerNodePoolPlacementPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.placement_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `queued_provisioning` after provisioning.\n"]
    pub fn queued_provisioning(&self) -> ListRef<ContainerNodePoolQueuedProvisioningElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.queued_provisioning", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerNodePoolTimeoutsElRef {
        ContainerNodePoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `upgrade_settings` after provisioning.\n"]
    pub fn upgrade_settings(&self) -> ListRef<ContainerNodePoolUpgradeSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.upgrade_settings", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolAutoscalingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    location_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_max_node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_min_node_count: Option<PrimField<f64>>,
}
impl ContainerNodePoolAutoscalingEl {
    #[doc = "Set the field `location_policy`.\nLocation policy specifies the algorithm used when scaling-up the node pool. \"BALANCED\" - Is a best effort policy that aims to balance the sizes of available zones. \"ANY\" - Instructs the cluster autoscaler to prioritize utilization of unused reservations, and reduces preemption risk for Spot VMs."]
    pub fn set_location_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `max_node_count`.\nMaximum number of nodes per zone in the node pool. Must be >= min_node_count. Cannot be used with total limits."]
    pub fn set_max_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `min_node_count`.\nMinimum number of nodes per zone in the node pool. Must be >=0 and <= max_node_count. Cannot be used with total limits."]
    pub fn set_min_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `total_max_node_count`.\nMaximum number of all nodes in the node pool. Must be >= total_min_node_count. Cannot be used with per zone limits."]
    pub fn set_total_max_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.total_max_node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `total_min_node_count`.\nMinimum number of all nodes in the node pool. Must be >=0 and <= total_max_node_count. Cannot be used with per zone limits."]
    pub fn set_total_min_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.total_min_node_count = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerNodePoolAutoscalingEl {
    type O = BlockAssignable<ContainerNodePoolAutoscalingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolAutoscalingEl {}
impl BuildContainerNodePoolAutoscalingEl {
    pub fn build(self) -> ContainerNodePoolAutoscalingEl {
        ContainerNodePoolAutoscalingEl {
            location_policy: core::default::Default::default(),
            max_node_count: core::default::Default::default(),
            min_node_count: core::default::Default::default(),
            total_max_node_count: core::default::Default::default(),
            total_min_node_count: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolAutoscalingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolAutoscalingElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolAutoscalingElRef {
        ContainerNodePoolAutoscalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolAutoscalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `location_policy` after provisioning.\nLocation policy specifies the algorithm used when scaling-up the node pool. \"BALANCED\" - Is a best effort policy that aims to balance the sizes of available zones. \"ANY\" - Instructs the cluster autoscaler to prioritize utilization of unused reservations, and reduces preemption risk for Spot VMs."]
    pub fn location_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_node_count` after provisioning.\nMaximum number of nodes per zone in the node pool. Must be >= min_node_count. Cannot be used with total limits."]
    pub fn max_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_node_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_node_count` after provisioning.\nMinimum number of nodes per zone in the node pool. Must be >=0 and <= max_node_count. Cannot be used with total limits."]
    pub fn min_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_node_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `total_max_node_count` after provisioning.\nMaximum number of all nodes in the node pool. Must be >= total_min_node_count. Cannot be used with per zone limits."]
    pub fn total_max_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_max_node_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `total_min_node_count` after provisioning.\nMinimum number of all nodes in the node pool. Must be >=0 and <= total_max_node_count. Cannot be used with per zone limits."]
    pub fn total_min_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_min_node_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolManagementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_repair: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_upgrade: Option<PrimField<bool>>,
}
impl ContainerNodePoolManagementEl {
    #[doc = "Set the field `auto_repair`.\nWhether the nodes will be automatically repaired. Enabled by default."]
    pub fn set_auto_repair(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_repair = Some(v.into());
        self
    }
    #[doc = "Set the field `auto_upgrade`.\nWhether the nodes will be automatically upgraded. Enabled by default."]
    pub fn set_auto_upgrade(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_upgrade = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerNodePoolManagementEl {
    type O = BlockAssignable<ContainerNodePoolManagementEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolManagementEl {}
impl BuildContainerNodePoolManagementEl {
    pub fn build(self) -> ContainerNodePoolManagementEl {
        ContainerNodePoolManagementEl {
            auto_repair: core::default::Default::default(),
            auto_upgrade: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolManagementElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolManagementElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolManagementElRef {
        ContainerNodePoolManagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolManagementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto_repair` after provisioning.\nWhether the nodes will be automatically repaired. Enabled by default."]
    pub fn auto_repair(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_repair", self.base))
    }
    #[doc = "Get a reference to the value of field `auto_upgrade` after provisioning.\nWhether the nodes will be automatically upgraded. Enabled by default."]
    pub fn auto_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_upgrade", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNetworkConfigElAdditionalNodeNetworkConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
}
impl ContainerNodePoolNetworkConfigElAdditionalNodeNetworkConfigsEl {
    #[doc = "Set the field `network`.\nName of the VPC where the additional interface belongs."]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\nName of the subnetwork where the additional interface belongs."]
    pub fn set_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerNodePoolNetworkConfigElAdditionalNodeNetworkConfigsEl {
    type O = BlockAssignable<ContainerNodePoolNetworkConfigElAdditionalNodeNetworkConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNetworkConfigElAdditionalNodeNetworkConfigsEl {}
impl BuildContainerNodePoolNetworkConfigElAdditionalNodeNetworkConfigsEl {
    pub fn build(self) -> ContainerNodePoolNetworkConfigElAdditionalNodeNetworkConfigsEl {
        ContainerNodePoolNetworkConfigElAdditionalNodeNetworkConfigsEl {
            network: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolNetworkConfigElAdditionalNodeNetworkConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNetworkConfigElAdditionalNodeNetworkConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNetworkConfigElAdditionalNodeNetworkConfigsElRef {
        ContainerNodePoolNetworkConfigElAdditionalNodeNetworkConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNetworkConfigElAdditionalNodeNetworkConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nName of the VPC where the additional interface belongs."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nName of the subnetwork where the additional interface belongs."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNetworkConfigElAdditionalPodNetworkConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_pods_per_node: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_pod_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
}
impl ContainerNodePoolNetworkConfigElAdditionalPodNetworkConfigsEl {
    #[doc = "Set the field `max_pods_per_node`.\nThe maximum number of pods per node which use this pod network."]
    pub fn set_max_pods_per_node(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_pods_per_node = Some(v.into());
        self
    }
    #[doc = "Set the field `secondary_pod_range`.\nThe name of the secondary range on the subnet which provides IP address for this pod range."]
    pub fn set_secondary_pod_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secondary_pod_range = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\nName of the subnetwork where the additional pod network belongs."]
    pub fn set_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerNodePoolNetworkConfigElAdditionalPodNetworkConfigsEl {
    type O = BlockAssignable<ContainerNodePoolNetworkConfigElAdditionalPodNetworkConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNetworkConfigElAdditionalPodNetworkConfigsEl {}
impl BuildContainerNodePoolNetworkConfigElAdditionalPodNetworkConfigsEl {
    pub fn build(self) -> ContainerNodePoolNetworkConfigElAdditionalPodNetworkConfigsEl {
        ContainerNodePoolNetworkConfigElAdditionalPodNetworkConfigsEl {
            max_pods_per_node: core::default::Default::default(),
            secondary_pod_range: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolNetworkConfigElAdditionalPodNetworkConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNetworkConfigElAdditionalPodNetworkConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNetworkConfigElAdditionalPodNetworkConfigsElRef {
        ContainerNodePoolNetworkConfigElAdditionalPodNetworkConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNetworkConfigElAdditionalPodNetworkConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_pods_per_node` after provisioning.\nThe maximum number of pods per node which use this pod network."]
    pub fn max_pods_per_node(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_pods_per_node", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_pod_range` after provisioning.\nThe name of the secondary range on the subnet which provides IP address for this pod range."]
    pub fn secondary_pod_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secondary_pod_range", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nName of the subnetwork where the additional pod network belongs."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl {
    total_egress_bandwidth_tier: PrimField<String>,
}
impl ContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl {}
impl ToListMappable for ContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl {
    type O = BlockAssignable<ContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl {
    #[doc = "Specifies the total network bandwidth tier for the NodePool. [Valid values](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters.nodePools#NodePool.Tier) include: \"TIER_1\" and \"TIER_UNSPECIFIED\"."]
    pub total_egress_bandwidth_tier: PrimField<String>,
}
impl BuildContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl {
    pub fn build(self) -> ContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl {
        ContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl {
            total_egress_bandwidth_tier: self.total_egress_bandwidth_tier,
        }
    }
}
pub struct ContainerNodePoolNetworkConfigElNetworkPerformanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNetworkConfigElNetworkPerformanceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNetworkConfigElNetworkPerformanceConfigElRef {
        ContainerNodePoolNetworkConfigElNetworkPerformanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNetworkConfigElNetworkPerformanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `total_egress_bandwidth_tier` after provisioning.\nSpecifies the total network bandwidth tier for the NodePool. [Valid values](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters.nodePools#NodePool.Tier) include: \"TIER_1\" and \"TIER_UNSPECIFIED\"."]
    pub fn total_egress_bandwidth_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_egress_bandwidth_tier", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl {
    disabled: PrimField<bool>,
}
impl ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl {}
impl ToListMappable for ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl {
    type O = BlockAssignable<ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl {
    #[doc = ""]
    pub disabled: PrimField<bool>,
}
impl BuildContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl {
    pub fn build(self) -> ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl {
        ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl {
            disabled: self.disabled,
        }
    }
}
pub struct ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigElRef {
        ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}
#[derive(Serialize, Default)]
struct ContainerNodePoolNetworkConfigElDynamic {
    additional_node_network_configs:
        Option<DynamicBlock<ContainerNodePoolNetworkConfigElAdditionalNodeNetworkConfigsEl>>,
    additional_pod_network_configs:
        Option<DynamicBlock<ContainerNodePoolNetworkConfigElAdditionalPodNetworkConfigsEl>>,
    network_performance_config:
        Option<DynamicBlock<ContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl>>,
    pod_cidr_overprovision_config:
        Option<DynamicBlock<ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl>>,
}
#[derive(Serialize)]
pub struct ContainerNodePoolNetworkConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_network_profile: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_pod_range: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_private_nodes: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_ipv4_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_node_network_configs:
        Option<Vec<ContainerNodePoolNetworkConfigElAdditionalNodeNetworkConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_pod_network_configs:
        Option<Vec<ContainerNodePoolNetworkConfigElAdditionalPodNetworkConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_performance_config:
        Option<Vec<ContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_cidr_overprovision_config:
        Option<Vec<ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl>>,
    dynamic: ContainerNodePoolNetworkConfigElDynamic,
}
impl ContainerNodePoolNetworkConfigEl {
    #[doc = "Set the field `accelerator_network_profile`.\nThe accelerator network profile to use for this node pool."]
    pub fn set_accelerator_network_profile(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.accelerator_network_profile = Some(v.into());
        self
    }
    #[doc = "Set the field `create_pod_range`.\nWhether to create a new range for pod IPs in this node pool. Defaults are provided for pod_range and pod_ipv4_cidr_block if they are not specified."]
    pub fn set_create_pod_range(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.create_pod_range = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_private_nodes`.\nWhether nodes have internal IP addresses only."]
    pub fn set_enable_private_nodes(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_private_nodes = Some(v.into());
        self
    }
    #[doc = "Set the field `pod_ipv4_cidr_block`.\nThe IP address range for pod IPs in this node pool. Only applicable if create_pod_range is true. Set to blank to have a range chosen with the default size. Set to /netmask (e.g. /14) to have a range chosen with a specific netmask. Set to a CIDR notation (e.g. 10.96.0.0/14) to pick a specific range to use."]
    pub fn set_pod_ipv4_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pod_ipv4_cidr_block = Some(v.into());
        self
    }
    #[doc = "Set the field `pod_range`.\nThe ID of the secondary range for pod IPs. If create_pod_range is true, this ID is used for the new range. If create_pod_range is false, uses an existing secondary range with this ID."]
    pub fn set_pod_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pod_range = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\nThe subnetwork name/path for the node pool. Format: subnetwork or projects/{project}/regions/{region}/subnetworks/{subnetwork}. This value may be specified via the nested network_config block (setting this attribute directly is supported for backward compatibility). Once created the node pool's subnetwork is immutable. If not set, the provider/API will choose the subnetwork (e.g. based on IP utilization) and report it here."]
    pub fn set_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork = Some(v.into());
        self
    }
    #[doc = "Set the field `additional_node_network_configs`.\n"]
    pub fn set_additional_node_network_configs(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNetworkConfigElAdditionalNodeNetworkConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.additional_node_network_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_node_network_configs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `additional_pod_network_configs`.\n"]
    pub fn set_additional_pod_network_configs(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNetworkConfigElAdditionalPodNetworkConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.additional_pod_network_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_pod_network_configs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_performance_config`.\n"]
    pub fn set_network_performance_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_performance_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_performance_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `pod_cidr_overprovision_config`.\n"]
    pub fn set_pod_cidr_overprovision_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pod_cidr_overprovision_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pod_cidr_overprovision_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ContainerNodePoolNetworkConfigEl {
    type O = BlockAssignable<ContainerNodePoolNetworkConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNetworkConfigEl {}
impl BuildContainerNodePoolNetworkConfigEl {
    pub fn build(self) -> ContainerNodePoolNetworkConfigEl {
        ContainerNodePoolNetworkConfigEl {
            accelerator_network_profile: core::default::Default::default(),
            create_pod_range: core::default::Default::default(),
            enable_private_nodes: core::default::Default::default(),
            pod_ipv4_cidr_block: core::default::Default::default(),
            pod_range: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            additional_node_network_configs: core::default::Default::default(),
            additional_pod_network_configs: core::default::Default::default(),
            network_performance_config: core::default::Default::default(),
            pod_cidr_overprovision_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContainerNodePoolNetworkConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNetworkConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNetworkConfigElRef {
        ContainerNodePoolNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNetworkConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accelerator_network_profile` after provisioning.\nThe accelerator network profile to use for this node pool."]
    pub fn accelerator_network_profile(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_network_profile", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_pod_range` after provisioning.\nWhether to create a new range for pod IPs in this node pool. Defaults are provided for pod_range and pod_ipv4_cidr_block if they are not specified."]
    pub fn create_pod_range(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_pod_range", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_private_nodes` after provisioning.\nWhether nodes have internal IP addresses only."]
    pub fn enable_private_nodes(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_private_nodes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pod_ipv4_cidr_block` after provisioning.\nThe IP address range for pod IPs in this node pool. Only applicable if create_pod_range is true. Set to blank to have a range chosen with the default size. Set to /netmask (e.g. /14) to have a range chosen with a specific netmask. Set to a CIDR notation (e.g. 10.96.0.0/14) to pick a specific range to use."]
    pub fn pod_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pod_ipv4_cidr_block", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pod_range` after provisioning.\nThe ID of the secondary range for pod IPs. If create_pod_range is true, this ID is used for the new range. If create_pod_range is false, uses an existing secondary range with this ID."]
    pub fn pod_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pod_range", self.base))
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nThe subnetwork name/path for the node pool. Format: subnetwork or projects/{project}/regions/{region}/subnetworks/{subnetwork}. This value may be specified via the nested network_config block (setting this attribute directly is supported for backward compatibility). Once created the node pool's subnetwork is immutable. If not set, the provider/API will choose the subnetwork (e.g. based on IP utilization) and report it here."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
    #[doc = "Get a reference to the value of field `additional_node_network_configs` after provisioning.\n"]
    pub fn additional_node_network_configs(
        &self,
    ) -> ListRef<ContainerNodePoolNetworkConfigElAdditionalNodeNetworkConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_node_network_configs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `additional_pod_network_configs` after provisioning.\n"]
    pub fn additional_pod_network_configs(
        &self,
    ) -> ListRef<ContainerNodePoolNetworkConfigElAdditionalPodNetworkConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_pod_network_configs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network_performance_config` after provisioning.\n"]
    pub fn network_performance_config(
        &self,
    ) -> ListRef<ContainerNodePoolNetworkConfigElNetworkPerformanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_performance_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pod_cidr_overprovision_config` after provisioning.\n"]
    pub fn pod_cidr_overprovision_config(
        &self,
    ) -> ListRef<ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pod_cidr_overprovision_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElEffectiveTaintsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ContainerNodePoolNodeConfigElEffectiveTaintsEl {
    #[doc = "Set the field `effect`.\n"]
    pub fn set_effect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.effect = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeConfigElEffectiveTaintsEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElEffectiveTaintsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElEffectiveTaintsEl {}
impl BuildContainerNodePoolNodeConfigElEffectiveTaintsEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElEffectiveTaintsEl {
        ContainerNodePoolNodeConfigElEffectiveTaintsEl {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElEffectiveTaintsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElEffectiveTaintsElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElEffectiveTaintsElRef {
        ContainerNodePoolNodeConfigElEffectiveTaintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElEffectiveTaintsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `effect` after provisioning.\n"]
    pub fn effect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_nested_virtualization: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performance_monitoring_unit: Option<PrimField<String>>,
    threads_per_core: PrimField<f64>,
}
impl ContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl {
    #[doc = "Set the field `enable_nested_virtualization`.\nWhether the node should have nested virtualization enabled."]
    pub fn set_enable_nested_virtualization(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_nested_virtualization = Some(v.into());
        self
    }
    #[doc = "Set the field `performance_monitoring_unit`.\nLevel of Performance Monitoring Unit (PMU) requested. If unset, no access to the PMU is assumed."]
    pub fn set_performance_monitoring_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.performance_monitoring_unit = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl {
    #[doc = "The number of threads per physical core. To disable simultaneous multithreading (SMT) set this to 1. If unset, the maximum number of threads supported per core by the underlying processor is assumed."]
    pub threads_per_core: PrimField<f64>,
}
impl BuildContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl {
        ContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl {
            enable_nested_virtualization: core::default::Default::default(),
            performance_monitoring_unit: core::default::Default::default(),
            threads_per_core: self.threads_per_core,
        }
    }
}
pub struct ContainerNodePoolNodeConfigElAdvancedMachineFeaturesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElAdvancedMachineFeaturesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElAdvancedMachineFeaturesElRef {
        ContainerNodePoolNodeConfigElAdvancedMachineFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElAdvancedMachineFeaturesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_nested_virtualization` after provisioning.\nWhether the node should have nested virtualization enabled."]
    pub fn enable_nested_virtualization(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_nested_virtualization", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `performance_monitoring_unit` after provisioning.\nLevel of Performance Monitoring Unit (PMU) requested. If unset, no access to the PMU is assumed."]
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
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElBootDiskEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioned_iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioned_throughput: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size_gb: Option<PrimField<f64>>,
}
impl ContainerNodePoolNodeConfigElBootDiskEl {
    #[doc = "Set the field `disk_type`.\nType of the disk attached to each node. Such as pd-standard, pd-balanced or pd-ssd"]
    pub fn set_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_type = Some(v.into());
        self
    }
    #[doc = "Set the field `provisioned_iops`.\nConfigured IOPs provisioning. Only valid with disk type hyperdisk-balanced."]
    pub fn set_provisioned_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.provisioned_iops = Some(v.into());
        self
    }
    #[doc = "Set the field `provisioned_throughput`.\nConfigured throughput provisioning. Only valid with disk type hyperdisk-balanced."]
    pub fn set_provisioned_throughput(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.provisioned_throughput = Some(v.into());
        self
    }
    #[doc = "Set the field `size_gb`.\nSize of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB."]
    pub fn set_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size_gb = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeConfigElBootDiskEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElBootDiskEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElBootDiskEl {}
impl BuildContainerNodePoolNodeConfigElBootDiskEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElBootDiskEl {
        ContainerNodePoolNodeConfigElBootDiskEl {
            disk_type: core::default::Default::default(),
            provisioned_iops: core::default::Default::default(),
            provisioned_throughput: core::default::Default::default(),
            size_gb: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElBootDiskElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElBootDiskElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElBootDiskElRef {
        ContainerNodePoolNodeConfigElBootDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElBootDiskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk_type` after provisioning.\nType of the disk attached to each node. Such as pd-standard, pd-balanced or pd-ssd"]
    pub fn disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }
    #[doc = "Get a reference to the value of field `provisioned_iops` after provisioning.\nConfigured IOPs provisioning. Only valid with disk type hyperdisk-balanced."]
    pub fn provisioned_iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_iops", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `provisioned_throughput` after provisioning.\nConfigured throughput provisioning. Only valid with disk type hyperdisk-balanced."]
    pub fn provisioned_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_throughput", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `size_gb` after provisioning.\nSize of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB."]
    pub fn size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_gb", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElConfidentialNodesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    confidential_instance_type: Option<PrimField<String>>,
    enabled: PrimField<bool>,
}
impl ContainerNodePoolNodeConfigElConfidentialNodesEl {
    #[doc = "Set the field `confidential_instance_type`.\nDefines the type of technology used by the confidential node."]
    pub fn set_confidential_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.confidential_instance_type = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeConfigElConfidentialNodesEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElConfidentialNodesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElConfidentialNodesEl {
    #[doc = "Whether Confidential Nodes feature is enabled for all nodes in this pool."]
    pub enabled: PrimField<bool>,
}
impl BuildContainerNodePoolNodeConfigElConfidentialNodesEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElConfidentialNodesEl {
        ContainerNodePoolNodeConfigElConfidentialNodesEl {
            confidential_instance_type: core::default::Default::default(),
            enabled: self.enabled,
        }
    }
}
pub struct ContainerNodePoolNodeConfigElConfidentialNodesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElConfidentialNodesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElConfidentialNodesElRef {
        ContainerNodePoolNodeConfigElConfidentialNodesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElConfidentialNodesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `confidential_instance_type` after provisioning.\nDefines the type of technology used by the confidential node."]
    pub fn confidential_instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.confidential_instance_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether Confidential Nodes feature is enabled for all nodes in this pool."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl
{
    secret_uri: PrimField<String>,
}
impl ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl { }
impl ToListMappable for ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl { type O = BlockAssignable < ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl
{
    #[doc = "URI for the secret that hosts a certificate. Must be in the format 'projects/PROJECT_NUM/secrets/SECRET_NAME/versions/VERSION_OR_LATEST'."]
    pub secret_uri: PrimField<String>,
}
impl BuildContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl { pub fn build (self) -> ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl { ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl { secret_uri : self . secret_uri , } } }
pub struct ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigElRef { fn new (shared : StackShared , base : String) -> ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigElRef { ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigElRef { shared : shared , base : base . to_string () , } } }
impl ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `secret_uri` after provisioning.\nURI for the secret that hosts a certificate. Must be in the format 'projects/PROJECT_NUM/secrets/SECRET_NAME/versions/VERSION_OR_LATEST'."] pub fn secret_uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.secret_uri" , self . base)) } }
#[derive(Serialize, Default)]
struct ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElDynamic { gcp_secret_manager_certificate_config : Option < DynamicBlock < ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl >> , }
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl { fqdns : ListField < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] gcp_secret_manager_certificate_config : Option < Vec < ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl > > , dynamic : ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElDynamic , }
impl ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl { # [doc = "Set the field `gcp_secret_manager_certificate_config`.\n"] pub fn set_gcp_secret_manager_certificate_config (mut self , v : impl Into < BlockAssignable < ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . gcp_secret_manager_certificate_config = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . gcp_secret_manager_certificate_config = Some (d) ; } } self } }
impl ToListMappable for ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl { type O = BlockAssignable < ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl
{
    #[doc = "List of fully-qualified-domain-names. IPv4s and port specification are supported."]
    pub fqdns: ListField<PrimField<String>>,
}
impl BuildContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl { pub fn build (self) -> ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl { ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl { fqdns : self . fqdns , gcp_secret_manager_certificate_config : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElRef { fn new (shared : StackShared , base : String) -> ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElRef { ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElRef { shared : shared , base : base . to_string () , } } }
impl ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `fqdns` after provisioning.\nList of fully-qualified-domain-names. IPv4s and port specification are supported."] pub fn fqdns (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.fqdns" , self . base)) } # [doc = "Get a reference to the value of field `gcp_secret_manager_certificate_config` after provisioning.\n"] pub fn gcp_secret_manager_certificate_config (& self) -> ListRef < ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.gcp_secret_manager_certificate_config" , self . base)) } }
#[derive(Serialize, Default)]
struct ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElDynamic { certificate_authority_domain_config : Option < DynamicBlock < ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl >> , }
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl { enabled : PrimField < bool > , # [serde (skip_serializing_if = "Option::is_none")] certificate_authority_domain_config : Option < Vec < ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl > > , dynamic : ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElDynamic , }
impl ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl {
    #[doc = "Set the field `certificate_authority_domain_config`.\n"]
    pub fn set_certificate_authority_domain_config(
        mut self,
        v : impl Into < BlockAssignable < ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.certificate_authority_domain_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.certificate_authority_domain_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl
{
    type O = BlockAssignable<
        ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl {
    #[doc = "Whether or not private registries are configured."]
    pub enabled: PrimField<bool>,
}
impl BuildContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl {
    pub fn build(
        self,
    ) -> ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl {
        ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl {
            enabled: self.enabled,
            certificate_authority_domain_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElRef {
        ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether or not private registries are configured."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `certificate_authority_domain_config` after provisioning.\n"]    pub fn certificate_authority_domain_config (& self) -> ListRef < ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.certificate_authority_domain_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_secret_manager_secret_uri: Option<PrimField<String>>,
}
impl ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl {
    #[doc = "Set the field `gcp_secret_manager_secret_uri`.\nURI for the Secret Manager secret that hosts the certificate."]
    pub fn set_gcp_secret_manager_secret_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_secret_manager_secret_uri = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl {
    type O =
        BlockAssignable<ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl {}
impl BuildContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl {
    pub fn build(
        self,
    ) -> ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl {
        ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl {
            gcp_secret_manager_secret_uri: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElCaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElCaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElCaElRef {
        ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElCaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElCaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gcp_secret_manager_secret_uri` after provisioning.\nURI for the Secret Manager secret that hosts the certificate."]
    pub fn gcp_secret_manager_secret_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_secret_manager_secret_uri", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_secret_manager_secret_uri: Option<PrimField<String>>,
}
impl ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl {
    #[doc = "Set the field `gcp_secret_manager_secret_uri`.\nURI for the Secret Manager secret that hosts the client certificate."]
    pub fn set_gcp_secret_manager_secret_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_secret_manager_secret_uri = Some(v.into());
        self
    }
}
impl ToListMappable
    for ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl
{
    type O = BlockAssignable<
        ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl
{}
impl BuildContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl {
    pub fn build(
        self,
    ) -> ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl {
        ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl {
            gcp_secret_manager_secret_uri: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertElRef
    {
        ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gcp_secret_manager_secret_uri` after provisioning.\nURI for the Secret Manager secret that hosts the client certificate."]
    pub fn gcp_secret_manager_secret_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_secret_manager_secret_uri", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_secret_manager_secret_uri: Option<PrimField<String>>,
}
impl ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl {
    #[doc = "Set the field `gcp_secret_manager_secret_uri`.\nURI for the Secret Manager secret that hosts the private key."]
    pub fn set_gcp_secret_manager_secret_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_secret_manager_secret_uri = Some(v.into());
        self
    }
}
impl ToListMappable
    for ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl
{
    type O = BlockAssignable<
        ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl
{}
impl BuildContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl {
    pub fn build(
        self,
    ) -> ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl {
        ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl {
            gcp_secret_manager_secret_uri: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyElRef {
        ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gcp_secret_manager_secret_uri` after provisioning.\nURI for the Secret Manager secret that hosts the private key."]
    pub fn gcp_secret_manager_secret_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_secret_manager_secret_uri", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElDynamic {
    cert: Option<
        DynamicBlock<
            ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl,
        >,
    >,
    key: Option<
        DynamicBlock<
            ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cert: Option<
        Vec<ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<
        Vec<ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl>,
    >,
    dynamic: ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElDynamic,
}
impl ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl {
    #[doc = "Set the field `cert`.\n"]
    pub fn set_cert(
        mut self,
        v: impl Into<
            BlockAssignable<
                ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cert = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cert = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(
        mut self,
        v: impl Into<
            BlockAssignable<
                ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.key = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl
{
    type O = BlockAssignable<
        ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl {}
impl BuildContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl {
    pub fn build(
        self,
    ) -> ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl {
        ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl {
            cert: core::default::Default::default(),
            key: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElRef {
        ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cert` after provisioning.\n"]
    pub fn cert(
        &self,
    ) -> ListRef<
        ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.cert", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(
        &self,
    ) -> ListRef<
        ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.key", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl {
    key: PrimField<String>,
    value: ListField<PrimField<String>>,
}
impl ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl {}
impl ToListMappable
    for ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl
{
    type O = BlockAssignable<
        ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl {
    #[doc = "Configures the header key."]
    pub key: PrimField<String>,
    #[doc = "Configures the header value."]
    pub value: ListField<PrimField<String>>,
}
impl BuildContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl {
    pub fn build(
        self,
    ) -> ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl {
        ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl {
            key: self.key,
            value: self.value,
        }
    }
}
pub struct ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderElRef {
        ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nConfigures the header key."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nConfigures the header value."]
    pub fn value(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElDynamic {
    ca: Option<
        DynamicBlock<ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl>,
    >,
    client: Option<
        DynamicBlock<ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl>,
    >,
    header: Option<
        DynamicBlock<ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl>,
    >,
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    capabilities: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dial_timeout: Option<PrimField<String>>,
    host: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    override_path: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ca: Option<Vec<ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client:
        Option<Vec<ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header:
        Option<Vec<ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl>>,
    dynamic: ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElDynamic,
}
impl ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsEl {
    #[doc = "Set the field `capabilities`.\nRepresent the capabilities of the registry host, specifying what operations a host is capable of performing."]
    pub fn set_capabilities(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.capabilities = Some(v.into());
        self
    }
    #[doc = "Set the field `dial_timeout`.\nSpecifies the maximum duration allowed for a connection attempt to complete."]
    pub fn set_dial_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dial_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `override_path`.\nIndicate the host's API root endpoint is defined in the URL path rather than by the API specification."]
    pub fn set_override_path(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.override_path = Some(v.into());
        self
    }
    #[doc = "Set the field `ca`.\n"]
    pub fn set_ca(
        mut self,
        v: impl Into<
            BlockAssignable<
                ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ca = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ca = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `client`.\n"]
    pub fn set_client(
        mut self,
        v: impl Into<
            BlockAssignable<
                ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.client = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.client = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v: impl Into<
            BlockAssignable<
                ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsEl {
    #[doc = "Configures the registry host/mirror."]
    pub host: PrimField<String>,
}
impl BuildContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsEl {
        ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsEl {
            capabilities: core::default::Default::default(),
            dial_timeout: core::default::Default::default(),
            host: self.host,
            override_path: core::default::Default::default(),
            ca: core::default::Default::default(),
            client: core::default::Default::default(),
            header: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElRef {
        ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `capabilities` after provisioning.\nRepresent the capabilities of the registry host, specifying what operations a host is capable of performing."]
    pub fn capabilities(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.capabilities", self.base))
    }
    #[doc = "Get a reference to the value of field `dial_timeout` after provisioning.\nSpecifies the maximum duration allowed for a connection attempt to complete."]
    pub fn dial_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dial_timeout", self.base))
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\nConfigures the registry host/mirror."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `override_path` after provisioning.\nIndicate the host's API root endpoint is defined in the URL path rather than by the API specification."]
    pub fn override_path(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.override_path", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ca` after provisioning.\n"]
    pub fn ca(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElCaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ca", self.base))
    }
    #[doc = "Get a reference to the value of field `client` after provisioning.\n"]
    pub fn client(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.client", self.base))
    }
    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }
}
#[derive(Serialize, Default)]
struct ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElDynamic {
    hosts:
        Option<DynamicBlock<ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsEl>>,
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsEl {
    server: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hosts: Option<Vec<ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsEl>>,
    dynamic: ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElDynamic,
}
impl ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsEl {
    #[doc = "Set the field `hosts`.\n"]
    pub fn set_hosts(
        mut self,
        v: impl Into<
            BlockAssignable<ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hosts = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hosts = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsEl {
    #[doc = "Defines the host name of the registry server."]
    pub server: PrimField<String>,
}
impl BuildContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsEl {
        ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsEl {
            server: self.server,
            hosts: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElRef {
        ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `server` after provisioning.\nDefines the host name of the registry server."]
    pub fn server(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server", self.base))
    }
    #[doc = "Get a reference to the value of field `hosts` after provisioning.\n"]
    pub fn hosts(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElHostsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hosts", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElContainerdConfigElWritableCgroupsEl {
    enabled: PrimField<bool>,
}
impl ContainerNodePoolNodeConfigElContainerdConfigElWritableCgroupsEl {}
impl ToListMappable for ContainerNodePoolNodeConfigElContainerdConfigElWritableCgroupsEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElContainerdConfigElWritableCgroupsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElContainerdConfigElWritableCgroupsEl {
    #[doc = "Whether writable cgroups are enabled."]
    pub enabled: PrimField<bool>,
}
impl BuildContainerNodePoolNodeConfigElContainerdConfigElWritableCgroupsEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElContainerdConfigElWritableCgroupsEl {
        ContainerNodePoolNodeConfigElContainerdConfigElWritableCgroupsEl {
            enabled: self.enabled,
        }
    }
}
pub struct ContainerNodePoolNodeConfigElContainerdConfigElWritableCgroupsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElContainerdConfigElWritableCgroupsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElContainerdConfigElWritableCgroupsElRef {
        ContainerNodePoolNodeConfigElContainerdConfigElWritableCgroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElContainerdConfigElWritableCgroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether writable cgroups are enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize, Default)]
struct ContainerNodePoolNodeConfigElContainerdConfigElDynamic {
    private_registry_access_config: Option<
        DynamicBlock<ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl>,
    >,
    registry_hosts:
        Option<DynamicBlock<ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsEl>>,
    writable_cgroups:
        Option<DynamicBlock<ContainerNodePoolNodeConfigElContainerdConfigElWritableCgroupsEl>>,
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElContainerdConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    private_registry_access_config:
        Option<Vec<ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registry_hosts: Option<Vec<ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    writable_cgroups: Option<Vec<ContainerNodePoolNodeConfigElContainerdConfigElWritableCgroupsEl>>,
    dynamic: ContainerNodePoolNodeConfigElContainerdConfigElDynamic,
}
impl ContainerNodePoolNodeConfigElContainerdConfigEl {
    #[doc = "Set the field `private_registry_access_config`.\n"]
    pub fn set_private_registry_access_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.private_registry_access_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.private_registry_access_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `registry_hosts`.\n"]
    pub fn set_registry_hosts(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.registry_hosts = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.registry_hosts = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `writable_cgroups`.\n"]
    pub fn set_writable_cgroups(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElContainerdConfigElWritableCgroupsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.writable_cgroups = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.writable_cgroups = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeConfigElContainerdConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElContainerdConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElContainerdConfigEl {}
impl BuildContainerNodePoolNodeConfigElContainerdConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElContainerdConfigEl {
        ContainerNodePoolNodeConfigElContainerdConfigEl {
            private_registry_access_config: core::default::Default::default(),
            registry_hosts: core::default::Default::default(),
            writable_cgroups: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElContainerdConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElContainerdConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElContainerdConfigElRef {
        ContainerNodePoolNodeConfigElContainerdConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElContainerdConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `private_registry_access_config` after provisioning.\n"]
    pub fn private_registry_access_config(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_registry_access_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `registry_hosts` after provisioning.\n"]
    pub fn registry_hosts(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElContainerdConfigElRegistryHostsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.registry_hosts", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `writable_cgroups` after provisioning.\n"]
    pub fn writable_cgroups(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElContainerdConfigElWritableCgroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.writable_cgroups", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_cache_count: Option<PrimField<f64>>,
    local_ssd_count: PrimField<f64>,
}
impl ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl {
    #[doc = "Set the field `data_cache_count`.\nNumber of local SSDs to be utilized for GKE Data Cache. Uses NVMe interfaces."]
    pub fn set_data_cache_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.data_cache_count = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl {
    #[doc = "Number of local SSDs to use to back ephemeral storage. Uses NVMe interfaces. Each local SSD must be 375 or 3000 GB in size, and all local SSDs must share the same size."]
    pub local_ssd_count: PrimField<f64>,
}
impl BuildContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl {
        ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl {
            data_cache_count: core::default::Default::default(),
            local_ssd_count: self.local_ssd_count,
        }
    }
}
pub struct ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigElRef {
        ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_cache_count` after provisioning.\nNumber of local SSDs to be utilized for GKE Data Cache. Uses NVMe interfaces."]
    pub fn data_cache_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_cache_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_ssd_count` after provisioning.\nNumber of local SSDs to use to back ephemeral storage. Uses NVMe interfaces. Each local SSD must be 375 or 3000 GB in size, and all local SSDs must share the same size."]
    pub fn local_ssd_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_ssd_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElFastSocketEl {
    enabled: PrimField<bool>,
}
impl ContainerNodePoolNodeConfigElFastSocketEl {}
impl ToListMappable for ContainerNodePoolNodeConfigElFastSocketEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElFastSocketEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElFastSocketEl {
    #[doc = "Whether or not NCCL Fast Socket is enabled"]
    pub enabled: PrimField<bool>,
}
impl BuildContainerNodePoolNodeConfigElFastSocketEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElFastSocketEl {
        ContainerNodePoolNodeConfigElFastSocketEl {
            enabled: self.enabled,
        }
    }
}
pub struct ContainerNodePoolNodeConfigElFastSocketElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElFastSocketElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElFastSocketElRef {
        ContainerNodePoolNodeConfigElFastSocketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElFastSocketElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether or not NCCL Fast Socket is enabled"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElGcfsConfigEl {
    enabled: PrimField<bool>,
}
impl ContainerNodePoolNodeConfigElGcfsConfigEl {}
impl ToListMappable for ContainerNodePoolNodeConfigElGcfsConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElGcfsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElGcfsConfigEl {
    #[doc = "Whether or not GCFS is enabled"]
    pub enabled: PrimField<bool>,
}
impl BuildContainerNodePoolNodeConfigElGcfsConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElGcfsConfigEl {
        ContainerNodePoolNodeConfigElGcfsConfigEl {
            enabled: self.enabled,
        }
    }
}
pub struct ContainerNodePoolNodeConfigElGcfsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElGcfsConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElGcfsConfigElRef {
        ContainerNodePoolNodeConfigElGcfsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElGcfsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether or not GCFS is enabled"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    gpu_driver_version: PrimField<String>,
}
impl ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {}
impl ToListMappable
    for ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl
{
    type O = BlockAssignable<
        ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    #[doc = "Mode for how the GPU driver is installed."]
    pub gpu_driver_version: PrimField<String>,
}
impl BuildContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    pub fn build(
        self,
    ) -> ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
        ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
            gpu_driver_version: self.gpu_driver_version,
        }
    }
}
pub struct ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
        ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gpu_driver_version` after provisioning.\nMode for how the GPU driver is installed."]
    pub fn gpu_driver_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gpu_driver_version", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    gpu_sharing_strategy: PrimField<String>,
    max_shared_clients_per_gpu: PrimField<f64>,
}
impl ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigEl {}
impl ToListMappable for ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    #[doc = "The type of GPU sharing strategy to enable on the GPU node. Possible values are described in the API package (https://pkg.go.dev/google.golang.org/api/container/v1#GPUSharingConfig)"]
    pub gpu_sharing_strategy: PrimField<String>,
    #[doc = "The maximum number of containers that can share a GPU."]
    pub max_shared_clients_per_gpu: PrimField<f64>,
}
impl BuildContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
        ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
            gpu_sharing_strategy: self.gpu_sharing_strategy,
            max_shared_clients_per_gpu: self.max_shared_clients_per_gpu,
        }
    }
}
pub struct ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
        ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gpu_sharing_strategy` after provisioning.\nThe type of GPU sharing strategy to enable on the GPU node. Possible values are described in the API package (https://pkg.go.dev/google.golang.org/api/container/v1#GPUSharingConfig)"]
    pub fn gpu_sharing_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gpu_sharing_strategy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_shared_clients_per_gpu` after provisioning.\nThe maximum number of containers that can share a GPU."]
    pub fn max_shared_clients_per_gpu(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_shared_clients_per_gpu", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ContainerNodePoolNodeConfigElGuestAcceleratorElDynamic {
    gpu_driver_installation_config: Option<
        DynamicBlock<ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl>,
    >,
    gpu_sharing_config:
        Option<DynamicBlock<ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigEl>>,
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElGuestAcceleratorEl {
    count: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_partition_size: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_driver_installation_config:
        Option<Vec<ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_sharing_config:
        Option<Vec<ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigEl>>,
    dynamic: ContainerNodePoolNodeConfigElGuestAcceleratorElDynamic,
}
impl ContainerNodePoolNodeConfigElGuestAcceleratorEl {
    #[doc = "Set the field `gpu_partition_size`.\nSize of partitions to create on the GPU. Valid values are described in the NVIDIA mig user guide (https://docs.nvidia.com/datacenter/tesla/mig-user-guide/#partitioning)"]
    pub fn set_gpu_partition_size(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gpu_partition_size = Some(v.into());
        self
    }
    #[doc = "Set the field `gpu_driver_installation_config`.\n"]
    pub fn set_gpu_driver_installation_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gpu_driver_installation_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gpu_driver_installation_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `gpu_sharing_config`.\n"]
    pub fn set_gpu_sharing_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gpu_sharing_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gpu_sharing_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeConfigElGuestAcceleratorEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElGuestAcceleratorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElGuestAcceleratorEl {
    #[doc = "The number of the accelerator cards exposed to an instance."]
    pub count: PrimField<f64>,
    #[doc = "The accelerator type resource name."]
    pub type_: PrimField<String>,
}
impl BuildContainerNodePoolNodeConfigElGuestAcceleratorEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElGuestAcceleratorEl {
        ContainerNodePoolNodeConfigElGuestAcceleratorEl {
            count: self.count,
            gpu_partition_size: core::default::Default::default(),
            type_: self.type_,
            gpu_driver_installation_config: core::default::Default::default(),
            gpu_sharing_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElGuestAcceleratorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElGuestAcceleratorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElGuestAcceleratorElRef {
        ContainerNodePoolNodeConfigElGuestAcceleratorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElGuestAcceleratorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\nThe number of the accelerator cards exposed to an instance."]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `gpu_partition_size` after provisioning.\nSize of partitions to create on the GPU. Valid values are described in the NVIDIA mig user guide (https://docs.nvidia.com/datacenter/tesla/mig-user-guide/#partitioning)"]
    pub fn gpu_partition_size(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gpu_partition_size", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe accelerator type resource name."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `gpu_driver_installation_config` after provisioning.\n"]
    pub fn gpu_driver_installation_config(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gpu_driver_installation_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gpu_sharing_config` after provisioning.\n"]
    pub fn gpu_sharing_config(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gpu_sharing_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElGvnicEl {
    enabled: PrimField<bool>,
}
impl ContainerNodePoolNodeConfigElGvnicEl {}
impl ToListMappable for ContainerNodePoolNodeConfigElGvnicEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElGvnicEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElGvnicEl {
    #[doc = "Whether or not gvnic is enabled"]
    pub enabled: PrimField<bool>,
}
impl BuildContainerNodePoolNodeConfigElGvnicEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElGvnicEl {
        ContainerNodePoolNodeConfigElGvnicEl {
            enabled: self.enabled,
        }
    }
}
pub struct ContainerNodePoolNodeConfigElGvnicElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElGvnicElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElGvnicElRef {
        ContainerNodePoolNodeConfigElGvnicElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElGvnicElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether or not gvnic is enabled"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElHostMaintenancePolicyEl {
    maintenance_interval: PrimField<String>,
}
impl ContainerNodePoolNodeConfigElHostMaintenancePolicyEl {}
impl ToListMappable for ContainerNodePoolNodeConfigElHostMaintenancePolicyEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElHostMaintenancePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElHostMaintenancePolicyEl {
    #[doc = "."]
    pub maintenance_interval: PrimField<String>,
}
impl BuildContainerNodePoolNodeConfigElHostMaintenancePolicyEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElHostMaintenancePolicyEl {
        ContainerNodePoolNodeConfigElHostMaintenancePolicyEl {
            maintenance_interval: self.maintenance_interval,
        }
    }
}
pub struct ContainerNodePoolNodeConfigElHostMaintenancePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElHostMaintenancePolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElHostMaintenancePolicyElRef {
        ContainerNodePoolNodeConfigElHostMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElHostMaintenancePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `maintenance_interval` after provisioning.\n."]
    pub fn maintenance_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_interval", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElKubeletConfigElEvictionMinimumReclaimEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    imagefs_available: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    imagefs_inodes_free: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_available: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodefs_available: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodefs_inodes_free: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pid_available: Option<PrimField<String>>,
}
impl ContainerNodePoolNodeConfigElKubeletConfigElEvictionMinimumReclaimEl {
    #[doc = "Set the field `imagefs_available`.\nDefines percentage of minimum reclaim for imagefs.available."]
    pub fn set_imagefs_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.imagefs_available = Some(v.into());
        self
    }
    #[doc = "Set the field `imagefs_inodes_free`.\nDefines percentage of minimum reclaim for imagefs.inodesFree."]
    pub fn set_imagefs_inodes_free(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.imagefs_inodes_free = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_available`.\nDefines percentage of minimum reclaim for memory.available."]
    pub fn set_memory_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.memory_available = Some(v.into());
        self
    }
    #[doc = "Set the field `nodefs_available`.\nDefines percentage of minimum reclaim for nodefs.available."]
    pub fn set_nodefs_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nodefs_available = Some(v.into());
        self
    }
    #[doc = "Set the field `nodefs_inodes_free`.\nDefines percentage of minimum reclaim for nodefs.inodesFree."]
    pub fn set_nodefs_inodes_free(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nodefs_inodes_free = Some(v.into());
        self
    }
    #[doc = "Set the field `pid_available`.\nDefines percentage of minimum reclaim for pid.available."]
    pub fn set_pid_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pid_available = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeConfigElKubeletConfigElEvictionMinimumReclaimEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElKubeletConfigElEvictionMinimumReclaimEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElKubeletConfigElEvictionMinimumReclaimEl {}
impl BuildContainerNodePoolNodeConfigElKubeletConfigElEvictionMinimumReclaimEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElKubeletConfigElEvictionMinimumReclaimEl {
        ContainerNodePoolNodeConfigElKubeletConfigElEvictionMinimumReclaimEl {
            imagefs_available: core::default::Default::default(),
            imagefs_inodes_free: core::default::Default::default(),
            memory_available: core::default::Default::default(),
            nodefs_available: core::default::Default::default(),
            nodefs_inodes_free: core::default::Default::default(),
            pid_available: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElKubeletConfigElEvictionMinimumReclaimElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElKubeletConfigElEvictionMinimumReclaimElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElKubeletConfigElEvictionMinimumReclaimElRef {
        ContainerNodePoolNodeConfigElKubeletConfigElEvictionMinimumReclaimElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElKubeletConfigElEvictionMinimumReclaimElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `imagefs_available` after provisioning.\nDefines percentage of minimum reclaim for imagefs.available."]
    pub fn imagefs_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.imagefs_available", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `imagefs_inodes_free` after provisioning.\nDefines percentage of minimum reclaim for imagefs.inodesFree."]
    pub fn imagefs_inodes_free(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.imagefs_inodes_free", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_available` after provisioning.\nDefines percentage of minimum reclaim for memory.available."]
    pub fn memory_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_available", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nodefs_available` after provisioning.\nDefines percentage of minimum reclaim for nodefs.available."]
    pub fn nodefs_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nodefs_available", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nodefs_inodes_free` after provisioning.\nDefines percentage of minimum reclaim for nodefs.inodesFree."]
    pub fn nodefs_inodes_free(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nodefs_inodes_free", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pid_available` after provisioning.\nDefines percentage of minimum reclaim for pid.available."]
    pub fn pid_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pid_available", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    imagefs_available: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    imagefs_inodes_free: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_available: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodefs_available: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodefs_inodes_free: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pid_available: Option<PrimField<String>>,
}
impl ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftEl {
    #[doc = "Set the field `imagefs_available`.\nDefines percentage of soft eviction threshold for imagefs.available."]
    pub fn set_imagefs_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.imagefs_available = Some(v.into());
        self
    }
    #[doc = "Set the field `imagefs_inodes_free`.\nDefines percentage of soft eviction threshold for imagefs.inodesFree."]
    pub fn set_imagefs_inodes_free(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.imagefs_inodes_free = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_available`.\nDefines quantity of soft eviction threshold for memory.available."]
    pub fn set_memory_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.memory_available = Some(v.into());
        self
    }
    #[doc = "Set the field `nodefs_available`.\nDefines percentage of soft eviction threshold for nodefs.available."]
    pub fn set_nodefs_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nodefs_available = Some(v.into());
        self
    }
    #[doc = "Set the field `nodefs_inodes_free`.\nDefines percentage of soft eviction threshold for nodefs.inodesFree."]
    pub fn set_nodefs_inodes_free(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nodefs_inodes_free = Some(v.into());
        self
    }
    #[doc = "Set the field `pid_available`.\nDefines percentage of soft eviction threshold for pid.available."]
    pub fn set_pid_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pid_available = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftEl {}
impl BuildContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftEl {
        ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftEl {
            imagefs_available: core::default::Default::default(),
            imagefs_inodes_free: core::default::Default::default(),
            memory_available: core::default::Default::default(),
            nodefs_available: core::default::Default::default(),
            nodefs_inodes_free: core::default::Default::default(),
            pid_available: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftElRef {
        ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `imagefs_available` after provisioning.\nDefines percentage of soft eviction threshold for imagefs.available."]
    pub fn imagefs_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.imagefs_available", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `imagefs_inodes_free` after provisioning.\nDefines percentage of soft eviction threshold for imagefs.inodesFree."]
    pub fn imagefs_inodes_free(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.imagefs_inodes_free", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_available` after provisioning.\nDefines quantity of soft eviction threshold for memory.available."]
    pub fn memory_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_available", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nodefs_available` after provisioning.\nDefines percentage of soft eviction threshold for nodefs.available."]
    pub fn nodefs_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nodefs_available", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nodefs_inodes_free` after provisioning.\nDefines percentage of soft eviction threshold for nodefs.inodesFree."]
    pub fn nodefs_inodes_free(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nodefs_inodes_free", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pid_available` after provisioning.\nDefines percentage of soft eviction threshold for pid.available."]
    pub fn pid_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pid_available", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    imagefs_available: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    imagefs_inodes_free: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_available: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodefs_available: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodefs_inodes_free: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pid_available: Option<PrimField<String>>,
}
impl ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl {
    #[doc = "Set the field `imagefs_available`.\nDefines grace period for the imagefs.available soft eviction threshold"]
    pub fn set_imagefs_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.imagefs_available = Some(v.into());
        self
    }
    #[doc = "Set the field `imagefs_inodes_free`.\nDefines grace period for the imagefs.inodesFree soft eviction threshold."]
    pub fn set_imagefs_inodes_free(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.imagefs_inodes_free = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_available`.\nDefines grace period for the memory.available soft eviction threshold."]
    pub fn set_memory_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.memory_available = Some(v.into());
        self
    }
    #[doc = "Set the field `nodefs_available`.\nDefines grace period for the nodefs.available soft eviction threshold."]
    pub fn set_nodefs_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nodefs_available = Some(v.into());
        self
    }
    #[doc = "Set the field `nodefs_inodes_free`.\nDefines grace period for the nodefs.inodesFree soft eviction threshold."]
    pub fn set_nodefs_inodes_free(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nodefs_inodes_free = Some(v.into());
        self
    }
    #[doc = "Set the field `pid_available`.\nDefines grace period for the pid.available soft eviction threshold."]
    pub fn set_pid_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pid_available = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl {}
impl BuildContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl {
        ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl {
            imagefs_available: core::default::Default::default(),
            imagefs_inodes_free: core::default::Default::default(),
            memory_available: core::default::Default::default(),
            nodefs_available: core::default::Default::default(),
            nodefs_inodes_free: core::default::Default::default(),
            pid_available: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftGracePeriodElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftGracePeriodElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftGracePeriodElRef {
        ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftGracePeriodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftGracePeriodElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `imagefs_available` after provisioning.\nDefines grace period for the imagefs.available soft eviction threshold"]
    pub fn imagefs_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.imagefs_available", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `imagefs_inodes_free` after provisioning.\nDefines grace period for the imagefs.inodesFree soft eviction threshold."]
    pub fn imagefs_inodes_free(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.imagefs_inodes_free", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_available` after provisioning.\nDefines grace period for the memory.available soft eviction threshold."]
    pub fn memory_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_available", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nodefs_available` after provisioning.\nDefines grace period for the nodefs.available soft eviction threshold."]
    pub fn nodefs_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nodefs_available", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nodefs_inodes_free` after provisioning.\nDefines grace period for the nodefs.inodesFree soft eviction threshold."]
    pub fn nodefs_inodes_free(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nodefs_inodes_free", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pid_available` after provisioning.\nDefines grace period for the pid.available soft eviction threshold."]
    pub fn pid_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pid_available", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElKubeletConfigElMemoryManagerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<PrimField<String>>,
}
impl ContainerNodePoolNodeConfigElKubeletConfigElMemoryManagerEl {
    #[doc = "Set the field `policy`.\nThe Memory Manager policy to use. This policy guides how memory and hugepages are allocated and managed for pods on the node, influencing NUMA affinity."]
    pub fn set_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeConfigElKubeletConfigElMemoryManagerEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElKubeletConfigElMemoryManagerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElKubeletConfigElMemoryManagerEl {}
impl BuildContainerNodePoolNodeConfigElKubeletConfigElMemoryManagerEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElKubeletConfigElMemoryManagerEl {
        ContainerNodePoolNodeConfigElKubeletConfigElMemoryManagerEl {
            policy: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElKubeletConfigElMemoryManagerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElKubeletConfigElMemoryManagerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElKubeletConfigElMemoryManagerElRef {
        ContainerNodePoolNodeConfigElKubeletConfigElMemoryManagerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElKubeletConfigElMemoryManagerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `policy` after provisioning.\nThe Memory Manager policy to use. This policy guides how memory and hugepages are allocated and managed for pods on the node, influencing NUMA affinity."]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElKubeletConfigElTopologyManagerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
}
impl ContainerNodePoolNodeConfigElKubeletConfigElTopologyManagerEl {
    #[doc = "Set the field `policy`.\nThe Topology Manager policy to use. This policy dictates how resource alignment is handled on the node."]
    pub fn set_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy = Some(v.into());
        self
    }
    #[doc = "Set the field `scope`.\nThe Topology Manager scope, defining the granularity at which policy decisions are applied. Valid values are \"container\" (resources are aligned per container within a pod) or \"pod\" (resources are aligned for the entire pod)."]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeConfigElKubeletConfigElTopologyManagerEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElKubeletConfigElTopologyManagerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElKubeletConfigElTopologyManagerEl {}
impl BuildContainerNodePoolNodeConfigElKubeletConfigElTopologyManagerEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElKubeletConfigElTopologyManagerEl {
        ContainerNodePoolNodeConfigElKubeletConfigElTopologyManagerEl {
            policy: core::default::Default::default(),
            scope: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElKubeletConfigElTopologyManagerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElKubeletConfigElTopologyManagerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElKubeletConfigElTopologyManagerElRef {
        ContainerNodePoolNodeConfigElKubeletConfigElTopologyManagerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElKubeletConfigElTopologyManagerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `policy` after provisioning.\nThe Topology Manager policy to use. This policy dictates how resource alignment is handled on the node."]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\nThe Topology Manager scope, defining the granularity at which policy decisions are applied. Valid values are \"container\" (resources are aligned per container within a pod) or \"pod\" (resources are aligned for the entire pod)."]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}
#[derive(Serialize, Default)]
struct ContainerNodePoolNodeConfigElKubeletConfigElDynamic {
    eviction_minimum_reclaim:
        Option<DynamicBlock<ContainerNodePoolNodeConfigElKubeletConfigElEvictionMinimumReclaimEl>>,
    eviction_soft: Option<DynamicBlock<ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftEl>>,
    eviction_soft_grace_period:
        Option<DynamicBlock<ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl>>,
    memory_manager:
        Option<DynamicBlock<ContainerNodePoolNodeConfigElKubeletConfigElMemoryManagerEl>>,
    topology_manager:
        Option<DynamicBlock<ContainerNodePoolNodeConfigElKubeletConfigElTopologyManagerEl>>,
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElKubeletConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_unsafe_sysctls: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_log_max_files: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_log_max_size: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_cfs_quota: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_cfs_quota_period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_manager_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eviction_max_pod_grace_period_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_gc_high_threshold_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_gc_low_threshold_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_maximum_gc_age: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_minimum_gc_age: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insecure_kubelet_readonly_port_enabled: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_parallel_image_pulls: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_pids_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    single_process_oom_kill: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eviction_minimum_reclaim:
        Option<Vec<ContainerNodePoolNodeConfigElKubeletConfigElEvictionMinimumReclaimEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eviction_soft: Option<Vec<ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eviction_soft_grace_period:
        Option<Vec<ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_manager: Option<Vec<ContainerNodePoolNodeConfigElKubeletConfigElMemoryManagerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topology_manager: Option<Vec<ContainerNodePoolNodeConfigElKubeletConfigElTopologyManagerEl>>,
    dynamic: ContainerNodePoolNodeConfigElKubeletConfigElDynamic,
}
impl ContainerNodePoolNodeConfigElKubeletConfigEl {
    #[doc = "Set the field `allowed_unsafe_sysctls`.\nDefines a comma-separated allowlist of unsafe sysctls or sysctl patterns which can be set on the Pods."]
    pub fn set_allowed_unsafe_sysctls(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.allowed_unsafe_sysctls = Some(v.into());
        self
    }
    #[doc = "Set the field `container_log_max_files`.\nDefines the maximum number of container log files that can be present for a container."]
    pub fn set_container_log_max_files(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.container_log_max_files = Some(v.into());
        self
    }
    #[doc = "Set the field `container_log_max_size`.\nDefines the maximum size of the container log file before it is rotated."]
    pub fn set_container_log_max_size(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.container_log_max_size = Some(v.into());
        self
    }
    #[doc = "Set the field `cpu_cfs_quota`.\nEnable CPU CFS quota enforcement for containers that specify CPU limits."]
    pub fn set_cpu_cfs_quota(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.cpu_cfs_quota = Some(v.into());
        self
    }
    #[doc = "Set the field `cpu_cfs_quota_period`.\nSet the CPU CFS quota period value 'cpu.cfs_period_us'."]
    pub fn set_cpu_cfs_quota_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cpu_cfs_quota_period = Some(v.into());
        self
    }
    #[doc = "Set the field `cpu_manager_policy`.\nControl the CPU management policy on the node."]
    pub fn set_cpu_manager_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cpu_manager_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `eviction_max_pod_grace_period_seconds`.\nDefines the maximum allowed grace period (in seconds) to use when terminating pods in response to a soft eviction threshold being met."]
    pub fn set_eviction_max_pod_grace_period_seconds(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.eviction_max_pod_grace_period_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `image_gc_high_threshold_percent`.\nDefines the percent of disk usage after which image garbage collection is always run."]
    pub fn set_image_gc_high_threshold_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.image_gc_high_threshold_percent = Some(v.into());
        self
    }
    #[doc = "Set the field `image_gc_low_threshold_percent`.\nDefines the percent of disk usage before which image garbage collection is never run. Lowest disk usage to garbage collect to."]
    pub fn set_image_gc_low_threshold_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.image_gc_low_threshold_percent = Some(v.into());
        self
    }
    #[doc = "Set the field `image_maximum_gc_age`.\nDefines the maximum age an image can be unused before it is garbage collected."]
    pub fn set_image_maximum_gc_age(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_maximum_gc_age = Some(v.into());
        self
    }
    #[doc = "Set the field `image_minimum_gc_age`.\nDefines the minimum age for an unused image before it is garbage collected."]
    pub fn set_image_minimum_gc_age(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_minimum_gc_age = Some(v.into());
        self
    }
    #[doc = "Set the field `insecure_kubelet_readonly_port_enabled`.\nControls whether the kubelet read-only port is enabled. It is strongly recommended to set this to `FALSE`. Possible values: `TRUE`, `FALSE`."]
    pub fn set_insecure_kubelet_readonly_port_enabled(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.insecure_kubelet_readonly_port_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `max_parallel_image_pulls`.\nSet the maximum number of image pulls in parallel."]
    pub fn set_max_parallel_image_pulls(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_parallel_image_pulls = Some(v.into());
        self
    }
    #[doc = "Set the field `pod_pids_limit`.\nControls the maximum number of processes allowed to run in a pod."]
    pub fn set_pod_pids_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.pod_pids_limit = Some(v.into());
        self
    }
    #[doc = "Set the field `single_process_oom_kill`.\nDefines whether to enable single process OOM killer."]
    pub fn set_single_process_oom_kill(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.single_process_oom_kill = Some(v.into());
        self
    }
    #[doc = "Set the field `eviction_minimum_reclaim`.\n"]
    pub fn set_eviction_minimum_reclaim(
        mut self,
        v: impl Into<
            BlockAssignable<ContainerNodePoolNodeConfigElKubeletConfigElEvictionMinimumReclaimEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.eviction_minimum_reclaim = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.eviction_minimum_reclaim = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `eviction_soft`.\n"]
    pub fn set_eviction_soft(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.eviction_soft = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.eviction_soft = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `eviction_soft_grace_period`.\n"]
    pub fn set_eviction_soft_grace_period(
        mut self,
        v: impl Into<
            BlockAssignable<ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.eviction_soft_grace_period = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.eviction_soft_grace_period = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `memory_manager`.\n"]
    pub fn set_memory_manager(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElKubeletConfigElMemoryManagerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.memory_manager = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.memory_manager = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `topology_manager`.\n"]
    pub fn set_topology_manager(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElKubeletConfigElTopologyManagerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.topology_manager = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.topology_manager = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeConfigElKubeletConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElKubeletConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElKubeletConfigEl {}
impl BuildContainerNodePoolNodeConfigElKubeletConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElKubeletConfigEl {
        ContainerNodePoolNodeConfigElKubeletConfigEl {
            allowed_unsafe_sysctls: core::default::Default::default(),
            container_log_max_files: core::default::Default::default(),
            container_log_max_size: core::default::Default::default(),
            cpu_cfs_quota: core::default::Default::default(),
            cpu_cfs_quota_period: core::default::Default::default(),
            cpu_manager_policy: core::default::Default::default(),
            eviction_max_pod_grace_period_seconds: core::default::Default::default(),
            image_gc_high_threshold_percent: core::default::Default::default(),
            image_gc_low_threshold_percent: core::default::Default::default(),
            image_maximum_gc_age: core::default::Default::default(),
            image_minimum_gc_age: core::default::Default::default(),
            insecure_kubelet_readonly_port_enabled: core::default::Default::default(),
            max_parallel_image_pulls: core::default::Default::default(),
            pod_pids_limit: core::default::Default::default(),
            single_process_oom_kill: core::default::Default::default(),
            eviction_minimum_reclaim: core::default::Default::default(),
            eviction_soft: core::default::Default::default(),
            eviction_soft_grace_period: core::default::Default::default(),
            memory_manager: core::default::Default::default(),
            topology_manager: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElKubeletConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElKubeletConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElKubeletConfigElRef {
        ContainerNodePoolNodeConfigElKubeletConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElKubeletConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_unsafe_sysctls` after provisioning.\nDefines a comma-separated allowlist of unsafe sysctls or sysctl patterns which can be set on the Pods."]
    pub fn allowed_unsafe_sysctls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_unsafe_sysctls", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `container_log_max_files` after provisioning.\nDefines the maximum number of container log files that can be present for a container."]
    pub fn container_log_max_files(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_log_max_files", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `container_log_max_size` after provisioning.\nDefines the maximum size of the container log file before it is rotated."]
    pub fn container_log_max_size(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_log_max_size", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cpu_cfs_quota` after provisioning.\nEnable CPU CFS quota enforcement for containers that specify CPU limits."]
    pub fn cpu_cfs_quota(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_cfs_quota", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cpu_cfs_quota_period` after provisioning.\nSet the CPU CFS quota period value 'cpu.cfs_period_us'."]
    pub fn cpu_cfs_quota_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_cfs_quota_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cpu_manager_policy` after provisioning.\nControl the CPU management policy on the node."]
    pub fn cpu_manager_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_manager_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `eviction_max_pod_grace_period_seconds` after provisioning.\nDefines the maximum allowed grace period (in seconds) to use when terminating pods in response to a soft eviction threshold being met."]
    pub fn eviction_max_pod_grace_period_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.eviction_max_pod_grace_period_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_gc_high_threshold_percent` after provisioning.\nDefines the percent of disk usage after which image garbage collection is always run."]
    pub fn image_gc_high_threshold_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_gc_high_threshold_percent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_gc_low_threshold_percent` after provisioning.\nDefines the percent of disk usage before which image garbage collection is never run. Lowest disk usage to garbage collect to."]
    pub fn image_gc_low_threshold_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_gc_low_threshold_percent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_maximum_gc_age` after provisioning.\nDefines the maximum age an image can be unused before it is garbage collected."]
    pub fn image_maximum_gc_age(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_maximum_gc_age", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_minimum_gc_age` after provisioning.\nDefines the minimum age for an unused image before it is garbage collected."]
    pub fn image_minimum_gc_age(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_minimum_gc_age", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `insecure_kubelet_readonly_port_enabled` after provisioning.\nControls whether the kubelet read-only port is enabled. It is strongly recommended to set this to `FALSE`. Possible values: `TRUE`, `FALSE`."]
    pub fn insecure_kubelet_readonly_port_enabled(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.insecure_kubelet_readonly_port_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_parallel_image_pulls` after provisioning.\nSet the maximum number of image pulls in parallel."]
    pub fn max_parallel_image_pulls(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_parallel_image_pulls", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pod_pids_limit` after provisioning.\nControls the maximum number of processes allowed to run in a pod."]
    pub fn pod_pids_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pod_pids_limit", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `single_process_oom_kill` after provisioning.\nDefines whether to enable single process OOM killer."]
    pub fn single_process_oom_kill(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.single_process_oom_kill", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `eviction_minimum_reclaim` after provisioning.\n"]
    pub fn eviction_minimum_reclaim(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElKubeletConfigElEvictionMinimumReclaimElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.eviction_minimum_reclaim", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `eviction_soft` after provisioning.\n"]
    pub fn eviction_soft(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.eviction_soft", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `eviction_soft_grace_period` after provisioning.\n"]
    pub fn eviction_soft_grace_period(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElKubeletConfigElEvictionSoftGracePeriodElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.eviction_soft_grace_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_manager` after provisioning.\n"]
    pub fn memory_manager(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElKubeletConfigElMemoryManagerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.memory_manager", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `topology_manager` after provisioning.\n"]
    pub fn topology_manager(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElKubeletConfigElTopologyManagerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.topology_manager", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_ptp_kvm_time_sync: Option<PrimField<bool>>,
}
impl ContainerNodePoolNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl {
    #[doc = "Set the field `enable_ptp_kvm_time_sync`.\nWhether to enable accurate time synchronization with PTP-KVM."]
    pub fn set_enable_ptp_kvm_time_sync(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_ptp_kvm_time_sync = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl {}
impl BuildContainerNodePoolNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl {
        ContainerNodePoolNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl {
            enable_ptp_kvm_time_sync: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElLinuxNodeConfigElAccurateTimeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElLinuxNodeConfigElAccurateTimeConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElLinuxNodeConfigElAccurateTimeConfigElRef {
        ContainerNodePoolNodeConfigElLinuxNodeConfigElAccurateTimeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElLinuxNodeConfigElAccurateTimeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_ptp_kvm_time_sync` after provisioning.\nWhether to enable accurate time synchronization with PTP-KVM."]
    pub fn enable_ptp_kvm_time_sync(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_ptp_kvm_time_sync", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElLinuxNodeConfigElHugepagesConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hugepage_size_1g: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hugepage_size_2m: Option<PrimField<f64>>,
}
impl ContainerNodePoolNodeConfigElLinuxNodeConfigElHugepagesConfigEl {
    #[doc = "Set the field `hugepage_size_1g`.\nAmount of 1G hugepages."]
    pub fn set_hugepage_size_1g(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hugepage_size_1g = Some(v.into());
        self
    }
    #[doc = "Set the field `hugepage_size_2m`.\nAmount of 2M hugepages."]
    pub fn set_hugepage_size_2m(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hugepage_size_2m = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeConfigElLinuxNodeConfigElHugepagesConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElLinuxNodeConfigElHugepagesConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElLinuxNodeConfigElHugepagesConfigEl {}
impl BuildContainerNodePoolNodeConfigElLinuxNodeConfigElHugepagesConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElLinuxNodeConfigElHugepagesConfigEl {
        ContainerNodePoolNodeConfigElLinuxNodeConfigElHugepagesConfigEl {
            hugepage_size_1g: core::default::Default::default(),
            hugepage_size_2m: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElLinuxNodeConfigElHugepagesConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElLinuxNodeConfigElHugepagesConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElLinuxNodeConfigElHugepagesConfigElRef {
        ContainerNodePoolNodeConfigElLinuxNodeConfigElHugepagesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElLinuxNodeConfigElHugepagesConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hugepage_size_1g` after provisioning.\nAmount of 1G hugepages."]
    pub fn hugepage_size_1g(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hugepage_size_1g", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hugepage_size_2m` after provisioning.\nAmount of 2M hugepages."]
    pub fn hugepage_size_2m(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hugepage_size_2m", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<PrimField<String>>,
}
impl ContainerNodePoolNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl {
    #[doc = "Set the field `policy`.\nThe policy for kernel module loading."]
    pub fn set_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl {
    type O =
        BlockAssignable<ContainerNodePoolNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl {}
impl BuildContainerNodePoolNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl {
        ContainerNodePoolNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl {
            policy: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingElRef {
        ContainerNodePoolNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `policy` after provisioning.\nThe policy for kernel module loading."]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    swap_size_gib: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    swap_size_percent: Option<PrimField<f64>>,
}
impl ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl {
    #[doc = "Set the field `swap_size_gib`.\nSpecifies the size of the swap space in gibibytes (GiB)."]
    pub fn set_swap_size_gib(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.swap_size_gib = Some(v.into());
        self
    }
    #[doc = "Set the field `swap_size_percent`.\nSpecifies the size of the swap space as a percentage of the boot disk size."]
    pub fn set_swap_size_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.swap_size_percent = Some(v.into());
        self
    }
}
impl ToListMappable
    for ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl
{
    type O = BlockAssignable<
        ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl {}
impl BuildContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl {
    pub fn build(
        self,
    ) -> ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl {
        ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl {
            swap_size_gib: core::default::Default::default(),
            swap_size_percent: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileElRef {
        ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `swap_size_gib` after provisioning.\nSpecifies the size of the swap space in gibibytes (GiB)."]
    pub fn swap_size_gib(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.swap_size_gib", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `swap_size_percent` after provisioning.\nSpecifies the size of the swap space as a percentage of the boot disk size."]
    pub fn swap_size_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.swap_size_percent", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_count: Option<PrimField<f64>>,
}
impl ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl {
    #[doc = "Set the field `disk_count`.\nThe number of physical local NVMe SSD disks to attach."]
    pub fn set_disk_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_count = Some(v.into());
        self
    }
}
impl ToListMappable
    for ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl
{
    type O = BlockAssignable<
        ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl
{}
impl BuildContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl {
    pub fn build(
        self,
    ) -> ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl {
        ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl {
            disk_count: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileElRef
    {
        ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk_count` after provisioning.\nThe number of physical local NVMe SSD disks to attach."]
    pub fn disk_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_count", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
}
impl ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl {
    #[doc = "Set the field `disabled`.\nIf true, swap space will not be encrypted. Defaults to false (encrypted)."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}
impl ToListMappable
    for ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl
{
    type O = BlockAssignable<
        ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl {}
impl BuildContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl {
    pub fn build(
        self,
    ) -> ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl {
        ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl {
            disabled: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigElRef {
        ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nIf true, swap space will not be encrypted. Defaults to false (encrypted)."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    swap_size_gib: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    swap_size_percent: Option<PrimField<f64>>,
}
impl ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl {
    #[doc = "Set the field `swap_size_gib`.\nSpecifies the size of the swap space in gibibytes (GiB)."]
    pub fn set_swap_size_gib(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.swap_size_gib = Some(v.into());
        self
    }
    #[doc = "Set the field `swap_size_percent`.\nSpecifies the size of the swap space as a percentage of the ephemeral local SSD capacity."]
    pub fn set_swap_size_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.swap_size_percent = Some(v.into());
        self
    }
}
impl ToListMappable
    for ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl
{
    type O = BlockAssignable<
        ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl
{}
impl BuildContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl {
    pub fn build(
        self,
    ) -> ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl {
        ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl {
            swap_size_gib: core::default::Default::default(),
            swap_size_percent: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileElRef
    {
        ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `swap_size_gib` after provisioning.\nSpecifies the size of the swap space in gibibytes (GiB)."]
    pub fn swap_size_gib(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.swap_size_gib", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `swap_size_percent` after provisioning.\nSpecifies the size of the swap space as a percentage of the ephemeral local SSD capacity."]
    pub fn swap_size_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.swap_size_percent", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElDynamic {
    boot_disk_profile: Option<
        DynamicBlock<ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl>,
    >,
    dedicated_local_ssd_profile: Option<
        DynamicBlock<
            ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl,
        >,
    >,
    encryption_config: Option<
        DynamicBlock<ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl>,
    >,
    ephemeral_local_ssd_profile: Option<
        DynamicBlock<
            ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_profile:
        Option<Vec<ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dedicated_local_ssd_profile: Option<
        Vec<ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_config:
        Option<Vec<ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ephemeral_local_ssd_profile: Option<
        Vec<ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl>,
    >,
    dynamic: ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElDynamic,
}
impl ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigEl {
    #[doc = "Set the field `enabled`.\nEnables or disables swap for the node pool."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `boot_disk_profile`.\n"]
    pub fn set_boot_disk_profile(
        mut self,
        v: impl Into<
            BlockAssignable<
                ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.boot_disk_profile = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.boot_disk_profile = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dedicated_local_ssd_profile`.\n"]
    pub fn set_dedicated_local_ssd_profile(
        mut self,
        v : impl Into < BlockAssignable < ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dedicated_local_ssd_profile = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dedicated_local_ssd_profile = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.encryption_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ephemeral_local_ssd_profile`.\n"]
    pub fn set_ephemeral_local_ssd_profile(
        mut self,
        v : impl Into < BlockAssignable < ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ephemeral_local_ssd_profile = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ephemeral_local_ssd_profile = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigEl {}
impl BuildContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigEl {
        ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigEl {
            enabled: core::default::Default::default(),
            boot_disk_profile: core::default::Default::default(),
            dedicated_local_ssd_profile: core::default::Default::default(),
            encryption_config: core::default::Default::default(),
            ephemeral_local_ssd_profile: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElRef {
        ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nEnables or disables swap for the node pool."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `boot_disk_profile` after provisioning.\n"]
    pub fn boot_disk_profile(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.boot_disk_profile", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dedicated_local_ssd_profile` after provisioning.\n"]
    pub fn dedicated_local_ssd_profile(
        &self,
    ) -> ListRef<
        ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dedicated_local_ssd_profile", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ephemeral_local_ssd_profile` after provisioning.\n"]
    pub fn ephemeral_local_ssd_profile(
        &self,
    ) -> ListRef<
        ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ephemeral_local_ssd_profile", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ContainerNodePoolNodeConfigElLinuxNodeConfigElDynamic {
    accurate_time_config:
        Option<DynamicBlock<ContainerNodePoolNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl>>,
    hugepages_config:
        Option<DynamicBlock<ContainerNodePoolNodeConfigElLinuxNodeConfigElHugepagesConfigEl>>,
    node_kernel_module_loading: Option<
        DynamicBlock<ContainerNodePoolNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl>,
    >,
    swap_config: Option<DynamicBlock<ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigEl>>,
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElLinuxNodeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cgroup_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sysctls: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transparent_hugepage_defrag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transparent_hugepage_enabled: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accurate_time_config:
        Option<Vec<ContainerNodePoolNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hugepages_config: Option<Vec<ContainerNodePoolNodeConfigElLinuxNodeConfigElHugepagesConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_kernel_module_loading:
        Option<Vec<ContainerNodePoolNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    swap_config: Option<Vec<ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigEl>>,
    dynamic: ContainerNodePoolNodeConfigElLinuxNodeConfigElDynamic,
}
impl ContainerNodePoolNodeConfigElLinuxNodeConfigEl {
    #[doc = "Set the field `cgroup_mode`.\ncgroupMode specifies the cgroup mode to be used on the node."]
    pub fn set_cgroup_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cgroup_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `sysctls`.\nThe Linux kernel parameters to be applied to the nodes and all pods running on the nodes."]
    pub fn set_sysctls(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.sysctls = Some(v.into());
        self
    }
    #[doc = "Set the field `transparent_hugepage_defrag`.\nThe Linux kernel transparent hugepage defrag setting."]
    pub fn set_transparent_hugepage_defrag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.transparent_hugepage_defrag = Some(v.into());
        self
    }
    #[doc = "Set the field `transparent_hugepage_enabled`.\nThe Linux kernel transparent hugepage setting."]
    pub fn set_transparent_hugepage_enabled(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.transparent_hugepage_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `accurate_time_config`.\n"]
    pub fn set_accurate_time_config(
        mut self,
        v: impl Into<
            BlockAssignable<ContainerNodePoolNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.accurate_time_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.accurate_time_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `hugepages_config`.\n"]
    pub fn set_hugepages_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElLinuxNodeConfigElHugepagesConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hugepages_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hugepages_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `node_kernel_module_loading`.\n"]
    pub fn set_node_kernel_module_loading(
        mut self,
        v: impl Into<
            BlockAssignable<
                ContainerNodePoolNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.node_kernel_module_loading = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.node_kernel_module_loading = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `swap_config`.\n"]
    pub fn set_swap_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.swap_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.swap_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeConfigElLinuxNodeConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElLinuxNodeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElLinuxNodeConfigEl {}
impl BuildContainerNodePoolNodeConfigElLinuxNodeConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElLinuxNodeConfigEl {
        ContainerNodePoolNodeConfigElLinuxNodeConfigEl {
            cgroup_mode: core::default::Default::default(),
            sysctls: core::default::Default::default(),
            transparent_hugepage_defrag: core::default::Default::default(),
            transparent_hugepage_enabled: core::default::Default::default(),
            accurate_time_config: core::default::Default::default(),
            hugepages_config: core::default::Default::default(),
            node_kernel_module_loading: core::default::Default::default(),
            swap_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElLinuxNodeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElLinuxNodeConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElLinuxNodeConfigElRef {
        ContainerNodePoolNodeConfigElLinuxNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElLinuxNodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cgroup_mode` after provisioning.\ncgroupMode specifies the cgroup mode to be used on the node."]
    pub fn cgroup_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cgroup_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `sysctls` after provisioning.\nThe Linux kernel parameters to be applied to the nodes and all pods running on the nodes."]
    pub fn sysctls(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.sysctls", self.base))
    }
    #[doc = "Get a reference to the value of field `transparent_hugepage_defrag` after provisioning.\nThe Linux kernel transparent hugepage defrag setting."]
    pub fn transparent_hugepage_defrag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transparent_hugepage_defrag", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `transparent_hugepage_enabled` after provisioning.\nThe Linux kernel transparent hugepage setting."]
    pub fn transparent_hugepage_enabled(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transparent_hugepage_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `accurate_time_config` after provisioning.\n"]
    pub fn accurate_time_config(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElLinuxNodeConfigElAccurateTimeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.accurate_time_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hugepages_config` after provisioning.\n"]
    pub fn hugepages_config(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElLinuxNodeConfigElHugepagesConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hugepages_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_kernel_module_loading` after provisioning.\n"]
    pub fn node_kernel_module_loading(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_kernel_module_loading", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `swap_config` after provisioning.\n"]
    pub fn swap_config(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElLinuxNodeConfigElSwapConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.swap_config", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl {
    local_ssd_count: PrimField<f64>,
}
impl ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl {}
impl ToListMappable for ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl {
    #[doc = "Number of raw-block local NVMe SSD disks to be attached to the node. Each local SSD is 375 GB in size."]
    pub local_ssd_count: PrimField<f64>,
}
impl BuildContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl {
        ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl {
            local_ssd_count: self.local_ssd_count,
        }
    }
}
pub struct ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigElRef {
        ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `local_ssd_count` after provisioning.\nNumber of raw-block local NVMe SSD disks to be attached to the node. Each local SSD is 375 GB in size."]
    pub fn local_ssd_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_ssd_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElReservationAffinityEl {
    consume_reservation_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}
impl ContainerNodePoolNodeConfigElReservationAffinityEl {
    #[doc = "Set the field `key`.\nThe label key of a reservation resource."]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `values`.\nThe label values of the reservation resource."]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeConfigElReservationAffinityEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElReservationAffinityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElReservationAffinityEl {
    #[doc = "Corresponds to the type of reservation consumption."]
    pub consume_reservation_type: PrimField<String>,
}
impl BuildContainerNodePoolNodeConfigElReservationAffinityEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElReservationAffinityEl {
        ContainerNodePoolNodeConfigElReservationAffinityEl {
            consume_reservation_type: self.consume_reservation_type,
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElReservationAffinityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElReservationAffinityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElReservationAffinityElRef {
        ContainerNodePoolNodeConfigElReservationAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElReservationAffinityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `consume_reservation_type` after provisioning.\nCorresponds to the type of reservation consumption."]
    pub fn consume_reservation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consume_reservation_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nThe label key of a reservation resource."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\nThe label values of the reservation resource."]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElSandboxConfigEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl ContainerNodePoolNodeConfigElSandboxConfigEl {}
impl ToListMappable for ContainerNodePoolNodeConfigElSandboxConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElSandboxConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElSandboxConfigEl {
    #[doc = "Type of the sandbox to use for the node (e.g. 'GVISOR')."]
    pub type_: PrimField<String>,
}
impl BuildContainerNodePoolNodeConfigElSandboxConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElSandboxConfigEl {
        ContainerNodePoolNodeConfigElSandboxConfigEl { type_: self.type_ }
    }
}
pub struct ContainerNodePoolNodeConfigElSandboxConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElSandboxConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElSandboxConfigElRef {
        ContainerNodePoolNodeConfigElSandboxConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElSandboxConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of the sandbox to use for the node (e.g. 'GVISOR')."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElSecondaryBootDisksEl {
    disk_image: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}
impl ContainerNodePoolNodeConfigElSecondaryBootDisksEl {
    #[doc = "Set the field `mode`.\nMode for how the secondary boot disk is used."]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeConfigElSecondaryBootDisksEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElSecondaryBootDisksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElSecondaryBootDisksEl {
    #[doc = "Disk image to create the secondary boot disk from"]
    pub disk_image: PrimField<String>,
}
impl BuildContainerNodePoolNodeConfigElSecondaryBootDisksEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElSecondaryBootDisksEl {
        ContainerNodePoolNodeConfigElSecondaryBootDisksEl {
            disk_image: self.disk_image,
            mode: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElSecondaryBootDisksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElSecondaryBootDisksElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElSecondaryBootDisksElRef {
        ContainerNodePoolNodeConfigElSecondaryBootDisksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElSecondaryBootDisksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk_image` after provisioning.\nDisk image to create the secondary boot disk from"]
    pub fn disk_image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_image", self.base))
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nMode for how the secondary boot disk is used."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElShieldedInstanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_integrity_monitoring: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_secure_boot: Option<PrimField<bool>>,
}
impl ContainerNodePoolNodeConfigElShieldedInstanceConfigEl {
    #[doc = "Set the field `enable_integrity_monitoring`.\nDefines whether the instance has integrity monitoring enabled."]
    pub fn set_enable_integrity_monitoring(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_secure_boot`.\nDefines whether the instance has Secure Boot enabled."]
    pub fn set_enable_secure_boot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_secure_boot = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeConfigElShieldedInstanceConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElShieldedInstanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElShieldedInstanceConfigEl {}
impl BuildContainerNodePoolNodeConfigElShieldedInstanceConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElShieldedInstanceConfigEl {
        ContainerNodePoolNodeConfigElShieldedInstanceConfigEl {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElShieldedInstanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElShieldedInstanceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElShieldedInstanceConfigElRef {
        ContainerNodePoolNodeConfigElShieldedInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElShieldedInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\nDefines whether the instance has integrity monitoring enabled."]
    pub fn enable_integrity_monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_integrity_monitoring", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_secure_boot` after provisioning.\nDefines whether the instance has Secure Boot enabled."]
    pub fn enable_secure_boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_secure_boot", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl {
    key: PrimField<String>,
    operator: PrimField<String>,
    values: ListField<PrimField<String>>,
}
impl ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl {}
impl ToListMappable for ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl {
    #[doc = "."]
    pub key: PrimField<String>,
    #[doc = "."]
    pub operator: PrimField<String>,
    #[doc = "."]
    pub values: ListField<PrimField<String>>,
}
impl BuildContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl {
        ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl {
            key: self.key,
            operator: self.operator,
            values: self.values,
        }
    }
}
pub struct ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityElRef {
        ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\n."]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n."]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize, Default)]
struct ContainerNodePoolNodeConfigElSoleTenantConfigElDynamic {
    node_affinity:
        Option<DynamicBlock<ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl>>,
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElSoleTenantConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    min_node_cpus: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_affinity: Option<Vec<ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl>>,
    dynamic: ContainerNodePoolNodeConfigElSoleTenantConfigElDynamic,
}
impl ContainerNodePoolNodeConfigElSoleTenantConfigEl {
    #[doc = "Set the field `min_node_cpus`.\nSpecifies the minimum number of vCPUs that each sole tenant node must have to use CPU overcommit. If not specified, the CPU overcommit feature is disabled."]
    pub fn set_min_node_cpus(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_node_cpus = Some(v.into());
        self
    }
    #[doc = "Set the field `node_affinity`.\n"]
    pub fn set_node_affinity(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.node_affinity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.node_affinity = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeConfigElSoleTenantConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElSoleTenantConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElSoleTenantConfigEl {}
impl BuildContainerNodePoolNodeConfigElSoleTenantConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElSoleTenantConfigEl {
        ContainerNodePoolNodeConfigElSoleTenantConfigEl {
            min_node_cpus: core::default::Default::default(),
            node_affinity: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElSoleTenantConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElSoleTenantConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElSoleTenantConfigElRef {
        ContainerNodePoolNodeConfigElSoleTenantConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElSoleTenantConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `min_node_cpus` after provisioning.\nSpecifies the minimum number of vCPUs that each sole tenant node must have to use CPU overcommit. If not specified, the CPU overcommit feature is disabled."]
    pub fn min_node_cpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_node_cpus", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElTaintEl {
    effect: PrimField<String>,
    key: PrimField<String>,
    value: PrimField<String>,
}
impl ContainerNodePoolNodeConfigElTaintEl {}
impl ToListMappable for ContainerNodePoolNodeConfigElTaintEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElTaintEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElTaintEl {
    #[doc = "Effect for taint."]
    pub effect: PrimField<String>,
    #[doc = "Key for taint."]
    pub key: PrimField<String>,
    #[doc = "Value for taint."]
    pub value: PrimField<String>,
}
impl BuildContainerNodePoolNodeConfigElTaintEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElTaintEl {
        ContainerNodePoolNodeConfigElTaintEl {
            effect: self.effect,
            key: self.key,
            value: self.value,
        }
    }
}
pub struct ContainerNodePoolNodeConfigElTaintElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElTaintElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElTaintElRef {
        ContainerNodePoolNodeConfigElTaintElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElTaintElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `effect` after provisioning.\nEffect for taint."]
    pub fn effect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nKey for taint."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue for taint."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElWindowsNodeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    osversion: Option<PrimField<String>>,
}
impl ContainerNodePoolNodeConfigElWindowsNodeConfigEl {
    #[doc = "Set the field `osversion`.\nThe OS Version of the windows nodepool.Values are OS_VERSION_UNSPECIFIED,OS_VERSION_LTSC2019 and OS_VERSION_LTSC2022"]
    pub fn set_osversion(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.osversion = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeConfigElWindowsNodeConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElWindowsNodeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElWindowsNodeConfigEl {}
impl BuildContainerNodePoolNodeConfigElWindowsNodeConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElWindowsNodeConfigEl {
        ContainerNodePoolNodeConfigElWindowsNodeConfigEl {
            osversion: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElWindowsNodeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElWindowsNodeConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElWindowsNodeConfigElRef {
        ContainerNodePoolNodeConfigElWindowsNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElWindowsNodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `osversion` after provisioning.\nThe OS Version of the windows nodepool.Values are OS_VERSION_UNSPECIFIED,OS_VERSION_LTSC2019 and OS_VERSION_LTSC2022"]
    pub fn osversion(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.osversion", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElWorkloadMetadataConfigEl {
    mode: PrimField<String>,
}
impl ContainerNodePoolNodeConfigElWorkloadMetadataConfigEl {}
impl ToListMappable for ContainerNodePoolNodeConfigElWorkloadMetadataConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElWorkloadMetadataConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigElWorkloadMetadataConfigEl {
    #[doc = "Mode is the configuration for how to expose metadata to workloads running on the node."]
    pub mode: PrimField<String>,
}
impl BuildContainerNodePoolNodeConfigElWorkloadMetadataConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElWorkloadMetadataConfigEl {
        ContainerNodePoolNodeConfigElWorkloadMetadataConfigEl { mode: self.mode }
    }
}
pub struct ContainerNodePoolNodeConfigElWorkloadMetadataConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElWorkloadMetadataConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElWorkloadMetadataConfigElRef {
        ContainerNodePoolNodeConfigElWorkloadMetadataConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElWorkloadMetadataConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nMode is the configuration for how to expose metadata to workloads running on the node."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}
#[derive(Serialize, Default)]
struct ContainerNodePoolNodeConfigElDynamic {
    advanced_machine_features:
        Option<DynamicBlock<ContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl>>,
    boot_disk: Option<DynamicBlock<ContainerNodePoolNodeConfigElBootDiskEl>>,
    confidential_nodes: Option<DynamicBlock<ContainerNodePoolNodeConfigElConfidentialNodesEl>>,
    containerd_config: Option<DynamicBlock<ContainerNodePoolNodeConfigElContainerdConfigEl>>,
    ephemeral_storage_local_ssd_config:
        Option<DynamicBlock<ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl>>,
    fast_socket: Option<DynamicBlock<ContainerNodePoolNodeConfigElFastSocketEl>>,
    gcfs_config: Option<DynamicBlock<ContainerNodePoolNodeConfigElGcfsConfigEl>>,
    guest_accelerator: Option<DynamicBlock<ContainerNodePoolNodeConfigElGuestAcceleratorEl>>,
    gvnic: Option<DynamicBlock<ContainerNodePoolNodeConfigElGvnicEl>>,
    host_maintenance_policy:
        Option<DynamicBlock<ContainerNodePoolNodeConfigElHostMaintenancePolicyEl>>,
    kubelet_config: Option<DynamicBlock<ContainerNodePoolNodeConfigElKubeletConfigEl>>,
    linux_node_config: Option<DynamicBlock<ContainerNodePoolNodeConfigElLinuxNodeConfigEl>>,
    local_nvme_ssd_block_config:
        Option<DynamicBlock<ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl>>,
    reservation_affinity: Option<DynamicBlock<ContainerNodePoolNodeConfigElReservationAffinityEl>>,
    sandbox_config: Option<DynamicBlock<ContainerNodePoolNodeConfigElSandboxConfigEl>>,
    secondary_boot_disks: Option<DynamicBlock<ContainerNodePoolNodeConfigElSecondaryBootDisksEl>>,
    shielded_instance_config:
        Option<DynamicBlock<ContainerNodePoolNodeConfigElShieldedInstanceConfigEl>>,
    sole_tenant_config: Option<DynamicBlock<ContainerNodePoolNodeConfigElSoleTenantConfigEl>>,
    taint: Option<DynamicBlock<ContainerNodePoolNodeConfigElTaintEl>>,
    windows_node_config: Option<DynamicBlock<ContainerNodePoolNodeConfigElWindowsNodeConfigEl>>,
    workload_metadata_config:
        Option<DynamicBlock<ContainerNodePoolNodeConfigElWorkloadMetadataConfigEl>>,
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_kms_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_confidential_storage: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flex_start: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ssd_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ssd_encryption_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_variant: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_run_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_cpu_platform: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth_scopes: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preemptible: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_pools: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_machine_features: Option<Vec<ContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk: Option<Vec<ContainerNodePoolNodeConfigElBootDiskEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confidential_nodes: Option<Vec<ContainerNodePoolNodeConfigElConfidentialNodesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    containerd_config: Option<Vec<ContainerNodePoolNodeConfigElContainerdConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ephemeral_storage_local_ssd_config:
        Option<Vec<ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fast_socket: Option<Vec<ContainerNodePoolNodeConfigElFastSocketEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcfs_config: Option<Vec<ContainerNodePoolNodeConfigElGcfsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_accelerator: Option<Vec<ContainerNodePoolNodeConfigElGuestAcceleratorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gvnic: Option<Vec<ContainerNodePoolNodeConfigElGvnicEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_maintenance_policy: Option<Vec<ContainerNodePoolNodeConfigElHostMaintenancePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kubelet_config: Option<Vec<ContainerNodePoolNodeConfigElKubeletConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    linux_node_config: Option<Vec<ContainerNodePoolNodeConfigElLinuxNodeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_nvme_ssd_block_config:
        Option<Vec<ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reservation_affinity: Option<Vec<ContainerNodePoolNodeConfigElReservationAffinityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sandbox_config: Option<Vec<ContainerNodePoolNodeConfigElSandboxConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_boot_disks: Option<Vec<ContainerNodePoolNodeConfigElSecondaryBootDisksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shielded_instance_config: Option<Vec<ContainerNodePoolNodeConfigElShieldedInstanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sole_tenant_config: Option<Vec<ContainerNodePoolNodeConfigElSoleTenantConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    taint: Option<Vec<ContainerNodePoolNodeConfigElTaintEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    windows_node_config: Option<Vec<ContainerNodePoolNodeConfigElWindowsNodeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workload_metadata_config: Option<Vec<ContainerNodePoolNodeConfigElWorkloadMetadataConfigEl>>,
    dynamic: ContainerNodePoolNodeConfigElDynamic,
}
impl ContainerNodePoolNodeConfigEl {
    #[doc = "Set the field `boot_disk_kms_key`.\nThe Customer Managed Encryption Key used to encrypt the boot disk attached to each node in the node pool."]
    pub fn set_boot_disk_kms_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.boot_disk_kms_key = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_size_gb`.\nSize of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB."]
    pub fn set_disk_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_type`.\nType of the disk attached to each node. Such as pd-standard, pd-balanced or pd-ssd"]
    pub fn set_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_type = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_confidential_storage`.\nIf enabled boot disks are configured with confidential mode."]
    pub fn set_enable_confidential_storage(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_confidential_storage = Some(v.into());
        self
    }
    #[doc = "Set the field `flex_start`.\nEnables Flex Start provisioning model for the node pool"]
    pub fn set_flex_start(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.flex_start = Some(v.into());
        self
    }
    #[doc = "Set the field `image_type`.\nThe image type to use for this node. Note that for a given image type, the latest version of it will be used."]
    pub fn set_image_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_type = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nThe map of Kubernetes labels (key/value pairs) to be applied to each node. These will added in addition to any default label(s) that Kubernetes may apply to the node."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `local_ssd_count`.\nThe number of local SSD disks to be attached to the node."]
    pub fn set_local_ssd_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.local_ssd_count = Some(v.into());
        self
    }
    #[doc = "Set the field `local_ssd_encryption_mode`.\nLocalSsdEncryptionMode specified the method used for encrypting the local SSDs attached to the node."]
    pub fn set_local_ssd_encryption_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_ssd_encryption_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `logging_variant`.\nType of logging agent that is used as the default value for node pools in the cluster. Valid values include DEFAULT and MAX_THROUGHPUT."]
    pub fn set_logging_variant(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.logging_variant = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_type`.\nThe name of a Google Compute Engine machine type."]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
    #[doc = "Set the field `max_run_duration`.\nThe runtime of each node in the node pool in seconds, terminated by 's'. Example: \"3600s\"."]
    pub fn set_max_run_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_run_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata`.\nThe metadata key/value pairs assigned to instances in the cluster."]
    pub fn set_metadata(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.metadata = Some(v.into());
        self
    }
    #[doc = "Set the field `min_cpu_platform`.\nMinimum CPU platform to be used by this instance. The instance may be scheduled on the specified or newer CPU platform."]
    pub fn set_min_cpu_platform(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_cpu_platform = Some(v.into());
        self
    }
    #[doc = "Set the field `node_group`.\nSetting this field will assign instances of this pool to run on the specified node group. This is useful for running workloads on sole tenant nodes."]
    pub fn set_node_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_group = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth_scopes`.\nThe set of Google API scopes to be made available on all of the node VMs."]
    pub fn set_oauth_scopes(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.oauth_scopes = Some(v.into());
        self
    }
    #[doc = "Set the field `preemptible`.\nWhether the nodes are created as preemptible VM instances."]
    pub fn set_preemptible(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preemptible = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_labels`.\nThe GCE resource labels (a map of key/value pairs) to be applied to the node pool."]
    pub fn set_resource_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_labels = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_manager_tags`.\nA map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored (both PUT & PATCH) when empty."]
    pub fn set_resource_manager_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_manager_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\nThe Google Cloud Platform Service Account to be used by the node VMs."]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `spot`.\nWhether the nodes are created as spot VM instances."]
    pub fn set_spot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.spot = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_pools`.\nThe list of Storage Pools where boot disks are provisioned."]
    pub fn set_storage_pools(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.storage_pools = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\nThe list of instance tags applied to all nodes."]
    pub fn set_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
    #[doc = "Set the field `advanced_machine_features`.\n"]
    pub fn set_advanced_machine_features(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.advanced_machine_features = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_machine_features = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `boot_disk`.\n"]
    pub fn set_boot_disk(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElBootDiskEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.boot_disk = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.boot_disk = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `confidential_nodes`.\n"]
    pub fn set_confidential_nodes(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElConfidentialNodesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.confidential_nodes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.confidential_nodes = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `containerd_config`.\n"]
    pub fn set_containerd_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElContainerdConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.containerd_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.containerd_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ephemeral_storage_local_ssd_config`.\n"]
    pub fn set_ephemeral_storage_local_ssd_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ephemeral_storage_local_ssd_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ephemeral_storage_local_ssd_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `fast_socket`.\n"]
    pub fn set_fast_socket(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElFastSocketEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fast_socket = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fast_socket = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `gcfs_config`.\n"]
    pub fn set_gcfs_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElGcfsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gcfs_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gcfs_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `guest_accelerator`.\n"]
    pub fn set_guest_accelerator(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElGuestAcceleratorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.guest_accelerator = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.guest_accelerator = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `gvnic`.\n"]
    pub fn set_gvnic(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElGvnicEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gvnic = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gvnic = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `host_maintenance_policy`.\n"]
    pub fn set_host_maintenance_policy(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElHostMaintenancePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.host_maintenance_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.host_maintenance_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `kubelet_config`.\n"]
    pub fn set_kubelet_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElKubeletConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kubelet_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kubelet_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `linux_node_config`.\n"]
    pub fn set_linux_node_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElLinuxNodeConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.linux_node_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.linux_node_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `local_nvme_ssd_block_config`.\n"]
    pub fn set_local_nvme_ssd_block_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.local_nvme_ssd_block_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.local_nvme_ssd_block_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `reservation_affinity`.\n"]
    pub fn set_reservation_affinity(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElReservationAffinityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.reservation_affinity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.reservation_affinity = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sandbox_config`.\n"]
    pub fn set_sandbox_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElSandboxConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sandbox_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sandbox_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `secondary_boot_disks`.\n"]
    pub fn set_secondary_boot_disks(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElSecondaryBootDisksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.secondary_boot_disks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.secondary_boot_disks = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElShieldedInstanceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.shielded_instance_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.shielded_instance_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sole_tenant_config`.\n"]
    pub fn set_sole_tenant_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElSoleTenantConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sole_tenant_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sole_tenant_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `taint`.\n"]
    pub fn set_taint(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElTaintEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.taint = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.taint = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `windows_node_config`.\n"]
    pub fn set_windows_node_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElWindowsNodeConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.windows_node_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.windows_node_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `workload_metadata_config`.\n"]
    pub fn set_workload_metadata_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElWorkloadMetadataConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.workload_metadata_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.workload_metadata_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeConfigEl {}
impl BuildContainerNodePoolNodeConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigEl {
        ContainerNodePoolNodeConfigEl {
            boot_disk_kms_key: core::default::Default::default(),
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            enable_confidential_storage: core::default::Default::default(),
            flex_start: core::default::Default::default(),
            image_type: core::default::Default::default(),
            labels: core::default::Default::default(),
            local_ssd_count: core::default::Default::default(),
            local_ssd_encryption_mode: core::default::Default::default(),
            logging_variant: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            max_run_duration: core::default::Default::default(),
            metadata: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
            node_group: core::default::Default::default(),
            oauth_scopes: core::default::Default::default(),
            preemptible: core::default::Default::default(),
            resource_labels: core::default::Default::default(),
            resource_manager_tags: core::default::Default::default(),
            service_account: core::default::Default::default(),
            spot: core::default::Default::default(),
            storage_pools: core::default::Default::default(),
            tags: core::default::Default::default(),
            advanced_machine_features: core::default::Default::default(),
            boot_disk: core::default::Default::default(),
            confidential_nodes: core::default::Default::default(),
            containerd_config: core::default::Default::default(),
            ephemeral_storage_local_ssd_config: core::default::Default::default(),
            fast_socket: core::default::Default::default(),
            gcfs_config: core::default::Default::default(),
            guest_accelerator: core::default::Default::default(),
            gvnic: core::default::Default::default(),
            host_maintenance_policy: core::default::Default::default(),
            kubelet_config: core::default::Default::default(),
            linux_node_config: core::default::Default::default(),
            local_nvme_ssd_block_config: core::default::Default::default(),
            reservation_affinity: core::default::Default::default(),
            sandbox_config: core::default::Default::default(),
            secondary_boot_disks: core::default::Default::default(),
            shielded_instance_config: core::default::Default::default(),
            sole_tenant_config: core::default::Default::default(),
            taint: core::default::Default::default(),
            windows_node_config: core::default::Default::default(),
            workload_metadata_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElRef {
        ContainerNodePoolNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `boot_disk_kms_key` after provisioning.\nThe Customer Managed Encryption Key used to encrypt the boot disk attached to each node in the node pool."]
    pub fn boot_disk_kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.boot_disk_kms_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\nSize of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB."]
    pub fn disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `disk_type` after provisioning.\nType of the disk attached to each node. Such as pd-standard, pd-balanced or pd-ssd"]
    pub fn disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }
    #[doc = "Get a reference to the value of field `effective_taints` after provisioning.\nList of kubernetes taints applied to each node."]
    pub fn effective_taints(&self) -> ListRef<ContainerNodePoolNodeConfigElEffectiveTaintsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.effective_taints", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_confidential_storage` after provisioning.\nIf enabled boot disks are configured with confidential mode."]
    pub fn enable_confidential_storage(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_confidential_storage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `flex_start` after provisioning.\nEnables Flex Start provisioning model for the node pool"]
    pub fn flex_start(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.flex_start", self.base))
    }
    #[doc = "Get a reference to the value of field `image_type` after provisioning.\nThe image type to use for this node. Note that for a given image type, the latest version of it will be used."]
    pub fn image_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_type", self.base))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe map of Kubernetes labels (key/value pairs) to be applied to each node. These will added in addition to any default label(s) that Kubernetes may apply to the node."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `local_ssd_count` after provisioning.\nThe number of local SSD disks to be attached to the node."]
    pub fn local_ssd_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_ssd_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_ssd_encryption_mode` after provisioning.\nLocalSsdEncryptionMode specified the method used for encrypting the local SSDs attached to the node."]
    pub fn local_ssd_encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_ssd_encryption_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `logging_variant` after provisioning.\nType of logging agent that is used as the default value for node pools in the cluster. Valid values include DEFAULT and MAX_THROUGHPUT."]
    pub fn logging_variant(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logging_variant", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nThe name of a Google Compute Engine machine type."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `max_run_duration` after provisioning.\nThe runtime of each node in the node pool in seconds, terminated by 's'. Example: \"3600s\"."]
    pub fn max_run_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_run_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nThe metadata key/value pairs assigned to instances in the cluster."]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
    #[doc = "Get a reference to the value of field `min_cpu_platform` after provisioning.\nMinimum CPU platform to be used by this instance. The instance may be scheduled on the specified or newer CPU platform."]
    pub fn min_cpu_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_cpu_platform", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_group` after provisioning.\nSetting this field will assign instances of this pool to run on the specified node group. This is useful for running workloads on sole tenant nodes."]
    pub fn node_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_group", self.base))
    }
    #[doc = "Get a reference to the value of field `oauth_scopes` after provisioning.\nThe set of Google API scopes to be made available on all of the node VMs."]
    pub fn oauth_scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.oauth_scopes", self.base))
    }
    #[doc = "Get a reference to the value of field `preemptible` after provisioning.\nWhether the nodes are created as preemptible VM instances."]
    pub fn preemptible(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preemptible", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_labels` after provisioning.\nThe GCE resource labels (a map of key/value pairs) to be applied to the node pool."]
    pub fn resource_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_labels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\nA map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored (both PUT & PATCH) when empty."]
    pub fn resource_manager_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_manager_tags", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nThe Google Cloud Platform Service Account to be used by the node VMs."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `spot` after provisioning.\nWhether the nodes are created as spot VM instances."]
    pub fn spot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.spot", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_pools` after provisioning.\nThe list of Storage Pools where boot disks are provisioned."]
    pub fn storage_pools(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_pools", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nThe list of instance tags applied to all nodes."]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
    #[doc = "Get a reference to the value of field `advanced_machine_features` after provisioning.\n"]
    pub fn advanced_machine_features(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElAdvancedMachineFeaturesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_machine_features", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `boot_disk` after provisioning.\n"]
    pub fn boot_disk(&self) -> ListRef<ContainerNodePoolNodeConfigElBootDiskElRef> {
        ListRef::new(self.shared().clone(), format!("{}.boot_disk", self.base))
    }
    #[doc = "Get a reference to the value of field `confidential_nodes` after provisioning.\n"]
    pub fn confidential_nodes(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElConfidentialNodesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.confidential_nodes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `containerd_config` after provisioning.\n"]
    pub fn containerd_config(&self) -> ListRef<ContainerNodePoolNodeConfigElContainerdConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.containerd_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ephemeral_storage_local_ssd_config` after provisioning.\n"]
    pub fn ephemeral_storage_local_ssd_config(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ephemeral_storage_local_ssd_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fast_socket` after provisioning.\n"]
    pub fn fast_socket(&self) -> ListRef<ContainerNodePoolNodeConfigElFastSocketElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fast_socket", self.base))
    }
    #[doc = "Get a reference to the value of field `gcfs_config` after provisioning.\n"]
    pub fn gcfs_config(&self) -> ListRef<ContainerNodePoolNodeConfigElGcfsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gcfs_config", self.base))
    }
    #[doc = "Get a reference to the value of field `guest_accelerator` after provisioning.\n"]
    pub fn guest_accelerator(&self) -> ListRef<ContainerNodePoolNodeConfigElGuestAcceleratorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.guest_accelerator", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gvnic` after provisioning.\n"]
    pub fn gvnic(&self) -> ListRef<ContainerNodePoolNodeConfigElGvnicElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gvnic", self.base))
    }
    #[doc = "Get a reference to the value of field `host_maintenance_policy` after provisioning.\n"]
    pub fn host_maintenance_policy(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElHostMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.host_maintenance_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kubelet_config` after provisioning.\n"]
    pub fn kubelet_config(&self) -> ListRef<ContainerNodePoolNodeConfigElKubeletConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kubelet_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `linux_node_config` after provisioning.\n"]
    pub fn linux_node_config(&self) -> ListRef<ContainerNodePoolNodeConfigElLinuxNodeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.linux_node_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_nvme_ssd_block_config` after provisioning.\n"]
    pub fn local_nvme_ssd_block_config(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.local_nvme_ssd_block_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElReservationAffinityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reservation_affinity", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sandbox_config` after provisioning.\n"]
    pub fn sandbox_config(&self) -> ListRef<ContainerNodePoolNodeConfigElSandboxConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sandbox_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_boot_disks` after provisioning.\n"]
    pub fn secondary_boot_disks(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElSecondaryBootDisksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secondary_boot_disks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElShieldedInstanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.shielded_instance_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sole_tenant_config` after provisioning.\n"]
    pub fn sole_tenant_config(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElSoleTenantConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sole_tenant_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `taint` after provisioning.\n"]
    pub fn taint(&self) -> ListRef<ContainerNodePoolNodeConfigElTaintElRef> {
        ListRef::new(self.shared().clone(), format!("{}.taint", self.base))
    }
    #[doc = "Get a reference to the value of field `windows_node_config` after provisioning.\n"]
    pub fn windows_node_config(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElWindowsNodeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.windows_node_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `workload_metadata_config` after provisioning.\n"]
    pub fn workload_metadata_config(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElWorkloadMetadataConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workload_metadata_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolNodeDrainConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    respect_pdb_during_node_pool_deletion: Option<PrimField<bool>>,
}
impl ContainerNodePoolNodeDrainConfigEl {
    #[doc = "Set the field `respect_pdb_during_node_pool_deletion`.\nWhether to respect PodDisruptionBudget policy during node pool deletion."]
    pub fn set_respect_pdb_during_node_pool_deletion(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.respect_pdb_during_node_pool_deletion = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerNodePoolNodeDrainConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeDrainConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolNodeDrainConfigEl {}
impl BuildContainerNodePoolNodeDrainConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeDrainConfigEl {
        ContainerNodePoolNodeDrainConfigEl {
            respect_pdb_during_node_pool_deletion: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolNodeDrainConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolNodeDrainConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeDrainConfigElRef {
        ContainerNodePoolNodeDrainConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolNodeDrainConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `respect_pdb_during_node_pool_deletion` after provisioning.\nWhether to respect PodDisruptionBudget policy during node pool deletion."]
    pub fn respect_pdb_during_node_pool_deletion(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.respect_pdb_during_node_pool_deletion", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolPlacementPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tpu_topology: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl ContainerNodePoolPlacementPolicyEl {
    #[doc = "Set the field `policy_name`.\nIf set, refers to the name of a custom resource policy supplied by the user. The resource policy must be in the same project and region as the node pool. If not found, InvalidArgument error is returned."]
    pub fn set_policy_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy_name = Some(v.into());
        self
    }
    #[doc = "Set the field `tpu_topology`.\nThe TPU topology like \"2x4\" or \"2x2x2\". https://cloud.google.com/kubernetes-engine/docs/concepts/plan-tpus#topology"]
    pub fn set_tpu_topology(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tpu_topology = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerNodePoolPlacementPolicyEl {
    type O = BlockAssignable<ContainerNodePoolPlacementPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolPlacementPolicyEl {
    #[doc = "Type defines the type of placement policy"]
    pub type_: PrimField<String>,
}
impl BuildContainerNodePoolPlacementPolicyEl {
    pub fn build(self) -> ContainerNodePoolPlacementPolicyEl {
        ContainerNodePoolPlacementPolicyEl {
            policy_name: core::default::Default::default(),
            tpu_topology: core::default::Default::default(),
            type_: self.type_,
        }
    }
}
pub struct ContainerNodePoolPlacementPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolPlacementPolicyElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolPlacementPolicyElRef {
        ContainerNodePoolPlacementPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolPlacementPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `policy_name` after provisioning.\nIf set, refers to the name of a custom resource policy supplied by the user. The resource policy must be in the same project and region as the node pool. If not found, InvalidArgument error is returned."]
    pub fn policy_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_name", self.base))
    }
    #[doc = "Get a reference to the value of field `tpu_topology` after provisioning.\nThe TPU topology like \"2x4\" or \"2x2x2\". https://cloud.google.com/kubernetes-engine/docs/concepts/plan-tpus#topology"]
    pub fn tpu_topology(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tpu_topology", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType defines the type of placement policy"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolQueuedProvisioningEl {
    enabled: PrimField<bool>,
}
impl ContainerNodePoolQueuedProvisioningEl {}
impl ToListMappable for ContainerNodePoolQueuedProvisioningEl {
    type O = BlockAssignable<ContainerNodePoolQueuedProvisioningEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolQueuedProvisioningEl {
    #[doc = "Whether nodes in this node pool are obtainable solely through the ProvisioningRequest API"]
    pub enabled: PrimField<bool>,
}
impl BuildContainerNodePoolQueuedProvisioningEl {
    pub fn build(self) -> ContainerNodePoolQueuedProvisioningEl {
        ContainerNodePoolQueuedProvisioningEl {
            enabled: self.enabled,
        }
    }
}
pub struct ContainerNodePoolQueuedProvisioningElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolQueuedProvisioningElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolQueuedProvisioningElRef {
        ContainerNodePoolQueuedProvisioningElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolQueuedProvisioningElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether nodes in this node pool are obtainable solely through the ProvisioningRequest API"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerNodePoolTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ContainerNodePoolTimeoutsEl {
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
impl ToListMappable for ContainerNodePoolTimeoutsEl {
    type O = BlockAssignable<ContainerNodePoolTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolTimeoutsEl {}
impl BuildContainerNodePoolTimeoutsEl {
    pub fn build(self) -> ContainerNodePoolTimeoutsEl {
        ContainerNodePoolTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolTimeoutsElRef {
        ContainerNodePoolTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolTimeoutsElRef {
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
pub struct ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_soak_duration: Option<PrimField<String>>,
}
impl ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
    #[doc = "Set the field `batch_node_count`.\nNumber of blue nodes to drain in a batch."]
    pub fn set_batch_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.batch_node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `batch_percentage`.\nPercentage of the blue pool nodes to drain in a batch."]
    pub fn set_batch_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.batch_percentage = Some(v.into());
        self
    }
    #[doc = "Set the field `batch_soak_duration`.\nSoak time after each batch gets drained."]
    pub fn set_batch_soak_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.batch_soak_duration = Some(v.into());
        self
    }
}
impl ToListMappable
    for ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl
{
    type O = BlockAssignable<
        ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {}
impl BuildContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
    pub fn build(
        self,
    ) -> ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
        ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
            batch_node_count: core::default::Default::default(),
            batch_percentage: core::default::Default::default(),
            batch_soak_duration: core::default::Default::default(),
        }
    }
}
pub struct ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
        ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `batch_node_count` after provisioning.\nNumber of blue nodes to drain in a batch."]
    pub fn batch_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.batch_node_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `batch_percentage` after provisioning.\nPercentage of the blue pool nodes to drain in a batch."]
    pub fn batch_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.batch_percentage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `batch_soak_duration` after provisioning.\nSoak time after each batch gets drained."]
    pub fn batch_soak_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.batch_soak_duration", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElDynamic {
    standard_rollout_policy: Option<
        DynamicBlock<ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl>,
    >,
}
#[derive(Serialize)]
pub struct ContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    node_pool_soak_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    standard_rollout_policy:
        Option<Vec<ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl>>,
    dynamic: ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElDynamic,
}
impl ContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl {
    #[doc = "Set the field `node_pool_soak_duration`.\nTime needed after draining entire blue pool. After this period, blue pool will be cleaned up."]
    pub fn set_node_pool_soak_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_pool_soak_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `standard_rollout_policy`.\n"]
    pub fn set_standard_rollout_policy(
        mut self,
        v: impl Into<
            BlockAssignable<
                ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.standard_rollout_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.standard_rollout_policy = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl {
    type O = BlockAssignable<ContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl {}
impl BuildContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl {
    pub fn build(self) -> ContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl {
        ContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl {
            node_pool_soak_duration: core::default::Default::default(),
            standard_rollout_policy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElRef {
        ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `node_pool_soak_duration` after provisioning.\nTime needed after draining entire blue pool. After this period, blue pool will be cleaned up."]
    pub fn node_pool_soak_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_pool_soak_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `standard_rollout_policy` after provisioning.\n"]
    pub fn standard_rollout_policy(
        &self,
    ) -> ListRef<ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.standard_rollout_policy", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ContainerNodePoolUpgradeSettingsElDynamic {
    blue_green_settings:
        Option<DynamicBlock<ContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl>>,
}
#[derive(Serialize)]
pub struct ContainerNodePoolUpgradeSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_surge: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_unavailable: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blue_green_settings: Option<Vec<ContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl>>,
    dynamic: ContainerNodePoolUpgradeSettingsElDynamic,
}
impl ContainerNodePoolUpgradeSettingsEl {
    #[doc = "Set the field `max_surge`.\nThe number of additional nodes that can be added to the node pool during an upgrade. Increasing max_surge raises the number of nodes that can be upgraded simultaneously. Can be set to 0 or greater."]
    pub fn set_max_surge(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_surge = Some(v.into());
        self
    }
    #[doc = "Set the field `max_unavailable`.\nThe number of nodes that can be simultaneously unavailable during an upgrade. Increasing max_unavailable raises the number of nodes that can be upgraded in parallel. Can be set to 0 or greater."]
    pub fn set_max_unavailable(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_unavailable = Some(v.into());
        self
    }
    #[doc = "Set the field `strategy`.\nUpdate strategy for the given nodepool."]
    pub fn set_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.strategy = Some(v.into());
        self
    }
    #[doc = "Set the field `blue_green_settings`.\n"]
    pub fn set_blue_green_settings(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.blue_green_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.blue_green_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ContainerNodePoolUpgradeSettingsEl {
    type O = BlockAssignable<ContainerNodePoolUpgradeSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerNodePoolUpgradeSettingsEl {}
impl BuildContainerNodePoolUpgradeSettingsEl {
    pub fn build(self) -> ContainerNodePoolUpgradeSettingsEl {
        ContainerNodePoolUpgradeSettingsEl {
            max_surge: core::default::Default::default(),
            max_unavailable: core::default::Default::default(),
            strategy: core::default::Default::default(),
            blue_green_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContainerNodePoolUpgradeSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerNodePoolUpgradeSettingsElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolUpgradeSettingsElRef {
        ContainerNodePoolUpgradeSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerNodePoolUpgradeSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_surge` after provisioning.\nThe number of additional nodes that can be added to the node pool during an upgrade. Increasing max_surge raises the number of nodes that can be upgraded simultaneously. Can be set to 0 or greater."]
    pub fn max_surge(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_surge", self.base))
    }
    #[doc = "Get a reference to the value of field `max_unavailable` after provisioning.\nThe number of nodes that can be simultaneously unavailable during an upgrade. Increasing max_unavailable raises the number of nodes that can be upgraded in parallel. Can be set to 0 or greater."]
    pub fn max_unavailable(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_unavailable", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `strategy` after provisioning.\nUpdate strategy for the given nodepool."]
    pub fn strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.strategy", self.base))
    }
    #[doc = "Get a reference to the value of field `blue_green_settings` after provisioning.\n"]
    pub fn blue_green_settings(
        &self,
    ) -> ListRef<ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.blue_green_settings", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ContainerNodePoolDynamic {
    autoscaling: Option<DynamicBlock<ContainerNodePoolAutoscalingEl>>,
    management: Option<DynamicBlock<ContainerNodePoolManagementEl>>,
    network_config: Option<DynamicBlock<ContainerNodePoolNetworkConfigEl>>,
    node_config: Option<DynamicBlock<ContainerNodePoolNodeConfigEl>>,
    node_drain_config: Option<DynamicBlock<ContainerNodePoolNodeDrainConfigEl>>,
    placement_policy: Option<DynamicBlock<ContainerNodePoolPlacementPolicyEl>>,
    queued_provisioning: Option<DynamicBlock<ContainerNodePoolQueuedProvisioningEl>>,
    upgrade_settings: Option<DynamicBlock<ContainerNodePoolUpgradeSettingsEl>>,
}
