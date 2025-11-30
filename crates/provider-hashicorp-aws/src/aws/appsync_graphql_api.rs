use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct AppsyncGraphqlApiData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_type: Option<PrimField<String>>,
    authentication_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    introspection_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merged_api_execution_role_arn: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_depth_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resolver_count_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    xray_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_authentication_provider:
        Option<Vec<AppsyncGraphqlApiAdditionalAuthenticationProviderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enhanced_metrics_config: Option<Vec<AppsyncGraphqlApiEnhancedMetricsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_authorizer_config: Option<Vec<AppsyncGraphqlApiLambdaAuthorizerConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_config: Option<Vec<AppsyncGraphqlApiLogConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    openid_connect_config: Option<Vec<AppsyncGraphqlApiOpenidConnectConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_pool_config: Option<Vec<AppsyncGraphqlApiUserPoolConfigEl>>,
    dynamic: AppsyncGraphqlApiDynamic,
}

struct AppsyncGraphqlApi_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppsyncGraphqlApiData>,
}

#[derive(Clone)]
pub struct AppsyncGraphqlApi(Rc<AppsyncGraphqlApi_>);

impl AppsyncGraphqlApi {
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

    #[doc = "Set the field `api_type`.\n"]
    pub fn set_api_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().api_type = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `introspection_config`.\n"]
    pub fn set_introspection_config(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().introspection_config = Some(v.into());
        self
    }

