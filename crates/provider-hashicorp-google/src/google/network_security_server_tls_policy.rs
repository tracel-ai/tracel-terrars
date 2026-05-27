use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkSecurityServerTlsPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_open: Option<PrimField<bool>>,
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
    mtls_policy: Option<Vec<NetworkSecurityServerTlsPolicyMtlsPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_certificate: Option<Vec<NetworkSecurityServerTlsPolicyServerCertificateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkSecurityServerTlsPolicyTimeoutsEl>,
    dynamic: NetworkSecurityServerTlsPolicyDynamic,
}
struct NetworkSecurityServerTlsPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkSecurityServerTlsPolicyData>,
}
#[derive(Clone)]
pub struct NetworkSecurityServerTlsPolicy(Rc<NetworkSecurityServerTlsPolicy_>);
impl NetworkSecurityServerTlsPolicy {
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
    #[doc = "Set the field `allow_open`.\nThis field applies only for Traffic Director policies. It is must be set to false for external HTTPS load balancer policies.\nDetermines if server allows plaintext connections. If set to true, server allows plain text connections. By default, it is set to false. This setting is not exclusive of other encryption modes. For example, if allowOpen and mtlsPolicy are set, server allows both plain text and mTLS connections. See documentation of other encryption modes to confirm compatibility.\nConsider using it if you wish to upgrade in place your deployment to TLS while having mixed TLS and non-TLS traffic reaching port :80."]
    pub fn set_allow_open(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_open = Some(v.into());
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
    #[doc = "Set the field `labels`.\nSet of label tags associated with the ServerTlsPolicy resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe location of the server tls policy.\nThe default value is 'global'."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `mtls_policy`.\n"]
    pub fn set_mtls_policy(
        self,
        v: impl Into<BlockAssignable<NetworkSecurityServerTlsPolicyMtlsPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mtls_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mtls_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `server_certificate`.\n"]
    pub fn set_server_certificate(
        self,
        v: impl Into<BlockAssignable<NetworkSecurityServerTlsPolicyServerCertificateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().server_certificate = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.server_certificate = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkSecurityServerTlsPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `allow_open` after provisioning.\nThis field applies only for Traffic Director policies. It is must be set to false for external HTTPS load balancer policies.\nDetermines if server allows plaintext connections. If set to true, server allows plain text connections. By default, it is set to false. This setting is not exclusive of other encryption modes. For example, if allowOpen and mtlsPolicy are set, server allows both plain text and mTLS connections. See documentation of other encryption modes to confirm compatibility.\nConsider using it if you wish to upgrade in place your deployment to TLS while having mixed TLS and non-TLS traffic reaching port :80."]
    pub fn allow_open(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_open", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the ServerTlsPolicy was created in UTC."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the ServerTlsPolicy resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the server tls policy.\nThe default value is 'global'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the ServerTlsPolicy resource."]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the ServerTlsPolicy was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mtls_policy` after provisioning.\n"]
    pub fn mtls_policy(&self) -> ListRef<NetworkSecurityServerTlsPolicyMtlsPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mtls_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_certificate` after provisioning.\n"]
    pub fn server_certificate(
        &self,
    ) -> ListRef<NetworkSecurityServerTlsPolicyServerCertificateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.server_certificate", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkSecurityServerTlsPolicyTimeoutsElRef {
        NetworkSecurityServerTlsPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetworkSecurityServerTlsPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkSecurityServerTlsPolicy {}
impl ToListMappable for NetworkSecurityServerTlsPolicy {
    type O = ListRef<NetworkSecurityServerTlsPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkSecurityServerTlsPolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_network_security_server_tls_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkSecurityServerTlsPolicy {
    pub tf_id: String,
    #[doc = "Name of the ServerTlsPolicy resource."]
    pub name: PrimField<String>,
}
impl BuildNetworkSecurityServerTlsPolicy {
    pub fn build(self, stack: &mut Stack) -> NetworkSecurityServerTlsPolicy {
        let out = NetworkSecurityServerTlsPolicy(Rc::new(NetworkSecurityServerTlsPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkSecurityServerTlsPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allow_open: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                mtls_policy: core::default::Default::default(),
                server_certificate: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkSecurityServerTlsPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityServerTlsPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkSecurityServerTlsPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_open` after provisioning.\nThis field applies only for Traffic Director policies. It is must be set to false for external HTTPS load balancer policies.\nDetermines if server allows plaintext connections. If set to true, server allows plain text connections. By default, it is set to false. This setting is not exclusive of other encryption modes. For example, if allowOpen and mtlsPolicy are set, server allows both plain text and mTLS connections. See documentation of other encryption modes to confirm compatibility.\nConsider using it if you wish to upgrade in place your deployment to TLS while having mixed TLS and non-TLS traffic reaching port :80."]
    pub fn allow_open(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_open", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the ServerTlsPolicy was created in UTC."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the ServerTlsPolicy resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the server tls policy.\nThe default value is 'global'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the ServerTlsPolicy resource."]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the ServerTlsPolicy was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mtls_policy` after provisioning.\n"]
    pub fn mtls_policy(&self) -> ListRef<NetworkSecurityServerTlsPolicyMtlsPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mtls_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_certificate` after provisioning.\n"]
    pub fn server_certificate(
        &self,
    ) -> ListRef<NetworkSecurityServerTlsPolicyServerCertificateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.server_certificate", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkSecurityServerTlsPolicyTimeoutsElRef {
        NetworkSecurityServerTlsPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElCertificateProviderInstanceEl
{
    plugin_instance: PrimField<String>,
}
impl NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElCertificateProviderInstanceEl {}
impl ToListMappable
    for NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElCertificateProviderInstanceEl
{
    type O = BlockAssignable<
        NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElCertificateProviderInstanceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElCertificateProviderInstanceEl
{
    #[doc = "Plugin instance name, used to locate and load CertificateProvider instance configuration. Set to \"google_cloud_private_spiffe\" to use Certificate Authority Service certificate provider instance."]
    pub plugin_instance: PrimField<String>,
}
impl
    BuildNetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElCertificateProviderInstanceEl
{
    pub fn build(
        self,
    ) -> NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElCertificateProviderInstanceEl
    {
        NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElCertificateProviderInstanceEl { plugin_instance : self . plugin_instance , }
    }
}
pub struct NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElCertificateProviderInstanceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElCertificateProviderInstanceElRef { fn new (shared : StackShared , base : String) -> NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElCertificateProviderInstanceElRef { NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElCertificateProviderInstanceElRef { shared : shared , base : base . to_string () , } } }
impl
    NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElCertificateProviderInstanceElRef
{
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
pub struct NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElGrpcEndpointEl {
    target_uri: PrimField<String>,
}
impl NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElGrpcEndpointEl {}
impl ToListMappable
    for NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElGrpcEndpointEl
{
    type O = BlockAssignable<
        NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElGrpcEndpointEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElGrpcEndpointEl {
    #[doc = "The target URI of the gRPC endpoint. Only UDS path is supported, and should start with \"unix:\"."]
    pub target_uri: PrimField<String>,
}
impl BuildNetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElGrpcEndpointEl {
    pub fn build(
        self,
    ) -> NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElGrpcEndpointEl {
        NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElGrpcEndpointEl {
            target_uri: self.target_uri,
        }
    }
}
pub struct NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElGrpcEndpointElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElGrpcEndpointElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElGrpcEndpointElRef {
        NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElGrpcEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElGrpcEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target_uri` after provisioning.\nThe target URI of the gRPC endpoint. Only UDS path is supported, and should start with \"unix:\"."]
    pub fn target_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElDynamic { certificate_provider_instance : Option < DynamicBlock < NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElCertificateProviderInstanceEl >> , grpc_endpoint : Option < DynamicBlock < NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElGrpcEndpointEl >> , }
#[derive(Serialize)]
pub struct NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaEl { # [serde (skip_serializing_if = "Option::is_none")] certificate_provider_instance : Option < Vec < NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElCertificateProviderInstanceEl > > , # [serde (skip_serializing_if = "Option::is_none")] grpc_endpoint : Option < Vec < NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElGrpcEndpointEl > > , dynamic : NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElDynamic , }
impl NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaEl {
    #[doc = "Set the field `certificate_provider_instance`.\n"]
    pub fn set_certificate_provider_instance(
        mut self,
        v : impl Into < BlockAssignable < NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElCertificateProviderInstanceEl >>,
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
        v: impl Into<
            BlockAssignable<
                NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElGrpcEndpointEl,
            >,
        >,
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
impl ToListMappable for NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaEl {
    type O = BlockAssignable<NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaEl {}
impl BuildNetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaEl {
    pub fn build(self) -> NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaEl {
        NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaEl {
            certificate_provider_instance: core::default::Default::default(),
            grpc_endpoint: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElRef {
        NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificate_provider_instance` after provisioning.\n"]    pub fn certificate_provider_instance (& self) -> ListRef < NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElCertificateProviderInstanceElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.certificate_provider_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `grpc_endpoint` after provisioning.\n"]
    pub fn grpc_endpoint(
        &self,
    ) -> ListRef<NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElGrpcEndpointElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.grpc_endpoint", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct NetworkSecurityServerTlsPolicyMtlsPolicyElDynamic {
    client_validation_ca:
        Option<DynamicBlock<NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaEl>>,
}
#[derive(Serialize)]
pub struct NetworkSecurityServerTlsPolicyMtlsPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_validation_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_validation_trust_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_validation_ca:
        Option<Vec<NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaEl>>,
    dynamic: NetworkSecurityServerTlsPolicyMtlsPolicyElDynamic,
}
impl NetworkSecurityServerTlsPolicyMtlsPolicyEl {
    #[doc = "Set the field `client_validation_mode`.\nWhen the client presents an invalid certificate or no certificate to the load balancer, the clientValidationMode specifies how the client connection is handled.\nRequired if the policy is to be used with the external HTTPS load balancing. For Traffic Director it must be empty. Possible values: [\"CLIENT_VALIDATION_MODE_UNSPECIFIED\", \"ALLOW_INVALID_OR_MISSING_CLIENT_CERT\", \"REJECT_INVALID\"]"]
    pub fn set_client_validation_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_validation_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `client_validation_trust_config`.\nReference to the TrustConfig from certificatemanager.googleapis.com namespace.\nIf specified, the chain validation will be performed against certificates configured in the given TrustConfig.\nAllowed only if the policy is to be used with external HTTPS load balancers."]
    pub fn set_client_validation_trust_config(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_validation_trust_config = Some(v.into());
        self
    }
    #[doc = "Set the field `client_validation_ca`.\n"]
    pub fn set_client_validation_ca(
        mut self,
        v: impl Into<BlockAssignable<NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.client_validation_ca = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.client_validation_ca = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkSecurityServerTlsPolicyMtlsPolicyEl {
    type O = BlockAssignable<NetworkSecurityServerTlsPolicyMtlsPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityServerTlsPolicyMtlsPolicyEl {}
impl BuildNetworkSecurityServerTlsPolicyMtlsPolicyEl {
    pub fn build(self) -> NetworkSecurityServerTlsPolicyMtlsPolicyEl {
        NetworkSecurityServerTlsPolicyMtlsPolicyEl {
            client_validation_mode: core::default::Default::default(),
            client_validation_trust_config: core::default::Default::default(),
            client_validation_ca: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkSecurityServerTlsPolicyMtlsPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityServerTlsPolicyMtlsPolicyElRef {
    fn new(shared: StackShared, base: String) -> NetworkSecurityServerTlsPolicyMtlsPolicyElRef {
        NetworkSecurityServerTlsPolicyMtlsPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityServerTlsPolicyMtlsPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_validation_mode` after provisioning.\nWhen the client presents an invalid certificate or no certificate to the load balancer, the clientValidationMode specifies how the client connection is handled.\nRequired if the policy is to be used with the external HTTPS load balancing. For Traffic Director it must be empty. Possible values: [\"CLIENT_VALIDATION_MODE_UNSPECIFIED\", \"ALLOW_INVALID_OR_MISSING_CLIENT_CERT\", \"REJECT_INVALID\"]"]
    pub fn client_validation_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_validation_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_validation_trust_config` after provisioning.\nReference to the TrustConfig from certificatemanager.googleapis.com namespace.\nIf specified, the chain validation will be performed against certificates configured in the given TrustConfig.\nAllowed only if the policy is to be used with external HTTPS load balancers."]
    pub fn client_validation_trust_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_validation_trust_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_validation_ca` after provisioning.\n"]
    pub fn client_validation_ca(
        &self,
    ) -> ListRef<NetworkSecurityServerTlsPolicyMtlsPolicyElClientValidationCaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_validation_ca", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityServerTlsPolicyServerCertificateElCertificateProviderInstanceEl {
    plugin_instance: PrimField<String>,
}
impl NetworkSecurityServerTlsPolicyServerCertificateElCertificateProviderInstanceEl {}
impl ToListMappable
    for NetworkSecurityServerTlsPolicyServerCertificateElCertificateProviderInstanceEl
{
    type O = BlockAssignable<
        NetworkSecurityServerTlsPolicyServerCertificateElCertificateProviderInstanceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityServerTlsPolicyServerCertificateElCertificateProviderInstanceEl {
    #[doc = "Plugin instance name, used to locate and load CertificateProvider instance configuration. Set to \"google_cloud_private_spiffe\" to use Certificate Authority Service certificate provider instance."]
    pub plugin_instance: PrimField<String>,
}
impl BuildNetworkSecurityServerTlsPolicyServerCertificateElCertificateProviderInstanceEl {
    pub fn build(
        self,
    ) -> NetworkSecurityServerTlsPolicyServerCertificateElCertificateProviderInstanceEl {
        NetworkSecurityServerTlsPolicyServerCertificateElCertificateProviderInstanceEl {
            plugin_instance: self.plugin_instance,
        }
    }
}
pub struct NetworkSecurityServerTlsPolicyServerCertificateElCertificateProviderInstanceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityServerTlsPolicyServerCertificateElCertificateProviderInstanceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityServerTlsPolicyServerCertificateElCertificateProviderInstanceElRef {
        NetworkSecurityServerTlsPolicyServerCertificateElCertificateProviderInstanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityServerTlsPolicyServerCertificateElCertificateProviderInstanceElRef {
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
pub struct NetworkSecurityServerTlsPolicyServerCertificateElGrpcEndpointEl {
    target_uri: PrimField<String>,
}
impl NetworkSecurityServerTlsPolicyServerCertificateElGrpcEndpointEl {}
impl ToListMappable for NetworkSecurityServerTlsPolicyServerCertificateElGrpcEndpointEl {
    type O = BlockAssignable<NetworkSecurityServerTlsPolicyServerCertificateElGrpcEndpointEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityServerTlsPolicyServerCertificateElGrpcEndpointEl {
    #[doc = "The target URI of the gRPC endpoint. Only UDS path is supported, and should start with \"unix:\"."]
    pub target_uri: PrimField<String>,
}
impl BuildNetworkSecurityServerTlsPolicyServerCertificateElGrpcEndpointEl {
    pub fn build(self) -> NetworkSecurityServerTlsPolicyServerCertificateElGrpcEndpointEl {
        NetworkSecurityServerTlsPolicyServerCertificateElGrpcEndpointEl {
            target_uri: self.target_uri,
        }
    }
}
pub struct NetworkSecurityServerTlsPolicyServerCertificateElGrpcEndpointElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityServerTlsPolicyServerCertificateElGrpcEndpointElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityServerTlsPolicyServerCertificateElGrpcEndpointElRef {
        NetworkSecurityServerTlsPolicyServerCertificateElGrpcEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityServerTlsPolicyServerCertificateElGrpcEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target_uri` after provisioning.\nThe target URI of the gRPC endpoint. Only UDS path is supported, and should start with \"unix:\"."]
    pub fn target_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkSecurityServerTlsPolicyServerCertificateElDynamic {
    certificate_provider_instance: Option<
        DynamicBlock<
            NetworkSecurityServerTlsPolicyServerCertificateElCertificateProviderInstanceEl,
        >,
    >,
    grpc_endpoint:
        Option<DynamicBlock<NetworkSecurityServerTlsPolicyServerCertificateElGrpcEndpointEl>>,
}
#[derive(Serialize)]
pub struct NetworkSecurityServerTlsPolicyServerCertificateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_provider_instance:
        Option<Vec<NetworkSecurityServerTlsPolicyServerCertificateElCertificateProviderInstanceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc_endpoint: Option<Vec<NetworkSecurityServerTlsPolicyServerCertificateElGrpcEndpointEl>>,
    dynamic: NetworkSecurityServerTlsPolicyServerCertificateElDynamic,
}
impl NetworkSecurityServerTlsPolicyServerCertificateEl {
    #[doc = "Set the field `certificate_provider_instance`.\n"]
    pub fn set_certificate_provider_instance(
        mut self,
        v: impl Into<
            BlockAssignable<
                NetworkSecurityServerTlsPolicyServerCertificateElCertificateProviderInstanceEl,
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
        v: impl Into<BlockAssignable<NetworkSecurityServerTlsPolicyServerCertificateElGrpcEndpointEl>>,
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
impl ToListMappable for NetworkSecurityServerTlsPolicyServerCertificateEl {
    type O = BlockAssignable<NetworkSecurityServerTlsPolicyServerCertificateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityServerTlsPolicyServerCertificateEl {}
impl BuildNetworkSecurityServerTlsPolicyServerCertificateEl {
    pub fn build(self) -> NetworkSecurityServerTlsPolicyServerCertificateEl {
        NetworkSecurityServerTlsPolicyServerCertificateEl {
            certificate_provider_instance: core::default::Default::default(),
            grpc_endpoint: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkSecurityServerTlsPolicyServerCertificateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityServerTlsPolicyServerCertificateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityServerTlsPolicyServerCertificateElRef {
        NetworkSecurityServerTlsPolicyServerCertificateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityServerTlsPolicyServerCertificateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificate_provider_instance` after provisioning.\n"]
    pub fn certificate_provider_instance(
        &self,
    ) -> ListRef<NetworkSecurityServerTlsPolicyServerCertificateElCertificateProviderInstanceElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.certificate_provider_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `grpc_endpoint` after provisioning.\n"]
    pub fn grpc_endpoint(
        &self,
    ) -> ListRef<NetworkSecurityServerTlsPolicyServerCertificateElGrpcEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.grpc_endpoint", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityServerTlsPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkSecurityServerTlsPolicyTimeoutsEl {
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
impl ToListMappable for NetworkSecurityServerTlsPolicyTimeoutsEl {
    type O = BlockAssignable<NetworkSecurityServerTlsPolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityServerTlsPolicyTimeoutsEl {}
impl BuildNetworkSecurityServerTlsPolicyTimeoutsEl {
    pub fn build(self) -> NetworkSecurityServerTlsPolicyTimeoutsEl {
        NetworkSecurityServerTlsPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecurityServerTlsPolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityServerTlsPolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkSecurityServerTlsPolicyTimeoutsElRef {
        NetworkSecurityServerTlsPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityServerTlsPolicyTimeoutsElRef {
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
struct NetworkSecurityServerTlsPolicyDynamic {
    mtls_policy: Option<DynamicBlock<NetworkSecurityServerTlsPolicyMtlsPolicyEl>>,
    server_certificate: Option<DynamicBlock<NetworkSecurityServerTlsPolicyServerCertificateEl>>,
}
