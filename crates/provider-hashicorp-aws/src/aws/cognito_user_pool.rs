use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CognitoUserPoolData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alias_attributes: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_verified_attributes: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_verification_message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_verification_subject: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mfa_configuration: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sms_authentication_message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sms_verification_message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_pool_tier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username_attributes: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_recovery_setting: Option<Vec<CognitoUserPoolAccountRecoverySettingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_create_user_config: Option<Vec<CognitoUserPoolAdminCreateUserConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_configuration: Option<Vec<CognitoUserPoolDeviceConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_configuration: Option<Vec<CognitoUserPoolEmailConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_mfa_configuration: Option<Vec<CognitoUserPoolEmailMfaConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_config: Option<Vec<CognitoUserPoolLambdaConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_policy: Option<Vec<CognitoUserPoolPasswordPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<Vec<CognitoUserPoolSchemaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sign_in_policy: Option<Vec<CognitoUserPoolSignInPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sms_configuration: Option<Vec<CognitoUserPoolSmsConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    software_token_mfa_configuration: Option<Vec<CognitoUserPoolSoftwareTokenMfaConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_attribute_update_settings: Option<Vec<CognitoUserPoolUserAttributeUpdateSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_pool_add_ons: Option<Vec<CognitoUserPoolUserPoolAddOnsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username_configuration: Option<Vec<CognitoUserPoolUsernameConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification_message_template: Option<Vec<CognitoUserPoolVerificationMessageTemplateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_authn_configuration: Option<Vec<CognitoUserPoolWebAuthnConfigurationEl>>,
    dynamic: CognitoUserPoolDynamic,
}

struct CognitoUserPool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CognitoUserPoolData>,
}

#[derive(Clone)]
pub struct CognitoUserPool(Rc<CognitoUserPool_>);

impl CognitoUserPool {
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

    #[doc = "Set the field `alias_attributes`.\n"]
    pub fn set_alias_attributes(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().alias_attributes = Some(v.into());
        self
    }

    #[doc = "Set the field `auto_verified_attributes`.\n"]
    pub fn set_auto_verified_attributes(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().auto_verified_attributes = Some(v.into());
        self
    }

    #[doc = "Set the field `deletion_protection`.\n"]
    pub fn set_deletion_protection(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = Some(v.into());
        self
    }

    #[doc = "Set the field `email_verification_message`.\n"]
    pub fn set_email_verification_message(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().email_verification_message = Some(v.into());
        self
    }

