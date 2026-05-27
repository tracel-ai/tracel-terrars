use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkServicesAuthzExtensionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authority: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fail_open: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forward_headers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancing_scheme: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RecField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    service: PrimField<String>,
    timeout: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wire_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkServicesAuthzExtensionTimeoutsEl>,
}
struct NetworkServicesAuthzExtension_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkServicesAuthzExtensionData>,
}
#[derive(Clone)]
pub struct NetworkServicesAuthzExtension(Rc<NetworkServicesAuthzExtension_>);
impl NetworkServicesAuthzExtension {
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
    #[doc = "Set the field `authority`.\nThe :authority header in the gRPC request sent from Envoy to the extension service."]
    pub fn set_authority(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().authority = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nA human-readable description of the resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `fail_open`.\nDetermines how the proxy behaves if the call to the extension fails or times out.\nWhen set to TRUE, request or response processing continues without error. Any subsequent extensions in the extension chain are also executed. When set to FALSE or the default setting of FALSE is used, one of the following happens:\n* If response headers have not been delivered to the downstream client, a generic 500 error is returned to the client. The error response can be tailored by configuring a custom error response in the load balancer.\n* If response headers have been delivered, then the HTTP stream to the downstream client is reset."]
    pub fn set_fail_open(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().fail_open = Some(v.into());
        self
    }
    #[doc = "Set the field `forward_headers`.\nList of the HTTP headers to forward to the extension (from the client). If omitted, all headers are sent. Each element is a string indicating the header name."]
    pub fn set_forward_headers(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().forward_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nSet of labels associated with the AuthzExtension resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `load_balancing_scheme`.\nRequired when the service points to a backend service. All backend services and forwarding rules referenced by\nthis extension must share the same load balancing scheme. For more information, refer to\n[Backend services overview](https://cloud.google.com/load-balancing/docs/backend-service). Possible values: [\"INTERNAL_MANAGED\", \"EXTERNAL_MANAGED\"]"]
    pub fn set_load_balancing_scheme(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().load_balancing_scheme = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata`.\nThe metadata provided here is included as part of the metadata_context (of type google.protobuf.Struct) in the ProcessingRequest message sent to the extension server. The metadata is available under the namespace com.google.authz_extension.<resourceName>. The following variables are supported in the metadata Struct:\n\n{forwarding_rule_id} - substituted with the forwarding rule's fully qualified resource name."]
    pub fn set_metadata(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().metadata = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `wire_format`.\nThe format of communication supported by the callout extension. Applicable only when the policyProfile is REQUEST_AUTHZ.\nThis field is supported only for regional AuthzExtension resources. If not specified, the default value\nEXT_PROC_GRPC is used. Global AuthzExtension resources use the EXT_PROC_GRPC wire format.\n\nSupported values:\n- WIRE_FORMAT_UNSPECIFIED:\n    No wire format is explicitly specified. The backend automatically\n    defaults this value to EXT_PROC_GRPC.\n- EXT_PROC_GRPC:\n    Uses Envoy's External Processing (ext_proc) gRPC API over a single\n    gRPC stream. The backend service must support HTTP/2 or H2C.\n    All supported events for a client request are sent over the same\n    gRPC stream. This is the default wire format.\n- EXT_AUTHZ_GRPC:\n    Uses Envoy's external authorization (ext_authz) gRPC API.\n    The backend service must support HTTP/2 or H2C.\n    This option is only supported for regional AuthzExtension resources. Possible values: [\"WIRE_FORMAT_UNSPECIFIED\", \"EXT_PROC_GRPC\", \"EXT_AUTHZ_GRPC\"]"]
    pub fn set_wire_format(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().wire_format = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkServicesAuthzExtensionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `authority` after provisioning.\nThe :authority header in the gRPC request sent from Envoy to the extension service."]
    pub fn authority(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the resource was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fail_open` after provisioning.\nDetermines how the proxy behaves if the call to the extension fails or times out.\nWhen set to TRUE, request or response processing continues without error. Any subsequent extensions in the extension chain are also executed. When set to FALSE or the default setting of FALSE is used, one of the following happens:\n* If response headers have not been delivered to the downstream client, a generic 500 error is returned to the client. The error response can be tailored by configuring a custom error response in the load balancer.\n* If response headers have been delivered, then the HTTP stream to the downstream client is reset."]
    pub fn fail_open(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fail_open", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `forward_headers` after provisioning.\nList of the HTTP headers to forward to the extension (from the client). If omitted, all headers are sent. Each element is a string indicating the header name."]
    pub fn forward_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.forward_headers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of labels associated with the AuthzExtension resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancing_scheme` after provisioning.\nRequired when the service points to a backend service. All backend services and forwarding rules referenced by\nthis extension must share the same load balancing scheme. For more information, refer to\n[Backend services overview](https://cloud.google.com/load-balancing/docs/backend-service). Possible values: [\"INTERNAL_MANAGED\", \"EXTERNAL_MANAGED\"]"]
    pub fn load_balancing_scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancing_scheme", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nThe metadata provided here is included as part of the metadata_context (of type google.protobuf.Struct) in the ProcessingRequest message sent to the extension server. The metadata is available under the namespace com.google.authz_extension.<resourceName>. The following variables are supported in the metadata Struct:\n\n{forwarding_rule_id} - substituted with the forwarding rule's fully qualified resource name."]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Name of the AuthzExtension resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nThe service that runs the extension.\nThe following values and formats are accepted:\n* 'iap.googleapis.com' when the policyProfile is set to REQUEST_AUTHZ\n* 'modelarmor.{{region}}.rep.googleapis.com' when the policyProfile is set to CONTENT_AUTHZ\n* A fully qualified domain name that can be resolved by the dataplane\n* Backend service resource URI of the form 'https://www.googleapis.com/compute/v1/projects/{{project}}/regions/{{region}}/backendServices/{{name}}' or 'https://www.googleapis.com/compute/v1/projects/{{project}}/global/backendServices/{{name}}}}'"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\nSpecifies the timeout for each individual message on the stream. The timeout must be between 10-10000 milliseconds."]
    pub fn timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the resource was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `wire_format` after provisioning.\nThe format of communication supported by the callout extension. Applicable only when the policyProfile is REQUEST_AUTHZ.\nThis field is supported only for regional AuthzExtension resources. If not specified, the default value\nEXT_PROC_GRPC is used. Global AuthzExtension resources use the EXT_PROC_GRPC wire format.\n\nSupported values:\n- WIRE_FORMAT_UNSPECIFIED:\n    No wire format is explicitly specified. The backend automatically\n    defaults this value to EXT_PROC_GRPC.\n- EXT_PROC_GRPC:\n    Uses Envoy's External Processing (ext_proc) gRPC API over a single\n    gRPC stream. The backend service must support HTTP/2 or H2C.\n    All supported events for a client request are sent over the same\n    gRPC stream. This is the default wire format.\n- EXT_AUTHZ_GRPC:\n    Uses Envoy's external authorization (ext_authz) gRPC API.\n    The backend service must support HTTP/2 or H2C.\n    This option is only supported for regional AuthzExtension resources. Possible values: [\"WIRE_FORMAT_UNSPECIFIED\", \"EXT_PROC_GRPC\", \"EXT_AUTHZ_GRPC\"]"]
    pub fn wire_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wire_format", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesAuthzExtensionTimeoutsElRef {
        NetworkServicesAuthzExtensionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetworkServicesAuthzExtension {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkServicesAuthzExtension {}
impl ToListMappable for NetworkServicesAuthzExtension {
    type O = ListRef<NetworkServicesAuthzExtensionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkServicesAuthzExtension_ {
    fn extract_resource_type(&self) -> String {
        "google_network_services_authz_extension".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkServicesAuthzExtension {
    pub tf_id: String,
    #[doc = "The location of the resource."]
    pub location: PrimField<String>,
    #[doc = "Identifier. Name of the AuthzExtension resource."]
    pub name: PrimField<String>,
    #[doc = "The service that runs the extension.\nThe following values and formats are accepted:\n* 'iap.googleapis.com' when the policyProfile is set to REQUEST_AUTHZ\n* 'modelarmor.{{region}}.rep.googleapis.com' when the policyProfile is set to CONTENT_AUTHZ\n* A fully qualified domain name that can be resolved by the dataplane\n* Backend service resource URI of the form 'https://www.googleapis.com/compute/v1/projects/{{project}}/regions/{{region}}/backendServices/{{name}}' or 'https://www.googleapis.com/compute/v1/projects/{{project}}/global/backendServices/{{name}}}}'"]
    pub service: PrimField<String>,
    #[doc = "Specifies the timeout for each individual message on the stream. The timeout must be between 10-10000 milliseconds."]
    pub timeout: PrimField<String>,
}
impl BuildNetworkServicesAuthzExtension {
    pub fn build(self, stack: &mut Stack) -> NetworkServicesAuthzExtension {
        let out = NetworkServicesAuthzExtension(Rc::new(NetworkServicesAuthzExtension_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkServicesAuthzExtensionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                authority: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                fail_open: core::default::Default::default(),
                forward_headers: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                load_balancing_scheme: core::default::Default::default(),
                location: self.location,
                metadata: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                service: self.service,
                timeout: self.timeout,
                wire_format: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkServicesAuthzExtensionRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesAuthzExtensionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkServicesAuthzExtensionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authority` after provisioning.\nThe :authority header in the gRPC request sent from Envoy to the extension service."]
    pub fn authority(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the resource was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fail_open` after provisioning.\nDetermines how the proxy behaves if the call to the extension fails or times out.\nWhen set to TRUE, request or response processing continues without error. Any subsequent extensions in the extension chain are also executed. When set to FALSE or the default setting of FALSE is used, one of the following happens:\n* If response headers have not been delivered to the downstream client, a generic 500 error is returned to the client. The error response can be tailored by configuring a custom error response in the load balancer.\n* If response headers have been delivered, then the HTTP stream to the downstream client is reset."]
    pub fn fail_open(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fail_open", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `forward_headers` after provisioning.\nList of the HTTP headers to forward to the extension (from the client). If omitted, all headers are sent. Each element is a string indicating the header name."]
    pub fn forward_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.forward_headers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of labels associated with the AuthzExtension resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancing_scheme` after provisioning.\nRequired when the service points to a backend service. All backend services and forwarding rules referenced by\nthis extension must share the same load balancing scheme. For more information, refer to\n[Backend services overview](https://cloud.google.com/load-balancing/docs/backend-service). Possible values: [\"INTERNAL_MANAGED\", \"EXTERNAL_MANAGED\"]"]
    pub fn load_balancing_scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancing_scheme", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nThe metadata provided here is included as part of the metadata_context (of type google.protobuf.Struct) in the ProcessingRequest message sent to the extension server. The metadata is available under the namespace com.google.authz_extension.<resourceName>. The following variables are supported in the metadata Struct:\n\n{forwarding_rule_id} - substituted with the forwarding rule's fully qualified resource name."]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Name of the AuthzExtension resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nThe service that runs the extension.\nThe following values and formats are accepted:\n* 'iap.googleapis.com' when the policyProfile is set to REQUEST_AUTHZ\n* 'modelarmor.{{region}}.rep.googleapis.com' when the policyProfile is set to CONTENT_AUTHZ\n* A fully qualified domain name that can be resolved by the dataplane\n* Backend service resource URI of the form 'https://www.googleapis.com/compute/v1/projects/{{project}}/regions/{{region}}/backendServices/{{name}}' or 'https://www.googleapis.com/compute/v1/projects/{{project}}/global/backendServices/{{name}}}}'"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\nSpecifies the timeout for each individual message on the stream. The timeout must be between 10-10000 milliseconds."]
    pub fn timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the resource was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `wire_format` after provisioning.\nThe format of communication supported by the callout extension. Applicable only when the policyProfile is REQUEST_AUTHZ.\nThis field is supported only for regional AuthzExtension resources. If not specified, the default value\nEXT_PROC_GRPC is used. Global AuthzExtension resources use the EXT_PROC_GRPC wire format.\n\nSupported values:\n- WIRE_FORMAT_UNSPECIFIED:\n    No wire format is explicitly specified. The backend automatically\n    defaults this value to EXT_PROC_GRPC.\n- EXT_PROC_GRPC:\n    Uses Envoy's External Processing (ext_proc) gRPC API over a single\n    gRPC stream. The backend service must support HTTP/2 or H2C.\n    All supported events for a client request are sent over the same\n    gRPC stream. This is the default wire format.\n- EXT_AUTHZ_GRPC:\n    Uses Envoy's external authorization (ext_authz) gRPC API.\n    The backend service must support HTTP/2 or H2C.\n    This option is only supported for regional AuthzExtension resources. Possible values: [\"WIRE_FORMAT_UNSPECIFIED\", \"EXT_PROC_GRPC\", \"EXT_AUTHZ_GRPC\"]"]
    pub fn wire_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wire_format", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesAuthzExtensionTimeoutsElRef {
        NetworkServicesAuthzExtensionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesAuthzExtensionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkServicesAuthzExtensionTimeoutsEl {
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
impl ToListMappable for NetworkServicesAuthzExtensionTimeoutsEl {
    type O = BlockAssignable<NetworkServicesAuthzExtensionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesAuthzExtensionTimeoutsEl {}
impl BuildNetworkServicesAuthzExtensionTimeoutsEl {
    pub fn build(self) -> NetworkServicesAuthzExtensionTimeoutsEl {
        NetworkServicesAuthzExtensionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesAuthzExtensionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesAuthzExtensionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesAuthzExtensionTimeoutsElRef {
        NetworkServicesAuthzExtensionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesAuthzExtensionTimeoutsElRef {
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