    #[doc = "Set the field `merged_api_execution_role_arn`.\n"]
    pub fn set_merged_api_execution_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().merged_api_execution_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `query_depth_limit`.\n"]
    pub fn set_query_depth_limit(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().query_depth_limit = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `resolver_count_limit`.\n"]
    pub fn set_resolver_count_limit(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().resolver_count_limit = Some(v.into());
        self
    }

    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().schema = Some(v.into());
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

    #[doc = "Set the field `visibility`.\n"]
    pub fn set_visibility(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().visibility = Some(v.into());
        self
    }

    #[doc = "Set the field `xray_enabled`.\n"]
    pub fn set_xray_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().xray_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `additional_authentication_provider`.\n"]
    pub fn set_additional_authentication_provider(
        self,
        v: impl Into<BlockAssignable<AppsyncGraphqlApiAdditionalAuthenticationProviderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().additional_authentication_provider = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .additional_authentication_provider = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `enhanced_metrics_config`.\n"]
    pub fn set_enhanced_metrics_config(
        self,
        v: impl Into<BlockAssignable<AppsyncGraphqlApiEnhancedMetricsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().enhanced_metrics_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.enhanced_metrics_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `lambda_authorizer_config`.\n"]
    pub fn set_lambda_authorizer_config(
        self,
        v: impl Into<BlockAssignable<AppsyncGraphqlApiLambdaAuthorizerConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lambda_authorizer_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lambda_authorizer_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `log_config`.\n"]
    pub fn set_log_config(
        self,
        v: impl Into<BlockAssignable<AppsyncGraphqlApiLogConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `openid_connect_config`.\n"]
    pub fn set_openid_connect_config(
        self,
        v: impl Into<BlockAssignable<AppsyncGraphqlApiOpenidConnectConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().openid_connect_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.openid_connect_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `user_pool_config`.\n"]
    pub fn set_user_pool_config(
        self,
        v: impl Into<BlockAssignable<AppsyncGraphqlApiUserPoolConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user_pool_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user_pool_config = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `api_type` after provisioning.\n"]
    pub fn api_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication_type` after provisioning.\n"]
    pub fn authentication_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authentication_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `introspection_config` after provisioning.\n"]
    pub fn introspection_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.introspection_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `merged_api_execution_role_arn` after provisioning.\n"]
    pub fn merged_api_execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.merged_api_execution_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `query_depth_limit` after provisioning.\n"]
    pub fn query_depth_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_depth_limit", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resolver_count_limit` after provisioning.\n"]
    pub fn resolver_count_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resolver_count_limit", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schema", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `uris` after provisioning.\n"]
    pub fn uris(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.uris", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `visibility` after provisioning.\n"]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.visibility", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `xray_enabled` after provisioning.\n"]
    pub fn xray_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.xray_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `additional_authentication_provider` after provisioning.\n"]
    pub fn additional_authentication_provider(
        &self,
    ) -> ListRef<AppsyncGraphqlApiAdditionalAuthenticationProviderElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_authentication_provider", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enhanced_metrics_config` after provisioning.\n"]
    pub fn enhanced_metrics_config(&self) -> ListRef<AppsyncGraphqlApiEnhancedMetricsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.enhanced_metrics_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `lambda_authorizer_config` after provisioning.\n"]
    pub fn lambda_authorizer_config(
        &self,
    ) -> ListRef<AppsyncGraphqlApiLambdaAuthorizerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lambda_authorizer_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> ListRef<AppsyncGraphqlApiLogConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `openid_connect_config` after provisioning.\n"]
    pub fn openid_connect_config(&self) -> ListRef<AppsyncGraphqlApiOpenidConnectConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.openid_connect_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_pool_config` after provisioning.\n"]
    pub fn user_pool_config(&self) -> ListRef<AppsyncGraphqlApiUserPoolConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_pool_config", self.extract_ref()),
        )
    }
}

impl Referable for AppsyncGraphqlApi {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for AppsyncGraphqlApi {}

impl ToListMappable for AppsyncGraphqlApi {
    type O = ListRef<AppsyncGraphqlApiRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AppsyncGraphqlApi_ {
    fn extract_resource_type(&self) -> String {
        "aws_appsync_graphql_api".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppsyncGraphqlApi {
    pub tf_id: String,
    #[doc = ""]
    pub authentication_type: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildAppsyncGraphqlApi {
    pub fn build(self, stack: &mut Stack) -> AppsyncGraphqlApi {
        let out = AppsyncGraphqlApi(Rc::new(AppsyncGraphqlApi_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppsyncGraphqlApiData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                api_type: core::default::Default::default(),
                authentication_type: self.authentication_type,
                id: core::default::Default::default(),
                introspection_config: core::default::Default::default(),
                merged_api_execution_role_arn: core::default::Default::default(),
                name: self.name,
                query_depth_limit: core::default::Default::default(),
                region: core::default::Default::default(),
                resolver_count_limit: core::default::Default::default(),
                schema: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                visibility: core::default::Default::default(),
                xray_enabled: core::default::Default::default(),
                additional_authentication_provider: core::default::Default::default(),
                enhanced_metrics_config: core::default::Default::default(),
                lambda_authorizer_config: core::default::Default::default(),
                log_config: core::default::Default::default(),
                openid_connect_config: core::default::Default::default(),
                user_pool_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppsyncGraphqlApiRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncGraphqlApiRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl AppsyncGraphqlApiRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_type` after provisioning.\n"]
    pub fn api_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication_type` after provisioning.\n"]
    pub fn authentication_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authentication_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `introspection_config` after provisioning.\n"]
    pub fn introspection_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.introspection_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `merged_api_execution_role_arn` after provisioning.\n"]
    pub fn merged_api_execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.merged_api_execution_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `query_depth_limit` after provisioning.\n"]
    pub fn query_depth_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_depth_limit", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resolver_count_limit` after provisioning.\n"]
    pub fn resolver_count_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resolver_count_limit", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schema", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `uris` after provisioning.\n"]
    pub fn uris(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.uris", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `visibility` after provisioning.\n"]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.visibility", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `xray_enabled` after provisioning.\n"]
    pub fn xray_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.xray_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `additional_authentication_provider` after provisioning.\n"]
    pub fn additional_authentication_provider(
        &self,
    ) -> ListRef<AppsyncGraphqlApiAdditionalAuthenticationProviderElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_authentication_provider", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enhanced_metrics_config` after provisioning.\n"]
    pub fn enhanced_metrics_config(&self) -> ListRef<AppsyncGraphqlApiEnhancedMetricsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.enhanced_metrics_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `lambda_authorizer_config` after provisioning.\n"]
    pub fn lambda_authorizer_config(
        &self,
    ) -> ListRef<AppsyncGraphqlApiLambdaAuthorizerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lambda_authorizer_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> ListRef<AppsyncGraphqlApiLogConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `openid_connect_config` after provisioning.\n"]
    pub fn openid_connect_config(&self) -> ListRef<AppsyncGraphqlApiOpenidConnectConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.openid_connect_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_pool_config` after provisioning.\n"]
    pub fn user_pool_config(&self) -> ListRef<AppsyncGraphqlApiUserPoolConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_pool_config", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct AppsyncGraphqlApiAdditionalAuthenticationProviderElLambdaAuthorizerConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authorizer_result_ttl_in_seconds: Option<PrimField<f64>>,
    authorizer_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_validation_expression: Option<PrimField<String>>,
}

impl AppsyncGraphqlApiAdditionalAuthenticationProviderElLambdaAuthorizerConfigEl {
    #[doc = "Set the field `authorizer_result_ttl_in_seconds`.\n"]
    pub fn set_authorizer_result_ttl_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.authorizer_result_ttl_in_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `identity_validation_expression`.\n"]
    pub fn set_identity_validation_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_validation_expression = Some(v.into());
        self
    }
}

impl ToListMappable
    for AppsyncGraphqlApiAdditionalAuthenticationProviderElLambdaAuthorizerConfigEl
{
    type O = BlockAssignable<
        AppsyncGraphqlApiAdditionalAuthenticationProviderElLambdaAuthorizerConfigEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncGraphqlApiAdditionalAuthenticationProviderElLambdaAuthorizerConfigEl {
    #[doc = ""]
    pub authorizer_uri: PrimField<String>,
}

impl BuildAppsyncGraphqlApiAdditionalAuthenticationProviderElLambdaAuthorizerConfigEl {
    pub fn build(
        self,
    ) -> AppsyncGraphqlApiAdditionalAuthenticationProviderElLambdaAuthorizerConfigEl {
        AppsyncGraphqlApiAdditionalAuthenticationProviderElLambdaAuthorizerConfigEl {
            authorizer_result_ttl_in_seconds: core::default::Default::default(),
            authorizer_uri: self.authorizer_uri,
            identity_validation_expression: core::default::Default::default(),
        }
    }
}

pub struct AppsyncGraphqlApiAdditionalAuthenticationProviderElLambdaAuthorizerConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncGraphqlApiAdditionalAuthenticationProviderElLambdaAuthorizerConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppsyncGraphqlApiAdditionalAuthenticationProviderElLambdaAuthorizerConfigElRef {
        AppsyncGraphqlApiAdditionalAuthenticationProviderElLambdaAuthorizerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncGraphqlApiAdditionalAuthenticationProviderElLambdaAuthorizerConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorizer_result_ttl_in_seconds` after provisioning.\n"]
    pub fn authorizer_result_ttl_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorizer_result_ttl_in_seconds", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `authorizer_uri` after provisioning.\n"]
    pub fn authorizer_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorizer_uri", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `identity_validation_expression` after provisioning.\n"]
    pub fn identity_validation_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_validation_expression", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct AppsyncGraphqlApiAdditionalAuthenticationProviderElOpenidConnectConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iat_ttl: Option<PrimField<f64>>,
    issuer: PrimField<String>,
}

impl AppsyncGraphqlApiAdditionalAuthenticationProviderElOpenidConnectConfigEl {
    #[doc = "Set the field `auth_ttl`.\n"]
    pub fn set_auth_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.auth_ttl = Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id = Some(v.into());
        self
    }

