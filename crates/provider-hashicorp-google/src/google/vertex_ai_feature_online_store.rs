use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct VertexAiFeatureOnlineStoreData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bigtable: Option<Vec<VertexAiFeatureOnlineStoreBigtableEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dedicated_serving_endpoint: Option<Vec<VertexAiFeatureOnlineStoreDedicatedServingEndpointEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_spec: Option<Vec<VertexAiFeatureOnlineStoreEncryptionSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    optimized: Option<Vec<VertexAiFeatureOnlineStoreOptimizedEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VertexAiFeatureOnlineStoreTimeoutsEl>,
    dynamic: VertexAiFeatureOnlineStoreDynamic,
}
struct VertexAiFeatureOnlineStore_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VertexAiFeatureOnlineStoreData>,
}
#[derive(Clone)]
pub struct VertexAiFeatureOnlineStore(Rc<VertexAiFeatureOnlineStore_>);
impl VertexAiFeatureOnlineStore {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `force_destroy`.\nIf set to true, any FeatureViews and Features for this FeatureOnlineStore will also be deleted."]
    pub fn set_force_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nThe labels with user-defined metadata to organize your feature online stores.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nThe region of feature online store. eg us-central1"]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `bigtable`.\n"]
    pub fn set_bigtable(
        self,
        v: impl Into<BlockAssignable<VertexAiFeatureOnlineStoreBigtableEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bigtable = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bigtable = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dedicated_serving_endpoint`.\n"]
    pub fn set_dedicated_serving_endpoint(
        self,
        v: impl Into<BlockAssignable<VertexAiFeatureOnlineStoreDedicatedServingEndpointEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dedicated_serving_endpoint = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dedicated_serving_endpoint = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `encryption_spec`.\n"]
    pub fn set_encryption_spec(
        self,
        v: impl Into<BlockAssignable<VertexAiFeatureOnlineStoreEncryptionSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `optimized`.\n"]
    pub fn set_optimized(
        self,
        v: impl Into<BlockAssignable<VertexAiFeatureOnlineStoreOptimizedEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().optimized = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.optimized = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VertexAiFeatureOnlineStoreTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of when the feature online store was created in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
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
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nUsed to perform consistent read-modify-write updates."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\nIf set to true, any FeatureViews and Features for this FeatureOnlineStore will also be deleted."]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_destroy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels with user-defined metadata to organize your feature online stores.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Feature Online Store. This value may be up to 60 characters, and valid characters are [a-z0-9_]. The first character cannot be a number."]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of feature online store. eg us-central1"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the Feature Online Store. See the possible states in [this link](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.featureOnlineStores#state)."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp of when the feature online store was last updated in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bigtable` after provisioning.\n"]
    pub fn bigtable(&self) -> ListRef<VertexAiFeatureOnlineStoreBigtableElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bigtable", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dedicated_serving_endpoint` after provisioning.\n"]
    pub fn dedicated_serving_endpoint(
        &self,
    ) -> ListRef<VertexAiFeatureOnlineStoreDedicatedServingEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dedicated_serving_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> ListRef<VertexAiFeatureOnlineStoreEncryptionSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `optimized` after provisioning.\n"]
    pub fn optimized(&self) -> ListRef<VertexAiFeatureOnlineStoreOptimizedElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.optimized", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiFeatureOnlineStoreTimeoutsElRef {
        VertexAiFeatureOnlineStoreTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for VertexAiFeatureOnlineStore {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for VertexAiFeatureOnlineStore {}
impl ToListMappable for VertexAiFeatureOnlineStore {
    type O = ListRef<VertexAiFeatureOnlineStoreRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for VertexAiFeatureOnlineStore_ {
    fn extract_resource_type(&self) -> String {
        "google_vertex_ai_feature_online_store".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildVertexAiFeatureOnlineStore {
    pub tf_id: String,
    #[doc = "The resource name of the Feature Online Store. This value may be up to 60 characters, and valid characters are [a-z0-9_]. The first character cannot be a number."]
    pub name: PrimField<String>,
}
impl BuildVertexAiFeatureOnlineStore {
    pub fn build(self, stack: &mut Stack) -> VertexAiFeatureOnlineStore {
        let out = VertexAiFeatureOnlineStore(Rc::new(VertexAiFeatureOnlineStore_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VertexAiFeatureOnlineStoreData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                bigtable: core::default::Default::default(),
                dedicated_serving_endpoint: core::default::Default::default(),
                encryption_spec: core::default::Default::default(),
                optimized: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct VertexAiFeatureOnlineStoreRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiFeatureOnlineStoreRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl VertexAiFeatureOnlineStoreRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of when the feature online store was created in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
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
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nUsed to perform consistent read-modify-write updates."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\nIf set to true, any FeatureViews and Features for this FeatureOnlineStore will also be deleted."]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_destroy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels with user-defined metadata to organize your feature online stores.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Feature Online Store. This value may be up to 60 characters, and valid characters are [a-z0-9_]. The first character cannot be a number."]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of feature online store. eg us-central1"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the Feature Online Store. See the possible states in [this link](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.featureOnlineStores#state)."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp of when the feature online store was last updated in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bigtable` after provisioning.\n"]
    pub fn bigtable(&self) -> ListRef<VertexAiFeatureOnlineStoreBigtableElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bigtable", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dedicated_serving_endpoint` after provisioning.\n"]
    pub fn dedicated_serving_endpoint(
        &self,
    ) -> ListRef<VertexAiFeatureOnlineStoreDedicatedServingEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dedicated_serving_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> ListRef<VertexAiFeatureOnlineStoreEncryptionSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `optimized` after provisioning.\n"]
    pub fn optimized(&self) -> ListRef<VertexAiFeatureOnlineStoreOptimizedElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.optimized", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiFeatureOnlineStoreTimeoutsElRef {
        VertexAiFeatureOnlineStoreTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiFeatureOnlineStoreBigtableElAutoScalingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_utilization_target: Option<PrimField<f64>>,
    max_node_count: PrimField<f64>,
    min_node_count: PrimField<f64>,
}
impl VertexAiFeatureOnlineStoreBigtableElAutoScalingEl {
    #[doc = "Set the field `cpu_utilization_target`.\nA percentage of the cluster's CPU capacity. Can be from 10% to 80%. When a cluster's CPU utilization exceeds the target that you have set, Bigtable immediately adds nodes to the cluster. When CPU utilization is substantially lower than the target, Bigtable removes nodes. If not set will default to 50%."]
    pub fn set_cpu_utilization_target(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu_utilization_target = Some(v.into());
        self
    }
}
impl ToListMappable for VertexAiFeatureOnlineStoreBigtableElAutoScalingEl {
    type O = BlockAssignable<VertexAiFeatureOnlineStoreBigtableElAutoScalingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiFeatureOnlineStoreBigtableElAutoScalingEl {
    #[doc = "The maximum number of nodes to scale up to. Must be greater than or equal to minNodeCount, and less than or equal to 10 times of 'minNodeCount'."]
    pub max_node_count: PrimField<f64>,
    #[doc = "The minimum number of nodes to scale down to. Must be greater than or equal to 1."]
    pub min_node_count: PrimField<f64>,
}
impl BuildVertexAiFeatureOnlineStoreBigtableElAutoScalingEl {
    pub fn build(self) -> VertexAiFeatureOnlineStoreBigtableElAutoScalingEl {
        VertexAiFeatureOnlineStoreBigtableElAutoScalingEl {
            cpu_utilization_target: core::default::Default::default(),
            max_node_count: self.max_node_count,
            min_node_count: self.min_node_count,
        }
    }
}
pub struct VertexAiFeatureOnlineStoreBigtableElAutoScalingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiFeatureOnlineStoreBigtableElAutoScalingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiFeatureOnlineStoreBigtableElAutoScalingElRef {
        VertexAiFeatureOnlineStoreBigtableElAutoScalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiFeatureOnlineStoreBigtableElAutoScalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cpu_utilization_target` after provisioning.\nA percentage of the cluster's CPU capacity. Can be from 10% to 80%. When a cluster's CPU utilization exceeds the target that you have set, Bigtable immediately adds nodes to the cluster. When CPU utilization is substantially lower than the target, Bigtable removes nodes. If not set will default to 50%."]
    pub fn cpu_utilization_target(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_utilization_target", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_node_count` after provisioning.\nThe maximum number of nodes to scale up to. Must be greater than or equal to minNodeCount, and less than or equal to 10 times of 'minNodeCount'."]
    pub fn max_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_node_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_node_count` after provisioning.\nThe minimum number of nodes to scale down to. Must be greater than or equal to 1."]
    pub fn min_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_node_count", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct VertexAiFeatureOnlineStoreBigtableElDynamic {
    auto_scaling: Option<DynamicBlock<VertexAiFeatureOnlineStoreBigtableElAutoScalingEl>>,
}
#[derive(Serialize)]
pub struct VertexAiFeatureOnlineStoreBigtableEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_direct_bigtable_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_scaling: Option<Vec<VertexAiFeatureOnlineStoreBigtableElAutoScalingEl>>,
    dynamic: VertexAiFeatureOnlineStoreBigtableElDynamic,
}
impl VertexAiFeatureOnlineStoreBigtableEl {
    #[doc = "Set the field `enable_direct_bigtable_access`.\nOptional. If true, enable direct access to the Bigtable instance."]
    pub fn set_enable_direct_bigtable_access(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_direct_bigtable_access = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\nThe zone where the Bigtable instance will be created."]
    pub fn set_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zone = Some(v.into());
        self
    }
    #[doc = "Set the field `auto_scaling`.\n"]
    pub fn set_auto_scaling(
        mut self,
        v: impl Into<BlockAssignable<VertexAiFeatureOnlineStoreBigtableElAutoScalingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.auto_scaling = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.auto_scaling = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VertexAiFeatureOnlineStoreBigtableEl {
    type O = BlockAssignable<VertexAiFeatureOnlineStoreBigtableEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiFeatureOnlineStoreBigtableEl {}
impl BuildVertexAiFeatureOnlineStoreBigtableEl {
    pub fn build(self) -> VertexAiFeatureOnlineStoreBigtableEl {
        VertexAiFeatureOnlineStoreBigtableEl {
            enable_direct_bigtable_access: core::default::Default::default(),
            zone: core::default::Default::default(),
            auto_scaling: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiFeatureOnlineStoreBigtableElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiFeatureOnlineStoreBigtableElRef {
    fn new(shared: StackShared, base: String) -> VertexAiFeatureOnlineStoreBigtableElRef {
        VertexAiFeatureOnlineStoreBigtableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiFeatureOnlineStoreBigtableElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_direct_bigtable_access` after provisioning.\nOptional. If true, enable direct access to the Bigtable instance."]
    pub fn enable_direct_bigtable_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_direct_bigtable_access", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe zone where the Bigtable instance will be created."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
    #[doc = "Get a reference to the value of field `auto_scaling` after provisioning.\n"]
    pub fn auto_scaling(&self) -> ListRef<VertexAiFeatureOnlineStoreBigtableElAutoScalingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto_scaling", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiFeatureOnlineStoreDedicatedServingEndpointElPrivateServiceConnectConfigEl {
    enable_private_service_connect: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_allowlist: Option<ListField<PrimField<String>>>,
}
impl VertexAiFeatureOnlineStoreDedicatedServingEndpointElPrivateServiceConnectConfigEl {
    #[doc = "Set the field `project_allowlist`.\nA list of Projects from which the forwarding rule will target the service attachment."]
    pub fn set_project_allowlist(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.project_allowlist = Some(v.into());
        self
    }
}
impl ToListMappable
    for VertexAiFeatureOnlineStoreDedicatedServingEndpointElPrivateServiceConnectConfigEl
{
    type O = BlockAssignable<
        VertexAiFeatureOnlineStoreDedicatedServingEndpointElPrivateServiceConnectConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiFeatureOnlineStoreDedicatedServingEndpointElPrivateServiceConnectConfigEl {
    #[doc = "If set to true, customers will use private service connection to send request. Otherwise, the connection will set to public endpoint."]
    pub enable_private_service_connect: PrimField<bool>,
}
impl BuildVertexAiFeatureOnlineStoreDedicatedServingEndpointElPrivateServiceConnectConfigEl {
    pub fn build(
        self,
    ) -> VertexAiFeatureOnlineStoreDedicatedServingEndpointElPrivateServiceConnectConfigEl {
        VertexAiFeatureOnlineStoreDedicatedServingEndpointElPrivateServiceConnectConfigEl {
            enable_private_service_connect: self.enable_private_service_connect,
            project_allowlist: core::default::Default::default(),
        }
    }
}
pub struct VertexAiFeatureOnlineStoreDedicatedServingEndpointElPrivateServiceConnectConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiFeatureOnlineStoreDedicatedServingEndpointElPrivateServiceConnectConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiFeatureOnlineStoreDedicatedServingEndpointElPrivateServiceConnectConfigElRef {
        VertexAiFeatureOnlineStoreDedicatedServingEndpointElPrivateServiceConnectConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiFeatureOnlineStoreDedicatedServingEndpointElPrivateServiceConnectConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_private_service_connect` after provisioning.\nIf set to true, customers will use private service connection to send request. Otherwise, the connection will set to public endpoint."]
    pub fn enable_private_service_connect(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_private_service_connect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `project_allowlist` after provisioning.\nA list of Projects from which the forwarding rule will target the service attachment."]
    pub fn project_allowlist(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.project_allowlist", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct VertexAiFeatureOnlineStoreDedicatedServingEndpointElDynamic {
    private_service_connect_config: Option<
        DynamicBlock<
            VertexAiFeatureOnlineStoreDedicatedServingEndpointElPrivateServiceConnectConfigEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct VertexAiFeatureOnlineStoreDedicatedServingEndpointEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    private_service_connect_config: Option<
        Vec<VertexAiFeatureOnlineStoreDedicatedServingEndpointElPrivateServiceConnectConfigEl>,
    >,
    dynamic: VertexAiFeatureOnlineStoreDedicatedServingEndpointElDynamic,
}
impl VertexAiFeatureOnlineStoreDedicatedServingEndpointEl {
    #[doc = "Set the field `private_service_connect_config`.\n"]
    pub fn set_private_service_connect_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                VertexAiFeatureOnlineStoreDedicatedServingEndpointElPrivateServiceConnectConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.private_service_connect_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.private_service_connect_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VertexAiFeatureOnlineStoreDedicatedServingEndpointEl {
    type O = BlockAssignable<VertexAiFeatureOnlineStoreDedicatedServingEndpointEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiFeatureOnlineStoreDedicatedServingEndpointEl {}
impl BuildVertexAiFeatureOnlineStoreDedicatedServingEndpointEl {
    pub fn build(self) -> VertexAiFeatureOnlineStoreDedicatedServingEndpointEl {
        VertexAiFeatureOnlineStoreDedicatedServingEndpointEl {
            private_service_connect_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiFeatureOnlineStoreDedicatedServingEndpointElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiFeatureOnlineStoreDedicatedServingEndpointElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiFeatureOnlineStoreDedicatedServingEndpointElRef {
        VertexAiFeatureOnlineStoreDedicatedServingEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiFeatureOnlineStoreDedicatedServingEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `public_endpoint_domain_name` after provisioning.\nDomain name to use for this FeatureOnlineStore"]
    pub fn public_endpoint_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_endpoint_domain_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_attachment` after provisioning.\nName of the service attachment resource. Applicable only if private service connect is enabled and after FeatureViewSync is created."]
    pub fn service_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_attachment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `private_service_connect_config` after provisioning.\n"]
    pub fn private_service_connect_config(
        &self,
    ) -> ListRef<VertexAiFeatureOnlineStoreDedicatedServingEndpointElPrivateServiceConnectConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_service_connect_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiFeatureOnlineStoreEncryptionSpecEl {
    kms_key_name: PrimField<String>,
}
impl VertexAiFeatureOnlineStoreEncryptionSpecEl {}
impl ToListMappable for VertexAiFeatureOnlineStoreEncryptionSpecEl {
    type O = BlockAssignable<VertexAiFeatureOnlineStoreEncryptionSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiFeatureOnlineStoreEncryptionSpecEl {
    #[doc = "The Cloud KMS resource identifier of the customer managed encryption key used to protect a resource. Has the form: projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key. The key needs to be in the same region as where the compute resource is created."]
    pub kms_key_name: PrimField<String>,
}
impl BuildVertexAiFeatureOnlineStoreEncryptionSpecEl {
    pub fn build(self) -> VertexAiFeatureOnlineStoreEncryptionSpecEl {
        VertexAiFeatureOnlineStoreEncryptionSpecEl {
            kms_key_name: self.kms_key_name,
        }
    }
}
pub struct VertexAiFeatureOnlineStoreEncryptionSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiFeatureOnlineStoreEncryptionSpecElRef {
    fn new(shared: StackShared, base: String) -> VertexAiFeatureOnlineStoreEncryptionSpecElRef {
        VertexAiFeatureOnlineStoreEncryptionSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiFeatureOnlineStoreEncryptionSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nThe Cloud KMS resource identifier of the customer managed encryption key used to protect a resource. Has the form: projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key. The key needs to be in the same region as where the compute resource is created."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiFeatureOnlineStoreOptimizedEl {}
impl VertexAiFeatureOnlineStoreOptimizedEl {}
impl ToListMappable for VertexAiFeatureOnlineStoreOptimizedEl {
    type O = BlockAssignable<VertexAiFeatureOnlineStoreOptimizedEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiFeatureOnlineStoreOptimizedEl {}
impl BuildVertexAiFeatureOnlineStoreOptimizedEl {
    pub fn build(self) -> VertexAiFeatureOnlineStoreOptimizedEl {
        VertexAiFeatureOnlineStoreOptimizedEl {}
    }
}
pub struct VertexAiFeatureOnlineStoreOptimizedElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiFeatureOnlineStoreOptimizedElRef {
    fn new(shared: StackShared, base: String) -> VertexAiFeatureOnlineStoreOptimizedElRef {
        VertexAiFeatureOnlineStoreOptimizedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiFeatureOnlineStoreOptimizedElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct VertexAiFeatureOnlineStoreTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl VertexAiFeatureOnlineStoreTimeoutsEl {
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
impl ToListMappable for VertexAiFeatureOnlineStoreTimeoutsEl {
    type O = BlockAssignable<VertexAiFeatureOnlineStoreTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiFeatureOnlineStoreTimeoutsEl {}
impl BuildVertexAiFeatureOnlineStoreTimeoutsEl {
    pub fn build(self) -> VertexAiFeatureOnlineStoreTimeoutsEl {
        VertexAiFeatureOnlineStoreTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct VertexAiFeatureOnlineStoreTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiFeatureOnlineStoreTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VertexAiFeatureOnlineStoreTimeoutsElRef {
        VertexAiFeatureOnlineStoreTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiFeatureOnlineStoreTimeoutsElRef {
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
struct VertexAiFeatureOnlineStoreDynamic {
    bigtable: Option<DynamicBlock<VertexAiFeatureOnlineStoreBigtableEl>>,
    dedicated_serving_endpoint:
        Option<DynamicBlock<VertexAiFeatureOnlineStoreDedicatedServingEndpointEl>>,
    encryption_spec: Option<DynamicBlock<VertexAiFeatureOnlineStoreEncryptionSpecEl>>,
    optimized: Option<DynamicBlock<VertexAiFeatureOnlineStoreOptimizedEl>>,
}
