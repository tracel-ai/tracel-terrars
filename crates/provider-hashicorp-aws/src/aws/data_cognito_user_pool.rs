use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataCognitoUserPoolData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    user_pool_id: PrimField<String>,
}

struct DataCognitoUserPool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCognitoUserPoolData>,
}

#[derive(Clone)]
pub struct DataCognitoUserPool(Rc<DataCognitoUserPool_>);

impl DataCognitoUserPool {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_recovery_setting` after provisioning.\n"]
    pub fn account_recovery_setting(
        &self,
    ) -> ListRef<DataCognitoUserPoolAccountRecoverySettingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.account_recovery_setting", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `admin_create_user_config` after provisioning.\n"]
    pub fn admin_create_user_config(
        &self,
    ) -> ListRef<DataCognitoUserPoolAdminCreateUserConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.admin_create_user_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_verified_attributes` after provisioning.\n"]
    pub fn auto_verified_attributes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_verified_attributes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `creation_date` after provisioning.\n"]
    pub fn creation_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_date", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `custom_domain` after provisioning.\n"]
    pub fn custom_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_domain", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `device_configuration` after provisioning.\n"]
    pub fn device_configuration(&self) -> ListRef<DataCognitoUserPoolDeviceConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.device_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `email_configuration` after provisioning.\n"]
    pub fn email_configuration(&self) -> ListRef<DataCognitoUserPoolEmailConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.email_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `estimated_number_of_users` after provisioning.\n"]
    pub fn estimated_number_of_users(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.estimated_number_of_users", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lambda_config` after provisioning.\n"]
    pub fn lambda_config(&self) -> ListRef<DataCognitoUserPoolLambdaConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lambda_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_modified_date` after provisioning.\n"]
    pub fn last_modified_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified_date", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `mfa_configuration` after provisioning.\n"]
    pub fn mfa_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mfa_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `schema_attributes` after provisioning.\n"]
    pub fn schema_attributes(&self) -> ListRef<DataCognitoUserPoolSchemaAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schema_attributes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sms_authentication_message` after provisioning.\n"]
    pub fn sms_authentication_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sms_authentication_message", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sms_configuration_failure` after provisioning.\n"]
    pub fn sms_configuration_failure(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sms_configuration_failure", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sms_verification_message` after provisioning.\n"]
    pub fn sms_verification_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sms_verification_message", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_pool_add_ons` after provisioning.\n"]
    pub fn user_pool_add_ons(&self) -> ListRef<DataCognitoUserPoolUserPoolAddOnsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_pool_add_ons", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_pool_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_pool_tags` after provisioning.\n"]
    pub fn user_pool_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.user_pool_tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `username_attributes` after provisioning.\n"]
    pub fn username_attributes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.username_attributes", self.extract_ref()),
        )
    }
}

