use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct GkeonpremBareMetalNodePoolData {
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
    bare_metal_cluster: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_pool_config: Option<Vec<GkeonpremBareMetalNodePoolNodePoolConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<GkeonpremBareMetalNodePoolTimeoutsEl>,
    dynamic: GkeonpremBareMetalNodePoolDynamic,
}
struct GkeonpremBareMetalNodePool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GkeonpremBareMetalNodePoolData>,
}
#[derive(Clone)]
pub struct GkeonpremBareMetalNodePool(Rc<GkeonpremBareMetalNodePool_>);
impl GkeonpremBareMetalNodePool {
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
    #[doc = "Set the field `annotations`.\nAnnotations on the Bare Metal Node Pool.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nThe display name for the Bare Metal Node Pool."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
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
    #[doc = "Set the field `node_pool_config`.\n"]
    pub fn set_node_pool_config(
        self,
        v: impl Into<BlockAssignable<GkeonpremBareMetalNodePoolNodePoolConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_pool_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_pool_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<GkeonpremBareMetalNodePoolTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nAnnotations on the Bare Metal Node Pool.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bare_metal_cluster` after provisioning.\nThe cluster this node pool belongs to."]
    pub fn bare_metal_cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bare_metal_cluster", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name for the Bare Metal Node Pool."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe bare metal node pool name."]
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
    #[doc = "Get a reference to the value of field `status` after provisioning.\nSpecifies detailed node pool status."]
    pub fn status(&self) -> ListRef<GkeonpremBareMetalNodePoolStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe unique identifier of the Bare Metal Node Pool."]
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
    #[doc = "Get a reference to the value of field `node_pool_config` after provisioning.\n"]
    pub fn node_pool_config(&self) -> ListRef<GkeonpremBareMetalNodePoolNodePoolConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_pool_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeonpremBareMetalNodePoolTimeoutsElRef {
        GkeonpremBareMetalNodePoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for GkeonpremBareMetalNodePool {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for GkeonpremBareMetalNodePool {}
impl ToListMappable for GkeonpremBareMetalNodePool {
    type O = ListRef<GkeonpremBareMetalNodePoolRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for GkeonpremBareMetalNodePool_ {
    fn extract_resource_type(&self) -> String {
        "google_gkeonprem_bare_metal_node_pool".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildGkeonpremBareMetalNodePool {
    pub tf_id: String,
    #[doc = "The cluster this node pool belongs to."]
    pub bare_metal_cluster: PrimField<String>,
    #[doc = "The location of the resource."]
    pub location: PrimField<String>,
    #[doc = "The bare metal node pool name."]
    pub name: PrimField<String>,
}
impl BuildGkeonpremBareMetalNodePool {
    pub fn build(self, stack: &mut Stack) -> GkeonpremBareMetalNodePool {
        let out = GkeonpremBareMetalNodePool(Rc::new(GkeonpremBareMetalNodePool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GkeonpremBareMetalNodePoolData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                bare_metal_cluster: self.bare_metal_cluster,
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                node_pool_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct GkeonpremBareMetalNodePoolRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalNodePoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl GkeonpremBareMetalNodePoolRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nAnnotations on the Bare Metal Node Pool.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bare_metal_cluster` after provisioning.\nThe cluster this node pool belongs to."]
    pub fn bare_metal_cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bare_metal_cluster", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name for the Bare Metal Node Pool."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe bare metal node pool name."]
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
    #[doc = "Get a reference to the value of field `status` after provisioning.\nSpecifies detailed node pool status."]
    pub fn status(&self) -> ListRef<GkeonpremBareMetalNodePoolStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe unique identifier of the Bare Metal Node Pool."]
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
    #[doc = "Get a reference to the value of field `node_pool_config` after provisioning.\n"]
    pub fn node_pool_config(&self) -> ListRef<GkeonpremBareMetalNodePoolNodePoolConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_pool_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeonpremBareMetalNodePoolTimeoutsElRef {
        GkeonpremBareMetalNodePoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalNodePoolStatusElConditionsEl {
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
impl GkeonpremBareMetalNodePoolStatusElConditionsEl {
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
impl ToListMappable for GkeonpremBareMetalNodePoolStatusElConditionsEl {
    type O = BlockAssignable<GkeonpremBareMetalNodePoolStatusElConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalNodePoolStatusElConditionsEl {}
impl BuildGkeonpremBareMetalNodePoolStatusElConditionsEl {
    pub fn build(self) -> GkeonpremBareMetalNodePoolStatusElConditionsEl {
        GkeonpremBareMetalNodePoolStatusElConditionsEl {
            last_transition_time: core::default::Default::default(),
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
            state: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalNodePoolStatusElConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalNodePoolStatusElConditionsElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalNodePoolStatusElConditionsElRef {
        GkeonpremBareMetalNodePoolStatusElConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalNodePoolStatusElConditionsElRef {
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
pub struct GkeonpremBareMetalNodePoolStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions: Option<ListField<GkeonpremBareMetalNodePoolStatusElConditionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<PrimField<String>>,
}
impl GkeonpremBareMetalNodePoolStatusEl {
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl Into<ListField<GkeonpremBareMetalNodePoolStatusElConditionsEl>>,
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
impl ToListMappable for GkeonpremBareMetalNodePoolStatusEl {
    type O = BlockAssignable<GkeonpremBareMetalNodePoolStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalNodePoolStatusEl {}
impl BuildGkeonpremBareMetalNodePoolStatusEl {
    pub fn build(self) -> GkeonpremBareMetalNodePoolStatusEl {
        GkeonpremBareMetalNodePoolStatusEl {
            conditions: core::default::Default::default(),
            error_message: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalNodePoolStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalNodePoolStatusElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalNodePoolStatusElRef {
        GkeonpremBareMetalNodePoolStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalNodePoolStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> ListRef<GkeonpremBareMetalNodePoolStatusElConditionsElRef> {
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
pub struct GkeonpremBareMetalNodePoolNodePoolConfigElNodeConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_ip: Option<PrimField<String>>,
}
impl GkeonpremBareMetalNodePoolNodePoolConfigElNodeConfigsEl {
    #[doc = "Set the field `labels`.\nThe map of Kubernetes labels (key/value pairs) to be applied to\neach node. These will added in addition to any default label(s)\nthat Kubernetes may apply to the node. In case of conflict in\nlabel keys, the applied set may differ depending on the Kubernetes\nversion -- it's best to assume the behavior is undefined and\nconflicts should be avoided. For more information, including usage\nand the valid values, see:\n  - http://kubernetes.io/v1.1/docs/user-guide/labels.html\nAn object containing a list of \"key\": value pairs.\nFor example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
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
impl ToListMappable for GkeonpremBareMetalNodePoolNodePoolConfigElNodeConfigsEl {
    type O = BlockAssignable<GkeonpremBareMetalNodePoolNodePoolConfigElNodeConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalNodePoolNodePoolConfigElNodeConfigsEl {}
impl BuildGkeonpremBareMetalNodePoolNodePoolConfigElNodeConfigsEl {
    pub fn build(self) -> GkeonpremBareMetalNodePoolNodePoolConfigElNodeConfigsEl {
        GkeonpremBareMetalNodePoolNodePoolConfigElNodeConfigsEl {
            labels: core::default::Default::default(),
            node_ip: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalNodePoolNodePoolConfigElNodeConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalNodePoolNodePoolConfigElNodeConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalNodePoolNodePoolConfigElNodeConfigsElRef {
        GkeonpremBareMetalNodePoolNodePoolConfigElNodeConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalNodePoolNodePoolConfigElNodeConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe map of Kubernetes labels (key/value pairs) to be applied to\neach node. These will added in addition to any default label(s)\nthat Kubernetes may apply to the node. In case of conflict in\nlabel keys, the applied set may differ depending on the Kubernetes\nversion -- it's best to assume the behavior is undefined and\nconflicts should be avoided. For more information, including usage\nand the valid values, see:\n  - http://kubernetes.io/v1.1/docs/user-guide/labels.html\nAn object containing a list of \"key\": value pairs.\nFor example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `node_ip` after provisioning.\nThe default IPv4 address for SSH access and Kubernetes node.\nExample: 192.168.0.1"]
    pub fn node_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_ip", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalNodePoolNodePoolConfigElTaintsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl GkeonpremBareMetalNodePoolNodePoolConfigElTaintsEl {
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
impl ToListMappable for GkeonpremBareMetalNodePoolNodePoolConfigElTaintsEl {
    type O = BlockAssignable<GkeonpremBareMetalNodePoolNodePoolConfigElTaintsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalNodePoolNodePoolConfigElTaintsEl {}
impl BuildGkeonpremBareMetalNodePoolNodePoolConfigElTaintsEl {
    pub fn build(self) -> GkeonpremBareMetalNodePoolNodePoolConfigElTaintsEl {
        GkeonpremBareMetalNodePoolNodePoolConfigElTaintsEl {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalNodePoolNodePoolConfigElTaintsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalNodePoolNodePoolConfigElTaintsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeonpremBareMetalNodePoolNodePoolConfigElTaintsElRef {
        GkeonpremBareMetalNodePoolNodePoolConfigElTaintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalNodePoolNodePoolConfigElTaintsElRef {
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
struct GkeonpremBareMetalNodePoolNodePoolConfigElDynamic {
    node_configs: Option<DynamicBlock<GkeonpremBareMetalNodePoolNodePoolConfigElNodeConfigsEl>>,
    taints: Option<DynamicBlock<GkeonpremBareMetalNodePoolNodePoolConfigElTaintsEl>>,
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalNodePoolNodePoolConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operating_system: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_configs: Option<Vec<GkeonpremBareMetalNodePoolNodePoolConfigElNodeConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    taints: Option<Vec<GkeonpremBareMetalNodePoolNodePoolConfigElTaintsEl>>,
    dynamic: GkeonpremBareMetalNodePoolNodePoolConfigElDynamic,
}
impl GkeonpremBareMetalNodePoolNodePoolConfigEl {
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
        v: impl Into<BlockAssignable<GkeonpremBareMetalNodePoolNodePoolConfigElNodeConfigsEl>>,
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
        v: impl Into<BlockAssignable<GkeonpremBareMetalNodePoolNodePoolConfigElTaintsEl>>,
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
impl ToListMappable for GkeonpremBareMetalNodePoolNodePoolConfigEl {
    type O = BlockAssignable<GkeonpremBareMetalNodePoolNodePoolConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalNodePoolNodePoolConfigEl {}
impl BuildGkeonpremBareMetalNodePoolNodePoolConfigEl {
    pub fn build(self) -> GkeonpremBareMetalNodePoolNodePoolConfigEl {
        GkeonpremBareMetalNodePoolNodePoolConfigEl {
            labels: core::default::Default::default(),
            operating_system: core::default::Default::default(),
            node_configs: core::default::Default::default(),
            taints: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalNodePoolNodePoolConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalNodePoolNodePoolConfigElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalNodePoolNodePoolConfigElRef {
        GkeonpremBareMetalNodePoolNodePoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalNodePoolNodePoolConfigElRef {
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
    #[doc = "Get a reference to the value of field `node_configs` after provisioning.\n"]
    pub fn node_configs(
        &self,
    ) -> ListRef<GkeonpremBareMetalNodePoolNodePoolConfigElNodeConfigsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_configs", self.base))
    }
    #[doc = "Get a reference to the value of field `taints` after provisioning.\n"]
    pub fn taints(&self) -> ListRef<GkeonpremBareMetalNodePoolNodePoolConfigElTaintsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.taints", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeonpremBareMetalNodePoolTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl GkeonpremBareMetalNodePoolTimeoutsEl {
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
impl ToListMappable for GkeonpremBareMetalNodePoolTimeoutsEl {
    type O = BlockAssignable<GkeonpremBareMetalNodePoolTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeonpremBareMetalNodePoolTimeoutsEl {}
impl BuildGkeonpremBareMetalNodePoolTimeoutsEl {
    pub fn build(self) -> GkeonpremBareMetalNodePoolTimeoutsEl {
        GkeonpremBareMetalNodePoolTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct GkeonpremBareMetalNodePoolTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeonpremBareMetalNodePoolTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> GkeonpremBareMetalNodePoolTimeoutsElRef {
        GkeonpremBareMetalNodePoolTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeonpremBareMetalNodePoolTimeoutsElRef {
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
struct GkeonpremBareMetalNodePoolDynamic {
    node_pool_config: Option<DynamicBlock<GkeonpremBareMetalNodePoolNodePoolConfigEl>>,
}
