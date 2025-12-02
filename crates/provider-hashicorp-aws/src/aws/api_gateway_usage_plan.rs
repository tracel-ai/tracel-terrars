use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ApiGatewayUsagePlanData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_stages: Option<Vec<ApiGatewayUsagePlanApiStagesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quota_settings: Option<Vec<ApiGatewayUsagePlanQuotaSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttle_settings: Option<Vec<ApiGatewayUsagePlanThrottleSettingsEl>>,
    dynamic: ApiGatewayUsagePlanDynamic,
}
struct ApiGatewayUsagePlan_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApiGatewayUsagePlanData>,
}
#[derive(Clone)]
pub struct ApiGatewayUsagePlan(Rc<ApiGatewayUsagePlan_>);
impl ApiGatewayUsagePlan {
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
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `product_code`.\n"]
    pub fn set_product_code(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().product_code = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
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
    #[doc = "Set the field `api_stages`.\n"]
    pub fn set_api_stages(
        self,
        v: impl Into<BlockAssignable<ApiGatewayUsagePlanApiStagesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().api_stages = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.api_stages = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `quota_settings`.\n"]
    pub fn set_quota_settings(
        self,
        v: impl Into<BlockAssignable<ApiGatewayUsagePlanQuotaSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().quota_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.quota_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `throttle_settings`.\n"]
    pub fn set_throttle_settings(
        self,
        v: impl Into<BlockAssignable<ApiGatewayUsagePlanThrottleSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().throttle_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.throttle_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `product_code` after provisioning.\n"]
    pub fn product_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.product_code", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `quota_settings` after provisioning.\n"]
    pub fn quota_settings(&self) -> ListRef<ApiGatewayUsagePlanQuotaSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.quota_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `throttle_settings` after provisioning.\n"]
    pub fn throttle_settings(&self) -> ListRef<ApiGatewayUsagePlanThrottleSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.throttle_settings", self.extract_ref()),
        )
    }
}
impl Referable for ApiGatewayUsagePlan {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ApiGatewayUsagePlan {}
impl ToListMappable for ApiGatewayUsagePlan {
    type O = ListRef<ApiGatewayUsagePlanRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ApiGatewayUsagePlan_ {
    fn extract_resource_type(&self) -> String {
        "aws_api_gateway_usage_plan".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApiGatewayUsagePlan {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildApiGatewayUsagePlan {
    pub fn build(self, stack: &mut Stack) -> ApiGatewayUsagePlan {
        let out = ApiGatewayUsagePlan(Rc::new(ApiGatewayUsagePlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApiGatewayUsagePlanData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                product_code: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                api_stages: core::default::Default::default(),
                quota_settings: core::default::Default::default(),
                throttle_settings: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ApiGatewayUsagePlanRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApiGatewayUsagePlanRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ApiGatewayUsagePlanRef {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `product_code` after provisioning.\n"]
    pub fn product_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.product_code", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `quota_settings` after provisioning.\n"]
    pub fn quota_settings(&self) -> ListRef<ApiGatewayUsagePlanQuotaSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.quota_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `throttle_settings` after provisioning.\n"]
    pub fn throttle_settings(&self) -> ListRef<ApiGatewayUsagePlanThrottleSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.throttle_settings", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ApiGatewayUsagePlanApiStagesElThrottleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    burst_limit: Option<PrimField<f64>>,
    path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit: Option<PrimField<f64>>,
}
impl ApiGatewayUsagePlanApiStagesElThrottleEl {
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
impl ToListMappable for ApiGatewayUsagePlanApiStagesElThrottleEl {
    type O = BlockAssignable<ApiGatewayUsagePlanApiStagesElThrottleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApiGatewayUsagePlanApiStagesElThrottleEl {
    #[doc = ""]
    pub path: PrimField<String>,
}
impl BuildApiGatewayUsagePlanApiStagesElThrottleEl {
    pub fn build(self) -> ApiGatewayUsagePlanApiStagesElThrottleEl {
        ApiGatewayUsagePlanApiStagesElThrottleEl {
            burst_limit: core::default::Default::default(),
            path: self.path,
            rate_limit: core::default::Default::default(),
        }
    }
}
pub struct ApiGatewayUsagePlanApiStagesElThrottleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApiGatewayUsagePlanApiStagesElThrottleElRef {
    fn new(shared: StackShared, base: String) -> ApiGatewayUsagePlanApiStagesElThrottleElRef {
        ApiGatewayUsagePlanApiStagesElThrottleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApiGatewayUsagePlanApiStagesElThrottleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `burst_limit` after provisioning.\n"]
    pub fn burst_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.burst_limit", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `rate_limit` after provisioning.\n"]
    pub fn rate_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.rate_limit", self.base))
    }
}
#[derive(Serialize, Default)]
struct ApiGatewayUsagePlanApiStagesElDynamic {
    throttle: Option<DynamicBlock<ApiGatewayUsagePlanApiStagesElThrottleEl>>,
}
#[derive(Serialize)]
pub struct ApiGatewayUsagePlanApiStagesEl {
    api_id: PrimField<String>,
    stage: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttle: Option<Vec<ApiGatewayUsagePlanApiStagesElThrottleEl>>,
    dynamic: ApiGatewayUsagePlanApiStagesElDynamic,
}
impl ApiGatewayUsagePlanApiStagesEl {
    #[doc = "Set the field `throttle`.\n"]
    pub fn set_throttle(
        mut self,
        v: impl Into<BlockAssignable<ApiGatewayUsagePlanApiStagesElThrottleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.throttle = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.throttle = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApiGatewayUsagePlanApiStagesEl {
    type O = BlockAssignable<ApiGatewayUsagePlanApiStagesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApiGatewayUsagePlanApiStagesEl {
    #[doc = ""]
    pub api_id: PrimField<String>,
    #[doc = ""]
    pub stage: PrimField<String>,
}
impl BuildApiGatewayUsagePlanApiStagesEl {
    pub fn build(self) -> ApiGatewayUsagePlanApiStagesEl {
        ApiGatewayUsagePlanApiStagesEl {
            api_id: self.api_id,
            stage: self.stage,
            throttle: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApiGatewayUsagePlanApiStagesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApiGatewayUsagePlanApiStagesElRef {
    fn new(shared: StackShared, base: String) -> ApiGatewayUsagePlanApiStagesElRef {
        ApiGatewayUsagePlanApiStagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApiGatewayUsagePlanApiStagesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_id", self.base))
    }
    #[doc = "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stage", self.base))
    }
}
#[derive(Serialize)]
pub struct ApiGatewayUsagePlanQuotaSettingsEl {
    limit: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<PrimField<f64>>,
    period: PrimField<String>,
}
impl ApiGatewayUsagePlanQuotaSettingsEl {
    #[doc = "Set the field `offset`.\n"]
    pub fn set_offset(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.offset = Some(v.into());
        self
    }
}
impl ToListMappable for ApiGatewayUsagePlanQuotaSettingsEl {
    type O = BlockAssignable<ApiGatewayUsagePlanQuotaSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApiGatewayUsagePlanQuotaSettingsEl {
    #[doc = ""]
    pub limit: PrimField<f64>,
    #[doc = ""]
    pub period: PrimField<String>,
}
impl BuildApiGatewayUsagePlanQuotaSettingsEl {
    pub fn build(self) -> ApiGatewayUsagePlanQuotaSettingsEl {
        ApiGatewayUsagePlanQuotaSettingsEl {
            limit: self.limit,
            offset: core::default::Default::default(),
            period: self.period,
        }
    }
}
pub struct ApiGatewayUsagePlanQuotaSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApiGatewayUsagePlanQuotaSettingsElRef {
    fn new(shared: StackShared, base: String) -> ApiGatewayUsagePlanQuotaSettingsElRef {
        ApiGatewayUsagePlanQuotaSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApiGatewayUsagePlanQuotaSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `limit` after provisioning.\n"]
    pub fn limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.limit", self.base))
    }
    #[doc = "Get a reference to the value of field `offset` after provisioning.\n"]
    pub fn offset(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.offset", self.base))
    }
    #[doc = "Get a reference to the value of field `period` after provisioning.\n"]
    pub fn period(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.period", self.base))
    }
}
#[derive(Serialize)]
pub struct ApiGatewayUsagePlanThrottleSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    burst_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit: Option<PrimField<f64>>,
}
impl ApiGatewayUsagePlanThrottleSettingsEl {
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
impl ToListMappable for ApiGatewayUsagePlanThrottleSettingsEl {
    type O = BlockAssignable<ApiGatewayUsagePlanThrottleSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApiGatewayUsagePlanThrottleSettingsEl {}
impl BuildApiGatewayUsagePlanThrottleSettingsEl {
    pub fn build(self) -> ApiGatewayUsagePlanThrottleSettingsEl {
        ApiGatewayUsagePlanThrottleSettingsEl {
            burst_limit: core::default::Default::default(),
            rate_limit: core::default::Default::default(),
        }
    }
}
pub struct ApiGatewayUsagePlanThrottleSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApiGatewayUsagePlanThrottleSettingsElRef {
    fn new(shared: StackShared, base: String) -> ApiGatewayUsagePlanThrottleSettingsElRef {
        ApiGatewayUsagePlanThrottleSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApiGatewayUsagePlanThrottleSettingsElRef {
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
#[derive(Serialize, Default)]
struct ApiGatewayUsagePlanDynamic {
    api_stages: Option<DynamicBlock<ApiGatewayUsagePlanApiStagesEl>>,
    quota_settings: Option<DynamicBlock<ApiGatewayUsagePlanQuotaSettingsEl>>,
    throttle_settings: Option<DynamicBlock<ApiGatewayUsagePlanThrottleSettingsEl>>,
}
