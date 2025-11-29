use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ApiGatewayRestApiData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key_source: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    binary_media_types: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_execute_api_endpoint: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fail_on_warnings: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_compression_size: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    put_rest_api_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_configuration: Option<Vec<ApiGatewayRestApiEndpointConfigurationEl>>,
    dynamic: ApiGatewayRestApiDynamic,
}

struct ApiGatewayRestApi_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApiGatewayRestApiData>,
}

#[derive(Clone)]
pub struct ApiGatewayRestApi(Rc<ApiGatewayRestApi_>);

impl ApiGatewayRestApi {
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

    #[doc = "Set the field `api_key_source`.\n"]
    pub fn set_api_key_source(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().api_key_source = Some(v.into());
        self
    }

    #[doc = "Set the field `binary_media_types`.\n"]
    pub fn set_binary_media_types(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().binary_media_types = Some(v.into());
        self
    }

    #[doc = "Set the field `body`.\n"]
    pub fn set_body(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().body = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `disable_execute_api_endpoint`.\n"]
    pub fn set_disable_execute_api_endpoint(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_execute_api_endpoint = Some(v.into());
        self
    }

    #[doc = "Set the field `fail_on_warnings`.\n"]
    pub fn set_fail_on_warnings(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().fail_on_warnings = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `minimum_compression_size`.\n"]
    pub fn set_minimum_compression_size(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().minimum_compression_size = Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().parameters = Some(v.into());
        self
    }

    #[doc = "Set the field `policy`.\n"]
    pub fn set_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().policy = Some(v.into());
        self
    }

    #[doc = "Set the field `put_rest_api_mode`.\n"]
    pub fn set_put_rest_api_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().put_rest_api_mode = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
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

    #[doc = "Set the field `endpoint_configuration`.\n"]
    pub fn set_endpoint_configuration(
        self,
        v: impl Into<BlockAssignable<ApiGatewayRestApiEndpointConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `api_key_source` after provisioning.\n"]
    pub fn api_key_source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_key_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `binary_media_types` after provisioning.\n"]
    pub fn binary_media_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.binary_media_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `body` after provisioning.\n"]
    pub fn body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.body", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disable_execute_api_endpoint` after provisioning.\n"]
    pub fn disable_execute_api_endpoint(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_execute_api_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_arn` after provisioning.\n"]
    pub fn execution_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fail_on_warnings` after provisioning.\n"]
    pub fn fail_on_warnings(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fail_on_warnings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `minimum_compression_size` after provisioning.\n"]
    pub fn minimum_compression_size(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum_compression_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `put_rest_api_mode` after provisioning.\n"]
    pub fn put_rest_api_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.put_rest_api_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_resource_id` after provisioning.\n"]
    pub fn root_resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_resource_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_configuration` after provisioning.\n"]
    pub fn endpoint_configuration(&self) -> ListRef<ApiGatewayRestApiEndpointConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.endpoint_configuration", self.extract_ref()))
    }
}

impl Referable for ApiGatewayRestApi {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ApiGatewayRestApi { }

impl ToListMappable for ApiGatewayRestApi {
    type O = ListRef<ApiGatewayRestApiRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ApiGatewayRestApi_ {
    fn extract_resource_type(&self) -> String {
        "aws_api_gateway_rest_api".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApiGatewayRestApi {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildApiGatewayRestApi {
    pub fn build(self, stack: &mut Stack) -> ApiGatewayRestApi {
        let out = ApiGatewayRestApi(Rc::new(ApiGatewayRestApi_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApiGatewayRestApiData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                api_key_source: core::default::Default::default(),
                binary_media_types: core::default::Default::default(),
                body: core::default::Default::default(),
                description: core::default::Default::default(),
                disable_execute_api_endpoint: core::default::Default::default(),
                fail_on_warnings: core::default::Default::default(),
                id: core::default::Default::default(),
                minimum_compression_size: core::default::Default::default(),
                name: self.name,
                parameters: core::default::Default::default(),
                policy: core::default::Default::default(),
                put_rest_api_mode: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                endpoint_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApiGatewayRestApiRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApiGatewayRestApiRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl ApiGatewayRestApiRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key_source` after provisioning.\n"]
    pub fn api_key_source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_key_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `binary_media_types` after provisioning.\n"]
    pub fn binary_media_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.binary_media_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `body` after provisioning.\n"]
    pub fn body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.body", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disable_execute_api_endpoint` after provisioning.\n"]
    pub fn disable_execute_api_endpoint(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_execute_api_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_arn` after provisioning.\n"]
    pub fn execution_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fail_on_warnings` after provisioning.\n"]
    pub fn fail_on_warnings(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fail_on_warnings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `minimum_compression_size` after provisioning.\n"]
    pub fn minimum_compression_size(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum_compression_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `put_rest_api_mode` after provisioning.\n"]
    pub fn put_rest_api_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.put_rest_api_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_resource_id` after provisioning.\n"]
    pub fn root_resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_resource_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_configuration` after provisioning.\n"]
    pub fn endpoint_configuration(&self) -> ListRef<ApiGatewayRestApiEndpointConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.endpoint_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ApiGatewayRestApiEndpointConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address_type: Option<PrimField<String>>,
    types: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_endpoint_ids: Option<SetField<PrimField<String>>>,
}

impl ApiGatewayRestApiEndpointConfigurationEl {
    #[doc = "Set the field `ip_address_type`.\n"]
    pub fn set_ip_address_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address_type = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_endpoint_ids`.\n"]
    pub fn set_vpc_endpoint_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.vpc_endpoint_ids = Some(v.into());
        self
    }
}

impl ToListMappable for ApiGatewayRestApiEndpointConfigurationEl {
    type O = BlockAssignable<ApiGatewayRestApiEndpointConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiGatewayRestApiEndpointConfigurationEl {
    #[doc = ""]
    pub types: ListField<PrimField<String>>,
}

impl BuildApiGatewayRestApiEndpointConfigurationEl {
    pub fn build(self) -> ApiGatewayRestApiEndpointConfigurationEl {
        ApiGatewayRestApiEndpointConfigurationEl {
            ip_address_type: core::default::Default::default(),
            types: self.types,
            vpc_endpoint_ids: core::default::Default::default(),
        }
    }
}

pub struct ApiGatewayRestApiEndpointConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApiGatewayRestApiEndpointConfigurationElRef {
    fn new(shared: StackShared, base: String) -> ApiGatewayRestApiEndpointConfigurationElRef {
        ApiGatewayRestApiEndpointConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiGatewayRestApiEndpointConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.base))
    }

    #[doc = "Get a reference to the value of field `types` after provisioning.\n"]
    pub fn types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.types", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_ids` after provisioning.\n"]
    pub fn vpc_endpoint_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.vpc_endpoint_ids", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApiGatewayRestApiDynamic {
    endpoint_configuration: Option<DynamicBlock<ApiGatewayRestApiEndpointConfigurationEl>>,
}
