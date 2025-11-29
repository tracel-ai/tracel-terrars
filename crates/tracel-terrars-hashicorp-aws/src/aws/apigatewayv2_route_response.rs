use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Apigatewayv2RouteResponseData {
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
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_selection_expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_models: Option<RecField<PrimField<String>>>,
    route_id: PrimField<String>,
    route_response_key: PrimField<String>,
}

struct Apigatewayv2RouteResponse_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Apigatewayv2RouteResponseData>,
}

#[derive(Clone)]
pub struct Apigatewayv2RouteResponse(Rc<Apigatewayv2RouteResponse_>);

impl Apigatewayv2RouteResponse {
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

    #[doc = "Set the field `model_selection_expression`.\n"]
    pub fn set_model_selection_expression(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().model_selection_expression = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `response_models`.\n"]
    pub fn set_response_models(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().response_models = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_selection_expression` after provisioning.\n"]
    pub fn model_selection_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_selection_expression", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `response_models` after provisioning.\n"]
    pub fn response_models(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.response_models", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_id` after provisioning.\n"]
    pub fn route_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.route_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_response_key` after provisioning.\n"]
    pub fn route_response_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.route_response_key", self.extract_ref()))
    }
}

impl Referable for Apigatewayv2RouteResponse {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Apigatewayv2RouteResponse { }

impl ToListMappable for Apigatewayv2RouteResponse {
    type O = ListRef<Apigatewayv2RouteResponseRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Apigatewayv2RouteResponse_ {
    fn extract_resource_type(&self) -> String {
        "aws_apigatewayv2_route_response".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApigatewayv2RouteResponse {
    pub tf_id: String,
    #[doc = ""]
    pub api_id: PrimField<String>,
    #[doc = ""]
    pub route_id: PrimField<String>,
    #[doc = ""]
    pub route_response_key: PrimField<String>,
}

impl BuildApigatewayv2RouteResponse {
    pub fn build(self, stack: &mut Stack) -> Apigatewayv2RouteResponse {
        let out = Apigatewayv2RouteResponse(Rc::new(Apigatewayv2RouteResponse_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Apigatewayv2RouteResponseData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                api_id: self.api_id,
                id: core::default::Default::default(),
                model_selection_expression: core::default::Default::default(),
                region: core::default::Default::default(),
                response_models: core::default::Default::default(),
                route_id: self.route_id,
                route_response_key: self.route_response_key,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Apigatewayv2RouteResponseRef {
    shared: StackShared,
    base: String,
}

impl Ref for Apigatewayv2RouteResponseRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl Apigatewayv2RouteResponseRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_selection_expression` after provisioning.\n"]
    pub fn model_selection_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_selection_expression", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `response_models` after provisioning.\n"]
    pub fn response_models(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.response_models", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_id` after provisioning.\n"]
    pub fn route_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.route_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_response_key` after provisioning.\n"]
    pub fn route_response_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.route_response_key", self.extract_ref()))
    }
}
