use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CognitoRiskConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    user_pool_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_takeover_risk_configuration:
        Option<Vec<CognitoRiskConfigurationAccountTakeoverRiskConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compromised_credentials_risk_configuration:
        Option<Vec<CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    risk_exception_configuration: Option<Vec<CognitoRiskConfigurationRiskExceptionConfigurationEl>>,
    dynamic: CognitoRiskConfigurationDynamic,
}
struct CognitoRiskConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CognitoRiskConfigurationData>,
}
#[derive(Clone)]
pub struct CognitoRiskConfiguration(Rc<CognitoRiskConfiguration_>);
impl CognitoRiskConfiguration {
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
    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().client_id = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `account_takeover_risk_configuration`.\n"]
    pub fn set_account_takeover_risk_configuration(
        self,
        v: impl Into<BlockAssignable<CognitoRiskConfigurationAccountTakeoverRiskConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().account_takeover_risk_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .account_takeover_risk_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `compromised_credentials_risk_configuration`.\n"]
    pub fn set_compromised_credentials_risk_configuration(
        self,
        v: impl Into<BlockAssignable<CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0
                    .data
                    .borrow_mut()
                    .compromised_credentials_risk_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .compromised_credentials_risk_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `risk_exception_configuration`.\n"]
    pub fn set_risk_exception_configuration(
        self,
        v: impl Into<BlockAssignable<CognitoRiskConfigurationRiskExceptionConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().risk_exception_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .risk_exception_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_pool_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `account_takeover_risk_configuration` after provisioning.\n"]
    pub fn account_takeover_risk_configuration(
        &self,
    ) -> ListRef<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.account_takeover_risk_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compromised_credentials_risk_configuration` after provisioning.\n"]
    pub fn compromised_credentials_risk_configuration(
        &self,
    ) -> ListRef<CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.compromised_credentials_risk_configuration",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `risk_exception_configuration` after provisioning.\n"]
    pub fn risk_exception_configuration(
        &self,
    ) -> ListRef<CognitoRiskConfigurationRiskExceptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.risk_exception_configuration", self.extract_ref()),
        )
    }
}
impl Referable for CognitoRiskConfiguration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CognitoRiskConfiguration {}
impl ToListMappable for CognitoRiskConfiguration {
    type O = ListRef<CognitoRiskConfigurationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CognitoRiskConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_cognito_risk_configuration".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCognitoRiskConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub user_pool_id: PrimField<String>,
}
impl BuildCognitoRiskConfiguration {
    pub fn build(self, stack: &mut Stack) -> CognitoRiskConfiguration {
        let out = CognitoRiskConfiguration(Rc::new(CognitoRiskConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CognitoRiskConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                client_id: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                user_pool_id: self.user_pool_id,
                account_takeover_risk_configuration: core::default::Default::default(),
                compromised_credentials_risk_configuration: core::default::Default::default(),
                risk_exception_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CognitoRiskConfigurationRef {
    shared: StackShared,
    base: String,
}
impl Ref for CognitoRiskConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CognitoRiskConfigurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_pool_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `account_takeover_risk_configuration` after provisioning.\n"]
    pub fn account_takeover_risk_configuration(
        &self,
    ) -> ListRef<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.account_takeover_risk_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compromised_credentials_risk_configuration` after provisioning.\n"]
    pub fn compromised_credentials_risk_configuration(
        &self,
    ) -> ListRef<CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.compromised_credentials_risk_configuration",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `risk_exception_configuration` after provisioning.\n"]
    pub fn risk_exception_configuration(
        &self,
    ) -> ListRef<CognitoRiskConfigurationRiskExceptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.risk_exception_configuration", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionEl {
    event_action: PrimField<String>,
    notify: PrimField<bool>,
}
impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionEl {}
impl ToListMappable
    for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionEl
{
    type O = BlockAssignable<
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionEl {
    #[doc = ""]
    pub event_action: PrimField<String>,
    #[doc = ""]
    pub notify: PrimField<bool>,
}
impl BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionEl {
    pub fn build(
        self,
    ) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionEl {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionEl {
            event_action: self.event_action,
            notify: self.notify,
        }
    }
}
pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionElRef {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `event_action` after provisioning.\n"]
    pub fn event_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_action", self.base))
    }
    #[doc = "Get a reference to the value of field `notify` after provisioning.\n"]
    pub fn notify(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.notify", self.base))
    }
}
#[derive(Serialize)]
pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionEl {
    event_action: PrimField<String>,
    notify: PrimField<bool>,
}
impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionEl {}
impl ToListMappable
    for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionEl
{
    type O = BlockAssignable<
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionEl {
    #[doc = ""]
    pub event_action: PrimField<String>,
    #[doc = ""]
    pub notify: PrimField<bool>,
}
impl BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionEl {
    pub fn build(
        self,
    ) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionEl {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionEl {
            event_action: self.event_action,
            notify: self.notify,
        }
    }
}
pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionElRef {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `event_action` after provisioning.\n"]
    pub fn event_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_action", self.base))
    }
    #[doc = "Get a reference to the value of field `notify` after provisioning.\n"]
    pub fn notify(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.notify", self.base))
    }
}
#[derive(Serialize)]
pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionEl {
    event_action: PrimField<String>,
    notify: PrimField<bool>,
}
impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionEl {}
impl ToListMappable
    for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionEl
{
    type O = BlockAssignable<
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionEl {
    #[doc = ""]
    pub event_action: PrimField<String>,
    #[doc = ""]
    pub notify: PrimField<bool>,
}
impl BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionEl {
    pub fn build(
        self,
    ) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionEl {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionEl {
            event_action: self.event_action,
            notify: self.notify,
        }
    }
}
pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionElRef {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `event_action` after provisioning.\n"]
    pub fn event_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_action", self.base))
    }
    #[doc = "Get a reference to the value of field `notify` after provisioning.\n"]
    pub fn notify(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.notify", self.base))
    }
}
#[derive(Serialize, Default)]
struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElDynamic {
    high_action: Option<
        DynamicBlock<
            CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionEl,
        >,
    >,
    low_action: Option<
        DynamicBlock<
            CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionEl,
        >,
    >,
    medium_action: Option<
        DynamicBlock<
            CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    high_action: Option<
        Vec<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    low_action:
        Option<Vec<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    medium_action: Option<
        Vec<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionEl>,
    >,
    dynamic: CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElDynamic,
}
impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsEl {
    #[doc = "Set the field `high_action`.\n"]
    pub fn set_high_action(
        mut self,
        v: impl Into<
            BlockAssignable<
                CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.high_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.high_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `low_action`.\n"]
    pub fn set_low_action(
        mut self,
        v: impl Into<
            BlockAssignable<
                CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.low_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.low_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `medium_action`.\n"]
    pub fn set_medium_action(
        mut self,
        v: impl Into<
            BlockAssignable<
                CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.medium_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.medium_action = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsEl {
    type O = BlockAssignable<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsEl {}
impl BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsEl {
    pub fn build(self) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsEl {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsEl {
            high_action: core::default::Default::default(),
            low_action: core::default::Default::default(),
            medium_action: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElRef {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `high_action` after provisioning.\n"]
    pub fn high_action(
        &self,
    ) -> ListRef<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.high_action", self.base))
    }
    #[doc = "Get a reference to the value of field `low_action` after provisioning.\n"]
    pub fn low_action(
        &self,
    ) -> ListRef<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.low_action", self.base))
    }
    #[doc = "Get a reference to the value of field `medium_action` after provisioning.\n"]
    pub fn medium_action(
        &self,
    ) -> ListRef<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.medium_action", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailEl
{
    html_body: PrimField<String>,
    subject: PrimField<String>,
    text_body: PrimField<String>,
}
impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailEl {}
impl ToListMappable
    for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailEl
{
    type O = BlockAssignable<
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailEl
{
    #[doc = ""]
    pub html_body: PrimField<String>,
    #[doc = ""]
    pub subject: PrimField<String>,
    #[doc = ""]
    pub text_body: PrimField<String>,
}
impl
    BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailEl
{
    pub fn build(
        self,
    ) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailEl
    {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailEl { html_body : self . html_body , subject : self . subject , text_body : self . text_body , }
    }
}
pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailElRef { fn new (shared : StackShared , base : String) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailElRef { CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailElRef { shared : shared , base : base . to_string () , } } }
impl
    CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `html_body` after provisioning.\n"]
    pub fn html_body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.html_body", self.base))
    }
    #[doc = "Get a reference to the value of field `subject` after provisioning.\n"]
    pub fn subject(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subject", self.base))
    }
    #[doc = "Get a reference to the value of field `text_body` after provisioning.\n"]
    pub fn text_body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text_body", self.base))
    }
}
#[derive(Serialize)]
pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailEl
{
    html_body: PrimField<String>,
    subject: PrimField<String>,
    text_body: PrimField<String>,
}
impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailEl {}
impl ToListMappable
    for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailEl
{
    type O = BlockAssignable<
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailEl
{
    #[doc = ""]
    pub html_body: PrimField<String>,
    #[doc = ""]
    pub subject: PrimField<String>,
    #[doc = ""]
    pub text_body: PrimField<String>,
}
impl
    BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailEl
{
    pub fn build(
        self,
    ) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailEl
    {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailEl {
            html_body: self.html_body,
            subject: self.subject,
            text_body: self.text_body,
        }
    }
}
pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailElRef
    {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailElRef { shared : shared , base : base . to_string () , }
    }
}
impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `html_body` after provisioning.\n"]
    pub fn html_body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.html_body", self.base))
    }
    #[doc = "Get a reference to the value of field `subject` after provisioning.\n"]
    pub fn subject(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subject", self.base))
    }
    #[doc = "Get a reference to the value of field `text_body` after provisioning.\n"]
    pub fn text_body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text_body", self.base))
    }
}
#[derive(Serialize)]
pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailEl
{
    html_body: PrimField<String>,
    subject: PrimField<String>,
    text_body: PrimField<String>,
}
impl
    CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailEl
{
}
impl ToListMappable for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailEl { type O = BlockAssignable < CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailEl
{
    #[doc = ""]
    pub html_body: PrimField<String>,
    #[doc = ""]
    pub subject: PrimField<String>,
    #[doc = ""]
    pub text_body: PrimField<String>,
}
impl BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailEl { pub fn build (self) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailEl { CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailEl { html_body : self . html_body , subject : self . subject , text_body : self . text_body , } } }
pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailElRef { fn new (shared : StackShared , base : String) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailElRef { CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailElRef { shared : shared , base : base . to_string () , } } }
impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `html_body` after provisioning.\n"] pub fn html_body (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.html_body" , self . base)) } # [doc = "Get a reference to the value of field `subject` after provisioning.\n"] pub fn subject (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.subject" , self . base)) } # [doc = "Get a reference to the value of field `text_body` after provisioning.\n"] pub fn text_body (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.text_body" , self . base)) } }
#[derive(Serialize, Default)]
struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElDynamic { block_email : Option < DynamicBlock < CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailEl >> , mfa_email : Option < DynamicBlock < CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailEl >> , no_action_email : Option < DynamicBlock < CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailEl >> , }
#[derive(Serialize)]
pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationEl { # [serde (skip_serializing_if = "Option::is_none")] from : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] reply_to : Option < PrimField < String > > , source_arn : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] block_email : Option < Vec < CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailEl > > , # [serde (skip_serializing_if = "Option::is_none")] mfa_email : Option < Vec < CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailEl > > , # [serde (skip_serializing_if = "Option::is_none")] no_action_email : Option < Vec < CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailEl > > , dynamic : CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElDynamic , }
impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationEl {
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.from = Some(v.into());
        self
    }
    #[doc = "Set the field `reply_to`.\n"]
    pub fn set_reply_to(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reply_to = Some(v.into());
        self
    }
    #[doc = "Set the field `block_email`.\n"]
    pub fn set_block_email(
        mut self,
        v : impl Into < BlockAssignable < CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.block_email = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.block_email = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `mfa_email`.\n"]
    pub fn set_mfa_email(
        mut self,
        v : impl Into < BlockAssignable < CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mfa_email = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mfa_email = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `no_action_email`.\n"]
    pub fn set_no_action_email(
        mut self,
        v : impl Into < BlockAssignable < CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.no_action_email = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.no_action_email = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationEl
{
    type O = BlockAssignable<
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationEl {
    #[doc = ""]
    pub source_arn: PrimField<String>,
}
impl BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationEl {
    pub fn build(
        self,
    ) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationEl {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationEl {
            from: core::default::Default::default(),
            reply_to: core::default::Default::default(),
            source_arn: self.source_arn,
            block_email: core::default::Default::default(),
            mfa_email: core::default::Default::default(),
            no_action_email: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElRef {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }
    #[doc = "Get a reference to the value of field `reply_to` after provisioning.\n"]
    pub fn reply_to(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reply_to", self.base))
    }
    #[doc = "Get a reference to the value of field `source_arn` after provisioning.\n"]
    pub fn source_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `block_email` after provisioning.\n"]    pub fn block_email (& self) -> ListRef < CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailElRef >{
        ListRef::new(self.shared().clone(), format!("{}.block_email", self.base))
    }
    #[doc = "Get a reference to the value of field `mfa_email` after provisioning.\n"]    pub fn mfa_email (& self) -> ListRef < CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailElRef >{
        ListRef::new(self.shared().clone(), format!("{}.mfa_email", self.base))
    }
    #[doc = "Get a reference to the value of field `no_action_email` after provisioning.\n"]    pub fn no_action_email (& self) -> ListRef < CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.no_action_email", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElDynamic {
    actions:
        Option<DynamicBlock<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsEl>>,
    notify_configuration: Option<
        DynamicBlock<
            CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    actions: Option<Vec<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notify_configuration: Option<
        Vec<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationEl>,
    >,
    dynamic: CognitoRiskConfigurationAccountTakeoverRiskConfigurationElDynamic,
}
impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationEl {
    #[doc = "Set the field `actions`.\n"]
    pub fn set_actions(
        mut self,
        v: impl Into<
            BlockAssignable<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.actions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.actions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `notify_configuration`.\n"]
    pub fn set_notify_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.notify_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.notify_configuration = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CognitoRiskConfigurationAccountTakeoverRiskConfigurationEl {
    type O = BlockAssignable<CognitoRiskConfigurationAccountTakeoverRiskConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationEl {}
impl BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationEl {
    pub fn build(self) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationEl {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationEl {
            actions: core::default::Default::default(),
            notify_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElRef {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(
        &self,
    ) -> ListRef<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }
    #[doc = "Get a reference to the value of field `notify_configuration` after provisioning.\n"]
    pub fn notify_configuration(
        &self,
    ) -> ListRef<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notify_configuration", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsEl {
    event_action: PrimField<String>,
}
impl CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsEl {}
impl ToListMappable for CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsEl {
    type O =
        BlockAssignable<CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsEl {
    #[doc = ""]
    pub event_action: PrimField<String>,
}
impl BuildCognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsEl {
    pub fn build(
        self,
    ) -> CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsEl {
        CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsEl {
            event_action: self.event_action,
        }
    }
}
pub struct CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsElRef {
        CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `event_action` after provisioning.\n"]
    pub fn event_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_action", self.base))
    }
}
#[derive(Serialize, Default)]
struct CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElDynamic {
    actions: Option<
        DynamicBlock<CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsEl>,
    >,
}
#[derive(Serialize)]
pub struct CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    event_filter: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    actions:
        Option<Vec<CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsEl>>,
    dynamic: CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElDynamic,
}
impl CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationEl {
    #[doc = "Set the field `event_filter`.\n"]
    pub fn set_event_filter(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.event_filter = Some(v.into());
        self
    }
    #[doc = "Set the field `actions`.\n"]
    pub fn set_actions(
        mut self,
        v: impl Into<
            BlockAssignable<
                CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.actions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.actions = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationEl {
    type O = BlockAssignable<CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCognitoRiskConfigurationCompromisedCredentialsRiskConfigurationEl {}
impl BuildCognitoRiskConfigurationCompromisedCredentialsRiskConfigurationEl {
    pub fn build(self) -> CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationEl {
        CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationEl {
            event_filter: core::default::Default::default(),
            actions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElRef {
        CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `event_filter` after provisioning.\n"]
    pub fn event_filter(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.event_filter", self.base))
    }
    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(
        &self,
    ) -> ListRef<CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }
}
#[derive(Serialize)]
pub struct CognitoRiskConfigurationRiskExceptionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    blocked_ip_range_list: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skipped_ip_range_list: Option<SetField<PrimField<String>>>,
}
impl CognitoRiskConfigurationRiskExceptionConfigurationEl {
    #[doc = "Set the field `blocked_ip_range_list`.\n"]
    pub fn set_blocked_ip_range_list(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.blocked_ip_range_list = Some(v.into());
        self
    }
    #[doc = "Set the field `skipped_ip_range_list`.\n"]
    pub fn set_skipped_ip_range_list(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.skipped_ip_range_list = Some(v.into());
        self
    }
}
impl ToListMappable for CognitoRiskConfigurationRiskExceptionConfigurationEl {
    type O = BlockAssignable<CognitoRiskConfigurationRiskExceptionConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCognitoRiskConfigurationRiskExceptionConfigurationEl {}
impl BuildCognitoRiskConfigurationRiskExceptionConfigurationEl {
    pub fn build(self) -> CognitoRiskConfigurationRiskExceptionConfigurationEl {
        CognitoRiskConfigurationRiskExceptionConfigurationEl {
            blocked_ip_range_list: core::default::Default::default(),
            skipped_ip_range_list: core::default::Default::default(),
        }
    }
}
pub struct CognitoRiskConfigurationRiskExceptionConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CognitoRiskConfigurationRiskExceptionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CognitoRiskConfigurationRiskExceptionConfigurationElRef {
        CognitoRiskConfigurationRiskExceptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CognitoRiskConfigurationRiskExceptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `blocked_ip_range_list` after provisioning.\n"]
    pub fn blocked_ip_range_list(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.blocked_ip_range_list", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `skipped_ip_range_list` after provisioning.\n"]
    pub fn skipped_ip_range_list(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.skipped_ip_range_list", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CognitoRiskConfigurationDynamic {
    account_takeover_risk_configuration:
        Option<DynamicBlock<CognitoRiskConfigurationAccountTakeoverRiskConfigurationEl>>,
    compromised_credentials_risk_configuration:
        Option<DynamicBlock<CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationEl>>,
    risk_exception_configuration:
        Option<DynamicBlock<CognitoRiskConfigurationRiskExceptionConfigurationEl>>,
}
