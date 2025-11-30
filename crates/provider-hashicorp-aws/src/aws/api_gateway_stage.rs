use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ApiGatewayStageData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_cluster_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_cluster_size: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_certificate_id: Option<PrimField<String>>,
    deployment_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    documentation_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    rest_api_id: PrimField<String>,
    stage_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variables: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    xray_tracing_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_log_settings: Option<Vec<ApiGatewayStageAccessLogSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    canary_settings: Option<Vec<ApiGatewayStageCanarySettingsEl>>,
    dynamic: ApiGatewayStageDynamic,
}

struct ApiGatewayStage_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApiGatewayStageData>,
}

#[derive(Clone)]
pub struct ApiGatewayStage(Rc<ApiGatewayStage_>);

impl ApiGatewayStage {
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

    #[doc = "Set the field `cache_cluster_enabled`.\n"]
    pub fn set_cache_cluster_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().cache_cluster_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `cache_cluster_size`.\n"]
    pub fn set_cache_cluster_size(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cache_cluster_size = Some(v.into());
        self
    }

    #[doc = "Set the field `client_certificate_id`.\n"]
    pub fn set_client_certificate_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().client_certificate_id = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `documentation_version`.\n"]
    pub fn set_documentation_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().documentation_version = Some(v.into());
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

    #[doc = "Set the field `variables`.\n"]
    pub fn set_variables(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().variables = Some(v.into());
        self
    }

    #[doc = "Set the field `xray_tracing_enabled`.\n"]
    pub fn set_xray_tracing_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().xray_tracing_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `access_log_settings`.\n"]
    pub fn set_access_log_settings(
        self,
        v: impl Into<BlockAssignable<ApiGatewayStageAccessLogSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_log_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_log_settings = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `canary_settings`.\n"]
    pub fn set_canary_settings(
        self,
        v: impl Into<BlockAssignable<ApiGatewayStageCanarySettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().canary_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.canary_settings = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_cluster_enabled` after provisioning.\n"]
    pub fn cache_cluster_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cache_cluster_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cache_cluster_size` after provisioning.\n"]
    pub fn cache_cluster_size(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cache_cluster_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `client_certificate_id` after provisioning.\n"]
    pub fn client_certificate_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_certificate_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deployment_id` after provisioning.\n"]
    pub fn deployment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `documentation_version` after provisioning.\n"]
    pub fn documentation_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.documentation_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `execution_arn` after provisioning.\n"]
    pub fn execution_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `invoke_url` after provisioning.\n"]
    pub fn invoke_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.invoke_url", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rest_api_id` after provisioning.\n"]
    pub fn rest_api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rest_api_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `stage_name` after provisioning.\n"]
    pub fn stage_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stage_name", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `variables` after provisioning.\n"]
    pub fn variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.variables", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `web_acl_arn` after provisioning.\n"]
    pub fn web_acl_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.web_acl_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `xray_tracing_enabled` after provisioning.\n"]
    pub fn xray_tracing_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.xray_tracing_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `access_log_settings` after provisioning.\n"]
    pub fn access_log_settings(&self) -> ListRef<ApiGatewayStageAccessLogSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_log_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `canary_settings` after provisioning.\n"]
    pub fn canary_settings(&self) -> ListRef<ApiGatewayStageCanarySettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.canary_settings", self.extract_ref()),
        )
    }
}

