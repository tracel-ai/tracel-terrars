use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ContainerAttachedClusterData {
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
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    distribution: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    name: PrimField<String>,
    platform_version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization: Option<Vec<ContainerAttachedClusterAuthorizationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    binary_authorization: Option<Vec<ContainerAttachedClusterBinaryAuthorizationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fleet: Option<Vec<ContainerAttachedClusterFleetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_config: Option<Vec<ContainerAttachedClusterLoggingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitoring_config: Option<Vec<ContainerAttachedClusterMonitoringConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oidc_config: Option<Vec<ContainerAttachedClusterOidcConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_config: Option<Vec<ContainerAttachedClusterProxyConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_posture_config: Option<Vec<ContainerAttachedClusterSecurityPostureConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ContainerAttachedClusterTimeoutsEl>,
    dynamic: ContainerAttachedClusterDynamic,
}
struct ContainerAttachedCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ContainerAttachedClusterData>,
}
#[derive(Clone)]
pub struct ContainerAttachedCluster(Rc<ContainerAttachedCluster_>);
impl ContainerAttachedCluster {
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
    #[doc = "Set the field `annotations`.\nOptional. Annotations on the cluster. This field has the same\nrestrictions as Kubernetes annotations. The total size of all keys and\nvalues combined is limited to 256k. Key can have 2 segments: prefix (optional)\nand name (required), separated by a slash (/). Prefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/container_attached_cluster.html.markdown for specifics"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nA human readable description of this attached cluster. Cannot be longer\nthan 255 UTF-8 encoded bytes."]
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
    #[doc = "Set the field `authorization`.\n"]
    pub fn set_authorization(
        self,
        v: impl Into<BlockAssignable<ContainerAttachedClusterAuthorizationEl>>,
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
    #[doc = "Set the field `binary_authorization`.\n"]
    pub fn set_binary_authorization(
        self,
        v: impl Into<BlockAssignable<ContainerAttachedClusterBinaryAuthorizationEl>>,
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
    #[doc = "Set the field `fleet`.\n"]
    pub fn set_fleet(self, v: impl Into<BlockAssignable<ContainerAttachedClusterFleetEl>>) -> Self {
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
    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        self,
        v: impl Into<BlockAssignable<ContainerAttachedClusterLoggingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `monitoring_config`.\n"]
    pub fn set_monitoring_config(
        self,
        v: impl Into<BlockAssignable<ContainerAttachedClusterMonitoringConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().monitoring_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.monitoring_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `oidc_config`.\n"]
    pub fn set_oidc_config(
        self,
        v: impl Into<BlockAssignable<ContainerAttachedClusterOidcConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().oidc_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.oidc_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `proxy_config`.\n"]
    pub fn set_proxy_config(
        self,
        v: impl Into<BlockAssignable<ContainerAttachedClusterProxyConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().proxy_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.proxy_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `security_posture_config`.\n"]
    pub fn set_security_posture_config(
        self,
        v: impl Into<BlockAssignable<ContainerAttachedClusterSecurityPostureConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().security_posture_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.security_posture_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ContainerAttachedClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nOptional. Annotations on the cluster. This field has the same\nrestrictions as Kubernetes annotations. The total size of all keys and\nvalues combined is limited to 256k. Key can have 2 segments: prefix (optional)\nand name (required), separated by a slash (/). Prefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_region` after provisioning.\nOutput only. The region where this cluster runs.\n\nFor EKS clusters, this is an AWS region. For AKS clusters,\nthis is an Azure region."]
    pub fn cluster_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which this cluster was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/container_attached_cluster.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human readable description of this attached cluster. Cannot be longer\nthan 255 UTF-8 encoded bytes."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `distribution` after provisioning.\nThe Kubernetes distribution of the underlying attached cluster. Supported values:\n\"eks\", \"aks\", \"generic\". The generic distribution provides the ability to register\nor migrate any CNCF conformant cluster."]
    pub fn distribution(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.distribution", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `errors` after provisioning.\nA set of errors found in the cluster."]
    pub fn errors(&self) -> ListRef<ContainerAttachedClusterErrorsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.errors", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kubernetes_version` after provisioning.\nThe Kubernetes version of the cluster."]
    pub fn kubernetes_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kubernetes_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of this resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\nThe platform version for the cluster (e.g. '1.23.0-gke.1')."]
    pub fn platform_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.platform_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the cluster."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the cluster. Possible values:\nSTATE_UNSPECIFIED, PROVISIONING, RUNNING, RECONCILING, STOPPING, ERROR,\nDEGRADED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nA globally unique identifier for the cluster."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time at which this cluster was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_identity_config` after provisioning.\nWorkload Identity settings."]
    pub fn workload_identity_config(
        &self,
    ) -> ListRef<ContainerAttachedClusterWorkloadIdentityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workload_identity_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> ListRef<ContainerAttachedClusterAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\n"]
    pub fn binary_authorization(
        &self,
    ) -> ListRef<ContainerAttachedClusterBinaryAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.binary_authorization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet` after provisioning.\n"]
    pub fn fleet(&self) -> ListRef<ContainerAttachedClusterFleetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fleet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> ListRef<ContainerAttachedClusterLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monitoring_config` after provisioning.\n"]
    pub fn monitoring_config(&self) -> ListRef<ContainerAttachedClusterMonitoringConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.monitoring_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oidc_config` after provisioning.\n"]
    pub fn oidc_config(&self) -> ListRef<ContainerAttachedClusterOidcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oidc_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `proxy_config` after provisioning.\n"]
    pub fn proxy_config(&self) -> ListRef<ContainerAttachedClusterProxyConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.proxy_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_posture_config` after provisioning.\n"]
    pub fn security_posture_config(
        &self,
    ) -> ListRef<ContainerAttachedClusterSecurityPostureConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_posture_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerAttachedClusterTimeoutsElRef {
        ContainerAttachedClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ContainerAttachedCluster {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ContainerAttachedCluster {}
impl ToListMappable for ContainerAttachedCluster {
    type O = ListRef<ContainerAttachedClusterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ContainerAttachedCluster_ {
    fn extract_resource_type(&self) -> String {
        "google_container_attached_cluster".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildContainerAttachedCluster {
    pub tf_id: String,
    #[doc = "The Kubernetes distribution of the underlying attached cluster. Supported values:\n\"eks\", \"aks\", \"generic\". The generic distribution provides the ability to register\nor migrate any CNCF conformant cluster."]
    pub distribution: PrimField<String>,
    #[doc = "The location for the resource"]
    pub location: PrimField<String>,
    #[doc = "The name of this resource."]
    pub name: PrimField<String>,
    #[doc = "The platform version for the cluster (e.g. '1.23.0-gke.1')."]
    pub platform_version: PrimField<String>,
}
impl BuildContainerAttachedCluster {
    pub fn build(self, stack: &mut Stack) -> ContainerAttachedCluster {
        let out = ContainerAttachedCluster(Rc::new(ContainerAttachedCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ContainerAttachedClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                distribution: self.distribution,
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                platform_version: self.platform_version,
                project: core::default::Default::default(),
                authorization: core::default::Default::default(),
                binary_authorization: core::default::Default::default(),
                fleet: core::default::Default::default(),
                logging_config: core::default::Default::default(),
                monitoring_config: core::default::Default::default(),
                oidc_config: core::default::Default::default(),
                proxy_config: core::default::Default::default(),
                security_posture_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ContainerAttachedClusterRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAttachedClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ContainerAttachedClusterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nOptional. Annotations on the cluster. This field has the same\nrestrictions as Kubernetes annotations. The total size of all keys and\nvalues combined is limited to 256k. Key can have 2 segments: prefix (optional)\nand name (required), separated by a slash (/). Prefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_region` after provisioning.\nOutput only. The region where this cluster runs.\n\nFor EKS clusters, this is an AWS region. For AKS clusters,\nthis is an Azure region."]
    pub fn cluster_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which this cluster was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/container_attached_cluster.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human readable description of this attached cluster. Cannot be longer\nthan 255 UTF-8 encoded bytes."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `distribution` after provisioning.\nThe Kubernetes distribution of the underlying attached cluster. Supported values:\n\"eks\", \"aks\", \"generic\". The generic distribution provides the ability to register\nor migrate any CNCF conformant cluster."]
    pub fn distribution(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.distribution", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `errors` after provisioning.\nA set of errors found in the cluster."]
    pub fn errors(&self) -> ListRef<ContainerAttachedClusterErrorsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.errors", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kubernetes_version` after provisioning.\nThe Kubernetes version of the cluster."]
    pub fn kubernetes_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kubernetes_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of this resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\nThe platform version for the cluster (e.g. '1.23.0-gke.1')."]
    pub fn platform_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.platform_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the cluster."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the cluster. Possible values:\nSTATE_UNSPECIFIED, PROVISIONING, RUNNING, RECONCILING, STOPPING, ERROR,\nDEGRADED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nA globally unique identifier for the cluster."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time at which this cluster was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_identity_config` after provisioning.\nWorkload Identity settings."]
    pub fn workload_identity_config(
        &self,
    ) -> ListRef<ContainerAttachedClusterWorkloadIdentityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workload_identity_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> ListRef<ContainerAttachedClusterAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\n"]
    pub fn binary_authorization(
        &self,
    ) -> ListRef<ContainerAttachedClusterBinaryAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.binary_authorization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet` after provisioning.\n"]
    pub fn fleet(&self) -> ListRef<ContainerAttachedClusterFleetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fleet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> ListRef<ContainerAttachedClusterLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monitoring_config` after provisioning.\n"]
    pub fn monitoring_config(&self) -> ListRef<ContainerAttachedClusterMonitoringConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.monitoring_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oidc_config` after provisioning.\n"]
    pub fn oidc_config(&self) -> ListRef<ContainerAttachedClusterOidcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oidc_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `proxy_config` after provisioning.\n"]
    pub fn proxy_config(&self) -> ListRef<ContainerAttachedClusterProxyConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.proxy_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_posture_config` after provisioning.\n"]
    pub fn security_posture_config(
        &self,
    ) -> ListRef<ContainerAttachedClusterSecurityPostureConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_posture_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerAttachedClusterTimeoutsElRef {
        ContainerAttachedClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerAttachedClusterErrorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl ContainerAttachedClusterErrorsEl {
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerAttachedClusterErrorsEl {
    type O = BlockAssignable<ContainerAttachedClusterErrorsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAttachedClusterErrorsEl {}
impl BuildContainerAttachedClusterErrorsEl {
    pub fn build(self) -> ContainerAttachedClusterErrorsEl {
        ContainerAttachedClusterErrorsEl {
            message: core::default::Default::default(),
        }
    }
}
pub struct ContainerAttachedClusterErrorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAttachedClusterErrorsElRef {
    fn new(shared: StackShared, base: String) -> ContainerAttachedClusterErrorsElRef {
        ContainerAttachedClusterErrorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAttachedClusterErrorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerAttachedClusterWorkloadIdentityConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workload_pool: Option<PrimField<String>>,
}
impl ContainerAttachedClusterWorkloadIdentityConfigEl {
    #[doc = "Set the field `identity_provider`.\n"]
    pub fn set_identity_provider(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider = Some(v.into());
        self
    }
    #[doc = "Set the field `issuer_uri`.\n"]
    pub fn set_issuer_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issuer_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `workload_pool`.\n"]
    pub fn set_workload_pool(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.workload_pool = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerAttachedClusterWorkloadIdentityConfigEl {
    type O = BlockAssignable<ContainerAttachedClusterWorkloadIdentityConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAttachedClusterWorkloadIdentityConfigEl {}
impl BuildContainerAttachedClusterWorkloadIdentityConfigEl {
    pub fn build(self) -> ContainerAttachedClusterWorkloadIdentityConfigEl {
        ContainerAttachedClusterWorkloadIdentityConfigEl {
            identity_provider: core::default::Default::default(),
            issuer_uri: core::default::Default::default(),
            workload_pool: core::default::Default::default(),
        }
    }
}
pub struct ContainerAttachedClusterWorkloadIdentityConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAttachedClusterWorkloadIdentityConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerAttachedClusterWorkloadIdentityConfigElRef {
        ContainerAttachedClusterWorkloadIdentityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAttachedClusterWorkloadIdentityConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `identity_provider` after provisioning.\n"]
    pub fn identity_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_provider", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `issuer_uri` after provisioning.\n"]
    pub fn issuer_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `workload_pool` after provisioning.\n"]
    pub fn workload_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workload_pool", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerAttachedClusterAuthorizationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_groups: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_users: Option<ListField<PrimField<String>>>,
}
impl ContainerAttachedClusterAuthorizationEl {
    #[doc = "Set the field `admin_groups`.\nGroups that can perform operations as a cluster admin. A managed\nClusterRoleBinding will be created to grant the 'cluster-admin' ClusterRole\nto the groups. Up to ten admin groups can be provided.\n\nFor more info on RBAC, see\nhttps://kubernetes.io/docs/reference/access-authn-authz/rbac/#user-facing-roles"]
    pub fn set_admin_groups(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.admin_groups = Some(v.into());
        self
    }
    #[doc = "Set the field `admin_users`.\nUsers that can perform operations as a cluster admin. A managed\nClusterRoleBinding will be created to grant the 'cluster-admin' ClusterRole\nto the users. Up to ten admin users can be provided.\n\nFor more info on RBAC, see\nhttps://kubernetes.io/docs/reference/access-authn-authz/rbac/#user-facing-roles"]
    pub fn set_admin_users(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.admin_users = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerAttachedClusterAuthorizationEl {
    type O = BlockAssignable<ContainerAttachedClusterAuthorizationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAttachedClusterAuthorizationEl {}
impl BuildContainerAttachedClusterAuthorizationEl {
    pub fn build(self) -> ContainerAttachedClusterAuthorizationEl {
        ContainerAttachedClusterAuthorizationEl {
            admin_groups: core::default::Default::default(),
            admin_users: core::default::Default::default(),
        }
    }
}
pub struct ContainerAttachedClusterAuthorizationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAttachedClusterAuthorizationElRef {
    fn new(shared: StackShared, base: String) -> ContainerAttachedClusterAuthorizationElRef {
        ContainerAttachedClusterAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAttachedClusterAuthorizationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `admin_groups` after provisioning.\nGroups that can perform operations as a cluster admin. A managed\nClusterRoleBinding will be created to grant the 'cluster-admin' ClusterRole\nto the groups. Up to ten admin groups can be provided.\n\nFor more info on RBAC, see\nhttps://kubernetes.io/docs/reference/access-authn-authz/rbac/#user-facing-roles"]
    pub fn admin_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.admin_groups", self.base))
    }
    #[doc = "Get a reference to the value of field `admin_users` after provisioning.\nUsers that can perform operations as a cluster admin. A managed\nClusterRoleBinding will be created to grant the 'cluster-admin' ClusterRole\nto the users. Up to ten admin users can be provided.\n\nFor more info on RBAC, see\nhttps://kubernetes.io/docs/reference/access-authn-authz/rbac/#user-facing-roles"]
    pub fn admin_users(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.admin_users", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerAttachedClusterBinaryAuthorizationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    evaluation_mode: Option<PrimField<String>>,
}
impl ContainerAttachedClusterBinaryAuthorizationEl {
    #[doc = "Set the field `evaluation_mode`.\nConfigure Binary Authorization evaluation mode. Possible values: [\"DISABLED\", \"PROJECT_SINGLETON_POLICY_ENFORCE\"]"]
    pub fn set_evaluation_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.evaluation_mode = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerAttachedClusterBinaryAuthorizationEl {
    type O = BlockAssignable<ContainerAttachedClusterBinaryAuthorizationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAttachedClusterBinaryAuthorizationEl {}
impl BuildContainerAttachedClusterBinaryAuthorizationEl {
    pub fn build(self) -> ContainerAttachedClusterBinaryAuthorizationEl {
        ContainerAttachedClusterBinaryAuthorizationEl {
            evaluation_mode: core::default::Default::default(),
        }
    }
}
pub struct ContainerAttachedClusterBinaryAuthorizationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAttachedClusterBinaryAuthorizationElRef {
    fn new(shared: StackShared, base: String) -> ContainerAttachedClusterBinaryAuthorizationElRef {
        ContainerAttachedClusterBinaryAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAttachedClusterBinaryAuthorizationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `evaluation_mode` after provisioning.\nConfigure Binary Authorization evaluation mode. Possible values: [\"DISABLED\", \"PROJECT_SINGLETON_POLICY_ENFORCE\"]"]
    pub fn evaluation_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.evaluation_mode", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerAttachedClusterFleetEl {
    project: PrimField<String>,
}
impl ContainerAttachedClusterFleetEl {}
impl ToListMappable for ContainerAttachedClusterFleetEl {
    type O = BlockAssignable<ContainerAttachedClusterFleetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAttachedClusterFleetEl {
    #[doc = "The number of the Fleet host project where this cluster will be registered."]
    pub project: PrimField<String>,
}
impl BuildContainerAttachedClusterFleetEl {
    pub fn build(self) -> ContainerAttachedClusterFleetEl {
        ContainerAttachedClusterFleetEl {
            project: self.project,
        }
    }
}
pub struct ContainerAttachedClusterFleetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAttachedClusterFleetElRef {
    fn new(shared: StackShared, base: String) -> ContainerAttachedClusterFleetElRef {
        ContainerAttachedClusterFleetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAttachedClusterFleetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `membership` after provisioning.\nThe name of the managed Hub Membership resource associated to this\ncluster. Membership names are formatted as\nprojects/<project-number>/locations/global/membership/<cluster-id>."]
    pub fn membership(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.membership", self.base))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe number of the Fleet host project where this cluster will be registered."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerAttachedClusterLoggingConfigElComponentConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_components: Option<ListField<PrimField<String>>>,
}
impl ContainerAttachedClusterLoggingConfigElComponentConfigEl {
    #[doc = "Set the field `enable_components`.\nThe components to be enabled. Possible values: [\"SYSTEM_COMPONENTS\", \"WORKLOADS\"]"]
    pub fn set_enable_components(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.enable_components = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerAttachedClusterLoggingConfigElComponentConfigEl {
    type O = BlockAssignable<ContainerAttachedClusterLoggingConfigElComponentConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAttachedClusterLoggingConfigElComponentConfigEl {}
impl BuildContainerAttachedClusterLoggingConfigElComponentConfigEl {
    pub fn build(self) -> ContainerAttachedClusterLoggingConfigElComponentConfigEl {
        ContainerAttachedClusterLoggingConfigElComponentConfigEl {
            enable_components: core::default::Default::default(),
        }
    }
}
pub struct ContainerAttachedClusterLoggingConfigElComponentConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAttachedClusterLoggingConfigElComponentConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerAttachedClusterLoggingConfigElComponentConfigElRef {
        ContainerAttachedClusterLoggingConfigElComponentConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAttachedClusterLoggingConfigElComponentConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_components` after provisioning.\nThe components to be enabled. Possible values: [\"SYSTEM_COMPONENTS\", \"WORKLOADS\"]"]
    pub fn enable_components(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.enable_components", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ContainerAttachedClusterLoggingConfigElDynamic {
    component_config:
        Option<DynamicBlock<ContainerAttachedClusterLoggingConfigElComponentConfigEl>>,
}
#[derive(Serialize)]
pub struct ContainerAttachedClusterLoggingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    component_config: Option<Vec<ContainerAttachedClusterLoggingConfigElComponentConfigEl>>,
    dynamic: ContainerAttachedClusterLoggingConfigElDynamic,
}
impl ContainerAttachedClusterLoggingConfigEl {
    #[doc = "Set the field `component_config`.\n"]
    pub fn set_component_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerAttachedClusterLoggingConfigElComponentConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.component_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.component_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ContainerAttachedClusterLoggingConfigEl {
    type O = BlockAssignable<ContainerAttachedClusterLoggingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAttachedClusterLoggingConfigEl {}
impl BuildContainerAttachedClusterLoggingConfigEl {
    pub fn build(self) -> ContainerAttachedClusterLoggingConfigEl {
        ContainerAttachedClusterLoggingConfigEl {
            component_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContainerAttachedClusterLoggingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAttachedClusterLoggingConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerAttachedClusterLoggingConfigElRef {
        ContainerAttachedClusterLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAttachedClusterLoggingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `component_config` after provisioning.\n"]
    pub fn component_config(
        &self,
    ) -> ListRef<ContainerAttachedClusterLoggingConfigElComponentConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.component_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerAttachedClusterMonitoringConfigElManagedPrometheusConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl ContainerAttachedClusterMonitoringConfigElManagedPrometheusConfigEl {
    #[doc = "Set the field `enabled`.\nEnable Managed Collection."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerAttachedClusterMonitoringConfigElManagedPrometheusConfigEl {
    type O = BlockAssignable<ContainerAttachedClusterMonitoringConfigElManagedPrometheusConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAttachedClusterMonitoringConfigElManagedPrometheusConfigEl {}
impl BuildContainerAttachedClusterMonitoringConfigElManagedPrometheusConfigEl {
    pub fn build(self) -> ContainerAttachedClusterMonitoringConfigElManagedPrometheusConfigEl {
        ContainerAttachedClusterMonitoringConfigElManagedPrometheusConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct ContainerAttachedClusterMonitoringConfigElManagedPrometheusConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAttachedClusterMonitoringConfigElManagedPrometheusConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerAttachedClusterMonitoringConfigElManagedPrometheusConfigElRef {
        ContainerAttachedClusterMonitoringConfigElManagedPrometheusConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAttachedClusterMonitoringConfigElManagedPrometheusConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nEnable Managed Collection."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize, Default)]
struct ContainerAttachedClusterMonitoringConfigElDynamic {
    managed_prometheus_config:
        Option<DynamicBlock<ContainerAttachedClusterMonitoringConfigElManagedPrometheusConfigEl>>,
}
#[derive(Serialize)]
pub struct ContainerAttachedClusterMonitoringConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_prometheus_config:
        Option<Vec<ContainerAttachedClusterMonitoringConfigElManagedPrometheusConfigEl>>,
    dynamic: ContainerAttachedClusterMonitoringConfigElDynamic,
}
impl ContainerAttachedClusterMonitoringConfigEl {
    #[doc = "Set the field `managed_prometheus_config`.\n"]
    pub fn set_managed_prometheus_config(
        mut self,
        v: impl Into<
            BlockAssignable<ContainerAttachedClusterMonitoringConfigElManagedPrometheusConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.managed_prometheus_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_prometheus_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ContainerAttachedClusterMonitoringConfigEl {
    type O = BlockAssignable<ContainerAttachedClusterMonitoringConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAttachedClusterMonitoringConfigEl {}
impl BuildContainerAttachedClusterMonitoringConfigEl {
    pub fn build(self) -> ContainerAttachedClusterMonitoringConfigEl {
        ContainerAttachedClusterMonitoringConfigEl {
            managed_prometheus_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContainerAttachedClusterMonitoringConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAttachedClusterMonitoringConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerAttachedClusterMonitoringConfigElRef {
        ContainerAttachedClusterMonitoringConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAttachedClusterMonitoringConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `managed_prometheus_config` after provisioning.\n"]
    pub fn managed_prometheus_config(
        &self,
    ) -> ListRef<ContainerAttachedClusterMonitoringConfigElManagedPrometheusConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_prometheus_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerAttachedClusterOidcConfigEl {
    issuer_url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jwks: Option<PrimField<String>>,
}
impl ContainerAttachedClusterOidcConfigEl {
    #[doc = "Set the field `jwks`.\nOIDC verification keys in JWKS format (RFC 7517)."]
    pub fn set_jwks(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.jwks = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerAttachedClusterOidcConfigEl {
    type O = BlockAssignable<ContainerAttachedClusterOidcConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAttachedClusterOidcConfigEl {
    #[doc = "A JSON Web Token (JWT) issuer URI. 'issuer' must start with 'https://'"]
    pub issuer_url: PrimField<String>,
}
impl BuildContainerAttachedClusterOidcConfigEl {
    pub fn build(self) -> ContainerAttachedClusterOidcConfigEl {
        ContainerAttachedClusterOidcConfigEl {
            issuer_url: self.issuer_url,
            jwks: core::default::Default::default(),
        }
    }
}
pub struct ContainerAttachedClusterOidcConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAttachedClusterOidcConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerAttachedClusterOidcConfigElRef {
        ContainerAttachedClusterOidcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAttachedClusterOidcConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `issuer_url` after provisioning.\nA JSON Web Token (JWT) issuer URI. 'issuer' must start with 'https://'"]
    pub fn issuer_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer_url", self.base))
    }
    #[doc = "Get a reference to the value of field `jwks` after provisioning.\nOIDC verification keys in JWKS format (RFC 7517)."]
    pub fn jwks(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.jwks", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerAttachedClusterProxyConfigElKubernetesSecretEl {
    name: PrimField<String>,
    namespace: PrimField<String>,
}
impl ContainerAttachedClusterProxyConfigElKubernetesSecretEl {}
impl ToListMappable for ContainerAttachedClusterProxyConfigElKubernetesSecretEl {
    type O = BlockAssignable<ContainerAttachedClusterProxyConfigElKubernetesSecretEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAttachedClusterProxyConfigElKubernetesSecretEl {
    #[doc = "Name of the kubernetes secret containing the proxy config."]
    pub name: PrimField<String>,
    #[doc = "Namespace of the kubernetes secret containing the proxy config."]
    pub namespace: PrimField<String>,
}
impl BuildContainerAttachedClusterProxyConfigElKubernetesSecretEl {
    pub fn build(self) -> ContainerAttachedClusterProxyConfigElKubernetesSecretEl {
        ContainerAttachedClusterProxyConfigElKubernetesSecretEl {
            name: self.name,
            namespace: self.namespace,
        }
    }
}
pub struct ContainerAttachedClusterProxyConfigElKubernetesSecretElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAttachedClusterProxyConfigElKubernetesSecretElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerAttachedClusterProxyConfigElKubernetesSecretElRef {
        ContainerAttachedClusterProxyConfigElKubernetesSecretElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAttachedClusterProxyConfigElKubernetesSecretElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the kubernetes secret containing the proxy config."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\nNamespace of the kubernetes secret containing the proxy config."]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}
#[derive(Serialize, Default)]
struct ContainerAttachedClusterProxyConfigElDynamic {
    kubernetes_secret:
        Option<DynamicBlock<ContainerAttachedClusterProxyConfigElKubernetesSecretEl>>,
}
#[derive(Serialize)]
pub struct ContainerAttachedClusterProxyConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kubernetes_secret: Option<Vec<ContainerAttachedClusterProxyConfigElKubernetesSecretEl>>,
    dynamic: ContainerAttachedClusterProxyConfigElDynamic,
}
impl ContainerAttachedClusterProxyConfigEl {
    #[doc = "Set the field `kubernetes_secret`.\n"]
    pub fn set_kubernetes_secret(
        mut self,
        v: impl Into<BlockAssignable<ContainerAttachedClusterProxyConfigElKubernetesSecretEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kubernetes_secret = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kubernetes_secret = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ContainerAttachedClusterProxyConfigEl {
    type O = BlockAssignable<ContainerAttachedClusterProxyConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAttachedClusterProxyConfigEl {}
impl BuildContainerAttachedClusterProxyConfigEl {
    pub fn build(self) -> ContainerAttachedClusterProxyConfigEl {
        ContainerAttachedClusterProxyConfigEl {
            kubernetes_secret: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContainerAttachedClusterProxyConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAttachedClusterProxyConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerAttachedClusterProxyConfigElRef {
        ContainerAttachedClusterProxyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAttachedClusterProxyConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kubernetes_secret` after provisioning.\n"]
    pub fn kubernetes_secret(
        &self,
    ) -> ListRef<ContainerAttachedClusterProxyConfigElKubernetesSecretElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kubernetes_secret", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerAttachedClusterSecurityPostureConfigEl {
    vulnerability_mode: PrimField<String>,
}
impl ContainerAttachedClusterSecurityPostureConfigEl {}
impl ToListMappable for ContainerAttachedClusterSecurityPostureConfigEl {
    type O = BlockAssignable<ContainerAttachedClusterSecurityPostureConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAttachedClusterSecurityPostureConfigEl {
    #[doc = "Sets the mode of the Kubernetes security posture API's workload vulnerability scanning. Possible values: [\"VULNERABILITY_DISABLED\", \"VULNERABILITY_ENTERPRISE\"]"]
    pub vulnerability_mode: PrimField<String>,
}
impl BuildContainerAttachedClusterSecurityPostureConfigEl {
    pub fn build(self) -> ContainerAttachedClusterSecurityPostureConfigEl {
        ContainerAttachedClusterSecurityPostureConfigEl {
            vulnerability_mode: self.vulnerability_mode,
        }
    }
}
pub struct ContainerAttachedClusterSecurityPostureConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAttachedClusterSecurityPostureConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerAttachedClusterSecurityPostureConfigElRef {
        ContainerAttachedClusterSecurityPostureConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAttachedClusterSecurityPostureConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `vulnerability_mode` after provisioning.\nSets the mode of the Kubernetes security posture API's workload vulnerability scanning. Possible values: [\"VULNERABILITY_DISABLED\", \"VULNERABILITY_ENTERPRISE\"]"]
    pub fn vulnerability_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vulnerability_mode", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerAttachedClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ContainerAttachedClusterTimeoutsEl {
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
impl ToListMappable for ContainerAttachedClusterTimeoutsEl {
    type O = BlockAssignable<ContainerAttachedClusterTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAttachedClusterTimeoutsEl {}
impl BuildContainerAttachedClusterTimeoutsEl {
    pub fn build(self) -> ContainerAttachedClusterTimeoutsEl {
        ContainerAttachedClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ContainerAttachedClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAttachedClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ContainerAttachedClusterTimeoutsElRef {
        ContainerAttachedClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAttachedClusterTimeoutsElRef {
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
struct ContainerAttachedClusterDynamic {
    authorization: Option<DynamicBlock<ContainerAttachedClusterAuthorizationEl>>,
    binary_authorization: Option<DynamicBlock<ContainerAttachedClusterBinaryAuthorizationEl>>,
    fleet: Option<DynamicBlock<ContainerAttachedClusterFleetEl>>,
    logging_config: Option<DynamicBlock<ContainerAttachedClusterLoggingConfigEl>>,
    monitoring_config: Option<DynamicBlock<ContainerAttachedClusterMonitoringConfigEl>>,
    oidc_config: Option<DynamicBlock<ContainerAttachedClusterOidcConfigEl>>,
    proxy_config: Option<DynamicBlock<ContainerAttachedClusterProxyConfigEl>>,
    security_posture_config: Option<DynamicBlock<ContainerAttachedClusterSecurityPostureConfigEl>>,
}
