use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CognitoUserPoolClientData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_token_validity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_oauth_flows: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_oauth_flows_user_pool_client: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_oauth_scopes: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_session_validity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    callback_urls: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_redirect_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_propagate_additional_user_context_data: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_token_revocation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explicit_auth_flows: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generate_secret: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id_token_validity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logout_urls: Option<SetField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prevent_user_existence_errors: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_attributes: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token_validity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supported_identity_providers: Option<SetField<PrimField<String>>>,
    user_pool_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    write_attributes: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    analytics_configuration: Option<Vec<CognitoUserPoolClientAnalyticsConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token_rotation: Option<Vec<CognitoUserPoolClientRefreshTokenRotationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_validity_units: Option<Vec<CognitoUserPoolClientTokenValidityUnitsEl>>,
    dynamic: CognitoUserPoolClientDynamic,
}

struct CognitoUserPoolClient_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CognitoUserPoolClientData>,
}

#[derive(Clone)]
pub struct CognitoUserPoolClient(Rc<CognitoUserPoolClient_>);

impl CognitoUserPoolClient {
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

    #[doc = "Set the field `access_token_validity`.\n"]
    pub fn set_access_token_validity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().access_token_validity = Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_oauth_flows`.\n"]
    pub fn set_allowed_oauth_flows(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().allowed_oauth_flows = Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_oauth_flows_user_pool_client`.\n"]
    pub fn set_allowed_oauth_flows_user_pool_client(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allowed_oauth_flows_user_pool_client = Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_oauth_scopes`.\n"]
    pub fn set_allowed_oauth_scopes(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().allowed_oauth_scopes = Some(v.into());
        self
    }

    #[doc = "Set the field `auth_session_validity`.\n"]
    pub fn set_auth_session_validity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().auth_session_validity = Some(v.into());
        self
    }

    #[doc = "Set the field `callback_urls`.\n"]
    pub fn set_callback_urls(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().callback_urls = Some(v.into());
        self
    }

    #[doc = "Set the field `default_redirect_uri`.\n"]
    pub fn set_default_redirect_uri(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_redirect_uri = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_propagate_additional_user_context_data`.\n"]
    pub fn set_enable_propagate_additional_user_context_data(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_propagate_additional_user_context_data = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_token_revocation`.\n"]
    pub fn set_enable_token_revocation(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_token_revocation = Some(v.into());
        self
    }

    #[doc = "Set the field `explicit_auth_flows`.\n"]
    pub fn set_explicit_auth_flows(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().explicit_auth_flows = Some(v.into());
        self
    }

    #[doc = "Set the field `generate_secret`.\n"]
    pub fn set_generate_secret(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().generate_secret = Some(v.into());
        self
    }

    #[doc = "Set the field `id_token_validity`.\n"]
    pub fn set_id_token_validity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().id_token_validity = Some(v.into());
        self
    }

    #[doc = "Set the field `logout_urls`.\n"]
    pub fn set_logout_urls(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().logout_urls = Some(v.into());
        self
    }

    #[doc = "Set the field `prevent_user_existence_errors`.\n"]
    pub fn set_prevent_user_existence_errors(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().prevent_user_existence_errors = Some(v.into());
        self
    }

    #[doc = "Set the field `read_attributes`.\n"]
    pub fn set_read_attributes(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().read_attributes = Some(v.into());
        self
    }

    #[doc = "Set the field `refresh_token_validity`.\n"]
    pub fn set_refresh_token_validity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().refresh_token_validity = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `supported_identity_providers`.\n"]
    pub fn set_supported_identity_providers(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().supported_identity_providers = Some(v.into());
        self
    }

    #[doc = "Set the field `write_attributes`.\n"]
    pub fn set_write_attributes(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().write_attributes = Some(v.into());
        self
    }

    #[doc = "Set the field `analytics_configuration`.\n"]
    pub fn set_analytics_configuration(
        self,
        v: impl Into<BlockAssignable<CognitoUserPoolClientAnalyticsConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().analytics_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.analytics_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `refresh_token_rotation`.\n"]
    pub fn set_refresh_token_rotation(
        self,
        v: impl Into<BlockAssignable<CognitoUserPoolClientRefreshTokenRotationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().refresh_token_rotation = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.refresh_token_rotation = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `token_validity_units`.\n"]
    pub fn set_token_validity_units(
        self,
        v: impl Into<BlockAssignable<CognitoUserPoolClientTokenValidityUnitsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().token_validity_units = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.token_validity_units = Some(d);
            },
        }
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

    #[doc = "Get a reference to the value of field `auth_session_validity` after provisioning.\n"]
    pub fn auth_session_validity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_session_validity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `callback_urls` after provisioning.\n"]
    pub fn callback_urls(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.callback_urls", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `write_attributes` after provisioning.\n"]
    pub fn write_attributes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.write_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `analytics_configuration` after provisioning.\n"]
    pub fn analytics_configuration(&self) -> ListRef<CognitoUserPoolClientAnalyticsConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.analytics_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `refresh_token_rotation` after provisioning.\n"]
    pub fn refresh_token_rotation(&self) -> ListRef<CognitoUserPoolClientRefreshTokenRotationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.refresh_token_rotation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `token_validity_units` after provisioning.\n"]
    pub fn token_validity_units(&self) -> ListRef<CognitoUserPoolClientTokenValidityUnitsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.token_validity_units", self.extract_ref()))
    }
}

impl Referable for CognitoUserPoolClient {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for CognitoUserPoolClient { }

impl ToListMappable for CognitoUserPoolClient {
    type O = ListRef<CognitoUserPoolClientRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CognitoUserPoolClient_ {
    fn extract_resource_type(&self) -> String {
        "aws_cognito_user_pool_client".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCognitoUserPoolClient {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub user_pool_id: PrimField<String>,
}

impl BuildCognitoUserPoolClient {
    pub fn build(self, stack: &mut Stack) -> CognitoUserPoolClient {
        let out = CognitoUserPoolClient(Rc::new(CognitoUserPoolClient_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CognitoUserPoolClientData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                access_token_validity: core::default::Default::default(),
                allowed_oauth_flows: core::default::Default::default(),
                allowed_oauth_flows_user_pool_client: core::default::Default::default(),
                allowed_oauth_scopes: core::default::Default::default(),
                auth_session_validity: core::default::Default::default(),
                callback_urls: core::default::Default::default(),
                default_redirect_uri: core::default::Default::default(),
                enable_propagate_additional_user_context_data: core::default::Default::default(),
                enable_token_revocation: core::default::Default::default(),
                explicit_auth_flows: core::default::Default::default(),
                generate_secret: core::default::Default::default(),
                id_token_validity: core::default::Default::default(),
                logout_urls: core::default::Default::default(),
                name: self.name,
                prevent_user_existence_errors: core::default::Default::default(),
                read_attributes: core::default::Default::default(),
                refresh_token_validity: core::default::Default::default(),
                region: core::default::Default::default(),
                supported_identity_providers: core::default::Default::default(),
                user_pool_id: self.user_pool_id,
                write_attributes: core::default::Default::default(),
                analytics_configuration: core::default::Default::default(),
                refresh_token_rotation: core::default::Default::default(),
                token_validity_units: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CognitoUserPoolClientRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolClientRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl CognitoUserPoolClientRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `auth_session_validity` after provisioning.\n"]
    pub fn auth_session_validity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_session_validity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `callback_urls` after provisioning.\n"]
    pub fn callback_urls(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.callback_urls", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `write_attributes` after provisioning.\n"]
    pub fn write_attributes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.write_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `analytics_configuration` after provisioning.\n"]
    pub fn analytics_configuration(&self) -> ListRef<CognitoUserPoolClientAnalyticsConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.analytics_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `refresh_token_rotation` after provisioning.\n"]
    pub fn refresh_token_rotation(&self) -> ListRef<CognitoUserPoolClientRefreshTokenRotationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.refresh_token_rotation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `token_validity_units` after provisioning.\n"]
    pub fn token_validity_units(&self) -> ListRef<CognitoUserPoolClientTokenValidityUnitsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.token_validity_units", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolClientAnalyticsConfigurationEl {
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

impl CognitoUserPoolClientAnalyticsConfigurationEl {
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

impl ToListMappable for CognitoUserPoolClientAnalyticsConfigurationEl {
    type O = BlockAssignable<CognitoUserPoolClientAnalyticsConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolClientAnalyticsConfigurationEl {}

impl BuildCognitoUserPoolClientAnalyticsConfigurationEl {
    pub fn build(self) -> CognitoUserPoolClientAnalyticsConfigurationEl {
        CognitoUserPoolClientAnalyticsConfigurationEl {
            application_arn: core::default::Default::default(),
            application_id: core::default::Default::default(),
            external_id: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            user_data_shared: core::default::Default::default(),
        }
    }
}

pub struct CognitoUserPoolClientAnalyticsConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolClientAnalyticsConfigurationElRef {
    fn new(shared: StackShared, base: String) -> CognitoUserPoolClientAnalyticsConfigurationElRef {
        CognitoUserPoolClientAnalyticsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolClientAnalyticsConfigurationElRef {
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
pub struct CognitoUserPoolClientRefreshTokenRotationEl {
    feature: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_grace_period_seconds: Option<PrimField<f64>>,
}

impl CognitoUserPoolClientRefreshTokenRotationEl {
    #[doc = "Set the field `retry_grace_period_seconds`.\n"]
    pub fn set_retry_grace_period_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.retry_grace_period_seconds = Some(v.into());
        self
    }
}

impl ToListMappable for CognitoUserPoolClientRefreshTokenRotationEl {
    type O = BlockAssignable<CognitoUserPoolClientRefreshTokenRotationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolClientRefreshTokenRotationEl {
    #[doc = ""]
    pub feature: PrimField<String>,
}

impl BuildCognitoUserPoolClientRefreshTokenRotationEl {
    pub fn build(self) -> CognitoUserPoolClientRefreshTokenRotationEl {
        CognitoUserPoolClientRefreshTokenRotationEl {
            feature: self.feature,
            retry_grace_period_seconds: core::default::Default::default(),
        }
    }
}

pub struct CognitoUserPoolClientRefreshTokenRotationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolClientRefreshTokenRotationElRef {
    fn new(shared: StackShared, base: String) -> CognitoUserPoolClientRefreshTokenRotationElRef {
        CognitoUserPoolClientRefreshTokenRotationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolClientRefreshTokenRotationElRef {
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
pub struct CognitoUserPoolClientTokenValidityUnitsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token: Option<PrimField<String>>,
}

impl CognitoUserPoolClientTokenValidityUnitsEl {
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

impl ToListMappable for CognitoUserPoolClientTokenValidityUnitsEl {
    type O = BlockAssignable<CognitoUserPoolClientTokenValidityUnitsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolClientTokenValidityUnitsEl {}

impl BuildCognitoUserPoolClientTokenValidityUnitsEl {
    pub fn build(self) -> CognitoUserPoolClientTokenValidityUnitsEl {
        CognitoUserPoolClientTokenValidityUnitsEl {
            access_token: core::default::Default::default(),
            id_token: core::default::Default::default(),
            refresh_token: core::default::Default::default(),
        }
    }
}

pub struct CognitoUserPoolClientTokenValidityUnitsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoUserPoolClientTokenValidityUnitsElRef {
    fn new(shared: StackShared, base: String) -> CognitoUserPoolClientTokenValidityUnitsElRef {
        CognitoUserPoolClientTokenValidityUnitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolClientTokenValidityUnitsElRef {
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

#[derive(Serialize, Default)]
struct CognitoUserPoolClientDynamic {
    analytics_configuration: Option<DynamicBlock<CognitoUserPoolClientAnalyticsConfigurationEl>>,
    refresh_token_rotation: Option<DynamicBlock<CognitoUserPoolClientRefreshTokenRotationEl>>,
    token_validity_units: Option<DynamicBlock<CognitoUserPoolClientTokenValidityUnitsEl>>,
}
