use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct EdgecontainerClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_max_pods_per_node: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_load_balancer_ipv4_address_pools: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    release_channel: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization: Option<Vec<EdgecontainerClusterAuthorizationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    control_plane: Option<Vec<EdgecontainerClusterControlPlaneEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    control_plane_encryption: Option<Vec<EdgecontainerClusterControlPlaneEncryptionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fleet: Option<Vec<EdgecontainerClusterFleetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_policy: Option<Vec<EdgecontainerClusterMaintenancePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    networking: Option<Vec<EdgecontainerClusterNetworkingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_addons_config: Option<Vec<EdgecontainerClusterSystemAddonsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<EdgecontainerClusterTimeoutsEl>,
    dynamic: EdgecontainerClusterDynamic,
}
struct EdgecontainerCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EdgecontainerClusterData>,
}
#[derive(Clone)]
pub struct EdgecontainerCluster(Rc<EdgecontainerCluster_>);
impl EdgecontainerCluster {
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
    #[doc = "Set the field `default_max_pods_per_node`.\nThe default maximum number of pods per node used if a maximum value is not\nspecified explicitly for a node pool in this cluster. If unspecified, the\nKubernetes default value will be used."]
    pub fn set_default_max_pods_per_node(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().default_max_pods_per_node = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `external_load_balancer_ipv4_address_pools`.\nAddress pools for cluster data plane external load balancing."]
    pub fn set_external_load_balancer_ipv4_address_pools(
        self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .external_load_balancer_ipv4_address_pools = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nUser-defined labels for the edgecloud cluster.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `release_channel`.\nThe release channel a cluster is subscribed to. Possible values: [\"RELEASE_CHANNEL_UNSPECIFIED\", \"NONE\", \"REGULAR\"]"]
    pub fn set_release_channel(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().release_channel = Some(v.into());
        self
    }
    #[doc = "Set the field `target_version`.\nThe target cluster version. For example: \"1.5.0\"."]
    pub fn set_target_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().target_version = Some(v.into());
        self
    }
    #[doc = "Set the field `authorization`.\n"]
    pub fn set_authorization(
        self,
        v: impl Into<BlockAssignable<EdgecontainerClusterAuthorizationEl>>,
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
    #[doc = "Set the field `control_plane`.\n"]
    pub fn set_control_plane(
        self,
        v: impl Into<BlockAssignable<EdgecontainerClusterControlPlaneEl>>,
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
    #[doc = "Set the field `control_plane_encryption`.\n"]
    pub fn set_control_plane_encryption(
        self,
        v: impl Into<BlockAssignable<EdgecontainerClusterControlPlaneEncryptionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().control_plane_encryption = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.control_plane_encryption = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `fleet`.\n"]
    pub fn set_fleet(self, v: impl Into<BlockAssignable<EdgecontainerClusterFleetEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().fleet = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.fleet = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `maintenance_policy`.\n"]
    pub fn set_maintenance_policy(
        self,
        v: impl Into<BlockAssignable<EdgecontainerClusterMaintenancePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `networking`.\n"]
    pub fn set_networking(
        self,
        v: impl Into<BlockAssignable<EdgecontainerClusterNetworkingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().networking = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.networking = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `system_addons_config`.\n"]
    pub fn set_system_addons_config(
        self,
        v: impl Into<BlockAssignable<EdgecontainerClusterSystemAddonsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().system_addons_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.system_addons_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<EdgecontainerClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `cluster_ca_certificate` after provisioning.\nThe PEM-encoded public certificate of the cluster's CA."]
    pub fn cluster_ca_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_ca_certificate", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `control_plane_version` after provisioning.\nThe control plane release version."]
    pub fn control_plane_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.control_plane_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time the cluster was created, in RFC3339 text format."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_max_pods_per_node` after provisioning.\nThe default maximum number of pods per node used if a maximum value is not\nspecified explicitly for a node pool in this cluster. If unspecified, the\nKubernetes default value will be used."]
    pub fn default_max_pods_per_node(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_max_pods_per_node", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nThe IP address of the Kubernetes API server."]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_load_balancer_ipv4_address_pools` after provisioning.\nAddress pools for cluster data plane external load balancing."]
    pub fn external_load_balancer_ipv4_address_pools(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.external_load_balancer_ipv4_address_pools",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the edgecloud cluster.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_events` after provisioning.\nAll the maintenance events scheduled for the cluster, including the ones\nongoing, planned for the future and done in the past (up to 90 days)."]
    pub fn maintenance_events(&self) -> ListRef<EdgecontainerClusterMaintenanceEventsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_events", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe GDCE cluster name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_version` after provisioning.\nThe lowest release version among all worker nodes. This field can be empty\nif the cluster does not have any worker nodes."]
    pub fn node_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe port number of the Kubernetes API server."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `release_channel` after provisioning.\nThe release channel a cluster is subscribed to. Possible values: [\"RELEASE_CHANNEL_UNSPECIFIED\", \"NONE\", \"REGULAR\"]"]
    pub fn release_channel(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.release_channel", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nIndicates the status of the cluster."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_version` after provisioning.\nThe target cluster version. For example: \"1.5.0\"."]
    pub fn target_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time the cluster was last updated, in RFC3339 text format."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> ListRef<EdgecontainerClusterAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `control_plane` after provisioning.\n"]
    pub fn control_plane(&self) -> ListRef<EdgecontainerClusterControlPlaneElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `control_plane_encryption` after provisioning.\n"]
    pub fn control_plane_encryption(
        &self,
    ) -> ListRef<EdgecontainerClusterControlPlaneEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane_encryption", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet` after provisioning.\n"]
    pub fn fleet(&self) -> ListRef<EdgecontainerClusterFleetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fleet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> ListRef<EdgecontainerClusterMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `networking` after provisioning.\n"]
    pub fn networking(&self) -> ListRef<EdgecontainerClusterNetworkingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.networking", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `system_addons_config` after provisioning.\n"]
    pub fn system_addons_config(&self) -> ListRef<EdgecontainerClusterSystemAddonsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.system_addons_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EdgecontainerClusterTimeoutsElRef {
        EdgecontainerClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for EdgecontainerCluster {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for EdgecontainerCluster {}
impl ToListMappable for EdgecontainerCluster {
    type O = ListRef<EdgecontainerClusterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for EdgecontainerCluster_ {
    fn extract_resource_type(&self) -> String {
        "google_edgecontainer_cluster".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildEdgecontainerCluster {
    pub tf_id: String,
    #[doc = "The location of the resource."]
    pub location: PrimField<String>,
    #[doc = "The GDCE cluster name."]
    pub name: PrimField<String>,
}
impl BuildEdgecontainerCluster {
    pub fn build(self, stack: &mut Stack) -> EdgecontainerCluster {
        let out = EdgecontainerCluster(Rc::new(EdgecontainerCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EdgecontainerClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                default_max_pods_per_node: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                external_load_balancer_ipv4_address_pools: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                release_channel: core::default::Default::default(),
                target_version: core::default::Default::default(),
                authorization: core::default::Default::default(),
                control_plane: core::default::Default::default(),
                control_plane_encryption: core::default::Default::default(),
                fleet: core::default::Default::default(),
                maintenance_policy: core::default::Default::default(),
                networking: core::default::Default::default(),
                system_addons_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct EdgecontainerClusterRef {
    shared: StackShared,
    base: String,
}
impl Ref for EdgecontainerClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl EdgecontainerClusterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster_ca_certificate` after provisioning.\nThe PEM-encoded public certificate of the cluster's CA."]
    pub fn cluster_ca_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_ca_certificate", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `control_plane_version` after provisioning.\nThe control plane release version."]
    pub fn control_plane_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.control_plane_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time the cluster was created, in RFC3339 text format."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_max_pods_per_node` after provisioning.\nThe default maximum number of pods per node used if a maximum value is not\nspecified explicitly for a node pool in this cluster. If unspecified, the\nKubernetes default value will be used."]
    pub fn default_max_pods_per_node(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_max_pods_per_node", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nThe IP address of the Kubernetes API server."]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_load_balancer_ipv4_address_pools` after provisioning.\nAddress pools for cluster data plane external load balancing."]
    pub fn external_load_balancer_ipv4_address_pools(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.external_load_balancer_ipv4_address_pools",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the edgecloud cluster.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_events` after provisioning.\nAll the maintenance events scheduled for the cluster, including the ones\nongoing, planned for the future and done in the past (up to 90 days)."]
    pub fn maintenance_events(&self) -> ListRef<EdgecontainerClusterMaintenanceEventsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_events", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe GDCE cluster name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_version` after provisioning.\nThe lowest release version among all worker nodes. This field can be empty\nif the cluster does not have any worker nodes."]
    pub fn node_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe port number of the Kubernetes API server."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `release_channel` after provisioning.\nThe release channel a cluster is subscribed to. Possible values: [\"RELEASE_CHANNEL_UNSPECIFIED\", \"NONE\", \"REGULAR\"]"]
    pub fn release_channel(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.release_channel", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nIndicates the status of the cluster."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_version` after provisioning.\nThe target cluster version. For example: \"1.5.0\"."]
    pub fn target_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time the cluster was last updated, in RFC3339 text format."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> ListRef<EdgecontainerClusterAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `control_plane` after provisioning.\n"]
    pub fn control_plane(&self) -> ListRef<EdgecontainerClusterControlPlaneElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `control_plane_encryption` after provisioning.\n"]
    pub fn control_plane_encryption(
        &self,
    ) -> ListRef<EdgecontainerClusterControlPlaneEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane_encryption", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet` after provisioning.\n"]
    pub fn fleet(&self) -> ListRef<EdgecontainerClusterFleetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fleet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> ListRef<EdgecontainerClusterMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `networking` after provisioning.\n"]
    pub fn networking(&self) -> ListRef<EdgecontainerClusterNetworkingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.networking", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `system_addons_config` after provisioning.\n"]
    pub fn system_addons_config(&self) -> ListRef<EdgecontainerClusterSystemAddonsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.system_addons_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EdgecontainerClusterTimeoutsElRef {
        EdgecontainerClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct EdgecontainerClusterMaintenanceEventsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_version: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uuid: Option<PrimField<String>>,
}
impl EdgecontainerClusterMaintenanceEventsEl {
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `operation`.\n"]
    pub fn set_operation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operation = Some(v.into());
        self
    }
    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schedule = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `target_version`.\n"]
    pub fn set_target_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_version = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
    #[doc = "Set the field `uuid`.\n"]
    pub fn set_uuid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uuid = Some(v.into());
        self
    }
}
impl ToListMappable for EdgecontainerClusterMaintenanceEventsEl {
    type O = BlockAssignable<EdgecontainerClusterMaintenanceEventsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEdgecontainerClusterMaintenanceEventsEl {}
impl BuildEdgecontainerClusterMaintenanceEventsEl {
    pub fn build(self) -> EdgecontainerClusterMaintenanceEventsEl {
        EdgecontainerClusterMaintenanceEventsEl {
            create_time: core::default::Default::default(),
            end_time: core::default::Default::default(),
            operation: core::default::Default::default(),
            schedule: core::default::Default::default(),
            start_time: core::default::Default::default(),
            state: core::default::Default::default(),
            target_version: core::default::Default::default(),
            type_: core::default::Default::default(),
            update_time: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}
pub struct EdgecontainerClusterMaintenanceEventsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EdgecontainerClusterMaintenanceEventsElRef {
    fn new(shared: StackShared, base: String) -> EdgecontainerClusterMaintenanceEventsElRef {
        EdgecontainerClusterMaintenanceEventsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EdgecontainerClusterMaintenanceEventsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `target_version` after provisioning.\n"]
    pub fn target_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }
}
#[derive(Serialize)]
pub struct EdgecontainerClusterAuthorizationElAdminUsersEl {
    username: PrimField<String>,
}
impl EdgecontainerClusterAuthorizationElAdminUsersEl {}
impl ToListMappable for EdgecontainerClusterAuthorizationElAdminUsersEl {
    type O = BlockAssignable<EdgecontainerClusterAuthorizationElAdminUsersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEdgecontainerClusterAuthorizationElAdminUsersEl {
    #[doc = "An active Google username."]
    pub username: PrimField<String>,
}
impl BuildEdgecontainerClusterAuthorizationElAdminUsersEl {
    pub fn build(self) -> EdgecontainerClusterAuthorizationElAdminUsersEl {
        EdgecontainerClusterAuthorizationElAdminUsersEl {
            username: self.username,
        }
    }
}
pub struct EdgecontainerClusterAuthorizationElAdminUsersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EdgecontainerClusterAuthorizationElAdminUsersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EdgecontainerClusterAuthorizationElAdminUsersElRef {
        EdgecontainerClusterAuthorizationElAdminUsersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EdgecontainerClusterAuthorizationElAdminUsersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nAn active Google username."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize, Default)]
struct EdgecontainerClusterAuthorizationElDynamic {
    admin_users: Option<DynamicBlock<EdgecontainerClusterAuthorizationElAdminUsersEl>>,
}
#[derive(Serialize)]
pub struct EdgecontainerClusterAuthorizationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_users: Option<Vec<EdgecontainerClusterAuthorizationElAdminUsersEl>>,
    dynamic: EdgecontainerClusterAuthorizationElDynamic,
}
impl EdgecontainerClusterAuthorizationEl {
    #[doc = "Set the field `admin_users`.\n"]
    pub fn set_admin_users(
        mut self,
        v: impl Into<BlockAssignable<EdgecontainerClusterAuthorizationElAdminUsersEl>>,
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
impl ToListMappable for EdgecontainerClusterAuthorizationEl {
    type O = BlockAssignable<EdgecontainerClusterAuthorizationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEdgecontainerClusterAuthorizationEl {}
impl BuildEdgecontainerClusterAuthorizationEl {
    pub fn build(self) -> EdgecontainerClusterAuthorizationEl {
        EdgecontainerClusterAuthorizationEl {
            admin_users: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct EdgecontainerClusterAuthorizationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EdgecontainerClusterAuthorizationElRef {
    fn new(shared: StackShared, base: String) -> EdgecontainerClusterAuthorizationElRef {
        EdgecontainerClusterAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EdgecontainerClusterAuthorizationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `admin_users` after provisioning.\n"]
    pub fn admin_users(&self) -> ListRef<EdgecontainerClusterAuthorizationElAdminUsersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.admin_users", self.base))
    }
}
#[derive(Serialize)]
pub struct EdgecontainerClusterControlPlaneElLocalEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared_deployment_policy: Option<PrimField<String>>,
}
impl EdgecontainerClusterControlPlaneElLocalEl {
    #[doc = "Set the field `machine_filter`.\nOnly machines matching this filter will be allowed to host control\nplane nodes. The filtering language accepts strings like \"name=<name>\",\nand is documented here: [AIP-160](https://google.aip.dev/160)."]
    pub fn set_machine_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_filter = Some(v.into());
        self
    }
    #[doc = "Set the field `node_count`.\nThe number of nodes to serve as replicas of the Control Plane.\nOnly 1 and 3 are supported."]
    pub fn set_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `node_location`.\nName of the Google Distributed Cloud Edge zones where this node pool\nwill be created. For example: 'us-central1-edge-customer-a'."]
    pub fn set_node_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_location = Some(v.into());
        self
    }
    #[doc = "Set the field `shared_deployment_policy`.\nPolicy configuration about how user applications are deployed. Possible values: [\"SHARED_DEPLOYMENT_POLICY_UNSPECIFIED\", \"ALLOWED\", \"DISALLOWED\"]"]
    pub fn set_shared_deployment_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.shared_deployment_policy = Some(v.into());
        self
    }
}
impl ToListMappable for EdgecontainerClusterControlPlaneElLocalEl {
    type O = BlockAssignable<EdgecontainerClusterControlPlaneElLocalEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEdgecontainerClusterControlPlaneElLocalEl {}
impl BuildEdgecontainerClusterControlPlaneElLocalEl {
    pub fn build(self) -> EdgecontainerClusterControlPlaneElLocalEl {
        EdgecontainerClusterControlPlaneElLocalEl {
            machine_filter: core::default::Default::default(),
            node_count: core::default::Default::default(),
            node_location: core::default::Default::default(),
            shared_deployment_policy: core::default::Default::default(),
        }
    }
}
pub struct EdgecontainerClusterControlPlaneElLocalElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EdgecontainerClusterControlPlaneElLocalElRef {
    fn new(shared: StackShared, base: String) -> EdgecontainerClusterControlPlaneElLocalElRef {
        EdgecontainerClusterControlPlaneElLocalElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EdgecontainerClusterControlPlaneElLocalElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `machine_filter` after provisioning.\nOnly machines matching this filter will be allowed to host control\nplane nodes. The filtering language accepts strings like \"name=<name>\",\nand is documented here: [AIP-160](https://google.aip.dev/160)."]
    pub fn machine_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.machine_filter", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes to serve as replicas of the Control Plane.\nOnly 1 and 3 are supported."]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }
    #[doc = "Get a reference to the value of field `node_location` after provisioning.\nName of the Google Distributed Cloud Edge zones where this node pool\nwill be created. For example: 'us-central1-edge-customer-a'."]
    pub fn node_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_location", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `shared_deployment_policy` after provisioning.\nPolicy configuration about how user applications are deployed. Possible values: [\"SHARED_DEPLOYMENT_POLICY_UNSPECIFIED\", \"ALLOWED\", \"DISALLOWED\"]"]
    pub fn shared_deployment_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.shared_deployment_policy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct EdgecontainerClusterControlPlaneElRemoteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    node_location: Option<PrimField<String>>,
}
impl EdgecontainerClusterControlPlaneElRemoteEl {
    #[doc = "Set the field `node_location`.\nName of the Google Distributed Cloud Edge zones where this node pool\nwill be created. For example: 'us-central1-edge-customer-a'."]
    pub fn set_node_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_location = Some(v.into());
        self
    }
}
impl ToListMappable for EdgecontainerClusterControlPlaneElRemoteEl {
    type O = BlockAssignable<EdgecontainerClusterControlPlaneElRemoteEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEdgecontainerClusterControlPlaneElRemoteEl {}
impl BuildEdgecontainerClusterControlPlaneElRemoteEl {
    pub fn build(self) -> EdgecontainerClusterControlPlaneElRemoteEl {
        EdgecontainerClusterControlPlaneElRemoteEl {
            node_location: core::default::Default::default(),
        }
    }
}
pub struct EdgecontainerClusterControlPlaneElRemoteElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EdgecontainerClusterControlPlaneElRemoteElRef {
    fn new(shared: StackShared, base: String) -> EdgecontainerClusterControlPlaneElRemoteElRef {
        EdgecontainerClusterControlPlaneElRemoteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EdgecontainerClusterControlPlaneElRemoteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `node_location` after provisioning.\nName of the Google Distributed Cloud Edge zones where this node pool\nwill be created. For example: 'us-central1-edge-customer-a'."]
    pub fn node_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_location", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct EdgecontainerClusterControlPlaneElDynamic {
    local: Option<DynamicBlock<EdgecontainerClusterControlPlaneElLocalEl>>,
    remote: Option<DynamicBlock<EdgecontainerClusterControlPlaneElRemoteEl>>,
}
#[derive(Serialize)]
pub struct EdgecontainerClusterControlPlaneEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    local: Option<Vec<EdgecontainerClusterControlPlaneElLocalEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remote: Option<Vec<EdgecontainerClusterControlPlaneElRemoteEl>>,
    dynamic: EdgecontainerClusterControlPlaneElDynamic,
}
impl EdgecontainerClusterControlPlaneEl {
    #[doc = "Set the field `local`.\n"]
    pub fn set_local(
        mut self,
        v: impl Into<BlockAssignable<EdgecontainerClusterControlPlaneElLocalEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.local = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.local = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `remote`.\n"]
    pub fn set_remote(
        mut self,
        v: impl Into<BlockAssignable<EdgecontainerClusterControlPlaneElRemoteEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.remote = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.remote = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for EdgecontainerClusterControlPlaneEl {
    type O = BlockAssignable<EdgecontainerClusterControlPlaneEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEdgecontainerClusterControlPlaneEl {}
impl BuildEdgecontainerClusterControlPlaneEl {
    pub fn build(self) -> EdgecontainerClusterControlPlaneEl {
        EdgecontainerClusterControlPlaneEl {
            local: core::default::Default::default(),
            remote: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct EdgecontainerClusterControlPlaneElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EdgecontainerClusterControlPlaneElRef {
    fn new(shared: StackShared, base: String) -> EdgecontainerClusterControlPlaneElRef {
        EdgecontainerClusterControlPlaneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EdgecontainerClusterControlPlaneElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `local` after provisioning.\n"]
    pub fn local(&self) -> ListRef<EdgecontainerClusterControlPlaneElLocalElRef> {
        ListRef::new(self.shared().clone(), format!("{}.local", self.base))
    }
    #[doc = "Get a reference to the value of field `remote` after provisioning.\n"]
    pub fn remote(&self) -> ListRef<EdgecontainerClusterControlPlaneElRemoteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.remote", self.base))
    }
}
#[derive(Serialize)]
pub struct EdgecontainerClusterControlPlaneEncryptionElKmsStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl EdgecontainerClusterControlPlaneEncryptionElKmsStatusEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable for EdgecontainerClusterControlPlaneEncryptionElKmsStatusEl {
    type O = BlockAssignable<EdgecontainerClusterControlPlaneEncryptionElKmsStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEdgecontainerClusterControlPlaneEncryptionElKmsStatusEl {}
impl BuildEdgecontainerClusterControlPlaneEncryptionElKmsStatusEl {
    pub fn build(self) -> EdgecontainerClusterControlPlaneEncryptionElKmsStatusEl {
        EdgecontainerClusterControlPlaneEncryptionElKmsStatusEl {
            code: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct EdgecontainerClusterControlPlaneEncryptionElKmsStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EdgecontainerClusterControlPlaneEncryptionElKmsStatusElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EdgecontainerClusterControlPlaneEncryptionElKmsStatusElRef {
        EdgecontainerClusterControlPlaneEncryptionElKmsStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EdgecontainerClusterControlPlaneEncryptionElKmsStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct EdgecontainerClusterControlPlaneEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key: Option<PrimField<String>>,
}
impl EdgecontainerClusterControlPlaneEncryptionEl {
    #[doc = "Set the field `kms_key`.\nThe Cloud KMS CryptoKey e.g.\nprojects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{cryptoKey}\nto use for protecting control plane disks. If not specified, a\nGoogle-managed key will be used instead."]
    pub fn set_kms_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key = Some(v.into());
        self
    }
}
impl ToListMappable for EdgecontainerClusterControlPlaneEncryptionEl {
    type O = BlockAssignable<EdgecontainerClusterControlPlaneEncryptionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEdgecontainerClusterControlPlaneEncryptionEl {}
impl BuildEdgecontainerClusterControlPlaneEncryptionEl {
    pub fn build(self) -> EdgecontainerClusterControlPlaneEncryptionEl {
        EdgecontainerClusterControlPlaneEncryptionEl {
            kms_key: core::default::Default::default(),
        }
    }
}
pub struct EdgecontainerClusterControlPlaneEncryptionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EdgecontainerClusterControlPlaneEncryptionElRef {
    fn new(shared: StackShared, base: String) -> EdgecontainerClusterControlPlaneEncryptionElRef {
        EdgecontainerClusterControlPlaneEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EdgecontainerClusterControlPlaneEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nThe Cloud KMS CryptoKey e.g.\nprojects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{cryptoKey}\nto use for protecting control plane disks. If not specified, a\nGoogle-managed key will be used instead."]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
    #[doc = "Get a reference to the value of field `kms_key_active_version` after provisioning.\nThe Cloud KMS CryptoKeyVersion currently in use for protecting control\nplane disks. Only applicable if kms_key is set."]
    pub fn kms_key_active_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_active_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_state` after provisioning.\nAvailability of the Cloud KMS CryptoKey. If not 'KEY_AVAILABLE', then\nnodes may go offline as they cannot access their local data. This can be\ncaused by a lack of permissions to use the key, or if the key is disabled\nor deleted."]
    pub fn kms_key_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kms_status` after provisioning.\nError status returned by Cloud KMS when using this key. This field may be\npopulated only if 'kms_key_state' is not 'KMS_KEY_STATE_KEY_AVAILABLE'.\nIf populated, this field contains the error status reported by Cloud KMS."]
    pub fn kms_status(
        &self,
    ) -> ListRef<EdgecontainerClusterControlPlaneEncryptionElKmsStatusElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kms_status", self.base))
    }
}
#[derive(Serialize)]
pub struct EdgecontainerClusterFleetEl {
    project: PrimField<String>,
}
impl EdgecontainerClusterFleetEl {}
impl ToListMappable for EdgecontainerClusterFleetEl {
    type O = BlockAssignable<EdgecontainerClusterFleetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEdgecontainerClusterFleetEl {
    #[doc = "The name of the Fleet host project where this cluster will be registered.\nProject names are formatted as\n'projects/<project-number>'."]
    pub project: PrimField<String>,
}
impl BuildEdgecontainerClusterFleetEl {
    pub fn build(self) -> EdgecontainerClusterFleetEl {
        EdgecontainerClusterFleetEl {
            project: self.project,
        }
    }
}
pub struct EdgecontainerClusterFleetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EdgecontainerClusterFleetElRef {
    fn new(shared: StackShared, base: String) -> EdgecontainerClusterFleetElRef {
        EdgecontainerClusterFleetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EdgecontainerClusterFleetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `membership` after provisioning.\nThe name of the managed Hub Membership resource associated to this cluster.\nMembership names are formatted as\n'projects/<project-number>/locations/global/membership/<cluster-id>'."]
    pub fn membership(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.membership", self.base))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe name of the Fleet host project where this cluster will be registered.\nProject names are formatted as\n'projects/<project-number>'."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
}
#[derive(Serialize)]
pub struct EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsElWindowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsElWindowEl {
    #[doc = "Set the field `end_time`.\nThe time that the window ends. The end time must take place after the\nstart time."]
    pub fn set_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\nThe time that the window first starts."]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsElWindowEl {
    type O =
        BlockAssignable<EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsElWindowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsElWindowEl {}
impl BuildEdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsElWindowEl {
    pub fn build(self) -> EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsElWindowEl {
        EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsElWindowEl {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsElWindowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsElWindowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsElWindowElRef {
        EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsElWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsElWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\nThe time that the window ends. The end time must take place after the\nstart time."]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\nThe time that the window first starts."]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize, Default)]
struct EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsElDynamic {
    window: Option<
        DynamicBlock<EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsElWindowEl>,
    >,
}
#[derive(Serialize)]
pub struct EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    window: Option<Vec<EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsElWindowEl>>,
    dynamic: EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsElDynamic,
}
impl EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsEl {
    #[doc = "Set the field `id`.\nA unique (per cluster) id for the window."]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `window`.\n"]
    pub fn set_window(
        mut self,
        v: impl Into<
            BlockAssignable<EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsElWindowEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.window = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.window = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsEl {
    type O = BlockAssignable<EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsEl {}
impl BuildEdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsEl {
    pub fn build(self) -> EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsEl {
        EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsEl {
            id: core::default::Default::default(),
            window: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsElRef {
        EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nA unique (per cluster) id for the window."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `window` after provisioning.\n"]
    pub fn window(
        &self,
    ) -> ListRef<EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsElWindowElRef> {
        ListRef::new(self.shared().clone(), format!("{}.window", self.base))
    }
}
#[derive(Serialize)]
pub struct EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowEl {
    #[doc = "Set the field `end_time`.\nThe time that the window ends. The end time must take place after the\nstart time."]
    pub fn set_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\nThe time that the window first starts."]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowEl {
    type O =
        BlockAssignable<EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowEl {}
impl BuildEdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowEl {
    pub fn build(self) -> EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowEl {
        EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowEl {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowElRef {
        EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\nThe time that the window ends. The end time must take place after the\nstart time."]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\nThe time that the window first starts."]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize, Default)]
struct EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElDynamic {
    window: Option<
        DynamicBlock<EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowEl>,
    >,
}
#[derive(Serialize)]
pub struct EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    recurrence: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    window: Option<Vec<EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowEl>>,
    dynamic: EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElDynamic,
}
impl EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowEl {
    #[doc = "Set the field `recurrence`.\nAn RRULE (https://tools.ietf.org/html/rfc5545#section-3.8.5.3) for how\nthis window recurs. They go on for the span of time between the start and\nend time."]
    pub fn set_recurrence(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.recurrence = Some(v.into());
        self
    }
    #[doc = "Set the field `window`.\n"]
    pub fn set_window(
        mut self,
        v: impl Into<
            BlockAssignable<
                EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.window = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.window = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowEl {
    type O = BlockAssignable<EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowEl {}
impl BuildEdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowEl {
    pub fn build(self) -> EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowEl {
        EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowEl {
            recurrence: core::default::Default::default(),
            window: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElRef {
        EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `recurrence` after provisioning.\nAn RRULE (https://tools.ietf.org/html/rfc5545#section-3.8.5.3) for how\nthis window recurs. They go on for the span of time between the start and\nend time."]
    pub fn recurrence(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.recurrence", self.base))
    }
    #[doc = "Get a reference to the value of field `window` after provisioning.\n"]
    pub fn window(
        &self,
    ) -> ListRef<EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowElRef> {
        ListRef::new(self.shared().clone(), format!("{}.window", self.base))
    }
}
#[derive(Serialize, Default)]
struct EdgecontainerClusterMaintenancePolicyElWindowElDynamic {
    recurring_window:
        Option<DynamicBlock<EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowEl>>,
}
#[derive(Serialize)]
pub struct EdgecontainerClusterMaintenancePolicyElWindowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    recurring_window: Option<Vec<EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowEl>>,
    dynamic: EdgecontainerClusterMaintenancePolicyElWindowElDynamic,
}
impl EdgecontainerClusterMaintenancePolicyElWindowEl {
    #[doc = "Set the field `recurring_window`.\n"]
    pub fn set_recurring_window(
        mut self,
        v: impl Into<BlockAssignable<EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.recurring_window = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.recurring_window = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for EdgecontainerClusterMaintenancePolicyElWindowEl {
    type O = BlockAssignable<EdgecontainerClusterMaintenancePolicyElWindowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEdgecontainerClusterMaintenancePolicyElWindowEl {}
impl BuildEdgecontainerClusterMaintenancePolicyElWindowEl {
    pub fn build(self) -> EdgecontainerClusterMaintenancePolicyElWindowEl {
        EdgecontainerClusterMaintenancePolicyElWindowEl {
            recurring_window: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct EdgecontainerClusterMaintenancePolicyElWindowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EdgecontainerClusterMaintenancePolicyElWindowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EdgecontainerClusterMaintenancePolicyElWindowElRef {
        EdgecontainerClusterMaintenancePolicyElWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EdgecontainerClusterMaintenancePolicyElWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `recurring_window` after provisioning.\n"]
    pub fn recurring_window(
        &self,
    ) -> ListRef<EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.recurring_window", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct EdgecontainerClusterMaintenancePolicyElDynamic {
    maintenance_exclusions:
        Option<DynamicBlock<EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsEl>>,
    window: Option<DynamicBlock<EdgecontainerClusterMaintenancePolicyElWindowEl>>,
}
#[derive(Serialize)]
pub struct EdgecontainerClusterMaintenancePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_exclusions:
        Option<Vec<EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    window: Option<Vec<EdgecontainerClusterMaintenancePolicyElWindowEl>>,
    dynamic: EdgecontainerClusterMaintenancePolicyElDynamic,
}
impl EdgecontainerClusterMaintenancePolicyEl {
    #[doc = "Set the field `maintenance_exclusions`.\n"]
    pub fn set_maintenance_exclusions(
        mut self,
        v: impl Into<BlockAssignable<EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.maintenance_exclusions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.maintenance_exclusions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `window`.\n"]
    pub fn set_window(
        mut self,
        v: impl Into<BlockAssignable<EdgecontainerClusterMaintenancePolicyElWindowEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.window = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.window = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for EdgecontainerClusterMaintenancePolicyEl {
    type O = BlockAssignable<EdgecontainerClusterMaintenancePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEdgecontainerClusterMaintenancePolicyEl {}
impl BuildEdgecontainerClusterMaintenancePolicyEl {
    pub fn build(self) -> EdgecontainerClusterMaintenancePolicyEl {
        EdgecontainerClusterMaintenancePolicyEl {
            maintenance_exclusions: core::default::Default::default(),
            window: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct EdgecontainerClusterMaintenancePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EdgecontainerClusterMaintenancePolicyElRef {
    fn new(shared: StackShared, base: String) -> EdgecontainerClusterMaintenancePolicyElRef {
        EdgecontainerClusterMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EdgecontainerClusterMaintenancePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `maintenance_exclusions` after provisioning.\n"]
    pub fn maintenance_exclusions(
        &self,
    ) -> ListRef<EdgecontainerClusterMaintenancePolicyElMaintenanceExclusionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_exclusions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `window` after provisioning.\n"]
    pub fn window(&self) -> ListRef<EdgecontainerClusterMaintenancePolicyElWindowElRef> {
        ListRef::new(self.shared().clone(), format!("{}.window", self.base))
    }
}
#[derive(Serialize)]
pub struct EdgecontainerClusterNetworkingEl {
    cluster_ipv4_cidr_blocks: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_ipv6_cidr_blocks: Option<ListField<PrimField<String>>>,
    services_ipv4_cidr_blocks: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    services_ipv6_cidr_blocks: Option<ListField<PrimField<String>>>,
}
impl EdgecontainerClusterNetworkingEl {
    #[doc = "Set the field `cluster_ipv6_cidr_blocks`.\nIf specified, dual stack mode is enabled and all pods in the cluster are\nassigned an IPv6 address from these blocks alongside from an IPv4\naddress. Only a single block is supported. This field cannot be changed\nafter creation."]
    pub fn set_cluster_ipv6_cidr_blocks(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.cluster_ipv6_cidr_blocks = Some(v.into());
        self
    }
    #[doc = "Set the field `services_ipv6_cidr_blocks`.\nIf specified, dual stack mode is enabled and all services in the cluster are\nassigned an IPv6 address from these blocks alongside from an IPv4\naddress. Only a single block is supported. This field cannot be changed\nafter creation."]
    pub fn set_services_ipv6_cidr_blocks(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.services_ipv6_cidr_blocks = Some(v.into());
        self
    }
}
impl ToListMappable for EdgecontainerClusterNetworkingEl {
    type O = BlockAssignable<EdgecontainerClusterNetworkingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEdgecontainerClusterNetworkingEl {
    #[doc = "All pods in the cluster are assigned an RFC1918 IPv4 address from these\nblocks. Only a single block is supported. This field cannot be changed\nafter creation."]
    pub cluster_ipv4_cidr_blocks: ListField<PrimField<String>>,
    #[doc = "All services in the cluster are assigned an RFC1918 IPv4 address from these\nblocks. Only a single block is supported. This field cannot be changed\nafter creation."]
    pub services_ipv4_cidr_blocks: ListField<PrimField<String>>,
}
impl BuildEdgecontainerClusterNetworkingEl {
    pub fn build(self) -> EdgecontainerClusterNetworkingEl {
        EdgecontainerClusterNetworkingEl {
            cluster_ipv4_cidr_blocks: self.cluster_ipv4_cidr_blocks,
            cluster_ipv6_cidr_blocks: core::default::Default::default(),
            services_ipv4_cidr_blocks: self.services_ipv4_cidr_blocks,
            services_ipv6_cidr_blocks: core::default::Default::default(),
        }
    }
}
pub struct EdgecontainerClusterNetworkingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EdgecontainerClusterNetworkingElRef {
    fn new(shared: StackShared, base: String) -> EdgecontainerClusterNetworkingElRef {
        EdgecontainerClusterNetworkingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EdgecontainerClusterNetworkingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster_ipv4_cidr_blocks` after provisioning.\nAll pods in the cluster are assigned an RFC1918 IPv4 address from these\nblocks. Only a single block is supported. This field cannot be changed\nafter creation."]
    pub fn cluster_ipv4_cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_ipv4_cidr_blocks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_ipv6_cidr_blocks` after provisioning.\nIf specified, dual stack mode is enabled and all pods in the cluster are\nassigned an IPv6 address from these blocks alongside from an IPv4\naddress. Only a single block is supported. This field cannot be changed\nafter creation."]
    pub fn cluster_ipv6_cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_ipv6_cidr_blocks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network_type` after provisioning.\nIP addressing type of this cluster i.e. SINGLESTACK_V4 vs DUALSTACK_V4_V6."]
    pub fn network_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_type", self.base))
    }
    #[doc = "Get a reference to the value of field `services_ipv4_cidr_blocks` after provisioning.\nAll services in the cluster are assigned an RFC1918 IPv4 address from these\nblocks. Only a single block is supported. This field cannot be changed\nafter creation."]
    pub fn services_ipv4_cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.services_ipv4_cidr_blocks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `services_ipv6_cidr_blocks` after provisioning.\nIf specified, dual stack mode is enabled and all services in the cluster are\nassigned an IPv6 address from these blocks alongside from an IPv4\naddress. Only a single block is supported. This field cannot be changed\nafter creation."]
    pub fn services_ipv6_cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.services_ipv6_cidr_blocks", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct EdgecontainerClusterSystemAddonsConfigElIngressEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv4_vip: Option<PrimField<String>>,
}
impl EdgecontainerClusterSystemAddonsConfigElIngressEl {
    #[doc = "Set the field `disabled`.\nWhether Ingress is disabled."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `ipv4_vip`.\nIngress VIP."]
    pub fn set_ipv4_vip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv4_vip = Some(v.into());
        self
    }
}
impl ToListMappable for EdgecontainerClusterSystemAddonsConfigElIngressEl {
    type O = BlockAssignable<EdgecontainerClusterSystemAddonsConfigElIngressEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEdgecontainerClusterSystemAddonsConfigElIngressEl {}
impl BuildEdgecontainerClusterSystemAddonsConfigElIngressEl {
    pub fn build(self) -> EdgecontainerClusterSystemAddonsConfigElIngressEl {
        EdgecontainerClusterSystemAddonsConfigElIngressEl {
            disabled: core::default::Default::default(),
            ipv4_vip: core::default::Default::default(),
        }
    }
}
pub struct EdgecontainerClusterSystemAddonsConfigElIngressElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EdgecontainerClusterSystemAddonsConfigElIngressElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EdgecontainerClusterSystemAddonsConfigElIngressElRef {
        EdgecontainerClusterSystemAddonsConfigElIngressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EdgecontainerClusterSystemAddonsConfigElIngressElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether Ingress is disabled."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `ipv4_vip` after provisioning.\nIngress VIP."]
    pub fn ipv4_vip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv4_vip", self.base))
    }
}
#[derive(Serialize, Default)]
struct EdgecontainerClusterSystemAddonsConfigElDynamic {
    ingress: Option<DynamicBlock<EdgecontainerClusterSystemAddonsConfigElIngressEl>>,
}
#[derive(Serialize)]
pub struct EdgecontainerClusterSystemAddonsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress: Option<Vec<EdgecontainerClusterSystemAddonsConfigElIngressEl>>,
    dynamic: EdgecontainerClusterSystemAddonsConfigElDynamic,
}
impl EdgecontainerClusterSystemAddonsConfigEl {
    #[doc = "Set the field `ingress`.\n"]
    pub fn set_ingress(
        mut self,
        v: impl Into<BlockAssignable<EdgecontainerClusterSystemAddonsConfigElIngressEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ingress = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for EdgecontainerClusterSystemAddonsConfigEl {
    type O = BlockAssignable<EdgecontainerClusterSystemAddonsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEdgecontainerClusterSystemAddonsConfigEl {}
impl BuildEdgecontainerClusterSystemAddonsConfigEl {
    pub fn build(self) -> EdgecontainerClusterSystemAddonsConfigEl {
        EdgecontainerClusterSystemAddonsConfigEl {
            ingress: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct EdgecontainerClusterSystemAddonsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EdgecontainerClusterSystemAddonsConfigElRef {
    fn new(shared: StackShared, base: String) -> EdgecontainerClusterSystemAddonsConfigElRef {
        EdgecontainerClusterSystemAddonsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EdgecontainerClusterSystemAddonsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ingress` after provisioning.\n"]
    pub fn ingress(&self) -> ListRef<EdgecontainerClusterSystemAddonsConfigElIngressElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ingress", self.base))
    }
}
#[derive(Serialize)]
pub struct EdgecontainerClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl EdgecontainerClusterTimeoutsEl {
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
impl ToListMappable for EdgecontainerClusterTimeoutsEl {
    type O = BlockAssignable<EdgecontainerClusterTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEdgecontainerClusterTimeoutsEl {}
impl BuildEdgecontainerClusterTimeoutsEl {
    pub fn build(self) -> EdgecontainerClusterTimeoutsEl {
        EdgecontainerClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct EdgecontainerClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EdgecontainerClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> EdgecontainerClusterTimeoutsElRef {
        EdgecontainerClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EdgecontainerClusterTimeoutsElRef {
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
struct EdgecontainerClusterDynamic {
    authorization: Option<DynamicBlock<EdgecontainerClusterAuthorizationEl>>,
    control_plane: Option<DynamicBlock<EdgecontainerClusterControlPlaneEl>>,
    control_plane_encryption: Option<DynamicBlock<EdgecontainerClusterControlPlaneEncryptionEl>>,
    fleet: Option<DynamicBlock<EdgecontainerClusterFleetEl>>,
    maintenance_policy: Option<DynamicBlock<EdgecontainerClusterMaintenancePolicyEl>>,
    networking: Option<DynamicBlock<EdgecontainerClusterNetworkingEl>>,
    system_addons_config: Option<DynamicBlock<EdgecontainerClusterSystemAddonsConfigEl>>,
}
