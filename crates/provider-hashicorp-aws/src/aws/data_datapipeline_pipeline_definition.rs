use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataDatapipelinePipelineDefinitionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    pipeline_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter_value: Option<Vec<DataDatapipelinePipelineDefinitionParameterValueEl>>,
    dynamic: DataDatapipelinePipelineDefinitionDynamic,
}
struct DataDatapipelinePipelineDefinition_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataDatapipelinePipelineDefinitionData>,
}
#[derive(Clone)]
pub struct DataDatapipelinePipelineDefinition(Rc<DataDatapipelinePipelineDefinition_>);
impl DataDatapipelinePipelineDefinition {
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `parameter_value`.\n"]
    pub fn set_parameter_value(
        self,
        v: impl Into<BlockAssignable<DataDatapipelinePipelineDefinitionParameterValueEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().parameter_value = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.parameter_value = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `parameter_object` after provisioning.\n"]
    pub fn parameter_object(
        &self,
    ) -> SetRef<DataDatapipelinePipelineDefinitionParameterObjectElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.parameter_object", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pipeline_id` after provisioning.\n"]
    pub fn pipeline_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pipeline_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pipeline_object` after provisioning.\n"]
    pub fn pipeline_object(&self) -> SetRef<DataDatapipelinePipelineDefinitionPipelineObjectElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.pipeline_object", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}
impl Referable for DataDatapipelinePipelineDefinition {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataDatapipelinePipelineDefinition {}
impl ToListMappable for DataDatapipelinePipelineDefinition {
    type O = ListRef<DataDatapipelinePipelineDefinitionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataDatapipelinePipelineDefinition_ {
    fn extract_datasource_type(&self) -> String {
        "aws_datapipeline_pipeline_definition".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataDatapipelinePipelineDefinition {
    pub tf_id: String,
    #[doc = ""]
    pub pipeline_id: PrimField<String>,
}
impl BuildDataDatapipelinePipelineDefinition {
    pub fn build(self, stack: &mut Stack) -> DataDatapipelinePipelineDefinition {
        let out =
            DataDatapipelinePipelineDefinition(Rc::new(DataDatapipelinePipelineDefinition_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataDatapipelinePipelineDefinitionData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    id: core::default::Default::default(),
                    pipeline_id: self.pipeline_id,
                    region: core::default::Default::default(),
                    parameter_value: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataDatapipelinePipelineDefinitionRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDatapipelinePipelineDefinitionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataDatapipelinePipelineDefinitionRef {
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
    #[doc = "Get a reference to the value of field `parameter_object` after provisioning.\n"]
    pub fn parameter_object(
        &self,
    ) -> SetRef<DataDatapipelinePipelineDefinitionParameterObjectElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.parameter_object", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pipeline_id` after provisioning.\n"]
    pub fn pipeline_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pipeline_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pipeline_object` after provisioning.\n"]
    pub fn pipeline_object(&self) -> SetRef<DataDatapipelinePipelineDefinitionPipelineObjectElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.pipeline_object", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataDatapipelinePipelineDefinitionParameterObjectElAttributeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_value: Option<PrimField<String>>,
}
impl DataDatapipelinePipelineDefinitionParameterObjectElAttributeEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `string_value`.\n"]
    pub fn set_string_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.string_value = Some(v.into());
        self
    }
}
impl ToListMappable for DataDatapipelinePipelineDefinitionParameterObjectElAttributeEl {
    type O = BlockAssignable<DataDatapipelinePipelineDefinitionParameterObjectElAttributeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDatapipelinePipelineDefinitionParameterObjectElAttributeEl {}
impl BuildDataDatapipelinePipelineDefinitionParameterObjectElAttributeEl {
    pub fn build(self) -> DataDatapipelinePipelineDefinitionParameterObjectElAttributeEl {
        DataDatapipelinePipelineDefinitionParameterObjectElAttributeEl {
            key: core::default::Default::default(),
            string_value: core::default::Default::default(),
        }
    }
}
pub struct DataDatapipelinePipelineDefinitionParameterObjectElAttributeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDatapipelinePipelineDefinitionParameterObjectElAttributeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataDatapipelinePipelineDefinitionParameterObjectElAttributeElRef {
        DataDatapipelinePipelineDefinitionParameterObjectElAttributeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDatapipelinePipelineDefinitionParameterObjectElAttributeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `string_value` after provisioning.\n"]
    pub fn string_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.string_value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataDatapipelinePipelineDefinitionParameterObjectEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute: Option<SetField<DataDatapipelinePipelineDefinitionParameterObjectElAttributeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}
impl DataDatapipelinePipelineDefinitionParameterObjectEl {
    #[doc = "Set the field `attribute`.\n"]
    pub fn set_attribute(
        mut self,
        v: impl Into<SetField<DataDatapipelinePipelineDefinitionParameterObjectElAttributeEl>>,
    ) -> Self {
        self.attribute = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
}
impl ToListMappable for DataDatapipelinePipelineDefinitionParameterObjectEl {
    type O = BlockAssignable<DataDatapipelinePipelineDefinitionParameterObjectEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDatapipelinePipelineDefinitionParameterObjectEl {}
impl BuildDataDatapipelinePipelineDefinitionParameterObjectEl {
    pub fn build(self) -> DataDatapipelinePipelineDefinitionParameterObjectEl {
        DataDatapipelinePipelineDefinitionParameterObjectEl {
            attribute: core::default::Default::default(),
            id: core::default::Default::default(),
        }
    }
}
pub struct DataDatapipelinePipelineDefinitionParameterObjectElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDatapipelinePipelineDefinitionParameterObjectElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataDatapipelinePipelineDefinitionParameterObjectElRef {
        DataDatapipelinePipelineDefinitionParameterObjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDatapipelinePipelineDefinitionParameterObjectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(
        &self,
    ) -> SetRef<DataDatapipelinePipelineDefinitionParameterObjectElAttributeElRef> {
        SetRef::new(self.shared().clone(), format!("{}.attribute", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}
#[derive(Serialize)]
pub struct DataDatapipelinePipelineDefinitionPipelineObjectElFieldEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ref_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_value: Option<PrimField<String>>,
}
impl DataDatapipelinePipelineDefinitionPipelineObjectElFieldEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `ref_value`.\n"]
    pub fn set_ref_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ref_value = Some(v.into());
        self
    }
    #[doc = "Set the field `string_value`.\n"]
    pub fn set_string_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.string_value = Some(v.into());
        self
    }
}
impl ToListMappable for DataDatapipelinePipelineDefinitionPipelineObjectElFieldEl {
    type O = BlockAssignable<DataDatapipelinePipelineDefinitionPipelineObjectElFieldEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDatapipelinePipelineDefinitionPipelineObjectElFieldEl {}
impl BuildDataDatapipelinePipelineDefinitionPipelineObjectElFieldEl {
    pub fn build(self) -> DataDatapipelinePipelineDefinitionPipelineObjectElFieldEl {
        DataDatapipelinePipelineDefinitionPipelineObjectElFieldEl {
            key: core::default::Default::default(),
            ref_value: core::default::Default::default(),
            string_value: core::default::Default::default(),
        }
    }
}
pub struct DataDatapipelinePipelineDefinitionPipelineObjectElFieldElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDatapipelinePipelineDefinitionPipelineObjectElFieldElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataDatapipelinePipelineDefinitionPipelineObjectElFieldElRef {
        DataDatapipelinePipelineDefinitionPipelineObjectElFieldElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDatapipelinePipelineDefinitionPipelineObjectElFieldElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `ref_value` after provisioning.\n"]
    pub fn ref_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ref_value", self.base))
    }
    #[doc = "Get a reference to the value of field `string_value` after provisioning.\n"]
    pub fn string_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.string_value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataDatapipelinePipelineDefinitionPipelineObjectEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<SetField<DataDatapipelinePipelineDefinitionPipelineObjectElFieldEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataDatapipelinePipelineDefinitionPipelineObjectEl {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(
        mut self,
        v: impl Into<SetField<DataDatapipelinePipelineDefinitionPipelineObjectElFieldEl>>,
    ) -> Self {
        self.field = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DataDatapipelinePipelineDefinitionPipelineObjectEl {
    type O = BlockAssignable<DataDatapipelinePipelineDefinitionPipelineObjectEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDatapipelinePipelineDefinitionPipelineObjectEl {}
impl BuildDataDatapipelinePipelineDefinitionPipelineObjectEl {
    pub fn build(self) -> DataDatapipelinePipelineDefinitionPipelineObjectEl {
        DataDatapipelinePipelineDefinitionPipelineObjectEl {
            field: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DataDatapipelinePipelineDefinitionPipelineObjectElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDatapipelinePipelineDefinitionPipelineObjectElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataDatapipelinePipelineDefinitionPipelineObjectElRef {
        DataDatapipelinePipelineDefinitionPipelineObjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDatapipelinePipelineDefinitionPipelineObjectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> SetRef<DataDatapipelinePipelineDefinitionPipelineObjectElFieldElRef> {
        SetRef::new(self.shared().clone(), format!("{}.field", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataDatapipelinePipelineDefinitionParameterValueEl {}
impl DataDatapipelinePipelineDefinitionParameterValueEl {}
impl ToListMappable for DataDatapipelinePipelineDefinitionParameterValueEl {
    type O = BlockAssignable<DataDatapipelinePipelineDefinitionParameterValueEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDatapipelinePipelineDefinitionParameterValueEl {}
impl BuildDataDatapipelinePipelineDefinitionParameterValueEl {
    pub fn build(self) -> DataDatapipelinePipelineDefinitionParameterValueEl {
        DataDatapipelinePipelineDefinitionParameterValueEl {}
    }
}
pub struct DataDatapipelinePipelineDefinitionParameterValueElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDatapipelinePipelineDefinitionParameterValueElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataDatapipelinePipelineDefinitionParameterValueElRef {
        DataDatapipelinePipelineDefinitionParameterValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDatapipelinePipelineDefinitionParameterValueElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `string_value` after provisioning.\n"]
    pub fn string_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.string_value", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataDatapipelinePipelineDefinitionDynamic {
    parameter_value: Option<DynamicBlock<DataDatapipelinePipelineDefinitionParameterValueEl>>,
}
