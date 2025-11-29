use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockagentPromptData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_encryption_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_variant: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variant: Option<Vec<BedrockagentPromptVariantEl>>,
    dynamic: BedrockagentPromptDynamic,
}

struct BedrockagentPrompt_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BedrockagentPromptData>,
}

#[derive(Clone)]
pub struct BedrockagentPrompt(Rc<BedrockagentPrompt_>);

impl BedrockagentPrompt {
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

    #[doc = "Set the field `customer_encryption_key_arn`.\n"]
    pub fn set_customer_encryption_key_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().customer_encryption_key_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `default_variant`.\n"]
    pub fn set_default_variant(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_variant = Some(v.into());
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

    #[doc = "Set the field `variant`.\n"]
    pub fn set_variant(self, v: impl Into<BlockAssignable<BedrockagentPromptVariantEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().variant = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.variant = Some(d);
            },
        }
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

    #[doc = "Get a reference to the value of field `customer_encryption_key_arn` after provisioning.\n"]
    pub fn customer_encryption_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_encryption_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_variant` after provisioning.\n"]
    pub fn default_variant(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_variant", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `variant` after provisioning.\n"]
    pub fn variant(&self) -> ListRef<BedrockagentPromptVariantElRef> {
        ListRef::new(self.shared().clone(), format!("{}.variant", self.extract_ref()))
    }
}

impl Referable for BedrockagentPrompt {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for BedrockagentPrompt { }

impl ToListMappable for BedrockagentPrompt {
    type O = ListRef<BedrockagentPromptRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BedrockagentPrompt_ {
    fn extract_resource_type(&self) -> String {
        "aws_bedrockagent_prompt".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentPrompt {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildBedrockagentPrompt {
    pub fn build(self, stack: &mut Stack) -> BedrockagentPrompt {
        let out = BedrockagentPrompt(Rc::new(BedrockagentPrompt_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BedrockagentPromptData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                customer_encryption_key_arn: core::default::Default::default(),
                default_variant: core::default::Default::default(),
                description: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                variant: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentPromptRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentPromptRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl BedrockagentPromptRef {
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

    #[doc = "Get a reference to the value of field `customer_encryption_key_arn` after provisioning.\n"]
    pub fn customer_encryption_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_encryption_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_variant` after provisioning.\n"]
    pub fn default_variant(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_variant", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `variant` after provisioning.\n"]
    pub fn variant(&self) -> ListRef<BedrockagentPromptVariantElRef> {
        ListRef::new(self.shared().clone(), format!("{}.variant", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BedrockagentPromptVariantElGenAiResourceElAgentEl {
    agent_identifier: PrimField<String>,
}

impl BedrockagentPromptVariantElGenAiResourceElAgentEl { }

impl ToListMappable for BedrockagentPromptVariantElGenAiResourceElAgentEl {
    type O = BlockAssignable<BedrockagentPromptVariantElGenAiResourceElAgentEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPromptVariantElGenAiResourceElAgentEl {
    #[doc = ""]
    pub agent_identifier: PrimField<String>,
}

impl BuildBedrockagentPromptVariantElGenAiResourceElAgentEl {
    pub fn build(self) -> BedrockagentPromptVariantElGenAiResourceElAgentEl {
        BedrockagentPromptVariantElGenAiResourceElAgentEl { agent_identifier: self.agent_identifier }
    }
}

pub struct BedrockagentPromptVariantElGenAiResourceElAgentElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentPromptVariantElGenAiResourceElAgentElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentPromptVariantElGenAiResourceElAgentElRef {
        BedrockagentPromptVariantElGenAiResourceElAgentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPromptVariantElGenAiResourceElAgentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_identifier` after provisioning.\n"]
    pub fn agent_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_identifier", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentPromptVariantElGenAiResourceElDynamic {
    agent: Option<DynamicBlock<BedrockagentPromptVariantElGenAiResourceElAgentEl>>,
}

#[derive(Serialize)]
pub struct BedrockagentPromptVariantElGenAiResourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    agent: Option<Vec<BedrockagentPromptVariantElGenAiResourceElAgentEl>>,
    dynamic: BedrockagentPromptVariantElGenAiResourceElDynamic,
}

impl BedrockagentPromptVariantElGenAiResourceEl {
    #[doc = "Set the field `agent`.\n"]
    pub fn set_agent(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentPromptVariantElGenAiResourceElAgentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.agent = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.agent = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentPromptVariantElGenAiResourceEl {
    type O = BlockAssignable<BedrockagentPromptVariantElGenAiResourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPromptVariantElGenAiResourceEl {}

impl BuildBedrockagentPromptVariantElGenAiResourceEl {
    pub fn build(self) -> BedrockagentPromptVariantElGenAiResourceEl {
        BedrockagentPromptVariantElGenAiResourceEl {
            agent: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentPromptVariantElGenAiResourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentPromptVariantElGenAiResourceElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentPromptVariantElGenAiResourceElRef {
        BedrockagentPromptVariantElGenAiResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPromptVariantElGenAiResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent` after provisioning.\n"]
    pub fn agent(&self) -> ListRef<BedrockagentPromptVariantElGenAiResourceElAgentElRef> {
        ListRef::new(self.shared().clone(), format!("{}.agent", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentPromptVariantElInferenceConfigurationElTextEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_sequences: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<PrimField<f64>>,
}

impl BedrockagentPromptVariantElInferenceConfigurationElTextEl {
    #[doc = "Set the field `max_tokens`.\n"]
    pub fn set_max_tokens(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_tokens = Some(v.into());
        self
    }

    #[doc = "Set the field `stop_sequences`.\n"]
    pub fn set_stop_sequences(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.stop_sequences = Some(v.into());
        self
    }

    #[doc = "Set the field `temperature`.\n"]
    pub fn set_temperature(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.temperature = Some(v.into());
        self
    }

    #[doc = "Set the field `top_p`.\n"]
    pub fn set_top_p(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.top_p = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentPromptVariantElInferenceConfigurationElTextEl {
    type O = BlockAssignable<BedrockagentPromptVariantElInferenceConfigurationElTextEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPromptVariantElInferenceConfigurationElTextEl {}

impl BuildBedrockagentPromptVariantElInferenceConfigurationElTextEl {
    pub fn build(self) -> BedrockagentPromptVariantElInferenceConfigurationElTextEl {
        BedrockagentPromptVariantElInferenceConfigurationElTextEl {
            max_tokens: core::default::Default::default(),
            stop_sequences: core::default::Default::default(),
            temperature: core::default::Default::default(),
            top_p: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentPromptVariantElInferenceConfigurationElTextElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentPromptVariantElInferenceConfigurationElTextElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentPromptVariantElInferenceConfigurationElTextElRef {
        BedrockagentPromptVariantElInferenceConfigurationElTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPromptVariantElInferenceConfigurationElTextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_tokens` after provisioning.\n"]
    pub fn max_tokens(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_tokens", self.base))
    }

    #[doc = "Get a reference to the value of field `stop_sequences` after provisioning.\n"]
    pub fn stop_sequences(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.stop_sequences", self.base))
    }

    #[doc = "Get a reference to the value of field `temperature` after provisioning.\n"]
    pub fn temperature(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }

    #[doc = "Get a reference to the value of field `top_p` after provisioning.\n"]
    pub fn top_p(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.top_p", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentPromptVariantElInferenceConfigurationElDynamic {
    text: Option<DynamicBlock<BedrockagentPromptVariantElInferenceConfigurationElTextEl>>,
}

#[derive(Serialize)]
pub struct BedrockagentPromptVariantElInferenceConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Vec<BedrockagentPromptVariantElInferenceConfigurationElTextEl>>,
    dynamic: BedrockagentPromptVariantElInferenceConfigurationElDynamic,
}

impl BedrockagentPromptVariantElInferenceConfigurationEl {
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentPromptVariantElInferenceConfigurationElTextEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.text = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.text = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentPromptVariantElInferenceConfigurationEl {
    type O = BlockAssignable<BedrockagentPromptVariantElInferenceConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPromptVariantElInferenceConfigurationEl {}

impl BuildBedrockagentPromptVariantElInferenceConfigurationEl {
    pub fn build(self) -> BedrockagentPromptVariantElInferenceConfigurationEl {
        BedrockagentPromptVariantElInferenceConfigurationEl {
            text: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentPromptVariantElInferenceConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentPromptVariantElInferenceConfigurationElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentPromptVariantElInferenceConfigurationElRef {
        BedrockagentPromptVariantElInferenceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPromptVariantElInferenceConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> ListRef<BedrockagentPromptVariantElInferenceConfigurationElTextElRef> {
        ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentPromptVariantElMetadataEl {
    key: PrimField<String>,
    value: PrimField<String>,
}

impl BedrockagentPromptVariantElMetadataEl { }

impl ToListMappable for BedrockagentPromptVariantElMetadataEl {
    type O = BlockAssignable<BedrockagentPromptVariantElMetadataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPromptVariantElMetadataEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildBedrockagentPromptVariantElMetadataEl {
    pub fn build(self) -> BedrockagentPromptVariantElMetadataEl {
        BedrockagentPromptVariantElMetadataEl {
            key: self.key,
            value: self.value,
        }
    }
}

pub struct BedrockagentPromptVariantElMetadataElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentPromptVariantElMetadataElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentPromptVariantElMetadataElRef {
        BedrockagentPromptVariantElMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPromptVariantElMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElInputVariableEl {
    name: PrimField<String>,
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElInputVariableEl { }

impl ToListMappable for BedrockagentPromptVariantElTemplateConfigurationElChatElInputVariableEl {
    type O = BlockAssignable<BedrockagentPromptVariantElTemplateConfigurationElChatElInputVariableEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPromptVariantElTemplateConfigurationElChatElInputVariableEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildBedrockagentPromptVariantElTemplateConfigurationElChatElInputVariableEl {
    pub fn build(self) -> BedrockagentPromptVariantElTemplateConfigurationElChatElInputVariableEl {
        BedrockagentPromptVariantElTemplateConfigurationElChatElInputVariableEl { name: self.name }
    }
}

pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElInputVariableElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentPromptVariantElTemplateConfigurationElChatElInputVariableElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentPromptVariantElTemplateConfigurationElChatElInputVariableElRef {
        BedrockagentPromptVariantElTemplateConfigurationElChatElInputVariableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElInputVariableElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentElCachePointEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentElCachePointEl { }

impl ToListMappable for BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentElCachePointEl {
    type O = BlockAssignable<BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentElCachePointEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentElCachePointEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildBedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentElCachePointEl {
    pub fn build(self) -> BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentElCachePointEl {
        BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentElCachePointEl { type_: self.type_ }
    }
}

pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentElCachePointElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentElCachePointElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentElCachePointElRef {
        BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentElCachePointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentElCachePointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentElDynamic {
    cache_point: Option<
        DynamicBlock<BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentElCachePointEl>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_point: Option<Vec<BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentElCachePointEl>>,
    dynamic: BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentElDynamic,
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentEl {
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.text = Some(v.into());
        self
    }

    #[doc = "Set the field `cache_point`.\n"]
    pub fn set_cache_point(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentElCachePointEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cache_point = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_point = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentEl {
    type O = BlockAssignable<BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentEl {}

impl BuildBedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentEl {
    pub fn build(self) -> BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentEl {
        BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentEl {
            text: core::default::Default::default(),
            cache_point: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentElRef {
        BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_point` after provisioning.\n"]
    pub fn cache_point(
        &self,
    ) -> ListRef<BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentElCachePointElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cache_point", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElDynamic {
    content: Option<DynamicBlock<BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentEl>>,
}

#[derive(Serialize)]
pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElMessageEl {
    role: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<Vec<BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentEl>>,
    dynamic: BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElDynamic,
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElMessageEl {
    #[doc = "Set the field `content`.\n"]
    pub fn set_content(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.content = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.content = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentPromptVariantElTemplateConfigurationElChatElMessageEl {
    type O = BlockAssignable<BedrockagentPromptVariantElTemplateConfigurationElChatElMessageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPromptVariantElTemplateConfigurationElChatElMessageEl {
    #[doc = ""]
    pub role: PrimField<String>,
}

impl BuildBedrockagentPromptVariantElTemplateConfigurationElChatElMessageEl {
    pub fn build(self) -> BedrockagentPromptVariantElTemplateConfigurationElChatElMessageEl {
        BedrockagentPromptVariantElTemplateConfigurationElChatElMessageEl {
            role: self.role,
            content: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElRef {
        BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> ListRef<BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElContentElRef> {
        ListRef::new(self.shared().clone(), format!("{}.content", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElSystemElCachePointEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElSystemElCachePointEl { }

impl ToListMappable for BedrockagentPromptVariantElTemplateConfigurationElChatElSystemElCachePointEl {
    type O = BlockAssignable<BedrockagentPromptVariantElTemplateConfigurationElChatElSystemElCachePointEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPromptVariantElTemplateConfigurationElChatElSystemElCachePointEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildBedrockagentPromptVariantElTemplateConfigurationElChatElSystemElCachePointEl {
    pub fn build(self) -> BedrockagentPromptVariantElTemplateConfigurationElChatElSystemElCachePointEl {
        BedrockagentPromptVariantElTemplateConfigurationElChatElSystemElCachePointEl { type_: self.type_ }
    }
}

pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElSystemElCachePointElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentPromptVariantElTemplateConfigurationElChatElSystemElCachePointElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentPromptVariantElTemplateConfigurationElChatElSystemElCachePointElRef {
        BedrockagentPromptVariantElTemplateConfigurationElChatElSystemElCachePointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElSystemElCachePointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentPromptVariantElTemplateConfigurationElChatElSystemElDynamic {
    cache_point: Option<DynamicBlock<BedrockagentPromptVariantElTemplateConfigurationElChatElSystemElCachePointEl>>,
}

#[derive(Serialize)]
pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElSystemEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_point: Option<Vec<BedrockagentPromptVariantElTemplateConfigurationElChatElSystemElCachePointEl>>,
    dynamic: BedrockagentPromptVariantElTemplateConfigurationElChatElSystemElDynamic,
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElSystemEl {
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.text = Some(v.into());
        self
    }

    #[doc = "Set the field `cache_point`.\n"]
    pub fn set_cache_point(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentPromptVariantElTemplateConfigurationElChatElSystemElCachePointEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cache_point = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_point = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentPromptVariantElTemplateConfigurationElChatElSystemEl {
    type O = BlockAssignable<BedrockagentPromptVariantElTemplateConfigurationElChatElSystemEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPromptVariantElTemplateConfigurationElChatElSystemEl {}

impl BuildBedrockagentPromptVariantElTemplateConfigurationElChatElSystemEl {
    pub fn build(self) -> BedrockagentPromptVariantElTemplateConfigurationElChatElSystemEl {
        BedrockagentPromptVariantElTemplateConfigurationElChatElSystemEl {
            text: core::default::Default::default(),
            cache_point: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElSystemElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentPromptVariantElTemplateConfigurationElChatElSystemElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentPromptVariantElTemplateConfigurationElChatElSystemElRef {
        BedrockagentPromptVariantElTemplateConfigurationElChatElSystemElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElSystemElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_point` after provisioning.\n"]
    pub fn cache_point(
        &self,
    ) -> ListRef<BedrockagentPromptVariantElTemplateConfigurationElChatElSystemElCachePointElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cache_point", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElCachePointEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElCachePointEl { }

impl ToListMappable for BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElCachePointEl {
    type O =
        BlockAssignable<
            BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElCachePointEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElCachePointEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildBedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElCachePointEl {
    pub fn build(
        self,
    ) -> BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElCachePointEl {
        BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElCachePointEl {
            type_: self.type_,
        }
    }
}

pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElCachePointElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElCachePointElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElCachePointElRef {
        BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElCachePointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElCachePointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    json: Option<PrimField<String>>,
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaEl {
    #[doc = "Set the field `json`.\n"]
    pub fn set_json(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.json = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaEl {
    type O =
        BlockAssignable<
            BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaEl {}

impl BuildBedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaEl {
    pub fn build(
        self,
    ) -> BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaEl {
        BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaEl {
            json: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaElRef {
        BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.json", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElDynamic {
    input_schema: Option<
        DynamicBlock<
            BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_schema: Option<
        Vec<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaEl>,
    >,
    dynamic: BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElDynamic,
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc = "Set the field `input_schema`.\n"]
    pub fn set_input_schema(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.input_schema = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.input_schema = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecEl {
    type O =
        BlockAssignable<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildBedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecEl {
    pub fn build(
        self,
    ) -> BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecEl {
        BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecEl {
            description: core::default::Default::default(),
            name: self.name,
            input_schema: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElRef {
        BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `input_schema` after provisioning.\n"]
    pub fn input_schema(
        &self,
    ) -> ListRef<
        BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.input_schema", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElDynamic {
    cache_point: Option<
        DynamicBlock<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElCachePointEl>,
    >,
    tool_spec: Option<
        DynamicBlock<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_point: Option<
        Vec<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElCachePointEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_spec: Option<Vec<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecEl>>,
    dynamic: BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElDynamic,
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolEl {
    #[doc = "Set the field `cache_point`.\n"]
    pub fn set_cache_point(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElCachePointEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cache_point = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_point = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tool_spec`.\n"]
    pub fn set_tool_spec(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tool_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tool_spec = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolEl {
    type O = BlockAssignable<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolEl {}

impl BuildBedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolEl {
    pub fn build(self) -> BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolEl {
        BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolEl {
            cache_point: core::default::Default::default(),
            tool_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElRef {
        BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cache_point` after provisioning.\n"]
    pub fn cache_point(
        &self,
    ) -> ListRef<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElCachePointElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cache_point", self.base))
    }

    #[doc = "Get a reference to the value of field `tool_spec` after provisioning.\n"]
    pub fn tool_spec(
        &self,
    ) -> ListRef<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tool_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyEl {}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyEl { }

impl ToListMappable for BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyEl {
    type O =
        BlockAssignable<
            BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyEl {}

impl BuildBedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyEl {
    pub fn build(
        self,
    ) -> BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyEl {
        BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyEl {}
    }
}

pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyElRef {
        BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoEl {}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoEl { }

impl ToListMappable for BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoEl {
    type O =
        BlockAssignable<
            BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoEl {}

impl BuildBedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoEl {
    pub fn build(
        self,
    ) -> BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoEl {
        BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoEl {}
    }
}

pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoElRef {
        BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolEl {
    name: PrimField<String>,
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolEl { }

impl ToListMappable for BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolEl {
    type O =
        BlockAssignable<
            BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildBedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolEl {
    pub fn build(
        self,
    ) -> BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolEl {
        BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolEl {
            name: self.name,
        }
    }
}

pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolElRef {
        BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElDynamic {
    any: Option<
        DynamicBlock<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyEl>,
    >,
    auto: Option<
        DynamicBlock<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoEl>,
    >,
    tool: Option<
        DynamicBlock<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolEl>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    any: Option<Vec<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto: Option<Vec<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool: Option<Vec<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolEl>>,
    dynamic: BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElDynamic,
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceEl {
    #[doc = "Set the field `any`.\n"]
    pub fn set_any(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.any = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.any = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `auto`.\n"]
    pub fn set_auto(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.auto = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.auto = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tool`.\n"]
    pub fn set_tool(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tool = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tool = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceEl {
    type O =
        BlockAssignable<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceEl {}

impl BuildBedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceEl {
    pub fn build(self) -> BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceEl {
        BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceEl {
            any: core::default::Default::default(),
            auto: core::default::Default::default(),
            tool: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElRef {
        BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `any` after provisioning.\n"]
    pub fn any(
        &self,
    ) -> ListRef<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.any", self.base))
    }

    #[doc = "Get a reference to the value of field `auto` after provisioning.\n"]
    pub fn auto(
        &self,
    ) -> ListRef<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto", self.base))
    }

    #[doc = "Get a reference to the value of field `tool` after provisioning.\n"]
    pub fn tool(
        &self,
    ) -> ListRef<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tool", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElDynamic {
    tool: Option<DynamicBlock<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolEl>>,
    tool_choice: Option<
        DynamicBlock<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceEl>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tool: Option<Vec<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<Vec<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceEl>>,
    dynamic: BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElDynamic,
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationEl {
    #[doc = "Set the field `tool`.\n"]
    pub fn set_tool(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tool = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tool = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tool_choice`.\n"]
    pub fn set_tool_choice(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tool_choice = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tool_choice = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationEl {
    type O = BlockAssignable<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationEl {}

impl BuildBedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationEl {
    pub fn build(self) -> BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationEl {
        BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationEl {
            tool: core::default::Default::default(),
            tool_choice: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElRef {
        BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tool` after provisioning.\n"]
    pub fn tool(&self) -> ListRef<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tool", self.base))
    }

    #[doc = "Get a reference to the value of field `tool_choice` after provisioning.\n"]
    pub fn tool_choice(
        &self,
    ) -> ListRef<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElToolChoiceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tool_choice", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentPromptVariantElTemplateConfigurationElChatElDynamic {
    input_variable: Option<DynamicBlock<BedrockagentPromptVariantElTemplateConfigurationElChatElInputVariableEl>>,
    message: Option<DynamicBlock<BedrockagentPromptVariantElTemplateConfigurationElChatElMessageEl>>,
    system: Option<DynamicBlock<BedrockagentPromptVariantElTemplateConfigurationElChatElSystemEl>>,
    tool_configuration: Option<
        DynamicBlock<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentPromptVariantElTemplateConfigurationElChatEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    input_variable: Option<Vec<BedrockagentPromptVariantElTemplateConfigurationElChatElInputVariableEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<Vec<BedrockagentPromptVariantElTemplateConfigurationElChatElMessageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<Vec<BedrockagentPromptVariantElTemplateConfigurationElChatElSystemEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_configuration: Option<Vec<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationEl>>,
    dynamic: BedrockagentPromptVariantElTemplateConfigurationElChatElDynamic,
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatEl {
    #[doc = "Set the field `input_variable`.\n"]
    pub fn set_input_variable(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentPromptVariantElTemplateConfigurationElChatElInputVariableEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.input_variable = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.input_variable = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentPromptVariantElTemplateConfigurationElChatElMessageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.message = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.message = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `system`.\n"]
    pub fn set_system(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentPromptVariantElTemplateConfigurationElChatElSystemEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.system = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.system = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tool_configuration`.\n"]
    pub fn set_tool_configuration(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tool_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tool_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentPromptVariantElTemplateConfigurationElChatEl {
    type O = BlockAssignable<BedrockagentPromptVariantElTemplateConfigurationElChatEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPromptVariantElTemplateConfigurationElChatEl {}

impl BuildBedrockagentPromptVariantElTemplateConfigurationElChatEl {
    pub fn build(self) -> BedrockagentPromptVariantElTemplateConfigurationElChatEl {
        BedrockagentPromptVariantElTemplateConfigurationElChatEl {
            input_variable: core::default::Default::default(),
            message: core::default::Default::default(),
            system: core::default::Default::default(),
            tool_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentPromptVariantElTemplateConfigurationElChatElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentPromptVariantElTemplateConfigurationElChatElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentPromptVariantElTemplateConfigurationElChatElRef {
        BedrockagentPromptVariantElTemplateConfigurationElChatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPromptVariantElTemplateConfigurationElChatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_variable` after provisioning.\n"]
    pub fn input_variable(&self) -> ListRef<BedrockagentPromptVariantElTemplateConfigurationElChatElInputVariableElRef> {
        ListRef::new(self.shared().clone(), format!("{}.input_variable", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> ListRef<BedrockagentPromptVariantElTemplateConfigurationElChatElMessageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `system` after provisioning.\n"]
    pub fn system(&self) -> ListRef<BedrockagentPromptVariantElTemplateConfigurationElChatElSystemElRef> {
        ListRef::new(self.shared().clone(), format!("{}.system", self.base))
    }

    #[doc = "Get a reference to the value of field `tool_configuration` after provisioning.\n"]
    pub fn tool_configuration(
        &self,
    ) -> ListRef<BedrockagentPromptVariantElTemplateConfigurationElChatElToolConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tool_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentPromptVariantElTemplateConfigurationElTextElCachePointEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl BedrockagentPromptVariantElTemplateConfigurationElTextElCachePointEl { }

impl ToListMappable for BedrockagentPromptVariantElTemplateConfigurationElTextElCachePointEl {
    type O = BlockAssignable<BedrockagentPromptVariantElTemplateConfigurationElTextElCachePointEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPromptVariantElTemplateConfigurationElTextElCachePointEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildBedrockagentPromptVariantElTemplateConfigurationElTextElCachePointEl {
    pub fn build(self) -> BedrockagentPromptVariantElTemplateConfigurationElTextElCachePointEl {
        BedrockagentPromptVariantElTemplateConfigurationElTextElCachePointEl { type_: self.type_ }
    }
}

pub struct BedrockagentPromptVariantElTemplateConfigurationElTextElCachePointElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentPromptVariantElTemplateConfigurationElTextElCachePointElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentPromptVariantElTemplateConfigurationElTextElCachePointElRef {
        BedrockagentPromptVariantElTemplateConfigurationElTextElCachePointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPromptVariantElTemplateConfigurationElTextElCachePointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentPromptVariantElTemplateConfigurationElTextElInputVariableEl {
    name: PrimField<String>,
}

impl BedrockagentPromptVariantElTemplateConfigurationElTextElInputVariableEl { }

impl ToListMappable for BedrockagentPromptVariantElTemplateConfigurationElTextElInputVariableEl {
    type O = BlockAssignable<BedrockagentPromptVariantElTemplateConfigurationElTextElInputVariableEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPromptVariantElTemplateConfigurationElTextElInputVariableEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildBedrockagentPromptVariantElTemplateConfigurationElTextElInputVariableEl {
    pub fn build(self) -> BedrockagentPromptVariantElTemplateConfigurationElTextElInputVariableEl {
        BedrockagentPromptVariantElTemplateConfigurationElTextElInputVariableEl { name: self.name }
    }
}

pub struct BedrockagentPromptVariantElTemplateConfigurationElTextElInputVariableElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentPromptVariantElTemplateConfigurationElTextElInputVariableElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentPromptVariantElTemplateConfigurationElTextElInputVariableElRef {
        BedrockagentPromptVariantElTemplateConfigurationElTextElInputVariableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPromptVariantElTemplateConfigurationElTextElInputVariableElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentPromptVariantElTemplateConfigurationElTextElDynamic {
    cache_point: Option<DynamicBlock<BedrockagentPromptVariantElTemplateConfigurationElTextElCachePointEl>>,
    input_variable: Option<DynamicBlock<BedrockagentPromptVariantElTemplateConfigurationElTextElInputVariableEl>>,
}

#[derive(Serialize)]
pub struct BedrockagentPromptVariantElTemplateConfigurationElTextEl {
    text: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_point: Option<Vec<BedrockagentPromptVariantElTemplateConfigurationElTextElCachePointEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_variable: Option<Vec<BedrockagentPromptVariantElTemplateConfigurationElTextElInputVariableEl>>,
    dynamic: BedrockagentPromptVariantElTemplateConfigurationElTextElDynamic,
}

impl BedrockagentPromptVariantElTemplateConfigurationElTextEl {
    #[doc = "Set the field `cache_point`.\n"]
    pub fn set_cache_point(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentPromptVariantElTemplateConfigurationElTextElCachePointEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cache_point = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_point = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `input_variable`.\n"]
    pub fn set_input_variable(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentPromptVariantElTemplateConfigurationElTextElInputVariableEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.input_variable = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.input_variable = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentPromptVariantElTemplateConfigurationElTextEl {
    type O = BlockAssignable<BedrockagentPromptVariantElTemplateConfigurationElTextEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPromptVariantElTemplateConfigurationElTextEl {
    #[doc = ""]
    pub text: PrimField<String>,
}

impl BuildBedrockagentPromptVariantElTemplateConfigurationElTextEl {
    pub fn build(self) -> BedrockagentPromptVariantElTemplateConfigurationElTextEl {
        BedrockagentPromptVariantElTemplateConfigurationElTextEl {
            text: self.text,
            cache_point: core::default::Default::default(),
            input_variable: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentPromptVariantElTemplateConfigurationElTextElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentPromptVariantElTemplateConfigurationElTextElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentPromptVariantElTemplateConfigurationElTextElRef {
        BedrockagentPromptVariantElTemplateConfigurationElTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPromptVariantElTemplateConfigurationElTextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_point` after provisioning.\n"]
    pub fn cache_point(&self) -> ListRef<BedrockagentPromptVariantElTemplateConfigurationElTextElCachePointElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cache_point", self.base))
    }

    #[doc = "Get a reference to the value of field `input_variable` after provisioning.\n"]
    pub fn input_variable(&self) -> ListRef<BedrockagentPromptVariantElTemplateConfigurationElTextElInputVariableElRef> {
        ListRef::new(self.shared().clone(), format!("{}.input_variable", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentPromptVariantElTemplateConfigurationElDynamic {
    chat: Option<DynamicBlock<BedrockagentPromptVariantElTemplateConfigurationElChatEl>>,
    text: Option<DynamicBlock<BedrockagentPromptVariantElTemplateConfigurationElTextEl>>,
}

#[derive(Serialize)]
pub struct BedrockagentPromptVariantElTemplateConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat: Option<Vec<BedrockagentPromptVariantElTemplateConfigurationElChatEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Vec<BedrockagentPromptVariantElTemplateConfigurationElTextEl>>,
    dynamic: BedrockagentPromptVariantElTemplateConfigurationElDynamic,
}

impl BedrockagentPromptVariantElTemplateConfigurationEl {
    #[doc = "Set the field `chat`.\n"]
    pub fn set_chat(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentPromptVariantElTemplateConfigurationElChatEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.chat = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.chat = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `text`.\n"]
    pub fn set_text(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentPromptVariantElTemplateConfigurationElTextEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.text = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.text = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentPromptVariantElTemplateConfigurationEl {
    type O = BlockAssignable<BedrockagentPromptVariantElTemplateConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPromptVariantElTemplateConfigurationEl {}

impl BuildBedrockagentPromptVariantElTemplateConfigurationEl {
    pub fn build(self) -> BedrockagentPromptVariantElTemplateConfigurationEl {
        BedrockagentPromptVariantElTemplateConfigurationEl {
            chat: core::default::Default::default(),
            text: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentPromptVariantElTemplateConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentPromptVariantElTemplateConfigurationElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentPromptVariantElTemplateConfigurationElRef {
        BedrockagentPromptVariantElTemplateConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPromptVariantElTemplateConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `chat` after provisioning.\n"]
    pub fn chat(&self) -> ListRef<BedrockagentPromptVariantElTemplateConfigurationElChatElRef> {
        ListRef::new(self.shared().clone(), format!("{}.chat", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> ListRef<BedrockagentPromptVariantElTemplateConfigurationElTextElRef> {
        ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentPromptVariantElDynamic {
    gen_ai_resource: Option<DynamicBlock<BedrockagentPromptVariantElGenAiResourceEl>>,
    inference_configuration: Option<DynamicBlock<BedrockagentPromptVariantElInferenceConfigurationEl>>,
    metadata: Option<DynamicBlock<BedrockagentPromptVariantElMetadataEl>>,
    template_configuration: Option<DynamicBlock<BedrockagentPromptVariantElTemplateConfigurationEl>>,
}

#[derive(Serialize)]
pub struct BedrockagentPromptVariantEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_model_request_fields: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_id: Option<PrimField<String>>,
    name: PrimField<String>,
    template_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gen_ai_resource: Option<Vec<BedrockagentPromptVariantElGenAiResourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inference_configuration: Option<Vec<BedrockagentPromptVariantElInferenceConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Vec<BedrockagentPromptVariantElMetadataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_configuration: Option<Vec<BedrockagentPromptVariantElTemplateConfigurationEl>>,
    dynamic: BedrockagentPromptVariantElDynamic,
}

impl BedrockagentPromptVariantEl {
    #[doc = "Set the field `additional_model_request_fields`.\n"]
    pub fn set_additional_model_request_fields(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.additional_model_request_fields = Some(v.into());
        self
    }

    #[doc = "Set the field `model_id`.\n"]
    pub fn set_model_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model_id = Some(v.into());
        self
    }

    #[doc = "Set the field `gen_ai_resource`.\n"]
    pub fn set_gen_ai_resource(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentPromptVariantElGenAiResourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gen_ai_resource = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gen_ai_resource = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `inference_configuration`.\n"]
    pub fn set_inference_configuration(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentPromptVariantElInferenceConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.inference_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.inference_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(mut self, v: impl Into<BlockAssignable<BedrockagentPromptVariantElMetadataEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metadata = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metadata = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `template_configuration`.\n"]
    pub fn set_template_configuration(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentPromptVariantElTemplateConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.template_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.template_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentPromptVariantEl {
    type O = BlockAssignable<BedrockagentPromptVariantEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPromptVariantEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub template_type: PrimField<String>,
}

impl BuildBedrockagentPromptVariantEl {
    pub fn build(self) -> BedrockagentPromptVariantEl {
        BedrockagentPromptVariantEl {
            additional_model_request_fields: core::default::Default::default(),
            model_id: core::default::Default::default(),
            name: self.name,
            template_type: self.template_type,
            gen_ai_resource: core::default::Default::default(),
            inference_configuration: core::default::Default::default(),
            metadata: core::default::Default::default(),
            template_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentPromptVariantElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentPromptVariantElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentPromptVariantElRef {
        BedrockagentPromptVariantElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPromptVariantElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_model_request_fields` after provisioning.\n"]
    pub fn additional_model_request_fields(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.additional_model_request_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `model_id` after provisioning.\n"]
    pub fn model_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `template_type` after provisioning.\n"]
    pub fn template_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.template_type", self.base))
    }

    #[doc = "Get a reference to the value of field `gen_ai_resource` after provisioning.\n"]
    pub fn gen_ai_resource(&self) -> ListRef<BedrockagentPromptVariantElGenAiResourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gen_ai_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `inference_configuration` after provisioning.\n"]
    pub fn inference_configuration(&self) -> ListRef<BedrockagentPromptVariantElInferenceConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.inference_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> ListRef<BedrockagentPromptVariantElMetadataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `template_configuration` after provisioning.\n"]
    pub fn template_configuration(&self) -> ListRef<BedrockagentPromptVariantElTemplateConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.template_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentPromptDynamic {
    variant: Option<DynamicBlock<BedrockagentPromptVariantEl>>,
}
