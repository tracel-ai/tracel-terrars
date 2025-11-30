use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DatapipelinePipelineDefinitionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    pipeline_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter_object: Option<Vec<DatapipelinePipelineDefinitionParameterObjectEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter_value: Option<Vec<DatapipelinePipelineDefinitionParameterValueEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pipeline_object: Option<Vec<DatapipelinePipelineDefinitionPipelineObjectEl>>,
    dynamic: DatapipelinePipelineDefinitionDynamic,
}

struct DatapipelinePipelineDefinition_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DatapipelinePipelineDefinitionData>,
}

#[derive(Clone)]
pub struct DatapipelinePipelineDefinition(Rc<DatapipelinePipelineDefinition_>);

impl DatapipelinePipelineDefinition {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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

    #[doc = "Set the field `parameter_object`.\n"]
    pub fn set_parameter_object(
        self,
        v: impl Into<BlockAssignable<DatapipelinePipelineDefinitionParameterObjectEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().parameter_object = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.parameter_object = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `parameter_value`.\n"]
    pub fn set_parameter_value(
        self,
        v: impl Into<BlockAssignable<DatapipelinePipelineDefinitionParameterValueEl>>,
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

    #[doc = "Set the field `pipeline_object`.\n"]
    pub fn set_pipeline_object(
        self,
        v: impl Into<BlockAssignable<DatapipelinePipelineDefinitionPipelineObjectEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pipeline_object = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pipeline_object = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_id` after provisioning.\n"]
    pub fn pipeline_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pipeline_id", self.extract_ref()),
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

impl Referable for DatapipelinePipelineDefinition {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for DatapipelinePipelineDefinition {}

impl ToListMappable for DatapipelinePipelineDefinition {
    type O = ListRef<DatapipelinePipelineDefinitionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DatapipelinePipelineDefinition_ {
    fn extract_resource_type(&self) -> String {
        "aws_datapipeline_pipeline_definition".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatapipelinePipelineDefinition {
    pub tf_id: String,
    #[doc = ""]
    pub pipeline_id: PrimField<String>,
}

impl BuildDatapipelinePipelineDefinition {
    pub fn build(self, stack: &mut Stack) -> DatapipelinePipelineDefinition {
        let out = DatapipelinePipelineDefinition(Rc::new(DatapipelinePipelineDefinition_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DatapipelinePipelineDefinitionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                pipeline_id: self.pipeline_id,
                region: core::default::Default::default(),
                parameter_object: core::default::Default::default(),
                parameter_value: core::default::Default::default(),
                pipeline_object: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatapipelinePipelineDefinitionRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatapipelinePipelineDefinitionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DatapipelinePipelineDefinitionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_id` after provisioning.\n"]
    pub fn pipeline_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pipeline_id", self.extract_ref()),
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
pub struct DatapipelinePipelineDefinitionParameterObjectElAttributeEl {
    key: PrimField<String>,
    string_value: PrimField<String>,
}

impl DatapipelinePipelineDefinitionParameterObjectElAttributeEl {}

impl ToListMappable for DatapipelinePipelineDefinitionParameterObjectElAttributeEl {
    type O = BlockAssignable<DatapipelinePipelineDefinitionParameterObjectElAttributeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatapipelinePipelineDefinitionParameterObjectElAttributeEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub string_value: PrimField<String>,
}

impl BuildDatapipelinePipelineDefinitionParameterObjectElAttributeEl {
    pub fn build(self) -> DatapipelinePipelineDefinitionParameterObjectElAttributeEl {
        DatapipelinePipelineDefinitionParameterObjectElAttributeEl {
            key: self.key,
            string_value: self.string_value,
        }
    }
}

pub struct DatapipelinePipelineDefinitionParameterObjectElAttributeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatapipelinePipelineDefinitionParameterObjectElAttributeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatapipelinePipelineDefinitionParameterObjectElAttributeElRef {
        DatapipelinePipelineDefinitionParameterObjectElAttributeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatapipelinePipelineDefinitionParameterObjectElAttributeElRef {
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

#[derive(Serialize, Default)]
struct DatapipelinePipelineDefinitionParameterObjectElDynamic {
    attribute: Option<DynamicBlock<DatapipelinePipelineDefinitionParameterObjectElAttributeEl>>,
}

#[derive(Serialize)]
pub struct DatapipelinePipelineDefinitionParameterObjectEl {
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute: Option<Vec<DatapipelinePipelineDefinitionParameterObjectElAttributeEl>>,
    dynamic: DatapipelinePipelineDefinitionParameterObjectElDynamic,
}

impl DatapipelinePipelineDefinitionParameterObjectEl {
    #[doc = "Set the field `attribute`.\n"]
    pub fn set_attribute(
        mut self,
        v: impl Into<BlockAssignable<DatapipelinePipelineDefinitionParameterObjectElAttributeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.attribute = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.attribute = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for DatapipelinePipelineDefinitionParameterObjectEl {
    type O = BlockAssignable<DatapipelinePipelineDefinitionParameterObjectEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatapipelinePipelineDefinitionParameterObjectEl {
    #[doc = ""]
    pub id: PrimField<String>,
}

impl BuildDatapipelinePipelineDefinitionParameterObjectEl {
    pub fn build(self) -> DatapipelinePipelineDefinitionParameterObjectEl {
        DatapipelinePipelineDefinitionParameterObjectEl {
            id: self.id,
            attribute: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DatapipelinePipelineDefinitionParameterObjectElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatapipelinePipelineDefinitionParameterObjectElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatapipelinePipelineDefinitionParameterObjectElRef {
        DatapipelinePipelineDefinitionParameterObjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatapipelinePipelineDefinitionParameterObjectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct DatapipelinePipelineDefinitionParameterValueEl {
    id: PrimField<String>,
    string_value: PrimField<String>,
}

impl DatapipelinePipelineDefinitionParameterValueEl {}

impl ToListMappable for DatapipelinePipelineDefinitionParameterValueEl {
    type O = BlockAssignable<DatapipelinePipelineDefinitionParameterValueEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatapipelinePipelineDefinitionParameterValueEl {
    #[doc = ""]
    pub id: PrimField<String>,
    #[doc = ""]
    pub string_value: PrimField<String>,
}

impl BuildDatapipelinePipelineDefinitionParameterValueEl {
    pub fn build(self) -> DatapipelinePipelineDefinitionParameterValueEl {
        DatapipelinePipelineDefinitionParameterValueEl {
            id: self.id,
            string_value: self.string_value,
        }
    }
}

pub struct DatapipelinePipelineDefinitionParameterValueElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatapipelinePipelineDefinitionParameterValueElRef {
    fn new(shared: StackShared, base: String) -> DatapipelinePipelineDefinitionParameterValueElRef {
        DatapipelinePipelineDefinitionParameterValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatapipelinePipelineDefinitionParameterValueElRef {
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

#[derive(Serialize)]
pub struct DatapipelinePipelineDefinitionPipelineObjectElFieldEl {
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ref_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_value: Option<PrimField<String>>,
}

impl DatapipelinePipelineDefinitionPipelineObjectElFieldEl {
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

impl ToListMappable for DatapipelinePipelineDefinitionPipelineObjectElFieldEl {
    type O = BlockAssignable<DatapipelinePipelineDefinitionPipelineObjectElFieldEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatapipelinePipelineDefinitionPipelineObjectElFieldEl {
    #[doc = ""]
    pub key: PrimField<String>,
}

impl BuildDatapipelinePipelineDefinitionPipelineObjectElFieldEl {
    pub fn build(self) -> DatapipelinePipelineDefinitionPipelineObjectElFieldEl {
        DatapipelinePipelineDefinitionPipelineObjectElFieldEl {
            key: self.key,
            ref_value: core::default::Default::default(),
            string_value: core::default::Default::default(),
        }
    }
}

pub struct DatapipelinePipelineDefinitionPipelineObjectElFieldElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatapipelinePipelineDefinitionPipelineObjectElFieldElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatapipelinePipelineDefinitionPipelineObjectElFieldElRef {
        DatapipelinePipelineDefinitionPipelineObjectElFieldElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatapipelinePipelineDefinitionPipelineObjectElFieldElRef {
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

#[derive(Serialize, Default)]
struct DatapipelinePipelineDefinitionPipelineObjectElDynamic {
    field: Option<DynamicBlock<DatapipelinePipelineDefinitionPipelineObjectElFieldEl>>,
}

#[derive(Serialize)]
pub struct DatapipelinePipelineDefinitionPipelineObjectEl {
    id: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<Vec<DatapipelinePipelineDefinitionPipelineObjectElFieldEl>>,
    dynamic: DatapipelinePipelineDefinitionPipelineObjectElDynamic,
}

impl DatapipelinePipelineDefinitionPipelineObjectEl {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(
        mut self,
        v: impl Into<BlockAssignable<DatapipelinePipelineDefinitionPipelineObjectElFieldEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.field = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.field = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for DatapipelinePipelineDefinitionPipelineObjectEl {
    type O = BlockAssignable<DatapipelinePipelineDefinitionPipelineObjectEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatapipelinePipelineDefinitionPipelineObjectEl {
    #[doc = ""]
    pub id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildDatapipelinePipelineDefinitionPipelineObjectEl {
    pub fn build(self) -> DatapipelinePipelineDefinitionPipelineObjectEl {
        DatapipelinePipelineDefinitionPipelineObjectEl {
            id: self.id,
            name: self.name,
            field: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DatapipelinePipelineDefinitionPipelineObjectElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatapipelinePipelineDefinitionPipelineObjectElRef {
    fn new(shared: StackShared, base: String) -> DatapipelinePipelineDefinitionPipelineObjectElRef {
        DatapipelinePipelineDefinitionPipelineObjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatapipelinePipelineDefinitionPipelineObjectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
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

#[derive(Serialize, Default)]
struct DatapipelinePipelineDefinitionDynamic {
    parameter_object: Option<DynamicBlock<DatapipelinePipelineDefinitionParameterObjectEl>>,
    parameter_value: Option<DynamicBlock<DatapipelinePipelineDefinitionParameterValueEl>>,
    pipeline_object: Option<DynamicBlock<DatapipelinePipelineDefinitionPipelineObjectEl>>,
}
