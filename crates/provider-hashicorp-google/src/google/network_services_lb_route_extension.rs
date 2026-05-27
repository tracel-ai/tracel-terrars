use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkServicesLbRouteExtensionData {
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
    description: Option<PrimField<String>>,
    forwarding_rules: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    load_balancing_scheme: PrimField<String>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extension_chains: Option<Vec<NetworkServicesLbRouteExtensionExtensionChainsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkServicesLbRouteExtensionTimeoutsEl>,
    dynamic: NetworkServicesLbRouteExtensionDynamic,
}
struct NetworkServicesLbRouteExtension_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkServicesLbRouteExtensionData>,
}
#[derive(Clone)]
pub struct NetworkServicesLbRouteExtension(Rc<NetworkServicesLbRouteExtension_>);
impl NetworkServicesLbRouteExtension {
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
    #[doc = "Set the field `description`.\nA human-readable description of the resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nSet of labels associated with the LbRouteExtension resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `extension_chains`.\n"]
    pub fn set_extension_chains(
        self,
        v: impl Into<BlockAssignable<NetworkServicesLbRouteExtensionExtensionChainsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().extension_chains = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.extension_chains = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkServicesLbRouteExtensionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
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
    #[doc = "Get a reference to the value of field `forwarding_rules` after provisioning.\nA list of references to the forwarding rules to which this service extension is attached to.\nAt least one forwarding rule is required. There can be only one LbRouteExtension resource per forwarding rule."]
    pub fn forwarding_rules(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.forwarding_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of labels associated with the LbRouteExtension resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancing_scheme` after provisioning.\nAll backend services and forwarding rules referenced by this extension must share the same load balancing scheme.\nFor more information, refer to [Choosing a load balancer](https://cloud.google.com/load-balancing/docs/backend-service) and\n[Supported application load balancers](https://cloud.google.com/service-extensions/docs/callouts-overview#supported-lbs). Possible values: [\"INTERNAL_MANAGED\", \"EXTERNAL_MANAGED\"]"]
    pub fn load_balancing_scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancing_scheme", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the route extension"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the LbRouteExtension resource in the following format: projects/{project}/locations/{location}/lbRouteExtensions/{lbRouteExtension}"]
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
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `extension_chains` after provisioning.\n"]
    pub fn extension_chains(&self) -> ListRef<NetworkServicesLbRouteExtensionExtensionChainsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.extension_chains", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesLbRouteExtensionTimeoutsElRef {
        NetworkServicesLbRouteExtensionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetworkServicesLbRouteExtension {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkServicesLbRouteExtension {}
impl ToListMappable for NetworkServicesLbRouteExtension {
    type O = ListRef<NetworkServicesLbRouteExtensionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkServicesLbRouteExtension_ {
    fn extract_resource_type(&self) -> String {
        "google_network_services_lb_route_extension".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkServicesLbRouteExtension {
    pub tf_id: String,
    #[doc = "A list of references to the forwarding rules to which this service extension is attached to.\nAt least one forwarding rule is required. There can be only one LbRouteExtension resource per forwarding rule."]
    pub forwarding_rules: ListField<PrimField<String>>,
    #[doc = "All backend services and forwarding rules referenced by this extension must share the same load balancing scheme.\nFor more information, refer to [Choosing a load balancer](https://cloud.google.com/load-balancing/docs/backend-service) and\n[Supported application load balancers](https://cloud.google.com/service-extensions/docs/callouts-overview#supported-lbs). Possible values: [\"INTERNAL_MANAGED\", \"EXTERNAL_MANAGED\"]"]
    pub load_balancing_scheme: PrimField<String>,
    #[doc = "The location of the route extension"]
    pub location: PrimField<String>,
    #[doc = "Name of the LbRouteExtension resource in the following format: projects/{project}/locations/{location}/lbRouteExtensions/{lbRouteExtension}"]
    pub name: PrimField<String>,
}
impl BuildNetworkServicesLbRouteExtension {
    pub fn build(self, stack: &mut Stack) -> NetworkServicesLbRouteExtension {
        let out = NetworkServicesLbRouteExtension(Rc::new(NetworkServicesLbRouteExtension_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkServicesLbRouteExtensionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                forwarding_rules: self.forwarding_rules,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                load_balancing_scheme: self.load_balancing_scheme,
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                extension_chains: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkServicesLbRouteExtensionRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesLbRouteExtensionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkServicesLbRouteExtensionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `forwarding_rules` after provisioning.\nA list of references to the forwarding rules to which this service extension is attached to.\nAt least one forwarding rule is required. There can be only one LbRouteExtension resource per forwarding rule."]
    pub fn forwarding_rules(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.forwarding_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of labels associated with the LbRouteExtension resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancing_scheme` after provisioning.\nAll backend services and forwarding rules referenced by this extension must share the same load balancing scheme.\nFor more information, refer to [Choosing a load balancer](https://cloud.google.com/load-balancing/docs/backend-service) and\n[Supported application load balancers](https://cloud.google.com/service-extensions/docs/callouts-overview#supported-lbs). Possible values: [\"INTERNAL_MANAGED\", \"EXTERNAL_MANAGED\"]"]
    pub fn load_balancing_scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancing_scheme", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the route extension"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the LbRouteExtension resource in the following format: projects/{project}/locations/{location}/lbRouteExtensions/{lbRouteExtension}"]
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
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `extension_chains` after provisioning.\n"]
    pub fn extension_chains(&self) -> ListRef<NetworkServicesLbRouteExtensionExtensionChainsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.extension_chains", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesLbRouteExtensionTimeoutsElRef {
        NetworkServicesLbRouteExtensionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesLbRouteExtensionExtensionChainsElExtensionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authority: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fail_open: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forward_headers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RecField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    observability_mode: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_body_send_mode: Option<PrimField<String>>,
    service: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supported_events: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<PrimField<String>>,
}
impl NetworkServicesLbRouteExtensionExtensionChainsElExtensionsEl {
    #[doc = "Set the field `authority`.\nThe :authority header in the gRPC request sent from Envoy to the extension service."]
    pub fn set_authority(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.authority = Some(v.into());
        self
    }
    #[doc = "Set the field `fail_open`.\nDetermines how the proxy behaves if the call to the extension fails or times out.\nWhen set to TRUE, request or response processing continues without error.\nAny subsequent extensions in the extension chain are also executed.\nWhen set to FALSE: * If response headers have not been delivered to the downstream client,\na generic 500 error is returned to the client. The error response can be tailored by\nconfiguring a custom error response in the load balancer."]
    pub fn set_fail_open(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.fail_open = Some(v.into());
        self
    }
    #[doc = "Set the field `forward_headers`.\nList of the HTTP headers to forward to the extension (from the client or backend).\nIf omitted, all headers are sent. Each element is a string indicating the header name."]
    pub fn set_forward_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.forward_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata`.\nThe metadata provided here is included as part of the 'metadata_context' (of type 'google.protobuf.Struct')\nin the 'ProcessingRequest' message sent to the extension server.\nThe metadata is available under the namespace 'com.google.lb_route_extension.<resource_name>.<chain_name>.<extension_name>'.\nThe following variables are supported in the metadata: '{forwarding_rule_id}' - substituted with the forwarding rule's fully qualified resource name.\nThis field must not be set for plugin extensions. Setting it results in a validation error."]
    pub fn set_metadata(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.metadata = Some(v.into());
        self
    }
    #[doc = "Set the field `observability_mode`.\nWhen set to 'TRUE', enables 'observability_mode' on the 'ext_proc' filter.\nThis makes 'ext_proc' calls asynchronous. Envoy doesn't check for the response from 'ext_proc' calls.\nFor more information about the filter, see: https://www.envoyproxy.io/docs/envoy/v1.32.3/api-v3/extensions/filters/http/ext_proc/v3/ext_proc.proto\nThis field is helpful when you want to try out the extension in async log-only mode.\nSupported by regional 'LbTrafficExtension' and 'LbRouteExtension' resources.\nOnly 'STREAMED' (default) body processing mode is supported."]
    pub fn set_observability_mode(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.observability_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `request_body_send_mode`.\nConfigures the send mode for request body processing.\nThe field can only be set if 'supported_events' includes 'REQUEST_BODY'.\nIf 'supported_events' includes 'REQUEST_BODY', but 'request_body_send_mode' is unset, the default value 'STREAMED' is used.\nWhen this field is set to 'FULL_DUPLEX_STREAMED', 'supported_events' must include both 'REQUEST_BODY' and 'REQUEST_TRAILERS'.\nThis field can be set only when the 'service' field of the extension points to a 'BackendService'.\nOnly 'FULL_DUPLEX_STREAMED' mode is supported for 'LbRouteExtension' resources. Possible values: [\"BODY_SEND_MODE_UNSPECIFIED\", \"BODY_SEND_MODE_STREAMED\", \"BODY_SEND_MODE_FULL_DUPLEX_STREAMED\"]"]
    pub fn set_request_body_send_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.request_body_send_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `supported_events`.\nA set of events during request or response processing for which this extension is called.\nThis field is optional for the LbRouteExtension resource. If unspecified, 'REQUEST_HEADERS' event is assumed as supported.\nPossible values: 'REQUEST_HEADERS', 'REQUEST_BODY', 'REQUEST_TRAILERS'."]
    pub fn set_supported_events(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.supported_events = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout`.\nSpecifies the timeout for each individual message on the stream. The timeout must be between 10-1000 milliseconds.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn set_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timeout = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesLbRouteExtensionExtensionChainsElExtensionsEl {
    type O = BlockAssignable<NetworkServicesLbRouteExtensionExtensionChainsElExtensionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesLbRouteExtensionExtensionChainsElExtensionsEl {
    #[doc = "The name for this extension. The name is logged as part of the HTTP request logs.\nThe name must conform with RFC-1034, is restricted to lower-cased letters, numbers and hyphens,\nand can have a maximum length of 63 characters. Additionally, the first character must be a letter\nand the last a letter or a number."]
    pub name: PrimField<String>,
    #[doc = "The reference to the service that runs the extension.\n\n* To configure a callout extension, service must be a fully-qualified reference to a backend service.\n* To configure a plugin extension, service must be a reference to a WasmPlugin resource."]
    pub service: PrimField<String>,
}
impl BuildNetworkServicesLbRouteExtensionExtensionChainsElExtensionsEl {
    pub fn build(self) -> NetworkServicesLbRouteExtensionExtensionChainsElExtensionsEl {
        NetworkServicesLbRouteExtensionExtensionChainsElExtensionsEl {
            authority: core::default::Default::default(),
            fail_open: core::default::Default::default(),
            forward_headers: core::default::Default::default(),
            metadata: core::default::Default::default(),
            name: self.name,
            observability_mode: core::default::Default::default(),
            request_body_send_mode: core::default::Default::default(),
            service: self.service,
            supported_events: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesLbRouteExtensionExtensionChainsElExtensionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesLbRouteExtensionExtensionChainsElExtensionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesLbRouteExtensionExtensionChainsElExtensionsElRef {
        NetworkServicesLbRouteExtensionExtensionChainsElExtensionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesLbRouteExtensionExtensionChainsElExtensionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authority` after provisioning.\nThe :authority header in the gRPC request sent from Envoy to the extension service."]
    pub fn authority(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authority", self.base))
    }
    #[doc = "Get a reference to the value of field `fail_open` after provisioning.\nDetermines how the proxy behaves if the call to the extension fails or times out.\nWhen set to TRUE, request or response processing continues without error.\nAny subsequent extensions in the extension chain are also executed.\nWhen set to FALSE: * If response headers have not been delivered to the downstream client,\na generic 500 error is returned to the client. The error response can be tailored by\nconfiguring a custom error response in the load balancer."]
    pub fn fail_open(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fail_open", self.base))
    }
    #[doc = "Get a reference to the value of field `forward_headers` after provisioning.\nList of the HTTP headers to forward to the extension (from the client or backend).\nIf omitted, all headers are sent. Each element is a string indicating the header name."]
    pub fn forward_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.forward_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nThe metadata provided here is included as part of the 'metadata_context' (of type 'google.protobuf.Struct')\nin the 'ProcessingRequest' message sent to the extension server.\nThe metadata is available under the namespace 'com.google.lb_route_extension.<resource_name>.<chain_name>.<extension_name>'.\nThe following variables are supported in the metadata: '{forwarding_rule_id}' - substituted with the forwarding rule's fully qualified resource name.\nThis field must not be set for plugin extensions. Setting it results in a validation error."]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name for this extension. The name is logged as part of the HTTP request logs.\nThe name must conform with RFC-1034, is restricted to lower-cased letters, numbers and hyphens,\nand can have a maximum length of 63 characters. Additionally, the first character must be a letter\nand the last a letter or a number."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `observability_mode` after provisioning.\nWhen set to 'TRUE', enables 'observability_mode' on the 'ext_proc' filter.\nThis makes 'ext_proc' calls asynchronous. Envoy doesn't check for the response from 'ext_proc' calls.\nFor more information about the filter, see: https://www.envoyproxy.io/docs/envoy/v1.32.3/api-v3/extensions/filters/http/ext_proc/v3/ext_proc.proto\nThis field is helpful when you want to try out the extension in async log-only mode.\nSupported by regional 'LbTrafficExtension' and 'LbRouteExtension' resources.\nOnly 'STREAMED' (default) body processing mode is supported."]
    pub fn observability_mode(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.observability_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_body_send_mode` after provisioning.\nConfigures the send mode for request body processing.\nThe field can only be set if 'supported_events' includes 'REQUEST_BODY'.\nIf 'supported_events' includes 'REQUEST_BODY', but 'request_body_send_mode' is unset, the default value 'STREAMED' is used.\nWhen this field is set to 'FULL_DUPLEX_STREAMED', 'supported_events' must include both 'REQUEST_BODY' and 'REQUEST_TRAILERS'.\nThis field can be set only when the 'service' field of the extension points to a 'BackendService'.\nOnly 'FULL_DUPLEX_STREAMED' mode is supported for 'LbRouteExtension' resources. Possible values: [\"BODY_SEND_MODE_UNSPECIFIED\", \"BODY_SEND_MODE_STREAMED\", \"BODY_SEND_MODE_FULL_DUPLEX_STREAMED\"]"]
    pub fn request_body_send_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_body_send_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nThe reference to the service that runs the extension.\n\n* To configure a callout extension, service must be a fully-qualified reference to a backend service.\n* To configure a plugin extension, service must be a reference to a WasmPlugin resource."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
    #[doc = "Get a reference to the value of field `supported_events` after provisioning.\nA set of events during request or response processing for which this extension is called.\nThis field is optional for the LbRouteExtension resource. If unspecified, 'REQUEST_HEADERS' event is assumed as supported.\nPossible values: 'REQUEST_HEADERS', 'REQUEST_BODY', 'REQUEST_TRAILERS'."]
    pub fn supported_events(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.supported_events", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\nSpecifies the timeout for each individual message on the stream. The timeout must be between 10-1000 milliseconds.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesLbRouteExtensionExtensionChainsElMatchConditionEl {
    cel_expression: PrimField<String>,
}
impl NetworkServicesLbRouteExtensionExtensionChainsElMatchConditionEl {}
impl ToListMappable for NetworkServicesLbRouteExtensionExtensionChainsElMatchConditionEl {
    type O = BlockAssignable<NetworkServicesLbRouteExtensionExtensionChainsElMatchConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesLbRouteExtensionExtensionChainsElMatchConditionEl {
    #[doc = "A Common Expression Language (CEL) expression that is used to match requests for which the extension chain is executed."]
    pub cel_expression: PrimField<String>,
}
impl BuildNetworkServicesLbRouteExtensionExtensionChainsElMatchConditionEl {
    pub fn build(self) -> NetworkServicesLbRouteExtensionExtensionChainsElMatchConditionEl {
        NetworkServicesLbRouteExtensionExtensionChainsElMatchConditionEl {
            cel_expression: self.cel_expression,
        }
    }
}
pub struct NetworkServicesLbRouteExtensionExtensionChainsElMatchConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesLbRouteExtensionExtensionChainsElMatchConditionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesLbRouteExtensionExtensionChainsElMatchConditionElRef {
        NetworkServicesLbRouteExtensionExtensionChainsElMatchConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesLbRouteExtensionExtensionChainsElMatchConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cel_expression` after provisioning.\nA Common Expression Language (CEL) expression that is used to match requests for which the extension chain is executed."]
    pub fn cel_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cel_expression", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct NetworkServicesLbRouteExtensionExtensionChainsElDynamic {
    extensions: Option<DynamicBlock<NetworkServicesLbRouteExtensionExtensionChainsElExtensionsEl>>,
    match_condition:
        Option<DynamicBlock<NetworkServicesLbRouteExtensionExtensionChainsElMatchConditionEl>>,
}
#[derive(Serialize)]
pub struct NetworkServicesLbRouteExtensionExtensionChainsEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extensions: Option<Vec<NetworkServicesLbRouteExtensionExtensionChainsElExtensionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_condition: Option<Vec<NetworkServicesLbRouteExtensionExtensionChainsElMatchConditionEl>>,
    dynamic: NetworkServicesLbRouteExtensionExtensionChainsElDynamic,
}
impl NetworkServicesLbRouteExtensionExtensionChainsEl {
    #[doc = "Set the field `extensions`.\n"]
    pub fn set_extensions(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesLbRouteExtensionExtensionChainsElExtensionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.extensions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.extensions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `match_condition`.\n"]
    pub fn set_match_condition(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesLbRouteExtensionExtensionChainsElMatchConditionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.match_condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.match_condition = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkServicesLbRouteExtensionExtensionChainsEl {
    type O = BlockAssignable<NetworkServicesLbRouteExtensionExtensionChainsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesLbRouteExtensionExtensionChainsEl {
    #[doc = "The name for this extension chain. The name is logged as part of the HTTP request logs.\nThe name must conform with RFC-1034, is restricted to lower-cased letters, numbers and hyphens,\nand can have a maximum length of 63 characters. Additionally, the first character must be a letter\nand the last character must be a letter or a number."]
    pub name: PrimField<String>,
}
impl BuildNetworkServicesLbRouteExtensionExtensionChainsEl {
    pub fn build(self) -> NetworkServicesLbRouteExtensionExtensionChainsEl {
        NetworkServicesLbRouteExtensionExtensionChainsEl {
            name: self.name,
            extensions: core::default::Default::default(),
            match_condition: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkServicesLbRouteExtensionExtensionChainsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesLbRouteExtensionExtensionChainsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesLbRouteExtensionExtensionChainsElRef {
        NetworkServicesLbRouteExtensionExtensionChainsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesLbRouteExtensionExtensionChainsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name for this extension chain. The name is logged as part of the HTTP request logs.\nThe name must conform with RFC-1034, is restricted to lower-cased letters, numbers and hyphens,\nand can have a maximum length of 63 characters. Additionally, the first character must be a letter\nand the last character must be a letter or a number."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `extensions` after provisioning.\n"]
    pub fn extensions(
        &self,
    ) -> ListRef<NetworkServicesLbRouteExtensionExtensionChainsElExtensionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.extensions", self.base))
    }
    #[doc = "Get a reference to the value of field `match_condition` after provisioning.\n"]
    pub fn match_condition(
        &self,
    ) -> ListRef<NetworkServicesLbRouteExtensionExtensionChainsElMatchConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.match_condition", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesLbRouteExtensionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkServicesLbRouteExtensionTimeoutsEl {
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
impl ToListMappable for NetworkServicesLbRouteExtensionTimeoutsEl {
    type O = BlockAssignable<NetworkServicesLbRouteExtensionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesLbRouteExtensionTimeoutsEl {}
impl BuildNetworkServicesLbRouteExtensionTimeoutsEl {
    pub fn build(self) -> NetworkServicesLbRouteExtensionTimeoutsEl {
        NetworkServicesLbRouteExtensionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesLbRouteExtensionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesLbRouteExtensionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesLbRouteExtensionTimeoutsElRef {
        NetworkServicesLbRouteExtensionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesLbRouteExtensionTimeoutsElRef {
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
#[derive(Serialize, Default)]
struct NetworkServicesLbRouteExtensionDynamic {
    extension_chains: Option<DynamicBlock<NetworkServicesLbRouteExtensionExtensionChainsEl>>,
}
