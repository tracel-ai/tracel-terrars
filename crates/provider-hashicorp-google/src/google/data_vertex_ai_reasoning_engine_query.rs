use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataVertexAiReasoningEngineQueryData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    class_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    reasoning_engine_id: PrimField<String>,
    region: PrimField<String>,
}
struct DataVertexAiReasoningEngineQuery_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataVertexAiReasoningEngineQueryData>,
}
#[derive(Clone)]
pub struct DataVertexAiReasoningEngineQuery(Rc<DataVertexAiReasoningEngineQuery_>);
impl DataVertexAiReasoningEngineQuery {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `class_method`.\nClass method to be used for the query. It is optional and defaults to \"query\" if unspecified."]
    pub fn set_class_method(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().class_method = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `input`.\nInput content provided by users in JSON object format. Examples include text query, function calling parameters, media bytes, etc.."]
    pub fn set_input(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().input = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe project of the resource. If not provided, the provider default project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `class_method` after provisioning.\nClass method to be used for the query. It is optional and defaults to \"query\" if unspecified."]
    pub fn class_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.class_method", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `input` after provisioning.\nInput content provided by users in JSON object format. Examples include text query, function calling parameters, media bytes, etc.."]
    pub fn input(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `output` after provisioning.\nThe output of the query."]
    pub fn output(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project of the resource. If not provided, the provider default project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reasoning_engine_id` after provisioning.\nThe id of the Vertex Agent Engine to query."]
    pub fn reasoning_engine_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reasoning_engine_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe location of the resource."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}
impl Referable for DataVertexAiReasoningEngineQuery {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataVertexAiReasoningEngineQuery {}
impl ToListMappable for DataVertexAiReasoningEngineQuery {
    type O = ListRef<DataVertexAiReasoningEngineQueryRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataVertexAiReasoningEngineQuery_ {
    fn extract_datasource_type(&self) -> String {
        "google_vertex_ai_reasoning_engine_query".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataVertexAiReasoningEngineQuery {
    pub tf_id: String,
    #[doc = "The id of the Vertex Agent Engine to query."]
    pub reasoning_engine_id: PrimField<String>,
    #[doc = "The location of the resource."]
    pub region: PrimField<String>,
}
impl BuildDataVertexAiReasoningEngineQuery {
    pub fn build(self, stack: &mut Stack) -> DataVertexAiReasoningEngineQuery {
        let out = DataVertexAiReasoningEngineQuery(Rc::new(DataVertexAiReasoningEngineQuery_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataVertexAiReasoningEngineQueryData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                class_method: core::default::Default::default(),
                id: core::default::Default::default(),
                input: core::default::Default::default(),
                project: core::default::Default::default(),
                reasoning_engine_id: self.reasoning_engine_id,
                region: self.region,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataVertexAiReasoningEngineQueryRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVertexAiReasoningEngineQueryRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataVertexAiReasoningEngineQueryRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `class_method` after provisioning.\nClass method to be used for the query. It is optional and defaults to \"query\" if unspecified."]
    pub fn class_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.class_method", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `input` after provisioning.\nInput content provided by users in JSON object format. Examples include text query, function calling parameters, media bytes, etc.."]
    pub fn input(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `output` after provisioning.\nThe output of the query."]
    pub fn output(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project of the resource. If not provided, the provider default project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reasoning_engine_id` after provisioning.\nThe id of the Vertex Agent Engine to query."]
    pub fn reasoning_engine_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reasoning_engine_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe location of the resource."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}
