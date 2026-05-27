use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ContainerAzureNodePoolData {
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
    azure_availability_zone: Option<PrimField<String>>,
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
    autoscaling: Option<Vec<ContainerAzureNodePoolAutoscalingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Vec<ContainerAzureNodePoolConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    management: Option<Vec<ContainerAzureNodePoolManagementEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_pods_constraint: Option<Vec<ContainerAzureNodePoolMaxPodsConstraintEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ContainerAzureNodePoolTimeoutsEl>,
    dynamic: ContainerAzureNodePoolDynamic,
}
struct ContainerAzureNodePool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ContainerAzureNodePoolData>,
}
#[derive(Clone)]
pub struct ContainerAzureNodePool(Rc<ContainerAzureNodePool_>);
impl ContainerAzureNodePool {
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
    #[doc = "Set the field `annotations`.\nOptional. Annotations on the node pool. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Keys can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `azure_availability_zone`.\nOptional. The Azure availability zone of the nodes in this nodepool. When unspecified, it defaults to `1`."]
    pub fn set_azure_availability_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().azure_availability_zone = Some(v.into());
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
        v: impl Into<BlockAssignable<ContainerAzureNodePoolAutoscalingEl>>,
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
    pub fn set_config(self, v: impl Into<BlockAssignable<ContainerAzureNodePoolConfigEl>>) -> Self {
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
    #[doc = "Set the field `management`.\n"]
    pub fn set_management(
        self,
        v: impl Into<BlockAssignable<ContainerAzureNodePoolManagementEl>>,
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
        v: impl Into<BlockAssignable<ContainerAzureNodePoolMaxPodsConstraintEl>>,
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
    pub fn set_timeouts(self, v: impl Into<ContainerAzureNodePoolTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nOptional. Annotations on the node pool. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Keys can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `azure_availability_zone` after provisioning.\nOptional. The Azure availability zone of the nodes in this nodepool. When unspecified, it defaults to `1`."]
    pub fn azure_availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.azure_availability_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\nThe azureCluster for the resource"]
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
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. If set, there are currently pending changes to the node pool."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The current state of the node pool. Possible values: STATE_UNSPECIFIED, PROVISIONING, RUNNING, RECONCILING, STOPPING, ERROR, DEGRADED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\nThe ARM ID of the subnet where the node pool VMs run. Make sure it's a subnet under the virtual network in the cluster configuration."]
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
    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe Kubernetes version (e.g. `1.19.10-gke.1000`) running on this node pool."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `autoscaling` after provisioning.\n"]
    pub fn autoscaling(&self) -> ListRef<ContainerAzureNodePoolAutoscalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<ContainerAzureNodePoolConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> ListRef<ContainerAzureNodePoolManagementElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.management", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_pods_constraint` after provisioning.\n"]
    pub fn max_pods_constraint(&self) -> ListRef<ContainerAzureNodePoolMaxPodsConstraintElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.max_pods_constraint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerAzureNodePoolTimeoutsElRef {
        ContainerAzureNodePoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ContainerAzureNodePool {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ContainerAzureNodePool {}
impl ToListMappable for ContainerAzureNodePool {
    type O = ListRef<ContainerAzureNodePoolRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ContainerAzureNodePool_ {
    fn extract_resource_type(&self) -> String {
        "google_container_azure_node_pool".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildContainerAzureNodePool {
    pub tf_id: String,
    #[doc = "The azureCluster for the resource"]
    pub cluster: PrimField<String>,
    #[doc = "The location for the resource"]
    pub location: PrimField<String>,
    #[doc = "The name of this resource."]
    pub name: PrimField<String>,
    #[doc = "The ARM ID of the subnet where the node pool VMs run. Make sure it's a subnet under the virtual network in the cluster configuration."]
    pub subnet_id: PrimField<String>,
    #[doc = "The Kubernetes version (e.g. `1.19.10-gke.1000`) running on this node pool."]
    pub version: PrimField<String>,
}
impl BuildContainerAzureNodePool {
    pub fn build(self, stack: &mut Stack) -> ContainerAzureNodePool {
        let out = ContainerAzureNodePool(Rc::new(ContainerAzureNodePool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ContainerAzureNodePoolData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                azure_availability_zone: core::default::Default::default(),
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
                management: core::default::Default::default(),
                max_pods_constraint: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ContainerAzureNodePoolRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAzureNodePoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ContainerAzureNodePoolRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nOptional. Annotations on the node pool. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Keys can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `azure_availability_zone` after provisioning.\nOptional. The Azure availability zone of the nodes in this nodepool. When unspecified, it defaults to `1`."]
    pub fn azure_availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.azure_availability_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\nThe azureCluster for the resource"]
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
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. If set, there are currently pending changes to the node pool."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The current state of the node pool. Possible values: STATE_UNSPECIFIED, PROVISIONING, RUNNING, RECONCILING, STOPPING, ERROR, DEGRADED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\nThe ARM ID of the subnet where the node pool VMs run. Make sure it's a subnet under the virtual network in the cluster configuration."]
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
    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe Kubernetes version (e.g. `1.19.10-gke.1000`) running on this node pool."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `autoscaling` after provisioning.\n"]
    pub fn autoscaling(&self) -> ListRef<ContainerAzureNodePoolAutoscalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<ContainerAzureNodePoolConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> ListRef<ContainerAzureNodePoolManagementElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.management", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_pods_constraint` after provisioning.\n"]
    pub fn max_pods_constraint(&self) -> ListRef<ContainerAzureNodePoolMaxPodsConstraintElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.max_pods_constraint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerAzureNodePoolTimeoutsElRef {
        ContainerAzureNodePoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerAzureNodePoolAutoscalingEl {
    max_node_count: PrimField<f64>,
    min_node_count: PrimField<f64>,
}
impl ContainerAzureNodePoolAutoscalingEl {}
impl ToListMappable for ContainerAzureNodePoolAutoscalingEl {
    type O = BlockAssignable<ContainerAzureNodePoolAutoscalingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAzureNodePoolAutoscalingEl {
    #[doc = "Maximum number of nodes in the node pool. Must be >= min_node_count."]
    pub max_node_count: PrimField<f64>,
    #[doc = "Minimum number of nodes in the node pool. Must be >= 1 and <= max_node_count."]
    pub min_node_count: PrimField<f64>,
}
impl BuildContainerAzureNodePoolAutoscalingEl {
    pub fn build(self) -> ContainerAzureNodePoolAutoscalingEl {
        ContainerAzureNodePoolAutoscalingEl {
            max_node_count: self.max_node_count,
            min_node_count: self.min_node_count,
        }
    }
}
pub struct ContainerAzureNodePoolAutoscalingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAzureNodePoolAutoscalingElRef {
    fn new(shared: StackShared, base: String) -> ContainerAzureNodePoolAutoscalingElRef {
        ContainerAzureNodePoolAutoscalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAzureNodePoolAutoscalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_node_count` after provisioning.\nMaximum number of nodes in the node pool. Must be >= min_node_count."]
    pub fn max_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_node_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_node_count` after provisioning.\nMinimum number of nodes in the node pool. Must be >= 1 and <= max_node_count."]
    pub fn min_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_node_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerAzureNodePoolConfigElProxyConfigEl {
    resource_group_id: PrimField<String>,
    secret_id: PrimField<String>,
}
impl ContainerAzureNodePoolConfigElProxyConfigEl {}
impl ToListMappable for ContainerAzureNodePoolConfigElProxyConfigEl {
    type O = BlockAssignable<ContainerAzureNodePoolConfigElProxyConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAzureNodePoolConfigElProxyConfigEl {
    #[doc = "The ARM ID the of the resource group containing proxy keyvault. Resource group ids are formatted as `/subscriptions/<subscription-id>/resourceGroups/<resource-group-name>`"]
    pub resource_group_id: PrimField<String>,
    #[doc = "The URL the of the proxy setting secret with its version. Secret ids are formatted as `https:<key-vault-name>.vault.azure.net/secrets/<secret-name>/<secret-version>`."]
    pub secret_id: PrimField<String>,
}
impl BuildContainerAzureNodePoolConfigElProxyConfigEl {
    pub fn build(self) -> ContainerAzureNodePoolConfigElProxyConfigEl {
        ContainerAzureNodePoolConfigElProxyConfigEl {
            resource_group_id: self.resource_group_id,
            secret_id: self.secret_id,
        }
    }
}
pub struct ContainerAzureNodePoolConfigElProxyConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAzureNodePoolConfigElProxyConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerAzureNodePoolConfigElProxyConfigElRef {
        ContainerAzureNodePoolConfigElProxyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAzureNodePoolConfigElProxyConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_group_id` after provisioning.\nThe ARM ID the of the resource group containing proxy keyvault. Resource group ids are formatted as `/subscriptions/<subscription-id>/resourceGroups/<resource-group-name>`"]
    pub fn resource_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_group_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secret_id` after provisioning.\nThe URL the of the proxy setting secret with its version. Secret ids are formatted as `https:<key-vault-name>.vault.azure.net/secrets/<secret-name>/<secret-version>`."]
    pub fn secret_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerAzureNodePoolConfigElRootVolumeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    size_gib: Option<PrimField<f64>>,
}
impl ContainerAzureNodePoolConfigElRootVolumeEl {
    #[doc = "Set the field `size_gib`.\nOptional. The size of the disk, in GiBs. When unspecified, a default value is provided. See the specific reference in the parent resource."]
    pub fn set_size_gib(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size_gib = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerAzureNodePoolConfigElRootVolumeEl {
    type O = BlockAssignable<ContainerAzureNodePoolConfigElRootVolumeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAzureNodePoolConfigElRootVolumeEl {}
impl BuildContainerAzureNodePoolConfigElRootVolumeEl {
    pub fn build(self) -> ContainerAzureNodePoolConfigElRootVolumeEl {
        ContainerAzureNodePoolConfigElRootVolumeEl {
            size_gib: core::default::Default::default(),
        }
    }
}
pub struct ContainerAzureNodePoolConfigElRootVolumeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAzureNodePoolConfigElRootVolumeElRef {
    fn new(shared: StackShared, base: String) -> ContainerAzureNodePoolConfigElRootVolumeElRef {
        ContainerAzureNodePoolConfigElRootVolumeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAzureNodePoolConfigElRootVolumeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `size_gib` after provisioning.\nOptional. The size of the disk, in GiBs. When unspecified, a default value is provided. See the specific reference in the parent resource."]
    pub fn size_gib(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_gib", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerAzureNodePoolConfigElSshConfigEl {
    authorized_key: PrimField<String>,
}
impl ContainerAzureNodePoolConfigElSshConfigEl {}
impl ToListMappable for ContainerAzureNodePoolConfigElSshConfigEl {
    type O = BlockAssignable<ContainerAzureNodePoolConfigElSshConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAzureNodePoolConfigElSshConfigEl {
    #[doc = "The SSH public key data for VMs managed by Anthos. This accepts the authorized_keys file format used in OpenSSH according to the sshd(8) manual page."]
    pub authorized_key: PrimField<String>,
}
impl BuildContainerAzureNodePoolConfigElSshConfigEl {
    pub fn build(self) -> ContainerAzureNodePoolConfigElSshConfigEl {
        ContainerAzureNodePoolConfigElSshConfigEl {
            authorized_key: self.authorized_key,
        }
    }
}
pub struct ContainerAzureNodePoolConfigElSshConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAzureNodePoolConfigElSshConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerAzureNodePoolConfigElSshConfigElRef {
        ContainerAzureNodePoolConfigElSshConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAzureNodePoolConfigElSshConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authorized_key` after provisioning.\nThe SSH public key data for VMs managed by Anthos. This accepts the authorized_keys file format used in OpenSSH according to the sshd(8) manual page."]
    pub fn authorized_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorized_key", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ContainerAzureNodePoolConfigElDynamic {
    proxy_config: Option<DynamicBlock<ContainerAzureNodePoolConfigElProxyConfigEl>>,
    root_volume: Option<DynamicBlock<ContainerAzureNodePoolConfigElRootVolumeEl>>,
    ssh_config: Option<DynamicBlock<ContainerAzureNodePoolConfigElSshConfigEl>>,
}
#[derive(Serialize)]
pub struct ContainerAzureNodePoolConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vm_size: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_config: Option<Vec<ContainerAzureNodePoolConfigElProxyConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_volume: Option<Vec<ContainerAzureNodePoolConfigElRootVolumeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssh_config: Option<Vec<ContainerAzureNodePoolConfigElSshConfigEl>>,
    dynamic: ContainerAzureNodePoolConfigElDynamic,
}
impl ContainerAzureNodePoolConfigEl {
    #[doc = "Set the field `labels`.\nOptional. The initial labels assigned to nodes of this node pool. An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\nOptional. A set of tags to apply to all underlying Azure resources for this node pool. This currently only includes Virtual Machine Scale Sets. Specify at most 50 pairs containing alphanumerics, spaces, and symbols (.+-=_:@/). Keys can be up to 127 Unicode characters. Values can be up to 255 Unicode characters."]
    pub fn set_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
    #[doc = "Set the field `vm_size`.\nOptional. The Azure VM size name. Example: `Standard_DS2_v2`. See (/anthos/clusters/docs/azure/reference/supported-vms) for options. When unspecified, it defaults to `Standard_DS2_v2`."]
    pub fn set_vm_size(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vm_size = Some(v.into());
        self
    }
    #[doc = "Set the field `proxy_config`.\n"]
    pub fn set_proxy_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerAzureNodePoolConfigElProxyConfigEl>>,
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
        v: impl Into<BlockAssignable<ContainerAzureNodePoolConfigElRootVolumeEl>>,
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
        v: impl Into<BlockAssignable<ContainerAzureNodePoolConfigElSshConfigEl>>,
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
}
impl ToListMappable for ContainerAzureNodePoolConfigEl {
    type O = BlockAssignable<ContainerAzureNodePoolConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAzureNodePoolConfigEl {}
impl BuildContainerAzureNodePoolConfigEl {
    pub fn build(self) -> ContainerAzureNodePoolConfigEl {
        ContainerAzureNodePoolConfigEl {
            labels: core::default::Default::default(),
            tags: core::default::Default::default(),
            vm_size: core::default::Default::default(),
            proxy_config: core::default::Default::default(),
            root_volume: core::default::Default::default(),
            ssh_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContainerAzureNodePoolConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAzureNodePoolConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerAzureNodePoolConfigElRef {
        ContainerAzureNodePoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAzureNodePoolConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. The initial labels assigned to nodes of this node pool. An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nOptional. A set of tags to apply to all underlying Azure resources for this node pool. This currently only includes Virtual Machine Scale Sets. Specify at most 50 pairs containing alphanumerics, spaces, and symbols (.+-=_:@/). Keys can be up to 127 Unicode characters. Values can be up to 255 Unicode characters."]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
    #[doc = "Get a reference to the value of field `vm_size` after provisioning.\nOptional. The Azure VM size name. Example: `Standard_DS2_v2`. See (/anthos/clusters/docs/azure/reference/supported-vms) for options. When unspecified, it defaults to `Standard_DS2_v2`."]
    pub fn vm_size(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vm_size", self.base))
    }
    #[doc = "Get a reference to the value of field `proxy_config` after provisioning.\n"]
    pub fn proxy_config(&self) -> ListRef<ContainerAzureNodePoolConfigElProxyConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.proxy_config", self.base))
    }
    #[doc = "Get a reference to the value of field `root_volume` after provisioning.\n"]
    pub fn root_volume(&self) -> ListRef<ContainerAzureNodePoolConfigElRootVolumeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.root_volume", self.base))
    }
    #[doc = "Get a reference to the value of field `ssh_config` after provisioning.\n"]
    pub fn ssh_config(&self) -> ListRef<ContainerAzureNodePoolConfigElSshConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ssh_config", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerAzureNodePoolManagementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_repair: Option<PrimField<bool>>,
}
impl ContainerAzureNodePoolManagementEl {
    #[doc = "Set the field `auto_repair`.\nOptional. Whether or not the nodes will be automatically repaired."]
    pub fn set_auto_repair(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_repair = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerAzureNodePoolManagementEl {
    type O = BlockAssignable<ContainerAzureNodePoolManagementEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAzureNodePoolManagementEl {}
impl BuildContainerAzureNodePoolManagementEl {
    pub fn build(self) -> ContainerAzureNodePoolManagementEl {
        ContainerAzureNodePoolManagementEl {
            auto_repair: core::default::Default::default(),
        }
    }
}
pub struct ContainerAzureNodePoolManagementElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAzureNodePoolManagementElRef {
    fn new(shared: StackShared, base: String) -> ContainerAzureNodePoolManagementElRef {
        ContainerAzureNodePoolManagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAzureNodePoolManagementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto_repair` after provisioning.\nOptional. Whether or not the nodes will be automatically repaired."]
    pub fn auto_repair(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_repair", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerAzureNodePoolMaxPodsConstraintEl {
    max_pods_per_node: PrimField<f64>,
}
impl ContainerAzureNodePoolMaxPodsConstraintEl {}
impl ToListMappable for ContainerAzureNodePoolMaxPodsConstraintEl {
    type O = BlockAssignable<ContainerAzureNodePoolMaxPodsConstraintEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAzureNodePoolMaxPodsConstraintEl {
    #[doc = "The maximum number of pods to schedule on a single node."]
    pub max_pods_per_node: PrimField<f64>,
}
impl BuildContainerAzureNodePoolMaxPodsConstraintEl {
    pub fn build(self) -> ContainerAzureNodePoolMaxPodsConstraintEl {
        ContainerAzureNodePoolMaxPodsConstraintEl {
            max_pods_per_node: self.max_pods_per_node,
        }
    }
}
pub struct ContainerAzureNodePoolMaxPodsConstraintElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAzureNodePoolMaxPodsConstraintElRef {
    fn new(shared: StackShared, base: String) -> ContainerAzureNodePoolMaxPodsConstraintElRef {
        ContainerAzureNodePoolMaxPodsConstraintElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAzureNodePoolMaxPodsConstraintElRef {
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
pub struct ContainerAzureNodePoolTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ContainerAzureNodePoolTimeoutsEl {
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
impl ToListMappable for ContainerAzureNodePoolTimeoutsEl {
    type O = BlockAssignable<ContainerAzureNodePoolTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAzureNodePoolTimeoutsEl {}
impl BuildContainerAzureNodePoolTimeoutsEl {
    pub fn build(self) -> ContainerAzureNodePoolTimeoutsEl {
        ContainerAzureNodePoolTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ContainerAzureNodePoolTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAzureNodePoolTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ContainerAzureNodePoolTimeoutsElRef {
        ContainerAzureNodePoolTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAzureNodePoolTimeoutsElRef {
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
struct ContainerAzureNodePoolDynamic {
    autoscaling: Option<DynamicBlock<ContainerAzureNodePoolAutoscalingEl>>,
    config: Option<DynamicBlock<ContainerAzureNodePoolConfigEl>>,
    management: Option<DynamicBlock<ContainerAzureNodePoolManagementEl>>,
    max_pods_constraint: Option<DynamicBlock<ContainerAzureNodePoolMaxPodsConstraintEl>>,
}
