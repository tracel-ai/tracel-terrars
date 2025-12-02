use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SesReceiptRuleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recipients: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    rule_set_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scan_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    add_header_action: Option<Vec<SesReceiptRuleAddHeaderActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bounce_action: Option<Vec<SesReceiptRuleBounceActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_action: Option<Vec<SesReceiptRuleLambdaActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_action: Option<Vec<SesReceiptRuleS3ActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sns_action: Option<Vec<SesReceiptRuleSnsActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_action: Option<Vec<SesReceiptRuleStopActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workmail_action: Option<Vec<SesReceiptRuleWorkmailActionEl>>,
    dynamic: SesReceiptRuleDynamic,
}
struct SesReceiptRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SesReceiptRuleData>,
}
#[derive(Clone)]
pub struct SesReceiptRule(Rc<SesReceiptRule_>);
impl SesReceiptRule {
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
    #[doc = "Set the field `after`.\n"]
    pub fn set_after(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().after = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `recipients`.\n"]
    pub fn set_recipients(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().recipients = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `scan_enabled`.\n"]
    pub fn set_scan_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().scan_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `tls_policy`.\n"]
    pub fn set_tls_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tls_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `add_header_action`.\n"]
    pub fn set_add_header_action(
        self,
        v: impl Into<BlockAssignable<SesReceiptRuleAddHeaderActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().add_header_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.add_header_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `bounce_action`.\n"]
    pub fn set_bounce_action(
        self,
        v: impl Into<BlockAssignable<SesReceiptRuleBounceActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bounce_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bounce_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `lambda_action`.\n"]
    pub fn set_lambda_action(
        self,
        v: impl Into<BlockAssignable<SesReceiptRuleLambdaActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lambda_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lambda_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `s3_action`.\n"]
    pub fn set_s3_action(self, v: impl Into<BlockAssignable<SesReceiptRuleS3ActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().s3_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.s3_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sns_action`.\n"]
    pub fn set_sns_action(self, v: impl Into<BlockAssignable<SesReceiptRuleSnsActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sns_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sns_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `stop_action`.\n"]
    pub fn set_stop_action(
        self,
        v: impl Into<BlockAssignable<SesReceiptRuleStopActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stop_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stop_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `workmail_action`.\n"]
    pub fn set_workmail_action(
        self,
        v: impl Into<BlockAssignable<SesReceiptRuleWorkmailActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().workmail_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.workmail_action = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `after` after provisioning.\n"]
    pub fn after(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.after", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `recipients` after provisioning.\n"]
    pub fn recipients(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.recipients", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_set_name` after provisioning.\n"]
    pub fn rule_set_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_set_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scan_enabled` after provisioning.\n"]
    pub fn scan_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scan_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tls_policy` after provisioning.\n"]
    pub fn tls_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tls_policy", self.extract_ref()),
        )
    }
}
impl Referable for SesReceiptRule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SesReceiptRule {}
impl ToListMappable for SesReceiptRule {
    type O = ListRef<SesReceiptRuleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SesReceiptRule_ {
    fn extract_resource_type(&self) -> String {
        "aws_ses_receipt_rule".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSesReceiptRule {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub rule_set_name: PrimField<String>,
}
impl BuildSesReceiptRule {
    pub fn build(self, stack: &mut Stack) -> SesReceiptRule {
        let out = SesReceiptRule(Rc::new(SesReceiptRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SesReceiptRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                after: core::default::Default::default(),
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                recipients: core::default::Default::default(),
                region: core::default::Default::default(),
                rule_set_name: self.rule_set_name,
                scan_enabled: core::default::Default::default(),
                tls_policy: core::default::Default::default(),
                add_header_action: core::default::Default::default(),
                bounce_action: core::default::Default::default(),
                lambda_action: core::default::Default::default(),
                s3_action: core::default::Default::default(),
                sns_action: core::default::Default::default(),
                stop_action: core::default::Default::default(),
                workmail_action: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SesReceiptRuleRef {
    shared: StackShared,
    base: String,
}
impl Ref for SesReceiptRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SesReceiptRuleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `after` after provisioning.\n"]
    pub fn after(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.after", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `recipients` after provisioning.\n"]
    pub fn recipients(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.recipients", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_set_name` after provisioning.\n"]
    pub fn rule_set_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_set_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scan_enabled` after provisioning.\n"]
    pub fn scan_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scan_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tls_policy` after provisioning.\n"]
    pub fn tls_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tls_policy", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SesReceiptRuleAddHeaderActionEl {
    header_name: PrimField<String>,
    header_value: PrimField<String>,
    position: PrimField<f64>,
}
impl SesReceiptRuleAddHeaderActionEl {}
impl ToListMappable for SesReceiptRuleAddHeaderActionEl {
    type O = BlockAssignable<SesReceiptRuleAddHeaderActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSesReceiptRuleAddHeaderActionEl {
    #[doc = ""]
    pub header_name: PrimField<String>,
    #[doc = ""]
    pub header_value: PrimField<String>,
    #[doc = ""]
    pub position: PrimField<f64>,
}
impl BuildSesReceiptRuleAddHeaderActionEl {
    pub fn build(self) -> SesReceiptRuleAddHeaderActionEl {
        SesReceiptRuleAddHeaderActionEl {
            header_name: self.header_name,
            header_value: self.header_value,
            position: self.position,
        }
    }
}
pub struct SesReceiptRuleAddHeaderActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SesReceiptRuleAddHeaderActionElRef {
    fn new(shared: StackShared, base: String) -> SesReceiptRuleAddHeaderActionElRef {
        SesReceiptRuleAddHeaderActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SesReceiptRuleAddHeaderActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `header_name` after provisioning.\n"]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
    #[doc = "Get a reference to the value of field `header_value` after provisioning.\n"]
    pub fn header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }
    #[doc = "Get a reference to the value of field `position` after provisioning.\n"]
    pub fn position(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.position", self.base))
    }
}
#[derive(Serialize)]
pub struct SesReceiptRuleBounceActionEl {
    message: PrimField<String>,
    position: PrimField<f64>,
    sender: PrimField<String>,
    smtp_reply_code: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic_arn: Option<PrimField<String>>,
}
impl SesReceiptRuleBounceActionEl {
    #[doc = "Set the field `status_code`.\n"]
    pub fn set_status_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status_code = Some(v.into());
        self
    }
    #[doc = "Set the field `topic_arn`.\n"]
    pub fn set_topic_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.topic_arn = Some(v.into());
        self
    }
}
impl ToListMappable for SesReceiptRuleBounceActionEl {
    type O = BlockAssignable<SesReceiptRuleBounceActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSesReceiptRuleBounceActionEl {
    #[doc = ""]
    pub message: PrimField<String>,
    #[doc = ""]
    pub position: PrimField<f64>,
    #[doc = ""]
    pub sender: PrimField<String>,
    #[doc = ""]
    pub smtp_reply_code: PrimField<String>,
}
impl BuildSesReceiptRuleBounceActionEl {
    pub fn build(self) -> SesReceiptRuleBounceActionEl {
        SesReceiptRuleBounceActionEl {
            message: self.message,
            position: self.position,
            sender: self.sender,
            smtp_reply_code: self.smtp_reply_code,
            status_code: core::default::Default::default(),
            topic_arn: core::default::Default::default(),
        }
    }
}
pub struct SesReceiptRuleBounceActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SesReceiptRuleBounceActionElRef {
    fn new(shared: StackShared, base: String) -> SesReceiptRuleBounceActionElRef {
        SesReceiptRuleBounceActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SesReceiptRuleBounceActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `position` after provisioning.\n"]
    pub fn position(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.position", self.base))
    }
    #[doc = "Get a reference to the value of field `sender` after provisioning.\n"]
    pub fn sender(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sender", self.base))
    }
    #[doc = "Get a reference to the value of field `smtp_reply_code` after provisioning.\n"]
    pub fn smtp_reply_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.smtp_reply_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct SesReceiptRuleLambdaActionEl {
    function_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invocation_type: Option<PrimField<String>>,
    position: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic_arn: Option<PrimField<String>>,
}
impl SesReceiptRuleLambdaActionEl {
    #[doc = "Set the field `invocation_type`.\n"]
    pub fn set_invocation_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.invocation_type = Some(v.into());
        self
    }
    #[doc = "Set the field `topic_arn`.\n"]
    pub fn set_topic_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.topic_arn = Some(v.into());
        self
    }
}
impl ToListMappable for SesReceiptRuleLambdaActionEl {
    type O = BlockAssignable<SesReceiptRuleLambdaActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSesReceiptRuleLambdaActionEl {
    #[doc = ""]
    pub function_arn: PrimField<String>,
    #[doc = ""]
    pub position: PrimField<f64>,
}
impl BuildSesReceiptRuleLambdaActionEl {
    pub fn build(self) -> SesReceiptRuleLambdaActionEl {
        SesReceiptRuleLambdaActionEl {
            function_arn: self.function_arn,
            invocation_type: core::default::Default::default(),
            position: self.position,
            topic_arn: core::default::Default::default(),
        }
    }
}
pub struct SesReceiptRuleLambdaActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SesReceiptRuleLambdaActionElRef {
    fn new(shared: StackShared, base: String) -> SesReceiptRuleLambdaActionElRef {
        SesReceiptRuleLambdaActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SesReceiptRuleLambdaActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `function_arn` after provisioning.\n"]
    pub fn function_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.function_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `invocation_type` after provisioning.\n"]
    pub fn invocation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.invocation_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `position` after provisioning.\n"]
    pub fn position(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.position", self.base))
    }
    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct SesReceiptRuleS3ActionEl {
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_key_prefix: Option<PrimField<String>>,
    position: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic_arn: Option<PrimField<String>>,
}
impl SesReceiptRuleS3ActionEl {
    #[doc = "Set the field `iam_role_arn`.\n"]
    pub fn set_iam_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.iam_role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `object_key_prefix`.\n"]
    pub fn set_object_key_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.object_key_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `topic_arn`.\n"]
    pub fn set_topic_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.topic_arn = Some(v.into());
        self
    }
}
impl ToListMappable for SesReceiptRuleS3ActionEl {
    type O = BlockAssignable<SesReceiptRuleS3ActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSesReceiptRuleS3ActionEl {
    #[doc = ""]
    pub bucket_name: PrimField<String>,
    #[doc = ""]
    pub position: PrimField<f64>,
}
impl BuildSesReceiptRuleS3ActionEl {
    pub fn build(self) -> SesReceiptRuleS3ActionEl {
        SesReceiptRuleS3ActionEl {
            bucket_name: self.bucket_name,
            iam_role_arn: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
            object_key_prefix: core::default::Default::default(),
            position: self.position,
            topic_arn: core::default::Default::default(),
        }
    }
}
pub struct SesReceiptRuleS3ActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SesReceiptRuleS3ActionElRef {
    fn new(shared: StackShared, base: String) -> SesReceiptRuleS3ActionElRef {
        SesReceiptRuleS3ActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SesReceiptRuleS3ActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
    #[doc = "Get a reference to the value of field `iam_role_arn` after provisioning.\n"]
    pub fn iam_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam_role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `object_key_prefix` after provisioning.\n"]
    pub fn object_key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.object_key_prefix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `position` after provisioning.\n"]
    pub fn position(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.position", self.base))
    }
    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct SesReceiptRuleSnsActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<PrimField<String>>,
    position: PrimField<f64>,
    topic_arn: PrimField<String>,
}
impl SesReceiptRuleSnsActionEl {
    #[doc = "Set the field `encoding`.\n"]
    pub fn set_encoding(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encoding = Some(v.into());
        self
    }
}
impl ToListMappable for SesReceiptRuleSnsActionEl {
    type O = BlockAssignable<SesReceiptRuleSnsActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSesReceiptRuleSnsActionEl {
    #[doc = ""]
    pub position: PrimField<f64>,
    #[doc = ""]
    pub topic_arn: PrimField<String>,
}
impl BuildSesReceiptRuleSnsActionEl {
    pub fn build(self) -> SesReceiptRuleSnsActionEl {
        SesReceiptRuleSnsActionEl {
            encoding: core::default::Default::default(),
            position: self.position,
            topic_arn: self.topic_arn,
        }
    }
}
pub struct SesReceiptRuleSnsActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SesReceiptRuleSnsActionElRef {
    fn new(shared: StackShared, base: String) -> SesReceiptRuleSnsActionElRef {
        SesReceiptRuleSnsActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SesReceiptRuleSnsActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `encoding` after provisioning.\n"]
    pub fn encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }
    #[doc = "Get a reference to the value of field `position` after provisioning.\n"]
    pub fn position(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.position", self.base))
    }
    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct SesReceiptRuleStopActionEl {
    position: PrimField<f64>,
    scope: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic_arn: Option<PrimField<String>>,
}
impl SesReceiptRuleStopActionEl {
    #[doc = "Set the field `topic_arn`.\n"]
    pub fn set_topic_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.topic_arn = Some(v.into());
        self
    }
}
impl ToListMappable for SesReceiptRuleStopActionEl {
    type O = BlockAssignable<SesReceiptRuleStopActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSesReceiptRuleStopActionEl {
    #[doc = ""]
    pub position: PrimField<f64>,
    #[doc = ""]
    pub scope: PrimField<String>,
}
impl BuildSesReceiptRuleStopActionEl {
    pub fn build(self) -> SesReceiptRuleStopActionEl {
        SesReceiptRuleStopActionEl {
            position: self.position,
            scope: self.scope,
            topic_arn: core::default::Default::default(),
        }
    }
}
pub struct SesReceiptRuleStopActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SesReceiptRuleStopActionElRef {
    fn new(shared: StackShared, base: String) -> SesReceiptRuleStopActionElRef {
        SesReceiptRuleStopActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SesReceiptRuleStopActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `position` after provisioning.\n"]
    pub fn position(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.position", self.base))
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct SesReceiptRuleWorkmailActionEl {
    organization_arn: PrimField<String>,
    position: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic_arn: Option<PrimField<String>>,
}
impl SesReceiptRuleWorkmailActionEl {
    #[doc = "Set the field `topic_arn`.\n"]
    pub fn set_topic_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.topic_arn = Some(v.into());
        self
    }
}
impl ToListMappable for SesReceiptRuleWorkmailActionEl {
    type O = BlockAssignable<SesReceiptRuleWorkmailActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSesReceiptRuleWorkmailActionEl {
    #[doc = ""]
    pub organization_arn: PrimField<String>,
    #[doc = ""]
    pub position: PrimField<f64>,
}
impl BuildSesReceiptRuleWorkmailActionEl {
    pub fn build(self) -> SesReceiptRuleWorkmailActionEl {
        SesReceiptRuleWorkmailActionEl {
            organization_arn: self.organization_arn,
            position: self.position,
            topic_arn: core::default::Default::default(),
        }
    }
}
pub struct SesReceiptRuleWorkmailActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SesReceiptRuleWorkmailActionElRef {
    fn new(shared: StackShared, base: String) -> SesReceiptRuleWorkmailActionElRef {
        SesReceiptRuleWorkmailActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SesReceiptRuleWorkmailActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `organization_arn` after provisioning.\n"]
    pub fn organization_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `position` after provisioning.\n"]
    pub fn position(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.position", self.base))
    }
    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }
}
#[derive(Serialize, Default)]
struct SesReceiptRuleDynamic {
    add_header_action: Option<DynamicBlock<SesReceiptRuleAddHeaderActionEl>>,
    bounce_action: Option<DynamicBlock<SesReceiptRuleBounceActionEl>>,
    lambda_action: Option<DynamicBlock<SesReceiptRuleLambdaActionEl>>,
    s3_action: Option<DynamicBlock<SesReceiptRuleS3ActionEl>>,
    sns_action: Option<DynamicBlock<SesReceiptRuleSnsActionEl>>,
    stop_action: Option<DynamicBlock<SesReceiptRuleStopActionEl>>,
    workmail_action: Option<DynamicBlock<SesReceiptRuleWorkmailActionEl>>,
}
