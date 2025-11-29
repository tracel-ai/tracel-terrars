use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataGlueScriptData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dag_edge: Option<Vec<DataGlueScriptDagEdgeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dag_node: Option<Vec<DataGlueScriptDagNodeEl>>,
    dynamic: DataGlueScriptDynamic,
}

struct DataGlueScript_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataGlueScriptData>,
}

#[derive(Clone)]
pub struct DataGlueScript(Rc<DataGlueScript_>);

impl DataGlueScript {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `language`.\n"]
    pub fn set_language(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().language = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `dag_edge`.\n"]
    pub fn set_dag_edge(self, v: impl Into<BlockAssignable<DataGlueScriptDagEdgeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dag_edge = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dag_edge = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dag_node`.\n"]
    pub fn set_dag_node(self, v: impl Into<BlockAssignable<DataGlueScriptDagNodeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dag_node = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dag_node = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `language` after provisioning.\n"]
    pub fn language(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.language", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `python_script` after provisioning.\n"]
    pub fn python_script(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.python_script", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scala_code` after provisioning.\n"]
    pub fn scala_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scala_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dag_edge` after provisioning.\n"]
    pub fn dag_edge(&self) -> ListRef<DataGlueScriptDagEdgeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dag_edge", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dag_node` after provisioning.\n"]
    pub fn dag_node(&self) -> ListRef<DataGlueScriptDagNodeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dag_node", self.extract_ref()))
    }
}

impl Referable for DataGlueScript {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataGlueScript { }

impl ToListMappable for DataGlueScript {
    type O = ListRef<DataGlueScriptRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataGlueScript_ {
    fn extract_datasource_type(&self) -> String {
        "aws_glue_script".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataGlueScript {
    pub tf_id: String,
}

impl BuildDataGlueScript {
    pub fn build(self, stack: &mut Stack) -> DataGlueScript {
        let out = DataGlueScript(Rc::new(DataGlueScript_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataGlueScriptData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                language: core::default::Default::default(),
                region: core::default::Default::default(),
                dag_edge: core::default::Default::default(),
                dag_node: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataGlueScriptRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataGlueScriptRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataGlueScriptRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `language` after provisioning.\n"]
    pub fn language(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.language", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `python_script` after provisioning.\n"]
    pub fn python_script(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.python_script", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scala_code` after provisioning.\n"]
    pub fn scala_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scala_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dag_edge` after provisioning.\n"]
    pub fn dag_edge(&self) -> ListRef<DataGlueScriptDagEdgeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dag_edge", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dag_node` after provisioning.\n"]
    pub fn dag_node(&self) -> ListRef<DataGlueScriptDagNodeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dag_node", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataGlueScriptDagEdgeEl {
    source: PrimField<String>,
    target: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_parameter: Option<PrimField<String>>,
}

impl DataGlueScriptDagEdgeEl {
    #[doc = "Set the field `target_parameter`.\n"]
    pub fn set_target_parameter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_parameter = Some(v.into());
        self
    }
}

impl ToListMappable for DataGlueScriptDagEdgeEl {
    type O = BlockAssignable<DataGlueScriptDagEdgeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueScriptDagEdgeEl {
    #[doc = ""]
    pub source: PrimField<String>,
    #[doc = ""]
    pub target: PrimField<String>,
}

impl BuildDataGlueScriptDagEdgeEl {
    pub fn build(self) -> DataGlueScriptDagEdgeEl {
        DataGlueScriptDagEdgeEl {
            source: self.source,
            target: self.target,
            target_parameter: core::default::Default::default(),
        }
    }
}

pub struct DataGlueScriptDagEdgeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataGlueScriptDagEdgeElRef {
    fn new(shared: StackShared, base: String) -> DataGlueScriptDagEdgeElRef {
        DataGlueScriptDagEdgeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueScriptDagEdgeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc = "Get a reference to the value of field `target_parameter` after provisioning.\n"]
    pub fn target_parameter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_parameter", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGlueScriptDagNodeElArgsEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    param: Option<PrimField<bool>>,
    value: PrimField<String>,
}

impl DataGlueScriptDagNodeElArgsEl {
    #[doc = "Set the field `param`.\n"]
    pub fn set_param(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.param = Some(v.into());
        self
    }
}

impl ToListMappable for DataGlueScriptDagNodeElArgsEl {
    type O = BlockAssignable<DataGlueScriptDagNodeElArgsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueScriptDagNodeElArgsEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildDataGlueScriptDagNodeElArgsEl {
    pub fn build(self) -> DataGlueScriptDagNodeElArgsEl {
        DataGlueScriptDagNodeElArgsEl {
            name: self.name,
            param: core::default::Default::default(),
            value: self.value,
        }
    }
}

pub struct DataGlueScriptDagNodeElArgsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataGlueScriptDagNodeElArgsElRef {
    fn new(shared: StackShared, base: String) -> DataGlueScriptDagNodeElArgsElRef {
        DataGlueScriptDagNodeElArgsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueScriptDagNodeElArgsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `param` after provisioning.\n"]
    pub fn param(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.param", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataGlueScriptDagNodeElDynamic {
    args: Option<DynamicBlock<DataGlueScriptDagNodeElArgsEl>>,
}

#[derive(Serialize)]
pub struct DataGlueScriptDagNodeEl {
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_number: Option<PrimField<f64>>,
    node_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<Vec<DataGlueScriptDagNodeElArgsEl>>,
    dynamic: DataGlueScriptDagNodeElDynamic,
}

impl DataGlueScriptDagNodeEl {
    #[doc = "Set the field `line_number`.\n"]
    pub fn set_line_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.line_number = Some(v.into());
        self
    }

    #[doc = "Set the field `args`.\n"]
    pub fn set_args(mut self, v: impl Into<BlockAssignable<DataGlueScriptDagNodeElArgsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.args = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.args = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DataGlueScriptDagNodeEl {
    type O = BlockAssignable<DataGlueScriptDagNodeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueScriptDagNodeEl {
    #[doc = ""]
    pub id: PrimField<String>,
    #[doc = ""]
    pub node_type: PrimField<String>,
}

impl BuildDataGlueScriptDagNodeEl {
    pub fn build(self) -> DataGlueScriptDagNodeEl {
        DataGlueScriptDagNodeEl {
            id: self.id,
            line_number: core::default::Default::default(),
            node_type: self.node_type,
            args: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DataGlueScriptDagNodeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataGlueScriptDagNodeElRef {
    fn new(shared: StackShared, base: String) -> DataGlueScriptDagNodeElRef {
        DataGlueScriptDagNodeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueScriptDagNodeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `line_number` after provisioning.\n"]
    pub fn line_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.line_number", self.base))
    }

    #[doc = "Get a reference to the value of field `node_type` after provisioning.\n"]
    pub fn node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.base))
    }

    #[doc = "Get a reference to the value of field `args` after provisioning.\n"]
    pub fn args(&self) -> ListRef<DataGlueScriptDagNodeElArgsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataGlueScriptDynamic {
    dag_edge: Option<DynamicBlock<DataGlueScriptDagEdgeEl>>,
    dag_node: Option<DynamicBlock<DataGlueScriptDagNodeEl>>,
}
