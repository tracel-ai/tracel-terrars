use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct Apigatewayv2IntegrationData {
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
    connection_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_handling_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credentials_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    integration_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    integration_subtype: Option<PrimField<String>>,
    integration_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    integration_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    passthrough_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload_format_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_parameters: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_templates: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_selection_expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_milliseconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_parameters: Option<Vec<Apigatewayv2IntegrationResponseParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_config: Option<Vec<Apigatewayv2IntegrationTlsConfigEl>>,
    dynamic: Apigatewayv2IntegrationDynamic,
}
struct Apigatewayv2Integration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Apigatewayv2IntegrationData>,
}
#[derive(Clone)]
pub struct Apigatewayv2Integration(Rc<Apigatewayv2Integration_>);
impl Apigatewayv2Integration {
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
    #[doc = "Set the field `connection_id`.\n"]
    pub fn set_connection_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().connection_id = Some(v.into());
        self
    }
    #[doc = "Set the field `connection_type`.\n"]
    pub fn set_connection_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().connection_type = Some(v.into());
        self
    }
    #[doc = "Set the field `content_handling_strategy`.\n"]
    pub fn set_content_handling_strategy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().content_handling_strategy = Some(v.into());
        self
    }
    #[doc = "Set the field `credentials_arn`.\n"]
    pub fn set_credentials_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().credentials_arn = Some(v.into());
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
    #[doc = "Set the field `integration_method`.\n"]
    pub fn set_integration_method(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().integration_method = Some(v.into());
        self
    }
    #[doc = "Set the field `integration_subtype`.\n"]
    pub fn set_integration_subtype(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().integration_subtype = Some(v.into());
        self
    }
    #[doc = "Set the field `integration_uri`.\n"]
    pub fn set_integration_uri(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().integration_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `passthrough_behavior`.\n"]
    pub fn set_passthrough_behavior(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().passthrough_behavior = Some(v.into());
        self
    }
    #[doc = "Set the field `payload_format_version`.\n"]
    pub fn set_payload_format_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().payload_format_version = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `request_parameters`.\n"]
    pub fn set_request_parameters(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().request_parameters = Some(v.into());
        self
    }
    #[doc = "Set the field `request_templates`.\n"]
    pub fn set_request_templates(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().request_templates = Some(v.into());
        self
    }
    #[doc = "Set the field `template_selection_expression`.\n"]
    pub fn set_template_selection_expression(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().template_selection_expression = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout_milliseconds`.\n"]
    pub fn set_timeout_milliseconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().timeout_milliseconds = Some(v.into());
        self
    }
    #[doc = "Set the field `response_parameters`.\n"]
    pub fn set_response_parameters(
        self,
        v: impl Into<BlockAssignable<Apigatewayv2IntegrationResponseParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().response_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.response_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tls_config`.\n"]
    pub fn set_tls_config(
        self,
        v: impl Into<BlockAssignable<Apigatewayv2IntegrationTlsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tls_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tls_config = Some(d);
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
    #[doc = "Get a reference to the value of field `connection_id` after provisioning.\n"]
    pub fn connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_type` after provisioning.\n"]
    pub fn connection_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_handling_strategy` after provisioning.\n"]
    pub fn content_handling_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_handling_strategy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `credentials_arn` after provisioning.\n"]
    pub fn credentials_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credentials_arn", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `integration_method` after provisioning.\n"]
    pub fn integration_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.integration_method", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `integration_response_selection_expression` after provisioning.\n"]
    pub fn integration_response_selection_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.integration_response_selection_expression",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `integration_subtype` after provisioning.\n"]
    pub fn integration_subtype(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.integration_subtype", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `integration_type` after provisioning.\n"]
    pub fn integration_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.integration_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `integration_uri` after provisioning.\n"]
    pub fn integration_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.integration_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `passthrough_behavior` after provisioning.\n"]
    pub fn passthrough_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.passthrough_behavior", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `payload_format_version` after provisioning.\n"]
    pub fn payload_format_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.payload_format_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `request_parameters` after provisioning.\n"]
    pub fn request_parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.request_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `request_templates` after provisioning.\n"]
    pub fn request_templates(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.request_templates", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template_selection_expression` after provisioning.\n"]
    pub fn template_selection_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_selection_expression", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeout_milliseconds` after provisioning.\n"]
    pub fn timeout_milliseconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_milliseconds", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> ListRef<Apigatewayv2IntegrationTlsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tls_config", self.extract_ref()),
        )
    }
}
impl Referable for Apigatewayv2Integration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for Apigatewayv2Integration {}
impl ToListMappable for Apigatewayv2Integration {
    type O = ListRef<Apigatewayv2IntegrationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for Apigatewayv2Integration_ {
    fn extract_resource_type(&self) -> String {
        "aws_apigatewayv2_integration".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApigatewayv2Integration {
    pub tf_id: String,
    #[doc = ""]
    pub api_id: PrimField<String>,
    #[doc = ""]
    pub integration_type: PrimField<String>,
}
impl BuildApigatewayv2Integration {
    pub fn build(self, stack: &mut Stack) -> Apigatewayv2Integration {
        let out = Apigatewayv2Integration(Rc::new(Apigatewayv2Integration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Apigatewayv2IntegrationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                api_id: self.api_id,
                connection_id: core::default::Default::default(),
                connection_type: core::default::Default::default(),
                content_handling_strategy: core::default::Default::default(),
                credentials_arn: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                integration_method: core::default::Default::default(),
                integration_subtype: core::default::Default::default(),
                integration_type: self.integration_type,
                integration_uri: core::default::Default::default(),
                passthrough_behavior: core::default::Default::default(),
                payload_format_version: core::default::Default::default(),
                region: core::default::Default::default(),
                request_parameters: core::default::Default::default(),
                request_templates: core::default::Default::default(),
                template_selection_expression: core::default::Default::default(),
                timeout_milliseconds: core::default::Default::default(),
                response_parameters: core::default::Default::default(),
                tls_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct Apigatewayv2IntegrationRef {
    shared: StackShared,
    base: String,
}
impl Ref for Apigatewayv2IntegrationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl Apigatewayv2IntegrationRef {
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
    #[doc = "Get a reference to the value of field `connection_id` after provisioning.\n"]
    pub fn connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_type` after provisioning.\n"]
    pub fn connection_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_handling_strategy` after provisioning.\n"]
    pub fn content_handling_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_handling_strategy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `credentials_arn` after provisioning.\n"]
    pub fn credentials_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credentials_arn", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `integration_method` after provisioning.\n"]
    pub fn integration_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.integration_method", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `integration_response_selection_expression` after provisioning.\n"]
    pub fn integration_response_selection_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.integration_response_selection_expression",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `integration_subtype` after provisioning.\n"]
    pub fn integration_subtype(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.integration_subtype", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `integration_type` after provisioning.\n"]
    pub fn integration_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.integration_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `integration_uri` after provisioning.\n"]
    pub fn integration_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.integration_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `passthrough_behavior` after provisioning.\n"]
    pub fn passthrough_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.passthrough_behavior", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `payload_format_version` after provisioning.\n"]
    pub fn payload_format_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.payload_format_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `request_parameters` after provisioning.\n"]
    pub fn request_parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.request_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `request_templates` after provisioning.\n"]
    pub fn request_templates(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.request_templates", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template_selection_expression` after provisioning.\n"]
    pub fn template_selection_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_selection_expression", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeout_milliseconds` after provisioning.\n"]
    pub fn timeout_milliseconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_milliseconds", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> ListRef<Apigatewayv2IntegrationTlsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tls_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct Apigatewayv2IntegrationResponseParametersEl {
    mappings: RecField<PrimField<String>>,
    status_code: PrimField<String>,
}
impl Apigatewayv2IntegrationResponseParametersEl {}
impl ToListMappable for Apigatewayv2IntegrationResponseParametersEl {
    type O = BlockAssignable<Apigatewayv2IntegrationResponseParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigatewayv2IntegrationResponseParametersEl {
    #[doc = ""]
    pub mappings: RecField<PrimField<String>>,
    #[doc = ""]
    pub status_code: PrimField<String>,
}
impl BuildApigatewayv2IntegrationResponseParametersEl {
    pub fn build(self) -> Apigatewayv2IntegrationResponseParametersEl {
        Apigatewayv2IntegrationResponseParametersEl {
            mappings: self.mappings,
            status_code: self.status_code,
        }
    }
}
pub struct Apigatewayv2IntegrationResponseParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Apigatewayv2IntegrationResponseParametersElRef {
    fn new(shared: StackShared, base: String) -> Apigatewayv2IntegrationResponseParametersElRef {
        Apigatewayv2IntegrationResponseParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Apigatewayv2IntegrationResponseParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mappings` after provisioning.\n"]
    pub fn mappings(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.mappings", self.base))
    }
    #[doc = "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}
#[derive(Serialize)]
pub struct Apigatewayv2IntegrationTlsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    server_name_to_verify: Option<PrimField<String>>,
}
impl Apigatewayv2IntegrationTlsConfigEl {
    #[doc = "Set the field `server_name_to_verify`.\n"]
    pub fn set_server_name_to_verify(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.server_name_to_verify = Some(v.into());
        self
    }
}
impl ToListMappable for Apigatewayv2IntegrationTlsConfigEl {
    type O = BlockAssignable<Apigatewayv2IntegrationTlsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigatewayv2IntegrationTlsConfigEl {}
impl BuildApigatewayv2IntegrationTlsConfigEl {
    pub fn build(self) -> Apigatewayv2IntegrationTlsConfigEl {
        Apigatewayv2IntegrationTlsConfigEl {
            server_name_to_verify: core::default::Default::default(),
        }
    }
}
pub struct Apigatewayv2IntegrationTlsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Apigatewayv2IntegrationTlsConfigElRef {
    fn new(shared: StackShared, base: String) -> Apigatewayv2IntegrationTlsConfigElRef {
        Apigatewayv2IntegrationTlsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Apigatewayv2IntegrationTlsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `server_name_to_verify` after provisioning.\n"]
    pub fn server_name_to_verify(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_name_to_verify", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct Apigatewayv2IntegrationDynamic {
    response_parameters: Option<DynamicBlock<Apigatewayv2IntegrationResponseParametersEl>>,
    tls_config: Option<DynamicBlock<Apigatewayv2IntegrationTlsConfigEl>>,
}
