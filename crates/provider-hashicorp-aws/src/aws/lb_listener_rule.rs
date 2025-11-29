use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LbListenerRuleData {
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
    action: Option<Vec<LbListenerRuleActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Vec<LbListenerRuleConditionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transform: Option<Vec<LbListenerRuleTransformEl>>,
    dynamic: LbListenerRuleDynamic,
}

struct LbListenerRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LbListenerRuleData>,
}

#[derive(Clone)]
pub struct LbListenerRule(Rc<LbListenerRule_>);

impl LbListenerRule {
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
    pub fn set_action(self, v: impl Into<BlockAssignable<LbListenerRuleActionEl>>) -> Self {
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
    pub fn set_condition(self, v: impl Into<BlockAssignable<LbListenerRuleConditionEl>>) -> Self {
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
    pub fn set_transform(self, v: impl Into<BlockAssignable<LbListenerRuleTransformEl>>) -> Self {
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
    pub fn action(&self) -> ListRef<LbListenerRuleActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }
}

impl Referable for LbListenerRule {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for LbListenerRule { }

impl ToListMappable for LbListenerRule {
    type O = ListRef<LbListenerRuleRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for LbListenerRule_ {
    fn extract_resource_type(&self) -> String {
        "aws_lb_listener_rule".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLbListenerRule {
    pub tf_id: String,
    #[doc = ""]
    pub listener_arn: PrimField<String>,
}

impl BuildLbListenerRule {
    pub fn build(self, stack: &mut Stack) -> LbListenerRule {
        let out = LbListenerRule(Rc::new(LbListenerRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LbListenerRuleData {
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

pub struct LbListenerRuleRef {
    shared: StackShared,
    base: String,
}

impl Ref for LbListenerRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl LbListenerRuleRef {
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
    pub fn action(&self) -> ListRef<LbListenerRuleActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LbListenerRuleActionElAuthenticateCognitoEl {
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

impl LbListenerRuleActionElAuthenticateCognitoEl {
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

impl ToListMappable for LbListenerRuleActionElAuthenticateCognitoEl {
    type O = BlockAssignable<LbListenerRuleActionElAuthenticateCognitoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleActionElAuthenticateCognitoEl {
    #[doc = ""]
    pub user_pool_arn: PrimField<String>,
    #[doc = ""]
    pub user_pool_client_id: PrimField<String>,
    #[doc = ""]
    pub user_pool_domain: PrimField<String>,
}

impl BuildLbListenerRuleActionElAuthenticateCognitoEl {
    pub fn build(self) -> LbListenerRuleActionElAuthenticateCognitoEl {
        LbListenerRuleActionElAuthenticateCognitoEl {
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

pub struct LbListenerRuleActionElAuthenticateCognitoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LbListenerRuleActionElAuthenticateCognitoElRef {
    fn new(shared: StackShared, base: String) -> LbListenerRuleActionElAuthenticateCognitoElRef {
        LbListenerRuleActionElAuthenticateCognitoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleActionElAuthenticateCognitoElRef {
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
pub struct LbListenerRuleActionElAuthenticateOidcEl {
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

impl LbListenerRuleActionElAuthenticateOidcEl {
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

impl ToListMappable for LbListenerRuleActionElAuthenticateOidcEl {
    type O = BlockAssignable<LbListenerRuleActionElAuthenticateOidcEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleActionElAuthenticateOidcEl {
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

impl BuildLbListenerRuleActionElAuthenticateOidcEl {
    pub fn build(self) -> LbListenerRuleActionElAuthenticateOidcEl {
        LbListenerRuleActionElAuthenticateOidcEl {
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

pub struct LbListenerRuleActionElAuthenticateOidcElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LbListenerRuleActionElAuthenticateOidcElRef {
    fn new(shared: StackShared, base: String) -> LbListenerRuleActionElAuthenticateOidcElRef {
        LbListenerRuleActionElAuthenticateOidcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleActionElAuthenticateOidcElRef {
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
pub struct LbListenerRuleActionElFixedResponseEl {
    content_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_body: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code: Option<PrimField<String>>,
}

impl LbListenerRuleActionElFixedResponseEl {
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

impl ToListMappable for LbListenerRuleActionElFixedResponseEl {
    type O = BlockAssignable<LbListenerRuleActionElFixedResponseEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleActionElFixedResponseEl {
    #[doc = ""]
    pub content_type: PrimField<String>,
}

impl BuildLbListenerRuleActionElFixedResponseEl {
    pub fn build(self) -> LbListenerRuleActionElFixedResponseEl {
        LbListenerRuleActionElFixedResponseEl {
            content_type: self.content_type,
            message_body: core::default::Default::default(),
            status_code: core::default::Default::default(),
        }
    }
}

pub struct LbListenerRuleActionElFixedResponseElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LbListenerRuleActionElFixedResponseElRef {
    fn new(shared: StackShared, base: String) -> LbListenerRuleActionElFixedResponseElRef {
        LbListenerRuleActionElFixedResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleActionElFixedResponseElRef {
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
pub struct LbListenerRuleActionElForwardElStickinessEl {
    duration: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl LbListenerRuleActionElForwardElStickinessEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for LbListenerRuleActionElForwardElStickinessEl {
    type O = BlockAssignable<LbListenerRuleActionElForwardElStickinessEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleActionElForwardElStickinessEl {
    #[doc = ""]
    pub duration: PrimField<f64>,
}

impl BuildLbListenerRuleActionElForwardElStickinessEl {
    pub fn build(self) -> LbListenerRuleActionElForwardElStickinessEl {
        LbListenerRuleActionElForwardElStickinessEl {
            duration: self.duration,
            enabled: core::default::Default::default(),
        }
    }
}

pub struct LbListenerRuleActionElForwardElStickinessElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LbListenerRuleActionElForwardElStickinessElRef {
    fn new(shared: StackShared, base: String) -> LbListenerRuleActionElForwardElStickinessElRef {
        LbListenerRuleActionElForwardElStickinessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleActionElForwardElStickinessElRef {
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
pub struct LbListenerRuleActionElForwardElTargetGroupEl {
    arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}

impl LbListenerRuleActionElForwardElTargetGroupEl {
    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}

impl ToListMappable for LbListenerRuleActionElForwardElTargetGroupEl {
    type O = BlockAssignable<LbListenerRuleActionElForwardElTargetGroupEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleActionElForwardElTargetGroupEl {
    #[doc = ""]
    pub arn: PrimField<String>,
}

impl BuildLbListenerRuleActionElForwardElTargetGroupEl {
    pub fn build(self) -> LbListenerRuleActionElForwardElTargetGroupEl {
        LbListenerRuleActionElForwardElTargetGroupEl {
            arn: self.arn,
            weight: core::default::Default::default(),
        }
    }
}

pub struct LbListenerRuleActionElForwardElTargetGroupElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LbListenerRuleActionElForwardElTargetGroupElRef {
    fn new(shared: StackShared, base: String) -> LbListenerRuleActionElForwardElTargetGroupElRef {
        LbListenerRuleActionElForwardElTargetGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleActionElForwardElTargetGroupElRef {
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
struct LbListenerRuleActionElForwardElDynamic {
    stickiness: Option<DynamicBlock<LbListenerRuleActionElForwardElStickinessEl>>,
    target_group: Option<DynamicBlock<LbListenerRuleActionElForwardElTargetGroupEl>>,
}

#[derive(Serialize)]
pub struct LbListenerRuleActionElForwardEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    stickiness: Option<Vec<LbListenerRuleActionElForwardElStickinessEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_group: Option<Vec<LbListenerRuleActionElForwardElTargetGroupEl>>,
    dynamic: LbListenerRuleActionElForwardElDynamic,
}

impl LbListenerRuleActionElForwardEl {
    #[doc = "Set the field `stickiness`.\n"]
    pub fn set_stickiness(mut self, v: impl Into<BlockAssignable<LbListenerRuleActionElForwardElStickinessEl>>) -> Self {
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
        v: impl Into<BlockAssignable<LbListenerRuleActionElForwardElTargetGroupEl>>,
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

impl ToListMappable for LbListenerRuleActionElForwardEl {
    type O = BlockAssignable<LbListenerRuleActionElForwardEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleActionElForwardEl {}

impl BuildLbListenerRuleActionElForwardEl {
    pub fn build(self) -> LbListenerRuleActionElForwardEl {
        LbListenerRuleActionElForwardEl {
            stickiness: core::default::Default::default(),
            target_group: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct LbListenerRuleActionElForwardElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LbListenerRuleActionElForwardElRef {
    fn new(shared: StackShared, base: String) -> LbListenerRuleActionElForwardElRef {
        LbListenerRuleActionElForwardElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleActionElForwardElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stickiness` after provisioning.\n"]
    pub fn stickiness(&self) -> ListRef<LbListenerRuleActionElForwardElStickinessElRef> {
        ListRef::new(self.shared().clone(), format!("{}.stickiness", self.base))
    }
}

#[derive(Serialize)]
pub struct LbListenerRuleActionElJwtValidationElAdditionalClaimEl {
    format: PrimField<String>,
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl LbListenerRuleActionElJwtValidationElAdditionalClaimEl { }

impl ToListMappable for LbListenerRuleActionElJwtValidationElAdditionalClaimEl {
    type O = BlockAssignable<LbListenerRuleActionElJwtValidationElAdditionalClaimEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleActionElJwtValidationElAdditionalClaimEl {
    #[doc = ""]
    pub format: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildLbListenerRuleActionElJwtValidationElAdditionalClaimEl {
    pub fn build(self) -> LbListenerRuleActionElJwtValidationElAdditionalClaimEl {
        LbListenerRuleActionElJwtValidationElAdditionalClaimEl {
            format: self.format,
            name: self.name,
            values: self.values,
        }
    }
}

pub struct LbListenerRuleActionElJwtValidationElAdditionalClaimElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LbListenerRuleActionElJwtValidationElAdditionalClaimElRef {
    fn new(shared: StackShared, base: String) -> LbListenerRuleActionElJwtValidationElAdditionalClaimElRef {
        LbListenerRuleActionElJwtValidationElAdditionalClaimElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleActionElJwtValidationElAdditionalClaimElRef {
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
struct LbListenerRuleActionElJwtValidationElDynamic {
    additional_claim: Option<DynamicBlock<LbListenerRuleActionElJwtValidationElAdditionalClaimEl>>,
}

#[derive(Serialize)]
pub struct LbListenerRuleActionElJwtValidationEl {
    issuer: PrimField<String>,
    jwks_endpoint: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_claim: Option<Vec<LbListenerRuleActionElJwtValidationElAdditionalClaimEl>>,
    dynamic: LbListenerRuleActionElJwtValidationElDynamic,
}

impl LbListenerRuleActionElJwtValidationEl {
    #[doc = "Set the field `additional_claim`.\n"]
    pub fn set_additional_claim(
        mut self,
        v: impl Into<BlockAssignable<LbListenerRuleActionElJwtValidationElAdditionalClaimEl>>,
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

impl ToListMappable for LbListenerRuleActionElJwtValidationEl {
    type O = BlockAssignable<LbListenerRuleActionElJwtValidationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleActionElJwtValidationEl {
    #[doc = ""]
    pub issuer: PrimField<String>,
    #[doc = ""]
    pub jwks_endpoint: PrimField<String>,
}

impl BuildLbListenerRuleActionElJwtValidationEl {
    pub fn build(self) -> LbListenerRuleActionElJwtValidationEl {
        LbListenerRuleActionElJwtValidationEl {
            issuer: self.issuer,
            jwks_endpoint: self.jwks_endpoint,
            additional_claim: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct LbListenerRuleActionElJwtValidationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LbListenerRuleActionElJwtValidationElRef {
    fn new(shared: StackShared, base: String) -> LbListenerRuleActionElJwtValidationElRef {
        LbListenerRuleActionElJwtValidationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleActionElJwtValidationElRef {
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
pub struct LbListenerRuleActionElRedirectEl {
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

impl LbListenerRuleActionElRedirectEl {
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

impl ToListMappable for LbListenerRuleActionElRedirectEl {
    type O = BlockAssignable<LbListenerRuleActionElRedirectEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleActionElRedirectEl {
    #[doc = ""]
    pub status_code: PrimField<String>,
}

impl BuildLbListenerRuleActionElRedirectEl {
    pub fn build(self) -> LbListenerRuleActionElRedirectEl {
        LbListenerRuleActionElRedirectEl {
            host: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
            query: core::default::Default::default(),
            status_code: self.status_code,
        }
    }
}

pub struct LbListenerRuleActionElRedirectElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LbListenerRuleActionElRedirectElRef {
    fn new(shared: StackShared, base: String) -> LbListenerRuleActionElRedirectElRef {
        LbListenerRuleActionElRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleActionElRedirectElRef {
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
struct LbListenerRuleActionElDynamic {
    authenticate_cognito: Option<DynamicBlock<LbListenerRuleActionElAuthenticateCognitoEl>>,
    authenticate_oidc: Option<DynamicBlock<LbListenerRuleActionElAuthenticateOidcEl>>,
    fixed_response: Option<DynamicBlock<LbListenerRuleActionElFixedResponseEl>>,
    forward: Option<DynamicBlock<LbListenerRuleActionElForwardEl>>,
    jwt_validation: Option<DynamicBlock<LbListenerRuleActionElJwtValidationEl>>,
    redirect: Option<DynamicBlock<LbListenerRuleActionElRedirectEl>>,
}

#[derive(Serialize)]
pub struct LbListenerRuleActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_group_arn: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authenticate_cognito: Option<Vec<LbListenerRuleActionElAuthenticateCognitoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authenticate_oidc: Option<Vec<LbListenerRuleActionElAuthenticateOidcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_response: Option<Vec<LbListenerRuleActionElFixedResponseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forward: Option<Vec<LbListenerRuleActionElForwardEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jwt_validation: Option<Vec<LbListenerRuleActionElJwtValidationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect: Option<Vec<LbListenerRuleActionElRedirectEl>>,
    dynamic: LbListenerRuleActionElDynamic,
}

impl LbListenerRuleActionEl {
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
        v: impl Into<BlockAssignable<LbListenerRuleActionElAuthenticateCognitoEl>>,
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
        v: impl Into<BlockAssignable<LbListenerRuleActionElAuthenticateOidcEl>>,
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
    pub fn set_fixed_response(mut self, v: impl Into<BlockAssignable<LbListenerRuleActionElFixedResponseEl>>) -> Self {
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
    pub fn set_forward(mut self, v: impl Into<BlockAssignable<LbListenerRuleActionElForwardEl>>) -> Self {
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
    pub fn set_jwt_validation(mut self, v: impl Into<BlockAssignable<LbListenerRuleActionElJwtValidationEl>>) -> Self {
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
    pub fn set_redirect(mut self, v: impl Into<BlockAssignable<LbListenerRuleActionElRedirectEl>>) -> Self {
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

impl ToListMappable for LbListenerRuleActionEl {
    type O = BlockAssignable<LbListenerRuleActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleActionEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildLbListenerRuleActionEl {
    pub fn build(self) -> LbListenerRuleActionEl {
        LbListenerRuleActionEl {
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

pub struct LbListenerRuleActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LbListenerRuleActionElRef {
    fn new(shared: StackShared, base: String) -> LbListenerRuleActionElRef {
        LbListenerRuleActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleActionElRef {
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
    pub fn authenticate_cognito(&self) -> ListRef<LbListenerRuleActionElAuthenticateCognitoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.authenticate_cognito", self.base))
    }

    #[doc = "Get a reference to the value of field `authenticate_oidc` after provisioning.\n"]
    pub fn authenticate_oidc(&self) -> ListRef<LbListenerRuleActionElAuthenticateOidcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.authenticate_oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `fixed_response` after provisioning.\n"]
    pub fn fixed_response(&self) -> ListRef<LbListenerRuleActionElFixedResponseElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fixed_response", self.base))
    }

    #[doc = "Get a reference to the value of field `forward` after provisioning.\n"]
    pub fn forward(&self) -> ListRef<LbListenerRuleActionElForwardElRef> {
        ListRef::new(self.shared().clone(), format!("{}.forward", self.base))
    }

    #[doc = "Get a reference to the value of field `jwt_validation` after provisioning.\n"]
    pub fn jwt_validation(&self) -> ListRef<LbListenerRuleActionElJwtValidationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.jwt_validation", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> ListRef<LbListenerRuleActionElRedirectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.redirect", self.base))
    }
}

#[derive(Serialize)]
pub struct LbListenerRuleConditionElHostHeaderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    regex_values: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl LbListenerRuleConditionElHostHeaderEl {
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

impl ToListMappable for LbListenerRuleConditionElHostHeaderEl {
    type O = BlockAssignable<LbListenerRuleConditionElHostHeaderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleConditionElHostHeaderEl {}

impl BuildLbListenerRuleConditionElHostHeaderEl {
    pub fn build(self) -> LbListenerRuleConditionElHostHeaderEl {
        LbListenerRuleConditionElHostHeaderEl {
            regex_values: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct LbListenerRuleConditionElHostHeaderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LbListenerRuleConditionElHostHeaderElRef {
    fn new(shared: StackShared, base: String) -> LbListenerRuleConditionElHostHeaderElRef {
        LbListenerRuleConditionElHostHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleConditionElHostHeaderElRef {
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
pub struct LbListenerRuleConditionElHttpHeaderEl {
    http_header_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex_values: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl LbListenerRuleConditionElHttpHeaderEl {
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

impl ToListMappable for LbListenerRuleConditionElHttpHeaderEl {
    type O = BlockAssignable<LbListenerRuleConditionElHttpHeaderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleConditionElHttpHeaderEl {
    #[doc = ""]
    pub http_header_name: PrimField<String>,
}

impl BuildLbListenerRuleConditionElHttpHeaderEl {
    pub fn build(self) -> LbListenerRuleConditionElHttpHeaderEl {
        LbListenerRuleConditionElHttpHeaderEl {
            http_header_name: self.http_header_name,
            regex_values: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct LbListenerRuleConditionElHttpHeaderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LbListenerRuleConditionElHttpHeaderElRef {
    fn new(shared: StackShared, base: String) -> LbListenerRuleConditionElHttpHeaderElRef {
        LbListenerRuleConditionElHttpHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleConditionElHttpHeaderElRef {
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
pub struct LbListenerRuleConditionElHttpRequestMethodEl {
    values: SetField<PrimField<String>>,
}

impl LbListenerRuleConditionElHttpRequestMethodEl { }

impl ToListMappable for LbListenerRuleConditionElHttpRequestMethodEl {
    type O = BlockAssignable<LbListenerRuleConditionElHttpRequestMethodEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleConditionElHttpRequestMethodEl {
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildLbListenerRuleConditionElHttpRequestMethodEl {
    pub fn build(self) -> LbListenerRuleConditionElHttpRequestMethodEl {
        LbListenerRuleConditionElHttpRequestMethodEl { values: self.values }
    }
}

pub struct LbListenerRuleConditionElHttpRequestMethodElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LbListenerRuleConditionElHttpRequestMethodElRef {
    fn new(shared: StackShared, base: String) -> LbListenerRuleConditionElHttpRequestMethodElRef {
        LbListenerRuleConditionElHttpRequestMethodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleConditionElHttpRequestMethodElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct LbListenerRuleConditionElPathPatternEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    regex_values: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl LbListenerRuleConditionElPathPatternEl {
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

impl ToListMappable for LbListenerRuleConditionElPathPatternEl {
    type O = BlockAssignable<LbListenerRuleConditionElPathPatternEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleConditionElPathPatternEl {}

impl BuildLbListenerRuleConditionElPathPatternEl {
    pub fn build(self) -> LbListenerRuleConditionElPathPatternEl {
        LbListenerRuleConditionElPathPatternEl {
            regex_values: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct LbListenerRuleConditionElPathPatternElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LbListenerRuleConditionElPathPatternElRef {
    fn new(shared: StackShared, base: String) -> LbListenerRuleConditionElPathPatternElRef {
        LbListenerRuleConditionElPathPatternElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleConditionElPathPatternElRef {
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
pub struct LbListenerRuleConditionElQueryStringEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    value: PrimField<String>,
}

impl LbListenerRuleConditionElQueryStringEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
}

impl ToListMappable for LbListenerRuleConditionElQueryStringEl {
    type O = BlockAssignable<LbListenerRuleConditionElQueryStringEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleConditionElQueryStringEl {
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildLbListenerRuleConditionElQueryStringEl {
    pub fn build(self) -> LbListenerRuleConditionElQueryStringEl {
        LbListenerRuleConditionElQueryStringEl {
            key: core::default::Default::default(),
            value: self.value,
        }
    }
}

pub struct LbListenerRuleConditionElQueryStringElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LbListenerRuleConditionElQueryStringElRef {
    fn new(shared: StackShared, base: String) -> LbListenerRuleConditionElQueryStringElRef {
        LbListenerRuleConditionElQueryStringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleConditionElQueryStringElRef {
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
pub struct LbListenerRuleConditionElSourceIpEl {
    values: SetField<PrimField<String>>,
}

impl LbListenerRuleConditionElSourceIpEl { }

impl ToListMappable for LbListenerRuleConditionElSourceIpEl {
    type O = BlockAssignable<LbListenerRuleConditionElSourceIpEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleConditionElSourceIpEl {
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildLbListenerRuleConditionElSourceIpEl {
    pub fn build(self) -> LbListenerRuleConditionElSourceIpEl {
        LbListenerRuleConditionElSourceIpEl { values: self.values }
    }
}

pub struct LbListenerRuleConditionElSourceIpElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LbListenerRuleConditionElSourceIpElRef {
    fn new(shared: StackShared, base: String) -> LbListenerRuleConditionElSourceIpElRef {
        LbListenerRuleConditionElSourceIpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleConditionElSourceIpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct LbListenerRuleConditionElDynamic {
    host_header: Option<DynamicBlock<LbListenerRuleConditionElHostHeaderEl>>,
    http_header: Option<DynamicBlock<LbListenerRuleConditionElHttpHeaderEl>>,
    http_request_method: Option<DynamicBlock<LbListenerRuleConditionElHttpRequestMethodEl>>,
    path_pattern: Option<DynamicBlock<LbListenerRuleConditionElPathPatternEl>>,
    query_string: Option<DynamicBlock<LbListenerRuleConditionElQueryStringEl>>,
    source_ip: Option<DynamicBlock<LbListenerRuleConditionElSourceIpEl>>,
}

#[derive(Serialize)]
pub struct LbListenerRuleConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_header: Option<Vec<LbListenerRuleConditionElHostHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_header: Option<Vec<LbListenerRuleConditionElHttpHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_request_method: Option<Vec<LbListenerRuleConditionElHttpRequestMethodEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_pattern: Option<Vec<LbListenerRuleConditionElPathPatternEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_string: Option<Vec<LbListenerRuleConditionElQueryStringEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_ip: Option<Vec<LbListenerRuleConditionElSourceIpEl>>,
    dynamic: LbListenerRuleConditionElDynamic,
}

impl LbListenerRuleConditionEl {
    #[doc = "Set the field `host_header`.\n"]
    pub fn set_host_header(mut self, v: impl Into<BlockAssignable<LbListenerRuleConditionElHostHeaderEl>>) -> Self {
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
    pub fn set_http_header(mut self, v: impl Into<BlockAssignable<LbListenerRuleConditionElHttpHeaderEl>>) -> Self {
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
        v: impl Into<BlockAssignable<LbListenerRuleConditionElHttpRequestMethodEl>>,
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
    pub fn set_path_pattern(mut self, v: impl Into<BlockAssignable<LbListenerRuleConditionElPathPatternEl>>) -> Self {
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
    pub fn set_query_string(mut self, v: impl Into<BlockAssignable<LbListenerRuleConditionElQueryStringEl>>) -> Self {
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
    pub fn set_source_ip(mut self, v: impl Into<BlockAssignable<LbListenerRuleConditionElSourceIpEl>>) -> Self {
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

impl ToListMappable for LbListenerRuleConditionEl {
    type O = BlockAssignable<LbListenerRuleConditionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleConditionEl {}

impl BuildLbListenerRuleConditionEl {
    pub fn build(self) -> LbListenerRuleConditionEl {
        LbListenerRuleConditionEl {
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

pub struct LbListenerRuleConditionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LbListenerRuleConditionElRef {
    fn new(shared: StackShared, base: String) -> LbListenerRuleConditionElRef {
        LbListenerRuleConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host_header` after provisioning.\n"]
    pub fn host_header(&self) -> ListRef<LbListenerRuleConditionElHostHeaderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.host_header", self.base))
    }

    #[doc = "Get a reference to the value of field `http_header` after provisioning.\n"]
    pub fn http_header(&self) -> ListRef<LbListenerRuleConditionElHttpHeaderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_header", self.base))
    }

    #[doc = "Get a reference to the value of field `http_request_method` after provisioning.\n"]
    pub fn http_request_method(&self) -> ListRef<LbListenerRuleConditionElHttpRequestMethodElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_request_method", self.base))
    }

    #[doc = "Get a reference to the value of field `path_pattern` after provisioning.\n"]
    pub fn path_pattern(&self) -> ListRef<LbListenerRuleConditionElPathPatternElRef> {
        ListRef::new(self.shared().clone(), format!("{}.path_pattern", self.base))
    }

    #[doc = "Get a reference to the value of field `source_ip` after provisioning.\n"]
    pub fn source_ip(&self) -> ListRef<LbListenerRuleConditionElSourceIpElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_ip", self.base))
    }
}

#[derive(Serialize)]
pub struct LbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl {
    regex: PrimField<String>,
    replace: PrimField<String>,
}

impl LbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl { }

impl ToListMappable for LbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl {
    type O = BlockAssignable<LbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl {
    #[doc = ""]
    pub regex: PrimField<String>,
    #[doc = ""]
    pub replace: PrimField<String>,
}

impl BuildLbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl {
    pub fn build(self) -> LbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl {
        LbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl {
            regex: self.regex,
            replace: self.replace,
        }
    }
}

pub struct LbListenerRuleTransformElHostHeaderRewriteConfigElRewriteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LbListenerRuleTransformElHostHeaderRewriteConfigElRewriteElRef {
    fn new(shared: StackShared, base: String) -> LbListenerRuleTransformElHostHeaderRewriteConfigElRewriteElRef {
        LbListenerRuleTransformElHostHeaderRewriteConfigElRewriteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleTransformElHostHeaderRewriteConfigElRewriteElRef {
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
struct LbListenerRuleTransformElHostHeaderRewriteConfigElDynamic {
    rewrite: Option<DynamicBlock<LbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl>>,
}

#[derive(Serialize)]
pub struct LbListenerRuleTransformElHostHeaderRewriteConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rewrite: Option<Vec<LbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl>>,
    dynamic: LbListenerRuleTransformElHostHeaderRewriteConfigElDynamic,
}

impl LbListenerRuleTransformElHostHeaderRewriteConfigEl {
    #[doc = "Set the field `rewrite`.\n"]
    pub fn set_rewrite(
        mut self,
        v: impl Into<BlockAssignable<LbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl>>,
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

impl ToListMappable for LbListenerRuleTransformElHostHeaderRewriteConfigEl {
    type O = BlockAssignable<LbListenerRuleTransformElHostHeaderRewriteConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleTransformElHostHeaderRewriteConfigEl {}

impl BuildLbListenerRuleTransformElHostHeaderRewriteConfigEl {
    pub fn build(self) -> LbListenerRuleTransformElHostHeaderRewriteConfigEl {
        LbListenerRuleTransformElHostHeaderRewriteConfigEl {
            rewrite: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct LbListenerRuleTransformElHostHeaderRewriteConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LbListenerRuleTransformElHostHeaderRewriteConfigElRef {
    fn new(shared: StackShared, base: String) -> LbListenerRuleTransformElHostHeaderRewriteConfigElRef {
        LbListenerRuleTransformElHostHeaderRewriteConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleTransformElHostHeaderRewriteConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rewrite` after provisioning.\n"]
    pub fn rewrite(&self) -> ListRef<LbListenerRuleTransformElHostHeaderRewriteConfigElRewriteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rewrite", self.base))
    }
}

#[derive(Serialize)]
pub struct LbListenerRuleTransformElUrlRewriteConfigElRewriteEl {
    regex: PrimField<String>,
    replace: PrimField<String>,
}

impl LbListenerRuleTransformElUrlRewriteConfigElRewriteEl { }

impl ToListMappable for LbListenerRuleTransformElUrlRewriteConfigElRewriteEl {
    type O = BlockAssignable<LbListenerRuleTransformElUrlRewriteConfigElRewriteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleTransformElUrlRewriteConfigElRewriteEl {
    #[doc = ""]
    pub regex: PrimField<String>,
    #[doc = ""]
    pub replace: PrimField<String>,
}

impl BuildLbListenerRuleTransformElUrlRewriteConfigElRewriteEl {
    pub fn build(self) -> LbListenerRuleTransformElUrlRewriteConfigElRewriteEl {
        LbListenerRuleTransformElUrlRewriteConfigElRewriteEl {
            regex: self.regex,
            replace: self.replace,
        }
    }
}

pub struct LbListenerRuleTransformElUrlRewriteConfigElRewriteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LbListenerRuleTransformElUrlRewriteConfigElRewriteElRef {
    fn new(shared: StackShared, base: String) -> LbListenerRuleTransformElUrlRewriteConfigElRewriteElRef {
        LbListenerRuleTransformElUrlRewriteConfigElRewriteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleTransformElUrlRewriteConfigElRewriteElRef {
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
struct LbListenerRuleTransformElUrlRewriteConfigElDynamic {
    rewrite: Option<DynamicBlock<LbListenerRuleTransformElUrlRewriteConfigElRewriteEl>>,
}

#[derive(Serialize)]
pub struct LbListenerRuleTransformElUrlRewriteConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rewrite: Option<Vec<LbListenerRuleTransformElUrlRewriteConfigElRewriteEl>>,
    dynamic: LbListenerRuleTransformElUrlRewriteConfigElDynamic,
}

impl LbListenerRuleTransformElUrlRewriteConfigEl {
    #[doc = "Set the field `rewrite`.\n"]
    pub fn set_rewrite(
        mut self,
        v: impl Into<BlockAssignable<LbListenerRuleTransformElUrlRewriteConfigElRewriteEl>>,
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

impl ToListMappable for LbListenerRuleTransformElUrlRewriteConfigEl {
    type O = BlockAssignable<LbListenerRuleTransformElUrlRewriteConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleTransformElUrlRewriteConfigEl {}

impl BuildLbListenerRuleTransformElUrlRewriteConfigEl {
    pub fn build(self) -> LbListenerRuleTransformElUrlRewriteConfigEl {
        LbListenerRuleTransformElUrlRewriteConfigEl {
            rewrite: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct LbListenerRuleTransformElUrlRewriteConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LbListenerRuleTransformElUrlRewriteConfigElRef {
    fn new(shared: StackShared, base: String) -> LbListenerRuleTransformElUrlRewriteConfigElRef {
        LbListenerRuleTransformElUrlRewriteConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleTransformElUrlRewriteConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rewrite` after provisioning.\n"]
    pub fn rewrite(&self) -> ListRef<LbListenerRuleTransformElUrlRewriteConfigElRewriteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rewrite", self.base))
    }
}

#[derive(Serialize, Default)]
struct LbListenerRuleTransformElDynamic {
    host_header_rewrite_config: Option<DynamicBlock<LbListenerRuleTransformElHostHeaderRewriteConfigEl>>,
    url_rewrite_config: Option<DynamicBlock<LbListenerRuleTransformElUrlRewriteConfigEl>>,
}

#[derive(Serialize)]
pub struct LbListenerRuleTransformEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_header_rewrite_config: Option<Vec<LbListenerRuleTransformElHostHeaderRewriteConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_rewrite_config: Option<Vec<LbListenerRuleTransformElUrlRewriteConfigEl>>,
    dynamic: LbListenerRuleTransformElDynamic,
}

impl LbListenerRuleTransformEl {
    #[doc = "Set the field `host_header_rewrite_config`.\n"]
    pub fn set_host_header_rewrite_config(
        mut self,
        v: impl Into<BlockAssignable<LbListenerRuleTransformElHostHeaderRewriteConfigEl>>,
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
        v: impl Into<BlockAssignable<LbListenerRuleTransformElUrlRewriteConfigEl>>,
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

impl ToListMappable for LbListenerRuleTransformEl {
    type O = BlockAssignable<LbListenerRuleTransformEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleTransformEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildLbListenerRuleTransformEl {
    pub fn build(self) -> LbListenerRuleTransformEl {
        LbListenerRuleTransformEl {
            type_: self.type_,
            host_header_rewrite_config: core::default::Default::default(),
            url_rewrite_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct LbListenerRuleTransformElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LbListenerRuleTransformElRef {
    fn new(shared: StackShared, base: String) -> LbListenerRuleTransformElRef {
        LbListenerRuleTransformElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleTransformElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `host_header_rewrite_config` after provisioning.\n"]
    pub fn host_header_rewrite_config(&self) -> ListRef<LbListenerRuleTransformElHostHeaderRewriteConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.host_header_rewrite_config", self.base))
    }

    #[doc = "Get a reference to the value of field `url_rewrite_config` after provisioning.\n"]
    pub fn url_rewrite_config(&self) -> ListRef<LbListenerRuleTransformElUrlRewriteConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.url_rewrite_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct LbListenerRuleDynamic {
    action: Option<DynamicBlock<LbListenerRuleActionEl>>,
    condition: Option<DynamicBlock<LbListenerRuleConditionEl>>,
    transform: Option<DynamicBlock<LbListenerRuleTransformEl>>,
}
