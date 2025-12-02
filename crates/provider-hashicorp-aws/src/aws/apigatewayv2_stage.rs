use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct Apigatewayv2StageData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    api_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_deploy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_certificate_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stage_variables: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_log_settings: Option<Vec<Apigatewayv2StageAccessLogSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_route_settings: Option<Vec<Apigatewayv2StageDefaultRouteSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_settings: Option<Vec<Apigatewayv2StageRouteSettingsEl>>,
    dynamic: Apigatewayv2StageDynamic,
}
struct Apigatewayv2Stage_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Apigatewayv2StageData>,
}
#[derive(Clone)]
pub struct Apigatewayv2Stage(Rc<Apigatewayv2Stage_>);
impl Apigatewayv2Stage {
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
    #[doc = "Set the field `auto_deploy`.\n"]
    pub fn set_auto_deploy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_deploy = Some(v.into());
        self
    }
    #[doc = "Set the field `client_certificate_id`.\n"]
    pub fn set_client_certificate_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().client_certificate_id = Some(v.into());
        self
    }
    #[doc = "Set the field `deployment_id`.\n"]
    pub fn set_deployment_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deployment_id = Some(v.into());
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `stage_variables`.\n"]
    pub fn set_stage_variables(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().stage_variables = Some(v.into());
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
    #[doc = "Set the field `access_log_settings`.\n"]
    pub fn set_access_log_settings(
        self,
        v: impl Into<BlockAssignable<Apigatewayv2StageAccessLogSettingsEl>>,
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
    #[doc = "Set the field `default_route_settings`.\n"]
    pub fn set_default_route_settings(
        self,
        v: impl Into<BlockAssignable<Apigatewayv2StageDefaultRouteSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_route_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_route_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `route_settings`.\n"]
    pub fn set_route_settings(
        self,
        v: impl Into<BlockAssignable<Apigatewayv2StageRouteSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().route_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.route_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `auto_deploy` after provisioning.\n"]
    pub fn auto_deploy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_deploy", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stage_variables` after provisioning.\n"]
    pub fn stage_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.stage_variables", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `access_log_settings` after provisioning.\n"]
    pub fn access_log_settings(&self) -> ListRef<Apigatewayv2StageAccessLogSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_log_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_route_settings` after provisioning.\n"]
    pub fn default_route_settings(&self) -> ListRef<Apigatewayv2StageDefaultRouteSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_route_settings", self.extract_ref()),
        )
    }
}
impl Referable for Apigatewayv2Stage {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for Apigatewayv2Stage {}
impl ToListMappable for Apigatewayv2Stage {
    type O = ListRef<Apigatewayv2StageRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for Apigatewayv2Stage_ {
    fn extract_resource_type(&self) -> String {
        "aws_apigatewayv2_stage".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApigatewayv2Stage {
    pub tf_id: String,
    #[doc = ""]
    pub api_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildApigatewayv2Stage {
    pub fn build(self, stack: &mut Stack) -> Apigatewayv2Stage {
        let out = Apigatewayv2Stage(Rc::new(Apigatewayv2Stage_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Apigatewayv2StageData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                api_id: self.api_id,
                auto_deploy: core::default::Default::default(),
                client_certificate_id: core::default::Default::default(),
                deployment_id: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                stage_variables: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                access_log_settings: core::default::Default::default(),
                default_route_settings: core::default::Default::default(),
                route_settings: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct Apigatewayv2StageRef {
    shared: StackShared,
    base: String,
}
impl Ref for Apigatewayv2StageRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl Apigatewayv2StageRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `auto_deploy` after provisioning.\n"]
    pub fn auto_deploy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_deploy", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stage_variables` after provisioning.\n"]
    pub fn stage_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.stage_variables", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `access_log_settings` after provisioning.\n"]
    pub fn access_log_settings(&self) -> ListRef<Apigatewayv2StageAccessLogSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_log_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_route_settings` after provisioning.\n"]
    pub fn default_route_settings(&self) -> ListRef<Apigatewayv2StageDefaultRouteSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_route_settings", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct Apigatewayv2StageAccessLogSettingsEl {
    destination_arn: PrimField<String>,
    format: PrimField<String>,
}
impl Apigatewayv2StageAccessLogSettingsEl {}
impl ToListMappable for Apigatewayv2StageAccessLogSettingsEl {
    type O = BlockAssignable<Apigatewayv2StageAccessLogSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigatewayv2StageAccessLogSettingsEl {
    #[doc = ""]
    pub destination_arn: PrimField<String>,
    #[doc = ""]
    pub format: PrimField<String>,
}
impl BuildApigatewayv2StageAccessLogSettingsEl {
    pub fn build(self) -> Apigatewayv2StageAccessLogSettingsEl {
        Apigatewayv2StageAccessLogSettingsEl {
            destination_arn: self.destination_arn,
            format: self.format,
        }
    }
}
pub struct Apigatewayv2StageAccessLogSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Apigatewayv2StageAccessLogSettingsElRef {
    fn new(shared: StackShared, base: String) -> Apigatewayv2StageAccessLogSettingsElRef {
        Apigatewayv2StageAccessLogSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Apigatewayv2StageAccessLogSettingsElRef {
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
pub struct Apigatewayv2StageDefaultRouteSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_trace_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detailed_metrics_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttling_burst_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttling_rate_limit: Option<PrimField<f64>>,
}
impl Apigatewayv2StageDefaultRouteSettingsEl {
    #[doc = "Set the field `data_trace_enabled`.\n"]
    pub fn set_data_trace_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.data_trace_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `detailed_metrics_enabled`.\n"]
    pub fn set_detailed_metrics_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.detailed_metrics_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `logging_level`.\n"]
    pub fn set_logging_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.logging_level = Some(v.into());
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
}
impl ToListMappable for Apigatewayv2StageDefaultRouteSettingsEl {
    type O = BlockAssignable<Apigatewayv2StageDefaultRouteSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigatewayv2StageDefaultRouteSettingsEl {}
impl BuildApigatewayv2StageDefaultRouteSettingsEl {
    pub fn build(self) -> Apigatewayv2StageDefaultRouteSettingsEl {
        Apigatewayv2StageDefaultRouteSettingsEl {
            data_trace_enabled: core::default::Default::default(),
            detailed_metrics_enabled: core::default::Default::default(),
            logging_level: core::default::Default::default(),
            throttling_burst_limit: core::default::Default::default(),
            throttling_rate_limit: core::default::Default::default(),
        }
    }
}
pub struct Apigatewayv2StageDefaultRouteSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Apigatewayv2StageDefaultRouteSettingsElRef {
    fn new(shared: StackShared, base: String) -> Apigatewayv2StageDefaultRouteSettingsElRef {
        Apigatewayv2StageDefaultRouteSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Apigatewayv2StageDefaultRouteSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_trace_enabled` after provisioning.\n"]
    pub fn data_trace_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_trace_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `detailed_metrics_enabled` after provisioning.\n"]
    pub fn detailed_metrics_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.detailed_metrics_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `logging_level` after provisioning.\n"]
    pub fn logging_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logging_level", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `throttling_burst_limit` after provisioning.\n"]
    pub fn throttling_burst_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttling_burst_limit", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `throttling_rate_limit` after provisioning.\n"]
    pub fn throttling_rate_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttling_rate_limit", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Apigatewayv2StageRouteSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_trace_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detailed_metrics_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_level: Option<PrimField<String>>,
    route_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttling_burst_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttling_rate_limit: Option<PrimField<f64>>,
}
impl Apigatewayv2StageRouteSettingsEl {
    #[doc = "Set the field `data_trace_enabled`.\n"]
    pub fn set_data_trace_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.data_trace_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `detailed_metrics_enabled`.\n"]
    pub fn set_detailed_metrics_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.detailed_metrics_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `logging_level`.\n"]
    pub fn set_logging_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.logging_level = Some(v.into());
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
}
impl ToListMappable for Apigatewayv2StageRouteSettingsEl {
    type O = BlockAssignable<Apigatewayv2StageRouteSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigatewayv2StageRouteSettingsEl {
    #[doc = ""]
    pub route_key: PrimField<String>,
}
impl BuildApigatewayv2StageRouteSettingsEl {
    pub fn build(self) -> Apigatewayv2StageRouteSettingsEl {
        Apigatewayv2StageRouteSettingsEl {
            data_trace_enabled: core::default::Default::default(),
            detailed_metrics_enabled: core::default::Default::default(),
            logging_level: core::default::Default::default(),
            route_key: self.route_key,
            throttling_burst_limit: core::default::Default::default(),
            throttling_rate_limit: core::default::Default::default(),
        }
    }
}
pub struct Apigatewayv2StageRouteSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Apigatewayv2StageRouteSettingsElRef {
    fn new(shared: StackShared, base: String) -> Apigatewayv2StageRouteSettingsElRef {
        Apigatewayv2StageRouteSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Apigatewayv2StageRouteSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_trace_enabled` after provisioning.\n"]
    pub fn data_trace_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_trace_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `detailed_metrics_enabled` after provisioning.\n"]
    pub fn detailed_metrics_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.detailed_metrics_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `logging_level` after provisioning.\n"]
    pub fn logging_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logging_level", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `route_key` after provisioning.\n"]
    pub fn route_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.route_key", self.base))
    }
    #[doc = "Get a reference to the value of field `throttling_burst_limit` after provisioning.\n"]
    pub fn throttling_burst_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttling_burst_limit", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `throttling_rate_limit` after provisioning.\n"]
    pub fn throttling_rate_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttling_rate_limit", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct Apigatewayv2StageDynamic {
    access_log_settings: Option<DynamicBlock<Apigatewayv2StageAccessLogSettingsEl>>,
    default_route_settings: Option<DynamicBlock<Apigatewayv2StageDefaultRouteSettingsEl>>,
    route_settings: Option<DynamicBlock<Apigatewayv2StageRouteSettingsEl>>,
}
