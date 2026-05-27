use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DialogflowCxTestCaseData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notes: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    test_case_conversation_turns: Option<Vec<DialogflowCxTestCaseTestCaseConversationTurnsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    test_config: Option<Vec<DialogflowCxTestCaseTestConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DialogflowCxTestCaseTimeoutsEl>,
    dynamic: DialogflowCxTestCaseDynamic,
}
struct DialogflowCxTestCase_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DialogflowCxTestCaseData>,
}
#[derive(Clone)]
pub struct DialogflowCxTestCase(Rc<DialogflowCxTestCase_>);
impl DialogflowCxTestCase {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `notes`.\nAdditional freeform notes about the test case. Limit of 400 characters."]
    pub fn set_notes(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().notes = Some(v.into());
        self
    }
    #[doc = "Set the field `parent`.\nThe agent to create the test case for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn set_parent(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\nTags are short descriptions that users may apply to test cases for organizational and filtering purposes.\nEach tag should start with \"#\" and has a limit of 30 characters"]
    pub fn set_tags(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `test_case_conversation_turns`.\n"]
    pub fn set_test_case_conversation_turns(
        self,
        v: impl Into<BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().test_case_conversation_turns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .test_case_conversation_turns = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `test_config`.\n"]
    pub fn set_test_config(
        self,
        v: impl Into<BlockAssignable<DialogflowCxTestCaseTestConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().test_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.test_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DialogflowCxTestCaseTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\nWhen the test was created. A timestamp in RFC3339 text format."]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the test case, unique within the agent. Limit of 200 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_test_result` after provisioning.\nThe latest test result."]
    pub fn last_test_result(&self) -> ListRef<DialogflowCxTestCaseLastTestResultElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.last_test_result", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the test case.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/testCases/<TestCase ID>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notes` after provisioning.\nAdditional freeform notes about the test case. Limit of 400 characters."]
    pub fn notes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe agent to create the test case for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nTags are short descriptions that users may apply to test cases for organizational and filtering purposes.\nEach tag should start with \"#\" and has a limit of 30 characters"]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `test_case_conversation_turns` after provisioning.\n"]
    pub fn test_case_conversation_turns(
        &self,
    ) -> ListRef<DialogflowCxTestCaseTestCaseConversationTurnsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.test_case_conversation_turns", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `test_config` after provisioning.\n"]
    pub fn test_config(&self) -> ListRef<DialogflowCxTestCaseTestConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.test_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxTestCaseTimeoutsElRef {
        DialogflowCxTestCaseTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DialogflowCxTestCase {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DialogflowCxTestCase {}
impl ToListMappable for DialogflowCxTestCase {
    type O = ListRef<DialogflowCxTestCaseRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DialogflowCxTestCase_ {
    fn extract_resource_type(&self) -> String {
        "google_dialogflow_cx_test_case".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDialogflowCxTestCase {
    pub tf_id: String,
    #[doc = "The human-readable name of the test case, unique within the agent. Limit of 200 characters."]
    pub display_name: PrimField<String>,
}
impl BuildDialogflowCxTestCase {
    pub fn build(self, stack: &mut Stack) -> DialogflowCxTestCase {
        let out = DialogflowCxTestCase(Rc::new(DialogflowCxTestCase_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DialogflowCxTestCaseData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                notes: core::default::Default::default(),
                parent: core::default::Default::default(),
                tags: core::default::Default::default(),
                test_case_conversation_turns: core::default::Default::default(),
                test_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DialogflowCxTestCaseRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxTestCaseRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DialogflowCxTestCaseRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\nWhen the test was created. A timestamp in RFC3339 text format."]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the test case, unique within the agent. Limit of 200 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_test_result` after provisioning.\nThe latest test result."]
    pub fn last_test_result(&self) -> ListRef<DialogflowCxTestCaseLastTestResultElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.last_test_result", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the test case.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/testCases/<TestCase ID>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notes` after provisioning.\nAdditional freeform notes about the test case. Limit of 400 characters."]
    pub fn notes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe agent to create the test case for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nTags are short descriptions that users may apply to test cases for organizational and filtering purposes.\nEach tag should start with \"#\" and has a limit of 30 characters"]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `test_case_conversation_turns` after provisioning.\n"]
    pub fn test_case_conversation_turns(
        &self,
    ) -> ListRef<DialogflowCxTestCaseTestCaseConversationTurnsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.test_case_conversation_turns", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `test_config` after provisioning.\n"]
    pub fn test_config(&self) -> ListRef<DialogflowCxTestCaseTestConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.test_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxTestCaseTimeoutsElRef {
        DialogflowCxTestCaseTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    digits: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    finish_digit: Option<PrimField<String>>,
}
impl DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfEl {
    #[doc = "Set the field `digits`.\n"]
    pub fn set_digits(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.digits = Some(v.into());
        self
    }
    #[doc = "Set the field `finish_digit`.\n"]
    pub fn set_finish_digit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.finish_digit = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfEl
{
    type O = BlockAssignable<
        DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfEl {}
impl BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfEl {
    pub fn build(
        self,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfEl {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfEl {
            digits: core::default::Default::default(),
            finish_digit: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfElRef {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `digits` after provisioning.\n"]
    pub fn digits(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.digits", self.base))
    }
    #[doc = "Get a reference to the value of field `finish_digit` after provisioning.\n"]
    pub fn finish_digit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.finish_digit", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    event: Option<PrimField<String>>,
}
impl DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventEl {
    #[doc = "Set the field `event`.\n"]
    pub fn set_event(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.event = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventEl
{
    type O = BlockAssignable<
        DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventEl {
}
impl BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventEl {
    pub fn build(
        self,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventEl {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventEl {
            event: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventElRef {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `event` after provisioning.\n"]
    pub fn event(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
}
impl DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextEl {
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.text = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextEl
{
    type O = BlockAssignable<
        DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextEl {}
impl BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextEl {
    pub fn build(
        self,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextEl {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextEl {
            text: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextElRef {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dtmf: Option<
        ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    event: Option<
        ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<
        ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextEl>,
    >,
}
impl DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputEl {
    #[doc = "Set the field `dtmf`.\n"]
    pub fn set_dtmf(
        mut self,
        v: impl Into<
            ListField<
                DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfEl,
            >,
        >,
    ) -> Self {
        self.dtmf = Some(v.into());
        self
    }
    #[doc = "Set the field `event`.\n"]
    pub fn set_event(
        mut self,
        v: impl Into<
            ListField<
                DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventEl,
            >,
        >,
    ) -> Self {
        self.event = Some(v.into());
        self
    }
    #[doc = "Set the field `language_code`.\n"]
    pub fn set_language_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.language_code = Some(v.into());
        self
    }
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(
        mut self,
        v: impl Into<
            ListField<
                DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextEl,
            >,
        >,
    ) -> Self {
        self.text = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputEl {
    type O =
        BlockAssignable<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputEl {}
impl BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputEl {
    pub fn build(
        self,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputEl {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputEl {
            dtmf: core::default::Default::default(),
            event: core::default::Default::default(),
            language_code: core::default::Default::default(),
            text: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElRef {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dtmf` after provisioning.\n"]
    pub fn dtmf(
        &self,
    ) -> ListRef<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.dtmf", self.base))
    }
    #[doc = "Get a reference to the value of field `event` after provisioning.\n"]
    pub fn event(
        &self,
    ) -> ListRef<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.event", self.base))
    }
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(
        &self,
    ) -> ListRef<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_sentiment_analysis: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    injected_parameters: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<
        ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_webhook_enabled: Option<PrimField<bool>>,
}
impl DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputEl {
    #[doc = "Set the field `enable_sentiment_analysis`.\n"]
    pub fn set_enable_sentiment_analysis(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_sentiment_analysis = Some(v.into());
        self
    }
    #[doc = "Set the field `injected_parameters`.\n"]
    pub fn set_injected_parameters(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.injected_parameters = Some(v.into());
        self
    }
    #[doc = "Set the field `input`.\n"]
    pub fn set_input(
        mut self,
        v: impl Into<
            ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputEl>,
        >,
    ) -> Self {
        self.input = Some(v.into());
        self
    }
    #[doc = "Set the field `is_webhook_enabled`.\n"]
    pub fn set_is_webhook_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_webhook_enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputEl {
    type O = BlockAssignable<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputEl {}
impl BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputEl {
    pub fn build(self) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputEl {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputEl {
            enable_sentiment_analysis: core::default::Default::default(),
            injected_parameters: core::default::Default::default(),
            input: core::default::Default::default(),
            is_webhook_enabled: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElRef {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_sentiment_analysis` after provisioning.\n"]
    pub fn enable_sentiment_analysis(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_sentiment_analysis", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `injected_parameters` after provisioning.\n"]
    pub fn injected_parameters(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.injected_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(
        &self,
    ) -> ListRef<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.input", self.base))
    }
    #[doc = "Get a reference to the value of field `is_webhook_enabled` after provisioning.\n"]
    pub fn is_webhook_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_webhook_enabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageEl {
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageEl
{
    type O = BlockAssignable<
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageEl
{}
impl BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageEl {
    pub fn build(
        self,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageEl
    {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageEl {
            display_name: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageElRef
    {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageElRef { shared : shared , base : base . to_string () , }
    }
}
impl DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesEl
{
    type O = BlockAssignable<
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesEl
{}
impl BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesEl {
    pub fn build(
        self,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesEl
    {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesEl {
            description: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesElRef
    {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesElRef { shared : shared , base : base . to_string () , }
    }
}
impl DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.details = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusEl
{
    type O = BlockAssignable<
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusEl
{}
impl BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusEl {
    pub fn build(
        self,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusEl {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusEl {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusElRef
    {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.details", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<ListField<PrimField<String>>>,
}
impl DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesEl {
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.text = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesEl
{
    type O = BlockAssignable<
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesEl
{}
impl
    BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesEl
{
    pub fn build(
        self,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesEl
    {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesEl {
            text: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesElRef { fn new (shared : StackShared , base : String) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesElRef { DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentEl {
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentEl
{
    type O = BlockAssignable < DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentEl
{}
impl BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentEl { pub fn build (self) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentEl { DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentEl { display_name : core :: default :: Default :: default () , name : core :: default :: Default :: default () , } } }
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentElRef { fn new (shared : StackShared , base : String) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentElRef { DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentElRef { shared : shared , base : base . to_string () , } } }
impl
    DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputEl { # [serde (skip_serializing_if = "Option::is_none")] current_page : Option < ListField < DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageEl > > , # [serde (skip_serializing_if = "Option::is_none")] differences : Option < ListField < DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesEl > > , # [serde (skip_serializing_if = "Option::is_none")] session_parameters : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] status : Option < ListField < DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusEl > > , # [serde (skip_serializing_if = "Option::is_none")] text_responses : Option < ListField < DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesEl > > , # [serde (skip_serializing_if = "Option::is_none")] triggered_intent : Option < ListField < DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentEl > > , }
impl DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputEl {
    #[doc = "Set the field `current_page`.\n"]
    pub fn set_current_page(
        mut self,
        v : impl Into < ListField < DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageEl > >,
    ) -> Self {
        self.current_page = Some(v.into());
        self
    }
    #[doc = "Set the field `differences`.\n"]
    pub fn set_differences(
        mut self,
        v : impl Into < ListField < DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesEl > >,
    ) -> Self {
        self.differences = Some(v.into());
        self
    }
    #[doc = "Set the field `session_parameters`.\n"]
    pub fn set_session_parameters(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.session_parameters = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(
        mut self,
        v: impl Into<
            ListField<
                DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusEl,
            >,
        >,
    ) -> Self {
        self.status = Some(v.into());
        self
    }
    #[doc = "Set the field `text_responses`.\n"]
    pub fn set_text_responses(
        mut self,
        v : impl Into < ListField < DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesEl > >,
    ) -> Self {
        self.text_responses = Some(v.into());
        self
    }
    #[doc = "Set the field `triggered_intent`.\n"]
    pub fn set_triggered_intent(
        mut self,
        v : impl Into < ListField < DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentEl > >,
    ) -> Self {
        self.triggered_intent = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputEl
{
    type O = BlockAssignable<
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputEl {}
impl BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputEl {
    pub fn build(
        self,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputEl {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputEl {
            current_page: core::default::Default::default(),
            differences: core::default::Default::default(),
            session_parameters: core::default::Default::default(),
            status: core::default::Default::default(),
            text_responses: core::default::Default::default(),
            triggered_intent: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElRef {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `current_page` after provisioning.\n"]
    pub fn current_page(
        &self,
    ) -> ListRef<
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.current_page", self.base))
    }
    #[doc = "Get a reference to the value of field `differences` after provisioning.\n"]
    pub fn differences(
        &self,
    ) -> ListRef<
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.differences", self.base))
    }
    #[doc = "Get a reference to the value of field `session_parameters` after provisioning.\n"]
    pub fn session_parameters(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(
        &self,
    ) -> ListRef<
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.status", self.base))
    }
    #[doc = "Get a reference to the value of field `text_responses` after provisioning.\n"]    pub fn text_responses (& self) -> ListRef < DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.text_responses", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `triggered_intent` after provisioning.\n"]    pub fn triggered_intent (& self) -> ListRef < DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.triggered_intent", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    user_input:
        Option<ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_agent_output: Option<
        ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputEl>,
    >,
}
impl DialogflowCxTestCaseLastTestResultElConversationTurnsEl {
    #[doc = "Set the field `user_input`.\n"]
    pub fn set_user_input(
        mut self,
        v: impl Into<ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputEl>>,
    ) -> Self {
        self.user_input = Some(v.into());
        self
    }
    #[doc = "Set the field `virtual_agent_output`.\n"]
    pub fn set_virtual_agent_output(
        mut self,
        v: impl Into<
            ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputEl>,
        >,
    ) -> Self {
        self.virtual_agent_output = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxTestCaseLastTestResultElConversationTurnsEl {
    type O = BlockAssignable<DialogflowCxTestCaseLastTestResultElConversationTurnsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxTestCaseLastTestResultElConversationTurnsEl {}
impl BuildDialogflowCxTestCaseLastTestResultElConversationTurnsEl {
    pub fn build(self) -> DialogflowCxTestCaseLastTestResultElConversationTurnsEl {
        DialogflowCxTestCaseLastTestResultElConversationTurnsEl {
            user_input: core::default::Default::default(),
            virtual_agent_output: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxTestCaseLastTestResultElConversationTurnsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElRef {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxTestCaseLastTestResultElConversationTurnsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `user_input` after provisioning.\n"]
    pub fn user_input(
        &self,
    ) -> ListRef<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.user_input", self.base))
    }
    #[doc = "Get a reference to the value of field `virtual_agent_output` after provisioning.\n"]
    pub fn virtual_agent_output(
        &self,
    ) -> ListRef<DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.virtual_agent_output", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxTestCaseLastTestResultEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    conversation_turns: Option<ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    test_result: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    test_time: Option<PrimField<String>>,
}
impl DialogflowCxTestCaseLastTestResultEl {
    #[doc = "Set the field `conversation_turns`.\n"]
    pub fn set_conversation_turns(
        mut self,
        v: impl Into<ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsEl>>,
    ) -> Self {
        self.conversation_turns = Some(v.into());
        self
    }
    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.environment = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `test_result`.\n"]
    pub fn set_test_result(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.test_result = Some(v.into());
        self
    }
    #[doc = "Set the field `test_time`.\n"]
    pub fn set_test_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.test_time = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxTestCaseLastTestResultEl {
    type O = BlockAssignable<DialogflowCxTestCaseLastTestResultEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxTestCaseLastTestResultEl {}
impl BuildDialogflowCxTestCaseLastTestResultEl {
    pub fn build(self) -> DialogflowCxTestCaseLastTestResultEl {
        DialogflowCxTestCaseLastTestResultEl {
            conversation_turns: core::default::Default::default(),
            environment: core::default::Default::default(),
            name: core::default::Default::default(),
            test_result: core::default::Default::default(),
            test_time: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxTestCaseLastTestResultElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxTestCaseLastTestResultElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxTestCaseLastTestResultElRef {
        DialogflowCxTestCaseLastTestResultElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxTestCaseLastTestResultElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `conversation_turns` after provisioning.\n"]
    pub fn conversation_turns(
        &self,
    ) -> ListRef<DialogflowCxTestCaseLastTestResultElConversationTurnsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conversation_turns", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `test_result` after provisioning.\n"]
    pub fn test_result(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.test_result", self.base))
    }
    #[doc = "Get a reference to the value of field `test_time` after provisioning.\n"]
    pub fn test_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.test_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    digits: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    finish_digit: Option<PrimField<String>>,
}
impl DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfEl {
    #[doc = "Set the field `digits`.\nThe dtmf digits."]
    pub fn set_digits(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.digits = Some(v.into());
        self
    }
    #[doc = "Set the field `finish_digit`.\nThe finish digit (if any)."]
    pub fn set_finish_digit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.finish_digit = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfEl {
    type O =
        BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfEl {}
impl BuildDialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfEl {
    pub fn build(self) -> DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfEl {
        DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfEl {
            digits: core::default::Default::default(),
            finish_digit: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfElRef {
        DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `digits` after provisioning.\nThe dtmf digits."]
    pub fn digits(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.digits", self.base))
    }
    #[doc = "Get a reference to the value of field `finish_digit` after provisioning.\nThe finish digit (if any)."]
    pub fn finish_digit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.finish_digit", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventEl {
    event: PrimField<String>,
}
impl DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventEl {}
impl ToListMappable for DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventEl {
    type O =
        BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventEl {
    #[doc = "Name of the event."]
    pub event: PrimField<String>,
}
impl BuildDialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventEl {
    pub fn build(self) -> DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventEl {
        DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventEl {
            event: self.event,
        }
    }
}
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventElRef {
        DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `event` after provisioning.\nName of the event."]
    pub fn event(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextEl {
    text: PrimField<String>,
}
impl DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextEl {}
impl ToListMappable for DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextEl {
    type O =
        BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextEl {
    #[doc = "The natural language text to be processed. Text length must not exceed 256 characters."]
    pub text: PrimField<String>,
}
impl BuildDialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextEl {
    pub fn build(self) -> DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextEl {
        DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextEl { text: self.text }
    }
}
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextElRef {
        DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\nThe natural language text to be processed. Text length must not exceed 256 characters."]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDynamic {
    dtmf: Option<
        DynamicBlock<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfEl>,
    >,
    event: Option<
        DynamicBlock<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventEl>,
    >,
    text: Option<
        DynamicBlock<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextEl>,
    >,
}
#[derive(Serialize)]
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dtmf: Option<Vec<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event: Option<Vec<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Vec<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextEl>>,
    dynamic: DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDynamic,
}
impl DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputEl {
    #[doc = "Set the field `language_code`.\nThe language of the input. See [Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language) for a list of the currently supported language codes.\nNote that queries in the same session do not necessarily need to specify the same language."]
    pub fn set_language_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.language_code = Some(v.into());
        self
    }
    #[doc = "Set the field `dtmf`.\n"]
    pub fn set_dtmf(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dtmf = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dtmf = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `event`.\n"]
    pub fn set_event(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.event = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.event = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.text = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.text = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputEl {
    type O = BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputEl {}
impl BuildDialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputEl {
    pub fn build(self) -> DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputEl {
        DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputEl {
            language_code: core::default::Default::default(),
            dtmf: core::default::Default::default(),
            event: core::default::Default::default(),
            text: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElRef {
        DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\nThe language of the input. See [Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language) for a list of the currently supported language codes.\nNote that queries in the same session do not necessarily need to specify the same language."]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dtmf` after provisioning.\n"]
    pub fn dtmf(
        &self,
    ) -> ListRef<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dtmf", self.base))
    }
    #[doc = "Get a reference to the value of field `event` after provisioning.\n"]
    pub fn event(
        &self,
    ) -> ListRef<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventElRef> {
        ListRef::new(self.shared().clone(), format!("{}.event", self.base))
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(
        &self,
    ) -> ListRef<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextElRef> {
        ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElDynamic {
    input: Option<DynamicBlock<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElUserInputEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_sentiment_analysis: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    injected_parameters: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_webhook_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<Vec<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputEl>>,
    dynamic: DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElDynamic,
}
impl DialogflowCxTestCaseTestCaseConversationTurnsElUserInputEl {
    #[doc = "Set the field `enable_sentiment_analysis`.\nWhether sentiment analysis is enabled."]
    pub fn set_enable_sentiment_analysis(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_sentiment_analysis = Some(v.into());
        self
    }
    #[doc = "Set the field `injected_parameters`.\nParameters that need to be injected into the conversation during intent detection."]
    pub fn set_injected_parameters(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.injected_parameters = Some(v.into());
        self
    }
    #[doc = "Set the field `is_webhook_enabled`.\nIf webhooks should be allowed to trigger in response to the user utterance. Often if parameters are injected, webhooks should not be enabled."]
    pub fn set_is_webhook_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_webhook_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `input`.\n"]
    pub fn set_input(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.input = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.input = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxTestCaseTestCaseConversationTurnsElUserInputEl {
    type O = BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxTestCaseTestCaseConversationTurnsElUserInputEl {}
impl BuildDialogflowCxTestCaseTestCaseConversationTurnsElUserInputEl {
    pub fn build(self) -> DialogflowCxTestCaseTestCaseConversationTurnsElUserInputEl {
        DialogflowCxTestCaseTestCaseConversationTurnsElUserInputEl {
            enable_sentiment_analysis: core::default::Default::default(),
            injected_parameters: core::default::Default::default(),
            is_webhook_enabled: core::default::Default::default(),
            input: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElRef {
        DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_sentiment_analysis` after provisioning.\nWhether sentiment analysis is enabled."]
    pub fn enable_sentiment_analysis(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_sentiment_analysis", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `injected_parameters` after provisioning.\nParameters that need to be injected into the conversation during intent detection."]
    pub fn injected_parameters(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.injected_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `is_webhook_enabled` after provisioning.\nIf webhooks should be allowed to trigger in response to the user utterance. Often if parameters are injected, webhooks should not be enabled."]
    pub fn is_webhook_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_webhook_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(
        &self,
    ) -> ListRef<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.input", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageEl {
    #[doc = "Set the field `name`.\nThe unique identifier of the page.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageEl
{
    type O = BlockAssignable<
        DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageEl {
}
impl BuildDialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageEl {
    pub fn build(
        self,
    ) -> DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageEl {
        DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageEl {
            name: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageElRef {
        DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the page, unique within the flow."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the page.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<ListField<PrimField<String>>>,
}
impl DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesEl {
    #[doc = "Set the field `text`.\nA collection of text responses."]
    pub fn set_text(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.text = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesEl
{
    type O = BlockAssignable<
        DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesEl
{}
impl BuildDialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesEl {
    pub fn build(
        self,
    ) -> DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesEl {
        DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesEl {
            text: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesElRef {
        DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\nA collection of text responses."]
    pub fn text(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentEl {
    #[doc = "Set the field `name`.\nThe unique identifier of the intent.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/intents/<Intent ID>."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentEl
{
    type O = BlockAssignable<
        DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentEl
{}
impl BuildDialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentEl {
    pub fn build(
        self,
    ) -> DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentEl {
        DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentEl {
            name: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentElRef
    {
        DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the intent, unique within the agent."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the intent.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/intents/<Intent ID>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElDynamic {
    current_page: Option<
        DynamicBlock<
            DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageEl,
        >,
    >,
    text_responses: Option<
        DynamicBlock<
            DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesEl,
        >,
    >,
    triggered_intent: Option<
        DynamicBlock<
            DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    session_parameters: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    current_page: Option<
        Vec<DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_responses: Option<
        Vec<DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    triggered_intent: Option<
        Vec<DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentEl>,
    >,
    dynamic: DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElDynamic,
}
impl DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputEl {
    #[doc = "Set the field `session_parameters`.\nThe session parameters available to the bot at this point."]
    pub fn set_session_parameters(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.session_parameters = Some(v.into());
        self
    }
    #[doc = "Set the field `current_page`.\n"]
    pub fn set_current_page(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.current_page = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.current_page = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `text_responses`.\n"]
    pub fn set_text_responses(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.text_responses = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.text_responses = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `triggered_intent`.\n"]
    pub fn set_triggered_intent(
        mut self,
        v : impl Into < BlockAssignable < DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.triggered_intent = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.triggered_intent = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputEl {
    type O = BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputEl {}
impl BuildDialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputEl {
    pub fn build(self) -> DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputEl {
        DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputEl {
            session_parameters: core::default::Default::default(),
            current_page: core::default::Default::default(),
            text_responses: core::default::Default::default(),
            triggered_intent: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElRef {
        DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `session_parameters` after provisioning.\nThe session parameters available to the bot at this point."]
    pub fn session_parameters(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `current_page` after provisioning.\n"]
    pub fn current_page(
        &self,
    ) -> ListRef<DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.current_page", self.base))
    }
    #[doc = "Get a reference to the value of field `text_responses` after provisioning.\n"]
    pub fn text_responses(
        &self,
    ) -> ListRef<
        DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.text_responses", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `triggered_intent` after provisioning.\n"]
    pub fn triggered_intent(
        &self,
    ) -> ListRef<
        DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.triggered_intent", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxTestCaseTestCaseConversationTurnsElDynamic {
    user_input: Option<DynamicBlock<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputEl>>,
    virtual_agent_output:
        Option<DynamicBlock<DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxTestCaseTestCaseConversationTurnsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    user_input: Option<Vec<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_agent_output:
        Option<Vec<DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputEl>>,
    dynamic: DialogflowCxTestCaseTestCaseConversationTurnsElDynamic,
}
impl DialogflowCxTestCaseTestCaseConversationTurnsEl {
    #[doc = "Set the field `user_input`.\n"]
    pub fn set_user_input(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.user_input = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.user_input = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `virtual_agent_output`.\n"]
    pub fn set_virtual_agent_output(
        mut self,
        v: impl Into<
            BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.virtual_agent_output = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.virtual_agent_output = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxTestCaseTestCaseConversationTurnsEl {
    type O = BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxTestCaseTestCaseConversationTurnsEl {}
impl BuildDialogflowCxTestCaseTestCaseConversationTurnsEl {
    pub fn build(self) -> DialogflowCxTestCaseTestCaseConversationTurnsEl {
        DialogflowCxTestCaseTestCaseConversationTurnsEl {
            user_input: core::default::Default::default(),
            virtual_agent_output: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxTestCaseTestCaseConversationTurnsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxTestCaseTestCaseConversationTurnsElRef {
        DialogflowCxTestCaseTestCaseConversationTurnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxTestCaseTestCaseConversationTurnsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `user_input` after provisioning.\n"]
    pub fn user_input(
        &self,
    ) -> ListRef<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.user_input", self.base))
    }
    #[doc = "Get a reference to the value of field `virtual_agent_output` after provisioning.\n"]
    pub fn virtual_agent_output(
        &self,
    ) -> ListRef<DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.virtual_agent_output", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxTestCaseTestConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    flow: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tracking_parameters: Option<ListField<PrimField<String>>>,
}
impl DialogflowCxTestCaseTestConfigEl {
    #[doc = "Set the field `flow`.\nFlow name to start the test case with.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>.\nOnly one of flow and page should be set to indicate the starting point of the test case. If neither is set, the test case will start with start page on the default start flow."]
    pub fn set_flow(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.flow = Some(v.into());
        self
    }
    #[doc = "Set the field `page`.\nThe page to start the test case with.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>.\nOnly one of flow and page should be set to indicate the starting point of the test case. If neither is set, the test case will start with start page on the default start flow."]
    pub fn set_page(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.page = Some(v.into());
        self
    }
    #[doc = "Set the field `tracking_parameters`.\nSession parameters to be compared when calculating differences."]
    pub fn set_tracking_parameters(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tracking_parameters = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxTestCaseTestConfigEl {
    type O = BlockAssignable<DialogflowCxTestCaseTestConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxTestCaseTestConfigEl {}
impl BuildDialogflowCxTestCaseTestConfigEl {
    pub fn build(self) -> DialogflowCxTestCaseTestConfigEl {
        DialogflowCxTestCaseTestConfigEl {
            flow: core::default::Default::default(),
            page: core::default::Default::default(),
            tracking_parameters: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxTestCaseTestConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxTestCaseTestConfigElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxTestCaseTestConfigElRef {
        DialogflowCxTestCaseTestConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxTestCaseTestConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `flow` after provisioning.\nFlow name to start the test case with.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>.\nOnly one of flow and page should be set to indicate the starting point of the test case. If neither is set, the test case will start with start page on the default start flow."]
    pub fn flow(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.flow", self.base))
    }
    #[doc = "Get a reference to the value of field `page` after provisioning.\nThe page to start the test case with.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>.\nOnly one of flow and page should be set to indicate the starting point of the test case. If neither is set, the test case will start with start page on the default start flow."]
    pub fn page(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.page", self.base))
    }
    #[doc = "Get a reference to the value of field `tracking_parameters` after provisioning.\nSession parameters to be compared when calculating differences."]
    pub fn tracking_parameters(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tracking_parameters", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxTestCaseTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DialogflowCxTestCaseTimeoutsEl {
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
impl ToListMappable for DialogflowCxTestCaseTimeoutsEl {
    type O = BlockAssignable<DialogflowCxTestCaseTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxTestCaseTimeoutsEl {}
impl BuildDialogflowCxTestCaseTimeoutsEl {
    pub fn build(self) -> DialogflowCxTestCaseTimeoutsEl {
        DialogflowCxTestCaseTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxTestCaseTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxTestCaseTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxTestCaseTimeoutsElRef {
        DialogflowCxTestCaseTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxTestCaseTimeoutsElRef {
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
struct DialogflowCxTestCaseDynamic {
    test_case_conversation_turns:
        Option<DynamicBlock<DialogflowCxTestCaseTestCaseConversationTurnsEl>>,
    test_config: Option<DynamicBlock<DialogflowCxTestCaseTestConfigEl>>,
}
