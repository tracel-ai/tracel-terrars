use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DialogflowAgentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    classification_threshold: Option<PrimField<f64>>,
    default_language_code: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_logging: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supported_language_codes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tier: Option<PrimField<String>>,
    time_zone: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DialogflowAgentTimeoutsEl>,
}
struct DialogflowAgent_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DialogflowAgentData>,
}
#[derive(Clone)]
pub struct DialogflowAgent(Rc<DialogflowAgent_>);
impl DialogflowAgent {
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
    #[doc = "Set the field `api_version`.\nAPI version displayed in Dialogflow console. If not specified, V2 API is assumed. Clients are free to query\ndifferent service endpoints for different API versions. However, bots connectors and webhook calls will follow\nthe specified API version.\n* API_VERSION_V1: Legacy V1 API.\n* API_VERSION_V2: V2 API.\n* API_VERSION_V2_BETA_1: V2beta1 API. Possible values: [\"API_VERSION_V1\", \"API_VERSION_V2\", \"API_VERSION_V2_BETA_1\"]"]
    pub fn set_api_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().api_version = Some(v.into());
        self
    }
    #[doc = "Set the field `avatar_uri`.\nThe URI of the agent's avatar, which are used throughout the Dialogflow console. When an image URL is entered\ninto this field, the Dialogflow will save the image in the backend. The address of the backend image returned\nfrom the API will be shown in the [avatarUriBackend] field."]
    pub fn set_avatar_uri(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().avatar_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `classification_threshold`.\nTo filter out false positive results and still get variety in matched natural language inputs for your agent,\nyou can tune the machine learning classification threshold. If the returned score value is less than the threshold\nvalue, then a fallback intent will be triggered or, if there are no fallback intents defined, no intent will be\ntriggered. The score values range from 0.0 (completely uncertain) to 1.0 (completely certain). If set to 0.0, the\ndefault of 0.3 is used."]
    pub fn set_classification_threshold(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().classification_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nThe description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_logging`.\nDetermines whether this agent should log conversation queries."]
    pub fn set_enable_logging(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_logging = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `match_mode`.\nDetermines how intents are detected from user queries.\n* MATCH_MODE_HYBRID: Best for agents with a small number of examples in intents and/or wide use of templates\nsyntax and composite entities.\n* MATCH_MODE_ML_ONLY: Can be used for agents with a large number of examples in intents, especially the ones\nusing @sys.any or very large developer entities. Possible values: [\"MATCH_MODE_HYBRID\", \"MATCH_MODE_ML_ONLY\"]"]
    pub fn set_match_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().match_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `supported_language_codes`.\nThe list of all languages supported by this agent (except for the defaultLanguageCode)."]
    pub fn set_supported_language_codes(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().supported_language_codes = Some(v.into());
        self
    }
    #[doc = "Set the field `tier`.\nThe agent tier. If not specified, TIER_STANDARD is assumed.\n* TIER_STANDARD: Standard tier.\n* TIER_ENTERPRISE: Enterprise tier (Essentials).\n* TIER_ENTERPRISE_PLUS: Enterprise tier (Plus).\nNOTE: Due to consistency issues, the provider will not read this field from the API. Drift is possible between\nthe Terraform state and Dialogflow if the agent tier is changed outside of Terraform. Possible values: [\"TIER_STANDARD\", \"TIER_ENTERPRISE\", \"TIER_ENTERPRISE_PLUS\"]"]
    pub fn set_tier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tier = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DialogflowAgentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `api_version` after provisioning.\nAPI version displayed in Dialogflow console. If not specified, V2 API is assumed. Clients are free to query\ndifferent service endpoints for different API versions. However, bots connectors and webhook calls will follow\nthe specified API version.\n* API_VERSION_V1: Legacy V1 API.\n* API_VERSION_V2: V2 API.\n* API_VERSION_V2_BETA_1: V2beta1 API. Possible values: [\"API_VERSION_V1\", \"API_VERSION_V2\", \"API_VERSION_V2_BETA_1\"]"]
    pub fn api_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `avatar_uri` after provisioning.\nThe URI of the agent's avatar, which are used throughout the Dialogflow console. When an image URL is entered\ninto this field, the Dialogflow will save the image in the backend. The address of the backend image returned\nfrom the API will be shown in the [avatarUriBackend] field."]
    pub fn avatar_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.avatar_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `avatar_uri_backend` after provisioning.\nThe URI of the agent's avatar as returned from the API. Output only. To provide an image URL for the agent avatar,\nthe [avatarUri] field can be used."]
    pub fn avatar_uri_backend(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.avatar_uri_backend", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `classification_threshold` after provisioning.\nTo filter out false positive results and still get variety in matched natural language inputs for your agent,\nyou can tune the machine learning classification threshold. If the returned score value is less than the threshold\nvalue, then a fallback intent will be triggered or, if there are no fallback intents defined, no intent will be\ntriggered. The score values range from 0.0 (completely uncertain) to 1.0 (completely certain). If set to 0.0, the\ndefault of 0.3 is used."]
    pub fn classification_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.classification_threshold", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_language_code` after provisioning.\nThe default language of the agent as a language tag. [See Language Support](https://cloud.google.com/dialogflow/docs/reference/language)\nfor a list of the currently supported language codes. This field cannot be updated after creation."]
    pub fn default_language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_language_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe name of this agent."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_logging` after provisioning.\nDetermines whether this agent should log conversation queries."]
    pub fn enable_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_logging", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `match_mode` after provisioning.\nDetermines how intents are detected from user queries.\n* MATCH_MODE_HYBRID: Best for agents with a small number of examples in intents and/or wide use of templates\nsyntax and composite entities.\n* MATCH_MODE_ML_ONLY: Can be used for agents with a large number of examples in intents, especially the ones\nusing @sys.any or very large developer entities. Possible values: [\"MATCH_MODE_HYBRID\", \"MATCH_MODE_ML_ONLY\"]"]
    pub fn match_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.match_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `supported_language_codes` after provisioning.\nThe list of all languages supported by this agent (except for the defaultLanguageCode)."]
    pub fn supported_language_codes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_language_codes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tier` after provisioning.\nThe agent tier. If not specified, TIER_STANDARD is assumed.\n* TIER_STANDARD: Standard tier.\n* TIER_ENTERPRISE: Enterprise tier (Essentials).\n* TIER_ENTERPRISE_PLUS: Enterprise tier (Plus).\nNOTE: Due to consistency issues, the provider will not read this field from the API. Drift is possible between\nthe Terraform state and Dialogflow if the agent tier is changed outside of Terraform. Possible values: [\"TIER_STANDARD\", \"TIER_ENTERPRISE\", \"TIER_ENTERPRISE_PLUS\"]"]
    pub fn tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\nThe time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York,\nEurope/Paris."]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowAgentTimeoutsElRef {
        DialogflowAgentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DialogflowAgent {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DialogflowAgent {}
impl ToListMappable for DialogflowAgent {
    type O = ListRef<DialogflowAgentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DialogflowAgent_ {
    fn extract_resource_type(&self) -> String {
        "google_dialogflow_agent".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDialogflowAgent {
    pub tf_id: String,
    #[doc = "The default language of the agent as a language tag. [See Language Support](https://cloud.google.com/dialogflow/docs/reference/language)\nfor a list of the currently supported language codes. This field cannot be updated after creation."]
    pub default_language_code: PrimField<String>,
    #[doc = "The name of this agent."]
    pub display_name: PrimField<String>,
    #[doc = "The time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York,\nEurope/Paris."]
    pub time_zone: PrimField<String>,
}
impl BuildDialogflowAgent {
    pub fn build(self, stack: &mut Stack) -> DialogflowAgent {
        let out = DialogflowAgent(Rc::new(DialogflowAgent_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DialogflowAgentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                api_version: core::default::Default::default(),
                avatar_uri: core::default::Default::default(),
                classification_threshold: core::default::Default::default(),
                default_language_code: self.default_language_code,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                enable_logging: core::default::Default::default(),
                id: core::default::Default::default(),
                match_mode: core::default::Default::default(),
                project: core::default::Default::default(),
                supported_language_codes: core::default::Default::default(),
                tier: core::default::Default::default(),
                time_zone: self.time_zone,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DialogflowAgentRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowAgentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DialogflowAgentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_version` after provisioning.\nAPI version displayed in Dialogflow console. If not specified, V2 API is assumed. Clients are free to query\ndifferent service endpoints for different API versions. However, bots connectors and webhook calls will follow\nthe specified API version.\n* API_VERSION_V1: Legacy V1 API.\n* API_VERSION_V2: V2 API.\n* API_VERSION_V2_BETA_1: V2beta1 API. Possible values: [\"API_VERSION_V1\", \"API_VERSION_V2\", \"API_VERSION_V2_BETA_1\"]"]
    pub fn api_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `avatar_uri` after provisioning.\nThe URI of the agent's avatar, which are used throughout the Dialogflow console. When an image URL is entered\ninto this field, the Dialogflow will save the image in the backend. The address of the backend image returned\nfrom the API will be shown in the [avatarUriBackend] field."]
    pub fn avatar_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.avatar_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `avatar_uri_backend` after provisioning.\nThe URI of the agent's avatar as returned from the API. Output only. To provide an image URL for the agent avatar,\nthe [avatarUri] field can be used."]
    pub fn avatar_uri_backend(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.avatar_uri_backend", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `classification_threshold` after provisioning.\nTo filter out false positive results and still get variety in matched natural language inputs for your agent,\nyou can tune the machine learning classification threshold. If the returned score value is less than the threshold\nvalue, then a fallback intent will be triggered or, if there are no fallback intents defined, no intent will be\ntriggered. The score values range from 0.0 (completely uncertain) to 1.0 (completely certain). If set to 0.0, the\ndefault of 0.3 is used."]
    pub fn classification_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.classification_threshold", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_language_code` after provisioning.\nThe default language of the agent as a language tag. [See Language Support](https://cloud.google.com/dialogflow/docs/reference/language)\nfor a list of the currently supported language codes. This field cannot be updated after creation."]
    pub fn default_language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_language_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe name of this agent."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_logging` after provisioning.\nDetermines whether this agent should log conversation queries."]
    pub fn enable_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_logging", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `match_mode` after provisioning.\nDetermines how intents are detected from user queries.\n* MATCH_MODE_HYBRID: Best for agents with a small number of examples in intents and/or wide use of templates\nsyntax and composite entities.\n* MATCH_MODE_ML_ONLY: Can be used for agents with a large number of examples in intents, especially the ones\nusing @sys.any or very large developer entities. Possible values: [\"MATCH_MODE_HYBRID\", \"MATCH_MODE_ML_ONLY\"]"]
    pub fn match_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.match_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `supported_language_codes` after provisioning.\nThe list of all languages supported by this agent (except for the defaultLanguageCode)."]
    pub fn supported_language_codes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_language_codes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tier` after provisioning.\nThe agent tier. If not specified, TIER_STANDARD is assumed.\n* TIER_STANDARD: Standard tier.\n* TIER_ENTERPRISE: Enterprise tier (Essentials).\n* TIER_ENTERPRISE_PLUS: Enterprise tier (Plus).\nNOTE: Due to consistency issues, the provider will not read this field from the API. Drift is possible between\nthe Terraform state and Dialogflow if the agent tier is changed outside of Terraform. Possible values: [\"TIER_STANDARD\", \"TIER_ENTERPRISE\", \"TIER_ENTERPRISE_PLUS\"]"]
    pub fn tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\nThe time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York,\nEurope/Paris."]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowAgentTimeoutsElRef {
        DialogflowAgentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowAgentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DialogflowAgentTimeoutsEl {
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
impl ToListMappable for DialogflowAgentTimeoutsEl {
    type O = BlockAssignable<DialogflowAgentTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowAgentTimeoutsEl {}
impl BuildDialogflowAgentTimeoutsEl {
    pub fn build(self) -> DialogflowAgentTimeoutsEl {
        DialogflowAgentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DialogflowAgentTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowAgentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowAgentTimeoutsElRef {
        DialogflowAgentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowAgentTimeoutsElRef {
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
