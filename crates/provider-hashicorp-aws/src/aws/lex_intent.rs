use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct LexIntentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_version: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_intent_signature: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_utterances: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conclusion_statement: Option<Vec<LexIntentConclusionStatementEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confirmation_prompt: Option<Vec<LexIntentConfirmationPromptEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dialog_code_hook: Option<Vec<LexIntentDialogCodeHookEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    follow_up_prompt: Option<Vec<LexIntentFollowUpPromptEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fulfillment_activity: Option<Vec<LexIntentFulfillmentActivityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rejection_statement: Option<Vec<LexIntentRejectionStatementEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slot: Option<Vec<LexIntentSlotEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<LexIntentTimeoutsEl>,
    dynamic: LexIntentDynamic,
}
struct LexIntent_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LexIntentData>,
}
#[derive(Clone)]
pub struct LexIntent(Rc<LexIntent_>);
impl LexIntent {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `parent_intent_signature`.\n"]
    pub fn set_parent_intent_signature(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent_intent_signature = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `sample_utterances`.\n"]
    pub fn set_sample_utterances(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().sample_utterances = Some(v.into());
        self
    }
    #[doc = "Set the field `conclusion_statement`.\n"]
    pub fn set_conclusion_statement(
        self,
        v: impl Into<BlockAssignable<LexIntentConclusionStatementEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().conclusion_statement = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.conclusion_statement = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `confirmation_prompt`.\n"]
    pub fn set_confirmation_prompt(
        self,
        v: impl Into<BlockAssignable<LexIntentConfirmationPromptEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().confirmation_prompt = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.confirmation_prompt = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dialog_code_hook`.\n"]
    pub fn set_dialog_code_hook(
        self,
        v: impl Into<BlockAssignable<LexIntentDialogCodeHookEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dialog_code_hook = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dialog_code_hook = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `follow_up_prompt`.\n"]
    pub fn set_follow_up_prompt(
        self,
        v: impl Into<BlockAssignable<LexIntentFollowUpPromptEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().follow_up_prompt = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.follow_up_prompt = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `fulfillment_activity`.\n"]
    pub fn set_fulfillment_activity(
        self,
        v: impl Into<BlockAssignable<LexIntentFulfillmentActivityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().fulfillment_activity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.fulfillment_activity = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rejection_statement`.\n"]
    pub fn set_rejection_statement(
        self,
        v: impl Into<BlockAssignable<LexIntentRejectionStatementEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rejection_statement = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rejection_statement = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `slot`.\n"]
    pub fn set_slot(self, v: impl Into<BlockAssignable<LexIntentSlotEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().slot = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.slot = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<LexIntentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `checksum` after provisioning.\n"]
    pub fn checksum(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.checksum", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_version` after provisioning.\n"]
    pub fn create_version(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_updated_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent_intent_signature` after provisioning.\n"]
    pub fn parent_intent_signature(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_intent_signature", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sample_utterances` after provisioning.\n"]
    pub fn sample_utterances(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.sample_utterances", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `conclusion_statement` after provisioning.\n"]
    pub fn conclusion_statement(&self) -> ListRef<LexIntentConclusionStatementElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conclusion_statement", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `confirmation_prompt` after provisioning.\n"]
    pub fn confirmation_prompt(&self) -> ListRef<LexIntentConfirmationPromptElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.confirmation_prompt", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dialog_code_hook` after provisioning.\n"]
    pub fn dialog_code_hook(&self) -> ListRef<LexIntentDialogCodeHookElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dialog_code_hook", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `follow_up_prompt` after provisioning.\n"]
    pub fn follow_up_prompt(&self) -> ListRef<LexIntentFollowUpPromptElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.follow_up_prompt", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fulfillment_activity` after provisioning.\n"]
    pub fn fulfillment_activity(&self) -> ListRef<LexIntentFulfillmentActivityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fulfillment_activity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rejection_statement` after provisioning.\n"]
    pub fn rejection_statement(&self) -> ListRef<LexIntentRejectionStatementElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rejection_statement", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LexIntentTimeoutsElRef {
        LexIntentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for LexIntent {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for LexIntent {}
impl ToListMappable for LexIntent {
    type O = ListRef<LexIntentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for LexIntent_ {
    fn extract_resource_type(&self) -> String {
        "aws_lex_intent".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildLexIntent {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildLexIntent {
    pub fn build(self, stack: &mut Stack) -> LexIntent {
        let out = LexIntent(Rc::new(LexIntent_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LexIntentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                create_version: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                parent_intent_signature: core::default::Default::default(),
                region: core::default::Default::default(),
                sample_utterances: core::default::Default::default(),
                conclusion_statement: core::default::Default::default(),
                confirmation_prompt: core::default::Default::default(),
                dialog_code_hook: core::default::Default::default(),
                follow_up_prompt: core::default::Default::default(),
                fulfillment_activity: core::default::Default::default(),
                rejection_statement: core::default::Default::default(),
                slot: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct LexIntentRef {
    shared: StackShared,
    base: String,
}
impl Ref for LexIntentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl LexIntentRef {
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
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.checksum", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_version` after provisioning.\n"]
    pub fn create_version(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_updated_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent_intent_signature` after provisioning.\n"]
    pub fn parent_intent_signature(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_intent_signature", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sample_utterances` after provisioning.\n"]
    pub fn sample_utterances(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.sample_utterances", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `conclusion_statement` after provisioning.\n"]
    pub fn conclusion_statement(&self) -> ListRef<LexIntentConclusionStatementElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conclusion_statement", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `confirmation_prompt` after provisioning.\n"]
    pub fn confirmation_prompt(&self) -> ListRef<LexIntentConfirmationPromptElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.confirmation_prompt", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dialog_code_hook` after provisioning.\n"]
    pub fn dialog_code_hook(&self) -> ListRef<LexIntentDialogCodeHookElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dialog_code_hook", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `follow_up_prompt` after provisioning.\n"]
    pub fn follow_up_prompt(&self) -> ListRef<LexIntentFollowUpPromptElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.follow_up_prompt", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fulfillment_activity` after provisioning.\n"]
    pub fn fulfillment_activity(&self) -> ListRef<LexIntentFulfillmentActivityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fulfillment_activity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rejection_statement` after provisioning.\n"]
    pub fn rejection_statement(&self) -> ListRef<LexIntentRejectionStatementElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rejection_statement", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LexIntentTimeoutsElRef {
        LexIntentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct LexIntentConclusionStatementElMessageEl {
    content: PrimField<String>,
    content_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_number: Option<PrimField<f64>>,
}
impl LexIntentConclusionStatementElMessageEl {
    #[doc = "Set the field `group_number`.\n"]
    pub fn set_group_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.group_number = Some(v.into());
        self
    }
}
impl ToListMappable for LexIntentConclusionStatementElMessageEl {
    type O = BlockAssignable<LexIntentConclusionStatementElMessageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexIntentConclusionStatementElMessageEl {
    #[doc = ""]
    pub content: PrimField<String>,
    #[doc = ""]
    pub content_type: PrimField<String>,
}
impl BuildLexIntentConclusionStatementElMessageEl {
    pub fn build(self) -> LexIntentConclusionStatementElMessageEl {
        LexIntentConclusionStatementElMessageEl {
            content: self.content,
            content_type: self.content_type,
            group_number: core::default::Default::default(),
        }
    }
}
pub struct LexIntentConclusionStatementElMessageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LexIntentConclusionStatementElMessageElRef {
    fn new(shared: StackShared, base: String) -> LexIntentConclusionStatementElMessageElRef {
        LexIntentConclusionStatementElMessageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LexIntentConclusionStatementElMessageElRef {
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
struct LexIntentConclusionStatementElDynamic {
    message: Option<DynamicBlock<LexIntentConclusionStatementElMessageEl>>,
}
#[derive(Serialize)]
pub struct LexIntentConclusionStatementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    response_card: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<Vec<LexIntentConclusionStatementElMessageEl>>,
    dynamic: LexIntentConclusionStatementElDynamic,
}
impl LexIntentConclusionStatementEl {
    #[doc = "Set the field `response_card`.\n"]
    pub fn set_response_card(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response_card = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(
        mut self,
        v: impl Into<BlockAssignable<LexIntentConclusionStatementElMessageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.message = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.message = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for LexIntentConclusionStatementEl {
    type O = BlockAssignable<LexIntentConclusionStatementEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexIntentConclusionStatementEl {}
impl BuildLexIntentConclusionStatementEl {
    pub fn build(self) -> LexIntentConclusionStatementEl {
        LexIntentConclusionStatementEl {
            response_card: core::default::Default::default(),
            message: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct LexIntentConclusionStatementElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LexIntentConclusionStatementElRef {
    fn new(shared: StackShared, base: String) -> LexIntentConclusionStatementElRef {
        LexIntentConclusionStatementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LexIntentConclusionStatementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `response_card` after provisioning.\n"]
    pub fn response_card(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.response_card", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct LexIntentConfirmationPromptElMessageEl {
    content: PrimField<String>,
    content_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_number: Option<PrimField<f64>>,
}
impl LexIntentConfirmationPromptElMessageEl {
    #[doc = "Set the field `group_number`.\n"]
    pub fn set_group_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.group_number = Some(v.into());
        self
    }
}
impl ToListMappable for LexIntentConfirmationPromptElMessageEl {
    type O = BlockAssignable<LexIntentConfirmationPromptElMessageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexIntentConfirmationPromptElMessageEl {
    #[doc = ""]
    pub content: PrimField<String>,
    #[doc = ""]
    pub content_type: PrimField<String>,
}
impl BuildLexIntentConfirmationPromptElMessageEl {
    pub fn build(self) -> LexIntentConfirmationPromptElMessageEl {
        LexIntentConfirmationPromptElMessageEl {
            content: self.content,
            content_type: self.content_type,
            group_number: core::default::Default::default(),
        }
    }
}
pub struct LexIntentConfirmationPromptElMessageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LexIntentConfirmationPromptElMessageElRef {
    fn new(shared: StackShared, base: String) -> LexIntentConfirmationPromptElMessageElRef {
        LexIntentConfirmationPromptElMessageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LexIntentConfirmationPromptElMessageElRef {
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
struct LexIntentConfirmationPromptElDynamic {
    message: Option<DynamicBlock<LexIntentConfirmationPromptElMessageEl>>,
}
#[derive(Serialize)]
pub struct LexIntentConfirmationPromptEl {
    max_attempts: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_card: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<Vec<LexIntentConfirmationPromptElMessageEl>>,
    dynamic: LexIntentConfirmationPromptElDynamic,
}
impl LexIntentConfirmationPromptEl {
    #[doc = "Set the field `response_card`.\n"]
    pub fn set_response_card(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response_card = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(
        mut self,
        v: impl Into<BlockAssignable<LexIntentConfirmationPromptElMessageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.message = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.message = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for LexIntentConfirmationPromptEl {
    type O = BlockAssignable<LexIntentConfirmationPromptEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexIntentConfirmationPromptEl {
    #[doc = ""]
    pub max_attempts: PrimField<f64>,
}
impl BuildLexIntentConfirmationPromptEl {
    pub fn build(self) -> LexIntentConfirmationPromptEl {
        LexIntentConfirmationPromptEl {
            max_attempts: self.max_attempts,
            response_card: core::default::Default::default(),
            message: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct LexIntentConfirmationPromptElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LexIntentConfirmationPromptElRef {
    fn new(shared: StackShared, base: String) -> LexIntentConfirmationPromptElRef {
        LexIntentConfirmationPromptElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LexIntentConfirmationPromptElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_attempts` after provisioning.\n"]
    pub fn max_attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_attempts", self.base))
    }
    #[doc = "Get a reference to the value of field `response_card` after provisioning.\n"]
    pub fn response_card(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.response_card", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct LexIntentDialogCodeHookEl {
    message_version: PrimField<String>,
    uri: PrimField<String>,
}
impl LexIntentDialogCodeHookEl {}
impl ToListMappable for LexIntentDialogCodeHookEl {
    type O = BlockAssignable<LexIntentDialogCodeHookEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexIntentDialogCodeHookEl {
    #[doc = ""]
    pub message_version: PrimField<String>,
    #[doc = ""]
    pub uri: PrimField<String>,
}
impl BuildLexIntentDialogCodeHookEl {
    pub fn build(self) -> LexIntentDialogCodeHookEl {
        LexIntentDialogCodeHookEl {
            message_version: self.message_version,
            uri: self.uri,
        }
    }
}
pub struct LexIntentDialogCodeHookElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LexIntentDialogCodeHookElRef {
    fn new(shared: StackShared, base: String) -> LexIntentDialogCodeHookElRef {
        LexIntentDialogCodeHookElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LexIntentDialogCodeHookElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `message_version` after provisioning.\n"]
    pub fn message_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct LexIntentFollowUpPromptElPromptElMessageEl {
    content: PrimField<String>,
    content_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_number: Option<PrimField<f64>>,
}
impl LexIntentFollowUpPromptElPromptElMessageEl {
    #[doc = "Set the field `group_number`.\n"]
    pub fn set_group_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.group_number = Some(v.into());
        self
    }
}
impl ToListMappable for LexIntentFollowUpPromptElPromptElMessageEl {
    type O = BlockAssignable<LexIntentFollowUpPromptElPromptElMessageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexIntentFollowUpPromptElPromptElMessageEl {
    #[doc = ""]
    pub content: PrimField<String>,
    #[doc = ""]
    pub content_type: PrimField<String>,
}
impl BuildLexIntentFollowUpPromptElPromptElMessageEl {
    pub fn build(self) -> LexIntentFollowUpPromptElPromptElMessageEl {
        LexIntentFollowUpPromptElPromptElMessageEl {
            content: self.content,
            content_type: self.content_type,
            group_number: core::default::Default::default(),
        }
    }
}
pub struct LexIntentFollowUpPromptElPromptElMessageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LexIntentFollowUpPromptElPromptElMessageElRef {
    fn new(shared: StackShared, base: String) -> LexIntentFollowUpPromptElPromptElMessageElRef {
        LexIntentFollowUpPromptElPromptElMessageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LexIntentFollowUpPromptElPromptElMessageElRef {
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
struct LexIntentFollowUpPromptElPromptElDynamic {
    message: Option<DynamicBlock<LexIntentFollowUpPromptElPromptElMessageEl>>,
}
#[derive(Serialize)]
pub struct LexIntentFollowUpPromptElPromptEl {
    max_attempts: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_card: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<Vec<LexIntentFollowUpPromptElPromptElMessageEl>>,
    dynamic: LexIntentFollowUpPromptElPromptElDynamic,
}
impl LexIntentFollowUpPromptElPromptEl {
    #[doc = "Set the field `response_card`.\n"]
    pub fn set_response_card(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response_card = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(
        mut self,
        v: impl Into<BlockAssignable<LexIntentFollowUpPromptElPromptElMessageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.message = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.message = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for LexIntentFollowUpPromptElPromptEl {
    type O = BlockAssignable<LexIntentFollowUpPromptElPromptEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexIntentFollowUpPromptElPromptEl {
    #[doc = ""]
    pub max_attempts: PrimField<f64>,
}
impl BuildLexIntentFollowUpPromptElPromptEl {
    pub fn build(self) -> LexIntentFollowUpPromptElPromptEl {
        LexIntentFollowUpPromptElPromptEl {
            max_attempts: self.max_attempts,
            response_card: core::default::Default::default(),
            message: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct LexIntentFollowUpPromptElPromptElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LexIntentFollowUpPromptElPromptElRef {
    fn new(shared: StackShared, base: String) -> LexIntentFollowUpPromptElPromptElRef {
        LexIntentFollowUpPromptElPromptElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LexIntentFollowUpPromptElPromptElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_attempts` after provisioning.\n"]
    pub fn max_attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_attempts", self.base))
    }
    #[doc = "Get a reference to the value of field `response_card` after provisioning.\n"]
    pub fn response_card(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.response_card", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct LexIntentFollowUpPromptElRejectionStatementElMessageEl {
    content: PrimField<String>,
    content_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_number: Option<PrimField<f64>>,
}
impl LexIntentFollowUpPromptElRejectionStatementElMessageEl {
    #[doc = "Set the field `group_number`.\n"]
    pub fn set_group_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.group_number = Some(v.into());
        self
    }
}
impl ToListMappable for LexIntentFollowUpPromptElRejectionStatementElMessageEl {
    type O = BlockAssignable<LexIntentFollowUpPromptElRejectionStatementElMessageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexIntentFollowUpPromptElRejectionStatementElMessageEl {
    #[doc = ""]
    pub content: PrimField<String>,
    #[doc = ""]
    pub content_type: PrimField<String>,
}
impl BuildLexIntentFollowUpPromptElRejectionStatementElMessageEl {
    pub fn build(self) -> LexIntentFollowUpPromptElRejectionStatementElMessageEl {
        LexIntentFollowUpPromptElRejectionStatementElMessageEl {
            content: self.content,
            content_type: self.content_type,
            group_number: core::default::Default::default(),
        }
    }
}
pub struct LexIntentFollowUpPromptElRejectionStatementElMessageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LexIntentFollowUpPromptElRejectionStatementElMessageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LexIntentFollowUpPromptElRejectionStatementElMessageElRef {
        LexIntentFollowUpPromptElRejectionStatementElMessageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LexIntentFollowUpPromptElRejectionStatementElMessageElRef {
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
struct LexIntentFollowUpPromptElRejectionStatementElDynamic {
    message: Option<DynamicBlock<LexIntentFollowUpPromptElRejectionStatementElMessageEl>>,
}
#[derive(Serialize)]
pub struct LexIntentFollowUpPromptElRejectionStatementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    response_card: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<Vec<LexIntentFollowUpPromptElRejectionStatementElMessageEl>>,
    dynamic: LexIntentFollowUpPromptElRejectionStatementElDynamic,
}
impl LexIntentFollowUpPromptElRejectionStatementEl {
    #[doc = "Set the field `response_card`.\n"]
    pub fn set_response_card(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response_card = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(
        mut self,
        v: impl Into<BlockAssignable<LexIntentFollowUpPromptElRejectionStatementElMessageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.message = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.message = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for LexIntentFollowUpPromptElRejectionStatementEl {
    type O = BlockAssignable<LexIntentFollowUpPromptElRejectionStatementEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexIntentFollowUpPromptElRejectionStatementEl {}
impl BuildLexIntentFollowUpPromptElRejectionStatementEl {
    pub fn build(self) -> LexIntentFollowUpPromptElRejectionStatementEl {
        LexIntentFollowUpPromptElRejectionStatementEl {
            response_card: core::default::Default::default(),
            message: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct LexIntentFollowUpPromptElRejectionStatementElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LexIntentFollowUpPromptElRejectionStatementElRef {
    fn new(shared: StackShared, base: String) -> LexIntentFollowUpPromptElRejectionStatementElRef {
        LexIntentFollowUpPromptElRejectionStatementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LexIntentFollowUpPromptElRejectionStatementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `response_card` after provisioning.\n"]
    pub fn response_card(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.response_card", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct LexIntentFollowUpPromptElDynamic {
    prompt: Option<DynamicBlock<LexIntentFollowUpPromptElPromptEl>>,
    rejection_statement: Option<DynamicBlock<LexIntentFollowUpPromptElRejectionStatementEl>>,
}
#[derive(Serialize)]
pub struct LexIntentFollowUpPromptEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt: Option<Vec<LexIntentFollowUpPromptElPromptEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rejection_statement: Option<Vec<LexIntentFollowUpPromptElRejectionStatementEl>>,
    dynamic: LexIntentFollowUpPromptElDynamic,
}
impl LexIntentFollowUpPromptEl {
    #[doc = "Set the field `prompt`.\n"]
    pub fn set_prompt(
        mut self,
        v: impl Into<BlockAssignable<LexIntentFollowUpPromptElPromptEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.prompt = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.prompt = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rejection_statement`.\n"]
    pub fn set_rejection_statement(
        mut self,
        v: impl Into<BlockAssignable<LexIntentFollowUpPromptElRejectionStatementEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rejection_statement = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rejection_statement = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for LexIntentFollowUpPromptEl {
    type O = BlockAssignable<LexIntentFollowUpPromptEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexIntentFollowUpPromptEl {}
impl BuildLexIntentFollowUpPromptEl {
    pub fn build(self) -> LexIntentFollowUpPromptEl {
        LexIntentFollowUpPromptEl {
            prompt: core::default::Default::default(),
            rejection_statement: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct LexIntentFollowUpPromptElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LexIntentFollowUpPromptElRef {
    fn new(shared: StackShared, base: String) -> LexIntentFollowUpPromptElRef {
        LexIntentFollowUpPromptElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LexIntentFollowUpPromptElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `prompt` after provisioning.\n"]
    pub fn prompt(&self) -> ListRef<LexIntentFollowUpPromptElPromptElRef> {
        ListRef::new(self.shared().clone(), format!("{}.prompt", self.base))
    }
    #[doc = "Get a reference to the value of field `rejection_statement` after provisioning.\n"]
    pub fn rejection_statement(&self) -> ListRef<LexIntentFollowUpPromptElRejectionStatementElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rejection_statement", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct LexIntentFulfillmentActivityElCodeHookEl {
    message_version: PrimField<String>,
    uri: PrimField<String>,
}
impl LexIntentFulfillmentActivityElCodeHookEl {}
impl ToListMappable for LexIntentFulfillmentActivityElCodeHookEl {
    type O = BlockAssignable<LexIntentFulfillmentActivityElCodeHookEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexIntentFulfillmentActivityElCodeHookEl {
    #[doc = ""]
    pub message_version: PrimField<String>,
    #[doc = ""]
    pub uri: PrimField<String>,
}
impl BuildLexIntentFulfillmentActivityElCodeHookEl {
    pub fn build(self) -> LexIntentFulfillmentActivityElCodeHookEl {
        LexIntentFulfillmentActivityElCodeHookEl {
            message_version: self.message_version,
            uri: self.uri,
        }
    }
}
pub struct LexIntentFulfillmentActivityElCodeHookElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LexIntentFulfillmentActivityElCodeHookElRef {
    fn new(shared: StackShared, base: String) -> LexIntentFulfillmentActivityElCodeHookElRef {
        LexIntentFulfillmentActivityElCodeHookElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LexIntentFulfillmentActivityElCodeHookElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `message_version` after provisioning.\n"]
    pub fn message_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct LexIntentFulfillmentActivityElDynamic {
    code_hook: Option<DynamicBlock<LexIntentFulfillmentActivityElCodeHookEl>>,
}
#[derive(Serialize)]
pub struct LexIntentFulfillmentActivityEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_hook: Option<Vec<LexIntentFulfillmentActivityElCodeHookEl>>,
    dynamic: LexIntentFulfillmentActivityElDynamic,
}
impl LexIntentFulfillmentActivityEl {
    #[doc = "Set the field `code_hook`.\n"]
    pub fn set_code_hook(
        mut self,
        v: impl Into<BlockAssignable<LexIntentFulfillmentActivityElCodeHookEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.code_hook = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.code_hook = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for LexIntentFulfillmentActivityEl {
    type O = BlockAssignable<LexIntentFulfillmentActivityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexIntentFulfillmentActivityEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildLexIntentFulfillmentActivityEl {
    pub fn build(self) -> LexIntentFulfillmentActivityEl {
        LexIntentFulfillmentActivityEl {
            type_: self.type_,
            code_hook: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct LexIntentFulfillmentActivityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LexIntentFulfillmentActivityElRef {
    fn new(shared: StackShared, base: String) -> LexIntentFulfillmentActivityElRef {
        LexIntentFulfillmentActivityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LexIntentFulfillmentActivityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `code_hook` after provisioning.\n"]
    pub fn code_hook(&self) -> ListRef<LexIntentFulfillmentActivityElCodeHookElRef> {
        ListRef::new(self.shared().clone(), format!("{}.code_hook", self.base))
    }
}
#[derive(Serialize)]
pub struct LexIntentRejectionStatementElMessageEl {
    content: PrimField<String>,
    content_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_number: Option<PrimField<f64>>,
}
impl LexIntentRejectionStatementElMessageEl {
    #[doc = "Set the field `group_number`.\n"]
    pub fn set_group_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.group_number = Some(v.into());
        self
    }
}
impl ToListMappable for LexIntentRejectionStatementElMessageEl {
    type O = BlockAssignable<LexIntentRejectionStatementElMessageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexIntentRejectionStatementElMessageEl {
    #[doc = ""]
    pub content: PrimField<String>,
    #[doc = ""]
    pub content_type: PrimField<String>,
}
impl BuildLexIntentRejectionStatementElMessageEl {
    pub fn build(self) -> LexIntentRejectionStatementElMessageEl {
        LexIntentRejectionStatementElMessageEl {
            content: self.content,
            content_type: self.content_type,
            group_number: core::default::Default::default(),
        }
    }
}
pub struct LexIntentRejectionStatementElMessageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LexIntentRejectionStatementElMessageElRef {
    fn new(shared: StackShared, base: String) -> LexIntentRejectionStatementElMessageElRef {
        LexIntentRejectionStatementElMessageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LexIntentRejectionStatementElMessageElRef {
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
struct LexIntentRejectionStatementElDynamic {
    message: Option<DynamicBlock<LexIntentRejectionStatementElMessageEl>>,
}
#[derive(Serialize)]
pub struct LexIntentRejectionStatementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    response_card: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<Vec<LexIntentRejectionStatementElMessageEl>>,
    dynamic: LexIntentRejectionStatementElDynamic,
}
impl LexIntentRejectionStatementEl {
    #[doc = "Set the field `response_card`.\n"]
    pub fn set_response_card(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response_card = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(
        mut self,
        v: impl Into<BlockAssignable<LexIntentRejectionStatementElMessageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.message = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.message = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for LexIntentRejectionStatementEl {
    type O = BlockAssignable<LexIntentRejectionStatementEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexIntentRejectionStatementEl {}
impl BuildLexIntentRejectionStatementEl {
    pub fn build(self) -> LexIntentRejectionStatementEl {
        LexIntentRejectionStatementEl {
            response_card: core::default::Default::default(),
            message: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct LexIntentRejectionStatementElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LexIntentRejectionStatementElRef {
    fn new(shared: StackShared, base: String) -> LexIntentRejectionStatementElRef {
        LexIntentRejectionStatementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LexIntentRejectionStatementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `response_card` after provisioning.\n"]
    pub fn response_card(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.response_card", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct LexIntentSlotElValueElicitationPromptElMessageEl {
    content: PrimField<String>,
    content_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_number: Option<PrimField<f64>>,
}
impl LexIntentSlotElValueElicitationPromptElMessageEl {
    #[doc = "Set the field `group_number`.\n"]
    pub fn set_group_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.group_number = Some(v.into());
        self
    }
}
impl ToListMappable for LexIntentSlotElValueElicitationPromptElMessageEl {
    type O = BlockAssignable<LexIntentSlotElValueElicitationPromptElMessageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexIntentSlotElValueElicitationPromptElMessageEl {
    #[doc = ""]
    pub content: PrimField<String>,
    #[doc = ""]
    pub content_type: PrimField<String>,
}
impl BuildLexIntentSlotElValueElicitationPromptElMessageEl {
    pub fn build(self) -> LexIntentSlotElValueElicitationPromptElMessageEl {
        LexIntentSlotElValueElicitationPromptElMessageEl {
            content: self.content,
            content_type: self.content_type,
            group_number: core::default::Default::default(),
        }
    }
}
pub struct LexIntentSlotElValueElicitationPromptElMessageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LexIntentSlotElValueElicitationPromptElMessageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LexIntentSlotElValueElicitationPromptElMessageElRef {
        LexIntentSlotElValueElicitationPromptElMessageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LexIntentSlotElValueElicitationPromptElMessageElRef {
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
struct LexIntentSlotElValueElicitationPromptElDynamic {
    message: Option<DynamicBlock<LexIntentSlotElValueElicitationPromptElMessageEl>>,
}
#[derive(Serialize)]
pub struct LexIntentSlotElValueElicitationPromptEl {
    max_attempts: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_card: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<Vec<LexIntentSlotElValueElicitationPromptElMessageEl>>,
    dynamic: LexIntentSlotElValueElicitationPromptElDynamic,
}
impl LexIntentSlotElValueElicitationPromptEl {
    #[doc = "Set the field `response_card`.\n"]
    pub fn set_response_card(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response_card = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(
        mut self,
        v: impl Into<BlockAssignable<LexIntentSlotElValueElicitationPromptElMessageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.message = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.message = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for LexIntentSlotElValueElicitationPromptEl {
    type O = BlockAssignable<LexIntentSlotElValueElicitationPromptEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexIntentSlotElValueElicitationPromptEl {
    #[doc = ""]
    pub max_attempts: PrimField<f64>,
}
impl BuildLexIntentSlotElValueElicitationPromptEl {
    pub fn build(self) -> LexIntentSlotElValueElicitationPromptEl {
        LexIntentSlotElValueElicitationPromptEl {
            max_attempts: self.max_attempts,
            response_card: core::default::Default::default(),
            message: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct LexIntentSlotElValueElicitationPromptElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LexIntentSlotElValueElicitationPromptElRef {
    fn new(shared: StackShared, base: String) -> LexIntentSlotElValueElicitationPromptElRef {
        LexIntentSlotElValueElicitationPromptElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LexIntentSlotElValueElicitationPromptElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_attempts` after provisioning.\n"]
    pub fn max_attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_attempts", self.base))
    }
    #[doc = "Get a reference to the value of field `response_card` after provisioning.\n"]
    pub fn response_card(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.response_card", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct LexIntentSlotElDynamic {
    value_elicitation_prompt: Option<DynamicBlock<LexIntentSlotElValueElicitationPromptEl>>,
}
#[derive(Serialize)]
pub struct LexIntentSlotEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_card: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_utterances: Option<ListField<PrimField<String>>>,
    slot_constraint: PrimField<String>,
    slot_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slot_type_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_elicitation_prompt: Option<Vec<LexIntentSlotElValueElicitationPromptEl>>,
    dynamic: LexIntentSlotElDynamic,
}
impl LexIntentSlotEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.priority = Some(v.into());
        self
    }
    #[doc = "Set the field `response_card`.\n"]
    pub fn set_response_card(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response_card = Some(v.into());
        self
    }
    #[doc = "Set the field `sample_utterances`.\n"]
    pub fn set_sample_utterances(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.sample_utterances = Some(v.into());
        self
    }
    #[doc = "Set the field `slot_type_version`.\n"]
    pub fn set_slot_type_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.slot_type_version = Some(v.into());
        self
    }
    #[doc = "Set the field `value_elicitation_prompt`.\n"]
    pub fn set_value_elicitation_prompt(
        mut self,
        v: impl Into<BlockAssignable<LexIntentSlotElValueElicitationPromptEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.value_elicitation_prompt = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.value_elicitation_prompt = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for LexIntentSlotEl {
    type O = BlockAssignable<LexIntentSlotEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexIntentSlotEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub slot_constraint: PrimField<String>,
    #[doc = ""]
    pub slot_type: PrimField<String>,
}
impl BuildLexIntentSlotEl {
    pub fn build(self) -> LexIntentSlotEl {
        LexIntentSlotEl {
            description: core::default::Default::default(),
            name: self.name,
            priority: core::default::Default::default(),
            response_card: core::default::Default::default(),
            sample_utterances: core::default::Default::default(),
            slot_constraint: self.slot_constraint,
            slot_type: self.slot_type,
            slot_type_version: core::default::Default::default(),
            value_elicitation_prompt: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct LexIntentSlotElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LexIntentSlotElRef {
    fn new(shared: StackShared, base: String) -> LexIntentSlotElRef {
        LexIntentSlotElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LexIntentSlotElRef {
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
    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
    #[doc = "Get a reference to the value of field `response_card` after provisioning.\n"]
    pub fn response_card(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.response_card", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sample_utterances` after provisioning.\n"]
    pub fn sample_utterances(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sample_utterances", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `slot_constraint` after provisioning.\n"]
    pub fn slot_constraint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.slot_constraint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `slot_type` after provisioning.\n"]
    pub fn slot_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slot_type", self.base))
    }
    #[doc = "Get a reference to the value of field `slot_type_version` after provisioning.\n"]
    pub fn slot_type_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.slot_type_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `value_elicitation_prompt` after provisioning.\n"]
    pub fn value_elicitation_prompt(&self) -> ListRef<LexIntentSlotElValueElicitationPromptElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.value_elicitation_prompt", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct LexIntentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl LexIntentTimeoutsEl {
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
impl ToListMappable for LexIntentTimeoutsEl {
    type O = BlockAssignable<LexIntentTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexIntentTimeoutsEl {}
impl BuildLexIntentTimeoutsEl {
    pub fn build(self) -> LexIntentTimeoutsEl {
        LexIntentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct LexIntentTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LexIntentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> LexIntentTimeoutsElRef {
        LexIntentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LexIntentTimeoutsElRef {
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
struct LexIntentDynamic {
    conclusion_statement: Option<DynamicBlock<LexIntentConclusionStatementEl>>,
    confirmation_prompt: Option<DynamicBlock<LexIntentConfirmationPromptEl>>,
    dialog_code_hook: Option<DynamicBlock<LexIntentDialogCodeHookEl>>,
    follow_up_prompt: Option<DynamicBlock<LexIntentFollowUpPromptEl>>,
    fulfillment_activity: Option<DynamicBlock<LexIntentFulfillmentActivityEl>>,
    rejection_statement: Option<DynamicBlock<LexIntentRejectionStatementEl>>,
    slot: Option<DynamicBlock<LexIntentSlotEl>>,
}
