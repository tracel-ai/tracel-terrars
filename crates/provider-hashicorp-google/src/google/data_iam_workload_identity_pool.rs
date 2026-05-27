use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataIamWorkloadIdentityPoolData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    workload_identity_pool_id: PrimField<String>,
}
struct DataIamWorkloadIdentityPool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataIamWorkloadIdentityPoolData>,
}
#[derive(Clone)]
pub struct DataIamWorkloadIdentityPool(Rc<DataIamWorkloadIdentityPool_>);
impl DataIamWorkloadIdentityPool {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `attestation_rules` after provisioning.\nDefines which workloads can receive an identity within a pool. When an AttestationRule is\ndefined under a managed identity, matching workloads may receive that identity. A maximum of\n50 AttestationRules can be set."]
    pub fn attestation_rules(&self) -> SetRef<DataIamWorkloadIdentityPoolAttestationRulesElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.attestation_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the pool. Cannot exceed 256 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the pool is disabled. You cannot use a disabled pool to exchange tokens, or use\nexisting tokens to access resources. If the pool is re-enabled, existing tokens grant\naccess again."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA display name for the pool. Cannot exceed 32 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `inline_certificate_issuance_config` after provisioning.\nRepresents configuration for generating mutual TLS (mTLS) certificates for the identities\nwithin this pool. Defines the Certificate Authority (CA) pool resources and configurations\nrequired for issuance and rotation of mTLS workload certificates."]
    pub fn inline_certificate_issuance_config(
        &self,
    ) -> ListRef<DataIamWorkloadIdentityPoolInlineCertificateIssuanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inline_certificate_issuance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `inline_trust_config` after provisioning.\nRepresents config to add additional trusted trust domains. Defines configuration for extending\ntrust to additional trust domains. By establishing trust with another domain, the current\ndomain will recognize and accept certificates issued by entities within the trusted domains.\nNote that a trust domain automatically trusts itself, eliminating the need for explicit\nconfiguration."]
    pub fn inline_trust_config(
        &self,
    ) -> ListRef<DataIamWorkloadIdentityPoolInlineTrustConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inline_trust_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nThe mode for the pool is operating in. Pools with an unspecified mode will operate as if they\nare in 'FEDERATION_ONLY' mode.\n\n\n~> **Note** This field cannot be changed after the Workload Identity Pool is created. While\n'terraform plan' may show an update if you change this field's value, 'terraform apply'\n**will fail with an API error** (such as 'Error 400: Attempted to update an immutable field.').\nTo specify a different 'mode', please create a new Workload Identity Pool resource.\n\n* 'FEDERATION_ONLY': Pools can only be used for federating external workload identities into\nGoogle Cloud. Unless otherwise noted, no structure or format constraints are applied to\nworkload identities in a 'FEDERATION_ONLY' mode pool, and you may not create any resources\nwithin the pool besides providers.\n* 'TRUST_DOMAIN': Pools can be used to assign identities to Google Cloud workloads. All\nidentities within a 'TRUST_DOMAIN' mode pool must consist of a single namespace and individual\nworkload identifier. The subject identifier for all identities must conform to the following\nformat: 'ns/<namespace>/sa/<workload_identifier>'.\n'google_iam_workload_identity_pool_provider's cannot be created within 'TRUST_DOMAIN'\nmode pools.\n* 'SYSTEM_TRUST_DOMAIN': Pools are managed by Google Cloud services. Neither\n'google_iam_workload_identity_pool_namespace's nor 'google_iam_workload_identity_pool_provider's\ncan be created within 'SYSTEM_TRUST_DOMAIN' mode pools. All identities within a\n'SYSTEM_TRUST_DOMAIN' mode pool are in one of the following formats:\n\n    * 'spiffe://<trust-domain>/ns/<kubernetes-namespace>/sa/<kubernetes-service-account>'\n    * 'spiffe://<trust-domain>/resources/<resource-scope>/<resource-name>' Possible values: [\"FEDERATION_ONLY\", \"TRUST_DOMAIN\", \"SYSTEM_TRUST_DOMAIN\"]"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the pool as\n'projects/{project_number}/locations/global/workloadIdentityPools/{workload_identity_pool_id}'."]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the pool.\n* 'STATE_UNSPECIFIED': State unspecified.\n* 'ACTIVE': The pool is active, and may be used in Google Cloud policies.\n* 'DELETED': The pool is soft-deleted. Soft-deleted pools are permanently deleted after\n  approximately 30 days. You can restore a soft-deleted pool using\n  'UndeleteWorkloadIdentityPool'. You cannot reuse the ID of a soft-deleted pool until it is\n  permanently deleted. While a pool is deleted, you cannot use it to exchange tokens, or\n  use existing tokens to access resources. If the pool is undeleted, existing tokens grant\n  access again."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_identity_pool_id` after provisioning.\nThe ID to use for the pool, which becomes the final component of the resource name. This\nvalue should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn workload_identity_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workload_identity_pool_id", self.extract_ref()),
        )
    }
}
impl Referable for DataIamWorkloadIdentityPool {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataIamWorkloadIdentityPool {}
impl ToListMappable for DataIamWorkloadIdentityPool {
    type O = ListRef<DataIamWorkloadIdentityPoolRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataIamWorkloadIdentityPool_ {
    fn extract_datasource_type(&self) -> String {
        "google_iam_workload_identity_pool".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataIamWorkloadIdentityPool {
    pub tf_id: String,
    #[doc = "The ID to use for the pool, which becomes the final component of the resource name. This\nvalue should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub workload_identity_pool_id: PrimField<String>,
}
impl BuildDataIamWorkloadIdentityPool {
    pub fn build(self, stack: &mut Stack) -> DataIamWorkloadIdentityPool {
        let out = DataIamWorkloadIdentityPool(Rc::new(DataIamWorkloadIdentityPool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataIamWorkloadIdentityPoolData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                workload_identity_pool_id: self.workload_identity_pool_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataIamWorkloadIdentityPoolRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamWorkloadIdentityPoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataIamWorkloadIdentityPoolRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `attestation_rules` after provisioning.\nDefines which workloads can receive an identity within a pool. When an AttestationRule is\ndefined under a managed identity, matching workloads may receive that identity. A maximum of\n50 AttestationRules can be set."]
    pub fn attestation_rules(&self) -> SetRef<DataIamWorkloadIdentityPoolAttestationRulesElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.attestation_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the pool. Cannot exceed 256 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the pool is disabled. You cannot use a disabled pool to exchange tokens, or use\nexisting tokens to access resources. If the pool is re-enabled, existing tokens grant\naccess again."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA display name for the pool. Cannot exceed 32 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `inline_certificate_issuance_config` after provisioning.\nRepresents configuration for generating mutual TLS (mTLS) certificates for the identities\nwithin this pool. Defines the Certificate Authority (CA) pool resources and configurations\nrequired for issuance and rotation of mTLS workload certificates."]
    pub fn inline_certificate_issuance_config(
        &self,
    ) -> ListRef<DataIamWorkloadIdentityPoolInlineCertificateIssuanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inline_certificate_issuance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `inline_trust_config` after provisioning.\nRepresents config to add additional trusted trust domains. Defines configuration for extending\ntrust to additional trust domains. By establishing trust with another domain, the current\ndomain will recognize and accept certificates issued by entities within the trusted domains.\nNote that a trust domain automatically trusts itself, eliminating the need for explicit\nconfiguration."]
    pub fn inline_trust_config(
        &self,
    ) -> ListRef<DataIamWorkloadIdentityPoolInlineTrustConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inline_trust_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nThe mode for the pool is operating in. Pools with an unspecified mode will operate as if they\nare in 'FEDERATION_ONLY' mode.\n\n\n~> **Note** This field cannot be changed after the Workload Identity Pool is created. While\n'terraform plan' may show an update if you change this field's value, 'terraform apply'\n**will fail with an API error** (such as 'Error 400: Attempted to update an immutable field.').\nTo specify a different 'mode', please create a new Workload Identity Pool resource.\n\n* 'FEDERATION_ONLY': Pools can only be used for federating external workload identities into\nGoogle Cloud. Unless otherwise noted, no structure or format constraints are applied to\nworkload identities in a 'FEDERATION_ONLY' mode pool, and you may not create any resources\nwithin the pool besides providers.\n* 'TRUST_DOMAIN': Pools can be used to assign identities to Google Cloud workloads. All\nidentities within a 'TRUST_DOMAIN' mode pool must consist of a single namespace and individual\nworkload identifier. The subject identifier for all identities must conform to the following\nformat: 'ns/<namespace>/sa/<workload_identifier>'.\n'google_iam_workload_identity_pool_provider's cannot be created within 'TRUST_DOMAIN'\nmode pools.\n* 'SYSTEM_TRUST_DOMAIN': Pools are managed by Google Cloud services. Neither\n'google_iam_workload_identity_pool_namespace's nor 'google_iam_workload_identity_pool_provider's\ncan be created within 'SYSTEM_TRUST_DOMAIN' mode pools. All identities within a\n'SYSTEM_TRUST_DOMAIN' mode pool are in one of the following formats:\n\n    * 'spiffe://<trust-domain>/ns/<kubernetes-namespace>/sa/<kubernetes-service-account>'\n    * 'spiffe://<trust-domain>/resources/<resource-scope>/<resource-name>' Possible values: [\"FEDERATION_ONLY\", \"TRUST_DOMAIN\", \"SYSTEM_TRUST_DOMAIN\"]"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the pool as\n'projects/{project_number}/locations/global/workloadIdentityPools/{workload_identity_pool_id}'."]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the pool.\n* 'STATE_UNSPECIFIED': State unspecified.\n* 'ACTIVE': The pool is active, and may be used in Google Cloud policies.\n* 'DELETED': The pool is soft-deleted. Soft-deleted pools are permanently deleted after\n  approximately 30 days. You can restore a soft-deleted pool using\n  'UndeleteWorkloadIdentityPool'. You cannot reuse the ID of a soft-deleted pool until it is\n  permanently deleted. While a pool is deleted, you cannot use it to exchange tokens, or\n  use existing tokens to access resources. If the pool is undeleted, existing tokens grant\n  access again."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_identity_pool_id` after provisioning.\nThe ID to use for the pool, which becomes the final component of the resource name. This\nvalue should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn workload_identity_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workload_identity_pool_id", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataIamWorkloadIdentityPoolAttestationRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    google_cloud_resource: Option<PrimField<String>>,
}
impl DataIamWorkloadIdentityPoolAttestationRulesEl {
    #[doc = "Set the field `google_cloud_resource`.\n"]
    pub fn set_google_cloud_resource(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.google_cloud_resource = Some(v.into());
        self
    }
}
impl ToListMappable for DataIamWorkloadIdentityPoolAttestationRulesEl {
    type O = BlockAssignable<DataIamWorkloadIdentityPoolAttestationRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIamWorkloadIdentityPoolAttestationRulesEl {}
impl BuildDataIamWorkloadIdentityPoolAttestationRulesEl {
    pub fn build(self) -> DataIamWorkloadIdentityPoolAttestationRulesEl {
        DataIamWorkloadIdentityPoolAttestationRulesEl {
            google_cloud_resource: core::default::Default::default(),
        }
    }
}
pub struct DataIamWorkloadIdentityPoolAttestationRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamWorkloadIdentityPoolAttestationRulesElRef {
    fn new(shared: StackShared, base: String) -> DataIamWorkloadIdentityPoolAttestationRulesElRef {
        DataIamWorkloadIdentityPoolAttestationRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIamWorkloadIdentityPoolAttestationRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `google_cloud_resource` after provisioning.\n"]
    pub fn google_cloud_resource(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.google_cloud_resource", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataIamWorkloadIdentityPoolInlineCertificateIssuanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_pools: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifetime: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation_window_percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_default_shared_ca: Option<PrimField<bool>>,
}
impl DataIamWorkloadIdentityPoolInlineCertificateIssuanceConfigEl {
    #[doc = "Set the field `ca_pools`.\n"]
    pub fn set_ca_pools(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.ca_pools = Some(v.into());
        self
    }
    #[doc = "Set the field `key_algorithm`.\n"]
    pub fn set_key_algorithm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_algorithm = Some(v.into());
        self
    }
    #[doc = "Set the field `lifetime`.\n"]
    pub fn set_lifetime(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifetime = Some(v.into());
        self
    }
    #[doc = "Set the field `rotation_window_percentage`.\n"]
    pub fn set_rotation_window_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.rotation_window_percentage = Some(v.into());
        self
    }
    #[doc = "Set the field `use_default_shared_ca`.\n"]
    pub fn set_use_default_shared_ca(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_default_shared_ca = Some(v.into());
        self
    }
}
impl ToListMappable for DataIamWorkloadIdentityPoolInlineCertificateIssuanceConfigEl {
    type O = BlockAssignable<DataIamWorkloadIdentityPoolInlineCertificateIssuanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIamWorkloadIdentityPoolInlineCertificateIssuanceConfigEl {}
impl BuildDataIamWorkloadIdentityPoolInlineCertificateIssuanceConfigEl {
    pub fn build(self) -> DataIamWorkloadIdentityPoolInlineCertificateIssuanceConfigEl {
        DataIamWorkloadIdentityPoolInlineCertificateIssuanceConfigEl {
            ca_pools: core::default::Default::default(),
            key_algorithm: core::default::Default::default(),
            lifetime: core::default::Default::default(),
            rotation_window_percentage: core::default::Default::default(),
            use_default_shared_ca: core::default::Default::default(),
        }
    }
}
pub struct DataIamWorkloadIdentityPoolInlineCertificateIssuanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamWorkloadIdentityPoolInlineCertificateIssuanceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataIamWorkloadIdentityPoolInlineCertificateIssuanceConfigElRef {
        DataIamWorkloadIdentityPoolInlineCertificateIssuanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIamWorkloadIdentityPoolInlineCertificateIssuanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ca_pools` after provisioning.\n"]
    pub fn ca_pools(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.ca_pools", self.base))
    }
    #[doc = "Get a reference to the value of field `key_algorithm` after provisioning.\n"]
    pub fn key_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_algorithm", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lifetime` after provisioning.\n"]
    pub fn lifetime(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifetime", self.base))
    }
    #[doc = "Get a reference to the value of field `rotation_window_percentage` after provisioning.\n"]
    pub fn rotation_window_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rotation_window_percentage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `use_default_shared_ca` after provisioning.\n"]
    pub fn use_default_shared_ca(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_default_shared_ca", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pem_certificate: Option<PrimField<String>>,
}
impl DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsEl {
    #[doc = "Set the field `pem_certificate`.\n"]
    pub fn set_pem_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pem_certificate = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsEl
{
    type O = BlockAssignable<
        DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsEl
{}
impl BuildDataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsEl {
    pub fn build(
        self,
    ) -> DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsEl {
        DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsEl {
            pem_certificate: core::default::Default::default(),
        }
    }
}
pub struct DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsElRef
    {
        DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pem_certificate` after provisioning.\n"]
    pub fn pem_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pem_certificate", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    trust_anchors: Option<
        ListField<
            DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    trust_default_shared_ca: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trust_domain: Option<PrimField<String>>,
}
impl DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesEl {
    #[doc = "Set the field `trust_anchors`.\n"]
    pub fn set_trust_anchors(
        mut self,
        v : impl Into < ListField < DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsEl > >,
    ) -> Self {
        self.trust_anchors = Some(v.into());
        self
    }
    #[doc = "Set the field `trust_default_shared_ca`.\n"]
    pub fn set_trust_default_shared_ca(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.trust_default_shared_ca = Some(v.into());
        self
    }
    #[doc = "Set the field `trust_domain`.\n"]
    pub fn set_trust_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.trust_domain = Some(v.into());
        self
    }
}
impl ToListMappable for DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesEl {
    type O =
        BlockAssignable<DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesEl {}
impl BuildDataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesEl {
    pub fn build(self) -> DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesEl {
        DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesEl {
            trust_anchors: core::default::Default::default(),
            trust_default_shared_ca: core::default::Default::default(),
            trust_domain: core::default::Default::default(),
        }
    }
}
pub struct DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElRef {
        DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `trust_anchors` after provisioning.\n"]
    pub fn trust_anchors(
        &self,
    ) -> ListRef<
        DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trust_anchors", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `trust_default_shared_ca` after provisioning.\n"]
    pub fn trust_default_shared_ca(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trust_default_shared_ca", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `trust_domain` after provisioning.\n"]
    pub fn trust_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trust_domain", self.base))
    }
}
#[derive(Serialize)]
pub struct DataIamWorkloadIdentityPoolInlineTrustConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_trust_bundles:
        Option<SetField<DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesEl>>,
}
impl DataIamWorkloadIdentityPoolInlineTrustConfigEl {
    #[doc = "Set the field `additional_trust_bundles`.\n"]
    pub fn set_additional_trust_bundles(
        mut self,
        v: impl Into<SetField<DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesEl>>,
    ) -> Self {
        self.additional_trust_bundles = Some(v.into());
        self
    }
}
impl ToListMappable for DataIamWorkloadIdentityPoolInlineTrustConfigEl {
    type O = BlockAssignable<DataIamWorkloadIdentityPoolInlineTrustConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIamWorkloadIdentityPoolInlineTrustConfigEl {}
impl BuildDataIamWorkloadIdentityPoolInlineTrustConfigEl {
    pub fn build(self) -> DataIamWorkloadIdentityPoolInlineTrustConfigEl {
        DataIamWorkloadIdentityPoolInlineTrustConfigEl {
            additional_trust_bundles: core::default::Default::default(),
        }
    }
}
pub struct DataIamWorkloadIdentityPoolInlineTrustConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamWorkloadIdentityPoolInlineTrustConfigElRef {
    fn new(shared: StackShared, base: String) -> DataIamWorkloadIdentityPoolInlineTrustConfigElRef {
        DataIamWorkloadIdentityPoolInlineTrustConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIamWorkloadIdentityPoolInlineTrustConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `additional_trust_bundles` after provisioning.\n"]
    pub fn additional_trust_bundles(
        &self,
    ) -> SetRef<DataIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.additional_trust_bundles", self.base),
        )
    }
}
