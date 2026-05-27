use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct GkeonpremBareMetalAdminClusterData {
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
    bare_metal_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_operations: Option<Vec<GkeonpremBareMetalAdminClusterClusterOperationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    control_plane: Option<Vec<GkeonpremBareMetalAdminClusterControlPlaneEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancer: Option<Vec<GkeonpremBareMetalAdminClusterLoadBalancerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_config: Option<Vec<GkeonpremBareMetalAdminClusterMaintenanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_config: Option<Vec<GkeonpremBareMetalAdminClusterNetworkConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_access_config: Option<Vec<GkeonpremBareMetalAdminClusterNodeAccessConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_config: Option<Vec<GkeonpremBareMetalAdminClusterNodeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy: Option<Vec<GkeonpremBareMetalAdminClusterProxyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_config: Option<Vec<GkeonpremBareMetalAdminClusterSecurityConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage: Option<Vec<GkeonpremBareMetalAdminClusterStorageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<GkeonpremBareMetalAdminClusterTimeoutsEl>,
    dynamic: GkeonpremBareMetalAdminClusterDynamic,
}
struct GkeonpremBareMetalAdminCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GkeonpremBareMetalAdminClusterData>,
}
#[derive(Clone)]
pub struct GkeonpremBareMetalAdminCluster(Rc<GkeonpremBareMetalAdminCluster_>);
impl GkeonpremBareMetalAdminCluster {
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
    #[doc = "Set the field `annotations`.\nAnnotations on the Bare Metal Admin Cluster.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `bare_metal_version`.\nA human readable description of this Bare Metal Admin Cluster."]
    pub fn set_bare_metal_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().bare_metal_version = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nA human readable description of this Bare Metal Admin Cluster."]
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
    #[doc = "Set the field `cluster_operations`.\n"]
    pub fn set_cluster_operations(
        self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalAdminClusterClusterOperationsEl>>,
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
        v: impl Into<BlockAssignable<GkeonpremBareMetalAdminClusterControlPlaneEl>>,
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
        v: impl Into<BlockAssignable<GkeonpremBareMetalAdminClusterLoadBalancerEl>>,
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
        v: impl Into<BlockAssignable<GkeonpremBareMetalAdminClusterMaintenanceConfigEl>>,
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
        v: impl Into<BlockAssignable<GkeonpremBareMetalAdminClusterNetworkConfigEl>>,
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
        v: impl Into<BlockAssignable<GkeonpremBareMetalAdminClusterNodeAccessConfigEl>>,
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
        v: impl Into<BlockAssignable<GkeonpremBareMetalAdminClusterNodeConfigEl>>,
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
    #[doc = "Set the field `proxy`.\n"]
    pub fn set_proxy(
        self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalAdminClusterProxyEl>>,
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
        v: impl Into<BlockAssignable<GkeonpremBareMetalAdminClusterSecurityConfigEl>>,
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
        v: impl Into<BlockAssignable<GkeonpremBareMetalAdminClusterStorageEl>>,
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
    pub fn set_timeouts(self, v: impl Into<GkeonpremBareMetalAdminClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nAnnotations on the Bare Metal Admin Cluster.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bare_metal_version` after provisioning.\nA human readable description of this Bare Metal Admin Cluster."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human readable description of this Bare Metal Admin Cluster."]
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
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nThe IP address name of Bare Metal Admin Cluster's API server."]
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
    pub fn fleet(&self) -> ListRef<GkeonpremBareMetalAdminClusterFleetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fleet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `local_name` after provisioning.\nThe object name of the Bare Metal Admin Cluster custom resource on the\nassociated admin cluster. This field is used to support conflicting\nnames when enrolling existing clusters to the API. When used as a part of\ncluster enrollment, this field will differ from the ID in the resource\nname. For new clusters, this field will match the user provided cluster ID\nand be visible in the last component of the resource name. It is not\nmodifiable.\nAll users should use this name to access their cluster using gkectl or\nkubectl and should expect to see the local name when viewing admin\ncluster controller logs."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe bare metal admin cluster name."]
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
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the Bare Metal Admin Cluster."]
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
    pub fn status(&self) -> ListRef<GkeonpremBareMetalAdminClusterStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe unique identifier of the Bare Metal Admin Cluster."]
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
    #[doc = "Get a reference to the value of field `validation_check` after provisioning.\nSpecifies the security related settings for the Bare Metal Admin Cluster."]
    pub fn validation_check(&self) -> ListRef<GkeonpremBareMetalAdminClusterValidationCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.validation_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_operations` after provisioning.\n"]
    pub fn cluster_operations(
        &self,
    ) -> ListRef<GkeonpremBareMetalAdminClusterClusterOperationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_operations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `control_plane` after provisioning.\n"]
    pub fn control_plane(&self) -> ListRef<GkeonpremBareMetalAdminClusterControlPlaneElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancer` after provisioning.\n"]
    pub fn load_balancer(&self) -> ListRef<GkeonpremBareMetalAdminClusterLoadBalancerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.load_balancer", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_config` after provisioning.\n"]
    pub fn maintenance_config(
        &self,
    ) -> ListRef<GkeonpremBareMetalAdminClusterMaintenanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<GkeonpremBareMetalAdminClusterNetworkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_access_config` after provisioning.\n"]
    pub fn node_access_config(
        &self,
    ) -> ListRef<GkeonpremBareMetalAdminClusterNodeAccessConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_access_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> ListRef<GkeonpremBareMetalAdminClusterNodeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `proxy` after provisioning.\n"]
    pub fn proxy(&self) -> ListRef<GkeonpremBareMetalAdminClusterProxyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.proxy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_config` after provisioning.\n"]
    pub fn security_config(&self) -> ListRef<GkeonpremBareMetalAdminClusterSecurityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage` after provisioning.\n"]
    pub fn storage(&self) -> ListRef<GkeonpremBareMetalAdminClusterStorageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeonpremBareMetalAdminClusterTimeoutsElRef {
        GkeonpremBareMetalAdminClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for GkeonpremBareMetalAdminCluster {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for GkeonpremBareMetalAdminCluster {}
impl ToListMappable for GkeonpremBareMetalAdminCluster {
    type O = ListRef<GkeonpremBareMetalAdminClusterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for GkeonpremBareMetalAdminCluster_ {
    fn extract_resource_type(&self) -> String {
        "google_gkeonprem_bare_metal_admin_cluster".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildGkeonpremBareMetalAdminCluster {
    pub tf_id: String,
    #[doc = "The location of the resource."]
    pub location: PrimField<String>,
    #[doc = "The bare metal admin cluster name."]
    pub name: PrimField<String>,
}
impl BuildGkeonpremBareMetalAdminCluster {
    pub fn build(self, stack: &mut Stack) -> GkeonpremBareMetalAdminCluster {
        let out = GkeonpremBareMetalAdminCluster(Rc::new(GkeonpremBareMetalAdminCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GkeonpremBareMetalAdminClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                bare_metal_version: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                cluster_operations: core::default::Default::default(),
                control_plane: core::default::Default::default(),
                load_balancer: core::default::Default::default(),
                maintenance_config: core::default::Default::default(),
                network_config: core::default::Default::default(),
                node_access_config: core::default::Default::default(),
                node_config: core::default::Default::default(),
                proxy: core::default::Default::default(),
                security_config: core::default::Default::default(),
                storage: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct GkeonpremBareMetalAdminClusterRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl GkeonpremBareMetalAdminClusterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nAnnotations on the Bare Metal Admin Cluster.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bare_metal_version` after provisioning.\nA human readable description of this Bare Metal Admin Cluster."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human readable description of this Bare Metal Admin Cluster."]
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
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nThe IP address name of Bare Metal Admin Cluster's API server."]
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
    pub fn fleet(&self) -> ListRef<GkeonpremBareMetalAdminClusterFleetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fleet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `local_name` after provisioning.\nThe object name of the Bare Metal Admin Cluster custom resource on the\nassociated admin cluster. This field is used to support conflicting\nnames when enrolling existing clusters to the API. When used as a part of\ncluster enrollment, this field will differ from the ID in the resource\nname. For new clusters, this field will match the user provided cluster ID\nand be visible in the last component of the resource name. It is not\nmodifiable.\nAll users should use this name to access their cluster using gkectl or\nkubectl and should expect to see the local name when viewing admin\ncluster controller logs."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe bare metal admin cluster name."]
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
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the Bare Metal Admin Cluster."]
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
    pub fn status(&self) -> ListRef<GkeonpremBareMetalAdminClusterStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe unique identifier of the Bare Metal Admin Cluster."]
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
    #[doc = "Get a reference to the value of field `validation_check` after provisioning.\nSpecifies the security related settings for the Bare Metal Admin Cluster."]
    pub fn validation_check(&self) -> ListRef<GkeonpremBareMetalAdminClusterValidationCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.validation_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_operations` after provisioning.\n"]
    pub fn cluster_operations(
        &self,
    ) -> ListRef<GkeonpremBareMetalAdminClusterClusterOperationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_operations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `control_plane` after provisioning.\n"]
    pub fn control_plane(&self) -> ListRef<GkeonpremBareMetalAdminClusterControlPlaneElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancer` after provisioning.\n"]
    pub fn load_balancer(&self) -> ListRef<GkeonpremBareMetalAdminClusterLoadBalancerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.load_balancer", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_config` after provisioning.\n"]
    pub fn maintenance_config(
        &self,
    ) -> ListRef<GkeonpremBareMetalAdminClusterMaintenanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<GkeonpremBareMetalAdminClusterNetworkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_access_config` after provisioning.\n"]
    pub fn node_access_config(
        &self,
    ) -> ListRef<GkeonpremBareMetalAdminClusterNodeAccessConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_access_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> ListRef<GkeonpremBareMetalAdminClusterNodeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `proxy` after provisioning.\n"]
    pub fn proxy(&self) -> ListRef<GkeonpremBareMetalAdminClusterProxyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.proxy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_config` after provisioning.\n"]
    pub fn security_config(&self) -> ListRef<GkeonpremBareMetalAdminClusterSecurityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage` after provisioning.\n"]
    pub fn storage(&self) -> ListRef<GkeonpremBareMetalAdminClusterStorageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeonpremBareMetalAdminClusterTimeoutsElRef {
        GkeonpremBareMetalAdminClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterFleetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    membership: Option<PrimField<String>>,
}
impl GkeonpremBareMetalAdminClusterFleetEl {
    #[doc = "Set the field `membership`.\n"]
    pub fn set_membership(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.membership = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremBareMetalAdminClusterFleetEl {
    type O = BlockAssignable<GkeonpremBareMetalAdminClusterFleetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterFleetEl {}
impl BuildGkeonpremBareMetalAdminClusterFleetEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterFleetEl {
        GkeonpremBareMetalAdminClusterFleetEl {
            membership: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterFleetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterFleetElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalAdminClusterFleetElRef {
        GkeonpremBareMetalAdminClusterFleetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterFleetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `membership` after provisioning.\n"]
    pub fn membership(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.membership", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterStatusElConditionsEl {
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
impl GkeonpremBareMetalAdminClusterStatusElConditionsEl {
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
impl ToListMappable for GkeonpremBareMetalAdminClusterStatusElConditionsEl {
    type O = BlockAssignable<GkeonpremBareMetalAdminClusterStatusElConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterStatusElConditionsEl {}
impl BuildGkeonpremBareMetalAdminClusterStatusElConditionsEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterStatusElConditionsEl {
        GkeonpremBareMetalAdminClusterStatusElConditionsEl {
            last_transition_time: core::default::Default::default(),
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
            state: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterStatusElConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterStatusElConditionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalAdminClusterStatusElConditionsElRef {
        GkeonpremBareMetalAdminClusterStatusElConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterStatusElConditionsElRef {
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
pub struct GkeonpremBareMetalAdminClusterStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions: Option<ListField<GkeonpremBareMetalAdminClusterStatusElConditionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<PrimField<String>>,
}
impl GkeonpremBareMetalAdminClusterStatusEl {
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl Into<ListField<GkeonpremBareMetalAdminClusterStatusElConditionsEl>>,
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
impl ToListMappable for GkeonpremBareMetalAdminClusterStatusEl {
    type O = BlockAssignable<GkeonpremBareMetalAdminClusterStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterStatusEl {}
impl BuildGkeonpremBareMetalAdminClusterStatusEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterStatusEl {
        GkeonpremBareMetalAdminClusterStatusEl {
            conditions: core::default::Default::default(),
            error_message: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterStatusElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalAdminClusterStatusElRef {
        GkeonpremBareMetalAdminClusterStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> ListRef<GkeonpremBareMetalAdminClusterStatusElConditionsElRef> {
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
pub struct GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultEl {
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
impl GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultEl {
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
impl ToListMappable for GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultEl {
    type O = BlockAssignable<GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterValidationCheckElStatusElResultEl {}
impl BuildGkeonpremBareMetalAdminClusterValidationCheckElStatusElResultEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultEl {
        GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultEl {
            category: core::default::Default::default(),
            description: core::default::Default::default(),
            details: core::default::Default::default(),
            options: core::default::Default::default(),
            reason: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultElRef {
        GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultElRef {
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
pub struct GkeonpremBareMetalAdminClusterValidationCheckElStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<ListField<GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultEl>>,
}
impl GkeonpremBareMetalAdminClusterValidationCheckElStatusEl {
    #[doc = "Set the field `result`.\n"]
    pub fn set_result(
        mut self,
        v: impl Into<ListField<GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultEl>>,
    ) -> Self {
        self.result = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremBareMetalAdminClusterValidationCheckElStatusEl {
    type O = BlockAssignable<GkeonpremBareMetalAdminClusterValidationCheckElStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterValidationCheckElStatusEl {}
impl BuildGkeonpremBareMetalAdminClusterValidationCheckElStatusEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterValidationCheckElStatusEl {
        GkeonpremBareMetalAdminClusterValidationCheckElStatusEl {
            result: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterValidationCheckElStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterValidationCheckElStatusElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalAdminClusterValidationCheckElStatusElRef {
        GkeonpremBareMetalAdminClusterValidationCheckElStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterValidationCheckElStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `result` after provisioning.\n"]
    pub fn result(
        &self,
    ) -> ListRef<GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultElRef> {
        ListRef::new(self.shared().clone(), format!("{}.result", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterValidationCheckEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scenario: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<ListField<GkeonpremBareMetalAdminClusterValidationCheckElStatusEl>>,
}
impl GkeonpremBareMetalAdminClusterValidationCheckEl {
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
        v: impl Into<ListField<GkeonpremBareMetalAdminClusterValidationCheckElStatusEl>>,
    ) -> Self {
        self.status = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremBareMetalAdminClusterValidationCheckEl {
    type O = BlockAssignable<GkeonpremBareMetalAdminClusterValidationCheckEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterValidationCheckEl {}
impl BuildGkeonpremBareMetalAdminClusterValidationCheckEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterValidationCheckEl {
        GkeonpremBareMetalAdminClusterValidationCheckEl {
            options: core::default::Default::default(),
            scenario: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterValidationCheckElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterValidationCheckElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalAdminClusterValidationCheckElRef {
        GkeonpremBareMetalAdminClusterValidationCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterValidationCheckElRef {
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
    pub fn status(&self) -> ListRef<GkeonpremBareMetalAdminClusterValidationCheckElStatusElRef> {
        ListRef::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterClusterOperationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_application_logs: Option<PrimField<bool>>,
}
impl GkeonpremBareMetalAdminClusterClusterOperationsEl {
    #[doc = "Set the field `enable_application_logs`.\nWhether collection of application logs/metrics should be enabled (in addition to system logs/metrics)."]
    pub fn set_enable_application_logs(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_application_logs = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremBareMetalAdminClusterClusterOperationsEl {
    type O = BlockAssignable<GkeonpremBareMetalAdminClusterClusterOperationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterClusterOperationsEl {}
impl BuildGkeonpremBareMetalAdminClusterClusterOperationsEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterClusterOperationsEl {
        GkeonpremBareMetalAdminClusterClusterOperationsEl {
            enable_application_logs: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterClusterOperationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterClusterOperationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalAdminClusterClusterOperationsElRef {
        GkeonpremBareMetalAdminClusterClusterOperationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterClusterOperationsElRef {
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
pub struct GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsEl {
    argument: PrimField<String>,
    value: PrimField<String>,
}
impl GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsEl {}
impl ToListMappable for GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsEl {
    type O = BlockAssignable<GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsEl {
    #[doc = "The argument name as it appears on the API Server command line please make sure to remove the leading dashes."]
    pub argument: PrimField<String>,
    #[doc = "The value of the arg as it will be passed to the API Server command line."]
    pub value: PrimField<String>,
}
impl BuildGkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsEl {
        GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsEl {
            argument: self.argument,
            value: self.value,
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsElRef {
        GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsElRef {
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
pub struct GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_ip: Option<PrimField<String>>,
}
impl GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl { # [doc = "Set the field `labels`.\nThe map of Kubernetes labels (key/value pairs) to be applied to\neach node. These will added in addition to any default label(s)\nthat Kubernetes may apply to the node. In case of conflict in\nlabel keys, the applied set may differ depending on the Kubernetes\nversion -- it's best to assume the behavior is undefined and\nconflicts should be avoided. For more information, including usage\nand the valid values, see:\n  - http://kubernetes.io/v1.1/docs/user-guide/labels.html\nAn object containing a list of \"key\": value pairs.\nFor example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."] pub fn set_labels (mut self , v : impl Into < RecField < PrimField < String > > >) -> Self { self . labels = Some (v . into ()) ; self } # [doc = "Set the field `node_ip`.\nThe default IPv4 address for SSH access and Kubernetes node.\nExample: 192.168.0.1"] pub fn set_node_ip (mut self , v : impl Into < PrimField < String > >) -> Self { self . node_ip = Some (v . into ()) ; self } }
impl ToListMappable for GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl { type O = BlockAssignable < GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildGkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl
{}
impl BuildGkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl { pub fn build (self) -> GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl { GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl { labels : core :: default :: Default :: default () , node_ip : core :: default :: Default :: default () , } } }
pub struct GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsElRef { fn new (shared : StackShared , base : String) -> GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsElRef { GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsElRef { shared : shared , base : base . to_string () , } } }
impl GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `labels` after provisioning.\nThe map of Kubernetes labels (key/value pairs) to be applied to\neach node. These will added in addition to any default label(s)\nthat Kubernetes may apply to the node. In case of conflict in\nlabel keys, the applied set may differ depending on the Kubernetes\nversion -- it's best to assume the behavior is undefined and\nconflicts should be avoided. For more information, including usage\nand the valid values, see:\n  - http://kubernetes.io/v1.1/docs/user-guide/labels.html\nAn object containing a list of \"key\": value pairs.\nFor example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."] pub fn labels (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.labels" , self . base)) } # [doc = "Get a reference to the value of field `node_ip` after provisioning.\nThe default IPv4 address for SSH access and Kubernetes node.\nExample: 192.168.0.1"] pub fn node_ip (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.node_ip" , self . base)) } }
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl
    GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl
{
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
impl ToListMappable for GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl { type O = BlockAssignable < GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildGkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl
{}
impl BuildGkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl { pub fn build (self) -> GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl { GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl { effect : core :: default :: Default :: default () , key : core :: default :: Default :: default () , value : core :: default :: Default :: default () , } } }
pub struct GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsElRef { fn new (shared : StackShared , base : String) -> GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsElRef { GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsElRef { shared : shared , base : base . to_string () , } } }
impl GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `effect` after provisioning.\nSpecifies the nodes operating system (default: LINUX). Possible values: [\"EFFECT_UNSPECIFIED\", \"PREFER_NO_SCHEDULE\", \"NO_EXECUTE\"]"] pub fn effect (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.effect" , self . base)) } # [doc = "Get a reference to the value of field `key` after provisioning.\nKey associated with the effect."] pub fn key (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.key" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\nValue associated with the effect."] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElDynamic { node_configs : Option < DynamicBlock < GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl >> , taints : Option < DynamicBlock < GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl >> , }
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl { # [serde (skip_serializing_if = "Option::is_none")] labels : Option < RecField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] operating_system : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] node_configs : Option < Vec < GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl > > , # [serde (skip_serializing_if = "Option::is_none")] taints : Option < Vec < GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl > > , dynamic : GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElDynamic , }
impl GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl {
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
        v : impl Into < BlockAssignable < GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl >>,
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
        v : impl Into < BlockAssignable < GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl >>,
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
    for GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl
{
    type O = BlockAssignable<
        GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl
{}
impl BuildGkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl
    {
        GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl {
            labels: core::default::Default::default(),
            operating_system: core::default::Default::default(),
            node_configs: core::default::Default::default(),
            taints: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElRef
    {
        GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElRef { shared : shared , base : base . to_string () , }
    }
}
impl GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElRef {
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
    #[doc = "Get a reference to the value of field `node_configs` after provisioning.\n"]    pub fn node_configs (& self) -> ListRef < GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.node_configs", self.base))
    }
    #[doc = "Get a reference to the value of field `taints` after provisioning.\n"]    pub fn taints (& self) -> ListRef < GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.taints", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElDynamic { node_pool_config : Option < DynamicBlock < GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl >> , }
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigEl { # [serde (skip_serializing_if = "Option::is_none")] node_pool_config : Option < Vec < GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl > > , dynamic : GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElDynamic , }
impl GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigEl {
    #[doc = "Set the field `node_pool_config`.\n"]
    pub fn set_node_pool_config(
        mut self,
        v : impl Into < BlockAssignable < GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl >>,
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
impl ToListMappable for GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigEl {
    type O =
        BlockAssignable<GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigEl {}
impl BuildGkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigEl {
        GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigEl {
            node_pool_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElRef {
        GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `node_pool_config` after provisioning.\n"]
    pub fn node_pool_config(
        &self,
    ) -> ListRef<
        GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_pool_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminClusterControlPlaneElDynamic {
    api_server_args:
        Option<DynamicBlock<GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsEl>>,
    control_plane_node_pool_config: Option<
        DynamicBlock<GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterControlPlaneEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_server_args: Option<Vec<GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    control_plane_node_pool_config:
        Option<Vec<GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigEl>>,
    dynamic: GkeonpremBareMetalAdminClusterControlPlaneElDynamic,
}
impl GkeonpremBareMetalAdminClusterControlPlaneEl {
    #[doc = "Set the field `api_server_args`.\n"]
    pub fn set_api_server_args(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsEl>>,
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
            BlockAssignable<
                GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigEl,
            >,
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
impl ToListMappable for GkeonpremBareMetalAdminClusterControlPlaneEl {
    type O = BlockAssignable<GkeonpremBareMetalAdminClusterControlPlaneEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterControlPlaneEl {}
impl BuildGkeonpremBareMetalAdminClusterControlPlaneEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterControlPlaneEl {
        GkeonpremBareMetalAdminClusterControlPlaneEl {
            api_server_args: core::default::Default::default(),
            control_plane_node_pool_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterControlPlaneElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterControlPlaneElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalAdminClusterControlPlaneElRef {
        GkeonpremBareMetalAdminClusterControlPlaneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterControlPlaneElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_server_args` after provisioning.\n"]
    pub fn api_server_args(
        &self,
    ) -> ListRef<GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.api_server_args", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `control_plane_node_pool_config` after provisioning.\n"]
    pub fn control_plane_node_pool_config(
        &self,
    ) -> ListRef<GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane_node_pool_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElAddressPoolsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avoid_buggy_ips: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manual_assign: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pool: Option<PrimField<String>>,
}
impl GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElAddressPoolsEl {
    #[doc = "Set the field `addresses`.\nThe addresses that are part of this pool."]
    pub fn set_addresses(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.addresses = Some(v.into());
        self
    }
    #[doc = "Set the field `avoid_buggy_ips`.\nThis avoids buggy consumer devices mistakenly\ndropping IPv4 traffic for those special IP addresses."]
    pub fn set_avoid_buggy_ips(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.avoid_buggy_ips = Some(v.into());
        self
    }
    #[doc = "Set the field `manual_assign`.\nIf true, prevent IP addresses from being automatically assigned."]
    pub fn set_manual_assign(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.manual_assign = Some(v.into());
        self
    }
    #[doc = "Set the field `pool`.\n"]
    pub fn set_pool(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pool = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElAddressPoolsEl {
    type O =
        BlockAssignable<GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElAddressPoolsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElAddressPoolsEl {}
impl BuildGkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElAddressPoolsEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElAddressPoolsEl {
        GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElAddressPoolsEl {
            addresses: core::default::Default::default(),
            avoid_buggy_ips: core::default::Default::default(),
            manual_assign: core::default::Default::default(),
            pool: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElAddressPoolsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElAddressPoolsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElAddressPoolsElRef {
        GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElAddressPoolsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElAddressPoolsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `addresses` after provisioning.\nThe addresses that are part of this pool."]
    pub fn addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.addresses", self.base))
    }
    #[doc = "Get a reference to the value of field `avoid_buggy_ips` after provisioning.\nThis avoids buggy consumer devices mistakenly\ndropping IPv4 traffic for those special IP addresses."]
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
    #[doc = "Get a reference to the value of field `pool` after provisioning.\n"]
    pub fn pool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pool", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    asn: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    control_plane_nodes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
}
impl GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl {
    #[doc = "Set the field `asn`.\n"]
    pub fn set_asn(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.asn = Some(v.into());
        self
    }
    #[doc = "Set the field `control_plane_nodes`.\nThe IP address of the control plane node that\nconnects to the external peer."]
    pub fn set_control_plane_nodes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.control_plane_nodes = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl {
    type O =
        BlockAssignable<GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl {}
impl BuildGkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl {
        GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl {
            asn: core::default::Default::default(),
            control_plane_nodes: core::default::Default::default(),
            ip_address: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsElRef {
        GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `asn` after provisioning.\n"]
    pub fn asn(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.asn", self.base))
    }
    #[doc = "Get a reference to the value of field `control_plane_nodes` after provisioning.\nThe IP address of the control plane node that\nconnects to the external peer."]
    pub fn control_plane_nodes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane_nodes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    registry_burst: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registry_pull_qps: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serialize_image_pulls_disabled: Option<PrimField<bool>>,
}
impl GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl { # [doc = "Set the field `registry_burst`.\n"] pub fn set_registry_burst (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . registry_burst = Some (v . into ()) ; self } # [doc = "Set the field `registry_pull_qps`.\n"] pub fn set_registry_pull_qps (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . registry_pull_qps = Some (v . into ()) ; self } # [doc = "Set the field `serialize_image_pulls_disabled`.\n"] pub fn set_serialize_image_pulls_disabled (mut self , v : impl Into < PrimField < bool > >) -> Self { self . serialize_image_pulls_disabled = Some (v . into ()) ; self } }
impl ToListMappable for GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl { type O = BlockAssignable < GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildGkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl
{}
impl BuildGkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl { pub fn build (self) -> GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl { GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl { registry_burst : core :: default :: Default :: default () , registry_pull_qps : core :: default :: Default :: default () , serialize_image_pulls_disabled : core :: default :: Default :: default () , } } }
pub struct GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigElRef { fn new (shared : StackShared , base : String) -> GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigElRef { GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigElRef { shared : shared , base : base . to_string () , } } }
impl GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `registry_burst` after provisioning.\n"] pub fn registry_burst (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.registry_burst" , self . base)) } # [doc = "Get a reference to the value of field `registry_pull_qps` after provisioning.\n"] pub fn registry_pull_qps (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.registry_pull_qps" , self . base)) } # [doc = "Get a reference to the value of field `serialize_image_pulls_disabled` after provisioning.\n"] pub fn serialize_image_pulls_disabled (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.serialize_image_pulls_disabled" , self . base)) } }
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_ip: Option<PrimField<String>>,
}
impl GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl { # [doc = "Set the field `labels`.\nThe labels assigned to nodes of this node pool.\nAn object containing a list of key/value pairs.\nExample:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."] pub fn set_labels (mut self , v : impl Into < RecField < PrimField < String > > >) -> Self { self . labels = Some (v . into ()) ; self } # [doc = "Set the field `node_ip`.\n"] pub fn set_node_ip (mut self , v : impl Into < PrimField < String > >) -> Self { self . node_ip = Some (v . into ()) ; self } }
impl ToListMappable for GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl { type O = BlockAssignable < GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildGkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl
{}
impl BuildGkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl { pub fn build (self) -> GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl { GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl { labels : core :: default :: Default :: default () , node_ip : core :: default :: Default :: default () , } } }
pub struct GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef { fn new (shared : StackShared , base : String) -> GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef { GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef { shared : shared , base : base . to_string () , } } }
impl GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels assigned to nodes of this node pool.\nAn object containing a list of key/value pairs.\nExample:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."] pub fn labels (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.labels" , self . base)) } # [doc = "Get a reference to the value of field `node_ip` after provisioning.\n"] pub fn node_ip (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.node_ip" , self . base)) } }
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl { # [doc = "Set the field `effect`.\nAvailable taint effects."] pub fn set_effect (mut self , v : impl Into < PrimField < String > >) -> Self { self . effect = Some (v . into ()) ; self } # [doc = "Set the field `key`.\n"] pub fn set_key (mut self , v : impl Into < PrimField < String > >) -> Self { self . key = Some (v . into ()) ; self } # [doc = "Set the field `value`.\n"] pub fn set_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . value = Some (v . into ()) ; self } }
impl ToListMappable for GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl { type O = BlockAssignable < GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildGkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl
{}
impl BuildGkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl { pub fn build (self) -> GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl { GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl { effect : core :: default :: Default :: default () , key : core :: default :: Default :: default () , value : core :: default :: Default :: default () , } } }
pub struct GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef { fn new (shared : StackShared , base : String) -> GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef { GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef { shared : shared , base : base . to_string () , } } }
impl GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `effect` after provisioning.\nAvailable taint effects."] pub fn effect (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.effect" , self . base)) } # [doc = "Get a reference to the value of field `key` after provisioning.\n"] pub fn key (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.key" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElDynamic { kubelet_config : Option < DynamicBlock < GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl >> , node_configs : Option < DynamicBlock < GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl >> , taints : Option < DynamicBlock < GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl >> , }
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl { # [serde (skip_serializing_if = "Option::is_none")] labels : Option < RecField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] operating_system : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] kubelet_config : Option < Vec < GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] node_configs : Option < Vec < GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl > > , # [serde (skip_serializing_if = "Option::is_none")] taints : Option < Vec < GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl > > , dynamic : GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElDynamic , }
impl GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl { # [doc = "Set the field `labels`.\nThe labels assigned to nodes of this node pool.\nAn object containing a list of key/value pairs.\nExample:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."] pub fn set_labels (mut self , v : impl Into < RecField < PrimField < String > > >) -> Self { self . labels = Some (v . into ()) ; self } # [doc = "Set the field `operating_system`.\nThe available Operating Systems to be run in a Node."] pub fn set_operating_system (mut self , v : impl Into < PrimField < String > >) -> Self { self . operating_system = Some (v . into ()) ; self } # [doc = "Set the field `kubelet_config`.\n"] pub fn set_kubelet_config (mut self , v : impl Into < BlockAssignable < GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . kubelet_config = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . kubelet_config = Some (d) ; } } self } # [doc = "Set the field `node_configs`.\n"] pub fn set_node_configs (mut self , v : impl Into < BlockAssignable < GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . node_configs = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . node_configs = Some (d) ; } } self } # [doc = "Set the field `taints`.\n"] pub fn set_taints (mut self , v : impl Into < BlockAssignable < GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . taints = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . taints = Some (d) ; } } self } }
impl ToListMappable for GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl { type O = BlockAssignable < GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildGkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl
{}
impl BuildGkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl { pub fn build (self) -> GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl { GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl { labels : core :: default :: Default :: default () , operating_system : core :: default :: Default :: default () , kubelet_config : core :: default :: Default :: default () , node_configs : core :: default :: Default :: default () , taints : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef { fn new (shared : StackShared , base : String) -> GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef { GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef { shared : shared , base : base . to_string () , } } }
impl GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels assigned to nodes of this node pool.\nAn object containing a list of key/value pairs.\nExample:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."] pub fn labels (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.labels" , self . base)) } # [doc = "Get a reference to the value of field `operating_system` after provisioning.\nThe available Operating Systems to be run in a Node."] pub fn operating_system (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.operating_system" , self . base)) } # [doc = "Get a reference to the value of field `kubelet_config` after provisioning.\n"] pub fn kubelet_config (& self) -> ListRef < GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.kubelet_config" , self . base)) } # [doc = "Get a reference to the value of field `node_configs` after provisioning.\n"] pub fn node_configs (& self) -> ListRef < GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.node_configs" , self . base)) } # [doc = "Get a reference to the value of field `taints` after provisioning.\n"] pub fn taints (& self) -> ListRef < GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.taints" , self . base)) } }
#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElDynamic { node_pool_config : Option < DynamicBlock < GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl >> , }
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl { # [serde (skip_serializing_if = "Option::is_none")] node_pool_config : Option < Vec < GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl > > , dynamic : GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElDynamic , }
impl GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl {
    #[doc = "Set the field `node_pool_config`.\n"]
    pub fn set_node_pool_config(
        mut self,
        v : impl Into < BlockAssignable < GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl >>,
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
    for GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl
{
    type O = BlockAssignable<
        GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl
{}
impl BuildGkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl {
        GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl {
            node_pool_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElRef
    {
        GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `node_pool_config` after provisioning.\n"]    pub fn node_pool_config (& self) -> ListRef < GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_pool_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElDynamic {
    address_pools: Option<
        DynamicBlock<GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElAddressPoolsEl>,
    >,
    bgp_peer_configs: Option<
        DynamicBlock<GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl>,
    >,
    load_balancer_node_pool_config: Option<
        DynamicBlock<
            GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    asn: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_pools:
        Option<Vec<GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElAddressPoolsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bgp_peer_configs:
        Option<Vec<GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancer_node_pool_config: Option<
        Vec<GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl>,
    >,
    dynamic: GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElDynamic,
}
impl GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigEl {
    #[doc = "Set the field `asn`.\nBGP autonomous system number (ASN) of the cluster."]
    pub fn set_asn(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.asn = Some(v.into());
        self
    }
    #[doc = "Set the field `address_pools`.\n"]
    pub fn set_address_pools(
        mut self,
        v: impl Into<
            BlockAssignable<
                GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElAddressPoolsEl,
            >,
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
            BlockAssignable<
                GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl,
            >,
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
        v : impl Into < BlockAssignable < GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl >>,
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
impl ToListMappable for GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigEl {}
impl BuildGkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigEl {
        GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigEl {
            asn: core::default::Default::default(),
            address_pools: core::default::Default::default(),
            bgp_peer_configs: core::default::Default::default(),
            load_balancer_node_pool_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElRef {
        GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `asn` after provisioning.\nBGP autonomous system number (ASN) of the cluster."]
    pub fn asn(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.asn", self.base))
    }
    #[doc = "Get a reference to the value of field `address_pools` after provisioning.\n"]
    pub fn address_pools(
        &self,
    ) -> ListRef<GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElAddressPoolsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.address_pools", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `bgp_peer_configs` after provisioning.\n"]
    pub fn bgp_peer_configs(
        &self,
    ) -> ListRef<GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bgp_peer_configs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancer_node_pool_config` after provisioning.\n"]
    pub fn load_balancer_node_pool_config(
        &self,
    ) -> ListRef<
        GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.load_balancer_node_pool_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigEl {
    enabled: PrimField<bool>,
}
impl GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigEl {}
impl ToListMappable for GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigEl {
    #[doc = "Whether manual load balancing is enabled."]
    pub enabled: PrimField<bool>,
}
impl BuildGkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigEl {
        GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigEl {
            enabled: self.enabled,
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigElRef {
        GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether manual load balancing is enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigEl {
    control_plane_load_balancer_port: PrimField<f64>,
}
impl GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigEl {}
impl ToListMappable for GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterLoadBalancerElPortConfigEl {
    #[doc = "The port that control plane hosted load balancers will listen on."]
    pub control_plane_load_balancer_port: PrimField<f64>,
}
impl BuildGkeonpremBareMetalAdminClusterLoadBalancerElPortConfigEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigEl {
        GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigEl {
            control_plane_load_balancer_port: self.control_plane_load_balancer_port,
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigElRef {
        GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigElRef {
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
pub struct GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigEl {
    control_plane_vip: PrimField<String>,
}
impl GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigEl {}
impl ToListMappable for GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterLoadBalancerElVipConfigEl {
    #[doc = "The VIP which you previously set aside for the Kubernetes API of this Bare Metal Admin Cluster."]
    pub control_plane_vip: PrimField<String>,
}
impl BuildGkeonpremBareMetalAdminClusterLoadBalancerElVipConfigEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigEl {
        GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigEl {
            control_plane_vip: self.control_plane_vip,
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigElRef {
        GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `control_plane_vip` after provisioning.\nThe VIP which you previously set aside for the Kubernetes API of this Bare Metal Admin Cluster."]
    pub fn control_plane_vip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.control_plane_vip", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminClusterLoadBalancerElDynamic {
    bgp_lb_config: Option<DynamicBlock<GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigEl>>,
    manual_lb_config:
        Option<DynamicBlock<GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigEl>>,
    port_config: Option<DynamicBlock<GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigEl>>,
    vip_config: Option<DynamicBlock<GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigEl>>,
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterLoadBalancerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bgp_lb_config: Option<Vec<GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manual_lb_config: Option<Vec<GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_config: Option<Vec<GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vip_config: Option<Vec<GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigEl>>,
    dynamic: GkeonpremBareMetalAdminClusterLoadBalancerElDynamic,
}
impl GkeonpremBareMetalAdminClusterLoadBalancerEl {
    #[doc = "Set the field `bgp_lb_config`.\n"]
    pub fn set_bgp_lb_config(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigEl>>,
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
        v: impl Into<BlockAssignable<GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigEl>>,
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
    #[doc = "Set the field `port_config`.\n"]
    pub fn set_port_config(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigEl>>,
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
        v: impl Into<BlockAssignable<GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigEl>>,
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
impl ToListMappable for GkeonpremBareMetalAdminClusterLoadBalancerEl {
    type O = BlockAssignable<GkeonpremBareMetalAdminClusterLoadBalancerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterLoadBalancerEl {}
impl BuildGkeonpremBareMetalAdminClusterLoadBalancerEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterLoadBalancerEl {
        GkeonpremBareMetalAdminClusterLoadBalancerEl {
            bgp_lb_config: core::default::Default::default(),
            manual_lb_config: core::default::Default::default(),
            port_config: core::default::Default::default(),
            vip_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterLoadBalancerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterLoadBalancerElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalAdminClusterLoadBalancerElRef {
        GkeonpremBareMetalAdminClusterLoadBalancerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterLoadBalancerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bgp_lb_config` after provisioning.\n"]
    pub fn bgp_lb_config(
        &self,
    ) -> ListRef<GkeonpremBareMetalAdminClusterLoadBalancerElBgpLbConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bgp_lb_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `manual_lb_config` after provisioning.\n"]
    pub fn manual_lb_config(
        &self,
    ) -> ListRef<GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.manual_lb_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `port_config` after provisioning.\n"]
    pub fn port_config(
        &self,
    ) -> ListRef<GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.port_config", self.base))
    }
    #[doc = "Get a reference to the value of field `vip_config` after provisioning.\n"]
    pub fn vip_config(
        &self,
    ) -> ListRef<GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vip_config", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterMaintenanceConfigEl {
    maintenance_address_cidr_blocks: ListField<PrimField<String>>,
}
impl GkeonpremBareMetalAdminClusterMaintenanceConfigEl {}
impl ToListMappable for GkeonpremBareMetalAdminClusterMaintenanceConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalAdminClusterMaintenanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterMaintenanceConfigEl {
    #[doc = "All IPv4 address from these ranges will be placed into maintenance mode.\nNodes in maintenance mode will be cordoned and drained. When both of these\nare true, the \"baremetal.cluster.gke.io/maintenance\" annotation will be set\non the node resource."]
    pub maintenance_address_cidr_blocks: ListField<PrimField<String>>,
}
impl BuildGkeonpremBareMetalAdminClusterMaintenanceConfigEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterMaintenanceConfigEl {
        GkeonpremBareMetalAdminClusterMaintenanceConfigEl {
            maintenance_address_cidr_blocks: self.maintenance_address_cidr_blocks,
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterMaintenanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterMaintenanceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalAdminClusterMaintenanceConfigElRef {
        GkeonpremBareMetalAdminClusterMaintenanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterMaintenanceConfigElRef {
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
pub struct GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrEl {
    pod_address_cidr_blocks: ListField<PrimField<String>>,
    service_address_cidr_blocks: ListField<PrimField<String>>,
}
impl GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrEl {}
impl ToListMappable for GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrEl {
    type O = BlockAssignable<GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrEl {
    #[doc = "All pods in the cluster are assigned an RFC1918 IPv4 address from these ranges. This field cannot be changed after creation."]
    pub pod_address_cidr_blocks: ListField<PrimField<String>>,
    #[doc = "All services in the cluster are assigned an RFC1918 IPv4 address from these ranges. This field cannot be changed after creation."]
    pub service_address_cidr_blocks: ListField<PrimField<String>>,
}
impl BuildGkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrEl {
        GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrEl {
            pod_address_cidr_blocks: self.pod_address_cidr_blocks,
            service_address_cidr_blocks: self.service_address_cidr_blocks,
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrElRef {
        GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrElRef {
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
pub struct GkeonpremBareMetalAdminClusterNetworkConfigElMultipleNetworkInterfacesConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl GkeonpremBareMetalAdminClusterNetworkConfigElMultipleNetworkInterfacesConfigEl {
    #[doc = "Set the field `enabled`.\nWhen set network_config.advanced_networking is automatically\nset to true."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable
    for GkeonpremBareMetalAdminClusterNetworkConfigElMultipleNetworkInterfacesConfigEl
{
    type O = BlockAssignable<
        GkeonpremBareMetalAdminClusterNetworkConfigElMultipleNetworkInterfacesConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterNetworkConfigElMultipleNetworkInterfacesConfigEl {}
impl BuildGkeonpremBareMetalAdminClusterNetworkConfigElMultipleNetworkInterfacesConfigEl {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalAdminClusterNetworkConfigElMultipleNetworkInterfacesConfigEl {
        GkeonpremBareMetalAdminClusterNetworkConfigElMultipleNetworkInterfacesConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterNetworkConfigElMultipleNetworkInterfacesConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterNetworkConfigElMultipleNetworkInterfacesConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalAdminClusterNetworkConfigElMultipleNetworkInterfacesConfigElRef {
        GkeonpremBareMetalAdminClusterNetworkConfigElMultipleNetworkInterfacesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterNetworkConfigElMultipleNetworkInterfacesConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhen set network_config.advanced_networking is automatically\nset to true."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminClusterNetworkConfigElDynamic {
    island_mode_cidr:
        Option<DynamicBlock<GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrEl>>,
    multiple_network_interfaces_config: Option<
        DynamicBlock<
            GkeonpremBareMetalAdminClusterNetworkConfigElMultipleNetworkInterfacesConfigEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterNetworkConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_networking: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    island_mode_cidr: Option<Vec<GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multiple_network_interfaces_config:
        Option<Vec<GkeonpremBareMetalAdminClusterNetworkConfigElMultipleNetworkInterfacesConfigEl>>,
    dynamic: GkeonpremBareMetalAdminClusterNetworkConfigElDynamic,
}
impl GkeonpremBareMetalAdminClusterNetworkConfigEl {
    #[doc = "Set the field `advanced_networking`.\nEnables the use of advanced Anthos networking features."]
    pub fn set_advanced_networking(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.advanced_networking = Some(v.into());
        self
    }
    #[doc = "Set the field `island_mode_cidr`.\n"]
    pub fn set_island_mode_cidr(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrEl>>,
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
                GkeonpremBareMetalAdminClusterNetworkConfigElMultipleNetworkInterfacesConfigEl,
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
}
impl ToListMappable for GkeonpremBareMetalAdminClusterNetworkConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalAdminClusterNetworkConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterNetworkConfigEl {}
impl BuildGkeonpremBareMetalAdminClusterNetworkConfigEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterNetworkConfigEl {
        GkeonpremBareMetalAdminClusterNetworkConfigEl {
            advanced_networking: core::default::Default::default(),
            island_mode_cidr: core::default::Default::default(),
            multiple_network_interfaces_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterNetworkConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterNetworkConfigElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalAdminClusterNetworkConfigElRef {
        GkeonpremBareMetalAdminClusterNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterNetworkConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `advanced_networking` after provisioning.\nEnables the use of advanced Anthos networking features."]
    pub fn advanced_networking(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.advanced_networking", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `island_mode_cidr` after provisioning.\n"]
    pub fn island_mode_cidr(
        &self,
    ) -> ListRef<GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.island_mode_cidr", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `multiple_network_interfaces_config` after provisioning.\n"]
    pub fn multiple_network_interfaces_config(
        &self,
    ) -> ListRef<GkeonpremBareMetalAdminClusterNetworkConfigElMultipleNetworkInterfacesConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.multiple_network_interfaces_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterNodeAccessConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    login_user: Option<PrimField<String>>,
}
impl GkeonpremBareMetalAdminClusterNodeAccessConfigEl {
    #[doc = "Set the field `login_user`.\nLoginUser is the user name used to access node machines.\nIt defaults to \"root\" if not set."]
    pub fn set_login_user(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.login_user = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremBareMetalAdminClusterNodeAccessConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalAdminClusterNodeAccessConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterNodeAccessConfigEl {}
impl BuildGkeonpremBareMetalAdminClusterNodeAccessConfigEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterNodeAccessConfigEl {
        GkeonpremBareMetalAdminClusterNodeAccessConfigEl {
            login_user: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterNodeAccessConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterNodeAccessConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalAdminClusterNodeAccessConfigElRef {
        GkeonpremBareMetalAdminClusterNodeAccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterNodeAccessConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `login_user` after provisioning.\nLoginUser is the user name used to access node machines.\nIt defaults to \"root\" if not set."]
    pub fn login_user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.login_user", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterNodeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_pods_per_node: Option<PrimField<f64>>,
}
impl GkeonpremBareMetalAdminClusterNodeConfigEl {
    #[doc = "Set the field `max_pods_per_node`.\nThe maximum number of pods a node can run. The size of the CIDR range\nassigned to the node will be derived from this parameter."]
    pub fn set_max_pods_per_node(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_pods_per_node = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremBareMetalAdminClusterNodeConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalAdminClusterNodeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterNodeConfigEl {}
impl BuildGkeonpremBareMetalAdminClusterNodeConfigEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterNodeConfigEl {
        GkeonpremBareMetalAdminClusterNodeConfigEl {
            max_pods_per_node: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterNodeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterNodeConfigElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalAdminClusterNodeConfigElRef {
        GkeonpremBareMetalAdminClusterNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterNodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
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
pub struct GkeonpremBareMetalAdminClusterProxyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    no_proxy: Option<ListField<PrimField<String>>>,
    uri: PrimField<String>,
}
impl GkeonpremBareMetalAdminClusterProxyEl {
    #[doc = "Set the field `no_proxy`.\nA list of IPs, hostnames, and domains that should skip the proxy.\nFor example: [\"127.0.0.1\", \"example.com\", \".corp\", \"localhost\"]."]
    pub fn set_no_proxy(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.no_proxy = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremBareMetalAdminClusterProxyEl {
    type O = BlockAssignable<GkeonpremBareMetalAdminClusterProxyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterProxyEl {
    #[doc = "Specifies the address of your proxy server.\nFor Example: http://domain\nWARNING: Do not provide credentials in the format\nof http://(username:password@)domain these will be rejected by the server."]
    pub uri: PrimField<String>,
}
impl BuildGkeonpremBareMetalAdminClusterProxyEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterProxyEl {
        GkeonpremBareMetalAdminClusterProxyEl {
            no_proxy: core::default::Default::default(),
            uri: self.uri,
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterProxyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterProxyElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalAdminClusterProxyElRef {
        GkeonpremBareMetalAdminClusterProxyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterProxyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `no_proxy` after provisioning.\nA list of IPs, hostnames, and domains that should skip the proxy.\nFor example: [\"127.0.0.1\", \"example.com\", \".corp\", \"localhost\"]."]
    pub fn no_proxy(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.no_proxy", self.base))
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nSpecifies the address of your proxy server.\nFor Example: http://domain\nWARNING: Do not provide credentials in the format\nof http://(username:password@)domain these will be rejected by the server."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersEl {
    username: PrimField<String>,
}
impl GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersEl {}
impl ToListMappable for GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersEl {
    type O =
        BlockAssignable<GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersEl {
    #[doc = "The name of the user, e.g. 'my-gcp-id@gmail.com'."]
    pub username: PrimField<String>,
}
impl BuildGkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersEl {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersEl {
        GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersEl {
            username: self.username,
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersElRef {
        GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nThe name of the user, e.g. 'my-gcp-id@gmail.com'."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElDynamic {
    admin_users: Option<
        DynamicBlock<GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersEl>,
    >,
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_users:
        Option<Vec<GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersEl>>,
    dynamic: GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElDynamic,
}
impl GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationEl {
    #[doc = "Set the field `admin_users`.\n"]
    pub fn set_admin_users(
        mut self,
        v: impl Into<
            BlockAssignable<
                GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersEl,
            >,
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
impl ToListMappable for GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationEl {
    type O = BlockAssignable<GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationEl {}
impl BuildGkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationEl {
        GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationEl {
            admin_users: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElRef {
        GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `admin_users` after provisioning.\n"]
    pub fn admin_users(
        &self,
    ) -> ListRef<GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.admin_users", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminClusterSecurityConfigElDynamic {
    authorization:
        Option<DynamicBlock<GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationEl>>,
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterSecurityConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization: Option<Vec<GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationEl>>,
    dynamic: GkeonpremBareMetalAdminClusterSecurityConfigElDynamic,
}
impl GkeonpremBareMetalAdminClusterSecurityConfigEl {
    #[doc = "Set the field `authorization`.\n"]
    pub fn set_authorization(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationEl>>,
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
impl ToListMappable for GkeonpremBareMetalAdminClusterSecurityConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalAdminClusterSecurityConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterSecurityConfigEl {}
impl BuildGkeonpremBareMetalAdminClusterSecurityConfigEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterSecurityConfigEl {
        GkeonpremBareMetalAdminClusterSecurityConfigEl {
            authorization: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterSecurityConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterSecurityConfigElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalAdminClusterSecurityConfigElRef {
        GkeonpremBareMetalAdminClusterSecurityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterSecurityConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(
        &self,
    ) -> ListRef<GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorization", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigEl {
    path: PrimField<String>,
    storage_class: PrimField<String>,
}
impl GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigEl {}
impl ToListMappable for GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigEl {
    #[doc = "The host machine path."]
    pub path: PrimField<String>,
    #[doc = "The StorageClass name that PVs will be created with."]
    pub storage_class: PrimField<String>,
}
impl BuildGkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigEl {
        GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigEl {
            path: self.path,
            storage_class: self.storage_class,
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigElRef {
        GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigElRef {
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
pub struct GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigEl {
    path: PrimField<String>,
    storage_class: PrimField<String>,
}
impl GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigEl {}
impl ToListMappable for GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigEl {
    #[doc = "The host machine path."]
    pub path: PrimField<String>,
    #[doc = "The StorageClass name that PVs will be created with."]
    pub storage_class: PrimField<String>,
}
impl BuildGkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigEl {
        GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigEl {
            path: self.path,
            storage_class: self.storage_class,
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigElRef {
        GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigElRef {
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
struct GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElDynamic {
    lvp_config:
        Option<DynamicBlock<GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigEl>>,
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterStorageElLvpShareConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    shared_path_pv_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lvp_config: Option<Vec<GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigEl>>,
    dynamic: GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElDynamic,
}
impl GkeonpremBareMetalAdminClusterStorageElLvpShareConfigEl {
    #[doc = "Set the field `shared_path_pv_count`.\nThe number of subdirectories to create under path."]
    pub fn set_shared_path_pv_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.shared_path_pv_count = Some(v.into());
        self
    }
    #[doc = "Set the field `lvp_config`.\n"]
    pub fn set_lvp_config(
        mut self,
        v: impl Into<
            BlockAssignable<GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigEl>,
        >,
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
impl ToListMappable for GkeonpremBareMetalAdminClusterStorageElLvpShareConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalAdminClusterStorageElLvpShareConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterStorageElLvpShareConfigEl {}
impl BuildGkeonpremBareMetalAdminClusterStorageElLvpShareConfigEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterStorageElLvpShareConfigEl {
        GkeonpremBareMetalAdminClusterStorageElLvpShareConfigEl {
            shared_path_pv_count: core::default::Default::default(),
            lvp_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElRef {
        GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElRef {
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
    ) -> ListRef<GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lvp_config", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminClusterStorageElDynamic {
    lvp_node_mounts_config:
        Option<DynamicBlock<GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigEl>>,
    lvp_share_config: Option<DynamicBlock<GkeonpremBareMetalAdminClusterStorageElLvpShareConfigEl>>,
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterStorageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lvp_node_mounts_config:
        Option<Vec<GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lvp_share_config: Option<Vec<GkeonpremBareMetalAdminClusterStorageElLvpShareConfigEl>>,
    dynamic: GkeonpremBareMetalAdminClusterStorageElDynamic,
}
impl GkeonpremBareMetalAdminClusterStorageEl {
    #[doc = "Set the field `lvp_node_mounts_config`.\n"]
    pub fn set_lvp_node_mounts_config(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigEl>>,
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
        v: impl Into<BlockAssignable<GkeonpremBareMetalAdminClusterStorageElLvpShareConfigEl>>,
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
impl ToListMappable for GkeonpremBareMetalAdminClusterStorageEl {
    type O = BlockAssignable<GkeonpremBareMetalAdminClusterStorageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterStorageEl {}
impl BuildGkeonpremBareMetalAdminClusterStorageEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterStorageEl {
        GkeonpremBareMetalAdminClusterStorageEl {
            lvp_node_mounts_config: core::default::Default::default(),
            lvp_share_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterStorageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterStorageElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalAdminClusterStorageElRef {
        GkeonpremBareMetalAdminClusterStorageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterStorageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `lvp_node_mounts_config` after provisioning.\n"]
    pub fn lvp_node_mounts_config(
        &self,
    ) -> ListRef<GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lvp_node_mounts_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lvp_share_config` after provisioning.\n"]
    pub fn lvp_share_config(
        &self,
    ) -> ListRef<GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lvp_share_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl GkeonpremBareMetalAdminClusterTimeoutsEl {
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
impl ToListMappable for GkeonpremBareMetalAdminClusterTimeoutsEl {
    type O = BlockAssignable<GkeonpremBareMetalAdminClusterTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalAdminClusterTimeoutsEl {}
impl BuildGkeonpremBareMetalAdminClusterTimeoutsEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterTimeoutsEl {
        GkeonpremBareMetalAdminClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalAdminClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalAdminClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalAdminClusterTimeoutsElRef {
        GkeonpremBareMetalAdminClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalAdminClusterTimeoutsElRef {
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
#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminClusterDynamic {
    cluster_operations: Option<DynamicBlock<GkeonpremBareMetalAdminClusterClusterOperationsEl>>,
    control_plane: Option<DynamicBlock<GkeonpremBareMetalAdminClusterControlPlaneEl>>,
    load_balancer: Option<DynamicBlock<GkeonpremBareMetalAdminClusterLoadBalancerEl>>,
    maintenance_config: Option<DynamicBlock<GkeonpremBareMetalAdminClusterMaintenanceConfigEl>>,
    network_config: Option<DynamicBlock<GkeonpremBareMetalAdminClusterNetworkConfigEl>>,
    node_access_config: Option<DynamicBlock<GkeonpremBareMetalAdminClusterNodeAccessConfigEl>>,
    node_config: Option<DynamicBlock<GkeonpremBareMetalAdminClusterNodeConfigEl>>,
    proxy: Option<DynamicBlock<GkeonpremBareMetalAdminClusterProxyEl>>,
    security_config: Option<DynamicBlock<GkeonpremBareMetalAdminClusterSecurityConfigEl>>,
    storage: Option<DynamicBlock<GkeonpremBareMetalAdminClusterStorageEl>>,
}
