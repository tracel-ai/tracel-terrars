use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ApiGatewayMethodSettingsData {
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
    method_path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    rest_api_id: PrimField<String>,
    stage_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<Vec<ApiGatewayMethodSettingsSettingsEl>>,
    dynamic: ApiGatewayMethodSettingsDynamic,
}

struct ApiGatewayMethodSettings_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApiGatewayMethodSettingsData>,
}

#[derive(Clone)]
pub struct ApiGatewayMethodSettings(Rc<ApiGatewayMethodSettings_>);

impl ApiGatewayMethodSettings {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(self, v: impl Into<BlockAssignable<ApiGatewayMethodSettingsSettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.settings = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `method_path` after provisioning.\n"]
    pub fn method_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.method_path", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rest_api_id` after provisioning.\n"]
    pub fn rest_api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rest_api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stage_name` after provisioning.\n"]
    pub fn stage_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stage_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> ListRef<ApiGatewayMethodSettingsSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }
}

impl Referable for ApiGatewayMethodSettings {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ApiGatewayMethodSettings { }

impl ToListMappable for ApiGatewayMethodSettings {
    type O = ListRef<ApiGatewayMethodSettingsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ApiGatewayMethodSettings_ {
    fn extract_resource_type(&self) -> String {
        "aws_api_gateway_method_settings".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApiGatewayMethodSettings {
    pub tf_id: String,
    #[doc = ""]
    pub method_path: PrimField<String>,
    #[doc = ""]
    pub rest_api_id: PrimField<String>,
    #[doc = ""]
    pub stage_name: PrimField<String>,
}

impl BuildApiGatewayMethodSettings {
    pub fn build(self, stack: &mut Stack) -> ApiGatewayMethodSettings {
        let out = ApiGatewayMethodSettings(Rc::new(ApiGatewayMethodSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApiGatewayMethodSettingsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                method_path: self.method_path,
                region: core::default::Default::default(),
                rest_api_id: self.rest_api_id,
                stage_name: self.stage_name,
                settings: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApiGatewayMethodSettingsRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApiGatewayMethodSettingsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl ApiGatewayMethodSettingsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `method_path` after provisioning.\n"]
    pub fn method_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.method_path", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rest_api_id` after provisioning.\n"]
    pub fn rest_api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rest_api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stage_name` after provisioning.\n"]
    pub fn stage_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stage_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> ListRef<ApiGatewayMethodSettingsSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ApiGatewayMethodSettingsSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_data_encrypted: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_ttl_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caching_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_trace_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metrics_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_authorization_for_cache_control: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttling_burst_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttling_rate_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unauthorized_cache_control_header_strategy: Option<PrimField<String>>,
}

impl ApiGatewayMethodSettingsSettingsEl {
    #[doc = "Set the field `cache_data_encrypted`.\n"]
    pub fn set_cache_data_encrypted(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.cache_data_encrypted = Some(v.into());
        self
    }

    #[doc = "Set the field `cache_ttl_in_seconds`.\n"]
    pub fn set_cache_ttl_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cache_ttl_in_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `caching_enabled`.\n"]
    pub fn set_caching_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.caching_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `data_trace_enabled`.\n"]
    pub fn set_data_trace_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.data_trace_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `logging_level`.\n"]
    pub fn set_logging_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.logging_level = Some(v.into());
        self
    }

    #[doc = "Set the field `metrics_enabled`.\n"]
    pub fn set_metrics_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.metrics_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `require_authorization_for_cache_control`.\n"]
    pub fn set_require_authorization_for_cache_control(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_authorization_for_cache_control = Some(v.into());
        self
    }

    #[doc = "Set the field `throttling_burst_limit`.\n"]
    pub fn set_throttling_burst_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.throttling_burst_limit = Some(v.into());
        self
    }

    #[doc = "Set the field `throttling_rate_limit`.\n"]
    pub fn set_throttling_rate_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.throttling_rate_limit = Some(v.into());
        self
    }

    #[doc = "Set the field `unauthorized_cache_control_header_strategy`.\n"]
    pub fn set_unauthorized_cache_control_header_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unauthorized_cache_control_header_strategy = Some(v.into());
        self
    }
}

impl ToListMappable for ApiGatewayMethodSettingsSettingsEl {
    type O = BlockAssignable<ApiGatewayMethodSettingsSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiGatewayMethodSettingsSettingsEl {}

impl BuildApiGatewayMethodSettingsSettingsEl {
    pub fn build(self) -> ApiGatewayMethodSettingsSettingsEl {
        ApiGatewayMethodSettingsSettingsEl {
            cache_data_encrypted: core::default::Default::default(),
            cache_ttl_in_seconds: core::default::Default::default(),
            caching_enabled: core::default::Default::default(),
            data_trace_enabled: core::default::Default::default(),
            logging_level: core::default::Default::default(),
            metrics_enabled: core::default::Default::default(),
            require_authorization_for_cache_control: core::default::Default::default(),
            throttling_burst_limit: core::default::Default::default(),
            throttling_rate_limit: core::default::Default::default(),
            unauthorized_cache_control_header_strategy: core::default::Default::default(),
        }
    }
}

pub struct ApiGatewayMethodSettingsSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApiGatewayMethodSettingsSettingsElRef {
    fn new(shared: StackShared, base: String) -> ApiGatewayMethodSettingsSettingsElRef {
        ApiGatewayMethodSettingsSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiGatewayMethodSettingsSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cache_data_encrypted` after provisioning.\n"]
    pub fn cache_data_encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cache_data_encrypted", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_ttl_in_seconds` after provisioning.\n"]
    pub fn cache_ttl_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cache_ttl_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `caching_enabled` after provisioning.\n"]
    pub fn caching_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.caching_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `data_trace_enabled` after provisioning.\n"]
    pub fn data_trace_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_trace_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `logging_level` after provisioning.\n"]
    pub fn logging_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.logging_level", self.base))
    }

    #[doc = "Get a reference to the value of field `metrics_enabled` after provisioning.\n"]
    pub fn metrics_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.metrics_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `require_authorization_for_cache_control` after provisioning.\n"]
    pub fn require_authorization_for_cache_control(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_authorization_for_cache_control", self.base))
    }

    #[doc = "Get a reference to the value of field `throttling_burst_limit` after provisioning.\n"]
    pub fn throttling_burst_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.throttling_burst_limit", self.base))
    }

    #[doc = "Get a reference to the value of field `throttling_rate_limit` after provisioning.\n"]
    pub fn throttling_rate_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.throttling_rate_limit", self.base))
    }

    #[doc = "Get a reference to the value of field `unauthorized_cache_control_header_strategy` after provisioning.\n"]
    pub fn unauthorized_cache_control_header_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unauthorized_cache_control_header_strategy", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApiGatewayMethodSettingsDynamic {
    settings: Option<DynamicBlock<ApiGatewayMethodSettingsSettingsEl>>,
}