    #[doc = "Set the field `iat_ttl`.\n"]
    pub fn set_iat_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.iat_ttl = Some(v.into());
        self
    }
}

impl ToListMappable for AppsyncGraphqlApiAdditionalAuthenticationProviderElOpenidConnectConfigEl {
    type O =
        BlockAssignable<AppsyncGraphqlApiAdditionalAuthenticationProviderElOpenidConnectConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncGraphqlApiAdditionalAuthenticationProviderElOpenidConnectConfigEl {
    #[doc = ""]
    pub issuer: PrimField<String>,
}

impl BuildAppsyncGraphqlApiAdditionalAuthenticationProviderElOpenidConnectConfigEl {
    pub fn build(self) -> AppsyncGraphqlApiAdditionalAuthenticationProviderElOpenidConnectConfigEl {
        AppsyncGraphqlApiAdditionalAuthenticationProviderElOpenidConnectConfigEl {
            auth_ttl: core::default::Default::default(),
            client_id: core::default::Default::default(),
            iat_ttl: core::default::Default::default(),
            issuer: self.issuer,
        }
    }
}

pub struct AppsyncGraphqlApiAdditionalAuthenticationProviderElOpenidConnectConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncGraphqlApiAdditionalAuthenticationProviderElOpenidConnectConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppsyncGraphqlApiAdditionalAuthenticationProviderElOpenidConnectConfigElRef {
        AppsyncGraphqlApiAdditionalAuthenticationProviderElOpenidConnectConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncGraphqlApiAdditionalAuthenticationProviderElOpenidConnectConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_ttl` after provisioning.\n"]
    pub fn auth_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `iat_ttl` after provisioning.\n"]
    pub fn iat_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iat_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncGraphqlApiAdditionalAuthenticationProviderElUserPoolConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    app_id_client_regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_region: Option<PrimField<String>>,
    user_pool_id: PrimField<String>,
}

impl AppsyncGraphqlApiAdditionalAuthenticationProviderElUserPoolConfigEl {
    #[doc = "Set the field `app_id_client_regex`.\n"]
    pub fn set_app_id_client_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.app_id_client_regex = Some(v.into());
        self
    }

    #[doc = "Set the field `aws_region`.\n"]
    pub fn set_aws_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.aws_region = Some(v.into());
        self
    }
}

impl ToListMappable for AppsyncGraphqlApiAdditionalAuthenticationProviderElUserPoolConfigEl {
    type O = BlockAssignable<AppsyncGraphqlApiAdditionalAuthenticationProviderElUserPoolConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncGraphqlApiAdditionalAuthenticationProviderElUserPoolConfigEl {
    #[doc = ""]
    pub user_pool_id: PrimField<String>,
}

impl BuildAppsyncGraphqlApiAdditionalAuthenticationProviderElUserPoolConfigEl {
    pub fn build(self) -> AppsyncGraphqlApiAdditionalAuthenticationProviderElUserPoolConfigEl {
        AppsyncGraphqlApiAdditionalAuthenticationProviderElUserPoolConfigEl {
            app_id_client_regex: core::default::Default::default(),
            aws_region: core::default::Default::default(),
            user_pool_id: self.user_pool_id,
        }
    }
}

pub struct AppsyncGraphqlApiAdditionalAuthenticationProviderElUserPoolConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncGraphqlApiAdditionalAuthenticationProviderElUserPoolConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppsyncGraphqlApiAdditionalAuthenticationProviderElUserPoolConfigElRef {
        AppsyncGraphqlApiAdditionalAuthenticationProviderElUserPoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncGraphqlApiAdditionalAuthenticationProviderElUserPoolConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_id_client_regex` after provisioning.\n"]
    pub fn app_id_client_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_id_client_regex", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `aws_region` after provisioning.\n"]
    pub fn aws_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aws_region", self.base))
    }

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_pool_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppsyncGraphqlApiAdditionalAuthenticationProviderElDynamic {
    lambda_authorizer_config: Option<
        DynamicBlock<AppsyncGraphqlApiAdditionalAuthenticationProviderElLambdaAuthorizerConfigEl>,
    >,
    openid_connect_config: Option<
        DynamicBlock<AppsyncGraphqlApiAdditionalAuthenticationProviderElOpenidConnectConfigEl>,
    >,
    user_pool_config:
        Option<DynamicBlock<AppsyncGraphqlApiAdditionalAuthenticationProviderElUserPoolConfigEl>>,
}

