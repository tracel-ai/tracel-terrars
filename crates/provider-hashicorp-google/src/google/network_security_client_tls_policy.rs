use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkSecurityClientTlsPolicyData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sni: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_certificate: Option<Vec<NetworkSecurityClientTlsPolicyClientCertificateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_validation_ca: Option<Vec<NetworkSecurityClientTlsPolicyServerValidationCaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkSecurityClientTlsPolicyTimeoutsEl>,
    dynamic: NetworkSecurityClientTlsPolicyDynamic,
}
struct NetworkSecurityClientTlsPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkSecurityClientTlsPolicyData>,
}
#[derive(Clone)]
pub struct NetworkSecurityClientTlsPolicy(Rc<NetworkSecurityClientTlsPolicy_>);
impl NetworkSecurityClientTlsPolicy {
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
    #[doc = "Set the field `labels`.\nSet of label tags associated with the ClientTlsPolicy resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe location of the client tls policy.\nThe default value is 'global'."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `sni`.\nServer Name Indication string to present to the server during TLS handshake. E.g: \"secure.example.com\"."]
    pub fn set_sni(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sni = Some(v.into());
        self
    }
    #[doc = "Set the field `client_certificate`.\n"]
    pub fn set_client_certificate(
        self,
        v: impl Into<BlockAssignable<NetworkSecurityClientTlsPolicyClientCertificateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client_certificate = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client_certificate = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `server_validation_ca`.\n"]
    pub fn set_server_validation_ca(
        self,
        v: impl Into<BlockAssignable<NetworkSecurityClientTlsPolicyServerValidationCaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().server_validation_ca = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.server_validation_ca = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkSecurityClientTlsPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the ClientTlsPolicy was created in UTC."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the ClientTlsPolicy resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the client tls policy.\nThe default value is 'global'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the ClientTlsPolicy resource."]
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
    #[doc = "Get a reference to the value of field `sni` after provisioning.\nServer Name Indication string to present to the server during TLS handshake. E.g: \"secure.example.com\"."]
    pub fn sni(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sni", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the ClientTlsPolicy was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_certificate` after provisioning.\n"]
    pub fn client_certificate(
        &self,
    ) -> ListRef<NetworkSecurityClientTlsPolicyClientCertificateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_certificate", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_validation_ca` after provisioning.\n"]
    pub fn server_validation_ca(
        &self,
    ) -> ListRef<NetworkSecurityClientTlsPolicyServerValidationCaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.server_validation_ca", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkSecurityClientTlsPolicyTimeoutsElRef {
        NetworkSecurityClientTlsPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetworkSecurityClientTlsPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkSecurityClientTlsPolicy {}
impl ToListMappable for NetworkSecurityClientTlsPolicy {
    type O = ListRef<NetworkSecurityClientTlsPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkSecurityClientTlsPolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_network_security_client_tls_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkSecurityClientTlsPolicy {
    pub tf_id: String,
    #[doc = "Name of the ClientTlsPolicy resource."]
    pub name: PrimField<String>,
}
impl BuildNetworkSecurityClientTlsPolicy {
    pub fn build(self, stack: &mut Stack) -> NetworkSecurityClientTlsPolicy {
        let out = NetworkSecurityClientTlsPolicy(Rc::new(NetworkSecurityClientTlsPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkSecurityClientTlsPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                sni: core::default::Default::default(),
                client_certificate: core::default::Default::default(),
                server_validation_ca: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkSecurityClientTlsPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityClientTlsPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkSecurityClientTlsPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the ClientTlsPolicy was created in UTC."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the ClientTlsPolicy resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the client tls policy.\nThe default value is 'global'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the ClientTlsPolicy resource."]
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
    #[doc = "Get a reference to the value of field `sni` after provisioning.\nServer Name Indication string to present to the server during TLS handshake. E.g: \"secure.example.com\"."]
    pub fn sni(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sni", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the ClientTlsPolicy was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_certificate` after provisioning.\n"]
    pub fn client_certificate(
        &self,
    ) -> ListRef<NetworkSecurityClientTlsPolicyClientCertificateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_certificate", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_validation_ca` after provisioning.\n"]
    pub fn server_validation_ca(
        &self,
    ) -> ListRef<NetworkSecurityClientTlsPolicyServerValidationCaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.server_validation_ca", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkSecurityClientTlsPolicyTimeoutsElRef {
        NetworkSecurityClientTlsPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityClientTlsPolicyClientCertificateElCertificateProviderInstanceEl {
    plugin_instance: PrimField<String>,
}
impl NetworkSecurityClientTlsPolicyClientCertificateElCertificateProviderInstanceEl {}
impl ToListMappable
    for NetworkSecurityClientTlsPolicyClientCertificateElCertificateProviderInstanceEl
{
    type O = BlockAssignable<
        NetworkSecurityClientTlsPolicyClientCertificateElCertificateProviderInstanceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityClientTlsPolicyClientCertificateElCertificateProviderInstanceEl {
    #[doc = "Plugin instance name, used to locate and load CertificateProvider instance configuration. Set to \"google_cloud_private_spiffe\" to use Certificate Authority Service certificate provider instance."]
    pub plugin_instance: PrimField<String>,
}
impl BuildNetworkSecurityClientTlsPolicyClientCertificateElCertificateProviderInstanceEl {
    pub fn build(
        self,
    ) -> NetworkSecurityClientTlsPolicyClientCertificateElCertificateProviderInstanceEl {
        NetworkSecurityClientTlsPolicyClientCertificateElCertificateProviderInstanceEl {
            plugin_instance: self.plugin_instance,
        }
    }
}
pub struct NetworkSecurityClientTlsPolicyClientCertificateElCertificateProviderInstanceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityClientTlsPolicyClientCertificateElCertificateProviderInstanceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityClientTlsPolicyClientCertificateElCertificateProviderInstanceElRef {
        NetworkSecurityClientTlsPolicyClientCertificateElCertificateProviderInstanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityClientTlsPolicyClientCertificateElCertificateProviderInstanceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `plugin_instance` after provisioning.\nPlugin instance name, used to locate and load CertificateProvider instance configuration. Set to \"google_cloud_private_spiffe\" to use Certificate Authority Service certificate provider instance."]
    pub fn plugin_instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.plugin_instance", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityClientTlsPolicyClientCertificateElGrpcEndpointEl {
    target_uri: PrimField<String>,
}
impl NetworkSecurityClientTlsPolicyClientCertificateElGrpcEndpointEl {}
impl ToListMappable for NetworkSecurityClientTlsPolicyClientCertificateElGrpcEndpointEl {
    type O = BlockAssignable<NetworkSecurityClientTlsPolicyClientCertificateElGrpcEndpointEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityClientTlsPolicyClientCertificateElGrpcEndpointEl {
    #[doc = "The target URI of the gRPC endpoint. Only UDS path is supported, and should start with \"unix:\"."]
    pub target_uri: PrimField<String>,
}
impl BuildNetworkSecurityClientTlsPolicyClientCertificateElGrpcEndpointEl {
    pub fn build(self) -> NetworkSecurityClientTlsPolicyClientCertificateElGrpcEndpointEl {
        NetworkSecurityClientTlsPolicyClientCertificateElGrpcEndpointEl {
            target_uri: self.target_uri,
        }
    }
}
pub struct NetworkSecurityClientTlsPolicyClientCertificateElGrpcEndpointElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityClientTlsPolicyClientCertificateElGrpcEndpointElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityClientTlsPolicyClientCertificateElGrpcEndpointElRef {
        NetworkSecurityClientTlsPolicyClientCertificateElGrpcEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityClientTlsPolicyClientCertificateElGrpcEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target_uri` after provisioning.\nThe target URI of the gRPC endpoint. Only UDS path is supported, and should start with \"unix:\"."]
    pub fn target_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkSecurityClientTlsPolicyClientCertificateElDynamic {
    certificate_provider_instance: Option<
        DynamicBlock<
            NetworkSecurityClientTlsPolicyClientCertificateElCertificateProviderInstanceEl,
        >,
    >,
    grpc_endpoint:
        Option<DynamicBlock<NetworkSecurityClientTlsPolicyClientCertificateElGrpcEndpointEl>>,
}
#[derive(Serialize)]
pub struct NetworkSecurityClientTlsPolicyClientCertificateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_provider_instance:
        Option<Vec<NetworkSecurityClientTlsPolicyClientCertificateElCertificateProviderInstanceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc_endpoint: Option<Vec<NetworkSecurityClientTlsPolicyClientCertificateElGrpcEndpointEl>>,
    dynamic: NetworkSecurityClientTlsPolicyClientCertificateElDynamic,
}
impl NetworkSecurityClientTlsPolicyClientCertificateEl {
    #[doc = "Set the field `certificate_provider_instance`.\n"]
    pub fn set_certificate_provider_instance(
        mut self,
        v: impl Into<
            BlockAssignable<
                NetworkSecurityClientTlsPolicyClientCertificateElCertificateProviderInstanceEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.certificate_provider_instance = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.certificate_provider_instance = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `grpc_endpoint`.\n"]
    pub fn set_grpc_endpoint(
        mut self,
        v: impl Into<BlockAssignable<NetworkSecurityClientTlsPolicyClientCertificateElGrpcEndpointEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.grpc_endpoint = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc_endpoint = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkSecurityClientTlsPolicyClientCertificateEl {
    type O = BlockAssignable<NetworkSecurityClientTlsPolicyClientCertificateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityClientTlsPolicyClientCertificateEl {}
impl BuildNetworkSecurityClientTlsPolicyClientCertificateEl {
    pub fn build(self) -> NetworkSecurityClientTlsPolicyClientCertificateEl {
        NetworkSecurityClientTlsPolicyClientCertificateEl {
            certificate_provider_instance: core::default::Default::default(),
            grpc_endpoint: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkSecurityClientTlsPolicyClientCertificateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityClientTlsPolicyClientCertificateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityClientTlsPolicyClientCertificateElRef {
        NetworkSecurityClientTlsPolicyClientCertificateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityClientTlsPolicyClientCertificateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificate_provider_instance` after provisioning.\n"]
    pub fn certificate_provider_instance(
        &self,
    ) -> ListRef<NetworkSecurityClientTlsPolicyClientCertificateElCertificateProviderInstanceElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.certificate_provider_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `grpc_endpoint` after provisioning.\n"]
    pub fn grpc_endpoint(
        &self,
    ) -> ListRef<NetworkSecurityClientTlsPolicyClientCertificateElGrpcEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.grpc_endpoint", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityClientTlsPolicyServerValidationCaElCertificateProviderInstanceEl {
    plugin_instance: PrimField<String>,
}
impl NetworkSecurityClientTlsPolicyServerValidationCaElCertificateProviderInstanceEl {}
impl ToListMappable
    for NetworkSecurityClientTlsPolicyServerValidationCaElCertificateProviderInstanceEl
{
    type O = BlockAssignable<
        NetworkSecurityClientTlsPolicyServerValidationCaElCertificateProviderInstanceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityClientTlsPolicyServerValidationCaElCertificateProviderInstanceEl {
    #[doc = "Plugin instance name, used to locate and load CertificateProvider instance configuration. Set to \"google_cloud_private_spiffe\" to use Certificate Authority Service certificate provider instance."]
    pub plugin_instance: PrimField<String>,
}
impl BuildNetworkSecurityClientTlsPolicyServerValidationCaElCertificateProviderInstanceEl {
    pub fn build(
        self,
    ) -> NetworkSecurityClientTlsPolicyServerValidationCaElCertificateProviderInstanceEl {
        NetworkSecurityClientTlsPolicyServerValidationCaElCertificateProviderInstanceEl {
            plugin_instance: self.plugin_instance,
        }
    }
}
pub struct NetworkSecurityClientTlsPolicyServerValidationCaElCertificateProviderInstanceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityClientTlsPolicyServerValidationCaElCertificateProviderInstanceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityClientTlsPolicyServerValidationCaElCertificateProviderInstanceElRef {
        NetworkSecurityClientTlsPolicyServerValidationCaElCertificateProviderInstanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityClientTlsPolicyServerValidationCaElCertificateProviderInstanceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `plugin_instance` after provisioning.\nPlugin instance name, used to locate and load CertificateProvider instance configuration. Set to \"google_cloud_private_spiffe\" to use Certificate Authority Service certificate provider instance."]
    pub fn plugin_instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.plugin_instance", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityClientTlsPolicyServerValidationCaElGrpcEndpointEl {
    target_uri: PrimField<String>,
}
impl NetworkSecurityClientTlsPolicyServerValidationCaElGrpcEndpointEl {}
impl ToListMappable for NetworkSecurityClientTlsPolicyServerValidationCaElGrpcEndpointEl {
    type O = BlockAssignable<NetworkSecurityClientTlsPolicyServerValidationCaElGrpcEndpointEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityClientTlsPolicyServerValidationCaElGrpcEndpointEl {
    #[doc = "The target URI of the gRPC endpoint. Only UDS path is supported, and should start with \"unix:\"."]
    pub target_uri: PrimField<String>,
}
impl BuildNetworkSecurityClientTlsPolicyServerValidationCaElGrpcEndpointEl {
    pub fn build(self) -> NetworkSecurityClientTlsPolicyServerValidationCaElGrpcEndpointEl {
        NetworkSecurityClientTlsPolicyServerValidationCaElGrpcEndpointEl {
            target_uri: self.target_uri,
        }
    }
}
pub struct NetworkSecurityClientTlsPolicyServerValidationCaElGrpcEndpointElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityClientTlsPolicyServerValidationCaElGrpcEndpointElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityClientTlsPolicyServerValidationCaElGrpcEndpointElRef {
        NetworkSecurityClientTlsPolicyServerValidationCaElGrpcEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityClientTlsPolicyServerValidationCaElGrpcEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target_uri` after provisioning.\nThe target URI of the gRPC endpoint. Only UDS path is supported, and should start with \"unix:\"."]
    pub fn target_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkSecurityClientTlsPolicyServerValidationCaElDynamic {
    certificate_provider_instance: Option<
        DynamicBlock<
            NetworkSecurityClientTlsPolicyServerValidationCaElCertificateProviderInstanceEl,
        >,
    >,
    grpc_endpoint:
        Option<DynamicBlock<NetworkSecurityClientTlsPolicyServerValidationCaElGrpcEndpointEl>>,
}
#[derive(Serialize)]
pub struct NetworkSecurityClientTlsPolicyServerValidationCaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_provider_instance: Option<
        Vec<NetworkSecurityClientTlsPolicyServerValidationCaElCertificateProviderInstanceEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc_endpoint: Option<Vec<NetworkSecurityClientTlsPolicyServerValidationCaElGrpcEndpointEl>>,
    dynamic: NetworkSecurityClientTlsPolicyServerValidationCaElDynamic,
}
impl NetworkSecurityClientTlsPolicyServerValidationCaEl {
    #[doc = "Set the field `certificate_provider_instance`.\n"]
    pub fn set_certificate_provider_instance(
        mut self,
        v: impl Into<
            BlockAssignable<
                NetworkSecurityClientTlsPolicyServerValidationCaElCertificateProviderInstanceEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.certificate_provider_instance = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.certificate_provider_instance = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `grpc_endpoint`.\n"]
    pub fn set_grpc_endpoint(
        mut self,
        v: impl Into<BlockAssignable<NetworkSecurityClientTlsPolicyServerValidationCaElGrpcEndpointEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.grpc_endpoint = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc_endpoint = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkSecurityClientTlsPolicyServerValidationCaEl {
    type O = BlockAssignable<NetworkSecurityClientTlsPolicyServerValidationCaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityClientTlsPolicyServerValidationCaEl {}
impl BuildNetworkSecurityClientTlsPolicyServerValidationCaEl {
    pub fn build(self) -> NetworkSecurityClientTlsPolicyServerValidationCaEl {
        NetworkSecurityClientTlsPolicyServerValidationCaEl {
            certificate_provider_instance: core::default::Default::default(),
            grpc_endpoint: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkSecurityClientTlsPolicyServerValidationCaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityClientTlsPolicyServerValidationCaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityClientTlsPolicyServerValidationCaElRef {
        NetworkSecurityClientTlsPolicyServerValidationCaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityClientTlsPolicyServerValidationCaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificate_provider_instance` after provisioning.\n"]
    pub fn certificate_provider_instance(
        &self,
    ) -> ListRef<NetworkSecurityClientTlsPolicyServerValidationCaElCertificateProviderInstanceElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.certificate_provider_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `grpc_endpoint` after provisioning.\n"]
    pub fn grpc_endpoint(
        &self,
    ) -> ListRef<NetworkSecurityClientTlsPolicyServerValidationCaElGrpcEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.grpc_endpoint", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityClientTlsPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkSecurityClientTlsPolicyTimeoutsEl {
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
impl ToListMappable for NetworkSecurityClientTlsPolicyTimeoutsEl {
    type O = BlockAssignable<NetworkSecurityClientTlsPolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityClientTlsPolicyTimeoutsEl {}
impl BuildNetworkSecurityClientTlsPolicyTimeoutsEl {
    pub fn build(self) -> NetworkSecurityClientTlsPolicyTimeoutsEl {
        NetworkSecurityClientTlsPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecurityClientTlsPolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityClientTlsPolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkSecurityClientTlsPolicyTimeoutsElRef {
        NetworkSecurityClientTlsPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityClientTlsPolicyTimeoutsElRef {
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
struct NetworkSecurityClientTlsPolicyDynamic {
    client_certificate: Option<DynamicBlock<NetworkSecurityClientTlsPolicyClientCertificateEl>>,
    server_validation_ca: Option<DynamicBlock<NetworkSecurityClientTlsPolicyServerValidationCaEl>>,
}
