use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCognitoUserPoolClientData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    client_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    user_pool_id: PrimField<String>,
}

struct DataCognitoUserPoolClient_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCognitoUserPoolClientData>,
}

#[derive(Clone)]
pub struct DataCognitoUserPoolClient(Rc<DataCognitoUserPoolClient_>);

impl DataCognitoUserPoolClient {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `access_token_validity` after provisioning.\n"]
    pub fn access_token_validity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_token_validity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allowed_oauth_flows` after provisioning.\n"]
    pub fn allowed_oauth_flows(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allowed_oauth_flows", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allowed_oauth_flows_user_pool_client` after provisioning.\n"]
    pub fn allowed_oauth_flows_user_pool_client(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allowed_oauth_flows_user_pool_client", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allowed_oauth_scopes` after provisioning.\n"]
    pub fn allowed_oauth_scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allowed_oauth_scopes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `analytics_configuration` after provisioning.\n"]
    pub fn analytics_configuration(&self) -> ListRef<DataCognitoUserPoolClientAnalyticsConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.analytics_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `callback_urls` after provisioning.\n"]
    pub fn callback_urls(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.callback_urls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_redirect_uri` after provisioning.\n"]
    pub fn default_redirect_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_redirect_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_propagate_additional_user_context_data` after provisioning.\n"]
    pub fn enable_propagate_additional_user_context_data(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_propagate_additional_user_context_data", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_token_revocation` after provisioning.\n"]
    pub fn enable_token_revocation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_token_revocation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `explicit_auth_flows` after provisioning.\n"]
    pub fn explicit_auth_flows(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.explicit_auth_flows", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `generate_secret` after provisioning.\n"]
    pub fn generate_secret(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.generate_secret", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id_token_validity` after provisioning.\n"]
    pub fn id_token_validity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id_token_validity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logout_urls` after provisioning.\n"]
    pub fn logout_urls(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.logout_urls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `prevent_user_existence_errors` after provisioning.\n"]
    pub fn prevent_user_existence_errors(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prevent_user_existence_errors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `read_attributes` after provisioning.\n"]
    pub fn read_attributes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.read_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `refresh_token_rotation` after provisioning.\n"]
    pub fn refresh_token_rotation(&self) -> ListRef<DataCognitoUserPoolClientRefreshTokenRotationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.refresh_token_rotation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `refresh_token_validity` after provisioning.\n"]
    pub fn refresh_token_validity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.refresh_token_validity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_identity_providers` after provisioning.\n"]
    pub fn supported_identity_providers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.supported_identity_providers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `token_validity_units` after provisioning.\n"]
    pub fn token_validity_units(&self) -> ListRef<DataCognitoUserPoolClientTokenValidityUnitsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.token_validity_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `write_attributes` after provisioning.\n"]
    pub fn write_attributes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.write_attributes", self.extract_ref()))
    }
}

