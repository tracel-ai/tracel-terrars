use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkServicesLbEdgeExtensionData {
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
    extension_chains: Option<Vec<NetworkServicesLbEdgeExtensionExtensionChainsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkServicesLbEdgeExtensionTimeoutsEl>,
    dynamic: NetworkServicesLbEdgeExtensionDynamic,
}
struct NetworkServicesLbEdgeExtension_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkServicesLbEdgeExtensionData>,
}
#[derive(Clone)]
pub struct NetworkServicesLbEdgeExtension(Rc<NetworkServicesLbEdgeExtension_>);
impl NetworkServicesLbEdgeExtension {
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
    #[doc = "Set the field `labels`.\nSet of labels associated with the LbEdgeExtension resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
        v: impl Into<BlockAssignable<NetworkServicesLbEdgeExtensionExtensionChainsEl>>,
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
    pub fn set_timeouts(self, v: impl Into<NetworkServicesLbEdgeExtensionTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `forwarding_rules` after provisioning.\nA list of references to the forwarding rules to which this service extension is attached.\nAt least one forwarding rule is required. Only one LbEdgeExtension resource can be associated with a forwarding rule."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of labels associated with the LbEdgeExtension resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancing_scheme` after provisioning.\nAll forwarding rules referenced by this extension must share the same load balancing scheme. Possible values: [\"EXTERNAL_MANAGED\"]"]
    pub fn load_balancing_scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancing_scheme", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the edge extension"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the LbEdgeExtension resource in the following format: projects/{project}/locations/{location}/lbEdgeExtensions/{lbEdgeExtensions}"]
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
    pub fn extension_chains(&self) -> ListRef<NetworkServicesLbEdgeExtensionExtensionChainsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.extension_chains", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesLbEdgeExtensionTimeoutsElRef {
        NetworkServicesLbEdgeExtensionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetworkServicesLbEdgeExtension {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkServicesLbEdgeExtension {}
impl ToListMappable for NetworkServicesLbEdgeExtension {
    type O = ListRef<NetworkServicesLbEdgeExtensionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkServicesLbEdgeExtension_ {
    fn extract_resource_type(&self) -> String {
        "google_network_services_lb_edge_extension".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkServicesLbEdgeExtension {
    pub tf_id: String,
    #[doc = "A list of references to the forwarding rules to which this service extension is attached.\nAt least one forwarding rule is required. Only one LbEdgeExtension resource can be associated with a forwarding rule."]
    pub forwarding_rules: ListField<PrimField<String>>,
    #[doc = "All forwarding rules referenced by this extension must share the same load balancing scheme. Possible values: [\"EXTERNAL_MANAGED\"]"]
    pub load_balancing_scheme: PrimField<String>,
    #[doc = "The location of the edge extension"]
    pub location: PrimField<String>,
    #[doc = "Name of the LbEdgeExtension resource in the following format: projects/{project}/locations/{location}/lbEdgeExtensions/{lbEdgeExtensions}"]
    pub name: PrimField<String>,
}
impl BuildNetworkServicesLbEdgeExtension {
    pub fn build(self, stack: &mut Stack) -> NetworkServicesLbEdgeExtension {
        let out = NetworkServicesLbEdgeExtension(Rc::new(NetworkServicesLbEdgeExtension_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkServicesLbEdgeExtensionData {
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
pub struct NetworkServicesLbEdgeExtensionRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesLbEdgeExtensionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkServicesLbEdgeExtensionRef {
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
    #[doc = "Get a reference to the value of field `forwarding_rules` after provisioning.\nA list of references to the forwarding rules to which this service extension is attached.\nAt least one forwarding rule is required. Only one LbEdgeExtension resource can be associated with a forwarding rule."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of labels associated with the LbEdgeExtension resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancing_scheme` after provisioning.\nAll forwarding rules referenced by this extension must share the same load balancing scheme. Possible values: [\"EXTERNAL_MANAGED\"]"]
    pub fn load_balancing_scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancing_scheme", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the edge extension"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the LbEdgeExtension resource in the following format: projects/{project}/locations/{location}/lbEdgeExtensions/{lbEdgeExtensions}"]
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
    pub fn extension_chains(&self) -> ListRef<NetworkServicesLbEdgeExtensionExtensionChainsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.extension_chains", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesLbEdgeExtensionTimeoutsElRef {
        NetworkServicesLbEdgeExtensionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesLbEdgeExtensionExtensionChainsElExtensionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fail_open: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forward_headers: Option<ListField<PrimField<String>>>,
    name: PrimField<String>,
    service: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supported_events: Option<ListField<PrimField<String>>>,
}
impl NetworkServicesLbEdgeExtensionExtensionChainsElExtensionsEl {
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
    #[doc = "Set the field `supported_events`.\nA set of events during request or response processing for which this extension is called.\nThis field is required for the LbEdgeExtension resource and only supports the value 'REQUEST_HEADERS'."]
    pub fn set_supported_events(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.supported_events = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesLbEdgeExtensionExtensionChainsElExtensionsEl {
    type O = BlockAssignable<NetworkServicesLbEdgeExtensionExtensionChainsElExtensionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesLbEdgeExtensionExtensionChainsElExtensionsEl {
    #[doc = "The name for this extension. The name is logged as part of the HTTP request logs.\nThe name must conform with RFC-1034, is restricted to lower-cased letters, numbers and hyphens,\nand can have a maximum length of 63 characters. Additionally, the first character must be a letter\nand the last a letter or a number."]
    pub name: PrimField<String>,
    #[doc = "The reference to the service that runs the extension.\n\n* To configure a callout extension, service must be a fully-qualified reference to a backend service.\n* To configure a plugin extension, service must be a reference to a WasmPlugin resource."]
    pub service: PrimField<String>,
}
impl BuildNetworkServicesLbEdgeExtensionExtensionChainsElExtensionsEl {
    pub fn build(self) -> NetworkServicesLbEdgeExtensionExtensionChainsElExtensionsEl {
        NetworkServicesLbEdgeExtensionExtensionChainsElExtensionsEl {
            fail_open: core::default::Default::default(),
            forward_headers: core::default::Default::default(),
            name: self.name,
            service: self.service,
            supported_events: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesLbEdgeExtensionExtensionChainsElExtensionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesLbEdgeExtensionExtensionChainsElExtensionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesLbEdgeExtensionExtensionChainsElExtensionsElRef {
        NetworkServicesLbEdgeExtensionExtensionChainsElExtensionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesLbEdgeExtensionExtensionChainsElExtensionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name for this extension. The name is logged as part of the HTTP request logs.\nThe name must conform with RFC-1034, is restricted to lower-cased letters, numbers and hyphens,\nand can have a maximum length of 63 characters. Additionally, the first character must be a letter\nand the last a letter or a number."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nThe reference to the service that runs the extension.\n\n* To configure a callout extension, service must be a fully-qualified reference to a backend service.\n* To configure a plugin extension, service must be a reference to a WasmPlugin resource."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
    #[doc = "Get a reference to the value of field `supported_events` after provisioning.\nA set of events during request or response processing for which this extension is called.\nThis field is required for the LbEdgeExtension resource and only supports the value 'REQUEST_HEADERS'."]
    pub fn supported_events(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_events", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesLbEdgeExtensionExtensionChainsElMatchConditionEl {
    cel_expression: PrimField<String>,
}
impl NetworkServicesLbEdgeExtensionExtensionChainsElMatchConditionEl {}
impl ToListMappable for NetworkServicesLbEdgeExtensionExtensionChainsElMatchConditionEl {
    type O = BlockAssignable<NetworkServicesLbEdgeExtensionExtensionChainsElMatchConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesLbEdgeExtensionExtensionChainsElMatchConditionEl {
    #[doc = "A Common Expression Language (CEL) expression that is used to match requests for which the extension chain is executed."]
    pub cel_expression: PrimField<String>,
}
impl BuildNetworkServicesLbEdgeExtensionExtensionChainsElMatchConditionEl {
    pub fn build(self) -> NetworkServicesLbEdgeExtensionExtensionChainsElMatchConditionEl {
        NetworkServicesLbEdgeExtensionExtensionChainsElMatchConditionEl {
            cel_expression: self.cel_expression,
        }
    }
}
pub struct NetworkServicesLbEdgeExtensionExtensionChainsElMatchConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesLbEdgeExtensionExtensionChainsElMatchConditionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesLbEdgeExtensionExtensionChainsElMatchConditionElRef {
        NetworkServicesLbEdgeExtensionExtensionChainsElMatchConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesLbEdgeExtensionExtensionChainsElMatchConditionElRef {
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
struct NetworkServicesLbEdgeExtensionExtensionChainsElDynamic {
    extensions: Option<DynamicBlock<NetworkServicesLbEdgeExtensionExtensionChainsElExtensionsEl>>,
    match_condition:
        Option<DynamicBlock<NetworkServicesLbEdgeExtensionExtensionChainsElMatchConditionEl>>,
}
#[derive(Serialize)]
pub struct NetworkServicesLbEdgeExtensionExtensionChainsEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extensions: Option<Vec<NetworkServicesLbEdgeExtensionExtensionChainsElExtensionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_condition: Option<Vec<NetworkServicesLbEdgeExtensionExtensionChainsElMatchConditionEl>>,
    dynamic: NetworkServicesLbEdgeExtensionExtensionChainsElDynamic,
}
impl NetworkServicesLbEdgeExtensionExtensionChainsEl {
    #[doc = "Set the field `extensions`.\n"]
    pub fn set_extensions(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesLbEdgeExtensionExtensionChainsElExtensionsEl>>,
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
        v: impl Into<BlockAssignable<NetworkServicesLbEdgeExtensionExtensionChainsElMatchConditionEl>>,
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
impl ToListMappable for NetworkServicesLbEdgeExtensionExtensionChainsEl {
    type O = BlockAssignable<NetworkServicesLbEdgeExtensionExtensionChainsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesLbEdgeExtensionExtensionChainsEl {
    #[doc = "The name for this extension chain. The name is logged as part of the HTTP request logs.\nThe name must conform with RFC-1034, is restricted to lower-cased letters, numbers and hyphens,\nand can have a maximum length of 63 characters. Additionally, the first character must be a letter\nand the last character must be a letter or a number."]
    pub name: PrimField<String>,
}
impl BuildNetworkServicesLbEdgeExtensionExtensionChainsEl {
    pub fn build(self) -> NetworkServicesLbEdgeExtensionExtensionChainsEl {
        NetworkServicesLbEdgeExtensionExtensionChainsEl {
            name: self.name,
            extensions: core::default::Default::default(),
            match_condition: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkServicesLbEdgeExtensionExtensionChainsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesLbEdgeExtensionExtensionChainsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesLbEdgeExtensionExtensionChainsElRef {
        NetworkServicesLbEdgeExtensionExtensionChainsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesLbEdgeExtensionExtensionChainsElRef {
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
    ) -> ListRef<NetworkServicesLbEdgeExtensionExtensionChainsElExtensionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.extensions", self.base))
    }
    #[doc = "Get a reference to the value of field `match_condition` after provisioning.\n"]
    pub fn match_condition(
        &self,
    ) -> ListRef<NetworkServicesLbEdgeExtensionExtensionChainsElMatchConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.match_condition", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesLbEdgeExtensionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkServicesLbEdgeExtensionTimeoutsEl {
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
impl ToListMappable for NetworkServicesLbEdgeExtensionTimeoutsEl {
    type O = BlockAssignable<NetworkServicesLbEdgeExtensionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesLbEdgeExtensionTimeoutsEl {}
impl BuildNetworkServicesLbEdgeExtensionTimeoutsEl {
    pub fn build(self) -> NetworkServicesLbEdgeExtensionTimeoutsEl {
        NetworkServicesLbEdgeExtensionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesLbEdgeExtensionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesLbEdgeExtensionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesLbEdgeExtensionTimeoutsElRef {
        NetworkServicesLbEdgeExtensionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesLbEdgeExtensionTimeoutsElRef {
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
struct NetworkServicesLbEdgeExtensionDynamic {
    extension_chains: Option<DynamicBlock<NetworkServicesLbEdgeExtensionExtensionChainsEl>>,
}
