use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockagentKnowledgeBaseData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    knowledge_base_configuration: Option<Vec<BedrockagentKnowledgeBaseKnowledgeBaseConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_configuration: Option<Vec<BedrockagentKnowledgeBaseStorageConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BedrockagentKnowledgeBaseTimeoutsEl>,
    dynamic: BedrockagentKnowledgeBaseDynamic,
}

struct BedrockagentKnowledgeBase_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BedrockagentKnowledgeBaseData>,
}

#[derive(Clone)]
pub struct BedrockagentKnowledgeBase(Rc<BedrockagentKnowledgeBase_>);

impl BedrockagentKnowledgeBase {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `knowledge_base_configuration`.\n"]
    pub fn set_knowledge_base_configuration(
        self,
        v: impl Into<BlockAssignable<BedrockagentKnowledgeBaseKnowledgeBaseConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().knowledge_base_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.knowledge_base_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage_configuration`.\n"]
    pub fn set_storage_configuration(
        self,
        v: impl Into<BlockAssignable<BedrockagentKnowledgeBaseStorageConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().storage_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.storage_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BedrockagentKnowledgeBaseTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failure_reasons` after provisioning.\n"]
    pub fn failure_reasons(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.failure_reasons", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `knowledge_base_configuration` after provisioning.\n"]
    pub fn knowledge_base_configuration(&self) -> ListRef<BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.knowledge_base_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_configuration` after provisioning.\n"]
    pub fn storage_configuration(&self) -> ListRef<BedrockagentKnowledgeBaseStorageConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.storage_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentKnowledgeBaseTimeoutsElRef {
        BedrockagentKnowledgeBaseTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for BedrockagentKnowledgeBase {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for BedrockagentKnowledgeBase { }

impl ToListMappable for BedrockagentKnowledgeBase {
    type O = ListRef<BedrockagentKnowledgeBaseRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BedrockagentKnowledgeBase_ {
    fn extract_resource_type(&self) -> String {
        "aws_bedrockagent_knowledge_base".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentKnowledgeBase {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}

impl BuildBedrockagentKnowledgeBase {
    pub fn build(self, stack: &mut Stack) -> BedrockagentKnowledgeBase {
        let out = BedrockagentKnowledgeBase(Rc::new(BedrockagentKnowledgeBase_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BedrockagentKnowledgeBaseData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                knowledge_base_configuration: core::default::Default::default(),
                storage_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentKnowledgeBaseRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentKnowledgeBaseRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl BedrockagentKnowledgeBaseRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failure_reasons` after provisioning.\n"]
    pub fn failure_reasons(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.failure_reasons", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `knowledge_base_configuration` after provisioning.\n"]
    pub fn knowledge_base_configuration(&self) -> ListRef<BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.knowledge_base_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_configuration` after provisioning.\n"]
    pub fn storage_configuration(&self) -> ListRef<BedrockagentKnowledgeBaseStorageConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.storage_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentKnowledgeBaseTimeoutsElRef {
        BedrockagentKnowledgeBaseTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationElBedrockEmbeddingModelConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    embedding_data_type: Option<PrimField<String>>,
}

impl BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationElBedrockEmbeddingModelConfigurationEl {
    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.dimensions = Some(v.into());
        self
    }

    #[doc = "Set the field `embedding_data_type`.\n"]
    pub fn set_embedding_data_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.embedding_data_type = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationElBedrockEmbeddingModelConfigurationEl {
    type O =
        BlockAssignable<
            BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationElBedrockEmbeddingModelConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationElBedrockEmbeddingModelConfigurationEl {}

impl BuildBedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationElBedrockEmbeddingModelConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationElBedrockEmbeddingModelConfigurationEl {
        BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationElBedrockEmbeddingModelConfigurationEl {
            dimensions: core::default::Default::default(),
            embedding_data_type: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationElBedrockEmbeddingModelConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationElBedrockEmbeddingModelConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationElBedrockEmbeddingModelConfigurationElRef {
        BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationElBedrockEmbeddingModelConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationElBedrockEmbeddingModelConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }

    #[doc = "Get a reference to the value of field `embedding_data_type` after provisioning.\n"]
    pub fn embedding_data_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.embedding_data_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationElDynamic {
    bedrock_embedding_model_configuration: Option<
        DynamicBlock<
            BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationElBedrockEmbeddingModelConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bedrock_embedding_model_configuration: Option<
        Vec<
            BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationElBedrockEmbeddingModelConfigurationEl,
        >,
    >,
    dynamic: BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationElDynamic,
}

impl BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationEl {
    #[doc = "Set the field `bedrock_embedding_model_configuration`.\n"]
    pub fn set_bedrock_embedding_model_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationElBedrockEmbeddingModelConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.bedrock_embedding_model_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.bedrock_embedding_model_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationEl {
    type O =
        BlockAssignable<
            BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationEl {}

impl BuildBedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationEl {
        BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationEl {
            bedrock_embedding_model_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationElRef {
        BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bedrock_embedding_model_configuration` after provisioning.\n"]
    pub fn bedrock_embedding_model_configuration(
        &self,
    ) -> ListRef<
        BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationElBedrockEmbeddingModelConfigurationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.bedrock_embedding_model_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationElS3LocationEl {
    uri: PrimField<String>,
}

impl BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationElS3LocationEl {

}

impl ToListMappable for BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationElS3LocationEl {
    type O =
        BlockAssignable<
            BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationElS3LocationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationElS3LocationEl {
    #[doc = ""]
    pub uri: PrimField<String>,
}

impl BuildBedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationElS3LocationEl {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationElS3LocationEl {
        BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationElS3LocationEl {
            uri: self.uri,
        }
    }
}

pub struct BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationElS3LocationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationElS3LocationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationElS3LocationElRef {
        BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationElS3LocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationElS3LocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationElDynamic {
    s3_location: Option<
        DynamicBlock<
            BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationElS3LocationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_location: Option<
        Vec<
            BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationElS3LocationEl,
        >,
    >,
    dynamic: BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationElDynamic,
}

impl BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationEl {
    #[doc = "Set the field `s3_location`.\n"]
    pub fn set_s3_location(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationElS3LocationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_location = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_location = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationEl {
    type O =
        BlockAssignable<
            BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildBedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationEl {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationEl {
        BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationEl {
            type_: self.type_,
            s3_location: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationElRef {
        BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_location` after provisioning.\n"]
    pub fn s3_location(
        &self,
    ) -> ListRef<
        BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationElS3LocationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.s3_location", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElDynamic {
    storage_location: Option<
        DynamicBlock<
            BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_location: Option<
        Vec<
            BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationEl,
        >,
    >,
    dynamic: BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElDynamic,
}

impl BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationEl {
    #[doc = "Set the field `storage_location`.\n"]
    pub fn set_storage_location(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.storage_location = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_location = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationEl {
    type O =
        BlockAssignable<
            BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationEl {}

impl BuildBedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationEl {
        BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationEl {
            storage_location: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElRef {
        BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `storage_location` after provisioning.\n"]
    pub fn storage_location(
        &self,
    ) -> ListRef<
        BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElStorageLocationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.storage_location", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElDynamic {
    embedding_model_configuration: Option<
        DynamicBlock<
            BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationEl,
        >,
    >,
    supplemental_data_storage_configuration: Option<
        DynamicBlock<
            BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationEl {
    embedding_model_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    embedding_model_configuration: Option<
        Vec<
            BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    supplemental_data_storage_configuration: Option<
        Vec<
            BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationEl,
        >,
    >,
    dynamic: BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElDynamic,
}

impl BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationEl {
    #[doc = "Set the field `embedding_model_configuration`.\n"]
    pub fn set_embedding_model_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.embedding_model_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.embedding_model_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `supplemental_data_storage_configuration`.\n"]
    pub fn set_supplemental_data_storage_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.supplemental_data_storage_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.supplemental_data_storage_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationEl {
    type O =
        BlockAssignable<BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationEl {
    #[doc = ""]
    pub embedding_model_arn: PrimField<String>,
}

impl BuildBedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationEl {
    pub fn build(self) -> BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationEl {
        BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationEl {
            embedding_model_arn: self.embedding_model_arn,
            embedding_model_configuration: core::default::Default::default(),
            supplemental_data_storage_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElRef {
        BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `embedding_model_arn` after provisioning.\n"]
    pub fn embedding_model_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.embedding_model_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `embedding_model_configuration` after provisioning.\n"]
    pub fn embedding_model_configuration(
        &self,
    ) -> ListRef<
        BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElEmbeddingModelConfigurationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.embedding_model_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `supplemental_data_storage_configuration` after provisioning.\n"]
    pub fn supplemental_data_storage_configuration(
        &self,
    ) -> ListRef<
        BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElSupplementalDataStorageConfigurationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.supplemental_data_storage_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElDynamic {
    vector_knowledge_base_configuration: Option<
        DynamicBlock<BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBaseKnowledgeBaseConfigurationEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vector_knowledge_base_configuration: Option<
        Vec<BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationEl>,
    >,
    dynamic: BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElDynamic,
}

impl BedrockagentKnowledgeBaseKnowledgeBaseConfigurationEl {
    #[doc = "Set the field `vector_knowledge_base_configuration`.\n"]
    pub fn set_vector_knowledge_base_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vector_knowledge_base_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vector_knowledge_base_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentKnowledgeBaseKnowledgeBaseConfigurationEl {
    type O = BlockAssignable<BedrockagentKnowledgeBaseKnowledgeBaseConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBaseKnowledgeBaseConfigurationEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildBedrockagentKnowledgeBaseKnowledgeBaseConfigurationEl {
    pub fn build(self) -> BedrockagentKnowledgeBaseKnowledgeBaseConfigurationEl {
        BedrockagentKnowledgeBaseKnowledgeBaseConfigurationEl {
            type_: self.type_,
            vector_knowledge_base_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElRef {
        BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `vector_knowledge_base_configuration` after provisioning.\n"]
    pub fn vector_knowledge_base_configuration(
        &self,
    ) -> ListRef<BedrockagentKnowledgeBaseKnowledgeBaseConfigurationElVectorKnowledgeBaseConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vector_knowledge_base_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationElFieldMappingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_field: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_field: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vector_field: Option<PrimField<String>>,
}

impl BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationElFieldMappingEl {
    #[doc = "Set the field `metadata_field`.\n"]
    pub fn set_metadata_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metadata_field = Some(v.into());
        self
    }

    #[doc = "Set the field `text_field`.\n"]
    pub fn set_text_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.text_field = Some(v.into());
        self
    }

    #[doc = "Set the field `vector_field`.\n"]
    pub fn set_vector_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vector_field = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationElFieldMappingEl {
    type O =
        BlockAssignable<
            BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationElFieldMappingEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationElFieldMappingEl {}

impl BuildBedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationElFieldMappingEl {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationElFieldMappingEl {
        BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationElFieldMappingEl {
            metadata_field: core::default::Default::default(),
            text_field: core::default::Default::default(),
            vector_field: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationElFieldMappingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationElFieldMappingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationElFieldMappingElRef {
        BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationElFieldMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationElFieldMappingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metadata_field` after provisioning.\n"]
    pub fn metadata_field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata_field", self.base))
    }

    #[doc = "Get a reference to the value of field `text_field` after provisioning.\n"]
    pub fn text_field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text_field", self.base))
    }

    #[doc = "Get a reference to the value of field `vector_field` after provisioning.\n"]
    pub fn vector_field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vector_field", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationElDynamic {
    field_mapping: Option<
        DynamicBlock<BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationElFieldMappingEl>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationEl {
    collection_arn: PrimField<String>,
    vector_index_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_mapping: Option<
        Vec<BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationElFieldMappingEl>,
    >,
    dynamic: BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationElDynamic,
}

impl BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationEl {
    #[doc = "Set the field `field_mapping`.\n"]
    pub fn set_field_mapping(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationElFieldMappingEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.field_mapping = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.field_mapping = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationEl {
    type O = BlockAssignable<BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationEl {
    #[doc = ""]
    pub collection_arn: PrimField<String>,
    #[doc = ""]
    pub vector_index_name: PrimField<String>,
}

impl BuildBedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationEl {
    pub fn build(self) -> BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationEl {
        BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationEl {
            collection_arn: self.collection_arn,
            vector_index_name: self.vector_index_name,
            field_mapping: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationElRef {
        BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `collection_arn` after provisioning.\n"]
    pub fn collection_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.collection_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `vector_index_name` after provisioning.\n"]
    pub fn vector_index_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vector_index_name", self.base))
    }

    #[doc = "Get a reference to the value of field `field_mapping` after provisioning.\n"]
    pub fn field_mapping(
        &self,
    ) -> ListRef<BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationElFieldMappingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.field_mapping", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationElFieldMappingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_field: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_field: Option<PrimField<String>>,
}

impl BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationElFieldMappingEl {
    #[doc = "Set the field `metadata_field`.\n"]
    pub fn set_metadata_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metadata_field = Some(v.into());
        self
    }

    #[doc = "Set the field `text_field`.\n"]
    pub fn set_text_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.text_field = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationElFieldMappingEl {
    type O = BlockAssignable<BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationElFieldMappingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationElFieldMappingEl {}

impl BuildBedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationElFieldMappingEl {
    pub fn build(self) -> BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationElFieldMappingEl {
        BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationElFieldMappingEl {
            metadata_field: core::default::Default::default(),
            text_field: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationElFieldMappingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationElFieldMappingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationElFieldMappingElRef {
        BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationElFieldMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationElFieldMappingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metadata_field` after provisioning.\n"]
    pub fn metadata_field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata_field", self.base))
    }

    #[doc = "Get a reference to the value of field `text_field` after provisioning.\n"]
    pub fn text_field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text_field", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationElDynamic {
    field_mapping: Option<
        DynamicBlock<BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationElFieldMappingEl>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationEl {
    connection_string: PrimField<String>,
    credentials_secret_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_mapping: Option<Vec<BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationElFieldMappingEl>>,
    dynamic: BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationElDynamic,
}

impl BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationEl {
    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespace = Some(v.into());
        self
    }

    #[doc = "Set the field `field_mapping`.\n"]
    pub fn set_field_mapping(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationElFieldMappingEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.field_mapping = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.field_mapping = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationEl {
    type O = BlockAssignable<BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationEl {
    #[doc = ""]
    pub connection_string: PrimField<String>,
    #[doc = ""]
    pub credentials_secret_arn: PrimField<String>,
}

impl BuildBedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationEl {
    pub fn build(self) -> BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationEl {
        BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationEl {
            connection_string: self.connection_string,
            credentials_secret_arn: self.credentials_secret_arn,
            namespace: core::default::Default::default(),
            field_mapping: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationElRef {
        BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_string` after provisioning.\n"]
    pub fn connection_string(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connection_string", self.base))
    }

    #[doc = "Get a reference to the value of field `credentials_secret_arn` after provisioning.\n"]
    pub fn credentials_secret_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.credentials_secret_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `field_mapping` after provisioning.\n"]
    pub fn field_mapping(
        &self,
    ) -> ListRef<BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationElFieldMappingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.field_mapping", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationElFieldMappingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_metadata_field: Option<PrimField<String>>,
    metadata_field: PrimField<String>,
    primary_key_field: PrimField<String>,
    text_field: PrimField<String>,
    vector_field: PrimField<String>,
}

impl BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationElFieldMappingEl {
    #[doc = "Set the field `custom_metadata_field`.\n"]
    pub fn set_custom_metadata_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.custom_metadata_field = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationElFieldMappingEl {
    type O = BlockAssignable<BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationElFieldMappingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationElFieldMappingEl {
    #[doc = ""]
    pub metadata_field: PrimField<String>,
    #[doc = ""]
    pub primary_key_field: PrimField<String>,
    #[doc = ""]
    pub text_field: PrimField<String>,
    #[doc = ""]
    pub vector_field: PrimField<String>,
}

impl BuildBedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationElFieldMappingEl {
    pub fn build(self) -> BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationElFieldMappingEl {
        BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationElFieldMappingEl {
            custom_metadata_field: core::default::Default::default(),
            metadata_field: self.metadata_field,
            primary_key_field: self.primary_key_field,
            text_field: self.text_field,
            vector_field: self.vector_field,
        }
    }
}

pub struct BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationElFieldMappingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationElFieldMappingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationElFieldMappingElRef {
        BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationElFieldMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationElFieldMappingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_metadata_field` after provisioning.\n"]
    pub fn custom_metadata_field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_metadata_field", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata_field` after provisioning.\n"]
    pub fn metadata_field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata_field", self.base))
    }

    #[doc = "Get a reference to the value of field `primary_key_field` after provisioning.\n"]
    pub fn primary_key_field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.primary_key_field", self.base))
    }

    #[doc = "Get a reference to the value of field `text_field` after provisioning.\n"]
    pub fn text_field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text_field", self.base))
    }

    #[doc = "Get a reference to the value of field `vector_field` after provisioning.\n"]
    pub fn vector_field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vector_field", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationElDynamic {
    field_mapping: Option<
        DynamicBlock<BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationElFieldMappingEl>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationEl {
    credentials_secret_arn: PrimField<String>,
    database_name: PrimField<String>,
    resource_arn: PrimField<String>,
    table_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_mapping: Option<Vec<BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationElFieldMappingEl>>,
    dynamic: BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationElDynamic,
}

impl BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationEl {
    #[doc = "Set the field `field_mapping`.\n"]
    pub fn set_field_mapping(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationElFieldMappingEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.field_mapping = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.field_mapping = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationEl {
    type O = BlockAssignable<BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationEl {
    #[doc = ""]
    pub credentials_secret_arn: PrimField<String>,
    #[doc = ""]
    pub database_name: PrimField<String>,
    #[doc = ""]
    pub resource_arn: PrimField<String>,
    #[doc = ""]
    pub table_name: PrimField<String>,
}

impl BuildBedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationEl {
    pub fn build(self) -> BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationEl {
        BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationEl {
            credentials_secret_arn: self.credentials_secret_arn,
            database_name: self.database_name,
            resource_arn: self.resource_arn,
            table_name: self.table_name,
            field_mapping: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationElRef {
        BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `credentials_secret_arn` after provisioning.\n"]
    pub fn credentials_secret_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.credentials_secret_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }

    #[doc = "Get a reference to the value of field `field_mapping` after provisioning.\n"]
    pub fn field_mapping(
        &self,
    ) -> ListRef<BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationElFieldMappingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.field_mapping", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationElFieldMappingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_field: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_field: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vector_field: Option<PrimField<String>>,
}

impl BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationElFieldMappingEl {
    #[doc = "Set the field `metadata_field`.\n"]
    pub fn set_metadata_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metadata_field = Some(v.into());
        self
    }

    #[doc = "Set the field `text_field`.\n"]
    pub fn set_text_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.text_field = Some(v.into());
        self
    }

    #[doc = "Set the field `vector_field`.\n"]
    pub fn set_vector_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vector_field = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationElFieldMappingEl {
    type O =
        BlockAssignable<
            BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationElFieldMappingEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationElFieldMappingEl {}

impl BuildBedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationElFieldMappingEl {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationElFieldMappingEl {
        BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationElFieldMappingEl {
            metadata_field: core::default::Default::default(),
            text_field: core::default::Default::default(),
            vector_field: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationElFieldMappingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationElFieldMappingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationElFieldMappingElRef {
        BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationElFieldMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationElFieldMappingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metadata_field` after provisioning.\n"]
    pub fn metadata_field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata_field", self.base))
    }

    #[doc = "Get a reference to the value of field `text_field` after provisioning.\n"]
    pub fn text_field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text_field", self.base))
    }

    #[doc = "Get a reference to the value of field `vector_field` after provisioning.\n"]
    pub fn vector_field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vector_field", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationElDynamic {
    field_mapping: Option<
        DynamicBlock<BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationElFieldMappingEl>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationEl {
    credentials_secret_arn: PrimField<String>,
    endpoint: PrimField<String>,
    vector_index_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_mapping: Option<
        Vec<BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationElFieldMappingEl>,
    >,
    dynamic: BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationElDynamic,
}

impl BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationEl {
    #[doc = "Set the field `field_mapping`.\n"]
    pub fn set_field_mapping(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationElFieldMappingEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.field_mapping = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.field_mapping = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationEl {
    type O = BlockAssignable<BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationEl {
    #[doc = ""]
    pub credentials_secret_arn: PrimField<String>,
    #[doc = ""]
    pub endpoint: PrimField<String>,
    #[doc = ""]
    pub vector_index_name: PrimField<String>,
}

impl BuildBedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationEl {
    pub fn build(self) -> BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationEl {
        BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationEl {
            credentials_secret_arn: self.credentials_secret_arn,
            endpoint: self.endpoint,
            vector_index_name: self.vector_index_name,
            field_mapping: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationElRef {
        BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `credentials_secret_arn` after provisioning.\n"]
    pub fn credentials_secret_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.credentials_secret_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `vector_index_name` after provisioning.\n"]
    pub fn vector_index_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vector_index_name", self.base))
    }

    #[doc = "Get a reference to the value of field `field_mapping` after provisioning.\n"]
    pub fn field_mapping(
        &self,
    ) -> ListRef<BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationElFieldMappingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.field_mapping", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBaseStorageConfigurationElDynamic {
    opensearch_serverless_configuration: Option<
        DynamicBlock<BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationEl>,
    >,
    pinecone_configuration: Option<
        DynamicBlock<BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationEl>,
    >,
    rds_configuration: Option<DynamicBlock<BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationEl>>,
    redis_enterprise_cloud_configuration: Option<
        DynamicBlock<BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBaseStorageConfigurationEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opensearch_serverless_configuration: Option<
        Vec<BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pinecone_configuration: Option<Vec<BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rds_configuration: Option<Vec<BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redis_enterprise_cloud_configuration: Option<
        Vec<BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationEl>,
    >,
    dynamic: BedrockagentKnowledgeBaseStorageConfigurationElDynamic,
}

impl BedrockagentKnowledgeBaseStorageConfigurationEl {
    #[doc = "Set the field `opensearch_serverless_configuration`.\n"]
    pub fn set_opensearch_serverless_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.opensearch_serverless_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.opensearch_serverless_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pinecone_configuration`.\n"]
    pub fn set_pinecone_configuration(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pinecone_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pinecone_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rds_configuration`.\n"]
    pub fn set_rds_configuration(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rds_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rds_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redis_enterprise_cloud_configuration`.\n"]
    pub fn set_redis_enterprise_cloud_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.redis_enterprise_cloud_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.redis_enterprise_cloud_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentKnowledgeBaseStorageConfigurationEl {
    type O = BlockAssignable<BedrockagentKnowledgeBaseStorageConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBaseStorageConfigurationEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildBedrockagentKnowledgeBaseStorageConfigurationEl {
    pub fn build(self) -> BedrockagentKnowledgeBaseStorageConfigurationEl {
        BedrockagentKnowledgeBaseStorageConfigurationEl {
            type_: self.type_,
            opensearch_serverless_configuration: core::default::Default::default(),
            pinecone_configuration: core::default::Default::default(),
            rds_configuration: core::default::Default::default(),
            redis_enterprise_cloud_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentKnowledgeBaseStorageConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentKnowledgeBaseStorageConfigurationElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentKnowledgeBaseStorageConfigurationElRef {
        BedrockagentKnowledgeBaseStorageConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBaseStorageConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `opensearch_serverless_configuration` after provisioning.\n"]
    pub fn opensearch_serverless_configuration(
        &self,
    ) -> ListRef<BedrockagentKnowledgeBaseStorageConfigurationElOpensearchServerlessConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.opensearch_serverless_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `pinecone_configuration` after provisioning.\n"]
    pub fn pinecone_configuration(
        &self,
    ) -> ListRef<BedrockagentKnowledgeBaseStorageConfigurationElPineconeConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pinecone_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `rds_configuration` after provisioning.\n"]
    pub fn rds_configuration(&self) -> ListRef<BedrockagentKnowledgeBaseStorageConfigurationElRdsConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rds_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `redis_enterprise_cloud_configuration` after provisioning.\n"]
    pub fn redis_enterprise_cloud_configuration(
        &self,
    ) -> ListRef<BedrockagentKnowledgeBaseStorageConfigurationElRedisEnterpriseCloudConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.redis_enterprise_cloud_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBaseTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl BedrockagentKnowledgeBaseTimeoutsEl {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentKnowledgeBaseTimeoutsEl {
    type O = BlockAssignable<BedrockagentKnowledgeBaseTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBaseTimeoutsEl {}

impl BuildBedrockagentKnowledgeBaseTimeoutsEl {
    pub fn build(self) -> BedrockagentKnowledgeBaseTimeoutsEl {
        BedrockagentKnowledgeBaseTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentKnowledgeBaseTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentKnowledgeBaseTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentKnowledgeBaseTimeoutsElRef {
        BedrockagentKnowledgeBaseTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBaseTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBaseDynamic {
    knowledge_base_configuration: Option<DynamicBlock<BedrockagentKnowledgeBaseKnowledgeBaseConfigurationEl>>,
    storage_configuration: Option<DynamicBlock<BedrockagentKnowledgeBaseStorageConfigurationEl>>,
}
