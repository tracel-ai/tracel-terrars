use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataLbListenerRuleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    listener_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<DataLbListenerRuleActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Vec<DataLbListenerRuleConditionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transform: Option<Vec<DataLbListenerRuleTransformEl>>,
    dynamic: DataLbListenerRuleDynamic,
}
struct DataLbListenerRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataLbListenerRuleData>,
}
#[derive(Clone)]
pub struct DataLbListenerRule(Rc<DataLbListenerRule_>);
impl DataLbListenerRule {
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
    #[doc = "Set the field `listener_arn`.\n"]
    pub fn set_listener_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().listener_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().priority = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(self, v: impl Into<BlockAssignable<DataLbListenerRuleActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        self,
        v: impl Into<BlockAssignable<DataLbListenerRuleConditionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.condition = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `transform`.\n"]
    pub fn set_transform(
        self,
        v: impl Into<BlockAssignable<DataLbListenerRuleTransformEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().transform = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.transform = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `listener_arn` after provisioning.\n"]
    pub fn listener_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.listener_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<DataLbListenerRuleActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.action", self.extract_ref()),
        )
    }
}
impl Referable for DataLbListenerRule {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataLbListenerRule {}
impl ToListMappable for DataLbListenerRule {
    type O = ListRef<DataLbListenerRuleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataLbListenerRule_ {
    fn extract_datasource_type(&self) -> String {
        "aws_lb_listener_rule".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataLbListenerRule {
    pub tf_id: String,
}
impl BuildDataLbListenerRule {
    pub fn build(self, stack: &mut Stack) -> DataLbListenerRule {
        let out = DataLbListenerRule(Rc::new(DataLbListenerRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataLbListenerRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                arn: core::default::Default::default(),
                listener_arn: core::default::Default::default(),
                priority: core::default::Default::default(),
                region: core::default::Default::default(),
                action: core::default::Default::default(),
                condition: core::default::Default::default(),
                transform: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataLbListenerRuleRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLbListenerRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataLbListenerRuleRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `listener_arn` after provisioning.\n"]
    pub fn listener_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.listener_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<DataLbListenerRuleActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.action", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataLbListenerRuleActionElAuthenticateCognitoEl {}
impl DataLbListenerRuleActionElAuthenticateCognitoEl {}
impl ToListMappable for DataLbListenerRuleActionElAuthenticateCognitoEl {
    type O = BlockAssignable<DataLbListenerRuleActionElAuthenticateCognitoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLbListenerRuleActionElAuthenticateCognitoEl {}
impl BuildDataLbListenerRuleActionElAuthenticateCognitoEl {
    pub fn build(self) -> DataLbListenerRuleActionElAuthenticateCognitoEl {
        DataLbListenerRuleActionElAuthenticateCognitoEl {}
    }
}
pub struct DataLbListenerRuleActionElAuthenticateCognitoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLbListenerRuleActionElAuthenticateCognitoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLbListenerRuleActionElAuthenticateCognitoElRef {
        DataLbListenerRuleActionElAuthenticateCognitoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLbListenerRuleActionElAuthenticateCognitoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authentication_request_extra_params` after provisioning.\n"]
    pub fn authentication_request_extra_params(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.authentication_request_extra_params", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `on_unauthenticated_request` after provisioning.\n"]
    pub fn on_unauthenticated_request(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_unauthenticated_request", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
    #[doc = "Get a reference to the value of field `session_cookie_name` after provisioning.\n"]
    pub fn session_cookie_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_cookie_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `session_timeout` after provisioning.\n"]
    pub fn session_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `user_pool_arn` after provisioning.\n"]
    pub fn user_pool_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_pool_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `user_pool_client_id` after provisioning.\n"]
    pub fn user_pool_client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_pool_client_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `user_pool_domain` after provisioning.\n"]
    pub fn user_pool_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_pool_domain", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLbListenerRuleActionElAuthenticateOidcEl {}
impl DataLbListenerRuleActionElAuthenticateOidcEl {}
impl ToListMappable for DataLbListenerRuleActionElAuthenticateOidcEl {
    type O = BlockAssignable<DataLbListenerRuleActionElAuthenticateOidcEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLbListenerRuleActionElAuthenticateOidcEl {}
impl BuildDataLbListenerRuleActionElAuthenticateOidcEl {
    pub fn build(self) -> DataLbListenerRuleActionElAuthenticateOidcEl {
        DataLbListenerRuleActionElAuthenticateOidcEl {}
    }
}
pub struct DataLbListenerRuleActionElAuthenticateOidcElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLbListenerRuleActionElAuthenticateOidcElRef {
    fn new(shared: StackShared, base: String) -> DataLbListenerRuleActionElAuthenticateOidcElRef {
        DataLbListenerRuleActionElAuthenticateOidcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLbListenerRuleActionElAuthenticateOidcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authentication_request_extra_params` after provisioning.\n"]
    pub fn authentication_request_extra_params(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.authentication_request_extra_params", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `authorization_endpoint` after provisioning.\n"]
    pub fn authorization_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorization_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }
    #[doc = "Get a reference to the value of field `on_unauthenticated_request` after provisioning.\n"]
    pub fn on_unauthenticated_request(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_unauthenticated_request", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
    #[doc = "Get a reference to the value of field `session_cookie_name` after provisioning.\n"]
    pub fn session_cookie_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_cookie_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `session_timeout` after provisioning.\n"]
    pub fn session_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\n"]
    pub fn token_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.token_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `user_info_endpoint` after provisioning.\n"]
    pub fn user_info_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_info_endpoint", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLbListenerRuleActionElFixedResponseEl {}
impl DataLbListenerRuleActionElFixedResponseEl {}
impl ToListMappable for DataLbListenerRuleActionElFixedResponseEl {
    type O = BlockAssignable<DataLbListenerRuleActionElFixedResponseEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLbListenerRuleActionElFixedResponseEl {}
impl BuildDataLbListenerRuleActionElFixedResponseEl {
    pub fn build(self) -> DataLbListenerRuleActionElFixedResponseEl {
        DataLbListenerRuleActionElFixedResponseEl {}
    }
}
pub struct DataLbListenerRuleActionElFixedResponseElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLbListenerRuleActionElFixedResponseElRef {
    fn new(shared: StackShared, base: String) -> DataLbListenerRuleActionElFixedResponseElRef {
        DataLbListenerRuleActionElFixedResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLbListenerRuleActionElFixedResponseElRef {
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
pub struct DataLbListenerRuleActionElForwardElStickinessEl {}
impl DataLbListenerRuleActionElForwardElStickinessEl {}
impl ToListMappable for DataLbListenerRuleActionElForwardElStickinessEl {
    type O = BlockAssignable<DataLbListenerRuleActionElForwardElStickinessEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLbListenerRuleActionElForwardElStickinessEl {}
impl BuildDataLbListenerRuleActionElForwardElStickinessEl {
    pub fn build(self) -> DataLbListenerRuleActionElForwardElStickinessEl {
        DataLbListenerRuleActionElForwardElStickinessEl {}
    }
}
pub struct DataLbListenerRuleActionElForwardElStickinessElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLbListenerRuleActionElForwardElStickinessElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLbListenerRuleActionElForwardElStickinessElRef {
        DataLbListenerRuleActionElForwardElStickinessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLbListenerRuleActionElForwardElStickinessElRef {
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
pub struct DataLbListenerRuleActionElForwardElTargetGroupEl {}
impl DataLbListenerRuleActionElForwardElTargetGroupEl {}
impl ToListMappable for DataLbListenerRuleActionElForwardElTargetGroupEl {
    type O = BlockAssignable<DataLbListenerRuleActionElForwardElTargetGroupEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLbListenerRuleActionElForwardElTargetGroupEl {}
impl BuildDataLbListenerRuleActionElForwardElTargetGroupEl {
    pub fn build(self) -> DataLbListenerRuleActionElForwardElTargetGroupEl {
        DataLbListenerRuleActionElForwardElTargetGroupEl {}
    }
}
pub struct DataLbListenerRuleActionElForwardElTargetGroupElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLbListenerRuleActionElForwardElTargetGroupElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLbListenerRuleActionElForwardElTargetGroupElRef {
        DataLbListenerRuleActionElForwardElTargetGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLbListenerRuleActionElForwardElTargetGroupElRef {
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
struct DataLbListenerRuleActionElForwardElDynamic {
    stickiness: Option<DynamicBlock<DataLbListenerRuleActionElForwardElStickinessEl>>,
    target_group: Option<DynamicBlock<DataLbListenerRuleActionElForwardElTargetGroupEl>>,
}
#[derive(Serialize)]
pub struct DataLbListenerRuleActionElForwardEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    stickiness: Option<Vec<DataLbListenerRuleActionElForwardElStickinessEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_group: Option<Vec<DataLbListenerRuleActionElForwardElTargetGroupEl>>,
    dynamic: DataLbListenerRuleActionElForwardElDynamic,
}
impl DataLbListenerRuleActionElForwardEl {
    #[doc = "Set the field `stickiness`.\n"]
    pub fn set_stickiness(
        mut self,
        v: impl Into<BlockAssignable<DataLbListenerRuleActionElForwardElStickinessEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.stickiness = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.stickiness = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `target_group`.\n"]
    pub fn set_target_group(
        mut self,
        v: impl Into<BlockAssignable<DataLbListenerRuleActionElForwardElTargetGroupEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target_group = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target_group = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLbListenerRuleActionElForwardEl {
    type O = BlockAssignable<DataLbListenerRuleActionElForwardEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLbListenerRuleActionElForwardEl {}
impl BuildDataLbListenerRuleActionElForwardEl {
    pub fn build(self) -> DataLbListenerRuleActionElForwardEl {
        DataLbListenerRuleActionElForwardEl {
            stickiness: core::default::Default::default(),
            target_group: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLbListenerRuleActionElForwardElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLbListenerRuleActionElForwardElRef {
    fn new(shared: StackShared, base: String) -> DataLbListenerRuleActionElForwardElRef {
        DataLbListenerRuleActionElForwardElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLbListenerRuleActionElForwardElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `stickiness` after provisioning.\n"]
    pub fn stickiness(&self) -> ListRef<DataLbListenerRuleActionElForwardElStickinessElRef> {
        ListRef::new(self.shared().clone(), format!("{}.stickiness", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLbListenerRuleActionElJwtValidationElAdditionalClaimEl {}
impl DataLbListenerRuleActionElJwtValidationElAdditionalClaimEl {}
impl ToListMappable for DataLbListenerRuleActionElJwtValidationElAdditionalClaimEl {
    type O = BlockAssignable<DataLbListenerRuleActionElJwtValidationElAdditionalClaimEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLbListenerRuleActionElJwtValidationElAdditionalClaimEl {}
impl BuildDataLbListenerRuleActionElJwtValidationElAdditionalClaimEl {
    pub fn build(self) -> DataLbListenerRuleActionElJwtValidationElAdditionalClaimEl {
        DataLbListenerRuleActionElJwtValidationElAdditionalClaimEl {}
    }
}
pub struct DataLbListenerRuleActionElJwtValidationElAdditionalClaimElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLbListenerRuleActionElJwtValidationElAdditionalClaimElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLbListenerRuleActionElJwtValidationElAdditionalClaimElRef {
        DataLbListenerRuleActionElJwtValidationElAdditionalClaimElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLbListenerRuleActionElJwtValidationElAdditionalClaimElRef {
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
struct DataLbListenerRuleActionElJwtValidationElDynamic {
    additional_claim:
        Option<DynamicBlock<DataLbListenerRuleActionElJwtValidationElAdditionalClaimEl>>,
}
#[derive(Serialize)]
pub struct DataLbListenerRuleActionElJwtValidationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_claim: Option<Vec<DataLbListenerRuleActionElJwtValidationElAdditionalClaimEl>>,
    dynamic: DataLbListenerRuleActionElJwtValidationElDynamic,
}
impl DataLbListenerRuleActionElJwtValidationEl {
    #[doc = "Set the field `additional_claim`.\n"]
    pub fn set_additional_claim(
        mut self,
        v: impl Into<BlockAssignable<DataLbListenerRuleActionElJwtValidationElAdditionalClaimEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.additional_claim = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_claim = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLbListenerRuleActionElJwtValidationEl {
    type O = BlockAssignable<DataLbListenerRuleActionElJwtValidationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLbListenerRuleActionElJwtValidationEl {}
impl BuildDataLbListenerRuleActionElJwtValidationEl {
    pub fn build(self) -> DataLbListenerRuleActionElJwtValidationEl {
        DataLbListenerRuleActionElJwtValidationEl {
            additional_claim: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLbListenerRuleActionElJwtValidationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLbListenerRuleActionElJwtValidationElRef {
    fn new(shared: StackShared, base: String) -> DataLbListenerRuleActionElJwtValidationElRef {
        DataLbListenerRuleActionElJwtValidationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLbListenerRuleActionElJwtValidationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }
    #[doc = "Get a reference to the value of field `jwks_endpoint` after provisioning.\n"]
    pub fn jwks_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.jwks_endpoint", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLbListenerRuleActionElRedirectEl {}
impl DataLbListenerRuleActionElRedirectEl {}
impl ToListMappable for DataLbListenerRuleActionElRedirectEl {
    type O = BlockAssignable<DataLbListenerRuleActionElRedirectEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLbListenerRuleActionElRedirectEl {}
impl BuildDataLbListenerRuleActionElRedirectEl {
    pub fn build(self) -> DataLbListenerRuleActionElRedirectEl {
        DataLbListenerRuleActionElRedirectEl {}
    }
}
pub struct DataLbListenerRuleActionElRedirectElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLbListenerRuleActionElRedirectElRef {
    fn new(shared: StackShared, base: String) -> DataLbListenerRuleActionElRedirectElRef {
        DataLbListenerRuleActionElRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLbListenerRuleActionElRedirectElRef {
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
struct DataLbListenerRuleActionElDynamic {
    authenticate_cognito: Option<DynamicBlock<DataLbListenerRuleActionElAuthenticateCognitoEl>>,
    authenticate_oidc: Option<DynamicBlock<DataLbListenerRuleActionElAuthenticateOidcEl>>,
    fixed_response: Option<DynamicBlock<DataLbListenerRuleActionElFixedResponseEl>>,
    forward: Option<DynamicBlock<DataLbListenerRuleActionElForwardEl>>,
    jwt_validation: Option<DynamicBlock<DataLbListenerRuleActionElJwtValidationEl>>,
    redirect: Option<DynamicBlock<DataLbListenerRuleActionElRedirectEl>>,
}
#[derive(Serialize)]
pub struct DataLbListenerRuleActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authenticate_cognito: Option<Vec<DataLbListenerRuleActionElAuthenticateCognitoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authenticate_oidc: Option<Vec<DataLbListenerRuleActionElAuthenticateOidcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_response: Option<Vec<DataLbListenerRuleActionElFixedResponseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forward: Option<Vec<DataLbListenerRuleActionElForwardEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jwt_validation: Option<Vec<DataLbListenerRuleActionElJwtValidationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect: Option<Vec<DataLbListenerRuleActionElRedirectEl>>,
    dynamic: DataLbListenerRuleActionElDynamic,
}
impl DataLbListenerRuleActionEl {
    #[doc = "Set the field `authenticate_cognito`.\n"]
    pub fn set_authenticate_cognito(
        mut self,
        v: impl Into<BlockAssignable<DataLbListenerRuleActionElAuthenticateCognitoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.authenticate_cognito = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.authenticate_cognito = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `authenticate_oidc`.\n"]
    pub fn set_authenticate_oidc(
        mut self,
        v: impl Into<BlockAssignable<DataLbListenerRuleActionElAuthenticateOidcEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.authenticate_oidc = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.authenticate_oidc = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `fixed_response`.\n"]
    pub fn set_fixed_response(
        mut self,
        v: impl Into<BlockAssignable<DataLbListenerRuleActionElFixedResponseEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fixed_response = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fixed_response = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `forward`.\n"]
    pub fn set_forward(
        mut self,
        v: impl Into<BlockAssignable<DataLbListenerRuleActionElForwardEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.forward = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.forward = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `jwt_validation`.\n"]
    pub fn set_jwt_validation(
        mut self,
        v: impl Into<BlockAssignable<DataLbListenerRuleActionElJwtValidationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.jwt_validation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.jwt_validation = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `redirect`.\n"]
    pub fn set_redirect(
        mut self,
        v: impl Into<BlockAssignable<DataLbListenerRuleActionElRedirectEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.redirect = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.redirect = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLbListenerRuleActionEl {
    type O = BlockAssignable<DataLbListenerRuleActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLbListenerRuleActionEl {}
impl BuildDataLbListenerRuleActionEl {
    pub fn build(self) -> DataLbListenerRuleActionEl {
        DataLbListenerRuleActionEl {
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
pub struct DataLbListenerRuleActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLbListenerRuleActionElRef {
    fn new(shared: StackShared, base: String) -> DataLbListenerRuleActionElRef {
        DataLbListenerRuleActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLbListenerRuleActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `order` after provisioning.\n"]
    pub fn order(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `authenticate_cognito` after provisioning.\n"]
    pub fn authenticate_cognito(
        &self,
    ) -> ListRef<DataLbListenerRuleActionElAuthenticateCognitoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authenticate_cognito", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `authenticate_oidc` after provisioning.\n"]
    pub fn authenticate_oidc(&self) -> ListRef<DataLbListenerRuleActionElAuthenticateOidcElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authenticate_oidc", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fixed_response` after provisioning.\n"]
    pub fn fixed_response(&self) -> ListRef<DataLbListenerRuleActionElFixedResponseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fixed_response", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `forward` after provisioning.\n"]
    pub fn forward(&self) -> ListRef<DataLbListenerRuleActionElForwardElRef> {
        ListRef::new(self.shared().clone(), format!("{}.forward", self.base))
    }
    #[doc = "Get a reference to the value of field `jwt_validation` after provisioning.\n"]
    pub fn jwt_validation(&self) -> ListRef<DataLbListenerRuleActionElJwtValidationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.jwt_validation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> ListRef<DataLbListenerRuleActionElRedirectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.redirect", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLbListenerRuleConditionElHostHeaderEl {}
impl DataLbListenerRuleConditionElHostHeaderEl {}
impl ToListMappable for DataLbListenerRuleConditionElHostHeaderEl {
    type O = BlockAssignable<DataLbListenerRuleConditionElHostHeaderEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLbListenerRuleConditionElHostHeaderEl {}
impl BuildDataLbListenerRuleConditionElHostHeaderEl {
    pub fn build(self) -> DataLbListenerRuleConditionElHostHeaderEl {
        DataLbListenerRuleConditionElHostHeaderEl {}
    }
}
pub struct DataLbListenerRuleConditionElHostHeaderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLbListenerRuleConditionElHostHeaderElRef {
    fn new(shared: StackShared, base: String) -> DataLbListenerRuleConditionElHostHeaderElRef {
        DataLbListenerRuleConditionElHostHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLbListenerRuleConditionElHostHeaderElRef {
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
pub struct DataLbListenerRuleConditionElHttpHeaderEl {}
impl DataLbListenerRuleConditionElHttpHeaderEl {}
impl ToListMappable for DataLbListenerRuleConditionElHttpHeaderEl {
    type O = BlockAssignable<DataLbListenerRuleConditionElHttpHeaderEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLbListenerRuleConditionElHttpHeaderEl {}
impl BuildDataLbListenerRuleConditionElHttpHeaderEl {
    pub fn build(self) -> DataLbListenerRuleConditionElHttpHeaderEl {
        DataLbListenerRuleConditionElHttpHeaderEl {}
    }
}
pub struct DataLbListenerRuleConditionElHttpHeaderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLbListenerRuleConditionElHttpHeaderElRef {
    fn new(shared: StackShared, base: String) -> DataLbListenerRuleConditionElHttpHeaderElRef {
        DataLbListenerRuleConditionElHttpHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLbListenerRuleConditionElHttpHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `http_header_name` after provisioning.\n"]
    pub fn http_header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_header_name", self.base),
        )
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
pub struct DataLbListenerRuleConditionElHttpRequestMethodEl {}
impl DataLbListenerRuleConditionElHttpRequestMethodEl {}
impl ToListMappable for DataLbListenerRuleConditionElHttpRequestMethodEl {
    type O = BlockAssignable<DataLbListenerRuleConditionElHttpRequestMethodEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLbListenerRuleConditionElHttpRequestMethodEl {}
impl BuildDataLbListenerRuleConditionElHttpRequestMethodEl {
    pub fn build(self) -> DataLbListenerRuleConditionElHttpRequestMethodEl {
        DataLbListenerRuleConditionElHttpRequestMethodEl {}
    }
}
pub struct DataLbListenerRuleConditionElHttpRequestMethodElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLbListenerRuleConditionElHttpRequestMethodElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLbListenerRuleConditionElHttpRequestMethodElRef {
        DataLbListenerRuleConditionElHttpRequestMethodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLbListenerRuleConditionElHttpRequestMethodElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLbListenerRuleConditionElPathPatternEl {}
impl DataLbListenerRuleConditionElPathPatternEl {}
impl ToListMappable for DataLbListenerRuleConditionElPathPatternEl {
    type O = BlockAssignable<DataLbListenerRuleConditionElPathPatternEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLbListenerRuleConditionElPathPatternEl {}
impl BuildDataLbListenerRuleConditionElPathPatternEl {
    pub fn build(self) -> DataLbListenerRuleConditionElPathPatternEl {
        DataLbListenerRuleConditionElPathPatternEl {}
    }
}
pub struct DataLbListenerRuleConditionElPathPatternElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLbListenerRuleConditionElPathPatternElRef {
    fn new(shared: StackShared, base: String) -> DataLbListenerRuleConditionElPathPatternElRef {
        DataLbListenerRuleConditionElPathPatternElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLbListenerRuleConditionElPathPatternElRef {
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
pub struct DataLbListenerRuleConditionElQueryStringElValuesEl {}
impl DataLbListenerRuleConditionElQueryStringElValuesEl {}
impl ToListMappable for DataLbListenerRuleConditionElQueryStringElValuesEl {
    type O = BlockAssignable<DataLbListenerRuleConditionElQueryStringElValuesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLbListenerRuleConditionElQueryStringElValuesEl {}
impl BuildDataLbListenerRuleConditionElQueryStringElValuesEl {
    pub fn build(self) -> DataLbListenerRuleConditionElQueryStringElValuesEl {
        DataLbListenerRuleConditionElQueryStringElValuesEl {}
    }
}
pub struct DataLbListenerRuleConditionElQueryStringElValuesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLbListenerRuleConditionElQueryStringElValuesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLbListenerRuleConditionElQueryStringElValuesElRef {
        DataLbListenerRuleConditionElQueryStringElValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLbListenerRuleConditionElQueryStringElValuesElRef {
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
#[derive(Serialize, Default)]
struct DataLbListenerRuleConditionElQueryStringElDynamic {
    values: Option<DynamicBlock<DataLbListenerRuleConditionElQueryStringElValuesEl>>,
}
#[derive(Serialize)]
pub struct DataLbListenerRuleConditionElQueryStringEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<Vec<DataLbListenerRuleConditionElQueryStringElValuesEl>>,
    dynamic: DataLbListenerRuleConditionElQueryStringElDynamic,
}
impl DataLbListenerRuleConditionElQueryStringEl {
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl Into<BlockAssignable<DataLbListenerRuleConditionElQueryStringElValuesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.values = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.values = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLbListenerRuleConditionElQueryStringEl {
    type O = BlockAssignable<DataLbListenerRuleConditionElQueryStringEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLbListenerRuleConditionElQueryStringEl {}
impl BuildDataLbListenerRuleConditionElQueryStringEl {
    pub fn build(self) -> DataLbListenerRuleConditionElQueryStringEl {
        DataLbListenerRuleConditionElQueryStringEl {
            values: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLbListenerRuleConditionElQueryStringElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLbListenerRuleConditionElQueryStringElRef {
    fn new(shared: StackShared, base: String) -> DataLbListenerRuleConditionElQueryStringElRef {
        DataLbListenerRuleConditionElQueryStringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLbListenerRuleConditionElQueryStringElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DataLbListenerRuleConditionElSourceIpEl {}
impl DataLbListenerRuleConditionElSourceIpEl {}
impl ToListMappable for DataLbListenerRuleConditionElSourceIpEl {
    type O = BlockAssignable<DataLbListenerRuleConditionElSourceIpEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLbListenerRuleConditionElSourceIpEl {}
impl BuildDataLbListenerRuleConditionElSourceIpEl {
    pub fn build(self) -> DataLbListenerRuleConditionElSourceIpEl {
        DataLbListenerRuleConditionElSourceIpEl {}
    }
}
pub struct DataLbListenerRuleConditionElSourceIpElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLbListenerRuleConditionElSourceIpElRef {
    fn new(shared: StackShared, base: String) -> DataLbListenerRuleConditionElSourceIpElRef {
        DataLbListenerRuleConditionElSourceIpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLbListenerRuleConditionElSourceIpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataLbListenerRuleConditionElDynamic {
    host_header: Option<DynamicBlock<DataLbListenerRuleConditionElHostHeaderEl>>,
    http_header: Option<DynamicBlock<DataLbListenerRuleConditionElHttpHeaderEl>>,
    http_request_method: Option<DynamicBlock<DataLbListenerRuleConditionElHttpRequestMethodEl>>,
    path_pattern: Option<DynamicBlock<DataLbListenerRuleConditionElPathPatternEl>>,
    query_string: Option<DynamicBlock<DataLbListenerRuleConditionElQueryStringEl>>,
    source_ip: Option<DynamicBlock<DataLbListenerRuleConditionElSourceIpEl>>,
}
#[derive(Serialize)]
pub struct DataLbListenerRuleConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_header: Option<Vec<DataLbListenerRuleConditionElHostHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_header: Option<Vec<DataLbListenerRuleConditionElHttpHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_request_method: Option<Vec<DataLbListenerRuleConditionElHttpRequestMethodEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_pattern: Option<Vec<DataLbListenerRuleConditionElPathPatternEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_string: Option<Vec<DataLbListenerRuleConditionElQueryStringEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_ip: Option<Vec<DataLbListenerRuleConditionElSourceIpEl>>,
    dynamic: DataLbListenerRuleConditionElDynamic,
}
impl DataLbListenerRuleConditionEl {
    #[doc = "Set the field `host_header`.\n"]
    pub fn set_host_header(
        mut self,
        v: impl Into<BlockAssignable<DataLbListenerRuleConditionElHostHeaderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.host_header = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.host_header = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `http_header`.\n"]
    pub fn set_http_header(
        mut self,
        v: impl Into<BlockAssignable<DataLbListenerRuleConditionElHttpHeaderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_header = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_header = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `http_request_method`.\n"]
    pub fn set_http_request_method(
        mut self,
        v: impl Into<BlockAssignable<DataLbListenerRuleConditionElHttpRequestMethodEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_request_method = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_request_method = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `path_pattern`.\n"]
    pub fn set_path_pattern(
        mut self,
        v: impl Into<BlockAssignable<DataLbListenerRuleConditionElPathPatternEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.path_pattern = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.path_pattern = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `query_string`.\n"]
    pub fn set_query_string(
        mut self,
        v: impl Into<BlockAssignable<DataLbListenerRuleConditionElQueryStringEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query_string = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query_string = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source_ip`.\n"]
    pub fn set_source_ip(
        mut self,
        v: impl Into<BlockAssignable<DataLbListenerRuleConditionElSourceIpEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_ip = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_ip = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLbListenerRuleConditionEl {
    type O = BlockAssignable<DataLbListenerRuleConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLbListenerRuleConditionEl {}
impl BuildDataLbListenerRuleConditionEl {
    pub fn build(self) -> DataLbListenerRuleConditionEl {
        DataLbListenerRuleConditionEl {
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
pub struct DataLbListenerRuleConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLbListenerRuleConditionElRef {
    fn new(shared: StackShared, base: String) -> DataLbListenerRuleConditionElRef {
        DataLbListenerRuleConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLbListenerRuleConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host_header` after provisioning.\n"]
    pub fn host_header(&self) -> ListRef<DataLbListenerRuleConditionElHostHeaderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.host_header", self.base))
    }
    #[doc = "Get a reference to the value of field `http_header` after provisioning.\n"]
    pub fn http_header(&self) -> ListRef<DataLbListenerRuleConditionElHttpHeaderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_header", self.base))
    }
    #[doc = "Get a reference to the value of field `http_request_method` after provisioning.\n"]
    pub fn http_request_method(
        &self,
    ) -> ListRef<DataLbListenerRuleConditionElHttpRequestMethodElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http_request_method", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `path_pattern` after provisioning.\n"]
    pub fn path_pattern(&self) -> ListRef<DataLbListenerRuleConditionElPathPatternElRef> {
        ListRef::new(self.shared().clone(), format!("{}.path_pattern", self.base))
    }
    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(&self) -> ListRef<DataLbListenerRuleConditionElQueryStringElRef> {
        ListRef::new(self.shared().clone(), format!("{}.query_string", self.base))
    }
    #[doc = "Get a reference to the value of field `source_ip` after provisioning.\n"]
    pub fn source_ip(&self) -> ListRef<DataLbListenerRuleConditionElSourceIpElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_ip", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl {}
impl DataLbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl {}
impl ToListMappable for DataLbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl {
    type O = BlockAssignable<DataLbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl {}
impl BuildDataLbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl {
    pub fn build(self) -> DataLbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl {
        DataLbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl {}
    }
}
pub struct DataLbListenerRuleTransformElHostHeaderRewriteConfigElRewriteElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLbListenerRuleTransformElHostHeaderRewriteConfigElRewriteElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLbListenerRuleTransformElHostHeaderRewriteConfigElRewriteElRef {
        DataLbListenerRuleTransformElHostHeaderRewriteConfigElRewriteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLbListenerRuleTransformElHostHeaderRewriteConfigElRewriteElRef {
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
struct DataLbListenerRuleTransformElHostHeaderRewriteConfigElDynamic {
    rewrite: Option<DynamicBlock<DataLbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl>>,
}
#[derive(Serialize)]
pub struct DataLbListenerRuleTransformElHostHeaderRewriteConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rewrite: Option<Vec<DataLbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl>>,
    dynamic: DataLbListenerRuleTransformElHostHeaderRewriteConfigElDynamic,
}
impl DataLbListenerRuleTransformElHostHeaderRewriteConfigEl {
    #[doc = "Set the field `rewrite`.\n"]
    pub fn set_rewrite(
        mut self,
        v: impl Into<BlockAssignable<DataLbListenerRuleTransformElHostHeaderRewriteConfigElRewriteEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rewrite = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rewrite = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLbListenerRuleTransformElHostHeaderRewriteConfigEl {
    type O = BlockAssignable<DataLbListenerRuleTransformElHostHeaderRewriteConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLbListenerRuleTransformElHostHeaderRewriteConfigEl {}
impl BuildDataLbListenerRuleTransformElHostHeaderRewriteConfigEl {
    pub fn build(self) -> DataLbListenerRuleTransformElHostHeaderRewriteConfigEl {
        DataLbListenerRuleTransformElHostHeaderRewriteConfigEl {
            rewrite: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLbListenerRuleTransformElHostHeaderRewriteConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLbListenerRuleTransformElHostHeaderRewriteConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLbListenerRuleTransformElHostHeaderRewriteConfigElRef {
        DataLbListenerRuleTransformElHostHeaderRewriteConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLbListenerRuleTransformElHostHeaderRewriteConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `rewrite` after provisioning.\n"]
    pub fn rewrite(
        &self,
    ) -> ListRef<DataLbListenerRuleTransformElHostHeaderRewriteConfigElRewriteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rewrite", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLbListenerRuleTransformElUrlRewriteConfigElRewriteEl {}
impl DataLbListenerRuleTransformElUrlRewriteConfigElRewriteEl {}
impl ToListMappable for DataLbListenerRuleTransformElUrlRewriteConfigElRewriteEl {
    type O = BlockAssignable<DataLbListenerRuleTransformElUrlRewriteConfigElRewriteEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLbListenerRuleTransformElUrlRewriteConfigElRewriteEl {}
impl BuildDataLbListenerRuleTransformElUrlRewriteConfigElRewriteEl {
    pub fn build(self) -> DataLbListenerRuleTransformElUrlRewriteConfigElRewriteEl {
        DataLbListenerRuleTransformElUrlRewriteConfigElRewriteEl {}
    }
}
pub struct DataLbListenerRuleTransformElUrlRewriteConfigElRewriteElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLbListenerRuleTransformElUrlRewriteConfigElRewriteElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLbListenerRuleTransformElUrlRewriteConfigElRewriteElRef {
        DataLbListenerRuleTransformElUrlRewriteConfigElRewriteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLbListenerRuleTransformElUrlRewriteConfigElRewriteElRef {
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
struct DataLbListenerRuleTransformElUrlRewriteConfigElDynamic {
    rewrite: Option<DynamicBlock<DataLbListenerRuleTransformElUrlRewriteConfigElRewriteEl>>,
}
#[derive(Serialize)]
pub struct DataLbListenerRuleTransformElUrlRewriteConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rewrite: Option<Vec<DataLbListenerRuleTransformElUrlRewriteConfigElRewriteEl>>,
    dynamic: DataLbListenerRuleTransformElUrlRewriteConfigElDynamic,
}
impl DataLbListenerRuleTransformElUrlRewriteConfigEl {
    #[doc = "Set the field `rewrite`.\n"]
    pub fn set_rewrite(
        mut self,
        v: impl Into<BlockAssignable<DataLbListenerRuleTransformElUrlRewriteConfigElRewriteEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rewrite = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rewrite = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLbListenerRuleTransformElUrlRewriteConfigEl {
    type O = BlockAssignable<DataLbListenerRuleTransformElUrlRewriteConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLbListenerRuleTransformElUrlRewriteConfigEl {}
impl BuildDataLbListenerRuleTransformElUrlRewriteConfigEl {
    pub fn build(self) -> DataLbListenerRuleTransformElUrlRewriteConfigEl {
        DataLbListenerRuleTransformElUrlRewriteConfigEl {
            rewrite: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLbListenerRuleTransformElUrlRewriteConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLbListenerRuleTransformElUrlRewriteConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLbListenerRuleTransformElUrlRewriteConfigElRef {
        DataLbListenerRuleTransformElUrlRewriteConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLbListenerRuleTransformElUrlRewriteConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `rewrite` after provisioning.\n"]
    pub fn rewrite(&self) -> ListRef<DataLbListenerRuleTransformElUrlRewriteConfigElRewriteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rewrite", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataLbListenerRuleTransformElDynamic {
    host_header_rewrite_config:
        Option<DynamicBlock<DataLbListenerRuleTransformElHostHeaderRewriteConfigEl>>,
    url_rewrite_config: Option<DynamicBlock<DataLbListenerRuleTransformElUrlRewriteConfigEl>>,
}
#[derive(Serialize)]
pub struct DataLbListenerRuleTransformEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_header_rewrite_config: Option<Vec<DataLbListenerRuleTransformElHostHeaderRewriteConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_rewrite_config: Option<Vec<DataLbListenerRuleTransformElUrlRewriteConfigEl>>,
    dynamic: DataLbListenerRuleTransformElDynamic,
}
impl DataLbListenerRuleTransformEl {
    #[doc = "Set the field `host_header_rewrite_config`.\n"]
    pub fn set_host_header_rewrite_config(
        mut self,
        v: impl Into<BlockAssignable<DataLbListenerRuleTransformElHostHeaderRewriteConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.host_header_rewrite_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.host_header_rewrite_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `url_rewrite_config`.\n"]
    pub fn set_url_rewrite_config(
        mut self,
        v: impl Into<BlockAssignable<DataLbListenerRuleTransformElUrlRewriteConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.url_rewrite_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.url_rewrite_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLbListenerRuleTransformEl {
    type O = BlockAssignable<DataLbListenerRuleTransformEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLbListenerRuleTransformEl {}
impl BuildDataLbListenerRuleTransformEl {
    pub fn build(self) -> DataLbListenerRuleTransformEl {
        DataLbListenerRuleTransformEl {
            host_header_rewrite_config: core::default::Default::default(),
            url_rewrite_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLbListenerRuleTransformElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLbListenerRuleTransformElRef {
    fn new(shared: StackShared, base: String) -> DataLbListenerRuleTransformElRef {
        DataLbListenerRuleTransformElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLbListenerRuleTransformElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `host_header_rewrite_config` after provisioning.\n"]
    pub fn host_header_rewrite_config(
        &self,
    ) -> ListRef<DataLbListenerRuleTransformElHostHeaderRewriteConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.host_header_rewrite_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `url_rewrite_config` after provisioning.\n"]
    pub fn url_rewrite_config(
        &self,
    ) -> ListRef<DataLbListenerRuleTransformElUrlRewriteConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.url_rewrite_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataLbListenerRuleDynamic {
    action: Option<DynamicBlock<DataLbListenerRuleActionEl>>,
    condition: Option<DynamicBlock<DataLbListenerRuleConditionEl>>,
    transform: Option<DynamicBlock<DataLbListenerRuleTransformEl>>,
}
