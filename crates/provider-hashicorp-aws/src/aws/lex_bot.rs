use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LexBotData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    child_directed: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_version: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detect_sentiment: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_model_improvements: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_session_ttl_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locale: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nlu_intent_confidence_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    process_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    voice_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    abort_statement: Option<Vec<LexBotAbortStatementEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    clarification_prompt: Option<Vec<LexBotClarificationPromptEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    intent: Option<Vec<LexBotIntentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<LexBotTimeoutsEl>,
    dynamic: LexBotDynamic,
}

struct LexBot_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LexBotData>,
}

#[derive(Clone)]
pub struct LexBot(Rc<LexBot_>);

impl LexBot {
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

    #[doc = "Set the field `create_version`.\n"]
    pub fn set_create_version(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().create_version = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `detect_sentiment`.\n"]
    pub fn set_detect_sentiment(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().detect_sentiment = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_model_improvements`.\n"]
    pub fn set_enable_model_improvements(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_model_improvements = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `idle_session_ttl_in_seconds`.\n"]
    pub fn set_idle_session_ttl_in_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().idle_session_ttl_in_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `locale`.\n"]
    pub fn set_locale(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().locale = Some(v.into());
        self
    }

    #[doc = "Set the field `nlu_intent_confidence_threshold`.\n"]
    pub fn set_nlu_intent_confidence_threshold(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().nlu_intent_confidence_threshold = Some(v.into());
        self
    }

    #[doc = "Set the field `process_behavior`.\n"]
    pub fn set_process_behavior(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().process_behavior = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `voice_id`.\n"]
    pub fn set_voice_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().voice_id = Some(v.into());
        self
    }

    #[doc = "Set the field `abort_statement`.\n"]
    pub fn set_abort_statement(self, v: impl Into<BlockAssignable<LexBotAbortStatementEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().abort_statement = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.abort_statement = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `clarification_prompt`.\n"]
    pub fn set_clarification_prompt(self, v: impl Into<BlockAssignable<LexBotClarificationPromptEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().clarification_prompt = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.clarification_prompt = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `intent`.\n"]
    pub fn set_intent(self, v: impl Into<BlockAssignable<LexBotIntentEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().intent = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.intent = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<LexBotTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `checksum` after provisioning.\n"]
    pub fn checksum(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.checksum", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `child_directed` after provisioning.\n"]
    pub fn child_directed(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.child_directed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_version` after provisioning.\n"]
    pub fn create_version(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `detect_sentiment` after provisioning.\n"]
    pub fn detect_sentiment(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.detect_sentiment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_model_improvements` after provisioning.\n"]
    pub fn enable_model_improvements(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_model_improvements", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failure_reason` after provisioning.\n"]
    pub fn failure_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.failure_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idle_session_ttl_in_seconds` after provisioning.\n"]
    pub fn idle_session_ttl_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.idle_session_ttl_in_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locale` after provisioning.\n"]
    pub fn locale(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.locale", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nlu_intent_confidence_threshold` after provisioning.\n"]
    pub fn nlu_intent_confidence_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nlu_intent_confidence_threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `process_behavior` after provisioning.\n"]
    pub fn process_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.process_behavior", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `voice_id` after provisioning.\n"]
    pub fn voice_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.voice_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `abort_statement` after provisioning.\n"]
    pub fn abort_statement(&self) -> ListRef<LexBotAbortStatementElRef> {
        ListRef::new(self.shared().clone(), format!("{}.abort_statement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `clarification_prompt` after provisioning.\n"]
    pub fn clarification_prompt(&self) -> ListRef<LexBotClarificationPromptElRef> {
        ListRef::new(self.shared().clone(), format!("{}.clarification_prompt", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LexBotTimeoutsElRef {
        LexBotTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for LexBot {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for LexBot { }

impl ToListMappable for LexBot {
    type O = ListRef<LexBotRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for LexBot_ {
    fn extract_resource_type(&self) -> String {
        "aws_lex_bot".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLexBot {
    pub tf_id: String,
    #[doc = ""]
    pub child_directed: PrimField<bool>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildLexBot {
    pub fn build(self, stack: &mut Stack) -> LexBot {
        let out = LexBot(Rc::new(LexBot_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LexBotData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                child_directed: self.child_directed,
                create_version: core::default::Default::default(),
                description: core::default::Default::default(),
                detect_sentiment: core::default::Default::default(),
                enable_model_improvements: core::default::Default::default(),
                id: core::default::Default::default(),
                idle_session_ttl_in_seconds: core::default::Default::default(),
                locale: core::default::Default::default(),
                name: self.name,
                nlu_intent_confidence_threshold: core::default::Default::default(),
                process_behavior: core::default::Default::default(),
                region: core::default::Default::default(),
                voice_id: core::default::Default::default(),
                abort_statement: core::default::Default::default(),
                clarification_prompt: core::default::Default::default(),
                intent: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LexBotRef {
    shared: StackShared,
    base: String,
}

impl Ref for LexBotRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl LexBotRef {
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

    #[doc = "Get a reference to the value of field `checksum` after provisioning.\n"]
    pub fn checksum(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.checksum", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `child_directed` after provisioning.\n"]
    pub fn child_directed(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.child_directed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_version` after provisioning.\n"]
    pub fn create_version(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `detect_sentiment` after provisioning.\n"]
    pub fn detect_sentiment(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.detect_sentiment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_model_improvements` after provisioning.\n"]
    pub fn enable_model_improvements(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_model_improvements", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failure_reason` after provisioning.\n"]
    pub fn failure_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.failure_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idle_session_ttl_in_seconds` after provisioning.\n"]
    pub fn idle_session_ttl_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.idle_session_ttl_in_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locale` after provisioning.\n"]
    pub fn locale(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.locale", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nlu_intent_confidence_threshold` after provisioning.\n"]
    pub fn nlu_intent_confidence_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nlu_intent_confidence_threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `process_behavior` after provisioning.\n"]
    pub fn process_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.process_behavior", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `voice_id` after provisioning.\n"]
    pub fn voice_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.voice_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `abort_statement` after provisioning.\n"]
    pub fn abort_statement(&self) -> ListRef<LexBotAbortStatementElRef> {
        ListRef::new(self.shared().clone(), format!("{}.abort_statement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `clarification_prompt` after provisioning.\n"]
    pub fn clarification_prompt(&self) -> ListRef<LexBotClarificationPromptElRef> {
        ListRef::new(self.shared().clone(), format!("{}.clarification_prompt", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LexBotTimeoutsElRef {
        LexBotTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LexBotAbortStatementElMessageEl {
    content: PrimField<String>,
    content_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_number: Option<PrimField<f64>>,
}

impl LexBotAbortStatementElMessageEl {
    #[doc = "Set the field `group_number`.\n"]
    pub fn set_group_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.group_number = Some(v.into());
        self
    }
}

impl ToListMappable for LexBotAbortStatementElMessageEl {
    type O = BlockAssignable<LexBotAbortStatementElMessageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexBotAbortStatementElMessageEl {
    #[doc = ""]
    pub content: PrimField<String>,
    #[doc = ""]
    pub content_type: PrimField<String>,
}

impl BuildLexBotAbortStatementElMessageEl {
    pub fn build(self) -> LexBotAbortStatementElMessageEl {
        LexBotAbortStatementElMessageEl {
            content: self.content,
            content_type: self.content_type,
            group_number: core::default::Default::default(),
        }
    }
}

pub struct LexBotAbortStatementElMessageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LexBotAbortStatementElMessageElRef {
    fn new(shared: StackShared, base: String) -> LexBotAbortStatementElMessageElRef {
        LexBotAbortStatementElMessageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexBotAbortStatementElMessageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `group_number` after provisioning.\n"]
    pub fn group_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_number", self.base))
    }
}

#[derive(Serialize, Default)]
struct LexBotAbortStatementElDynamic {
    message: Option<DynamicBlock<LexBotAbortStatementElMessageEl>>,
}

#[derive(Serialize)]
pub struct LexBotAbortStatementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    response_card: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<Vec<LexBotAbortStatementElMessageEl>>,
    dynamic: LexBotAbortStatementElDynamic,
}

impl LexBotAbortStatementEl {
    #[doc = "Set the field `response_card`.\n"]
    pub fn set_response_card(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response_card = Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<BlockAssignable<LexBotAbortStatementElMessageEl>>) -> Self {
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
}

impl ToListMappable for LexBotAbortStatementEl {
    type O = BlockAssignable<LexBotAbortStatementEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexBotAbortStatementEl {}

impl BuildLexBotAbortStatementEl {
    pub fn build(self) -> LexBotAbortStatementEl {
        LexBotAbortStatementEl {
            response_card: core::default::Default::default(),
            message: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct LexBotAbortStatementElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LexBotAbortStatementElRef {
    fn new(shared: StackShared, base: String) -> LexBotAbortStatementElRef {
        LexBotAbortStatementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexBotAbortStatementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `response_card` after provisioning.\n"]
    pub fn response_card(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.response_card", self.base))
    }
}

#[derive(Serialize)]
pub struct LexBotClarificationPromptElMessageEl {
    content: PrimField<String>,
    content_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_number: Option<PrimField<f64>>,
}

impl LexBotClarificationPromptElMessageEl {
    #[doc = "Set the field `group_number`.\n"]
    pub fn set_group_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.group_number = Some(v.into());
        self
    }
}

impl ToListMappable for LexBotClarificationPromptElMessageEl {
    type O = BlockAssignable<LexBotClarificationPromptElMessageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexBotClarificationPromptElMessageEl {
    #[doc = ""]
    pub content: PrimField<String>,
    #[doc = ""]
    pub content_type: PrimField<String>,
}

impl BuildLexBotClarificationPromptElMessageEl {
    pub fn build(self) -> LexBotClarificationPromptElMessageEl {
        LexBotClarificationPromptElMessageEl {
            content: self.content,
            content_type: self.content_type,
            group_number: core::default::Default::default(),
        }
    }
}

pub struct LexBotClarificationPromptElMessageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LexBotClarificationPromptElMessageElRef {
    fn new(shared: StackShared, base: String) -> LexBotClarificationPromptElMessageElRef {
        LexBotClarificationPromptElMessageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexBotClarificationPromptElMessageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `group_number` after provisioning.\n"]
    pub fn group_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_number", self.base))
    }
}

#[derive(Serialize, Default)]
struct LexBotClarificationPromptElDynamic {
    message: Option<DynamicBlock<LexBotClarificationPromptElMessageEl>>,
}

#[derive(Serialize)]
pub struct LexBotClarificationPromptEl {
    max_attempts: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_card: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<Vec<LexBotClarificationPromptElMessageEl>>,
    dynamic: LexBotClarificationPromptElDynamic,
}

impl LexBotClarificationPromptEl {
    #[doc = "Set the field `response_card`.\n"]
    pub fn set_response_card(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response_card = Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<BlockAssignable<LexBotClarificationPromptElMessageEl>>) -> Self {
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
}

impl ToListMappable for LexBotClarificationPromptEl {
    type O = BlockAssignable<LexBotClarificationPromptEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexBotClarificationPromptEl {
    #[doc = ""]
    pub max_attempts: PrimField<f64>,
}

impl BuildLexBotClarificationPromptEl {
    pub fn build(self) -> LexBotClarificationPromptEl {
        LexBotClarificationPromptEl {
            max_attempts: self.max_attempts,
            response_card: core::default::Default::default(),
            message: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct LexBotClarificationPromptElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LexBotClarificationPromptElRef {
    fn new(shared: StackShared, base: String) -> LexBotClarificationPromptElRef {
        LexBotClarificationPromptElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexBotClarificationPromptElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_attempts` after provisioning.\n"]
    pub fn max_attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_attempts", self.base))
    }

    #[doc = "Get a reference to the value of field `response_card` after provisioning.\n"]
    pub fn response_card(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.response_card", self.base))
    }
}

#[derive(Serialize)]
pub struct LexBotIntentEl {
    intent_name: PrimField<String>,
    intent_version: PrimField<String>,
}

impl LexBotIntentEl { }

impl ToListMappable for LexBotIntentEl {
    type O = BlockAssignable<LexBotIntentEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexBotIntentEl {
    #[doc = ""]
    pub intent_name: PrimField<String>,
    #[doc = ""]
    pub intent_version: PrimField<String>,
}

impl BuildLexBotIntentEl {
    pub fn build(self) -> LexBotIntentEl {
        LexBotIntentEl {
            intent_name: self.intent_name,
            intent_version: self.intent_version,
        }
    }
}

pub struct LexBotIntentElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LexBotIntentElRef {
    fn new(shared: StackShared, base: String) -> LexBotIntentElRef {
        LexBotIntentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexBotIntentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `intent_name` after provisioning.\n"]
    pub fn intent_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.intent_name", self.base))
    }

    #[doc = "Get a reference to the value of field `intent_version` after provisioning.\n"]
    pub fn intent_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.intent_version", self.base))
    }
}

#[derive(Serialize)]
pub struct LexBotTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl LexBotTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for LexBotTimeoutsEl {
    type O = BlockAssignable<LexBotTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexBotTimeoutsEl {}

impl BuildLexBotTimeoutsEl {
    pub fn build(self) -> LexBotTimeoutsEl {
        LexBotTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct LexBotTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LexBotTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> LexBotTimeoutsElRef {
        LexBotTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexBotTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct LexBotDynamic {
    abort_statement: Option<DynamicBlock<LexBotAbortStatementEl>>,
    clarification_prompt: Option<DynamicBlock<LexBotClarificationPromptEl>>,
    intent: Option<DynamicBlock<LexBotIntentEl>>,
}
