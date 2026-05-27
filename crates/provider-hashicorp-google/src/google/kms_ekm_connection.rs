use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct KmsEkmConnectionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    crypto_space_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    etag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_management_mode: Option<PrimField<String>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_resolvers: Option<Vec<KmsEkmConnectionServiceResolversEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<KmsEkmConnectionTimeoutsEl>,
    dynamic: KmsEkmConnectionDynamic,
}
struct KmsEkmConnection_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<KmsEkmConnectionData>,
}
#[derive(Clone)]
pub struct KmsEkmConnection(Rc<KmsEkmConnection_>);
impl KmsEkmConnection {
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
    #[doc = "Set the field `crypto_space_path`.\nOptional. Identifies the EKM Crypto Space that this EkmConnection maps to. Note: This field is required if KeyManagementMode is CLOUD_KMS."]
    pub fn set_crypto_space_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().crypto_space_path = Some(v.into());
        self
    }
    #[doc = "Set the field `etag`.\nOptional. Etag of the currently stored EkmConnection."]
    pub fn set_etag(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().etag = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `key_management_mode`.\nOptional. Describes who can perform control plane operations on the EKM. If unset, this defaults to MANUAL Default value: \"MANUAL\" Possible values: [\"MANUAL\", \"CLOUD_KMS\"]"]
    pub fn set_key_management_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().key_management_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `service_resolvers`.\n"]
    pub fn set_service_resolvers(
        self,
        v: impl Into<BlockAssignable<KmsEkmConnectionServiceResolversEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_resolvers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_resolvers = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<KmsEkmConnectionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which the EkmConnection was created.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `crypto_space_path` after provisioning.\nOptional. Identifies the EKM Crypto Space that this EkmConnection maps to. Note: This field is required if KeyManagementMode is CLOUD_KMS."]
    pub fn crypto_space_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.crypto_space_path", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nOptional. Etag of the currently stored EkmConnection."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `key_management_mode` after provisioning.\nOptional. Describes who can perform control plane operations on the EKM. If unset, this defaults to MANUAL Default value: \"MANUAL\" Possible values: [\"MANUAL\", \"CLOUD_KMS\"]"]
    pub fn key_management_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_management_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the EkmConnection.\nA full list of valid locations can be found by running 'gcloud kms locations list'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for the EkmConnection."]
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
    #[doc = "Get a reference to the value of field `service_resolvers` after provisioning.\n"]
    pub fn service_resolvers(&self) -> ListRef<KmsEkmConnectionServiceResolversElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_resolvers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KmsEkmConnectionTimeoutsElRef {
        KmsEkmConnectionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for KmsEkmConnection {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for KmsEkmConnection {}
impl ToListMappable for KmsEkmConnection {
    type O = ListRef<KmsEkmConnectionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for KmsEkmConnection_ {
    fn extract_resource_type(&self) -> String {
        "google_kms_ekm_connection".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildKmsEkmConnection {
    pub tf_id: String,
    #[doc = "The location for the EkmConnection.\nA full list of valid locations can be found by running 'gcloud kms locations list'."]
    pub location: PrimField<String>,
    #[doc = "The resource name for the EkmConnection."]
    pub name: PrimField<String>,
}
impl BuildKmsEkmConnection {
    pub fn build(self, stack: &mut Stack) -> KmsEkmConnection {
        let out = KmsEkmConnection(Rc::new(KmsEkmConnection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(KmsEkmConnectionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                crypto_space_path: core::default::Default::default(),
                etag: core::default::Default::default(),
                id: core::default::Default::default(),
                key_management_mode: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                service_resolvers: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct KmsEkmConnectionRef {
    shared: StackShared,
    base: String,
}
impl Ref for KmsEkmConnectionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl KmsEkmConnectionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which the EkmConnection was created.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `crypto_space_path` after provisioning.\nOptional. Identifies the EKM Crypto Space that this EkmConnection maps to. Note: This field is required if KeyManagementMode is CLOUD_KMS."]
    pub fn crypto_space_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.crypto_space_path", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nOptional. Etag of the currently stored EkmConnection."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `key_management_mode` after provisioning.\nOptional. Describes who can perform control plane operations on the EKM. If unset, this defaults to MANUAL Default value: \"MANUAL\" Possible values: [\"MANUAL\", \"CLOUD_KMS\"]"]
    pub fn key_management_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_management_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the EkmConnection.\nA full list of valid locations can be found by running 'gcloud kms locations list'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for the EkmConnection."]
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
    #[doc = "Get a reference to the value of field `service_resolvers` after provisioning.\n"]
    pub fn service_resolvers(&self) -> ListRef<KmsEkmConnectionServiceResolversElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_resolvers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KmsEkmConnectionTimeoutsElRef {
        KmsEkmConnectionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct KmsEkmConnectionServiceResolversElServerCertificatesEl {
    raw_der: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_alternative_dns_names: Option<ListField<PrimField<String>>>,
}
impl KmsEkmConnectionServiceResolversElServerCertificatesEl {
    #[doc = "Set the field `subject_alternative_dns_names`.\nOutput only. The subject Alternative DNS names. Only present if parsed is true."]
    pub fn set_subject_alternative_dns_names(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.subject_alternative_dns_names = Some(v.into());
        self
    }
}
impl ToListMappable for KmsEkmConnectionServiceResolversElServerCertificatesEl {
    type O = BlockAssignable<KmsEkmConnectionServiceResolversElServerCertificatesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKmsEkmConnectionServiceResolversElServerCertificatesEl {
    #[doc = "Required. The raw certificate bytes in DER format. A base64-encoded string."]
    pub raw_der: PrimField<String>,
}
impl BuildKmsEkmConnectionServiceResolversElServerCertificatesEl {
    pub fn build(self) -> KmsEkmConnectionServiceResolversElServerCertificatesEl {
        KmsEkmConnectionServiceResolversElServerCertificatesEl {
            raw_der: self.raw_der,
            subject_alternative_dns_names: core::default::Default::default(),
        }
    }
}
pub struct KmsEkmConnectionServiceResolversElServerCertificatesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KmsEkmConnectionServiceResolversElServerCertificatesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KmsEkmConnectionServiceResolversElServerCertificatesElRef {
        KmsEkmConnectionServiceResolversElServerCertificatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KmsEkmConnectionServiceResolversElServerCertificatesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `issuer` after provisioning.\nOutput only. The issuer distinguished name in RFC 2253 format. Only present if parsed is true."]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }
    #[doc = "Get a reference to the value of field `not_after_time` after provisioning.\nOutput only. The certificate is not valid after this time. Only present if parsed is true.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn not_after_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.not_after_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `not_before_time` after provisioning.\nOutput only. The certificate is not valid before this time. Only present if parsed is true.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn not_before_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.not_before_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `parsed` after provisioning.\nOutput only. True if the certificate was parsed successfully."]
    pub fn parsed(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.parsed", self.base))
    }
    #[doc = "Get a reference to the value of field `raw_der` after provisioning.\nRequired. The raw certificate bytes in DER format. A base64-encoded string."]
    pub fn raw_der(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.raw_der", self.base))
    }
    #[doc = "Get a reference to the value of field `serial_number` after provisioning.\nOutput only. The certificate serial number as a hex string. Only present if parsed is true."]
    pub fn serial_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.serial_number", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sha256_fingerprint` after provisioning.\nOutput only. The SHA-256 certificate fingerprint as a hex string. Only present if parsed is true."]
    pub fn sha256_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sha256_fingerprint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subject` after provisioning.\nOutput only. The subject distinguished name in RFC 2253 format. Only present if parsed is true."]
    pub fn subject(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subject", self.base))
    }
    #[doc = "Get a reference to the value of field `subject_alternative_dns_names` after provisioning.\nOutput only. The subject Alternative DNS names. Only present if parsed is true."]
    pub fn subject_alternative_dns_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subject_alternative_dns_names", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct KmsEkmConnectionServiceResolversElDynamic {
    server_certificates:
        Option<DynamicBlock<KmsEkmConnectionServiceResolversElServerCertificatesEl>>,
}
#[derive(Serialize)]
pub struct KmsEkmConnectionServiceResolversEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_filter: Option<PrimField<String>>,
    hostname: PrimField<String>,
    service_directory_service: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_certificates: Option<Vec<KmsEkmConnectionServiceResolversElServerCertificatesEl>>,
    dynamic: KmsEkmConnectionServiceResolversElDynamic,
}
impl KmsEkmConnectionServiceResolversEl {
    #[doc = "Set the field `endpoint_filter`.\nOptional. The filter applied to the endpoints of the resolved service. If no filter is specified, all endpoints will be considered. An endpoint will be chosen arbitrarily from the filtered list for each request. For endpoint filter syntax and examples, see https://cloud.google.com/service-directory/docs/reference/rpc/google.cloud.servicedirectory.v1#resolveservicerequest."]
    pub fn set_endpoint_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint_filter = Some(v.into());
        self
    }
    #[doc = "Set the field `server_certificates`.\n"]
    pub fn set_server_certificates(
        mut self,
        v: impl Into<BlockAssignable<KmsEkmConnectionServiceResolversElServerCertificatesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.server_certificates = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.server_certificates = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for KmsEkmConnectionServiceResolversEl {
    type O = BlockAssignable<KmsEkmConnectionServiceResolversEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKmsEkmConnectionServiceResolversEl {
    #[doc = "Required. The hostname of the EKM replica used at TLS and HTTP layers."]
    pub hostname: PrimField<String>,
    #[doc = "Required. The resource name of the Service Directory service pointing to an EKM replica, in the format projects/*/locations/*/namespaces/*/services/*"]
    pub service_directory_service: PrimField<String>,
}
impl BuildKmsEkmConnectionServiceResolversEl {
    pub fn build(self) -> KmsEkmConnectionServiceResolversEl {
        KmsEkmConnectionServiceResolversEl {
            endpoint_filter: core::default::Default::default(),
            hostname: self.hostname,
            service_directory_service: self.service_directory_service,
            server_certificates: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct KmsEkmConnectionServiceResolversElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KmsEkmConnectionServiceResolversElRef {
    fn new(shared: StackShared, base: String) -> KmsEkmConnectionServiceResolversElRef {
        KmsEkmConnectionServiceResolversElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KmsEkmConnectionServiceResolversElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `endpoint_filter` after provisioning.\nOptional. The filter applied to the endpoints of the resolved service. If no filter is specified, all endpoints will be considered. An endpoint will be chosen arbitrarily from the filtered list for each request. For endpoint filter syntax and examples, see https://cloud.google.com/service-directory/docs/reference/rpc/google.cloud.servicedirectory.v1#resolveservicerequest."]
    pub fn endpoint_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_filter", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nRequired. The hostname of the EKM replica used at TLS and HTTP layers."]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
    #[doc = "Get a reference to the value of field `service_directory_service` after provisioning.\nRequired. The resource name of the Service Directory service pointing to an EKM replica, in the format projects/*/locations/*/namespaces/*/services/*"]
    pub fn service_directory_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_directory_service", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `server_certificates` after provisioning.\n"]
    pub fn server_certificates(
        &self,
    ) -> ListRef<KmsEkmConnectionServiceResolversElServerCertificatesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.server_certificates", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct KmsEkmConnectionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl KmsEkmConnectionTimeoutsEl {
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
impl ToListMappable for KmsEkmConnectionTimeoutsEl {
    type O = BlockAssignable<KmsEkmConnectionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKmsEkmConnectionTimeoutsEl {}
impl BuildKmsEkmConnectionTimeoutsEl {
    pub fn build(self) -> KmsEkmConnectionTimeoutsEl {
        KmsEkmConnectionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct KmsEkmConnectionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KmsEkmConnectionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> KmsEkmConnectionTimeoutsElRef {
        KmsEkmConnectionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KmsEkmConnectionTimeoutsElRef {
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
struct KmsEkmConnectionDynamic {
    service_resolvers: Option<DynamicBlock<KmsEkmConnectionServiceResolversEl>>,
}
