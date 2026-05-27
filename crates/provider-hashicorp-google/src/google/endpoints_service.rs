use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct EndpointsServiceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    openapi_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protoc_output_base64: Option<PrimField<String>>,
    service_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<EndpointsServiceTimeoutsEl>,
}
struct EndpointsService_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EndpointsServiceData>,
}
#[derive(Clone)]
pub struct EndpointsService(Rc<EndpointsService_>);
impl EndpointsService {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `grpc_config`.\nThe full text of the Service Config YAML file (Example located here). If provided, must also provide protoc_output_base64. open_api config must not be provided."]
    pub fn set_grpc_config(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().grpc_config = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `openapi_config`.\nThe full text of the OpenAPI YAML configuration as described here. Either this, or both of grpc_config and protoc_output_base64 must be specified."]
    pub fn set_openapi_config(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().openapi_config = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe project ID that the service belongs to. If not provided, provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `protoc_output_base64`.\nThe full contents of the Service Descriptor File generated by protoc. This should be a compiled .pb file, base64-encoded."]
    pub fn set_protoc_output_base64(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().protoc_output_base64 = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<EndpointsServiceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `apis` after provisioning.\nA list of API objects."]
    pub fn apis(&self) -> ListRef<EndpointsServiceApisElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.apis", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config_id` after provisioning.\nThe autogenerated ID for the configuration that is rolled out as part of the creation of this resource. Must be provided to compute engine instances as a tag."]
    pub fn config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.config_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_address` after provisioning.\nThe address at which the service can be found - usually the same as the service name."]
    pub fn dns_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\nA list of Endpoint objects."]
    pub fn endpoints(&self) -> ListRef<EndpointsServiceEndpointsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `grpc_config` after provisioning.\nThe full text of the Service Config YAML file (Example located here). If provided, must also provide protoc_output_base64. open_api config must not be provided."]
    pub fn grpc_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.grpc_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `openapi_config` after provisioning.\nThe full text of the OpenAPI YAML configuration as described here. Either this, or both of grpc_config and protoc_output_base64 must be specified."]
    pub fn openapi_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.openapi_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project ID that the service belongs to. If not provided, provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protoc_output_base64` after provisioning.\nThe full contents of the Service Descriptor File generated by protoc. This should be a compiled .pb file, base64-encoded."]
    pub fn protoc_output_base64(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protoc_output_base64", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_name` after provisioning.\nThe name of the service. Usually of the form $apiname.endpoints.$projectid.cloud.goog."]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EndpointsServiceTimeoutsElRef {
        EndpointsServiceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for EndpointsService {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for EndpointsService {}
impl ToListMappable for EndpointsService {
    type O = ListRef<EndpointsServiceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for EndpointsService_ {
    fn extract_resource_type(&self) -> String {
        "google_endpoints_service".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildEndpointsService {
    pub tf_id: String,
    #[doc = "The name of the service. Usually of the form $apiname.endpoints.$projectid.cloud.goog."]
    pub service_name: PrimField<String>,
}
impl BuildEndpointsService {
    pub fn build(self, stack: &mut Stack) -> EndpointsService {
        let out = EndpointsService(Rc::new(EndpointsService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EndpointsServiceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                grpc_config: core::default::Default::default(),
                id: core::default::Default::default(),
                openapi_config: core::default::Default::default(),
                project: core::default::Default::default(),
                protoc_output_base64: core::default::Default::default(),
                service_name: self.service_name,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct EndpointsServiceRef {
    shared: StackShared,
    base: String,
}
impl Ref for EndpointsServiceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl EndpointsServiceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `apis` after provisioning.\nA list of API objects."]
    pub fn apis(&self) -> ListRef<EndpointsServiceApisElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.apis", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config_id` after provisioning.\nThe autogenerated ID for the configuration that is rolled out as part of the creation of this resource. Must be provided to compute engine instances as a tag."]
    pub fn config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.config_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_address` after provisioning.\nThe address at which the service can be found - usually the same as the service name."]
    pub fn dns_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\nA list of Endpoint objects."]
    pub fn endpoints(&self) -> ListRef<EndpointsServiceEndpointsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `grpc_config` after provisioning.\nThe full text of the Service Config YAML file (Example located here). If provided, must also provide protoc_output_base64. open_api config must not be provided."]
    pub fn grpc_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.grpc_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `openapi_config` after provisioning.\nThe full text of the OpenAPI YAML configuration as described here. Either this, or both of grpc_config and protoc_output_base64 must be specified."]
    pub fn openapi_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.openapi_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project ID that the service belongs to. If not provided, provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protoc_output_base64` after provisioning.\nThe full contents of the Service Descriptor File generated by protoc. This should be a compiled .pb file, base64-encoded."]
    pub fn protoc_output_base64(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protoc_output_base64", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_name` after provisioning.\nThe name of the service. Usually of the form $apiname.endpoints.$projectid.cloud.goog."]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EndpointsServiceTimeoutsElRef {
        EndpointsServiceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct EndpointsServiceApisElMethodsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    syntax: Option<PrimField<String>>,
}
impl EndpointsServiceApisElMethodsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `request_type`.\n"]
    pub fn set_request_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.request_type = Some(v.into());
        self
    }
    #[doc = "Set the field `response_type`.\n"]
    pub fn set_response_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response_type = Some(v.into());
        self
    }
    #[doc = "Set the field `syntax`.\n"]
    pub fn set_syntax(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.syntax = Some(v.into());
        self
    }
}
impl ToListMappable for EndpointsServiceApisElMethodsEl {
    type O = BlockAssignable<EndpointsServiceApisElMethodsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEndpointsServiceApisElMethodsEl {}
impl BuildEndpointsServiceApisElMethodsEl {
    pub fn build(self) -> EndpointsServiceApisElMethodsEl {
        EndpointsServiceApisElMethodsEl {
            name: core::default::Default::default(),
            request_type: core::default::Default::default(),
            response_type: core::default::Default::default(),
            syntax: core::default::Default::default(),
        }
    }
}
pub struct EndpointsServiceApisElMethodsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EndpointsServiceApisElMethodsElRef {
    fn new(shared: StackShared, base: String) -> EndpointsServiceApisElMethodsElRef {
        EndpointsServiceApisElMethodsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EndpointsServiceApisElMethodsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `request_type` after provisioning.\n"]
    pub fn request_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_type", self.base))
    }
    #[doc = "Get a reference to the value of field `response_type` after provisioning.\n"]
    pub fn response_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.response_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `syntax` after provisioning.\n"]
    pub fn syntax(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.syntax", self.base))
    }
}
#[derive(Serialize)]
pub struct EndpointsServiceApisEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    methods: Option<ListField<EndpointsServiceApisElMethodsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    syntax: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl EndpointsServiceApisEl {
    #[doc = "Set the field `methods`.\n"]
    pub fn set_methods(mut self, v: impl Into<ListField<EndpointsServiceApisElMethodsEl>>) -> Self {
        self.methods = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `syntax`.\n"]
    pub fn set_syntax(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.syntax = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for EndpointsServiceApisEl {
    type O = BlockAssignable<EndpointsServiceApisEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEndpointsServiceApisEl {}
impl BuildEndpointsServiceApisEl {
    pub fn build(self) -> EndpointsServiceApisEl {
        EndpointsServiceApisEl {
            methods: core::default::Default::default(),
            name: core::default::Default::default(),
            syntax: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct EndpointsServiceApisElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EndpointsServiceApisElRef {
    fn new(shared: StackShared, base: String) -> EndpointsServiceApisElRef {
        EndpointsServiceApisElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EndpointsServiceApisElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `methods` after provisioning.\n"]
    pub fn methods(&self) -> ListRef<EndpointsServiceApisElMethodsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.methods", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `syntax` after provisioning.\n"]
    pub fn syntax(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.syntax", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize)]
pub struct EndpointsServiceEndpointsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl EndpointsServiceEndpointsEl {
    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for EndpointsServiceEndpointsEl {
    type O = BlockAssignable<EndpointsServiceEndpointsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEndpointsServiceEndpointsEl {}
impl BuildEndpointsServiceEndpointsEl {
    pub fn build(self) -> EndpointsServiceEndpointsEl {
        EndpointsServiceEndpointsEl {
            address: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct EndpointsServiceEndpointsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EndpointsServiceEndpointsElRef {
    fn new(shared: StackShared, base: String) -> EndpointsServiceEndpointsElRef {
        EndpointsServiceEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EndpointsServiceEndpointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct EndpointsServiceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl EndpointsServiceTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for EndpointsServiceTimeoutsEl {
    type O = BlockAssignable<EndpointsServiceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEndpointsServiceTimeoutsEl {}
impl BuildEndpointsServiceTimeoutsEl {
    pub fn build(self) -> EndpointsServiceTimeoutsEl {
        EndpointsServiceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct EndpointsServiceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EndpointsServiceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> EndpointsServiceTimeoutsElRef {
        EndpointsServiceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EndpointsServiceTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