    #[doc = "Set the field `email_verification_subject`.\n"]
    pub fn set_email_verification_subject(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().email_verification_subject = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `mfa_configuration`.\n"]
    pub fn set_mfa_configuration(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().mfa_configuration = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `sms_authentication_message`.\n"]
    pub fn set_sms_authentication_message(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sms_authentication_message = Some(v.into());
        self
    }

    #[doc = "Set the field `sms_verification_message`.\n"]
    pub fn set_sms_verification_message(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sms_verification_message = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `user_pool_tier`.\n"]
    pub fn set_user_pool_tier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().user_pool_tier = Some(v.into());
        self
    }

    #[doc = "Set the field `username_attributes`.\n"]
    pub fn set_username_attributes(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().username_attributes = Some(v.into());
        self
    }

    #[doc = "Set the field `account_recovery_setting`.\n"]
    pub fn set_account_recovery_setting(
        self,
        v: impl Into<BlockAssignable<CognitoUserPoolAccountRecoverySettingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().account_recovery_setting = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.account_recovery_setting = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `admin_create_user_config`.\n"]
    pub fn set_admin_create_user_config(
        self,
        v: impl Into<BlockAssignable<CognitoUserPoolAdminCreateUserConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().admin_create_user_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.admin_create_user_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `device_configuration`.\n"]
    pub fn set_device_configuration(self, v: impl Into<BlockAssignable<CognitoUserPoolDeviceConfigurationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().device_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.device_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `email_configuration`.\n"]
    pub fn set_email_configuration(self, v: impl Into<BlockAssignable<CognitoUserPoolEmailConfigurationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().email_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.email_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `email_mfa_configuration`.\n"]
    pub fn set_email_mfa_configuration(
        self,
        v: impl Into<BlockAssignable<CognitoUserPoolEmailMfaConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().email_mfa_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.email_mfa_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lambda_config`.\n"]
    pub fn set_lambda_config(self, v: impl Into<BlockAssignable<CognitoUserPoolLambdaConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lambda_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lambda_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `password_policy`.\n"]
    pub fn set_password_policy(self, v: impl Into<BlockAssignable<CognitoUserPoolPasswordPolicyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().password_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.password_policy = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(self, v: impl Into<BlockAssignable<CognitoUserPoolSchemaEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schema = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schema = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sign_in_policy`.\n"]
    pub fn set_sign_in_policy(self, v: impl Into<BlockAssignable<CognitoUserPoolSignInPolicyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sign_in_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sign_in_policy = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sms_configuration`.\n"]
    pub fn set_sms_configuration(self, v: impl Into<BlockAssignable<CognitoUserPoolSmsConfigurationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sms_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sms_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `software_token_mfa_configuration`.\n"]
    pub fn set_software_token_mfa_configuration(
        self,
        v: impl Into<BlockAssignable<CognitoUserPoolSoftwareTokenMfaConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().software_token_mfa_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.software_token_mfa_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user_attribute_update_settings`.\n"]
    pub fn set_user_attribute_update_settings(
        self,
        v: impl Into<BlockAssignable<CognitoUserPoolUserAttributeUpdateSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user_attribute_update_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user_attribute_update_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user_pool_add_ons`.\n"]
    pub fn set_user_pool_add_ons(self, v: impl Into<BlockAssignable<CognitoUserPoolUserPoolAddOnsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user_pool_add_ons = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user_pool_add_ons = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `username_configuration`.\n"]
    pub fn set_username_configuration(
        self,
        v: impl Into<BlockAssignable<CognitoUserPoolUsernameConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().username_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.username_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `verification_message_template`.\n"]
    pub fn set_verification_message_template(
        self,
        v: impl Into<BlockAssignable<CognitoUserPoolVerificationMessageTemplateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().verification_message_template = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.verification_message_template = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `web_authn_configuration`.\n"]
    pub fn set_web_authn_configuration(
        self,
        v: impl Into<BlockAssignable<CognitoUserPoolWebAuthnConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().web_authn_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.web_authn_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `alias_attributes` after provisioning.\n"]
    pub fn alias_attributes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.alias_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_verified_attributes` after provisioning.\n"]
    pub fn auto_verified_attributes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.auto_verified_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_date` after provisioning.\n"]
    pub fn creation_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_domain` after provisioning.\n"]
    pub fn custom_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_verification_message` after provisioning.\n"]
    pub fn email_verification_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_verification_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_verification_subject` after provisioning.\n"]
    pub fn email_verification_subject(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_verification_subject", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `estimated_number_of_users` after provisioning.\n"]
    pub fn estimated_number_of_users(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.estimated_number_of_users", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified_date` after provisioning.\n"]
    pub fn last_modified_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_modified_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mfa_configuration` after provisioning.\n"]
    pub fn mfa_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mfa_configuration", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `sms_authentication_message` after provisioning.\n"]
    pub fn sms_authentication_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sms_authentication_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sms_verification_message` after provisioning.\n"]
    pub fn sms_verification_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sms_verification_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_pool_tier` after provisioning.\n"]
    pub fn user_pool_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_pool_tier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `username_attributes` after provisioning.\n"]
    pub fn username_attributes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.username_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `account_recovery_setting` after provisioning.\n"]
    pub fn account_recovery_setting(&self) -> ListRef<CognitoUserPoolAccountRecoverySettingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.account_recovery_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `admin_create_user_config` after provisioning.\n"]
    pub fn admin_create_user_config(&self) -> ListRef<CognitoUserPoolAdminCreateUserConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.admin_create_user_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `device_configuration` after provisioning.\n"]
    pub fn device_configuration(&self) -> ListRef<CognitoUserPoolDeviceConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.device_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_configuration` after provisioning.\n"]
    pub fn email_configuration(&self) -> ListRef<CognitoUserPoolEmailConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.email_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_mfa_configuration` after provisioning.\n"]
    pub fn email_mfa_configuration(&self) -> ListRef<CognitoUserPoolEmailMfaConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.email_mfa_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lambda_config` after provisioning.\n"]
    pub fn lambda_config(&self) -> ListRef<CognitoUserPoolLambdaConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lambda_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password_policy` after provisioning.\n"]
    pub fn password_policy(&self) -> ListRef<CognitoUserPoolPasswordPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.password_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sign_in_policy` after provisioning.\n"]
    pub fn sign_in_policy(&self) -> ListRef<CognitoUserPoolSignInPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sign_in_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sms_configuration` after provisioning.\n"]
    pub fn sms_configuration(&self) -> ListRef<CognitoUserPoolSmsConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sms_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `software_token_mfa_configuration` after provisioning.\n"]
    pub fn software_token_mfa_configuration(&self) -> ListRef<CognitoUserPoolSoftwareTokenMfaConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.software_token_mfa_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_attribute_update_settings` after provisioning.\n"]
    pub fn user_attribute_update_settings(&self) -> ListRef<CognitoUserPoolUserAttributeUpdateSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.user_attribute_update_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_pool_add_ons` after provisioning.\n"]
    pub fn user_pool_add_ons(&self) -> ListRef<CognitoUserPoolUserPoolAddOnsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.user_pool_add_ons", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `username_configuration` after provisioning.\n"]
    pub fn username_configuration(&self) -> ListRef<CognitoUserPoolUsernameConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.username_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `verification_message_template` after provisioning.\n"]
    pub fn verification_message_template(&self) -> ListRef<CognitoUserPoolVerificationMessageTemplateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.verification_message_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `web_authn_configuration` after provisioning.\n"]
    pub fn web_authn_configuration(&self) -> ListRef<CognitoUserPoolWebAuthnConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.web_authn_configuration", self.extract_ref()))
    }
}

impl Referable for CognitoUserPool {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for CognitoUserPool { }

impl ToListMappable for CognitoUserPool {
    type O = ListRef<CognitoUserPoolRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CognitoUserPool_ {
    fn extract_resource_type(&self) -> String {
        "aws_cognito_user_pool".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCognitoUserPool {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildCognitoUserPool {
    pub fn build(self, stack: &mut Stack) -> CognitoUserPool {
        let out = CognitoUserPool(Rc::new(CognitoUserPool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CognitoUserPoolData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                alias_attributes: core::default::Default::default(),
                auto_verified_attributes: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                email_verification_message: core::default::Default::default(),
                email_verification_subject: core::default::Default::default(),
                id: core::default::Default::default(),
                mfa_configuration: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                sms_authentication_message: core::default::Default::default(),
                sms_verification_message: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                user_pool_tier: core::default::Default::default(),
                username_attributes: core::default::Default::default(),
                account_recovery_setting: core::default::Default::default(),
                admin_create_user_config: core::default::Default::default(),
                device_configuration: core::default::Default::default(),
                email_configuration: core::default::Default::default(),
                email_mfa_configuration: core::default::Default::default(),
                lambda_config: core::default::Default::default(),
                password_policy: core::default::Default::default(),
                schema: core::default::Default::default(),
                sign_in_policy: core::default::Default::default(),
                sms_configuration: core::default::Default::default(),
                software_token_mfa_configuration: core::default::Default::default(),
                user_attribute_update_settings: core::default::Default::default(),
                user_pool_add_ons: core::default::Default::default(),
                username_configuration: core::default::Default::default(),
                verification_message_template: core::default::Default::default(),
                web_authn_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CognitoUserPoolRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl CognitoUserPoolRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alias_attributes` after provisioning.\n"]
    pub fn alias_attributes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.alias_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_verified_attributes` after provisioning.\n"]
    pub fn auto_verified_attributes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.auto_verified_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_date` after provisioning.\n"]
    pub fn creation_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_domain` after provisioning.\n"]
    pub fn custom_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_verification_message` after provisioning.\n"]
    pub fn email_verification_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_verification_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_verification_subject` after provisioning.\n"]
    pub fn email_verification_subject(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_verification_subject", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `estimated_number_of_users` after provisioning.\n"]
    pub fn estimated_number_of_users(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.estimated_number_of_users", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified_date` after provisioning.\n"]
    pub fn last_modified_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_modified_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mfa_configuration` after provisioning.\n"]
    pub fn mfa_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mfa_configuration", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `sms_authentication_message` after provisioning.\n"]
    pub fn sms_authentication_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sms_authentication_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sms_verification_message` after provisioning.\n"]
    pub fn sms_verification_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sms_verification_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_pool_tier` after provisioning.\n"]
    pub fn user_pool_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_pool_tier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `username_attributes` after provisioning.\n"]
    pub fn username_attributes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.username_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `account_recovery_setting` after provisioning.\n"]
    pub fn account_recovery_setting(&self) -> ListRef<CognitoUserPoolAccountRecoverySettingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.account_recovery_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `admin_create_user_config` after provisioning.\n"]
    pub fn admin_create_user_config(&self) -> ListRef<CognitoUserPoolAdminCreateUserConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.admin_create_user_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `device_configuration` after provisioning.\n"]
    pub fn device_configuration(&self) -> ListRef<CognitoUserPoolDeviceConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.device_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_configuration` after provisioning.\n"]
    pub fn email_configuration(&self) -> ListRef<CognitoUserPoolEmailConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.email_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_mfa_configuration` after provisioning.\n"]
    pub fn email_mfa_configuration(&self) -> ListRef<CognitoUserPoolEmailMfaConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.email_mfa_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lambda_config` after provisioning.\n"]
    pub fn lambda_config(&self) -> ListRef<CognitoUserPoolLambdaConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lambda_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password_policy` after provisioning.\n"]
    pub fn password_policy(&self) -> ListRef<CognitoUserPoolPasswordPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.password_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sign_in_policy` after provisioning.\n"]
    pub fn sign_in_policy(&self) -> ListRef<CognitoUserPoolSignInPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sign_in_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sms_configuration` after provisioning.\n"]
    pub fn sms_configuration(&self) -> ListRef<CognitoUserPoolSmsConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sms_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `software_token_mfa_configuration` after provisioning.\n"]
    pub fn software_token_mfa_configuration(&self) -> ListRef<CognitoUserPoolSoftwareTokenMfaConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.software_token_mfa_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_attribute_update_settings` after provisioning.\n"]
    pub fn user_attribute_update_settings(&self) -> ListRef<CognitoUserPoolUserAttributeUpdateSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.user_attribute_update_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_pool_add_ons` after provisioning.\n"]
    pub fn user_pool_add_ons(&self) -> ListRef<CognitoUserPoolUserPoolAddOnsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.user_pool_add_ons", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `username_configuration` after provisioning.\n"]
    pub fn username_configuration(&self) -> ListRef<CognitoUserPoolUsernameConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.username_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `verification_message_template` after provisioning.\n"]
    pub fn verification_message_template(&self) -> ListRef<CognitoUserPoolVerificationMessageTemplateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.verification_message_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `web_authn_configuration` after provisioning.\n"]
    pub fn web_authn_configuration(&self) -> ListRef<CognitoUserPoolWebAuthnConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.web_authn_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl {
    name: PrimField<String>,
    priority: PrimField<f64>,
}

impl CognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl { }

impl ToListMappable for CognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl {
    type O = BlockAssignable<CognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub priority: PrimField<f64>,
}

impl BuildCognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl {
    pub fn build(self) -> CognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl {
        CognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl {
            name: self.name,
            priority: self.priority,
        }
    }
}

pub struct CognitoUserPoolAccountRecoverySettingElRecoveryMechanismElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolAccountRecoverySettingElRecoveryMechanismElRef {
    fn new(shared: StackShared, base: String) -> CognitoUserPoolAccountRecoverySettingElRecoveryMechanismElRef {
        CognitoUserPoolAccountRecoverySettingElRecoveryMechanismElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolAccountRecoverySettingElRecoveryMechanismElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
}

#[derive(Serialize, Default)]
struct CognitoUserPoolAccountRecoverySettingElDynamic {
    recovery_mechanism: Option<DynamicBlock<CognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl>>,
}

#[derive(Serialize)]
pub struct CognitoUserPoolAccountRecoverySettingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    recovery_mechanism: Option<Vec<CognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl>>,
    dynamic: CognitoUserPoolAccountRecoverySettingElDynamic,
}

impl CognitoUserPoolAccountRecoverySettingEl {
    #[doc = "Set the field `recovery_mechanism`.\n"]
    pub fn set_recovery_mechanism(
        mut self,
        v: impl Into<BlockAssignable<CognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.recovery_mechanism = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.recovery_mechanism = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CognitoUserPoolAccountRecoverySettingEl {
    type O = BlockAssignable<CognitoUserPoolAccountRecoverySettingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolAccountRecoverySettingEl {}

impl BuildCognitoUserPoolAccountRecoverySettingEl {
    pub fn build(self) -> CognitoUserPoolAccountRecoverySettingEl {
        CognitoUserPoolAccountRecoverySettingEl {
            recovery_mechanism: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CognitoUserPoolAccountRecoverySettingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolAccountRecoverySettingElRef {
    fn new(shared: StackShared, base: String) -> CognitoUserPoolAccountRecoverySettingElRef {
        CognitoUserPoolAccountRecoverySettingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolAccountRecoverySettingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    email_message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_subject: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sms_message: Option<PrimField<String>>,
}

impl CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl {
    #[doc = "Set the field `email_message`.\n"]
    pub fn set_email_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email_message = Some(v.into());
        self
    }

    #[doc = "Set the field `email_subject`.\n"]
    pub fn set_email_subject(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email_subject = Some(v.into());
        self
    }

    #[doc = "Set the field `sms_message`.\n"]
    pub fn set_sms_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sms_message = Some(v.into());
        self
    }
}

impl ToListMappable for CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl {
    type O = BlockAssignable<CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl {}

impl BuildCognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl {
    pub fn build(self) -> CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl {
        CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl {
            email_message: core::default::Default::default(),
            email_subject: core::default::Default::default(),
            sms_message: core::default::Default::default(),
        }
    }
}

pub struct CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateElRef {
    fn new(shared: StackShared, base: String) -> CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateElRef {
        CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email_message` after provisioning.\n"]
    pub fn email_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_message", self.base))
    }

    #[doc = "Get a reference to the value of field `email_subject` after provisioning.\n"]
    pub fn email_subject(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_subject", self.base))
    }

    #[doc = "Get a reference to the value of field `sms_message` after provisioning.\n"]
    pub fn sms_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sms_message", self.base))
    }
}

#[derive(Serialize, Default)]
struct CognitoUserPoolAdminCreateUserConfigElDynamic {
    invite_message_template: Option<DynamicBlock<CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl>>,
}

#[derive(Serialize)]
pub struct CognitoUserPoolAdminCreateUserConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_admin_create_user_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invite_message_template: Option<Vec<CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl>>,
    dynamic: CognitoUserPoolAdminCreateUserConfigElDynamic,
}

impl CognitoUserPoolAdminCreateUserConfigEl {
    #[doc = "Set the field `allow_admin_create_user_only`.\n"]
    pub fn set_allow_admin_create_user_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_admin_create_user_only = Some(v.into());
        self
    }

    #[doc = "Set the field `invite_message_template`.\n"]
    pub fn set_invite_message_template(
        mut self,
        v: impl Into<BlockAssignable<CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.invite_message_template = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.invite_message_template = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CognitoUserPoolAdminCreateUserConfigEl {
    type O = BlockAssignable<CognitoUserPoolAdminCreateUserConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolAdminCreateUserConfigEl {}

impl BuildCognitoUserPoolAdminCreateUserConfigEl {
    pub fn build(self) -> CognitoUserPoolAdminCreateUserConfigEl {
        CognitoUserPoolAdminCreateUserConfigEl {
            allow_admin_create_user_only: core::default::Default::default(),
            invite_message_template: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CognitoUserPoolAdminCreateUserConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolAdminCreateUserConfigElRef {
    fn new(shared: StackShared, base: String) -> CognitoUserPoolAdminCreateUserConfigElRef {
        CognitoUserPoolAdminCreateUserConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolAdminCreateUserConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_admin_create_user_only` after provisioning.\n"]
    pub fn allow_admin_create_user_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_admin_create_user_only", self.base))
    }

    #[doc = "Get a reference to the value of field `invite_message_template` after provisioning.\n"]
    pub fn invite_message_template(&self) -> ListRef<CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.invite_message_template", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolDeviceConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    challenge_required_on_new_device: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_only_remembered_on_user_prompt: Option<PrimField<bool>>,
}

impl CognitoUserPoolDeviceConfigurationEl {
    #[doc = "Set the field `challenge_required_on_new_device`.\n"]
    pub fn set_challenge_required_on_new_device(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.challenge_required_on_new_device = Some(v.into());
        self
    }

    #[doc = "Set the field `device_only_remembered_on_user_prompt`.\n"]
    pub fn set_device_only_remembered_on_user_prompt(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.device_only_remembered_on_user_prompt = Some(v.into());
        self
    }
}

impl ToListMappable for CognitoUserPoolDeviceConfigurationEl {
    type O = BlockAssignable<CognitoUserPoolDeviceConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolDeviceConfigurationEl {}

impl BuildCognitoUserPoolDeviceConfigurationEl {
    pub fn build(self) -> CognitoUserPoolDeviceConfigurationEl {
        CognitoUserPoolDeviceConfigurationEl {
            challenge_required_on_new_device: core::default::Default::default(),
            device_only_remembered_on_user_prompt: core::default::Default::default(),
        }
    }
}

pub struct CognitoUserPoolDeviceConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolDeviceConfigurationElRef {
    fn new(shared: StackShared, base: String) -> CognitoUserPoolDeviceConfigurationElRef {
        CognitoUserPoolDeviceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolDeviceConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `challenge_required_on_new_device` after provisioning.\n"]
    pub fn challenge_required_on_new_device(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.challenge_required_on_new_device", self.base))
    }

    #[doc = "Get a reference to the value of field `device_only_remembered_on_user_prompt` after provisioning.\n"]
    pub fn device_only_remembered_on_user_prompt(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_only_remembered_on_user_prompt", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolEmailConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration_set: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_sending_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_email_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_email_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_arn: Option<PrimField<String>>,
}

impl CognitoUserPoolEmailConfigurationEl {
    #[doc = "Set the field `configuration_set`.\n"]
    pub fn set_configuration_set(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.configuration_set = Some(v.into());
        self
    }

    #[doc = "Set the field `email_sending_account`.\n"]
    pub fn set_email_sending_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email_sending_account = Some(v.into());
        self
    }

    #[doc = "Set the field `from_email_address`.\n"]
    pub fn set_from_email_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.from_email_address = Some(v.into());
        self
    }

    #[doc = "Set the field `reply_to_email_address`.\n"]
    pub fn set_reply_to_email_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reply_to_email_address = Some(v.into());
        self
    }

    #[doc = "Set the field `source_arn`.\n"]
    pub fn set_source_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_arn = Some(v.into());
        self
    }
}

impl ToListMappable for CognitoUserPoolEmailConfigurationEl {
    type O = BlockAssignable<CognitoUserPoolEmailConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolEmailConfigurationEl {}

impl BuildCognitoUserPoolEmailConfigurationEl {
    pub fn build(self) -> CognitoUserPoolEmailConfigurationEl {
        CognitoUserPoolEmailConfigurationEl {
            configuration_set: core::default::Default::default(),
            email_sending_account: core::default::Default::default(),
            from_email_address: core::default::Default::default(),
            reply_to_email_address: core::default::Default::default(),
            source_arn: core::default::Default::default(),
        }
    }
}

pub struct CognitoUserPoolEmailConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolEmailConfigurationElRef {
    fn new(shared: StackShared, base: String) -> CognitoUserPoolEmailConfigurationElRef {
        CognitoUserPoolEmailConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolEmailConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configuration_set` after provisioning.\n"]
    pub fn configuration_set(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.configuration_set", self.base))
    }

    #[doc = "Get a reference to the value of field `email_sending_account` after provisioning.\n"]
    pub fn email_sending_account(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_sending_account", self.base))
    }

    #[doc = "Get a reference to the value of field `from_email_address` after provisioning.\n"]
    pub fn from_email_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_email_address", self.base))
    }

    #[doc = "Get a reference to the value of field `reply_to_email_address` after provisioning.\n"]
    pub fn reply_to_email_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reply_to_email_address", self.base))
    }

    #[doc = "Get a reference to the value of field `source_arn` after provisioning.\n"]
    pub fn source_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolEmailMfaConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject: Option<PrimField<String>>,
}

impl CognitoUserPoolEmailMfaConfigurationEl {
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }

    #[doc = "Set the field `subject`.\n"]
    pub fn set_subject(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subject = Some(v.into());
        self
    }
}

impl ToListMappable for CognitoUserPoolEmailMfaConfigurationEl {
    type O = BlockAssignable<CognitoUserPoolEmailMfaConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolEmailMfaConfigurationEl {}

impl BuildCognitoUserPoolEmailMfaConfigurationEl {
    pub fn build(self) -> CognitoUserPoolEmailMfaConfigurationEl {
        CognitoUserPoolEmailMfaConfigurationEl {
            message: core::default::Default::default(),
            subject: core::default::Default::default(),
        }
    }
}

pub struct CognitoUserPoolEmailMfaConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolEmailMfaConfigurationElRef {
    fn new(shared: StackShared, base: String) -> CognitoUserPoolEmailMfaConfigurationElRef {
        CognitoUserPoolEmailMfaConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolEmailMfaConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `subject` after provisioning.\n"]
    pub fn subject(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subject", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolLambdaConfigElCustomEmailSenderEl {
    lambda_arn: PrimField<String>,
    lambda_version: PrimField<String>,
}

impl CognitoUserPoolLambdaConfigElCustomEmailSenderEl { }

impl ToListMappable for CognitoUserPoolLambdaConfigElCustomEmailSenderEl {
    type O = BlockAssignable<CognitoUserPoolLambdaConfigElCustomEmailSenderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolLambdaConfigElCustomEmailSenderEl {
    #[doc = ""]
    pub lambda_arn: PrimField<String>,
    #[doc = ""]
    pub lambda_version: PrimField<String>,
}

impl BuildCognitoUserPoolLambdaConfigElCustomEmailSenderEl {
    pub fn build(self) -> CognitoUserPoolLambdaConfigElCustomEmailSenderEl {
        CognitoUserPoolLambdaConfigElCustomEmailSenderEl {
            lambda_arn: self.lambda_arn,
            lambda_version: self.lambda_version,
        }
    }
}

pub struct CognitoUserPoolLambdaConfigElCustomEmailSenderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolLambdaConfigElCustomEmailSenderElRef {
    fn new(shared: StackShared, base: String) -> CognitoUserPoolLambdaConfigElCustomEmailSenderElRef {
        CognitoUserPoolLambdaConfigElCustomEmailSenderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolLambdaConfigElCustomEmailSenderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lambda_arn` after provisioning.\n"]
    pub fn lambda_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lambda_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_version` after provisioning.\n"]
    pub fn lambda_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lambda_version", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolLambdaConfigElCustomSmsSenderEl {
    lambda_arn: PrimField<String>,
    lambda_version: PrimField<String>,
}

impl CognitoUserPoolLambdaConfigElCustomSmsSenderEl { }

impl ToListMappable for CognitoUserPoolLambdaConfigElCustomSmsSenderEl {
    type O = BlockAssignable<CognitoUserPoolLambdaConfigElCustomSmsSenderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolLambdaConfigElCustomSmsSenderEl {
    #[doc = ""]
    pub lambda_arn: PrimField<String>,
    #[doc = ""]
    pub lambda_version: PrimField<String>,
}

impl BuildCognitoUserPoolLambdaConfigElCustomSmsSenderEl {
    pub fn build(self) -> CognitoUserPoolLambdaConfigElCustomSmsSenderEl {
        CognitoUserPoolLambdaConfigElCustomSmsSenderEl {
            lambda_arn: self.lambda_arn,
            lambda_version: self.lambda_version,
        }
    }
}

pub struct CognitoUserPoolLambdaConfigElCustomSmsSenderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolLambdaConfigElCustomSmsSenderElRef {
    fn new(shared: StackShared, base: String) -> CognitoUserPoolLambdaConfigElCustomSmsSenderElRef {
        CognitoUserPoolLambdaConfigElCustomSmsSenderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolLambdaConfigElCustomSmsSenderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lambda_arn` after provisioning.\n"]
    pub fn lambda_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lambda_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_version` after provisioning.\n"]
    pub fn lambda_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lambda_version", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolLambdaConfigElPreTokenGenerationConfigEl {
    lambda_arn: PrimField<String>,
    lambda_version: PrimField<String>,
}

impl CognitoUserPoolLambdaConfigElPreTokenGenerationConfigEl { }

impl ToListMappable for CognitoUserPoolLambdaConfigElPreTokenGenerationConfigEl {
    type O = BlockAssignable<CognitoUserPoolLambdaConfigElPreTokenGenerationConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolLambdaConfigElPreTokenGenerationConfigEl {
    #[doc = ""]
    pub lambda_arn: PrimField<String>,
    #[doc = ""]
    pub lambda_version: PrimField<String>,
}

impl BuildCognitoUserPoolLambdaConfigElPreTokenGenerationConfigEl {
    pub fn build(self) -> CognitoUserPoolLambdaConfigElPreTokenGenerationConfigEl {
        CognitoUserPoolLambdaConfigElPreTokenGenerationConfigEl {
            lambda_arn: self.lambda_arn,
            lambda_version: self.lambda_version,
        }
    }
}

pub struct CognitoUserPoolLambdaConfigElPreTokenGenerationConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolLambdaConfigElPreTokenGenerationConfigElRef {
    fn new(shared: StackShared, base: String) -> CognitoUserPoolLambdaConfigElPreTokenGenerationConfigElRef {
        CognitoUserPoolLambdaConfigElPreTokenGenerationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolLambdaConfigElPreTokenGenerationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lambda_arn` after provisioning.\n"]
    pub fn lambda_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lambda_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_version` after provisioning.\n"]
    pub fn lambda_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lambda_version", self.base))
    }
}

#[derive(Serialize, Default)]
struct CognitoUserPoolLambdaConfigElDynamic {
    custom_email_sender: Option<DynamicBlock<CognitoUserPoolLambdaConfigElCustomEmailSenderEl>>,
    custom_sms_sender: Option<DynamicBlock<CognitoUserPoolLambdaConfigElCustomSmsSenderEl>>,
    pre_token_generation_config: Option<DynamicBlock<CognitoUserPoolLambdaConfigElPreTokenGenerationConfigEl>>,
}

#[derive(Serialize)]
pub struct CognitoUserPoolLambdaConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create_auth_challenge: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    define_auth_challenge: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_authentication: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_confirmation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pre_authentication: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pre_sign_up: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pre_token_generation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_migration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verify_auth_challenge_response: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_email_sender: Option<Vec<CognitoUserPoolLambdaConfigElCustomEmailSenderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_sms_sender: Option<Vec<CognitoUserPoolLambdaConfigElCustomSmsSenderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pre_token_generation_config: Option<Vec<CognitoUserPoolLambdaConfigElPreTokenGenerationConfigEl>>,
    dynamic: CognitoUserPoolLambdaConfigElDynamic,
}

impl CognitoUserPoolLambdaConfigEl {
    #[doc = "Set the field `create_auth_challenge`.\n"]
    pub fn set_create_auth_challenge(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_auth_challenge = Some(v.into());
        self
    }

    #[doc = "Set the field `custom_message`.\n"]
    pub fn set_custom_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.custom_message = Some(v.into());
        self
    }

    #[doc = "Set the field `define_auth_challenge`.\n"]
    pub fn set_define_auth_challenge(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.define_auth_challenge = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `post_authentication`.\n"]
    pub fn set_post_authentication(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.post_authentication = Some(v.into());
        self
    }

    #[doc = "Set the field `post_confirmation`.\n"]
    pub fn set_post_confirmation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.post_confirmation = Some(v.into());
        self
    }

    #[doc = "Set the field `pre_authentication`.\n"]
    pub fn set_pre_authentication(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pre_authentication = Some(v.into());
        self
    }

    #[doc = "Set the field `pre_sign_up`.\n"]
    pub fn set_pre_sign_up(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pre_sign_up = Some(v.into());
        self
    }

    #[doc = "Set the field `pre_token_generation`.\n"]
    pub fn set_pre_token_generation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pre_token_generation = Some(v.into());
        self
    }

    #[doc = "Set the field `user_migration`.\n"]
    pub fn set_user_migration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_migration = Some(v.into());
        self
    }

    #[doc = "Set the field `verify_auth_challenge_response`.\n"]
    pub fn set_verify_auth_challenge_response(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.verify_auth_challenge_response = Some(v.into());
        self
    }

    #[doc = "Set the field `custom_email_sender`.\n"]
    pub fn set_custom_email_sender(
        mut self,
        v: impl Into<BlockAssignable<CognitoUserPoolLambdaConfigElCustomEmailSenderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_email_sender = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_email_sender = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_sms_sender`.\n"]
    pub fn set_custom_sms_sender(
        mut self,
        v: impl Into<BlockAssignable<CognitoUserPoolLambdaConfigElCustomSmsSenderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_sms_sender = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_sms_sender = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pre_token_generation_config`.\n"]
    pub fn set_pre_token_generation_config(
        mut self,
        v: impl Into<BlockAssignable<CognitoUserPoolLambdaConfigElPreTokenGenerationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pre_token_generation_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pre_token_generation_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CognitoUserPoolLambdaConfigEl {
    type O = BlockAssignable<CognitoUserPoolLambdaConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolLambdaConfigEl {}

impl BuildCognitoUserPoolLambdaConfigEl {
    pub fn build(self) -> CognitoUserPoolLambdaConfigEl {
        CognitoUserPoolLambdaConfigEl {
            create_auth_challenge: core::default::Default::default(),
            custom_message: core::default::Default::default(),
            define_auth_challenge: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            post_authentication: core::default::Default::default(),
            post_confirmation: core::default::Default::default(),
            pre_authentication: core::default::Default::default(),
            pre_sign_up: core::default::Default::default(),
            pre_token_generation: core::default::Default::default(),
            user_migration: core::default::Default::default(),
            verify_auth_challenge_response: core::default::Default::default(),
            custom_email_sender: core::default::Default::default(),
            custom_sms_sender: core::default::Default::default(),
            pre_token_generation_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CognitoUserPoolLambdaConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolLambdaConfigElRef {
    fn new(shared: StackShared, base: String) -> CognitoUserPoolLambdaConfigElRef {
        CognitoUserPoolLambdaConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolLambdaConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create_auth_challenge` after provisioning.\n"]
    pub fn create_auth_challenge(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_auth_challenge", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_message` after provisioning.\n"]
    pub fn custom_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_message", self.base))
    }

    #[doc = "Get a reference to the value of field `define_auth_challenge` after provisioning.\n"]
    pub fn define_auth_challenge(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.define_auth_challenge", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `post_authentication` after provisioning.\n"]
    pub fn post_authentication(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.post_authentication", self.base))
    }

    #[doc = "Get a reference to the value of field `post_confirmation` after provisioning.\n"]
    pub fn post_confirmation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.post_confirmation", self.base))
    }

    #[doc = "Get a reference to the value of field `pre_authentication` after provisioning.\n"]
    pub fn pre_authentication(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pre_authentication", self.base))
    }

    #[doc = "Get a reference to the value of field `pre_sign_up` after provisioning.\n"]
    pub fn pre_sign_up(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pre_sign_up", self.base))
    }

    #[doc = "Get a reference to the value of field `pre_token_generation` after provisioning.\n"]
    pub fn pre_token_generation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pre_token_generation", self.base))
    }

    #[doc = "Get a reference to the value of field `user_migration` after provisioning.\n"]
    pub fn user_migration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_migration", self.base))
    }

    #[doc = "Get a reference to the value of field `verify_auth_challenge_response` after provisioning.\n"]
    pub fn verify_auth_challenge_response(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.verify_auth_challenge_response", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_email_sender` after provisioning.\n"]
    pub fn custom_email_sender(&self) -> ListRef<CognitoUserPoolLambdaConfigElCustomEmailSenderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_email_sender", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_sms_sender` after provisioning.\n"]
    pub fn custom_sms_sender(&self) -> ListRef<CognitoUserPoolLambdaConfigElCustomSmsSenderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_sms_sender", self.base))
    }

    #[doc = "Get a reference to the value of field `pre_token_generation_config` after provisioning.\n"]
    pub fn pre_token_generation_config(&self) -> ListRef<CognitoUserPoolLambdaConfigElPreTokenGenerationConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pre_token_generation_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolPasswordPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_history_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_lowercase: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_numbers: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_symbols: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_uppercase: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temporary_password_validity_days: Option<PrimField<f64>>,
}

impl CognitoUserPoolPasswordPolicyEl {
    #[doc = "Set the field `minimum_length`.\n"]
    pub fn set_minimum_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minimum_length = Some(v.into());
        self
    }

    #[doc = "Set the field `password_history_size`.\n"]
    pub fn set_password_history_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.password_history_size = Some(v.into());
        self
    }

    #[doc = "Set the field `require_lowercase`.\n"]
    pub fn set_require_lowercase(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_lowercase = Some(v.into());
        self
    }

    #[doc = "Set the field `require_numbers`.\n"]
    pub fn set_require_numbers(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_numbers = Some(v.into());
        self
    }

    #[doc = "Set the field `require_symbols`.\n"]
    pub fn set_require_symbols(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_symbols = Some(v.into());
        self
    }

    #[doc = "Set the field `require_uppercase`.\n"]
    pub fn set_require_uppercase(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_uppercase = Some(v.into());
        self
    }

    #[doc = "Set the field `temporary_password_validity_days`.\n"]
    pub fn set_temporary_password_validity_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.temporary_password_validity_days = Some(v.into());
        self
    }
}

impl ToListMappable for CognitoUserPoolPasswordPolicyEl {
    type O = BlockAssignable<CognitoUserPoolPasswordPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolPasswordPolicyEl {}

impl BuildCognitoUserPoolPasswordPolicyEl {
    pub fn build(self) -> CognitoUserPoolPasswordPolicyEl {
        CognitoUserPoolPasswordPolicyEl {
            minimum_length: core::default::Default::default(),
            password_history_size: core::default::Default::default(),
            require_lowercase: core::default::Default::default(),
            require_numbers: core::default::Default::default(),
            require_symbols: core::default::Default::default(),
            require_uppercase: core::default::Default::default(),
            temporary_password_validity_days: core::default::Default::default(),
        }
    }
}

pub struct CognitoUserPoolPasswordPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolPasswordPolicyElRef {
    fn new(shared: StackShared, base: String) -> CognitoUserPoolPasswordPolicyElRef {
        CognitoUserPoolPasswordPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolPasswordPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `minimum_length` after provisioning.\n"]
    pub fn minimum_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum_length", self.base))
    }

    #[doc = "Get a reference to the value of field `password_history_size` after provisioning.\n"]
    pub fn password_history_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_history_size", self.base))
    }

    #[doc = "Get a reference to the value of field `require_lowercase` after provisioning.\n"]
    pub fn require_lowercase(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_lowercase", self.base))
    }

    #[doc = "Get a reference to the value of field `require_numbers` after provisioning.\n"]
    pub fn require_numbers(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_numbers", self.base))
    }

    #[doc = "Get a reference to the value of field `require_symbols` after provisioning.\n"]
    pub fn require_symbols(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_symbols", self.base))
    }

    #[doc = "Get a reference to the value of field `require_uppercase` after provisioning.\n"]
    pub fn require_uppercase(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_uppercase", self.base))
    }

    #[doc = "Get a reference to the value of field `temporary_password_validity_days` after provisioning.\n"]
    pub fn temporary_password_validity_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.temporary_password_validity_days", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolSchemaElNumberAttributeConstraintsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_value: Option<PrimField<String>>,
}

impl CognitoUserPoolSchemaElNumberAttributeConstraintsEl {
    #[doc = "Set the field `max_value`.\n"]
    pub fn set_max_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_value = Some(v.into());
        self
    }

    #[doc = "Set the field `min_value`.\n"]
    pub fn set_min_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_value = Some(v.into());
        self
    }
}

impl ToListMappable for CognitoUserPoolSchemaElNumberAttributeConstraintsEl {
    type O = BlockAssignable<CognitoUserPoolSchemaElNumberAttributeConstraintsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolSchemaElNumberAttributeConstraintsEl {}

impl BuildCognitoUserPoolSchemaElNumberAttributeConstraintsEl {
    pub fn build(self) -> CognitoUserPoolSchemaElNumberAttributeConstraintsEl {
        CognitoUserPoolSchemaElNumberAttributeConstraintsEl {
            max_value: core::default::Default::default(),
            min_value: core::default::Default::default(),
        }
    }
}

pub struct CognitoUserPoolSchemaElNumberAttributeConstraintsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolSchemaElNumberAttributeConstraintsElRef {
    fn new(shared: StackShared, base: String) -> CognitoUserPoolSchemaElNumberAttributeConstraintsElRef {
        CognitoUserPoolSchemaElNumberAttributeConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolSchemaElNumberAttributeConstraintsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_value` after provisioning.\n"]
    pub fn max_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_value", self.base))
    }

    #[doc = "Get a reference to the value of field `min_value` after provisioning.\n"]
    pub fn min_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_value", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolSchemaElStringAttributeConstraintsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_length: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_length: Option<PrimField<String>>,
}

impl CognitoUserPoolSchemaElStringAttributeConstraintsEl {
    #[doc = "Set the field `max_length`.\n"]
    pub fn set_max_length(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_length = Some(v.into());
        self
    }

    #[doc = "Set the field `min_length`.\n"]
    pub fn set_min_length(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_length = Some(v.into());
        self
    }
}

impl ToListMappable for CognitoUserPoolSchemaElStringAttributeConstraintsEl {
    type O = BlockAssignable<CognitoUserPoolSchemaElStringAttributeConstraintsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolSchemaElStringAttributeConstraintsEl {}

impl BuildCognitoUserPoolSchemaElStringAttributeConstraintsEl {
    pub fn build(self) -> CognitoUserPoolSchemaElStringAttributeConstraintsEl {
        CognitoUserPoolSchemaElStringAttributeConstraintsEl {
            max_length: core::default::Default::default(),
            min_length: core::default::Default::default(),
        }
    }
}

pub struct CognitoUserPoolSchemaElStringAttributeConstraintsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolSchemaElStringAttributeConstraintsElRef {
    fn new(shared: StackShared, base: String) -> CognitoUserPoolSchemaElStringAttributeConstraintsElRef {
        CognitoUserPoolSchemaElStringAttributeConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolSchemaElStringAttributeConstraintsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_length` after provisioning.\n"]
    pub fn max_length(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_length", self.base))
    }

    #[doc = "Get a reference to the value of field `min_length` after provisioning.\n"]
    pub fn min_length(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_length", self.base))
    }
}

#[derive(Serialize, Default)]
struct CognitoUserPoolSchemaElDynamic {
    number_attribute_constraints: Option<DynamicBlock<CognitoUserPoolSchemaElNumberAttributeConstraintsEl>>,
    string_attribute_constraints: Option<DynamicBlock<CognitoUserPoolSchemaElStringAttributeConstraintsEl>>,
}

#[derive(Serialize)]
pub struct CognitoUserPoolSchemaEl {
    attribute_data_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    developer_only_attribute: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mutable: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number_attribute_constraints: Option<Vec<CognitoUserPoolSchemaElNumberAttributeConstraintsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_attribute_constraints: Option<Vec<CognitoUserPoolSchemaElStringAttributeConstraintsEl>>,
    dynamic: CognitoUserPoolSchemaElDynamic,
}

impl CognitoUserPoolSchemaEl {
    #[doc = "Set the field `developer_only_attribute`.\n"]
    pub fn set_developer_only_attribute(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.developer_only_attribute = Some(v.into());
        self
    }

    #[doc = "Set the field `mutable`.\n"]
    pub fn set_mutable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.mutable = Some(v.into());
        self
    }

    #[doc = "Set the field `required`.\n"]
    pub fn set_required(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.required = Some(v.into());
        self
    }

    #[doc = "Set the field `number_attribute_constraints`.\n"]
    pub fn set_number_attribute_constraints(
        mut self,
        v: impl Into<BlockAssignable<CognitoUserPoolSchemaElNumberAttributeConstraintsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.number_attribute_constraints = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.number_attribute_constraints = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `string_attribute_constraints`.\n"]
    pub fn set_string_attribute_constraints(
        mut self,
        v: impl Into<BlockAssignable<CognitoUserPoolSchemaElStringAttributeConstraintsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.string_attribute_constraints = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.string_attribute_constraints = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CognitoUserPoolSchemaEl {
    type O = BlockAssignable<CognitoUserPoolSchemaEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolSchemaEl {
    #[doc = ""]
    pub attribute_data_type: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildCognitoUserPoolSchemaEl {
    pub fn build(self) -> CognitoUserPoolSchemaEl {
        CognitoUserPoolSchemaEl {
            attribute_data_type: self.attribute_data_type,
            developer_only_attribute: core::default::Default::default(),
            mutable: core::default::Default::default(),
            name: self.name,
            required: core::default::Default::default(),
            number_attribute_constraints: core::default::Default::default(),
            string_attribute_constraints: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CognitoUserPoolSchemaElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolSchemaElRef {
    fn new(shared: StackShared, base: String) -> CognitoUserPoolSchemaElRef {
        CognitoUserPoolSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolSchemaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute_data_type` after provisioning.\n"]
    pub fn attribute_data_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attribute_data_type", self.base))
    }

    #[doc = "Get a reference to the value of field `developer_only_attribute` after provisioning.\n"]
    pub fn developer_only_attribute(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.developer_only_attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `mutable` after provisioning.\n"]
    pub fn mutable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.mutable", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `required` after provisioning.\n"]
    pub fn required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }

    #[doc = "Get a reference to the value of field `number_attribute_constraints` after provisioning.\n"]
    pub fn number_attribute_constraints(&self) -> ListRef<CognitoUserPoolSchemaElNumberAttributeConstraintsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.number_attribute_constraints", self.base))
    }

    #[doc = "Get a reference to the value of field `string_attribute_constraints` after provisioning.\n"]
    pub fn string_attribute_constraints(&self) -> ListRef<CognitoUserPoolSchemaElStringAttributeConstraintsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.string_attribute_constraints", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolSignInPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_first_auth_factors: Option<SetField<PrimField<String>>>,
}

impl CognitoUserPoolSignInPolicyEl {
    #[doc = "Set the field `allowed_first_auth_factors`.\n"]
    pub fn set_allowed_first_auth_factors(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allowed_first_auth_factors = Some(v.into());
        self
    }
}

impl ToListMappable for CognitoUserPoolSignInPolicyEl {
    type O = BlockAssignable<CognitoUserPoolSignInPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolSignInPolicyEl {}

impl BuildCognitoUserPoolSignInPolicyEl {
    pub fn build(self) -> CognitoUserPoolSignInPolicyEl {
        CognitoUserPoolSignInPolicyEl { allowed_first_auth_factors: core::default::Default::default() }
    }
}

pub struct CognitoUserPoolSignInPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolSignInPolicyElRef {
    fn new(shared: StackShared, base: String) -> CognitoUserPoolSignInPolicyElRef {
        CognitoUserPoolSignInPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolSignInPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_first_auth_factors` after provisioning.\n"]
    pub fn allowed_first_auth_factors(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allowed_first_auth_factors", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolSmsConfigurationEl {
    external_id: PrimField<String>,
    sns_caller_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sns_region: Option<PrimField<String>>,
}

impl CognitoUserPoolSmsConfigurationEl {
    #[doc = "Set the field `sns_region`.\n"]
    pub fn set_sns_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sns_region = Some(v.into());
        self
    }
}

impl ToListMappable for CognitoUserPoolSmsConfigurationEl {
    type O = BlockAssignable<CognitoUserPoolSmsConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolSmsConfigurationEl {
    #[doc = ""]
    pub external_id: PrimField<String>,
    #[doc = ""]
    pub sns_caller_arn: PrimField<String>,
}

impl BuildCognitoUserPoolSmsConfigurationEl {
    pub fn build(self) -> CognitoUserPoolSmsConfigurationEl {
        CognitoUserPoolSmsConfigurationEl {
            external_id: self.external_id,
            sns_caller_arn: self.sns_caller_arn,
            sns_region: core::default::Default::default(),
        }
    }
}

pub struct CognitoUserPoolSmsConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolSmsConfigurationElRef {
    fn new(shared: StackShared, base: String) -> CognitoUserPoolSmsConfigurationElRef {
        CognitoUserPoolSmsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolSmsConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }

    #[doc = "Get a reference to the value of field `sns_caller_arn` after provisioning.\n"]
    pub fn sns_caller_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sns_caller_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sns_region` after provisioning.\n"]
    pub fn sns_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sns_region", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolSoftwareTokenMfaConfigurationEl {
    enabled: PrimField<bool>,
}

impl CognitoUserPoolSoftwareTokenMfaConfigurationEl { }

impl ToListMappable for CognitoUserPoolSoftwareTokenMfaConfigurationEl {
    type O = BlockAssignable<CognitoUserPoolSoftwareTokenMfaConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolSoftwareTokenMfaConfigurationEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}

impl BuildCognitoUserPoolSoftwareTokenMfaConfigurationEl {
    pub fn build(self) -> CognitoUserPoolSoftwareTokenMfaConfigurationEl {
        CognitoUserPoolSoftwareTokenMfaConfigurationEl { enabled: self.enabled }
    }
}

pub struct CognitoUserPoolSoftwareTokenMfaConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolSoftwareTokenMfaConfigurationElRef {
    fn new(shared: StackShared, base: String) -> CognitoUserPoolSoftwareTokenMfaConfigurationElRef {
        CognitoUserPoolSoftwareTokenMfaConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolSoftwareTokenMfaConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolUserAttributeUpdateSettingsEl {
    attributes_require_verification_before_update: SetField<PrimField<String>>,
}

impl CognitoUserPoolUserAttributeUpdateSettingsEl { }

impl ToListMappable for CognitoUserPoolUserAttributeUpdateSettingsEl {
    type O = BlockAssignable<CognitoUserPoolUserAttributeUpdateSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolUserAttributeUpdateSettingsEl {
    #[doc = ""]
    pub attributes_require_verification_before_update: SetField<PrimField<String>>,
}

impl BuildCognitoUserPoolUserAttributeUpdateSettingsEl {
    pub fn build(self) -> CognitoUserPoolUserAttributeUpdateSettingsEl {
        CognitoUserPoolUserAttributeUpdateSettingsEl {
            attributes_require_verification_before_update: self.attributes_require_verification_before_update,
        }
    }
}

pub struct CognitoUserPoolUserAttributeUpdateSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolUserAttributeUpdateSettingsElRef {
    fn new(shared: StackShared, base: String) -> CognitoUserPoolUserAttributeUpdateSettingsElRef {
        CognitoUserPoolUserAttributeUpdateSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolUserAttributeUpdateSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attributes_require_verification_before_update` after provisioning.\n"]
    pub fn attributes_require_verification_before_update(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.attributes_require_verification_before_update", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_auth_mode: Option<PrimField<String>>,
}

impl CognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsEl {
    #[doc = "Set the field `custom_auth_mode`.\n"]
    pub fn set_custom_auth_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.custom_auth_mode = Some(v.into());
        self
    }
}

impl ToListMappable for CognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsEl {
    type O = BlockAssignable<CognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsEl {}

impl BuildCognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsEl {
    pub fn build(self) -> CognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsEl {
        CognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsEl {
            custom_auth_mode: core::default::Default::default(),
        }
    }
}

pub struct CognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsElRef {
    fn new(shared: StackShared, base: String) -> CognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsElRef {
        CognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_auth_mode` after provisioning.\n"]
    pub fn custom_auth_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_auth_mode", self.base))
    }
}

#[derive(Serialize, Default)]
struct CognitoUserPoolUserPoolAddOnsElDynamic {
    advanced_security_additional_flows: Option<
        DynamicBlock<CognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsEl>,
    >,
}

#[derive(Serialize)]
pub struct CognitoUserPoolUserPoolAddOnsEl {
    advanced_security_mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_security_additional_flows: Option<Vec<CognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsEl>>,
    dynamic: CognitoUserPoolUserPoolAddOnsElDynamic,
}

impl CognitoUserPoolUserPoolAddOnsEl {
    #[doc = "Set the field `advanced_security_additional_flows`.\n"]
    pub fn set_advanced_security_additional_flows(
        mut self,
        v: impl Into<BlockAssignable<CognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.advanced_security_additional_flows = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_security_additional_flows = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CognitoUserPoolUserPoolAddOnsEl {
    type O = BlockAssignable<CognitoUserPoolUserPoolAddOnsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolUserPoolAddOnsEl {
    #[doc = ""]
    pub advanced_security_mode: PrimField<String>,
}

impl BuildCognitoUserPoolUserPoolAddOnsEl {
    pub fn build(self) -> CognitoUserPoolUserPoolAddOnsEl {
        CognitoUserPoolUserPoolAddOnsEl {
            advanced_security_mode: self.advanced_security_mode,
            advanced_security_additional_flows: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CognitoUserPoolUserPoolAddOnsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolUserPoolAddOnsElRef {
    fn new(shared: StackShared, base: String) -> CognitoUserPoolUserPoolAddOnsElRef {
        CognitoUserPoolUserPoolAddOnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolUserPoolAddOnsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `advanced_security_mode` after provisioning.\n"]
    pub fn advanced_security_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.advanced_security_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `advanced_security_additional_flows` after provisioning.\n"]
    pub fn advanced_security_additional_flows(
        &self,
    ) -> ListRef<CognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.advanced_security_additional_flows", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolUsernameConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    case_sensitive: Option<PrimField<bool>>,
}

impl CognitoUserPoolUsernameConfigurationEl {
    #[doc = "Set the field `case_sensitive`.\n"]
    pub fn set_case_sensitive(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.case_sensitive = Some(v.into());
        self
    }
}

impl ToListMappable for CognitoUserPoolUsernameConfigurationEl {
    type O = BlockAssignable<CognitoUserPoolUsernameConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolUsernameConfigurationEl {}

impl BuildCognitoUserPoolUsernameConfigurationEl {
    pub fn build(self) -> CognitoUserPoolUsernameConfigurationEl {
        CognitoUserPoolUsernameConfigurationEl { case_sensitive: core::default::Default::default() }
    }
}

pub struct CognitoUserPoolUsernameConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolUsernameConfigurationElRef {
    fn new(shared: StackShared, base: String) -> CognitoUserPoolUsernameConfigurationElRef {
        CognitoUserPoolUsernameConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolUsernameConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `case_sensitive` after provisioning.\n"]
    pub fn case_sensitive(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.case_sensitive", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolVerificationMessageTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_email_option: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_message_by_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_subject: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_subject_by_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sms_message: Option<PrimField<String>>,
}

impl CognitoUserPoolVerificationMessageTemplateEl {
    #[doc = "Set the field `default_email_option`.\n"]
    pub fn set_default_email_option(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_email_option = Some(v.into());
        self
    }

    #[doc = "Set the field `email_message`.\n"]
    pub fn set_email_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email_message = Some(v.into());
        self
    }

    #[doc = "Set the field `email_message_by_link`.\n"]
    pub fn set_email_message_by_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email_message_by_link = Some(v.into());
        self
    }

    #[doc = "Set the field `email_subject`.\n"]
    pub fn set_email_subject(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email_subject = Some(v.into());
        self
    }

    #[doc = "Set the field `email_subject_by_link`.\n"]
    pub fn set_email_subject_by_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email_subject_by_link = Some(v.into());
        self
    }

    #[doc = "Set the field `sms_message`.\n"]
    pub fn set_sms_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sms_message = Some(v.into());
        self
    }
}

impl ToListMappable for CognitoUserPoolVerificationMessageTemplateEl {
    type O = BlockAssignable<CognitoUserPoolVerificationMessageTemplateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolVerificationMessageTemplateEl {}

impl BuildCognitoUserPoolVerificationMessageTemplateEl {
    pub fn build(self) -> CognitoUserPoolVerificationMessageTemplateEl {
        CognitoUserPoolVerificationMessageTemplateEl {
            default_email_option: core::default::Default::default(),
            email_message: core::default::Default::default(),
            email_message_by_link: core::default::Default::default(),
            email_subject: core::default::Default::default(),
            email_subject_by_link: core::default::Default::default(),
            sms_message: core::default::Default::default(),
        }
    }
}

pub struct CognitoUserPoolVerificationMessageTemplateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolVerificationMessageTemplateElRef {
    fn new(shared: StackShared, base: String) -> CognitoUserPoolVerificationMessageTemplateElRef {
        CognitoUserPoolVerificationMessageTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolVerificationMessageTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_email_option` after provisioning.\n"]
    pub fn default_email_option(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_email_option", self.base))
    }

    #[doc = "Get a reference to the value of field `email_message` after provisioning.\n"]
    pub fn email_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_message", self.base))
    }

    #[doc = "Get a reference to the value of field `email_message_by_link` after provisioning.\n"]
    pub fn email_message_by_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_message_by_link", self.base))
    }

    #[doc = "Get a reference to the value of field `email_subject` after provisioning.\n"]
    pub fn email_subject(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_subject", self.base))
    }

    #[doc = "Get a reference to the value of field `email_subject_by_link` after provisioning.\n"]
    pub fn email_subject_by_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_subject_by_link", self.base))
    }

    #[doc = "Get a reference to the value of field `sms_message` after provisioning.\n"]
    pub fn sms_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sms_message", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolWebAuthnConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    relying_party_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_verification: Option<PrimField<String>>,
}

impl CognitoUserPoolWebAuthnConfigurationEl {
    #[doc = "Set the field `relying_party_id`.\n"]
    pub fn set_relying_party_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.relying_party_id = Some(v.into());
        self
    }

    #[doc = "Set the field `user_verification`.\n"]
    pub fn set_user_verification(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_verification = Some(v.into());
        self
    }
}

impl ToListMappable for CognitoUserPoolWebAuthnConfigurationEl {
    type O = BlockAssignable<CognitoUserPoolWebAuthnConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolWebAuthnConfigurationEl {}

impl BuildCognitoUserPoolWebAuthnConfigurationEl {
    pub fn build(self) -> CognitoUserPoolWebAuthnConfigurationEl {
        CognitoUserPoolWebAuthnConfigurationEl {
            relying_party_id: core::default::Default::default(),
            user_verification: core::default::Default::default(),
        }
    }
}

pub struct CognitoUserPoolWebAuthnConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolWebAuthnConfigurationElRef {
    fn new(shared: StackShared, base: String) -> CognitoUserPoolWebAuthnConfigurationElRef {
        CognitoUserPoolWebAuthnConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolWebAuthnConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `relying_party_id` after provisioning.\n"]
    pub fn relying_party_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.relying_party_id", self.base))
    }

    #[doc = "Get a reference to the value of field `user_verification` after provisioning.\n"]
    pub fn user_verification(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_verification", self.base))
    }
}

#[derive(Serialize, Default)]
struct CognitoUserPoolDynamic {
    account_recovery_setting: Option<DynamicBlock<CognitoUserPoolAccountRecoverySettingEl>>,
    admin_create_user_config: Option<DynamicBlock<CognitoUserPoolAdminCreateUserConfigEl>>,
    device_configuration: Option<DynamicBlock<CognitoUserPoolDeviceConfigurationEl>>,
    email_configuration: Option<DynamicBlock<CognitoUserPoolEmailConfigurationEl>>,
    email_mfa_configuration: Option<DynamicBlock<CognitoUserPoolEmailMfaConfigurationEl>>,
    lambda_config: Option<DynamicBlock<CognitoUserPoolLambdaConfigEl>>,
    password_policy: Option<DynamicBlock<CognitoUserPoolPasswordPolicyEl>>,
    schema: Option<DynamicBlock<CognitoUserPoolSchemaEl>>,
    sign_in_policy: Option<DynamicBlock<CognitoUserPoolSignInPolicyEl>>,
    sms_configuration: Option<DynamicBlock<CognitoUserPoolSmsConfigurationEl>>,
    software_token_mfa_configuration: Option<DynamicBlock<CognitoUserPoolSoftwareTokenMfaConfigurationEl>>,
    user_attribute_update_settings: Option<DynamicBlock<CognitoUserPoolUserAttributeUpdateSettingsEl>>,
    user_pool_add_ons: Option<DynamicBlock<CognitoUserPoolUserPoolAddOnsEl>>,
    username_configuration: Option<DynamicBlock<CognitoUserPoolUsernameConfigurationEl>>,
    verification_message_template: Option<DynamicBlock<CognitoUserPoolVerificationMessageTemplateEl>>,
    web_authn_configuration: Option<DynamicBlock<CognitoUserPoolWebAuthnConfigurationEl>>,
}
