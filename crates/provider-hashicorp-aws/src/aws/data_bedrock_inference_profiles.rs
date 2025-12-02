use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataBedrockInferenceProfilesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
struct DataBedrockInferenceProfiles_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBedrockInferenceProfilesData>,
}
#[derive(Clone)]
pub struct DataBedrockInferenceProfiles(Rc<DataBedrockInferenceProfiles_>);
impl DataBedrockInferenceProfiles {
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
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `inference_profile_summaries` after provisioning.\n"]
    pub fn inference_profile_summaries(
        &self,
    ) -> ListRef<DataBedrockInferenceProfilesInferenceProfileSummariesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inference_profile_summaries", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
}
impl Referable for DataBedrockInferenceProfiles {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataBedrockInferenceProfiles {}
impl ToListMappable for DataBedrockInferenceProfiles {
    type O = ListRef<DataBedrockInferenceProfilesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataBedrockInferenceProfiles_ {
    fn extract_datasource_type(&self) -> String {
        "aws_bedrock_inference_profiles".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataBedrockInferenceProfiles {
    pub tf_id: String,
}
impl BuildDataBedrockInferenceProfiles {
    pub fn build(self, stack: &mut Stack) -> DataBedrockInferenceProfiles {
        let out = DataBedrockInferenceProfiles(Rc::new(DataBedrockInferenceProfiles_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBedrockInferenceProfilesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                region: core::default::Default::default(),
                type_: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataBedrockInferenceProfilesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBedrockInferenceProfilesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataBedrockInferenceProfilesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `inference_profile_summaries` after provisioning.\n"]
    pub fn inference_profile_summaries(
        &self,
    ) -> ListRef<DataBedrockInferenceProfilesInferenceProfileSummariesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inference_profile_summaries", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataBedrockInferenceProfilesInferenceProfileSummariesElModelsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    model_arn: Option<PrimField<String>>,
}
impl DataBedrockInferenceProfilesInferenceProfileSummariesElModelsEl {
    #[doc = "Set the field `model_arn`.\n"]
    pub fn set_model_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model_arn = Some(v.into());
        self
    }
}
impl ToListMappable for DataBedrockInferenceProfilesInferenceProfileSummariesElModelsEl {
    type O = BlockAssignable<DataBedrockInferenceProfilesInferenceProfileSummariesElModelsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBedrockInferenceProfilesInferenceProfileSummariesElModelsEl {}
impl BuildDataBedrockInferenceProfilesInferenceProfileSummariesElModelsEl {
    pub fn build(self) -> DataBedrockInferenceProfilesInferenceProfileSummariesElModelsEl {
        DataBedrockInferenceProfilesInferenceProfileSummariesElModelsEl {
            model_arn: core::default::Default::default(),
        }
    }
}
pub struct DataBedrockInferenceProfilesInferenceProfileSummariesElModelsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBedrockInferenceProfilesInferenceProfileSummariesElModelsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBedrockInferenceProfilesInferenceProfileSummariesElModelsElRef {
        DataBedrockInferenceProfilesInferenceProfileSummariesElModelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBedrockInferenceProfilesInferenceProfileSummariesElModelsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `model_arn` after provisioning.\n"]
    pub fn model_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBedrockInferenceProfilesInferenceProfileSummariesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inference_profile_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inference_profile_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inference_profile_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    models: Option<ListField<DataBedrockInferenceProfilesInferenceProfileSummariesElModelsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<PrimField<String>>,
}
impl DataBedrockInferenceProfilesInferenceProfileSummariesEl {
    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_at = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `inference_profile_arn`.\n"]
    pub fn set_inference_profile_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.inference_profile_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `inference_profile_id`.\n"]
    pub fn set_inference_profile_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.inference_profile_id = Some(v.into());
        self
    }
    #[doc = "Set the field `inference_profile_name`.\n"]
    pub fn set_inference_profile_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.inference_profile_name = Some(v.into());
        self
    }
    #[doc = "Set the field `models`.\n"]
    pub fn set_models(
        mut self,
        v: impl Into<ListField<DataBedrockInferenceProfilesInferenceProfileSummariesElModelsEl>>,
    ) -> Self {
        self.models = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.updated_at = Some(v.into());
        self
    }
}
impl ToListMappable for DataBedrockInferenceProfilesInferenceProfileSummariesEl {
    type O = BlockAssignable<DataBedrockInferenceProfilesInferenceProfileSummariesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBedrockInferenceProfilesInferenceProfileSummariesEl {}
impl BuildDataBedrockInferenceProfilesInferenceProfileSummariesEl {
    pub fn build(self) -> DataBedrockInferenceProfilesInferenceProfileSummariesEl {
        DataBedrockInferenceProfilesInferenceProfileSummariesEl {
            created_at: core::default::Default::default(),
            description: core::default::Default::default(),
            inference_profile_arn: core::default::Default::default(),
            inference_profile_id: core::default::Default::default(),
            inference_profile_name: core::default::Default::default(),
            models: core::default::Default::default(),
            status: core::default::Default::default(),
            type_: core::default::Default::default(),
            updated_at: core::default::Default::default(),
        }
    }
}
pub struct DataBedrockInferenceProfilesInferenceProfileSummariesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBedrockInferenceProfilesInferenceProfileSummariesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBedrockInferenceProfilesInferenceProfileSummariesElRef {
        DataBedrockInferenceProfilesInferenceProfileSummariesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBedrockInferenceProfilesInferenceProfileSummariesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `inference_profile_arn` after provisioning.\n"]
    pub fn inference_profile_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inference_profile_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `inference_profile_id` after provisioning.\n"]
    pub fn inference_profile_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inference_profile_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `inference_profile_name` after provisioning.\n"]
    pub fn inference_profile_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inference_profile_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `models` after provisioning.\n"]
    pub fn models(
        &self,
    ) -> ListRef<DataBedrockInferenceProfilesInferenceProfileSummariesElModelsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.models", self.base))
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }
}
