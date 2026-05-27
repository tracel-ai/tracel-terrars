use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DialogflowCxWebhookData {
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
    disabled: Option<PrimField<bool>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_spell_correction: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_stackdriver_logging: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_settings: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generic_web_service: Option<Vec<DialogflowCxWebhookGenericWebServiceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_directory: Option<Vec<DialogflowCxWebhookServiceDirectoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DialogflowCxWebhookTimeoutsEl>,
    dynamic: DialogflowCxWebhookDynamic,
}
struct DialogflowCxWebhook_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DialogflowCxWebhookData>,
}
#[derive(Clone)]
pub struct DialogflowCxWebhook(Rc<DialogflowCxWebhook_>);
impl DialogflowCxWebhook {
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
    #[doc = "Set the field `disabled`.\nIndicates whether the webhook is disabled."]
    pub fn set_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_spell_correction`.\nDeprecated. Indicates if automatic spell correction is enabled in detect intent requests."]
    pub fn set_enable_spell_correction(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_spell_correction = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_stackdriver_logging`.\nDeprecated. Determines whether this agent should log conversation queries."]
    pub fn set_enable_stackdriver_logging(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_stackdriver_logging = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `parent`.\nThe agent to create a webhook for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn set_parent(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent = Some(v.into());
        self
    }
    #[doc = "Set the field `security_settings`.\nDeprecated. Name of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>."]
    pub fn set_security_settings(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().security_settings = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout`.\nWebhook execution timeout."]
    pub fn set_timeout(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `generic_web_service`.\n"]
    pub fn set_generic_web_service(
        self,
        v: impl Into<BlockAssignable<DialogflowCxWebhookGenericWebServiceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().generic_web_service = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.generic_web_service = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `service_directory`.\n"]
    pub fn set_service_directory(
        self,
        v: impl Into<BlockAssignable<DialogflowCxWebhookServiceDirectoryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_directory = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_directory = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DialogflowCxWebhookTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nIndicates whether the webhook is disabled."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the webhook, unique within the agent."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_spell_correction` after provisioning.\nDeprecated. Indicates if automatic spell correction is enabled in detect intent requests."]
    pub fn enable_spell_correction(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_spell_correction", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_stackdriver_logging` after provisioning.\nDeprecated. Determines whether this agent should log conversation queries."]
    pub fn enable_stackdriver_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_stackdriver_logging", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the webhook.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe agent to create a webhook for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_settings` after provisioning.\nDeprecated. Name of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>."]
    pub fn security_settings(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `start_flow` after provisioning.\nDeprecated. Name of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn start_flow(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_flow", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\nWebhook execution timeout."]
    pub fn timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generic_web_service` after provisioning.\n"]
    pub fn generic_web_service(&self) -> ListRef<DialogflowCxWebhookGenericWebServiceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.generic_web_service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_directory` after provisioning.\n"]
    pub fn service_directory(&self) -> ListRef<DialogflowCxWebhookServiceDirectoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_directory", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxWebhookTimeoutsElRef {
        DialogflowCxWebhookTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DialogflowCxWebhook {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DialogflowCxWebhook {}
impl ToListMappable for DialogflowCxWebhook {
    type O = ListRef<DialogflowCxWebhookRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DialogflowCxWebhook_ {
    fn extract_resource_type(&self) -> String {
        "google_dialogflow_cx_webhook".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDialogflowCxWebhook {
    pub tf_id: String,
    #[doc = "The human-readable name of the webhook, unique within the agent."]
    pub display_name: PrimField<String>,
}
impl BuildDialogflowCxWebhook {
    pub fn build(self, stack: &mut Stack) -> DialogflowCxWebhook {
        let out = DialogflowCxWebhook(Rc::new(DialogflowCxWebhook_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DialogflowCxWebhookData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                disabled: core::default::Default::default(),
                display_name: self.display_name,
                enable_spell_correction: core::default::Default::default(),
                enable_stackdriver_logging: core::default::Default::default(),
                id: core::default::Default::default(),
                parent: core::default::Default::default(),
                security_settings: core::default::Default::default(),
                timeout: core::default::Default::default(),
                generic_web_service: core::default::Default::default(),
                service_directory: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DialogflowCxWebhookRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxWebhookRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DialogflowCxWebhookRef {
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
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nIndicates whether the webhook is disabled."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the webhook, unique within the agent."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_spell_correction` after provisioning.\nDeprecated. Indicates if automatic spell correction is enabled in detect intent requests."]
    pub fn enable_spell_correction(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_spell_correction", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_stackdriver_logging` after provisioning.\nDeprecated. Determines whether this agent should log conversation queries."]
    pub fn enable_stackdriver_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_stackdriver_logging", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the webhook.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe agent to create a webhook for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_settings` after provisioning.\nDeprecated. Name of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>."]
    pub fn security_settings(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `start_flow` after provisioning.\nDeprecated. Name of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn start_flow(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_flow", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\nWebhook execution timeout."]
    pub fn timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generic_web_service` after provisioning.\n"]
    pub fn generic_web_service(&self) -> ListRef<DialogflowCxWebhookGenericWebServiceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.generic_web_service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_directory` after provisioning.\n"]
    pub fn service_directory(&self) -> ListRef<DialogflowCxWebhookServiceDirectoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_directory", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxWebhookTimeoutsElRef {
        DialogflowCxWebhookTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxWebhookGenericWebServiceElOauthConfigEl {
    client_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scopes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_version_for_client_secret: Option<PrimField<String>>,
    token_endpoint: PrimField<String>,
}
impl DialogflowCxWebhookGenericWebServiceElOauthConfigEl {
    #[doc = "Set the field `client_secret`.\nThe client secret provided by the 3rd party platform.  If the\n'secret_version_for_client_secret' field is set, this field will be\nignored."]
    pub fn set_client_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_secret = Some(v.into());
        self
    }
    #[doc = "Set the field `scopes`.\nThe OAuth scopes to grant."]
    pub fn set_scopes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.scopes = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_version_for_client_secret`.\nThe name of the SecretManager secret version resource storing the\nclient secret. If this field is set, the 'client_secret' field will be\nignored.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'"]
    pub fn set_secret_version_for_client_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_version_for_client_secret = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxWebhookGenericWebServiceElOauthConfigEl {
    type O = BlockAssignable<DialogflowCxWebhookGenericWebServiceElOauthConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxWebhookGenericWebServiceElOauthConfigEl {
    #[doc = "The client ID provided by the 3rd party platform."]
    pub client_id: PrimField<String>,
    #[doc = "The token endpoint provided by the 3rd party platform to exchange an\naccess token."]
    pub token_endpoint: PrimField<String>,
}
impl BuildDialogflowCxWebhookGenericWebServiceElOauthConfigEl {
    pub fn build(self) -> DialogflowCxWebhookGenericWebServiceElOauthConfigEl {
        DialogflowCxWebhookGenericWebServiceElOauthConfigEl {
            client_id: self.client_id,
            client_secret: core::default::Default::default(),
            scopes: core::default::Default::default(),
            secret_version_for_client_secret: core::default::Default::default(),
            token_endpoint: self.token_endpoint,
        }
    }
}
pub struct DialogflowCxWebhookGenericWebServiceElOauthConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxWebhookGenericWebServiceElOauthConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxWebhookGenericWebServiceElOauthConfigElRef {
        DialogflowCxWebhookGenericWebServiceElOauthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxWebhookGenericWebServiceElOauthConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nThe client ID provided by the 3rd party platform."]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\nThe client secret provided by the 3rd party platform.  If the\n'secret_version_for_client_secret' field is set, this field will be\nignored."]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scopes` after provisioning.\nThe OAuth scopes to grant."]
    pub fn scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_version_for_client_secret` after provisioning.\nThe name of the SecretManager secret version resource storing the\nclient secret. If this field is set, the 'client_secret' field will be\nignored.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'"]
    pub fn secret_version_for_client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_version_for_client_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\nThe token endpoint provided by the 3rd party platform to exchange an\naccess token."]
    pub fn token_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.token_endpoint", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxWebhookGenericWebServiceElSecretVersionsForRequestHeadersEl {
    key: PrimField<String>,
    secret_version: PrimField<String>,
}
impl DialogflowCxWebhookGenericWebServiceElSecretVersionsForRequestHeadersEl {}
impl ToListMappable for DialogflowCxWebhookGenericWebServiceElSecretVersionsForRequestHeadersEl {
    type O =
        BlockAssignable<DialogflowCxWebhookGenericWebServiceElSecretVersionsForRequestHeadersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxWebhookGenericWebServiceElSecretVersionsForRequestHeadersEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = "The SecretManager secret version resource storing the header value.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'"]
    pub secret_version: PrimField<String>,
}
impl BuildDialogflowCxWebhookGenericWebServiceElSecretVersionsForRequestHeadersEl {
    pub fn build(self) -> DialogflowCxWebhookGenericWebServiceElSecretVersionsForRequestHeadersEl {
        DialogflowCxWebhookGenericWebServiceElSecretVersionsForRequestHeadersEl {
            key: self.key,
            secret_version: self.secret_version,
        }
    }
}
pub struct DialogflowCxWebhookGenericWebServiceElSecretVersionsForRequestHeadersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxWebhookGenericWebServiceElSecretVersionsForRequestHeadersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxWebhookGenericWebServiceElSecretVersionsForRequestHeadersElRef {
        DialogflowCxWebhookGenericWebServiceElSecretVersionsForRequestHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxWebhookGenericWebServiceElSecretVersionsForRequestHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_version` after provisioning.\nThe SecretManager secret version resource storing the header value.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'"]
    pub fn secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_version", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxWebhookGenericWebServiceElServiceAccountAuthConfigEl {
    service_account: PrimField<String>,
}
impl DialogflowCxWebhookGenericWebServiceElServiceAccountAuthConfigEl {}
impl ToListMappable for DialogflowCxWebhookGenericWebServiceElServiceAccountAuthConfigEl {
    type O = BlockAssignable<DialogflowCxWebhookGenericWebServiceElServiceAccountAuthConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxWebhookGenericWebServiceElServiceAccountAuthConfigEl {
    #[doc = "The email address of the service account used to authenticate the webhook call.\nDialogflow uses this service account to exchange an access token and the access\ntoken is then sent in the **Authorization** header of the webhook request.\n\nThe service account must have the **roles/iam.serviceAccountTokenCreator** role\ngranted to the\n[Dialogflow service agent](https://cloud.google.com/iam/docs/service-agents?_gl=1*1jsujvh*_ga*NjYxMzU3OTg2LjE3Njc3MzQ4NjM.*_ga_WH2QY8WWF5*czE3Njc3MzQ2MjgkbzIkZzEkdDE3Njc3MzQ3NzQkajYwJGwwJGgw#dialogflow-service-agent)."]
    pub service_account: PrimField<String>,
}
impl BuildDialogflowCxWebhookGenericWebServiceElServiceAccountAuthConfigEl {
    pub fn build(self) -> DialogflowCxWebhookGenericWebServiceElServiceAccountAuthConfigEl {
        DialogflowCxWebhookGenericWebServiceElServiceAccountAuthConfigEl {
            service_account: self.service_account,
        }
    }
}
pub struct DialogflowCxWebhookGenericWebServiceElServiceAccountAuthConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxWebhookGenericWebServiceElServiceAccountAuthConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxWebhookGenericWebServiceElServiceAccountAuthConfigElRef {
        DialogflowCxWebhookGenericWebServiceElServiceAccountAuthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxWebhookGenericWebServiceElServiceAccountAuthConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nThe email address of the service account used to authenticate the webhook call.\nDialogflow uses this service account to exchange an access token and the access\ntoken is then sent in the **Authorization** header of the webhook request.\n\nThe service account must have the **roles/iam.serviceAccountTokenCreator** role\ngranted to the\n[Dialogflow service agent](https://cloud.google.com/iam/docs/service-agents?_gl=1*1jsujvh*_ga*NjYxMzU3OTg2LjE3Njc3MzQ4NjM.*_ga_WH2QY8WWF5*czE3Njc3MzQ2MjgkbzIkZzEkdDE3Njc3MzQ3NzQkajYwJGwwJGgw#dialogflow-service-agent)."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxWebhookGenericWebServiceElDynamic {
    oauth_config: Option<DynamicBlock<DialogflowCxWebhookGenericWebServiceElOauthConfigEl>>,
    secret_versions_for_request_headers: Option<
        DynamicBlock<DialogflowCxWebhookGenericWebServiceElSecretVersionsForRequestHeadersEl>,
    >,
    service_account_auth_config:
        Option<DynamicBlock<DialogflowCxWebhookGenericWebServiceElServiceAccountAuthConfigEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxWebhookGenericWebServiceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_ca_certs: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter_mapping: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_body: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_version_for_username_password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_agent_auth: Option<PrimField<String>>,
    uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    webhook_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth_config: Option<Vec<DialogflowCxWebhookGenericWebServiceElOauthConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_versions_for_request_headers:
        Option<Vec<DialogflowCxWebhookGenericWebServiceElSecretVersionsForRequestHeadersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_auth_config:
        Option<Vec<DialogflowCxWebhookGenericWebServiceElServiceAccountAuthConfigEl>>,
    dynamic: DialogflowCxWebhookGenericWebServiceElDynamic,
}
impl DialogflowCxWebhookGenericWebServiceEl {
    #[doc = "Set the field `allowed_ca_certs`.\nSpecifies a list of allowed custom CA certificates (in DER format) for\nHTTPS verification. This overrides the default SSL trust store. If this\nis empty or unspecified, Dialogflow will use Google's default trust store\nto verify certificates.\nN.B. Make sure the HTTPS server certificates are signed with \"subject alt\nname\". For instance a certificate can be self-signed using the following\ncommand,\nopenssl x509 -req -days 200 -in example.com.csr \\\n-signkey example.com.key \\\n-out example.com.crt \\\n-extfile <(printf \"\\nsubjectAltName='DNS:www.example.com'\")"]
    pub fn set_allowed_ca_certs(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowed_ca_certs = Some(v.into());
        self
    }
    #[doc = "Set the field `http_method`.\nHTTP method for the flexible webhook calls. Standard webhook always uses\nPOST. Possible values: [\"POST\", \"GET\", \"HEAD\", \"PUT\", \"DELETE\", \"PATCH\", \"OPTIONS\"]"]
    pub fn set_http_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http_method = Some(v.into());
        self
    }
    #[doc = "Set the field `parameter_mapping`.\nMaps the values extracted from specific fields of the flexible webhook\nresponse into session parameters.\n- Key: session parameter name\n- Value: field path in the webhook response"]
    pub fn set_parameter_mapping(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.parameter_mapping = Some(v.into());
        self
    }
    #[doc = "Set the field `request_body`.\nDefines a custom JSON object as request body to send to flexible webhook."]
    pub fn set_request_body(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.request_body = Some(v.into());
        self
    }
    #[doc = "Set the field `request_headers`.\nThe HTTP request headers to send together with webhook requests."]
    pub fn set_request_headers(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.request_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_version_for_username_password`.\nThe SecretManager secret version resource storing the username:password\npair for HTTP Basic authentication.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'"]
    pub fn set_secret_version_for_username_password(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.secret_version_for_username_password = Some(v.into());
        self
    }
    #[doc = "Set the field `service_agent_auth`.\nIndicate the auth token type generated from the [Diglogflow service\nagent](https://cloud.google.com/iam/docs/service-agents#dialogflow-service-agent).\nThe generated token is sent in the Authorization header. Possible values: [\"NONE\", \"ID_TOKEN\", \"ACCESS_TOKEN\"]"]
    pub fn set_service_agent_auth(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_agent_auth = Some(v.into());
        self
    }
    #[doc = "Set the field `webhook_type`.\nType of the webhook. Possible values: [\"STANDARD\", \"FLEXIBLE\"]"]
    pub fn set_webhook_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.webhook_type = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth_config`.\n"]
    pub fn set_oauth_config(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxWebhookGenericWebServiceElOauthConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oauth_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `secret_versions_for_request_headers`.\n"]
    pub fn set_secret_versions_for_request_headers(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxWebhookGenericWebServiceElSecretVersionsForRequestHeadersEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.secret_versions_for_request_headers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_versions_for_request_headers = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `service_account_auth_config`.\n"]
    pub fn set_service_account_auth_config(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxWebhookGenericWebServiceElServiceAccountAuthConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_account_auth_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_account_auth_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxWebhookGenericWebServiceEl {
    type O = BlockAssignable<DialogflowCxWebhookGenericWebServiceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxWebhookGenericWebServiceEl {
    #[doc = "The webhook URI for receiving POST requests. It must use https protocol."]
    pub uri: PrimField<String>,
}
impl BuildDialogflowCxWebhookGenericWebServiceEl {
    pub fn build(self) -> DialogflowCxWebhookGenericWebServiceEl {
        DialogflowCxWebhookGenericWebServiceEl {
            allowed_ca_certs: core::default::Default::default(),
            http_method: core::default::Default::default(),
            parameter_mapping: core::default::Default::default(),
            request_body: core::default::Default::default(),
            request_headers: core::default::Default::default(),
            secret_version_for_username_password: core::default::Default::default(),
            service_agent_auth: core::default::Default::default(),
            uri: self.uri,
            webhook_type: core::default::Default::default(),
            oauth_config: core::default::Default::default(),
            secret_versions_for_request_headers: core::default::Default::default(),
            service_account_auth_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxWebhookGenericWebServiceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxWebhookGenericWebServiceElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxWebhookGenericWebServiceElRef {
        DialogflowCxWebhookGenericWebServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxWebhookGenericWebServiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_ca_certs` after provisioning.\nSpecifies a list of allowed custom CA certificates (in DER format) for\nHTTPS verification. This overrides the default SSL trust store. If this\nis empty or unspecified, Dialogflow will use Google's default trust store\nto verify certificates.\nN.B. Make sure the HTTPS server certificates are signed with \"subject alt\nname\". For instance a certificate can be self-signed using the following\ncommand,\nopenssl x509 -req -days 200 -in example.com.csr \\\n-signkey example.com.key \\\n-out example.com.crt \\\n-extfile <(printf \"\\nsubjectAltName='DNS:www.example.com'\")"]
    pub fn allowed_ca_certs(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_ca_certs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `http_method` after provisioning.\nHTTP method for the flexible webhook calls. Standard webhook always uses\nPOST. Possible values: [\"POST\", \"GET\", \"HEAD\", \"PUT\", \"DELETE\", \"PATCH\", \"OPTIONS\"]"]
    pub fn http_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_method", self.base))
    }
    #[doc = "Get a reference to the value of field `parameter_mapping` after provisioning.\nMaps the values extracted from specific fields of the flexible webhook\nresponse into session parameters.\n- Key: session parameter name\n- Value: field path in the webhook response"]
    pub fn parameter_mapping(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.parameter_mapping", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_body` after provisioning.\nDefines a custom JSON object as request body to send to flexible webhook."]
    pub fn request_body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_body", self.base))
    }
    #[doc = "Get a reference to the value of field `request_headers` after provisioning.\nThe HTTP request headers to send together with webhook requests."]
    pub fn request_headers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.request_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secret_version_for_username_password` after provisioning.\nThe SecretManager secret version resource storing the username:password\npair for HTTP Basic authentication.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'"]
    pub fn secret_version_for_username_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_version_for_username_password", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_agent_auth` after provisioning.\nIndicate the auth token type generated from the [Diglogflow service\nagent](https://cloud.google.com/iam/docs/service-agents#dialogflow-service-agent).\nThe generated token is sent in the Authorization header. Possible values: [\"NONE\", \"ID_TOKEN\", \"ACCESS_TOKEN\"]"]
    pub fn service_agent_auth(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_agent_auth", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe webhook URI for receiving POST requests. It must use https protocol."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
    #[doc = "Get a reference to the value of field `webhook_type` after provisioning.\nType of the webhook. Possible values: [\"STANDARD\", \"FLEXIBLE\"]"]
    pub fn webhook_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.webhook_type", self.base))
    }
    #[doc = "Get a reference to the value of field `oauth_config` after provisioning.\n"]
    pub fn oauth_config(&self) -> ListRef<DialogflowCxWebhookGenericWebServiceElOauthConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oauth_config", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account_auth_config` after provisioning.\n"]
    pub fn service_account_auth_config(
        &self,
    ) -> ListRef<DialogflowCxWebhookGenericWebServiceElServiceAccountAuthConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_account_auth_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxWebhookServiceDirectoryElGenericWebServiceElOauthConfigEl {
    client_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scopes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_version_for_client_secret: Option<PrimField<String>>,
    token_endpoint: PrimField<String>,
}
impl DialogflowCxWebhookServiceDirectoryElGenericWebServiceElOauthConfigEl {
    #[doc = "Set the field `client_secret`.\nThe client secret provided by the 3rd party platform.  If the\n'secret_version_for_client_secret' field is set, this field will be\nignored."]
    pub fn set_client_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_secret = Some(v.into());
        self
    }
    #[doc = "Set the field `scopes`.\nThe OAuth scopes to grant."]
    pub fn set_scopes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.scopes = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_version_for_client_secret`.\nThe name of the SecretManager secret version resource storing the\nclient secret. If this field is set, the 'client_secret' field will be\nignored.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'"]
    pub fn set_secret_version_for_client_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_version_for_client_secret = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxWebhookServiceDirectoryElGenericWebServiceElOauthConfigEl {
    type O = BlockAssignable<DialogflowCxWebhookServiceDirectoryElGenericWebServiceElOauthConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxWebhookServiceDirectoryElGenericWebServiceElOauthConfigEl {
    #[doc = "The client ID provided by the 3rd party platform."]
    pub client_id: PrimField<String>,
    #[doc = "The token endpoint provided by the 3rd party platform to exchange an\naccess token."]
    pub token_endpoint: PrimField<String>,
}
impl BuildDialogflowCxWebhookServiceDirectoryElGenericWebServiceElOauthConfigEl {
    pub fn build(self) -> DialogflowCxWebhookServiceDirectoryElGenericWebServiceElOauthConfigEl {
        DialogflowCxWebhookServiceDirectoryElGenericWebServiceElOauthConfigEl {
            client_id: self.client_id,
            client_secret: core::default::Default::default(),
            scopes: core::default::Default::default(),
            secret_version_for_client_secret: core::default::Default::default(),
            token_endpoint: self.token_endpoint,
        }
    }
}
pub struct DialogflowCxWebhookServiceDirectoryElGenericWebServiceElOauthConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxWebhookServiceDirectoryElGenericWebServiceElOauthConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxWebhookServiceDirectoryElGenericWebServiceElOauthConfigElRef {
        DialogflowCxWebhookServiceDirectoryElGenericWebServiceElOauthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxWebhookServiceDirectoryElGenericWebServiceElOauthConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nThe client ID provided by the 3rd party platform."]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\nThe client secret provided by the 3rd party platform.  If the\n'secret_version_for_client_secret' field is set, this field will be\nignored."]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scopes` after provisioning.\nThe OAuth scopes to grant."]
    pub fn scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_version_for_client_secret` after provisioning.\nThe name of the SecretManager secret version resource storing the\nclient secret. If this field is set, the 'client_secret' field will be\nignored.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'"]
    pub fn secret_version_for_client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_version_for_client_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\nThe token endpoint provided by the 3rd party platform to exchange an\naccess token."]
    pub fn token_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.token_endpoint", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxWebhookServiceDirectoryElGenericWebServiceElSecretVersionsForRequestHeadersEl
{
    key: PrimField<String>,
    secret_version: PrimField<String>,
}
impl DialogflowCxWebhookServiceDirectoryElGenericWebServiceElSecretVersionsForRequestHeadersEl {}
impl ToListMappable
    for DialogflowCxWebhookServiceDirectoryElGenericWebServiceElSecretVersionsForRequestHeadersEl
{
    type O = BlockAssignable<
        DialogflowCxWebhookServiceDirectoryElGenericWebServiceElSecretVersionsForRequestHeadersEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxWebhookServiceDirectoryElGenericWebServiceElSecretVersionsForRequestHeadersEl
{
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = "The SecretManager secret version resource storing the header value.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'"]
    pub secret_version: PrimField<String>,
}
impl
    BuildDialogflowCxWebhookServiceDirectoryElGenericWebServiceElSecretVersionsForRequestHeadersEl
{
    pub fn build(
        self,
    ) -> DialogflowCxWebhookServiceDirectoryElGenericWebServiceElSecretVersionsForRequestHeadersEl
    {
        DialogflowCxWebhookServiceDirectoryElGenericWebServiceElSecretVersionsForRequestHeadersEl {
            key: self.key,
            secret_version: self.secret_version,
        }
    }
}
pub struct DialogflowCxWebhookServiceDirectoryElGenericWebServiceElSecretVersionsForRequestHeadersElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DialogflowCxWebhookServiceDirectoryElGenericWebServiceElSecretVersionsForRequestHeadersElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxWebhookServiceDirectoryElGenericWebServiceElSecretVersionsForRequestHeadersElRef
    {
        DialogflowCxWebhookServiceDirectoryElGenericWebServiceElSecretVersionsForRequestHeadersElRef { shared : shared , base : base . to_string () , }
    }
}
impl DialogflowCxWebhookServiceDirectoryElGenericWebServiceElSecretVersionsForRequestHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_version` after provisioning.\nThe SecretManager secret version resource storing the header value.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'"]
    pub fn secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_version", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxWebhookServiceDirectoryElGenericWebServiceElServiceAccountAuthConfigEl {
    service_account: PrimField<String>,
}
impl DialogflowCxWebhookServiceDirectoryElGenericWebServiceElServiceAccountAuthConfigEl {}
impl ToListMappable
    for DialogflowCxWebhookServiceDirectoryElGenericWebServiceElServiceAccountAuthConfigEl
{
    type O = BlockAssignable<
        DialogflowCxWebhookServiceDirectoryElGenericWebServiceElServiceAccountAuthConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxWebhookServiceDirectoryElGenericWebServiceElServiceAccountAuthConfigEl {
    #[doc = "The email address of the service account used to authenticate the webhook call.\nDialogflow uses this service account to exchange an access token and the access\ntoken is then sent in the **Authorization** header of the webhook request.\n\nThe service account must have the **roles/iam.serviceAccountTokenCreator** role\ngranted to the\n[Dialogflow service agent](https://cloud.google.com/iam/docs/service-agents?_gl=1*1jsujvh*_ga*NjYxMzU3OTg2LjE3Njc3MzQ4NjM.*_ga_WH2QY8WWF5*czE3Njc3MzQ2MjgkbzIkZzEkdDE3Njc3MzQ3NzQkajYwJGwwJGgw#dialogflow-service-agent)."]
    pub service_account: PrimField<String>,
}
impl BuildDialogflowCxWebhookServiceDirectoryElGenericWebServiceElServiceAccountAuthConfigEl {
    pub fn build(
        self,
    ) -> DialogflowCxWebhookServiceDirectoryElGenericWebServiceElServiceAccountAuthConfigEl {
        DialogflowCxWebhookServiceDirectoryElGenericWebServiceElServiceAccountAuthConfigEl {
            service_account: self.service_account,
        }
    }
}
pub struct DialogflowCxWebhookServiceDirectoryElGenericWebServiceElServiceAccountAuthConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxWebhookServiceDirectoryElGenericWebServiceElServiceAccountAuthConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxWebhookServiceDirectoryElGenericWebServiceElServiceAccountAuthConfigElRef {
        DialogflowCxWebhookServiceDirectoryElGenericWebServiceElServiceAccountAuthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxWebhookServiceDirectoryElGenericWebServiceElServiceAccountAuthConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nThe email address of the service account used to authenticate the webhook call.\nDialogflow uses this service account to exchange an access token and the access\ntoken is then sent in the **Authorization** header of the webhook request.\n\nThe service account must have the **roles/iam.serviceAccountTokenCreator** role\ngranted to the\n[Dialogflow service agent](https://cloud.google.com/iam/docs/service-agents?_gl=1*1jsujvh*_ga*NjYxMzU3OTg2LjE3Njc3MzQ4NjM.*_ga_WH2QY8WWF5*czE3Njc3MzQ2MjgkbzIkZzEkdDE3Njc3MzQ3NzQkajYwJGwwJGgw#dialogflow-service-agent)."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxWebhookServiceDirectoryElGenericWebServiceElDynamic { oauth_config : Option < DynamicBlock < DialogflowCxWebhookServiceDirectoryElGenericWebServiceElOauthConfigEl >> , secret_versions_for_request_headers : Option < DynamicBlock < DialogflowCxWebhookServiceDirectoryElGenericWebServiceElSecretVersionsForRequestHeadersEl >> , service_account_auth_config : Option < DynamicBlock < DialogflowCxWebhookServiceDirectoryElGenericWebServiceElServiceAccountAuthConfigEl >> , }
#[derive(Serialize)]
pub struct DialogflowCxWebhookServiceDirectoryElGenericWebServiceEl { # [serde (skip_serializing_if = "Option::is_none")] allowed_ca_certs : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] http_method : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] parameter_mapping : Option < RecField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] request_body : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] request_headers : Option < RecField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] secret_version_for_username_password : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] service_agent_auth : Option < PrimField < String > > , uri : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] webhook_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] oauth_config : Option < Vec < DialogflowCxWebhookServiceDirectoryElGenericWebServiceElOauthConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] secret_versions_for_request_headers : Option < Vec < DialogflowCxWebhookServiceDirectoryElGenericWebServiceElSecretVersionsForRequestHeadersEl > > , # [serde (skip_serializing_if = "Option::is_none")] service_account_auth_config : Option < Vec < DialogflowCxWebhookServiceDirectoryElGenericWebServiceElServiceAccountAuthConfigEl > > , dynamic : DialogflowCxWebhookServiceDirectoryElGenericWebServiceElDynamic , }
impl DialogflowCxWebhookServiceDirectoryElGenericWebServiceEl {
    #[doc = "Set the field `allowed_ca_certs`.\nSpecifies a list of allowed custom CA certificates (in DER format) for\nHTTPS verification. This overrides the default SSL trust store. If this\nis empty or unspecified, Dialogflow will use Google's default trust store\nto verify certificates.\nN.B. Make sure the HTTPS server certificates are signed with \"subject alt\nname\". For instance a certificate can be self-signed using the following\ncommand,\nopenssl x509 -req -days 200 -in example.com.csr \\\n-signkey example.com.key \\\n-out example.com.crt \\\n-extfile <(printf \"\\nsubjectAltName='DNS:www.example.com'\")"]
    pub fn set_allowed_ca_certs(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowed_ca_certs = Some(v.into());
        self
    }
    #[doc = "Set the field `http_method`.\nHTTP method for the flexible webhook calls. Standard webhook always uses\nPOST. Possible values: [\"POST\", \"GET\", \"HEAD\", \"PUT\", \"DELETE\", \"PATCH\", \"OPTIONS\"]"]
    pub fn set_http_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http_method = Some(v.into());
        self
    }
    #[doc = "Set the field `parameter_mapping`.\nMaps the values extracted from specific fields of the flexible webhook\nresponse into session parameters.\n- Key: session parameter name\n- Value: field path in the webhook response"]
    pub fn set_parameter_mapping(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.parameter_mapping = Some(v.into());
        self
    }
    #[doc = "Set the field `request_body`.\nDefines a custom JSON object as request body to send to flexible webhook."]
    pub fn set_request_body(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.request_body = Some(v.into());
        self
    }
    #[doc = "Set the field `request_headers`.\nThe HTTP request headers to send together with webhook requests."]
    pub fn set_request_headers(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.request_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_version_for_username_password`.\nThe SecretManager secret version resource storing the username:password\npair for HTTP Basic authentication.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'"]
    pub fn set_secret_version_for_username_password(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.secret_version_for_username_password = Some(v.into());
        self
    }
    #[doc = "Set the field `service_agent_auth`.\nIndicate the auth token type generated from the [Diglogflow service\nagent](https://cloud.google.com/iam/docs/service-agents#dialogflow-service-agent).\nThe generated token is sent in the Authorization header. Possible values: [\"NONE\", \"ID_TOKEN\", \"ACCESS_TOKEN\"]"]
    pub fn set_service_agent_auth(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_agent_auth = Some(v.into());
        self
    }
    #[doc = "Set the field `webhook_type`.\nType of the webhook. Possible values: [\"STANDARD\", \"FLEXIBLE\"]"]
    pub fn set_webhook_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.webhook_type = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth_config`.\n"]
    pub fn set_oauth_config(
        mut self,
        v: impl Into<
            BlockAssignable<DialogflowCxWebhookServiceDirectoryElGenericWebServiceElOauthConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oauth_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `secret_versions_for_request_headers`.\n"]
    pub fn set_secret_versions_for_request_headers(
        mut self,
        v : impl Into < BlockAssignable < DialogflowCxWebhookServiceDirectoryElGenericWebServiceElSecretVersionsForRequestHeadersEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.secret_versions_for_request_headers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_versions_for_request_headers = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `service_account_auth_config`.\n"]
    pub fn set_service_account_auth_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxWebhookServiceDirectoryElGenericWebServiceElServiceAccountAuthConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_account_auth_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_account_auth_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxWebhookServiceDirectoryElGenericWebServiceEl {
    type O = BlockAssignable<DialogflowCxWebhookServiceDirectoryElGenericWebServiceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxWebhookServiceDirectoryElGenericWebServiceEl {
    #[doc = "The webhook URI for receiving POST requests. It must use https protocol."]
    pub uri: PrimField<String>,
}
impl BuildDialogflowCxWebhookServiceDirectoryElGenericWebServiceEl {
    pub fn build(self) -> DialogflowCxWebhookServiceDirectoryElGenericWebServiceEl {
        DialogflowCxWebhookServiceDirectoryElGenericWebServiceEl {
            allowed_ca_certs: core::default::Default::default(),
            http_method: core::default::Default::default(),
            parameter_mapping: core::default::Default::default(),
            request_body: core::default::Default::default(),
            request_headers: core::default::Default::default(),
            secret_version_for_username_password: core::default::Default::default(),
            service_agent_auth: core::default::Default::default(),
            uri: self.uri,
            webhook_type: core::default::Default::default(),
            oauth_config: core::default::Default::default(),
            secret_versions_for_request_headers: core::default::Default::default(),
            service_account_auth_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxWebhookServiceDirectoryElGenericWebServiceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxWebhookServiceDirectoryElGenericWebServiceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxWebhookServiceDirectoryElGenericWebServiceElRef {
        DialogflowCxWebhookServiceDirectoryElGenericWebServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxWebhookServiceDirectoryElGenericWebServiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_ca_certs` after provisioning.\nSpecifies a list of allowed custom CA certificates (in DER format) for\nHTTPS verification. This overrides the default SSL trust store. If this\nis empty or unspecified, Dialogflow will use Google's default trust store\nto verify certificates.\nN.B. Make sure the HTTPS server certificates are signed with \"subject alt\nname\". For instance a certificate can be self-signed using the following\ncommand,\nopenssl x509 -req -days 200 -in example.com.csr \\\n-signkey example.com.key \\\n-out example.com.crt \\\n-extfile <(printf \"\\nsubjectAltName='DNS:www.example.com'\")"]
    pub fn allowed_ca_certs(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_ca_certs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `http_method` after provisioning.\nHTTP method for the flexible webhook calls. Standard webhook always uses\nPOST. Possible values: [\"POST\", \"GET\", \"HEAD\", \"PUT\", \"DELETE\", \"PATCH\", \"OPTIONS\"]"]
    pub fn http_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_method", self.base))
    }
    #[doc = "Get a reference to the value of field `parameter_mapping` after provisioning.\nMaps the values extracted from specific fields of the flexible webhook\nresponse into session parameters.\n- Key: session parameter name\n- Value: field path in the webhook response"]
    pub fn parameter_mapping(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.parameter_mapping", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_body` after provisioning.\nDefines a custom JSON object as request body to send to flexible webhook."]
    pub fn request_body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_body", self.base))
    }
    #[doc = "Get a reference to the value of field `request_headers` after provisioning.\nThe HTTP request headers to send together with webhook requests."]
    pub fn request_headers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.request_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secret_version_for_username_password` after provisioning.\nThe SecretManager secret version resource storing the username:password\npair for HTTP Basic authentication.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'"]
    pub fn secret_version_for_username_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_version_for_username_password", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_agent_auth` after provisioning.\nIndicate the auth token type generated from the [Diglogflow service\nagent](https://cloud.google.com/iam/docs/service-agents#dialogflow-service-agent).\nThe generated token is sent in the Authorization header. Possible values: [\"NONE\", \"ID_TOKEN\", \"ACCESS_TOKEN\"]"]
    pub fn service_agent_auth(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_agent_auth", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe webhook URI for receiving POST requests. It must use https protocol."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
    #[doc = "Get a reference to the value of field `webhook_type` after provisioning.\nType of the webhook. Possible values: [\"STANDARD\", \"FLEXIBLE\"]"]
    pub fn webhook_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.webhook_type", self.base))
    }
    #[doc = "Get a reference to the value of field `oauth_config` after provisioning.\n"]
    pub fn oauth_config(
        &self,
    ) -> ListRef<DialogflowCxWebhookServiceDirectoryElGenericWebServiceElOauthConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oauth_config", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account_auth_config` after provisioning.\n"]
    pub fn service_account_auth_config(
        &self,
    ) -> ListRef<
        DialogflowCxWebhookServiceDirectoryElGenericWebServiceElServiceAccountAuthConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_account_auth_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxWebhookServiceDirectoryElDynamic {
    generic_web_service:
        Option<DynamicBlock<DialogflowCxWebhookServiceDirectoryElGenericWebServiceEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxWebhookServiceDirectoryEl {
    service: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generic_web_service: Option<Vec<DialogflowCxWebhookServiceDirectoryElGenericWebServiceEl>>,
    dynamic: DialogflowCxWebhookServiceDirectoryElDynamic,
}
impl DialogflowCxWebhookServiceDirectoryEl {
    #[doc = "Set the field `generic_web_service`.\n"]
    pub fn set_generic_web_service(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxWebhookServiceDirectoryElGenericWebServiceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.generic_web_service = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.generic_web_service = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxWebhookServiceDirectoryEl {
    type O = BlockAssignable<DialogflowCxWebhookServiceDirectoryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxWebhookServiceDirectoryEl {
    #[doc = "The name of Service Directory service."]
    pub service: PrimField<String>,
}
impl BuildDialogflowCxWebhookServiceDirectoryEl {
    pub fn build(self) -> DialogflowCxWebhookServiceDirectoryEl {
        DialogflowCxWebhookServiceDirectoryEl {
            service: self.service,
            generic_web_service: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxWebhookServiceDirectoryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxWebhookServiceDirectoryElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxWebhookServiceDirectoryElRef {
        DialogflowCxWebhookServiceDirectoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxWebhookServiceDirectoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nThe name of Service Directory service."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
    #[doc = "Get a reference to the value of field `generic_web_service` after provisioning.\n"]
    pub fn generic_web_service(
        &self,
    ) -> ListRef<DialogflowCxWebhookServiceDirectoryElGenericWebServiceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.generic_web_service", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxWebhookTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DialogflowCxWebhookTimeoutsEl {
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
impl ToListMappable for DialogflowCxWebhookTimeoutsEl {
    type O = BlockAssignable<DialogflowCxWebhookTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxWebhookTimeoutsEl {}
impl BuildDialogflowCxWebhookTimeoutsEl {
    pub fn build(self) -> DialogflowCxWebhookTimeoutsEl {
        DialogflowCxWebhookTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxWebhookTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxWebhookTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxWebhookTimeoutsElRef {
        DialogflowCxWebhookTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxWebhookTimeoutsElRef {
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
struct DialogflowCxWebhookDynamic {
    generic_web_service: Option<DynamicBlock<DialogflowCxWebhookGenericWebServiceEl>>,
    service_directory: Option<DynamicBlock<DialogflowCxWebhookServiceDirectoryEl>>,
}