impl Referable for ApiGatewayStage {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for ApiGatewayStage {}

impl ToListMappable for ApiGatewayStage {
    type O = ListRef<ApiGatewayStageRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ApiGatewayStage_ {
    fn extract_resource_type(&self) -> String {
        "aws_api_gateway_stage".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApiGatewayStage {
    pub tf_id: String,
    #[doc = ""]
    pub deployment_id: PrimField<String>,
    #[doc = ""]
    pub rest_api_id: PrimField<String>,
    #[doc = ""]
    pub stage_name: PrimField<String>,
}

impl BuildApiGatewayStage {
    pub fn build(self, stack: &mut Stack) -> ApiGatewayStage {
        let out = ApiGatewayStage(Rc::new(ApiGatewayStage_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApiGatewayStageData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cache_cluster_enabled: core::default::Default::default(),
                cache_cluster_size: core::default::Default::default(),
                client_certificate_id: core::default::Default::default(),
                deployment_id: self.deployment_id,
                description: core::default::Default::default(),
                documentation_version: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                rest_api_id: self.rest_api_id,
                stage_name: self.stage_name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                variables: core::default::Default::default(),
                xray_tracing_enabled: core::default::Default::default(),
                access_log_settings: core::default::Default::default(),
                canary_settings: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApiGatewayStageRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApiGatewayStageRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl ApiGatewayStageRef {
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

    #[doc = "Get a reference to the value of field `cache_cluster_enabled` after provisioning.\n"]
    pub fn cache_cluster_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cache_cluster_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cache_cluster_size` after provisioning.\n"]
    pub fn cache_cluster_size(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cache_cluster_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `client_certificate_id` after provisioning.\n"]
    pub fn client_certificate_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_certificate_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deployment_id` after provisioning.\n"]
    pub fn deployment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `documentation_version` after provisioning.\n"]
    pub fn documentation_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.documentation_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `execution_arn` after provisioning.\n"]
    pub fn execution_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `invoke_url` after provisioning.\n"]
    pub fn invoke_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.invoke_url", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rest_api_id` after provisioning.\n"]
    pub fn rest_api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rest_api_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `stage_name` after provisioning.\n"]
    pub fn stage_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stage_name", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `variables` after provisioning.\n"]
    pub fn variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.variables", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `web_acl_arn` after provisioning.\n"]
    pub fn web_acl_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.web_acl_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `xray_tracing_enabled` after provisioning.\n"]
    pub fn xray_tracing_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.xray_tracing_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `access_log_settings` after provisioning.\n"]
    pub fn access_log_settings(&self) -> ListRef<ApiGatewayStageAccessLogSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_log_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `canary_settings` after provisioning.\n"]
    pub fn canary_settings(&self) -> ListRef<ApiGatewayStageCanarySettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.canary_settings", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ApiGatewayStageAccessLogSettingsEl {
    destination_arn: PrimField<String>,
    format: PrimField<String>,
}

impl ApiGatewayStageAccessLogSettingsEl {}

impl ToListMappable for ApiGatewayStageAccessLogSettingsEl {
    type O = BlockAssignable<ApiGatewayStageAccessLogSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiGatewayStageAccessLogSettingsEl {
    #[doc = ""]
    pub destination_arn: PrimField<String>,
    #[doc = ""]
    pub format: PrimField<String>,
}

impl BuildApiGatewayStageAccessLogSettingsEl {
    pub fn build(self) -> ApiGatewayStageAccessLogSettingsEl {
        ApiGatewayStageAccessLogSettingsEl {
            destination_arn: self.destination_arn,
            format: self.format,
        }
    }
}

pub struct ApiGatewayStageAccessLogSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApiGatewayStageAccessLogSettingsElRef {
    fn new(shared: StackShared, base: String) -> ApiGatewayStageAccessLogSettingsElRef {
        ApiGatewayStageAccessLogSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiGatewayStageAccessLogSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_arn` after provisioning.\n"]
    pub fn destination_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }
}

#[derive(Serialize)]
pub struct ApiGatewayStageCanarySettingsEl {
    deployment_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percent_traffic: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stage_variable_overrides: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_stage_cache: Option<PrimField<bool>>,
}

impl ApiGatewayStageCanarySettingsEl {
    #[doc = "Set the field `percent_traffic`.\n"]
    pub fn set_percent_traffic(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percent_traffic = Some(v.into());
        self
    }

    #[doc = "Set the field `stage_variable_overrides`.\n"]
    pub fn set_stage_variable_overrides(
        mut self,
        v: impl Into<RecField<PrimField<String>>>,
    ) -> Self {
        self.stage_variable_overrides = Some(v.into());
        self
    }

    #[doc = "Set the field `use_stage_cache`.\n"]
    pub fn set_use_stage_cache(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_stage_cache = Some(v.into());
        self
    }
}

impl ToListMappable for ApiGatewayStageCanarySettingsEl {
    type O = BlockAssignable<ApiGatewayStageCanarySettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiGatewayStageCanarySettingsEl {
    #[doc = ""]
    pub deployment_id: PrimField<String>,
}

impl BuildApiGatewayStageCanarySettingsEl {
    pub fn build(self) -> ApiGatewayStageCanarySettingsEl {
        ApiGatewayStageCanarySettingsEl {
            deployment_id: self.deployment_id,
            percent_traffic: core::default::Default::default(),
            stage_variable_overrides: core::default::Default::default(),
            use_stage_cache: core::default::Default::default(),
        }
    }
}

pub struct ApiGatewayStageCanarySettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApiGatewayStageCanarySettingsElRef {
    fn new(shared: StackShared, base: String) -> ApiGatewayStageCanarySettingsElRef {
        ApiGatewayStageCanarySettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiGatewayStageCanarySettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deployment_id` after provisioning.\n"]
    pub fn deployment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `percent_traffic` after provisioning.\n"]
    pub fn percent_traffic(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.percent_traffic", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `stage_variable_overrides` after provisioning.\n"]
    pub fn stage_variable_overrides(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.stage_variable_overrides", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `use_stage_cache` after provisioning.\n"]
    pub fn use_stage_cache(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_stage_cache", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct ApiGatewayStageDynamic {
    access_log_settings: Option<DynamicBlock<ApiGatewayStageAccessLogSettingsEl>>,
    canary_settings: Option<DynamicBlock<ApiGatewayStageCanarySettingsEl>>,
}
