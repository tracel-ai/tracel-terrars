use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataBedrockFoundationModelsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    by_customization_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    by_inference_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    by_output_modality: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    by_provider: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataBedrockFoundationModels_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBedrockFoundationModelsData>,
}

#[derive(Clone)]
pub struct DataBedrockFoundationModels(Rc<DataBedrockFoundationModels_>);

impl DataBedrockFoundationModels {
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

    #[doc = "Set the field `by_customization_type`.\n"]
    pub fn set_by_customization_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().by_customization_type = Some(v.into());
        self
    }

    #[doc = "Set the field `by_inference_type`.\n"]
    pub fn set_by_inference_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().by_inference_type = Some(v.into());
        self
    }

    #[doc = "Set the field `by_output_modality`.\n"]
    pub fn set_by_output_modality(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().by_output_modality = Some(v.into());
        self
    }

    #[doc = "Set the field `by_provider`.\n"]
    pub fn set_by_provider(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().by_provider = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `by_customization_type` after provisioning.\n"]
    pub fn by_customization_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.by_customization_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `by_inference_type` after provisioning.\n"]
    pub fn by_inference_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.by_inference_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `by_output_modality` after provisioning.\n"]
    pub fn by_output_modality(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.by_output_modality", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `by_provider` after provisioning.\n"]
    pub fn by_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.by_provider", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_summaries` after provisioning.\n"]
    pub fn model_summaries(&self) -> ListRef<DataBedrockFoundationModelsModelSummariesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.model_summaries", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl Referable for DataBedrockFoundationModels {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataBedrockFoundationModels { }

impl ToListMappable for DataBedrockFoundationModels {
    type O = ListRef<DataBedrockFoundationModelsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataBedrockFoundationModels_ {
    fn extract_datasource_type(&self) -> String {
        "aws_bedrock_foundation_models".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBedrockFoundationModels {
    pub tf_id: String,
}

impl BuildDataBedrockFoundationModels {
    pub fn build(self, stack: &mut Stack) -> DataBedrockFoundationModels {
        let out = DataBedrockFoundationModels(Rc::new(DataBedrockFoundationModels_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBedrockFoundationModelsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                by_customization_type: core::default::Default::default(),
                by_inference_type: core::default::Default::default(),
                by_output_modality: core::default::Default::default(),
                by_provider: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBedrockFoundationModelsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBedrockFoundationModelsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataBedrockFoundationModelsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `by_customization_type` after provisioning.\n"]
    pub fn by_customization_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.by_customization_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `by_inference_type` after provisioning.\n"]
    pub fn by_inference_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.by_inference_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `by_output_modality` after provisioning.\n"]
    pub fn by_output_modality(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.by_output_modality", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `by_provider` after provisioning.\n"]
    pub fn by_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.by_provider", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_summaries` after provisioning.\n"]
    pub fn model_summaries(&self) -> ListRef<DataBedrockFoundationModelsModelSummariesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.model_summaries", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBedrockFoundationModelsModelSummariesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    customizations_supported: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inference_types_supported: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_modalities: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_modalities: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_streaming_supported: Option<PrimField<bool>>,
}

impl DataBedrockFoundationModelsModelSummariesEl {
    #[doc = "Set the field `customizations_supported`.\n"]
    pub fn set_customizations_supported(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.customizations_supported = Some(v.into());
        self
    }

    #[doc = "Set the field `inference_types_supported`.\n"]
    pub fn set_inference_types_supported(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.inference_types_supported = Some(v.into());
        self
    }

    #[doc = "Set the field `input_modalities`.\n"]
    pub fn set_input_modalities(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.input_modalities = Some(v.into());
        self
    }

    #[doc = "Set the field `model_arn`.\n"]
    pub fn set_model_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `model_id`.\n"]
    pub fn set_model_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model_id = Some(v.into());
        self
    }

    #[doc = "Set the field `model_name`.\n"]
    pub fn set_model_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model_name = Some(v.into());
        self
    }

    #[doc = "Set the field `output_modalities`.\n"]
    pub fn set_output_modalities(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.output_modalities = Some(v.into());
        self
    }

    #[doc = "Set the field `provider_name`.\n"]
    pub fn set_provider_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.provider_name = Some(v.into());
        self
    }

    #[doc = "Set the field `response_streaming_supported`.\n"]
    pub fn set_response_streaming_supported(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.response_streaming_supported = Some(v.into());
        self
    }
}

impl ToListMappable for DataBedrockFoundationModelsModelSummariesEl {
    type O = BlockAssignable<DataBedrockFoundationModelsModelSummariesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockFoundationModelsModelSummariesEl {}

impl BuildDataBedrockFoundationModelsModelSummariesEl {
    pub fn build(self) -> DataBedrockFoundationModelsModelSummariesEl {
        DataBedrockFoundationModelsModelSummariesEl {
            customizations_supported: core::default::Default::default(),
            inference_types_supported: core::default::Default::default(),
            input_modalities: core::default::Default::default(),
            model_arn: core::default::Default::default(),
            model_id: core::default::Default::default(),
            model_name: core::default::Default::default(),
            output_modalities: core::default::Default::default(),
            provider_name: core::default::Default::default(),
            response_streaming_supported: core::default::Default::default(),
        }
    }
}

pub struct DataBedrockFoundationModelsModelSummariesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBedrockFoundationModelsModelSummariesElRef {
    fn new(shared: StackShared, base: String) -> DataBedrockFoundationModelsModelSummariesElRef {
        DataBedrockFoundationModelsModelSummariesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockFoundationModelsModelSummariesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `customizations_supported` after provisioning.\n"]
    pub fn customizations_supported(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.customizations_supported", self.base))
    }

    #[doc = "Get a reference to the value of field `inference_types_supported` after provisioning.\n"]
    pub fn inference_types_supported(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.inference_types_supported", self.base))
    }

    #[doc = "Get a reference to the value of field `input_modalities` after provisioning.\n"]
    pub fn input_modalities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.input_modalities", self.base))
    }

    #[doc = "Get a reference to the value of field `model_arn` after provisioning.\n"]
    pub fn model_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `model_id` after provisioning.\n"]
    pub fn model_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.base))
    }

    #[doc = "Get a reference to the value of field `model_name` after provisioning.\n"]
    pub fn model_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_name", self.base))
    }

    #[doc = "Get a reference to the value of field `output_modalities` after provisioning.\n"]
    pub fn output_modalities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.output_modalities", self.base))
    }

    #[doc = "Get a reference to the value of field `provider_name` after provisioning.\n"]
    pub fn provider_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provider_name", self.base))
    }

    #[doc = "Get a reference to the value of field `response_streaming_supported` after provisioning.\n"]
    pub fn response_streaming_supported(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.response_streaming_supported", self.base))
    }
}
