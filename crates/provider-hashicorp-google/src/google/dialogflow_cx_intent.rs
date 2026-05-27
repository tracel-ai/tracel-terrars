use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DialogflowCxIntentData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_default_negative_intent: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_default_welcome_intent: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_fallback: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<Vec<DialogflowCxIntentParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DialogflowCxIntentTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    training_phrases: Option<Vec<DialogflowCxIntentTrainingPhrasesEl>>,
    dynamic: DialogflowCxIntentDynamic,
}
struct DialogflowCxIntent_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DialogflowCxIntentData>,
}
#[derive(Clone)]
pub struct DialogflowCxIntent(Rc<DialogflowCxIntent_>);
impl DialogflowCxIntent {
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
    #[doc = "Set the field `description`.\nHuman readable description for better understanding an intent like its scope, content, result etc. Maximum character limit: 140 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `is_default_negative_intent`.\nMarks this as the [Default Negative Intent](https://cloud.google.com/dialogflow/cx/docs/concept/intent#negative) for an agent. When you create an agent, a Default Negative Intent is created automatically.\nThe Default Negative Intent cannot be deleted; deleting the 'google_dialogflow_cx_intent' resource does nothing to the underlying GCP resources.\n\n~> Avoid having multiple 'google_dialogflow_cx_intent' resources linked to the same agent with 'is_default_negative_intent = true' because they will compete to control a single Default Negative Intent resource in GCP."]
    pub fn set_is_default_negative_intent(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_default_negative_intent = Some(v.into());
        self
    }
    #[doc = "Set the field `is_default_welcome_intent`.\nMarks this as the [Default Welcome Intent](https://cloud.google.com/dialogflow/cx/docs/concept/intent#welcome) for an agent. When you create an agent, a Default Welcome Intent is created automatically.\nThe Default Welcome Intent cannot be deleted; deleting the 'google_dialogflow_cx_intent' resource does nothing to the underlying GCP resources.\n\n~> Avoid having multiple 'google_dialogflow_cx_intent' resources linked to the same agent with 'is_default_welcome_intent = true' because they will compete to control a single Default Welcome Intent resource in GCP."]
    pub fn set_is_default_welcome_intent(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_default_welcome_intent = Some(v.into());
        self
    }
    #[doc = "Set the field `is_fallback`.\nIndicates whether this is a fallback intent. Currently only default fallback intent is allowed in the agent, which is added upon agent creation.\nAdding training phrases to fallback intent is useful in the case of requests that are mistakenly matched, since training phrases assigned to fallback intents act as negative examples that triggers no-match event.\nTo manage the fallback intent, set 'is_default_negative_intent = true'"]
    pub fn set_is_fallback(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_fallback = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nThe key/value metadata to label an intent. Labels can contain lowercase letters, digits and the symbols '-' and '_'. International characters are allowed, including letters from unicase alphabets. Keys must start with a letter. Keys and values can be no longer than 63 characters and no more than 128 bytes.\nPrefix \"sys-\" is reserved for Dialogflow defined labels. Currently allowed Dialogflow defined labels include: * sys-head * sys-contextual The above labels do not require value. \"sys-head\" means the intent is a head intent. \"sys.contextual\" means the intent is a contextual intent.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `language_code`.\nThe language of the following fields in intent:\nIntent.training_phrases.parts.text\nIf not specified, the agent's default language is used. Many languages are supported. Note: languages must be enabled in the agent before they can be used."]
    pub fn set_language_code(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().language_code = Some(v.into());
        self
    }
    #[doc = "Set the field `parent`.\nThe agent to create an intent for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn set_parent(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent = Some(v.into());
        self
    }
    #[doc = "Set the field `priority`.\nThe priority of this intent. Higher numbers represent higher priorities.\nIf the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the Normal priority in the console.\nIf the supplied value is negative, the intent is ignored in runtime detect intent requests."]
    pub fn set_priority(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().priority = Some(v.into());
        self
    }
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        self,
        v: impl Into<BlockAssignable<DialogflowCxIntentParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DialogflowCxIntentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `training_phrases`.\n"]
    pub fn set_training_phrases(
        self,
        v: impl Into<BlockAssignable<DialogflowCxIntentTrainingPhrasesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().training_phrases = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.training_phrases = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nHuman readable description for better understanding an intent like its scope, content, result etc. Maximum character limit: 140 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the intent, unique within the agent."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `is_default_negative_intent` after provisioning.\nMarks this as the [Default Negative Intent](https://cloud.google.com/dialogflow/cx/docs/concept/intent#negative) for an agent. When you create an agent, a Default Negative Intent is created automatically.\nThe Default Negative Intent cannot be deleted; deleting the 'google_dialogflow_cx_intent' resource does nothing to the underlying GCP resources.\n\n~> Avoid having multiple 'google_dialogflow_cx_intent' resources linked to the same agent with 'is_default_negative_intent = true' because they will compete to control a single Default Negative Intent resource in GCP."]
    pub fn is_default_negative_intent(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_default_negative_intent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `is_default_welcome_intent` after provisioning.\nMarks this as the [Default Welcome Intent](https://cloud.google.com/dialogflow/cx/docs/concept/intent#welcome) for an agent. When you create an agent, a Default Welcome Intent is created automatically.\nThe Default Welcome Intent cannot be deleted; deleting the 'google_dialogflow_cx_intent' resource does nothing to the underlying GCP resources.\n\n~> Avoid having multiple 'google_dialogflow_cx_intent' resources linked to the same agent with 'is_default_welcome_intent = true' because they will compete to control a single Default Welcome Intent resource in GCP."]
    pub fn is_default_welcome_intent(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_default_welcome_intent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `is_fallback` after provisioning.\nIndicates whether this is a fallback intent. Currently only default fallback intent is allowed in the agent, which is added upon agent creation.\nAdding training phrases to fallback intent is useful in the case of requests that are mistakenly matched, since training phrases assigned to fallback intents act as negative examples that triggers no-match event.\nTo manage the fallback intent, set 'is_default_negative_intent = true'"]
    pub fn is_fallback(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_fallback", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe key/value metadata to label an intent. Labels can contain lowercase letters, digits and the symbols '-' and '_'. International characters are allowed, including letters from unicase alphabets. Keys must start with a letter. Keys and values can be no longer than 63 characters and no more than 128 bytes.\nPrefix \"sys-\" is reserved for Dialogflow defined labels. Currently allowed Dialogflow defined labels include: * sys-head * sys-contextual The above labels do not require value. \"sys-head\" means the intent is a head intent. \"sys.contextual\" means the intent is a contextual intent.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\nThe language of the following fields in intent:\nIntent.training_phrases.parts.text\nIf not specified, the agent's default language is used. Many languages are supported. Note: languages must be enabled in the agent before they can be used."]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the intent.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/intents/<Intent ID>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe agent to create an intent for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nThe priority of this intent. Higher numbers represent higher priorities.\nIf the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the Normal priority in the console.\nIf the supplied value is negative, the intent is ignored in runtime detect intent requests."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> ListRef<DialogflowCxIntentParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxIntentTimeoutsElRef {
        DialogflowCxIntentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `training_phrases` after provisioning.\n"]
    pub fn training_phrases(&self) -> ListRef<DialogflowCxIntentTrainingPhrasesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.training_phrases", self.extract_ref()),
        )
    }
}
impl Referable for DialogflowCxIntent {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DialogflowCxIntent {}
impl ToListMappable for DialogflowCxIntent {
    type O = ListRef<DialogflowCxIntentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DialogflowCxIntent_ {
    fn extract_resource_type(&self) -> String {
        "google_dialogflow_cx_intent".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDialogflowCxIntent {
    pub tf_id: String,
    #[doc = "The human-readable name of the intent, unique within the agent."]
    pub display_name: PrimField<String>,
}
impl BuildDialogflowCxIntent {
    pub fn build(self, stack: &mut Stack) -> DialogflowCxIntent {
        let out = DialogflowCxIntent(Rc::new(DialogflowCxIntent_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DialogflowCxIntentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                is_default_negative_intent: core::default::Default::default(),
                is_default_welcome_intent: core::default::Default::default(),
                is_fallback: core::default::Default::default(),
                labels: core::default::Default::default(),
                language_code: core::default::Default::default(),
                parent: core::default::Default::default(),
                priority: core::default::Default::default(),
                parameters: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                training_phrases: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DialogflowCxIntentRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxIntentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DialogflowCxIntentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nHuman readable description for better understanding an intent like its scope, content, result etc. Maximum character limit: 140 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the intent, unique within the agent."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `is_default_negative_intent` after provisioning.\nMarks this as the [Default Negative Intent](https://cloud.google.com/dialogflow/cx/docs/concept/intent#negative) for an agent. When you create an agent, a Default Negative Intent is created automatically.\nThe Default Negative Intent cannot be deleted; deleting the 'google_dialogflow_cx_intent' resource does nothing to the underlying GCP resources.\n\n~> Avoid having multiple 'google_dialogflow_cx_intent' resources linked to the same agent with 'is_default_negative_intent = true' because they will compete to control a single Default Negative Intent resource in GCP."]
    pub fn is_default_negative_intent(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_default_negative_intent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `is_default_welcome_intent` after provisioning.\nMarks this as the [Default Welcome Intent](https://cloud.google.com/dialogflow/cx/docs/concept/intent#welcome) for an agent. When you create an agent, a Default Welcome Intent is created automatically.\nThe Default Welcome Intent cannot be deleted; deleting the 'google_dialogflow_cx_intent' resource does nothing to the underlying GCP resources.\n\n~> Avoid having multiple 'google_dialogflow_cx_intent' resources linked to the same agent with 'is_default_welcome_intent = true' because they will compete to control a single Default Welcome Intent resource in GCP."]
    pub fn is_default_welcome_intent(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_default_welcome_intent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `is_fallback` after provisioning.\nIndicates whether this is a fallback intent. Currently only default fallback intent is allowed in the agent, which is added upon agent creation.\nAdding training phrases to fallback intent is useful in the case of requests that are mistakenly matched, since training phrases assigned to fallback intents act as negative examples that triggers no-match event.\nTo manage the fallback intent, set 'is_default_negative_intent = true'"]
    pub fn is_fallback(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_fallback", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe key/value metadata to label an intent. Labels can contain lowercase letters, digits and the symbols '-' and '_'. International characters are allowed, including letters from unicase alphabets. Keys must start with a letter. Keys and values can be no longer than 63 characters and no more than 128 bytes.\nPrefix \"sys-\" is reserved for Dialogflow defined labels. Currently allowed Dialogflow defined labels include: * sys-head * sys-contextual The above labels do not require value. \"sys-head\" means the intent is a head intent. \"sys.contextual\" means the intent is a contextual intent.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\nThe language of the following fields in intent:\nIntent.training_phrases.parts.text\nIf not specified, the agent's default language is used. Many languages are supported. Note: languages must be enabled in the agent before they can be used."]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the intent.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/intents/<Intent ID>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe agent to create an intent for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nThe priority of this intent. Higher numbers represent higher priorities.\nIf the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the Normal priority in the console.\nIf the supplied value is negative, the intent is ignored in runtime detect intent requests."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> ListRef<DialogflowCxIntentParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxIntentTimeoutsElRef {
        DialogflowCxIntentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `training_phrases` after provisioning.\n"]
    pub fn training_phrases(&self) -> ListRef<DialogflowCxIntentTrainingPhrasesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.training_phrases", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxIntentParametersEl {
    entity_type: PrimField<String>,
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_list: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redact: Option<PrimField<bool>>,
}
impl DialogflowCxIntentParametersEl {
    #[doc = "Set the field `is_list`.\nIndicates whether the parameter represents a list of values."]
    pub fn set_is_list(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_list = Some(v.into());
        self
    }
    #[doc = "Set the field `redact`.\nIndicates whether the parameter content should be redacted in log. If redaction is enabled, the parameter content will be replaced by parameter name during logging.\nNote: the parameter content is subject to redaction if either parameter level redaction or entity type level redaction is enabled."]
    pub fn set_redact(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.redact = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxIntentParametersEl {
    type O = BlockAssignable<DialogflowCxIntentParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxIntentParametersEl {
    #[doc = "The entity type of the parameter.\nFormat: projects/-/locations/-/agents/-/entityTypes/<System Entity Type ID> for system entity types (for example, projects/-/locations/-/agents/-/entityTypes/sys.date), or projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/entityTypes/<Entity Type ID> for developer entity types."]
    pub entity_type: PrimField<String>,
    #[doc = "The unique identifier of the parameter. This field is used by training phrases to annotate their parts."]
    pub id: PrimField<String>,
}
impl BuildDialogflowCxIntentParametersEl {
    pub fn build(self) -> DialogflowCxIntentParametersEl {
        DialogflowCxIntentParametersEl {
            entity_type: self.entity_type,
            id: self.id,
            is_list: core::default::Default::default(),
            redact: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxIntentParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxIntentParametersElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxIntentParametersElRef {
        DialogflowCxIntentParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxIntentParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `entity_type` after provisioning.\nThe entity type of the parameter.\nFormat: projects/-/locations/-/agents/-/entityTypes/<System Entity Type ID> for system entity types (for example, projects/-/locations/-/agents/-/entityTypes/sys.date), or projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/entityTypes/<Entity Type ID> for developer entity types."]
    pub fn entity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.entity_type", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique identifier of the parameter. This field is used by training phrases to annotate their parts."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `is_list` after provisioning.\nIndicates whether the parameter represents a list of values."]
    pub fn is_list(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_list", self.base))
    }
    #[doc = "Get a reference to the value of field `redact` after provisioning.\nIndicates whether the parameter content should be redacted in log. If redaction is enabled, the parameter content will be replaced by parameter name during logging.\nNote: the parameter content is subject to redaction if either parameter level redaction or entity type level redaction is enabled."]
    pub fn redact(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.redact", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxIntentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DialogflowCxIntentTimeoutsEl {
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
impl ToListMappable for DialogflowCxIntentTimeoutsEl {
    type O = BlockAssignable<DialogflowCxIntentTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxIntentTimeoutsEl {}
impl BuildDialogflowCxIntentTimeoutsEl {
    pub fn build(self) -> DialogflowCxIntentTimeoutsEl {
        DialogflowCxIntentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxIntentTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxIntentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxIntentTimeoutsElRef {
        DialogflowCxIntentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxIntentTimeoutsElRef {
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
#[derive(Serialize)]
pub struct DialogflowCxIntentTrainingPhrasesElPartsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter_id: Option<PrimField<String>>,
    text: PrimField<String>,
}
impl DialogflowCxIntentTrainingPhrasesElPartsEl {
    #[doc = "Set the field `parameter_id`.\nThe parameter used to annotate this part of the training phrase. This field is required for annotated parts of the training phrase."]
    pub fn set_parameter_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.parameter_id = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxIntentTrainingPhrasesElPartsEl {
    type O = BlockAssignable<DialogflowCxIntentTrainingPhrasesElPartsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxIntentTrainingPhrasesElPartsEl {
    #[doc = "The text for this part."]
    pub text: PrimField<String>,
}
impl BuildDialogflowCxIntentTrainingPhrasesElPartsEl {
    pub fn build(self) -> DialogflowCxIntentTrainingPhrasesElPartsEl {
        DialogflowCxIntentTrainingPhrasesElPartsEl {
            parameter_id: core::default::Default::default(),
            text: self.text,
        }
    }
}
pub struct DialogflowCxIntentTrainingPhrasesElPartsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxIntentTrainingPhrasesElPartsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxIntentTrainingPhrasesElPartsElRef {
        DialogflowCxIntentTrainingPhrasesElPartsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxIntentTrainingPhrasesElPartsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `parameter_id` after provisioning.\nThe parameter used to annotate this part of the training phrase. This field is required for annotated parts of the training phrase."]
    pub fn parameter_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parameter_id", self.base))
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\nThe text for this part."]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxIntentTrainingPhrasesElDynamic {
    parts: Option<DynamicBlock<DialogflowCxIntentTrainingPhrasesElPartsEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxIntentTrainingPhrasesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    repeat_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parts: Option<Vec<DialogflowCxIntentTrainingPhrasesElPartsEl>>,
    dynamic: DialogflowCxIntentTrainingPhrasesElDynamic,
}
impl DialogflowCxIntentTrainingPhrasesEl {
    #[doc = "Set the field `repeat_count`.\nIndicates how many times this example was added to the intent."]
    pub fn set_repeat_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.repeat_count = Some(v.into());
        self
    }
    #[doc = "Set the field `parts`.\n"]
    pub fn set_parts(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxIntentTrainingPhrasesElPartsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parts = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parts = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxIntentTrainingPhrasesEl {
    type O = BlockAssignable<DialogflowCxIntentTrainingPhrasesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxIntentTrainingPhrasesEl {}
impl BuildDialogflowCxIntentTrainingPhrasesEl {
    pub fn build(self) -> DialogflowCxIntentTrainingPhrasesEl {
        DialogflowCxIntentTrainingPhrasesEl {
            repeat_count: core::default::Default::default(),
            parts: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxIntentTrainingPhrasesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxIntentTrainingPhrasesElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxIntentTrainingPhrasesElRef {
        DialogflowCxIntentTrainingPhrasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxIntentTrainingPhrasesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique identifier of the training phrase."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `repeat_count` after provisioning.\nIndicates how many times this example was added to the intent."]
    pub fn repeat_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.repeat_count", self.base))
    }
    #[doc = "Get a reference to the value of field `parts` after provisioning.\n"]
    pub fn parts(&self) -> ListRef<DialogflowCxIntentTrainingPhrasesElPartsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.parts", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxIntentDynamic {
    parameters: Option<DynamicBlock<DialogflowCxIntentParametersEl>>,
    training_phrases: Option<DynamicBlock<DialogflowCxIntentTrainingPhrasesEl>>,
}
