use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct IntegrationsAuthConfigData {
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
    expiry_notification_duration: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    override_valid_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_certificate: Option<Vec<IntegrationsAuthConfigClientCertificateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    decrypted_credential: Option<Vec<IntegrationsAuthConfigDecryptedCredentialEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<IntegrationsAuthConfigTimeoutsEl>,
    dynamic: IntegrationsAuthConfigDynamic,
}
struct IntegrationsAuthConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IntegrationsAuthConfigData>,
}
#[derive(Clone)]
pub struct IntegrationsAuthConfig(Rc<IntegrationsAuthConfig_>);
impl IntegrationsAuthConfig {
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
    #[doc = "Set the field `description`.\nA description of the auth config."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `expiry_notification_duration`.\nUser can define the time to receive notification after which the auth config becomes invalid. Support up to 30 days. Support granularity in hours.\n\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn set_expiry_notification_duration(
        self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.0.data.borrow_mut().expiry_notification_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `override_valid_time`.\nUser provided expiry time to override. For the example of Salesforce, username/password credentials can be valid for 6 months depending on the instance settings.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn set_override_valid_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().override_valid_time = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `visibility`.\nThe visibility of the auth config. Possible values: [\"PRIVATE\", \"CLIENT_VISIBLE\"]"]
    pub fn set_visibility(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().visibility = Some(v.into());
        self
    }
    #[doc = "Set the field `client_certificate`.\n"]
    pub fn set_client_certificate(
        self,
        v: impl Into<BlockAssignable<IntegrationsAuthConfigClientCertificateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client_certificate = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client_certificate = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `decrypted_credential`.\n"]
    pub fn set_decrypted_credential(
        self,
        v: impl Into<BlockAssignable<IntegrationsAuthConfigDecryptedCredentialEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().decrypted_credential = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.decrypted_credential = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<IntegrationsAuthConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `certificate_id` after provisioning.\nCertificate id for client certificate."]
    pub fn certificate_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the auth config is created.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creator_email` after provisioning.\nThe creator's email address. Generated based on the End User Credentials/LOAS role of the user making the call."]
    pub fn creator_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creator_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `credential_type` after provisioning.\nCredential type of the encrypted credential."]
    pub fn credential_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credential_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the auth config."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe name of the auth config."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encrypted_credential` after provisioning.\nAuth credential encrypted by Cloud KMS. Can be decrypted as Credential with proper KMS key.\n\nA base64-encoded string."]
    pub fn encrypted_credential(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encrypted_credential", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expiry_notification_duration` after provisioning.\nUser can define the time to receive notification after which the auth config becomes invalid. Support up to 30 days. Support granularity in hours.\n\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn expiry_notification_duration(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.expiry_notification_duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_modifier_email` after provisioning.\nThe last modifier's email address. Generated based on the End User Credentials/LOAS role of the user making the call."]
    pub fn last_modifier_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modifier_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation in which client needs to be provisioned."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource name of the auth config."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `override_valid_time` after provisioning.\nUser provided expiry time to override. For the example of Salesforce, username/password credentials can be valid for 6 months depending on the instance settings.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn override_valid_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.override_valid_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reason` after provisioning.\nThe reason / details of the current status."]
    pub fn reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reason", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe status of the auth config."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the auth config is modified.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `valid_time` after provisioning.\nThe time until the auth config is valid. Empty or max value is considered the auth config won't expire.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn valid_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.valid_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `visibility` after provisioning.\nThe visibility of the auth config. Possible values: [\"PRIVATE\", \"CLIENT_VISIBLE\"]"]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.visibility", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_certificate` after provisioning.\n"]
    pub fn client_certificate(&self) -> ListRef<IntegrationsAuthConfigClientCertificateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_certificate", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `decrypted_credential` after provisioning.\n"]
    pub fn decrypted_credential(&self) -> ListRef<IntegrationsAuthConfigDecryptedCredentialElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.decrypted_credential", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IntegrationsAuthConfigTimeoutsElRef {
        IntegrationsAuthConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for IntegrationsAuthConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for IntegrationsAuthConfig {}
impl ToListMappable for IntegrationsAuthConfig {
    type O = ListRef<IntegrationsAuthConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for IntegrationsAuthConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_integrations_auth_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildIntegrationsAuthConfig {
    pub tf_id: String,
    #[doc = "The name of the auth config."]
    pub display_name: PrimField<String>,
    #[doc = "Location in which client needs to be provisioned."]
    pub location: PrimField<String>,
}
impl BuildIntegrationsAuthConfig {
    pub fn build(self, stack: &mut Stack) -> IntegrationsAuthConfig {
        let out = IntegrationsAuthConfig(Rc::new(IntegrationsAuthConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IntegrationsAuthConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                expiry_notification_duration: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                override_valid_time: core::default::Default::default(),
                project: core::default::Default::default(),
                visibility: core::default::Default::default(),
                client_certificate: core::default::Default::default(),
                decrypted_credential: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct IntegrationsAuthConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for IntegrationsAuthConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl IntegrationsAuthConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificate_id` after provisioning.\nCertificate id for client certificate."]
    pub fn certificate_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the auth config is created.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creator_email` after provisioning.\nThe creator's email address. Generated based on the End User Credentials/LOAS role of the user making the call."]
    pub fn creator_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creator_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `credential_type` after provisioning.\nCredential type of the encrypted credential."]
    pub fn credential_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credential_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the auth config."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe name of the auth config."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encrypted_credential` after provisioning.\nAuth credential encrypted by Cloud KMS. Can be decrypted as Credential with proper KMS key.\n\nA base64-encoded string."]
    pub fn encrypted_credential(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encrypted_credential", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expiry_notification_duration` after provisioning.\nUser can define the time to receive notification after which the auth config becomes invalid. Support up to 30 days. Support granularity in hours.\n\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn expiry_notification_duration(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.expiry_notification_duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_modifier_email` after provisioning.\nThe last modifier's email address. Generated based on the End User Credentials/LOAS role of the user making the call."]
    pub fn last_modifier_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modifier_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation in which client needs to be provisioned."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource name of the auth config."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `override_valid_time` after provisioning.\nUser provided expiry time to override. For the example of Salesforce, username/password credentials can be valid for 6 months depending on the instance settings.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn override_valid_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.override_valid_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reason` after provisioning.\nThe reason / details of the current status."]
    pub fn reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reason", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe status of the auth config."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the auth config is modified.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `valid_time` after provisioning.\nThe time until the auth config is valid. Empty or max value is considered the auth config won't expire.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn valid_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.valid_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `visibility` after provisioning.\nThe visibility of the auth config. Possible values: [\"PRIVATE\", \"CLIENT_VISIBLE\"]"]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.visibility", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_certificate` after provisioning.\n"]
    pub fn client_certificate(&self) -> ListRef<IntegrationsAuthConfigClientCertificateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_certificate", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `decrypted_credential` after provisioning.\n"]
    pub fn decrypted_credential(&self) -> ListRef<IntegrationsAuthConfigDecryptedCredentialElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.decrypted_credential", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IntegrationsAuthConfigTimeoutsElRef {
        IntegrationsAuthConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct IntegrationsAuthConfigClientCertificateEl {
    encrypted_private_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    passphrase: Option<PrimField<String>>,
    ssl_certificate: PrimField<String>,
}
impl IntegrationsAuthConfigClientCertificateEl {
    #[doc = "Set the field `passphrase`.\n'passphrase' should be left unset if private key is not encrypted.\nNote that 'passphrase' is not the password for web server, but an extra layer of security to protected private key."]
    pub fn set_passphrase(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.passphrase = Some(v.into());
        self
    }
}
impl ToListMappable for IntegrationsAuthConfigClientCertificateEl {
    type O = BlockAssignable<IntegrationsAuthConfigClientCertificateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIntegrationsAuthConfigClientCertificateEl {
    #[doc = "The ssl certificate encoded in PEM format. This string must include the begin header and end footer lines."]
    pub encrypted_private_key: PrimField<String>,
    #[doc = "The ssl certificate encoded in PEM format. This string must include the begin header and end footer lines."]
    pub ssl_certificate: PrimField<String>,
}
impl BuildIntegrationsAuthConfigClientCertificateEl {
    pub fn build(self) -> IntegrationsAuthConfigClientCertificateEl {
        IntegrationsAuthConfigClientCertificateEl {
            encrypted_private_key: self.encrypted_private_key,
            passphrase: core::default::Default::default(),
            ssl_certificate: self.ssl_certificate,
        }
    }
}
pub struct IntegrationsAuthConfigClientCertificateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IntegrationsAuthConfigClientCertificateElRef {
    fn new(shared: StackShared, base: String) -> IntegrationsAuthConfigClientCertificateElRef {
        IntegrationsAuthConfigClientCertificateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IntegrationsAuthConfigClientCertificateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `encrypted_private_key` after provisioning.\nThe ssl certificate encoded in PEM format. This string must include the begin header and end footer lines."]
    pub fn encrypted_private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encrypted_private_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `passphrase` after provisioning.\n'passphrase' should be left unset if private key is not encrypted.\nNote that 'passphrase' is not the password for web server, but an extra layer of security to protected private key."]
    pub fn passphrase(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.passphrase", self.base))
    }
    #[doc = "Get a reference to the value of field `ssl_certificate` after provisioning.\nThe ssl certificate encoded in PEM format. This string must include the begin header and end footer lines."]
    pub fn ssl_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_certificate", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IntegrationsAuthConfigDecryptedCredentialElAuthTokenEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl IntegrationsAuthConfigDecryptedCredentialElAuthTokenEl {
    #[doc = "Set the field `token`.\nThe token for the auth type."]
    pub fn set_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nAuthentication type, e.g. \"Basic\", \"Bearer\", etc."]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for IntegrationsAuthConfigDecryptedCredentialElAuthTokenEl {
    type O = BlockAssignable<IntegrationsAuthConfigDecryptedCredentialElAuthTokenEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIntegrationsAuthConfigDecryptedCredentialElAuthTokenEl {}
impl BuildIntegrationsAuthConfigDecryptedCredentialElAuthTokenEl {
    pub fn build(self) -> IntegrationsAuthConfigDecryptedCredentialElAuthTokenEl {
        IntegrationsAuthConfigDecryptedCredentialElAuthTokenEl {
            token: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct IntegrationsAuthConfigDecryptedCredentialElAuthTokenElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IntegrationsAuthConfigDecryptedCredentialElAuthTokenElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IntegrationsAuthConfigDecryptedCredentialElAuthTokenElRef {
        IntegrationsAuthConfigDecryptedCredentialElAuthTokenElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IntegrationsAuthConfigDecryptedCredentialElAuthTokenElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `token` after provisioning.\nThe token for the auth type."]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nAuthentication type, e.g. \"Basic\", \"Bearer\", etc."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct IntegrationsAuthConfigDecryptedCredentialElJwtEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    jwt_header: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jwt_payload: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<PrimField<String>>,
}
impl IntegrationsAuthConfigDecryptedCredentialElJwtEl {
    #[doc = "Set the field `jwt_header`.\nIdentifies which algorithm is used to generate the signature."]
    pub fn set_jwt_header(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.jwt_header = Some(v.into());
        self
    }
    #[doc = "Set the field `jwt_payload`.\nContains a set of claims. The JWT specification defines seven Registered Claim Names which are the standard fields commonly included in tokens. Custom claims are usually also included, depending on the purpose of the token."]
    pub fn set_jwt_payload(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.jwt_payload = Some(v.into());
        self
    }
    #[doc = "Set the field `secret`.\nUser's pre-shared secret to sign the token."]
    pub fn set_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret = Some(v.into());
        self
    }
}
impl ToListMappable for IntegrationsAuthConfigDecryptedCredentialElJwtEl {
    type O = BlockAssignable<IntegrationsAuthConfigDecryptedCredentialElJwtEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIntegrationsAuthConfigDecryptedCredentialElJwtEl {}
impl BuildIntegrationsAuthConfigDecryptedCredentialElJwtEl {
    pub fn build(self) -> IntegrationsAuthConfigDecryptedCredentialElJwtEl {
        IntegrationsAuthConfigDecryptedCredentialElJwtEl {
            jwt_header: core::default::Default::default(),
            jwt_payload: core::default::Default::default(),
            secret: core::default::Default::default(),
        }
    }
}
pub struct IntegrationsAuthConfigDecryptedCredentialElJwtElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IntegrationsAuthConfigDecryptedCredentialElJwtElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IntegrationsAuthConfigDecryptedCredentialElJwtElRef {
        IntegrationsAuthConfigDecryptedCredentialElJwtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IntegrationsAuthConfigDecryptedCredentialElJwtElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `jwt` after provisioning.\nThe token calculated by the header, payload and signature."]
    pub fn jwt(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.jwt", self.base))
    }
    #[doc = "Get a reference to the value of field `jwt_header` after provisioning.\nIdentifies which algorithm is used to generate the signature."]
    pub fn jwt_header(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.jwt_header", self.base))
    }
    #[doc = "Get a reference to the value of field `jwt_payload` after provisioning.\nContains a set of claims. The JWT specification defines seven Registered Claim Names which are the standard fields commonly included in tokens. Custom claims are usually also included, depending on the purpose of the token."]
    pub fn jwt_payload(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.jwt_payload", self.base))
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\nUser's pre-shared secret to sign the token."]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }
}
#[derive(Serialize)]
pub struct IntegrationsAuthConfigDecryptedCredentialElOauth2AuthorizationCodeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_endpoint: Option<PrimField<String>>,
}
impl IntegrationsAuthConfigDecryptedCredentialElOauth2AuthorizationCodeEl {
    #[doc = "Set the field `auth_endpoint`.\nThe auth url endpoint to send the auth code request to."]
    pub fn set_auth_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `client_id`.\nThe client's id."]
    pub fn set_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id = Some(v.into());
        self
    }
    #[doc = "Set the field `client_secret`.\nThe client's secret."]
    pub fn set_client_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_secret = Some(v.into());
        self
    }
    #[doc = "Set the field `scope`.\nA space-delimited list of requested scope permissions."]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }
    #[doc = "Set the field `token_endpoint`.\nThe token url endpoint to send the token request to."]
    pub fn set_token_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token_endpoint = Some(v.into());
        self
    }
}
impl ToListMappable for IntegrationsAuthConfigDecryptedCredentialElOauth2AuthorizationCodeEl {
    type O = BlockAssignable<IntegrationsAuthConfigDecryptedCredentialElOauth2AuthorizationCodeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIntegrationsAuthConfigDecryptedCredentialElOauth2AuthorizationCodeEl {}
impl BuildIntegrationsAuthConfigDecryptedCredentialElOauth2AuthorizationCodeEl {
    pub fn build(self) -> IntegrationsAuthConfigDecryptedCredentialElOauth2AuthorizationCodeEl {
        IntegrationsAuthConfigDecryptedCredentialElOauth2AuthorizationCodeEl {
            auth_endpoint: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            scope: core::default::Default::default(),
            token_endpoint: core::default::Default::default(),
        }
    }
}
pub struct IntegrationsAuthConfigDecryptedCredentialElOauth2AuthorizationCodeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IntegrationsAuthConfigDecryptedCredentialElOauth2AuthorizationCodeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IntegrationsAuthConfigDecryptedCredentialElOauth2AuthorizationCodeElRef {
        IntegrationsAuthConfigDecryptedCredentialElOauth2AuthorizationCodeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IntegrationsAuthConfigDecryptedCredentialElOauth2AuthorizationCodeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auth_endpoint` after provisioning.\nThe auth url endpoint to send the auth code request to."]
    pub fn auth_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auth_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nThe client's id."]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\nThe client's secret."]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\nA space-delimited list of requested scope permissions."]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\nThe token url endpoint to send the token request to."]
    pub fn token_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.token_endpoint", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyElLiteralValueEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    string_value: Option<PrimField<String>>,
}
impl IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyElLiteralValueEl { # [doc = "Set the field `string_value`.\nString."] pub fn set_string_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . string_value = Some (v . into ()) ; self } }
impl ToListMappable for IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyElLiteralValueEl { type O = BlockAssignable < IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyElLiteralValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildIntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyElLiteralValueEl
{}
impl BuildIntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyElLiteralValueEl { pub fn build (self) -> IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyElLiteralValueEl { IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyElLiteralValueEl { string_value : core :: default :: Default :: default () , } } }
pub struct IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyElLiteralValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyElLiteralValueElRef { fn new (shared : StackShared , base : String) -> IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyElLiteralValueElRef { IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyElLiteralValueElRef { shared : shared , base : base . to_string () , } } }
impl IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyElLiteralValueElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `string_value` after provisioning.\nString."] pub fn string_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.string_value" , self . base)) } }
#[derive(Serialize, Default)]
struct IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyElDynamic { literal_value : Option < DynamicBlock < IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyElLiteralValueEl >> , }
#[derive(Serialize)]
pub struct IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyEl { # [serde (skip_serializing_if = "Option::is_none")] literal_value : Option < Vec < IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyElLiteralValueEl > > , dynamic : IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyElDynamic , }
impl
    IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyEl
{
    #[doc = "Set the field `literal_value`.\n"]
    pub fn set_literal_value(
        mut self,
        v : impl Into < BlockAssignable < IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyElLiteralValueEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.literal_value = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.literal_value = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyEl { type O = BlockAssignable < IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildIntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyEl
{}
impl BuildIntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyEl { pub fn build (self) -> IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyEl { IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyEl { literal_value : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyElRef { fn new (shared : StackShared , base : String) -> IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyElRef { IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyElRef { shared : shared , base : base . to_string () , } } }
impl IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `literal_value` after provisioning.\n"] pub fn literal_value (& self) -> ListRef < IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyElLiteralValueElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.literal_value" , self . base)) } }
#[derive(Serialize)]
pub struct IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueElLiteralValueEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    string_value: Option<PrimField<String>>,
}
impl IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueElLiteralValueEl { # [doc = "Set the field `string_value`.\nString."] pub fn set_string_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . string_value = Some (v . into ()) ; self } }
impl ToListMappable for IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueElLiteralValueEl { type O = BlockAssignable < IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueElLiteralValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildIntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueElLiteralValueEl
{}
impl BuildIntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueElLiteralValueEl { pub fn build (self) -> IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueElLiteralValueEl { IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueElLiteralValueEl { string_value : core :: default :: Default :: default () , } } }
pub struct IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueElLiteralValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueElLiteralValueElRef { fn new (shared : StackShared , base : String) -> IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueElLiteralValueElRef { IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueElLiteralValueElRef { shared : shared , base : base . to_string () , } } }
impl IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueElLiteralValueElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `string_value` after provisioning.\nString."] pub fn string_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.string_value" , self . base)) } }
#[derive(Serialize, Default)]
struct IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueElDynamic { literal_value : Option < DynamicBlock < IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueElLiteralValueEl >> , }
#[derive(Serialize)]
pub struct IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueEl { # [serde (skip_serializing_if = "Option::is_none")] literal_value : Option < Vec < IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueElLiteralValueEl > > , dynamic : IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueElDynamic , }
impl IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueEl { # [doc = "Set the field `literal_value`.\n"] pub fn set_literal_value (mut self , v : impl Into < BlockAssignable < IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueElLiteralValueEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . literal_value = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . literal_value = Some (d) ; } } self } }
impl ToListMappable for IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueEl { type O = BlockAssignable < IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildIntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueEl
{}
impl BuildIntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueEl { pub fn build (self) -> IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueEl { IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueEl { literal_value : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueElRef { fn new (shared : StackShared , base : String) -> IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueElRef { IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueElRef { shared : shared , base : base . to_string () , } } }
impl IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `literal_value` after provisioning.\n"] pub fn literal_value (& self) -> ListRef < IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueElLiteralValueElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.literal_value" , self . base)) } }
#[derive(Serialize, Default)]
struct IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElDynamic { key : Option < DynamicBlock < IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyEl >> , value : Option < DynamicBlock < IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueEl >> , }
#[derive(Serialize)]
pub struct IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesEl { # [serde (skip_serializing_if = "Option::is_none")] key : Option < Vec < IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyEl > > , # [serde (skip_serializing_if = "Option::is_none")] value : Option < Vec < IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueEl > > , dynamic : IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElDynamic , }
impl IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(
        mut self,
        v : impl Into < BlockAssignable < IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.key = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(
        mut self,
        v : impl Into < BlockAssignable < IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.value = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.value = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesEl
{
    type O = BlockAssignable<
        IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesEl
{}
impl
    BuildIntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesEl
{
    pub fn build(
        self,
    ) -> IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesEl
    {
        IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesEl {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElRef { fn new (shared : StackShared , base : String) -> IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElRef { IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElRef { shared : shared , base : base . to_string () , } } }
impl IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]    pub fn key (& self) -> ListRef < IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElKeyElRef >{
        ListRef::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]    pub fn value (& self) -> ListRef < IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElValueElRef >{
        ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElDynamic { entries : Option < DynamicBlock < IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesEl >> , }
#[derive(Serialize)]
pub struct IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsEl { # [serde (skip_serializing_if = "Option::is_none")] entries : Option < Vec < IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesEl > > , dynamic : IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElDynamic , }
impl IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsEl {
    #[doc = "Set the field `entries`.\n"]
    pub fn set_entries(
        mut self,
        v : impl Into < BlockAssignable < IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.entries = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.entries = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsEl
{
    type O = BlockAssignable<
        IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsEl {
}
impl BuildIntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsEl {
    pub fn build(
        self,
    ) -> IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsEl {
        IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsEl {
            entries: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElRef {
        IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `entries` after provisioning.\n"]    pub fn entries (& self) -> ListRef < IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElEntriesElRef >{
        ListRef::new(self.shared().clone(), format!("{}.entries", self.base))
    }
}
#[derive(Serialize, Default)]
struct IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElDynamic {
    token_params: Option<
        DynamicBlock<
            IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_params: Option<
        Vec<IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsEl>,
    >,
    dynamic: IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElDynamic,
}
impl IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsEl {
    #[doc = "Set the field `client_id`.\nThe client's ID."]
    pub fn set_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id = Some(v.into());
        self
    }
    #[doc = "Set the field `client_secret`.\nThe client's secret."]
    pub fn set_client_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_secret = Some(v.into());
        self
    }
    #[doc = "Set the field `request_type`.\nRepresent how to pass parameters to fetch access token Possible values: [\"REQUEST_TYPE_UNSPECIFIED\", \"REQUEST_BODY\", \"QUERY_PARAMETERS\", \"ENCODED_HEADER\"]"]
    pub fn set_request_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.request_type = Some(v.into());
        self
    }
    #[doc = "Set the field `scope`.\nA space-delimited list of requested scope permissions."]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }
    #[doc = "Set the field `token_endpoint`.\nThe token endpoint is used by the client to obtain an access token by presenting its authorization grant or refresh token."]
    pub fn set_token_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `token_params`.\n"]
    pub fn set_token_params(
        mut self,
        v: impl Into<
            BlockAssignable<
                IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.token_params = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.token_params = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsEl {
    type O = BlockAssignable<IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsEl {}
impl BuildIntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsEl {
    pub fn build(self) -> IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsEl {
        IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsEl {
            client_id: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            request_type: core::default::Default::default(),
            scope: core::default::Default::default(),
            token_endpoint: core::default::Default::default(),
            token_params: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElRef {
        IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nThe client's ID."]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\nThe client's secret."]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_type` after provisioning.\nRepresent how to pass parameters to fetch access token Possible values: [\"REQUEST_TYPE_UNSPECIFIED\", \"REQUEST_BODY\", \"QUERY_PARAMETERS\", \"ENCODED_HEADER\"]"]
    pub fn request_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_type", self.base))
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\nA space-delimited list of requested scope permissions."]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\nThe token endpoint is used by the client to obtain an access token by presenting its authorization grant or refresh token."]
    pub fn token_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.token_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `token_params` after provisioning.\n"]
    pub fn token_params(
        &self,
    ) -> ListRef<IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElTokenParamsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.token_params", self.base))
    }
}
#[derive(Serialize)]
pub struct IntegrationsAuthConfigDecryptedCredentialElOidcTokenEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audience: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_email: Option<PrimField<String>>,
}
impl IntegrationsAuthConfigDecryptedCredentialElOidcTokenEl {
    #[doc = "Set the field `audience`.\nAudience to be used when generating OIDC token. The audience claim identifies the recipients that the JWT is intended for."]
    pub fn set_audience(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.audience = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account_email`.\nThe service account email to be used as the identity for the token."]
    pub fn set_service_account_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account_email = Some(v.into());
        self
    }
}
impl ToListMappable for IntegrationsAuthConfigDecryptedCredentialElOidcTokenEl {
    type O = BlockAssignable<IntegrationsAuthConfigDecryptedCredentialElOidcTokenEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIntegrationsAuthConfigDecryptedCredentialElOidcTokenEl {}
impl BuildIntegrationsAuthConfigDecryptedCredentialElOidcTokenEl {
    pub fn build(self) -> IntegrationsAuthConfigDecryptedCredentialElOidcTokenEl {
        IntegrationsAuthConfigDecryptedCredentialElOidcTokenEl {
            audience: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
        }
    }
}
pub struct IntegrationsAuthConfigDecryptedCredentialElOidcTokenElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IntegrationsAuthConfigDecryptedCredentialElOidcTokenElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IntegrationsAuthConfigDecryptedCredentialElOidcTokenElRef {
        IntegrationsAuthConfigDecryptedCredentialElOidcTokenElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IntegrationsAuthConfigDecryptedCredentialElOidcTokenElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `audience` after provisioning.\nAudience to be used when generating OIDC token. The audience claim identifies the recipients that the JWT is intended for."]
    pub fn audience(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.audience", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\nThe service account email to be used as the identity for the token."]
    pub fn service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_email", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `token` after provisioning.\nID token obtained for the service account."]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
    #[doc = "Get a reference to the value of field `token_expire_time` after provisioning.\nThe approximate time until the token retrieved is valid.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn token_expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.token_expire_time", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IntegrationsAuthConfigDecryptedCredentialElServiceAccountCredentialsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
}
impl IntegrationsAuthConfigDecryptedCredentialElServiceAccountCredentialsEl {
    #[doc = "Set the field `scope`.\nA space-delimited list of requested scope permissions."]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\nName of the service account that has the permission to make the request."]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
}
impl ToListMappable for IntegrationsAuthConfigDecryptedCredentialElServiceAccountCredentialsEl {
    type O =
        BlockAssignable<IntegrationsAuthConfigDecryptedCredentialElServiceAccountCredentialsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIntegrationsAuthConfigDecryptedCredentialElServiceAccountCredentialsEl {}
impl BuildIntegrationsAuthConfigDecryptedCredentialElServiceAccountCredentialsEl {
    pub fn build(self) -> IntegrationsAuthConfigDecryptedCredentialElServiceAccountCredentialsEl {
        IntegrationsAuthConfigDecryptedCredentialElServiceAccountCredentialsEl {
            scope: core::default::Default::default(),
            service_account: core::default::Default::default(),
        }
    }
}
pub struct IntegrationsAuthConfigDecryptedCredentialElServiceAccountCredentialsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IntegrationsAuthConfigDecryptedCredentialElServiceAccountCredentialsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IntegrationsAuthConfigDecryptedCredentialElServiceAccountCredentialsElRef {
        IntegrationsAuthConfigDecryptedCredentialElServiceAccountCredentialsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IntegrationsAuthConfigDecryptedCredentialElServiceAccountCredentialsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\nA space-delimited list of requested scope permissions."]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nName of the service account that has the permission to make the request."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IntegrationsAuthConfigDecryptedCredentialElUsernameAndPasswordEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
}
impl IntegrationsAuthConfigDecryptedCredentialElUsernameAndPasswordEl {
    #[doc = "Set the field `password`.\nPassword to be used."]
    pub fn set_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password = Some(v.into());
        self
    }
    #[doc = "Set the field `username`.\nUsername to be used."]
    pub fn set_username(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.username = Some(v.into());
        self
    }
}
impl ToListMappable for IntegrationsAuthConfigDecryptedCredentialElUsernameAndPasswordEl {
    type O = BlockAssignable<IntegrationsAuthConfigDecryptedCredentialElUsernameAndPasswordEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIntegrationsAuthConfigDecryptedCredentialElUsernameAndPasswordEl {}
impl BuildIntegrationsAuthConfigDecryptedCredentialElUsernameAndPasswordEl {
    pub fn build(self) -> IntegrationsAuthConfigDecryptedCredentialElUsernameAndPasswordEl {
        IntegrationsAuthConfigDecryptedCredentialElUsernameAndPasswordEl {
            password: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}
pub struct IntegrationsAuthConfigDecryptedCredentialElUsernameAndPasswordElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IntegrationsAuthConfigDecryptedCredentialElUsernameAndPasswordElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IntegrationsAuthConfigDecryptedCredentialElUsernameAndPasswordElRef {
        IntegrationsAuthConfigDecryptedCredentialElUsernameAndPasswordElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IntegrationsAuthConfigDecryptedCredentialElUsernameAndPasswordElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nPassword to be used."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername to be used."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize, Default)]
struct IntegrationsAuthConfigDecryptedCredentialElDynamic {
    auth_token: Option<DynamicBlock<IntegrationsAuthConfigDecryptedCredentialElAuthTokenEl>>,
    jwt: Option<DynamicBlock<IntegrationsAuthConfigDecryptedCredentialElJwtEl>>,
    oauth2_authorization_code:
        Option<DynamicBlock<IntegrationsAuthConfigDecryptedCredentialElOauth2AuthorizationCodeEl>>,
    oauth2_client_credentials:
        Option<DynamicBlock<IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsEl>>,
    oidc_token: Option<DynamicBlock<IntegrationsAuthConfigDecryptedCredentialElOidcTokenEl>>,
    service_account_credentials: Option<
        DynamicBlock<IntegrationsAuthConfigDecryptedCredentialElServiceAccountCredentialsEl>,
    >,
    username_and_password:
        Option<DynamicBlock<IntegrationsAuthConfigDecryptedCredentialElUsernameAndPasswordEl>>,
}
#[derive(Serialize)]
pub struct IntegrationsAuthConfigDecryptedCredentialEl {
    credential_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_token: Option<Vec<IntegrationsAuthConfigDecryptedCredentialElAuthTokenEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jwt: Option<Vec<IntegrationsAuthConfigDecryptedCredentialElJwtEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth2_authorization_code:
        Option<Vec<IntegrationsAuthConfigDecryptedCredentialElOauth2AuthorizationCodeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth2_client_credentials:
        Option<Vec<IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oidc_token: Option<Vec<IntegrationsAuthConfigDecryptedCredentialElOidcTokenEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_credentials:
        Option<Vec<IntegrationsAuthConfigDecryptedCredentialElServiceAccountCredentialsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username_and_password:
        Option<Vec<IntegrationsAuthConfigDecryptedCredentialElUsernameAndPasswordEl>>,
    dynamic: IntegrationsAuthConfigDecryptedCredentialElDynamic,
}
impl IntegrationsAuthConfigDecryptedCredentialEl {
    #[doc = "Set the field `auth_token`.\n"]
    pub fn set_auth_token(
        mut self,
        v: impl Into<BlockAssignable<IntegrationsAuthConfigDecryptedCredentialElAuthTokenEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.auth_token = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.auth_token = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `jwt`.\n"]
    pub fn set_jwt(
        mut self,
        v: impl Into<BlockAssignable<IntegrationsAuthConfigDecryptedCredentialElJwtEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.jwt = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.jwt = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `oauth2_authorization_code`.\n"]
    pub fn set_oauth2_authorization_code(
        mut self,
        v: impl Into<
            BlockAssignable<IntegrationsAuthConfigDecryptedCredentialElOauth2AuthorizationCodeEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oauth2_authorization_code = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth2_authorization_code = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `oauth2_client_credentials`.\n"]
    pub fn set_oauth2_client_credentials(
        mut self,
        v: impl Into<
            BlockAssignable<IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oauth2_client_credentials = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth2_client_credentials = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `oidc_token`.\n"]
    pub fn set_oidc_token(
        mut self,
        v: impl Into<BlockAssignable<IntegrationsAuthConfigDecryptedCredentialElOidcTokenEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oidc_token = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oidc_token = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `service_account_credentials`.\n"]
    pub fn set_service_account_credentials(
        mut self,
        v: impl Into<
            BlockAssignable<IntegrationsAuthConfigDecryptedCredentialElServiceAccountCredentialsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_account_credentials = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_account_credentials = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `username_and_password`.\n"]
    pub fn set_username_and_password(
        mut self,
        v: impl Into<BlockAssignable<IntegrationsAuthConfigDecryptedCredentialElUsernameAndPasswordEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.username_and_password = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.username_and_password = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IntegrationsAuthConfigDecryptedCredentialEl {
    type O = BlockAssignable<IntegrationsAuthConfigDecryptedCredentialEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIntegrationsAuthConfigDecryptedCredentialEl {
    #[doc = "Credential type associated with auth configs."]
    pub credential_type: PrimField<String>,
}
impl BuildIntegrationsAuthConfigDecryptedCredentialEl {
    pub fn build(self) -> IntegrationsAuthConfigDecryptedCredentialEl {
        IntegrationsAuthConfigDecryptedCredentialEl {
            credential_type: self.credential_type,
            auth_token: core::default::Default::default(),
            jwt: core::default::Default::default(),
            oauth2_authorization_code: core::default::Default::default(),
            oauth2_client_credentials: core::default::Default::default(),
            oidc_token: core::default::Default::default(),
            service_account_credentials: core::default::Default::default(),
            username_and_password: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IntegrationsAuthConfigDecryptedCredentialElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IntegrationsAuthConfigDecryptedCredentialElRef {
    fn new(shared: StackShared, base: String) -> IntegrationsAuthConfigDecryptedCredentialElRef {
        IntegrationsAuthConfigDecryptedCredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IntegrationsAuthConfigDecryptedCredentialElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `credential_type` after provisioning.\nCredential type associated with auth configs."]
    pub fn credential_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credential_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `auth_token` after provisioning.\n"]
    pub fn auth_token(&self) -> ListRef<IntegrationsAuthConfigDecryptedCredentialElAuthTokenElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auth_token", self.base))
    }
    #[doc = "Get a reference to the value of field `jwt` after provisioning.\n"]
    pub fn jwt(&self) -> ListRef<IntegrationsAuthConfigDecryptedCredentialElJwtElRef> {
        ListRef::new(self.shared().clone(), format!("{}.jwt", self.base))
    }
    #[doc = "Get a reference to the value of field `oauth2_authorization_code` after provisioning.\n"]
    pub fn oauth2_authorization_code(
        &self,
    ) -> ListRef<IntegrationsAuthConfigDecryptedCredentialElOauth2AuthorizationCodeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oauth2_authorization_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth2_client_credentials` after provisioning.\n"]
    pub fn oauth2_client_credentials(
        &self,
    ) -> ListRef<IntegrationsAuthConfigDecryptedCredentialElOauth2ClientCredentialsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oauth2_client_credentials", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oidc_token` after provisioning.\n"]
    pub fn oidc_token(&self) -> ListRef<IntegrationsAuthConfigDecryptedCredentialElOidcTokenElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oidc_token", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account_credentials` after provisioning.\n"]
    pub fn service_account_credentials(
        &self,
    ) -> ListRef<IntegrationsAuthConfigDecryptedCredentialElServiceAccountCredentialsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_account_credentials", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `username_and_password` after provisioning.\n"]
    pub fn username_and_password(
        &self,
    ) -> ListRef<IntegrationsAuthConfigDecryptedCredentialElUsernameAndPasswordElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.username_and_password", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IntegrationsAuthConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl IntegrationsAuthConfigTimeoutsEl {
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
impl ToListMappable for IntegrationsAuthConfigTimeoutsEl {
    type O = BlockAssignable<IntegrationsAuthConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIntegrationsAuthConfigTimeoutsEl {}
impl BuildIntegrationsAuthConfigTimeoutsEl {
    pub fn build(self) -> IntegrationsAuthConfigTimeoutsEl {
        IntegrationsAuthConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct IntegrationsAuthConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IntegrationsAuthConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> IntegrationsAuthConfigTimeoutsElRef {
        IntegrationsAuthConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IntegrationsAuthConfigTimeoutsElRef {
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
struct IntegrationsAuthConfigDynamic {
    client_certificate: Option<DynamicBlock<IntegrationsAuthConfigClientCertificateEl>>,
    decrypted_credential: Option<DynamicBlock<IntegrationsAuthConfigDecryptedCredentialEl>>,
}
