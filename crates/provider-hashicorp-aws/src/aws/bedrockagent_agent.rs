use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BedrockagentAgentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_collaboration: Option<PrimField<String>>,
    agent_name: PrimField<String>,
    agent_resource_role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_encryption_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    foundation_model: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guardrail_configuration: Option<ListField<BedrockagentAgentGuardrailConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_session_ttl_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instruction: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_configuration: Option<ListField<BedrockagentAgentMemoryConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prepare_agent: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt_override_configuration:
        Option<ListField<BedrockagentAgentPromptOverrideConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_resource_in_use_check: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BedrockagentAgentTimeoutsEl>,
}
struct BedrockagentAgent_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BedrockagentAgentData>,
}
#[derive(Clone)]
pub struct BedrockagentAgent(Rc<BedrockagentAgent_>);
impl BedrockagentAgent {
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
    #[doc = "Set the field `agent_collaboration`.\n"]
    pub fn set_agent_collaboration(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().agent_collaboration = Some(v.into());
        self
    }
    #[doc = "Set the field `customer_encryption_key_arn`.\n"]
    pub fn set_customer_encryption_key_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().customer_encryption_key_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `guardrail_configuration`.\n"]
    pub fn set_guardrail_configuration(
        self,
        v: impl Into<ListField<BedrockagentAgentGuardrailConfigurationEl>>,
    ) -> Self {
        self.0.data.borrow_mut().guardrail_configuration = Some(v.into());
        self
    }
    #[doc = "Set the field `idle_session_ttl_in_seconds`.\n"]
    pub fn set_idle_session_ttl_in_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().idle_session_ttl_in_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `instruction`.\n"]
    pub fn set_instruction(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().instruction = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_configuration`.\n"]
    pub fn set_memory_configuration(
        self,
        v: impl Into<ListField<BedrockagentAgentMemoryConfigurationEl>>,
    ) -> Self {
        self.0.data.borrow_mut().memory_configuration = Some(v.into());
        self
    }
    #[doc = "Set the field `prepare_agent`.\n"]
    pub fn set_prepare_agent(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().prepare_agent = Some(v.into());
        self
    }
    #[doc = "Set the field `prompt_override_configuration`.\n"]
    pub fn set_prompt_override_configuration(
        self,
        v: impl Into<ListField<BedrockagentAgentPromptOverrideConfigurationEl>>,
    ) -> Self {
        self.0.data.borrow_mut().prompt_override_configuration = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `skip_resource_in_use_check`.\n"]
    pub fn set_skip_resource_in_use_check(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_resource_in_use_check = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BedrockagentAgentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `agent_arn` after provisioning.\n"]
    pub fn agent_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.agent_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `agent_collaboration` after provisioning.\n"]
    pub fn agent_collaboration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.agent_collaboration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `agent_id` after provisioning.\n"]
    pub fn agent_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.agent_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `agent_name` after provisioning.\n"]
    pub fn agent_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.agent_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `agent_resource_role_arn` after provisioning.\n"]
    pub fn agent_resource_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.agent_resource_role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `agent_version` after provisioning.\n"]
    pub fn agent_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.agent_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_encryption_key_arn` after provisioning.\n"]
    pub fn customer_encryption_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_encryption_key_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `foundation_model` after provisioning.\n"]
    pub fn foundation_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.foundation_model", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `guardrail_configuration` after provisioning.\n"]
    pub fn guardrail_configuration(&self) -> ListRef<BedrockagentAgentGuardrailConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.guardrail_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `idle_session_ttl_in_seconds` after provisioning.\n"]
    pub fn idle_session_ttl_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.idle_session_ttl_in_seconds", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instruction` after provisioning.\n"]
    pub fn instruction(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instruction", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `memory_configuration` after provisioning.\n"]
    pub fn memory_configuration(&self) -> ListRef<BedrockagentAgentMemoryConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.memory_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prepare_agent` after provisioning.\n"]
    pub fn prepare_agent(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prepare_agent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prepared_at` after provisioning.\n"]
    pub fn prepared_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prepared_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prompt_override_configuration` after provisioning.\n"]
    pub fn prompt_override_configuration(
        &self,
    ) -> ListRef<BedrockagentAgentPromptOverrideConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.prompt_override_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `skip_resource_in_use_check` after provisioning.\n"]
    pub fn skip_resource_in_use_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_resource_in_use_check", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentAgentTimeoutsElRef {
        BedrockagentAgentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BedrockagentAgent {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BedrockagentAgent {}
impl ToListMappable for BedrockagentAgent {
    type O = ListRef<BedrockagentAgentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BedrockagentAgent_ {
    fn extract_resource_type(&self) -> String {
        "aws_bedrockagent_agent".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBedrockagentAgent {
    pub tf_id: String,
    #[doc = ""]
    pub agent_name: PrimField<String>,
    #[doc = ""]
    pub agent_resource_role_arn: PrimField<String>,
    #[doc = ""]
    pub foundation_model: PrimField<String>,
}
impl BuildBedrockagentAgent {
    pub fn build(self, stack: &mut Stack) -> BedrockagentAgent {
        let out = BedrockagentAgent(Rc::new(BedrockagentAgent_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BedrockagentAgentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                agent_collaboration: core::default::Default::default(),
                agent_name: self.agent_name,
                agent_resource_role_arn: self.agent_resource_role_arn,
                customer_encryption_key_arn: core::default::Default::default(),
                description: core::default::Default::default(),
                foundation_model: self.foundation_model,
                guardrail_configuration: core::default::Default::default(),
                idle_session_ttl_in_seconds: core::default::Default::default(),
                instruction: core::default::Default::default(),
                memory_configuration: core::default::Default::default(),
                prepare_agent: core::default::Default::default(),
                prompt_override_configuration: core::default::Default::default(),
                region: core::default::Default::default(),
                skip_resource_in_use_check: core::default::Default::default(),
                tags: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BedrockagentAgentRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentAgentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BedrockagentAgentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `agent_arn` after provisioning.\n"]
    pub fn agent_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.agent_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `agent_collaboration` after provisioning.\n"]
    pub fn agent_collaboration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.agent_collaboration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `agent_id` after provisioning.\n"]
    pub fn agent_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.agent_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `agent_name` after provisioning.\n"]
    pub fn agent_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.agent_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `agent_resource_role_arn` after provisioning.\n"]
    pub fn agent_resource_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.agent_resource_role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `agent_version` after provisioning.\n"]
    pub fn agent_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.agent_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_encryption_key_arn` after provisioning.\n"]
    pub fn customer_encryption_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_encryption_key_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `foundation_model` after provisioning.\n"]
    pub fn foundation_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.foundation_model", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `guardrail_configuration` after provisioning.\n"]
    pub fn guardrail_configuration(&self) -> ListRef<BedrockagentAgentGuardrailConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.guardrail_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `idle_session_ttl_in_seconds` after provisioning.\n"]
    pub fn idle_session_ttl_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.idle_session_ttl_in_seconds", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instruction` after provisioning.\n"]
    pub fn instruction(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instruction", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `memory_configuration` after provisioning.\n"]
    pub fn memory_configuration(&self) -> ListRef<BedrockagentAgentMemoryConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.memory_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prepare_agent` after provisioning.\n"]
    pub fn prepare_agent(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prepare_agent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prepared_at` after provisioning.\n"]
    pub fn prepared_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prepared_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prompt_override_configuration` after provisioning.\n"]
    pub fn prompt_override_configuration(
        &self,
    ) -> ListRef<BedrockagentAgentPromptOverrideConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.prompt_override_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `skip_resource_in_use_check` after provisioning.\n"]
    pub fn skip_resource_in_use_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_resource_in_use_check", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentAgentTimeoutsElRef {
        BedrockagentAgentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BedrockagentAgentGuardrailConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    guardrail_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guardrail_version: Option<PrimField<String>>,
}
impl BedrockagentAgentGuardrailConfigurationEl {
    #[doc = "Set the field `guardrail_identifier`.\n"]
    pub fn set_guardrail_identifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.guardrail_identifier = Some(v.into());
        self
    }
    #[doc = "Set the field `guardrail_version`.\n"]
    pub fn set_guardrail_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.guardrail_version = Some(v.into());
        self
    }
}
impl ToListMappable for BedrockagentAgentGuardrailConfigurationEl {
    type O = BlockAssignable<BedrockagentAgentGuardrailConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentAgentGuardrailConfigurationEl {}
impl BuildBedrockagentAgentGuardrailConfigurationEl {
    pub fn build(self) -> BedrockagentAgentGuardrailConfigurationEl {
        BedrockagentAgentGuardrailConfigurationEl {
            guardrail_identifier: core::default::Default::default(),
            guardrail_version: core::default::Default::default(),
        }
    }
}
pub struct BedrockagentAgentGuardrailConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentAgentGuardrailConfigurationElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentAgentGuardrailConfigurationElRef {
        BedrockagentAgentGuardrailConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentAgentGuardrailConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `guardrail_identifier` after provisioning.\n"]
    pub fn guardrail_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.guardrail_identifier", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `guardrail_version` after provisioning.\n"]
    pub fn guardrail_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.guardrail_version", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BedrockagentAgentMemoryConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled_memory_types: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_days: Option<PrimField<f64>>,
}
impl BedrockagentAgentMemoryConfigurationEl {
    #[doc = "Set the field `enabled_memory_types`.\n"]
    pub fn set_enabled_memory_types(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.enabled_memory_types = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_days`.\n"]
    pub fn set_storage_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.storage_days = Some(v.into());
        self
    }
}
impl ToListMappable for BedrockagentAgentMemoryConfigurationEl {
    type O = BlockAssignable<BedrockagentAgentMemoryConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentAgentMemoryConfigurationEl {}
impl BuildBedrockagentAgentMemoryConfigurationEl {
    pub fn build(self) -> BedrockagentAgentMemoryConfigurationEl {
        BedrockagentAgentMemoryConfigurationEl {
            enabled_memory_types: core::default::Default::default(),
            storage_days: core::default::Default::default(),
        }
    }
}
pub struct BedrockagentAgentMemoryConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentAgentMemoryConfigurationElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentAgentMemoryConfigurationElRef {
        BedrockagentAgentMemoryConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentAgentMemoryConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled_memory_types` after provisioning.\n"]
    pub fn enabled_memory_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.enabled_memory_types", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `storage_days` after provisioning.\n"]
    pub fn storage_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_days", self.base))
    }
}
#[derive(Serialize)]
pub struct BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsElInferenceConfigurationEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_sequences: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_k: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<PrimField<f64>>,
}
impl BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsElInferenceConfigurationEl {
    #[doc = "Set the field `max_length`.\n"]
    pub fn set_max_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_length = Some(v.into());
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
    #[doc = "Set the field `top_k`.\n"]
    pub fn set_top_k(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.top_k = Some(v.into());
        self
    }
    #[doc = "Set the field `top_p`.\n"]
    pub fn set_top_p(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.top_p = Some(v.into());
        self
    }
}
impl ToListMappable
    for BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsElInferenceConfigurationEl
{
    type O = BlockAssignable < BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsElInferenceConfigurationEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsElInferenceConfigurationEl
{}
impl BuildBedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsElInferenceConfigurationEl { pub fn build (self) -> BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsElInferenceConfigurationEl { BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsElInferenceConfigurationEl { max_length : core :: default :: Default :: default () , stop_sequences : core :: default :: Default :: default () , temperature : core :: default :: Default :: default () , top_k : core :: default :: Default :: default () , top_p : core :: default :: Default :: default () , } } }
pub struct BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsElInferenceConfigurationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsElInferenceConfigurationElRef { fn new (shared : StackShared , base : String) -> BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsElInferenceConfigurationElRef { BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsElInferenceConfigurationElRef { shared : shared , base : base . to_string () , } } }
impl
    BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsElInferenceConfigurationElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_length` after provisioning.\n"]
    pub fn max_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_length", self.base))
    }
    #[doc = "Get a reference to the value of field `stop_sequences` after provisioning.\n"]
    pub fn stop_sequences(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stop_sequences", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `temperature` after provisioning.\n"]
    pub fn temperature(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }
    #[doc = "Get a reference to the value of field `top_k` after provisioning.\n"]
    pub fn top_k(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.top_k", self.base))
    }
    #[doc = "Get a reference to the value of field `top_p` after provisioning.\n"]
    pub fn top_p(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.top_p", self.base))
    }
}
#[derive(Serialize)]
pub struct BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsEl { # [serde (skip_serializing_if = "Option::is_none")] base_prompt_template : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] inference_configuration : Option < ListField < BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsElInferenceConfigurationEl > > , # [serde (skip_serializing_if = "Option::is_none")] parser_mode : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] prompt_creation_mode : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] prompt_state : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] prompt_type : Option < PrimField < String > > , }
impl BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsEl {
    #[doc = "Set the field `base_prompt_template`.\n"]
    pub fn set_base_prompt_template(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.base_prompt_template = Some(v.into());
        self
    }
    #[doc = "Set the field `inference_configuration`.\n"]
    pub fn set_inference_configuration(
        mut self,
        v : impl Into < ListField < BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsElInferenceConfigurationEl > >,
    ) -> Self {
        self.inference_configuration = Some(v.into());
        self
    }
    #[doc = "Set the field `parser_mode`.\n"]
    pub fn set_parser_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.parser_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `prompt_creation_mode`.\n"]
    pub fn set_prompt_creation_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prompt_creation_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `prompt_state`.\n"]
    pub fn set_prompt_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prompt_state = Some(v.into());
        self
    }
    #[doc = "Set the field `prompt_type`.\n"]
    pub fn set_prompt_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prompt_type = Some(v.into());
        self
    }
}
impl ToListMappable for BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsEl {
    type O = BlockAssignable<BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsEl {}
impl BuildBedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsEl {
    pub fn build(self) -> BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsEl {
        BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsEl {
            base_prompt_template: core::default::Default::default(),
            inference_configuration: core::default::Default::default(),
            parser_mode: core::default::Default::default(),
            prompt_creation_mode: core::default::Default::default(),
            prompt_state: core::default::Default::default(),
            prompt_type: core::default::Default::default(),
        }
    }
}
pub struct BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsElRef {
        BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `base_prompt_template` after provisioning.\n"]
    pub fn base_prompt_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_prompt_template", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `inference_configuration` after provisioning.\n"]    pub fn inference_configuration (& self) -> ListRef < BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsElInferenceConfigurationElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.inference_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `parser_mode` after provisioning.\n"]
    pub fn parser_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parser_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `prompt_creation_mode` after provisioning.\n"]
    pub fn prompt_creation_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prompt_creation_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `prompt_state` after provisioning.\n"]
    pub fn prompt_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prompt_state", self.base))
    }
    #[doc = "Get a reference to the value of field `prompt_type` after provisioning.\n"]
    pub fn prompt_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prompt_type", self.base))
    }
}
#[derive(Serialize)]
pub struct BedrockagentAgentPromptOverrideConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    override_lambda: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt_configurations:
        Option<SetField<BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsEl>>,
}
impl BedrockagentAgentPromptOverrideConfigurationEl {
    #[doc = "Set the field `override_lambda`.\n"]
    pub fn set_override_lambda(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.override_lambda = Some(v.into());
        self
    }
    #[doc = "Set the field `prompt_configurations`.\n"]
    pub fn set_prompt_configurations(
        mut self,
        v: impl Into<SetField<BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsEl>>,
    ) -> Self {
        self.prompt_configurations = Some(v.into());
        self
    }
}
impl ToListMappable for BedrockagentAgentPromptOverrideConfigurationEl {
    type O = BlockAssignable<BedrockagentAgentPromptOverrideConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentAgentPromptOverrideConfigurationEl {}
impl BuildBedrockagentAgentPromptOverrideConfigurationEl {
    pub fn build(self) -> BedrockagentAgentPromptOverrideConfigurationEl {
        BedrockagentAgentPromptOverrideConfigurationEl {
            override_lambda: core::default::Default::default(),
            prompt_configurations: core::default::Default::default(),
        }
    }
}
pub struct BedrockagentAgentPromptOverrideConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentAgentPromptOverrideConfigurationElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentAgentPromptOverrideConfigurationElRef {
        BedrockagentAgentPromptOverrideConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentAgentPromptOverrideConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `override_lambda` after provisioning.\n"]
    pub fn override_lambda(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.override_lambda", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `prompt_configurations` after provisioning.\n"]
    pub fn prompt_configurations(
        &self,
    ) -> SetRef<BedrockagentAgentPromptOverrideConfigurationElPromptConfigurationsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.prompt_configurations", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BedrockagentAgentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BedrockagentAgentTimeoutsEl {
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
    #[doc = "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for BedrockagentAgentTimeoutsEl {
    type O = BlockAssignable<BedrockagentAgentTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentAgentTimeoutsEl {}
impl BuildBedrockagentAgentTimeoutsEl {
    pub fn build(self) -> BedrockagentAgentTimeoutsEl {
        BedrockagentAgentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BedrockagentAgentTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentAgentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentAgentTimeoutsElRef {
        BedrockagentAgentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentAgentTimeoutsElRef {
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
    #[doc = "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
