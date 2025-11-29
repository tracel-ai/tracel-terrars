use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataBedrockCustomModelData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    model_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataBedrockCustomModel_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBedrockCustomModelData>,
}

#[derive(Clone)]
pub struct DataBedrockCustomModel(Rc<DataBedrockCustomModel_>);

impl DataBedrockCustomModel {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `base_model_arn` after provisioning.\n"]
    pub fn base_model_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.base_model_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hyperparameters` after provisioning.\n"]
    pub fn hyperparameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.hyperparameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_arn` after provisioning.\n"]
    pub fn job_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_name` after provisioning.\n"]
    pub fn job_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_tags` after provisioning.\n"]
    pub fn job_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.job_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_arn` after provisioning.\n"]
    pub fn model_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_id` after provisioning.\n"]
    pub fn model_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_kms_key_arn` after provisioning.\n"]
    pub fn model_kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_name` after provisioning.\n"]
    pub fn model_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_tags` after provisioning.\n"]
    pub fn model_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.model_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_data_config` after provisioning.\n"]
    pub fn output_data_config(&self) -> ListRef<DataBedrockCustomModelOutputDataConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.output_data_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `training_data_config` after provisioning.\n"]
    pub fn training_data_config(&self) -> ListRef<DataBedrockCustomModelTrainingDataConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.training_data_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `training_metrics` after provisioning.\n"]
    pub fn training_metrics(&self) -> ListRef<DataBedrockCustomModelTrainingMetricsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.training_metrics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validation_data_config` after provisioning.\n"]
    pub fn validation_data_config(&self) -> ListRef<DataBedrockCustomModelValidationDataConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.validation_data_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validation_metrics` after provisioning.\n"]
    pub fn validation_metrics(&self) -> ListRef<DataBedrockCustomModelValidationMetricsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.validation_metrics", self.extract_ref()))
    }
}

