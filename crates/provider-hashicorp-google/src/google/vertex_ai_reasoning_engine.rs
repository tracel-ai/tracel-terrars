use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct VertexAiReasoningEngineData {
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
    description: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_spec: Option<Vec<VertexAiReasoningEngineEncryptionSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spec: Option<Vec<VertexAiReasoningEngineSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VertexAiReasoningEngineTimeoutsEl>,
    dynamic: VertexAiReasoningEngineDynamic,
}
struct VertexAiReasoningEngine_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VertexAiReasoningEngineData>,
}
#[derive(Clone)]
pub struct VertexAiReasoningEngine(Rc<VertexAiReasoningEngine_>);
impl VertexAiReasoningEngine {
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
    #[doc = "Set the field `deletion_policy`.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/vertex_ai_reasoning_engine.html.markdown for specifics"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nThe description of the ReasoningEngine."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nThe labels associated with this ReasoningEngine. You can use these to\norganize and group your ReasoningEngines.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nThe region of the reasoning engine. eg us-central1"]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_spec`.\n"]
    pub fn set_encryption_spec(
        self,
        v: impl Into<BlockAssignable<VertexAiReasoningEngineEncryptionSpecEl>>,
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
    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(self, v: impl Into<BlockAssignable<VertexAiReasoningEngineSpecEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VertexAiReasoningEngineTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of when the Index was created in RFC3339 UTC \"Zulu\" format,\nwith nanosecond resolution and up to nine fractional digits."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/vertex_ai_reasoning_engine.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the ReasoningEngine."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the ReasoningEngine."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels associated with this ReasoningEngine. You can use these to\norganize and group your ReasoningEngines.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe generated name of the ReasoningEngine, in the format\nprojects/{project}/locations/{location}/reasoningEngines/{reasoningEngine}"]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the reasoning engine. eg us-central1"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp of when the Index was last updated in RFC3339 UTC \"Zulu\"\nformat, with nanosecond resolution and up to nine fractional digits."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> ListRef<VertexAiReasoningEngineEncryptionSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<VertexAiReasoningEngineSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiReasoningEngineTimeoutsElRef {
        VertexAiReasoningEngineTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for VertexAiReasoningEngine {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for VertexAiReasoningEngine {}
impl ToListMappable for VertexAiReasoningEngine {
    type O = ListRef<VertexAiReasoningEngineRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for VertexAiReasoningEngine_ {
    fn extract_resource_type(&self) -> String {
        "google_vertex_ai_reasoning_engine".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildVertexAiReasoningEngine {
    pub tf_id: String,
    #[doc = "The display name of the ReasoningEngine."]
    pub display_name: PrimField<String>,
}
impl BuildVertexAiReasoningEngine {
    pub fn build(self, stack: &mut Stack) -> VertexAiReasoningEngine {
        let out = VertexAiReasoningEngine(Rc::new(VertexAiReasoningEngine_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VertexAiReasoningEngineData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                encryption_spec: core::default::Default::default(),
                spec: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct VertexAiReasoningEngineRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiReasoningEngineRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl VertexAiReasoningEngineRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of when the Index was created in RFC3339 UTC \"Zulu\" format,\nwith nanosecond resolution and up to nine fractional digits."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/vertex_ai_reasoning_engine.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the ReasoningEngine."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the ReasoningEngine."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels associated with this ReasoningEngine. You can use these to\norganize and group your ReasoningEngines.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe generated name of the ReasoningEngine, in the format\nprojects/{project}/locations/{location}/reasoningEngines/{reasoningEngine}"]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the reasoning engine. eg us-central1"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp of when the Index was last updated in RFC3339 UTC \"Zulu\"\nformat, with nanosecond resolution and up to nine fractional digits."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> ListRef<VertexAiReasoningEngineEncryptionSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<VertexAiReasoningEngineSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiReasoningEngineTimeoutsElRef {
        VertexAiReasoningEngineTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiReasoningEngineEncryptionSpecEl {
    kms_key_name: PrimField<String>,
}
impl VertexAiReasoningEngineEncryptionSpecEl {}
impl ToListMappable for VertexAiReasoningEngineEncryptionSpecEl {
    type O = BlockAssignable<VertexAiReasoningEngineEncryptionSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiReasoningEngineEncryptionSpecEl {
    #[doc = "Required. The Cloud KMS resource identifier of the customer managed\nencryption key used to protect a resource. Has the form:\nprojects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key.\nThe key needs to be in the same region as where the compute resource\nis created."]
    pub kms_key_name: PrimField<String>,
}
impl BuildVertexAiReasoningEngineEncryptionSpecEl {
    pub fn build(self) -> VertexAiReasoningEngineEncryptionSpecEl {
        VertexAiReasoningEngineEncryptionSpecEl {
            kms_key_name: self.kms_key_name,
        }
    }
}
pub struct VertexAiReasoningEngineEncryptionSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiReasoningEngineEncryptionSpecElRef {
    fn new(shared: StackShared, base: String) -> VertexAiReasoningEngineEncryptionSpecElRef {
        VertexAiReasoningEngineEncryptionSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiReasoningEngineEncryptionSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nRequired. The Cloud KMS resource identifier of the customer managed\nencryption key used to protect a resource. Has the form:\nprojects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key.\nThe key needs to be in the same region as where the compute resource\nis created."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiReasoningEngineSpecElContainerSpecEl {
    image_uri: PrimField<String>,
}
impl VertexAiReasoningEngineSpecElContainerSpecEl {}
impl ToListMappable for VertexAiReasoningEngineSpecElContainerSpecEl {
    type O = BlockAssignable<VertexAiReasoningEngineSpecElContainerSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiReasoningEngineSpecElContainerSpecEl {
    #[doc = "The Artifact Registry Docker image URI (e.g.,\n'us-central1-docker.pkg.dev/my-project/my-repo/my-image:tag') of the\ncontainer image that is to be run on each worker replica."]
    pub image_uri: PrimField<String>,
}
impl BuildVertexAiReasoningEngineSpecElContainerSpecEl {
    pub fn build(self) -> VertexAiReasoningEngineSpecElContainerSpecEl {
        VertexAiReasoningEngineSpecElContainerSpecEl {
            image_uri: self.image_uri,
        }
    }
}
pub struct VertexAiReasoningEngineSpecElContainerSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiReasoningEngineSpecElContainerSpecElRef {
    fn new(shared: StackShared, base: String) -> VertexAiReasoningEngineSpecElContainerSpecElRef {
        VertexAiReasoningEngineSpecElContainerSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiReasoningEngineSpecElContainerSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `image_uri` after provisioning.\nThe Artifact Registry Docker image URI (e.g.,\n'us-central1-docker.pkg.dev/my-project/my-repo/my-image:tag') of the\ncontainer image that is to be run on each worker replica."]
    pub fn image_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_uri", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiReasoningEngineSpecElDeploymentSpecElEnvEl {
    name: PrimField<String>,
    value: PrimField<String>,
}
impl VertexAiReasoningEngineSpecElDeploymentSpecElEnvEl {}
impl ToListMappable for VertexAiReasoningEngineSpecElDeploymentSpecElEnvEl {
    type O = BlockAssignable<VertexAiReasoningEngineSpecElDeploymentSpecElEnvEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiReasoningEngineSpecElDeploymentSpecElEnvEl {
    #[doc = "The name of the environment variable. Must be a valid\nC identifier."]
    pub name: PrimField<String>,
    #[doc = "Variables that reference a $(VAR_NAME) are expanded using\nthe previous defined environment variables in the container\nand any service environment variables. If a variable cannot\nbe resolved, the reference in the input string will be\nunchanged. The $(VAR_NAME) syntax can be escaped with a\ndouble $$, ie: $$(VAR_NAME). Escaped references will never\nbe expanded, regardless of whether the variable exists\nor not."]
    pub value: PrimField<String>,
}
impl BuildVertexAiReasoningEngineSpecElDeploymentSpecElEnvEl {
    pub fn build(self) -> VertexAiReasoningEngineSpecElDeploymentSpecElEnvEl {
        VertexAiReasoningEngineSpecElDeploymentSpecElEnvEl {
            name: self.name,
            value: self.value,
        }
    }
}
pub struct VertexAiReasoningEngineSpecElDeploymentSpecElEnvElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiReasoningEngineSpecElDeploymentSpecElEnvElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiReasoningEngineSpecElDeploymentSpecElEnvElRef {
        VertexAiReasoningEngineSpecElDeploymentSpecElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiReasoningEngineSpecElDeploymentSpecElEnvElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the environment variable. Must be a valid\nC identifier."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nVariables that reference a $(VAR_NAME) are expanded using\nthe previous defined environment variables in the container\nand any service environment variables. If a variable cannot\nbe resolved, the reference in the input string will be\nunchanged. The $(VAR_NAME) syntax can be escaped with a\ndouble $$, ie: $$(VAR_NAME). Escaped references will never\nbe expanded, regardless of whether the variable exists\nor not."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigElDnsPeeringConfigsEl {
    domain: PrimField<String>,
    target_network: PrimField<String>,
    target_project: PrimField<String>,
}
impl VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigElDnsPeeringConfigsEl {}
impl ToListMappable
    for VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigElDnsPeeringConfigsEl
{
    type O = BlockAssignable<
        VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigElDnsPeeringConfigsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigElDnsPeeringConfigsEl
{
    #[doc = "Required. The DNS name suffix of the zone being peered\nto, e.g., \"my-internal-domain.corp.\".\nMust end with a dot."]
    pub domain: PrimField<String>,
    #[doc = "Required. The VPC network name in the targetProject\nwhere the DNS zone specified by 'domain' is visible."]
    pub target_network: PrimField<String>,
    #[doc = "Required. The project id hosting the Cloud DNS managed\nzone that contains the 'domain'.\nThe Vertex AI service Agent requires the dns.peer role\non this project."]
    pub target_project: PrimField<String>,
}
impl BuildVertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigElDnsPeeringConfigsEl {
    pub fn build(
        self,
    ) -> VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigElDnsPeeringConfigsEl {
        VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigElDnsPeeringConfigsEl {
            domain: self.domain,
            target_network: self.target_network,
            target_project: self.target_project,
        }
    }
}
pub struct VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigElDnsPeeringConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigElDnsPeeringConfigsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigElDnsPeeringConfigsElRef
    {
        VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigElDnsPeeringConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigElDnsPeeringConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\nRequired. The DNS name suffix of the zone being peered\nto, e.g., \"my-internal-domain.corp.\".\nMust end with a dot."]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
    #[doc = "Get a reference to the value of field `target_network` after provisioning.\nRequired. The VPC network name in the targetProject\nwhere the DNS zone specified by 'domain' is visible."]
    pub fn target_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_network", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_project` after provisioning.\nRequired. The project id hosting the Cloud DNS managed\nzone that contains the 'domain'.\nThe Vertex AI service Agent requires the dns.peer role\non this project."]
    pub fn target_project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_project", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigElDynamic {
    dns_peering_configs: Option<
        DynamicBlock<
            VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigElDnsPeeringConfigsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network_attachment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_peering_configs: Option<
        Vec<VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigElDnsPeeringConfigsEl>,
    >,
    dynamic: VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigElDynamic,
}
impl VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigEl {
    #[doc = "Set the field `network_attachment`.\nOptional. The name of the Compute Engine network attachment\nto attach to the resource within the region and user project.\nTo specify this field, you must have already created a network attachment.\nThis field is only used for resources using PSC-Interface."]
    pub fn set_network_attachment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_attachment = Some(v.into());
        self
    }
    #[doc = "Set the field `dns_peering_configs`.\n"]
    pub fn set_dns_peering_configs(
        mut self,
        v : impl Into < BlockAssignable < VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigElDnsPeeringConfigsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dns_peering_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dns_peering_configs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigEl {
    type O = BlockAssignable<VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigEl {}
impl BuildVertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigEl {
    pub fn build(self) -> VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigEl {
        VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigEl {
            network_attachment: core::default::Default::default(),
            dns_peering_configs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigElRef {
        VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network_attachment` after provisioning.\nOptional. The name of the Compute Engine network attachment\nto attach to the resource within the region and user project.\nTo specify this field, you must have already created a network attachment.\nThis field is only used for resources using PSC-Interface."]
    pub fn network_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_attachment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dns_peering_configs` after provisioning.\n"]
    pub fn dns_peering_configs(
        &self,
    ) -> ListRef<
        VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigElDnsPeeringConfigsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dns_peering_configs", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvElSecretRefEl {
    secret: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvElSecretRefEl {
    #[doc = "Set the field `version`.\nThe Cloud Secret Manager secret version. Can be 'latest'\nfor the latest version, an integer for a specific\nversion, or a version alias."]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvElSecretRefEl {
    type O = BlockAssignable<VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvElSecretRefEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvElSecretRefEl {
    #[doc = "The name of the secret in Cloud Secret Manager.\nFormat: {secret_name}."]
    pub secret: PrimField<String>,
}
impl BuildVertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvElSecretRefEl {
    pub fn build(self) -> VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvElSecretRefEl {
        VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvElSecretRefEl {
            secret: self.secret,
            version: core::default::Default::default(),
        }
    }
}
pub struct VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvElSecretRefElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvElSecretRefElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvElSecretRefElRef {
        VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvElSecretRefElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvElSecretRefElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\nThe name of the secret in Cloud Secret Manager.\nFormat: {secret_name}."]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe Cloud Secret Manager secret version. Can be 'latest'\nfor the latest version, an integer for a specific\nversion, or a version alias."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize, Default)]
struct VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvElDynamic {
    secret_ref:
        Option<DynamicBlock<VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvElSecretRefEl>>,
}
#[derive(Serialize)]
pub struct VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_ref: Option<Vec<VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvElSecretRefEl>>,
    dynamic: VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvElDynamic,
}
impl VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvEl {
    #[doc = "Set the field `secret_ref`.\n"]
    pub fn set_secret_ref(
        mut self,
        v: impl Into<
            BlockAssignable<VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvElSecretRefEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.secret_ref = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_ref = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvEl {
    type O = BlockAssignable<VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvEl {
    #[doc = "The name of the environment variable. Must be a valid C\nidentifier."]
    pub name: PrimField<String>,
}
impl BuildVertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvEl {
    pub fn build(self) -> VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvEl {
        VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvEl {
            name: self.name,
            secret_ref: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvElRef {
        VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the environment variable. Must be a valid C\nidentifier."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_ref` after provisioning.\n"]
    pub fn secret_ref(
        &self,
    ) -> ListRef<VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvElSecretRefElRef> {
        ListRef::new(self.shared().clone(), format!("{}.secret_ref", self.base))
    }
}
#[derive(Serialize, Default)]
struct VertexAiReasoningEngineSpecElDeploymentSpecElDynamic {
    env: Option<DynamicBlock<VertexAiReasoningEngineSpecElDeploymentSpecElEnvEl>>,
    psc_interface_config:
        Option<DynamicBlock<VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigEl>>,
    secret_env: Option<DynamicBlock<VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvEl>>,
}
#[derive(Serialize)]
pub struct VertexAiReasoningEngineSpecElDeploymentSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_concurrency: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_instances: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_instances: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_limits: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<Vec<VertexAiReasoningEngineSpecElDeploymentSpecElEnvEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_interface_config:
        Option<Vec<VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_env: Option<Vec<VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvEl>>,
    dynamic: VertexAiReasoningEngineSpecElDeploymentSpecElDynamic,
}
impl VertexAiReasoningEngineSpecElDeploymentSpecEl {
    #[doc = "Set the field `container_concurrency`.\nOptional. Concurrency for each container and agent server.\nRecommended value: 2 * cpu + 1. Defaults to 9."]
    pub fn set_container_concurrency(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.container_concurrency = Some(v.into());
        self
    }
    #[doc = "Set the field `max_instances`.\nOptional. The maximum number of application instances that can be\nlaunched to handle increased traffic. Defaults to 100.\nRange: [1, 1000]. If VPC-SC or PSC-I is enabled, the acceptable\nrange is [1, 100]."]
    pub fn set_max_instances(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_instances = Some(v.into());
        self
    }
    #[doc = "Set the field `min_instances`.\nOptional. The minimum number of application instances that will be\nkept running at all times. Defaults to 1. Range: [0, 10]."]
    pub fn set_min_instances(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_instances = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_limits`.\nOptional. Resource limits for each container.\nOnly 'cpu' and 'memory' keys are supported.\n\nDefaults to {\"cpu\": \"4\", \"memory\": \"4Gi\"}.\n\nThe only supported values for CPU are '1', '2', '4', '6' and '8'.\nFor more information, go to\nhttps://cloud.google.com/run/docs/configuring/cpu.\n\nThe only supported values for memory are '1Gi', '2Gi', ... '32 Gi'.\nFor more information, go to\nhttps://cloud.google.com/run/docs/configuring/memory-limits."]
    pub fn set_resource_limits(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_limits = Some(v.into());
        self
    }
    #[doc = "Set the field `env`.\n"]
    pub fn set_env(
        mut self,
        v: impl Into<BlockAssignable<VertexAiReasoningEngineSpecElDeploymentSpecElEnvEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.env = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.env = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `psc_interface_config`.\n"]
    pub fn set_psc_interface_config(
        mut self,
        v: impl Into<BlockAssignable<VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.psc_interface_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.psc_interface_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `secret_env`.\n"]
    pub fn set_secret_env(
        mut self,
        v: impl Into<BlockAssignable<VertexAiReasoningEngineSpecElDeploymentSpecElSecretEnvEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.secret_env = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_env = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VertexAiReasoningEngineSpecElDeploymentSpecEl {
    type O = BlockAssignable<VertexAiReasoningEngineSpecElDeploymentSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiReasoningEngineSpecElDeploymentSpecEl {}
impl BuildVertexAiReasoningEngineSpecElDeploymentSpecEl {
    pub fn build(self) -> VertexAiReasoningEngineSpecElDeploymentSpecEl {
        VertexAiReasoningEngineSpecElDeploymentSpecEl {
            container_concurrency: core::default::Default::default(),
            max_instances: core::default::Default::default(),
            min_instances: core::default::Default::default(),
            resource_limits: core::default::Default::default(),
            env: core::default::Default::default(),
            psc_interface_config: core::default::Default::default(),
            secret_env: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiReasoningEngineSpecElDeploymentSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiReasoningEngineSpecElDeploymentSpecElRef {
    fn new(shared: StackShared, base: String) -> VertexAiReasoningEngineSpecElDeploymentSpecElRef {
        VertexAiReasoningEngineSpecElDeploymentSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiReasoningEngineSpecElDeploymentSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container_concurrency` after provisioning.\nOptional. Concurrency for each container and agent server.\nRecommended value: 2 * cpu + 1. Defaults to 9."]
    pub fn container_concurrency(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_concurrency", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_instances` after provisioning.\nOptional. The maximum number of application instances that can be\nlaunched to handle increased traffic. Defaults to 100.\nRange: [1, 1000]. If VPC-SC or PSC-I is enabled, the acceptable\nrange is [1, 100]."]
    pub fn max_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_instances", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_instances` after provisioning.\nOptional. The minimum number of application instances that will be\nkept running at all times. Defaults to 1. Range: [0, 10]."]
    pub fn min_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_instances", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_limits` after provisioning.\nOptional. Resource limits for each container.\nOnly 'cpu' and 'memory' keys are supported.\n\nDefaults to {\"cpu\": \"4\", \"memory\": \"4Gi\"}.\n\nThe only supported values for CPU are '1', '2', '4', '6' and '8'.\nFor more information, go to\nhttps://cloud.google.com/run/docs/configuring/cpu.\n\nThe only supported values for memory are '1Gi', '2Gi', ... '32 Gi'.\nFor more information, go to\nhttps://cloud.google.com/run/docs/configuring/memory-limits."]
    pub fn resource_limits(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_limits", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `psc_interface_config` after provisioning.\n"]
    pub fn psc_interface_config(
        &self,
    ) -> ListRef<VertexAiReasoningEngineSpecElDeploymentSpecElPscInterfaceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_interface_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiReasoningEngineSpecElPackageSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dependency_files_gcs_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pickle_object_gcs_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    python_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requirements_gcs_uri: Option<PrimField<String>>,
}
impl VertexAiReasoningEngineSpecElPackageSpecEl {
    #[doc = "Set the field `dependency_files_gcs_uri`.\nOptional. The Cloud Storage URI of the dependency files in tar.gz\nformat."]
    pub fn set_dependency_files_gcs_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dependency_files_gcs_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `pickle_object_gcs_uri`.\nOptional. The Cloud Storage URI of the pickled python object."]
    pub fn set_pickle_object_gcs_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pickle_object_gcs_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `python_version`.\nOptional. The Python version. Currently support 3.8, 3.9, 3.10,\n3.11, 3.12, 3.13. If not specified, default value is 3.10."]
    pub fn set_python_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.python_version = Some(v.into());
        self
    }
    #[doc = "Set the field `requirements_gcs_uri`.\nOptional. The Cloud Storage URI of the requirements.txtfile"]
    pub fn set_requirements_gcs_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.requirements_gcs_uri = Some(v.into());
        self
    }
}
impl ToListMappable for VertexAiReasoningEngineSpecElPackageSpecEl {
    type O = BlockAssignable<VertexAiReasoningEngineSpecElPackageSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiReasoningEngineSpecElPackageSpecEl {}
impl BuildVertexAiReasoningEngineSpecElPackageSpecEl {
    pub fn build(self) -> VertexAiReasoningEngineSpecElPackageSpecEl {
        VertexAiReasoningEngineSpecElPackageSpecEl {
            dependency_files_gcs_uri: core::default::Default::default(),
            pickle_object_gcs_uri: core::default::Default::default(),
            python_version: core::default::Default::default(),
            requirements_gcs_uri: core::default::Default::default(),
        }
    }
}
pub struct VertexAiReasoningEngineSpecElPackageSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiReasoningEngineSpecElPackageSpecElRef {
    fn new(shared: StackShared, base: String) -> VertexAiReasoningEngineSpecElPackageSpecElRef {
        VertexAiReasoningEngineSpecElPackageSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiReasoningEngineSpecElPackageSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dependency_files_gcs_uri` after provisioning.\nOptional. The Cloud Storage URI of the dependency files in tar.gz\nformat."]
    pub fn dependency_files_gcs_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dependency_files_gcs_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pickle_object_gcs_uri` after provisioning.\nOptional. The Cloud Storage URI of the pickled python object."]
    pub fn pickle_object_gcs_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pickle_object_gcs_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `python_version` after provisioning.\nOptional. The Python version. Currently support 3.8, 3.9, 3.10,\n3.11, 3.12, 3.13. If not specified, default value is 3.10."]
    pub fn python_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.python_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `requirements_gcs_uri` after provisioning.\nOptional. The Cloud Storage URI of the requirements.txtfile"]
    pub fn requirements_gcs_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.requirements_gcs_uri", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceElConfigEl {
    dir: PrimField<String>,
    git_repository_link: PrimField<String>,
    revision: PrimField<String>,
}
impl VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceElConfigEl {}
impl ToListMappable
    for VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceElConfigEl
{
    type O = BlockAssignable<
        VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceElConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceElConfigEl {
    #[doc = "Directory, relative to the source root, in which to run the build."]
    pub dir: PrimField<String>,
    #[doc = "The Developer Connect Git repository link, formatted as projects/*/locations/*/connections/*/gitRepositoryLink/*."]
    pub git_repository_link: PrimField<String>,
    #[doc = "The revision to fetch from the Git repository such as a branch, a tag, a commit SHA, or any Git ref."]
    pub revision: PrimField<String>,
}
impl BuildVertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceElConfigEl {
    pub fn build(
        self,
    ) -> VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceElConfigEl {
        VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceElConfigEl {
            dir: self.dir,
            git_repository_link: self.git_repository_link,
            revision: self.revision,
        }
    }
}
pub struct VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceElConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceElConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceElConfigElRef {
        VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceElConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceElConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dir` after provisioning.\nDirectory, relative to the source root, in which to run the build."]
    pub fn dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dir", self.base))
    }
    #[doc = "Get a reference to the value of field `git_repository_link` after provisioning.\nThe Developer Connect Git repository link, formatted as projects/*/locations/*/connections/*/gitRepositoryLink/*."]
    pub fn git_repository_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.git_repository_link", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `revision` after provisioning.\nThe revision to fetch from the Git repository such as a branch, a tag, a commit SHA, or any Git ref."]
    pub fn revision(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }
}
#[derive(Serialize, Default)]
struct VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceElDynamic {
    config: Option<
        DynamicBlock<VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceElConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    config:
        Option<Vec<VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceElConfigEl>>,
    dynamic: VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceElDynamic,
}
impl VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceEl {
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceElConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceEl {
    type O = BlockAssignable<VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceEl {}
impl BuildVertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceEl {
    pub fn build(self) -> VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceEl {
        VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceEl {
            config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceElRef {
        VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(
        &self,
    ) -> ListRef<VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceElConfigElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiReasoningEngineSpecElSourceCodeSpecElImageSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    build_args: Option<RecField<PrimField<String>>>,
}
impl VertexAiReasoningEngineSpecElSourceCodeSpecElImageSpecEl {
    #[doc = "Set the field `build_args`.\nBuild arguments to be used. They will be passed through --build-arg flags."]
    pub fn set_build_args(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.build_args = Some(v.into());
        self
    }
}
impl ToListMappable for VertexAiReasoningEngineSpecElSourceCodeSpecElImageSpecEl {
    type O = BlockAssignable<VertexAiReasoningEngineSpecElSourceCodeSpecElImageSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiReasoningEngineSpecElSourceCodeSpecElImageSpecEl {}
impl BuildVertexAiReasoningEngineSpecElSourceCodeSpecElImageSpecEl {
    pub fn build(self) -> VertexAiReasoningEngineSpecElSourceCodeSpecElImageSpecEl {
        VertexAiReasoningEngineSpecElSourceCodeSpecElImageSpecEl {
            build_args: core::default::Default::default(),
        }
    }
}
pub struct VertexAiReasoningEngineSpecElSourceCodeSpecElImageSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiReasoningEngineSpecElSourceCodeSpecElImageSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiReasoningEngineSpecElSourceCodeSpecElImageSpecElRef {
        VertexAiReasoningEngineSpecElSourceCodeSpecElImageSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiReasoningEngineSpecElSourceCodeSpecElImageSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `build_args` after provisioning.\nBuild arguments to be used. They will be passed through --build-arg flags."]
    pub fn build_args(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.build_args", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiReasoningEngineSpecElSourceCodeSpecElInlineSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    source_archive: Option<PrimField<String>>,
}
impl VertexAiReasoningEngineSpecElSourceCodeSpecElInlineSourceEl {
    #[doc = "Set the field `source_archive`.\nRequired. Input only.\nThe application source code archive, provided as a compressed\ntarball (.tar.gz) file. A base64-encoded string."]
    pub fn set_source_archive(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_archive = Some(v.into());
        self
    }
}
impl ToListMappable for VertexAiReasoningEngineSpecElSourceCodeSpecElInlineSourceEl {
    type O = BlockAssignable<VertexAiReasoningEngineSpecElSourceCodeSpecElInlineSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiReasoningEngineSpecElSourceCodeSpecElInlineSourceEl {}
impl BuildVertexAiReasoningEngineSpecElSourceCodeSpecElInlineSourceEl {
    pub fn build(self) -> VertexAiReasoningEngineSpecElSourceCodeSpecElInlineSourceEl {
        VertexAiReasoningEngineSpecElSourceCodeSpecElInlineSourceEl {
            source_archive: core::default::Default::default(),
        }
    }
}
pub struct VertexAiReasoningEngineSpecElSourceCodeSpecElInlineSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiReasoningEngineSpecElSourceCodeSpecElInlineSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiReasoningEngineSpecElSourceCodeSpecElInlineSourceElRef {
        VertexAiReasoningEngineSpecElSourceCodeSpecElInlineSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiReasoningEngineSpecElSourceCodeSpecElInlineSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `source_archive` after provisioning.\nRequired. Input only.\nThe application source code archive, provided as a compressed\ntarball (.tar.gz) file. A base64-encoded string."]
    pub fn source_archive(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_archive", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiReasoningEngineSpecElSourceCodeSpecElPythonSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    entrypoint_module: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entrypoint_object: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requirements_file: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl VertexAiReasoningEngineSpecElSourceCodeSpecElPythonSpecEl {
    #[doc = "Set the field `entrypoint_module`.\nOptional. The Python module to load as the entrypoint,\nspecified as a fully qualified module name. For example:\npath.to.agent. If not specified, defaults to \"agent\".\nThe project root will be added to Python sys.path, allowing\nimports to be specified relative to the root."]
    pub fn set_entrypoint_module(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.entrypoint_module = Some(v.into());
        self
    }
    #[doc = "Set the field `entrypoint_object`.\nOptional. The name of the callable object within the\nentrypointModule to use as the application If not specified,\ndefaults to \"root_agent\"."]
    pub fn set_entrypoint_object(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.entrypoint_object = Some(v.into());
        self
    }
    #[doc = "Set the field `requirements_file`.\nOptional. The path to the requirements file, relative to the\nsource root. If not specified, defaults to \"requirements.txt\"."]
    pub fn set_requirements_file(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.requirements_file = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\nOptional. The version of Python to use. Support version\nincludes 3.9, 3.10, 3.11, 3.12, 3.13. If not specified,\ndefault value is 3.10."]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for VertexAiReasoningEngineSpecElSourceCodeSpecElPythonSpecEl {
    type O = BlockAssignable<VertexAiReasoningEngineSpecElSourceCodeSpecElPythonSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiReasoningEngineSpecElSourceCodeSpecElPythonSpecEl {}
impl BuildVertexAiReasoningEngineSpecElSourceCodeSpecElPythonSpecEl {
    pub fn build(self) -> VertexAiReasoningEngineSpecElSourceCodeSpecElPythonSpecEl {
        VertexAiReasoningEngineSpecElSourceCodeSpecElPythonSpecEl {
            entrypoint_module: core::default::Default::default(),
            entrypoint_object: core::default::Default::default(),
            requirements_file: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct VertexAiReasoningEngineSpecElSourceCodeSpecElPythonSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiReasoningEngineSpecElSourceCodeSpecElPythonSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiReasoningEngineSpecElSourceCodeSpecElPythonSpecElRef {
        VertexAiReasoningEngineSpecElSourceCodeSpecElPythonSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiReasoningEngineSpecElSourceCodeSpecElPythonSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `entrypoint_module` after provisioning.\nOptional. The Python module to load as the entrypoint,\nspecified as a fully qualified module name. For example:\npath.to.agent. If not specified, defaults to \"agent\".\nThe project root will be added to Python sys.path, allowing\nimports to be specified relative to the root."]
    pub fn entrypoint_module(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entrypoint_module", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `entrypoint_object` after provisioning.\nOptional. The name of the callable object within the\nentrypointModule to use as the application If not specified,\ndefaults to \"root_agent\"."]
    pub fn entrypoint_object(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entrypoint_object", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `requirements_file` after provisioning.\nOptional. The path to the requirements file, relative to the\nsource root. If not specified, defaults to \"requirements.txt\"."]
    pub fn requirements_file(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.requirements_file", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nOptional. The version of Python to use. Support version\nincludes 3.9, 3.10, 3.11, 3.12, 3.13. If not specified,\ndefault value is 3.10."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize, Default)]
struct VertexAiReasoningEngineSpecElSourceCodeSpecElDynamic {
    developer_connect_source:
        Option<DynamicBlock<VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceEl>>,
    image_spec: Option<DynamicBlock<VertexAiReasoningEngineSpecElSourceCodeSpecElImageSpecEl>>,
    inline_source:
        Option<DynamicBlock<VertexAiReasoningEngineSpecElSourceCodeSpecElInlineSourceEl>>,
    python_spec: Option<DynamicBlock<VertexAiReasoningEngineSpecElSourceCodeSpecElPythonSpecEl>>,
}
#[derive(Serialize)]
pub struct VertexAiReasoningEngineSpecElSourceCodeSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    developer_connect_source:
        Option<Vec<VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_spec: Option<Vec<VertexAiReasoningEngineSpecElSourceCodeSpecElImageSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_source: Option<Vec<VertexAiReasoningEngineSpecElSourceCodeSpecElInlineSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    python_spec: Option<Vec<VertexAiReasoningEngineSpecElSourceCodeSpecElPythonSpecEl>>,
    dynamic: VertexAiReasoningEngineSpecElSourceCodeSpecElDynamic,
}
impl VertexAiReasoningEngineSpecElSourceCodeSpecEl {
    #[doc = "Set the field `developer_connect_source`.\n"]
    pub fn set_developer_connect_source(
        mut self,
        v: impl Into<
            BlockAssignable<VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.developer_connect_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.developer_connect_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `image_spec`.\n"]
    pub fn set_image_spec(
        mut self,
        v: impl Into<BlockAssignable<VertexAiReasoningEngineSpecElSourceCodeSpecElImageSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.image_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.image_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `inline_source`.\n"]
    pub fn set_inline_source(
        mut self,
        v: impl Into<BlockAssignable<VertexAiReasoningEngineSpecElSourceCodeSpecElInlineSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.inline_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.inline_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `python_spec`.\n"]
    pub fn set_python_spec(
        mut self,
        v: impl Into<BlockAssignable<VertexAiReasoningEngineSpecElSourceCodeSpecElPythonSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.python_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.python_spec = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VertexAiReasoningEngineSpecElSourceCodeSpecEl {
    type O = BlockAssignable<VertexAiReasoningEngineSpecElSourceCodeSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiReasoningEngineSpecElSourceCodeSpecEl {}
impl BuildVertexAiReasoningEngineSpecElSourceCodeSpecEl {
    pub fn build(self) -> VertexAiReasoningEngineSpecElSourceCodeSpecEl {
        VertexAiReasoningEngineSpecElSourceCodeSpecEl {
            developer_connect_source: core::default::Default::default(),
            image_spec: core::default::Default::default(),
            inline_source: core::default::Default::default(),
            python_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiReasoningEngineSpecElSourceCodeSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiReasoningEngineSpecElSourceCodeSpecElRef {
    fn new(shared: StackShared, base: String) -> VertexAiReasoningEngineSpecElSourceCodeSpecElRef {
        VertexAiReasoningEngineSpecElSourceCodeSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiReasoningEngineSpecElSourceCodeSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `developer_connect_source` after provisioning.\n"]
    pub fn developer_connect_source(
        &self,
    ) -> ListRef<VertexAiReasoningEngineSpecElSourceCodeSpecElDeveloperConnectSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.developer_connect_source", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_spec` after provisioning.\n"]
    pub fn image_spec(
        &self,
    ) -> ListRef<VertexAiReasoningEngineSpecElSourceCodeSpecElImageSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image_spec", self.base))
    }
    #[doc = "Get a reference to the value of field `inline_source` after provisioning.\n"]
    pub fn inline_source(
        &self,
    ) -> ListRef<VertexAiReasoningEngineSpecElSourceCodeSpecElInlineSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inline_source", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `python_spec` after provisioning.\n"]
    pub fn python_spec(
        &self,
    ) -> ListRef<VertexAiReasoningEngineSpecElSourceCodeSpecElPythonSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.python_spec", self.base))
    }
}
#[derive(Serialize, Default)]
struct VertexAiReasoningEngineSpecElDynamic {
    container_spec: Option<DynamicBlock<VertexAiReasoningEngineSpecElContainerSpecEl>>,
    deployment_spec: Option<DynamicBlock<VertexAiReasoningEngineSpecElDeploymentSpecEl>>,
    package_spec: Option<DynamicBlock<VertexAiReasoningEngineSpecElPackageSpecEl>>,
    source_code_spec: Option<DynamicBlock<VertexAiReasoningEngineSpecElSourceCodeSpecEl>>,
}
#[derive(Serialize)]
pub struct VertexAiReasoningEngineSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_framework: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    class_methods: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_spec: Option<Vec<VertexAiReasoningEngineSpecElContainerSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_spec: Option<Vec<VertexAiReasoningEngineSpecElDeploymentSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    package_spec: Option<Vec<VertexAiReasoningEngineSpecElPackageSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_code_spec: Option<Vec<VertexAiReasoningEngineSpecElSourceCodeSpecEl>>,
    dynamic: VertexAiReasoningEngineSpecElDynamic,
}
impl VertexAiReasoningEngineSpecEl {
    #[doc = "Set the field `agent_framework`.\nOptional. The OSS agent framework used to develop the agent."]
    pub fn set_agent_framework(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.agent_framework = Some(v.into());
        self
    }
    #[doc = "Set the field `class_methods`.\nOptional. Declarations for object class methods in OpenAPI\nspecification format."]
    pub fn set_class_methods(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.class_methods = Some(v.into());
        self
    }
    #[doc = "Set the field `identity_type`.\nOptional. The identity type to use for the Reasoning Engine.\nIf not specified, the 'service_account' field will be used if set,\notherwise the default Vertex AI Reasoning Engine Service Agent in the project will be used.\nPossible values:\n* 'SERVICE_ACCOUNT': Use a custom service account if the 'service_account' field is set, otherwise use the default Vertex AI Reasoning Engine Service Agent in the project.\n* 'AGENT_IDENTITY': Use Agent Identity. The 'service_account' field must not be set. Possible values: [\"SERVICE_ACCOUNT\", \"AGENT_IDENTITY\"]"]
    pub fn set_identity_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_type = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\nOptional. The service account that the Reasoning Engine artifact runs\nas. It should have \"roles/storage.objectViewer\" for reading the user\nproject's Cloud Storage and \"roles/aiplatform.user\" for using Vertex\nextensions. If not specified, the Vertex AI Reasoning Engine service\nAgent in the project will be used."]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `container_spec`.\n"]
    pub fn set_container_spec(
        mut self,
        v: impl Into<BlockAssignable<VertexAiReasoningEngineSpecElContainerSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.container_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.container_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `deployment_spec`.\n"]
    pub fn set_deployment_spec(
        mut self,
        v: impl Into<BlockAssignable<VertexAiReasoningEngineSpecElDeploymentSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.deployment_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.deployment_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `package_spec`.\n"]
    pub fn set_package_spec(
        mut self,
        v: impl Into<BlockAssignable<VertexAiReasoningEngineSpecElPackageSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.package_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.package_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source_code_spec`.\n"]
    pub fn set_source_code_spec(
        mut self,
        v: impl Into<BlockAssignable<VertexAiReasoningEngineSpecElSourceCodeSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_code_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_code_spec = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VertexAiReasoningEngineSpecEl {
    type O = BlockAssignable<VertexAiReasoningEngineSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiReasoningEngineSpecEl {}
impl BuildVertexAiReasoningEngineSpecEl {
    pub fn build(self) -> VertexAiReasoningEngineSpecEl {
        VertexAiReasoningEngineSpecEl {
            agent_framework: core::default::Default::default(),
            class_methods: core::default::Default::default(),
            identity_type: core::default::Default::default(),
            service_account: core::default::Default::default(),
            container_spec: core::default::Default::default(),
            deployment_spec: core::default::Default::default(),
            package_spec: core::default::Default::default(),
            source_code_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiReasoningEngineSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiReasoningEngineSpecElRef {
    fn new(shared: StackShared, base: String) -> VertexAiReasoningEngineSpecElRef {
        VertexAiReasoningEngineSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiReasoningEngineSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `agent_framework` after provisioning.\nOptional. The OSS agent framework used to develop the agent."]
    pub fn agent_framework(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.agent_framework", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `class_methods` after provisioning.\nOptional. Declarations for object class methods in OpenAPI\nspecification format."]
    pub fn class_methods(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.class_methods", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `effective_identity` after provisioning.\nThe identity to use for the Reasoning Engine."]
    pub fn effective_identity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_identity", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `identity_type` after provisioning.\nOptional. The identity type to use for the Reasoning Engine.\nIf not specified, the 'service_account' field will be used if set,\notherwise the default Vertex AI Reasoning Engine Service Agent in the project will be used.\nPossible values:\n* 'SERVICE_ACCOUNT': Use a custom service account if the 'service_account' field is set, otherwise use the default Vertex AI Reasoning Engine Service Agent in the project.\n* 'AGENT_IDENTITY': Use Agent Identity. The 'service_account' field must not be set. Possible values: [\"SERVICE_ACCOUNT\", \"AGENT_IDENTITY\"]"]
    pub fn identity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nOptional. The service account that the Reasoning Engine artifact runs\nas. It should have \"roles/storage.objectViewer\" for reading the user\nproject's Cloud Storage and \"roles/aiplatform.user\" for using Vertex\nextensions. If not specified, the Vertex AI Reasoning Engine service\nAgent in the project will be used."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `container_spec` after provisioning.\n"]
    pub fn container_spec(&self) -> ListRef<VertexAiReasoningEngineSpecElContainerSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.container_spec", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `deployment_spec` after provisioning.\n"]
    pub fn deployment_spec(&self) -> ListRef<VertexAiReasoningEngineSpecElDeploymentSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.deployment_spec", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `package_spec` after provisioning.\n"]
    pub fn package_spec(&self) -> ListRef<VertexAiReasoningEngineSpecElPackageSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.package_spec", self.base))
    }
    #[doc = "Get a reference to the value of field `source_code_spec` after provisioning.\n"]
    pub fn source_code_spec(&self) -> ListRef<VertexAiReasoningEngineSpecElSourceCodeSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_code_spec", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiReasoningEngineTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl VertexAiReasoningEngineTimeoutsEl {
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
impl ToListMappable for VertexAiReasoningEngineTimeoutsEl {
    type O = BlockAssignable<VertexAiReasoningEngineTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiReasoningEngineTimeoutsEl {}
impl BuildVertexAiReasoningEngineTimeoutsEl {
    pub fn build(self) -> VertexAiReasoningEngineTimeoutsEl {
        VertexAiReasoningEngineTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct VertexAiReasoningEngineTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiReasoningEngineTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VertexAiReasoningEngineTimeoutsElRef {
        VertexAiReasoningEngineTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiReasoningEngineTimeoutsElRef {
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
struct VertexAiReasoningEngineDynamic {
    encryption_spec: Option<DynamicBlock<VertexAiReasoningEngineEncryptionSpecEl>>,
    spec: Option<DynamicBlock<VertexAiReasoningEngineSpecEl>>,
}
