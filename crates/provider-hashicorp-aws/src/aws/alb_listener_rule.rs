use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AlbListenerRuleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    listener_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<AlbListenerRuleActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Vec<AlbListenerRuleConditionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transform: Option<Vec<AlbListenerRuleTransformEl>>,
    dynamic: AlbListenerRuleDynamic,
}

struct AlbListenerRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AlbListenerRuleData>,
}

#[derive(Clone)]
pub struct AlbListenerRule(Rc<AlbListenerRule_>);

impl AlbListenerRule {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().priority = Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(self, v: impl Into<BlockAssignable<AlbListenerRuleActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.action = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(self, v: impl Into<BlockAssignable<AlbListenerRuleConditionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().condition = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.condition = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `transform`.\n"]
    pub fn set_transform(self, v: impl Into<BlockAssignable<AlbListenerRuleTransformEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().transform = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.transform = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `listener_arn` after provisioning.\n"]
    pub fn listener_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.listener_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<AlbListenerRuleActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }
}

impl Referable for AlbListenerRule {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for AlbListenerRule { }

impl ToListMappable for AlbListenerRule {
    type O = ListRef<AlbListenerRuleRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AlbListenerRule_ {
    fn extract_resource_type(&self) -> String {
        "aws_alb_listener_rule".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAlbListenerRule {
    pub tf_id: String,
    #[doc = ""]
    pub listener_arn: PrimField<String>,
}

impl BuildAlbListenerRule {
    pub fn build(self, stack: &mut Stack) -> AlbListenerRule {
        let out = AlbListenerRule(Rc::new(AlbListenerRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AlbListenerRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                listener_arn: self.listener_arn,
                priority: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                action: core::default::Default::default(),
                condition: core::default::Default::default(),
                transform: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AlbListenerRuleRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl AlbListenerRuleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `listener_arn` after provisioning.\n"]
    pub fn listener_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.listener_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<AlbListenerRuleActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AlbListenerRuleActionElAuthenticateCognitoEl {
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
    user_pool_arn: PrimField<String>,
    user_pool_client_id: PrimField<String>,
    user_pool_domain: PrimField<String>,
}

impl AlbListenerRuleActionElAuthenticateCognitoEl {
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
}

impl ToListMappable for AlbListenerRuleActionElAuthenticateCognitoEl {
    type O = BlockAssignable<AlbListenerRuleActionElAuthenticateCognitoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleActionElAuthenticateCognitoEl {
    #[doc = ""]
    pub user_pool_arn: PrimField<String>,
    #[doc = ""]
    pub user_pool_client_id: PrimField<String>,
    #[doc = ""]
    pub user_pool_domain: PrimField<String>,
}

impl BuildAlbListenerRuleActionElAuthenticateCognitoEl {
    pub fn build(self) -> AlbListenerRuleActionElAuthenticateCognitoEl {
        AlbListenerRuleActionElAuthenticateCognitoEl {
            authentication_request_extra_params: core::default::Default::default(),
            on_unauthenticated_request: core::default::Default::default(),
            scope: core::default::Default::default(),
            session_cookie_name: core::default::Default::default(),
            session_timeout: core::default::Default::default(),
            user_pool_arn: self.user_pool_arn,
            user_pool_client_id: self.user_pool_client_id,
            user_pool_domain: self.user_pool_domain,
        }
    }
}

pub struct AlbListenerRuleActionElAuthenticateCognitoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerRuleActionElAuthenticateCognitoElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerRuleActionElAuthenticateCognitoElRef {
        AlbListenerRuleActionElAuthenticateCognitoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleActionElAuthenticateCognitoElRef {
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
pub struct AlbListenerRuleActionElAuthenticateOidcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_request_extra_params: Option<RecField<PrimField<String>>>,
    authorization_endpoint: PrimField<String>,
    client_id: PrimField<String>,
    client_secret: PrimField<String>,
    issuer: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_unauthenticated_request: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_cookie_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_timeout: Option<PrimField<f64>>,
    token_endpoint: PrimField<String>,
    user_info_endpoint: PrimField<String>,
}

impl AlbListenerRuleActionElAuthenticateOidcEl {
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
}

impl ToListMappable for AlbListenerRuleActionElAuthenticateOidcEl {
    type O = BlockAssignable<AlbListenerRuleActionElAuthenticateOidcEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleActionElAuthenticateOidcEl {
    #[doc = ""]
    pub authorization_endpoint: PrimField<String>,
    #[doc = ""]
    pub client_id: PrimField<String>,
    #[doc = ""]
    pub client_secret: PrimField<String>,
    #[doc = ""]
    pub issuer: PrimField<String>,
    #[doc = ""]
    pub token_endpoint: PrimField<String>,
    #[doc = ""]
    pub user_info_endpoint: PrimField<String>,
}

impl BuildAlbListenerRuleActionElAuthenticateOidcEl {
    pub fn build(self) -> AlbListenerRuleActionElAuthenticateOidcEl {
        AlbListenerRuleActionElAuthenticateOidcEl {
            authentication_request_extra_params: core::default::Default::default(),
            authorization_endpoint: self.authorization_endpoint,
            client_id: self.client_id,
            client_secret: self.client_secret,
            issuer: self.issuer,
            on_unauthenticated_request: core::default::Default::default(),
            scope: core::default::Default::default(),
            session_cookie_name: core::default::Default::default(),
            session_timeout: core::default::Default::default(),
            token_endpoint: self.token_endpoint,
            user_info_endpoint: self.user_info_endpoint,
        }
    }
}

pub struct AlbListenerRuleActionElAuthenticateOidcElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerRuleActionElAuthenticateOidcElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerRuleActionElAuthenticateOidcElRef {
        AlbListenerRuleActionElAuthenticateOidcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleActionElAuthenticateOidcElRef {
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
pub struct AlbListenerRuleActionElFixedResponseEl {
    content_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_body: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code: Option<PrimField<String>>,
}

impl AlbListenerRuleActionElFixedResponseEl {
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

impl ToListMappable for AlbListenerRuleActionElFixedResponseEl {
    type O = BlockAssignable<AlbListenerRuleActionElFixedResponseEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleActionElFixedResponseEl {
    #[doc = ""]
    pub content_type: PrimField<String>,
}

impl BuildAlbListenerRuleActionElFixedResponseEl {
    pub fn build(self) -> AlbListenerRuleActionElFixedResponseEl {
        AlbListenerRuleActionElFixedResponseEl {
            content_type: self.content_type,
            message_body: core::default::Default::default(),
            status_code: core::default::Default::default(),
        }
    }
}

pub struct AlbListenerRuleActionElFixedResponseElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerRuleActionElFixedResponseElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerRuleActionElFixedResponseElRef {
        AlbListenerRuleActionElFixedResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleActionElFixedResponseElRef {
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
pub struct AlbListenerRuleActionElForwardElStickinessEl {
    duration: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl AlbListenerRuleActionElForwardElStickinessEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for AlbListenerRuleActionElForwardElStickinessEl {
    type O = BlockAssignable<AlbListenerRuleActionElForwardElStickinessEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleActionElForwardElStickinessEl {
    #[doc = ""]
    pub duration: PrimField<f64>,
}

impl BuildAlbListenerRuleActionElForwardElStickinessEl {
    pub fn build(self) -> AlbListenerRuleActionElForwardElStickinessEl {
        AlbListenerRuleActionElForwardElStickinessEl {
            duration: self.duration,
            enabled: core::default::Default::default(),
        }
    }
}

pub struct AlbListenerRuleActionElForwardElStickinessElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerRuleActionElForwardElStickinessElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerRuleActionElForwardElStickinessElRef {
        AlbListenerRuleActionElForwardElStickinessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleActionElForwardElStickinessElRef {
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
pub struct AlbListenerRuleActionElForwardElTargetGroupEl {
    arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}

impl AlbListenerRuleActionElForwardElTargetGroupEl {
    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}

impl ToListMappable for AlbListenerRuleActionElForwardElTargetGroupEl {
    type O = BlockAssignable<AlbListenerRuleActionElForwardElTargetGroupEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleActionElForwardElTargetGroupEl {
    #[doc = ""]
    pub arn: PrimField<String>,
}

impl BuildAlbListenerRuleActionElForwardElTargetGroupEl {
    pub fn build(self) -> AlbListenerRuleActionElForwardElTargetGroupEl {
        AlbListenerRuleActionElForwardElTargetGroupEl {
            arn: self.arn,
            weight: core::default::Default::default(),
        }
    }
}

pub struct AlbListenerRuleActionElForwardElTargetGroupElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerRuleActionElForwardElTargetGroupElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerRuleActionElForwardElTargetGroupElRef {
        AlbListenerRuleActionElForwardElTargetGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleActionElForwardElTargetGroupElRef {
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

#[derive(Serialize, Default)]
struct AlbListenerRuleActionElForwardElDynamic {
    stickiness: Option<DynamicBlock<AlbListenerRuleActionElForwardElStickinessEl>>,
    target_group: Option<DynamicBlock<AlbListenerRuleActionElForwardElTargetGroupEl>>,
}

#[derive(Serialize)]
pub struct AlbListenerRuleActionElForwardEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    stickiness: Option<Vec<AlbListenerRuleActionElForwardElStickinessEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_group: Option<Vec<AlbListenerRuleActionElForwardElTargetGroupEl>>,
    dynamic: AlbListenerRuleActionElForwardElDynamic,
}

impl AlbListenerRuleActionElForwardEl {
    #[doc = "Set the field `stickiness`.\n"]
    pub fn set_stickiness(
        mut self,
        v: impl Into<BlockAssignable<AlbListenerRuleActionElForwardElStickinessEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.stickiness = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.stickiness = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_group`.\n"]
    pub fn set_target_group(
        mut self,
        v: impl Into<BlockAssignable<AlbListenerRuleActionElForwardElTargetGroupEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target_group = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target_group = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AlbListenerRuleActionElForwardEl {
    type O = BlockAssignable<AlbListenerRuleActionElForwardEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleActionElForwardEl {}

impl BuildAlbListenerRuleActionElForwardEl {
    pub fn build(self) -> AlbListenerRuleActionElForwardEl {
        AlbListenerRuleActionElForwardEl {
            stickiness: core::default::Default::default(),
            target_group: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AlbListenerRuleActionElForwardElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerRuleActionElForwardElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerRuleActionElForwardElRef {
        AlbListenerRuleActionElForwardElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleActionElForwardElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stickiness` after provisioning.\n"]
    pub fn stickiness(&self) -> ListRef<AlbListenerRuleActionElForwardElStickinessElRef> {
        ListRef::new(self.shared().clone(), format!("{}.stickiness", self.base))
    }
}

#[derive(Serialize)]
pub struct AlbListenerRuleActionElJwtValidationElAdditionalClaimEl {
    format: PrimField<String>,
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl AlbListenerRuleActionElJwtValidationElAdditionalClaimEl { }

impl ToListMappable for AlbListenerRuleActionElJwtValidationElAdditionalClaimEl {
    type O = BlockAssignable<AlbListenerRuleActionElJwtValidationElAdditionalClaimEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleActionElJwtValidationElAdditionalClaimEl {
    #[doc = ""]
    pub format: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildAlbListenerRuleActionElJwtValidationElAdditionalClaimEl {
    pub fn build(self) -> AlbListenerRuleActionElJwtValidationElAdditionalClaimEl {
        AlbListenerRuleActionElJwtValidationElAdditionalClaimEl {
            format: self.format,
            name: self.name,
            values: self.values,
        }
    }
}

pub struct AlbListenerRuleActionElJwtValidationElAdditionalClaimElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerRuleActionElJwtValidationElAdditionalClaimElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerRuleActionElJwtValidationElAdditionalClaimElRef {
        AlbListenerRuleActionElJwtValidationElAdditionalClaimElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleActionElJwtValidationElAdditionalClaimElRef {
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

#[derive(Serialize, Default)]
struct AlbListenerRuleActionElJwtValidationElDynamic {
    additional_claim: Option<DynamicBlock<AlbListenerRuleActionElJwtValidationElAdditionalClaimEl>>,
}

#[derive(Serialize)]
pub struct AlbListenerRuleActionElJwtValidationEl {
    issuer: PrimField<String>,
    jwks_endpoint: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_claim: Option<Vec<AlbListenerRuleActionElJwtValidationElAdditionalClaimEl>>,
    dynamic: AlbListenerRuleActionElJwtValidationElDynamic,
}

impl AlbListenerRuleActionElJwtValidationEl {
    #[doc = "Set the field `additional_claim`.\n"]
    pub fn set_additional_claim(
        mut self,
        v: impl Into<BlockAssignable<AlbListenerRuleActionElJwtValidationElAdditionalClaimEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.additional_claim = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_claim = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AlbListenerRuleActionElJwtValidationEl {
    type O = BlockAssignable<AlbListenerRuleActionElJwtValidationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleActionElJwtValidationEl {
    #[doc = ""]
    pub issuer: PrimField<String>,
    #[doc = ""]
    pub jwks_endpoint: PrimField<String>,
}

impl BuildAlbListenerRuleActionElJwtValidationEl {
    pub fn build(self) -> AlbListenerRuleActionElJwtValidationEl {
        AlbListenerRuleActionElJwtValidationEl {
            issuer: self.issuer,
            jwks_endpoint: self.jwks_endpoint,
            additional_claim: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AlbListenerRuleActionElJwtValidationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerRuleActionElJwtValidationElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerRuleActionElJwtValidationElRef {
        AlbListenerRuleActionElJwtValidationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleActionElJwtValidationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
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
pub struct AlbListenerRuleActionElRedirectEl {
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
    status_code: PrimField<String>,
}

impl AlbListenerRuleActionElRedirectEl {
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
}

impl ToListMappable for AlbListenerRuleActionElRedirectEl {
    type O = BlockAssignable<AlbListenerRuleActionElRedirectEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleActionElRedirectEl {
    #[doc = ""]
    pub status_code: PrimField<String>,
}

impl BuildAlbListenerRuleActionElRedirectEl {
    pub fn build(self) -> AlbListenerRuleActionElRedirectEl {
        AlbListenerRuleActionElRedirectEl {
            host: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
            query: core::default::Default::default(),
            status_code: self.status_code,
        }
    }
}

pub struct AlbListenerRuleActionElRedirectElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerRuleActionElRedirectElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerRuleActionElRedirectElRef {
        AlbListenerRuleActionElRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleActionElRedirectElRef {
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

#[derive(Serialize, Default)]
struct AlbListenerRuleActionElDynamic {
    authenticate_cognito: Option<DynamicBlock<AlbListenerRuleActionElAuthenticateCognitoEl>>,
    authenticate_oidc: Option<DynamicBlock<AlbListenerRuleActionElAuthenticateOidcEl>>,
    fixed_response: Option<DynamicBlock<AlbListenerRuleActionElFixedResponseEl>>,
    forward: Option<DynamicBlock<AlbListenerRuleActionElForwardEl>>,
    jwt_validation: Option<DynamicBlock<AlbListenerRuleActionElJwtValidationEl>>,
    redirect: Option<DynamicBlock<AlbListenerRuleActionElRedirectEl>>,
}

#[derive(Serialize)]
pub struct AlbListenerRuleActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_group_arn: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authenticate_cognito: Option<Vec<AlbListenerRuleActionElAuthenticateCognitoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authenticate_oidc: Option<Vec<AlbListenerRuleActionElAuthenticateOidcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_response: Option<Vec<AlbListenerRuleActionElFixedResponseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forward: Option<Vec<AlbListenerRuleActionElForwardEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jwt_validation: Option<Vec<AlbListenerRuleActionElJwtValidationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect: Option<Vec<AlbListenerRuleActionElRedirectEl>>,
    dynamic: AlbListenerRuleActionElDynamic,
}

impl AlbListenerRuleActionEl {
    #[doc = "Set the field `order`.\n"]
    pub fn set_order(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.order = Some(v.into());
        self
    }

    #[doc = "Set the field `target_group_arn`.\n"]
    pub fn set_target_group_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_group_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `authenticate_cognito`.\n"]
    pub fn set_authenticate_cognito(
        mut self,
        v: impl Into<BlockAssignable<AlbListenerRuleActionElAuthenticateCognitoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.authenticate_cognito = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.authenticate_cognito = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `authenticate_oidc`.\n"]
    pub fn set_authenticate_oidc(
        mut self,
        v: impl Into<BlockAssignable<AlbListenerRuleActionElAuthenticateOidcEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.authenticate_oidc = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.authenticate_oidc = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fixed_response`.\n"]
    pub fn set_fixed_response(mut self, v: impl Into<BlockAssignable<AlbListenerRuleActionElFixedResponseEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fixed_response = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fixed_response = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `forward`.\n"]
    pub fn set_forward(mut self, v: impl Into<BlockAssignable<AlbListenerRuleActionElForwardEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.forward = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.forward = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `jwt_validation`.\n"]
    pub fn set_jwt_validation(mut self, v: impl Into<BlockAssignable<AlbListenerRuleActionElJwtValidationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.jwt_validation = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.jwt_validation = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redirect`.\n"]
    pub fn set_redirect(mut self, v: impl Into<BlockAssignable<AlbListenerRuleActionElRedirectEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.redirect = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.redirect = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AlbListenerRuleActionEl {
    type O = BlockAssignable<AlbListenerRuleActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleActionEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildAlbListenerRuleActionEl {
    pub fn build(self) -> AlbListenerRuleActionEl {
        AlbListenerRuleActionEl {
            order: core::default::Default::default(),
            target_group_arn: core::default::Default::default(),
            type_: self.type_,
            authenticate_cognito: core::default::Default::default(),
            authenticate_oidc: core::default::Default::default(),
            fixed_response: core::default::Default::default(),
            forward: core::default::Default::default(),
            jwt_validation: core::default::Default::default(),
            redirect: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AlbListenerRuleActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerRuleActionElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerRuleActionElRef {
        AlbListenerRuleActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `order` after provisioning.\n"]
    pub fn order(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }

    #[doc = "Get a reference to the value of field `target_group_arn` after provisioning.\n"]
    pub fn target_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `authenticate_cognito` after provisioning.\n"]
    pub fn authenticate_cognito(&self) -> ListRef<AlbListenerRuleActionElAuthenticateCognitoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.authenticate_cognito", self.base))
    }

    #[doc = "Get a reference to the value of field `authenticate_oidc` after provisioning.\n"]
    pub fn authenticate_oidc(&self) -> ListRef<AlbListenerRuleActionElAuthenticateOidcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.authenticate_oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `fixed_response` after provisioning.\n"]
    pub fn fixed_response(&self) -> ListRef<AlbListenerRuleActionElFixedResponseElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fixed_response", self.base))
    }

    #[doc = "Get a reference to the value of field `forward` after provisioning.\n"]
    pub fn forward(&self) -> ListRef<AlbListenerRuleActionElForwardElRef> {
        ListRef::new(self.shared().clone(), format!("{}.forward", self.base))
    }

    #[doc = "Get a reference to the value of field `jwt_validation` after provisioning.\n"]
    pub fn jwt_validation(&self) -> ListRef<AlbListenerRuleActionElJwtValidationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.jwt_validation", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> ListRef<AlbListenerRuleActionElRedirectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.redirect", self.base))
    }
}

#[derive(Serialize)]
pub struct AlbListenerRuleConditionElHostHeaderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    regex_values: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl AlbListenerRuleConditionElHostHeaderEl {
    #[doc = "Set the field `regex_values`.\n"]
    pub fn set_regex_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.regex_values = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for AlbListenerRuleConditionElHostHeaderEl {
    type O = BlockAssignable<AlbListenerRuleConditionElHostHeaderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleConditionElHostHeaderEl {}

impl BuildAlbListenerRuleConditionElHostHeaderEl {
    pub fn build(self) -> AlbListenerRuleConditionElHostHeaderEl {
        AlbListenerRuleConditionElHostHeaderEl {
            regex_values: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct AlbListenerRuleConditionElHostHeaderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerRuleConditionElHostHeaderElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerRuleConditionElHostHeaderElRef {
        AlbListenerRuleConditionElHostHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleConditionElHostHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `regex_values` after provisioning.\n"]
    pub fn regex_values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.regex_values", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct AlbListenerRuleConditionElHttpHeaderEl {
    http_header_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex_values: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl AlbListenerRuleConditionElHttpHeaderEl {
    #[doc = "Set the field `regex_values`.\n"]
    pub fn set_regex_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.regex_values = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for AlbListenerRuleConditionElHttpHeaderEl {
    type O = BlockAssignable<AlbListenerRuleConditionElHttpHeaderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleConditionElHttpHeaderEl {
    #[doc = ""]
    pub http_header_name: PrimField<String>,
}

impl BuildAlbListenerRuleConditionElHttpHeaderEl {
    pub fn build(self) -> AlbListenerRuleConditionElHttpHeaderEl {
        AlbListenerRuleConditionElHttpHeaderEl {
            http_header_name: self.http_header_name,
            regex_values: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct AlbListenerRuleConditionElHttpHeaderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerRuleConditionElHttpHeaderElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerRuleConditionElHttpHeaderElRef {
        AlbListenerRuleConditionElHttpHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleConditionElHttpHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `http_header_name` after provisioning.\n"]
    pub fn http_header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_header_name", self.base))
    }

    #[doc = "Get a reference to the value of field `regex_values` after provisioning.\n"]
    pub fn regex_values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.regex_values", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct AlbListenerRuleConditionElHttpRequestMethodEl {
    values: SetField<PrimField<String>>,
}

impl AlbListenerRuleConditionElHttpRequestMethodEl { }

impl ToListMappable for AlbListenerRuleConditionElHttpRequestMethodEl {
    type O = BlockAssignable<AlbListenerRuleConditionElHttpRequestMethodEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleConditionElHttpRequestMethodEl {
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildAlbListenerRuleConditionElHttpRequestMethodEl {
    pub fn build(self) -> AlbListenerRuleConditionElHttpRequestMethodEl {
        AlbListenerRuleConditionElHttpRequestMethodEl { values: self.values }
    }
}

pub struct AlbListenerRuleConditionElHttpRequestMethodElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerRuleConditionElHttpRequestMethodElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerRuleConditionElHttpRequestMethodElRef {
        AlbListenerRuleConditionElHttpRequestMethodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleConditionElHttpRequestMethodElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct AlbListenerRuleConditionElPathPatternEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    regex_values: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl AlbListenerRuleConditionElPathPatternEl {
    #[doc = "Set the field `regex_values`.\n"]
    pub fn set_regex_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.regex_values = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for AlbListenerRuleConditionElPathPatternEl {
    type O = BlockAssignable<AlbListenerRuleConditionElPathPatternEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleConditionElPathPatternEl {}

impl BuildAlbListenerRuleConditionElPathPatternEl {
    pub fn build(self) -> AlbListenerRuleConditionElPathPatternEl {
        AlbListenerRuleConditionElPathPatternEl {
            regex_values: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct AlbListenerRuleConditionElPathPatternElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerRuleConditionElPathPatternElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerRuleConditionElPathPatternElRef {
        AlbListenerRuleConditionElPathPatternElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleConditionElPathPatternElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `regex_values` after provisioning.\n"]
    pub fn regex_values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.regex_values", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct AlbListenerRuleConditionElQueryStringEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    value: PrimField<String>,
}

impl AlbListenerRuleConditionElQueryStringEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
}

impl ToListMappable for AlbListenerRuleConditionElQueryStringEl {
    type O = BlockAssignable<AlbListenerRuleConditionElQueryStringEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleConditionElQueryStringEl {
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildAlbListenerRuleConditionElQueryStringEl {
    pub fn build(self) -> AlbListenerRuleConditionElQueryStringEl {
        AlbListenerRuleConditionElQueryStringEl {
            key: core::default::Default::default(),
            value: self.value,
        }
    }
}

pub struct AlbListenerRuleConditionElQueryStringElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerRuleConditionElQueryStringElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerRuleConditionElQueryStringElRef {
        AlbListenerRuleConditionElQueryStringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleConditionElQueryStringElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct AlbListenerRuleConditionElSourceIpEl {
    values: SetField<PrimField<String>>,
}

impl AlbListenerRuleConditionElSourceIpEl { }

impl ToListMappable for AlbListenerRuleConditionElSourceIpEl {
    type O = BlockAssignable<AlbListenerRuleConditionElSourceIpEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleConditionElSourceIpEl {
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildAlbListenerRuleConditionElSourceIpEl {
    pub fn build(self) -> AlbListenerRuleConditionElSourceIpEl {
        AlbListenerRuleConditionElSourceIpEl { values: self.values }
    }
}

pub struct AlbListenerRuleConditionElSourceIpElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerRuleConditionElSourceIpElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerRuleConditionElSourceIpElRef {
        AlbListenerRuleConditionElSourceIpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleConditionElSourceIpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct AlbListenerRuleConditionElDynamic {
    host_header: Option<DynamicBlock<AlbListenerRuleConditionElHostHeaderEl>>,
    http_header: Option<DynamicBlock<AlbListenerRuleConditionElHttpHeaderEl>>,
    http_request_method: Option<DynamicBlock<AlbListenerRuleConditionElHttpRequestMethodEl>>,
    path_pattern: Option<DynamicBlock<AlbListenerRuleConditionElPathPatternEl>>,
    query_string: Option<DynamicBlock<AlbListenerRuleConditionElQueryStringEl>>,
    source_ip: Option<DynamicBlock<AlbListenerRuleConditionElSourceIpEl>>,
}

#[derive(Serialize)]
pub struct AlbListenerRuleConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_header: Option<Vec<AlbListenerRuleConditionElHostHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_header: Option<Vec<AlbListenerRuleConditionElHttpHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_request_method: Option<Vec<AlbListenerRuleConditionElHttpRequestMethodEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_pattern: Option<Vec<AlbListenerRuleConditionElPathPatternEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_string: Option<Vec<AlbListenerRuleConditionElQueryStringEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_ip: Option<Vec<AlbListenerRuleConditionElSourceIpEl>>,
    dynamic: AlbListenerRuleConditionElDynamic,
}

impl AlbListenerRuleConditionEl {
    #[doc = "Set the field `host_header`.\n"]
    pub fn set_host_header(mut self, v: impl Into<BlockAssignable<AlbListenerRuleConditionElHostHeaderEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.host_header = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.host_header = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http_header`.\n"]
    pub fn set_http_header(mut self, v: impl Into<BlockAssignable<AlbListenerRuleConditionElHttpHeaderEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_header = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_header = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http_request_method`.\n"]
    pub fn set_http_request_method(
        mut self,
        v: impl Into<BlockAssignable<AlbListenerRuleConditionElHttpRequestMethodEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_request_method = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_request_method = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `path_pattern`.\n"]
    pub fn set_path_pattern(mut self, v: impl Into<BlockAssignable<AlbListenerRuleConditionElPathPatternEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.path_pattern = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.path_pattern = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_string`.\n"]
    pub fn set_query_string(mut self, v: impl Into<BlockAssignable<AlbListenerRuleConditionElQueryStringEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query_string = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query_string = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_ip`.\n"]
    pub fn set_source_ip(mut self, v: impl Into<BlockAssignable<AlbListenerRuleConditionElSourceIpEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_ip = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_ip = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AlbListenerRuleConditionEl {
    type O = BlockAssignable<AlbListenerRuleConditionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleConditionEl {}

impl BuildAlbListenerRuleConditionEl {
    pub fn build(self) -> AlbListenerRuleConditionEl {
        AlbListenerRuleConditionEl {
            host_header: core::default::Default::default(),
            http_header: core::default::Default::default(),
            http_request_method: core::default::Default::default(),
            path_pattern: core::default::Default::default(),
            query_string: core::default::Default::default(),
            source_ip: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AlbListenerRuleConditionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerRuleConditionElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerRuleConditionElRef {
        AlbListenerRuleConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host_header` after provisioning.\n"]
    pub fn host_header(&self) -> ListRef<AlbListenerRuleConditionElHostHeaderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.host_header", self.base))
    }

    #[doc = "Get a reference to the value of field `http_header` after provisioning.\n"]
    pub fn http_header(&self) -> ListRef<AlbListenerRuleConditionElHttpHeaderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_header", self.base))
    }

    #[doc = "Get a reference to the value of field `http_request_method` after provisioning.\n"]
    pub fn http_request_method(&self) -> ListRef<AlbListenerRuleConditionElHttpRequestMethodElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_request_method", self.base))
    }

    #[doc = "Get a reference to the value of field `path_pattern` after provisioning.\n"]
    pub fn path_pattern(&self) -> ListRef<AlbListenerRuleConditionElPathPatternElRef> {
        ListRef::new(self.shared().clone(), format!("{}.path_pattern", self.base))
    }

    #[doc = "Get a reference to the value of field `source_ip` after provisioning.\n"]
    pub fn source_ip(&self) -> ListRef<AlbListenerRuleConditionElSourceIpElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_ip", self.base))
    }
}

#[derive(Serialize)]
pub struct AlbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl {
    regex: PrimField<String>,
    replace: PrimField<String>,
}

impl AlbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl { }

impl ToListMappable for AlbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl {
    type O = BlockAssignable<AlbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl {
    #[doc = ""]
    pub regex: PrimField<String>,
    #[doc = ""]
    pub replace: PrimField<String>,
}

impl BuildAlbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl {
    pub fn build(self) -> AlbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl {
        AlbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl {
            regex: self.regex,
            replace: self.replace,
        }
    }
}

pub struct AlbListenerRuleTransformElHostHeaderRewriteConfigElRewriteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerRuleTransformElHostHeaderRewriteConfigElRewriteElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerRuleTransformElHostHeaderRewriteConfigElRewriteElRef {
        AlbListenerRuleTransformElHostHeaderRewriteConfigElRewriteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleTransformElHostHeaderRewriteConfigElRewriteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }

    #[doc = "Get a reference to the value of field `replace` after provisioning.\n"]
    pub fn replace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize, Default)]
struct AlbListenerRuleTransformElHostHeaderRewriteConfigElDynamic {
    rewrite: Option<DynamicBlock<AlbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl>>,
}

#[derive(Serialize)]
pub struct AlbListenerRuleTransformElHostHeaderRewriteConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rewrite: Option<Vec<AlbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl>>,
    dynamic: AlbListenerRuleTransformElHostHeaderRewriteConfigElDynamic,
}

impl AlbListenerRuleTransformElHostHeaderRewriteConfigEl {
    #[doc = "Set the field `rewrite`.\n"]
    pub fn set_rewrite(
        mut self,
        v: impl Into<BlockAssignable<AlbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rewrite = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rewrite = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AlbListenerRuleTransformElHostHeaderRewriteConfigEl {
    type O = BlockAssignable<AlbListenerRuleTransformElHostHeaderRewriteConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleTransformElHostHeaderRewriteConfigEl {}

impl BuildAlbListenerRuleTransformElHostHeaderRewriteConfigEl {
    pub fn build(self) -> AlbListenerRuleTransformElHostHeaderRewriteConfigEl {
        AlbListenerRuleTransformElHostHeaderRewriteConfigEl {
            rewrite: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AlbListenerRuleTransformElHostHeaderRewriteConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerRuleTransformElHostHeaderRewriteConfigElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerRuleTransformElHostHeaderRewriteConfigElRef {
        AlbListenerRuleTransformElHostHeaderRewriteConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleTransformElHostHeaderRewriteConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rewrite` after provisioning.\n"]
    pub fn rewrite(&self) -> ListRef<AlbListenerRuleTransformElHostHeaderRewriteConfigElRewriteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rewrite", self.base))
    }
}

#[derive(Serialize)]
pub struct AlbListenerRuleTransformElUrlRewriteConfigElRewriteEl {
    regex: PrimField<String>,
    replace: PrimField<String>,
}

impl AlbListenerRuleTransformElUrlRewriteConfigElRewriteEl { }

impl ToListMappable for AlbListenerRuleTransformElUrlRewriteConfigElRewriteEl {
    type O = BlockAssignable<AlbListenerRuleTransformElUrlRewriteConfigElRewriteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleTransformElUrlRewriteConfigElRewriteEl {
    #[doc = ""]
    pub regex: PrimField<String>,
    #[doc = ""]
    pub replace: PrimField<String>,
}

impl BuildAlbListenerRuleTransformElUrlRewriteConfigElRewriteEl {
    pub fn build(self) -> AlbListenerRuleTransformElUrlRewriteConfigElRewriteEl {
        AlbListenerRuleTransformElUrlRewriteConfigElRewriteEl {
            regex: self.regex,
            replace: self.replace,
        }
    }
}

pub struct AlbListenerRuleTransformElUrlRewriteConfigElRewriteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerRuleTransformElUrlRewriteConfigElRewriteElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerRuleTransformElUrlRewriteConfigElRewriteElRef {
        AlbListenerRuleTransformElUrlRewriteConfigElRewriteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleTransformElUrlRewriteConfigElRewriteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }

    #[doc = "Get a reference to the value of field `replace` after provisioning.\n"]
    pub fn replace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize, Default)]
struct AlbListenerRuleTransformElUrlRewriteConfigElDynamic {
    rewrite: Option<DynamicBlock<AlbListenerRuleTransformElUrlRewriteConfigElRewriteEl>>,
}

#[derive(Serialize)]
pub struct AlbListenerRuleTransformElUrlRewriteConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rewrite: Option<Vec<AlbListenerRuleTransformElUrlRewriteConfigElRewriteEl>>,
    dynamic: AlbListenerRuleTransformElUrlRewriteConfigElDynamic,
}

impl AlbListenerRuleTransformElUrlRewriteConfigEl {
    #[doc = "Set the field `rewrite`.\n"]
    pub fn set_rewrite(
        mut self,
        v: impl Into<BlockAssignable<AlbListenerRuleTransformElUrlRewriteConfigElRewriteEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rewrite = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rewrite = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AlbListenerRuleTransformElUrlRewriteConfigEl {
    type O = BlockAssignable<AlbListenerRuleTransformElUrlRewriteConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleTransformElUrlRewriteConfigEl {}

impl BuildAlbListenerRuleTransformElUrlRewriteConfigEl {
    pub fn build(self) -> AlbListenerRuleTransformElUrlRewriteConfigEl {
        AlbListenerRuleTransformElUrlRewriteConfigEl {
            rewrite: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AlbListenerRuleTransformElUrlRewriteConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerRuleTransformElUrlRewriteConfigElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerRuleTransformElUrlRewriteConfigElRef {
        AlbListenerRuleTransformElUrlRewriteConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleTransformElUrlRewriteConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rewrite` after provisioning.\n"]
    pub fn rewrite(&self) -> ListRef<AlbListenerRuleTransformElUrlRewriteConfigElRewriteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rewrite", self.base))
    }
}

#[derive(Serialize, Default)]
struct AlbListenerRuleTransformElDynamic {
    host_header_rewrite_config: Option<DynamicBlock<AlbListenerRuleTransformElHostHeaderRewriteConfigEl>>,
    url_rewrite_config: Option<DynamicBlock<AlbListenerRuleTransformElUrlRewriteConfigEl>>,
}

#[derive(Serialize)]
pub struct AlbListenerRuleTransformEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_header_rewrite_config: Option<Vec<AlbListenerRuleTransformElHostHeaderRewriteConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_rewrite_config: Option<Vec<AlbListenerRuleTransformElUrlRewriteConfigEl>>,
    dynamic: AlbListenerRuleTransformElDynamic,
}

impl AlbListenerRuleTransformEl {
    #[doc = "Set the field `host_header_rewrite_config`.\n"]
    pub fn set_host_header_rewrite_config(
        mut self,
        v: impl Into<BlockAssignable<AlbListenerRuleTransformElHostHeaderRewriteConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.host_header_rewrite_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.host_header_rewrite_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `url_rewrite_config`.\n"]
    pub fn set_url_rewrite_config(
        mut self,
        v: impl Into<BlockAssignable<AlbListenerRuleTransformElUrlRewriteConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.url_rewrite_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.url_rewrite_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AlbListenerRuleTransformEl {
    type O = BlockAssignable<AlbListenerRuleTransformEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleTransformEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildAlbListenerRuleTransformEl {
    pub fn build(self) -> AlbListenerRuleTransformEl {
        AlbListenerRuleTransformEl {
            type_: self.type_,
            host_header_rewrite_config: core::default::Default::default(),
            url_rewrite_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AlbListenerRuleTransformElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerRuleTransformElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerRuleTransformElRef {
        AlbListenerRuleTransformElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleTransformElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `host_header_rewrite_config` after provisioning.\n"]
    pub fn host_header_rewrite_config(&self) -> ListRef<AlbListenerRuleTransformElHostHeaderRewriteConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.host_header_rewrite_config", self.base))
    }

    #[doc = "Get a reference to the value of field `url_rewrite_config` after provisioning.\n"]
    pub fn url_rewrite_config(&self) -> ListRef<AlbListenerRuleTransformElUrlRewriteConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.url_rewrite_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct AlbListenerRuleDynamic {
    action: Option<DynamicBlock<AlbListenerRuleActionEl>>,
    condition: Option<DynamicBlock<AlbListenerRuleConditionEl>>,
    transform: Option<DynamicBlock<AlbListenerRuleTransformEl>>,
}