impl Referable for DataCognitoUserPool {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataCognitoUserPool {}

impl ToListMappable for DataCognitoUserPool {
    type O = ListRef<DataCognitoUserPoolRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataCognitoUserPool_ {
    fn extract_datasource_type(&self) -> String {
        "aws_cognito_user_pool".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCognitoUserPool {
    pub tf_id: String,
    #[doc = ""]
    pub user_pool_id: PrimField<String>,
}

impl BuildDataCognitoUserPool {
    pub fn build(self, stack: &mut Stack) -> DataCognitoUserPool {
        let out = DataCognitoUserPool(Rc::new(DataCognitoUserPool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCognitoUserPoolData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                region: core::default::Default::default(),
                user_pool_id: self.user_pool_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCognitoUserPoolRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCognitoUserPoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataCognitoUserPoolRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_recovery_setting` after provisioning.\n"]
    pub fn account_recovery_setting(
        &self,
    ) -> ListRef<DataCognitoUserPoolAccountRecoverySettingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.account_recovery_setting", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `admin_create_user_config` after provisioning.\n"]
    pub fn admin_create_user_config(
        &self,
    ) -> ListRef<DataCognitoUserPoolAdminCreateUserConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.admin_create_user_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_verified_attributes` after provisioning.\n"]
    pub fn auto_verified_attributes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_verified_attributes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `creation_date` after provisioning.\n"]
    pub fn creation_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_date", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `custom_domain` after provisioning.\n"]
    pub fn custom_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_domain", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `device_configuration` after provisioning.\n"]
    pub fn device_configuration(&self) -> ListRef<DataCognitoUserPoolDeviceConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.device_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `email_configuration` after provisioning.\n"]
    pub fn email_configuration(&self) -> ListRef<DataCognitoUserPoolEmailConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.email_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `estimated_number_of_users` after provisioning.\n"]
    pub fn estimated_number_of_users(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.estimated_number_of_users", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lambda_config` after provisioning.\n"]
    pub fn lambda_config(&self) -> ListRef<DataCognitoUserPoolLambdaConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lambda_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_modified_date` after provisioning.\n"]
    pub fn last_modified_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified_date", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `mfa_configuration` after provisioning.\n"]
    pub fn mfa_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mfa_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `schema_attributes` after provisioning.\n"]
    pub fn schema_attributes(&self) -> ListRef<DataCognitoUserPoolSchemaAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schema_attributes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sms_authentication_message` after provisioning.\n"]
    pub fn sms_authentication_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sms_authentication_message", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sms_configuration_failure` after provisioning.\n"]
    pub fn sms_configuration_failure(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sms_configuration_failure", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sms_verification_message` after provisioning.\n"]
    pub fn sms_verification_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sms_verification_message", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_pool_add_ons` after provisioning.\n"]
    pub fn user_pool_add_ons(&self) -> ListRef<DataCognitoUserPoolUserPoolAddOnsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_pool_add_ons", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_pool_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_pool_tags` after provisioning.\n"]
    pub fn user_pool_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.user_pool_tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `username_attributes` after provisioning.\n"]
    pub fn username_attributes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.username_attributes", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataCognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
}

impl DataCognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.priority = Some(v.into());
        self
    }
}

impl ToListMappable for DataCognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl {
    type O = BlockAssignable<DataCognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl {}

impl BuildDataCognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl {
    pub fn build(self) -> DataCognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl {
        DataCognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl {
            name: core::default::Default::default(),
            priority: core::default::Default::default(),
        }
    }
}

pub struct DataCognitoUserPoolAccountRecoverySettingElRecoveryMechanismElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCognitoUserPoolAccountRecoverySettingElRecoveryMechanismElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCognitoUserPoolAccountRecoverySettingElRecoveryMechanismElRef {
        DataCognitoUserPoolAccountRecoverySettingElRecoveryMechanismElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPoolAccountRecoverySettingElRecoveryMechanismElRef {
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

#[derive(Serialize)]
pub struct DataCognitoUserPoolAccountRecoverySettingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    recovery_mechanism:
        Option<ListField<DataCognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl>>,
}

impl DataCognitoUserPoolAccountRecoverySettingEl {
    #[doc = "Set the field `recovery_mechanism`.\n"]
    pub fn set_recovery_mechanism(
        mut self,
        v: impl Into<ListField<DataCognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl>>,
    ) -> Self {
        self.recovery_mechanism = Some(v.into());
        self
    }
}

impl ToListMappable for DataCognitoUserPoolAccountRecoverySettingEl {
    type O = BlockAssignable<DataCognitoUserPoolAccountRecoverySettingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPoolAccountRecoverySettingEl {}

impl BuildDataCognitoUserPoolAccountRecoverySettingEl {
    pub fn build(self) -> DataCognitoUserPoolAccountRecoverySettingEl {
        DataCognitoUserPoolAccountRecoverySettingEl {
            recovery_mechanism: core::default::Default::default(),
        }
    }
}

pub struct DataCognitoUserPoolAccountRecoverySettingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCognitoUserPoolAccountRecoverySettingElRef {
    fn new(shared: StackShared, base: String) -> DataCognitoUserPoolAccountRecoverySettingElRef {
        DataCognitoUserPoolAccountRecoverySettingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPoolAccountRecoverySettingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `recovery_mechanism` after provisioning.\n"]
    pub fn recovery_mechanism(
        &self,
    ) -> ListRef<DataCognitoUserPoolAccountRecoverySettingElRecoveryMechanismElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.recovery_mechanism", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataCognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    email_message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_subject: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sms_message: Option<PrimField<String>>,
}

impl DataCognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl {
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

impl ToListMappable for DataCognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl {
    type O = BlockAssignable<DataCognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl {}

impl BuildDataCognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl {
    pub fn build(self) -> DataCognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl {
        DataCognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl {
            email_message: core::default::Default::default(),
            email_subject: core::default::Default::default(),
            sms_message: core::default::Default::default(),
        }
    }
}

pub struct DataCognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateElRef {
        DataCognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email_message` after provisioning.\n"]
    pub fn email_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.email_message", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_subject` after provisioning.\n"]
    pub fn email_subject(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.email_subject", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `sms_message` after provisioning.\n"]
    pub fn sms_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sms_message", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCognitoUserPoolAdminCreateUserConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_admin_create_user_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invite_message_template:
        Option<ListField<DataCognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unused_account_validity_days: Option<PrimField<f64>>,
}

impl DataCognitoUserPoolAdminCreateUserConfigEl {
    #[doc = "Set the field `allow_admin_create_user_only`.\n"]
    pub fn set_allow_admin_create_user_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_admin_create_user_only = Some(v.into());
        self
    }

