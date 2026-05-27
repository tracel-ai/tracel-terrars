use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct GkeonpremVmwareAdminClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bootstrap_cluster_membership: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_advanced_cluster: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_type: Option<PrimField<String>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_prem_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    addon_node: Option<Vec<GkeonpremVmwareAdminClusterAddonNodeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    anti_affinity_groups: Option<Vec<GkeonpremVmwareAdminClusterAntiAffinityGroupsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization: Option<Vec<GkeonpremVmwareAdminClusterAuthorizationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_repair_config: Option<Vec<GkeonpremVmwareAdminClusterAutoRepairConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    control_plane_node: Option<Vec<GkeonpremVmwareAdminClusterControlPlaneNodeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancer: Option<Vec<GkeonpremVmwareAdminClusterLoadBalancerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_config: Option<Vec<GkeonpremVmwareAdminClusterNetworkConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform_config: Option<Vec<GkeonpremVmwareAdminClusterPlatformConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_registry_config: Option<Vec<GkeonpremVmwareAdminClusterPrivateRegistryConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy: Option<Vec<GkeonpremVmwareAdminClusterProxyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<GkeonpremVmwareAdminClusterTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcenter: Option<Vec<GkeonpremVmwareAdminClusterVcenterEl>>,
    dynamic: GkeonpremVmwareAdminClusterDynamic,
}
struct GkeonpremVmwareAdminCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GkeonpremVmwareAdminClusterData>,
}
#[derive(Clone)]
pub struct GkeonpremVmwareAdminCluster(Rc<GkeonpremVmwareAdminCluster_>);
impl GkeonpremVmwareAdminCluster {
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
    #[doc = "Set the field `annotations`.\nAnnotations on the VMware Admin Cluster.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `bootstrap_cluster_membership`.\nThe bootstrap cluster this VMware admin cluster belongs to."]
    pub fn set_bootstrap_cluster_membership(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().bootstrap_cluster_membership = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nA human readable description of this VMware admin cluster."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_advanced_cluster`.\nIf set, the advanced cluster feature is enabled."]
    pub fn set_enable_advanced_cluster(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_advanced_cluster = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `image_type`.\nThe OS image type for the VMware admin cluster."]
    pub fn set_image_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().image_type = Some(v.into());
        self
    }
    #[doc = "Set the field `on_prem_version`.\nThe Anthos clusters on the VMware version for the admin cluster."]
    pub fn set_on_prem_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().on_prem_version = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `addon_node`.\n"]
    pub fn set_addon_node(
        self,
        v: impl Into<BlockAssignable<GkeonpremVmwareAdminClusterAddonNodeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().addon_node = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.addon_node = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `anti_affinity_groups`.\n"]
    pub fn set_anti_affinity_groups(
        self,
        v: impl Into<BlockAssignable<GkeonpremVmwareAdminClusterAntiAffinityGroupsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().anti_affinity_groups = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.anti_affinity_groups = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `authorization`.\n"]
    pub fn set_authorization(
        self,
        v: impl Into<BlockAssignable<GkeonpremVmwareAdminClusterAuthorizationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().authorization = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.authorization = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `auto_repair_config`.\n"]
    pub fn set_auto_repair_config(
        self,
        v: impl Into<BlockAssignable<GkeonpremVmwareAdminClusterAutoRepairConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_repair_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_repair_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `control_plane_node`.\n"]
    pub fn set_control_plane_node(
        self,
        v: impl Into<BlockAssignable<GkeonpremVmwareAdminClusterControlPlaneNodeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().control_plane_node = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.control_plane_node = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `load_balancer`.\n"]
    pub fn set_load_balancer(
        self,
        v: impl Into<BlockAssignable<GkeonpremVmwareAdminClusterLoadBalancerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().load_balancer = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.load_balancer = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_config`.\n"]
    pub fn set_network_config(
        self,
        v: impl Into<BlockAssignable<GkeonpremVmwareAdminClusterNetworkConfigEl>>,
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
    #[doc = "Set the field `platform_config`.\n"]
    pub fn set_platform_config(
        self,
        v: impl Into<BlockAssignable<GkeonpremVmwareAdminClusterPlatformConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().platform_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.platform_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `private_registry_config`.\n"]
    pub fn set_private_registry_config(
        self,
        v: impl Into<BlockAssignable<GkeonpremVmwareAdminClusterPrivateRegistryConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().private_registry_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.private_registry_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `proxy`.\n"]
    pub fn set_proxy(
        self,
        v: impl Into<BlockAssignable<GkeonpremVmwareAdminClusterProxyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().proxy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.proxy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<GkeonpremVmwareAdminClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `vcenter`.\n"]
    pub fn set_vcenter(
        self,
        v: impl Into<BlockAssignable<GkeonpremVmwareAdminClusterVcenterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vcenter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vcenter = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nAnnotations on the VMware Admin Cluster.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bootstrap_cluster_membership` after provisioning.\nThe bootstrap cluster this VMware admin cluster belongs to."]
    pub fn bootstrap_cluster_membership(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_cluster_membership", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time the cluster was created, in RFC3339 text format."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human readable description of this VMware admin cluster."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_advanced_cluster` after provisioning.\nIf set, the advanced cluster feature is enabled."]
    pub fn enable_advanced_cluster(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_advanced_cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nThe DNS name of VMware admin cluster's API server."]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other\nfields, and may be sent on update and delete requests to ensure the\nclient has an up-to-date value before proceeding.\nAllows clients to perform consistent read-modify-writes\nthrough optimistic concurrency control."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet` after provisioning.\nFleet configuration for the cluster."]
    pub fn fleet(&self) -> ListRef<GkeonpremVmwareAdminClusterFleetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fleet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `image_type` after provisioning.\nThe OS image type for the VMware admin cluster."]
    pub fn image_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `local_name` after provisioning.\nThe object name of the VMwareAdminCluster custom resource on the\nassociated admin cluster. This field is used to support conflicting\nnames when enrolling existing clusters to the API. When used as a part of\ncluster enrollment, this field will differ from the ID in the resource\nname. For new clusters, this field will match the user provided cluster ID\nand be visible in the last component of the resource name. It is not\nmodifiable.\nAll users should use this name to access their cluster using gkectl or\nkubectl and should expect to see the local name when viewing admin\ncluster controller logs."]
    pub fn local_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe VMware admin cluster resource name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `on_prem_version` after provisioning.\nThe Anthos clusters on the VMware version for the admin cluster."]
    pub fn on_prem_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_prem_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the VMware admin cluster."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe lifecycle state of the VMware admin cluster."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nResourceStatus representing detailed cluster state."]
    pub fn status(&self) -> ListRef<GkeonpremVmwareAdminClusterStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe unique identifier of the VMware Admin Cluster."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time the cluster was last updated, in RFC3339 text format."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `addon_node` after provisioning.\n"]
    pub fn addon_node(&self) -> ListRef<GkeonpremVmwareAdminClusterAddonNodeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.addon_node", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `anti_affinity_groups` after provisioning.\n"]
    pub fn anti_affinity_groups(
        &self,
    ) -> ListRef<GkeonpremVmwareAdminClusterAntiAffinityGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.anti_affinity_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> ListRef<GkeonpremVmwareAdminClusterAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auto_repair_config` after provisioning.\n"]
    pub fn auto_repair_config(&self) -> ListRef<GkeonpremVmwareAdminClusterAutoRepairConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_repair_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `control_plane_node` after provisioning.\n"]
    pub fn control_plane_node(&self) -> ListRef<GkeonpremVmwareAdminClusterControlPlaneNodeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane_node", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancer` after provisioning.\n"]
    pub fn load_balancer(&self) -> ListRef<GkeonpremVmwareAdminClusterLoadBalancerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.load_balancer", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<GkeonpremVmwareAdminClusterNetworkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `platform_config` after provisioning.\n"]
    pub fn platform_config(&self) -> ListRef<GkeonpremVmwareAdminClusterPlatformConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.platform_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_registry_config` after provisioning.\n"]
    pub fn private_registry_config(
        &self,
    ) -> ListRef<GkeonpremVmwareAdminClusterPrivateRegistryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_registry_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `proxy` after provisioning.\n"]
    pub fn proxy(&self) -> ListRef<GkeonpremVmwareAdminClusterProxyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.proxy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeonpremVmwareAdminClusterTimeoutsElRef {
        GkeonpremVmwareAdminClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vcenter` after provisioning.\n"]
    pub fn vcenter(&self) -> ListRef<GkeonpremVmwareAdminClusterVcenterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vcenter", self.extract_ref()),
        )
    }
}
impl Referable for GkeonpremVmwareAdminCluster {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for GkeonpremVmwareAdminCluster {}
impl ToListMappable for GkeonpremVmwareAdminCluster {
    type O = ListRef<GkeonpremVmwareAdminClusterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for GkeonpremVmwareAdminCluster_ {
    fn extract_resource_type(&self) -> String {
        "google_gkeonprem_vmware_admin_cluster".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildGkeonpremVmwareAdminCluster {
    pub tf_id: String,
    #[doc = "The location of the resource."]
    pub location: PrimField<String>,
    #[doc = "The VMware admin cluster resource name."]
    pub name: PrimField<String>,
}
impl BuildGkeonpremVmwareAdminCluster {
    pub fn build(self, stack: &mut Stack) -> GkeonpremVmwareAdminCluster {
        let out = GkeonpremVmwareAdminCluster(Rc::new(GkeonpremVmwareAdminCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GkeonpremVmwareAdminClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                bootstrap_cluster_membership: core::default::Default::default(),
                description: core::default::Default::default(),
                enable_advanced_cluster: core::default::Default::default(),
                id: core::default::Default::default(),
                image_type: core::default::Default::default(),
                location: self.location,
                name: self.name,
                on_prem_version: core::default::Default::default(),
                project: core::default::Default::default(),
                addon_node: core::default::Default::default(),
                anti_affinity_groups: core::default::Default::default(),
                authorization: core::default::Default::default(),
                auto_repair_config: core::default::Default::default(),
                control_plane_node: core::default::Default::default(),
                load_balancer: core::default::Default::default(),
                network_config: core::default::Default::default(),
                platform_config: core::default::Default::default(),
                private_registry_config: core::default::Default::default(),
                proxy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vcenter: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct GkeonpremVmwareAdminClusterRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl GkeonpremVmwareAdminClusterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nAnnotations on the VMware Admin Cluster.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bootstrap_cluster_membership` after provisioning.\nThe bootstrap cluster this VMware admin cluster belongs to."]
    pub fn bootstrap_cluster_membership(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_cluster_membership", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time the cluster was created, in RFC3339 text format."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human readable description of this VMware admin cluster."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_advanced_cluster` after provisioning.\nIf set, the advanced cluster feature is enabled."]
    pub fn enable_advanced_cluster(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_advanced_cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nThe DNS name of VMware admin cluster's API server."]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other\nfields, and may be sent on update and delete requests to ensure the\nclient has an up-to-date value before proceeding.\nAllows clients to perform consistent read-modify-writes\nthrough optimistic concurrency control."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet` after provisioning.\nFleet configuration for the cluster."]
    pub fn fleet(&self) -> ListRef<GkeonpremVmwareAdminClusterFleetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fleet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `image_type` after provisioning.\nThe OS image type for the VMware admin cluster."]
    pub fn image_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `local_name` after provisioning.\nThe object name of the VMwareAdminCluster custom resource on the\nassociated admin cluster. This field is used to support conflicting\nnames when enrolling existing clusters to the API. When used as a part of\ncluster enrollment, this field will differ from the ID in the resource\nname. For new clusters, this field will match the user provided cluster ID\nand be visible in the last component of the resource name. It is not\nmodifiable.\nAll users should use this name to access their cluster using gkectl or\nkubectl and should expect to see the local name when viewing admin\ncluster controller logs."]
    pub fn local_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe VMware admin cluster resource name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `on_prem_version` after provisioning.\nThe Anthos clusters on the VMware version for the admin cluster."]
    pub fn on_prem_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_prem_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the VMware admin cluster."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe lifecycle state of the VMware admin cluster."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nResourceStatus representing detailed cluster state."]
    pub fn status(&self) -> ListRef<GkeonpremVmwareAdminClusterStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe unique identifier of the VMware Admin Cluster."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time the cluster was last updated, in RFC3339 text format."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `addon_node` after provisioning.\n"]
    pub fn addon_node(&self) -> ListRef<GkeonpremVmwareAdminClusterAddonNodeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.addon_node", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `anti_affinity_groups` after provisioning.\n"]
    pub fn anti_affinity_groups(
        &self,
    ) -> ListRef<GkeonpremVmwareAdminClusterAntiAffinityGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.anti_affinity_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> ListRef<GkeonpremVmwareAdminClusterAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auto_repair_config` after provisioning.\n"]
    pub fn auto_repair_config(&self) -> ListRef<GkeonpremVmwareAdminClusterAutoRepairConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_repair_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `control_plane_node` after provisioning.\n"]
    pub fn control_plane_node(&self) -> ListRef<GkeonpremVmwareAdminClusterControlPlaneNodeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane_node", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancer` after provisioning.\n"]
    pub fn load_balancer(&self) -> ListRef<GkeonpremVmwareAdminClusterLoadBalancerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.load_balancer", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<GkeonpremVmwareAdminClusterNetworkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `platform_config` after provisioning.\n"]
    pub fn platform_config(&self) -> ListRef<GkeonpremVmwareAdminClusterPlatformConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.platform_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_registry_config` after provisioning.\n"]
    pub fn private_registry_config(
        &self,
    ) -> ListRef<GkeonpremVmwareAdminClusterPrivateRegistryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_registry_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `proxy` after provisioning.\n"]
    pub fn proxy(&self) -> ListRef<GkeonpremVmwareAdminClusterProxyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.proxy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeonpremVmwareAdminClusterTimeoutsElRef {
        GkeonpremVmwareAdminClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vcenter` after provisioning.\n"]
    pub fn vcenter(&self) -> ListRef<GkeonpremVmwareAdminClusterVcenterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vcenter", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterFleetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    membership: Option<PrimField<String>>,
}
impl GkeonpremVmwareAdminClusterFleetEl {
    #[doc = "Set the field `membership`.\n"]
    pub fn set_membership(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.membership = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareAdminClusterFleetEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterFleetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterFleetEl {}
impl BuildGkeonpremVmwareAdminClusterFleetEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterFleetEl {
        GkeonpremVmwareAdminClusterFleetEl {
            membership: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareAdminClusterFleetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterFleetElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareAdminClusterFleetElRef {
        GkeonpremVmwareAdminClusterFleetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterFleetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `membership` after provisioning.\n"]
    pub fn membership(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.membership", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterStatusElConditionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    last_transition_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl GkeonpremVmwareAdminClusterStatusElConditionsEl {
    #[doc = "Set the field `last_transition_time`.\n"]
    pub fn set_last_transition_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_transition_time = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reason = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareAdminClusterStatusElConditionsEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterStatusElConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterStatusElConditionsEl {}
impl BuildGkeonpremVmwareAdminClusterStatusElConditionsEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterStatusElConditionsEl {
        GkeonpremVmwareAdminClusterStatusElConditionsEl {
            last_transition_time: core::default::Default::default(),
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
            state: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareAdminClusterStatusElConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterStatusElConditionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareAdminClusterStatusElConditionsElRef {
        GkeonpremVmwareAdminClusterStatusElConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterStatusElConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `last_transition_time` after provisioning.\n"]
    pub fn last_transition_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_transition_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions: Option<ListField<GkeonpremVmwareAdminClusterStatusElConditionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<PrimField<String>>,
}
impl GkeonpremVmwareAdminClusterStatusEl {
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl Into<ListField<GkeonpremVmwareAdminClusterStatusElConditionsEl>>,
    ) -> Self {
        self.conditions = Some(v.into());
        self
    }
    #[doc = "Set the field `error_message`.\n"]
    pub fn set_error_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.error_message = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareAdminClusterStatusEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterStatusEl {}
impl BuildGkeonpremVmwareAdminClusterStatusEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterStatusEl {
        GkeonpremVmwareAdminClusterStatusEl {
            conditions: core::default::Default::default(),
            error_message: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareAdminClusterStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterStatusElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareAdminClusterStatusElRef {
        GkeonpremVmwareAdminClusterStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> ListRef<GkeonpremVmwareAdminClusterStatusElConditionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }
    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.error_message", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterAddonNodeElAutoResizeConfigEl {
    enabled: PrimField<bool>,
}
impl GkeonpremVmwareAdminClusterAddonNodeElAutoResizeConfigEl {}
impl ToListMappable for GkeonpremVmwareAdminClusterAddonNodeElAutoResizeConfigEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterAddonNodeElAutoResizeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterAddonNodeElAutoResizeConfigEl {
    #[doc = "Whether to enable controle plane node auto resizing."]
    pub enabled: PrimField<bool>,
}
impl BuildGkeonpremVmwareAdminClusterAddonNodeElAutoResizeConfigEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterAddonNodeElAutoResizeConfigEl {
        GkeonpremVmwareAdminClusterAddonNodeElAutoResizeConfigEl {
            enabled: self.enabled,
        }
    }
}
pub struct GkeonpremVmwareAdminClusterAddonNodeElAutoResizeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterAddonNodeElAutoResizeConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareAdminClusterAddonNodeElAutoResizeConfigElRef {
        GkeonpremVmwareAdminClusterAddonNodeElAutoResizeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterAddonNodeElAutoResizeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether to enable controle plane node auto resizing."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremVmwareAdminClusterAddonNodeElDynamic {
    auto_resize_config:
        Option<DynamicBlock<GkeonpremVmwareAdminClusterAddonNodeElAutoResizeConfigEl>>,
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterAddonNodeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_resize_config: Option<Vec<GkeonpremVmwareAdminClusterAddonNodeElAutoResizeConfigEl>>,
    dynamic: GkeonpremVmwareAdminClusterAddonNodeElDynamic,
}
impl GkeonpremVmwareAdminClusterAddonNodeEl {
    #[doc = "Set the field `auto_resize_config`.\n"]
    pub fn set_auto_resize_config(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremVmwareAdminClusterAddonNodeElAutoResizeConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.auto_resize_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.auto_resize_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeonpremVmwareAdminClusterAddonNodeEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterAddonNodeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterAddonNodeEl {}
impl BuildGkeonpremVmwareAdminClusterAddonNodeEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterAddonNodeEl {
        GkeonpremVmwareAdminClusterAddonNodeEl {
            auto_resize_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremVmwareAdminClusterAddonNodeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterAddonNodeElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareAdminClusterAddonNodeElRef {
        GkeonpremVmwareAdminClusterAddonNodeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterAddonNodeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto_resize_config` after provisioning.\n"]
    pub fn auto_resize_config(
        &self,
    ) -> ListRef<GkeonpremVmwareAdminClusterAddonNodeElAutoResizeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_resize_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterAntiAffinityGroupsEl {
    aag_config_disabled: PrimField<bool>,
}
impl GkeonpremVmwareAdminClusterAntiAffinityGroupsEl {}
impl ToListMappable for GkeonpremVmwareAdminClusterAntiAffinityGroupsEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterAntiAffinityGroupsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterAntiAffinityGroupsEl {
    #[doc = "Spread nodes across at least three physical hosts (requires at least three\nhosts).\nEnabled by default."]
    pub aag_config_disabled: PrimField<bool>,
}
impl BuildGkeonpremVmwareAdminClusterAntiAffinityGroupsEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterAntiAffinityGroupsEl {
        GkeonpremVmwareAdminClusterAntiAffinityGroupsEl {
            aag_config_disabled: self.aag_config_disabled,
        }
    }
}
pub struct GkeonpremVmwareAdminClusterAntiAffinityGroupsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterAntiAffinityGroupsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareAdminClusterAntiAffinityGroupsElRef {
        GkeonpremVmwareAdminClusterAntiAffinityGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterAntiAffinityGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aag_config_disabled` after provisioning.\nSpread nodes across at least three physical hosts (requires at least three\nhosts).\nEnabled by default."]
    pub fn aag_config_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aag_config_disabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterAuthorizationElViewerUsersEl {
    username: PrimField<String>,
}
impl GkeonpremVmwareAdminClusterAuthorizationElViewerUsersEl {}
impl ToListMappable for GkeonpremVmwareAdminClusterAuthorizationElViewerUsersEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterAuthorizationElViewerUsersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterAuthorizationElViewerUsersEl {
    #[doc = "The name of the user, e.g. 'my-gcp-id@gmail.com'."]
    pub username: PrimField<String>,
}
impl BuildGkeonpremVmwareAdminClusterAuthorizationElViewerUsersEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterAuthorizationElViewerUsersEl {
        GkeonpremVmwareAdminClusterAuthorizationElViewerUsersEl {
            username: self.username,
        }
    }
}
pub struct GkeonpremVmwareAdminClusterAuthorizationElViewerUsersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterAuthorizationElViewerUsersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareAdminClusterAuthorizationElViewerUsersElRef {
        GkeonpremVmwareAdminClusterAuthorizationElViewerUsersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterAuthorizationElViewerUsersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nThe name of the user, e.g. 'my-gcp-id@gmail.com'."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremVmwareAdminClusterAuthorizationElDynamic {
    viewer_users: Option<DynamicBlock<GkeonpremVmwareAdminClusterAuthorizationElViewerUsersEl>>,
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterAuthorizationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    viewer_users: Option<Vec<GkeonpremVmwareAdminClusterAuthorizationElViewerUsersEl>>,
    dynamic: GkeonpremVmwareAdminClusterAuthorizationElDynamic,
}
impl GkeonpremVmwareAdminClusterAuthorizationEl {
    #[doc = "Set the field `viewer_users`.\n"]
    pub fn set_viewer_users(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremVmwareAdminClusterAuthorizationElViewerUsersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.viewer_users = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.viewer_users = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeonpremVmwareAdminClusterAuthorizationEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterAuthorizationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterAuthorizationEl {}
impl BuildGkeonpremVmwareAdminClusterAuthorizationEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterAuthorizationEl {
        GkeonpremVmwareAdminClusterAuthorizationEl {
            viewer_users: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremVmwareAdminClusterAuthorizationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterAuthorizationElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareAdminClusterAuthorizationElRef {
        GkeonpremVmwareAdminClusterAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterAuthorizationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `viewer_users` after provisioning.\n"]
    pub fn viewer_users(
        &self,
    ) -> ListRef<GkeonpremVmwareAdminClusterAuthorizationElViewerUsersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.viewer_users", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterAutoRepairConfigEl {
    enabled: PrimField<bool>,
}
impl GkeonpremVmwareAdminClusterAutoRepairConfigEl {}
impl ToListMappable for GkeonpremVmwareAdminClusterAutoRepairConfigEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterAutoRepairConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterAutoRepairConfigEl {
    #[doc = "Whether auto repair is enabled."]
    pub enabled: PrimField<bool>,
}
impl BuildGkeonpremVmwareAdminClusterAutoRepairConfigEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterAutoRepairConfigEl {
        GkeonpremVmwareAdminClusterAutoRepairConfigEl {
            enabled: self.enabled,
        }
    }
}
pub struct GkeonpremVmwareAdminClusterAutoRepairConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterAutoRepairConfigElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareAdminClusterAutoRepairConfigElRef {
        GkeonpremVmwareAdminClusterAutoRepairConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterAutoRepairConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether auto repair is enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterControlPlaneNodeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpus: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replicas: Option<PrimField<f64>>,
}
impl GkeonpremVmwareAdminClusterControlPlaneNodeEl {
    #[doc = "Set the field `cpus`.\nThe number of vCPUs for the control-plane node of the admin cluster."]
    pub fn set_cpus(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpus = Some(v.into());
        self
    }
    #[doc = "Set the field `memory`.\nThe number of mebibytes of memory for the control-plane node of the admin cluster."]
    pub fn set_memory(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory = Some(v.into());
        self
    }
    #[doc = "Set the field `replicas`.\nThe number of control plane nodes for this VMware admin cluster."]
    pub fn set_replicas(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.replicas = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareAdminClusterControlPlaneNodeEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterControlPlaneNodeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterControlPlaneNodeEl {}
impl BuildGkeonpremVmwareAdminClusterControlPlaneNodeEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterControlPlaneNodeEl {
        GkeonpremVmwareAdminClusterControlPlaneNodeEl {
            cpus: core::default::Default::default(),
            memory: core::default::Default::default(),
            replicas: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareAdminClusterControlPlaneNodeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterControlPlaneNodeElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareAdminClusterControlPlaneNodeElRef {
        GkeonpremVmwareAdminClusterControlPlaneNodeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterControlPlaneNodeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cpus` after provisioning.\nThe number of vCPUs for the control-plane node of the admin cluster."]
    pub fn cpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpus", self.base))
    }
    #[doc = "Get a reference to the value of field `memory` after provisioning.\nThe number of mebibytes of memory for the control-plane node of the admin cluster."]
    pub fn memory(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
    #[doc = "Get a reference to the value of field `replicas` after provisioning.\nThe number of control plane nodes for this VMware admin cluster."]
    pub fn replicas(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.replicas", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterLoadBalancerElF5ConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snat_pool: Option<PrimField<String>>,
}
impl GkeonpremVmwareAdminClusterLoadBalancerElF5ConfigEl {
    #[doc = "Set the field `address`.\nThe load balancer's IP address."]
    pub fn set_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address = Some(v.into());
        self
    }
    #[doc = "Set the field `partition`.\nhe preexisting partition to be used by the load balancer. T\nhis partition is usually created for the admin cluster for example:\n'my-f5-admin-partition'."]
    pub fn set_partition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.partition = Some(v.into());
        self
    }
    #[doc = "Set the field `snat_pool`.\nThe pool name. Only necessary, if using SNAT."]
    pub fn set_snat_pool(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.snat_pool = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareAdminClusterLoadBalancerElF5ConfigEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterLoadBalancerElF5ConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterLoadBalancerElF5ConfigEl {}
impl BuildGkeonpremVmwareAdminClusterLoadBalancerElF5ConfigEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterLoadBalancerElF5ConfigEl {
        GkeonpremVmwareAdminClusterLoadBalancerElF5ConfigEl {
            address: core::default::Default::default(),
            partition: core::default::Default::default(),
            snat_pool: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareAdminClusterLoadBalancerElF5ConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterLoadBalancerElF5ConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareAdminClusterLoadBalancerElF5ConfigElRef {
        GkeonpremVmwareAdminClusterLoadBalancerElF5ConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterLoadBalancerElF5ConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address` after provisioning.\nThe load balancer's IP address."]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }
    #[doc = "Get a reference to the value of field `partition` after provisioning.\nhe preexisting partition to be used by the load balancer. T\nhis partition is usually created for the admin cluster for example:\n'my-f5-admin-partition'."]
    pub fn partition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.partition", self.base))
    }
    #[doc = "Get a reference to the value of field `snat_pool` after provisioning.\nThe pool name. Only necessary, if using SNAT."]
    pub fn snat_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snat_pool", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterLoadBalancerElManualLbConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    addons_node_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    control_plane_node_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress_http_node_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress_https_node_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    konnectivity_server_node_port: Option<PrimField<f64>>,
}
impl GkeonpremVmwareAdminClusterLoadBalancerElManualLbConfigEl {
    #[doc = "Set the field `addons_node_port`.\nNodePort for add-ons server in the admin cluster."]
    pub fn set_addons_node_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.addons_node_port = Some(v.into());
        self
    }
    #[doc = "Set the field `control_plane_node_port`.\nNodePort for control plane service. The Kubernetes API server in the admin\ncluster is implemented as a Service of type NodePort (ex. 30968)."]
    pub fn set_control_plane_node_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.control_plane_node_port = Some(v.into());
        self
    }
    #[doc = "Set the field `ingress_http_node_port`.\nNodePort for ingress service's http. The ingress service in the admin\ncluster is implemented as a Service of type NodePort (ex. 32527)."]
    pub fn set_ingress_http_node_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ingress_http_node_port = Some(v.into());
        self
    }
    #[doc = "Set the field `ingress_https_node_port`.\nNodePort for ingress service's https. The ingress service in the admin\ncluster is implemented as a Service of type NodePort (ex. 30139)."]
    pub fn set_ingress_https_node_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ingress_https_node_port = Some(v.into());
        self
    }
    #[doc = "Set the field `konnectivity_server_node_port`.\nNodePort for konnectivity server service running as a sidecar in each\nkube-apiserver pod (ex. 30564)."]
    pub fn set_konnectivity_server_node_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.konnectivity_server_node_port = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareAdminClusterLoadBalancerElManualLbConfigEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterLoadBalancerElManualLbConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterLoadBalancerElManualLbConfigEl {}
impl BuildGkeonpremVmwareAdminClusterLoadBalancerElManualLbConfigEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterLoadBalancerElManualLbConfigEl {
        GkeonpremVmwareAdminClusterLoadBalancerElManualLbConfigEl {
            addons_node_port: core::default::Default::default(),
            control_plane_node_port: core::default::Default::default(),
            ingress_http_node_port: core::default::Default::default(),
            ingress_https_node_port: core::default::Default::default(),
            konnectivity_server_node_port: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareAdminClusterLoadBalancerElManualLbConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterLoadBalancerElManualLbConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareAdminClusterLoadBalancerElManualLbConfigElRef {
        GkeonpremVmwareAdminClusterLoadBalancerElManualLbConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterLoadBalancerElManualLbConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `addons_node_port` after provisioning.\nNodePort for add-ons server in the admin cluster."]
    pub fn addons_node_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.addons_node_port", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `control_plane_node_port` after provisioning.\nNodePort for control plane service. The Kubernetes API server in the admin\ncluster is implemented as a Service of type NodePort (ex. 30968)."]
    pub fn control_plane_node_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.control_plane_node_port", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ingress_http_node_port` after provisioning.\nNodePort for ingress service's http. The ingress service in the admin\ncluster is implemented as a Service of type NodePort (ex. 32527)."]
    pub fn ingress_http_node_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ingress_http_node_port", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ingress_https_node_port` after provisioning.\nNodePort for ingress service's https. The ingress service in the admin\ncluster is implemented as a Service of type NodePort (ex. 30139)."]
    pub fn ingress_https_node_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ingress_https_node_port", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `konnectivity_server_node_port` after provisioning.\nNodePort for konnectivity server service running as a sidecar in each\nkube-apiserver pod (ex. 30564)."]
    pub fn konnectivity_server_node_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.konnectivity_server_node_port", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterLoadBalancerElMetalLbConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl GkeonpremVmwareAdminClusterLoadBalancerElMetalLbConfigEl {
    #[doc = "Set the field `enabled`.\nMetal LB is enabled."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareAdminClusterLoadBalancerElMetalLbConfigEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterLoadBalancerElMetalLbConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterLoadBalancerElMetalLbConfigEl {}
impl BuildGkeonpremVmwareAdminClusterLoadBalancerElMetalLbConfigEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterLoadBalancerElMetalLbConfigEl {
        GkeonpremVmwareAdminClusterLoadBalancerElMetalLbConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareAdminClusterLoadBalancerElMetalLbConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterLoadBalancerElMetalLbConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareAdminClusterLoadBalancerElMetalLbConfigElRef {
        GkeonpremVmwareAdminClusterLoadBalancerElMetalLbConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterLoadBalancerElMetalLbConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nMetal LB is enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterLoadBalancerElVipConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    addons_vip: Option<PrimField<String>>,
    control_plane_vip: PrimField<String>,
}
impl GkeonpremVmwareAdminClusterLoadBalancerElVipConfigEl {
    #[doc = "Set the field `addons_vip`.\nThe VIP to configure the load balancer for add-ons."]
    pub fn set_addons_vip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.addons_vip = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareAdminClusterLoadBalancerElVipConfigEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterLoadBalancerElVipConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterLoadBalancerElVipConfigEl {
    #[doc = "The VIP which you previously set aside for the Kubernetes\nAPI of this VMware Admin Cluster."]
    pub control_plane_vip: PrimField<String>,
}
impl BuildGkeonpremVmwareAdminClusterLoadBalancerElVipConfigEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterLoadBalancerElVipConfigEl {
        GkeonpremVmwareAdminClusterLoadBalancerElVipConfigEl {
            addons_vip: core::default::Default::default(),
            control_plane_vip: self.control_plane_vip,
        }
    }
}
pub struct GkeonpremVmwareAdminClusterLoadBalancerElVipConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterLoadBalancerElVipConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareAdminClusterLoadBalancerElVipConfigElRef {
        GkeonpremVmwareAdminClusterLoadBalancerElVipConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterLoadBalancerElVipConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `addons_vip` after provisioning.\nThe VIP to configure the load balancer for add-ons."]
    pub fn addons_vip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.addons_vip", self.base))
    }
    #[doc = "Get a reference to the value of field `control_plane_vip` after provisioning.\nThe VIP which you previously set aside for the Kubernetes\nAPI of this VMware Admin Cluster."]
    pub fn control_plane_vip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.control_plane_vip", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct GkeonpremVmwareAdminClusterLoadBalancerElDynamic {
    f5_config: Option<DynamicBlock<GkeonpremVmwareAdminClusterLoadBalancerElF5ConfigEl>>,
    manual_lb_config:
        Option<DynamicBlock<GkeonpremVmwareAdminClusterLoadBalancerElManualLbConfigEl>>,
    metal_lb_config: Option<DynamicBlock<GkeonpremVmwareAdminClusterLoadBalancerElMetalLbConfigEl>>,
    vip_config: Option<DynamicBlock<GkeonpremVmwareAdminClusterLoadBalancerElVipConfigEl>>,
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterLoadBalancerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    f5_config: Option<Vec<GkeonpremVmwareAdminClusterLoadBalancerElF5ConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manual_lb_config: Option<Vec<GkeonpremVmwareAdminClusterLoadBalancerElManualLbConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metal_lb_config: Option<Vec<GkeonpremVmwareAdminClusterLoadBalancerElMetalLbConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vip_config: Option<Vec<GkeonpremVmwareAdminClusterLoadBalancerElVipConfigEl>>,
    dynamic: GkeonpremVmwareAdminClusterLoadBalancerElDynamic,
}
impl GkeonpremVmwareAdminClusterLoadBalancerEl {
    #[doc = "Set the field `f5_config`.\n"]
    pub fn set_f5_config(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremVmwareAdminClusterLoadBalancerElF5ConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.f5_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.f5_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `manual_lb_config`.\n"]
    pub fn set_manual_lb_config(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremVmwareAdminClusterLoadBalancerElManualLbConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.manual_lb_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.manual_lb_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `metal_lb_config`.\n"]
    pub fn set_metal_lb_config(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremVmwareAdminClusterLoadBalancerElMetalLbConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metal_lb_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metal_lb_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `vip_config`.\n"]
    pub fn set_vip_config(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremVmwareAdminClusterLoadBalancerElVipConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vip_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vip_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeonpremVmwareAdminClusterLoadBalancerEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterLoadBalancerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterLoadBalancerEl {}
impl BuildGkeonpremVmwareAdminClusterLoadBalancerEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterLoadBalancerEl {
        GkeonpremVmwareAdminClusterLoadBalancerEl {
            f5_config: core::default::Default::default(),
            manual_lb_config: core::default::Default::default(),
            metal_lb_config: core::default::Default::default(),
            vip_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremVmwareAdminClusterLoadBalancerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterLoadBalancerElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareAdminClusterLoadBalancerElRef {
        GkeonpremVmwareAdminClusterLoadBalancerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterLoadBalancerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `f5_config` after provisioning.\n"]
    pub fn f5_config(&self) -> ListRef<GkeonpremVmwareAdminClusterLoadBalancerElF5ConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.f5_config", self.base))
    }
    #[doc = "Get a reference to the value of field `manual_lb_config` after provisioning.\n"]
    pub fn manual_lb_config(
        &self,
    ) -> ListRef<GkeonpremVmwareAdminClusterLoadBalancerElManualLbConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.manual_lb_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metal_lb_config` after provisioning.\n"]
    pub fn metal_lb_config(
        &self,
    ) -> ListRef<GkeonpremVmwareAdminClusterLoadBalancerElMetalLbConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metal_lb_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vip_config` after provisioning.\n"]
    pub fn vip_config(&self) -> ListRef<GkeonpremVmwareAdminClusterLoadBalancerElVipConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vip_config", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterNetworkConfigElDhcpIpConfigEl {
    enabled: PrimField<bool>,
}
impl GkeonpremVmwareAdminClusterNetworkConfigElDhcpIpConfigEl {}
impl ToListMappable for GkeonpremVmwareAdminClusterNetworkConfigElDhcpIpConfigEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterNetworkConfigElDhcpIpConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterNetworkConfigElDhcpIpConfigEl {
    #[doc = "enabled is a flag to mark if DHCP IP allocation is\nused for VMware admin clusters."]
    pub enabled: PrimField<bool>,
}
impl BuildGkeonpremVmwareAdminClusterNetworkConfigElDhcpIpConfigEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterNetworkConfigElDhcpIpConfigEl {
        GkeonpremVmwareAdminClusterNetworkConfigElDhcpIpConfigEl {
            enabled: self.enabled,
        }
    }
}
pub struct GkeonpremVmwareAdminClusterNetworkConfigElDhcpIpConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterNetworkConfigElDhcpIpConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareAdminClusterNetworkConfigElDhcpIpConfigElRef {
        GkeonpremVmwareAdminClusterNetworkConfigElDhcpIpConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterNetworkConfigElDhcpIpConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nenabled is a flag to mark if DHCP IP allocation is\nused for VMware admin clusters."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockElIpsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname: Option<PrimField<String>>,
    ip: PrimField<String>,
}
impl GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockElIpsEl {
    #[doc = "Set the field `hostname`.\nHostname of the machine. VM's name will be used if this field is empty."]
    pub fn set_hostname(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hostname = Some(v.into());
        self
    }
}
impl ToListMappable
    for GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockElIpsEl
{
    type O = BlockAssignable<
        GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockElIpsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockElIpsEl
{
    #[doc = "IP could be an IP address (like 1.2.3.4) or a CIDR (like 1.2.3.0/24)."]
    pub ip: PrimField<String>,
}
impl
    BuildGkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockElIpsEl
{
    pub fn build(
        self,
    ) -> GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockElIpsEl
    {
        GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockElIpsEl {
            hostname: core::default::Default::default(),
            ip: self.ip,
        }
    }
}
pub struct GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockElIpsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockElIpsElRef { fn new (shared : StackShared , base : String) -> GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockElIpsElRef { GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockElIpsElRef { shared : shared , base : base . to_string () , } } }
impl GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockElIpsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nHostname of the machine. VM's name will be used if this field is empty."]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
    #[doc = "Get a reference to the value of field `ip` after provisioning.\nIP could be an IP address (like 1.2.3.4) or a CIDR (like 1.2.3.0/24)."]
    pub fn ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockElDynamic { ips : Option < DynamicBlock < GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockElIpsEl >> , }
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockEl { gateway : PrimField < String > , netmask : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] ips : Option < Vec < GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockElIpsEl > > , dynamic : GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockElDynamic , }
impl GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockEl {
    #[doc = "Set the field `ips`.\n"]
    pub fn set_ips(
        mut self,
        v : impl Into < BlockAssignable < GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockElIpsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ips = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ips = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockEl
{
    type O = BlockAssignable<
        GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockEl
{
    #[doc = "The network gateway used by the VMware Admin Cluster."]
    pub gateway: PrimField<String>,
    #[doc = "The netmask used by the VMware Admin Cluster."]
    pub netmask: PrimField<String>,
}
impl BuildGkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockEl {
    pub fn build(
        self,
    ) -> GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockEl {
        GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockEl {
            gateway: self.gateway,
            netmask: self.netmask,
            ips: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockElRef
    {
        GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gateway` after provisioning.\nThe network gateway used by the VMware Admin Cluster."]
    pub fn gateway(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gateway", self.base))
    }
    #[doc = "Get a reference to the value of field `netmask` after provisioning.\nThe netmask used by the VMware Admin Cluster."]
    pub fn netmask(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.netmask", self.base))
    }
    #[doc = "Get a reference to the value of field `ips` after provisioning.\n"]    pub fn ips (& self) -> ListRef < GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockElIpsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.ips", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElDynamic {
    control_plane_ip_block: Option<
        DynamicBlock<
            GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    control_plane_ip_block: Option<
        Vec<GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockEl>,
    >,
    dynamic: GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElDynamic,
}
impl GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigEl {
    #[doc = "Set the field `control_plane_ip_block`.\n"]
    pub fn set_control_plane_ip_block(
        mut self,
        v : impl Into < BlockAssignable < GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.control_plane_ip_block = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.control_plane_ip_block = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigEl {}
impl BuildGkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigEl {
        GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigEl {
            control_plane_ip_block: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElRef {
        GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `control_plane_ip_block` after provisioning.\n"]
    pub fn control_plane_ip_block(
        &self,
    ) -> ListRef<
        GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElControlPlaneIpBlockElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane_ip_block", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterNetworkConfigElHostConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_search_domains: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_servers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ntp_servers: Option<ListField<PrimField<String>>>,
}
impl GkeonpremVmwareAdminClusterNetworkConfigElHostConfigEl {
    #[doc = "Set the field `dns_search_domains`.\nDNS search domains."]
    pub fn set_dns_search_domains(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.dns_search_domains = Some(v.into());
        self
    }
    #[doc = "Set the field `dns_servers`.\nDNS servers."]
    pub fn set_dns_servers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.dns_servers = Some(v.into());
        self
    }
    #[doc = "Set the field `ntp_servers`.\nNTP servers."]
    pub fn set_ntp_servers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ntp_servers = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareAdminClusterNetworkConfigElHostConfigEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterNetworkConfigElHostConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterNetworkConfigElHostConfigEl {}
impl BuildGkeonpremVmwareAdminClusterNetworkConfigElHostConfigEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterNetworkConfigElHostConfigEl {
        GkeonpremVmwareAdminClusterNetworkConfigElHostConfigEl {
            dns_search_domains: core::default::Default::default(),
            dns_servers: core::default::Default::default(),
            ntp_servers: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareAdminClusterNetworkConfigElHostConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterNetworkConfigElHostConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareAdminClusterNetworkConfigElHostConfigElRef {
        GkeonpremVmwareAdminClusterNetworkConfigElHostConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterNetworkConfigElHostConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dns_search_domains` after provisioning.\nDNS search domains."]
    pub fn dns_search_domains(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dns_search_domains", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dns_servers` after provisioning.\nDNS servers."]
    pub fn dns_servers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.dns_servers", self.base))
    }
    #[doc = "Get a reference to the value of field `ntp_servers` after provisioning.\nNTP servers."]
    pub fn ntp_servers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ntp_servers", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname: Option<PrimField<String>>,
    ip: PrimField<String>,
}
impl GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsEl {
    #[doc = "Set the field `hostname`.\nHostname of the machine. VM's name will be used if this field is empty."]
    pub fn set_hostname(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hostname = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsEl {
    type O =
        BlockAssignable<GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsEl {
    #[doc = "IP could be an IP address (like 1.2.3.4) or a CIDR (like 1.2.3.0/24)."]
    pub ip: PrimField<String>,
}
impl BuildGkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsEl {
    pub fn build(
        self,
    ) -> GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsEl {
        GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsEl {
            hostname: core::default::Default::default(),
            ip: self.ip,
        }
    }
}
pub struct GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsElRef {
        GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nHostname of the machine. VM's name will be used if this field is empty."]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
    #[doc = "Get a reference to the value of field `ip` after provisioning.\nIP could be an IP address (like 1.2.3.4) or a CIDR (like 1.2.3.0/24)."]
    pub fn ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksElDynamic {
    ips: Option<
        DynamicBlock<GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsEl>,
    >,
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksEl {
    gateway: PrimField<String>,
    netmask: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ips: Option<Vec<GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsEl>>,
    dynamic: GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksElDynamic,
}
impl GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksEl {
    #[doc = "Set the field `ips`.\n"]
    pub fn set_ips(
        mut self,
        v: impl Into<
            BlockAssignable<
                GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ips = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ips = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksEl {
    #[doc = "The network gateway used by the VMware Admin Cluster."]
    pub gateway: PrimField<String>,
    #[doc = "The netmask used by the VMware Admin Cluster."]
    pub netmask: PrimField<String>,
}
impl BuildGkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksEl {
        GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksEl {
            gateway: self.gateway,
            netmask: self.netmask,
            ips: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksElRef {
        GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gateway` after provisioning.\nThe network gateway used by the VMware Admin Cluster."]
    pub fn gateway(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gateway", self.base))
    }
    #[doc = "Get a reference to the value of field `netmask` after provisioning.\nThe netmask used by the VMware Admin Cluster."]
    pub fn netmask(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.netmask", self.base))
    }
    #[doc = "Get a reference to the value of field `ips` after provisioning.\n"]
    pub fn ips(
        &self,
    ) -> ListRef<GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ips", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElDynamic {
    ip_blocks:
        Option<DynamicBlock<GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksEl>>,
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_blocks: Option<Vec<GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksEl>>,
    dynamic: GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElDynamic,
}
impl GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigEl {
    #[doc = "Set the field `ip_blocks`.\n"]
    pub fn set_ip_blocks(
        mut self,
        v: impl Into<
            BlockAssignable<GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ip_blocks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_blocks = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigEl {}
impl BuildGkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigEl {
        GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigEl {
            ip_blocks: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElRef {
        GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_blocks` after provisioning.\n"]
    pub fn ip_blocks(
        &self,
    ) -> ListRef<GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElIpBlocksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ip_blocks", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremVmwareAdminClusterNetworkConfigElDynamic {
    dhcp_ip_config: Option<DynamicBlock<GkeonpremVmwareAdminClusterNetworkConfigElDhcpIpConfigEl>>,
    ha_control_plane_config:
        Option<DynamicBlock<GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigEl>>,
    host_config: Option<DynamicBlock<GkeonpremVmwareAdminClusterNetworkConfigElHostConfigEl>>,
    static_ip_config:
        Option<DynamicBlock<GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigEl>>,
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterNetworkConfigEl {
    pod_address_cidr_blocks: ListField<PrimField<String>>,
    service_address_cidr_blocks: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcenter_network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dhcp_ip_config: Option<Vec<GkeonpremVmwareAdminClusterNetworkConfigElDhcpIpConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ha_control_plane_config:
        Option<Vec<GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_config: Option<Vec<GkeonpremVmwareAdminClusterNetworkConfigElHostConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    static_ip_config: Option<Vec<GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigEl>>,
    dynamic: GkeonpremVmwareAdminClusterNetworkConfigElDynamic,
}
impl GkeonpremVmwareAdminClusterNetworkConfigEl {
    #[doc = "Set the field `vcenter_network`.\nvcenter_network specifies vCenter network name."]
    pub fn set_vcenter_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vcenter_network = Some(v.into());
        self
    }
    #[doc = "Set the field `dhcp_ip_config`.\n"]
    pub fn set_dhcp_ip_config(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremVmwareAdminClusterNetworkConfigElDhcpIpConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dhcp_ip_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dhcp_ip_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ha_control_plane_config`.\n"]
    pub fn set_ha_control_plane_config(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ha_control_plane_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ha_control_plane_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `host_config`.\n"]
    pub fn set_host_config(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremVmwareAdminClusterNetworkConfigElHostConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.host_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.host_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `static_ip_config`.\n"]
    pub fn set_static_ip_config(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.static_ip_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.static_ip_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeonpremVmwareAdminClusterNetworkConfigEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterNetworkConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterNetworkConfigEl {
    #[doc = "All pods in the cluster are assigned an RFC1918 IPv4 address from these ranges.\nOnly a single range is supported. This field cannot be changed after creation."]
    pub pod_address_cidr_blocks: ListField<PrimField<String>>,
    #[doc = "All services in the cluster are assigned an RFC1918 IPv4 address\nfrom these ranges. Only a single range is supported.. This field\ncannot be changed after creation."]
    pub service_address_cidr_blocks: ListField<PrimField<String>>,
}
impl BuildGkeonpremVmwareAdminClusterNetworkConfigEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterNetworkConfigEl {
        GkeonpremVmwareAdminClusterNetworkConfigEl {
            pod_address_cidr_blocks: self.pod_address_cidr_blocks,
            service_address_cidr_blocks: self.service_address_cidr_blocks,
            vcenter_network: core::default::Default::default(),
            dhcp_ip_config: core::default::Default::default(),
            ha_control_plane_config: core::default::Default::default(),
            host_config: core::default::Default::default(),
            static_ip_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremVmwareAdminClusterNetworkConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterNetworkConfigElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareAdminClusterNetworkConfigElRef {
        GkeonpremVmwareAdminClusterNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterNetworkConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pod_address_cidr_blocks` after provisioning.\nAll pods in the cluster are assigned an RFC1918 IPv4 address from these ranges.\nOnly a single range is supported. This field cannot be changed after creation."]
    pub fn pod_address_cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pod_address_cidr_blocks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_address_cidr_blocks` after provisioning.\nAll services in the cluster are assigned an RFC1918 IPv4 address\nfrom these ranges. Only a single range is supported.. This field\ncannot be changed after creation."]
    pub fn service_address_cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_address_cidr_blocks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vcenter_network` after provisioning.\nvcenter_network specifies vCenter network name."]
    pub fn vcenter_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vcenter_network", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dhcp_ip_config` after provisioning.\n"]
    pub fn dhcp_ip_config(
        &self,
    ) -> ListRef<GkeonpremVmwareAdminClusterNetworkConfigElDhcpIpConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dhcp_ip_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ha_control_plane_config` after provisioning.\n"]
    pub fn ha_control_plane_config(
        &self,
    ) -> ListRef<GkeonpremVmwareAdminClusterNetworkConfigElHaControlPlaneConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ha_control_plane_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `host_config` after provisioning.\n"]
    pub fn host_config(
        &self,
    ) -> ListRef<GkeonpremVmwareAdminClusterNetworkConfigElHostConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.host_config", self.base))
    }
    #[doc = "Get a reference to the value of field `static_ip_config` after provisioning.\n"]
    pub fn static_ip_config(
        &self,
    ) -> ListRef<GkeonpremVmwareAdminClusterNetworkConfigElStaticIpConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.static_ip_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusElConditionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    last_transition_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusElConditionsEl {
    #[doc = "Set the field `last_transition_time`.\n"]
    pub fn set_last_transition_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_transition_time = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reason = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusElConditionsEl {
    type O =
        BlockAssignable<GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusElConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusElConditionsEl {}
impl BuildGkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusElConditionsEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusElConditionsEl {
        GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusElConditionsEl {
            last_transition_time: core::default::Default::default(),
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
            state: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusElConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusElConditionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusElConditionsElRef {
        GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusElConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusElConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `last_transition_time` after provisioning.\n"]
    pub fn last_transition_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_transition_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions:
        Option<ListField<GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusElConditionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<PrimField<String>>,
}
impl GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusEl {
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl Into<
            ListField<GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusElConditionsEl>,
        >,
    ) -> Self {
        self.conditions = Some(v.into());
        self
    }
    #[doc = "Set the field `error_message`.\n"]
    pub fn set_error_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.error_message = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusEl {}
impl BuildGkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusEl {
        GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusEl {
            conditions: core::default::Default::default(),
            error_message: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusElRef {
        GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(
        &self,
    ) -> ListRef<GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusElConditionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }
    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.error_message", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterPlatformConfigElBundlesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<ListField<GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl GkeonpremVmwareAdminClusterPlatformConfigElBundlesEl {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(
        mut self,
        v: impl Into<ListField<GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusEl>>,
    ) -> Self {
        self.status = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareAdminClusterPlatformConfigElBundlesEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterPlatformConfigElBundlesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterPlatformConfigElBundlesEl {}
impl BuildGkeonpremVmwareAdminClusterPlatformConfigElBundlesEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterPlatformConfigElBundlesEl {
        GkeonpremVmwareAdminClusterPlatformConfigElBundlesEl {
            status: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareAdminClusterPlatformConfigElBundlesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterPlatformConfigElBundlesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareAdminClusterPlatformConfigElBundlesElRef {
        GkeonpremVmwareAdminClusterPlatformConfigElBundlesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterPlatformConfigElBundlesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(
        &self,
    ) -> ListRef<GkeonpremVmwareAdminClusterPlatformConfigElBundlesElStatusElRef> {
        ListRef::new(self.shared().clone(), format!("{}.status", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterPlatformConfigElStatusElConditionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    last_transition_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl GkeonpremVmwareAdminClusterPlatformConfigElStatusElConditionsEl {
    #[doc = "Set the field `last_transition_time`.\n"]
    pub fn set_last_transition_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_transition_time = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reason = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareAdminClusterPlatformConfigElStatusElConditionsEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterPlatformConfigElStatusElConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterPlatformConfigElStatusElConditionsEl {}
impl BuildGkeonpremVmwareAdminClusterPlatformConfigElStatusElConditionsEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterPlatformConfigElStatusElConditionsEl {
        GkeonpremVmwareAdminClusterPlatformConfigElStatusElConditionsEl {
            last_transition_time: core::default::Default::default(),
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
            state: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareAdminClusterPlatformConfigElStatusElConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterPlatformConfigElStatusElConditionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareAdminClusterPlatformConfigElStatusElConditionsElRef {
        GkeonpremVmwareAdminClusterPlatformConfigElStatusElConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterPlatformConfigElStatusElConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `last_transition_time` after provisioning.\n"]
    pub fn last_transition_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_transition_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterPlatformConfigElStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions: Option<ListField<GkeonpremVmwareAdminClusterPlatformConfigElStatusElConditionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<PrimField<String>>,
}
impl GkeonpremVmwareAdminClusterPlatformConfigElStatusEl {
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl Into<ListField<GkeonpremVmwareAdminClusterPlatformConfigElStatusElConditionsEl>>,
    ) -> Self {
        self.conditions = Some(v.into());
        self
    }
    #[doc = "Set the field `error_message`.\n"]
    pub fn set_error_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.error_message = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareAdminClusterPlatformConfigElStatusEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterPlatformConfigElStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterPlatformConfigElStatusEl {}
impl BuildGkeonpremVmwareAdminClusterPlatformConfigElStatusEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterPlatformConfigElStatusEl {
        GkeonpremVmwareAdminClusterPlatformConfigElStatusEl {
            conditions: core::default::Default::default(),
            error_message: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareAdminClusterPlatformConfigElStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterPlatformConfigElStatusElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareAdminClusterPlatformConfigElStatusElRef {
        GkeonpremVmwareAdminClusterPlatformConfigElStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterPlatformConfigElStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(
        &self,
    ) -> ListRef<GkeonpremVmwareAdminClusterPlatformConfigElStatusElConditionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }
    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.error_message", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterPlatformConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    required_platform_version: Option<PrimField<String>>,
}
impl GkeonpremVmwareAdminClusterPlatformConfigEl {
    #[doc = "Set the field `required_platform_version`.\nThe required platform version e.g. 1.13.1.\nIf the current platform version is lower than the target version,\nthe platform version will be updated to the target version.\nIf the target version is not installed in the platform\n(bundle versions), download the target version bundle."]
    pub fn set_required_platform_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.required_platform_version = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareAdminClusterPlatformConfigEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterPlatformConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterPlatformConfigEl {}
impl BuildGkeonpremVmwareAdminClusterPlatformConfigEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterPlatformConfigEl {
        GkeonpremVmwareAdminClusterPlatformConfigEl {
            required_platform_version: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareAdminClusterPlatformConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterPlatformConfigElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareAdminClusterPlatformConfigElRef {
        GkeonpremVmwareAdminClusterPlatformConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterPlatformConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bundles` after provisioning.\nThe list of bundles installed in the admin cluster."]
    pub fn bundles(&self) -> ListRef<GkeonpremVmwareAdminClusterPlatformConfigElBundlesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bundles", self.base))
    }
    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\nThe platform version e.g. 1.13.2."]
    pub fn platform_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.platform_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `required_platform_version` after provisioning.\nThe required platform version e.g. 1.13.1.\nIf the current platform version is lower than the target version,\nthe platform version will be updated to the target version.\nIf the target version is not installed in the platform\n(bundle versions), download the target version bundle."]
    pub fn required_platform_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.required_platform_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nResourceStatus representing detailed cluster state."]
    pub fn status(&self) -> ListRef<GkeonpremVmwareAdminClusterPlatformConfigElStatusElRef> {
        ListRef::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterPrivateRegistryConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_cert: Option<PrimField<String>>,
}
impl GkeonpremVmwareAdminClusterPrivateRegistryConfigEl {
    #[doc = "Set the field `address`.\nThe registry address."]
    pub fn set_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address = Some(v.into());
        self
    }
    #[doc = "Set the field `ca_cert`.\nThe CA certificate public key for private registry."]
    pub fn set_ca_cert(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ca_cert = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareAdminClusterPrivateRegistryConfigEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterPrivateRegistryConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterPrivateRegistryConfigEl {}
impl BuildGkeonpremVmwareAdminClusterPrivateRegistryConfigEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterPrivateRegistryConfigEl {
        GkeonpremVmwareAdminClusterPrivateRegistryConfigEl {
            address: core::default::Default::default(),
            ca_cert: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareAdminClusterPrivateRegistryConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterPrivateRegistryConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareAdminClusterPrivateRegistryConfigElRef {
        GkeonpremVmwareAdminClusterPrivateRegistryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterPrivateRegistryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address` after provisioning.\nThe registry address."]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }
    #[doc = "Get a reference to the value of field `ca_cert` after provisioning.\nThe CA certificate public key for private registry."]
    pub fn ca_cert(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ca_cert", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterProxyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    no_proxy: Option<PrimField<String>>,
    url: PrimField<String>,
}
impl GkeonpremVmwareAdminClusterProxyEl {
    #[doc = "Set the field `no_proxy`.\nA comma-separated list of IP addresses, IP address ranges,\nhost names, and domain names that should not go through the proxy server."]
    pub fn set_no_proxy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.no_proxy = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareAdminClusterProxyEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterProxyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterProxyEl {
    #[doc = "The proxy url."]
    pub url: PrimField<String>,
}
impl BuildGkeonpremVmwareAdminClusterProxyEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterProxyEl {
        GkeonpremVmwareAdminClusterProxyEl {
            no_proxy: core::default::Default::default(),
            url: self.url,
        }
    }
}
pub struct GkeonpremVmwareAdminClusterProxyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterProxyElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareAdminClusterProxyElRef {
        GkeonpremVmwareAdminClusterProxyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterProxyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `no_proxy` after provisioning.\nA comma-separated list of IP addresses, IP address ranges,\nhost names, and domain names that should not go through the proxy server."]
    pub fn no_proxy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.no_proxy", self.base))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe proxy url."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareAdminClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl GkeonpremVmwareAdminClusterTimeoutsEl {
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
impl ToListMappable for GkeonpremVmwareAdminClusterTimeoutsEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterTimeoutsEl {}
impl BuildGkeonpremVmwareAdminClusterTimeoutsEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterTimeoutsEl {
        GkeonpremVmwareAdminClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareAdminClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareAdminClusterTimeoutsElRef {
        GkeonpremVmwareAdminClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterTimeoutsElRef {
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
pub struct GkeonpremVmwareAdminClusterVcenterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_cert_data: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_disk: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    datacenter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    datastore: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    folder: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_pool: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_policy_name: Option<PrimField<String>>,
}
impl GkeonpremVmwareAdminClusterVcenterEl {
    #[doc = "Set the field `address`.\nThe vCenter IP address."]
    pub fn set_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address = Some(v.into());
        self
    }
    #[doc = "Set the field `ca_cert_data`.\nContains the vCenter CA certificate public key for SSL verification."]
    pub fn set_ca_cert_data(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ca_cert_data = Some(v.into());
        self
    }
    #[doc = "Set the field `cluster`.\nThe name of the vCenter cluster for the admin cluster."]
    pub fn set_cluster(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster = Some(v.into());
        self
    }
    #[doc = "Set the field `data_disk`.\nThe name of the virtual machine disk (VMDK) for the admin cluster."]
    pub fn set_data_disk(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_disk = Some(v.into());
        self
    }
    #[doc = "Set the field `datacenter`.\nThe name of the vCenter datacenter for the admin cluster."]
    pub fn set_datacenter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.datacenter = Some(v.into());
        self
    }
    #[doc = "Set the field `datastore`.\nThe name of the vCenter datastore for the admin cluster."]
    pub fn set_datastore(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.datastore = Some(v.into());
        self
    }
    #[doc = "Set the field `folder`.\nThe name of the vCenter folder for the admin cluster."]
    pub fn set_folder(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.folder = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_pool`.\nThe name of the vCenter resource pool for the admin cluster."]
    pub fn set_resource_pool(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_pool = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_policy_name`.\nThe name of the vCenter storage policy for the user cluster."]
    pub fn set_storage_policy_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.storage_policy_name = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareAdminClusterVcenterEl {
    type O = BlockAssignable<GkeonpremVmwareAdminClusterVcenterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareAdminClusterVcenterEl {}
impl BuildGkeonpremVmwareAdminClusterVcenterEl {
    pub fn build(self) -> GkeonpremVmwareAdminClusterVcenterEl {
        GkeonpremVmwareAdminClusterVcenterEl {
            address: core::default::Default::default(),
            ca_cert_data: core::default::Default::default(),
            cluster: core::default::Default::default(),
            data_disk: core::default::Default::default(),
            datacenter: core::default::Default::default(),
            datastore: core::default::Default::default(),
            folder: core::default::Default::default(),
            resource_pool: core::default::Default::default(),
            storage_policy_name: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareAdminClusterVcenterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareAdminClusterVcenterElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareAdminClusterVcenterElRef {
        GkeonpremVmwareAdminClusterVcenterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareAdminClusterVcenterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address` after provisioning.\nThe vCenter IP address."]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }
    #[doc = "Get a reference to the value of field `ca_cert_data` after provisioning.\nContains the vCenter CA certificate public key for SSL verification."]
    pub fn ca_cert_data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ca_cert_data", self.base))
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\nThe name of the vCenter cluster for the admin cluster."]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }
    #[doc = "Get a reference to the value of field `data_disk` after provisioning.\nThe name of the virtual machine disk (VMDK) for the admin cluster."]
    pub fn data_disk(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_disk", self.base))
    }
    #[doc = "Get a reference to the value of field `datacenter` after provisioning.\nThe name of the vCenter datacenter for the admin cluster."]
    pub fn datacenter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.datacenter", self.base))
    }
    #[doc = "Get a reference to the value of field `datastore` after provisioning.\nThe name of the vCenter datastore for the admin cluster."]
    pub fn datastore(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.datastore", self.base))
    }
    #[doc = "Get a reference to the value of field `folder` after provisioning.\nThe name of the vCenter folder for the admin cluster."]
    pub fn folder(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.folder", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_pool` after provisioning.\nThe name of the vCenter resource pool for the admin cluster."]
    pub fn resource_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_pool", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `storage_policy_name` after provisioning.\nThe name of the vCenter storage policy for the user cluster."]
    pub fn storage_policy_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_policy_name", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct GkeonpremVmwareAdminClusterDynamic {
    addon_node: Option<DynamicBlock<GkeonpremVmwareAdminClusterAddonNodeEl>>,
    anti_affinity_groups: Option<DynamicBlock<GkeonpremVmwareAdminClusterAntiAffinityGroupsEl>>,
    authorization: Option<DynamicBlock<GkeonpremVmwareAdminClusterAuthorizationEl>>,
    auto_repair_config: Option<DynamicBlock<GkeonpremVmwareAdminClusterAutoRepairConfigEl>>,
    control_plane_node: Option<DynamicBlock<GkeonpremVmwareAdminClusterControlPlaneNodeEl>>,
    load_balancer: Option<DynamicBlock<GkeonpremVmwareAdminClusterLoadBalancerEl>>,
    network_config: Option<DynamicBlock<GkeonpremVmwareAdminClusterNetworkConfigEl>>,
    platform_config: Option<DynamicBlock<GkeonpremVmwareAdminClusterPlatformConfigEl>>,
    private_registry_config:
        Option<DynamicBlock<GkeonpremVmwareAdminClusterPrivateRegistryConfigEl>>,
    proxy: Option<DynamicBlock<GkeonpremVmwareAdminClusterProxyEl>>,
    vcenter: Option<DynamicBlock<GkeonpremVmwareAdminClusterVcenterEl>>,
}
