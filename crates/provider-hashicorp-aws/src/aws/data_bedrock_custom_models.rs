use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataBedrockCustomModelsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataBedrockCustomModels_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBedrockCustomModelsData>,
}
#[derive(Clone)]
pub struct DataBedrockCustomModels(Rc<DataBedrockCustomModels_>);
impl DataBedrockCustomModels {
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `model_summaries` after provisioning.\n"]
    pub fn model_summaries(&self) -> ListRef<DataBedrockCustomModelsModelSummariesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_summaries", self.extract_ref()),
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
impl Referable for DataBedrockCustomModels {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataBedrockCustomModels {}
impl ToListMappable for DataBedrockCustomModels {
    type O = ListRef<DataBedrockCustomModelsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataBedrockCustomModels_ {
    fn extract_datasource_type(&self) -> String {
        "aws_bedrock_custom_models".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataBedrockCustomModels {
    pub tf_id: String,
}
impl BuildDataBedrockCustomModels {
    pub fn build(self, stack: &mut Stack) -> DataBedrockCustomModels {
        let out = DataBedrockCustomModels(Rc::new(DataBedrockCustomModels_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBedrockCustomModelsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataBedrockCustomModelsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBedrockCustomModelsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataBedrockCustomModelsRef {
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
    #[doc = "Get a reference to the value of field `model_summaries` after provisioning.\n"]
    pub fn model_summaries(&self) -> ListRef<DataBedrockCustomModelsModelSummariesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_summaries", self.extract_ref()),
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
pub struct DataBedrockCustomModelsModelSummariesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    creation_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_name: Option<PrimField<String>>,
}
impl DataBedrockCustomModelsModelSummariesEl {
    #[doc = "Set the field `creation_time`.\n"]
    pub fn set_creation_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.creation_time = Some(v.into());
        self
    }
    #[doc = "Set the field `model_arn`.\n"]
    pub fn set_model_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `model_name`.\n"]
    pub fn set_model_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataBedrockCustomModelsModelSummariesEl {
    type O = BlockAssignable<DataBedrockCustomModelsModelSummariesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBedrockCustomModelsModelSummariesEl {}
impl BuildDataBedrockCustomModelsModelSummariesEl {
    pub fn build(self) -> DataBedrockCustomModelsModelSummariesEl {
        DataBedrockCustomModelsModelSummariesEl {
            creation_time: core::default::Default::default(),
            model_arn: core::default::Default::default(),
            model_name: core::default::Default::default(),
        }
    }
}
pub struct DataBedrockCustomModelsModelSummariesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBedrockCustomModelsModelSummariesElRef {
    fn new(shared: StackShared, base: String) -> DataBedrockCustomModelsModelSummariesElRef {
        DataBedrockCustomModelsModelSummariesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBedrockCustomModelsModelSummariesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `model_arn` after provisioning.\n"]
    pub fn model_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `model_name` after provisioning.\n"]
    pub fn model_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_name", self.base))
    }
}
