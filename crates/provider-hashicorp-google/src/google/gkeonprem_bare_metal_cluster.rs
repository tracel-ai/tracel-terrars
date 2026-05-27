use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct GkeonpremBareMetalClusterData {
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
    bare_metal_version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    binary_authorization: Option<Vec<GkeonpremBareMetalClusterBinaryAuthorizationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_operations: Option<Vec<GkeonpremBareMetalClusterClusterOperationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    control_plane: Option<Vec<GkeonpremBareMetalClusterControlPlaneEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancer: Option<Vec<GkeonpremBareMetalClusterLoadBalancerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_config: Option<Vec<GkeonpremBareMetalClusterMaintenanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_config: Option<Vec<GkeonpremBareMetalClusterNetworkConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_access_config: Option<Vec<GkeonpremBareMetalClusterNodeAccessConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_config: Option<Vec<GkeonpremBareMetalClusterNodeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os_environment_config: Option<Vec<GkeonpremBareMetalClusterOsEnvironmentConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy: Option<Vec<GkeonpremBareMetalClusterProxyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_config: Option<Vec<GkeonpremBareMetalClusterSecurityConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage: Option<Vec<GkeonpremBareMetalClusterStorageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<GkeonpremBareMetalClusterTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upgrade_policy: Option<Vec<GkeonpremBareMetalClusterUpgradePolicyEl>>,
    dynamic: GkeonpremBareMetalClusterDynamic,
}
struct GkeonpremBareMetalCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GkeonpremBareMetalClusterData>,
}
#[derive(Clone)]
pub struct GkeonpremBareMetalCluster(Rc<GkeonpremBareMetalCluster_>);
impl GkeonpremBareMetalCluster {
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
    #[doc = "Set the field `annotations`.\nAnnotations on the Bare Metal User Cluster.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nA human readable description of this Bare Metal User Cluster."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
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
    #[doc = "Set the field `binary_authorization`.\n"]
    pub fn set_binary_authorization(
        self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalClusterBinaryAuthorizationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().binary_authorization = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.binary_authorization = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cluster_operations`.\n"]
    pub fn set_cluster_operations(
        self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalClusterClusterOperationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cluster_operations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cluster_operations = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `control_plane`.\n"]
    pub fn set_control_plane(
        self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalClusterControlPlaneEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().control_plane = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.control_plane = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `load_balancer`.\n"]
    pub fn set_load_balancer(
        self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalClusterLoadBalancerEl>>,
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
    #[doc = "Set the field `maintenance_config`.\n"]
    pub fn set_maintenance_config(
        self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalClusterMaintenanceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_config`.\n"]
    pub fn set_network_config(
        self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalClusterNetworkConfigEl>>,
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
    #[doc = "Set the field `node_access_config`.\n"]
    pub fn set_node_access_config(
        self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalClusterNodeAccessConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_access_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_access_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `node_config`.\n"]
    pub fn set_node_config(
        self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalClusterNodeConfigEl>>,
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
    #[doc = "Set the field `os_environment_config`.\n"]
    pub fn set_os_environment_config(
        self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalClusterOsEnvironmentConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().os_environment_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.os_environment_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `proxy`.\n"]
    pub fn set_proxy(
        self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalClusterProxyEl>>,
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
    #[doc = "Set the field `security_config`.\n"]
    pub fn set_security_config(
        self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalClusterSecurityConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().security_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.security_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `storage`.\n"]
    pub fn set_storage(
        self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalClusterStorageEl>>,
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
    pub fn set_timeouts(self, v: impl Into<GkeonpremBareMetalClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `upgrade_policy`.\n"]
    pub fn set_upgrade_policy(
        self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalClusterUpgradePolicyEl>>,
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
    #[doc = "Get a reference to the value of field `admin_cluster_membership` after provisioning.\nThe Admin Cluster this Bare Metal User Cluster belongs to.\nThis is the full resource name of the Admin Cluster's hub membership."]
    pub fn admin_cluster_membership(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_cluster_membership", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nAnnotations on the Bare Metal User Cluster.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bare_metal_version` after provisioning.\nA human readable description of this Bare Metal User Cluster."]
    pub fn bare_metal_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bare_metal_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time the cluster was created, in RFC3339 text format."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nThe time the cluster was deleted, in RFC3339 text format."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human readable description of this Bare Metal User Cluster."]
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
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nThe IP address name of Bare Metal User Cluster's API server."]
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
    #[doc = "Get a reference to the value of field `fleet` after provisioning.\nFleet related configuration.\nFleets are a Google Cloud concept for logically organizing clusters,\nletting you use and manage multi-cluster capabilities and apply\nconsistent policies across your systems.\nSee [Anthos Fleets](https://cloud.google.com/anthos/multicluster-management/fleets) for\nmore details on Anthos multi-cluster capabilities using Fleets."]
    pub fn fleet(&self) -> ListRef<GkeonpremBareMetalClusterFleetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fleet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `local_name` after provisioning.\nThe object name of the Bare Metal Cluster custom resource on the\nassociated admin cluster. This field is used to support conflicting\nnames when enrolling existing clusters to the API. When used as a part of\ncluster enrollment, this field will differ from the ID in the resource\nname. For new clusters, this field will match the user provided cluster ID\nand be visible in the last component of the resource name. It is not\nmodifiable.\nAll users should use this name to access their cluster using gkectl or\nkubectl and should expect to see the local name when viewing admin\ncluster controller logs."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe bare metal cluster name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the Bare Metal User Cluster."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of this cluster."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nSpecifies detailed cluster status."]
    pub fn status(&self) -> ListRef<GkeonpremBareMetalClusterStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe unique identifier of the Bare Metal User Cluster."]
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
    #[doc = "Get a reference to the value of field `validation_check` after provisioning.\nSpecifies the security related settings for the Bare Metal User Cluster."]
    pub fn validation_check(&self) -> ListRef<GkeonpremBareMetalClusterValidationCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.validation_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\n"]
    pub fn binary_authorization(
        &self,
    ) -> ListRef<GkeonpremBareMetalClusterBinaryAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.binary_authorization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_operations` after provisioning.\n"]
    pub fn cluster_operations(&self) -> ListRef<GkeonpremBareMetalClusterClusterOperationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_operations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `control_plane` after provisioning.\n"]
    pub fn control_plane(&self) -> ListRef<GkeonpremBareMetalClusterControlPlaneElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancer` after provisioning.\n"]
    pub fn load_balancer(&self) -> ListRef<GkeonpremBareMetalClusterLoadBalancerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.load_balancer", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_config` after provisioning.\n"]
    pub fn maintenance_config(&self) -> ListRef<GkeonpremBareMetalClusterMaintenanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<GkeonpremBareMetalClusterNetworkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_access_config` after provisioning.\n"]
    pub fn node_access_config(&self) -> ListRef<GkeonpremBareMetalClusterNodeAccessConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_access_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> ListRef<GkeonpremBareMetalClusterNodeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `os_environment_config` after provisioning.\n"]
    pub fn os_environment_config(
        &self,
    ) -> ListRef<GkeonpremBareMetalClusterOsEnvironmentConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.os_environment_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `proxy` after provisioning.\n"]
    pub fn proxy(&self) -> ListRef<GkeonpremBareMetalClusterProxyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.proxy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_config` after provisioning.\n"]
    pub fn security_config(&self) -> ListRef<GkeonpremBareMetalClusterSecurityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage` after provisioning.\n"]
    pub fn storage(&self) -> ListRef<GkeonpremBareMetalClusterStorageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeonpremBareMetalClusterTimeoutsElRef {
        GkeonpremBareMetalClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `upgrade_policy` after provisioning.\n"]
    pub fn upgrade_policy(&self) -> ListRef<GkeonpremBareMetalClusterUpgradePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.upgrade_policy", self.extract_ref()),
        )
    }
}
impl Referable for GkeonpremBareMetalCluster {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for GkeonpremBareMetalCluster {}
impl ToListMappable for GkeonpremBareMetalCluster {
    type O = ListRef<GkeonpremBareMetalClusterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for GkeonpremBareMetalCluster_ {
    fn extract_resource_type(&self) -> String {
        "google_gkeonprem_bare_metal_cluster".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildGkeonpremBareMetalCluster {
    pub tf_id: String,
    #[doc = "The Admin Cluster this Bare Metal User Cluster belongs to.\nThis is the full resource name of the Admin Cluster's hub membership."]
    pub admin_cluster_membership: PrimField<String>,
    #[doc = "A human readable description of this Bare Metal User Cluster."]
    pub bare_metal_version: PrimField<String>,
    #[doc = "The location of the resource."]
    pub location: PrimField<String>,
    #[doc = "The bare metal cluster name."]
    pub name: PrimField<String>,
}
impl BuildGkeonpremBareMetalCluster {
    pub fn build(self, stack: &mut Stack) -> GkeonpremBareMetalCluster {
        let out = GkeonpremBareMetalCluster(Rc::new(GkeonpremBareMetalCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GkeonpremBareMetalClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                admin_cluster_membership: self.admin_cluster_membership,
                annotations: core::default::Default::default(),
                bare_metal_version: self.bare_metal_version,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                binary_authorization: core::default::Default::default(),
                cluster_operations: core::default::Default::default(),
                control_plane: core::default::Default::default(),
                load_balancer: core::default::Default::default(),
                maintenance_config: core::default::Default::default(),
                network_config: core::default::Default::default(),
                node_access_config: core::default::Default::default(),
                node_config: core::default::Default::default(),
                os_environment_config: core::default::Default::default(),
                proxy: core::default::Default::default(),
                security_config: core::default::Default::default(),
                storage: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                upgrade_policy: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct GkeonpremBareMetalClusterRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl GkeonpremBareMetalClusterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `admin_cluster_membership` after provisioning.\nThe Admin Cluster this Bare Metal User Cluster belongs to.\nThis is the full resource name of the Admin Cluster's hub membership."]
    pub fn admin_cluster_membership(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_cluster_membership", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nAnnotations on the Bare Metal User Cluster.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bare_metal_version` after provisioning.\nA human readable description of this Bare Metal User Cluster."]
    pub fn bare_metal_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bare_metal_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time the cluster was created, in RFC3339 text format."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nThe time the cluster was deleted, in RFC3339 text format."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human readable description of this Bare Metal User Cluster."]
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
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nThe IP address name of Bare Metal User Cluster's API server."]
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
    #[doc = "Get a reference to the value of field `fleet` after provisioning.\nFleet related configuration.\nFleets are a Google Cloud concept for logically organizing clusters,\nletting you use and manage multi-cluster capabilities and apply\nconsistent policies across your systems.\nSee [Anthos Fleets](https://cloud.google.com/anthos/multicluster-management/fleets) for\nmore details on Anthos multi-cluster capabilities using Fleets."]
    pub fn fleet(&self) -> ListRef<GkeonpremBareMetalClusterFleetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fleet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `local_name` after provisioning.\nThe object name of the Bare Metal Cluster custom resource on the\nassociated admin cluster. This field is used to support conflicting\nnames when enrolling existing clusters to the API. When used as a part of\ncluster enrollment, this field will differ from the ID in the resource\nname. For new clusters, this field will match the user provided cluster ID\nand be visible in the last component of the resource name. It is not\nmodifiable.\nAll users should use this name to access their cluster using gkectl or\nkubectl and should expect to see the local name when viewing admin\ncluster controller logs."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe bare metal cluster name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the Bare Metal User Cluster."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of this cluster."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nSpecifies detailed cluster status."]
    pub fn status(&self) -> ListRef<GkeonpremBareMetalClusterStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe unique identifier of the Bare Metal User Cluster."]
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
    #[doc = "Get a reference to the value of field `validation_check` after provisioning.\nSpecifies the security related settings for the Bare Metal User Cluster."]
    pub fn validation_check(&self) -> ListRef<GkeonpremBareMetalClusterValidationCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.validation_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\n"]
    pub fn binary_authorization(
        &self,
    ) -> ListRef<GkeonpremBareMetalClusterBinaryAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.binary_authorization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_operations` after provisioning.\n"]
    pub fn cluster_operations(&self) -> ListRef<GkeonpremBareMetalClusterClusterOperationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_operations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `control_plane` after provisioning.\n"]
    pub fn control_plane(&self) -> ListRef<GkeonpremBareMetalClusterControlPlaneElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancer` after provisioning.\n"]
    pub fn load_balancer(&self) -> ListRef<GkeonpremBareMetalClusterLoadBalancerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.load_balancer", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_config` after provisioning.\n"]
    pub fn maintenance_config(&self) -> ListRef<GkeonpremBareMetalClusterMaintenanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<GkeonpremBareMetalClusterNetworkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_access_config` after provisioning.\n"]
    pub fn node_access_config(&self) -> ListRef<GkeonpremBareMetalClusterNodeAccessConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_access_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> ListRef<GkeonpremBareMetalClusterNodeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `os_environment_config` after provisioning.\n"]
    pub fn os_environment_config(
        &self,
    ) -> ListRef<GkeonpremBareMetalClusterOsEnvironmentConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.os_environment_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `proxy` after provisioning.\n"]
    pub fn proxy(&self) -> ListRef<GkeonpremBareMetalClusterProxyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.proxy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_config` after provisioning.\n"]
    pub fn security_config(&self) -> ListRef<GkeonpremBareMetalClusterSecurityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage` after provisioning.\n"]
    pub fn storage(&self) -> ListRef<GkeonpremBareMetalClusterStorageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeonpremBareMetalClusterTimeoutsElRef {
        GkeonpremBareMetalClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `upgrade_policy` after provisioning.\n"]
    pub fn upgrade_policy(&self) -> ListRef<GkeonpremBareMetalClusterUpgradePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.upgrade_policy", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterFleetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    membership: Option<PrimField<String>>,
}
impl GkeonpremBareMetalClusterFleetEl {
    #[doc = "Set the field `membership`.\n"]
    pub fn set_membership(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.membership = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremBareMetalClusterFleetEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterFleetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterFleetEl {}
impl BuildGkeonpremBareMetalClusterFleetEl {
    pub fn build(self) -> GkeonpremBareMetalClusterFleetEl {
        GkeonpremBareMetalClusterFleetEl {
            membership: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalClusterFleetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterFleetElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalClusterFleetElRef {
        GkeonpremBareMetalClusterFleetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterFleetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `membership` after provisioning.\n"]
    pub fn membership(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.membership", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterStatusElConditionsEl {
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
impl GkeonpremBareMetalClusterStatusElConditionsEl {
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
impl ToListMappable for GkeonpremBareMetalClusterStatusElConditionsEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterStatusElConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterStatusElConditionsEl {}
impl BuildGkeonpremBareMetalClusterStatusElConditionsEl {
    pub fn build(self) -> GkeonpremBareMetalClusterStatusElConditionsEl {
        GkeonpremBareMetalClusterStatusElConditionsEl {
            last_transition_time: core::default::Default::default(),
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
            state: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalClusterStatusElConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterStatusElConditionsElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalClusterStatusElConditionsElRef {
        GkeonpremBareMetalClusterStatusElConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterStatusElConditionsElRef {
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
pub struct GkeonpremBareMetalClusterStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions: Option<ListField<GkeonpremBareMetalClusterStatusElConditionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<PrimField<String>>,
}
impl GkeonpremBareMetalClusterStatusEl {
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl Into<ListField<GkeonpremBareMetalClusterStatusElConditionsEl>>,
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
impl ToListMappable for GkeonpremBareMetalClusterStatusEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterStatusEl {}
impl BuildGkeonpremBareMetalClusterStatusEl {
    pub fn build(self) -> GkeonpremBareMetalClusterStatusEl {
        GkeonpremBareMetalClusterStatusEl {
            conditions: core::default::Default::default(),
            error_message: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalClusterStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterStatusElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalClusterStatusElRef {
        GkeonpremBareMetalClusterStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> ListRef<GkeonpremBareMetalClusterStatusElConditionsElRef> {
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
pub struct GkeonpremBareMetalClusterValidationCheckElStatusElResultEl {
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
impl GkeonpremBareMetalClusterValidationCheckElStatusElResultEl {
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
impl ToListMappable for GkeonpremBareMetalClusterValidationCheckElStatusElResultEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterValidationCheckElStatusElResultEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterValidationCheckElStatusElResultEl {}
impl BuildGkeonpremBareMetalClusterValidationCheckElStatusElResultEl {
    pub fn build(self) -> GkeonpremBareMetalClusterValidationCheckElStatusElResultEl {
        GkeonpremBareMetalClusterValidationCheckElStatusElResultEl {
            category: core::default::Default::default(),
            description: core::default::Default::default(),
            details: core::default::Default::default(),
            options: core::default::Default::default(),
            reason: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalClusterValidationCheckElStatusElResultElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterValidationCheckElStatusElResultElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalClusterValidationCheckElStatusElResultElRef {
        GkeonpremBareMetalClusterValidationCheckElStatusElResultElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterValidationCheckElStatusElResultElRef {
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
pub struct GkeonpremBareMetalClusterValidationCheckElStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<ListField<GkeonpremBareMetalClusterValidationCheckElStatusElResultEl>>,
}
impl GkeonpremBareMetalClusterValidationCheckElStatusEl {
    #[doc = "Set the field `result`.\n"]
    pub fn set_result(
        mut self,
        v: impl Into<ListField<GkeonpremBareMetalClusterValidationCheckElStatusElResultEl>>,
    ) -> Self {
        self.result = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremBareMetalClusterValidationCheckElStatusEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterValidationCheckElStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterValidationCheckElStatusEl {}
impl BuildGkeonpremBareMetalClusterValidationCheckElStatusEl {
    pub fn build(self) -> GkeonpremBareMetalClusterValidationCheckElStatusEl {
        GkeonpremBareMetalClusterValidationCheckElStatusEl {
            result: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalClusterValidationCheckElStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterValidationCheckElStatusElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalClusterValidationCheckElStatusElRef {
        GkeonpremBareMetalClusterValidationCheckElStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterValidationCheckElStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `result` after provisioning.\n"]
    pub fn result(&self) -> ListRef<GkeonpremBareMetalClusterValidationCheckElStatusElResultElRef> {
        ListRef::new(self.shared().clone(), format!("{}.result", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterValidationCheckEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scenario: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<ListField<GkeonpremBareMetalClusterValidationCheckElStatusEl>>,
}
impl GkeonpremBareMetalClusterValidationCheckEl {
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
        v: impl Into<ListField<GkeonpremBareMetalClusterValidationCheckElStatusEl>>,
    ) -> Self {
        self.status = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremBareMetalClusterValidationCheckEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterValidationCheckEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterValidationCheckEl {}
impl BuildGkeonpremBareMetalClusterValidationCheckEl {
    pub fn build(self) -> GkeonpremBareMetalClusterValidationCheckEl {
        GkeonpremBareMetalClusterValidationCheckEl {
            options: core::default::Default::default(),
            scenario: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalClusterValidationCheckElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterValidationCheckElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalClusterValidationCheckElRef {
        GkeonpremBareMetalClusterValidationCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterValidationCheckElRef {
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
    pub fn status(&self) -> ListRef<GkeonpremBareMetalClusterValidationCheckElStatusElRef> {
        ListRef::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterBinaryAuthorizationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    evaluation_mode: Option<PrimField<String>>,
}
impl GkeonpremBareMetalClusterBinaryAuthorizationEl {
    #[doc = "Set the field `evaluation_mode`.\nMode of operation for binauthz policy evaluation. If unspecified,\ndefaults to DISABLED. Possible values: [\"DISABLED\", \"PROJECT_SINGLETON_POLICY_ENFORCE\"]"]
    pub fn set_evaluation_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.evaluation_mode = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremBareMetalClusterBinaryAuthorizationEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterBinaryAuthorizationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterBinaryAuthorizationEl {}
impl BuildGkeonpremBareMetalClusterBinaryAuthorizationEl {
    pub fn build(self) -> GkeonpremBareMetalClusterBinaryAuthorizationEl {
        GkeonpremBareMetalClusterBinaryAuthorizationEl {
            evaluation_mode: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalClusterBinaryAuthorizationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterBinaryAuthorizationElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalClusterBinaryAuthorizationElRef {
        GkeonpremBareMetalClusterBinaryAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterBinaryAuthorizationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `evaluation_mode` after provisioning.\nMode of operation for binauthz policy evaluation. If unspecified,\ndefaults to DISABLED. Possible values: [\"DISABLED\", \"PROJECT_SINGLETON_POLICY_ENFORCE\"]"]
    pub fn evaluation_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.evaluation_mode", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterClusterOperationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_application_logs: Option<PrimField<bool>>,
}
impl GkeonpremBareMetalClusterClusterOperationsEl {
    #[doc = "Set the field `enable_application_logs`.\nWhether collection of application logs/metrics should be enabled (in addition to system logs/metrics)."]
    pub fn set_enable_application_logs(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_application_logs = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremBareMetalClusterClusterOperationsEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterClusterOperationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterClusterOperationsEl {}
impl BuildGkeonpremBareMetalClusterClusterOperationsEl {
    pub fn build(self) -> GkeonpremBareMetalClusterClusterOperationsEl {
        GkeonpremBareMetalClusterClusterOperationsEl {
            enable_application_logs: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalClusterClusterOperationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterClusterOperationsElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalClusterClusterOperationsElRef {
        GkeonpremBareMetalClusterClusterOperationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterClusterOperationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_application_logs` after provisioning.\nWhether collection of application logs/metrics should be enabled (in addition to system logs/metrics)."]
    pub fn enable_application_logs(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_application_logs", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterControlPlaneElApiServerArgsEl {
    argument: PrimField<String>,
    value: PrimField<String>,
}
impl GkeonpremBareMetalClusterControlPlaneElApiServerArgsEl {}
impl ToListMappable for GkeonpremBareMetalClusterControlPlaneElApiServerArgsEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterControlPlaneElApiServerArgsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterControlPlaneElApiServerArgsEl {
    #[doc = "The argument name as it appears on the API Server command line please make sure to remove the leading dashes."]
    pub argument: PrimField<String>,
    #[doc = "The value of the arg as it will be passed to the API Server command line."]
    pub value: PrimField<String>,
}
impl BuildGkeonpremBareMetalClusterControlPlaneElApiServerArgsEl {
    pub fn build(self) -> GkeonpremBareMetalClusterControlPlaneElApiServerArgsEl {
        GkeonpremBareMetalClusterControlPlaneElApiServerArgsEl {
            argument: self.argument,
            value: self.value,
        }
    }
}
pub struct GkeonpremBareMetalClusterControlPlaneElApiServerArgsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterControlPlaneElApiServerArgsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalClusterControlPlaneElApiServerArgsElRef {
        GkeonpremBareMetalClusterControlPlaneElApiServerArgsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterControlPlaneElApiServerArgsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `argument` after provisioning.\nThe argument name as it appears on the API Server command line please make sure to remove the leading dashes."]
    pub fn argument(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.argument", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value of the arg as it will be passed to the API Server command line."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_ip: Option<PrimField<String>>,
}
impl
    GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl
{
    #[doc = "Set the field `labels`.\nThe map of Kubernetes labels (key/value pairs) to be applied to\neach node. These will added in addition to any default label(s)\nthat Kubernetes may apply to the node. In case of conflict in\nlabel keys, the applied set may differ depending on the Kubernetes\nversion -- it's best to assume the behavior is undefined and\nconflicts should be avoided. For more information, including usage\nand the valid values, see:\n  - http://kubernetes.io/v1.1/docs/user-guide/labels.html\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `node_ip`.\nThe default IPv4 address for SSH access and Kubernetes node.\nExample: 192.168.0.1"]
    pub fn set_node_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_ip = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl { type O = BlockAssignable < GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildGkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl
{}
impl BuildGkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl { pub fn build (self) -> GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl { GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl { labels : core :: default :: Default :: default () , node_ip : core :: default :: Default :: default () , } } }
pub struct GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsElRef { fn new (shared : StackShared , base : String) -> GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsElRef { GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsElRef { shared : shared , base : base . to_string () , } } }
impl GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `labels` after provisioning.\nThe map of Kubernetes labels (key/value pairs) to be applied to\neach node. These will added in addition to any default label(s)\nthat Kubernetes may apply to the node. In case of conflict in\nlabel keys, the applied set may differ depending on the Kubernetes\nversion -- it's best to assume the behavior is undefined and\nconflicts should be avoided. For more information, including usage\nand the valid values, see:\n  - http://kubernetes.io/v1.1/docs/user-guide/labels.html\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."] pub fn labels (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.labels" , self . base)) } # [doc = "Get a reference to the value of field `node_ip` after provisioning.\nThe default IPv4 address for SSH access and Kubernetes node.\nExample: 192.168.0.1"] pub fn node_ip (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.node_ip" , self . base)) } }
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl {
    #[doc = "Set the field `effect`.\nSpecifies the nodes operating system (default: LINUX). Possible values: [\"EFFECT_UNSPECIFIED\", \"PREFER_NO_SCHEDULE\", \"NO_EXECUTE\"]"]
    pub fn set_effect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.effect = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\nKey associated with the effect."]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\nValue associated with the effect."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl
{
    type O = BlockAssignable<
        GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl
{}
impl
    BuildGkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl
{
    pub fn build(
        self,
    ) -> GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl
    {
        GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl { effect : core :: default :: Default :: default () , key : core :: default :: Default :: default () , value : core :: default :: Default :: default () , }
    }
}
pub struct GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsElRef { fn new (shared : StackShared , base : String) -> GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsElRef { GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsElRef { shared : shared , base : base . to_string () , } } }
impl
    GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `effect` after provisioning.\nSpecifies the nodes operating system (default: LINUX). Possible values: [\"EFFECT_UNSPECIFIED\", \"PREFER_NO_SCHEDULE\", \"NO_EXECUTE\"]"]
    pub fn effect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nKey associated with the effect."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue associated with the effect."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElDynamic { node_configs : Option < DynamicBlock < GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl >> , taints : Option < DynamicBlock < GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl >> , }
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl { # [serde (skip_serializing_if = "Option::is_none")] labels : Option < RecField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] operating_system : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] node_configs : Option < Vec < GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl > > , # [serde (skip_serializing_if = "Option::is_none")] taints : Option < Vec < GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl > > , dynamic : GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElDynamic , }
impl GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl {
    #[doc = "Set the field `labels`.\nThe map of Kubernetes labels (key/value pairs) to be applied to\neach node. These will added in addition to any default label(s)\nthat Kubernetes may apply to the node. In case of conflict in\nlabel keys, the applied set may differ depending on the Kubernetes\nversion -- it's best to assume the behavior is undefined and\nconflicts should be avoided. For more information, including usage\nand the valid values, see:\n  - http://kubernetes.io/v1.1/docs/user-guide/labels.html\nAn object containing a list of \"key\": value pairs.\nFor example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `operating_system`.\nSpecifies the nodes operating system (default: LINUX)."]
    pub fn set_operating_system(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operating_system = Some(v.into());
        self
    }
    #[doc = "Set the field `node_configs`.\n"]
    pub fn set_node_configs(
        mut self,
        v : impl Into < BlockAssignable < GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.node_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.node_configs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `taints`.\n"]
    pub fn set_taints(
        mut self,
        v : impl Into < BlockAssignable < GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.taints = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.taints = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl
{
    type O = BlockAssignable<
        GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl
{}
impl BuildGkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl {
        GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl {
            labels: core::default::Default::default(),
            operating_system: core::default::Default::default(),
            node_configs: core::default::Default::default(),
            taints: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElRef
    {
        GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe map of Kubernetes labels (key/value pairs) to be applied to\neach node. These will added in addition to any default label(s)\nthat Kubernetes may apply to the node. In case of conflict in\nlabel keys, the applied set may differ depending on the Kubernetes\nversion -- it's best to assume the behavior is undefined and\nconflicts should be avoided. For more information, including usage\nand the valid values, see:\n  - http://kubernetes.io/v1.1/docs/user-guide/labels.html\nAn object containing a list of \"key\": value pairs.\nFor example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `operating_system` after provisioning.\nSpecifies the nodes operating system (default: LINUX)."]
    pub fn operating_system(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.operating_system", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_configs` after provisioning.\n"]    pub fn node_configs (& self) -> ListRef < GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.node_configs", self.base))
    }
    #[doc = "Get a reference to the value of field `taints` after provisioning.\n"]    pub fn taints (& self) -> ListRef < GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.taints", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElDynamic {
    node_pool_config: Option<
        DynamicBlock<
            GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    node_pool_config: Option<
        Vec<GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl>,
    >,
    dynamic: GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElDynamic,
}
impl GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigEl {
    #[doc = "Set the field `node_pool_config`.\n"]
    pub fn set_node_pool_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.node_pool_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.node_pool_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigEl {}
impl BuildGkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigEl {
        GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigEl {
            node_pool_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElRef {
        GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `node_pool_config` after provisioning.\n"]
    pub fn node_pool_config(
        &self,
    ) -> ListRef<
        GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_pool_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterControlPlaneElDynamic {
    api_server_args: Option<DynamicBlock<GkeonpremBareMetalClusterControlPlaneElApiServerArgsEl>>,
    control_plane_node_pool_config:
        Option<DynamicBlock<GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigEl>>,
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterControlPlaneEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_server_args: Option<Vec<GkeonpremBareMetalClusterControlPlaneElApiServerArgsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    control_plane_node_pool_config:
        Option<Vec<GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigEl>>,
    dynamic: GkeonpremBareMetalClusterControlPlaneElDynamic,
}
impl GkeonpremBareMetalClusterControlPlaneEl {
    #[doc = "Set the field `api_server_args`.\n"]
    pub fn set_api_server_args(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalClusterControlPlaneElApiServerArgsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.api_server_args = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.api_server_args = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `control_plane_node_pool_config`.\n"]
    pub fn set_control_plane_node_pool_config(
        mut self,
        v: impl Into<
            BlockAssignable<GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.control_plane_node_pool_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.control_plane_node_pool_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeonpremBareMetalClusterControlPlaneEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterControlPlaneEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterControlPlaneEl {}
impl BuildGkeonpremBareMetalClusterControlPlaneEl {
    pub fn build(self) -> GkeonpremBareMetalClusterControlPlaneEl {
        GkeonpremBareMetalClusterControlPlaneEl {
            api_server_args: core::default::Default::default(),
            control_plane_node_pool_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalClusterControlPlaneElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterControlPlaneElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalClusterControlPlaneElRef {
        GkeonpremBareMetalClusterControlPlaneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterControlPlaneElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_server_args` after provisioning.\n"]
    pub fn api_server_args(
        &self,
    ) -> ListRef<GkeonpremBareMetalClusterControlPlaneElApiServerArgsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.api_server_args", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `control_plane_node_pool_config` after provisioning.\n"]
    pub fn control_plane_node_pool_config(
        &self,
    ) -> ListRef<GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane_node_pool_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsEl {
    addresses: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avoid_buggy_ips: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manual_assign: Option<PrimField<bool>>,
    pool: PrimField<String>,
}
impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsEl {
    #[doc = "Set the field `avoid_buggy_ips`.\nIf true, avoid using IPs ending in .0 or .255.\nThis avoids buggy consumer devices mistakenly dropping IPv4 traffic for those special IP addresses."]
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
impl ToListMappable for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsEl {
    #[doc = "The addresses that are part of this pool. Each address must be either in the CIDR form (1.2.3.0/24) or range form (1.2.3.1-1.2.3.5)."]
    pub addresses: ListField<PrimField<String>>,
    #[doc = "The name of the address pool."]
    pub pool: PrimField<String>,
}
impl BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsEl {
    pub fn build(self) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsEl {
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsEl {
            addresses: self.addresses,
            avoid_buggy_ips: core::default::Default::default(),
            manual_assign: core::default::Default::default(),
            pool: self.pool,
        }
    }
}
pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsElRef {
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `addresses` after provisioning.\nThe addresses that are part of this pool. Each address must be either in the CIDR form (1.2.3.0/24) or range form (1.2.3.1-1.2.3.5)."]
    pub fn addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.addresses", self.base))
    }
    #[doc = "Get a reference to the value of field `avoid_buggy_ips` after provisioning.\nIf true, avoid using IPs ending in .0 or .255.\nThis avoids buggy consumer devices mistakenly dropping IPv4 traffic for those special IP addresses."]
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
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl {
    asn: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    control_plane_nodes: Option<ListField<PrimField<String>>>,
    ip_address: PrimField<String>,
}
impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl {
    #[doc = "Set the field `control_plane_nodes`.\nThe IP address of the control plane node that connects to the external\npeer.\nIf you don't specify any control plane nodes, all control plane nodes\ncan connect to the external peer. If you specify one or more IP addresses,\nonly the nodes specified participate in peering sessions."]
    pub fn set_control_plane_nodes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.control_plane_nodes = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl {
    #[doc = "BGP autonomous system number (ASN) for the network that contains the\nexternal peer device."]
    pub asn: PrimField<f64>,
    #[doc = "The IP address of the external peer device."]
    pub ip_address: PrimField<String>,
}
impl BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl {
    pub fn build(self) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl {
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl {
            asn: self.asn,
            control_plane_nodes: core::default::Default::default(),
            ip_address: self.ip_address,
        }
    }
}
pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsElRef {
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `asn` after provisioning.\nBGP autonomous system number (ASN) for the network that contains the\nexternal peer device."]
    pub fn asn(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.asn", self.base))
    }
    #[doc = "Get a reference to the value of field `control_plane_nodes` after provisioning.\nThe IP address of the control plane node that connects to the external\npeer.\nIf you don't specify any control plane nodes, all control plane nodes\ncan connect to the external peer. If you specify one or more IP addresses,\nonly the nodes specified participate in peering sessions."]
    pub fn control_plane_nodes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane_nodes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\nThe IP address of the external peer device."]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    registry_burst: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registry_pull_qps: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serialize_image_pulls_disabled: Option<PrimField<bool>>,
}
impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl { # [doc = "Set the field `registry_burst`.\nThe maximum size of bursty pulls, temporarily allows pulls to burst to this\nnumber, while still not exceeding registry_pull_qps.\nThe value must not be a negative number.\nUpdating this field may impact scalability by changing the amount of\ntraffic produced by image pulls.\nDefaults to 10."] pub fn set_registry_burst (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . registry_burst = Some (v . into ()) ; self } # [doc = "Set the field `registry_pull_qps`.\nThe limit of registry pulls per second.\nSetting this value to 0 means no limit.\nUpdating this field may impact scalability by changing the amount of\ntraffic produced by image pulls.\nDefaults to 5."] pub fn set_registry_pull_qps (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . registry_pull_qps = Some (v . into ()) ; self } # [doc = "Set the field `serialize_image_pulls_disabled`.\nPrevents the Kubelet from pulling multiple images at a time.\nWe recommend *not* changing the default value on nodes that run docker\ndaemon with version  < 1.9 or an Another Union File System (Aufs) storage\nbackend. Issue https://github.com/kubernetes/kubernetes/issues/10959 has\nmore details."] pub fn set_serialize_image_pulls_disabled (mut self , v : impl Into < PrimField < bool > >) -> Self { self . serialize_image_pulls_disabled = Some (v . into ()) ; self } }
impl ToListMappable for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl { type O = BlockAssignable < GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl
{}
impl BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl { pub fn build (self) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl { GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl { registry_burst : core :: default :: Default :: default () , registry_pull_qps : core :: default :: Default :: default () , serialize_image_pulls_disabled : core :: default :: Default :: default () , } } }
pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigElRef { fn new (shared : StackShared , base : String) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigElRef { GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigElRef { shared : shared , base : base . to_string () , } } }
impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `registry_burst` after provisioning.\nThe maximum size of bursty pulls, temporarily allows pulls to burst to this\nnumber, while still not exceeding registry_pull_qps.\nThe value must not be a negative number.\nUpdating this field may impact scalability by changing the amount of\ntraffic produced by image pulls.\nDefaults to 10."] pub fn registry_burst (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.registry_burst" , self . base)) } # [doc = "Get a reference to the value of field `registry_pull_qps` after provisioning.\nThe limit of registry pulls per second.\nSetting this value to 0 means no limit.\nUpdating this field may impact scalability by changing the amount of\ntraffic produced by image pulls.\nDefaults to 5."] pub fn registry_pull_qps (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.registry_pull_qps" , self . base)) } # [doc = "Get a reference to the value of field `serialize_image_pulls_disabled` after provisioning.\nPrevents the Kubelet from pulling multiple images at a time.\nWe recommend *not* changing the default value on nodes that run docker\ndaemon with version  < 1.9 or an Another Union File System (Aufs) storage\nbackend. Issue https://github.com/kubernetes/kubernetes/issues/10959 has\nmore details."] pub fn serialize_image_pulls_disabled (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.serialize_image_pulls_disabled" , self . base)) } }
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_ip: Option<PrimField<String>>,
}
impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl { # [doc = "Set the field `labels`.\nThe map of Kubernetes labels (key/value pairs) to be applied to\neach node. These will added in addition to any default label(s)\nthat Kubernetes may apply to the node. In case of conflict in\nlabel keys, the applied set may differ depending on the Kubernetes\nversion -- it's best to assume the behavior is undefined and\nconflicts should be avoided. For more information, including usage\nand the valid values, see:\n  - http://kubernetes.io/v1.1/docs/user-guide/labels.html\nAn object containing a list of \"key\": value pairs.\nFor example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."] pub fn set_labels (mut self , v : impl Into < RecField < PrimField < String > > >) -> Self { self . labels = Some (v . into ()) ; self } # [doc = "Set the field `node_ip`.\nThe default IPv4 address for SSH access and Kubernetes node.\nExample: 192.168.0.1"] pub fn set_node_ip (mut self , v : impl Into < PrimField < String > >) -> Self { self . node_ip = Some (v . into ()) ; self } }
impl ToListMappable for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl { type O = BlockAssignable < GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl
{}
impl BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl { pub fn build (self) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl { GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl { labels : core :: default :: Default :: default () , node_ip : core :: default :: Default :: default () , } } }
pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef { fn new (shared : StackShared , base : String) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef { GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef { shared : shared , base : base . to_string () , } } }
impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `labels` after provisioning.\nThe map of Kubernetes labels (key/value pairs) to be applied to\neach node. These will added in addition to any default label(s)\nthat Kubernetes may apply to the node. In case of conflict in\nlabel keys, the applied set may differ depending on the Kubernetes\nversion -- it's best to assume the behavior is undefined and\nconflicts should be avoided. For more information, including usage\nand the valid values, see:\n  - http://kubernetes.io/v1.1/docs/user-guide/labels.html\nAn object containing a list of \"key\": value pairs.\nFor example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."] pub fn labels (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.labels" , self . base)) } # [doc = "Get a reference to the value of field `node_ip` after provisioning.\nThe default IPv4 address for SSH access and Kubernetes node.\nExample: 192.168.0.1"] pub fn node_ip (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.node_ip" , self . base)) } }
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl { # [doc = "Set the field `effect`.\nSpecifies the nodes operating system (default: LINUX). Possible values: [\"EFFECT_UNSPECIFIED\", \"PREFER_NO_SCHEDULE\", \"NO_EXECUTE\"]"] pub fn set_effect (mut self , v : impl Into < PrimField < String > >) -> Self { self . effect = Some (v . into ()) ; self } # [doc = "Set the field `key`.\nKey associated with the effect."] pub fn set_key (mut self , v : impl Into < PrimField < String > >) -> Self { self . key = Some (v . into ()) ; self } # [doc = "Set the field `value`.\nValue associated with the effect."] pub fn set_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . value = Some (v . into ()) ; self } }
impl ToListMappable for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl { type O = BlockAssignable < GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl
{}
impl BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl { pub fn build (self) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl { GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl { effect : core :: default :: Default :: default () , key : core :: default :: Default :: default () , value : core :: default :: Default :: default () , } } }
pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef { fn new (shared : StackShared , base : String) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef { GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef { shared : shared , base : base . to_string () , } } }
impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `effect` after provisioning.\nSpecifies the nodes operating system (default: LINUX). Possible values: [\"EFFECT_UNSPECIFIED\", \"PREFER_NO_SCHEDULE\", \"NO_EXECUTE\"]"] pub fn effect (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.effect" , self . base)) } # [doc = "Get a reference to the value of field `key` after provisioning.\nKey associated with the effect."] pub fn key (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.key" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\nValue associated with the effect."] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElDynamic { kubelet_config : Option < DynamicBlock < GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl >> , node_configs : Option < DynamicBlock < GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl >> , taints : Option < DynamicBlock < GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl >> , }
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl { # [serde (skip_serializing_if = "Option::is_none")] labels : Option < RecField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] operating_system : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] kubelet_config : Option < Vec < GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] node_configs : Option < Vec < GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl > > , # [serde (skip_serializing_if = "Option::is_none")] taints : Option < Vec < GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl > > , dynamic : GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElDynamic , }
impl
    GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl
{
    #[doc = "Set the field `labels`.\nThe map of Kubernetes labels (key/value pairs) to be applied to\neach node. These will added in addition to any default label(s)\nthat Kubernetes may apply to the node. In case of conflict in\nlabel keys, the applied set may differ depending on the Kubernetes\nversion -- it's best to assume the behavior is undefined and\nconflicts should be avoided. For more information, including usage\nand the valid values, see:\n  - http://kubernetes.io/v1.1/docs/user-guide/labels.html\nAn object containing a list of \"key\": value pairs.\nFor example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `operating_system`.\nSpecifies the nodes operating system (default: LINUX)."]
    pub fn set_operating_system(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operating_system = Some(v.into());
        self
    }
    #[doc = "Set the field `kubelet_config`.\n"]
    pub fn set_kubelet_config(
        mut self,
        v : impl Into < BlockAssignable < GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl >>,
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
    #[doc = "Set the field `node_configs`.\n"]
    pub fn set_node_configs(
        mut self,
        v : impl Into < BlockAssignable < GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.node_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.node_configs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `taints`.\n"]
    pub fn set_taints(
        mut self,
        v : impl Into < BlockAssignable < GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.taints = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.taints = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl { type O = BlockAssignable < GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl
{}
impl BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl { pub fn build (self) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl { GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl { labels : core :: default :: Default :: default () , operating_system : core :: default :: Default :: default () , kubelet_config : core :: default :: Default :: default () , node_configs : core :: default :: Default :: default () , taints : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef { fn new (shared : StackShared , base : String) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef { GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef { shared : shared , base : base . to_string () , } } }
impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `labels` after provisioning.\nThe map of Kubernetes labels (key/value pairs) to be applied to\neach node. These will added in addition to any default label(s)\nthat Kubernetes may apply to the node. In case of conflict in\nlabel keys, the applied set may differ depending on the Kubernetes\nversion -- it's best to assume the behavior is undefined and\nconflicts should be avoided. For more information, including usage\nand the valid values, see:\n  - http://kubernetes.io/v1.1/docs/user-guide/labels.html\nAn object containing a list of \"key\": value pairs.\nFor example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."] pub fn labels (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.labels" , self . base)) } # [doc = "Get a reference to the value of field `operating_system` after provisioning.\nSpecifies the nodes operating system (default: LINUX)."] pub fn operating_system (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.operating_system" , self . base)) } # [doc = "Get a reference to the value of field `kubelet_config` after provisioning.\n"] pub fn kubelet_config (& self) -> ListRef < GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.kubelet_config" , self . base)) } # [doc = "Get a reference to the value of field `node_configs` after provisioning.\n"] pub fn node_configs (& self) -> ListRef < GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.node_configs" , self . base)) } # [doc = "Get a reference to the value of field `taints` after provisioning.\n"] pub fn taints (& self) -> ListRef < GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.taints" , self . base)) } }
#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElDynamic { node_pool_config : Option < DynamicBlock < GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl >> , }
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl { # [serde (skip_serializing_if = "Option::is_none")] node_pool_config : Option < Vec < GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl > > , dynamic : GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElDynamic , }
impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl {
    #[doc = "Set the field `node_pool_config`.\n"]
    pub fn set_node_pool_config(
        mut self,
        v : impl Into < BlockAssignable < GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.node_pool_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.node_pool_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl
{
    type O = BlockAssignable<
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl {
}
impl BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl {
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl {
            node_pool_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElRef {
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `node_pool_config` after provisioning.\n"]    pub fn node_pool_config (& self) -> ListRef < GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_pool_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElDynamic {
    address_pools:
        Option<DynamicBlock<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsEl>>,
    bgp_peer_configs:
        Option<DynamicBlock<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl>>,
    load_balancer_node_pool_config: Option<
        DynamicBlock<
            GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigEl {
    asn: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_pools: Option<Vec<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bgp_peer_configs:
        Option<Vec<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancer_node_pool_config: Option<
        Vec<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl>,
    >,
    dynamic: GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElDynamic,
}
impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigEl {
    #[doc = "Set the field `address_pools`.\n"]
    pub fn set_address_pools(
        mut self,
        v: impl Into<
            BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsEl>,
        >,
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
    #[doc = "Set the field `bgp_peer_configs`.\n"]
    pub fn set_bgp_peer_configs(
        mut self,
        v: impl Into<
            BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.bgp_peer_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.bgp_peer_configs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `load_balancer_node_pool_config`.\n"]
    pub fn set_load_balancer_node_pool_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.load_balancer_node_pool_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.load_balancer_node_pool_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigEl {
    #[doc = "BGP autonomous system number (ASN) of the cluster.\nThis field can be updated after cluster creation."]
    pub asn: PrimField<f64>,
}
impl BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigEl {
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigEl {
            asn: self.asn,
            address_pools: core::default::Default::default(),
            bgp_peer_configs: core::default::Default::default(),
            load_balancer_node_pool_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElRef {
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `asn` after provisioning.\nBGP autonomous system number (ASN) of the cluster.\nThis field can be updated after cluster creation."]
    pub fn asn(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.asn", self.base))
    }
    #[doc = "Get a reference to the value of field `address_pools` after provisioning.\n"]
    pub fn address_pools(
        &self,
    ) -> ListRef<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.address_pools", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `bgp_peer_configs` after provisioning.\n"]
    pub fn bgp_peer_configs(
        &self,
    ) -> ListRef<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bgp_peer_configs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancer_node_pool_config` after provisioning.\n"]
    pub fn load_balancer_node_pool_config(
        &self,
    ) -> ListRef<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.load_balancer_node_pool_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElManualLbConfigEl {
    enabled: PrimField<bool>,
}
impl GkeonpremBareMetalClusterLoadBalancerElManualLbConfigEl {}
impl ToListMappable for GkeonpremBareMetalClusterLoadBalancerElManualLbConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElManualLbConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterLoadBalancerElManualLbConfigEl {
    #[doc = "Whether manual load balancing is enabled."]
    pub enabled: PrimField<bool>,
}
impl BuildGkeonpremBareMetalClusterLoadBalancerElManualLbConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterLoadBalancerElManualLbConfigEl {
        GkeonpremBareMetalClusterLoadBalancerElManualLbConfigEl {
            enabled: self.enabled,
        }
    }
}
pub struct GkeonpremBareMetalClusterLoadBalancerElManualLbConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterLoadBalancerElManualLbConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalClusterLoadBalancerElManualLbConfigElRef {
        GkeonpremBareMetalClusterLoadBalancerElManualLbConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterLoadBalancerElManualLbConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether manual load balancing is enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsEl {
    addresses: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avoid_buggy_ips: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manual_assign: Option<PrimField<bool>>,
    pool: PrimField<String>,
}
impl GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsEl {
    #[doc = "Set the field `avoid_buggy_ips`.\nIf true, avoid using IPs ending in .0 or .255.\nThis avoids buggy consumer devices mistakenly dropping IPv4 traffic for those special IP addresses."]
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
impl ToListMappable for GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsEl {
    #[doc = "The addresses that are part of this pool. Each address must be either in the CIDR form (1.2.3.0/24) or range form (1.2.3.1-1.2.3.5)."]
    pub addresses: ListField<PrimField<String>>,
    #[doc = "The name of the address pool."]
    pub pool: PrimField<String>,
}
impl BuildGkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsEl {
    pub fn build(self) -> GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsEl {
        GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsEl {
            addresses: self.addresses,
            avoid_buggy_ips: core::default::Default::default(),
            manual_assign: core::default::Default::default(),
            pool: self.pool,
        }
    }
}
pub struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsElRef {
        GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `addresses` after provisioning.\nThe addresses that are part of this pool. Each address must be either in the CIDR form (1.2.3.0/24) or range form (1.2.3.1-1.2.3.5)."]
    pub fn addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.addresses", self.base))
    }
    #[doc = "Get a reference to the value of field `avoid_buggy_ips` after provisioning.\nIf true, avoid using IPs ending in .0 or .255.\nThis avoids buggy consumer devices mistakenly dropping IPv4 traffic for those special IP addresses."]
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
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_ip: Option<PrimField<String>>,
}
impl GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl { # [doc = "Set the field `labels`.\nThe map of Kubernetes labels (key/value pairs) to be applied to\neach node. These will added in addition to any default label(s)\nthat Kubernetes may apply to the node. In case of conflict in\nlabel keys, the applied set may differ depending on the Kubernetes\nversion -- it's best to assume the behavior is undefined and\nconflicts should be avoided. For more information, including usage\nand the valid values, see:\n  - http://kubernetes.io/v1.1/docs/user-guide/labels.html\nAn object containing a list of \"key\": value pairs.\nFor example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."] pub fn set_labels (mut self , v : impl Into < RecField < PrimField < String > > >) -> Self { self . labels = Some (v . into ()) ; self } # [doc = "Set the field `node_ip`.\nThe default IPv4 address for SSH access and Kubernetes node.\nExample: 192.168.0.1"] pub fn set_node_ip (mut self , v : impl Into < PrimField < String > >) -> Self { self . node_ip = Some (v . into ()) ; self } }
impl ToListMappable for GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl { type O = BlockAssignable < GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildGkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl
{}
impl BuildGkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl { pub fn build (self) -> GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl { GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl { labels : core :: default :: Default :: default () , node_ip : core :: default :: Default :: default () , } } }
pub struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef { fn new (shared : StackShared , base : String) -> GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef { GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef { shared : shared , base : base . to_string () , } } }
impl GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `labels` after provisioning.\nThe map of Kubernetes labels (key/value pairs) to be applied to\neach node. These will added in addition to any default label(s)\nthat Kubernetes may apply to the node. In case of conflict in\nlabel keys, the applied set may differ depending on the Kubernetes\nversion -- it's best to assume the behavior is undefined and\nconflicts should be avoided. For more information, including usage\nand the valid values, see:\n  - http://kubernetes.io/v1.1/docs/user-guide/labels.html\nAn object containing a list of \"key\": value pairs.\nFor example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."] pub fn labels (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.labels" , self . base)) } # [doc = "Get a reference to the value of field `node_ip` after provisioning.\nThe default IPv4 address for SSH access and Kubernetes node.\nExample: 192.168.0.1"] pub fn node_ip (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.node_ip" , self . base)) } }
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl { # [doc = "Set the field `effect`.\nSpecifies the nodes operating system (default: LINUX). Possible values: [\"EFFECT_UNSPECIFIED\", \"PREFER_NO_SCHEDULE\", \"NO_EXECUTE\"]"] pub fn set_effect (mut self , v : impl Into < PrimField < String > >) -> Self { self . effect = Some (v . into ()) ; self } # [doc = "Set the field `key`.\nKey associated with the effect."] pub fn set_key (mut self , v : impl Into < PrimField < String > >) -> Self { self . key = Some (v . into ()) ; self } # [doc = "Set the field `value`.\nValue associated with the effect."] pub fn set_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . value = Some (v . into ()) ; self } }
impl ToListMappable for GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl { type O = BlockAssignable < GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildGkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl
{}
impl BuildGkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl { pub fn build (self) -> GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl { GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl { effect : core :: default :: Default :: default () , key : core :: default :: Default :: default () , value : core :: default :: Default :: default () , } } }
pub struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef { fn new (shared : StackShared , base : String) -> GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef { GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef { shared : shared , base : base . to_string () , } } }
impl GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `effect` after provisioning.\nSpecifies the nodes operating system (default: LINUX). Possible values: [\"EFFECT_UNSPECIFIED\", \"PREFER_NO_SCHEDULE\", \"NO_EXECUTE\"]"] pub fn effect (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.effect" , self . base)) } # [doc = "Get a reference to the value of field `key` after provisioning.\nKey associated with the effect."] pub fn key (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.key" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\nValue associated with the effect."] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElDynamic { node_configs : Option < DynamicBlock < GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl >> , taints : Option < DynamicBlock < GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl >> , }
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl { # [serde (skip_serializing_if = "Option::is_none")] labels : Option < RecField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] operating_system : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] node_configs : Option < Vec < GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl > > , # [serde (skip_serializing_if = "Option::is_none")] taints : Option < Vec < GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl > > , dynamic : GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElDynamic , }
impl GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl { # [doc = "Set the field `labels`.\nThe map of Kubernetes labels (key/value pairs) to be applied to\neach node. These will added in addition to any default label(s)\nthat Kubernetes may apply to the node. In case of conflict in\nlabel keys, the applied set may differ depending on the Kubernetes\nversion -- it's best to assume the behavior is undefined and\nconflicts should be avoided. For more information, including usage\nand the valid values, see:\n  - http://kubernetes.io/v1.1/docs/user-guide/labels.html\nAn object containing a list of \"key\": value pairs.\nFor example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."] pub fn set_labels (mut self , v : impl Into < RecField < PrimField < String > > >) -> Self { self . labels = Some (v . into ()) ; self } # [doc = "Set the field `operating_system`.\nSpecifies the nodes operating system (default: LINUX)."] pub fn set_operating_system (mut self , v : impl Into < PrimField < String > >) -> Self { self . operating_system = Some (v . into ()) ; self } # [doc = "Set the field `node_configs`.\n"] pub fn set_node_configs (mut self , v : impl Into < BlockAssignable < GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . node_configs = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . node_configs = Some (d) ; } } self } # [doc = "Set the field `taints`.\n"] pub fn set_taints (mut self , v : impl Into < BlockAssignable < GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . taints = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . taints = Some (d) ; } } self } }
impl ToListMappable for GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl { type O = BlockAssignable < GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildGkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl
{}
impl BuildGkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl { pub fn build (self) -> GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl { GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl { labels : core :: default :: Default :: default () , operating_system : core :: default :: Default :: default () , node_configs : core :: default :: Default :: default () , taints : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef { fn new (shared : StackShared , base : String) -> GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef { GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef { shared : shared , base : base . to_string () , } } }
impl GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `labels` after provisioning.\nThe map of Kubernetes labels (key/value pairs) to be applied to\neach node. These will added in addition to any default label(s)\nthat Kubernetes may apply to the node. In case of conflict in\nlabel keys, the applied set may differ depending on the Kubernetes\nversion -- it's best to assume the behavior is undefined and\nconflicts should be avoided. For more information, including usage\nand the valid values, see:\n  - http://kubernetes.io/v1.1/docs/user-guide/labels.html\nAn object containing a list of \"key\": value pairs.\nFor example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."] pub fn labels (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.labels" , self . base)) } # [doc = "Get a reference to the value of field `operating_system` after provisioning.\nSpecifies the nodes operating system (default: LINUX)."] pub fn operating_system (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.operating_system" , self . base)) } # [doc = "Get a reference to the value of field `node_configs` after provisioning.\n"] pub fn node_configs (& self) -> ListRef < GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.node_configs" , self . base)) } # [doc = "Get a reference to the value of field `taints` after provisioning.\n"] pub fn taints (& self) -> ListRef < GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.taints" , self . base)) } }
#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElDynamic { node_pool_config : Option < DynamicBlock < GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl >> , }
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigEl { # [serde (skip_serializing_if = "Option::is_none")] node_pool_config : Option < Vec < GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl > > , dynamic : GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElDynamic , }
impl GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigEl {
    #[doc = "Set the field `node_pool_config`.\n"]
    pub fn set_node_pool_config(
        mut self,
        v : impl Into < BlockAssignable < GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.node_pool_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.node_pool_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigEl
{
    type O = BlockAssignable<
        GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigEl
{}
impl BuildGkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigEl {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigEl {
        GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigEl {
            node_pool_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElRef {
        GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `node_pool_config` after provisioning.\n"]    pub fn node_pool_config (& self) -> ListRef < GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_pool_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElDynamic {
    address_pools:
        Option<DynamicBlock<GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsEl>>,
    load_balancer_node_pool_config: Option<
        DynamicBlock<
            GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address_pools:
        Option<Vec<GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancer_node_pool_config: Option<
        Vec<GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigEl>,
    >,
    dynamic: GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElDynamic,
}
impl GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigEl {
    #[doc = "Set the field `address_pools`.\n"]
    pub fn set_address_pools(
        mut self,
        v: impl Into<
            BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsEl>,
        >,
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
    #[doc = "Set the field `load_balancer_node_pool_config`.\n"]
    pub fn set_load_balancer_node_pool_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.load_balancer_node_pool_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.load_balancer_node_pool_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterLoadBalancerElMetalLbConfigEl {}
impl BuildGkeonpremBareMetalClusterLoadBalancerElMetalLbConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigEl {
        GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigEl {
            address_pools: core::default::Default::default(),
            load_balancer_node_pool_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElRef {
        GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address_pools` after provisioning.\n"]
    pub fn address_pools(
        &self,
    ) -> ListRef<GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.address_pools", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancer_node_pool_config` after provisioning.\n"]
    pub fn load_balancer_node_pool_config(
        &self,
    ) -> ListRef<
        GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.load_balancer_node_pool_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElPortConfigEl {
    control_plane_load_balancer_port: PrimField<f64>,
}
impl GkeonpremBareMetalClusterLoadBalancerElPortConfigEl {}
impl ToListMappable for GkeonpremBareMetalClusterLoadBalancerElPortConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElPortConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterLoadBalancerElPortConfigEl {
    #[doc = "The port that control plane hosted load balancers will listen on."]
    pub control_plane_load_balancer_port: PrimField<f64>,
}
impl BuildGkeonpremBareMetalClusterLoadBalancerElPortConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterLoadBalancerElPortConfigEl {
        GkeonpremBareMetalClusterLoadBalancerElPortConfigEl {
            control_plane_load_balancer_port: self.control_plane_load_balancer_port,
        }
    }
}
pub struct GkeonpremBareMetalClusterLoadBalancerElPortConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterLoadBalancerElPortConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalClusterLoadBalancerElPortConfigElRef {
        GkeonpremBareMetalClusterLoadBalancerElPortConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterLoadBalancerElPortConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `control_plane_load_balancer_port` after provisioning.\nThe port that control plane hosted load balancers will listen on."]
    pub fn control_plane_load_balancer_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.control_plane_load_balancer_port", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElVipConfigEl {
    control_plane_vip: PrimField<String>,
    ingress_vip: PrimField<String>,
}
impl GkeonpremBareMetalClusterLoadBalancerElVipConfigEl {}
impl ToListMappable for GkeonpremBareMetalClusterLoadBalancerElVipConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElVipConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterLoadBalancerElVipConfigEl {
    #[doc = "The VIP which you previously set aside for the Kubernetes API of this Bare Metal User Cluster."]
    pub control_plane_vip: PrimField<String>,
    #[doc = "The VIP which you previously set aside for ingress traffic into this Bare Metal User Cluster."]
    pub ingress_vip: PrimField<String>,
}
impl BuildGkeonpremBareMetalClusterLoadBalancerElVipConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterLoadBalancerElVipConfigEl {
        GkeonpremBareMetalClusterLoadBalancerElVipConfigEl {
            control_plane_vip: self.control_plane_vip,
            ingress_vip: self.ingress_vip,
        }
    }
}
pub struct GkeonpremBareMetalClusterLoadBalancerElVipConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterLoadBalancerElVipConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalClusterLoadBalancerElVipConfigElRef {
        GkeonpremBareMetalClusterLoadBalancerElVipConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterLoadBalancerElVipConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `control_plane_vip` after provisioning.\nThe VIP which you previously set aside for the Kubernetes API of this Bare Metal User Cluster."]
    pub fn control_plane_vip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.control_plane_vip", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ingress_vip` after provisioning.\nThe VIP which you previously set aside for ingress traffic into this Bare Metal User Cluster."]
    pub fn ingress_vip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ingress_vip", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterLoadBalancerElDynamic {
    bgp_lb_config: Option<DynamicBlock<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigEl>>,
    manual_lb_config: Option<DynamicBlock<GkeonpremBareMetalClusterLoadBalancerElManualLbConfigEl>>,
    metal_lb_config: Option<DynamicBlock<GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigEl>>,
    port_config: Option<DynamicBlock<GkeonpremBareMetalClusterLoadBalancerElPortConfigEl>>,
    vip_config: Option<DynamicBlock<GkeonpremBareMetalClusterLoadBalancerElVipConfigEl>>,
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bgp_lb_config: Option<Vec<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manual_lb_config: Option<Vec<GkeonpremBareMetalClusterLoadBalancerElManualLbConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metal_lb_config: Option<Vec<GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_config: Option<Vec<GkeonpremBareMetalClusterLoadBalancerElPortConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vip_config: Option<Vec<GkeonpremBareMetalClusterLoadBalancerElVipConfigEl>>,
    dynamic: GkeonpremBareMetalClusterLoadBalancerElDynamic,
}
impl GkeonpremBareMetalClusterLoadBalancerEl {
    #[doc = "Set the field `bgp_lb_config`.\n"]
    pub fn set_bgp_lb_config(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.bgp_lb_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.bgp_lb_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `manual_lb_config`.\n"]
    pub fn set_manual_lb_config(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElManualLbConfigEl>>,
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
        v: impl Into<BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigEl>>,
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
    #[doc = "Set the field `port_config`.\n"]
    pub fn set_port_config(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElPortConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.port_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.port_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `vip_config`.\n"]
    pub fn set_vip_config(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElVipConfigEl>>,
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
impl ToListMappable for GkeonpremBareMetalClusterLoadBalancerEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterLoadBalancerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterLoadBalancerEl {}
impl BuildGkeonpremBareMetalClusterLoadBalancerEl {
    pub fn build(self) -> GkeonpremBareMetalClusterLoadBalancerEl {
        GkeonpremBareMetalClusterLoadBalancerEl {
            bgp_lb_config: core::default::Default::default(),
            manual_lb_config: core::default::Default::default(),
            metal_lb_config: core::default::Default::default(),
            port_config: core::default::Default::default(),
            vip_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalClusterLoadBalancerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterLoadBalancerElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalClusterLoadBalancerElRef {
        GkeonpremBareMetalClusterLoadBalancerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterLoadBalancerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bgp_lb_config` after provisioning.\n"]
    pub fn bgp_lb_config(
        &self,
    ) -> ListRef<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bgp_lb_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `manual_lb_config` after provisioning.\n"]
    pub fn manual_lb_config(
        &self,
    ) -> ListRef<GkeonpremBareMetalClusterLoadBalancerElManualLbConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.manual_lb_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metal_lb_config` after provisioning.\n"]
    pub fn metal_lb_config(
        &self,
    ) -> ListRef<GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metal_lb_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `port_config` after provisioning.\n"]
    pub fn port_config(&self) -> ListRef<GkeonpremBareMetalClusterLoadBalancerElPortConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.port_config", self.base))
    }
    #[doc = "Get a reference to the value of field `vip_config` after provisioning.\n"]
    pub fn vip_config(&self) -> ListRef<GkeonpremBareMetalClusterLoadBalancerElVipConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vip_config", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterMaintenanceConfigEl {
    maintenance_address_cidr_blocks: ListField<PrimField<String>>,
}
impl GkeonpremBareMetalClusterMaintenanceConfigEl {}
impl ToListMappable for GkeonpremBareMetalClusterMaintenanceConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterMaintenanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterMaintenanceConfigEl {
    #[doc = "All IPv4 address from these ranges will be placed into maintenance mode.\nNodes in maintenance mode will be cordoned and drained. When both of these\nare true, the \"baremetal.cluster.gke.io/maintenance\" annotation will be set\non the node resource."]
    pub maintenance_address_cidr_blocks: ListField<PrimField<String>>,
}
impl BuildGkeonpremBareMetalClusterMaintenanceConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterMaintenanceConfigEl {
        GkeonpremBareMetalClusterMaintenanceConfigEl {
            maintenance_address_cidr_blocks: self.maintenance_address_cidr_blocks,
        }
    }
}
pub struct GkeonpremBareMetalClusterMaintenanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterMaintenanceConfigElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalClusterMaintenanceConfigElRef {
        GkeonpremBareMetalClusterMaintenanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterMaintenanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `maintenance_address_cidr_blocks` after provisioning.\nAll IPv4 address from these ranges will be placed into maintenance mode.\nNodes in maintenance mode will be cordoned and drained. When both of these\nare true, the \"baremetal.cluster.gke.io/maintenance\" annotation will be set\non the node resource."]
    pub fn maintenance_address_cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_address_cidr_blocks", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrEl {
    pod_address_cidr_blocks: ListField<PrimField<String>>,
    service_address_cidr_blocks: ListField<PrimField<String>>,
}
impl GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrEl {}
impl ToListMappable for GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterNetworkConfigElIslandModeCidrEl {
    #[doc = "All pods in the cluster are assigned an RFC1918 IPv4 address from these ranges. This field cannot be changed after creation."]
    pub pod_address_cidr_blocks: ListField<PrimField<String>>,
    #[doc = "All services in the cluster are assigned an RFC1918 IPv4 address from these ranges. This field cannot be changed after creation."]
    pub service_address_cidr_blocks: ListField<PrimField<String>>,
}
impl BuildGkeonpremBareMetalClusterNetworkConfigElIslandModeCidrEl {
    pub fn build(self) -> GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrEl {
        GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrEl {
            pod_address_cidr_blocks: self.pod_address_cidr_blocks,
            service_address_cidr_blocks: self.service_address_cidr_blocks,
        }
    }
}
pub struct GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrElRef {
        GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pod_address_cidr_blocks` after provisioning.\nAll pods in the cluster are assigned an RFC1918 IPv4 address from these ranges. This field cannot be changed after creation."]
    pub fn pod_address_cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pod_address_cidr_blocks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_address_cidr_blocks` after provisioning.\nAll services in the cluster are assigned an RFC1918 IPv4 address from these ranges. This field cannot be changed after creation."]
    pub fn service_address_cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_address_cidr_blocks", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigEl {
    #[doc = "Set the field `enabled`.\nWhether to enable multiple network interfaces for your pods.\nWhen set network_config.advanced_networking is automatically\nset to true."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigEl {
    type O =
        BlockAssignable<GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigEl {}
impl BuildGkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigEl {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigEl {
        GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigElRef {
        GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether to enable multiple network interfaces for your pods.\nWhen set network_config.advanced_networking is automatically\nset to true."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterNetworkConfigElSrIovConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl GkeonpremBareMetalClusterNetworkConfigElSrIovConfigEl {
    #[doc = "Set the field `enabled`.\nWhether to install the SR-IOV operator."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremBareMetalClusterNetworkConfigElSrIovConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterNetworkConfigElSrIovConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterNetworkConfigElSrIovConfigEl {}
impl BuildGkeonpremBareMetalClusterNetworkConfigElSrIovConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterNetworkConfigElSrIovConfigEl {
        GkeonpremBareMetalClusterNetworkConfigElSrIovConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalClusterNetworkConfigElSrIovConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterNetworkConfigElSrIovConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalClusterNetworkConfigElSrIovConfigElRef {
        GkeonpremBareMetalClusterNetworkConfigElSrIovConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterNetworkConfigElSrIovConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether to install the SR-IOV operator."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterNetworkConfigElDynamic {
    island_mode_cidr:
        Option<DynamicBlock<GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrEl>>,
    multiple_network_interfaces_config: Option<
        DynamicBlock<GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigEl>,
    >,
    sr_iov_config: Option<DynamicBlock<GkeonpremBareMetalClusterNetworkConfigElSrIovConfigEl>>,
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterNetworkConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_networking: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    island_mode_cidr: Option<Vec<GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multiple_network_interfaces_config:
        Option<Vec<GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sr_iov_config: Option<Vec<GkeonpremBareMetalClusterNetworkConfigElSrIovConfigEl>>,
    dynamic: GkeonpremBareMetalClusterNetworkConfigElDynamic,
}
impl GkeonpremBareMetalClusterNetworkConfigEl {
    #[doc = "Set the field `advanced_networking`.\nEnables the use of advanced Anthos networking features, such as Bundled\nLoad Balancing with BGP or the egress NAT gateway.\nSetting configuration for advanced networking features will automatically\nset this flag."]
    pub fn set_advanced_networking(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.advanced_networking = Some(v.into());
        self
    }
    #[doc = "Set the field `island_mode_cidr`.\n"]
    pub fn set_island_mode_cidr(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.island_mode_cidr = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.island_mode_cidr = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `multiple_network_interfaces_config`.\n"]
    pub fn set_multiple_network_interfaces_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.multiple_network_interfaces_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.multiple_network_interfaces_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sr_iov_config`.\n"]
    pub fn set_sr_iov_config(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalClusterNetworkConfigElSrIovConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sr_iov_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sr_iov_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeonpremBareMetalClusterNetworkConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterNetworkConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterNetworkConfigEl {}
impl BuildGkeonpremBareMetalClusterNetworkConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterNetworkConfigEl {
        GkeonpremBareMetalClusterNetworkConfigEl {
            advanced_networking: core::default::Default::default(),
            island_mode_cidr: core::default::Default::default(),
            multiple_network_interfaces_config: core::default::Default::default(),
            sr_iov_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalClusterNetworkConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterNetworkConfigElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalClusterNetworkConfigElRef {
        GkeonpremBareMetalClusterNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterNetworkConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `advanced_networking` after provisioning.\nEnables the use of advanced Anthos networking features, such as Bundled\nLoad Balancing with BGP or the egress NAT gateway.\nSetting configuration for advanced networking features will automatically\nset this flag."]
    pub fn advanced_networking(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.advanced_networking", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `island_mode_cidr` after provisioning.\n"]
    pub fn island_mode_cidr(
        &self,
    ) -> ListRef<GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.island_mode_cidr", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `multiple_network_interfaces_config` after provisioning.\n"]
    pub fn multiple_network_interfaces_config(
        &self,
    ) -> ListRef<GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.multiple_network_interfaces_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sr_iov_config` after provisioning.\n"]
    pub fn sr_iov_config(
        &self,
    ) -> ListRef<GkeonpremBareMetalClusterNetworkConfigElSrIovConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sr_iov_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterNodeAccessConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    login_user: Option<PrimField<String>>,
}
impl GkeonpremBareMetalClusterNodeAccessConfigEl {
    #[doc = "Set the field `login_user`.\nLoginUser is the user name used to access node machines.\nIt defaults to \"root\" if not set."]
    pub fn set_login_user(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.login_user = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremBareMetalClusterNodeAccessConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterNodeAccessConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterNodeAccessConfigEl {}
impl BuildGkeonpremBareMetalClusterNodeAccessConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterNodeAccessConfigEl {
        GkeonpremBareMetalClusterNodeAccessConfigEl {
            login_user: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalClusterNodeAccessConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterNodeAccessConfigElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalClusterNodeAccessConfigElRef {
        GkeonpremBareMetalClusterNodeAccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterNodeAccessConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `login_user` after provisioning.\nLoginUser is the user name used to access node machines.\nIt defaults to \"root\" if not set."]
    pub fn login_user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.login_user", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterNodeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_runtime: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_pods_per_node: Option<PrimField<f64>>,
}
impl GkeonpremBareMetalClusterNodeConfigEl {
    #[doc = "Set the field `container_runtime`.\nThe available runtimes that can be used to run containers in a Bare Metal User Cluster. Possible values: [\"CONTAINER_RUNTIME_UNSPECIFIED\", \"DOCKER\", \"CONTAINERD\"]"]
    pub fn set_container_runtime(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.container_runtime = Some(v.into());
        self
    }
    #[doc = "Set the field `max_pods_per_node`.\nThe maximum number of pods a node can run. The size of the CIDR range\nassigned to the node will be derived from this parameter."]
    pub fn set_max_pods_per_node(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_pods_per_node = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremBareMetalClusterNodeConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterNodeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterNodeConfigEl {}
impl BuildGkeonpremBareMetalClusterNodeConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterNodeConfigEl {
        GkeonpremBareMetalClusterNodeConfigEl {
            container_runtime: core::default::Default::default(),
            max_pods_per_node: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalClusterNodeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterNodeConfigElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalClusterNodeConfigElRef {
        GkeonpremBareMetalClusterNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterNodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container_runtime` after provisioning.\nThe available runtimes that can be used to run containers in a Bare Metal User Cluster. Possible values: [\"CONTAINER_RUNTIME_UNSPECIFIED\", \"DOCKER\", \"CONTAINERD\"]"]
    pub fn container_runtime(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_runtime", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_pods_per_node` after provisioning.\nThe maximum number of pods a node can run. The size of the CIDR range\nassigned to the node will be derived from this parameter."]
    pub fn max_pods_per_node(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_pods_per_node", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterOsEnvironmentConfigEl {
    package_repo_excluded: PrimField<bool>,
}
impl GkeonpremBareMetalClusterOsEnvironmentConfigEl {}
impl ToListMappable for GkeonpremBareMetalClusterOsEnvironmentConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterOsEnvironmentConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterOsEnvironmentConfigEl {
    #[doc = "Whether the package repo should not be included when initializing\nbare metal machines."]
    pub package_repo_excluded: PrimField<bool>,
}
impl BuildGkeonpremBareMetalClusterOsEnvironmentConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterOsEnvironmentConfigEl {
        GkeonpremBareMetalClusterOsEnvironmentConfigEl {
            package_repo_excluded: self.package_repo_excluded,
        }
    }
}
pub struct GkeonpremBareMetalClusterOsEnvironmentConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterOsEnvironmentConfigElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalClusterOsEnvironmentConfigElRef {
        GkeonpremBareMetalClusterOsEnvironmentConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterOsEnvironmentConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `package_repo_excluded` after provisioning.\nWhether the package repo should not be included when initializing\nbare metal machines."]
    pub fn package_repo_excluded(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.package_repo_excluded", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterProxyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    no_proxy: Option<ListField<PrimField<String>>>,
    uri: PrimField<String>,
}
impl GkeonpremBareMetalClusterProxyEl {
    #[doc = "Set the field `no_proxy`.\nA list of IPs, hostnames, and domains that should skip the proxy.\nFor example [\"127.0.0.1\", \"example.com\", \".corp\", \"localhost\"]."]
    pub fn set_no_proxy(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.no_proxy = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremBareMetalClusterProxyEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterProxyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterProxyEl {
    #[doc = "Specifies the address of your proxy server.\nFor example: http://domain\nWARNING: Do not provide credentials in the format\nof http://(username:password@)domain these will be rejected by the server."]
    pub uri: PrimField<String>,
}
impl BuildGkeonpremBareMetalClusterProxyEl {
    pub fn build(self) -> GkeonpremBareMetalClusterProxyEl {
        GkeonpremBareMetalClusterProxyEl {
            no_proxy: core::default::Default::default(),
            uri: self.uri,
        }
    }
}
pub struct GkeonpremBareMetalClusterProxyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterProxyElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalClusterProxyElRef {
        GkeonpremBareMetalClusterProxyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterProxyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `no_proxy` after provisioning.\nA list of IPs, hostnames, and domains that should skip the proxy.\nFor example [\"127.0.0.1\", \"example.com\", \".corp\", \"localhost\"]."]
    pub fn no_proxy(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.no_proxy", self.base))
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nSpecifies the address of your proxy server.\nFor example: http://domain\nWARNING: Do not provide credentials in the format\nof http://(username:password@)domain these will be rejected by the server."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersEl {
    username: PrimField<String>,
}
impl GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersEl {}
impl ToListMappable for GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersEl {
    #[doc = "The name of the user, e.g. 'my-gcp-id@gmail.com'."]
    pub username: PrimField<String>,
}
impl BuildGkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersEl {
    pub fn build(self) -> GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersEl {
        GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersEl {
            username: self.username,
        }
    }
}
pub struct GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersElRef {
        GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nThe name of the user, e.g. 'my-gcp-id@gmail.com'."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterSecurityConfigElAuthorizationElDynamic {
    admin_users:
        Option<DynamicBlock<GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersEl>>,
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterSecurityConfigElAuthorizationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_users: Option<Vec<GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersEl>>,
    dynamic: GkeonpremBareMetalClusterSecurityConfigElAuthorizationElDynamic,
}
impl GkeonpremBareMetalClusterSecurityConfigElAuthorizationEl {
    #[doc = "Set the field `admin_users`.\n"]
    pub fn set_admin_users(
        mut self,
        v: impl Into<
            BlockAssignable<GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersEl>,
        >,
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
impl ToListMappable for GkeonpremBareMetalClusterSecurityConfigElAuthorizationEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterSecurityConfigElAuthorizationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterSecurityConfigElAuthorizationEl {}
impl BuildGkeonpremBareMetalClusterSecurityConfigElAuthorizationEl {
    pub fn build(self) -> GkeonpremBareMetalClusterSecurityConfigElAuthorizationEl {
        GkeonpremBareMetalClusterSecurityConfigElAuthorizationEl {
            admin_users: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalClusterSecurityConfigElAuthorizationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterSecurityConfigElAuthorizationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalClusterSecurityConfigElAuthorizationElRef {
        GkeonpremBareMetalClusterSecurityConfigElAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterSecurityConfigElAuthorizationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `admin_users` after provisioning.\n"]
    pub fn admin_users(
        &self,
    ) -> ListRef<GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.admin_users", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterSecurityConfigElDynamic {
    authorization: Option<DynamicBlock<GkeonpremBareMetalClusterSecurityConfigElAuthorizationEl>>,
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterSecurityConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization: Option<Vec<GkeonpremBareMetalClusterSecurityConfigElAuthorizationEl>>,
    dynamic: GkeonpremBareMetalClusterSecurityConfigElDynamic,
}
impl GkeonpremBareMetalClusterSecurityConfigEl {
    #[doc = "Set the field `authorization`.\n"]
    pub fn set_authorization(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalClusterSecurityConfigElAuthorizationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.authorization = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.authorization = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeonpremBareMetalClusterSecurityConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterSecurityConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterSecurityConfigEl {}
impl BuildGkeonpremBareMetalClusterSecurityConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterSecurityConfigEl {
        GkeonpremBareMetalClusterSecurityConfigEl {
            authorization: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalClusterSecurityConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterSecurityConfigElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalClusterSecurityConfigElRef {
        GkeonpremBareMetalClusterSecurityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterSecurityConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(
        &self,
    ) -> ListRef<GkeonpremBareMetalClusterSecurityConfigElAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorization", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigEl {
    path: PrimField<String>,
    storage_class: PrimField<String>,
}
impl GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigEl {}
impl ToListMappable for GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterStorageElLvpNodeMountsConfigEl {
    #[doc = "The host machine path."]
    pub path: PrimField<String>,
    #[doc = "The StorageClass name that PVs will be created with."]
    pub storage_class: PrimField<String>,
}
impl BuildGkeonpremBareMetalClusterStorageElLvpNodeMountsConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigEl {
        GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigEl {
            path: self.path,
            storage_class: self.storage_class,
        }
    }
}
pub struct GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigElRef {
        GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nThe host machine path."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\nThe StorageClass name that PVs will be created with."]
    pub fn storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_class", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigEl {
    path: PrimField<String>,
    storage_class: PrimField<String>,
}
impl GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigEl {}
impl ToListMappable for GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigEl {
    #[doc = "The host machine path."]
    pub path: PrimField<String>,
    #[doc = "The StorageClass name that PVs will be created with."]
    pub storage_class: PrimField<String>,
}
impl BuildGkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigEl {
        GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigEl {
            path: self.path,
            storage_class: self.storage_class,
        }
    }
}
pub struct GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigElRef {
        GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nThe host machine path."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\nThe StorageClass name that PVs will be created with."]
    pub fn storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_class", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterStorageElLvpShareConfigElDynamic {
    lvp_config: Option<DynamicBlock<GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigEl>>,
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterStorageElLvpShareConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    shared_path_pv_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lvp_config: Option<Vec<GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigEl>>,
    dynamic: GkeonpremBareMetalClusterStorageElLvpShareConfigElDynamic,
}
impl GkeonpremBareMetalClusterStorageElLvpShareConfigEl {
    #[doc = "Set the field `shared_path_pv_count`.\nThe number of subdirectories to create under path."]
    pub fn set_shared_path_pv_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.shared_path_pv_count = Some(v.into());
        self
    }
    #[doc = "Set the field `lvp_config`.\n"]
    pub fn set_lvp_config(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lvp_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lvp_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeonpremBareMetalClusterStorageElLvpShareConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterStorageElLvpShareConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterStorageElLvpShareConfigEl {}
impl BuildGkeonpremBareMetalClusterStorageElLvpShareConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterStorageElLvpShareConfigEl {
        GkeonpremBareMetalClusterStorageElLvpShareConfigEl {
            shared_path_pv_count: core::default::Default::default(),
            lvp_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalClusterStorageElLvpShareConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterStorageElLvpShareConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalClusterStorageElLvpShareConfigElRef {
        GkeonpremBareMetalClusterStorageElLvpShareConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterStorageElLvpShareConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `shared_path_pv_count` after provisioning.\nThe number of subdirectories to create under path."]
    pub fn shared_path_pv_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.shared_path_pv_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lvp_config` after provisioning.\n"]
    pub fn lvp_config(
        &self,
    ) -> ListRef<GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lvp_config", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterStorageElDynamic {
    lvp_node_mounts_config:
        Option<DynamicBlock<GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigEl>>,
    lvp_share_config: Option<DynamicBlock<GkeonpremBareMetalClusterStorageElLvpShareConfigEl>>,
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterStorageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lvp_node_mounts_config: Option<Vec<GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lvp_share_config: Option<Vec<GkeonpremBareMetalClusterStorageElLvpShareConfigEl>>,
    dynamic: GkeonpremBareMetalClusterStorageElDynamic,
}
impl GkeonpremBareMetalClusterStorageEl {
    #[doc = "Set the field `lvp_node_mounts_config`.\n"]
    pub fn set_lvp_node_mounts_config(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lvp_node_mounts_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lvp_node_mounts_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `lvp_share_config`.\n"]
    pub fn set_lvp_share_config(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalClusterStorageElLvpShareConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lvp_share_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lvp_share_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeonpremBareMetalClusterStorageEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterStorageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterStorageEl {}
impl BuildGkeonpremBareMetalClusterStorageEl {
    pub fn build(self) -> GkeonpremBareMetalClusterStorageEl {
        GkeonpremBareMetalClusterStorageEl {
            lvp_node_mounts_config: core::default::Default::default(),
            lvp_share_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalClusterStorageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterStorageElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalClusterStorageElRef {
        GkeonpremBareMetalClusterStorageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterStorageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `lvp_node_mounts_config` after provisioning.\n"]
    pub fn lvp_node_mounts_config(
        &self,
    ) -> ListRef<GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lvp_node_mounts_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lvp_share_config` after provisioning.\n"]
    pub fn lvp_share_config(
        &self,
    ) -> ListRef<GkeonpremBareMetalClusterStorageElLvpShareConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lvp_share_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl GkeonpremBareMetalClusterTimeoutsEl {
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
impl ToListMappable for GkeonpremBareMetalClusterTimeoutsEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterTimeoutsEl {}
impl BuildGkeonpremBareMetalClusterTimeoutsEl {
    pub fn build(self) -> GkeonpremBareMetalClusterTimeoutsEl {
        GkeonpremBareMetalClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalClusterTimeoutsElRef {
        GkeonpremBareMetalClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterTimeoutsElRef {
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
pub struct GkeonpremBareMetalClusterUpgradePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<PrimField<String>>,
}
impl GkeonpremBareMetalClusterUpgradePolicyEl {
    #[doc = "Set the field `policy`.\nSpecifies which upgrade policy to use. Possible values: [\"SERIAL\", \"CONCURRENT\"]"]
    pub fn set_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremBareMetalClusterUpgradePolicyEl {
    type O = BlockAssignable<GkeonpremBareMetalClusterUpgradePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalClusterUpgradePolicyEl {}
impl BuildGkeonpremBareMetalClusterUpgradePolicyEl {
    pub fn build(self) -> GkeonpremBareMetalClusterUpgradePolicyEl {
        GkeonpremBareMetalClusterUpgradePolicyEl {
            policy: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalClusterUpgradePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalClusterUpgradePolicyElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalClusterUpgradePolicyElRef {
        GkeonpremBareMetalClusterUpgradePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalClusterUpgradePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `policy` after provisioning.\nSpecifies which upgrade policy to use. Possible values: [\"SERIAL\", \"CONCURRENT\"]"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterDynamic {
    binary_authorization: Option<DynamicBlock<GkeonpremBareMetalClusterBinaryAuthorizationEl>>,
    cluster_operations: Option<DynamicBlock<GkeonpremBareMetalClusterClusterOperationsEl>>,
    control_plane: Option<DynamicBlock<GkeonpremBareMetalClusterControlPlaneEl>>,
    load_balancer: Option<DynamicBlock<GkeonpremBareMetalClusterLoadBalancerEl>>,
    maintenance_config: Option<DynamicBlock<GkeonpremBareMetalClusterMaintenanceConfigEl>>,
    network_config: Option<DynamicBlock<GkeonpremBareMetalClusterNetworkConfigEl>>,
    node_access_config: Option<DynamicBlock<GkeonpremBareMetalClusterNodeAccessConfigEl>>,
    node_config: Option<DynamicBlock<GkeonpremBareMetalClusterNodeConfigEl>>,
    os_environment_config: Option<DynamicBlock<GkeonpremBareMetalClusterOsEnvironmentConfigEl>>,
    proxy: Option<DynamicBlock<GkeonpremBareMetalClusterProxyEl>>,
    security_config: Option<DynamicBlock<GkeonpremBareMetalClusterSecurityConfigEl>>,
    storage: Option<DynamicBlock<GkeonpremBareMetalClusterStorageEl>>,
    upgrade_policy: Option<DynamicBlock<GkeonpremBareMetalClusterUpgradePolicyEl>>,
}