impl Referable for DataCognitoUserPoolClient {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataCognitoUserPoolClient { }

impl ToListMappable for DataCognitoUserPoolClient {
    type O = ListRef<DataCognitoUserPoolClientRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataCognitoUserPoolClient_ {
    fn extract_datasource_type(&self) -> String {
        "aws_cognito_user_pool_client".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCognitoUserPoolClient {
    pub tf_id: String,
    #[doc = ""]
    pub client_id: PrimField<String>,
    #[doc = ""]
    pub user_pool_id: PrimField<String>,
}

impl BuildDataCognitoUserPoolClient {
    pub fn build(self, stack: &mut Stack) -> DataCognitoUserPoolClient {
        let out = DataCognitoUserPoolClient(Rc::new(DataCognitoUserPoolClient_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCognitoUserPoolClientData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                client_id: self.client_id,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                user_pool_id: self.user_pool_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCognitoUserPoolClientRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCognitoUserPoolClientRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataCognitoUserPoolClientRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `access_token_validity` after provisioning.\n"]
    pub fn access_token_validity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_token_validity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allowed_oauth_flows` after provisioning.\n"]
    pub fn allowed_oauth_flows(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allowed_oauth_flows", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allowed_oauth_flows_user_pool_client` after provisioning.\n"]
    pub fn allowed_oauth_flows_user_pool_client(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allowed_oauth_flows_user_pool_client", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allowed_oauth_scopes` after provisioning.\n"]
    pub fn allowed_oauth_scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allowed_oauth_scopes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `analytics_configuration` after provisioning.\n"]
    pub fn analytics_configuration(&self) -> ListRef<DataCognitoUserPoolClientAnalyticsConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.analytics_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `callback_urls` after provisioning.\n"]
    pub fn callback_urls(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.callback_urls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_redirect_uri` after provisioning.\n"]
    pub fn default_redirect_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_redirect_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_propagate_additional_user_context_data` after provisioning.\n"]
    pub fn enable_propagate_additional_user_context_data(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_propagate_additional_user_context_data", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_token_revocation` after provisioning.\n"]
    pub fn enable_token_revocation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_token_revocation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `explicit_auth_flows` after provisioning.\n"]
    pub fn explicit_auth_flows(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.explicit_auth_flows", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `generate_secret` after provisioning.\n"]
    pub fn generate_secret(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.generate_secret", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id_token_validity` after provisioning.\n"]
    pub fn id_token_validity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id_token_validity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logout_urls` after provisioning.\n"]
    pub fn logout_urls(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.logout_urls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `prevent_user_existence_errors` after provisioning.\n"]
    pub fn prevent_user_existence_errors(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prevent_user_existence_errors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `read_attributes` after provisioning.\n"]
    pub fn read_attributes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.read_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `refresh_token_rotation` after provisioning.\n"]
    pub fn refresh_token_rotation(&self) -> ListRef<DataCognitoUserPoolClientRefreshTokenRotationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.refresh_token_rotation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `refresh_token_validity` after provisioning.\n"]
    pub fn refresh_token_validity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.refresh_token_validity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_identity_providers` after provisioning.\n"]
    pub fn supported_identity_providers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.supported_identity_providers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `token_validity_units` after provisioning.\n"]
    pub fn token_validity_units(&self) -> ListRef<DataCognitoUserPoolClientTokenValidityUnitsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.token_validity_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `write_attributes` after provisioning.\n"]
    pub fn write_attributes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.write_attributes", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCognitoUserPoolClientAnalyticsConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    application_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_data_shared: Option<PrimField<bool>>,
}

impl DataCognitoUserPoolClientAnalyticsConfigurationEl {
    #[doc = "Set the field `application_arn`.\n"]
    pub fn set_application_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.application_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `application_id`.\n"]
    pub fn set_application_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.application_id = Some(v.into());
        self
    }

    #[doc = "Set the field `external_id`.\n"]
    pub fn set_external_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.external_id = Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `user_data_shared`.\n"]
    pub fn set_user_data_shared(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.user_data_shared = Some(v.into());
        self
    }
}

impl ToListMappable for DataCognitoUserPoolClientAnalyticsConfigurationEl {
    type O = BlockAssignable<DataCognitoUserPoolClientAnalyticsConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPoolClientAnalyticsConfigurationEl {}

impl BuildDataCognitoUserPoolClientAnalyticsConfigurationEl {
    pub fn build(self) -> DataCognitoUserPoolClientAnalyticsConfigurationEl {
        DataCognitoUserPoolClientAnalyticsConfigurationEl {
            application_arn: core::default::Default::default(),
            application_id: core::default::Default::default(),
            external_id: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            user_data_shared: core::default::Default::default(),
        }
    }
}

pub struct DataCognitoUserPoolClientAnalyticsConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCognitoUserPoolClientAnalyticsConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataCognitoUserPoolClientAnalyticsConfigurationElRef {
        DataCognitoUserPoolClientAnalyticsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPoolClientAnalyticsConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_arn` after provisioning.\n"]
    pub fn application_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `application_id` after provisioning.\n"]
    pub fn application_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_id", self.base))
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `user_data_shared` after provisioning.\n"]
    pub fn user_data_shared(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_data_shared", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCognitoUserPoolClientRefreshTokenRotationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    feature: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_grace_period_seconds: Option<PrimField<f64>>,
}

impl DataCognitoUserPoolClientRefreshTokenRotationEl {
    #[doc = "Set the field `feature`.\n"]
    pub fn set_feature(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.feature = Some(v.into());
        self
    }

    #[doc = "Set the field `retry_grace_period_seconds`.\n"]
    pub fn set_retry_grace_period_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.retry_grace_period_seconds = Some(v.into());
        self
    }
}

impl ToListMappable for DataCognitoUserPoolClientRefreshTokenRotationEl {
    type O = BlockAssignable<DataCognitoUserPoolClientRefreshTokenRotationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPoolClientRefreshTokenRotationEl {}

impl BuildDataCognitoUserPoolClientRefreshTokenRotationEl {
    pub fn build(self) -> DataCognitoUserPoolClientRefreshTokenRotationEl {
        DataCognitoUserPoolClientRefreshTokenRotationEl {
            feature: core::default::Default::default(),
            retry_grace_period_seconds: core::default::Default::default(),
        }
    }
}

pub struct DataCognitoUserPoolClientRefreshTokenRotationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCognitoUserPoolClientRefreshTokenRotationElRef {
    fn new(shared: StackShared, base: String) -> DataCognitoUserPoolClientRefreshTokenRotationElRef {
        DataCognitoUserPoolClientRefreshTokenRotationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPoolClientRefreshTokenRotationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `feature` after provisioning.\n"]
    pub fn feature(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.feature", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_grace_period_seconds` after provisioning.\n"]
    pub fn retry_grace_period_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retry_grace_period_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCognitoUserPoolClientTokenValidityUnitsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token: Option<PrimField<String>>,
}

impl DataCognitoUserPoolClientTokenValidityUnitsEl {
    #[doc = "Set the field `access_token`.\n"]
    pub fn set_access_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_token = Some(v.into());
        self
    }

    #[doc = "Set the field `id_token`.\n"]
    pub fn set_id_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id_token = Some(v.into());
        self
    }

    #[doc = "Set the field `refresh_token`.\n"]
    pub fn set_refresh_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.refresh_token = Some(v.into());
        self
    }
}

impl ToListMappable for DataCognitoUserPoolClientTokenValidityUnitsEl {
    type O = BlockAssignable<DataCognitoUserPoolClientTokenValidityUnitsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPoolClientTokenValidityUnitsEl {}

impl BuildDataCognitoUserPoolClientTokenValidityUnitsEl {
    pub fn build(self) -> DataCognitoUserPoolClientTokenValidityUnitsEl {
        DataCognitoUserPoolClientTokenValidityUnitsEl {
            access_token: core::default::Default::default(),
            id_token: core::default::Default::default(),
            refresh_token: core::default::Default::default(),
        }
    }
}

pub struct DataCognitoUserPoolClientTokenValidityUnitsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCognitoUserPoolClientTokenValidityUnitsElRef {
    fn new(shared: StackShared, base: String) -> DataCognitoUserPoolClientTokenValidityUnitsElRef {
        DataCognitoUserPoolClientTokenValidityUnitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPoolClientTokenValidityUnitsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_token` after provisioning.\n"]
    pub fn access_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_token", self.base))
    }

    #[doc = "Get a reference to the value of field `id_token` after provisioning.\n"]
    pub fn id_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id_token", self.base))
    }

    #[doc = "Get a reference to the value of field `refresh_token` after provisioning.\n"]
    pub fn refresh_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.refresh_token", self.base))
    }
}
