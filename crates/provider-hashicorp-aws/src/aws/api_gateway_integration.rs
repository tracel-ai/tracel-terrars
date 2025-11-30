use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ApiGatewayIntegrationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_key_parameters: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_handling: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credentials: Option<PrimField<String>>,
    http_method: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    integration_http_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    passthrough_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_parameters: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_templates: Option<RecField<PrimField<String>>>,
    resource_id: PrimField<String>,
    rest_api_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_milliseconds: Option<PrimField<f64>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_config: Option<Vec<ApiGatewayIntegrationTlsConfigEl>>,
    dynamic: ApiGatewayIntegrationDynamic,
}

struct ApiGatewayIntegration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApiGatewayIntegrationData>,
}

#[derive(Clone)]
pub struct ApiGatewayIntegration(Rc<ApiGatewayIntegration_>);

impl ApiGatewayIntegration {
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

    #[doc = "Set the field `cache_key_parameters`.\n"]
    pub fn set_cache_key_parameters(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().cache_key_parameters = Some(v.into());
        self
    }

    #[doc = "Set the field `cache_namespace`.\n"]
    pub fn set_cache_namespace(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cache_namespace = Some(v.into());
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

    #[doc = "Set the field `content_handling`.\n"]
    pub fn set_content_handling(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().content_handling = Some(v.into());
        self
    }

    #[doc = "Set the field `credentials`.\n"]
    pub fn set_credentials(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().credentials = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `integration_http_method`.\n"]
    pub fn set_integration_http_method(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().integration_http_method = Some(v.into());
        self
    }

    #[doc = "Set the field `passthrough_behavior`.\n"]
    pub fn set_passthrough_behavior(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().passthrough_behavior = Some(v.into());
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

    #[doc = "Set the field `timeout_milliseconds`.\n"]
    pub fn set_timeout_milliseconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().timeout_milliseconds = Some(v.into());
        self
    }

    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().uri = Some(v.into());
        self
    }

    #[doc = "Set the field `tls_config`.\n"]
    pub fn set_tls_config(
        self,
        v: impl Into<BlockAssignable<ApiGatewayIntegrationTlsConfigEl>>,
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

    #[doc = "Get a reference to the value of field `cache_key_parameters` after provisioning.\n"]
    pub fn cache_key_parameters(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.cache_key_parameters", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cache_namespace` after provisioning.\n"]
    pub fn cache_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cache_namespace", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `content_handling` after provisioning.\n"]
    pub fn content_handling(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_handling", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `credentials` after provisioning.\n"]
    pub fn credentials(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credentials", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `http_method` after provisioning.\n"]
    pub fn http_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_method", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration_http_method` after provisioning.\n"]
    pub fn integration_http_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.integration_http_method", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `passthrough_behavior` after provisioning.\n"]
    pub fn passthrough_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.passthrough_behavior", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rest_api_id` after provisioning.\n"]
    pub fn rest_api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rest_api_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeout_milliseconds` after provisioning.\n"]
    pub fn timeout_milliseconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_milliseconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> ListRef<ApiGatewayIntegrationTlsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tls_config", self.extract_ref()),
        )
    }
}

impl Referable for ApiGatewayIntegration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for ApiGatewayIntegration {}

impl ToListMappable for ApiGatewayIntegration {
    type O = ListRef<ApiGatewayIntegrationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ApiGatewayIntegration_ {
    fn extract_resource_type(&self) -> String {
        "aws_api_gateway_integration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApiGatewayIntegration {
    pub tf_id: String,
    #[doc = ""]
    pub http_method: PrimField<String>,
    #[doc = ""]
    pub resource_id: PrimField<String>,
    #[doc = ""]
    pub rest_api_id: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildApiGatewayIntegration {
    pub fn build(self, stack: &mut Stack) -> ApiGatewayIntegration {
        let out = ApiGatewayIntegration(Rc::new(ApiGatewayIntegration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApiGatewayIntegrationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cache_key_parameters: core::default::Default::default(),
                cache_namespace: core::default::Default::default(),
                connection_id: core::default::Default::default(),
                connection_type: core::default::Default::default(),
                content_handling: core::default::Default::default(),
                credentials: core::default::Default::default(),
                http_method: self.http_method,
                id: core::default::Default::default(),
                integration_http_method: core::default::Default::default(),
                passthrough_behavior: core::default::Default::default(),
                region: core::default::Default::default(),
                request_parameters: core::default::Default::default(),
                request_templates: core::default::Default::default(),
                resource_id: self.resource_id,
                rest_api_id: self.rest_api_id,
                timeout_milliseconds: core::default::Default::default(),
                type_: self.type_,
                uri: core::default::Default::default(),
                tls_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApiGatewayIntegrationRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApiGatewayIntegrationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl ApiGatewayIntegrationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cache_key_parameters` after provisioning.\n"]
    pub fn cache_key_parameters(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.cache_key_parameters", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cache_namespace` after provisioning.\n"]
    pub fn cache_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cache_namespace", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `content_handling` after provisioning.\n"]
    pub fn content_handling(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_handling", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `credentials` after provisioning.\n"]
    pub fn credentials(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credentials", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `http_method` after provisioning.\n"]
    pub fn http_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_method", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration_http_method` after provisioning.\n"]
    pub fn integration_http_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.integration_http_method", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `passthrough_behavior` after provisioning.\n"]
    pub fn passthrough_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.passthrough_behavior", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rest_api_id` after provisioning.\n"]
    pub fn rest_api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rest_api_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeout_milliseconds` after provisioning.\n"]
    pub fn timeout_milliseconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_milliseconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> ListRef<ApiGatewayIntegrationTlsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tls_config", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ApiGatewayIntegrationTlsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    insecure_skip_verification: Option<PrimField<bool>>,
}

impl ApiGatewayIntegrationTlsConfigEl {
    #[doc = "Set the field `insecure_skip_verification`.\n"]
    pub fn set_insecure_skip_verification(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.insecure_skip_verification = Some(v.into());
        self
    }
}

impl ToListMappable for ApiGatewayIntegrationTlsConfigEl {
    type O = BlockAssignable<ApiGatewayIntegrationTlsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiGatewayIntegrationTlsConfigEl {}

impl BuildApiGatewayIntegrationTlsConfigEl {
    pub fn build(self) -> ApiGatewayIntegrationTlsConfigEl {
        ApiGatewayIntegrationTlsConfigEl {
            insecure_skip_verification: core::default::Default::default(),
        }
    }
}

pub struct ApiGatewayIntegrationTlsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApiGatewayIntegrationTlsConfigElRef {
    fn new(shared: StackShared, base: String) -> ApiGatewayIntegrationTlsConfigElRef {
        ApiGatewayIntegrationTlsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiGatewayIntegrationTlsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `insecure_skip_verification` after provisioning.\n"]
    pub fn insecure_skip_verification(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.insecure_skip_verification", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct ApiGatewayIntegrationDynamic {
    tls_config: Option<DynamicBlock<ApiGatewayIntegrationTlsConfigEl>>,
}