    #[doc = "Set the field `invite_message_template`.\n"]
    pub fn set_invite_message_template(
        mut self,
        v: impl Into<ListField<DataCognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl>>,
    ) -> Self {
        self.invite_message_template = Some(v.into());
        self
    }

    #[doc = "Set the field `unused_account_validity_days`.\n"]
    pub fn set_unused_account_validity_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.unused_account_validity_days = Some(v.into());
        self
    }
}

impl ToListMappable for DataCognitoUserPoolAdminCreateUserConfigEl {
    type O = BlockAssignable<DataCognitoUserPoolAdminCreateUserConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPoolAdminCreateUserConfigEl {}

impl BuildDataCognitoUserPoolAdminCreateUserConfigEl {
    pub fn build(self) -> DataCognitoUserPoolAdminCreateUserConfigEl {
        DataCognitoUserPoolAdminCreateUserConfigEl {
            allow_admin_create_user_only: core::default::Default::default(),
            invite_message_template: core::default::Default::default(),
            unused_account_validity_days: core::default::Default::default(),
        }
    }
}

pub struct DataCognitoUserPoolAdminCreateUserConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCognitoUserPoolAdminCreateUserConfigElRef {
    fn new(shared: StackShared, base: String) -> DataCognitoUserPoolAdminCreateUserConfigElRef {
        DataCognitoUserPoolAdminCreateUserConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPoolAdminCreateUserConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_admin_create_user_only` after provisioning.\n"]
    pub fn allow_admin_create_user_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_admin_create_user_only", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `invite_message_template` after provisioning.\n"]
    pub fn invite_message_template(
        &self,
    ) -> ListRef<DataCognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.invite_message_template", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `unused_account_validity_days` after provisioning.\n"]
    pub fn unused_account_validity_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unused_account_validity_days", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataCognitoUserPoolDeviceConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    challenge_required_on_new_device: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_only_remembered_on_user_prompt: Option<PrimField<bool>>,
}

impl DataCognitoUserPoolDeviceConfigurationEl {
    #[doc = "Set the field `challenge_required_on_new_device`.\n"]
    pub fn set_challenge_required_on_new_device(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.challenge_required_on_new_device = Some(v.into());
        self
    }

    #[doc = "Set the field `device_only_remembered_on_user_prompt`.\n"]
    pub fn set_device_only_remembered_on_user_prompt(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.device_only_remembered_on_user_prompt = Some(v.into());
        self
    }
}

impl ToListMappable for DataCognitoUserPoolDeviceConfigurationEl {
    type O = BlockAssignable<DataCognitoUserPoolDeviceConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPoolDeviceConfigurationEl {}

impl BuildDataCognitoUserPoolDeviceConfigurationEl {
    pub fn build(self) -> DataCognitoUserPoolDeviceConfigurationEl {
        DataCognitoUserPoolDeviceConfigurationEl {
            challenge_required_on_new_device: core::default::Default::default(),
            device_only_remembered_on_user_prompt: core::default::Default::default(),
        }
    }
}

pub struct DataCognitoUserPoolDeviceConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCognitoUserPoolDeviceConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataCognitoUserPoolDeviceConfigurationElRef {
        DataCognitoUserPoolDeviceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPoolDeviceConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `challenge_required_on_new_device` after provisioning.\n"]
    pub fn challenge_required_on_new_device(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.challenge_required_on_new_device", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `device_only_remembered_on_user_prompt` after provisioning.\n"]
    pub fn device_only_remembered_on_user_prompt(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.device_only_remembered_on_user_prompt", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataCognitoUserPoolEmailConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration_set: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_sending_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_email_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_arn: Option<PrimField<String>>,
}

impl DataCognitoUserPoolEmailConfigurationEl {
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

    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.from = Some(v.into());
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

impl ToListMappable for DataCognitoUserPoolEmailConfigurationEl {
    type O = BlockAssignable<DataCognitoUserPoolEmailConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPoolEmailConfigurationEl {}

impl BuildDataCognitoUserPoolEmailConfigurationEl {
    pub fn build(self) -> DataCognitoUserPoolEmailConfigurationEl {
        DataCognitoUserPoolEmailConfigurationEl {
            configuration_set: core::default::Default::default(),
            email_sending_account: core::default::Default::default(),
            from: core::default::Default::default(),
            reply_to_email_address: core::default::Default::default(),
            source_arn: core::default::Default::default(),
        }
    }
}

pub struct DataCognitoUserPoolEmailConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCognitoUserPoolEmailConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataCognitoUserPoolEmailConfigurationElRef {
        DataCognitoUserPoolEmailConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPoolEmailConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configuration_set` after provisioning.\n"]
    pub fn configuration_set(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.configuration_set", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_sending_account` after provisioning.\n"]
    pub fn email_sending_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.email_sending_account", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `reply_to_email_address` after provisioning.\n"]
    pub fn reply_to_email_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reply_to_email_address", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `source_arn` after provisioning.\n"]
    pub fn source_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCognitoUserPoolLambdaConfigElCustomEmailSenderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_version: Option<PrimField<String>>,
}

impl DataCognitoUserPoolLambdaConfigElCustomEmailSenderEl {
    #[doc = "Set the field `lambda_arn`.\n"]
    pub fn set_lambda_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lambda_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `lambda_version`.\n"]
    pub fn set_lambda_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lambda_version = Some(v.into());
        self
    }
}

impl ToListMappable for DataCognitoUserPoolLambdaConfigElCustomEmailSenderEl {
    type O = BlockAssignable<DataCognitoUserPoolLambdaConfigElCustomEmailSenderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPoolLambdaConfigElCustomEmailSenderEl {}

impl BuildDataCognitoUserPoolLambdaConfigElCustomEmailSenderEl {
    pub fn build(self) -> DataCognitoUserPoolLambdaConfigElCustomEmailSenderEl {
        DataCognitoUserPoolLambdaConfigElCustomEmailSenderEl {
            lambda_arn: core::default::Default::default(),
            lambda_version: core::default::Default::default(),
        }
    }
}

pub struct DataCognitoUserPoolLambdaConfigElCustomEmailSenderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCognitoUserPoolLambdaConfigElCustomEmailSenderElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCognitoUserPoolLambdaConfigElCustomEmailSenderElRef {
        DataCognitoUserPoolLambdaConfigElCustomEmailSenderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPoolLambdaConfigElCustomEmailSenderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lambda_arn` after provisioning.\n"]
    pub fn lambda_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lambda_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_version` after provisioning.\n"]
    pub fn lambda_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lambda_version", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataCognitoUserPoolLambdaConfigElCustomSmsSenderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_version: Option<PrimField<String>>,
}

impl DataCognitoUserPoolLambdaConfigElCustomSmsSenderEl {
    #[doc = "Set the field `lambda_arn`.\n"]
    pub fn set_lambda_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lambda_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `lambda_version`.\n"]
    pub fn set_lambda_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lambda_version = Some(v.into());
        self
    }
}

impl ToListMappable for DataCognitoUserPoolLambdaConfigElCustomSmsSenderEl {
    type O = BlockAssignable<DataCognitoUserPoolLambdaConfigElCustomSmsSenderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPoolLambdaConfigElCustomSmsSenderEl {}

impl BuildDataCognitoUserPoolLambdaConfigElCustomSmsSenderEl {
    pub fn build(self) -> DataCognitoUserPoolLambdaConfigElCustomSmsSenderEl {
        DataCognitoUserPoolLambdaConfigElCustomSmsSenderEl {
            lambda_arn: core::default::Default::default(),
            lambda_version: core::default::Default::default(),
        }
    }
}

pub struct DataCognitoUserPoolLambdaConfigElCustomSmsSenderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCognitoUserPoolLambdaConfigElCustomSmsSenderElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCognitoUserPoolLambdaConfigElCustomSmsSenderElRef {
        DataCognitoUserPoolLambdaConfigElCustomSmsSenderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPoolLambdaConfigElCustomSmsSenderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lambda_arn` after provisioning.\n"]
    pub fn lambda_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lambda_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_version` after provisioning.\n"]
    pub fn lambda_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lambda_version", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataCognitoUserPoolLambdaConfigElPreTokenGenerationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_version: Option<PrimField<String>>,
}

impl DataCognitoUserPoolLambdaConfigElPreTokenGenerationConfigEl {
    #[doc = "Set the field `lambda_arn`.\n"]
    pub fn set_lambda_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lambda_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `lambda_version`.\n"]
    pub fn set_lambda_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lambda_version = Some(v.into());
        self
    }
}

impl ToListMappable for DataCognitoUserPoolLambdaConfigElPreTokenGenerationConfigEl {
    type O = BlockAssignable<DataCognitoUserPoolLambdaConfigElPreTokenGenerationConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPoolLambdaConfigElPreTokenGenerationConfigEl {}

impl BuildDataCognitoUserPoolLambdaConfigElPreTokenGenerationConfigEl {
    pub fn build(self) -> DataCognitoUserPoolLambdaConfigElPreTokenGenerationConfigEl {
        DataCognitoUserPoolLambdaConfigElPreTokenGenerationConfigEl {
            lambda_arn: core::default::Default::default(),
            lambda_version: core::default::Default::default(),
        }
    }
}

pub struct DataCognitoUserPoolLambdaConfigElPreTokenGenerationConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCognitoUserPoolLambdaConfigElPreTokenGenerationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCognitoUserPoolLambdaConfigElPreTokenGenerationConfigElRef {
        DataCognitoUserPoolLambdaConfigElPreTokenGenerationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPoolLambdaConfigElPreTokenGenerationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lambda_arn` after provisioning.\n"]
    pub fn lambda_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lambda_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_version` after provisioning.\n"]
    pub fn lambda_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lambda_version", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataCognitoUserPoolLambdaConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create_auth_challenge: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_email_sender: Option<ListField<DataCognitoUserPoolLambdaConfigElCustomEmailSenderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_sms_sender: Option<ListField<DataCognitoUserPoolLambdaConfigElCustomSmsSenderEl>>,
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
    pre_token_generation_config:
        Option<ListField<DataCognitoUserPoolLambdaConfigElPreTokenGenerationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_migration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verify_auth_challenge_response: Option<PrimField<String>>,
}

impl DataCognitoUserPoolLambdaConfigEl {
    #[doc = "Set the field `create_auth_challenge`.\n"]
    pub fn set_create_auth_challenge(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_auth_challenge = Some(v.into());
        self
    }

