use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkServicesEndpointPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_tls_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_tls_policy: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_matcher: Option<Vec<NetworkServicesEndpointPolicyEndpointMatcherEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkServicesEndpointPolicyTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    traffic_port_selector: Option<Vec<NetworkServicesEndpointPolicyTrafficPortSelectorEl>>,
    dynamic: NetworkServicesEndpointPolicyDynamic,
}
struct NetworkServicesEndpointPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkServicesEndpointPolicyData>,
}
#[derive(Clone)]
pub struct NetworkServicesEndpointPolicy(Rc<NetworkServicesEndpointPolicy_>);
impl NetworkServicesEndpointPolicy {
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
    #[doc = "Set the field `authorization_policy`.\nThis field specifies the URL of AuthorizationPolicy resource that applies authorization policies to the inbound traffic at the matched endpoints."]
    pub fn set_authorization_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().authorization_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `client_tls_policy`.\nA URL referring to a ClientTlsPolicy resource. ClientTlsPolicy can be set to specify the authentication for traffic from the proxy to the actual endpoints."]
    pub fn set_client_tls_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().client_tls_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nA free-text description of the resource. Max length 1024 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nSet of label tags associated with the TcpRoute resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `server_tls_policy`.\nA URL referring to ServerTlsPolicy resource. ServerTlsPolicy is used to determine the authentication policy to be applied to terminate the inbound traffic at the identified backends."]
    pub fn set_server_tls_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().server_tls_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `endpoint_matcher`.\n"]
    pub fn set_endpoint_matcher(
        self,
        v: impl Into<BlockAssignable<NetworkServicesEndpointPolicyEndpointMatcherEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint_matcher = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint_matcher = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkServicesEndpointPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `traffic_port_selector`.\n"]
    pub fn set_traffic_port_selector(
        self,
        v: impl Into<BlockAssignable<NetworkServicesEndpointPolicyTrafficPortSelectorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().traffic_port_selector = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.traffic_port_selector = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `authorization_policy` after provisioning.\nThis field specifies the URL of AuthorizationPolicy resource that applies authorization policies to the inbound traffic at the matched endpoints."]
    pub fn authorization_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorization_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_tls_policy` after provisioning.\nA URL referring to a ClientTlsPolicy resource. ClientTlsPolicy can be set to specify the authentication for traffic from the proxy to the actual endpoints."]
    pub fn client_tls_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_tls_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the TcpRoute was created in UTC."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA free-text description of the resource. Max length 1024 characters."]
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the TcpRoute resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the EndpointPolicy resource."]
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
    #[doc = "Get a reference to the value of field `server_tls_policy` after provisioning.\nA URL referring to ServerTlsPolicy resource. ServerTlsPolicy is used to determine the authentication policy to be applied to terminate the inbound traffic at the identified backends."]
    pub fn server_tls_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_tls_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of endpoint policy. This is primarily used to validate the configuration. Possible values: [\"SIDECAR_PROXY\", \"GRPC_SERVER\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the TcpRoute was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_matcher` after provisioning.\n"]
    pub fn endpoint_matcher(&self) -> ListRef<NetworkServicesEndpointPolicyEndpointMatcherElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint_matcher", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesEndpointPolicyTimeoutsElRef {
        NetworkServicesEndpointPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `traffic_port_selector` after provisioning.\n"]
    pub fn traffic_port_selector(
        &self,
    ) -> ListRef<NetworkServicesEndpointPolicyTrafficPortSelectorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.traffic_port_selector", self.extract_ref()),
        )
    }
}
impl Referable for NetworkServicesEndpointPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkServicesEndpointPolicy {}
impl ToListMappable for NetworkServicesEndpointPolicy {
    type O = ListRef<NetworkServicesEndpointPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkServicesEndpointPolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_network_services_endpoint_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkServicesEndpointPolicy {
    pub tf_id: String,
    #[doc = "Name of the EndpointPolicy resource."]
    pub name: PrimField<String>,
    #[doc = "The type of endpoint policy. This is primarily used to validate the configuration. Possible values: [\"SIDECAR_PROXY\", \"GRPC_SERVER\"]"]
    pub type_: PrimField<String>,
}
impl BuildNetworkServicesEndpointPolicy {
    pub fn build(self, stack: &mut Stack) -> NetworkServicesEndpointPolicy {
        let out = NetworkServicesEndpointPolicy(Rc::new(NetworkServicesEndpointPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkServicesEndpointPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                authorization_policy: core::default::Default::default(),
                client_tls_policy: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                server_tls_policy: core::default::Default::default(),
                type_: self.type_,
                endpoint_matcher: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                traffic_port_selector: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkServicesEndpointPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEndpointPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkServicesEndpointPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authorization_policy` after provisioning.\nThis field specifies the URL of AuthorizationPolicy resource that applies authorization policies to the inbound traffic at the matched endpoints."]
    pub fn authorization_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorization_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_tls_policy` after provisioning.\nA URL referring to a ClientTlsPolicy resource. ClientTlsPolicy can be set to specify the authentication for traffic from the proxy to the actual endpoints."]
    pub fn client_tls_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_tls_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the TcpRoute was created in UTC."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA free-text description of the resource. Max length 1024 characters."]
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the TcpRoute resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the EndpointPolicy resource."]
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
    #[doc = "Get a reference to the value of field `server_tls_policy` after provisioning.\nA URL referring to ServerTlsPolicy resource. ServerTlsPolicy is used to determine the authentication policy to be applied to terminate the inbound traffic at the identified backends."]
    pub fn server_tls_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_tls_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of endpoint policy. This is primarily used to validate the configuration. Possible values: [\"SIDECAR_PROXY\", \"GRPC_SERVER\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the TcpRoute was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_matcher` after provisioning.\n"]
    pub fn endpoint_matcher(&self) -> ListRef<NetworkServicesEndpointPolicyEndpointMatcherElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint_matcher", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesEndpointPolicyTimeoutsElRef {
        NetworkServicesEndpointPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `traffic_port_selector` after provisioning.\n"]
    pub fn traffic_port_selector(
        &self,
    ) -> ListRef<NetworkServicesEndpointPolicyTrafficPortSelectorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.traffic_port_selector", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherElMetadataLabelsEl {
    label_name: PrimField<String>,
    label_value: PrimField<String>,
}
impl NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherElMetadataLabelsEl {}
impl ToListMappable
    for NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherElMetadataLabelsEl
{
    type O = BlockAssignable<
        NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherElMetadataLabelsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherElMetadataLabelsEl
{
    #[doc = "Required. Label name presented as key in xDS Node Metadata."]
    pub label_name: PrimField<String>,
    #[doc = "Required. Label value presented as value corresponding to the above key, in xDS Node Metadata."]
    pub label_value: PrimField<String>,
}
impl BuildNetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherElMetadataLabelsEl {
    pub fn build(
        self,
    ) -> NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherElMetadataLabelsEl {
        NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherElMetadataLabelsEl {
            label_name: self.label_name,
            label_value: self.label_value,
        }
    }
}
pub struct NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherElMetadataLabelsElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherElMetadataLabelsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherElMetadataLabelsElRef
    {
        NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherElMetadataLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherElMetadataLabelsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `label_name` after provisioning.\nRequired. Label name presented as key in xDS Node Metadata."]
    pub fn label_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label_name", self.base))
    }
    #[doc = "Get a reference to the value of field `label_value` after provisioning.\nRequired. Label value presented as value corresponding to the above key, in xDS Node Metadata."]
    pub fn label_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label_value", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherElDynamic {
    metadata_labels: Option<
        DynamicBlock<
            NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherElMetadataLabelsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherEl {
    metadata_label_match_criteria: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_labels: Option<
        Vec<NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherElMetadataLabelsEl>,
    >,
    dynamic: NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherElDynamic,
}
impl NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherEl {
    #[doc = "Set the field `metadata_labels`.\n"]
    pub fn set_metadata_labels(
        mut self,
        v : impl Into < BlockAssignable < NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherElMetadataLabelsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metadata_labels = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metadata_labels = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherEl {
    type O = BlockAssignable<NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherEl {
    #[doc = "Specifies how matching should be done. Possible values: [\"MATCH_ANY\", \"MATCH_ALL\"]"]
    pub metadata_label_match_criteria: PrimField<String>,
}
impl BuildNetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherEl {
    pub fn build(self) -> NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherEl {
        NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherEl {
            metadata_label_match_criteria: self.metadata_label_match_criteria,
            metadata_labels: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherElRef {
        NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metadata_label_match_criteria` after provisioning.\nSpecifies how matching should be done. Possible values: [\"MATCH_ANY\", \"MATCH_ALL\"]"]
    pub fn metadata_label_match_criteria(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metadata_label_match_criteria", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metadata_labels` after provisioning.\n"]
    pub fn metadata_labels(
        &self,
    ) -> ListRef<
        NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherElMetadataLabelsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata_labels", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct NetworkServicesEndpointPolicyEndpointMatcherElDynamic {
    metadata_label_matcher:
        Option<DynamicBlock<NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherEl>>,
}
#[derive(Serialize)]
pub struct NetworkServicesEndpointPolicyEndpointMatcherEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_label_matcher:
        Option<Vec<NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherEl>>,
    dynamic: NetworkServicesEndpointPolicyEndpointMatcherElDynamic,
}
impl NetworkServicesEndpointPolicyEndpointMatcherEl {
    #[doc = "Set the field `metadata_label_matcher`.\n"]
    pub fn set_metadata_label_matcher(
        mut self,
        v: impl Into<
            BlockAssignable<NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metadata_label_matcher = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metadata_label_matcher = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkServicesEndpointPolicyEndpointMatcherEl {
    type O = BlockAssignable<NetworkServicesEndpointPolicyEndpointMatcherEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEndpointPolicyEndpointMatcherEl {}
impl BuildNetworkServicesEndpointPolicyEndpointMatcherEl {
    pub fn build(self) -> NetworkServicesEndpointPolicyEndpointMatcherEl {
        NetworkServicesEndpointPolicyEndpointMatcherEl {
            metadata_label_matcher: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkServicesEndpointPolicyEndpointMatcherElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEndpointPolicyEndpointMatcherElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesEndpointPolicyEndpointMatcherElRef {
        NetworkServicesEndpointPolicyEndpointMatcherElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEndpointPolicyEndpointMatcherElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metadata_label_matcher` after provisioning.\n"]
    pub fn metadata_label_matcher(
        &self,
    ) -> ListRef<NetworkServicesEndpointPolicyEndpointMatcherElMetadataLabelMatcherElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata_label_matcher", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesEndpointPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkServicesEndpointPolicyTimeoutsEl {
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
impl ToListMappable for NetworkServicesEndpointPolicyTimeoutsEl {
    type O = BlockAssignable<NetworkServicesEndpointPolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEndpointPolicyTimeoutsEl {}
impl BuildNetworkServicesEndpointPolicyTimeoutsEl {
    pub fn build(self) -> NetworkServicesEndpointPolicyTimeoutsEl {
        NetworkServicesEndpointPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesEndpointPolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEndpointPolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesEndpointPolicyTimeoutsElRef {
        NetworkServicesEndpointPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEndpointPolicyTimeoutsElRef {
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
#[derive(Serialize)]
pub struct NetworkServicesEndpointPolicyTrafficPortSelectorEl {
    ports: ListField<PrimField<String>>,
}
impl NetworkServicesEndpointPolicyTrafficPortSelectorEl {}
impl ToListMappable for NetworkServicesEndpointPolicyTrafficPortSelectorEl {
    type O = BlockAssignable<NetworkServicesEndpointPolicyTrafficPortSelectorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEndpointPolicyTrafficPortSelectorEl {
    #[doc = "List of ports. Can be port numbers or port range (example, [80-90] specifies all ports from 80 to 90, including 80 and 90) or named ports or * to specify all ports. If the list is empty, all ports are selected."]
    pub ports: ListField<PrimField<String>>,
}
impl BuildNetworkServicesEndpointPolicyTrafficPortSelectorEl {
    pub fn build(self) -> NetworkServicesEndpointPolicyTrafficPortSelectorEl {
        NetworkServicesEndpointPolicyTrafficPortSelectorEl { ports: self.ports }
    }
}
pub struct NetworkServicesEndpointPolicyTrafficPortSelectorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEndpointPolicyTrafficPortSelectorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesEndpointPolicyTrafficPortSelectorElRef {
        NetworkServicesEndpointPolicyTrafficPortSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEndpointPolicyTrafficPortSelectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ports` after provisioning.\nList of ports. Can be port numbers or port range (example, [80-90] specifies all ports from 80 to 90, including 80 and 90) or named ports or * to specify all ports. If the list is empty, all ports are selected."]
    pub fn ports(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkServicesEndpointPolicyDynamic {
    endpoint_matcher: Option<DynamicBlock<NetworkServicesEndpointPolicyEndpointMatcherEl>>,
    traffic_port_selector: Option<DynamicBlock<NetworkServicesEndpointPolicyTrafficPortSelectorEl>>,
}
