use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAlbListenerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancer_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataAlbListenerTimeoutsEl>,
}

struct DataAlbListener_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAlbListenerData>,
}

#[derive(Clone)]
pub struct DataAlbListener(Rc<DataAlbListener_>);

impl DataAlbListener {
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

    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `load_balancer_arn`.\n"]
    pub fn set_load_balancer_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().load_balancer_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataAlbListenerTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `alpn_policy` after provisioning.\n"]
    pub fn alpn_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alpn_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> ListRef<DataAlbListenerDefaultActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer_arn` after provisioning.\n"]
    pub fn load_balancer_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mutual_authentication` after provisioning.\n"]
    pub fn mutual_authentication(&self) -> ListRef<DataAlbListenerMutualAuthenticationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mutual_authentication", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssl_policy` after provisioning.\n"]
    pub fn ssl_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataAlbListenerTimeoutsElRef {
        DataAlbListenerTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for DataAlbListener {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataAlbListener { }

impl ToListMappable for DataAlbListener {
    type O = ListRef<DataAlbListenerRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataAlbListener_ {
    fn extract_datasource_type(&self) -> String {
        "aws_alb_listener".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAlbListener {
    pub tf_id: String,
}

impl BuildDataAlbListener {
    pub fn build(self, stack: &mut Stack) -> DataAlbListener {
        let out = DataAlbListener(Rc::new(DataAlbListener_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAlbListenerData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                arn: core::default::Default::default(),
                id: core::default::Default::default(),
                load_balancer_arn: core::default::Default::default(),
                port: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAlbListenerRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAlbListenerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataAlbListenerRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `alpn_policy` after provisioning.\n"]
    pub fn alpn_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alpn_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> ListRef<DataAlbListenerDefaultActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer_arn` after provisioning.\n"]
    pub fn load_balancer_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mutual_authentication` after provisioning.\n"]
    pub fn mutual_authentication(&self) -> ListRef<DataAlbListenerMutualAuthenticationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mutual_authentication", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssl_policy` after provisioning.\n"]
    pub fn ssl_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataAlbListenerTimeoutsElRef {
        DataAlbListenerTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAlbListenerDefaultActionElAuthenticateCognitoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_request_extra_params: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_unauthenticated_request: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_cookie_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_pool_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_pool_client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_pool_domain: Option<PrimField<String>>,
}

impl DataAlbListenerDefaultActionElAuthenticateCognitoEl {
    #[doc = "Set the field `authentication_request_extra_params`.\n"]
    pub fn set_authentication_request_extra_params(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.authentication_request_extra_params = Some(v.into());
        self
    }

    #[doc = "Set the field `on_unauthenticated_request`.\n"]
    pub fn set_on_unauthenticated_request(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.on_unauthenticated_request = Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }

    #[doc = "Set the field `session_cookie_name`.\n"]
    pub fn set_session_cookie_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.session_cookie_name = Some(v.into());
        self
    }

    #[doc = "Set the field `session_timeout`.\n"]
    pub fn set_session_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.session_timeout = Some(v.into());
        self
    }

    #[doc = "Set the field `user_pool_arn`.\n"]
    pub fn set_user_pool_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_pool_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `user_pool_client_id`.\n"]
    pub fn set_user_pool_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_pool_client_id = Some(v.into());
        self
    }

    #[doc = "Set the field `user_pool_domain`.\n"]
    pub fn set_user_pool_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_pool_domain = Some(v.into());
        self
    }
}

impl ToListMappable for DataAlbListenerDefaultActionElAuthenticateCognitoEl {
    type O = BlockAssignable<DataAlbListenerDefaultActionElAuthenticateCognitoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlbListenerDefaultActionElAuthenticateCognitoEl {}

impl BuildDataAlbListenerDefaultActionElAuthenticateCognitoEl {
    pub fn build(self) -> DataAlbListenerDefaultActionElAuthenticateCognitoEl {
        DataAlbListenerDefaultActionElAuthenticateCognitoEl {
            authentication_request_extra_params: core::default::Default::default(),
            on_unauthenticated_request: core::default::Default::default(),
            scope: core::default::Default::default(),
            session_cookie_name: core::default::Default::default(),
            session_timeout: core::default::Default::default(),
            user_pool_arn: core::default::Default::default(),
            user_pool_client_id: core::default::Default::default(),
            user_pool_domain: core::default::Default::default(),
        }
    }
}

pub struct DataAlbListenerDefaultActionElAuthenticateCognitoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAlbListenerDefaultActionElAuthenticateCognitoElRef {
    fn new(shared: StackShared, base: String) -> DataAlbListenerDefaultActionElAuthenticateCognitoElRef {
        DataAlbListenerDefaultActionElAuthenticateCognitoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlbListenerDefaultActionElAuthenticateCognitoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication_request_extra_params` after provisioning.\n"]
    pub fn authentication_request_extra_params(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.authentication_request_extra_params", self.base))
    }

    #[doc = "Get a reference to the value of field `on_unauthenticated_request` after provisioning.\n"]
    pub fn on_unauthenticated_request(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.on_unauthenticated_request", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `session_cookie_name` after provisioning.\n"]
    pub fn session_cookie_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_cookie_name", self.base))
    }

    #[doc = "Get a reference to the value of field `session_timeout` after provisioning.\n"]
    pub fn session_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `user_pool_arn` after provisioning.\n"]
    pub fn user_pool_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_pool_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `user_pool_client_id` after provisioning.\n"]
    pub fn user_pool_client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_pool_client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `user_pool_domain` after provisioning.\n"]
    pub fn user_pool_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_pool_domain", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlbListenerDefaultActionElAuthenticateOidcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_request_extra_params: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_unauthenticated_request: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_cookie_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_info_endpoint: Option<PrimField<String>>,
}

impl DataAlbListenerDefaultActionElAuthenticateOidcEl {
    #[doc = "Set the field `authentication_request_extra_params`.\n"]
    pub fn set_authentication_request_extra_params(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.authentication_request_extra_params = Some(v.into());
        self
    }

    #[doc = "Set the field `authorization_endpoint`.\n"]
    pub fn set_authorization_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.authorization_endpoint = Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id = Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\n"]
    pub fn set_client_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_secret = Some(v.into());
        self
    }

    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issuer = Some(v.into());
        self
    }

    #[doc = "Set the field `on_unauthenticated_request`.\n"]
    pub fn set_on_unauthenticated_request(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.on_unauthenticated_request = Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }

    #[doc = "Set the field `session_cookie_name`.\n"]
    pub fn set_session_cookie_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.session_cookie_name = Some(v.into());
        self
    }

    #[doc = "Set the field `session_timeout`.\n"]
    pub fn set_session_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.session_timeout = Some(v.into());
        self
    }

    #[doc = "Set the field `token_endpoint`.\n"]
    pub fn set_token_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token_endpoint = Some(v.into());
        self
    }

    #[doc = "Set the field `user_info_endpoint`.\n"]
    pub fn set_user_info_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_info_endpoint = Some(v.into());
        self
    }
}

impl ToListMappable for DataAlbListenerDefaultActionElAuthenticateOidcEl {
    type O = BlockAssignable<DataAlbListenerDefaultActionElAuthenticateOidcEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlbListenerDefaultActionElAuthenticateOidcEl {}

impl BuildDataAlbListenerDefaultActionElAuthenticateOidcEl {
    pub fn build(self) -> DataAlbListenerDefaultActionElAuthenticateOidcEl {
        DataAlbListenerDefaultActionElAuthenticateOidcEl {
            authentication_request_extra_params: core::default::Default::default(),
            authorization_endpoint: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            issuer: core::default::Default::default(),
            on_unauthenticated_request: core::default::Default::default(),
            scope: core::default::Default::default(),
            session_cookie_name: core::default::Default::default(),
            session_timeout: core::default::Default::default(),
            token_endpoint: core::default::Default::default(),
            user_info_endpoint: core::default::Default::default(),
        }
    }
}

pub struct DataAlbListenerDefaultActionElAuthenticateOidcElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAlbListenerDefaultActionElAuthenticateOidcElRef {
    fn new(shared: StackShared, base: String) -> DataAlbListenerDefaultActionElAuthenticateOidcElRef {
        DataAlbListenerDefaultActionElAuthenticateOidcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlbListenerDefaultActionElAuthenticateOidcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication_request_extra_params` after provisioning.\n"]
    pub fn authentication_request_extra_params(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.authentication_request_extra_params", self.base))
    }

    #[doc = "Get a reference to the value of field `authorization_endpoint` after provisioning.\n"]
    pub fn authorization_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorization_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `on_unauthenticated_request` after provisioning.\n"]
    pub fn on_unauthenticated_request(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.on_unauthenticated_request", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `session_cookie_name` after provisioning.\n"]
    pub fn session_cookie_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_cookie_name", self.base))
    }

    #[doc = "Get a reference to the value of field `session_timeout` after provisioning.\n"]
    pub fn session_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\n"]
    pub fn token_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `user_info_endpoint` after provisioning.\n"]
    pub fn user_info_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_info_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlbListenerDefaultActionElFixedResponseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_body: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code: Option<PrimField<String>>,
}

impl DataAlbListenerDefaultActionElFixedResponseEl {
    #[doc = "Set the field `content_type`.\n"]
    pub fn set_content_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content_type = Some(v.into());
        self
    }

    #[doc = "Set the field `message_body`.\n"]
    pub fn set_message_body(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_body = Some(v.into());
        self
    }

    #[doc = "Set the field `status_code`.\n"]
    pub fn set_status_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status_code = Some(v.into());
        self
    }
}

impl ToListMappable for DataAlbListenerDefaultActionElFixedResponseEl {
    type O = BlockAssignable<DataAlbListenerDefaultActionElFixedResponseEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlbListenerDefaultActionElFixedResponseEl {}

impl BuildDataAlbListenerDefaultActionElFixedResponseEl {
    pub fn build(self) -> DataAlbListenerDefaultActionElFixedResponseEl {
        DataAlbListenerDefaultActionElFixedResponseEl {
            content_type: core::default::Default::default(),
            message_body: core::default::Default::default(),
            status_code: core::default::Default::default(),
        }
    }
}

pub struct DataAlbListenerDefaultActionElFixedResponseElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAlbListenerDefaultActionElFixedResponseElRef {
    fn new(shared: StackShared, base: String) -> DataAlbListenerDefaultActionElFixedResponseElRef {
        DataAlbListenerDefaultActionElFixedResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlbListenerDefaultActionElFixedResponseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `message_body` after provisioning.\n"]
    pub fn message_body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message_body", self.base))
    }

    #[doc = "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlbListenerDefaultActionElForwardElStickinessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataAlbListenerDefaultActionElForwardElStickinessEl {
    #[doc = "Set the field `duration`.\n"]
    pub fn set_duration(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.duration = Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataAlbListenerDefaultActionElForwardElStickinessEl {
    type O = BlockAssignable<DataAlbListenerDefaultActionElForwardElStickinessEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlbListenerDefaultActionElForwardElStickinessEl {}

impl BuildDataAlbListenerDefaultActionElForwardElStickinessEl {
    pub fn build(self) -> DataAlbListenerDefaultActionElForwardElStickinessEl {
        DataAlbListenerDefaultActionElForwardElStickinessEl {
            duration: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}

pub struct DataAlbListenerDefaultActionElForwardElStickinessElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAlbListenerDefaultActionElForwardElStickinessElRef {
    fn new(shared: StackShared, base: String) -> DataAlbListenerDefaultActionElForwardElStickinessElRef {
        DataAlbListenerDefaultActionElForwardElStickinessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlbListenerDefaultActionElForwardElStickinessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlbListenerDefaultActionElForwardElTargetGroupEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}

impl DataAlbListenerDefaultActionElForwardElTargetGroupEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}

impl ToListMappable for DataAlbListenerDefaultActionElForwardElTargetGroupEl {
    type O = BlockAssignable<DataAlbListenerDefaultActionElForwardElTargetGroupEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlbListenerDefaultActionElForwardElTargetGroupEl {}

impl BuildDataAlbListenerDefaultActionElForwardElTargetGroupEl {
    pub fn build(self) -> DataAlbListenerDefaultActionElForwardElTargetGroupEl {
        DataAlbListenerDefaultActionElForwardElTargetGroupEl {
            arn: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}

pub struct DataAlbListenerDefaultActionElForwardElTargetGroupElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAlbListenerDefaultActionElForwardElTargetGroupElRef {
    fn new(shared: StackShared, base: String) -> DataAlbListenerDefaultActionElForwardElTargetGroupElRef {
        DataAlbListenerDefaultActionElForwardElTargetGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlbListenerDefaultActionElForwardElTargetGroupElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlbListenerDefaultActionElForwardEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    stickiness: Option<ListField<DataAlbListenerDefaultActionElForwardElStickinessEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_group: Option<SetField<DataAlbListenerDefaultActionElForwardElTargetGroupEl>>,
}

impl DataAlbListenerDefaultActionElForwardEl {
    #[doc = "Set the field `stickiness`.\n"]
    pub fn set_stickiness(
        mut self,
        v: impl Into<ListField<DataAlbListenerDefaultActionElForwardElStickinessEl>>,
    ) -> Self {
        self.stickiness = Some(v.into());
        self
    }

    #[doc = "Set the field `target_group`.\n"]
    pub fn set_target_group(
        mut self,
        v: impl Into<SetField<DataAlbListenerDefaultActionElForwardElTargetGroupEl>>,
    ) -> Self {
        self.target_group = Some(v.into());
        self
    }
}

impl ToListMappable for DataAlbListenerDefaultActionElForwardEl {
    type O = BlockAssignable<DataAlbListenerDefaultActionElForwardEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlbListenerDefaultActionElForwardEl {}

impl BuildDataAlbListenerDefaultActionElForwardEl {
    pub fn build(self) -> DataAlbListenerDefaultActionElForwardEl {
        DataAlbListenerDefaultActionElForwardEl {
            stickiness: core::default::Default::default(),
            target_group: core::default::Default::default(),
        }
    }
}

pub struct DataAlbListenerDefaultActionElForwardElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAlbListenerDefaultActionElForwardElRef {
    fn new(shared: StackShared, base: String) -> DataAlbListenerDefaultActionElForwardElRef {
        DataAlbListenerDefaultActionElForwardElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlbListenerDefaultActionElForwardElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stickiness` after provisioning.\n"]
    pub fn stickiness(&self) -> ListRef<DataAlbListenerDefaultActionElForwardElStickinessElRef> {
        ListRef::new(self.shared().clone(), format!("{}.stickiness", self.base))
    }

    #[doc = "Get a reference to the value of field `target_group` after provisioning.\n"]
    pub fn target_group(&self) -> SetRef<DataAlbListenerDefaultActionElForwardElTargetGroupElRef> {
        SetRef::new(self.shared().clone(), format!("{}.target_group", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlbListenerDefaultActionElJwtValidationElAdditionalClaimEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataAlbListenerDefaultActionElJwtValidationElAdditionalClaimEl {
    #[doc = "Set the field `format`.\n"]
    pub fn set_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.format = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataAlbListenerDefaultActionElJwtValidationElAdditionalClaimEl {
    type O = BlockAssignable<DataAlbListenerDefaultActionElJwtValidationElAdditionalClaimEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlbListenerDefaultActionElJwtValidationElAdditionalClaimEl {}

impl BuildDataAlbListenerDefaultActionElJwtValidationElAdditionalClaimEl {
    pub fn build(self) -> DataAlbListenerDefaultActionElJwtValidationElAdditionalClaimEl {
        DataAlbListenerDefaultActionElJwtValidationElAdditionalClaimEl {
            format: core::default::Default::default(),
            name: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataAlbListenerDefaultActionElJwtValidationElAdditionalClaimElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAlbListenerDefaultActionElJwtValidationElAdditionalClaimElRef {
    fn new(shared: StackShared, base: String) -> DataAlbListenerDefaultActionElJwtValidationElAdditionalClaimElRef {
        DataAlbListenerDefaultActionElJwtValidationElAdditionalClaimElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlbListenerDefaultActionElJwtValidationElAdditionalClaimElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlbListenerDefaultActionElJwtValidationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_claim: Option<SetField<DataAlbListenerDefaultActionElJwtValidationElAdditionalClaimEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jwks_endpoint: Option<PrimField<String>>,
}

impl DataAlbListenerDefaultActionElJwtValidationEl {
    #[doc = "Set the field `additional_claim`.\n"]
    pub fn set_additional_claim(
        mut self,
        v: impl Into<SetField<DataAlbListenerDefaultActionElJwtValidationElAdditionalClaimEl>>,
    ) -> Self {
        self.additional_claim = Some(v.into());
        self
    }

    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issuer = Some(v.into());
        self
    }

    #[doc = "Set the field `jwks_endpoint`.\n"]
    pub fn set_jwks_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.jwks_endpoint = Some(v.into());
        self
    }
}

impl ToListMappable for DataAlbListenerDefaultActionElJwtValidationEl {
    type O = BlockAssignable<DataAlbListenerDefaultActionElJwtValidationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlbListenerDefaultActionElJwtValidationEl {}

impl BuildDataAlbListenerDefaultActionElJwtValidationEl {
    pub fn build(self) -> DataAlbListenerDefaultActionElJwtValidationEl {
        DataAlbListenerDefaultActionElJwtValidationEl {
            additional_claim: core::default::Default::default(),
            issuer: core::default::Default::default(),
            jwks_endpoint: core::default::Default::default(),
        }
    }
}

pub struct DataAlbListenerDefaultActionElJwtValidationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAlbListenerDefaultActionElJwtValidationElRef {
    fn new(shared: StackShared, base: String) -> DataAlbListenerDefaultActionElJwtValidationElRef {
        DataAlbListenerDefaultActionElJwtValidationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlbListenerDefaultActionElJwtValidationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_claim` after provisioning.\n"]
    pub fn additional_claim(&self) -> SetRef<DataAlbListenerDefaultActionElJwtValidationElAdditionalClaimElRef> {
        SetRef::new(self.shared().clone(), format!("{}.additional_claim", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `jwks_endpoint` after provisioning.\n"]
    pub fn jwks_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.jwks_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlbListenerDefaultActionElRedirectEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code: Option<PrimField<String>>,
}

impl DataAlbListenerDefaultActionElRedirectEl {
    #[doc = "Set the field `host`.\n"]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port = Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }

    #[doc = "Set the field `query`.\n"]
    pub fn set_query(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.query = Some(v.into());
        self
    }

    #[doc = "Set the field `status_code`.\n"]
    pub fn set_status_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status_code = Some(v.into());
        self
    }
}

impl ToListMappable for DataAlbListenerDefaultActionElRedirectEl {
    type O = BlockAssignable<DataAlbListenerDefaultActionElRedirectEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlbListenerDefaultActionElRedirectEl {}

impl BuildDataAlbListenerDefaultActionElRedirectEl {
    pub fn build(self) -> DataAlbListenerDefaultActionElRedirectEl {
        DataAlbListenerDefaultActionElRedirectEl {
            host: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
            query: core::default::Default::default(),
            status_code: core::default::Default::default(),
        }
    }
}

pub struct DataAlbListenerDefaultActionElRedirectElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAlbListenerDefaultActionElRedirectElRef {
    fn new(shared: StackShared, base: String) -> DataAlbListenerDefaultActionElRedirectElRef {
        DataAlbListenerDefaultActionElRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlbListenerDefaultActionElRedirectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }

    #[doc = "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlbListenerDefaultActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authenticate_cognito: Option<ListField<DataAlbListenerDefaultActionElAuthenticateCognitoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authenticate_oidc: Option<ListField<DataAlbListenerDefaultActionElAuthenticateOidcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_response: Option<ListField<DataAlbListenerDefaultActionElFixedResponseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forward: Option<ListField<DataAlbListenerDefaultActionElForwardEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jwt_validation: Option<ListField<DataAlbListenerDefaultActionElJwtValidationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect: Option<ListField<DataAlbListenerDefaultActionElRedirectEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_group_arn: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataAlbListenerDefaultActionEl {
    #[doc = "Set the field `authenticate_cognito`.\n"]
    pub fn set_authenticate_cognito(
        mut self,
        v: impl Into<ListField<DataAlbListenerDefaultActionElAuthenticateCognitoEl>>,
    ) -> Self {
        self.authenticate_cognito = Some(v.into());
        self
    }

    #[doc = "Set the field `authenticate_oidc`.\n"]
    pub fn set_authenticate_oidc(
        mut self,
        v: impl Into<ListField<DataAlbListenerDefaultActionElAuthenticateOidcEl>>,
    ) -> Self {
        self.authenticate_oidc = Some(v.into());
        self
    }

    #[doc = "Set the field `fixed_response`.\n"]
    pub fn set_fixed_response(mut self, v: impl Into<ListField<DataAlbListenerDefaultActionElFixedResponseEl>>) -> Self {
        self.fixed_response = Some(v.into());
        self
    }

    #[doc = "Set the field `forward`.\n"]
    pub fn set_forward(mut self, v: impl Into<ListField<DataAlbListenerDefaultActionElForwardEl>>) -> Self {
        self.forward = Some(v.into());
        self
    }

    #[doc = "Set the field `jwt_validation`.\n"]
    pub fn set_jwt_validation(mut self, v: impl Into<ListField<DataAlbListenerDefaultActionElJwtValidationEl>>) -> Self {
        self.jwt_validation = Some(v.into());
        self
    }

    #[doc = "Set the field `order`.\n"]
    pub fn set_order(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.order = Some(v.into());
        self
    }

    #[doc = "Set the field `redirect`.\n"]
    pub fn set_redirect(mut self, v: impl Into<ListField<DataAlbListenerDefaultActionElRedirectEl>>) -> Self {
        self.redirect = Some(v.into());
        self
    }

    #[doc = "Set the field `target_group_arn`.\n"]
    pub fn set_target_group_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_group_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataAlbListenerDefaultActionEl {
    type O = BlockAssignable<DataAlbListenerDefaultActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlbListenerDefaultActionEl {}

impl BuildDataAlbListenerDefaultActionEl {
    pub fn build(self) -> DataAlbListenerDefaultActionEl {
        DataAlbListenerDefaultActionEl {
            authenticate_cognito: core::default::Default::default(),
            authenticate_oidc: core::default::Default::default(),
            fixed_response: core::default::Default::default(),
            forward: core::default::Default::default(),
            jwt_validation: core::default::Default::default(),
            order: core::default::Default::default(),
            redirect: core::default::Default::default(),
            target_group_arn: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataAlbListenerDefaultActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAlbListenerDefaultActionElRef {
    fn new(shared: StackShared, base: String) -> DataAlbListenerDefaultActionElRef {
        DataAlbListenerDefaultActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlbListenerDefaultActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authenticate_cognito` after provisioning.\n"]
    pub fn authenticate_cognito(&self) -> ListRef<DataAlbListenerDefaultActionElAuthenticateCognitoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.authenticate_cognito", self.base))
    }

    #[doc = "Get a reference to the value of field `authenticate_oidc` after provisioning.\n"]
    pub fn authenticate_oidc(&self) -> ListRef<DataAlbListenerDefaultActionElAuthenticateOidcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.authenticate_oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `fixed_response` after provisioning.\n"]
    pub fn fixed_response(&self) -> ListRef<DataAlbListenerDefaultActionElFixedResponseElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fixed_response", self.base))
    }

    #[doc = "Get a reference to the value of field `forward` after provisioning.\n"]
    pub fn forward(&self) -> ListRef<DataAlbListenerDefaultActionElForwardElRef> {
        ListRef::new(self.shared().clone(), format!("{}.forward", self.base))
    }

    #[doc = "Get a reference to the value of field `jwt_validation` after provisioning.\n"]
    pub fn jwt_validation(&self) -> ListRef<DataAlbListenerDefaultActionElJwtValidationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.jwt_validation", self.base))
    }

    #[doc = "Get a reference to the value of field `order` after provisioning.\n"]
    pub fn order(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> ListRef<DataAlbListenerDefaultActionElRedirectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.redirect", self.base))
    }

    #[doc = "Get a reference to the value of field `target_group_arn` after provisioning.\n"]
    pub fn target_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlbListenerMutualAuthenticationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    advertise_trust_store_ca_names: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_client_certificate_expiry: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trust_store_arn: Option<PrimField<String>>,
}

impl DataAlbListenerMutualAuthenticationEl {
    #[doc = "Set the field `advertise_trust_store_ca_names`.\n"]
    pub fn set_advertise_trust_store_ca_names(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.advertise_trust_store_ca_names = Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_client_certificate_expiry`.\n"]
    pub fn set_ignore_client_certificate_expiry(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_client_certificate_expiry = Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }

    #[doc = "Set the field `trust_store_arn`.\n"]
    pub fn set_trust_store_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.trust_store_arn = Some(v.into());
        self
    }
}

impl ToListMappable for DataAlbListenerMutualAuthenticationEl {
    type O = BlockAssignable<DataAlbListenerMutualAuthenticationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlbListenerMutualAuthenticationEl {}

impl BuildDataAlbListenerMutualAuthenticationEl {
    pub fn build(self) -> DataAlbListenerMutualAuthenticationEl {
        DataAlbListenerMutualAuthenticationEl {
            advertise_trust_store_ca_names: core::default::Default::default(),
            ignore_client_certificate_expiry: core::default::Default::default(),
            mode: core::default::Default::default(),
            trust_store_arn: core::default::Default::default(),
        }
    }
}

pub struct DataAlbListenerMutualAuthenticationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAlbListenerMutualAuthenticationElRef {
    fn new(shared: StackShared, base: String) -> DataAlbListenerMutualAuthenticationElRef {
        DataAlbListenerMutualAuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlbListenerMutualAuthenticationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `advertise_trust_store_ca_names` after provisioning.\n"]
    pub fn advertise_trust_store_ca_names(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.advertise_trust_store_ca_names", self.base))
    }

    #[doc = "Get a reference to the value of field `ignore_client_certificate_expiry` after provisioning.\n"]
    pub fn ignore_client_certificate_expiry(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_client_certificate_expiry", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `trust_store_arn` after provisioning.\n"]
    pub fn trust_store_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trust_store_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlbListenerTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}

impl DataAlbListenerTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}

impl ToListMappable for DataAlbListenerTimeoutsEl {
    type O = BlockAssignable<DataAlbListenerTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlbListenerTimeoutsEl {}

impl BuildDataAlbListenerTimeoutsEl {
    pub fn build(self) -> DataAlbListenerTimeoutsEl {
        DataAlbListenerTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataAlbListenerTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAlbListenerTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataAlbListenerTimeoutsElRef {
        DataAlbListenerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlbListenerTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}