    #[doc = "Set the field `custom_email_sender`.\n"]
    pub fn set_custom_email_sender(
        mut self,
        v: impl Into<ListField<DataCognitoUserPoolLambdaConfigElCustomEmailSenderEl>>,
    ) -> Self {
        self.custom_email_sender = Some(v.into());
        self
    }

    #[doc = "Set the field `custom_message`.\n"]
    pub fn set_custom_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.custom_message = Some(v.into());
        self
    }

    #[doc = "Set the field `custom_sms_sender`.\n"]
    pub fn set_custom_sms_sender(
        mut self,
        v: impl Into<ListField<DataCognitoUserPoolLambdaConfigElCustomSmsSenderEl>>,
    ) -> Self {
        self.custom_sms_sender = Some(v.into());
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

    #[doc = "Set the field `pre_token_generation_config`.\n"]
    pub fn set_pre_token_generation_config(
        mut self,
        v: impl Into<ListField<DataCognitoUserPoolLambdaConfigElPreTokenGenerationConfigEl>>,
    ) -> Self {
        self.pre_token_generation_config = Some(v.into());
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
}

impl ToListMappable for DataCognitoUserPoolLambdaConfigEl {
    type O = BlockAssignable<DataCognitoUserPoolLambdaConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPoolLambdaConfigEl {}

impl BuildDataCognitoUserPoolLambdaConfigEl {
    pub fn build(self) -> DataCognitoUserPoolLambdaConfigEl {
        DataCognitoUserPoolLambdaConfigEl {
            create_auth_challenge: core::default::Default::default(),
            custom_email_sender: core::default::Default::default(),
            custom_message: core::default::Default::default(),
            custom_sms_sender: core::default::Default::default(),
            define_auth_challenge: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            post_authentication: core::default::Default::default(),
            post_confirmation: core::default::Default::default(),
            pre_authentication: core::default::Default::default(),
            pre_sign_up: core::default::Default::default(),
            pre_token_generation: core::default::Default::default(),
            pre_token_generation_config: core::default::Default::default(),
            user_migration: core::default::Default::default(),
            verify_auth_challenge_response: core::default::Default::default(),
        }
    }
}

pub struct DataCognitoUserPoolLambdaConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCognitoUserPoolLambdaConfigElRef {
    fn new(shared: StackShared, base: String) -> DataCognitoUserPoolLambdaConfigElRef {
        DataCognitoUserPoolLambdaConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPoolLambdaConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create_auth_challenge` after provisioning.\n"]
    pub fn create_auth_challenge(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_auth_challenge", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `custom_email_sender` after provisioning.\n"]
    pub fn custom_email_sender(
        &self,
    ) -> ListRef<DataCognitoUserPoolLambdaConfigElCustomEmailSenderElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_email_sender", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `custom_message` after provisioning.\n"]
    pub fn custom_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_message", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `custom_sms_sender` after provisioning.\n"]
    pub fn custom_sms_sender(
        &self,
    ) -> ListRef<DataCognitoUserPoolLambdaConfigElCustomSmsSenderElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_sms_sender", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `define_auth_challenge` after provisioning.\n"]
    pub fn define_auth_challenge(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.define_auth_challenge", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `post_authentication` after provisioning.\n"]
    pub fn post_authentication(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.post_authentication", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `post_confirmation` after provisioning.\n"]
    pub fn post_confirmation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.post_confirmation", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `pre_authentication` after provisioning.\n"]
    pub fn pre_authentication(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pre_authentication", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `pre_sign_up` after provisioning.\n"]
    pub fn pre_sign_up(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pre_sign_up", self.base))
    }

    #[doc = "Get a reference to the value of field `pre_token_generation` after provisioning.\n"]
    pub fn pre_token_generation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pre_token_generation", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `pre_token_generation_config` after provisioning.\n"]
    pub fn pre_token_generation_config(
        &self,
    ) -> ListRef<DataCognitoUserPoolLambdaConfigElPreTokenGenerationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pre_token_generation_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_migration` after provisioning.\n"]
    pub fn user_migration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_migration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `verify_auth_challenge_response` after provisioning.\n"]
    pub fn verify_auth_challenge_response(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.verify_auth_challenge_response", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataCognitoUserPoolSchemaAttributesElNumberAttributeConstraintsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_value: Option<PrimField<String>>,
}

impl DataCognitoUserPoolSchemaAttributesElNumberAttributeConstraintsEl {
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

impl ToListMappable for DataCognitoUserPoolSchemaAttributesElNumberAttributeConstraintsEl {
    type O = BlockAssignable<DataCognitoUserPoolSchemaAttributesElNumberAttributeConstraintsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPoolSchemaAttributesElNumberAttributeConstraintsEl {}

impl BuildDataCognitoUserPoolSchemaAttributesElNumberAttributeConstraintsEl {
    pub fn build(self) -> DataCognitoUserPoolSchemaAttributesElNumberAttributeConstraintsEl {
        DataCognitoUserPoolSchemaAttributesElNumberAttributeConstraintsEl {
            max_value: core::default::Default::default(),
            min_value: core::default::Default::default(),
        }
    }
}

pub struct DataCognitoUserPoolSchemaAttributesElNumberAttributeConstraintsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCognitoUserPoolSchemaAttributesElNumberAttributeConstraintsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCognitoUserPoolSchemaAttributesElNumberAttributeConstraintsElRef {
        DataCognitoUserPoolSchemaAttributesElNumberAttributeConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPoolSchemaAttributesElNumberAttributeConstraintsElRef {
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
pub struct DataCognitoUserPoolSchemaAttributesElStringAttributeConstraintsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_length: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_length: Option<PrimField<String>>,
}

impl DataCognitoUserPoolSchemaAttributesElStringAttributeConstraintsEl {
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

impl ToListMappable for DataCognitoUserPoolSchemaAttributesElStringAttributeConstraintsEl {
    type O = BlockAssignable<DataCognitoUserPoolSchemaAttributesElStringAttributeConstraintsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPoolSchemaAttributesElStringAttributeConstraintsEl {}

impl BuildDataCognitoUserPoolSchemaAttributesElStringAttributeConstraintsEl {
    pub fn build(self) -> DataCognitoUserPoolSchemaAttributesElStringAttributeConstraintsEl {
        DataCognitoUserPoolSchemaAttributesElStringAttributeConstraintsEl {
            max_length: core::default::Default::default(),
            min_length: core::default::Default::default(),
        }
    }
}

pub struct DataCognitoUserPoolSchemaAttributesElStringAttributeConstraintsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCognitoUserPoolSchemaAttributesElStringAttributeConstraintsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCognitoUserPoolSchemaAttributesElStringAttributeConstraintsElRef {
        DataCognitoUserPoolSchemaAttributesElStringAttributeConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPoolSchemaAttributesElStringAttributeConstraintsElRef {
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

#[derive(Serialize)]
pub struct DataCognitoUserPoolSchemaAttributesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute_data_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    developer_only_attribute: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mutable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number_attribute_constraints:
        Option<ListField<DataCognitoUserPoolSchemaAttributesElNumberAttributeConstraintsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_attribute_constraints:
        Option<ListField<DataCognitoUserPoolSchemaAttributesElStringAttributeConstraintsEl>>,
}

impl DataCognitoUserPoolSchemaAttributesEl {
    #[doc = "Set the field `attribute_data_type`.\n"]
    pub fn set_attribute_data_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attribute_data_type = Some(v.into());
        self
    }

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

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `number_attribute_constraints`.\n"]
    pub fn set_number_attribute_constraints(
        mut self,
        v: impl Into<ListField<DataCognitoUserPoolSchemaAttributesElNumberAttributeConstraintsEl>>,
    ) -> Self {
        self.number_attribute_constraints = Some(v.into());
        self
    }

    #[doc = "Set the field `required`.\n"]
    pub fn set_required(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.required = Some(v.into());
        self
    }

    #[doc = "Set the field `string_attribute_constraints`.\n"]
    pub fn set_string_attribute_constraints(
        mut self,
        v: impl Into<ListField<DataCognitoUserPoolSchemaAttributesElStringAttributeConstraintsEl>>,
    ) -> Self {
        self.string_attribute_constraints = Some(v.into());
        self
    }
}

impl ToListMappable for DataCognitoUserPoolSchemaAttributesEl {
    type O = BlockAssignable<DataCognitoUserPoolSchemaAttributesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPoolSchemaAttributesEl {}

impl BuildDataCognitoUserPoolSchemaAttributesEl {
    pub fn build(self) -> DataCognitoUserPoolSchemaAttributesEl {
        DataCognitoUserPoolSchemaAttributesEl {
            attribute_data_type: core::default::Default::default(),
            developer_only_attribute: core::default::Default::default(),
            mutable: core::default::Default::default(),
            name: core::default::Default::default(),
            number_attribute_constraints: core::default::Default::default(),
            required: core::default::Default::default(),
            string_attribute_constraints: core::default::Default::default(),
        }
    }
}

pub struct DataCognitoUserPoolSchemaAttributesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCognitoUserPoolSchemaAttributesElRef {
    fn new(shared: StackShared, base: String) -> DataCognitoUserPoolSchemaAttributesElRef {
        DataCognitoUserPoolSchemaAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPoolSchemaAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute_data_type` after provisioning.\n"]
    pub fn attribute_data_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.attribute_data_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `developer_only_attribute` after provisioning.\n"]
    pub fn developer_only_attribute(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.developer_only_attribute", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `mutable` after provisioning.\n"]
    pub fn mutable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.mutable", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `number_attribute_constraints` after provisioning.\n"]
    pub fn number_attribute_constraints(
        &self,
    ) -> ListRef<DataCognitoUserPoolSchemaAttributesElNumberAttributeConstraintsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.number_attribute_constraints", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `required` after provisioning.\n"]
    pub fn required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }

    #[doc = "Get a reference to the value of field `string_attribute_constraints` after provisioning.\n"]
    pub fn string_attribute_constraints(
        &self,
    ) -> ListRef<DataCognitoUserPoolSchemaAttributesElStringAttributeConstraintsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.string_attribute_constraints", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataCognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_auth_mode: Option<PrimField<String>>,
}

impl DataCognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsEl {
    #[doc = "Set the field `custom_auth_mode`.\n"]
    pub fn set_custom_auth_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.custom_auth_mode = Some(v.into());
        self
    }
}

impl ToListMappable for DataCognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsEl {
    type O = BlockAssignable<DataCognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsEl {}

impl BuildDataCognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsEl {
    pub fn build(self) -> DataCognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsEl {
        DataCognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsEl {
            custom_auth_mode: core::default::Default::default(),
        }
    }
}

pub struct DataCognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsElRef {
        DataCognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_auth_mode` after provisioning.\n"]
    pub fn custom_auth_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_auth_mode", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataCognitoUserPoolUserPoolAddOnsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_security_additional_flows:
        Option<ListField<DataCognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_security_mode: Option<PrimField<String>>,
}

impl DataCognitoUserPoolUserPoolAddOnsEl {
    #[doc = "Set the field `advanced_security_additional_flows`.\n"]
    pub fn set_advanced_security_additional_flows(
        mut self,
        v: impl Into<ListField<DataCognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsEl>>,
    ) -> Self {
        self.advanced_security_additional_flows = Some(v.into());
        self
    }

    #[doc = "Set the field `advanced_security_mode`.\n"]
    pub fn set_advanced_security_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.advanced_security_mode = Some(v.into());
        self
    }
}

impl ToListMappable for DataCognitoUserPoolUserPoolAddOnsEl {
    type O = BlockAssignable<DataCognitoUserPoolUserPoolAddOnsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPoolUserPoolAddOnsEl {}

impl BuildDataCognitoUserPoolUserPoolAddOnsEl {
    pub fn build(self) -> DataCognitoUserPoolUserPoolAddOnsEl {
        DataCognitoUserPoolUserPoolAddOnsEl {
            advanced_security_additional_flows: core::default::Default::default(),
            advanced_security_mode: core::default::Default::default(),
        }
    }
}

pub struct DataCognitoUserPoolUserPoolAddOnsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCognitoUserPoolUserPoolAddOnsElRef {
    fn new(shared: StackShared, base: String) -> DataCognitoUserPoolUserPoolAddOnsElRef {
        DataCognitoUserPoolUserPoolAddOnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPoolUserPoolAddOnsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `advanced_security_additional_flows` after provisioning.\n"]
    pub fn advanced_security_additional_flows(
        &self,
    ) -> ListRef<DataCognitoUserPoolUserPoolAddOnsElAdvancedSecurityAdditionalFlowsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_security_additional_flows", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `advanced_security_mode` after provisioning.\n"]
    pub fn advanced_security_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.advanced_security_mode", self.base),
        )
    }
}