impl Referable for DataBedrockCustomModel {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataBedrockCustomModel { }

impl ToListMappable for DataBedrockCustomModel {
    type O = ListRef<DataBedrockCustomModelRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataBedrockCustomModel_ {
    fn extract_datasource_type(&self) -> String {
        "aws_bedrock_custom_model".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBedrockCustomModel {
    pub tf_id: String,
    #[doc = ""]
    pub model_id: PrimField<String>,
}

impl BuildDataBedrockCustomModel {
    pub fn build(self, stack: &mut Stack) -> DataBedrockCustomModel {
        let out = DataBedrockCustomModel(Rc::new(DataBedrockCustomModel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBedrockCustomModelData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                model_id: self.model_id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBedrockCustomModelRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBedrockCustomModelRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataBedrockCustomModelRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `base_model_arn` after provisioning.\n"]
    pub fn base_model_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.base_model_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hyperparameters` after provisioning.\n"]
    pub fn hyperparameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.hyperparameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_arn` after provisioning.\n"]
    pub fn job_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_name` after provisioning.\n"]
    pub fn job_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_tags` after provisioning.\n"]
    pub fn job_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.job_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_arn` after provisioning.\n"]
    pub fn model_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_id` after provisioning.\n"]
    pub fn model_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_kms_key_arn` after provisioning.\n"]
    pub fn model_kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_name` after provisioning.\n"]
    pub fn model_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_tags` after provisioning.\n"]
    pub fn model_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.model_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_data_config` after provisioning.\n"]
    pub fn output_data_config(&self) -> ListRef<DataBedrockCustomModelOutputDataConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.output_data_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `training_data_config` after provisioning.\n"]
    pub fn training_data_config(&self) -> ListRef<DataBedrockCustomModelTrainingDataConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.training_data_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `training_metrics` after provisioning.\n"]
    pub fn training_metrics(&self) -> ListRef<DataBedrockCustomModelTrainingMetricsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.training_metrics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validation_data_config` after provisioning.\n"]
    pub fn validation_data_config(&self) -> ListRef<DataBedrockCustomModelValidationDataConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.validation_data_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validation_metrics` after provisioning.\n"]
    pub fn validation_metrics(&self) -> ListRef<DataBedrockCustomModelValidationMetricsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.validation_metrics", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBedrockCustomModelOutputDataConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_uri: Option<PrimField<String>>,
}

impl DataBedrockCustomModelOutputDataConfigEl {
    #[doc = "Set the field `s3_uri`.\n"]
    pub fn set_s3_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_uri = Some(v.into());
        self
    }
}

impl ToListMappable for DataBedrockCustomModelOutputDataConfigEl {
    type O = BlockAssignable<DataBedrockCustomModelOutputDataConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockCustomModelOutputDataConfigEl {}

impl BuildDataBedrockCustomModelOutputDataConfigEl {
    pub fn build(self) -> DataBedrockCustomModelOutputDataConfigEl {
        DataBedrockCustomModelOutputDataConfigEl { s3_uri: core::default::Default::default() }
    }
}

pub struct DataBedrockCustomModelOutputDataConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBedrockCustomModelOutputDataConfigElRef {
    fn new(shared: StackShared, base: String) -> DataBedrockCustomModelOutputDataConfigElRef {
        DataBedrockCustomModelOutputDataConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockCustomModelOutputDataConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBedrockCustomModelTrainingDataConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_uri: Option<PrimField<String>>,
}

impl DataBedrockCustomModelTrainingDataConfigEl {
    #[doc = "Set the field `s3_uri`.\n"]
    pub fn set_s3_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_uri = Some(v.into());
        self
    }
}

impl ToListMappable for DataBedrockCustomModelTrainingDataConfigEl {
    type O = BlockAssignable<DataBedrockCustomModelTrainingDataConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockCustomModelTrainingDataConfigEl {}

impl BuildDataBedrockCustomModelTrainingDataConfigEl {
    pub fn build(self) -> DataBedrockCustomModelTrainingDataConfigEl {
        DataBedrockCustomModelTrainingDataConfigEl { s3_uri: core::default::Default::default() }
    }
}

pub struct DataBedrockCustomModelTrainingDataConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBedrockCustomModelTrainingDataConfigElRef {
    fn new(shared: StackShared, base: String) -> DataBedrockCustomModelTrainingDataConfigElRef {
        DataBedrockCustomModelTrainingDataConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockCustomModelTrainingDataConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBedrockCustomModelTrainingMetricsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    training_loss: Option<PrimField<f64>>,
}

impl DataBedrockCustomModelTrainingMetricsEl {
    #[doc = "Set the field `training_loss`.\n"]
    pub fn set_training_loss(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.training_loss = Some(v.into());
        self
    }
}

impl ToListMappable for DataBedrockCustomModelTrainingMetricsEl {
    type O = BlockAssignable<DataBedrockCustomModelTrainingMetricsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockCustomModelTrainingMetricsEl {}

impl BuildDataBedrockCustomModelTrainingMetricsEl {
    pub fn build(self) -> DataBedrockCustomModelTrainingMetricsEl {
        DataBedrockCustomModelTrainingMetricsEl { training_loss: core::default::Default::default() }
    }
}

pub struct DataBedrockCustomModelTrainingMetricsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBedrockCustomModelTrainingMetricsElRef {
    fn new(shared: StackShared, base: String) -> DataBedrockCustomModelTrainingMetricsElRef {
        DataBedrockCustomModelTrainingMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockCustomModelTrainingMetricsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `training_loss` after provisioning.\n"]
    pub fn training_loss(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.training_loss", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBedrockCustomModelValidationDataConfigElValidatorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_uri: Option<PrimField<String>>,
}

impl DataBedrockCustomModelValidationDataConfigElValidatorEl {
    #[doc = "Set the field `s3_uri`.\n"]
    pub fn set_s3_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_uri = Some(v.into());
        self
    }
}

impl ToListMappable for DataBedrockCustomModelValidationDataConfigElValidatorEl {
    type O = BlockAssignable<DataBedrockCustomModelValidationDataConfigElValidatorEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockCustomModelValidationDataConfigElValidatorEl {}

impl BuildDataBedrockCustomModelValidationDataConfigElValidatorEl {
    pub fn build(self) -> DataBedrockCustomModelValidationDataConfigElValidatorEl {
        DataBedrockCustomModelValidationDataConfigElValidatorEl { s3_uri: core::default::Default::default() }
    }
}

pub struct DataBedrockCustomModelValidationDataConfigElValidatorElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBedrockCustomModelValidationDataConfigElValidatorElRef {
    fn new(shared: StackShared, base: String) -> DataBedrockCustomModelValidationDataConfigElValidatorElRef {
        DataBedrockCustomModelValidationDataConfigElValidatorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockCustomModelValidationDataConfigElValidatorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBedrockCustomModelValidationDataConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    validator: Option<ListField<DataBedrockCustomModelValidationDataConfigElValidatorEl>>,
}

impl DataBedrockCustomModelValidationDataConfigEl {
    #[doc = "Set the field `validator`.\n"]
    pub fn set_validator(
        mut self,
        v: impl Into<ListField<DataBedrockCustomModelValidationDataConfigElValidatorEl>>,
    ) -> Self {
        self.validator = Some(v.into());
        self
    }
}

impl ToListMappable for DataBedrockCustomModelValidationDataConfigEl {
    type O = BlockAssignable<DataBedrockCustomModelValidationDataConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockCustomModelValidationDataConfigEl {}

impl BuildDataBedrockCustomModelValidationDataConfigEl {
    pub fn build(self) -> DataBedrockCustomModelValidationDataConfigEl {
        DataBedrockCustomModelValidationDataConfigEl { validator: core::default::Default::default() }
    }
}

pub struct DataBedrockCustomModelValidationDataConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBedrockCustomModelValidationDataConfigElRef {
    fn new(shared: StackShared, base: String) -> DataBedrockCustomModelValidationDataConfigElRef {
        DataBedrockCustomModelValidationDataConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockCustomModelValidationDataConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `validator` after provisioning.\n"]
    pub fn validator(&self) -> ListRef<DataBedrockCustomModelValidationDataConfigElValidatorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.validator", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBedrockCustomModelValidationMetricsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_loss: Option<PrimField<f64>>,
}

impl DataBedrockCustomModelValidationMetricsEl {
    #[doc = "Set the field `validation_loss`.\n"]
    pub fn set_validation_loss(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.validation_loss = Some(v.into());
        self
    }
}

impl ToListMappable for DataBedrockCustomModelValidationMetricsEl {
    type O = BlockAssignable<DataBedrockCustomModelValidationMetricsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockCustomModelValidationMetricsEl {}

impl BuildDataBedrockCustomModelValidationMetricsEl {
    pub fn build(self) -> DataBedrockCustomModelValidationMetricsEl {
        DataBedrockCustomModelValidationMetricsEl { validation_loss: core::default::Default::default() }
    }
}

pub struct DataBedrockCustomModelValidationMetricsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBedrockCustomModelValidationMetricsElRef {
    fn new(shared: StackShared, base: String) -> DataBedrockCustomModelValidationMetricsElRef {
        DataBedrockCustomModelValidationMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockCustomModelValidationMetricsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `validation_loss` after provisioning.\n"]
    pub fn validation_loss(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.validation_loss", self.base))
    }
}
