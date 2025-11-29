use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockGuardrailData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    blocked_input_messaging: PrimField<String>,
    blocked_outputs_messaging: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_policy_config: Option<Vec<BedrockGuardrailContentPolicyConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contextual_grounding_policy_config: Option<Vec<BedrockGuardrailContextualGroundingPolicyConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cross_region_config: Option<Vec<BedrockGuardrailCrossRegionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sensitive_information_policy_config: Option<Vec<BedrockGuardrailSensitiveInformationPolicyConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BedrockGuardrailTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic_policy_config: Option<Vec<BedrockGuardrailTopicPolicyConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    word_policy_config: Option<Vec<BedrockGuardrailWordPolicyConfigEl>>,
    dynamic: BedrockGuardrailDynamic,
}

struct BedrockGuardrail_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BedrockGuardrailData>,
}

#[derive(Clone)]
pub struct BedrockGuardrail(Rc<BedrockGuardrail_>);

impl BedrockGuardrail {
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

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_arn = Some(v.into());
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

    #[doc = "Set the field `content_policy_config`.\n"]
    pub fn set_content_policy_config(
        self,
        v: impl Into<BlockAssignable<BedrockGuardrailContentPolicyConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().content_policy_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.content_policy_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `contextual_grounding_policy_config`.\n"]
    pub fn set_contextual_grounding_policy_config(
        self,
        v: impl Into<BlockAssignable<BedrockGuardrailContextualGroundingPolicyConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().contextual_grounding_policy_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.contextual_grounding_policy_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cross_region_config`.\n"]
    pub fn set_cross_region_config(self, v: impl Into<BlockAssignable<BedrockGuardrailCrossRegionConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cross_region_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cross_region_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sensitive_information_policy_config`.\n"]
    pub fn set_sensitive_information_policy_config(
        self,
        v: impl Into<BlockAssignable<BedrockGuardrailSensitiveInformationPolicyConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sensitive_information_policy_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sensitive_information_policy_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BedrockGuardrailTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Set the field `topic_policy_config`.\n"]
    pub fn set_topic_policy_config(self, v: impl Into<BlockAssignable<BedrockGuardrailTopicPolicyConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().topic_policy_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.topic_policy_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `word_policy_config`.\n"]
    pub fn set_word_policy_config(self, v: impl Into<BlockAssignable<BedrockGuardrailWordPolicyConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().word_policy_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.word_policy_config = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `blocked_input_messaging` after provisioning.\n"]
    pub fn blocked_input_messaging(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.blocked_input_messaging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `blocked_outputs_messaging` after provisioning.\n"]
    pub fn blocked_outputs_messaging(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.blocked_outputs_messaging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `guardrail_arn` after provisioning.\n"]
    pub fn guardrail_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.guardrail_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `guardrail_id` after provisioning.\n"]
    pub fn guardrail_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.guardrail_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content_policy_config` after provisioning.\n"]
    pub fn content_policy_config(&self) -> ListRef<BedrockGuardrailContentPolicyConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.content_policy_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `contextual_grounding_policy_config` after provisioning.\n"]
    pub fn contextual_grounding_policy_config(&self) -> ListRef<BedrockGuardrailContextualGroundingPolicyConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.contextual_grounding_policy_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cross_region_config` after provisioning.\n"]
    pub fn cross_region_config(&self) -> ListRef<BedrockGuardrailCrossRegionConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cross_region_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sensitive_information_policy_config` after provisioning.\n"]
    pub fn sensitive_information_policy_config(&self) -> ListRef<BedrockGuardrailSensitiveInformationPolicyConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sensitive_information_policy_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockGuardrailTimeoutsElRef {
        BedrockGuardrailTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `topic_policy_config` after provisioning.\n"]
    pub fn topic_policy_config(&self) -> ListRef<BedrockGuardrailTopicPolicyConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.topic_policy_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `word_policy_config` after provisioning.\n"]
    pub fn word_policy_config(&self) -> ListRef<BedrockGuardrailWordPolicyConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.word_policy_config", self.extract_ref()))
    }
}

impl Referable for BedrockGuardrail {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for BedrockGuardrail { }

impl ToListMappable for BedrockGuardrail {
    type O = ListRef<BedrockGuardrailRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BedrockGuardrail_ {
    fn extract_resource_type(&self) -> String {
        "aws_bedrock_guardrail".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockGuardrail {
    pub tf_id: String,
    #[doc = ""]
    pub blocked_input_messaging: PrimField<String>,
    #[doc = ""]
    pub blocked_outputs_messaging: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildBedrockGuardrail {
    pub fn build(self, stack: &mut Stack) -> BedrockGuardrail {
        let out = BedrockGuardrail(Rc::new(BedrockGuardrail_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BedrockGuardrailData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                blocked_input_messaging: self.blocked_input_messaging,
                blocked_outputs_messaging: self.blocked_outputs_messaging,
                description: core::default::Default::default(),
                kms_key_arn: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                content_policy_config: core::default::Default::default(),
                contextual_grounding_policy_config: core::default::Default::default(),
                cross_region_config: core::default::Default::default(),
                sensitive_information_policy_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                topic_policy_config: core::default::Default::default(),
                word_policy_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockGuardrailRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockGuardrailRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl BedrockGuardrailRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `blocked_input_messaging` after provisioning.\n"]
    pub fn blocked_input_messaging(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.blocked_input_messaging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `blocked_outputs_messaging` after provisioning.\n"]
    pub fn blocked_outputs_messaging(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.blocked_outputs_messaging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `guardrail_arn` after provisioning.\n"]
    pub fn guardrail_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.guardrail_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `guardrail_id` after provisioning.\n"]
    pub fn guardrail_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.guardrail_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content_policy_config` after provisioning.\n"]
    pub fn content_policy_config(&self) -> ListRef<BedrockGuardrailContentPolicyConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.content_policy_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `contextual_grounding_policy_config` after provisioning.\n"]
    pub fn contextual_grounding_policy_config(&self) -> ListRef<BedrockGuardrailContextualGroundingPolicyConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.contextual_grounding_policy_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cross_region_config` after provisioning.\n"]
    pub fn cross_region_config(&self) -> ListRef<BedrockGuardrailCrossRegionConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cross_region_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sensitive_information_policy_config` after provisioning.\n"]
    pub fn sensitive_information_policy_config(&self) -> ListRef<BedrockGuardrailSensitiveInformationPolicyConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sensitive_information_policy_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockGuardrailTimeoutsElRef {
        BedrockGuardrailTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `topic_policy_config` after provisioning.\n"]
    pub fn topic_policy_config(&self) -> ListRef<BedrockGuardrailTopicPolicyConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.topic_policy_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `word_policy_config` after provisioning.\n"]
    pub fn word_policy_config(&self) -> ListRef<BedrockGuardrailWordPolicyConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.word_policy_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BedrockGuardrailContentPolicyConfigElTierConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tier_name: Option<PrimField<String>>,
}

impl BedrockGuardrailContentPolicyConfigElTierConfigEl {
    #[doc = "Set the field `tier_name`.\n"]
    pub fn set_tier_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tier_name = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockGuardrailContentPolicyConfigElTierConfigEl {
    type O = BlockAssignable<BedrockGuardrailContentPolicyConfigElTierConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrailContentPolicyConfigElTierConfigEl {}

impl BuildBedrockGuardrailContentPolicyConfigElTierConfigEl {
    pub fn build(self) -> BedrockGuardrailContentPolicyConfigElTierConfigEl {
        BedrockGuardrailContentPolicyConfigElTierConfigEl { tier_name: core::default::Default::default() }
    }
}

pub struct BedrockGuardrailContentPolicyConfigElTierConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockGuardrailContentPolicyConfigElTierConfigElRef {
    fn new(shared: StackShared, base: String) -> BedrockGuardrailContentPolicyConfigElTierConfigElRef {
        BedrockGuardrailContentPolicyConfigElTierConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrailContentPolicyConfigElTierConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tier_name` after provisioning.\n"]
    pub fn tier_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tier_name", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockGuardrailContentPolicyConfigElFiltersConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    input_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_modalities: Option<ListField<PrimField<String>>>,
    input_strength: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_modalities: Option<ListField<PrimField<String>>>,
    output_strength: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl BedrockGuardrailContentPolicyConfigElFiltersConfigEl {
    #[doc = "Set the field `input_action`.\n"]
    pub fn set_input_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.input_action = Some(v.into());
        self
    }

    #[doc = "Set the field `input_enabled`.\n"]
    pub fn set_input_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.input_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `input_modalities`.\n"]
    pub fn set_input_modalities(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.input_modalities = Some(v.into());
        self
    }

    #[doc = "Set the field `output_action`.\n"]
    pub fn set_output_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_action = Some(v.into());
        self
    }

    #[doc = "Set the field `output_enabled`.\n"]
    pub fn set_output_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.output_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `output_modalities`.\n"]
    pub fn set_output_modalities(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.output_modalities = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockGuardrailContentPolicyConfigElFiltersConfigEl {
    type O = BlockAssignable<BedrockGuardrailContentPolicyConfigElFiltersConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrailContentPolicyConfigElFiltersConfigEl {
    #[doc = ""]
    pub input_strength: PrimField<String>,
    #[doc = ""]
    pub output_strength: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildBedrockGuardrailContentPolicyConfigElFiltersConfigEl {
    pub fn build(self) -> BedrockGuardrailContentPolicyConfigElFiltersConfigEl {
        BedrockGuardrailContentPolicyConfigElFiltersConfigEl {
            input_action: core::default::Default::default(),
            input_enabled: core::default::Default::default(),
            input_modalities: core::default::Default::default(),
            input_strength: self.input_strength,
            output_action: core::default::Default::default(),
            output_enabled: core::default::Default::default(),
            output_modalities: core::default::Default::default(),
            output_strength: self.output_strength,
            type_: self.type_,
        }
    }
}

pub struct BedrockGuardrailContentPolicyConfigElFiltersConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockGuardrailContentPolicyConfigElFiltersConfigElRef {
    fn new(shared: StackShared, base: String) -> BedrockGuardrailContentPolicyConfigElFiltersConfigElRef {
        BedrockGuardrailContentPolicyConfigElFiltersConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrailContentPolicyConfigElFiltersConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_action` after provisioning.\n"]
    pub fn input_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.input_action", self.base))
    }

    #[doc = "Get a reference to the value of field `input_enabled` after provisioning.\n"]
    pub fn input_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.input_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `input_modalities` after provisioning.\n"]
    pub fn input_modalities(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.input_modalities", self.base))
    }

    #[doc = "Get a reference to the value of field `input_strength` after provisioning.\n"]
    pub fn input_strength(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.input_strength", self.base))
    }

    #[doc = "Get a reference to the value of field `output_action` after provisioning.\n"]
    pub fn output_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_action", self.base))
    }

    #[doc = "Get a reference to the value of field `output_enabled` after provisioning.\n"]
    pub fn output_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `output_modalities` after provisioning.\n"]
    pub fn output_modalities(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.output_modalities", self.base))
    }

    #[doc = "Get a reference to the value of field `output_strength` after provisioning.\n"]
    pub fn output_strength(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_strength", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockGuardrailContentPolicyConfigElDynamic {
    filters_config: Option<DynamicBlock<BedrockGuardrailContentPolicyConfigElFiltersConfigEl>>,
}

#[derive(Serialize)]
pub struct BedrockGuardrailContentPolicyConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tier_config: Option<ListField<BedrockGuardrailContentPolicyConfigElTierConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filters_config: Option<Vec<BedrockGuardrailContentPolicyConfigElFiltersConfigEl>>,
    dynamic: BedrockGuardrailContentPolicyConfigElDynamic,
}

impl BedrockGuardrailContentPolicyConfigEl {
    #[doc = "Set the field `tier_config`.\n"]
    pub fn set_tier_config(
        mut self,
        v: impl Into<ListField<BedrockGuardrailContentPolicyConfigElTierConfigEl>>,
    ) -> Self {
        self.tier_config = Some(v.into());
        self
    }

    #[doc = "Set the field `filters_config`.\n"]
    pub fn set_filters_config(
        mut self,
        v: impl Into<BlockAssignable<BedrockGuardrailContentPolicyConfigElFiltersConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filters_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filters_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockGuardrailContentPolicyConfigEl {
    type O = BlockAssignable<BedrockGuardrailContentPolicyConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrailContentPolicyConfigEl {}

impl BuildBedrockGuardrailContentPolicyConfigEl {
    pub fn build(self) -> BedrockGuardrailContentPolicyConfigEl {
        BedrockGuardrailContentPolicyConfigEl {
            tier_config: core::default::Default::default(),
            filters_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockGuardrailContentPolicyConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockGuardrailContentPolicyConfigElRef {
    fn new(shared: StackShared, base: String) -> BedrockGuardrailContentPolicyConfigElRef {
        BedrockGuardrailContentPolicyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrailContentPolicyConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tier_config` after provisioning.\n"]
    pub fn tier_config(&self) -> ListRef<BedrockGuardrailContentPolicyConfigElTierConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tier_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockGuardrailContextualGroundingPolicyConfigElFiltersConfigEl {
    threshold: PrimField<f64>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl BedrockGuardrailContextualGroundingPolicyConfigElFiltersConfigEl { }

impl ToListMappable for BedrockGuardrailContextualGroundingPolicyConfigElFiltersConfigEl {
    type O = BlockAssignable<BedrockGuardrailContextualGroundingPolicyConfigElFiltersConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrailContextualGroundingPolicyConfigElFiltersConfigEl {
    #[doc = ""]
    pub threshold: PrimField<f64>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildBedrockGuardrailContextualGroundingPolicyConfigElFiltersConfigEl {
    pub fn build(self) -> BedrockGuardrailContextualGroundingPolicyConfigElFiltersConfigEl {
        BedrockGuardrailContextualGroundingPolicyConfigElFiltersConfigEl {
            threshold: self.threshold,
            type_: self.type_,
        }
    }
}

pub struct BedrockGuardrailContextualGroundingPolicyConfigElFiltersConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockGuardrailContextualGroundingPolicyConfigElFiltersConfigElRef {
    fn new(shared: StackShared, base: String) -> BedrockGuardrailContextualGroundingPolicyConfigElFiltersConfigElRef {
        BedrockGuardrailContextualGroundingPolicyConfigElFiltersConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrailContextualGroundingPolicyConfigElFiltersConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `threshold` after provisioning.\n"]
    pub fn threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockGuardrailContextualGroundingPolicyConfigElDynamic {
    filters_config: Option<DynamicBlock<BedrockGuardrailContextualGroundingPolicyConfigElFiltersConfigEl>>,
}

#[derive(Serialize)]
pub struct BedrockGuardrailContextualGroundingPolicyConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    filters_config: Option<Vec<BedrockGuardrailContextualGroundingPolicyConfigElFiltersConfigEl>>,
    dynamic: BedrockGuardrailContextualGroundingPolicyConfigElDynamic,
}

impl BedrockGuardrailContextualGroundingPolicyConfigEl {
    #[doc = "Set the field `filters_config`.\n"]
    pub fn set_filters_config(
        mut self,
        v: impl Into<BlockAssignable<BedrockGuardrailContextualGroundingPolicyConfigElFiltersConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filters_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filters_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockGuardrailContextualGroundingPolicyConfigEl {
    type O = BlockAssignable<BedrockGuardrailContextualGroundingPolicyConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrailContextualGroundingPolicyConfigEl {}

impl BuildBedrockGuardrailContextualGroundingPolicyConfigEl {
    pub fn build(self) -> BedrockGuardrailContextualGroundingPolicyConfigEl {
        BedrockGuardrailContextualGroundingPolicyConfigEl {
            filters_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockGuardrailContextualGroundingPolicyConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockGuardrailContextualGroundingPolicyConfigElRef {
    fn new(shared: StackShared, base: String) -> BedrockGuardrailContextualGroundingPolicyConfigElRef {
        BedrockGuardrailContextualGroundingPolicyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrailContextualGroundingPolicyConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filters_config` after provisioning.\n"]
    pub fn filters_config(&self) -> ListRef<BedrockGuardrailContextualGroundingPolicyConfigElFiltersConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filters_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockGuardrailCrossRegionConfigEl {
    guardrail_profile_identifier: PrimField<String>,
}

impl BedrockGuardrailCrossRegionConfigEl { }

impl ToListMappable for BedrockGuardrailCrossRegionConfigEl {
    type O = BlockAssignable<BedrockGuardrailCrossRegionConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrailCrossRegionConfigEl {
    #[doc = ""]
    pub guardrail_profile_identifier: PrimField<String>,
}

impl BuildBedrockGuardrailCrossRegionConfigEl {
    pub fn build(self) -> BedrockGuardrailCrossRegionConfigEl {
        BedrockGuardrailCrossRegionConfigEl { guardrail_profile_identifier: self.guardrail_profile_identifier }
    }
}

pub struct BedrockGuardrailCrossRegionConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockGuardrailCrossRegionConfigElRef {
    fn new(shared: StackShared, base: String) -> BedrockGuardrailCrossRegionConfigElRef {
        BedrockGuardrailCrossRegionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrailCrossRegionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `guardrail_profile_identifier` after provisioning.\n"]
    pub fn guardrail_profile_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.guardrail_profile_identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockGuardrailSensitiveInformationPolicyConfigElPiiEntitiesConfigEl {
    action: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_enabled: Option<PrimField<bool>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl BedrockGuardrailSensitiveInformationPolicyConfigElPiiEntitiesConfigEl {
    #[doc = "Set the field `input_action`.\n"]
    pub fn set_input_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.input_action = Some(v.into());
        self
    }

    #[doc = "Set the field `input_enabled`.\n"]
    pub fn set_input_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.input_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `output_action`.\n"]
    pub fn set_output_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_action = Some(v.into());
        self
    }

    #[doc = "Set the field `output_enabled`.\n"]
    pub fn set_output_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.output_enabled = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockGuardrailSensitiveInformationPolicyConfigElPiiEntitiesConfigEl {
    type O = BlockAssignable<BedrockGuardrailSensitiveInformationPolicyConfigElPiiEntitiesConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrailSensitiveInformationPolicyConfigElPiiEntitiesConfigEl {
    #[doc = ""]
    pub action: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildBedrockGuardrailSensitiveInformationPolicyConfigElPiiEntitiesConfigEl {
    pub fn build(self) -> BedrockGuardrailSensitiveInformationPolicyConfigElPiiEntitiesConfigEl {
        BedrockGuardrailSensitiveInformationPolicyConfigElPiiEntitiesConfigEl {
            action: self.action,
            input_action: core::default::Default::default(),
            input_enabled: core::default::Default::default(),
            output_action: core::default::Default::default(),
            output_enabled: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct BedrockGuardrailSensitiveInformationPolicyConfigElPiiEntitiesConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockGuardrailSensitiveInformationPolicyConfigElPiiEntitiesConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockGuardrailSensitiveInformationPolicyConfigElPiiEntitiesConfigElRef {
        BedrockGuardrailSensitiveInformationPolicyConfigElPiiEntitiesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrailSensitiveInformationPolicyConfigElPiiEntitiesConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `input_action` after provisioning.\n"]
    pub fn input_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.input_action", self.base))
    }

    #[doc = "Get a reference to the value of field `input_enabled` after provisioning.\n"]
    pub fn input_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.input_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `output_action` after provisioning.\n"]
    pub fn output_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_action", self.base))
    }

    #[doc = "Get a reference to the value of field `output_enabled` after provisioning.\n"]
    pub fn output_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockGuardrailSensitiveInformationPolicyConfigElRegexesConfigEl {
    action: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_enabled: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_enabled: Option<PrimField<bool>>,
    pattern: PrimField<String>,
}

impl BedrockGuardrailSensitiveInformationPolicyConfigElRegexesConfigEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc = "Set the field `input_action`.\n"]
    pub fn set_input_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.input_action = Some(v.into());
        self
    }

    #[doc = "Set the field `input_enabled`.\n"]
    pub fn set_input_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.input_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `output_action`.\n"]
    pub fn set_output_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_action = Some(v.into());
        self
    }

    #[doc = "Set the field `output_enabled`.\n"]
    pub fn set_output_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.output_enabled = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockGuardrailSensitiveInformationPolicyConfigElRegexesConfigEl {
    type O = BlockAssignable<BedrockGuardrailSensitiveInformationPolicyConfigElRegexesConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrailSensitiveInformationPolicyConfigElRegexesConfigEl {
    #[doc = ""]
    pub action: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub pattern: PrimField<String>,
}

impl BuildBedrockGuardrailSensitiveInformationPolicyConfigElRegexesConfigEl {
    pub fn build(self) -> BedrockGuardrailSensitiveInformationPolicyConfigElRegexesConfigEl {
        BedrockGuardrailSensitiveInformationPolicyConfigElRegexesConfigEl {
            action: self.action,
            description: core::default::Default::default(),
            input_action: core::default::Default::default(),
            input_enabled: core::default::Default::default(),
            name: self.name,
            output_action: core::default::Default::default(),
            output_enabled: core::default::Default::default(),
            pattern: self.pattern,
        }
    }
}

pub struct BedrockGuardrailSensitiveInformationPolicyConfigElRegexesConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockGuardrailSensitiveInformationPolicyConfigElRegexesConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockGuardrailSensitiveInformationPolicyConfigElRegexesConfigElRef {
        BedrockGuardrailSensitiveInformationPolicyConfigElRegexesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrailSensitiveInformationPolicyConfigElRegexesConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `input_action` after provisioning.\n"]
    pub fn input_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.input_action", self.base))
    }

    #[doc = "Get a reference to the value of field `input_enabled` after provisioning.\n"]
    pub fn input_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.input_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `output_action` after provisioning.\n"]
    pub fn output_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_action", self.base))
    }

    #[doc = "Get a reference to the value of field `output_enabled` after provisioning.\n"]
    pub fn output_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `pattern` after provisioning.\n"]
    pub fn pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockGuardrailSensitiveInformationPolicyConfigElDynamic {
    pii_entities_config: Option<DynamicBlock<BedrockGuardrailSensitiveInformationPolicyConfigElPiiEntitiesConfigEl>>,
    regexes_config: Option<DynamicBlock<BedrockGuardrailSensitiveInformationPolicyConfigElRegexesConfigEl>>,
}

#[derive(Serialize)]
pub struct BedrockGuardrailSensitiveInformationPolicyConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pii_entities_config: Option<Vec<BedrockGuardrailSensitiveInformationPolicyConfigElPiiEntitiesConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regexes_config: Option<Vec<BedrockGuardrailSensitiveInformationPolicyConfigElRegexesConfigEl>>,
    dynamic: BedrockGuardrailSensitiveInformationPolicyConfigElDynamic,
}

impl BedrockGuardrailSensitiveInformationPolicyConfigEl {
    #[doc = "Set the field `pii_entities_config`.\n"]
    pub fn set_pii_entities_config(
        mut self,
        v: impl Into<BlockAssignable<BedrockGuardrailSensitiveInformationPolicyConfigElPiiEntitiesConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pii_entities_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pii_entities_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `regexes_config`.\n"]
    pub fn set_regexes_config(
        mut self,
        v: impl Into<BlockAssignable<BedrockGuardrailSensitiveInformationPolicyConfigElRegexesConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.regexes_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.regexes_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockGuardrailSensitiveInformationPolicyConfigEl {
    type O = BlockAssignable<BedrockGuardrailSensitiveInformationPolicyConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrailSensitiveInformationPolicyConfigEl {}

impl BuildBedrockGuardrailSensitiveInformationPolicyConfigEl {
    pub fn build(self) -> BedrockGuardrailSensitiveInformationPolicyConfigEl {
        BedrockGuardrailSensitiveInformationPolicyConfigEl {
            pii_entities_config: core::default::Default::default(),
            regexes_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockGuardrailSensitiveInformationPolicyConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockGuardrailSensitiveInformationPolicyConfigElRef {
    fn new(shared: StackShared, base: String) -> BedrockGuardrailSensitiveInformationPolicyConfigElRef {
        BedrockGuardrailSensitiveInformationPolicyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrailSensitiveInformationPolicyConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pii_entities_config` after provisioning.\n"]
    pub fn pii_entities_config(
        &self,
    ) -> ListRef<BedrockGuardrailSensitiveInformationPolicyConfigElPiiEntitiesConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pii_entities_config", self.base))
    }

    #[doc = "Get a reference to the value of field `regexes_config` after provisioning.\n"]
    pub fn regexes_config(&self) -> ListRef<BedrockGuardrailSensitiveInformationPolicyConfigElRegexesConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.regexes_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockGuardrailTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl BedrockGuardrailTimeoutsEl {
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

impl ToListMappable for BedrockGuardrailTimeoutsEl {
    type O = BlockAssignable<BedrockGuardrailTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrailTimeoutsEl {}

impl BuildBedrockGuardrailTimeoutsEl {
    pub fn build(self) -> BedrockGuardrailTimeoutsEl {
        BedrockGuardrailTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct BedrockGuardrailTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockGuardrailTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BedrockGuardrailTimeoutsElRef {
        BedrockGuardrailTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrailTimeoutsElRef {
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

#[derive(Serialize)]
pub struct BedrockGuardrailTopicPolicyConfigElTierConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tier_name: Option<PrimField<String>>,
}

impl BedrockGuardrailTopicPolicyConfigElTierConfigEl {
    #[doc = "Set the field `tier_name`.\n"]
    pub fn set_tier_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tier_name = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockGuardrailTopicPolicyConfigElTierConfigEl {
    type O = BlockAssignable<BedrockGuardrailTopicPolicyConfigElTierConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrailTopicPolicyConfigElTierConfigEl {}

impl BuildBedrockGuardrailTopicPolicyConfigElTierConfigEl {
    pub fn build(self) -> BedrockGuardrailTopicPolicyConfigElTierConfigEl {
        BedrockGuardrailTopicPolicyConfigElTierConfigEl { tier_name: core::default::Default::default() }
    }
}

pub struct BedrockGuardrailTopicPolicyConfigElTierConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockGuardrailTopicPolicyConfigElTierConfigElRef {
    fn new(shared: StackShared, base: String) -> BedrockGuardrailTopicPolicyConfigElTierConfigElRef {
        BedrockGuardrailTopicPolicyConfigElTierConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrailTopicPolicyConfigElTierConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tier_name` after provisioning.\n"]
    pub fn tier_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tier_name", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockGuardrailTopicPolicyConfigElTopicsConfigEl {
    definition: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    examples: Option<ListField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl BedrockGuardrailTopicPolicyConfigElTopicsConfigEl {
    #[doc = "Set the field `examples`.\n"]
    pub fn set_examples(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.examples = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockGuardrailTopicPolicyConfigElTopicsConfigEl {
    type O = BlockAssignable<BedrockGuardrailTopicPolicyConfigElTopicsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrailTopicPolicyConfigElTopicsConfigEl {
    #[doc = ""]
    pub definition: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildBedrockGuardrailTopicPolicyConfigElTopicsConfigEl {
    pub fn build(self) -> BedrockGuardrailTopicPolicyConfigElTopicsConfigEl {
        BedrockGuardrailTopicPolicyConfigElTopicsConfigEl {
            definition: self.definition,
            examples: core::default::Default::default(),
            name: self.name,
            type_: self.type_,
        }
    }
}

pub struct BedrockGuardrailTopicPolicyConfigElTopicsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockGuardrailTopicPolicyConfigElTopicsConfigElRef {
    fn new(shared: StackShared, base: String) -> BedrockGuardrailTopicPolicyConfigElTopicsConfigElRef {
        BedrockGuardrailTopicPolicyConfigElTopicsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrailTopicPolicyConfigElTopicsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.definition", self.base))
    }

    #[doc = "Get a reference to the value of field `examples` after provisioning.\n"]
    pub fn examples(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.examples", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockGuardrailTopicPolicyConfigElDynamic {
    topics_config: Option<DynamicBlock<BedrockGuardrailTopicPolicyConfigElTopicsConfigEl>>,
}

#[derive(Serialize)]
pub struct BedrockGuardrailTopicPolicyConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tier_config: Option<ListField<BedrockGuardrailTopicPolicyConfigElTierConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topics_config: Option<Vec<BedrockGuardrailTopicPolicyConfigElTopicsConfigEl>>,
    dynamic: BedrockGuardrailTopicPolicyConfigElDynamic,
}

impl BedrockGuardrailTopicPolicyConfigEl {
    #[doc = "Set the field `tier_config`.\n"]
    pub fn set_tier_config(mut self, v: impl Into<ListField<BedrockGuardrailTopicPolicyConfigElTierConfigEl>>) -> Self {
        self.tier_config = Some(v.into());
        self
    }

    #[doc = "Set the field `topics_config`.\n"]
    pub fn set_topics_config(
        mut self,
        v: impl Into<BlockAssignable<BedrockGuardrailTopicPolicyConfigElTopicsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.topics_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.topics_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockGuardrailTopicPolicyConfigEl {
    type O = BlockAssignable<BedrockGuardrailTopicPolicyConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrailTopicPolicyConfigEl {}

impl BuildBedrockGuardrailTopicPolicyConfigEl {
    pub fn build(self) -> BedrockGuardrailTopicPolicyConfigEl {
        BedrockGuardrailTopicPolicyConfigEl {
            tier_config: core::default::Default::default(),
            topics_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockGuardrailTopicPolicyConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockGuardrailTopicPolicyConfigElRef {
    fn new(shared: StackShared, base: String) -> BedrockGuardrailTopicPolicyConfigElRef {
        BedrockGuardrailTopicPolicyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrailTopicPolicyConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tier_config` after provisioning.\n"]
    pub fn tier_config(&self) -> ListRef<BedrockGuardrailTopicPolicyConfigElTierConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tier_config", self.base))
    }

    #[doc = "Get a reference to the value of field `topics_config` after provisioning.\n"]
    pub fn topics_config(&self) -> ListRef<BedrockGuardrailTopicPolicyConfigElTopicsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.topics_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockGuardrailWordPolicyConfigElManagedWordListsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    input_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_enabled: Option<PrimField<bool>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl BedrockGuardrailWordPolicyConfigElManagedWordListsConfigEl {
    #[doc = "Set the field `input_action`.\n"]
    pub fn set_input_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.input_action = Some(v.into());
        self
    }

    #[doc = "Set the field `input_enabled`.\n"]
    pub fn set_input_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.input_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `output_action`.\n"]
    pub fn set_output_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_action = Some(v.into());
        self
    }

    #[doc = "Set the field `output_enabled`.\n"]
    pub fn set_output_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.output_enabled = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockGuardrailWordPolicyConfigElManagedWordListsConfigEl {
    type O = BlockAssignable<BedrockGuardrailWordPolicyConfigElManagedWordListsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrailWordPolicyConfigElManagedWordListsConfigEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildBedrockGuardrailWordPolicyConfigElManagedWordListsConfigEl {
    pub fn build(self) -> BedrockGuardrailWordPolicyConfigElManagedWordListsConfigEl {
        BedrockGuardrailWordPolicyConfigElManagedWordListsConfigEl {
            input_action: core::default::Default::default(),
            input_enabled: core::default::Default::default(),
            output_action: core::default::Default::default(),
            output_enabled: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct BedrockGuardrailWordPolicyConfigElManagedWordListsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockGuardrailWordPolicyConfigElManagedWordListsConfigElRef {
    fn new(shared: StackShared, base: String) -> BedrockGuardrailWordPolicyConfigElManagedWordListsConfigElRef {
        BedrockGuardrailWordPolicyConfigElManagedWordListsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrailWordPolicyConfigElManagedWordListsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_action` after provisioning.\n"]
    pub fn input_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.input_action", self.base))
    }

    #[doc = "Get a reference to the value of field `input_enabled` after provisioning.\n"]
    pub fn input_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.input_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `output_action` after provisioning.\n"]
    pub fn output_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_action", self.base))
    }

    #[doc = "Get a reference to the value of field `output_enabled` after provisioning.\n"]
    pub fn output_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockGuardrailWordPolicyConfigElWordsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    input_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_enabled: Option<PrimField<bool>>,
    text: PrimField<String>,
}

impl BedrockGuardrailWordPolicyConfigElWordsConfigEl {
    #[doc = "Set the field `input_action`.\n"]
    pub fn set_input_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.input_action = Some(v.into());
        self
    }

    #[doc = "Set the field `input_enabled`.\n"]
    pub fn set_input_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.input_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `output_action`.\n"]
    pub fn set_output_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_action = Some(v.into());
        self
    }

    #[doc = "Set the field `output_enabled`.\n"]
    pub fn set_output_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.output_enabled = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockGuardrailWordPolicyConfigElWordsConfigEl {
    type O = BlockAssignable<BedrockGuardrailWordPolicyConfigElWordsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrailWordPolicyConfigElWordsConfigEl {
    #[doc = ""]
    pub text: PrimField<String>,
}

impl BuildBedrockGuardrailWordPolicyConfigElWordsConfigEl {
    pub fn build(self) -> BedrockGuardrailWordPolicyConfigElWordsConfigEl {
        BedrockGuardrailWordPolicyConfigElWordsConfigEl {
            input_action: core::default::Default::default(),
            input_enabled: core::default::Default::default(),
            output_action: core::default::Default::default(),
            output_enabled: core::default::Default::default(),
            text: self.text,
        }
    }
}

pub struct BedrockGuardrailWordPolicyConfigElWordsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockGuardrailWordPolicyConfigElWordsConfigElRef {
    fn new(shared: StackShared, base: String) -> BedrockGuardrailWordPolicyConfigElWordsConfigElRef {
        BedrockGuardrailWordPolicyConfigElWordsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrailWordPolicyConfigElWordsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_action` after provisioning.\n"]
    pub fn input_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.input_action", self.base))
    }

    #[doc = "Get a reference to the value of field `input_enabled` after provisioning.\n"]
    pub fn input_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.input_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `output_action` after provisioning.\n"]
    pub fn output_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_action", self.base))
    }

    #[doc = "Get a reference to the value of field `output_enabled` after provisioning.\n"]
    pub fn output_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockGuardrailWordPolicyConfigElDynamic {
    managed_word_lists_config: Option<DynamicBlock<BedrockGuardrailWordPolicyConfigElManagedWordListsConfigEl>>,
    words_config: Option<DynamicBlock<BedrockGuardrailWordPolicyConfigElWordsConfigEl>>,
}

#[derive(Serialize)]
pub struct BedrockGuardrailWordPolicyConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_word_lists_config: Option<Vec<BedrockGuardrailWordPolicyConfigElManagedWordListsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    words_config: Option<Vec<BedrockGuardrailWordPolicyConfigElWordsConfigEl>>,
    dynamic: BedrockGuardrailWordPolicyConfigElDynamic,
}

impl BedrockGuardrailWordPolicyConfigEl {
    #[doc = "Set the field `managed_word_lists_config`.\n"]
    pub fn set_managed_word_lists_config(
        mut self,
        v: impl Into<BlockAssignable<BedrockGuardrailWordPolicyConfigElManagedWordListsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.managed_word_lists_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_word_lists_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `words_config`.\n"]
    pub fn set_words_config(
        mut self,
        v: impl Into<BlockAssignable<BedrockGuardrailWordPolicyConfigElWordsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.words_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.words_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockGuardrailWordPolicyConfigEl {
    type O = BlockAssignable<BedrockGuardrailWordPolicyConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrailWordPolicyConfigEl {}

impl BuildBedrockGuardrailWordPolicyConfigEl {
    pub fn build(self) -> BedrockGuardrailWordPolicyConfigEl {
        BedrockGuardrailWordPolicyConfigEl {
            managed_word_lists_config: core::default::Default::default(),
            words_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockGuardrailWordPolicyConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockGuardrailWordPolicyConfigElRef {
    fn new(shared: StackShared, base: String) -> BedrockGuardrailWordPolicyConfigElRef {
        BedrockGuardrailWordPolicyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrailWordPolicyConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `managed_word_lists_config` after provisioning.\n"]
    pub fn managed_word_lists_config(&self) -> ListRef<BedrockGuardrailWordPolicyConfigElManagedWordListsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.managed_word_lists_config", self.base))
    }

    #[doc = "Get a reference to the value of field `words_config` after provisioning.\n"]
    pub fn words_config(&self) -> ListRef<BedrockGuardrailWordPolicyConfigElWordsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.words_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockGuardrailDynamic {
    content_policy_config: Option<DynamicBlock<BedrockGuardrailContentPolicyConfigEl>>,
    contextual_grounding_policy_config: Option<DynamicBlock<BedrockGuardrailContextualGroundingPolicyConfigEl>>,
    cross_region_config: Option<DynamicBlock<BedrockGuardrailCrossRegionConfigEl>>,
    sensitive_information_policy_config: Option<DynamicBlock<BedrockGuardrailSensitiveInformationPolicyConfigEl>>,
    topic_policy_config: Option<DynamicBlock<BedrockGuardrailTopicPolicyConfigEl>>,
    word_policy_config: Option<DynamicBlock<BedrockGuardrailWordPolicyConfigEl>>,
}
