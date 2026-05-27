use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct GkeonpremVmwareNodePoolData {
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
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_prem_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    vmware_cluster: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Vec<GkeonpremVmwareNodePoolConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_pool_autoscaling: Option<Vec<GkeonpremVmwareNodePoolNodePoolAutoscalingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<GkeonpremVmwareNodePoolTimeoutsEl>,
    dynamic: GkeonpremVmwareNodePoolDynamic,
}
struct GkeonpremVmwareNodePool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GkeonpremVmwareNodePoolData>,
}
#[derive(Clone)]
pub struct GkeonpremVmwareNodePool(Rc<GkeonpremVmwareNodePool_>);
impl GkeonpremVmwareNodePool {
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
    #[doc = "Set the field `annotations`.\nAnnotations on the node Pool.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nThe display name for the node pool."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `on_prem_version`.\nAnthos version for the node pool. Defaults to the user cluster version."]
    pub fn set_on_prem_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().on_prem_version = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        self,
        v: impl Into<BlockAssignable<GkeonpremVmwareNodePoolConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `node_pool_autoscaling`.\n"]
    pub fn set_node_pool_autoscaling(
        self,
        v: impl Into<BlockAssignable<GkeonpremVmwareNodePoolNodePoolAutoscalingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_pool_autoscaling = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_pool_autoscaling = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<GkeonpremVmwareNodePoolTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nAnnotations on the node Pool.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name for the node pool."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other\nfields, and may be sent on update and delete requests to ensure the\nclient has an up-to-date value before proceeding.\nAllows clients to perform consistent read-modify-writes\nthrough optimistic concurrency control."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe vmware node pool name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `on_prem_version` after provisioning.\nAnthos version for the node pool. Defaults to the user cluster version."]
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
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the node pool."]
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
    #[doc = "Get a reference to the value of field `status` after provisioning.\nResourceStatus representing detailed cluster state."]
    pub fn status(&self) -> ListRef<GkeonpremVmwareNodePoolStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe unique identifier of the node pool."]
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
    #[doc = "Get a reference to the value of field `vmware_cluster` after provisioning.\nThe cluster this node pool belongs to."]
    pub fn vmware_cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vmware_cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<GkeonpremVmwareNodePoolConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_pool_autoscaling` after provisioning.\n"]
    pub fn node_pool_autoscaling(
        &self,
    ) -> ListRef<GkeonpremVmwareNodePoolNodePoolAutoscalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_pool_autoscaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeonpremVmwareNodePoolTimeoutsElRef {
        GkeonpremVmwareNodePoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for GkeonpremVmwareNodePool {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for GkeonpremVmwareNodePool {}
impl ToListMappable for GkeonpremVmwareNodePool {
    type O = ListRef<GkeonpremVmwareNodePoolRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for GkeonpremVmwareNodePool_ {
    fn extract_resource_type(&self) -> String {
        "google_gkeonprem_vmware_node_pool".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildGkeonpremVmwareNodePool {
    pub tf_id: String,
    #[doc = "The location of the resource."]
    pub location: PrimField<String>,
    #[doc = "The vmware node pool name."]
    pub name: PrimField<String>,
    #[doc = "The cluster this node pool belongs to."]
    pub vmware_cluster: PrimField<String>,
}
impl BuildGkeonpremVmwareNodePool {
    pub fn build(self, stack: &mut Stack) -> GkeonpremVmwareNodePool {
        let out = GkeonpremVmwareNodePool(Rc::new(GkeonpremVmwareNodePool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GkeonpremVmwareNodePoolData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                on_prem_version: core::default::Default::default(),
                project: core::default::Default::default(),
                vmware_cluster: self.vmware_cluster,
                config: core::default::Default::default(),
                node_pool_autoscaling: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct GkeonpremVmwareNodePoolRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareNodePoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl GkeonpremVmwareNodePoolRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nAnnotations on the node Pool.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name for the node pool."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other\nfields, and may be sent on update and delete requests to ensure the\nclient has an up-to-date value before proceeding.\nAllows clients to perform consistent read-modify-writes\nthrough optimistic concurrency control."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe vmware node pool name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `on_prem_version` after provisioning.\nAnthos version for the node pool. Defaults to the user cluster version."]
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
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the node pool."]
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
    #[doc = "Get a reference to the value of field `status` after provisioning.\nResourceStatus representing detailed cluster state."]
    pub fn status(&self) -> ListRef<GkeonpremVmwareNodePoolStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe unique identifier of the node pool."]
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
    #[doc = "Get a reference to the value of field `vmware_cluster` after provisioning.\nThe cluster this node pool belongs to."]
    pub fn vmware_cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vmware_cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<GkeonpremVmwareNodePoolConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_pool_autoscaling` after provisioning.\n"]
    pub fn node_pool_autoscaling(
        &self,
    ) -> ListRef<GkeonpremVmwareNodePoolNodePoolAutoscalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_pool_autoscaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeonpremVmwareNodePoolTimeoutsElRef {
        GkeonpremVmwareNodePoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareNodePoolStatusElConditionsEl {
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
impl GkeonpremVmwareNodePoolStatusElConditionsEl {
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
impl ToListMappable for GkeonpremVmwareNodePoolStatusElConditionsEl {
    type O = BlockAssignable<GkeonpremVmwareNodePoolStatusElConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareNodePoolStatusElConditionsEl {}
impl BuildGkeonpremVmwareNodePoolStatusElConditionsEl {
    pub fn build(self) -> GkeonpremVmwareNodePoolStatusElConditionsEl {
        GkeonpremVmwareNodePoolStatusElConditionsEl {
            last_transition_time: core::default::Default::default(),
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
            state: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareNodePoolStatusElConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareNodePoolStatusElConditionsElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareNodePoolStatusElConditionsElRef {
        GkeonpremVmwareNodePoolStatusElConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareNodePoolStatusElConditionsElRef {
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
pub struct GkeonpremVmwareNodePoolStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions: Option<ListField<GkeonpremVmwareNodePoolStatusElConditionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<PrimField<String>>,
}
impl GkeonpremVmwareNodePoolStatusEl {
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl Into<ListField<GkeonpremVmwareNodePoolStatusElConditionsEl>>,
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
impl ToListMappable for GkeonpremVmwareNodePoolStatusEl {
    type O = BlockAssignable<GkeonpremVmwareNodePoolStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareNodePoolStatusEl {}
impl BuildGkeonpremVmwareNodePoolStatusEl {
    pub fn build(self) -> GkeonpremVmwareNodePoolStatusEl {
        GkeonpremVmwareNodePoolStatusEl {
            conditions: core::default::Default::default(),
            error_message: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareNodePoolStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareNodePoolStatusElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareNodePoolStatusElRef {
        GkeonpremVmwareNodePoolStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareNodePoolStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> ListRef<GkeonpremVmwareNodePoolStatusElConditionsElRef> {
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
pub struct GkeonpremVmwareNodePoolConfigElTaintsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<PrimField<String>>,
    key: PrimField<String>,
    value: PrimField<String>,
}
impl GkeonpremVmwareNodePoolConfigElTaintsEl {
    #[doc = "Set the field `effect`.\nAvailable taint effects. Possible values: [\"EFFECT_UNSPECIFIED\", \"NO_SCHEDULE\", \"PREFER_NO_SCHEDULE\", \"NO_EXECUTE\"]"]
    pub fn set_effect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.effect = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareNodePoolConfigElTaintsEl {
    type O = BlockAssignable<GkeonpremVmwareNodePoolConfigElTaintsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareNodePoolConfigElTaintsEl {
    #[doc = "Key associated with the effect."]
    pub key: PrimField<String>,
    #[doc = "Value associated with the effect."]
    pub value: PrimField<String>,
}
impl BuildGkeonpremVmwareNodePoolConfigElTaintsEl {
    pub fn build(self) -> GkeonpremVmwareNodePoolConfigElTaintsEl {
        GkeonpremVmwareNodePoolConfigElTaintsEl {
            effect: core::default::Default::default(),
            key: self.key,
            value: self.value,
        }
    }
}
pub struct GkeonpremVmwareNodePoolConfigElTaintsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareNodePoolConfigElTaintsElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareNodePoolConfigElTaintsElRef {
        GkeonpremVmwareNodePoolConfigElTaintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareNodePoolConfigElTaintsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `effect` after provisioning.\nAvailable taint effects. Possible values: [\"EFFECT_UNSPECIFIED\", \"NO_SCHEDULE\", \"PREFER_NO_SCHEDULE\", \"NO_EXECUTE\"]"]
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
#[derive(Serialize)]
pub struct GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
}
impl GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsEl {
    #[doc = "Set the field `category`.\nThe Vsphere tag category."]
    pub fn set_category(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.category = Some(v.into());
        self
    }
    #[doc = "Set the field `tag`.\nThe Vsphere tag name."]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
}
impl ToListMappable for GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsEl {
    type O = BlockAssignable<GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareNodePoolConfigElVsphereConfigElTagsEl {}
impl BuildGkeonpremVmwareNodePoolConfigElVsphereConfigElTagsEl {
    pub fn build(self) -> GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsEl {
        GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsEl {
            category: core::default::Default::default(),
            tag: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsElRef {
        GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `category` after provisioning.\nThe Vsphere tag category."]
    pub fn category(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\nThe Vsphere tag name."]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremVmwareNodePoolConfigElVsphereConfigElDynamic {
    tags: Option<DynamicBlock<GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsEl>>,
}
#[derive(Serialize)]
pub struct GkeonpremVmwareNodePoolConfigElVsphereConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    datastore: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_groups: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsEl>>,
    dynamic: GkeonpremVmwareNodePoolConfigElVsphereConfigElDynamic,
}
impl GkeonpremVmwareNodePoolConfigElVsphereConfigEl {
    #[doc = "Set the field `datastore`.\nThe name of the vCenter datastore. Inherited from the user cluster."]
    pub fn set_datastore(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.datastore = Some(v.into());
        self
    }
    #[doc = "Set the field `host_groups`.\nVsphere host groups to apply to all VMs in the node pool"]
    pub fn set_host_groups(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.host_groups = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tags = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeonpremVmwareNodePoolConfigElVsphereConfigEl {
    type O = BlockAssignable<GkeonpremVmwareNodePoolConfigElVsphereConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareNodePoolConfigElVsphereConfigEl {}
impl BuildGkeonpremVmwareNodePoolConfigElVsphereConfigEl {
    pub fn build(self) -> GkeonpremVmwareNodePoolConfigElVsphereConfigEl {
        GkeonpremVmwareNodePoolConfigElVsphereConfigEl {
            datastore: core::default::Default::default(),
            host_groups: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremVmwareNodePoolConfigElVsphereConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareNodePoolConfigElVsphereConfigElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareNodePoolConfigElVsphereConfigElRef {
        GkeonpremVmwareNodePoolConfigElVsphereConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareNodePoolConfigElVsphereConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `datastore` after provisioning.\nThe name of the vCenter datastore. Inherited from the user cluster."]
    pub fn datastore(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.datastore", self.base))
    }
    #[doc = "Get a reference to the value of field `host_groups` after provisioning.\nVsphere host groups to apply to all VMs in the node pool"]
    pub fn host_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.host_groups", self.base))
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeonpremVmwareNodePoolConfigElDynamic {
    taints: Option<DynamicBlock<GkeonpremVmwareNodePoolConfigElTaintsEl>>,
    vsphere_config: Option<DynamicBlock<GkeonpremVmwareNodePoolConfigElVsphereConfigEl>>,
}
#[derive(Serialize)]
pub struct GkeonpremVmwareNodePoolConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpus: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_load_balancer: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<PrimField<String>>,
    image_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_mb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replicas: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    taints: Option<Vec<GkeonpremVmwareNodePoolConfigElTaintsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vsphere_config: Option<Vec<GkeonpremVmwareNodePoolConfigElVsphereConfigEl>>,
    dynamic: GkeonpremVmwareNodePoolConfigElDynamic,
}
impl GkeonpremVmwareNodePoolConfigEl {
    #[doc = "Set the field `boot_disk_size_gb`.\nVMware disk size to be used during creation."]
    pub fn set_boot_disk_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.boot_disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `cpus`.\nThe number of CPUs for each node in the node pool."]
    pub fn set_cpus(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpus = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_load_balancer`.\nAllow node pool traffic to be load balanced. Only works for clusters with\nMetalLB load balancers."]
    pub fn set_enable_load_balancer(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_load_balancer = Some(v.into());
        self
    }
    #[doc = "Set the field `image`.\nThe OS image name in vCenter, only valid when using Windows."]
    pub fn set_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nThe map of Kubernetes labels (key/value pairs) to be applied to each node.\nThese will added in addition to any default label(s) that\nKubernetes may apply to the node.\nIn case of conflict in label keys, the applied set may differ depending on\nthe Kubernetes version -- it's best to assume the behavior is undefined\nand conflicts should be avoided."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_mb`.\nThe megabytes of memory for each node in the node pool."]
    pub fn set_memory_mb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_mb = Some(v.into());
        self
    }
    #[doc = "Set the field `replicas`.\nThe number of nodes in the node pool."]
    pub fn set_replicas(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.replicas = Some(v.into());
        self
    }
    #[doc = "Set the field `taints`.\n"]
    pub fn set_taints(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremVmwareNodePoolConfigElTaintsEl>>,
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
    #[doc = "Set the field `vsphere_config`.\n"]
    pub fn set_vsphere_config(
        mut self,
        v: impl Into<BlockAssignable<GkeonpremVmwareNodePoolConfigElVsphereConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vsphere_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vsphere_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeonpremVmwareNodePoolConfigEl {
    type O = BlockAssignable<GkeonpremVmwareNodePoolConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareNodePoolConfigEl {
    #[doc = "The OS image to be used for each node in a node pool.\nCurrently 'cos', 'cos_cgv2', 'ubuntu', 'ubuntu_cgv2', 'ubuntu_containerd' and 'windows' are supported."]
    pub image_type: PrimField<String>,
}
impl BuildGkeonpremVmwareNodePoolConfigEl {
    pub fn build(self) -> GkeonpremVmwareNodePoolConfigEl {
        GkeonpremVmwareNodePoolConfigEl {
            boot_disk_size_gb: core::default::Default::default(),
            cpus: core::default::Default::default(),
            enable_load_balancer: core::default::Default::default(),
            image: core::default::Default::default(),
            image_type: self.image_type,
            labels: core::default::Default::default(),
            memory_mb: core::default::Default::default(),
            replicas: core::default::Default::default(),
            taints: core::default::Default::default(),
            vsphere_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremVmwareNodePoolConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareNodePoolConfigElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareNodePoolConfigElRef {
        GkeonpremVmwareNodePoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareNodePoolConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `boot_disk_size_gb` after provisioning.\nVMware disk size to be used during creation."]
    pub fn boot_disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.boot_disk_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cpus` after provisioning.\nThe number of CPUs for each node in the node pool."]
    pub fn cpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpus", self.base))
    }
    #[doc = "Get a reference to the value of field `enable_load_balancer` after provisioning.\nAllow node pool traffic to be load balanced. Only works for clusters with\nMetalLB load balancers."]
    pub fn enable_load_balancer(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_load_balancer", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image` after provisioning.\nThe OS image name in vCenter, only valid when using Windows."]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
    #[doc = "Get a reference to the value of field `image_type` after provisioning.\nThe OS image to be used for each node in a node pool.\nCurrently 'cos', 'cos_cgv2', 'ubuntu', 'ubuntu_cgv2', 'ubuntu_containerd' and 'windows' are supported."]
    pub fn image_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_type", self.base))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe map of Kubernetes labels (key/value pairs) to be applied to each node.\nThese will added in addition to any default label(s) that\nKubernetes may apply to the node.\nIn case of conflict in label keys, the applied set may differ depending on\nthe Kubernetes version -- it's best to assume the behavior is undefined\nand conflicts should be avoided."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `memory_mb` after provisioning.\nThe megabytes of memory for each node in the node pool."]
    pub fn memory_mb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_mb", self.base))
    }
    #[doc = "Get a reference to the value of field `replicas` after provisioning.\nThe number of nodes in the node pool."]
    pub fn replicas(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.replicas", self.base))
    }
    #[doc = "Get a reference to the value of field `taints` after provisioning.\n"]
    pub fn taints(&self) -> ListRef<GkeonpremVmwareNodePoolConfigElTaintsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.taints", self.base))
    }
    #[doc = "Get a reference to the value of field `vsphere_config` after provisioning.\n"]
    pub fn vsphere_config(&self) -> ListRef<GkeonpremVmwareNodePoolConfigElVsphereConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vsphere_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareNodePoolNodePoolAutoscalingEl {
    max_replicas: PrimField<f64>,
    min_replicas: PrimField<f64>,
}
impl GkeonpremVmwareNodePoolNodePoolAutoscalingEl {}
impl ToListMappable for GkeonpremVmwareNodePoolNodePoolAutoscalingEl {
    type O = BlockAssignable<GkeonpremVmwareNodePoolNodePoolAutoscalingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareNodePoolNodePoolAutoscalingEl {
    #[doc = "Maximum number of replicas in the NodePool."]
    pub max_replicas: PrimField<f64>,
    #[doc = "Minimum number of replicas in the NodePool."]
    pub min_replicas: PrimField<f64>,
}
impl BuildGkeonpremVmwareNodePoolNodePoolAutoscalingEl {
    pub fn build(self) -> GkeonpremVmwareNodePoolNodePoolAutoscalingEl {
        GkeonpremVmwareNodePoolNodePoolAutoscalingEl {
            max_replicas: self.max_replicas,
            min_replicas: self.min_replicas,
        }
    }
}
pub struct GkeonpremVmwareNodePoolNodePoolAutoscalingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareNodePoolNodePoolAutoscalingElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareNodePoolNodePoolAutoscalingElRef {
        GkeonpremVmwareNodePoolNodePoolAutoscalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareNodePoolNodePoolAutoscalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_replicas` after provisioning.\nMaximum number of replicas in the NodePool."]
    pub fn max_replicas(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_replicas", self.base))
    }
    #[doc = "Get a reference to the value of field `min_replicas` after provisioning.\nMinimum number of replicas in the NodePool."]
    pub fn min_replicas(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_replicas", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremVmwareNodePoolTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl GkeonpremVmwareNodePoolTimeoutsEl {
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
impl ToListMappable for GkeonpremVmwareNodePoolTimeoutsEl {
    type O = BlockAssignable<GkeonpremVmwareNodePoolTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremVmwareNodePoolTimeoutsEl {}
impl BuildGkeonpremVmwareNodePoolTimeoutsEl {
    pub fn build(self) -> GkeonpremVmwareNodePoolTimeoutsEl {
        GkeonpremVmwareNodePoolTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremVmwareNodePoolTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremVmwareNodePoolTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremVmwareNodePoolTimeoutsElRef {
        GkeonpremVmwareNodePoolTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremVmwareNodePoolTimeoutsElRef {
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
struct GkeonpremVmwareNodePoolDynamic {
    config: Option<DynamicBlock<GkeonpremVmwareNodePoolConfigEl>>,
    node_pool_autoscaling: Option<DynamicBlock<GkeonpremVmwareNodePoolNodePoolAutoscalingEl>>,
}
