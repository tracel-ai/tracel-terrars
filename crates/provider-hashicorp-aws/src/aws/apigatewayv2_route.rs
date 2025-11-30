use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct Apigatewayv2RouteData {
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
    api_key_required: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_scopes: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorizer_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_selection_expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operation_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_models: Option<RecField<PrimField<String>>>,
    route_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_response_selection_expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_parameter: Option<Vec<Apigatewayv2RouteRequestParameterEl>>,
    dynamic: Apigatewayv2RouteDynamic,
}

struct Apigatewayv2Route_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Apigatewayv2RouteData>,
}

#[derive(Clone)]
pub struct Apigatewayv2Route(Rc<Apigatewayv2Route_>);

impl Apigatewayv2Route {
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

    #[doc = "Set the field `authorization_type`.\n"]
    pub fn set_authorization_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().authorization_type = Some(v.into());
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

    #[doc = "Set the field `model_selection_expression`.\n"]
    pub fn set_model_selection_expression(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().model_selection_expression = Some(v.into());
        self
    }

    #[doc = "Set the field `operation_name`.\n"]
    pub fn set_operation_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().operation_name = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `request_models`.\n"]
    pub fn set_request_models(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().request_models = Some(v.into());
        self
    }

    #[doc = "Set the field `route_response_selection_expression`.\n"]
    pub fn set_route_response_selection_expression(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().route_response_selection_expression = Some(v.into());
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().target = Some(v.into());
        self
    }

    #[doc = "Set the field `request_parameter`.\n"]
    pub fn set_request_parameter(
        self,
        v: impl Into<BlockAssignable<Apigatewayv2RouteRequestParameterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().request_parameter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.request_parameter = Some(d);
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

    #[doc = "Get a reference to the value of field `api_key_required` after provisioning.\n"]
    pub fn api_key_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_key_required", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `authorization_scopes` after provisioning.\n"]
    pub fn authorization_scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.authorization_scopes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `authorization_type` after provisioning.\n"]
    pub fn authorization_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorization_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `authorizer_id` after provisioning.\n"]
    pub fn authorizer_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorizer_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_selection_expression` after provisioning.\n"]
    pub fn model_selection_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.model_selection_expression", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `operation_name` after provisioning.\n"]
    pub fn operation_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.operation_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `request_models` after provisioning.\n"]
    pub fn request_models(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.request_models", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `route_key` after provisioning.\n"]
    pub fn route_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.route_key", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `route_response_selection_expression` after provisioning.\n"]
    pub fn route_response_selection_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.route_response_selection_expression", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target", self.extract_ref()),
        )
    }
}

impl Referable for Apigatewayv2Route {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for Apigatewayv2Route {}

impl ToListMappable for Apigatewayv2Route {
    type O = ListRef<Apigatewayv2RouteRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Apigatewayv2Route_ {
    fn extract_resource_type(&self) -> String {
        "aws_apigatewayv2_route".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApigatewayv2Route {
    pub tf_id: String,
    #[doc = ""]
    pub api_id: PrimField<String>,
    #[doc = ""]
    pub route_key: PrimField<String>,
}

impl BuildApigatewayv2Route {
    pub fn build(self, stack: &mut Stack) -> Apigatewayv2Route {
        let out = Apigatewayv2Route(Rc::new(Apigatewayv2Route_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Apigatewayv2RouteData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                api_id: self.api_id,
                api_key_required: core::default::Default::default(),
                authorization_scopes: core::default::Default::default(),
                authorization_type: core::default::Default::default(),
                authorizer_id: core::default::Default::default(),
                id: core::default::Default::default(),
                model_selection_expression: core::default::Default::default(),
                operation_name: core::default::Default::default(),
                region: core::default::Default::default(),
                request_models: core::default::Default::default(),
                route_key: self.route_key,
                route_response_selection_expression: core::default::Default::default(),
                target: core::default::Default::default(),
                request_parameter: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Apigatewayv2RouteRef {
    shared: StackShared,
    base: String,
}

impl Ref for Apigatewayv2RouteRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl Apigatewayv2RouteRef {
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

    #[doc = "Get a reference to the value of field `api_key_required` after provisioning.\n"]
    pub fn api_key_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_key_required", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `authorization_scopes` after provisioning.\n"]
    pub fn authorization_scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.authorization_scopes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `authorization_type` after provisioning.\n"]
    pub fn authorization_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorization_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `authorizer_id` after provisioning.\n"]
    pub fn authorizer_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorizer_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_selection_expression` after provisioning.\n"]
    pub fn model_selection_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.model_selection_expression", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `operation_name` after provisioning.\n"]
    pub fn operation_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.operation_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `request_models` after provisioning.\n"]
    pub fn request_models(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.request_models", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `route_key` after provisioning.\n"]
    pub fn route_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.route_key", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `route_response_selection_expression` after provisioning.\n"]
    pub fn route_response_selection_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.route_response_selection_expression", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct Apigatewayv2RouteRequestParameterEl {
    request_parameter_key: PrimField<String>,
    required: PrimField<bool>,
}

impl Apigatewayv2RouteRequestParameterEl {}

impl ToListMappable for Apigatewayv2RouteRequestParameterEl {
    type O = BlockAssignable<Apigatewayv2RouteRequestParameterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigatewayv2RouteRequestParameterEl {
    #[doc = ""]
    pub request_parameter_key: PrimField<String>,
    #[doc = ""]
    pub required: PrimField<bool>,
}

impl BuildApigatewayv2RouteRequestParameterEl {
    pub fn build(self) -> Apigatewayv2RouteRequestParameterEl {
        Apigatewayv2RouteRequestParameterEl {
            request_parameter_key: self.request_parameter_key,
            required: self.required,
        }
    }
}

pub struct Apigatewayv2RouteRequestParameterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Apigatewayv2RouteRequestParameterElRef {
    fn new(shared: StackShared, base: String) -> Apigatewayv2RouteRequestParameterElRef {
        Apigatewayv2RouteRequestParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Apigatewayv2RouteRequestParameterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `request_parameter_key` after provisioning.\n"]
    pub fn request_parameter_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_parameter_key", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `required` after provisioning.\n"]
    pub fn required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }
}

#[derive(Serialize, Default)]
struct Apigatewayv2RouteDynamic {
    request_parameter: Option<DynamicBlock<Apigatewayv2RouteRequestParameterEl>>,
}
