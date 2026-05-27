use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct PrivatecaCertificateAuthorityData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    certificate_authority_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs_bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_active_certificates_on_deletion: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifetime: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pem_ca_certificate: Option<PrimField<String>>,
    pool: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_grace_period: Option<PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Vec<PrivatecaCertificateAuthorityConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_spec: Option<Vec<PrivatecaCertificateAuthorityKeySpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subordinate_config: Option<Vec<PrivatecaCertificateAuthoritySubordinateConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<PrivatecaCertificateAuthorityTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_defined_access_urls: Option<Vec<PrivatecaCertificateAuthorityUserDefinedAccessUrlsEl>>,
    dynamic: PrivatecaCertificateAuthorityDynamic,
}
struct PrivatecaCertificateAuthority_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<PrivatecaCertificateAuthorityData>,
}
#[derive(Clone)]
pub struct PrivatecaCertificateAuthority(Rc<PrivatecaCertificateAuthority_>);
impl PrivatecaCertificateAuthority {
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
    #[doc = "Set the field `deletion_protection`.\nWhether Terraform will be prevented from destroying the CertificateAuthority.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the CertificateAuthority will fail.\nWhen the field is set to false, deleting the CertificateAuthority is allowed."]
    pub fn set_deletion_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = Some(v.into());
        self
    }
    #[doc = "Set the field `desired_state`.\nDesired state of the CertificateAuthority. Set this field to 'STAGED' to create a 'STAGED' root CA.\nPossible values: ENABLED, DISABLED, STAGED."]
    pub fn set_desired_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().desired_state = Some(v.into());
        self
    }
    #[doc = "Set the field `gcs_bucket`.\nThe name of a Cloud Storage bucket where this CertificateAuthority will publish content,\nsuch as the CA certificate and CRLs. This must be a bucket name, without any prefixes\n(such as 'gs://') or suffixes (such as '.googleapis.com'). For example, to use a bucket named\nmy-bucket, you would simply specify 'my-bucket'. If not specified, a managed bucket will be\ncreated."]
    pub fn set_gcs_bucket(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().gcs_bucket = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_active_certificates_on_deletion`.\nThis field allows the CA to be deleted even if the CA has active certs. Active certs include both unrevoked and unexpired certs.\nUse with care. Defaults to 'false'."]
    pub fn set_ignore_active_certificates_on_deletion(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0
            .data
            .borrow_mut()
            .ignore_active_certificates_on_deletion = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels with user-defined metadata.\n\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\":\n\"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `lifetime`.\nThe desired lifetime of the CA certificate. Used to create the \"notBeforeTime\" and\n\"notAfterTime\" fields inside an X.509 certificate. A duration in seconds with up to nine\nfractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_lifetime(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().lifetime = Some(v.into());
        self
    }
    #[doc = "Set the field `pem_ca_certificate`.\nThe signed CA certificate issued from the subordinated CA's CSR. This is needed when activating the subordiante CA with a third party issuer."]
    pub fn set_pem_ca_certificate(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().pem_ca_certificate = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `skip_grace_period`.\nIf this flag is set, the Certificate Authority will be deleted as soon as\npossible without a 30-day grace period where undeletion would have been\nallowed. If you proceed, there will be no way to recover this CA.\nUse with care. Defaults to 'false'."]
    pub fn set_skip_grace_period(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_grace_period = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nThe Type of this CertificateAuthority.\n\n~> **Note:** For 'SUBORDINATE' Certificate Authorities, they need to\nbe activated before they can issue certificates. Default value: \"SELF_SIGNED\" Possible values: [\"SELF_SIGNED\", \"SUBORDINATE\"]"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        self,
        v: impl Into<BlockAssignable<PrivatecaCertificateAuthorityConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `key_spec`.\n"]
    pub fn set_key_spec(
        self,
        v: impl Into<BlockAssignable<PrivatecaCertificateAuthorityKeySpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().key_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.key_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `subordinate_config`.\n"]
    pub fn set_subordinate_config(
        self,
        v: impl Into<BlockAssignable<PrivatecaCertificateAuthoritySubordinateConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().subordinate_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.subordinate_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<PrivatecaCertificateAuthorityTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `user_defined_access_urls`.\n"]
    pub fn set_user_defined_access_urls(
        self,
        v: impl Into<BlockAssignable<PrivatecaCertificateAuthorityUserDefinedAccessUrlsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user_defined_access_urls = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user_defined_access_urls = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `access_urls` after provisioning.\nURLs for accessing content published by this CA, such as the CA certificate and CRLs."]
    pub fn access_urls(&self) -> ListRef<PrivatecaCertificateAuthorityAccessUrlsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_urls", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `certificate_authority_id` after provisioning.\nThe user provided Resource ID for this Certificate Authority."]
    pub fn certificate_authority_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_authority_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time at which this CertificateAuthority was created.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine\nfractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
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
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the CertificateAuthority.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the CertificateAuthority will fail.\nWhen the field is set to false, deleting the CertificateAuthority is allowed."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desired_state` after provisioning.\nDesired state of the CertificateAuthority. Set this field to 'STAGED' to create a 'STAGED' root CA.\nPossible values: ENABLED, DISABLED, STAGED."]
    pub fn desired_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_bucket` after provisioning.\nThe name of a Cloud Storage bucket where this CertificateAuthority will publish content,\nsuch as the CA certificate and CRLs. This must be a bucket name, without any prefixes\n(such as 'gs://') or suffixes (such as '.googleapis.com'). For example, to use a bucket named\nmy-bucket, you would simply specify 'my-bucket'. If not specified, a managed bucket will be\ncreated."]
    pub fn gcs_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcs_bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ignore_active_certificates_on_deletion` after provisioning.\nThis field allows the CA to be deleted even if the CA has active certs. Active certs include both unrevoked and unexpired certs.\nUse with care. Defaults to 'false'."]
    pub fn ignore_active_certificates_on_deletion(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.ignore_active_certificates_on_deletion",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels with user-defined metadata.\n\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\":\n\"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lifetime` after provisioning.\nThe desired lifetime of the CA certificate. Used to create the \"notBeforeTime\" and\n\"notAfterTime\" fields inside an X.509 certificate. A duration in seconds with up to nine\nfractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn lifetime(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifetime", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation of the CertificateAuthority. A full list of valid locations can be found by\nrunning 'gcloud privateca locations list'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for this CertificateAuthority in the format\nprojects/*/locations/*/certificateAuthorities/*."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pem_ca_certificate` after provisioning.\nThe signed CA certificate issued from the subordinated CA's CSR. This is needed when activating the subordiante CA with a third party issuer."]
    pub fn pem_ca_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pem_ca_certificate", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pem_ca_certificates` after provisioning.\nThis CertificateAuthority's certificate chain, including the current\nCertificateAuthority's certificate. Ordered such that the root issuer is the final\nelement (consistent with RFC 5246). For a self-signed CA, this will only list the current\nCertificateAuthority's certificate."]
    pub fn pem_ca_certificates(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pem_ca_certificates", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pool` after provisioning.\nThe name of the CaPool this Certificate Authority belongs to."]
    pub fn pool(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `skip_grace_period` after provisioning.\nIf this flag is set, the Certificate Authority will be deleted as soon as\npossible without a 30-day grace period where undeletion would have been\nallowed. If you proceed, there will be no way to recover this CA.\nUse with care. Defaults to 'false'."]
    pub fn skip_grace_period(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_grace_period", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe State for this CertificateAuthority."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe Type of this CertificateAuthority.\n\n~> **Note:** For 'SUBORDINATE' Certificate Authorities, they need to\nbe activated before they can issue certificates. Default value: \"SELF_SIGNED\" Possible values: [\"SELF_SIGNED\", \"SUBORDINATE\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time at which this CertificateAuthority was updated.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine\nfractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<PrivatecaCertificateAuthorityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `key_spec` after provisioning.\n"]
    pub fn key_spec(&self) -> ListRef<PrivatecaCertificateAuthorityKeySpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.key_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subordinate_config` after provisioning.\n"]
    pub fn subordinate_config(
        &self,
    ) -> ListRef<PrivatecaCertificateAuthoritySubordinateConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subordinate_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrivatecaCertificateAuthorityTimeoutsElRef {
        PrivatecaCertificateAuthorityTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_defined_access_urls` after provisioning.\n"]
    pub fn user_defined_access_urls(
        &self,
    ) -> ListRef<PrivatecaCertificateAuthorityUserDefinedAccessUrlsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_defined_access_urls", self.extract_ref()),
        )
    }
}
impl Referable for PrivatecaCertificateAuthority {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for PrivatecaCertificateAuthority {}
impl ToListMappable for PrivatecaCertificateAuthority {
    type O = ListRef<PrivatecaCertificateAuthorityRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for PrivatecaCertificateAuthority_ {
    fn extract_resource_type(&self) -> String {
        "google_privateca_certificate_authority".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildPrivatecaCertificateAuthority {
    pub tf_id: String,
    #[doc = "The user provided Resource ID for this Certificate Authority."]
    pub certificate_authority_id: PrimField<String>,
    #[doc = "Location of the CertificateAuthority. A full list of valid locations can be found by\nrunning 'gcloud privateca locations list'."]
    pub location: PrimField<String>,
    #[doc = "The name of the CaPool this Certificate Authority belongs to."]
    pub pool: PrimField<String>,
}
impl BuildPrivatecaCertificateAuthority {
    pub fn build(self, stack: &mut Stack) -> PrivatecaCertificateAuthority {
        let out = PrivatecaCertificateAuthority(Rc::new(PrivatecaCertificateAuthority_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(PrivatecaCertificateAuthorityData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                certificate_authority_id: self.certificate_authority_id,
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                desired_state: core::default::Default::default(),
                gcs_bucket: core::default::Default::default(),
                id: core::default::Default::default(),
                ignore_active_certificates_on_deletion: core::default::Default::default(),
                labels: core::default::Default::default(),
                lifetime: core::default::Default::default(),
                location: self.location,
                pem_ca_certificate: core::default::Default::default(),
                pool: self.pool,
                project: core::default::Default::default(),
                skip_grace_period: core::default::Default::default(),
                type_: core::default::Default::default(),
                config: core::default::Default::default(),
                key_spec: core::default::Default::default(),
                subordinate_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                user_defined_access_urls: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct PrivatecaCertificateAuthorityRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateAuthorityRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl PrivatecaCertificateAuthorityRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_urls` after provisioning.\nURLs for accessing content published by this CA, such as the CA certificate and CRLs."]
    pub fn access_urls(&self) -> ListRef<PrivatecaCertificateAuthorityAccessUrlsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_urls", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `certificate_authority_id` after provisioning.\nThe user provided Resource ID for this Certificate Authority."]
    pub fn certificate_authority_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_authority_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time at which this CertificateAuthority was created.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine\nfractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
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
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the CertificateAuthority.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the CertificateAuthority will fail.\nWhen the field is set to false, deleting the CertificateAuthority is allowed."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desired_state` after provisioning.\nDesired state of the CertificateAuthority. Set this field to 'STAGED' to create a 'STAGED' root CA.\nPossible values: ENABLED, DISABLED, STAGED."]
    pub fn desired_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_bucket` after provisioning.\nThe name of a Cloud Storage bucket where this CertificateAuthority will publish content,\nsuch as the CA certificate and CRLs. This must be a bucket name, without any prefixes\n(such as 'gs://') or suffixes (such as '.googleapis.com'). For example, to use a bucket named\nmy-bucket, you would simply specify 'my-bucket'. If not specified, a managed bucket will be\ncreated."]
    pub fn gcs_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcs_bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ignore_active_certificates_on_deletion` after provisioning.\nThis field allows the CA to be deleted even if the CA has active certs. Active certs include both unrevoked and unexpired certs.\nUse with care. Defaults to 'false'."]
    pub fn ignore_active_certificates_on_deletion(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.ignore_active_certificates_on_deletion",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels with user-defined metadata.\n\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\":\n\"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lifetime` after provisioning.\nThe desired lifetime of the CA certificate. Used to create the \"notBeforeTime\" and\n\"notAfterTime\" fields inside an X.509 certificate. A duration in seconds with up to nine\nfractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn lifetime(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifetime", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation of the CertificateAuthority. A full list of valid locations can be found by\nrunning 'gcloud privateca locations list'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for this CertificateAuthority in the format\nprojects/*/locations/*/certificateAuthorities/*."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pem_ca_certificate` after provisioning.\nThe signed CA certificate issued from the subordinated CA's CSR. This is needed when activating the subordiante CA with a third party issuer."]
    pub fn pem_ca_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pem_ca_certificate", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pem_ca_certificates` after provisioning.\nThis CertificateAuthority's certificate chain, including the current\nCertificateAuthority's certificate. Ordered such that the root issuer is the final\nelement (consistent with RFC 5246). For a self-signed CA, this will only list the current\nCertificateAuthority's certificate."]
    pub fn pem_ca_certificates(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pem_ca_certificates", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pool` after provisioning.\nThe name of the CaPool this Certificate Authority belongs to."]
    pub fn pool(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `skip_grace_period` after provisioning.\nIf this flag is set, the Certificate Authority will be deleted as soon as\npossible without a 30-day grace period where undeletion would have been\nallowed. If you proceed, there will be no way to recover this CA.\nUse with care. Defaults to 'false'."]
    pub fn skip_grace_period(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_grace_period", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe State for this CertificateAuthority."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe Type of this CertificateAuthority.\n\n~> **Note:** For 'SUBORDINATE' Certificate Authorities, they need to\nbe activated before they can issue certificates. Default value: \"SELF_SIGNED\" Possible values: [\"SELF_SIGNED\", \"SUBORDINATE\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time at which this CertificateAuthority was updated.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine\nfractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<PrivatecaCertificateAuthorityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `key_spec` after provisioning.\n"]
    pub fn key_spec(&self) -> ListRef<PrivatecaCertificateAuthorityKeySpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.key_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subordinate_config` after provisioning.\n"]
    pub fn subordinate_config(
        &self,
    ) -> ListRef<PrivatecaCertificateAuthoritySubordinateConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subordinate_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrivatecaCertificateAuthorityTimeoutsElRef {
        PrivatecaCertificateAuthorityTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_defined_access_urls` after provisioning.\n"]
    pub fn user_defined_access_urls(
        &self,
    ) -> ListRef<PrivatecaCertificateAuthorityUserDefinedAccessUrlsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_defined_access_urls", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct PrivatecaCertificateAuthorityAccessUrlsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_certificate_access_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    crl_access_urls: Option<ListField<PrimField<String>>>,
}
impl PrivatecaCertificateAuthorityAccessUrlsEl {
    #[doc = "Set the field `ca_certificate_access_url`.\n"]
    pub fn set_ca_certificate_access_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ca_certificate_access_url = Some(v.into());
        self
    }
    #[doc = "Set the field `crl_access_urls`.\n"]
    pub fn set_crl_access_urls(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.crl_access_urls = Some(v.into());
        self
    }
}
impl ToListMappable for PrivatecaCertificateAuthorityAccessUrlsEl {
    type O = BlockAssignable<PrivatecaCertificateAuthorityAccessUrlsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateAuthorityAccessUrlsEl {}
impl BuildPrivatecaCertificateAuthorityAccessUrlsEl {
    pub fn build(self) -> PrivatecaCertificateAuthorityAccessUrlsEl {
        PrivatecaCertificateAuthorityAccessUrlsEl {
            ca_certificate_access_url: core::default::Default::default(),
            crl_access_urls: core::default::Default::default(),
        }
    }
}
pub struct PrivatecaCertificateAuthorityAccessUrlsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateAuthorityAccessUrlsElRef {
    fn new(shared: StackShared, base: String) -> PrivatecaCertificateAuthorityAccessUrlsElRef {
        PrivatecaCertificateAuthorityAccessUrlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateAuthorityAccessUrlsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ca_certificate_access_url` after provisioning.\n"]
    pub fn ca_certificate_access_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ca_certificate_access_url", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `crl_access_urls` after provisioning.\n"]
    pub fn crl_access_urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.crl_access_urls", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectEl {
    common_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locality: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organization: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organizational_unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    postal_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    province: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    street_address: Option<PrimField<String>>,
}
impl PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectEl {
    #[doc = "Set the field `country_code`.\nThe country code of the subject."]
    pub fn set_country_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.country_code = Some(v.into());
        self
    }
    #[doc = "Set the field `locality`.\nThe locality or city of the subject."]
    pub fn set_locality(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.locality = Some(v.into());
        self
    }
    #[doc = "Set the field `organization`.\nThe organization of the subject."]
    pub fn set_organization(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.organization = Some(v.into());
        self
    }
    #[doc = "Set the field `organizational_unit`.\nThe organizational unit of the subject."]
    pub fn set_organizational_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.organizational_unit = Some(v.into());
        self
    }
    #[doc = "Set the field `postal_code`.\nThe postal code of the subject."]
    pub fn set_postal_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.postal_code = Some(v.into());
        self
    }
    #[doc = "Set the field `province`.\nThe province, territory, or regional state of the subject."]
    pub fn set_province(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.province = Some(v.into());
        self
    }
    #[doc = "Set the field `street_address`.\nThe street address of the subject."]
    pub fn set_street_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.street_address = Some(v.into());
        self
    }
}
impl ToListMappable for PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectEl {
    type O = BlockAssignable<PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectEl {
    #[doc = "The common name of the distinguished name."]
    pub common_name: PrimField<String>,
}
impl BuildPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectEl {
    pub fn build(self) -> PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectEl {
        PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectEl {
            common_name: self.common_name,
            country_code: core::default::Default::default(),
            locality: core::default::Default::default(),
            organization: core::default::Default::default(),
            organizational_unit: core::default::Default::default(),
            postal_code: core::default::Default::default(),
            province: core::default::Default::default(),
            street_address: core::default::Default::default(),
        }
    }
}
pub struct PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectElRef {
        PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `common_name` after provisioning.\nThe common name of the distinguished name."]
    pub fn common_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.base))
    }
    #[doc = "Get a reference to the value of field `country_code` after provisioning.\nThe country code of the subject."]
    pub fn country_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.country_code", self.base))
    }
    #[doc = "Get a reference to the value of field `locality` after provisioning.\nThe locality or city of the subject."]
    pub fn locality(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.locality", self.base))
    }
    #[doc = "Get a reference to the value of field `organization` after provisioning.\nThe organization of the subject."]
    pub fn organization(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.organization", self.base))
    }
    #[doc = "Get a reference to the value of field `organizational_unit` after provisioning.\nThe organizational unit of the subject."]
    pub fn organizational_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organizational_unit", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `postal_code` after provisioning.\nThe postal code of the subject."]
    pub fn postal_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.postal_code", self.base))
    }
    #[doc = "Get a reference to the value of field `province` after provisioning.\nThe province, territory, or regional state of the subject."]
    pub fn province(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.province", self.base))
    }
    #[doc = "Get a reference to the value of field `street_address` after provisioning.\nThe street address of the subject."]
    pub fn street_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.street_address", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_names: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uris: Option<ListField<PrimField<String>>>,
}
impl PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameEl {
    #[doc = "Set the field `dns_names`.\nContains only valid, fully-qualified host names."]
    pub fn set_dns_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.dns_names = Some(v.into());
        self
    }
    #[doc = "Set the field `email_addresses`.\nContains only valid RFC 2822 E-mail addresses."]
    pub fn set_email_addresses(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.email_addresses = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_addresses`.\nContains only valid 32-bit IPv4 addresses or RFC 4291 IPv6 addresses."]
    pub fn set_ip_addresses(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ip_addresses = Some(v.into());
        self
    }
    #[doc = "Set the field `uris`.\nContains only valid RFC 3986 URIs."]
    pub fn set_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.uris = Some(v.into());
        self
    }
}
impl ToListMappable for PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameEl {
    type O = BlockAssignable<PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameEl {}
impl BuildPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameEl {
    pub fn build(self) -> PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameEl {
        PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameEl {
            dns_names: core::default::Default::default(),
            email_addresses: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
            uris: core::default::Default::default(),
        }
    }
}
pub struct PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameElRef {
        PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dns_names` after provisioning.\nContains only valid, fully-qualified host names."]
    pub fn dns_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.dns_names", self.base))
    }
    #[doc = "Get a reference to the value of field `email_addresses` after provisioning.\nContains only valid RFC 2822 E-mail addresses."]
    pub fn email_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.email_addresses", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ip_addresses` after provisioning.\nContains only valid 32-bit IPv4 addresses or RFC 4291 IPv6 addresses."]
    pub fn ip_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip_addresses", self.base))
    }
    #[doc = "Get a reference to the value of field `uris` after provisioning.\nContains only valid RFC 3986 URIs."]
    pub fn uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.uris", self.base))
    }
}
#[derive(Serialize, Default)]
struct PrivatecaCertificateAuthorityConfigElSubjectConfigElDynamic {
    subject: Option<DynamicBlock<PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectEl>>,
    subject_alt_name:
        Option<DynamicBlock<PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameEl>>,
}
#[derive(Serialize)]
pub struct PrivatecaCertificateAuthorityConfigElSubjectConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    subject: Option<Vec<PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_alt_name:
        Option<Vec<PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameEl>>,
    dynamic: PrivatecaCertificateAuthorityConfigElSubjectConfigElDynamic,
}
impl PrivatecaCertificateAuthorityConfigElSubjectConfigEl {
    #[doc = "Set the field `subject`.\n"]
    pub fn set_subject(
        mut self,
        v: impl Into<BlockAssignable<PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.subject = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.subject = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `subject_alt_name`.\n"]
    pub fn set_subject_alt_name(
        mut self,
        v: impl Into<
            BlockAssignable<PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.subject_alt_name = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.subject_alt_name = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PrivatecaCertificateAuthorityConfigElSubjectConfigEl {
    type O = BlockAssignable<PrivatecaCertificateAuthorityConfigElSubjectConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateAuthorityConfigElSubjectConfigEl {}
impl BuildPrivatecaCertificateAuthorityConfigElSubjectConfigEl {
    pub fn build(self) -> PrivatecaCertificateAuthorityConfigElSubjectConfigEl {
        PrivatecaCertificateAuthorityConfigElSubjectConfigEl {
            subject: core::default::Default::default(),
            subject_alt_name: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PrivatecaCertificateAuthorityConfigElSubjectConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateAuthorityConfigElSubjectConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateAuthorityConfigElSubjectConfigElRef {
        PrivatecaCertificateAuthorityConfigElSubjectConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateAuthorityConfigElSubjectConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `subject` after provisioning.\n"]
    pub fn subject(
        &self,
    ) -> ListRef<PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.subject", self.base))
    }
    #[doc = "Get a reference to the value of field `subject_alt_name` after provisioning.\n"]
    pub fn subject_alt_name(
        &self,
    ) -> ListRef<PrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subject_alt_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrivatecaCertificateAuthorityConfigElSubjectKeyIdEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key_id: Option<PrimField<String>>,
}
impl PrivatecaCertificateAuthorityConfigElSubjectKeyIdEl {
    #[doc = "Set the field `key_id`.\nThe value of the KeyId in lowercase hexadecimal."]
    pub fn set_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_id = Some(v.into());
        self
    }
}
impl ToListMappable for PrivatecaCertificateAuthorityConfigElSubjectKeyIdEl {
    type O = BlockAssignable<PrivatecaCertificateAuthorityConfigElSubjectKeyIdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateAuthorityConfigElSubjectKeyIdEl {}
impl BuildPrivatecaCertificateAuthorityConfigElSubjectKeyIdEl {
    pub fn build(self) -> PrivatecaCertificateAuthorityConfigElSubjectKeyIdEl {
        PrivatecaCertificateAuthorityConfigElSubjectKeyIdEl {
            key_id: core::default::Default::default(),
        }
    }
}
pub struct PrivatecaCertificateAuthorityConfigElSubjectKeyIdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateAuthorityConfigElSubjectKeyIdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateAuthorityConfigElSubjectKeyIdElRef {
        PrivatecaCertificateAuthorityConfigElSubjectKeyIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateAuthorityConfigElSubjectKeyIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key_id` after provisioning.\nThe value of the KeyId in lowercase hexadecimal."]
    pub fn key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.base))
    }
}
#[derive(Serialize)]
pub struct PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdEl {
    object_id_path: ListField<PrimField<f64>>,
}
impl PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdEl {}
impl ToListMappable
    for PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdEl
{
    type O = BlockAssignable<
        PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdEl {
    #[doc = "An ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages."]
    pub object_id_path: ListField<PrimField<f64>>,
}
impl BuildPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdEl {
    pub fn build(
        self,
    ) -> PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdEl {
        PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdEl {
            object_id_path: self.object_id_path,
        }
    }
}
pub struct PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdElRef {
        PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `object_id_path` after provisioning.\nAn ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages."]
    pub fn object_id_path(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.object_id_path", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElDynamic {
    object_id: Option<
        DynamicBlock<
            PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsEl {
    critical: PrimField<bool>,
    value: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_id: Option<
        Vec<PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdEl>,
    >,
    dynamic: PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElDynamic,
}
impl PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsEl {
    #[doc = "Set the field `object_id`.\n"]
    pub fn set_object_id(
        mut self,
        v: impl Into<
            BlockAssignable<
                PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.object_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.object_id = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsEl {
    type O =
        BlockAssignable<PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsEl {
    #[doc = "Indicates whether or not this extension is critical (i.e., if the client does not know how to\nhandle this extension, the client should consider this to be an error)."]
    pub critical: PrimField<bool>,
    #[doc = "The value of this X.509 extension. A base64-encoded string."]
    pub value: PrimField<String>,
}
impl BuildPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsEl {
    pub fn build(self) -> PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsEl {
        PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsEl {
            critical: self.critical,
            value: self.value,
            object_id: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElRef {
        PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `critical` after provisioning.\nIndicates whether or not this extension is critical (i.e., if the client does not know how to\nhandle this extension, the client should consider this to be an error)."]
    pub fn critical(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.critical", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value of this X.509 extension. A base64-encoded string."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
    #[doc = "Get a reference to the value of field `object_id` after provisioning.\n"]
    pub fn object_id(
        &self,
    ) -> ListRef<PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.object_id", self.base))
    }
}
#[derive(Serialize)]
pub struct PrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsEl {
    is_ca: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_issuer_path_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    non_ca: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zero_max_issuer_path_length: Option<PrimField<bool>>,
}
impl PrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsEl {
    #[doc = "Set the field `max_issuer_path_length`.\nRefers to the \"path length constraint\" in Basic Constraints extension. For a CA certificate, this value describes the depth of\nsubordinate CA certificates that are allowed. If this value is less than 0, the request will fail. Setting the value to 0\nrequires setting 'zero_max_issuer_path_length = true'."]
    pub fn set_max_issuer_path_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_issuer_path_length = Some(v.into());
        self
    }
    #[doc = "Set the field `non_ca`.\nWhen true, the \"CA\" in Basic Constraints extension will be set to false.\nIf both 'is_ca' and 'non_ca' are unset, the extension will be omitted from the CA certificate."]
    pub fn set_non_ca(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.non_ca = Some(v.into());
        self
    }
    #[doc = "Set the field `zero_max_issuer_path_length`.\nWhen true, the \"path length constraint\" in Basic Constraints extension will be set to 0.\nIf both 'max_issuer_path_length' and 'zero_max_issuer_path_length' are unset,\nthe max path length will be omitted from the CA certificate."]
    pub fn set_zero_max_issuer_path_length(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.zero_max_issuer_path_length = Some(v.into());
        self
    }
}
impl ToListMappable for PrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsEl {
    type O = BlockAssignable<PrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsEl {
    #[doc = "When true, the \"CA\" in Basic Constraints extension will be set to true."]
    pub is_ca: PrimField<bool>,
}
impl BuildPrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsEl {
    pub fn build(self) -> PrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsEl {
        PrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsEl {
            is_ca: self.is_ca,
            max_issuer_path_length: core::default::Default::default(),
            non_ca: core::default::Default::default(),
            zero_max_issuer_path_length: core::default::Default::default(),
        }
    }
}
pub struct PrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsElRef {
        PrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `is_ca` after provisioning.\nWhen true, the \"CA\" in Basic Constraints extension will be set to true."]
    pub fn is_ca(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_ca", self.base))
    }
    #[doc = "Get a reference to the value of field `max_issuer_path_length` after provisioning.\nRefers to the \"path length constraint\" in Basic Constraints extension. For a CA certificate, this value describes the depth of\nsubordinate CA certificates that are allowed. If this value is less than 0, the request will fail. Setting the value to 0\nrequires setting 'zero_max_issuer_path_length = true'."]
    pub fn max_issuer_path_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_issuer_path_length", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `non_ca` after provisioning.\nWhen true, the \"CA\" in Basic Constraints extension will be set to false.\nIf both 'is_ca' and 'non_ca' are unset, the extension will be omitted from the CA certificate."]
    pub fn non_ca(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.non_ca", self.base))
    }
    #[doc = "Get a reference to the value of field `zero_max_issuer_path_length` after provisioning.\nWhen true, the \"path length constraint\" in Basic Constraints extension will be set to 0.\nIf both 'max_issuer_path_length' and 'zero_max_issuer_path_length' are unset,\nthe max path length will be omitted from the CA certificate."]
    pub fn zero_max_issuer_path_length(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zero_max_issuer_path_length", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cert_sign: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_commitment: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    crl_sign: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_encipherment: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    decipher_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    digital_signature: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encipher_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_agreement: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_encipherment: Option<PrimField<bool>>,
}
impl PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageEl {
    #[doc = "Set the field `cert_sign`.\nThe key may be used to sign certificates."]
    pub fn set_cert_sign(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.cert_sign = Some(v.into());
        self
    }
    #[doc = "Set the field `content_commitment`.\nThe key may be used for cryptographic commitments. Note that this may also be referred to as \"non-repudiation\"."]
    pub fn set_content_commitment(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.content_commitment = Some(v.into());
        self
    }
    #[doc = "Set the field `crl_sign`.\nThe key may be used sign certificate revocation lists."]
    pub fn set_crl_sign(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.crl_sign = Some(v.into());
        self
    }
    #[doc = "Set the field `data_encipherment`.\nThe key may be used to encipher data."]
    pub fn set_data_encipherment(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.data_encipherment = Some(v.into());
        self
    }
    #[doc = "Set the field `decipher_only`.\nThe key may be used to decipher only."]
    pub fn set_decipher_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.decipher_only = Some(v.into());
        self
    }
    #[doc = "Set the field `digital_signature`.\nThe key may be used for digital signatures."]
    pub fn set_digital_signature(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.digital_signature = Some(v.into());
        self
    }
    #[doc = "Set the field `encipher_only`.\nThe key may be used to encipher only."]
    pub fn set_encipher_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.encipher_only = Some(v.into());
        self
    }
    #[doc = "Set the field `key_agreement`.\nThe key may be used in a key agreement protocol."]
    pub fn set_key_agreement(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.key_agreement = Some(v.into());
        self
    }
    #[doc = "Set the field `key_encipherment`.\nThe key may be used to encipher other keys."]
    pub fn set_key_encipherment(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.key_encipherment = Some(v.into());
        self
    }
}
impl ToListMappable for PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageEl {
    type O =
        BlockAssignable<PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageEl {}
impl BuildPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageEl {
    pub fn build(
        self,
    ) -> PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageEl {
        PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageEl {
            cert_sign: core::default::Default::default(),
            content_commitment: core::default::Default::default(),
            crl_sign: core::default::Default::default(),
            data_encipherment: core::default::Default::default(),
            decipher_only: core::default::Default::default(),
            digital_signature: core::default::Default::default(),
            encipher_only: core::default::Default::default(),
            key_agreement: core::default::Default::default(),
            key_encipherment: core::default::Default::default(),
        }
    }
}
pub struct PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageElRef {
        PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cert_sign` after provisioning.\nThe key may be used to sign certificates."]
    pub fn cert_sign(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cert_sign", self.base))
    }
    #[doc = "Get a reference to the value of field `content_commitment` after provisioning.\nThe key may be used for cryptographic commitments. Note that this may also be referred to as \"non-repudiation\"."]
    pub fn content_commitment(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_commitment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `crl_sign` after provisioning.\nThe key may be used sign certificate revocation lists."]
    pub fn crl_sign(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.crl_sign", self.base))
    }
    #[doc = "Get a reference to the value of field `data_encipherment` after provisioning.\nThe key may be used to encipher data."]
    pub fn data_encipherment(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_encipherment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `decipher_only` after provisioning.\nThe key may be used to decipher only."]
    pub fn decipher_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.decipher_only", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `digital_signature` after provisioning.\nThe key may be used for digital signatures."]
    pub fn digital_signature(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.digital_signature", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encipher_only` after provisioning.\nThe key may be used to encipher only."]
    pub fn encipher_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encipher_only", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key_agreement` after provisioning.\nThe key may be used in a key agreement protocol."]
    pub fn key_agreement(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_agreement", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key_encipherment` after provisioning.\nThe key may be used to encipher other keys."]
    pub fn key_encipherment(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_encipherment", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_auth: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_signing: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ocsp_signing: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_auth: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_stamping: Option<PrimField<bool>>,
}
impl PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl {
    #[doc = "Set the field `client_auth`.\nCorresponds to OID 1.3.6.1.5.5.7.3.2. Officially described as \"TLS WWW client authentication\", though regularly used for non-WWW TLS."]
    pub fn set_client_auth(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.client_auth = Some(v.into());
        self
    }
    #[doc = "Set the field `code_signing`.\nCorresponds to OID 1.3.6.1.5.5.7.3.3. Officially described as \"Signing of downloadable executable code client authentication\"."]
    pub fn set_code_signing(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.code_signing = Some(v.into());
        self
    }
    #[doc = "Set the field `email_protection`.\nCorresponds to OID 1.3.6.1.5.5.7.3.4. Officially described as \"Email protection\"."]
    pub fn set_email_protection(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.email_protection = Some(v.into());
        self
    }
    #[doc = "Set the field `ocsp_signing`.\nCorresponds to OID 1.3.6.1.5.5.7.3.9. Officially described as \"Signing OCSP responses\"."]
    pub fn set_ocsp_signing(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ocsp_signing = Some(v.into());
        self
    }
    #[doc = "Set the field `server_auth`.\nCorresponds to OID 1.3.6.1.5.5.7.3.1. Officially described as \"TLS WWW server authentication\", though regularly used for non-WWW TLS."]
    pub fn set_server_auth(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.server_auth = Some(v.into());
        self
    }
    #[doc = "Set the field `time_stamping`.\nCorresponds to OID 1.3.6.1.5.5.7.3.8. Officially described as \"Binding the hash of an object to a time\"."]
    pub fn set_time_stamping(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.time_stamping = Some(v.into());
        self
    }
}
impl ToListMappable
    for PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl
{
    type O = BlockAssignable<
        PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl {}
impl BuildPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl {
    pub fn build(
        self,
    ) -> PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl {
        PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl {
            client_auth: core::default::Default::default(),
            code_signing: core::default::Default::default(),
            email_protection: core::default::Default::default(),
            ocsp_signing: core::default::Default::default(),
            server_auth: core::default::Default::default(),
            time_stamping: core::default::Default::default(),
        }
    }
}
pub struct PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageElRef {
        PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_auth` after provisioning.\nCorresponds to OID 1.3.6.1.5.5.7.3.2. Officially described as \"TLS WWW client authentication\", though regularly used for non-WWW TLS."]
    pub fn client_auth(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_auth", self.base))
    }
    #[doc = "Get a reference to the value of field `code_signing` after provisioning.\nCorresponds to OID 1.3.6.1.5.5.7.3.3. Officially described as \"Signing of downloadable executable code client authentication\"."]
    pub fn code_signing(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.code_signing", self.base))
    }
    #[doc = "Get a reference to the value of field `email_protection` after provisioning.\nCorresponds to OID 1.3.6.1.5.5.7.3.4. Officially described as \"Email protection\"."]
    pub fn email_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.email_protection", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ocsp_signing` after provisioning.\nCorresponds to OID 1.3.6.1.5.5.7.3.9. Officially described as \"Signing OCSP responses\"."]
    pub fn ocsp_signing(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ocsp_signing", self.base))
    }
    #[doc = "Get a reference to the value of field `server_auth` after provisioning.\nCorresponds to OID 1.3.6.1.5.5.7.3.1. Officially described as \"TLS WWW server authentication\", though regularly used for non-WWW TLS."]
    pub fn server_auth(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_auth", self.base))
    }
    #[doc = "Get a reference to the value of field `time_stamping` after provisioning.\nCorresponds to OID 1.3.6.1.5.5.7.3.8. Officially described as \"Binding the hash of an object to a time\"."]
    pub fn time_stamping(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_stamping", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl {
    object_id_path: ListField<PrimField<f64>>,
}
impl PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl {}
impl ToListMappable
    for PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl
{
    type O = BlockAssignable<
        PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl
{
    #[doc = "An ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages."]
    pub object_id_path: ListField<PrimField<f64>>,
}
impl BuildPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl {
    pub fn build(
        self,
    ) -> PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl {
        PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl {
            object_id_path: self.object_id_path,
        }
    }
}
pub struct PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesElRef
    {
        PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `object_id_path` after provisioning.\nAn ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages."]
    pub fn object_id_path(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.object_id_path", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElDynamic {
    base_key_usage: Option<
        DynamicBlock<PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageEl>,
    >,
    extended_key_usage: Option<
        DynamicBlock<PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl>,
    >,
    unknown_extended_key_usages: Option<
        DynamicBlock<
            PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    base_key_usage:
        Option<Vec<PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extended_key_usage:
        Option<Vec<PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unknown_extended_key_usages: Option<
        Vec<PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl>,
    >,
    dynamic: PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElDynamic,
}
impl PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageEl {
    #[doc = "Set the field `base_key_usage`.\n"]
    pub fn set_base_key_usage(
        mut self,
        v: impl Into<
            BlockAssignable<
                PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.base_key_usage = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.base_key_usage = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `extended_key_usage`.\n"]
    pub fn set_extended_key_usage(
        mut self,
        v: impl Into<
            BlockAssignable<
                PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.extended_key_usage = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.extended_key_usage = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `unknown_extended_key_usages`.\n"]
    pub fn set_unknown_extended_key_usages(
        mut self,
        v : impl Into < BlockAssignable < PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.unknown_extended_key_usages = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.unknown_extended_key_usages = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageEl {
    type O = BlockAssignable<PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageEl {}
impl BuildPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageEl {
    pub fn build(self) -> PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageEl {
        PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageEl {
            base_key_usage: core::default::Default::default(),
            extended_key_usage: core::default::Default::default(),
            unknown_extended_key_usages: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElRef {
        PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `base_key_usage` after provisioning.\n"]
    pub fn base_key_usage(
        &self,
    ) -> ListRef<PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.base_key_usage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `extended_key_usage` after provisioning.\n"]
    pub fn extended_key_usage(
        &self,
    ) -> ListRef<PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.extended_key_usage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `unknown_extended_key_usages` after provisioning.\n"]
    pub fn unknown_extended_key_usages(
        &self,
    ) -> ListRef<
        PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.unknown_extended_key_usages", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsEl {
    critical: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_dns_names: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_email_addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_ip_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_uris: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permitted_dns_names: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permitted_email_addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permitted_ip_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permitted_uris: Option<ListField<PrimField<String>>>,
}
impl PrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsEl {
    #[doc = "Set the field `excluded_dns_names`.\nContains excluded DNS names. Any DNS name that can be\nconstructed by simply adding zero or more labels to\nthe left-hand side of the name satisfies the name constraint.\nFor example, 'example.com', 'www.example.com', 'www.sub.example.com'\nwould satisfy 'example.com' while 'example1.com' does not."]
    pub fn set_excluded_dns_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.excluded_dns_names = Some(v.into());
        self
    }
    #[doc = "Set the field `excluded_email_addresses`.\nContains the excluded email addresses. The value can be a particular\nemail address, a hostname to indicate all email addresses on that host or\na domain with a leading period (e.g. '.example.com') to indicate\nall email addresses in that domain."]
    pub fn set_excluded_email_addresses(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.excluded_email_addresses = Some(v.into());
        self
    }
    #[doc = "Set the field `excluded_ip_ranges`.\nContains the excluded IP ranges. For IPv4 addresses, the ranges\nare expressed using CIDR notation as specified in RFC 4632.\nFor IPv6 addresses, the ranges are expressed in similar encoding as IPv4\naddresses."]
    pub fn set_excluded_ip_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.excluded_ip_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `excluded_uris`.\nContains the excluded URIs that apply to the host part of the name.\nThe value can be a hostname or a domain with a\nleading period (like '.example.com')"]
    pub fn set_excluded_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.excluded_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `permitted_dns_names`.\nContains permitted DNS names. Any DNS name that can be\nconstructed by simply adding zero or more labels to\nthe left-hand side of the name satisfies the name constraint.\nFor example, 'example.com', 'www.example.com', 'www.sub.example.com'\nwould satisfy 'example.com' while 'example1.com' does not."]
    pub fn set_permitted_dns_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.permitted_dns_names = Some(v.into());
        self
    }
    #[doc = "Set the field `permitted_email_addresses`.\nContains the permitted email addresses. The value can be a particular\nemail address, a hostname to indicate all email addresses on that host or\na domain with a leading period (e.g. '.example.com') to indicate\nall email addresses in that domain."]
    pub fn set_permitted_email_addresses(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.permitted_email_addresses = Some(v.into());
        self
    }
    #[doc = "Set the field `permitted_ip_ranges`.\nContains the permitted IP ranges. For IPv4 addresses, the ranges\nare expressed using CIDR notation as specified in RFC 4632.\nFor IPv6 addresses, the ranges are expressed in similar encoding as IPv4\naddresses."]
    pub fn set_permitted_ip_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.permitted_ip_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `permitted_uris`.\nContains the permitted URIs that apply to the host part of the name.\nThe value can be a hostname or a domain with a\nleading period (like '.example.com')"]
    pub fn set_permitted_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.permitted_uris = Some(v.into());
        self
    }
}
impl ToListMappable for PrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsEl {
    type O = BlockAssignable<PrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsEl {
    #[doc = "Indicates whether or not the name constraints are marked critical."]
    pub critical: PrimField<bool>,
}
impl BuildPrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsEl {
    pub fn build(self) -> PrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsEl {
        PrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsEl {
            critical: self.critical,
            excluded_dns_names: core::default::Default::default(),
            excluded_email_addresses: core::default::Default::default(),
            excluded_ip_ranges: core::default::Default::default(),
            excluded_uris: core::default::Default::default(),
            permitted_dns_names: core::default::Default::default(),
            permitted_email_addresses: core::default::Default::default(),
            permitted_ip_ranges: core::default::Default::default(),
            permitted_uris: core::default::Default::default(),
        }
    }
}
pub struct PrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsElRef {
        PrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `critical` after provisioning.\nIndicates whether or not the name constraints are marked critical."]
    pub fn critical(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.critical", self.base))
    }
    #[doc = "Get a reference to the value of field `excluded_dns_names` after provisioning.\nContains excluded DNS names. Any DNS name that can be\nconstructed by simply adding zero or more labels to\nthe left-hand side of the name satisfies the name constraint.\nFor example, 'example.com', 'www.example.com', 'www.sub.example.com'\nwould satisfy 'example.com' while 'example1.com' does not."]
    pub fn excluded_dns_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.excluded_dns_names", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `excluded_email_addresses` after provisioning.\nContains the excluded email addresses. The value can be a particular\nemail address, a hostname to indicate all email addresses on that host or\na domain with a leading period (e.g. '.example.com') to indicate\nall email addresses in that domain."]
    pub fn excluded_email_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.excluded_email_addresses", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `excluded_ip_ranges` after provisioning.\nContains the excluded IP ranges. For IPv4 addresses, the ranges\nare expressed using CIDR notation as specified in RFC 4632.\nFor IPv6 addresses, the ranges are expressed in similar encoding as IPv4\naddresses."]
    pub fn excluded_ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.excluded_ip_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `excluded_uris` after provisioning.\nContains the excluded URIs that apply to the host part of the name.\nThe value can be a hostname or a domain with a\nleading period (like '.example.com')"]
    pub fn excluded_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.excluded_uris", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `permitted_dns_names` after provisioning.\nContains permitted DNS names. Any DNS name that can be\nconstructed by simply adding zero or more labels to\nthe left-hand side of the name satisfies the name constraint.\nFor example, 'example.com', 'www.example.com', 'www.sub.example.com'\nwould satisfy 'example.com' while 'example1.com' does not."]
    pub fn permitted_dns_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.permitted_dns_names", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `permitted_email_addresses` after provisioning.\nContains the permitted email addresses. The value can be a particular\nemail address, a hostname to indicate all email addresses on that host or\na domain with a leading period (e.g. '.example.com') to indicate\nall email addresses in that domain."]
    pub fn permitted_email_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.permitted_email_addresses", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `permitted_ip_ranges` after provisioning.\nContains the permitted IP ranges. For IPv4 addresses, the ranges\nare expressed using CIDR notation as specified in RFC 4632.\nFor IPv6 addresses, the ranges are expressed in similar encoding as IPv4\naddresses."]
    pub fn permitted_ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.permitted_ip_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `permitted_uris` after provisioning.\nContains the permitted URIs that apply to the host part of the name.\nThe value can be a hostname or a domain with a\nleading period (like '.example.com')"]
    pub fn permitted_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.permitted_uris", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsEl {
    object_id_path: ListField<PrimField<f64>>,
}
impl PrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsEl {}
impl ToListMappable for PrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsEl {
    type O = BlockAssignable<PrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsEl {
    #[doc = "An ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages."]
    pub object_id_path: ListField<PrimField<f64>>,
}
impl BuildPrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsEl {
    pub fn build(self) -> PrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsEl {
        PrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsEl {
            object_id_path: self.object_id_path,
        }
    }
}
pub struct PrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsElRef {
        PrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `object_id_path` after provisioning.\nAn ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages."]
    pub fn object_id_path(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.object_id_path", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct PrivatecaCertificateAuthorityConfigElX509ConfigElDynamic {
    additional_extensions: Option<
        DynamicBlock<PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsEl>,
    >,
    ca_options: Option<DynamicBlock<PrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsEl>>,
    key_usage: Option<DynamicBlock<PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageEl>>,
    name_constraints:
        Option<DynamicBlock<PrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsEl>>,
    policy_ids: Option<DynamicBlock<PrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsEl>>,
}
#[derive(Serialize)]
pub struct PrivatecaCertificateAuthorityConfigElX509ConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aia_ocsp_servers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_extensions:
        Option<Vec<PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_options: Option<Vec<PrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_usage: Option<Vec<PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_constraints:
        Option<Vec<PrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_ids: Option<Vec<PrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsEl>>,
    dynamic: PrivatecaCertificateAuthorityConfigElX509ConfigElDynamic,
}
impl PrivatecaCertificateAuthorityConfigElX509ConfigEl {
    #[doc = "Set the field `aia_ocsp_servers`.\nDescribes Online Certificate Status Protocol (OCSP) endpoint addresses that appear in the\n\"Authority Information Access\" extension in the certificate."]
    pub fn set_aia_ocsp_servers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.aia_ocsp_servers = Some(v.into());
        self
    }
    #[doc = "Set the field `additional_extensions`.\n"]
    pub fn set_additional_extensions(
        mut self,
        v: impl Into<
            BlockAssignable<
                PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.additional_extensions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_extensions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ca_options`.\n"]
    pub fn set_ca_options(
        mut self,
        v: impl Into<BlockAssignable<PrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ca_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ca_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `key_usage`.\n"]
    pub fn set_key_usage(
        mut self,
        v: impl Into<BlockAssignable<PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.key_usage = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.key_usage = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `name_constraints`.\n"]
    pub fn set_name_constraints(
        mut self,
        v: impl Into<
            BlockAssignable<PrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.name_constraints = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.name_constraints = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `policy_ids`.\n"]
    pub fn set_policy_ids(
        mut self,
        v: impl Into<BlockAssignable<PrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.policy_ids = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.policy_ids = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PrivatecaCertificateAuthorityConfigElX509ConfigEl {
    type O = BlockAssignable<PrivatecaCertificateAuthorityConfigElX509ConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateAuthorityConfigElX509ConfigEl {}
impl BuildPrivatecaCertificateAuthorityConfigElX509ConfigEl {
    pub fn build(self) -> PrivatecaCertificateAuthorityConfigElX509ConfigEl {
        PrivatecaCertificateAuthorityConfigElX509ConfigEl {
            aia_ocsp_servers: core::default::Default::default(),
            additional_extensions: core::default::Default::default(),
            ca_options: core::default::Default::default(),
            key_usage: core::default::Default::default(),
            name_constraints: core::default::Default::default(),
            policy_ids: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PrivatecaCertificateAuthorityConfigElX509ConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateAuthorityConfigElX509ConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateAuthorityConfigElX509ConfigElRef {
        PrivatecaCertificateAuthorityConfigElX509ConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateAuthorityConfigElX509ConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aia_ocsp_servers` after provisioning.\nDescribes Online Certificate Status Protocol (OCSP) endpoint addresses that appear in the\n\"Authority Information Access\" extension in the certificate."]
    pub fn aia_ocsp_servers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.aia_ocsp_servers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `additional_extensions` after provisioning.\n"]
    pub fn additional_extensions(
        &self,
    ) -> ListRef<PrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_extensions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ca_options` after provisioning.\n"]
    pub fn ca_options(
        &self,
    ) -> ListRef<PrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ca_options", self.base))
    }
    #[doc = "Get a reference to the value of field `key_usage` after provisioning.\n"]
    pub fn key_usage(
        &self,
    ) -> ListRef<PrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.key_usage", self.base))
    }
    #[doc = "Get a reference to the value of field `name_constraints` after provisioning.\n"]
    pub fn name_constraints(
        &self,
    ) -> ListRef<PrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.name_constraints", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `policy_ids` after provisioning.\n"]
    pub fn policy_ids(
        &self,
    ) -> ListRef<PrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.policy_ids", self.base))
    }
}
#[derive(Serialize, Default)]
struct PrivatecaCertificateAuthorityConfigElDynamic {
    subject_config: Option<DynamicBlock<PrivatecaCertificateAuthorityConfigElSubjectConfigEl>>,
    subject_key_id: Option<DynamicBlock<PrivatecaCertificateAuthorityConfigElSubjectKeyIdEl>>,
    x509_config: Option<DynamicBlock<PrivatecaCertificateAuthorityConfigElX509ConfigEl>>,
}
#[derive(Serialize)]
pub struct PrivatecaCertificateAuthorityConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_config: Option<Vec<PrivatecaCertificateAuthorityConfigElSubjectConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_key_id: Option<Vec<PrivatecaCertificateAuthorityConfigElSubjectKeyIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x509_config: Option<Vec<PrivatecaCertificateAuthorityConfigElX509ConfigEl>>,
    dynamic: PrivatecaCertificateAuthorityConfigElDynamic,
}
impl PrivatecaCertificateAuthorityConfigEl {
    #[doc = "Set the field `subject_config`.\n"]
    pub fn set_subject_config(
        mut self,
        v: impl Into<BlockAssignable<PrivatecaCertificateAuthorityConfigElSubjectConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.subject_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.subject_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `subject_key_id`.\n"]
    pub fn set_subject_key_id(
        mut self,
        v: impl Into<BlockAssignable<PrivatecaCertificateAuthorityConfigElSubjectKeyIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.subject_key_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.subject_key_id = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `x509_config`.\n"]
    pub fn set_x509_config(
        mut self,
        v: impl Into<BlockAssignable<PrivatecaCertificateAuthorityConfigElX509ConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.x509_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.x509_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PrivatecaCertificateAuthorityConfigEl {
    type O = BlockAssignable<PrivatecaCertificateAuthorityConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateAuthorityConfigEl {}
impl BuildPrivatecaCertificateAuthorityConfigEl {
    pub fn build(self) -> PrivatecaCertificateAuthorityConfigEl {
        PrivatecaCertificateAuthorityConfigEl {
            subject_config: core::default::Default::default(),
            subject_key_id: core::default::Default::default(),
            x509_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PrivatecaCertificateAuthorityConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateAuthorityConfigElRef {
    fn new(shared: StackShared, base: String) -> PrivatecaCertificateAuthorityConfigElRef {
        PrivatecaCertificateAuthorityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateAuthorityConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `subject_config` after provisioning.\n"]
    pub fn subject_config(
        &self,
    ) -> ListRef<PrivatecaCertificateAuthorityConfigElSubjectConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subject_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subject_key_id` after provisioning.\n"]
    pub fn subject_key_id(
        &self,
    ) -> ListRef<PrivatecaCertificateAuthorityConfigElSubjectKeyIdElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subject_key_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `x509_config` after provisioning.\n"]
    pub fn x509_config(&self) -> ListRef<PrivatecaCertificateAuthorityConfigElX509ConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.x509_config", self.base))
    }
}
#[derive(Serialize)]
pub struct PrivatecaCertificateAuthorityKeySpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_kms_key_version: Option<PrimField<String>>,
}
impl PrivatecaCertificateAuthorityKeySpecEl {
    #[doc = "Set the field `algorithm`.\nThe algorithm to use for creating a managed Cloud KMS key for a for a simplified\nexperience. All managed keys will be have their ProtectionLevel as HSM. Possible values: [\"SIGN_HASH_ALGORITHM_UNSPECIFIED\", \"RSA_PSS_2048_SHA256\", \"RSA_PSS_3072_SHA256\", \"RSA_PSS_4096_SHA256\", \"RSA_PKCS1_2048_SHA256\", \"RSA_PKCS1_3072_SHA256\", \"RSA_PKCS1_4096_SHA256\", \"EC_P256_SHA256\", \"EC_P384_SHA384\"]"]
    pub fn set_algorithm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.algorithm = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_kms_key_version`.\nThe resource name for an existing Cloud KMS CryptoKeyVersion in the format\n'projects/*/locations/*/keyRings/*/cryptoKeys/*/cryptoKeyVersions/*'."]
    pub fn set_cloud_kms_key_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cloud_kms_key_version = Some(v.into());
        self
    }
}
impl ToListMappable for PrivatecaCertificateAuthorityKeySpecEl {
    type O = BlockAssignable<PrivatecaCertificateAuthorityKeySpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateAuthorityKeySpecEl {}
impl BuildPrivatecaCertificateAuthorityKeySpecEl {
    pub fn build(self) -> PrivatecaCertificateAuthorityKeySpecEl {
        PrivatecaCertificateAuthorityKeySpecEl {
            algorithm: core::default::Default::default(),
            cloud_kms_key_version: core::default::Default::default(),
        }
    }
}
pub struct PrivatecaCertificateAuthorityKeySpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateAuthorityKeySpecElRef {
    fn new(shared: StackShared, base: String) -> PrivatecaCertificateAuthorityKeySpecElRef {
        PrivatecaCertificateAuthorityKeySpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateAuthorityKeySpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\nThe algorithm to use for creating a managed Cloud KMS key for a for a simplified\nexperience. All managed keys will be have their ProtectionLevel as HSM. Possible values: [\"SIGN_HASH_ALGORITHM_UNSPECIFIED\", \"RSA_PSS_2048_SHA256\", \"RSA_PSS_3072_SHA256\", \"RSA_PSS_4096_SHA256\", \"RSA_PKCS1_2048_SHA256\", \"RSA_PKCS1_3072_SHA256\", \"RSA_PKCS1_4096_SHA256\", \"EC_P256_SHA256\", \"EC_P384_SHA384\"]"]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }
    #[doc = "Get a reference to the value of field `cloud_kms_key_version` after provisioning.\nThe resource name for an existing Cloud KMS CryptoKeyVersion in the format\n'projects/*/locations/*/keyRings/*/cryptoKeys/*/cryptoKeyVersions/*'."]
    pub fn cloud_kms_key_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_kms_key_version", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pem_certificates: Option<ListField<PrimField<String>>>,
}
impl PrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainEl {
    #[doc = "Set the field `pem_certificates`.\nExpected to be in leaf-to-root order according to RFC 5246."]
    pub fn set_pem_certificates(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.pem_certificates = Some(v.into());
        self
    }
}
impl ToListMappable for PrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainEl {
    type O = BlockAssignable<PrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainEl {}
impl BuildPrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainEl {
    pub fn build(self) -> PrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainEl {
        PrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainEl {
            pem_certificates: core::default::Default::default(),
        }
    }
}
pub struct PrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainElRef {
        PrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pem_certificates` after provisioning.\nExpected to be in leaf-to-root order according to RFC 5246."]
    pub fn pem_certificates(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pem_certificates", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct PrivatecaCertificateAuthoritySubordinateConfigElDynamic {
    pem_issuer_chain:
        Option<DynamicBlock<PrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainEl>>,
}
#[derive(Serialize)]
pub struct PrivatecaCertificateAuthoritySubordinateConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_authority: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pem_issuer_chain: Option<Vec<PrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainEl>>,
    dynamic: PrivatecaCertificateAuthoritySubordinateConfigElDynamic,
}
impl PrivatecaCertificateAuthoritySubordinateConfigEl {
    #[doc = "Set the field `certificate_authority`.\nThis can refer to a CertificateAuthority that was used to create a\nsubordinate CertificateAuthority. This field is used for information\nand usability purposes only. The resource name is in the format\n'projects/*/locations/*/caPools/*/certificateAuthorities/*'."]
    pub fn set_certificate_authority(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_authority = Some(v.into());
        self
    }
    #[doc = "Set the field `pem_issuer_chain`.\n"]
    pub fn set_pem_issuer_chain(
        mut self,
        v: impl Into<BlockAssignable<PrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pem_issuer_chain = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pem_issuer_chain = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PrivatecaCertificateAuthoritySubordinateConfigEl {
    type O = BlockAssignable<PrivatecaCertificateAuthoritySubordinateConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateAuthoritySubordinateConfigEl {}
impl BuildPrivatecaCertificateAuthoritySubordinateConfigEl {
    pub fn build(self) -> PrivatecaCertificateAuthoritySubordinateConfigEl {
        PrivatecaCertificateAuthoritySubordinateConfigEl {
            certificate_authority: core::default::Default::default(),
            pem_issuer_chain: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PrivatecaCertificateAuthoritySubordinateConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateAuthoritySubordinateConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateAuthoritySubordinateConfigElRef {
        PrivatecaCertificateAuthoritySubordinateConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateAuthoritySubordinateConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificate_authority` after provisioning.\nThis can refer to a CertificateAuthority that was used to create a\nsubordinate CertificateAuthority. This field is used for information\nand usability purposes only. The resource name is in the format\n'projects/*/locations/*/caPools/*/certificateAuthorities/*'."]
    pub fn certificate_authority(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_authority", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pem_issuer_chain` after provisioning.\n"]
    pub fn pem_issuer_chain(
        &self,
    ) -> ListRef<PrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pem_issuer_chain", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrivatecaCertificateAuthorityTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl PrivatecaCertificateAuthorityTimeoutsEl {
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
impl ToListMappable for PrivatecaCertificateAuthorityTimeoutsEl {
    type O = BlockAssignable<PrivatecaCertificateAuthorityTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateAuthorityTimeoutsEl {}
impl BuildPrivatecaCertificateAuthorityTimeoutsEl {
    pub fn build(self) -> PrivatecaCertificateAuthorityTimeoutsEl {
        PrivatecaCertificateAuthorityTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct PrivatecaCertificateAuthorityTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateAuthorityTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> PrivatecaCertificateAuthorityTimeoutsElRef {
        PrivatecaCertificateAuthorityTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateAuthorityTimeoutsElRef {
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
pub struct PrivatecaCertificateAuthorityUserDefinedAccessUrlsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aia_issuing_certificate_urls: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    crl_access_urls: Option<ListField<PrimField<String>>>,
}
impl PrivatecaCertificateAuthorityUserDefinedAccessUrlsEl {
    #[doc = "Set the field `aia_issuing_certificate_urls`.\nA list of URLs where this CertificateAuthority's CA certificate is published that is specified by users."]
    pub fn set_aia_issuing_certificate_urls(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.aia_issuing_certificate_urls = Some(v.into());
        self
    }
    #[doc = "Set the field `crl_access_urls`.\nA list of URLs where this CertificateAuthority's CRLs are published that is specified by users."]
    pub fn set_crl_access_urls(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.crl_access_urls = Some(v.into());
        self
    }
}
impl ToListMappable for PrivatecaCertificateAuthorityUserDefinedAccessUrlsEl {
    type O = BlockAssignable<PrivatecaCertificateAuthorityUserDefinedAccessUrlsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateAuthorityUserDefinedAccessUrlsEl {}
impl BuildPrivatecaCertificateAuthorityUserDefinedAccessUrlsEl {
    pub fn build(self) -> PrivatecaCertificateAuthorityUserDefinedAccessUrlsEl {
        PrivatecaCertificateAuthorityUserDefinedAccessUrlsEl {
            aia_issuing_certificate_urls: core::default::Default::default(),
            crl_access_urls: core::default::Default::default(),
        }
    }
}
pub struct PrivatecaCertificateAuthorityUserDefinedAccessUrlsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateAuthorityUserDefinedAccessUrlsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateAuthorityUserDefinedAccessUrlsElRef {
        PrivatecaCertificateAuthorityUserDefinedAccessUrlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateAuthorityUserDefinedAccessUrlsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aia_issuing_certificate_urls` after provisioning.\nA list of URLs where this CertificateAuthority's CA certificate is published that is specified by users."]
    pub fn aia_issuing_certificate_urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.aia_issuing_certificate_urls", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `crl_access_urls` after provisioning.\nA list of URLs where this CertificateAuthority's CRLs are published that is specified by users."]
    pub fn crl_access_urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.crl_access_urls", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct PrivatecaCertificateAuthorityDynamic {
    config: Option<DynamicBlock<PrivatecaCertificateAuthorityConfigEl>>,
    key_spec: Option<DynamicBlock<PrivatecaCertificateAuthorityKeySpecEl>>,
    subordinate_config: Option<DynamicBlock<PrivatecaCertificateAuthoritySubordinateConfigEl>>,
    user_defined_access_urls:
        Option<DynamicBlock<PrivatecaCertificateAuthorityUserDefinedAccessUrlsEl>>,
}
