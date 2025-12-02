use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ApiGatewayAccountData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct ApiGatewayAccount_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApiGatewayAccountData>,
}
#[derive(Clone)]
pub struct ApiGatewayAccount(Rc<ApiGatewayAccount_>);
impl ApiGatewayAccount {
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
    #[doc = "Set the field `cloudwatch_role_arn`.\n"]
    pub fn set_cloudwatch_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cloudwatch_role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `api_key_version` after provisioning.\n"]
    pub fn api_key_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_key_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloudwatch_role_arn` after provisioning.\n"]
    pub fn cloudwatch_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloudwatch_role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `features` after provisioning.\n"]
    pub fn features(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.features", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `throttle_settings` after provisioning.\n"]
    pub fn throttle_settings(&self) -> ListRef<ApiGatewayAccountThrottleSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.throttle_settings", self.extract_ref()),
        )
    }
}
impl Referable for ApiGatewayAccount {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ApiGatewayAccount {}
impl ToListMappable for ApiGatewayAccount {
    type O = ListRef<ApiGatewayAccountRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ApiGatewayAccount_ {
    fn extract_resource_type(&self) -> String {
        "aws_api_gateway_account".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApiGatewayAccount {
    pub tf_id: String,
}
impl BuildApiGatewayAccount {
    pub fn build(self, stack: &mut Stack) -> ApiGatewayAccount {
        let out = ApiGatewayAccount(Rc::new(ApiGatewayAccount_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApiGatewayAccountData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cloudwatch_role_arn: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ApiGatewayAccountRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApiGatewayAccountRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ApiGatewayAccountRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_key_version` after provisioning.\n"]
    pub fn api_key_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_key_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloudwatch_role_arn` after provisioning.\n"]
    pub fn cloudwatch_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloudwatch_role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `features` after provisioning.\n"]
    pub fn features(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.features", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `throttle_settings` after provisioning.\n"]
    pub fn throttle_settings(&self) -> ListRef<ApiGatewayAccountThrottleSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.throttle_settings", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ApiGatewayAccountThrottleSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    burst_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit: Option<PrimField<f64>>,
}
impl ApiGatewayAccountThrottleSettingsEl {
    #[doc = "Set the field `burst_limit`.\n"]
    pub fn set_burst_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.burst_limit = Some(v.into());
        self
    }
    #[doc = "Set the field `rate_limit`.\n"]
    pub fn set_rate_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.rate_limit = Some(v.into());
        self
    }
}
impl ToListMappable for ApiGatewayAccountThrottleSettingsEl {
    type O = BlockAssignable<ApiGatewayAccountThrottleSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApiGatewayAccountThrottleSettingsEl {}
impl BuildApiGatewayAccountThrottleSettingsEl {
    pub fn build(self) -> ApiGatewayAccountThrottleSettingsEl {
        ApiGatewayAccountThrottleSettingsEl {
            burst_limit: core::default::Default::default(),
            rate_limit: core::default::Default::default(),
        }
    }
}
pub struct ApiGatewayAccountThrottleSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApiGatewayAccountThrottleSettingsElRef {
    fn new(shared: StackShared, base: String) -> ApiGatewayAccountThrottleSettingsElRef {
        ApiGatewayAccountThrottleSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApiGatewayAccountThrottleSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `burst_limit` after provisioning.\n"]
    pub fn burst_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.burst_limit", self.base))
    }
    #[doc = "Get a reference to the value of field `rate_limit` after provisioning.\n"]
    pub fn rate_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.rate_limit", self.base))
    }
}
