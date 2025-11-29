use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ApiGatewayMethodData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key_required: Option<PrimField<bool>>,
    authorization: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_scopes: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorizer_id: Option<PrimField<String>>,
    http_method: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operation_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_models: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_parameters: Option<RecField<PrimField<bool>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_validator_id: Option<PrimField<String>>,
    resource_id: PrimField<String>,
    rest_api_id: PrimField<String>,
}

struct ApiGatewayMethod_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApiGatewayMethodData>,
}

#[derive(Clone)]
pub struct ApiGatewayMethod(Rc<ApiGatewayMethod_>);

impl ApiGatewayMethod {
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

    #[doc = "Set the field `api_key_required`.\n"]
    pub fn set_api_key_required(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().api_key_required = Some(v.into());
        self
    }

    #[doc = "Set the field `authorization_scopes`.\n"]
    pub fn set_authorization_scopes(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().authorization_scopes = Some(v.into());
        self
    }

    #[doc = "Set the field `authorizer_id`.\n"]
    pub fn set_authorizer_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().authorizer_id = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `operation_name`.\n"]
    pub fn set_operation_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().operation_name = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `request_models`.\n"]
    pub fn set_request_models(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().request_models = Some(v.into());
        self
    }

    #[doc = "Set the field `request_parameters`.\n"]
    pub fn set_request_parameters(self, v: impl Into<RecField<PrimField<bool>>>) -> Self {
        self.0.data.borrow_mut().request_parameters = Some(v.into());
        self
    }

    #[doc = "Set the field `request_validator_id`.\n"]
    pub fn set_request_validator_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().request_validator_id = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `api_key_required` after provisioning.\n"]
    pub fn api_key_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_key_required", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorization_scopes` after provisioning.\n"]
    pub fn authorization_scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.authorization_scopes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorizer_id` after provisioning.\n"]
    pub fn authorizer_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorizer_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_method` after provisioning.\n"]
    pub fn http_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_method", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operation_name` after provisioning.\n"]
    pub fn operation_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operation_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_models` after provisioning.\n"]
    pub fn request_models(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.request_models", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_parameters` after provisioning.\n"]
    pub fn request_parameters(&self) -> RecRef<PrimExpr<bool>> {
        RecRef::new(self.shared().clone(), format!("{}.request_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_validator_id` after provisioning.\n"]
    pub fn request_validator_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_validator_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rest_api_id` after provisioning.\n"]
    pub fn rest_api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rest_api_id", self.extract_ref()))
    }
}

impl Referable for ApiGatewayMethod {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ApiGatewayMethod { }

impl ToListMappable for ApiGatewayMethod {
    type O = ListRef<ApiGatewayMethodRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ApiGatewayMethod_ {
    fn extract_resource_type(&self) -> String {
        "aws_api_gateway_method".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApiGatewayMethod {
    pub tf_id: String,
    #[doc = ""]
    pub authorization: PrimField<String>,
    #[doc = ""]
    pub http_method: PrimField<String>,
    #[doc = ""]
    pub resource_id: PrimField<String>,
    #[doc = ""]
    pub rest_api_id: PrimField<String>,
}

impl BuildApiGatewayMethod {
    pub fn build(self, stack: &mut Stack) -> ApiGatewayMethod {
        let out = ApiGatewayMethod(Rc::new(ApiGatewayMethod_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApiGatewayMethodData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                api_key_required: core::default::Default::default(),
                authorization: self.authorization,
                authorization_scopes: core::default::Default::default(),
                authorizer_id: core::default::Default::default(),
                http_method: self.http_method,
                id: core::default::Default::default(),
                operation_name: core::default::Default::default(),
                region: core::default::Default::default(),
                request_models: core::default::Default::default(),
                request_parameters: core::default::Default::default(),
                request_validator_id: core::default::Default::default(),
                resource_id: self.resource_id,
                rest_api_id: self.rest_api_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApiGatewayMethodRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApiGatewayMethodRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl ApiGatewayMethodRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key_required` after provisioning.\n"]
    pub fn api_key_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_key_required", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorization_scopes` after provisioning.\n"]
    pub fn authorization_scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.authorization_scopes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorizer_id` after provisioning.\n"]
    pub fn authorizer_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorizer_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_method` after provisioning.\n"]
    pub fn http_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_method", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operation_name` after provisioning.\n"]
    pub fn operation_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operation_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_models` after provisioning.\n"]
    pub fn request_models(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.request_models", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_parameters` after provisioning.\n"]
    pub fn request_parameters(&self) -> RecRef<PrimExpr<bool>> {
        RecRef::new(self.shared().clone(), format!("{}.request_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_validator_id` after provisioning.\n"]
    pub fn request_validator_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_validator_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rest_api_id` after provisioning.\n"]
    pub fn rest_api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rest_api_id", self.extract_ref()))
    }
}
