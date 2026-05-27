use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct IamWorkloadIdentityPoolData {
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
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    workload_identity_pool_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attestation_rules: Option<Vec<IamWorkloadIdentityPoolAttestationRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_certificate_issuance_config:
        Option<Vec<IamWorkloadIdentityPoolInlineCertificateIssuanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_trust_config: Option<Vec<IamWorkloadIdentityPoolInlineTrustConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<IamWorkloadIdentityPoolTimeoutsEl>,
    dynamic: IamWorkloadIdentityPoolDynamic,
}
struct IamWorkloadIdentityPool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IamWorkloadIdentityPoolData>,
}
#[derive(Clone)]
pub struct IamWorkloadIdentityPool(Rc<IamWorkloadIdentityPool_>);
impl IamWorkloadIdentityPool {
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
    #[doc = "Set the field `description`.\nA description of the pool. Cannot exceed 256 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nWhether the pool is disabled. You cannot use a disabled pool to exchange tokens, or use\nexisting tokens to access resources. If the pool is re-enabled, existing tokens grant\naccess again."]
    pub fn set_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nA display name for the pool. Cannot exceed 32 characters."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `mode`.\nThe mode for the pool is operating in. Pools with an unspecified mode will operate as if they\nare in 'FEDERATION_ONLY' mode.\n\n\n~> **Note** This field cannot be changed after the Workload Identity Pool is created. While\n'terraform plan' may show an update if you change this field's value, 'terraform apply'\n**will fail with an API error** (such as 'Error 400: Attempted to update an immutable field.').\nTo specify a different 'mode', please create a new Workload Identity Pool resource.\n\n* 'FEDERATION_ONLY': Pools can only be used for federating external workload identities into\nGoogle Cloud. Unless otherwise noted, no structure or format constraints are applied to\nworkload identities in a 'FEDERATION_ONLY' mode pool, and you may not create any resources\nwithin the pool besides providers.\n* 'TRUST_DOMAIN': Pools can be used to assign identities to Google Cloud workloads. All\nidentities within a 'TRUST_DOMAIN' mode pool must consist of a single namespace and individual\nworkload identifier. The subject identifier for all identities must conform to the following\nformat: 'ns/<namespace>/sa/<workload_identifier>'.\n'google_iam_workload_identity_pool_provider's cannot be created within 'TRUST_DOMAIN'\nmode pools.\n* 'SYSTEM_TRUST_DOMAIN': Pools are managed by Google Cloud services. Neither\n'google_iam_workload_identity_pool_namespace's nor 'google_iam_workload_identity_pool_provider's\ncan be created within 'SYSTEM_TRUST_DOMAIN' mode pools. All identities within a\n'SYSTEM_TRUST_DOMAIN' mode pool are in one of the following formats:\n\n    * 'spiffe://<trust-domain>/ns/<kubernetes-namespace>/sa/<kubernetes-service-account>'\n    * 'spiffe://<trust-domain>/resources/<resource-scope>/<resource-name>' Possible values: [\"FEDERATION_ONLY\", \"TRUST_DOMAIN\", \"SYSTEM_TRUST_DOMAIN\"]"]
    pub fn set_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().mode = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `attestation_rules`.\n"]
    pub fn set_attestation_rules(
        self,
        v: impl Into<BlockAssignable<IamWorkloadIdentityPoolAttestationRulesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attestation_rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attestation_rules = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `inline_certificate_issuance_config`.\n"]
    pub fn set_inline_certificate_issuance_config(
        self,
        v: impl Into<BlockAssignable<IamWorkloadIdentityPoolInlineCertificateIssuanceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().inline_certificate_issuance_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .inline_certificate_issuance_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `inline_trust_config`.\n"]
    pub fn set_inline_trust_config(
        self,
        v: impl Into<BlockAssignable<IamWorkloadIdentityPoolInlineTrustConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().inline_trust_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.inline_trust_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<IamWorkloadIdentityPoolTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `inline_certificate_issuance_config` after provisioning.\n"]
    pub fn inline_certificate_issuance_config(
        &self,
    ) -> ListRef<IamWorkloadIdentityPoolInlineCertificateIssuanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inline_certificate_issuance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `inline_trust_config` after provisioning.\n"]
    pub fn inline_trust_config(&self) -> ListRef<IamWorkloadIdentityPoolInlineTrustConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inline_trust_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamWorkloadIdentityPoolTimeoutsElRef {
        IamWorkloadIdentityPoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for IamWorkloadIdentityPool {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for IamWorkloadIdentityPool {}
impl ToListMappable for IamWorkloadIdentityPool {
    type O = ListRef<IamWorkloadIdentityPoolRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for IamWorkloadIdentityPool_ {
    fn extract_resource_type(&self) -> String {
        "google_iam_workload_identity_pool".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildIamWorkloadIdentityPool {
    pub tf_id: String,
    #[doc = "The ID to use for the pool, which becomes the final component of the resource name. This\nvalue should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub workload_identity_pool_id: PrimField<String>,
}
impl BuildIamWorkloadIdentityPool {
    pub fn build(self, stack: &mut Stack) -> IamWorkloadIdentityPool {
        let out = IamWorkloadIdentityPool(Rc::new(IamWorkloadIdentityPool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IamWorkloadIdentityPoolData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                disabled: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                mode: core::default::Default::default(),
                project: core::default::Default::default(),
                workload_identity_pool_id: self.workload_identity_pool_id,
                attestation_rules: core::default::Default::default(),
                inline_certificate_issuance_config: core::default::Default::default(),
                inline_trust_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct IamWorkloadIdentityPoolRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkloadIdentityPoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl IamWorkloadIdentityPoolRef {
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
    #[doc = "Get a reference to the value of field `inline_certificate_issuance_config` after provisioning.\n"]
    pub fn inline_certificate_issuance_config(
        &self,
    ) -> ListRef<IamWorkloadIdentityPoolInlineCertificateIssuanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inline_certificate_issuance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `inline_trust_config` after provisioning.\n"]
    pub fn inline_trust_config(&self) -> ListRef<IamWorkloadIdentityPoolInlineTrustConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inline_trust_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamWorkloadIdentityPoolTimeoutsElRef {
        IamWorkloadIdentityPoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct IamWorkloadIdentityPoolAttestationRulesEl {
    google_cloud_resource: PrimField<String>,
}
impl IamWorkloadIdentityPoolAttestationRulesEl {}
impl ToListMappable for IamWorkloadIdentityPoolAttestationRulesEl {
    type O = BlockAssignable<IamWorkloadIdentityPoolAttestationRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamWorkloadIdentityPoolAttestationRulesEl {
    #[doc = "A single workload operating on Google Cloud. For example:\n'//run.googleapis.com/projects/123/type/Service/*'."]
    pub google_cloud_resource: PrimField<String>,
}
impl BuildIamWorkloadIdentityPoolAttestationRulesEl {
    pub fn build(self) -> IamWorkloadIdentityPoolAttestationRulesEl {
        IamWorkloadIdentityPoolAttestationRulesEl {
            google_cloud_resource: self.google_cloud_resource,
        }
    }
}
pub struct IamWorkloadIdentityPoolAttestationRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkloadIdentityPoolAttestationRulesElRef {
    fn new(shared: StackShared, base: String) -> IamWorkloadIdentityPoolAttestationRulesElRef {
        IamWorkloadIdentityPoolAttestationRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamWorkloadIdentityPoolAttestationRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `google_cloud_resource` after provisioning.\nA single workload operating on Google Cloud. For example:\n'//run.googleapis.com/projects/123/type/Service/*'."]
    pub fn google_cloud_resource(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.google_cloud_resource", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IamWorkloadIdentityPoolInlineCertificateIssuanceConfigEl {
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
impl IamWorkloadIdentityPoolInlineCertificateIssuanceConfigEl {
    #[doc = "Set the field `ca_pools`.\nA required mapping of a cloud region to the CA pool resource located in that region used\nfor certificate issuance, adhering to these constraints:\n\n* **Key format:** A supported cloud region name equivalent to the location identifier in\nthe corresponding map entry's value.\n* **Value format:** A valid CA pool resource path format like:\n'projects/{project}/locations/{location}/caPools/{ca_pool}'\n* **Region Matching:** Workloads are ONLY issued certificates from CA pools within the\nsame region. Also the CA pool region (in value) must match the workload's region (key)."]
    pub fn set_ca_pools(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.ca_pools = Some(v.into());
        self
    }
    #[doc = "Set the field `key_algorithm`.\nKey algorithm to use when generating the key pair. This key pair will be used to create\nthe certificate. If unspecified, this will default to 'ECDSA_P256'.\n\n* 'RSA_2048': Specifies RSA with a 2048-bit modulus.\n* 'RSA_3072': Specifies RSA with a 3072-bit modulus.\n* 'RSA_4096': Specifies RSA with a 4096-bit modulus.\n* 'ECDSA_P256': Specifies ECDSA with curve P256.\n* 'ECDSA_P384': Specifies ECDSA with curve P384. Possible values: [\"RSA_2048\", \"RSA_3072\", \"RSA_4096\", \"ECDSA_P256\", \"ECDSA_P384\"]"]
    pub fn set_key_algorithm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_algorithm = Some(v.into());
        self
    }
    #[doc = "Set the field `lifetime`.\nLifetime of the workload certificates issued by the CA pool in seconds. Must be between\n'86400s' (24 hours) to '2592000s' (30 days), ends in the suffix \"'s'\" (indicating seconds)\nand is preceded by the number of seconds. If unspecified, this will be defaulted to\n'86400s' (24 hours)."]
    pub fn set_lifetime(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifetime = Some(v.into());
        self
    }
    #[doc = "Set the field `rotation_window_percentage`.\nRotation window percentage indicating when certificate rotation should be initiated based\non remaining lifetime. Must be between '50' - '80'. If unspecified, this will be defaulted\nto '50'."]
    pub fn set_rotation_window_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.rotation_window_percentage = Some(v.into());
        self
    }
    #[doc = "Set the field `use_default_shared_ca`.\nIf set to true, the trust domain will utilize the GCP-provisioned default CA. A default\nCA in the same region as the workload will be selected to issue the certificate. Enabling\nthis will clear any existing 'ca_pools' configuration to provision the certificates.\n\n\n~> **Note** This field is mutually exclusive with 'ca_pools'. If this flag is enabled,\ncertificates will be automatically provisioned from the default shared CAs. This flag should\nnot be set if you want to use your own CA pools to provision the certificates."]
    pub fn set_use_default_shared_ca(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_default_shared_ca = Some(v.into());
        self
    }
}
impl ToListMappable for IamWorkloadIdentityPoolInlineCertificateIssuanceConfigEl {
    type O = BlockAssignable<IamWorkloadIdentityPoolInlineCertificateIssuanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamWorkloadIdentityPoolInlineCertificateIssuanceConfigEl {}
impl BuildIamWorkloadIdentityPoolInlineCertificateIssuanceConfigEl {
    pub fn build(self) -> IamWorkloadIdentityPoolInlineCertificateIssuanceConfigEl {
        IamWorkloadIdentityPoolInlineCertificateIssuanceConfigEl {
            ca_pools: core::default::Default::default(),
            key_algorithm: core::default::Default::default(),
            lifetime: core::default::Default::default(),
            rotation_window_percentage: core::default::Default::default(),
            use_default_shared_ca: core::default::Default::default(),
        }
    }
}
pub struct IamWorkloadIdentityPoolInlineCertificateIssuanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkloadIdentityPoolInlineCertificateIssuanceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IamWorkloadIdentityPoolInlineCertificateIssuanceConfigElRef {
        IamWorkloadIdentityPoolInlineCertificateIssuanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamWorkloadIdentityPoolInlineCertificateIssuanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ca_pools` after provisioning.\nA required mapping of a cloud region to the CA pool resource located in that region used\nfor certificate issuance, adhering to these constraints:\n\n* **Key format:** A supported cloud region name equivalent to the location identifier in\nthe corresponding map entry's value.\n* **Value format:** A valid CA pool resource path format like:\n'projects/{project}/locations/{location}/caPools/{ca_pool}'\n* **Region Matching:** Workloads are ONLY issued certificates from CA pools within the\nsame region. Also the CA pool region (in value) must match the workload's region (key)."]
    pub fn ca_pools(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.ca_pools", self.base))
    }
    #[doc = "Get a reference to the value of field `key_algorithm` after provisioning.\nKey algorithm to use when generating the key pair. This key pair will be used to create\nthe certificate. If unspecified, this will default to 'ECDSA_P256'.\n\n* 'RSA_2048': Specifies RSA with a 2048-bit modulus.\n* 'RSA_3072': Specifies RSA with a 3072-bit modulus.\n* 'RSA_4096': Specifies RSA with a 4096-bit modulus.\n* 'ECDSA_P256': Specifies ECDSA with curve P256.\n* 'ECDSA_P384': Specifies ECDSA with curve P384. Possible values: [\"RSA_2048\", \"RSA_3072\", \"RSA_4096\", \"ECDSA_P256\", \"ECDSA_P384\"]"]
    pub fn key_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_algorithm", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lifetime` after provisioning.\nLifetime of the workload certificates issued by the CA pool in seconds. Must be between\n'86400s' (24 hours) to '2592000s' (30 days), ends in the suffix \"'s'\" (indicating seconds)\nand is preceded by the number of seconds. If unspecified, this will be defaulted to\n'86400s' (24 hours)."]
    pub fn lifetime(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifetime", self.base))
    }
    #[doc = "Get a reference to the value of field `rotation_window_percentage` after provisioning.\nRotation window percentage indicating when certificate rotation should be initiated based\non remaining lifetime. Must be between '50' - '80'. If unspecified, this will be defaulted\nto '50'."]
    pub fn rotation_window_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rotation_window_percentage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `use_default_shared_ca` after provisioning.\nIf set to true, the trust domain will utilize the GCP-provisioned default CA. A default\nCA in the same region as the workload will be selected to issue the certificate. Enabling\nthis will clear any existing 'ca_pools' configuration to provision the certificates.\n\n\n~> **Note** This field is mutually exclusive with 'ca_pools'. If this flag is enabled,\ncertificates will be automatically provisioned from the default shared CAs. This flag should\nnot be set if you want to use your own CA pools to provision the certificates."]
    pub fn use_default_shared_ca(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_default_shared_ca", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsEl {
    pem_certificate: PrimField<String>,
}
impl IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsEl {}
impl ToListMappable
    for IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsEl
{
    type O = BlockAssignable<
        IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsEl {
    #[doc = "PEM certificate of the PKI used for validation. Must only contain one ca\ncertificate(either root or intermediate cert)."]
    pub pem_certificate: PrimField<String>,
}
impl BuildIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsEl {
    pub fn build(
        self,
    ) -> IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsEl {
        IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsEl {
            pem_certificate: self.pem_certificate,
        }
    }
}
pub struct IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsElRef {
        IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pem_certificate` after provisioning.\nPEM certificate of the PKI used for validation. Must only contain one ca\ncertificate(either root or intermediate cert)."]
    pub fn pem_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pem_certificate", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElDynamic {
    trust_anchors: Option<
        DynamicBlock<
            IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    trust_default_shared_ca: Option<PrimField<bool>>,
    trust_domain: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trust_anchors: Option<
        Vec<IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsEl>,
    >,
    dynamic: IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElDynamic,
}
impl IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesEl {
    #[doc = "Set the field `trust_default_shared_ca`.\nIf set to True, the trust bundle will include the private ca managed identity regional root\npublic certificates.\n\n\n~> **Note** 'trust_default_shared_ca' is only supported for managed identity trust domain\nresource."]
    pub fn set_trust_default_shared_ca(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.trust_default_shared_ca = Some(v.into());
        self
    }
    #[doc = "Set the field `trust_anchors`.\n"]
    pub fn set_trust_anchors(
        mut self,
        v: impl Into<
            BlockAssignable<
                IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.trust_anchors = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.trust_anchors = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesEl {
    type O = BlockAssignable<IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesEl {
    #[doc = ""]
    pub trust_domain: PrimField<String>,
}
impl BuildIamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesEl {
    pub fn build(self) -> IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesEl {
        IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesEl {
            trust_default_shared_ca: core::default::Default::default(),
            trust_domain: self.trust_domain,
            trust_anchors: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElRef {
        IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `trust_default_shared_ca` after provisioning.\nIf set to True, the trust bundle will include the private ca managed identity regional root\npublic certificates.\n\n\n~> **Note** 'trust_default_shared_ca' is only supported for managed identity trust domain\nresource."]
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
    #[doc = "Get a reference to the value of field `trust_anchors` after provisioning.\n"]
    pub fn trust_anchors(
        &self,
    ) -> ListRef<IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesElTrustAnchorsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trust_anchors", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct IamWorkloadIdentityPoolInlineTrustConfigElDynamic {
    additional_trust_bundles:
        Option<DynamicBlock<IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesEl>>,
}
#[derive(Serialize)]
pub struct IamWorkloadIdentityPoolInlineTrustConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_trust_bundles:
        Option<Vec<IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesEl>>,
    dynamic: IamWorkloadIdentityPoolInlineTrustConfigElDynamic,
}
impl IamWorkloadIdentityPoolInlineTrustConfigEl {
    #[doc = "Set the field `additional_trust_bundles`.\n"]
    pub fn set_additional_trust_bundles(
        mut self,
        v: impl Into<
            BlockAssignable<IamWorkloadIdentityPoolInlineTrustConfigElAdditionalTrustBundlesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.additional_trust_bundles = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_trust_bundles = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IamWorkloadIdentityPoolInlineTrustConfigEl {
    type O = BlockAssignable<IamWorkloadIdentityPoolInlineTrustConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamWorkloadIdentityPoolInlineTrustConfigEl {}
impl BuildIamWorkloadIdentityPoolInlineTrustConfigEl {
    pub fn build(self) -> IamWorkloadIdentityPoolInlineTrustConfigEl {
        IamWorkloadIdentityPoolInlineTrustConfigEl {
            additional_trust_bundles: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IamWorkloadIdentityPoolInlineTrustConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkloadIdentityPoolInlineTrustConfigElRef {
    fn new(shared: StackShared, base: String) -> IamWorkloadIdentityPoolInlineTrustConfigElRef {
        IamWorkloadIdentityPoolInlineTrustConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamWorkloadIdentityPoolInlineTrustConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct IamWorkloadIdentityPoolTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl IamWorkloadIdentityPoolTimeoutsEl {
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
impl ToListMappable for IamWorkloadIdentityPoolTimeoutsEl {
    type O = BlockAssignable<IamWorkloadIdentityPoolTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamWorkloadIdentityPoolTimeoutsEl {}
impl BuildIamWorkloadIdentityPoolTimeoutsEl {
    pub fn build(self) -> IamWorkloadIdentityPoolTimeoutsEl {
        IamWorkloadIdentityPoolTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct IamWorkloadIdentityPoolTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkloadIdentityPoolTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> IamWorkloadIdentityPoolTimeoutsElRef {
        IamWorkloadIdentityPoolTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamWorkloadIdentityPoolTimeoutsElRef {
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
struct IamWorkloadIdentityPoolDynamic {
    attestation_rules: Option<DynamicBlock<IamWorkloadIdentityPoolAttestationRulesEl>>,
    inline_certificate_issuance_config:
        Option<DynamicBlock<IamWorkloadIdentityPoolInlineCertificateIssuanceConfigEl>>,
    inline_trust_config: Option<DynamicBlock<IamWorkloadIdentityPoolInlineTrustConfigEl>>,
}
