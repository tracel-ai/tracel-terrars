use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataBedrockInferenceProfileData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    inference_profile_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataBedrockInferenceProfile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBedrockInferenceProfileData>,
}

#[derive(Clone)]
pub struct DataBedrockInferenceProfile(Rc<DataBedrockInferenceProfile_>);

impl DataBedrockInferenceProfile {
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

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `inference_profile_arn` after provisioning.\n"]
    pub fn inference_profile_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inference_profile_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `inference_profile_id` after provisioning.\n"]
    pub fn inference_profile_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inference_profile_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `inference_profile_name` after provisioning.\n"]
    pub fn inference_profile_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inference_profile_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `models` after provisioning.\n"]
    pub fn models(&self) -> ListRef<DataBedrockInferenceProfileModelsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.models", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.updated_at", self.extract_ref()),
        )
    }
}

impl Referable for DataBedrockInferenceProfile {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataBedrockInferenceProfile {}

impl ToListMappable for DataBedrockInferenceProfile {
    type O = ListRef<DataBedrockInferenceProfileRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataBedrockInferenceProfile_ {
    fn extract_datasource_type(&self) -> String {
        "aws_bedrock_inference_profile".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBedrockInferenceProfile {
    pub tf_id: String,
    #[doc = ""]
    pub inference_profile_id: PrimField<String>,
}

impl BuildDataBedrockInferenceProfile {
    pub fn build(self, stack: &mut Stack) -> DataBedrockInferenceProfile {
        let out = DataBedrockInferenceProfile(Rc::new(DataBedrockInferenceProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBedrockInferenceProfileData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                inference_profile_id: self.inference_profile_id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBedrockInferenceProfileRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBedrockInferenceProfileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataBedrockInferenceProfileRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `inference_profile_arn` after provisioning.\n"]
    pub fn inference_profile_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inference_profile_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `inference_profile_id` after provisioning.\n"]
    pub fn inference_profile_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inference_profile_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `inference_profile_name` after provisioning.\n"]
    pub fn inference_profile_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inference_profile_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `models` after provisioning.\n"]
    pub fn models(&self) -> ListRef<DataBedrockInferenceProfileModelsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.models", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.updated_at", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataBedrockInferenceProfileModelsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    model_arn: Option<PrimField<String>>,
}

impl DataBedrockInferenceProfileModelsEl {
    #[doc = "Set the field `model_arn`.\n"]
    pub fn set_model_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model_arn = Some(v.into());
        self
    }
}

impl ToListMappable for DataBedrockInferenceProfileModelsEl {
    type O = BlockAssignable<DataBedrockInferenceProfileModelsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockInferenceProfileModelsEl {}

impl BuildDataBedrockInferenceProfileModelsEl {
    pub fn build(self) -> DataBedrockInferenceProfileModelsEl {
        DataBedrockInferenceProfileModelsEl {
            model_arn: core::default::Default::default(),
        }
    }
}

pub struct DataBedrockInferenceProfileModelsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBedrockInferenceProfileModelsElRef {
    fn new(shared: StackShared, base: String) -> DataBedrockInferenceProfileModelsElRef {
        DataBedrockInferenceProfileModelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockInferenceProfileModelsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `model_arn` after provisioning.\n"]
    pub fn model_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_arn", self.base))
    }
}
