use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BedrockCustomModelData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    base_model_identifier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_model_kms_key_id: Option<PrimField<String>>,
    custom_model_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customization_type: Option<PrimField<String>>,
    hyperparameters: RecField<PrimField<String>>,
    job_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_data_config: Option<Vec<BedrockCustomModelOutputDataConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BedrockCustomModelTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    training_data_config: Option<Vec<BedrockCustomModelTrainingDataConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_data_config: Option<Vec<BedrockCustomModelValidationDataConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_config: Option<Vec<BedrockCustomModelVpcConfigEl>>,
    dynamic: BedrockCustomModelDynamic,
}
struct BedrockCustomModel_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BedrockCustomModelData>,
}
#[derive(Clone)]
pub struct BedrockCustomModel(Rc<BedrockCustomModel_>);
impl BedrockCustomModel {
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
    #[doc = "Set the field `custom_model_kms_key_id`.\n"]
    pub fn set_custom_model_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().custom_model_kms_key_id = Some(v.into());
        self
    }
    #[doc = "Set the field `customization_type`.\n"]
    pub fn set_customization_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().customization_type = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `output_data_config`.\n"]
    pub fn set_output_data_config(
        self,
        v: impl Into<BlockAssignable<BedrockCustomModelOutputDataConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().output_data_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.output_data_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BedrockCustomModelTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `training_data_config`.\n"]
    pub fn set_training_data_config(
        self,
        v: impl Into<BlockAssignable<BedrockCustomModelTrainingDataConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().training_data_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.training_data_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `validation_data_config`.\n"]
    pub fn set_validation_data_config(
        self,
        v: impl Into<BlockAssignable<BedrockCustomModelValidationDataConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().validation_data_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.validation_data_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        self,
        v: impl Into<BlockAssignable<BedrockCustomModelVpcConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_config = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `base_model_identifier` after provisioning.\n"]
    pub fn base_model_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_model_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_model_arn` after provisioning.\n"]
    pub fn custom_model_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_model_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_model_kms_key_id` after provisioning.\n"]
    pub fn custom_model_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_model_kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_model_name` after provisioning.\n"]
    pub fn custom_model_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_model_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customization_type` after provisioning.\n"]
    pub fn customization_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customization_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hyperparameters` after provisioning.\n"]
    pub fn hyperparameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.hyperparameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `job_arn` after provisioning.\n"]
    pub fn job_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `job_name` after provisioning.\n"]
    pub fn job_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `job_status` after provisioning.\n"]
    pub fn job_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `training_metrics` after provisioning.\n"]
    pub fn training_metrics(&self) -> ListRef<BedrockCustomModelTrainingMetricsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.training_metrics", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `validation_metrics` after provisioning.\n"]
    pub fn validation_metrics(&self) -> ListRef<BedrockCustomModelValidationMetricsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.validation_metrics", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `output_data_config` after provisioning.\n"]
    pub fn output_data_config(&self) -> ListRef<BedrockCustomModelOutputDataConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_data_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockCustomModelTimeoutsElRef {
        BedrockCustomModelTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `training_data_config` after provisioning.\n"]
    pub fn training_data_config(&self) -> ListRef<BedrockCustomModelTrainingDataConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.training_data_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `validation_data_config` after provisioning.\n"]
    pub fn validation_data_config(&self) -> ListRef<BedrockCustomModelValidationDataConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.validation_data_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<BedrockCustomModelVpcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_config", self.extract_ref()),
        )
    }
}
impl Referable for BedrockCustomModel {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BedrockCustomModel {}
impl ToListMappable for BedrockCustomModel {
    type O = ListRef<BedrockCustomModelRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BedrockCustomModel_ {
    fn extract_resource_type(&self) -> String {
        "aws_bedrock_custom_model".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBedrockCustomModel {
    pub tf_id: String,
    #[doc = ""]
    pub base_model_identifier: PrimField<String>,
    #[doc = ""]
    pub custom_model_name: PrimField<String>,
    #[doc = ""]
    pub hyperparameters: RecField<PrimField<String>>,
    #[doc = ""]
    pub job_name: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}
impl BuildBedrockCustomModel {
    pub fn build(self, stack: &mut Stack) -> BedrockCustomModel {
        let out = BedrockCustomModel(Rc::new(BedrockCustomModel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BedrockCustomModelData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                base_model_identifier: self.base_model_identifier,
                custom_model_kms_key_id: core::default::Default::default(),
                custom_model_name: self.custom_model_name,
                customization_type: core::default::Default::default(),
                hyperparameters: self.hyperparameters,
                job_name: self.job_name,
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                output_data_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                training_data_config: core::default::Default::default(),
                validation_data_config: core::default::Default::default(),
                vpc_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BedrockCustomModelRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockCustomModelRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BedrockCustomModelRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `base_model_identifier` after provisioning.\n"]
    pub fn base_model_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_model_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_model_arn` after provisioning.\n"]
    pub fn custom_model_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_model_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_model_kms_key_id` after provisioning.\n"]
    pub fn custom_model_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_model_kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_model_name` after provisioning.\n"]
    pub fn custom_model_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_model_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customization_type` after provisioning.\n"]
    pub fn customization_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customization_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hyperparameters` after provisioning.\n"]
    pub fn hyperparameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.hyperparameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `job_arn` after provisioning.\n"]
    pub fn job_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `job_name` after provisioning.\n"]
    pub fn job_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `job_status` after provisioning.\n"]
    pub fn job_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `training_metrics` after provisioning.\n"]
    pub fn training_metrics(&self) -> ListRef<BedrockCustomModelTrainingMetricsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.training_metrics", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `validation_metrics` after provisioning.\n"]
    pub fn validation_metrics(&self) -> ListRef<BedrockCustomModelValidationMetricsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.validation_metrics", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `output_data_config` after provisioning.\n"]
    pub fn output_data_config(&self) -> ListRef<BedrockCustomModelOutputDataConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_data_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockCustomModelTimeoutsElRef {
        BedrockCustomModelTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `training_data_config` after provisioning.\n"]
    pub fn training_data_config(&self) -> ListRef<BedrockCustomModelTrainingDataConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.training_data_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `validation_data_config` after provisioning.\n"]
    pub fn validation_data_config(&self) -> ListRef<BedrockCustomModelValidationDataConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.validation_data_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<BedrockCustomModelVpcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BedrockCustomModelTrainingMetricsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    training_loss: Option<PrimField<f64>>,
}
impl BedrockCustomModelTrainingMetricsEl {
    #[doc = "Set the field `training_loss`.\n"]
    pub fn set_training_loss(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.training_loss = Some(v.into());
        self
    }
}
impl ToListMappable for BedrockCustomModelTrainingMetricsEl {
    type O = BlockAssignable<BedrockCustomModelTrainingMetricsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockCustomModelTrainingMetricsEl {}
impl BuildBedrockCustomModelTrainingMetricsEl {
    pub fn build(self) -> BedrockCustomModelTrainingMetricsEl {
        BedrockCustomModelTrainingMetricsEl {
            training_loss: core::default::Default::default(),
        }
    }
}
pub struct BedrockCustomModelTrainingMetricsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockCustomModelTrainingMetricsElRef {
    fn new(shared: StackShared, base: String) -> BedrockCustomModelTrainingMetricsElRef {
        BedrockCustomModelTrainingMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockCustomModelTrainingMetricsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `training_loss` after provisioning.\n"]
    pub fn training_loss(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.training_loss", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BedrockCustomModelValidationMetricsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_loss: Option<PrimField<f64>>,
}
impl BedrockCustomModelValidationMetricsEl {
    #[doc = "Set the field `validation_loss`.\n"]
    pub fn set_validation_loss(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.validation_loss = Some(v.into());
        self
    }
}
impl ToListMappable for BedrockCustomModelValidationMetricsEl {
    type O = BlockAssignable<BedrockCustomModelValidationMetricsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockCustomModelValidationMetricsEl {}
impl BuildBedrockCustomModelValidationMetricsEl {
    pub fn build(self) -> BedrockCustomModelValidationMetricsEl {
        BedrockCustomModelValidationMetricsEl {
            validation_loss: core::default::Default::default(),
        }
    }
}
pub struct BedrockCustomModelValidationMetricsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockCustomModelValidationMetricsElRef {
    fn new(shared: StackShared, base: String) -> BedrockCustomModelValidationMetricsElRef {
        BedrockCustomModelValidationMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockCustomModelValidationMetricsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `validation_loss` after provisioning.\n"]
    pub fn validation_loss(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.validation_loss", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BedrockCustomModelOutputDataConfigEl {
    s3_uri: PrimField<String>,
}
impl BedrockCustomModelOutputDataConfigEl {}
impl ToListMappable for BedrockCustomModelOutputDataConfigEl {
    type O = BlockAssignable<BedrockCustomModelOutputDataConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockCustomModelOutputDataConfigEl {
    #[doc = ""]
    pub s3_uri: PrimField<String>,
}
impl BuildBedrockCustomModelOutputDataConfigEl {
    pub fn build(self) -> BedrockCustomModelOutputDataConfigEl {
        BedrockCustomModelOutputDataConfigEl {
            s3_uri: self.s3_uri,
        }
    }
}
pub struct BedrockCustomModelOutputDataConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockCustomModelOutputDataConfigElRef {
    fn new(shared: StackShared, base: String) -> BedrockCustomModelOutputDataConfigElRef {
        BedrockCustomModelOutputDataConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockCustomModelOutputDataConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
}
#[derive(Serialize)]
pub struct BedrockCustomModelTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl BedrockCustomModelTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
}
impl ToListMappable for BedrockCustomModelTimeoutsEl {
    type O = BlockAssignable<BedrockCustomModelTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockCustomModelTimeoutsEl {}
impl BuildBedrockCustomModelTimeoutsEl {
    pub fn build(self) -> BedrockCustomModelTimeoutsEl {
        BedrockCustomModelTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct BedrockCustomModelTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockCustomModelTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BedrockCustomModelTimeoutsElRef {
        BedrockCustomModelTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockCustomModelTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}
#[derive(Serialize)]
pub struct BedrockCustomModelTrainingDataConfigEl {
    s3_uri: PrimField<String>,
}
impl BedrockCustomModelTrainingDataConfigEl {}
impl ToListMappable for BedrockCustomModelTrainingDataConfigEl {
    type O = BlockAssignable<BedrockCustomModelTrainingDataConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockCustomModelTrainingDataConfigEl {
    #[doc = ""]
    pub s3_uri: PrimField<String>,
}
impl BuildBedrockCustomModelTrainingDataConfigEl {
    pub fn build(self) -> BedrockCustomModelTrainingDataConfigEl {
        BedrockCustomModelTrainingDataConfigEl {
            s3_uri: self.s3_uri,
        }
    }
}
pub struct BedrockCustomModelTrainingDataConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockCustomModelTrainingDataConfigElRef {
    fn new(shared: StackShared, base: String) -> BedrockCustomModelTrainingDataConfigElRef {
        BedrockCustomModelTrainingDataConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockCustomModelTrainingDataConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
}
#[derive(Serialize)]
pub struct BedrockCustomModelValidationDataConfigElValidatorEl {
    s3_uri: PrimField<String>,
}
impl BedrockCustomModelValidationDataConfigElValidatorEl {}
impl ToListMappable for BedrockCustomModelValidationDataConfigElValidatorEl {
    type O = BlockAssignable<BedrockCustomModelValidationDataConfigElValidatorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockCustomModelValidationDataConfigElValidatorEl {
    #[doc = ""]
    pub s3_uri: PrimField<String>,
}
impl BuildBedrockCustomModelValidationDataConfigElValidatorEl {
    pub fn build(self) -> BedrockCustomModelValidationDataConfigElValidatorEl {
        BedrockCustomModelValidationDataConfigElValidatorEl {
            s3_uri: self.s3_uri,
        }
    }
}
pub struct BedrockCustomModelValidationDataConfigElValidatorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockCustomModelValidationDataConfigElValidatorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockCustomModelValidationDataConfigElValidatorElRef {
        BedrockCustomModelValidationDataConfigElValidatorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockCustomModelValidationDataConfigElValidatorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct BedrockCustomModelValidationDataConfigElDynamic {
    validator: Option<DynamicBlock<BedrockCustomModelValidationDataConfigElValidatorEl>>,
}
#[derive(Serialize)]
pub struct BedrockCustomModelValidationDataConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    validator: Option<Vec<BedrockCustomModelValidationDataConfigElValidatorEl>>,
    dynamic: BedrockCustomModelValidationDataConfigElDynamic,
}
impl BedrockCustomModelValidationDataConfigEl {
    #[doc = "Set the field `validator`.\n"]
    pub fn set_validator(
        mut self,
        v: impl Into<BlockAssignable<BedrockCustomModelValidationDataConfigElValidatorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.validator = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.validator = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BedrockCustomModelValidationDataConfigEl {
    type O = BlockAssignable<BedrockCustomModelValidationDataConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockCustomModelValidationDataConfigEl {}
impl BuildBedrockCustomModelValidationDataConfigEl {
    pub fn build(self) -> BedrockCustomModelValidationDataConfigEl {
        BedrockCustomModelValidationDataConfigEl {
            validator: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BedrockCustomModelValidationDataConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockCustomModelValidationDataConfigElRef {
    fn new(shared: StackShared, base: String) -> BedrockCustomModelValidationDataConfigElRef {
        BedrockCustomModelValidationDataConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockCustomModelValidationDataConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `validator` after provisioning.\n"]
    pub fn validator(&self) -> ListRef<BedrockCustomModelValidationDataConfigElValidatorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.validator", self.base))
    }
}
#[derive(Serialize)]
pub struct BedrockCustomModelVpcConfigEl {
    security_group_ids: SetField<PrimField<String>>,
    subnet_ids: SetField<PrimField<String>>,
}
impl BedrockCustomModelVpcConfigEl {}
impl ToListMappable for BedrockCustomModelVpcConfigEl {
    type O = BlockAssignable<BedrockCustomModelVpcConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockCustomModelVpcConfigEl {
    #[doc = ""]
    pub security_group_ids: SetField<PrimField<String>>,
    #[doc = ""]
    pub subnet_ids: SetField<PrimField<String>>,
}
impl BuildBedrockCustomModelVpcConfigEl {
    pub fn build(self) -> BedrockCustomModelVpcConfigEl {
        BedrockCustomModelVpcConfigEl {
            security_group_ids: self.security_group_ids,
            subnet_ids: self.subnet_ids,
        }
    }
}
pub struct BedrockCustomModelVpcConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockCustomModelVpcConfigElRef {
    fn new(shared: StackShared, base: String) -> BedrockCustomModelVpcConfigElRef {
        BedrockCustomModelVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockCustomModelVpcConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}
#[derive(Serialize, Default)]
struct BedrockCustomModelDynamic {
    output_data_config: Option<DynamicBlock<BedrockCustomModelOutputDataConfigEl>>,
    training_data_config: Option<DynamicBlock<BedrockCustomModelTrainingDataConfigEl>>,
    validation_data_config: Option<DynamicBlock<BedrockCustomModelValidationDataConfigEl>>,
    vpc_config: Option<DynamicBlock<BedrockCustomModelVpcConfigEl>>,
}
