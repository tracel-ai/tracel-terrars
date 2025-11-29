use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Apigatewayv2AuthorizerData {
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
    authorizer_credentials_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorizer_payload_format_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorizer_result_ttl_in_seconds: Option<PrimField<f64>>,
    authorizer_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorizer_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_simple_responses: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_sources: Option<SetField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jwt_configuration: Option<Vec<Apigatewayv2AuthorizerJwtConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<Apigatewayv2AuthorizerTimeoutsEl>,
    dynamic: Apigatewayv2AuthorizerDynamic,
}

struct Apigatewayv2Authorizer_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Apigatewayv2AuthorizerData>,
}

#[derive(Clone)]
pub struct Apigatewayv2Authorizer(Rc<Apigatewayv2Authorizer_>);

impl Apigatewayv2Authorizer {
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

    #[doc = "Set the field `authorizer_credentials_arn`.\n"]
    pub fn set_authorizer_credentials_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().authorizer_credentials_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `authorizer_payload_format_version`.\n"]
    pub fn set_authorizer_payload_format_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().authorizer_payload_format_version = Some(v.into());
        self
    }

    #[doc = "Set the field `authorizer_result_ttl_in_seconds`.\n"]
    pub fn set_authorizer_result_ttl_in_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().authorizer_result_ttl_in_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `authorizer_uri`.\n"]
    pub fn set_authorizer_uri(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().authorizer_uri = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_simple_responses`.\n"]
    pub fn set_enable_simple_responses(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_simple_responses = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `identity_sources`.\n"]
    pub fn set_identity_sources(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().identity_sources = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `jwt_configuration`.\n"]
    pub fn set_jwt_configuration(self, v: impl Into<BlockAssignable<Apigatewayv2AuthorizerJwtConfigurationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().jwt_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.jwt_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<Apigatewayv2AuthorizerTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorizer_credentials_arn` after provisioning.\n"]
    pub fn authorizer_credentials_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorizer_credentials_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorizer_payload_format_version` after provisioning.\n"]
    pub fn authorizer_payload_format_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorizer_payload_format_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorizer_result_ttl_in_seconds` after provisioning.\n"]
    pub fn authorizer_result_ttl_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorizer_result_ttl_in_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorizer_type` after provisioning.\n"]
    pub fn authorizer_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorizer_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorizer_uri` after provisioning.\n"]
    pub fn authorizer_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorizer_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_simple_responses` after provisioning.\n"]
    pub fn enable_simple_responses(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_simple_responses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_sources` after provisioning.\n"]
    pub fn identity_sources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.identity_sources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `jwt_configuration` after provisioning.\n"]
    pub fn jwt_configuration(&self) -> ListRef<Apigatewayv2AuthorizerJwtConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.jwt_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Apigatewayv2AuthorizerTimeoutsElRef {
        Apigatewayv2AuthorizerTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for Apigatewayv2Authorizer {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Apigatewayv2Authorizer { }

impl ToListMappable for Apigatewayv2Authorizer {
    type O = ListRef<Apigatewayv2AuthorizerRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Apigatewayv2Authorizer_ {
    fn extract_resource_type(&self) -> String {
        "aws_apigatewayv2_authorizer".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApigatewayv2Authorizer {
    pub tf_id: String,
    #[doc = ""]
    pub api_id: PrimField<String>,
    #[doc = ""]
    pub authorizer_type: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildApigatewayv2Authorizer {
    pub fn build(self, stack: &mut Stack) -> Apigatewayv2Authorizer {
        let out = Apigatewayv2Authorizer(Rc::new(Apigatewayv2Authorizer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Apigatewayv2AuthorizerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                api_id: self.api_id,
                authorizer_credentials_arn: core::default::Default::default(),
                authorizer_payload_format_version: core::default::Default::default(),
                authorizer_result_ttl_in_seconds: core::default::Default::default(),
                authorizer_type: self.authorizer_type,
                authorizer_uri: core::default::Default::default(),
                enable_simple_responses: core::default::Default::default(),
                id: core::default::Default::default(),
                identity_sources: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                jwt_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Apigatewayv2AuthorizerRef {
    shared: StackShared,
    base: String,
}

impl Ref for Apigatewayv2AuthorizerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl Apigatewayv2AuthorizerRef {
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

    #[doc = "Get a reference to the value of field `authorizer_credentials_arn` after provisioning.\n"]
    pub fn authorizer_credentials_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorizer_credentials_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorizer_payload_format_version` after provisioning.\n"]
    pub fn authorizer_payload_format_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorizer_payload_format_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorizer_result_ttl_in_seconds` after provisioning.\n"]
    pub fn authorizer_result_ttl_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorizer_result_ttl_in_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorizer_type` after provisioning.\n"]
    pub fn authorizer_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorizer_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorizer_uri` after provisioning.\n"]
    pub fn authorizer_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorizer_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_simple_responses` after provisioning.\n"]
    pub fn enable_simple_responses(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_simple_responses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_sources` after provisioning.\n"]
    pub fn identity_sources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.identity_sources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `jwt_configuration` after provisioning.\n"]
    pub fn jwt_configuration(&self) -> ListRef<Apigatewayv2AuthorizerJwtConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.jwt_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Apigatewayv2AuthorizerTimeoutsElRef {
        Apigatewayv2AuthorizerTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Apigatewayv2AuthorizerJwtConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audience: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<PrimField<String>>,
}

impl Apigatewayv2AuthorizerJwtConfigurationEl {
    #[doc = "Set the field `audience`.\n"]
    pub fn set_audience(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.audience = Some(v.into());
        self
    }

    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issuer = Some(v.into());
        self
    }
}

impl ToListMappable for Apigatewayv2AuthorizerJwtConfigurationEl {
    type O = BlockAssignable<Apigatewayv2AuthorizerJwtConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigatewayv2AuthorizerJwtConfigurationEl {}

impl BuildApigatewayv2AuthorizerJwtConfigurationEl {
    pub fn build(self) -> Apigatewayv2AuthorizerJwtConfigurationEl {
        Apigatewayv2AuthorizerJwtConfigurationEl {
            audience: core::default::Default::default(),
            issuer: core::default::Default::default(),
        }
    }
}

pub struct Apigatewayv2AuthorizerJwtConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Apigatewayv2AuthorizerJwtConfigurationElRef {
    fn new(shared: StackShared, base: String) -> Apigatewayv2AuthorizerJwtConfigurationElRef {
        Apigatewayv2AuthorizerJwtConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Apigatewayv2AuthorizerJwtConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audience` after provisioning.\n"]
    pub fn audience(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.audience", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }
}

#[derive(Serialize)]
pub struct Apigatewayv2AuthorizerTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl Apigatewayv2AuthorizerTimeoutsEl {
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
}

impl ToListMappable for Apigatewayv2AuthorizerTimeoutsEl {
    type O = BlockAssignable<Apigatewayv2AuthorizerTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigatewayv2AuthorizerTimeoutsEl {}

impl BuildApigatewayv2AuthorizerTimeoutsEl {
    pub fn build(self) -> Apigatewayv2AuthorizerTimeoutsEl {
        Apigatewayv2AuthorizerTimeoutsEl { delete: core::default::Default::default() }
    }
}

pub struct Apigatewayv2AuthorizerTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Apigatewayv2AuthorizerTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> Apigatewayv2AuthorizerTimeoutsElRef {
        Apigatewayv2AuthorizerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Apigatewayv2AuthorizerTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}

#[derive(Serialize, Default)]
struct Apigatewayv2AuthorizerDynamic {
    jwt_configuration: Option<DynamicBlock<Apigatewayv2AuthorizerJwtConfigurationEl>>,
}
