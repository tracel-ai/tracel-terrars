use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct GkeonpremVmwareClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    admin_cluster_membership: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_bundled_ingress: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_advanced_cluster: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_control_plane_v2: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    name: PrimField<String>,
    on_prem_version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_validations: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vm_tracking_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    anti_affinity_groups: Option<Vec<GkeonpremVmwareClusterAntiAffinityGroupsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization: Option<Vec<GkeonpremVmwareClusterAuthorizationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_repair_config: Option<Vec<GkeonpremVmwareClusterAutoRepairConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    control_plane_node: Option<Vec<GkeonpremVmwareClusterControlPlaneNodeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dataplane_v2: Option<Vec<GkeonpremVmwareClusterDataplaneV2El>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancer: Option<Vec<GkeonpremVmwareClusterLoadBalancerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_config: Option<Vec<GkeonpremVmwareClusterNetworkConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage: Option<Vec<GkeonpremVmwareClusterStorageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<GkeonpremVmwareClusterTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upgrade_policy: Option<Vec<GkeonpremVmwareClusterUpgradePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcenter: Option<Vec<GkeonpremVmwareClusterVcenterEl>>,
    dynamic: GkeonpremVmwareClusterDynamic,
}
struct GkeonpremVmwareCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GkeonpremVmwareClusterData>,
}
#[derive(Clone)]
pub struct GkeonpremVmwareCluster(Rc<GkeonpremVmwareCluster_>);
impl GkeonpremVmwareCluster {
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
    #[doc = "Set the field `annotations`.\nAnnotations on the VMware User Cluster.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nA human readable description of this VMware User Cluster."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_bundled_ingress`.\nDisable bundled ingress."]
    pub fn set_disable_bundled_ingress(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_bundled_ingress = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_advanced_cluster`.\nEnable advanced cluster. Default to false."]
    pub fn set_enable_advanced_cluster(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_advanced_cluster = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_control_plane_v2`.\nEnable control plane V2. Default to false."]
    pub fn set_enable_control_plane_v2(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_control_plane_v2 = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `skip_validations`.\nA list of validations to skip during preflight checks. Possible values: [\"VALIDATION_SKIP_UNSPECIFIED\", \"ALL\", \"WORKSTATION\", \"CONFIG\", \"DOCKER\", \"INFRA\", \"LOAD_BALANCER\", \"VIPS\", \"NODE_IPS\", \"DNS\", \"TOD\", \"NET_CONFIG\", \"STORAGE_DRIVER\", \"PROXY\", \"INTERNET\", \"GCP\", \"GKEHUB\", \"RESERVED_IPS\", \"STACKDRIVER\", \"NODEPOOL_AUTOSCALING\", \"OS_IMAGES\", \"CLUSTER_VERSION\", \"CLUSTER_HEALTH\", \"WINDOWS\", \"HSM_SECRET_ENCRYPTION\", \"BACKUP_ADMIN\", \"CONNECTIVITY\", \"CLUSTER_SECRETS_CONFIG\", \"CSI_WORKLOAD\", \"VSPHERE_VERSION\", \"MIGRATION\"]"]
    pub fn set_skip_validations(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().skip_validations = Some(v.into());
        self
    }
    #[doc = "Set the field `vm_tracking_enabled`.\nEnable VM tracking."]
    pub fn set_vm_tracking_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().vm_tracking_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `anti_affinity_groups`.\n"]
    pub fn set_anti_affinity_groups(
        self,
        v: impl Into<BlockAssignable<GkeonpremVmwareClusterAntiAffinityGroupsEl>>,
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
        v: impl Into<BlockAssignable<GkeonpremVmwareClusterAuthorizationEl>>,
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
        v: impl Into<BlockAssignable<GkeonpremVmwareClusterAutoRepairConfigEl>>,
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
        v: impl Into<BlockAssignable<GkeonpremVmwareClusterControlPlaneNodeEl>>,
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
    #[doc = "Set the field `dataplane_v2`.\n"]
    pub fn set_dataplane_v2(
        self,
        v: impl Into<BlockAssignable<GkeonpremVmwareClusterDataplaneV2El>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dataplane_v2 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dataplane_v2 = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `load_balancer`.\n"]
    pub fn set_load_balancer(
        self,
        v: impl Into<BlockAssignable<GkeonpremVmwareClusterLoadBalancerEl>>,
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
        v: impl Into<BlockAssignable<GkeonpremVmwareClusterNetworkConfigEl>>,
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
    #[doc = "Set the field `storage`.\n"]
    pub fn set_storage(
        self,
        v: impl Into<BlockAssignable<GkeonpremVmwareClusterStorageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().storage = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.storage = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<GkeonpremVmwareClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `upgrade_policy`.\n"]
    pub fn set_upgrade_policy(
        self,
        v: impl Into<BlockAssignable<GkeonpremVmwareClusterUpgradePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().upgrade_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.upgrade_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `vcenter`.\n"]
    pub fn set_vcenter(
        self,
        v: impl Into<BlockAssignable<GkeonpremVmwareClusterVcenterEl>>,
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
    #[doc = "Get a reference to the value of field `admin_cluster_membership` after provisioning.\nThe admin cluster this VMware User Cluster belongs to.\nThis is the full resource name of the admin cluster's hub membership.\nIn the future, references to other resource types might be allowed if\nadmin clusters are modeled as their own resources."]
    pub fn admin_cluster_membership(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_cluster_membership", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nAnnotations on the VMware User Cluster.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time at which VMware User Cluster was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nThe time at which VMware User Cluster was deleted."]
    pub fn delete_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human readable description of this VMware User Cluster."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disable_bundled_ingress` after provisioning.\nDisable bundled ingress."]
    pub fn disable_bundled_ingress(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_bundled_ingress", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_advanced_cluster` after provisioning.\nEnable advanced cluster. Default to false."]
    pub fn enable_advanced_cluster(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_advanced_cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_control_plane_v2` after provisioning.\nEnable control plane V2. Default to false."]
    pub fn enable_control_plane_v2(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_control_plane_v2", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nThe DNS name of VMware User Cluster's API server."]
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
    pub fn fleet(&self) -> ListRef<GkeonpremVmwareClusterFleetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fleet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `local_name` after provisioning.\nThe object name of the VMware OnPremUserCluster custom resource on the\nassociated admin cluster. This field is used to support conflicting\nnames when enrolling existing clusters to the API. When used as a part of\ncluster enrollment, this field will differ from the ID in the resource\nname. For new clusters, this field will match the user provided cluster ID\nand be visible in the last component of the resource name. It is not\nmodifiable.\n\nAll users should use this name to access their cluster using gkectl or\nkubectl and should expect to see the local name when viewing admin\ncluster controller logs."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe VMware cluster name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `on_prem_version` after provisioning.\nThe Anthos clusters on the VMware version for your user cluster."]
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
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the VMware User Cluster."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `skip_validations` after provisioning.\nA list of validations to skip during preflight checks. Possible values: [\"VALIDATION_SKIP_UNSPECIFIED\", \"ALL\", \"WORKSTATION\", \"CONFIG\", \"DOCKER\", \"INFRA\", \"LOAD_BALANCER\", \"VIPS\", \"NODE_IPS\", \"DNS\", \"TOD\", \"NET_CONFIG\", \"STORAGE_DRIVER\", \"PROXY\", \"INTERNET\", \"GCP\", \"GKEHUB\", \"RESERVED_IPS\", \"STACKDRIVER\", \"NODEPOOL_AUTOSCALING\", \"OS_IMAGES\", \"CLUSTER_VERSION\", \"CLUSTER_HEALTH\", \"WINDOWS\", \"HSM_SECRET_ENCRYPTION\", \"BACKUP_ADMIN\", \"CONNECTIVITY\", \"CLUSTER_SECRETS_CONFIG\", \"CSI_WORKLOAD\", \"VSPHERE_VERSION\", \"MIGRATION\"]"]
    pub fn skip_validations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.skip_validations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of this cluster."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nResourceStatus representing detailed cluster state."]
    pub fn status(&self) -> ListRef<GkeonpremVmwareClusterStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe unique identifier of the VMware User Cluster."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time at which VMware User Cluster was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `validation_check` after provisioning.\nValidationCheck represents the result of the preflight check job."]
    pub fn validation_check(&self) -> ListRef<GkeonpremVmwareClusterValidationCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.validation_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vm_tracking_enabled` after provisioning.\nEnable VM tracking."]
    pub fn vm_tracking_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vm_tracking_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `anti_affinity_groups` after provisioning.\n"]
    pub fn anti_affinity_groups(&self) -> ListRef<GkeonpremVmwareClusterAntiAffinityGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.anti_affinity_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> ListRef<GkeonpremVmwareClusterAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auto_repair_config` after provisioning.\n"]
    pub fn auto_repair_config(&self) -> ListRef<GkeonpremVmwareClusterAutoRepairConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_repair_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `control_plane_node` after provisioning.\n"]
    pub fn control_plane_node(&self) -> ListRef<GkeonpremVmwareClusterControlPlaneNodeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane_node", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataplane_v2` after provisioning.\n"]
    pub fn dataplane_v2(&self) -> ListRef<GkeonpremVmwareClusterDataplaneV2ElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dataplane_v2", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancer` after provisioning.\n"]
    pub fn load_balancer(&self) -> ListRef<GkeonpremVmwareClusterLoadBalancerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.load_balancer", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<GkeonpremVmwareClusterNetworkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage` after provisioning.\n"]
    pub fn storage(&self) -> ListRef<GkeonpremVmwareClusterStorageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeonpremVmwareClusterTimeoutsElRef {
        GkeonpremVmwareClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `upgrade_policy` after provisioning.\n"]
    pub fn upgrade_policy(&self) -> ListRef<GkeonpremVmwareClusterUpgradePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.upgrade_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vcenter` after provisioning.\n"]
    pub fn vcenter(&self) -> ListRef<GkeonpremVmwareClusterVcenterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vcenter", self.extract_ref()),
        )
    }
}
impl Referable for GkeonpremVmwareCluster {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for GkeonpremVmwareCluster {}
impl ToListMappable for GkeonpremVmwareCluster {
    type O = ListRef<GkeonpremVmwareClusterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for GkeonpremVmwareCluster_ {
    fn extract_resource_type(&self) -> String {
        "google_gkeonprem_vmware_cluster".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildGkeonpremVmwareCluster {
    pub tf_id: String,
    #[doc = "The admin cluster this VMware User Cluster belongs to.\nThis is the full resource name of the admin cluster's hub membership.\nIn the future, references to other resource types might be allowed if\nadmin clusters are modeled as their own resources."]
    pub admin_cluster_membership: PrimField<String>,
    #[doc = "The location of the resource."]
    pub location: PrimField<String>,
    #[doc = "The VMware cluster name."]
    pub name: PrimField<String>,
    #[doc = "The Anthos clusters on the VMware version for your user cluster."]
    pub on_prem_version: PrimField<String>,
}
impl BuildGkeonpremVmwareCluster {
    pub fn build(self, stack: &mut Stack) -> GkeonpremVmwareCluster {
        let out = GkeonpremVmwareCluster(Rc::new(GkeonpremVmwareCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GkeonpremVmwareClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                admin_cluster_membership: self.admin_cluster_membership,
                annotations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                disable_bundled_ingress: core::default::Default::default(),
                enable_advanced_cluster: core::default::Default::default(),
                enable_control_plane_v2: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                on_prem_version: self.on_prem_version,
                project: core::default::Default::default(),
                skip_validations: core::default::Default::default(),
                vm_tracking_enabled: core::default::Default::default(),
                anti_affinity_groups: core::default::Default::default(),
                authorization: core::default::Default::default(),
                auto_repair_config: core::default::Default::default(),
                control_plane_node: core::default::Default::default(),
                dataplane_v2: core::default::Default::default(),
                load_balancer: core::default::Default::default(),
                network_config: core::default::Default::default(),
                storage: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                upgrade_policy: core::default::Default::default(),
                vcenter: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct GkeonpremVmwareClusterRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl GkeonpremVmwareClusterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `admin_cluster_membership` after provisioning.\nThe admin cluster this VMware User Cluster belongs to.\nThis is the full resource name of the admin cluster's hub membership.\nIn the future, references to other resource types might be allowed if\nadmin clusters are modeled as their own resources."]
    pub fn admin_cluster_membership(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_cluster_membership", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nAnnotations on the VMware User Cluster.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time at which VMware User Cluster was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nThe time at which VMware User Cluster was deleted."]
    pub fn delete_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human readable description of this VMware User Cluster."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disable_bundled_ingress` after provisioning.\nDisable bundled ingress."]
    pub fn disable_bundled_ingress(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_bundled_ingress", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_advanced_cluster` after provisioning.\nEnable advanced cluster. Default to false."]
    pub fn enable_advanced_cluster(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_advanced_cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_control_plane_v2` after provisioning.\nEnable control plane V2. Default to false."]
    pub fn enable_control_plane_v2(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_control_plane_v2", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nThe DNS name of VMware User Cluster's API server."]
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
    pub fn fleet(&self) -> ListRef<GkeonpremVmwareClusterFleetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fleet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `local_name` after provisioning.\nThe object name of the VMware OnPremUserCluster custom resource on the\nassociated admin cluster. This field is used to support conflicting\nnames when enrolling existing clusters to the API. When used as a part of\ncluster enrollment, this field will differ from the ID in the resource\nname. For new clusters, this field will match the user provided cluster ID\nand be visible in the last component of the resource name. It is not\nmodifiable.\n\nAll users should use this name to access their cluster using gkectl or\nkubectl and should expect to see the local name when viewing admin\ncluster controller logs."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe VMware cluster name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `on_prem_version` after provisioning.\nThe Anthos clusters on the VMware version for your user cluster."]
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
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the VMware User Cluster."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `skip_validations` after provisioning.\nA list of validations to skip during preflight checks. Possible values: [\"VALIDATION_SKIP_UNSPECIFIED\", \"ALL\", \"WORKSTATION\", \"CONFIG\", \"DOCKER\", \"INFRA\", \"LOAD_BALANCER\", \"VIPS\", \"NODE_IPS\", \"DNS\", \"TOD\", \"NET_CONFIG\", \"STORAGE_DRIVER\", \"PROXY\", \"INTERNET\", \"GCP\", \"GKEHUB\", \"RESERVED_IPS\", \"STACKDRIVER\", \"NODEPOOL_AUTOSCALING\", \"OS_IMAGES\", \"CLUSTER_VERSION\", \"CLUSTER_HEALTH\", \"WINDOWS\", \"HSM_SECRET_ENCRYPTION\", \"BACKUP_ADMIN\", \"CONNECTIVITY\", \"CLUSTER_SECRETS_CONFIG\", \"CSI_WORKLOAD\", \"VSPHERE_VERSION\", \"MIGRATION\"]"]
    pub fn skip_validations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.skip_validations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of this cluster."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nResourceStatus representing detailed cluster state."]
    pub fn status(&self) -> ListRef<GkeonpremVmwareClusterStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe unique identifier of the VMware User Cluster."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time at which VMware User Cluster was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `validation_check` after provisioning.\nValidationCheck represents the result of the preflight check job."]
    pub fn validation_check(&self) -> ListRef<GkeonpremVmwareClusterValidationCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.validation_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vm_tracking_enabled` after provisioning.\nEnable VM tracking."]
    pub fn vm_tracking_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vm_tracking_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `anti_affinity_groups` after provisioning.\n"]
    pub fn anti_affinity_groups(&self) -> ListRef<GkeonpremVmwareClusterAntiAffinityGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.anti_affinity_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> ListRef<GkeonpremVmwareClusterAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auto_repair_config` after provisioning.\n"]
    pub fn auto_repair_config(&self) -> ListRef<GkeonpremVmwareClusterAutoRepairConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_repair_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `control_plane_node` after provisioning.\n"]
    pub fn control_plane_node(&self) -> ListRef<GkeonpremVmwareClusterControlPlaneNodeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane_node", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataplane_v2` after provisioning.\n"]
    pub fn dataplane_v2(&self) -> ListRef<GkeonpremVmwareClusterDataplaneV2ElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dataplane_v2", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancer` after provisioning.\n"]
    pub fn load_balancer(&self) -> ListRef<GkeonpremVmwareClusterLoadBalancerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.load_balancer", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<GkeonpremVmwareClusterNetworkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage` after provisioning.\n"]
    pub fn storage(&self) -> ListRef<GkeonpremVmwareClusterStorageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeonpremVmwareClusterTimeoutsElRef {
        GkeonpremVmwareClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `upgrade_policy` after provisioning.\n"]
    pub fn upgrade_policy(&self) -> ListRef<GkeonpremVmwareClusterUpgradePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.upgrade_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vcenter` after provisioning.\n"]
    pub fn vcenter(&self) -> ListRef<GkeonpremVmwareClusterVcenterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vcenter", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareClusterFleetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    membership: Option<PrimField<String>>,
}
impl GkeonpremVmwareClusterFleetEl {
    #[doc = "Set the field `membership`.\n"]
    pub fn set_membership(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.membership = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareClusterFleetEl {
    type O = BlockAssignable<GkeonpremVmwareClusterFleetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterFleetEl {}
impl BuildGkeonpremVmwareClusterFleetEl {
    pub fn build(self) -> GkeonpremVmwareClusterFleetEl {
        GkeonpremVmwareClusterFleetEl {
            membership: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareClusterFleetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterFleetElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareClusterFleetElRef {
        GkeonpremVmwareClusterFleetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterFleetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `membership` after provisioning.\n"]
    pub fn membership(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.membership", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareClusterStatusElConditionsEl {
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
impl GkeonpremVmwareClusterStatusElConditionsEl {
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
impl ToListMappable for GkeonpremVmwareClusterStatusElConditionsEl {
    type O = BlockAssignable<GkeonpremVmwareClusterStatusElConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterStatusElConditionsEl {}
impl BuildGkeonpremVmwareClusterStatusElConditionsEl {
    pub fn build(self) -> GkeonpremVmwareClusterStatusElConditionsEl {
        GkeonpremVmwareClusterStatusElConditionsEl {
            last_transition_time: core::default::Default::default(),
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
            state: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareClusterStatusElConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterStatusElConditionsElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareClusterStatusElConditionsElRef {
        GkeonpremVmwareClusterStatusElConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterStatusElConditionsElRef {
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
pub struct GkeonpremVmwareClusterStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions: Option<ListField<GkeonpremVmwareClusterStatusElConditionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<PrimField<String>>,
}
impl GkeonpremVmwareClusterStatusEl {
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl Into<ListField<GkeonpremVmwareClusterStatusElConditionsEl>>,
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
impl ToListMappable for GkeonpremVmwareClusterStatusEl {
    type O = BlockAssignable<GkeonpremVmwareClusterStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterStatusEl {}
impl BuildGkeonpremVmwareClusterStatusEl {
    pub fn build(self) -> GkeonpremVmwareClusterStatusEl {
        GkeonpremVmwareClusterStatusEl {
            conditions: core::default::Default::default(),
            error_message: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareClusterStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterStatusElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareClusterStatusElRef {
        GkeonpremVmwareClusterStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> ListRef<GkeonpremVmwareClusterStatusElConditionsElRef> {
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
pub struct GkeonpremVmwareClusterValidationCheckElStatusElResultEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<PrimField<String>>,
}
impl GkeonpremVmwareClusterValidationCheckElStatusElResultEl {
    #[doc = "Set the field `category`.\n"]
    pub fn set_category(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.category = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.details = Some(v.into());
        self
    }
    #[doc = "Set the field `options`.\n"]
    pub fn set_options(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.options = Some(v.into());
        self
    }
    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reason = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareClusterValidationCheckElStatusElResultEl {
    type O = BlockAssignable<GkeonpremVmwareClusterValidationCheckElStatusElResultEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterValidationCheckElStatusElResultEl {}
impl BuildGkeonpremVmwareClusterValidationCheckElStatusElResultEl {
    pub fn build(self) -> GkeonpremVmwareClusterValidationCheckElStatusElResultEl {
        GkeonpremVmwareClusterValidationCheckElStatusElResultEl {
            category: core::default::Default::default(),
            description: core::default::Default::default(),
            details: core::default::Default::default(),
            options: core::default::Default::default(),
            reason: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareClusterValidationCheckElStatusElResultElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterValidationCheckElStatusElResultElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareClusterValidationCheckElStatusElResultElRef {
        GkeonpremVmwareClusterValidationCheckElStatusElResultElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterValidationCheckElStatusElResultElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.details", self.base))
    }
    #[doc = "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.options", self.base))
    }
    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareClusterValidationCheckElStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<ListField<GkeonpremVmwareClusterValidationCheckElStatusElResultEl>>,
}
impl GkeonpremVmwareClusterValidationCheckElStatusEl {
    #[doc = "Set the field `result`.\n"]
    pub fn set_result(
        mut self,
        v: impl Into<ListField<GkeonpremVmwareClusterValidationCheckElStatusElResultEl>>,
    ) -> Self {
        self.result = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareClusterValidationCheckElStatusEl {
    type O = BlockAssignable<GkeonpremVmwareClusterValidationCheckElStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterValidationCheckElStatusEl {}
impl BuildGkeonpremVmwareClusterValidationCheckElStatusEl {
    pub fn build(self) -> GkeonpremVmwareClusterValidationCheckElStatusEl {
        GkeonpremVmwareClusterValidationCheckElStatusEl {
            result: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareClusterValidationCheckElStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterValidationCheckElStatusElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareClusterValidationCheckElStatusElRef {
        GkeonpremVmwareClusterValidationCheckElStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterValidationCheckElStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `result` after provisioning.\n"]
    pub fn result(&self) -> ListRef<GkeonpremVmwareClusterValidationCheckElStatusElResultElRef> {
        ListRef::new(self.shared().clone(), format!("{}.result", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareClusterValidationCheckEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scenario: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<ListField<GkeonpremVmwareClusterValidationCheckElStatusEl>>,
}
impl GkeonpremVmwareClusterValidationCheckEl {
    #[doc = "Set the field `options`.\n"]
    pub fn set_options(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.options = Some(v.into());
        self
    }
    #[doc = "Set the field `scenario`.\n"]
    pub fn set_scenario(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scenario = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(
        mut self,
        v: impl Into<ListField<GkeonpremVmwareClusterValidationCheckElStatusEl>>,
    ) -> Self {
        self.status = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareClusterValidationCheckEl {
    type O = BlockAssignable<GkeonpremVmwareClusterValidationCheckEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterValidationCheckEl {}
impl BuildGkeonpremVmwareClusterValidationCheckEl {
    pub fn build(self) -> GkeonpremVmwareClusterValidationCheckEl {
        GkeonpremVmwareClusterValidationCheckEl {
            options: core::default::Default::default(),
            scenario: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareClusterValidationCheckElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterValidationCheckElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareClusterValidationCheckElRef {
        GkeonpremVmwareClusterValidationCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterValidationCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.options", self.base))
    }
    #[doc = "Get a reference to the value of field `scenario` after provisioning.\n"]
    pub fn scenario(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scenario", self.base))
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> ListRef<GkeonpremVmwareClusterValidationCheckElStatusElRef> {
        ListRef::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareClusterAntiAffinityGroupsEl {
    aag_config_disabled: PrimField<bool>,
}
impl GkeonpremVmwareClusterAntiAffinityGroupsEl {}
impl ToListMappable for GkeonpremVmwareClusterAntiAffinityGroupsEl {
    type O = BlockAssignable<GkeonpremVmwareClusterAntiAffinityGroupsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterAntiAffinityGroupsEl {
    #[doc = "Spread nodes across at least three physical hosts (requires at least three\nhosts).\nEnabled by default."]
    pub aag_config_disabled: PrimField<bool>,
}
impl BuildGkeonpremVmwareClusterAntiAffinityGroupsEl {
    pub fn build(self) -> GkeonpremVmwareClusterAntiAffinityGroupsEl {
        GkeonpremVmwareClusterAntiAffinityGroupsEl {
            aag_config_disabled: self.aag_config_disabled,
        }
    }
}
pub struct GkeonpremVmwareClusterAntiAffinityGroupsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterAntiAffinityGroupsElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareClusterAntiAffinityGroupsElRef {
        GkeonpremVmwareClusterAntiAffinityGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterAntiAffinityGroupsElRef {
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
pub struct GkeonpremVmwareClusterAuthorizationElAdminUsersEl {
    username: PrimField<String>,
}
impl GkeonpremVmwareClusterAuthorizationElAdminUsersEl {}
impl ToListMappable for GkeonpremVmwareClusterAuthorizationElAdminUsersEl {
    type O = BlockAssignable<GkeonpremVmwareClusterAuthorizationElAdminUsersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterAuthorizationElAdminUsersEl {
    #[doc = "The name of the user, e.g. 'my-gcp-id@gmail.com'."]
    pub username: PrimField<String>,
}
impl BuildGkeonpremVmwareClusterAuthorizationElAdminUsersEl {
    pub fn build(self) -> GkeonpremVmwareClusterAuthorizationElAdminUsersEl {
        GkeonpremVmwareClusterAuthorizationElAdminUsersEl {
            username: self.username,
        }
    }
}
pub struct GkeonpremVmwareClusterAuthorizationElAdminUsersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterAuthorizationElAdminUsersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareClusterAuthorizationElAdminUsersElRef {
        GkeonpremVmwareClusterAuthorizationElAdminUsersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterAuthorizationElAdminUsersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nThe name of the user, e.g. 'my-gcp-id@gmail.com'."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremVmwareClusterAuthorizationElDynamic {
    admin_users: Option<DynamicBlock<GkeonpremVmwareClusterAuthorizationElAdminUsersEl>>,
}
#[derive(Serialize)]
pub struct GkeonpremVmwareClusterAuthorizationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_users: Option<Vec<GkeonpremVmwareClusterAuthorizationElAdminUsersEl>>,
    dynamic: GkeonpremVmwareClusterAuthorizationElDynamic,
}
impl GkeonpremVmwareClusterAuthorizationEl {
    #[doc = "Set the field `admin_users`.\n"]
    pub fn set_admin_users(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremVmwareClusterAuthorizationElAdminUsersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.admin_users = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.admin_users = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeonpremVmwareClusterAuthorizationEl {
    type O = BlockAssignable<GkeonpremVmwareClusterAuthorizationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterAuthorizationEl {}
impl BuildGkeonpremVmwareClusterAuthorizationEl {
    pub fn build(self) -> GkeonpremVmwareClusterAuthorizationEl {
        GkeonpremVmwareClusterAuthorizationEl {
            admin_users: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremVmwareClusterAuthorizationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterAuthorizationElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareClusterAuthorizationElRef {
        GkeonpremVmwareClusterAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterAuthorizationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `admin_users` after provisioning.\n"]
    pub fn admin_users(&self) -> ListRef<GkeonpremVmwareClusterAuthorizationElAdminUsersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.admin_users", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareClusterAutoRepairConfigEl {
    enabled: PrimField<bool>,
}
impl GkeonpremVmwareClusterAutoRepairConfigEl {}
impl ToListMappable for GkeonpremVmwareClusterAutoRepairConfigEl {
    type O = BlockAssignable<GkeonpremVmwareClusterAutoRepairConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterAutoRepairConfigEl {
    #[doc = "Whether auto repair is enabled."]
    pub enabled: PrimField<bool>,
}
impl BuildGkeonpremVmwareClusterAutoRepairConfigEl {
    pub fn build(self) -> GkeonpremVmwareClusterAutoRepairConfigEl {
        GkeonpremVmwareClusterAutoRepairConfigEl {
            enabled: self.enabled,
        }
    }
}
pub struct GkeonpremVmwareClusterAutoRepairConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterAutoRepairConfigElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareClusterAutoRepairConfigElRef {
        GkeonpremVmwareClusterAutoRepairConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterAutoRepairConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether auto repair is enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareClusterControlPlaneNodeElVsphereConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    datastore: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_policy_name: Option<PrimField<String>>,
}
impl GkeonpremVmwareClusterControlPlaneNodeElVsphereConfigEl {
    #[doc = "Set the field `datastore`.\n"]
    pub fn set_datastore(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.datastore = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_policy_name`.\n"]
    pub fn set_storage_policy_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.storage_policy_name = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareClusterControlPlaneNodeElVsphereConfigEl {
    type O = BlockAssignable<GkeonpremVmwareClusterControlPlaneNodeElVsphereConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterControlPlaneNodeElVsphereConfigEl {}
impl BuildGkeonpremVmwareClusterControlPlaneNodeElVsphereConfigEl {
    pub fn build(self) -> GkeonpremVmwareClusterControlPlaneNodeElVsphereConfigEl {
        GkeonpremVmwareClusterControlPlaneNodeElVsphereConfigEl {
            datastore: core::default::Default::default(),
            storage_policy_name: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareClusterControlPlaneNodeElVsphereConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterControlPlaneNodeElVsphereConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareClusterControlPlaneNodeElVsphereConfigElRef {
        GkeonpremVmwareClusterControlPlaneNodeElVsphereConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterControlPlaneNodeElVsphereConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `datastore` after provisioning.\n"]
    pub fn datastore(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.datastore", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_policy_name` after provisioning.\n"]
    pub fn storage_policy_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_policy_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareClusterControlPlaneNodeElAutoResizeConfigEl {
    enabled: PrimField<bool>,
}
impl GkeonpremVmwareClusterControlPlaneNodeElAutoResizeConfigEl {}
impl ToListMappable for GkeonpremVmwareClusterControlPlaneNodeElAutoResizeConfigEl {
    type O = BlockAssignable<GkeonpremVmwareClusterControlPlaneNodeElAutoResizeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterControlPlaneNodeElAutoResizeConfigEl {
    #[doc = "Whether to enable control plane node auto resizing."]
    pub enabled: PrimField<bool>,
}
impl BuildGkeonpremVmwareClusterControlPlaneNodeElAutoResizeConfigEl {
    pub fn build(self) -> GkeonpremVmwareClusterControlPlaneNodeElAutoResizeConfigEl {
        GkeonpremVmwareClusterControlPlaneNodeElAutoResizeConfigEl {
            enabled: self.enabled,
        }
    }
}
pub struct GkeonpremVmwareClusterControlPlaneNodeElAutoResizeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterControlPlaneNodeElAutoResizeConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareClusterControlPlaneNodeElAutoResizeConfigElRef {
        GkeonpremVmwareClusterControlPlaneNodeElAutoResizeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterControlPlaneNodeElAutoResizeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether to enable control plane node auto resizing."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremVmwareClusterControlPlaneNodeElDynamic {
    auto_resize_config:
        Option<DynamicBlock<GkeonpremVmwareClusterControlPlaneNodeElAutoResizeConfigEl>>,
}
#[derive(Serialize)]
pub struct GkeonpremVmwareClusterControlPlaneNodeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpus: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replicas: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_resize_config: Option<Vec<GkeonpremVmwareClusterControlPlaneNodeElAutoResizeConfigEl>>,
    dynamic: GkeonpremVmwareClusterControlPlaneNodeElDynamic,
}
impl GkeonpremVmwareClusterControlPlaneNodeEl {
    #[doc = "Set the field `cpus`.\nThe number of CPUs for each admin cluster node that serve as control planes\nfor this VMware User Cluster. (default: 4 CPUs)"]
    pub fn set_cpus(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpus = Some(v.into());
        self
    }
    #[doc = "Set the field `memory`.\nThe megabytes of memory for each admin cluster node that serves as a\ncontrol plane for this VMware User Cluster (default: 8192 MB memory)."]
    pub fn set_memory(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory = Some(v.into());
        self
    }
    #[doc = "Set the field `replicas`.\nThe number of control plane nodes for this VMware User Cluster.\n(default: 1 replica)."]
    pub fn set_replicas(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.replicas = Some(v.into());
        self
    }
    #[doc = "Set the field `auto_resize_config`.\n"]
    pub fn set_auto_resize_config(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremVmwareClusterControlPlaneNodeElAutoResizeConfigEl>>,
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
impl ToListMappable for GkeonpremVmwareClusterControlPlaneNodeEl {
    type O = BlockAssignable<GkeonpremVmwareClusterControlPlaneNodeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterControlPlaneNodeEl {}
impl BuildGkeonpremVmwareClusterControlPlaneNodeEl {
    pub fn build(self) -> GkeonpremVmwareClusterControlPlaneNodeEl {
        GkeonpremVmwareClusterControlPlaneNodeEl {
            cpus: core::default::Default::default(),
            memory: core::default::Default::default(),
            replicas: core::default::Default::default(),
            auto_resize_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremVmwareClusterControlPlaneNodeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterControlPlaneNodeElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareClusterControlPlaneNodeElRef {
        GkeonpremVmwareClusterControlPlaneNodeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterControlPlaneNodeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cpus` after provisioning.\nThe number of CPUs for each admin cluster node that serve as control planes\nfor this VMware User Cluster. (default: 4 CPUs)"]
    pub fn cpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpus", self.base))
    }
    #[doc = "Get a reference to the value of field `memory` after provisioning.\nThe megabytes of memory for each admin cluster node that serves as a\ncontrol plane for this VMware User Cluster (default: 8192 MB memory)."]
    pub fn memory(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
    #[doc = "Get a reference to the value of field `replicas` after provisioning.\nThe number of control plane nodes for this VMware User Cluster.\n(default: 1 replica)."]
    pub fn replicas(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.replicas", self.base))
    }
    #[doc = "Get a reference to the value of field `vsphere_config` after provisioning.\nVsphere-specific config."]
    pub fn vsphere_config(
        &self,
    ) -> ListRef<GkeonpremVmwareClusterControlPlaneNodeElVsphereConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vsphere_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `auto_resize_config` after provisioning.\n"]
    pub fn auto_resize_config(
        &self,
    ) -> ListRef<GkeonpremVmwareClusterControlPlaneNodeElAutoResizeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_resize_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareClusterDataplaneV2El {
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_networking: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dataplane_v2_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    windows_dataplane_v2_enabled: Option<PrimField<bool>>,
}
impl GkeonpremVmwareClusterDataplaneV2El {
    #[doc = "Set the field `advanced_networking`.\nEnable advanced networking which requires dataplane_v2_enabled to be set true."]
    pub fn set_advanced_networking(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.advanced_networking = Some(v.into());
        self
    }
    #[doc = "Set the field `dataplane_v2_enabled`.\nEnables Dataplane V2."]
    pub fn set_dataplane_v2_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.dataplane_v2_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `windows_dataplane_v2_enabled`.\nEnable Dataplane V2 for clusters with Windows nodes."]
    pub fn set_windows_dataplane_v2_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.windows_dataplane_v2_enabled = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareClusterDataplaneV2El {
    type O = BlockAssignable<GkeonpremVmwareClusterDataplaneV2El>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterDataplaneV2El {}
impl BuildGkeonpremVmwareClusterDataplaneV2El {
    pub fn build(self) -> GkeonpremVmwareClusterDataplaneV2El {
        GkeonpremVmwareClusterDataplaneV2El {
            advanced_networking: core::default::Default::default(),
            dataplane_v2_enabled: core::default::Default::default(),
            windows_dataplane_v2_enabled: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareClusterDataplaneV2ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterDataplaneV2ElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareClusterDataplaneV2ElRef {
        GkeonpremVmwareClusterDataplaneV2ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterDataplaneV2ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `advanced_networking` after provisioning.\nEnable advanced networking which requires dataplane_v2_enabled to be set true."]
    pub fn advanced_networking(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.advanced_networking", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dataplane_v2_enabled` after provisioning.\nEnables Dataplane V2."]
    pub fn dataplane_v2_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataplane_v2_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `windows_dataplane_v2_enabled` after provisioning.\nEnable Dataplane V2 for clusters with Windows nodes."]
    pub fn windows_dataplane_v2_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.windows_dataplane_v2_enabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareClusterLoadBalancerElF5ConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snat_pool: Option<PrimField<String>>,
}
impl GkeonpremVmwareClusterLoadBalancerElF5ConfigEl {
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
impl ToListMappable for GkeonpremVmwareClusterLoadBalancerElF5ConfigEl {
    type O = BlockAssignable<GkeonpremVmwareClusterLoadBalancerElF5ConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterLoadBalancerElF5ConfigEl {}
impl BuildGkeonpremVmwareClusterLoadBalancerElF5ConfigEl {
    pub fn build(self) -> GkeonpremVmwareClusterLoadBalancerElF5ConfigEl {
        GkeonpremVmwareClusterLoadBalancerElF5ConfigEl {
            address: core::default::Default::default(),
            partition: core::default::Default::default(),
            snat_pool: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareClusterLoadBalancerElF5ConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterLoadBalancerElF5ConfigElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareClusterLoadBalancerElF5ConfigElRef {
        GkeonpremVmwareClusterLoadBalancerElF5ConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterLoadBalancerElF5ConfigElRef {
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
pub struct GkeonpremVmwareClusterLoadBalancerElManualLbConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    control_plane_node_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress_http_node_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress_https_node_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    konnectivity_server_node_port: Option<PrimField<f64>>,
}
impl GkeonpremVmwareClusterLoadBalancerElManualLbConfigEl {
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
impl ToListMappable for GkeonpremVmwareClusterLoadBalancerElManualLbConfigEl {
    type O = BlockAssignable<GkeonpremVmwareClusterLoadBalancerElManualLbConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterLoadBalancerElManualLbConfigEl {}
impl BuildGkeonpremVmwareClusterLoadBalancerElManualLbConfigEl {
    pub fn build(self) -> GkeonpremVmwareClusterLoadBalancerElManualLbConfigEl {
        GkeonpremVmwareClusterLoadBalancerElManualLbConfigEl {
            control_plane_node_port: core::default::Default::default(),
            ingress_http_node_port: core::default::Default::default(),
            ingress_https_node_port: core::default::Default::default(),
            konnectivity_server_node_port: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareClusterLoadBalancerElManualLbConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterLoadBalancerElManualLbConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareClusterLoadBalancerElManualLbConfigElRef {
        GkeonpremVmwareClusterLoadBalancerElManualLbConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterLoadBalancerElManualLbConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
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
pub struct GkeonpremVmwareClusterLoadBalancerElMetalLbConfigElAddressPoolsEl {
    addresses: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avoid_buggy_ips: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manual_assign: Option<PrimField<bool>>,
    pool: PrimField<String>,
}
impl GkeonpremVmwareClusterLoadBalancerElMetalLbConfigElAddressPoolsEl {
    #[doc = "Set the field `avoid_buggy_ips`.\nIf true, avoid using IPs ending in .0 or .255.\nThis avoids buggy consumer devices mistakenly dropping IPv4 traffic for\nthose special IP addresses."]
    pub fn set_avoid_buggy_ips(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.avoid_buggy_ips = Some(v.into());
        self
    }
    #[doc = "Set the field `manual_assign`.\nIf true, prevent IP addresses from being automatically assigned."]
    pub fn set_manual_assign(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.manual_assign = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareClusterLoadBalancerElMetalLbConfigElAddressPoolsEl {
    type O = BlockAssignable<GkeonpremVmwareClusterLoadBalancerElMetalLbConfigElAddressPoolsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterLoadBalancerElMetalLbConfigElAddressPoolsEl {
    #[doc = "The addresses that are part of this pool. Each address\nmust be either in the CIDR form (1.2.3.0/24) or range\nform (1.2.3.1-1.2.3.5)."]
    pub addresses: ListField<PrimField<String>>,
    #[doc = "The name of the address pool."]
    pub pool: PrimField<String>,
}
impl BuildGkeonpremVmwareClusterLoadBalancerElMetalLbConfigElAddressPoolsEl {
    pub fn build(self) -> GkeonpremVmwareClusterLoadBalancerElMetalLbConfigElAddressPoolsEl {
        GkeonpremVmwareClusterLoadBalancerElMetalLbConfigElAddressPoolsEl {
            addresses: self.addresses,
            avoid_buggy_ips: core::default::Default::default(),
            manual_assign: core::default::Default::default(),
            pool: self.pool,
        }
    }
}
pub struct GkeonpremVmwareClusterLoadBalancerElMetalLbConfigElAddressPoolsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterLoadBalancerElMetalLbConfigElAddressPoolsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareClusterLoadBalancerElMetalLbConfigElAddressPoolsElRef {
        GkeonpremVmwareClusterLoadBalancerElMetalLbConfigElAddressPoolsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterLoadBalancerElMetalLbConfigElAddressPoolsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `addresses` after provisioning.\nThe addresses that are part of this pool. Each address\nmust be either in the CIDR form (1.2.3.0/24) or range\nform (1.2.3.1-1.2.3.5)."]
    pub fn addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.addresses", self.base))
    }
    #[doc = "Get a reference to the value of field `avoid_buggy_ips` after provisioning.\nIf true, avoid using IPs ending in .0 or .255.\nThis avoids buggy consumer devices mistakenly dropping IPv4 traffic for\nthose special IP addresses."]
    pub fn avoid_buggy_ips(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.avoid_buggy_ips", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `manual_assign` after provisioning.\nIf true, prevent IP addresses from being automatically assigned."]
    pub fn manual_assign(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.manual_assign", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pool` after provisioning.\nThe name of the address pool."]
    pub fn pool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pool", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremVmwareClusterLoadBalancerElMetalLbConfigElDynamic {
    address_pools:
        Option<DynamicBlock<GkeonpremVmwareClusterLoadBalancerElMetalLbConfigElAddressPoolsEl>>,
}
#[derive(Serialize)]
pub struct GkeonpremVmwareClusterLoadBalancerElMetalLbConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address_pools: Option<Vec<GkeonpremVmwareClusterLoadBalancerElMetalLbConfigElAddressPoolsEl>>,
    dynamic: GkeonpremVmwareClusterLoadBalancerElMetalLbConfigElDynamic,
}
impl GkeonpremVmwareClusterLoadBalancerElMetalLbConfigEl {
    #[doc = "Set the field `address_pools`.\n"]
    pub fn set_address_pools(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremVmwareClusterLoadBalancerElMetalLbConfigElAddressPoolsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.address_pools = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.address_pools = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeonpremVmwareClusterLoadBalancerElMetalLbConfigEl {
    type O = BlockAssignable<GkeonpremVmwareClusterLoadBalancerElMetalLbConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterLoadBalancerElMetalLbConfigEl {}
impl BuildGkeonpremVmwareClusterLoadBalancerElMetalLbConfigEl {
    pub fn build(self) -> GkeonpremVmwareClusterLoadBalancerElMetalLbConfigEl {
        GkeonpremVmwareClusterLoadBalancerElMetalLbConfigEl {
            address_pools: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremVmwareClusterLoadBalancerElMetalLbConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterLoadBalancerElMetalLbConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareClusterLoadBalancerElMetalLbConfigElRef {
        GkeonpremVmwareClusterLoadBalancerElMetalLbConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterLoadBalancerElMetalLbConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address_pools` after provisioning.\n"]
    pub fn address_pools(
        &self,
    ) -> ListRef<GkeonpremVmwareClusterLoadBalancerElMetalLbConfigElAddressPoolsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.address_pools", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareClusterLoadBalancerElVipConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    control_plane_vip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress_vip: Option<PrimField<String>>,
}
impl GkeonpremVmwareClusterLoadBalancerElVipConfigEl {
    #[doc = "Set the field `control_plane_vip`.\nThe VIP which you previously set aside for the Kubernetes API of this cluster."]
    pub fn set_control_plane_vip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.control_plane_vip = Some(v.into());
        self
    }
    #[doc = "Set the field `ingress_vip`.\nThe VIP which you previously set aside for ingress traffic into this cluster."]
    pub fn set_ingress_vip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ingress_vip = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareClusterLoadBalancerElVipConfigEl {
    type O = BlockAssignable<GkeonpremVmwareClusterLoadBalancerElVipConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterLoadBalancerElVipConfigEl {}
impl BuildGkeonpremVmwareClusterLoadBalancerElVipConfigEl {
    pub fn build(self) -> GkeonpremVmwareClusterLoadBalancerElVipConfigEl {
        GkeonpremVmwareClusterLoadBalancerElVipConfigEl {
            control_plane_vip: core::default::Default::default(),
            ingress_vip: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareClusterLoadBalancerElVipConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterLoadBalancerElVipConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareClusterLoadBalancerElVipConfigElRef {
        GkeonpremVmwareClusterLoadBalancerElVipConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterLoadBalancerElVipConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `control_plane_vip` after provisioning.\nThe VIP which you previously set aside for the Kubernetes API of this cluster."]
    pub fn control_plane_vip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.control_plane_vip", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ingress_vip` after provisioning.\nThe VIP which you previously set aside for ingress traffic into this cluster."]
    pub fn ingress_vip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ingress_vip", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremVmwareClusterLoadBalancerElDynamic {
    f5_config: Option<DynamicBlock<GkeonpremVmwareClusterLoadBalancerElF5ConfigEl>>,
    manual_lb_config: Option<DynamicBlock<GkeonpremVmwareClusterLoadBalancerElManualLbConfigEl>>,
    metal_lb_config: Option<DynamicBlock<GkeonpremVmwareClusterLoadBalancerElMetalLbConfigEl>>,
    vip_config: Option<DynamicBlock<GkeonpremVmwareClusterLoadBalancerElVipConfigEl>>,
}
#[derive(Serialize)]
pub struct GkeonpremVmwareClusterLoadBalancerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    f5_config: Option<Vec<GkeonpremVmwareClusterLoadBalancerElF5ConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manual_lb_config: Option<Vec<GkeonpremVmwareClusterLoadBalancerElManualLbConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metal_lb_config: Option<Vec<GkeonpremVmwareClusterLoadBalancerElMetalLbConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vip_config: Option<Vec<GkeonpremVmwareClusterLoadBalancerElVipConfigEl>>,
    dynamic: GkeonpremVmwareClusterLoadBalancerElDynamic,
}
impl GkeonpremVmwareClusterLoadBalancerEl {
    #[doc = "Set the field `f5_config`.\n"]
    pub fn set_f5_config(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremVmwareClusterLoadBalancerElF5ConfigEl>>,
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
        v: impl Into<BlockAssignable<GkeonpremVmwareClusterLoadBalancerElManualLbConfigEl>>,
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
        v: impl Into<BlockAssignable<GkeonpremVmwareClusterLoadBalancerElMetalLbConfigEl>>,
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
        v: impl Into<BlockAssignable<GkeonpremVmwareClusterLoadBalancerElVipConfigEl>>,
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
impl ToListMappable for GkeonpremVmwareClusterLoadBalancerEl {
    type O = BlockAssignable<GkeonpremVmwareClusterLoadBalancerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterLoadBalancerEl {}
impl BuildGkeonpremVmwareClusterLoadBalancerEl {
    pub fn build(self) -> GkeonpremVmwareClusterLoadBalancerEl {
        GkeonpremVmwareClusterLoadBalancerEl {
            f5_config: core::default::Default::default(),
            manual_lb_config: core::default::Default::default(),
            metal_lb_config: core::default::Default::default(),
            vip_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremVmwareClusterLoadBalancerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterLoadBalancerElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareClusterLoadBalancerElRef {
        GkeonpremVmwareClusterLoadBalancerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterLoadBalancerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `f5_config` after provisioning.\n"]
    pub fn f5_config(&self) -> ListRef<GkeonpremVmwareClusterLoadBalancerElF5ConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.f5_config", self.base))
    }
    #[doc = "Get a reference to the value of field `manual_lb_config` after provisioning.\n"]
    pub fn manual_lb_config(
        &self,
    ) -> ListRef<GkeonpremVmwareClusterLoadBalancerElManualLbConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.manual_lb_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metal_lb_config` after provisioning.\n"]
    pub fn metal_lb_config(
        &self,
    ) -> ListRef<GkeonpremVmwareClusterLoadBalancerElMetalLbConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metal_lb_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vip_config` after provisioning.\n"]
    pub fn vip_config(&self) -> ListRef<GkeonpremVmwareClusterLoadBalancerElVipConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vip_config", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockElIpsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<PrimField<String>>,
}
impl GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockElIpsEl {
    #[doc = "Set the field `hostname`.\nHostname of the machine. VM's name will be used if this field is empty."]
    pub fn set_hostname(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hostname = Some(v.into());
        self
    }
    #[doc = "Set the field `ip`.\nIP could be an IP address (like 1.2.3.4) or a CIDR (like 1.2.3.0/24)."]
    pub fn set_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip = Some(v.into());
        self
    }
}
impl ToListMappable
    for GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockElIpsEl
{
    type O = BlockAssignable<
        GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockElIpsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockElIpsEl
{}
impl BuildGkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockElIpsEl {
    pub fn build(
        self,
    ) -> GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockElIpsEl {
        GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockElIpsEl {
            hostname: core::default::Default::default(),
            ip: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockElIpsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockElIpsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockElIpsElRef
    {
        GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockElIpsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockElIpsElRef {
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
struct GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockElDynamic {
    ips: Option<
        DynamicBlock<
            GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockElIpsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gateway: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    netmask: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ips: Option<
        Vec<GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockElIpsEl>,
    >,
    dynamic:
        GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockElDynamic,
}
impl GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockEl {
    #[doc = "Set the field `gateway`.\nThe network gateway used by the VMware User Cluster."]
    pub fn set_gateway(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gateway = Some(v.into());
        self
    }
    #[doc = "Set the field `netmask`.\nThe netmask used by the VMware User Cluster."]
    pub fn set_netmask(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.netmask = Some(v.into());
        self
    }
    #[doc = "Set the field `ips`.\n"]
    pub fn set_ips(
        mut self,
        v : impl Into < BlockAssignable < GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockElIpsEl >>,
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
    for GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockEl
{
    type O = BlockAssignable<
        GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockEl {
}
impl BuildGkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockEl {
    pub fn build(
        self,
    ) -> GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockEl {
        GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockEl {
            gateway: core::default::Default::default(),
            netmask: core::default::Default::default(),
            ips: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockElRef {
        GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gateway` after provisioning.\nThe network gateway used by the VMware User Cluster."]
    pub fn gateway(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gateway", self.base))
    }
    #[doc = "Get a reference to the value of field `netmask` after provisioning.\nThe netmask used by the VMware User Cluster."]
    pub fn netmask(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.netmask", self.base))
    }
    #[doc = "Get a reference to the value of field `ips` after provisioning.\n"]
    pub fn ips(
        &self,
    ) -> ListRef<
        GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockElIpsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.ips", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElDynamic {
    control_plane_ip_block: Option<
        DynamicBlock<
            GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    control_plane_ip_block: Option<
        Vec<GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockEl>,
    >,
    dynamic: GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElDynamic,
}
impl GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigEl {
    #[doc = "Set the field `control_plane_ip_block`.\n"]
    pub fn set_control_plane_ip_block(
        mut self,
        v: impl Into<
            BlockAssignable<
                GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockEl,
            >,
        >,
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
impl ToListMappable for GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigEl {
    type O = BlockAssignable<GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigEl {}
impl BuildGkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigEl {
    pub fn build(self) -> GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigEl {
        GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigEl {
            control_plane_ip_block: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElRef {
        GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `control_plane_ip_block` after provisioning.\n"]
    pub fn control_plane_ip_block(
        &self,
    ) -> ListRef<GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElControlPlaneIpBlockElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane_ip_block", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareClusterNetworkConfigElDhcpIpConfigEl {
    enabled: PrimField<bool>,
}
impl GkeonpremVmwareClusterNetworkConfigElDhcpIpConfigEl {}
impl ToListMappable for GkeonpremVmwareClusterNetworkConfigElDhcpIpConfigEl {
    type O = BlockAssignable<GkeonpremVmwareClusterNetworkConfigElDhcpIpConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterNetworkConfigElDhcpIpConfigEl {
    #[doc = "enabled is a flag to mark if DHCP IP allocation is\nused for VMware user clusters."]
    pub enabled: PrimField<bool>,
}
impl BuildGkeonpremVmwareClusterNetworkConfigElDhcpIpConfigEl {
    pub fn build(self) -> GkeonpremVmwareClusterNetworkConfigElDhcpIpConfigEl {
        GkeonpremVmwareClusterNetworkConfigElDhcpIpConfigEl {
            enabled: self.enabled,
        }
    }
}
pub struct GkeonpremVmwareClusterNetworkConfigElDhcpIpConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterNetworkConfigElDhcpIpConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareClusterNetworkConfigElDhcpIpConfigElRef {
        GkeonpremVmwareClusterNetworkConfigElDhcpIpConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterNetworkConfigElDhcpIpConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nenabled is a flag to mark if DHCP IP allocation is\nused for VMware user clusters."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareClusterNetworkConfigElHostConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_search_domains: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_servers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ntp_servers: Option<ListField<PrimField<String>>>,
}
impl GkeonpremVmwareClusterNetworkConfigElHostConfigEl {
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
impl ToListMappable for GkeonpremVmwareClusterNetworkConfigElHostConfigEl {
    type O = BlockAssignable<GkeonpremVmwareClusterNetworkConfigElHostConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterNetworkConfigElHostConfigEl {}
impl BuildGkeonpremVmwareClusterNetworkConfigElHostConfigEl {
    pub fn build(self) -> GkeonpremVmwareClusterNetworkConfigElHostConfigEl {
        GkeonpremVmwareClusterNetworkConfigElHostConfigEl {
            dns_search_domains: core::default::Default::default(),
            dns_servers: core::default::Default::default(),
            ntp_servers: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareClusterNetworkConfigElHostConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterNetworkConfigElHostConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareClusterNetworkConfigElHostConfigElRef {
        GkeonpremVmwareClusterNetworkConfigElHostConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterNetworkConfigElHostConfigElRef {
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
pub struct GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname: Option<PrimField<String>>,
    ip: PrimField<String>,
}
impl GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsEl {
    #[doc = "Set the field `hostname`.\nHostname of the machine. VM's name will be used if this field is empty."]
    pub fn set_hostname(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hostname = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsEl {
    type O = BlockAssignable<GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsEl {
    #[doc = "IP could be an IP address (like 1.2.3.4) or a CIDR (like 1.2.3.0/24)."]
    pub ip: PrimField<String>,
}
impl BuildGkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsEl {
    pub fn build(self) -> GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsEl {
        GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsEl {
            hostname: core::default::Default::default(),
            ip: self.ip,
        }
    }
}
pub struct GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsElRef {
        GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsElRef {
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
struct GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksElDynamic {
    ips: Option<DynamicBlock<GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsEl>>,
}
#[derive(Serialize)]
pub struct GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksEl {
    gateway: PrimField<String>,
    netmask: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ips: Option<Vec<GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsEl>>,
    dynamic: GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksElDynamic,
}
impl GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksEl {
    #[doc = "Set the field `ips`.\n"]
    pub fn set_ips(
        mut self,
        v: impl Into<
            BlockAssignable<GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsEl>,
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
impl ToListMappable for GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksEl {
    type O = BlockAssignable<GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksEl {
    #[doc = "The network gateway used by the VMware User Cluster."]
    pub gateway: PrimField<String>,
    #[doc = "The netmask used by the VMware User Cluster."]
    pub netmask: PrimField<String>,
}
impl BuildGkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksEl {
    pub fn build(self) -> GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksEl {
        GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksEl {
            gateway: self.gateway,
            netmask: self.netmask,
            ips: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksElRef {
        GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gateway` after provisioning.\nThe network gateway used by the VMware User Cluster."]
    pub fn gateway(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gateway", self.base))
    }
    #[doc = "Get a reference to the value of field `netmask` after provisioning.\nThe netmask used by the VMware User Cluster."]
    pub fn netmask(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.netmask", self.base))
    }
    #[doc = "Get a reference to the value of field `ips` after provisioning.\n"]
    pub fn ips(
        &self,
    ) -> ListRef<GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksElIpsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ips", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElDynamic {
    ip_blocks:
        Option<DynamicBlock<GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksEl>>,
}
#[derive(Serialize)]
pub struct GkeonpremVmwareClusterNetworkConfigElStaticIpConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_blocks: Option<Vec<GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksEl>>,
    dynamic: GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElDynamic,
}
impl GkeonpremVmwareClusterNetworkConfigElStaticIpConfigEl {
    #[doc = "Set the field `ip_blocks`.\n"]
    pub fn set_ip_blocks(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksEl>>,
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
impl ToListMappable for GkeonpremVmwareClusterNetworkConfigElStaticIpConfigEl {
    type O = BlockAssignable<GkeonpremVmwareClusterNetworkConfigElStaticIpConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterNetworkConfigElStaticIpConfigEl {}
impl BuildGkeonpremVmwareClusterNetworkConfigElStaticIpConfigEl {
    pub fn build(self) -> GkeonpremVmwareClusterNetworkConfigElStaticIpConfigEl {
        GkeonpremVmwareClusterNetworkConfigElStaticIpConfigEl {
            ip_blocks: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElRef {
        GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_blocks` after provisioning.\n"]
    pub fn ip_blocks(
        &self,
    ) -> ListRef<GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElIpBlocksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ip_blocks", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremVmwareClusterNetworkConfigElDynamic {
    control_plane_v2_config:
        Option<DynamicBlock<GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigEl>>,
    dhcp_ip_config: Option<DynamicBlock<GkeonpremVmwareClusterNetworkConfigElDhcpIpConfigEl>>,
    host_config: Option<DynamicBlock<GkeonpremVmwareClusterNetworkConfigElHostConfigEl>>,
    static_ip_config: Option<DynamicBlock<GkeonpremVmwareClusterNetworkConfigElStaticIpConfigEl>>,
}
#[derive(Serialize)]
pub struct GkeonpremVmwareClusterNetworkConfigEl {
    pod_address_cidr_blocks: ListField<PrimField<String>>,
    service_address_cidr_blocks: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcenter_network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    control_plane_v2_config:
        Option<Vec<GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dhcp_ip_config: Option<Vec<GkeonpremVmwareClusterNetworkConfigElDhcpIpConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_config: Option<Vec<GkeonpremVmwareClusterNetworkConfigElHostConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    static_ip_config: Option<Vec<GkeonpremVmwareClusterNetworkConfigElStaticIpConfigEl>>,
    dynamic: GkeonpremVmwareClusterNetworkConfigElDynamic,
}
impl GkeonpremVmwareClusterNetworkConfigEl {
    #[doc = "Set the field `vcenter_network`.\nvcenter_network specifies vCenter network name. Inherited from the admin cluster."]
    pub fn set_vcenter_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vcenter_network = Some(v.into());
        self
    }
    #[doc = "Set the field `control_plane_v2_config`.\n"]
    pub fn set_control_plane_v2_config(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.control_plane_v2_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.control_plane_v2_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dhcp_ip_config`.\n"]
    pub fn set_dhcp_ip_config(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremVmwareClusterNetworkConfigElDhcpIpConfigEl>>,
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
    #[doc = "Set the field `host_config`.\n"]
    pub fn set_host_config(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremVmwareClusterNetworkConfigElHostConfigEl>>,
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
        v: impl Into<BlockAssignable<GkeonpremVmwareClusterNetworkConfigElStaticIpConfigEl>>,
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
impl ToListMappable for GkeonpremVmwareClusterNetworkConfigEl {
    type O = BlockAssignable<GkeonpremVmwareClusterNetworkConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterNetworkConfigEl {
    #[doc = "All pods in the cluster are assigned an RFC1918 IPv4 address from these ranges.\nOnly a single range is supported. This field cannot be changed after creation."]
    pub pod_address_cidr_blocks: ListField<PrimField<String>>,
    #[doc = "All services in the cluster are assigned an RFC1918 IPv4 address\nfrom these ranges. Only a single range is supported.. This field\ncannot be changed after creation."]
    pub service_address_cidr_blocks: ListField<PrimField<String>>,
}
impl BuildGkeonpremVmwareClusterNetworkConfigEl {
    pub fn build(self) -> GkeonpremVmwareClusterNetworkConfigEl {
        GkeonpremVmwareClusterNetworkConfigEl {
            pod_address_cidr_blocks: self.pod_address_cidr_blocks,
            service_address_cidr_blocks: self.service_address_cidr_blocks,
            vcenter_network: core::default::Default::default(),
            control_plane_v2_config: core::default::Default::default(),
            dhcp_ip_config: core::default::Default::default(),
            host_config: core::default::Default::default(),
            static_ip_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremVmwareClusterNetworkConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterNetworkConfigElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareClusterNetworkConfigElRef {
        GkeonpremVmwareClusterNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterNetworkConfigElRef {
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
    #[doc = "Get a reference to the value of field `vcenter_network` after provisioning.\nvcenter_network specifies vCenter network name. Inherited from the admin cluster."]
    pub fn vcenter_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vcenter_network", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `control_plane_v2_config` after provisioning.\n"]
    pub fn control_plane_v2_config(
        &self,
    ) -> ListRef<GkeonpremVmwareClusterNetworkConfigElControlPlaneV2ConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane_v2_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dhcp_ip_config` after provisioning.\n"]
    pub fn dhcp_ip_config(
        &self,
    ) -> ListRef<GkeonpremVmwareClusterNetworkConfigElDhcpIpConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dhcp_ip_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `host_config` after provisioning.\n"]
    pub fn host_config(&self) -> ListRef<GkeonpremVmwareClusterNetworkConfigElHostConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.host_config", self.base))
    }
    #[doc = "Get a reference to the value of field `static_ip_config` after provisioning.\n"]
    pub fn static_ip_config(
        &self,
    ) -> ListRef<GkeonpremVmwareClusterNetworkConfigElStaticIpConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.static_ip_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareClusterStorageEl {
    vsphere_csi_disabled: PrimField<bool>,
}
impl GkeonpremVmwareClusterStorageEl {}
impl ToListMappable for GkeonpremVmwareClusterStorageEl {
    type O = BlockAssignable<GkeonpremVmwareClusterStorageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterStorageEl {
    #[doc = "Whether or not to deploy vSphere CSI components in the VMware User Cluster.\nEnabled by default."]
    pub vsphere_csi_disabled: PrimField<bool>,
}
impl BuildGkeonpremVmwareClusterStorageEl {
    pub fn build(self) -> GkeonpremVmwareClusterStorageEl {
        GkeonpremVmwareClusterStorageEl {
            vsphere_csi_disabled: self.vsphere_csi_disabled,
        }
    }
}
pub struct GkeonpremVmwareClusterStorageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterStorageElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareClusterStorageElRef {
        GkeonpremVmwareClusterStorageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterStorageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `vsphere_csi_disabled` after provisioning.\nWhether or not to deploy vSphere CSI components in the VMware User Cluster.\nEnabled by default."]
    pub fn vsphere_csi_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vsphere_csi_disabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl GkeonpremVmwareClusterTimeoutsEl {
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
impl ToListMappable for GkeonpremVmwareClusterTimeoutsEl {
    type O = BlockAssignable<GkeonpremVmwareClusterTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterTimeoutsEl {}
impl BuildGkeonpremVmwareClusterTimeoutsEl {
    pub fn build(self) -> GkeonpremVmwareClusterTimeoutsEl {
        GkeonpremVmwareClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareClusterTimeoutsElRef {
        GkeonpremVmwareClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterTimeoutsElRef {
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
pub struct GkeonpremVmwareClusterUpgradePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    control_plane_only: Option<PrimField<bool>>,
}
impl GkeonpremVmwareClusterUpgradePolicyEl {
    #[doc = "Set the field `control_plane_only`.\nControls whether the upgrade applies to the control plane only."]
    pub fn set_control_plane_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.control_plane_only = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareClusterUpgradePolicyEl {
    type O = BlockAssignable<GkeonpremVmwareClusterUpgradePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterUpgradePolicyEl {}
impl BuildGkeonpremVmwareClusterUpgradePolicyEl {
    pub fn build(self) -> GkeonpremVmwareClusterUpgradePolicyEl {
        GkeonpremVmwareClusterUpgradePolicyEl {
            control_plane_only: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareClusterUpgradePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterUpgradePolicyElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareClusterUpgradePolicyElRef {
        GkeonpremVmwareClusterUpgradePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterUpgradePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `control_plane_only` after provisioning.\nControls whether the upgrade applies to the control plane only."]
    pub fn control_plane_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.control_plane_only", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareClusterVcenterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_cert_data: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster: Option<PrimField<String>>,
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
impl GkeonpremVmwareClusterVcenterEl {
    #[doc = "Set the field `ca_cert_data`.\nContains the vCenter CA certificate public key for SSL verification."]
    pub fn set_ca_cert_data(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ca_cert_data = Some(v.into());
        self
    }
    #[doc = "Set the field `cluster`.\nThe name of the vCenter cluster for the user cluster."]
    pub fn set_cluster(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster = Some(v.into());
        self
    }
    #[doc = "Set the field `datacenter`.\nThe name of the vCenter datacenter for the user cluster."]
    pub fn set_datacenter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.datacenter = Some(v.into());
        self
    }
    #[doc = "Set the field `datastore`.\nThe name of the vCenter datastore for the user cluster."]
    pub fn set_datastore(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.datastore = Some(v.into());
        self
    }
    #[doc = "Set the field `folder`.\nThe name of the vCenter folder for the user cluster."]
    pub fn set_folder(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.folder = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_pool`.\nThe name of the vCenter resource pool for the user cluster."]
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
impl ToListMappable for GkeonpremVmwareClusterVcenterEl {
    type O = BlockAssignable<GkeonpremVmwareClusterVcenterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareClusterVcenterEl {}
impl BuildGkeonpremVmwareClusterVcenterEl {
    pub fn build(self) -> GkeonpremVmwareClusterVcenterEl {
        GkeonpremVmwareClusterVcenterEl {
            ca_cert_data: core::default::Default::default(),
            cluster: core::default::Default::default(),
            datacenter: core::default::Default::default(),
            datastore: core::default::Default::default(),
            folder: core::default::Default::default(),
            resource_pool: core::default::Default::default(),
            storage_policy_name: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareClusterVcenterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareClusterVcenterElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareClusterVcenterElRef {
        GkeonpremVmwareClusterVcenterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareClusterVcenterElRef {
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
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\nThe name of the vCenter cluster for the user cluster."]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }
    #[doc = "Get a reference to the value of field `datacenter` after provisioning.\nThe name of the vCenter datacenter for the user cluster."]
    pub fn datacenter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.datacenter", self.base))
    }
    #[doc = "Get a reference to the value of field `datastore` after provisioning.\nThe name of the vCenter datastore for the user cluster."]
    pub fn datastore(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.datastore", self.base))
    }
    #[doc = "Get a reference to the value of field `folder` after provisioning.\nThe name of the vCenter folder for the user cluster."]
    pub fn folder(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.folder", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_pool` after provisioning.\nThe name of the vCenter resource pool for the user cluster."]
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
struct GkeonpremVmwareClusterDynamic {
    anti_affinity_groups: Option<DynamicBlock<GkeonpremVmwareClusterAntiAffinityGroupsEl>>,
    authorization: Option<DynamicBlock<GkeonpremVmwareClusterAuthorizationEl>>,
    auto_repair_config: Option<DynamicBlock<GkeonpremVmwareClusterAutoRepairConfigEl>>,
    control_plane_node: Option<DynamicBlock<GkeonpremVmwareClusterControlPlaneNodeEl>>,
    dataplane_v2: Option<DynamicBlock<GkeonpremVmwareClusterDataplaneV2El>>,
    load_balancer: Option<DynamicBlock<GkeonpremVmwareClusterLoadBalancerEl>>,
    network_config: Option<DynamicBlock<GkeonpremVmwareClusterNetworkConfigEl>>,
    storage: Option<DynamicBlock<GkeonpremVmwareClusterStorageEl>>,
    upgrade_policy: Option<DynamicBlock<GkeonpremVmwareClusterUpgradePolicyEl>>,
    vcenter: Option<DynamicBlock<GkeonpremVmwareClusterVcenterEl>>,
}
