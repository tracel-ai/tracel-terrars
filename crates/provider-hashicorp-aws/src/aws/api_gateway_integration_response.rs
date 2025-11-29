use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ApiGatewayIntegrationResponseData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_handling: Option<PrimField<String>>,
    http_method: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    resource_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_parameters: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_templates: Option<RecField<PrimField<String>>>,
    rest_api_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selection_pattern: Option<PrimField<String>>,
    status_code: PrimField<String>,
}

struct ApiGatewayIntegrationResponse_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApiGatewayIntegrationResponseData>,
}

#[derive(Clone)]
pub struct ApiGatewayIntegrationResponse(Rc<ApiGatewayIntegrationResponse_>);

impl ApiGatewayIntegrationResponse {
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

    #[doc = "Set the field `content_handling`.\n"]
    pub fn set_content_handling(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().content_handling = Some(v.into());
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

    #[doc = "Set the field `response_parameters`.\n"]
    pub fn set_response_parameters(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().response_parameters = Some(v.into());
        self
    }

    #[doc = "Set the field `response_templates`.\n"]
    pub fn set_response_templates(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().response_templates = Some(v.into());
        self
    }

    #[doc = "Set the field `selection_pattern`.\n"]
    pub fn set_selection_pattern(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().selection_pattern = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `content_handling` after provisioning.\n"]
    pub fn content_handling(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_handling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_method` after provisioning.\n"]
    pub fn http_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_method", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `response_parameters` after provisioning.\n"]
    pub fn response_parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.response_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `response_templates` after provisioning.\n"]
    pub fn response_templates(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.response_templates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rest_api_id` after provisioning.\n"]
    pub fn rest_api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rest_api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `selection_pattern` after provisioning.\n"]
    pub fn selection_pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.selection_pattern", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.extract_ref()))
    }
}

impl Referable for ApiGatewayIntegrationResponse {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ApiGatewayIntegrationResponse { }

impl ToListMappable for ApiGatewayIntegrationResponse {
    type O = ListRef<ApiGatewayIntegrationResponseRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ApiGatewayIntegrationResponse_ {
    fn extract_resource_type(&self) -> String {
        "aws_api_gateway_integration_response".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApiGatewayIntegrationResponse {
    pub tf_id: String,
    #[doc = ""]
    pub http_method: PrimField<String>,
    #[doc = ""]
    pub resource_id: PrimField<String>,
    #[doc = ""]
    pub rest_api_id: PrimField<String>,
    #[doc = ""]
    pub status_code: PrimField<String>,
}

impl BuildApiGatewayIntegrationResponse {
    pub fn build(self, stack: &mut Stack) -> ApiGatewayIntegrationResponse {
        let out = ApiGatewayIntegrationResponse(Rc::new(ApiGatewayIntegrationResponse_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApiGatewayIntegrationResponseData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                content_handling: core::default::Default::default(),
                http_method: self.http_method,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                resource_id: self.resource_id,
                response_parameters: core::default::Default::default(),
                response_templates: core::default::Default::default(),
                rest_api_id: self.rest_api_id,
                selection_pattern: core::default::Default::default(),
                status_code: self.status_code,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApiGatewayIntegrationResponseRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApiGatewayIntegrationResponseRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl ApiGatewayIntegrationResponseRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content_handling` after provisioning.\n"]
    pub fn content_handling(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_handling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_method` after provisioning.\n"]
    pub fn http_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_method", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `response_parameters` after provisioning.\n"]
    pub fn response_parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.response_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `response_templates` after provisioning.\n"]
    pub fn response_templates(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.response_templates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rest_api_id` after provisioning.\n"]
    pub fn rest_api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rest_api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `selection_pattern` after provisioning.\n"]
    pub fn selection_pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.selection_pattern", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.extract_ref()))
    }
}
