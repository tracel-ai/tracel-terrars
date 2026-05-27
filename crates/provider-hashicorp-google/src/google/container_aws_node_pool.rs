use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ContainerAwsNodePoolData {
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
    cluster: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    subnet_id: PrimField<String>,
    version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling: Option<Vec<ContainerAwsNodePoolAutoscalingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Vec<ContainerAwsNodePoolConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kubelet_config: Option<Vec<ContainerAwsNodePoolKubeletConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    management: Option<Vec<ContainerAwsNodePoolManagementEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_pods_constraint: Option<Vec<ContainerAwsNodePoolMaxPodsConstraintEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ContainerAwsNodePoolTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_settings: Option<Vec<ContainerAwsNodePoolUpdateSettingsEl>>,
    dynamic: ContainerAwsNodePoolDynamic,
}
struct ContainerAwsNodePool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ContainerAwsNodePoolData>,
}
#[derive(Clone)]
pub struct ContainerAwsNodePool(Rc<ContainerAwsNodePool_>);
impl ContainerAwsNodePool {
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
    #[doc = "Set the field `annotations`.\nOptional. Annotations on the node pool. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
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
    #[doc = "Set the field `project`.\nThe project for the resource"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `autoscaling`.\n"]
    pub fn set_autoscaling(
        self,
        v: impl Into<BlockAssignable<ContainerAwsNodePoolAutoscalingEl>>,
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
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(self, v: impl Into<BlockAssignable<ContainerAwsNodePoolConfigEl>>) -> Self {
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
    #[doc = "Set the field `kubelet_config`.\n"]
    pub fn set_kubelet_config(
        self,
        v: impl Into<BlockAssignable<ContainerAwsNodePoolKubeletConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kubelet_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kubelet_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `management`.\n"]
    pub fn set_management(
        self,
        v: impl Into<BlockAssignable<ContainerAwsNodePoolManagementEl>>,
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
    #[doc = "Set the field `max_pods_constraint`.\n"]
    pub fn set_max_pods_constraint(
        self,
        v: impl Into<BlockAssignable<ContainerAwsNodePoolMaxPodsConstraintEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().max_pods_constraint = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.max_pods_constraint = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ContainerAwsNodePoolTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `update_settings`.\n"]
    pub fn set_update_settings(
        self,
        v: impl Into<BlockAssignable<ContainerAwsNodePoolUpdateSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().update_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.update_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nOptional. Annotations on the node pool. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\nThe awsCluster for the resource"]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which this node pool was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nAllows clients to perform consistent read-modify-writes through optimistic concurrency control. May be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
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
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. If set, there are currently changes in flight to the node pool."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The lifecycle state of the node pool. Possible values: STATE_UNSPECIFIED, PROVISIONING, RUNNING, RECONCILING, STOPPING, ERROR, DEGRADED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\nThe subnet where the node pool node run."]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnet_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. A globally unique identifier for the node pool."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time at which this node pool was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe Kubernetes version to run on this node pool (e.g. `1.19.10-gke.1000`). You can list all supported versions on a given Google Cloud region by calling GetAwsServerConfig."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `autoscaling` after provisioning.\n"]
    pub fn autoscaling(&self) -> ListRef<ContainerAwsNodePoolAutoscalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<ContainerAwsNodePoolConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kubelet_config` after provisioning.\n"]
    pub fn kubelet_config(&self) -> ListRef<ContainerAwsNodePoolKubeletConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kubelet_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> ListRef<ContainerAwsNodePoolManagementElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.management", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_pods_constraint` after provisioning.\n"]
    pub fn max_pods_constraint(&self) -> ListRef<ContainerAwsNodePoolMaxPodsConstraintElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.max_pods_constraint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerAwsNodePoolTimeoutsElRef {
        ContainerAwsNodePoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_settings` after provisioning.\n"]
    pub fn update_settings(&self) -> ListRef<ContainerAwsNodePoolUpdateSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.update_settings", self.extract_ref()),
        )
    }
}
impl Referable for ContainerAwsNodePool {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ContainerAwsNodePool {}
impl ToListMappable for ContainerAwsNodePool {
    type O = ListRef<ContainerAwsNodePoolRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ContainerAwsNodePool_ {
    fn extract_resource_type(&self) -> String {
        "google_container_aws_node_pool".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildContainerAwsNodePool {
    pub tf_id: String,
    #[doc = "The awsCluster for the resource"]
    pub cluster: PrimField<String>,
    #[doc = "The location for the resource"]
    pub location: PrimField<String>,
    #[doc = "The name of this resource."]
    pub name: PrimField<String>,
    #[doc = "The subnet where the node pool node run."]
    pub subnet_id: PrimField<String>,
    #[doc = "The Kubernetes version to run on this node pool (e.g. `1.19.10-gke.1000`). You can list all supported versions on a given Google Cloud region by calling GetAwsServerConfig."]
    pub version: PrimField<String>,
}
impl BuildContainerAwsNodePool {
    pub fn build(self, stack: &mut Stack) -> ContainerAwsNodePool {
        let out = ContainerAwsNodePool(Rc::new(ContainerAwsNodePool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ContainerAwsNodePoolData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                cluster: self.cluster,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                subnet_id: self.subnet_id,
                version: self.version,
                autoscaling: core::default::Default::default(),
                config: core::default::Default::default(),
                kubelet_config: core::default::Default::default(),
                management: core::default::Default::default(),
                max_pods_constraint: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                update_settings: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ContainerAwsNodePoolRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAwsNodePoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ContainerAwsNodePoolRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nOptional. Annotations on the node pool. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\nThe awsCluster for the resource"]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which this node pool was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nAllows clients to perform consistent read-modify-writes through optimistic concurrency control. May be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
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
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. If set, there are currently changes in flight to the node pool."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The lifecycle state of the node pool. Possible values: STATE_UNSPECIFIED, PROVISIONING, RUNNING, RECONCILING, STOPPING, ERROR, DEGRADED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\nThe subnet where the node pool node run."]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnet_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. A globally unique identifier for the node pool."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time at which this node pool was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe Kubernetes version to run on this node pool (e.g. `1.19.10-gke.1000`). You can list all supported versions on a given Google Cloud region by calling GetAwsServerConfig."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `autoscaling` after provisioning.\n"]
    pub fn autoscaling(&self) -> ListRef<ContainerAwsNodePoolAutoscalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<ContainerAwsNodePoolConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kubelet_config` after provisioning.\n"]
    pub fn kubelet_config(&self) -> ListRef<ContainerAwsNodePoolKubeletConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kubelet_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> ListRef<ContainerAwsNodePoolManagementElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.management", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_pods_constraint` after provisioning.\n"]
    pub fn max_pods_constraint(&self) -> ListRef<ContainerAwsNodePoolMaxPodsConstraintElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.max_pods_constraint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerAwsNodePoolTimeoutsElRef {
        ContainerAwsNodePoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_settings` after provisioning.\n"]
    pub fn update_settings(&self) -> ListRef<ContainerAwsNodePoolUpdateSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.update_settings", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerAwsNodePoolAutoscalingEl {
    max_node_count: PrimField<f64>,
    min_node_count: PrimField<f64>,
}
impl ContainerAwsNodePoolAutoscalingEl {}
impl ToListMappable for ContainerAwsNodePoolAutoscalingEl {
    type O = BlockAssignable<ContainerAwsNodePoolAutoscalingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAwsNodePoolAutoscalingEl {
    #[doc = "Maximum number of nodes in the NodePool. Must be >= min_node_count."]
    pub max_node_count: PrimField<f64>,
    #[doc = "Minimum number of nodes in the NodePool. Must be >= 1 and <= max_node_count."]
    pub min_node_count: PrimField<f64>,
}
impl BuildContainerAwsNodePoolAutoscalingEl {
    pub fn build(self) -> ContainerAwsNodePoolAutoscalingEl {
        ContainerAwsNodePoolAutoscalingEl {
            max_node_count: self.max_node_count,
            min_node_count: self.min_node_count,
        }
    }
}
pub struct ContainerAwsNodePoolAutoscalingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAwsNodePoolAutoscalingElRef {
    fn new(shared: StackShared, base: String) -> ContainerAwsNodePoolAutoscalingElRef {
        ContainerAwsNodePoolAutoscalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAwsNodePoolAutoscalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_node_count` after provisioning.\nMaximum number of nodes in the NodePool. Must be >= min_node_count."]
    pub fn max_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_node_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_node_count` after provisioning.\nMinimum number of nodes in the NodePool. Must be >= 1 and <= max_node_count."]
    pub fn min_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_node_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerAwsNodePoolConfigElAutoscalingMetricsCollectionEl {
    granularity: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metrics: Option<ListField<PrimField<String>>>,
}
impl ContainerAwsNodePoolConfigElAutoscalingMetricsCollectionEl {
    #[doc = "Set the field `metrics`.\nThe metrics to enable. For a list of valid metrics, see https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_EnableMetricsCollection.html. If you specify granularity and don't specify any metrics, all metrics are enabled."]
    pub fn set_metrics(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.metrics = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerAwsNodePoolConfigElAutoscalingMetricsCollectionEl {
    type O = BlockAssignable<ContainerAwsNodePoolConfigElAutoscalingMetricsCollectionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAwsNodePoolConfigElAutoscalingMetricsCollectionEl {
    #[doc = "The frequency at which EC2 Auto Scaling sends aggregated data to AWS CloudWatch. The only valid value is \"1Minute\"."]
    pub granularity: PrimField<String>,
}
impl BuildContainerAwsNodePoolConfigElAutoscalingMetricsCollectionEl {
    pub fn build(self) -> ContainerAwsNodePoolConfigElAutoscalingMetricsCollectionEl {
        ContainerAwsNodePoolConfigElAutoscalingMetricsCollectionEl {
            granularity: self.granularity,
            metrics: core::default::Default::default(),
        }
    }
}
pub struct ContainerAwsNodePoolConfigElAutoscalingMetricsCollectionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAwsNodePoolConfigElAutoscalingMetricsCollectionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerAwsNodePoolConfigElAutoscalingMetricsCollectionElRef {
        ContainerAwsNodePoolConfigElAutoscalingMetricsCollectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAwsNodePoolConfigElAutoscalingMetricsCollectionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `granularity` after provisioning.\nThe frequency at which EC2 Auto Scaling sends aggregated data to AWS CloudWatch. The only valid value is \"1Minute\"."]
    pub fn granularity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.granularity", self.base))
    }
    #[doc = "Get a reference to the value of field `metrics` after provisioning.\nThe metrics to enable. For a list of valid metrics, see https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_EnableMetricsCollection.html. If you specify granularity and don't specify any metrics, all metrics are enabled."]
    pub fn metrics(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.metrics", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerAwsNodePoolConfigElConfigEncryptionEl {
    kms_key_arn: PrimField<String>,
}
impl ContainerAwsNodePoolConfigElConfigEncryptionEl {}
impl ToListMappable for ContainerAwsNodePoolConfigElConfigEncryptionEl {
    type O = BlockAssignable<ContainerAwsNodePoolConfigElConfigEncryptionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAwsNodePoolConfigElConfigEncryptionEl {
    #[doc = "The ARN of the AWS KMS key used to encrypt node pool configuration."]
    pub kms_key_arn: PrimField<String>,
}
impl BuildContainerAwsNodePoolConfigElConfigEncryptionEl {
    pub fn build(self) -> ContainerAwsNodePoolConfigElConfigEncryptionEl {
        ContainerAwsNodePoolConfigElConfigEncryptionEl {
            kms_key_arn: self.kms_key_arn,
        }
    }
}
pub struct ContainerAwsNodePoolConfigElConfigEncryptionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAwsNodePoolConfigElConfigEncryptionElRef {
    fn new(shared: StackShared, base: String) -> ContainerAwsNodePoolConfigElConfigEncryptionElRef {
        ContainerAwsNodePoolConfigElConfigEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAwsNodePoolConfigElConfigEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\nThe ARN of the AWS KMS key used to encrypt node pool configuration."]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerAwsNodePoolConfigElProxyConfigEl {
    secret_arn: PrimField<String>,
    secret_version: PrimField<String>,
}
impl ContainerAwsNodePoolConfigElProxyConfigEl {}
impl ToListMappable for ContainerAwsNodePoolConfigElProxyConfigEl {
    type O = BlockAssignable<ContainerAwsNodePoolConfigElProxyConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAwsNodePoolConfigElProxyConfigEl {
    #[doc = "The ARN of the AWS Secret Manager secret that contains the HTTP(S) proxy configuration."]
    pub secret_arn: PrimField<String>,
    #[doc = "The version string of the AWS Secret Manager secret that contains the HTTP(S) proxy configuration."]
    pub secret_version: PrimField<String>,
}
impl BuildContainerAwsNodePoolConfigElProxyConfigEl {
    pub fn build(self) -> ContainerAwsNodePoolConfigElProxyConfigEl {
        ContainerAwsNodePoolConfigElProxyConfigEl {
            secret_arn: self.secret_arn,
            secret_version: self.secret_version,
        }
    }
}
pub struct ContainerAwsNodePoolConfigElProxyConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAwsNodePoolConfigElProxyConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerAwsNodePoolConfigElProxyConfigElRef {
        ContainerAwsNodePoolConfigElProxyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAwsNodePoolConfigElProxyConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\nThe ARN of the AWS Secret Manager secret that contains the HTTP(S) proxy configuration."]
    pub fn secret_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_version` after provisioning.\nThe version string of the AWS Secret Manager secret that contains the HTTP(S) proxy configuration."]
    pub fn secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_version", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerAwsNodePoolConfigElRootVolumeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size_gib: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throughput: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_type: Option<PrimField<String>>,
}
impl ContainerAwsNodePoolConfigElRootVolumeEl {
    #[doc = "Set the field `iops`.\nOptional. The number of I/O operations per second (IOPS) to provision for GP3 volume."]
    pub fn set_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.iops = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_arn`.\nOptional. The Amazon Resource Name (ARN) of the Customer Managed Key (CMK) used to encrypt AWS EBS volumes. If not specified, the default Amazon managed key associated to the AWS region where this cluster runs will be used."]
    pub fn set_kms_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `size_gib`.\nOptional. The size of the volume, in GiBs. When unspecified, a default value is provided. See the specific reference in the parent resource."]
    pub fn set_size_gib(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size_gib = Some(v.into());
        self
    }
    #[doc = "Set the field `throughput`.\nOptional. The throughput to provision for the volume, in MiB/s. Only valid if the volume type is GP3. If volume type is gp3 and throughput is not specified, the throughput will defaults to 125."]
    pub fn set_throughput(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.throughput = Some(v.into());
        self
    }
    #[doc = "Set the field `volume_type`.\nOptional. Type of the EBS volume. When unspecified, it defaults to GP2 volume. Possible values: VOLUME_TYPE_UNSPECIFIED, GP2, GP3"]
    pub fn set_volume_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.volume_type = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerAwsNodePoolConfigElRootVolumeEl {
    type O = BlockAssignable<ContainerAwsNodePoolConfigElRootVolumeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAwsNodePoolConfigElRootVolumeEl {}
impl BuildContainerAwsNodePoolConfigElRootVolumeEl {
    pub fn build(self) -> ContainerAwsNodePoolConfigElRootVolumeEl {
        ContainerAwsNodePoolConfigElRootVolumeEl {
            iops: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
            size_gib: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}
pub struct ContainerAwsNodePoolConfigElRootVolumeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAwsNodePoolConfigElRootVolumeElRef {
    fn new(shared: StackShared, base: String) -> ContainerAwsNodePoolConfigElRootVolumeElRef {
        ContainerAwsNodePoolConfigElRootVolumeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAwsNodePoolConfigElRootVolumeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `iops` after provisioning.\nOptional. The number of I/O operations per second (IOPS) to provision for GP3 volume."]
    pub fn iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }
    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\nOptional. The Amazon Resource Name (ARN) of the Customer Managed Key (CMK) used to encrypt AWS EBS volumes. If not specified, the default Amazon managed key associated to the AWS region where this cluster runs will be used."]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `size_gib` after provisioning.\nOptional. The size of the volume, in GiBs. When unspecified, a default value is provided. See the specific reference in the parent resource."]
    pub fn size_gib(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_gib", self.base))
    }
    #[doc = "Get a reference to the value of field `throughput` after provisioning.\nOptional. The throughput to provision for the volume, in MiB/s. Only valid if the volume type is GP3. If volume type is gp3 and throughput is not specified, the throughput will defaults to 125."]
    pub fn throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.throughput", self.base))
    }
    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\nOptional. Type of the EBS volume. When unspecified, it defaults to GP2 volume. Possible values: VOLUME_TYPE_UNSPECIFIED, GP2, GP3"]
    pub fn volume_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerAwsNodePoolConfigElSshConfigEl {
    ec2_key_pair: PrimField<String>,
}
impl ContainerAwsNodePoolConfigElSshConfigEl {}
impl ToListMappable for ContainerAwsNodePoolConfigElSshConfigEl {
    type O = BlockAssignable<ContainerAwsNodePoolConfigElSshConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAwsNodePoolConfigElSshConfigEl {
    #[doc = "The name of the EC2 key pair used to login into cluster machines."]
    pub ec2_key_pair: PrimField<String>,
}
impl BuildContainerAwsNodePoolConfigElSshConfigEl {
    pub fn build(self) -> ContainerAwsNodePoolConfigElSshConfigEl {
        ContainerAwsNodePoolConfigElSshConfigEl {
            ec2_key_pair: self.ec2_key_pair,
        }
    }
}
pub struct ContainerAwsNodePoolConfigElSshConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAwsNodePoolConfigElSshConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerAwsNodePoolConfigElSshConfigElRef {
        ContainerAwsNodePoolConfigElSshConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAwsNodePoolConfigElSshConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ec2_key_pair` after provisioning.\nThe name of the EC2 key pair used to login into cluster machines."]
    pub fn ec2_key_pair(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ec2_key_pair", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerAwsNodePoolConfigElTaintsEl {
    effect: PrimField<String>,
    key: PrimField<String>,
    value: PrimField<String>,
}
impl ContainerAwsNodePoolConfigElTaintsEl {}
impl ToListMappable for ContainerAwsNodePoolConfigElTaintsEl {
    type O = BlockAssignable<ContainerAwsNodePoolConfigElTaintsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAwsNodePoolConfigElTaintsEl {
    #[doc = "The taint effect. Possible values: EFFECT_UNSPECIFIED, NO_SCHEDULE, PREFER_NO_SCHEDULE, NO_EXECUTE"]
    pub effect: PrimField<String>,
    #[doc = "Key for the taint."]
    pub key: PrimField<String>,
    #[doc = "Value for the taint."]
    pub value: PrimField<String>,
}
impl BuildContainerAwsNodePoolConfigElTaintsEl {
    pub fn build(self) -> ContainerAwsNodePoolConfigElTaintsEl {
        ContainerAwsNodePoolConfigElTaintsEl {
            effect: self.effect,
            key: self.key,
            value: self.value,
        }
    }
}
pub struct ContainerAwsNodePoolConfigElTaintsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAwsNodePoolConfigElTaintsElRef {
    fn new(shared: StackShared, base: String) -> ContainerAwsNodePoolConfigElTaintsElRef {
        ContainerAwsNodePoolConfigElTaintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAwsNodePoolConfigElTaintsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `effect` after provisioning.\nThe taint effect. Possible values: EFFECT_UNSPECIFIED, NO_SCHEDULE, PREFER_NO_SCHEDULE, NO_EXECUTE"]
    pub fn effect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nKey for the taint."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue for the taint."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct ContainerAwsNodePoolConfigElDynamic {
    autoscaling_metrics_collection:
        Option<DynamicBlock<ContainerAwsNodePoolConfigElAutoscalingMetricsCollectionEl>>,
    config_encryption: Option<DynamicBlock<ContainerAwsNodePoolConfigElConfigEncryptionEl>>,
    proxy_config: Option<DynamicBlock<ContainerAwsNodePoolConfigElProxyConfigEl>>,
    root_volume: Option<DynamicBlock<ContainerAwsNodePoolConfigElRootVolumeEl>>,
    ssh_config: Option<DynamicBlock<ContainerAwsNodePoolConfigElSshConfigEl>>,
    taints: Option<DynamicBlock<ContainerAwsNodePoolConfigElTaintsEl>>,
}
#[derive(Serialize)]
pub struct ContainerAwsNodePoolConfigEl {
    iam_instance_profile: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling_metrics_collection:
        Option<Vec<ContainerAwsNodePoolConfigElAutoscalingMetricsCollectionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config_encryption: Option<Vec<ContainerAwsNodePoolConfigElConfigEncryptionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_config: Option<Vec<ContainerAwsNodePoolConfigElProxyConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_volume: Option<Vec<ContainerAwsNodePoolConfigElRootVolumeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssh_config: Option<Vec<ContainerAwsNodePoolConfigElSshConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    taints: Option<Vec<ContainerAwsNodePoolConfigElTaintsEl>>,
    dynamic: ContainerAwsNodePoolConfigElDynamic,
}
impl ContainerAwsNodePoolConfigEl {
    #[doc = "Set the field `instance_type`.\nOptional. The AWS instance type. When unspecified, it defaults to `m5.large`."]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nOptional. The initial labels assigned to nodes of this node pool. An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `security_group_ids`.\nOptional. The IDs of additional security groups to add to nodes in this pool. The manager will automatically create security groups with minimum rules needed for a functioning cluster."]
    pub fn set_security_group_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.security_group_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\nOptional. Key/value metadata to assign to each underlying AWS resource. Specify at most 50 pairs containing alphanumerics, spaces, and symbols (.+-=_:@/). Keys can be up to 127 Unicode characters. Values can be up to 255 Unicode characters."]
    pub fn set_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
    #[doc = "Set the field `autoscaling_metrics_collection`.\n"]
    pub fn set_autoscaling_metrics_collection(
        mut self,
        v: impl Into<BlockAssignable<ContainerAwsNodePoolConfigElAutoscalingMetricsCollectionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.autoscaling_metrics_collection = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling_metrics_collection = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `config_encryption`.\n"]
    pub fn set_config_encryption(
        mut self,
        v: impl Into<BlockAssignable<ContainerAwsNodePoolConfigElConfigEncryptionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.config_encryption = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.config_encryption = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `proxy_config`.\n"]
    pub fn set_proxy_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerAwsNodePoolConfigElProxyConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.proxy_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.proxy_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `root_volume`.\n"]
    pub fn set_root_volume(
        mut self,
        v: impl Into<BlockAssignable<ContainerAwsNodePoolConfigElRootVolumeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.root_volume = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.root_volume = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ssh_config`.\n"]
    pub fn set_ssh_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerAwsNodePoolConfigElSshConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ssh_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ssh_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `taints`.\n"]
    pub fn set_taints(
        mut self,
        v: impl Into<BlockAssignable<ContainerAwsNodePoolConfigElTaintsEl>>,
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
impl ToListMappable for ContainerAwsNodePoolConfigEl {
    type O = BlockAssignable<ContainerAwsNodePoolConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAwsNodePoolConfigEl {
    #[doc = "The name of the AWS IAM role assigned to nodes in the pool."]
    pub iam_instance_profile: PrimField<String>,
}
impl BuildContainerAwsNodePoolConfigEl {
    pub fn build(self) -> ContainerAwsNodePoolConfigEl {
        ContainerAwsNodePoolConfigEl {
            iam_instance_profile: self.iam_instance_profile,
            instance_type: core::default::Default::default(),
            labels: core::default::Default::default(),
            security_group_ids: core::default::Default::default(),
            tags: core::default::Default::default(),
            autoscaling_metrics_collection: core::default::Default::default(),
            config_encryption: core::default::Default::default(),
            proxy_config: core::default::Default::default(),
            root_volume: core::default::Default::default(),
            ssh_config: core::default::Default::default(),
            taints: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContainerAwsNodePoolConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAwsNodePoolConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerAwsNodePoolConfigElRef {
        ContainerAwsNodePoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAwsNodePoolConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `iam_instance_profile` after provisioning.\nThe name of the AWS IAM role assigned to nodes in the pool."]
    pub fn iam_instance_profile(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_instance_profile", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\nOptional. The AWS instance type. When unspecified, it defaults to `m5.large`."]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. The initial labels assigned to nodes of this node pool. An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\nOptional. The IDs of additional security groups to add to nodes in this pool. The manager will automatically create security groups with minimum rules needed for a functioning cluster."]
    pub fn security_group_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nOptional. Key/value metadata to assign to each underlying AWS resource. Specify at most 50 pairs containing alphanumerics, spaces, and symbols (.+-=_:@/). Keys can be up to 127 Unicode characters. Values can be up to 255 Unicode characters."]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
    #[doc = "Get a reference to the value of field `autoscaling_metrics_collection` after provisioning.\n"]
    pub fn autoscaling_metrics_collection(
        &self,
    ) -> ListRef<ContainerAwsNodePoolConfigElAutoscalingMetricsCollectionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_metrics_collection", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `config_encryption` after provisioning.\n"]
    pub fn config_encryption(&self) -> ListRef<ContainerAwsNodePoolConfigElConfigEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.config_encryption", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `proxy_config` after provisioning.\n"]
    pub fn proxy_config(&self) -> ListRef<ContainerAwsNodePoolConfigElProxyConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.proxy_config", self.base))
    }
    #[doc = "Get a reference to the value of field `root_volume` after provisioning.\n"]
    pub fn root_volume(&self) -> ListRef<ContainerAwsNodePoolConfigElRootVolumeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.root_volume", self.base))
    }
    #[doc = "Get a reference to the value of field `ssh_config` after provisioning.\n"]
    pub fn ssh_config(&self) -> ListRef<ContainerAwsNodePoolConfigElSshConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ssh_config", self.base))
    }
    #[doc = "Get a reference to the value of field `taints` after provisioning.\n"]
    pub fn taints(&self) -> ListRef<ContainerAwsNodePoolConfigElTaintsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.taints", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerAwsNodePoolKubeletConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_cfs_quota: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_cfs_quota_period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_manager_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_pids_limit: Option<PrimField<f64>>,
}
impl ContainerAwsNodePoolKubeletConfigEl {
    #[doc = "Set the field `cpu_cfs_quota`.\nWhether or not to enable CPU CFS quota. Defaults to true."]
    pub fn set_cpu_cfs_quota(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.cpu_cfs_quota = Some(v.into());
        self
    }
    #[doc = "Set the field `cpu_cfs_quota_period`.\nOptional. The CPU CFS quota period to use for the node. Defaults to \"100ms\"."]
    pub fn set_cpu_cfs_quota_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cpu_cfs_quota_period = Some(v.into());
        self
    }
    #[doc = "Set the field `cpu_manager_policy`.\nThe CpuManagerPolicy to use for the node. Defaults to \"none\"."]
    pub fn set_cpu_manager_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cpu_manager_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `pod_pids_limit`.\nOptional. The maximum number of PIDs in each pod running on the node. The limit scales automatically based on underlying machine size if left unset."]
    pub fn set_pod_pids_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.pod_pids_limit = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerAwsNodePoolKubeletConfigEl {
    type O = BlockAssignable<ContainerAwsNodePoolKubeletConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAwsNodePoolKubeletConfigEl {}
impl BuildContainerAwsNodePoolKubeletConfigEl {
    pub fn build(self) -> ContainerAwsNodePoolKubeletConfigEl {
        ContainerAwsNodePoolKubeletConfigEl {
            cpu_cfs_quota: core::default::Default::default(),
            cpu_cfs_quota_period: core::default::Default::default(),
            cpu_manager_policy: core::default::Default::default(),
            pod_pids_limit: core::default::Default::default(),
        }
    }
}
pub struct ContainerAwsNodePoolKubeletConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAwsNodePoolKubeletConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerAwsNodePoolKubeletConfigElRef {
        ContainerAwsNodePoolKubeletConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAwsNodePoolKubeletConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cpu_cfs_quota` after provisioning.\nWhether or not to enable CPU CFS quota. Defaults to true."]
    pub fn cpu_cfs_quota(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_cfs_quota", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cpu_cfs_quota_period` after provisioning.\nOptional. The CPU CFS quota period to use for the node. Defaults to \"100ms\"."]
    pub fn cpu_cfs_quota_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_cfs_quota_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cpu_manager_policy` after provisioning.\nThe CpuManagerPolicy to use for the node. Defaults to \"none\"."]
    pub fn cpu_manager_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_manager_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pod_pids_limit` after provisioning.\nOptional. The maximum number of PIDs in each pod running on the node. The limit scales automatically based on underlying machine size if left unset."]
    pub fn pod_pids_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pod_pids_limit", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerAwsNodePoolManagementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_repair: Option<PrimField<bool>>,
}
impl ContainerAwsNodePoolManagementEl {
    #[doc = "Set the field `auto_repair`.\nOptional. Whether or not the nodes will be automatically repaired."]
    pub fn set_auto_repair(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_repair = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerAwsNodePoolManagementEl {
    type O = BlockAssignable<ContainerAwsNodePoolManagementEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAwsNodePoolManagementEl {}
impl BuildContainerAwsNodePoolManagementEl {
    pub fn build(self) -> ContainerAwsNodePoolManagementEl {
        ContainerAwsNodePoolManagementEl {
            auto_repair: core::default::Default::default(),
        }
    }
}
pub struct ContainerAwsNodePoolManagementElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAwsNodePoolManagementElRef {
    fn new(shared: StackShared, base: String) -> ContainerAwsNodePoolManagementElRef {
        ContainerAwsNodePoolManagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAwsNodePoolManagementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto_repair` after provisioning.\nOptional. Whether or not the nodes will be automatically repaired."]
    pub fn auto_repair(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_repair", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerAwsNodePoolMaxPodsConstraintEl {
    max_pods_per_node: PrimField<f64>,
}
impl ContainerAwsNodePoolMaxPodsConstraintEl {}
impl ToListMappable for ContainerAwsNodePoolMaxPodsConstraintEl {
    type O = BlockAssignable<ContainerAwsNodePoolMaxPodsConstraintEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAwsNodePoolMaxPodsConstraintEl {
    #[doc = "The maximum number of pods to schedule on a single node."]
    pub max_pods_per_node: PrimField<f64>,
}
impl BuildContainerAwsNodePoolMaxPodsConstraintEl {
    pub fn build(self) -> ContainerAwsNodePoolMaxPodsConstraintEl {
        ContainerAwsNodePoolMaxPodsConstraintEl {
            max_pods_per_node: self.max_pods_per_node,
        }
    }
}
pub struct ContainerAwsNodePoolMaxPodsConstraintElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAwsNodePoolMaxPodsConstraintElRef {
    fn new(shared: StackShared, base: String) -> ContainerAwsNodePoolMaxPodsConstraintElRef {
        ContainerAwsNodePoolMaxPodsConstraintElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAwsNodePoolMaxPodsConstraintElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_pods_per_node` after provisioning.\nThe maximum number of pods to schedule on a single node."]
    pub fn max_pods_per_node(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_pods_per_node", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerAwsNodePoolTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ContainerAwsNodePoolTimeoutsEl {
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
impl ToListMappable for ContainerAwsNodePoolTimeoutsEl {
    type O = BlockAssignable<ContainerAwsNodePoolTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAwsNodePoolTimeoutsEl {}
impl BuildContainerAwsNodePoolTimeoutsEl {
    pub fn build(self) -> ContainerAwsNodePoolTimeoutsEl {
        ContainerAwsNodePoolTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ContainerAwsNodePoolTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAwsNodePoolTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ContainerAwsNodePoolTimeoutsElRef {
        ContainerAwsNodePoolTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAwsNodePoolTimeoutsElRef {
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
pub struct ContainerAwsNodePoolUpdateSettingsElSurgeSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_surge: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_unavailable: Option<PrimField<f64>>,
}
impl ContainerAwsNodePoolUpdateSettingsElSurgeSettingsEl {
    #[doc = "Set the field `max_surge`.\nOptional. The maximum number of nodes that can be created beyond the current size of the node pool during the update process."]
    pub fn set_max_surge(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_surge = Some(v.into());
        self
    }
    #[doc = "Set the field `max_unavailable`.\nOptional. The maximum number of nodes that can be simultaneously unavailable during the update process. A node is considered unavailable if its status is not Ready."]
    pub fn set_max_unavailable(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_unavailable = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerAwsNodePoolUpdateSettingsElSurgeSettingsEl {
    type O = BlockAssignable<ContainerAwsNodePoolUpdateSettingsElSurgeSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAwsNodePoolUpdateSettingsElSurgeSettingsEl {}
impl BuildContainerAwsNodePoolUpdateSettingsElSurgeSettingsEl {
    pub fn build(self) -> ContainerAwsNodePoolUpdateSettingsElSurgeSettingsEl {
        ContainerAwsNodePoolUpdateSettingsElSurgeSettingsEl {
            max_surge: core::default::Default::default(),
            max_unavailable: core::default::Default::default(),
        }
    }
}
pub struct ContainerAwsNodePoolUpdateSettingsElSurgeSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAwsNodePoolUpdateSettingsElSurgeSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerAwsNodePoolUpdateSettingsElSurgeSettingsElRef {
        ContainerAwsNodePoolUpdateSettingsElSurgeSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAwsNodePoolUpdateSettingsElSurgeSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_surge` after provisioning.\nOptional. The maximum number of nodes that can be created beyond the current size of the node pool during the update process."]
    pub fn max_surge(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_surge", self.base))
    }
    #[doc = "Get a reference to the value of field `max_unavailable` after provisioning.\nOptional. The maximum number of nodes that can be simultaneously unavailable during the update process. A node is considered unavailable if its status is not Ready."]
    pub fn max_unavailable(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_unavailable", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ContainerAwsNodePoolUpdateSettingsElDynamic {
    surge_settings: Option<DynamicBlock<ContainerAwsNodePoolUpdateSettingsElSurgeSettingsEl>>,
}
#[derive(Serialize)]
pub struct ContainerAwsNodePoolUpdateSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    surge_settings: Option<Vec<ContainerAwsNodePoolUpdateSettingsElSurgeSettingsEl>>,
    dynamic: ContainerAwsNodePoolUpdateSettingsElDynamic,
}
impl ContainerAwsNodePoolUpdateSettingsEl {
    #[doc = "Set the field `surge_settings`.\n"]
    pub fn set_surge_settings(
        mut self,
        v: impl Into<BlockAssignable<ContainerAwsNodePoolUpdateSettingsElSurgeSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.surge_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.surge_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ContainerAwsNodePoolUpdateSettingsEl {
    type O = BlockAssignable<ContainerAwsNodePoolUpdateSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAwsNodePoolUpdateSettingsEl {}
impl BuildContainerAwsNodePoolUpdateSettingsEl {
    pub fn build(self) -> ContainerAwsNodePoolUpdateSettingsEl {
        ContainerAwsNodePoolUpdateSettingsEl {
            surge_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContainerAwsNodePoolUpdateSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAwsNodePoolUpdateSettingsElRef {
    fn new(shared: StackShared, base: String) -> ContainerAwsNodePoolUpdateSettingsElRef {
        ContainerAwsNodePoolUpdateSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAwsNodePoolUpdateSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `surge_settings` after provisioning.\n"]
    pub fn surge_settings(
        &self,
    ) -> ListRef<ContainerAwsNodePoolUpdateSettingsElSurgeSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.surge_settings", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ContainerAwsNodePoolDynamic {
    autoscaling: Option<DynamicBlock<ContainerAwsNodePoolAutoscalingEl>>,
    config: Option<DynamicBlock<ContainerAwsNodePoolConfigEl>>,
    kubelet_config: Option<DynamicBlock<ContainerAwsNodePoolKubeletConfigEl>>,
    management: Option<DynamicBlock<ContainerAwsNodePoolManagementEl>>,
    max_pods_constraint: Option<DynamicBlock<ContainerAwsNodePoolMaxPodsConstraintEl>>,
    update_settings: Option<DynamicBlock<ContainerAwsNodePoolUpdateSettingsEl>>,
}
