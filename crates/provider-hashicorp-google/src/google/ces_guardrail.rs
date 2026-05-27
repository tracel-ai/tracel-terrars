use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CesGuardrailData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    app: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    guardrail_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<CesGuardrailActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_callback: Option<Vec<CesGuardrailCodeCallbackEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_filter: Option<Vec<CesGuardrailContentFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    llm_policy: Option<Vec<CesGuardrailLlmPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    llm_prompt_security: Option<Vec<CesGuardrailLlmPromptSecurityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_safety: Option<Vec<CesGuardrailModelSafetyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CesGuardrailTimeoutsEl>,
    dynamic: CesGuardrailDynamic,
}
struct CesGuardrail_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CesGuardrailData>,
}
#[derive(Clone)]
pub struct CesGuardrail(Rc<CesGuardrail_>);
impl CesGuardrail {
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
    #[doc = "Set the field `description`.\nDescription of the guardrail."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\nWhether the guardrail is enabled."]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(self, v: impl Into<BlockAssignable<CesGuardrailActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `code_callback`.\n"]
    pub fn set_code_callback(
        self,
        v: impl Into<BlockAssignable<CesGuardrailCodeCallbackEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().code_callback = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.code_callback = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `content_filter`.\n"]
    pub fn set_content_filter(
        self,
        v: impl Into<BlockAssignable<CesGuardrailContentFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().content_filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.content_filter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `llm_policy`.\n"]
    pub fn set_llm_policy(self, v: impl Into<BlockAssignable<CesGuardrailLlmPolicyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().llm_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.llm_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `llm_prompt_security`.\n"]
    pub fn set_llm_prompt_security(
        self,
        v: impl Into<BlockAssignable<CesGuardrailLlmPromptSecurityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().llm_prompt_security = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.llm_prompt_security = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `model_safety`.\n"]
    pub fn set_model_safety(
        self,
        v: impl Into<BlockAssignable<CesGuardrailModelSafetyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().model_safety = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.model_safety = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CesGuardrailTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `app` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn app(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the guardrail was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the guardrail."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the guardrail."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether the guardrail is enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nEtag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `guardrail_id` after provisioning.\nThe ID to use for the guardrail, which will become the final component of\nthe guardrail's resource name. If not provided, a unique ID will be\nautomatically assigned for the guardrail."]
    pub fn guardrail_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.guardrail_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the guardrail.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/guardrails/{guardrail}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the guardrail was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<CesGuardrailActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `code_callback` after provisioning.\n"]
    pub fn code_callback(&self) -> ListRef<CesGuardrailCodeCallbackElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.code_callback", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_filter` after provisioning.\n"]
    pub fn content_filter(&self) -> ListRef<CesGuardrailContentFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.content_filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `llm_policy` after provisioning.\n"]
    pub fn llm_policy(&self) -> ListRef<CesGuardrailLlmPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.llm_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `llm_prompt_security` after provisioning.\n"]
    pub fn llm_prompt_security(&self) -> ListRef<CesGuardrailLlmPromptSecurityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.llm_prompt_security", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `model_safety` after provisioning.\n"]
    pub fn model_safety(&self) -> ListRef<CesGuardrailModelSafetyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_safety", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesGuardrailTimeoutsElRef {
        CesGuardrailTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for CesGuardrail {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CesGuardrail {}
impl ToListMappable for CesGuardrail {
    type O = ListRef<CesGuardrailRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CesGuardrail_ {
    fn extract_resource_type(&self) -> String {
        "google_ces_guardrail".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCesGuardrail {
    pub tf_id: String,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub app: PrimField<String>,
    #[doc = "Display name of the guardrail."]
    pub display_name: PrimField<String>,
    #[doc = "The ID to use for the guardrail, which will become the final component of\nthe guardrail's resource name. If not provided, a unique ID will be\nautomatically assigned for the guardrail."]
    pub guardrail_id: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
}
impl BuildCesGuardrail {
    pub fn build(self, stack: &mut Stack) -> CesGuardrail {
        let out = CesGuardrail(Rc::new(CesGuardrail_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CesGuardrailData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                app: self.app,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                enabled: core::default::Default::default(),
                guardrail_id: self.guardrail_id,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                action: core::default::Default::default(),
                code_callback: core::default::Default::default(),
                content_filter: core::default::Default::default(),
                llm_policy: core::default::Default::default(),
                llm_prompt_security: core::default::Default::default(),
                model_safety: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CesGuardrailRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesGuardrailRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CesGuardrailRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `app` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn app(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the guardrail was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the guardrail."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the guardrail."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether the guardrail is enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nEtag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `guardrail_id` after provisioning.\nThe ID to use for the guardrail, which will become the final component of\nthe guardrail's resource name. If not provided, a unique ID will be\nautomatically assigned for the guardrail."]
    pub fn guardrail_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.guardrail_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the guardrail.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/guardrails/{guardrail}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the guardrail was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<CesGuardrailActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `code_callback` after provisioning.\n"]
    pub fn code_callback(&self) -> ListRef<CesGuardrailCodeCallbackElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.code_callback", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_filter` after provisioning.\n"]
    pub fn content_filter(&self) -> ListRef<CesGuardrailContentFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.content_filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `llm_policy` after provisioning.\n"]
    pub fn llm_policy(&self) -> ListRef<CesGuardrailLlmPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.llm_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `llm_prompt_security` after provisioning.\n"]
    pub fn llm_prompt_security(&self) -> ListRef<CesGuardrailLlmPromptSecurityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.llm_prompt_security", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `model_safety` after provisioning.\n"]
    pub fn model_safety(&self) -> ListRef<CesGuardrailModelSafetyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_safety", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesGuardrailTimeoutsElRef {
        CesGuardrailTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CesGuardrailActionElGenerativeAnswerEl {
    prompt: PrimField<String>,
}
impl CesGuardrailActionElGenerativeAnswerEl {}
impl ToListMappable for CesGuardrailActionElGenerativeAnswerEl {
    type O = BlockAssignable<CesGuardrailActionElGenerativeAnswerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesGuardrailActionElGenerativeAnswerEl {
    #[doc = "The prompt to use for the generative answer."]
    pub prompt: PrimField<String>,
}
impl BuildCesGuardrailActionElGenerativeAnswerEl {
    pub fn build(self) -> CesGuardrailActionElGenerativeAnswerEl {
        CesGuardrailActionElGenerativeAnswerEl {
            prompt: self.prompt,
        }
    }
}
pub struct CesGuardrailActionElGenerativeAnswerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesGuardrailActionElGenerativeAnswerElRef {
    fn new(shared: StackShared, base: String) -> CesGuardrailActionElGenerativeAnswerElRef {
        CesGuardrailActionElGenerativeAnswerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesGuardrailActionElGenerativeAnswerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `prompt` after provisioning.\nThe prompt to use for the generative answer."]
    pub fn prompt(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prompt", self.base))
    }
}
#[derive(Serialize)]
pub struct CesGuardrailActionElRespondImmediatelyElResponsesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    text: PrimField<String>,
}
impl CesGuardrailActionElRespondImmediatelyElResponsesEl {
    #[doc = "Set the field `disabled`.\nWhether the response is disabled. Disabled responses are not used by the\nagent."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}
impl ToListMappable for CesGuardrailActionElRespondImmediatelyElResponsesEl {
    type O = BlockAssignable<CesGuardrailActionElRespondImmediatelyElResponsesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesGuardrailActionElRespondImmediatelyElResponsesEl {
    #[doc = "Text for the agent to respond with."]
    pub text: PrimField<String>,
}
impl BuildCesGuardrailActionElRespondImmediatelyElResponsesEl {
    pub fn build(self) -> CesGuardrailActionElRespondImmediatelyElResponsesEl {
        CesGuardrailActionElRespondImmediatelyElResponsesEl {
            disabled: core::default::Default::default(),
            text: self.text,
        }
    }
}
pub struct CesGuardrailActionElRespondImmediatelyElResponsesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesGuardrailActionElRespondImmediatelyElResponsesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesGuardrailActionElRespondImmediatelyElResponsesElRef {
        CesGuardrailActionElRespondImmediatelyElResponsesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesGuardrailActionElRespondImmediatelyElResponsesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the response is disabled. Disabled responses are not used by the\nagent."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\nText for the agent to respond with."]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize, Default)]
struct CesGuardrailActionElRespondImmediatelyElDynamic {
    responses: Option<DynamicBlock<CesGuardrailActionElRespondImmediatelyElResponsesEl>>,
}
#[derive(Serialize)]
pub struct CesGuardrailActionElRespondImmediatelyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    responses: Option<Vec<CesGuardrailActionElRespondImmediatelyElResponsesEl>>,
    dynamic: CesGuardrailActionElRespondImmediatelyElDynamic,
}
impl CesGuardrailActionElRespondImmediatelyEl {
    #[doc = "Set the field `responses`.\n"]
    pub fn set_responses(
        mut self,
        v: impl Into<BlockAssignable<CesGuardrailActionElRespondImmediatelyElResponsesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.responses = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.responses = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesGuardrailActionElRespondImmediatelyEl {
    type O = BlockAssignable<CesGuardrailActionElRespondImmediatelyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesGuardrailActionElRespondImmediatelyEl {}
impl BuildCesGuardrailActionElRespondImmediatelyEl {
    pub fn build(self) -> CesGuardrailActionElRespondImmediatelyEl {
        CesGuardrailActionElRespondImmediatelyEl {
            responses: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesGuardrailActionElRespondImmediatelyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesGuardrailActionElRespondImmediatelyElRef {
    fn new(shared: StackShared, base: String) -> CesGuardrailActionElRespondImmediatelyElRef {
        CesGuardrailActionElRespondImmediatelyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesGuardrailActionElRespondImmediatelyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `responses` after provisioning.\n"]
    pub fn responses(&self) -> ListRef<CesGuardrailActionElRespondImmediatelyElResponsesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.responses", self.base))
    }
}
#[derive(Serialize)]
pub struct CesGuardrailActionElTransferAgentEl {
    agent: PrimField<String>,
}
impl CesGuardrailActionElTransferAgentEl {}
impl ToListMappable for CesGuardrailActionElTransferAgentEl {
    type O = BlockAssignable<CesGuardrailActionElTransferAgentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesGuardrailActionElTransferAgentEl {
    #[doc = "The name of the agent to transfer the conversation to. The agent must be\nin the same app as the current agent.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub agent: PrimField<String>,
}
impl BuildCesGuardrailActionElTransferAgentEl {
    pub fn build(self) -> CesGuardrailActionElTransferAgentEl {
        CesGuardrailActionElTransferAgentEl { agent: self.agent }
    }
}
pub struct CesGuardrailActionElTransferAgentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesGuardrailActionElTransferAgentElRef {
    fn new(shared: StackShared, base: String) -> CesGuardrailActionElTransferAgentElRef {
        CesGuardrailActionElTransferAgentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesGuardrailActionElTransferAgentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `agent` after provisioning.\nThe name of the agent to transfer the conversation to. The agent must be\nin the same app as the current agent.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub fn agent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent", self.base))
    }
}
#[derive(Serialize, Default)]
struct CesGuardrailActionElDynamic {
    generative_answer: Option<DynamicBlock<CesGuardrailActionElGenerativeAnswerEl>>,
    respond_immediately: Option<DynamicBlock<CesGuardrailActionElRespondImmediatelyEl>>,
    transfer_agent: Option<DynamicBlock<CesGuardrailActionElTransferAgentEl>>,
}
#[derive(Serialize)]
pub struct CesGuardrailActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    generative_answer: Option<Vec<CesGuardrailActionElGenerativeAnswerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    respond_immediately: Option<Vec<CesGuardrailActionElRespondImmediatelyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_agent: Option<Vec<CesGuardrailActionElTransferAgentEl>>,
    dynamic: CesGuardrailActionElDynamic,
}
impl CesGuardrailActionEl {
    #[doc = "Set the field `generative_answer`.\n"]
    pub fn set_generative_answer(
        mut self,
        v: impl Into<BlockAssignable<CesGuardrailActionElGenerativeAnswerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.generative_answer = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.generative_answer = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `respond_immediately`.\n"]
    pub fn set_respond_immediately(
        mut self,
        v: impl Into<BlockAssignable<CesGuardrailActionElRespondImmediatelyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.respond_immediately = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.respond_immediately = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `transfer_agent`.\n"]
    pub fn set_transfer_agent(
        mut self,
        v: impl Into<BlockAssignable<CesGuardrailActionElTransferAgentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.transfer_agent = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.transfer_agent = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesGuardrailActionEl {
    type O = BlockAssignable<CesGuardrailActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesGuardrailActionEl {}
impl BuildCesGuardrailActionEl {
    pub fn build(self) -> CesGuardrailActionEl {
        CesGuardrailActionEl {
            generative_answer: core::default::Default::default(),
            respond_immediately: core::default::Default::default(),
            transfer_agent: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesGuardrailActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesGuardrailActionElRef {
    fn new(shared: StackShared, base: String) -> CesGuardrailActionElRef {
        CesGuardrailActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesGuardrailActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `generative_answer` after provisioning.\n"]
    pub fn generative_answer(&self) -> ListRef<CesGuardrailActionElGenerativeAnswerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.generative_answer", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `respond_immediately` after provisioning.\n"]
    pub fn respond_immediately(&self) -> ListRef<CesGuardrailActionElRespondImmediatelyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.respond_immediately", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `transfer_agent` after provisioning.\n"]
    pub fn transfer_agent(&self) -> ListRef<CesGuardrailActionElTransferAgentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.transfer_agent", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesGuardrailCodeCallbackElAfterAgentCallbackEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    python_code: PrimField<String>,
}
impl CesGuardrailCodeCallbackElAfterAgentCallbackEl {
    #[doc = "Set the field `description`.\nHuman-readable description of the callback."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}
impl ToListMappable for CesGuardrailCodeCallbackElAfterAgentCallbackEl {
    type O = BlockAssignable<CesGuardrailCodeCallbackElAfterAgentCallbackEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesGuardrailCodeCallbackElAfterAgentCallbackEl {
    #[doc = "The python code to execute for the callback."]
    pub python_code: PrimField<String>,
}
impl BuildCesGuardrailCodeCallbackElAfterAgentCallbackEl {
    pub fn build(self) -> CesGuardrailCodeCallbackElAfterAgentCallbackEl {
        CesGuardrailCodeCallbackElAfterAgentCallbackEl {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            python_code: self.python_code,
        }
    }
}
pub struct CesGuardrailCodeCallbackElAfterAgentCallbackElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesGuardrailCodeCallbackElAfterAgentCallbackElRef {
    fn new(shared: StackShared, base: String) -> CesGuardrailCodeCallbackElAfterAgentCallbackElRef {
        CesGuardrailCodeCallbackElAfterAgentCallbackElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesGuardrailCodeCallbackElAfterAgentCallbackElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the callback."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `python_code` after provisioning.\nThe python code to execute for the callback."]
    pub fn python_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}
#[derive(Serialize)]
pub struct CesGuardrailCodeCallbackElAfterModelCallbackEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    python_code: PrimField<String>,
}
impl CesGuardrailCodeCallbackElAfterModelCallbackEl {
    #[doc = "Set the field `description`.\nHuman-readable description of the callback."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}
impl ToListMappable for CesGuardrailCodeCallbackElAfterModelCallbackEl {
    type O = BlockAssignable<CesGuardrailCodeCallbackElAfterModelCallbackEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesGuardrailCodeCallbackElAfterModelCallbackEl {
    #[doc = "The python code to execute for the callback."]
    pub python_code: PrimField<String>,
}
impl BuildCesGuardrailCodeCallbackElAfterModelCallbackEl {
    pub fn build(self) -> CesGuardrailCodeCallbackElAfterModelCallbackEl {
        CesGuardrailCodeCallbackElAfterModelCallbackEl {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            python_code: self.python_code,
        }
    }
}
pub struct CesGuardrailCodeCallbackElAfterModelCallbackElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesGuardrailCodeCallbackElAfterModelCallbackElRef {
    fn new(shared: StackShared, base: String) -> CesGuardrailCodeCallbackElAfterModelCallbackElRef {
        CesGuardrailCodeCallbackElAfterModelCallbackElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesGuardrailCodeCallbackElAfterModelCallbackElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the callback."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `python_code` after provisioning.\nThe python code to execute for the callback."]
    pub fn python_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}
#[derive(Serialize)]
pub struct CesGuardrailCodeCallbackElBeforeAgentCallbackEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    python_code: PrimField<String>,
}
impl CesGuardrailCodeCallbackElBeforeAgentCallbackEl {
    #[doc = "Set the field `description`.\nHuman-readable description of the callback."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}
impl ToListMappable for CesGuardrailCodeCallbackElBeforeAgentCallbackEl {
    type O = BlockAssignable<CesGuardrailCodeCallbackElBeforeAgentCallbackEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesGuardrailCodeCallbackElBeforeAgentCallbackEl {
    #[doc = "The python code to execute for the callback."]
    pub python_code: PrimField<String>,
}
impl BuildCesGuardrailCodeCallbackElBeforeAgentCallbackEl {
    pub fn build(self) -> CesGuardrailCodeCallbackElBeforeAgentCallbackEl {
        CesGuardrailCodeCallbackElBeforeAgentCallbackEl {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            python_code: self.python_code,
        }
    }
}
pub struct CesGuardrailCodeCallbackElBeforeAgentCallbackElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesGuardrailCodeCallbackElBeforeAgentCallbackElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesGuardrailCodeCallbackElBeforeAgentCallbackElRef {
        CesGuardrailCodeCallbackElBeforeAgentCallbackElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesGuardrailCodeCallbackElBeforeAgentCallbackElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the callback."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `python_code` after provisioning.\nThe python code to execute for the callback."]
    pub fn python_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}
#[derive(Serialize)]
pub struct CesGuardrailCodeCallbackElBeforeModelCallbackEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    python_code: PrimField<String>,
}
impl CesGuardrailCodeCallbackElBeforeModelCallbackEl {
    #[doc = "Set the field `description`.\nHuman-readable description of the callback."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}
impl ToListMappable for CesGuardrailCodeCallbackElBeforeModelCallbackEl {
    type O = BlockAssignable<CesGuardrailCodeCallbackElBeforeModelCallbackEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesGuardrailCodeCallbackElBeforeModelCallbackEl {
    #[doc = "The python code to execute for the callback."]
    pub python_code: PrimField<String>,
}
impl BuildCesGuardrailCodeCallbackElBeforeModelCallbackEl {
    pub fn build(self) -> CesGuardrailCodeCallbackElBeforeModelCallbackEl {
        CesGuardrailCodeCallbackElBeforeModelCallbackEl {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            python_code: self.python_code,
        }
    }
}
pub struct CesGuardrailCodeCallbackElBeforeModelCallbackElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesGuardrailCodeCallbackElBeforeModelCallbackElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesGuardrailCodeCallbackElBeforeModelCallbackElRef {
        CesGuardrailCodeCallbackElBeforeModelCallbackElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesGuardrailCodeCallbackElBeforeModelCallbackElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the callback."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `python_code` after provisioning.\nThe python code to execute for the callback."]
    pub fn python_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}
#[derive(Serialize, Default)]
struct CesGuardrailCodeCallbackElDynamic {
    after_agent_callback: Option<DynamicBlock<CesGuardrailCodeCallbackElAfterAgentCallbackEl>>,
    after_model_callback: Option<DynamicBlock<CesGuardrailCodeCallbackElAfterModelCallbackEl>>,
    before_agent_callback: Option<DynamicBlock<CesGuardrailCodeCallbackElBeforeAgentCallbackEl>>,
    before_model_callback: Option<DynamicBlock<CesGuardrailCodeCallbackElBeforeModelCallbackEl>>,
}
#[derive(Serialize)]
pub struct CesGuardrailCodeCallbackEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    after_agent_callback: Option<Vec<CesGuardrailCodeCallbackElAfterAgentCallbackEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after_model_callback: Option<Vec<CesGuardrailCodeCallbackElAfterModelCallbackEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before_agent_callback: Option<Vec<CesGuardrailCodeCallbackElBeforeAgentCallbackEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before_model_callback: Option<Vec<CesGuardrailCodeCallbackElBeforeModelCallbackEl>>,
    dynamic: CesGuardrailCodeCallbackElDynamic,
}
impl CesGuardrailCodeCallbackEl {
    #[doc = "Set the field `after_agent_callback`.\n"]
    pub fn set_after_agent_callback(
        mut self,
        v: impl Into<BlockAssignable<CesGuardrailCodeCallbackElAfterAgentCallbackEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.after_agent_callback = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.after_agent_callback = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `after_model_callback`.\n"]
    pub fn set_after_model_callback(
        mut self,
        v: impl Into<BlockAssignable<CesGuardrailCodeCallbackElAfterModelCallbackEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.after_model_callback = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.after_model_callback = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `before_agent_callback`.\n"]
    pub fn set_before_agent_callback(
        mut self,
        v: impl Into<BlockAssignable<CesGuardrailCodeCallbackElBeforeAgentCallbackEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.before_agent_callback = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.before_agent_callback = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `before_model_callback`.\n"]
    pub fn set_before_model_callback(
        mut self,
        v: impl Into<BlockAssignable<CesGuardrailCodeCallbackElBeforeModelCallbackEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.before_model_callback = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.before_model_callback = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesGuardrailCodeCallbackEl {
    type O = BlockAssignable<CesGuardrailCodeCallbackEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesGuardrailCodeCallbackEl {}
impl BuildCesGuardrailCodeCallbackEl {
    pub fn build(self) -> CesGuardrailCodeCallbackEl {
        CesGuardrailCodeCallbackEl {
            after_agent_callback: core::default::Default::default(),
            after_model_callback: core::default::Default::default(),
            before_agent_callback: core::default::Default::default(),
            before_model_callback: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesGuardrailCodeCallbackElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesGuardrailCodeCallbackElRef {
    fn new(shared: StackShared, base: String) -> CesGuardrailCodeCallbackElRef {
        CesGuardrailCodeCallbackElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesGuardrailCodeCallbackElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `after_agent_callback` after provisioning.\n"]
    pub fn after_agent_callback(
        &self,
    ) -> ListRef<CesGuardrailCodeCallbackElAfterAgentCallbackElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.after_agent_callback", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `after_model_callback` after provisioning.\n"]
    pub fn after_model_callback(
        &self,
    ) -> ListRef<CesGuardrailCodeCallbackElAfterModelCallbackElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.after_model_callback", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `before_agent_callback` after provisioning.\n"]
    pub fn before_agent_callback(
        &self,
    ) -> ListRef<CesGuardrailCodeCallbackElBeforeAgentCallbackElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.before_agent_callback", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `before_model_callback` after provisioning.\n"]
    pub fn before_model_callback(
        &self,
    ) -> ListRef<CesGuardrailCodeCallbackElBeforeModelCallbackElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.before_model_callback", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesGuardrailContentFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    banned_contents: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    banned_contents_in_agent_response: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    banned_contents_in_user_input: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disregard_diacritics: Option<PrimField<bool>>,
    match_type: PrimField<String>,
}
impl CesGuardrailContentFilterEl {
    #[doc = "Set the field `banned_contents`.\nList of banned phrases. Applies to both user inputs and agent responses."]
    pub fn set_banned_contents(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.banned_contents = Some(v.into());
        self
    }
    #[doc = "Set the field `banned_contents_in_agent_response`.\nList of banned phrases. Applies only to agent responses."]
    pub fn set_banned_contents_in_agent_response(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.banned_contents_in_agent_response = Some(v.into());
        self
    }
    #[doc = "Set the field `banned_contents_in_user_input`.\nList of banned phrases. Applies only to user inputs."]
    pub fn set_banned_contents_in_user_input(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.banned_contents_in_user_input = Some(v.into());
        self
    }
    #[doc = "Set the field `disregard_diacritics`.\nIf true, diacritics are ignored during matching."]
    pub fn set_disregard_diacritics(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disregard_diacritics = Some(v.into());
        self
    }
}
impl ToListMappable for CesGuardrailContentFilterEl {
    type O = BlockAssignable<CesGuardrailContentFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesGuardrailContentFilterEl {
    #[doc = "Match type for the content filter.\nPossible values:\nSIMPLE_STRING_MATCH\nWORD_BOUNDARY_STRING_MATCH\nREGEXP_MATCH"]
    pub match_type: PrimField<String>,
}
impl BuildCesGuardrailContentFilterEl {
    pub fn build(self) -> CesGuardrailContentFilterEl {
        CesGuardrailContentFilterEl {
            banned_contents: core::default::Default::default(),
            banned_contents_in_agent_response: core::default::Default::default(),
            banned_contents_in_user_input: core::default::Default::default(),
            disregard_diacritics: core::default::Default::default(),
            match_type: self.match_type,
        }
    }
}
pub struct CesGuardrailContentFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesGuardrailContentFilterElRef {
    fn new(shared: StackShared, base: String) -> CesGuardrailContentFilterElRef {
        CesGuardrailContentFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesGuardrailContentFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `banned_contents` after provisioning.\nList of banned phrases. Applies to both user inputs and agent responses."]
    pub fn banned_contents(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.banned_contents", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `banned_contents_in_agent_response` after provisioning.\nList of banned phrases. Applies only to agent responses."]
    pub fn banned_contents_in_agent_response(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.banned_contents_in_agent_response", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `banned_contents_in_user_input` after provisioning.\nList of banned phrases. Applies only to user inputs."]
    pub fn banned_contents_in_user_input(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.banned_contents_in_user_input", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disregard_diacritics` after provisioning.\nIf true, diacritics are ignored during matching."]
    pub fn disregard_diacritics(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disregard_diacritics", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `match_type` after provisioning.\nMatch type for the content filter.\nPossible values:\nSIMPLE_STRING_MATCH\nWORD_BOUNDARY_STRING_MATCH\nREGEXP_MATCH"]
    pub fn match_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.match_type", self.base))
    }
}
#[derive(Serialize)]
pub struct CesGuardrailLlmPolicyElModelSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<PrimField<f64>>,
}
impl CesGuardrailLlmPolicyElModelSettingsEl {
    #[doc = "Set the field `model`.\nThe LLM model that the agent should use.\nIf not set, the agent will inherit the model from its parent agent."]
    pub fn set_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model = Some(v.into());
        self
    }
    #[doc = "Set the field `temperature`.\nIf set, this temperature will be used for the LLM model. Temperature\ncontrols the randomness of the model's responses. Lower temperatures\nproduce responses that are more predictable. Higher temperatures produce\nresponses that are more creative."]
    pub fn set_temperature(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.temperature = Some(v.into());
        self
    }
}
impl ToListMappable for CesGuardrailLlmPolicyElModelSettingsEl {
    type O = BlockAssignable<CesGuardrailLlmPolicyElModelSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesGuardrailLlmPolicyElModelSettingsEl {}
impl BuildCesGuardrailLlmPolicyElModelSettingsEl {
    pub fn build(self) -> CesGuardrailLlmPolicyElModelSettingsEl {
        CesGuardrailLlmPolicyElModelSettingsEl {
            model: core::default::Default::default(),
            temperature: core::default::Default::default(),
        }
    }
}
pub struct CesGuardrailLlmPolicyElModelSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesGuardrailLlmPolicyElModelSettingsElRef {
    fn new(shared: StackShared, base: String) -> CesGuardrailLlmPolicyElModelSettingsElRef {
        CesGuardrailLlmPolicyElModelSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesGuardrailLlmPolicyElModelSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `model` after provisioning.\nThe LLM model that the agent should use.\nIf not set, the agent will inherit the model from its parent agent."]
    pub fn model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }
    #[doc = "Get a reference to the value of field `temperature` after provisioning.\nIf set, this temperature will be used for the LLM model. Temperature\ncontrols the randomness of the model's responses. Lower temperatures\nproduce responses that are more predictable. Higher temperatures produce\nresponses that are more creative."]
    pub fn temperature(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }
}
#[derive(Serialize, Default)]
struct CesGuardrailLlmPolicyElDynamic {
    model_settings: Option<DynamicBlock<CesGuardrailLlmPolicyElModelSettingsEl>>,
}
#[derive(Serialize)]
pub struct CesGuardrailLlmPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_short_utterance: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fail_open: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_conversation_messages: Option<PrimField<f64>>,
    policy_scope: PrimField<String>,
    prompt: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_settings: Option<Vec<CesGuardrailLlmPolicyElModelSettingsEl>>,
    dynamic: CesGuardrailLlmPolicyElDynamic,
}
impl CesGuardrailLlmPolicyEl {
    #[doc = "Set the field `allow_short_utterance`.\nBy default, the LLM policy check is bypassed for short utterances.\nEnabling this setting applies the policy check to all utterances,\nincluding those that would normally be skipped."]
    pub fn set_allow_short_utterance(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_short_utterance = Some(v.into());
        self
    }
    #[doc = "Set the field `fail_open`.\nIf an error occurs during the policy check, fail open and do not trigger\nthe guardrail."]
    pub fn set_fail_open(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.fail_open = Some(v.into());
        self
    }
    #[doc = "Set the field `max_conversation_messages`.\nWhen checking this policy, consider the last 'n' messages in the\nconversation.\nWhen not set a default value of 10 will be used."]
    pub fn set_max_conversation_messages(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_conversation_messages = Some(v.into());
        self
    }
    #[doc = "Set the field `model_settings`.\n"]
    pub fn set_model_settings(
        mut self,
        v: impl Into<BlockAssignable<CesGuardrailLlmPolicyElModelSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.model_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.model_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesGuardrailLlmPolicyEl {
    type O = BlockAssignable<CesGuardrailLlmPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesGuardrailLlmPolicyEl {
    #[doc = "Defines when to apply the policy check during the conversation. If set to\n'POLICY_SCOPE_UNSPECIFIED', the policy will be applied to the user input.\nWhen applying the policy to the agent response, additional latency will\nbe introduced before the agent can respond.\nPossible values:\nUSER_QUERY\nAGENT_RESPONSE\nUSER_QUERY_AND_AGENT_RESPONSE Possible values: [\"USER_QUERY\", \"AGENT_RESPONSE\", \"USER_QUERY_AND_AGENT_RESPONSE\"]"]
    pub policy_scope: PrimField<String>,
    #[doc = "Policy prompt."]
    pub prompt: PrimField<String>,
}
impl BuildCesGuardrailLlmPolicyEl {
    pub fn build(self) -> CesGuardrailLlmPolicyEl {
        CesGuardrailLlmPolicyEl {
            allow_short_utterance: core::default::Default::default(),
            fail_open: core::default::Default::default(),
            max_conversation_messages: core::default::Default::default(),
            policy_scope: self.policy_scope,
            prompt: self.prompt,
            model_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesGuardrailLlmPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesGuardrailLlmPolicyElRef {
    fn new(shared: StackShared, base: String) -> CesGuardrailLlmPolicyElRef {
        CesGuardrailLlmPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesGuardrailLlmPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_short_utterance` after provisioning.\nBy default, the LLM policy check is bypassed for short utterances.\nEnabling this setting applies the policy check to all utterances,\nincluding those that would normally be skipped."]
    pub fn allow_short_utterance(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_short_utterance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fail_open` after provisioning.\nIf an error occurs during the policy check, fail open and do not trigger\nthe guardrail."]
    pub fn fail_open(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fail_open", self.base))
    }
    #[doc = "Get a reference to the value of field `max_conversation_messages` after provisioning.\nWhen checking this policy, consider the last 'n' messages in the\nconversation.\nWhen not set a default value of 10 will be used."]
    pub fn max_conversation_messages(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_conversation_messages", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `policy_scope` after provisioning.\nDefines when to apply the policy check during the conversation. If set to\n'POLICY_SCOPE_UNSPECIFIED', the policy will be applied to the user input.\nWhen applying the policy to the agent response, additional latency will\nbe introduced before the agent can respond.\nPossible values:\nUSER_QUERY\nAGENT_RESPONSE\nUSER_QUERY_AND_AGENT_RESPONSE Possible values: [\"USER_QUERY\", \"AGENT_RESPONSE\", \"USER_QUERY_AND_AGENT_RESPONSE\"]"]
    pub fn policy_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_scope", self.base))
    }
    #[doc = "Get a reference to the value of field `prompt` after provisioning.\nPolicy prompt."]
    pub fn prompt(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prompt", self.base))
    }
    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]
    pub fn model_settings(&self) -> ListRef<CesGuardrailLlmPolicyElModelSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesGuardrailLlmPromptSecurityElCustomPolicyElModelSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<PrimField<f64>>,
}
impl CesGuardrailLlmPromptSecurityElCustomPolicyElModelSettingsEl {
    #[doc = "Set the field `model`.\nThe LLM model that the agent should use.\nIf not set, the agent will inherit the model from its parent agent."]
    pub fn set_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model = Some(v.into());
        self
    }
    #[doc = "Set the field `temperature`.\nIf set, this temperature will be used for the LLM model. Temperature\ncontrols the randomness of the model's responses. Lower temperatures\nproduce responses that are more predictable. Higher temperatures produce\nresponses that are more creative."]
    pub fn set_temperature(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.temperature = Some(v.into());
        self
    }
}
impl ToListMappable for CesGuardrailLlmPromptSecurityElCustomPolicyElModelSettingsEl {
    type O = BlockAssignable<CesGuardrailLlmPromptSecurityElCustomPolicyElModelSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesGuardrailLlmPromptSecurityElCustomPolicyElModelSettingsEl {}
impl BuildCesGuardrailLlmPromptSecurityElCustomPolicyElModelSettingsEl {
    pub fn build(self) -> CesGuardrailLlmPromptSecurityElCustomPolicyElModelSettingsEl {
        CesGuardrailLlmPromptSecurityElCustomPolicyElModelSettingsEl {
            model: core::default::Default::default(),
            temperature: core::default::Default::default(),
        }
    }
}
pub struct CesGuardrailLlmPromptSecurityElCustomPolicyElModelSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesGuardrailLlmPromptSecurityElCustomPolicyElModelSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesGuardrailLlmPromptSecurityElCustomPolicyElModelSettingsElRef {
        CesGuardrailLlmPromptSecurityElCustomPolicyElModelSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesGuardrailLlmPromptSecurityElCustomPolicyElModelSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `model` after provisioning.\nThe LLM model that the agent should use.\nIf not set, the agent will inherit the model from its parent agent."]
    pub fn model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }
    #[doc = "Get a reference to the value of field `temperature` after provisioning.\nIf set, this temperature will be used for the LLM model. Temperature\ncontrols the randomness of the model's responses. Lower temperatures\nproduce responses that are more predictable. Higher temperatures produce\nresponses that are more creative."]
    pub fn temperature(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }
}
#[derive(Serialize, Default)]
struct CesGuardrailLlmPromptSecurityElCustomPolicyElDynamic {
    model_settings:
        Option<DynamicBlock<CesGuardrailLlmPromptSecurityElCustomPolicyElModelSettingsEl>>,
}
#[derive(Serialize)]
pub struct CesGuardrailLlmPromptSecurityElCustomPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_short_utterance: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fail_open: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_conversation_messages: Option<PrimField<f64>>,
    policy_scope: PrimField<String>,
    prompt: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_settings: Option<Vec<CesGuardrailLlmPromptSecurityElCustomPolicyElModelSettingsEl>>,
    dynamic: CesGuardrailLlmPromptSecurityElCustomPolicyElDynamic,
}
impl CesGuardrailLlmPromptSecurityElCustomPolicyEl {
    #[doc = "Set the field `allow_short_utterance`.\nBy default, the LLM policy check is bypassed for short utterances.\nEnabling this setting applies the policy check to all utterances,\nincluding those that would normally be skipped."]
    pub fn set_allow_short_utterance(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_short_utterance = Some(v.into());
        self
    }
    #[doc = "Set the field `fail_open`.\nIf an error occurs during the policy check, fail open and do not trigger\nthe guardrail."]
    pub fn set_fail_open(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.fail_open = Some(v.into());
        self
    }
    #[doc = "Set the field `max_conversation_messages`.\nWhen checking this policy, consider the last 'n' messages in the\nconversation.\nWhen not set a default value of 10 will be used."]
    pub fn set_max_conversation_messages(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_conversation_messages = Some(v.into());
        self
    }
    #[doc = "Set the field `model_settings`.\n"]
    pub fn set_model_settings(
        mut self,
        v: impl Into<BlockAssignable<CesGuardrailLlmPromptSecurityElCustomPolicyElModelSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.model_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.model_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesGuardrailLlmPromptSecurityElCustomPolicyEl {
    type O = BlockAssignable<CesGuardrailLlmPromptSecurityElCustomPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesGuardrailLlmPromptSecurityElCustomPolicyEl {
    #[doc = "Defines when to apply the policy check during the conversation. If set to\n'POLICY_SCOPE_UNSPECIFIED', the policy will be applied to the user input.\nWhen applying the policy to the agent response, additional latency will\nbe introduced before the agent can respond.\nPossible values:\nUSER_QUERY\nAGENT_RESPONSE\nUSER_QUERY_AND_AGENT_RESPONSE"]
    pub policy_scope: PrimField<String>,
    #[doc = "Policy prompt."]
    pub prompt: PrimField<String>,
}
impl BuildCesGuardrailLlmPromptSecurityElCustomPolicyEl {
    pub fn build(self) -> CesGuardrailLlmPromptSecurityElCustomPolicyEl {
        CesGuardrailLlmPromptSecurityElCustomPolicyEl {
            allow_short_utterance: core::default::Default::default(),
            fail_open: core::default::Default::default(),
            max_conversation_messages: core::default::Default::default(),
            policy_scope: self.policy_scope,
            prompt: self.prompt,
            model_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesGuardrailLlmPromptSecurityElCustomPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesGuardrailLlmPromptSecurityElCustomPolicyElRef {
    fn new(shared: StackShared, base: String) -> CesGuardrailLlmPromptSecurityElCustomPolicyElRef {
        CesGuardrailLlmPromptSecurityElCustomPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesGuardrailLlmPromptSecurityElCustomPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_short_utterance` after provisioning.\nBy default, the LLM policy check is bypassed for short utterances.\nEnabling this setting applies the policy check to all utterances,\nincluding those that would normally be skipped."]
    pub fn allow_short_utterance(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_short_utterance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fail_open` after provisioning.\nIf an error occurs during the policy check, fail open and do not trigger\nthe guardrail."]
    pub fn fail_open(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fail_open", self.base))
    }
    #[doc = "Get a reference to the value of field `max_conversation_messages` after provisioning.\nWhen checking this policy, consider the last 'n' messages in the\nconversation.\nWhen not set a default value of 10 will be used."]
    pub fn max_conversation_messages(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_conversation_messages", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `policy_scope` after provisioning.\nDefines when to apply the policy check during the conversation. If set to\n'POLICY_SCOPE_UNSPECIFIED', the policy will be applied to the user input.\nWhen applying the policy to the agent response, additional latency will\nbe introduced before the agent can respond.\nPossible values:\nUSER_QUERY\nAGENT_RESPONSE\nUSER_QUERY_AND_AGENT_RESPONSE"]
    pub fn policy_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_scope", self.base))
    }
    #[doc = "Get a reference to the value of field `prompt` after provisioning.\nPolicy prompt."]
    pub fn prompt(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prompt", self.base))
    }
    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]
    pub fn model_settings(
        &self,
    ) -> ListRef<CesGuardrailLlmPromptSecurityElCustomPolicyElModelSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesGuardrailLlmPromptSecurityElDefaultSettingsEl {}
impl CesGuardrailLlmPromptSecurityElDefaultSettingsEl {}
impl ToListMappable for CesGuardrailLlmPromptSecurityElDefaultSettingsEl {
    type O = BlockAssignable<CesGuardrailLlmPromptSecurityElDefaultSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesGuardrailLlmPromptSecurityElDefaultSettingsEl {}
impl BuildCesGuardrailLlmPromptSecurityElDefaultSettingsEl {
    pub fn build(self) -> CesGuardrailLlmPromptSecurityElDefaultSettingsEl {
        CesGuardrailLlmPromptSecurityElDefaultSettingsEl {}
    }
}
pub struct CesGuardrailLlmPromptSecurityElDefaultSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesGuardrailLlmPromptSecurityElDefaultSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesGuardrailLlmPromptSecurityElDefaultSettingsElRef {
        CesGuardrailLlmPromptSecurityElDefaultSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesGuardrailLlmPromptSecurityElDefaultSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_prompt_template` after provisioning.\nThe default prompt template used by the system.\nThis field is for display purposes to show the user what prompt\nthe system uses by default. It is OUTPUT_ONLY."]
    pub fn default_prompt_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_prompt_template", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CesGuardrailLlmPromptSecurityElDynamic {
    custom_policy: Option<DynamicBlock<CesGuardrailLlmPromptSecurityElCustomPolicyEl>>,
    default_settings: Option<DynamicBlock<CesGuardrailLlmPromptSecurityElDefaultSettingsEl>>,
}
#[derive(Serialize)]
pub struct CesGuardrailLlmPromptSecurityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_policy: Option<Vec<CesGuardrailLlmPromptSecurityElCustomPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_settings: Option<Vec<CesGuardrailLlmPromptSecurityElDefaultSettingsEl>>,
    dynamic: CesGuardrailLlmPromptSecurityElDynamic,
}
impl CesGuardrailLlmPromptSecurityEl {
    #[doc = "Set the field `custom_policy`.\n"]
    pub fn set_custom_policy(
        mut self,
        v: impl Into<BlockAssignable<CesGuardrailLlmPromptSecurityElCustomPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `default_settings`.\n"]
    pub fn set_default_settings(
        mut self,
        v: impl Into<BlockAssignable<CesGuardrailLlmPromptSecurityElDefaultSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesGuardrailLlmPromptSecurityEl {
    type O = BlockAssignable<CesGuardrailLlmPromptSecurityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesGuardrailLlmPromptSecurityEl {}
impl BuildCesGuardrailLlmPromptSecurityEl {
    pub fn build(self) -> CesGuardrailLlmPromptSecurityEl {
        CesGuardrailLlmPromptSecurityEl {
            custom_policy: core::default::Default::default(),
            default_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesGuardrailLlmPromptSecurityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesGuardrailLlmPromptSecurityElRef {
    fn new(shared: StackShared, base: String) -> CesGuardrailLlmPromptSecurityElRef {
        CesGuardrailLlmPromptSecurityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesGuardrailLlmPromptSecurityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_policy` after provisioning.\n"]
    pub fn custom_policy(&self) -> ListRef<CesGuardrailLlmPromptSecurityElCustomPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_settings` after provisioning.\n"]
    pub fn default_settings(&self) -> ListRef<CesGuardrailLlmPromptSecurityElDefaultSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesGuardrailModelSafetyElSafetySettingsEl {
    category: PrimField<String>,
    threshold: PrimField<String>,
}
impl CesGuardrailModelSafetyElSafetySettingsEl {}
impl ToListMappable for CesGuardrailModelSafetyElSafetySettingsEl {
    type O = BlockAssignable<CesGuardrailModelSafetyElSafetySettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesGuardrailModelSafetyElSafetySettingsEl {
    #[doc = "The harm category.\nPossible values:\nHARM_CATEGORY_HATE_SPEECH\nHARM_CATEGORY_DANGEROUS_CONTENT\nHARM_CATEGORY_HARASSMENT\nHARM_CATEGORY_SEXUALLY_EXPLICIT Possible values: [\"HARM_CATEGORY_HATE_SPEECH\", \"HARM_CATEGORY_DANGEROUS_CONTENT\", \"HARM_CATEGORY_HARASSMENT\", \"HARM_CATEGORY_SEXUALLY_EXPLICIT\"]"]
    pub category: PrimField<String>,
    #[doc = "The harm block threshold.\nPossible values:\nBLOCK_LOW_AND_ABOVE\nBLOCK_MEDIUM_AND_ABOVE\nBLOCK_ONLY_HIGH\nBLOCK_NONE\nOFF Possible values: [\"BLOCK_LOW_AND_ABOVE\", \"BLOCK_MEDIUM_AND_ABOVE\", \"BLOCK_ONLY_HIGH\", \"BLOCK_NONE\", \"OFF\"]"]
    pub threshold: PrimField<String>,
}
impl BuildCesGuardrailModelSafetyElSafetySettingsEl {
    pub fn build(self) -> CesGuardrailModelSafetyElSafetySettingsEl {
        CesGuardrailModelSafetyElSafetySettingsEl {
            category: self.category,
            threshold: self.threshold,
        }
    }
}
pub struct CesGuardrailModelSafetyElSafetySettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesGuardrailModelSafetyElSafetySettingsElRef {
    fn new(shared: StackShared, base: String) -> CesGuardrailModelSafetyElSafetySettingsElRef {
        CesGuardrailModelSafetyElSafetySettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesGuardrailModelSafetyElSafetySettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `category` after provisioning.\nThe harm category.\nPossible values:\nHARM_CATEGORY_HATE_SPEECH\nHARM_CATEGORY_DANGEROUS_CONTENT\nHARM_CATEGORY_HARASSMENT\nHARM_CATEGORY_SEXUALLY_EXPLICIT Possible values: [\"HARM_CATEGORY_HATE_SPEECH\", \"HARM_CATEGORY_DANGEROUS_CONTENT\", \"HARM_CATEGORY_HARASSMENT\", \"HARM_CATEGORY_SEXUALLY_EXPLICIT\"]"]
    pub fn category(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }
    #[doc = "Get a reference to the value of field `threshold` after provisioning.\nThe harm block threshold.\nPossible values:\nBLOCK_LOW_AND_ABOVE\nBLOCK_MEDIUM_AND_ABOVE\nBLOCK_ONLY_HIGH\nBLOCK_NONE\nOFF Possible values: [\"BLOCK_LOW_AND_ABOVE\", \"BLOCK_MEDIUM_AND_ABOVE\", \"BLOCK_ONLY_HIGH\", \"BLOCK_NONE\", \"OFF\"]"]
    pub fn threshold(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.base))
    }
}
#[derive(Serialize, Default)]
struct CesGuardrailModelSafetyElDynamic {
    safety_settings: Option<DynamicBlock<CesGuardrailModelSafetyElSafetySettingsEl>>,
}
#[derive(Serialize)]
pub struct CesGuardrailModelSafetyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    safety_settings: Option<Vec<CesGuardrailModelSafetyElSafetySettingsEl>>,
    dynamic: CesGuardrailModelSafetyElDynamic,
}
impl CesGuardrailModelSafetyEl {
    #[doc = "Set the field `safety_settings`.\n"]
    pub fn set_safety_settings(
        mut self,
        v: impl Into<BlockAssignable<CesGuardrailModelSafetyElSafetySettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.safety_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.safety_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesGuardrailModelSafetyEl {
    type O = BlockAssignable<CesGuardrailModelSafetyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesGuardrailModelSafetyEl {}
impl BuildCesGuardrailModelSafetyEl {
    pub fn build(self) -> CesGuardrailModelSafetyEl {
        CesGuardrailModelSafetyEl {
            safety_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesGuardrailModelSafetyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesGuardrailModelSafetyElRef {
    fn new(shared: StackShared, base: String) -> CesGuardrailModelSafetyElRef {
        CesGuardrailModelSafetyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesGuardrailModelSafetyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `safety_settings` after provisioning.\n"]
    pub fn safety_settings(&self) -> ListRef<CesGuardrailModelSafetyElSafetySettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.safety_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesGuardrailTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CesGuardrailTimeoutsEl {
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
impl ToListMappable for CesGuardrailTimeoutsEl {
    type O = BlockAssignable<CesGuardrailTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesGuardrailTimeoutsEl {}
impl BuildCesGuardrailTimeoutsEl {
    pub fn build(self) -> CesGuardrailTimeoutsEl {
        CesGuardrailTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct CesGuardrailTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesGuardrailTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CesGuardrailTimeoutsElRef {
        CesGuardrailTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesGuardrailTimeoutsElRef {
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
struct CesGuardrailDynamic {
    action: Option<DynamicBlock<CesGuardrailActionEl>>,
    code_callback: Option<DynamicBlock<CesGuardrailCodeCallbackEl>>,
    content_filter: Option<DynamicBlock<CesGuardrailContentFilterEl>>,
    llm_policy: Option<DynamicBlock<CesGuardrailLlmPolicyEl>>,
    llm_prompt_security: Option<DynamicBlock<CesGuardrailLlmPromptSecurityEl>>,
    model_safety: Option<DynamicBlock<CesGuardrailModelSafetyEl>>,
}