#[derive(Serialize)]
pub struct AppsyncGraphqlApiAdditionalAuthenticationProviderEl {
    authentication_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_authorizer_config:
        Option<Vec<AppsyncGraphqlApiAdditionalAuthenticationProviderElLambdaAuthorizerConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    openid_connect_config:
        Option<Vec<AppsyncGraphqlApiAdditionalAuthenticationProviderElOpenidConnectConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_pool_config:
        Option<Vec<AppsyncGraphqlApiAdditionalAuthenticationProviderElUserPoolConfigEl>>,
    dynamic: AppsyncGraphqlApiAdditionalAuthenticationProviderElDynamic,
}

impl AppsyncGraphqlApiAdditionalAuthenticationProviderEl {
    #[doc = "Set the field `lambda_authorizer_config`.\n"]
    pub fn set_lambda_authorizer_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                AppsyncGraphqlApiAdditionalAuthenticationProviderElLambdaAuthorizerConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lambda_authorizer_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_authorizer_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `openid_connect_config`.\n"]
    pub fn set_openid_connect_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                AppsyncGraphqlApiAdditionalAuthenticationProviderElOpenidConnectConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.openid_connect_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.openid_connect_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `user_pool_config`.\n"]
    pub fn set_user_pool_config(
        mut self,
        v: impl Into<
            BlockAssignable<AppsyncGraphqlApiAdditionalAuthenticationProviderElUserPoolConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.user_pool_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.user_pool_config = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppsyncGraphqlApiAdditionalAuthenticationProviderEl {
    type O = BlockAssignable<AppsyncGraphqlApiAdditionalAuthenticationProviderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncGraphqlApiAdditionalAuthenticationProviderEl {
    #[doc = ""]
    pub authentication_type: PrimField<String>,
}

impl BuildAppsyncGraphqlApiAdditionalAuthenticationProviderEl {
    pub fn build(self) -> AppsyncGraphqlApiAdditionalAuthenticationProviderEl {
        AppsyncGraphqlApiAdditionalAuthenticationProviderEl {
            authentication_type: self.authentication_type,
            lambda_authorizer_config: core::default::Default::default(),
            openid_connect_config: core::default::Default::default(),
            user_pool_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppsyncGraphqlApiAdditionalAuthenticationProviderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncGraphqlApiAdditionalAuthenticationProviderElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppsyncGraphqlApiAdditionalAuthenticationProviderElRef {
        AppsyncGraphqlApiAdditionalAuthenticationProviderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncGraphqlApiAdditionalAuthenticationProviderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication_type` after provisioning.\n"]
    pub fn authentication_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authentication_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `lambda_authorizer_config` after provisioning.\n"]
    pub fn lambda_authorizer_config(
        &self,
    ) -> ListRef<AppsyncGraphqlApiAdditionalAuthenticationProviderElLambdaAuthorizerConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lambda_authorizer_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `openid_connect_config` after provisioning.\n"]
    pub fn openid_connect_config(
        &self,
    ) -> ListRef<AppsyncGraphqlApiAdditionalAuthenticationProviderElOpenidConnectConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.openid_connect_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_pool_config` after provisioning.\n"]
    pub fn user_pool_config(
        &self,
    ) -> ListRef<AppsyncGraphqlApiAdditionalAuthenticationProviderElUserPoolConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_pool_config", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct AppsyncGraphqlApiEnhancedMetricsConfigEl {
    data_source_level_metrics_behavior: PrimField<String>,
    operation_level_metrics_config: PrimField<String>,
    resolver_level_metrics_behavior: PrimField<String>,
}

impl AppsyncGraphqlApiEnhancedMetricsConfigEl {}

impl ToListMappable for AppsyncGraphqlApiEnhancedMetricsConfigEl {
    type O = BlockAssignable<AppsyncGraphqlApiEnhancedMetricsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncGraphqlApiEnhancedMetricsConfigEl {
    #[doc = ""]
    pub data_source_level_metrics_behavior: PrimField<String>,
    #[doc = ""]
    pub operation_level_metrics_config: PrimField<String>,
    #[doc = ""]
    pub resolver_level_metrics_behavior: PrimField<String>,
}

impl BuildAppsyncGraphqlApiEnhancedMetricsConfigEl {
    pub fn build(self) -> AppsyncGraphqlApiEnhancedMetricsConfigEl {
        AppsyncGraphqlApiEnhancedMetricsConfigEl {
            data_source_level_metrics_behavior: self.data_source_level_metrics_behavior,
            operation_level_metrics_config: self.operation_level_metrics_config,
            resolver_level_metrics_behavior: self.resolver_level_metrics_behavior,
        }
    }
}

pub struct AppsyncGraphqlApiEnhancedMetricsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncGraphqlApiEnhancedMetricsConfigElRef {
    fn new(shared: StackShared, base: String) -> AppsyncGraphqlApiEnhancedMetricsConfigElRef {
        AppsyncGraphqlApiEnhancedMetricsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncGraphqlApiEnhancedMetricsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_source_level_metrics_behavior` after provisioning.\n"]
    pub fn data_source_level_metrics_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source_level_metrics_behavior", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `operation_level_metrics_config` after provisioning.\n"]
    pub fn operation_level_metrics_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.operation_level_metrics_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `resolver_level_metrics_behavior` after provisioning.\n"]
    pub fn resolver_level_metrics_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resolver_level_metrics_behavior", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct AppsyncGraphqlApiLambdaAuthorizerConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authorizer_result_ttl_in_seconds: Option<PrimField<f64>>,
    authorizer_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_validation_expression: Option<PrimField<String>>,
}

impl AppsyncGraphqlApiLambdaAuthorizerConfigEl {
    #[doc = "Set the field `authorizer_result_ttl_in_seconds`.\n"]
    pub fn set_authorizer_result_ttl_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.authorizer_result_ttl_in_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `identity_validation_expression`.\n"]
    pub fn set_identity_validation_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_validation_expression = Some(v.into());
        self
    }
}

impl ToListMappable for AppsyncGraphqlApiLambdaAuthorizerConfigEl {
    type O = BlockAssignable<AppsyncGraphqlApiLambdaAuthorizerConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncGraphqlApiLambdaAuthorizerConfigEl {
    #[doc = ""]
    pub authorizer_uri: PrimField<String>,
}

impl BuildAppsyncGraphqlApiLambdaAuthorizerConfigEl {
    pub fn build(self) -> AppsyncGraphqlApiLambdaAuthorizerConfigEl {
        AppsyncGraphqlApiLambdaAuthorizerConfigEl {
            authorizer_result_ttl_in_seconds: core::default::Default::default(),
            authorizer_uri: self.authorizer_uri,
            identity_validation_expression: core::default::Default::default(),
        }
    }
}

pub struct AppsyncGraphqlApiLambdaAuthorizerConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncGraphqlApiLambdaAuthorizerConfigElRef {
    fn new(shared: StackShared, base: String) -> AppsyncGraphqlApiLambdaAuthorizerConfigElRef {
        AppsyncGraphqlApiLambdaAuthorizerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncGraphqlApiLambdaAuthorizerConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorizer_result_ttl_in_seconds` after provisioning.\n"]
    pub fn authorizer_result_ttl_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorizer_result_ttl_in_seconds", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `authorizer_uri` after provisioning.\n"]
    pub fn authorizer_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorizer_uri", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `identity_validation_expression` after provisioning.\n"]
    pub fn identity_validation_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_validation_expression", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct AppsyncGraphqlApiLogConfigEl {
    cloudwatch_logs_role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_verbose_content: Option<PrimField<bool>>,
    field_log_level: PrimField<String>,
}

impl AppsyncGraphqlApiLogConfigEl {
    #[doc = "Set the field `exclude_verbose_content`.\n"]
    pub fn set_exclude_verbose_content(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.exclude_verbose_content = Some(v.into());
        self
    }
}

impl ToListMappable for AppsyncGraphqlApiLogConfigEl {
    type O = BlockAssignable<AppsyncGraphqlApiLogConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncGraphqlApiLogConfigEl {
    #[doc = ""]
    pub cloudwatch_logs_role_arn: PrimField<String>,
    #[doc = ""]
    pub field_log_level: PrimField<String>,
}

impl BuildAppsyncGraphqlApiLogConfigEl {
    pub fn build(self) -> AppsyncGraphqlApiLogConfigEl {
        AppsyncGraphqlApiLogConfigEl {
            cloudwatch_logs_role_arn: self.cloudwatch_logs_role_arn,
            exclude_verbose_content: core::default::Default::default(),
            field_log_level: self.field_log_level,
        }
    }
}

pub struct AppsyncGraphqlApiLogConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncGraphqlApiLogConfigElRef {
    fn new(shared: StackShared, base: String) -> AppsyncGraphqlApiLogConfigElRef {
        AppsyncGraphqlApiLogConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncGraphqlApiLogConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs_role_arn` after provisioning.\n"]
    pub fn cloudwatch_logs_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloudwatch_logs_role_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `exclude_verbose_content` after provisioning.\n"]
    pub fn exclude_verbose_content(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.exclude_verbose_content", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `field_log_level` after provisioning.\n"]
    pub fn field_log_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.field_log_level", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct AppsyncGraphqlApiOpenidConnectConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iat_ttl: Option<PrimField<f64>>,
    issuer: PrimField<String>,
}

impl AppsyncGraphqlApiOpenidConnectConfigEl {
    #[doc = "Set the field `auth_ttl`.\n"]
    pub fn set_auth_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.auth_ttl = Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id = Some(v.into());
        self
    }

    #[doc = "Set the field `iat_ttl`.\n"]
    pub fn set_iat_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.iat_ttl = Some(v.into());
        self
    }
}

impl ToListMappable for AppsyncGraphqlApiOpenidConnectConfigEl {
    type O = BlockAssignable<AppsyncGraphqlApiOpenidConnectConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncGraphqlApiOpenidConnectConfigEl {
    #[doc = ""]
    pub issuer: PrimField<String>,
}

impl BuildAppsyncGraphqlApiOpenidConnectConfigEl {
    pub fn build(self) -> AppsyncGraphqlApiOpenidConnectConfigEl {
        AppsyncGraphqlApiOpenidConnectConfigEl {
            auth_ttl: core::default::Default::default(),
            client_id: core::default::Default::default(),
            iat_ttl: core::default::Default::default(),
            issuer: self.issuer,
        }
    }
}

pub struct AppsyncGraphqlApiOpenidConnectConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncGraphqlApiOpenidConnectConfigElRef {
    fn new(shared: StackShared, base: String) -> AppsyncGraphqlApiOpenidConnectConfigElRef {
        AppsyncGraphqlApiOpenidConnectConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncGraphqlApiOpenidConnectConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_ttl` after provisioning.\n"]
    pub fn auth_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `iat_ttl` after provisioning.\n"]
    pub fn iat_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iat_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncGraphqlApiUserPoolConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    app_id_client_regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_region: Option<PrimField<String>>,
    default_action: PrimField<String>,
    user_pool_id: PrimField<String>,
}

impl AppsyncGraphqlApiUserPoolConfigEl {
    #[doc = "Set the field `app_id_client_regex`.\n"]
    pub fn set_app_id_client_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.app_id_client_regex = Some(v.into());
        self
    }

    #[doc = "Set the field `aws_region`.\n"]
    pub fn set_aws_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.aws_region = Some(v.into());
        self
    }
}

impl ToListMappable for AppsyncGraphqlApiUserPoolConfigEl {
    type O = BlockAssignable<AppsyncGraphqlApiUserPoolConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncGraphqlApiUserPoolConfigEl {
    #[doc = ""]
    pub default_action: PrimField<String>,
    #[doc = ""]
    pub user_pool_id: PrimField<String>,
}

impl BuildAppsyncGraphqlApiUserPoolConfigEl {
    pub fn build(self) -> AppsyncGraphqlApiUserPoolConfigEl {
        AppsyncGraphqlApiUserPoolConfigEl {
            app_id_client_regex: core::default::Default::default(),
            aws_region: core::default::Default::default(),
            default_action: self.default_action,
            user_pool_id: self.user_pool_id,
        }
    }
}

pub struct AppsyncGraphqlApiUserPoolConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncGraphqlApiUserPoolConfigElRef {
    fn new(shared: StackShared, base: String) -> AppsyncGraphqlApiUserPoolConfigElRef {
        AppsyncGraphqlApiUserPoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncGraphqlApiUserPoolConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_id_client_regex` after provisioning.\n"]
    pub fn app_id_client_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_id_client_regex", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `aws_region` after provisioning.\n"]
    pub fn aws_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aws_region", self.base))
    }

    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_action", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_pool_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppsyncGraphqlApiDynamic {
    additional_authentication_provider:
        Option<DynamicBlock<AppsyncGraphqlApiAdditionalAuthenticationProviderEl>>,
    enhanced_metrics_config: Option<DynamicBlock<AppsyncGraphqlApiEnhancedMetricsConfigEl>>,
    lambda_authorizer_config: Option<DynamicBlock<AppsyncGraphqlApiLambdaAuthorizerConfigEl>>,
    log_config: Option<DynamicBlock<AppsyncGraphqlApiLogConfigEl>>,
    openid_connect_config: Option<DynamicBlock<AppsyncGraphqlApiOpenidConnectConfigEl>>,
    user_pool_config: Option<DynamicBlock<AppsyncGraphqlApiUserPoolConfigEl>>,
}
