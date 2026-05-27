use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct PrivatecaCaPoolData {
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
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    tier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_spec: Option<Vec<PrivatecaCaPoolEncryptionSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuance_policy: Option<Vec<PrivatecaCaPoolIssuancePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publishing_options: Option<Vec<PrivatecaCaPoolPublishingOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<PrivatecaCaPoolTimeoutsEl>,
    dynamic: PrivatecaCaPoolDynamic,
}
struct PrivatecaCaPool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<PrivatecaCaPoolData>,
}
#[derive(Clone)]
pub struct PrivatecaCaPool(Rc<PrivatecaCaPool_>);
impl PrivatecaCaPool {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels with user-defined metadata.\n\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\":\n\"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_spec`.\n"]
    pub fn set_encryption_spec(
        self,
        v: impl Into<BlockAssignable<PrivatecaCaPoolEncryptionSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `issuance_policy`.\n"]
    pub fn set_issuance_policy(
        self,
        v: impl Into<BlockAssignable<PrivatecaCaPoolIssuancePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().issuance_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.issuance_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `publishing_options`.\n"]
    pub fn set_publishing_options(
        self,
        v: impl Into<BlockAssignable<PrivatecaCaPoolPublishingOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().publishing_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.publishing_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<PrivatecaCaPoolTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels with user-defined metadata.\n\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\":\n\"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation of the CaPool. A full list of valid locations can be found by\nrunning 'gcloud privateca locations list'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name for this CaPool."]
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
    #[doc = "Get a reference to the value of field `tier` after provisioning.\nThe Tier of this CaPool. Possible values: [\"ENTERPRISE\", \"DEVOPS\"]"]
    pub fn tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> ListRef<PrivatecaCaPoolEncryptionSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `issuance_policy` after provisioning.\n"]
    pub fn issuance_policy(&self) -> ListRef<PrivatecaCaPoolIssuancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.issuance_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `publishing_options` after provisioning.\n"]
    pub fn publishing_options(&self) -> ListRef<PrivatecaCaPoolPublishingOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.publishing_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrivatecaCaPoolTimeoutsElRef {
        PrivatecaCaPoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for PrivatecaCaPool {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for PrivatecaCaPool {}
impl ToListMappable for PrivatecaCaPool {
    type O = ListRef<PrivatecaCaPoolRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for PrivatecaCaPool_ {
    fn extract_resource_type(&self) -> String {
        "google_privateca_ca_pool".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildPrivatecaCaPool {
    pub tf_id: String,
    #[doc = "Location of the CaPool. A full list of valid locations can be found by\nrunning 'gcloud privateca locations list'."]
    pub location: PrimField<String>,
    #[doc = "The name for this CaPool."]
    pub name: PrimField<String>,
    #[doc = "The Tier of this CaPool. Possible values: [\"ENTERPRISE\", \"DEVOPS\"]"]
    pub tier: PrimField<String>,
}
impl BuildPrivatecaCaPool {
    pub fn build(self, stack: &mut Stack) -> PrivatecaCaPool {
        let out = PrivatecaCaPool(Rc::new(PrivatecaCaPool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(PrivatecaCaPoolData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                tier: self.tier,
                encryption_spec: core::default::Default::default(),
                issuance_policy: core::default::Default::default(),
                publishing_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct PrivatecaCaPoolRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCaPoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl PrivatecaCaPoolRef {
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels with user-defined metadata.\n\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\":\n\"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation of the CaPool. A full list of valid locations can be found by\nrunning 'gcloud privateca locations list'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name for this CaPool."]
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
    #[doc = "Get a reference to the value of field `tier` after provisioning.\nThe Tier of this CaPool. Possible values: [\"ENTERPRISE\", \"DEVOPS\"]"]
    pub fn tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> ListRef<PrivatecaCaPoolEncryptionSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `issuance_policy` after provisioning.\n"]
    pub fn issuance_policy(&self) -> ListRef<PrivatecaCaPoolIssuancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.issuance_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `publishing_options` after provisioning.\n"]
    pub fn publishing_options(&self) -> ListRef<PrivatecaCaPoolPublishingOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.publishing_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrivatecaCaPoolTimeoutsElRef {
        PrivatecaCaPoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct PrivatecaCaPoolEncryptionSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_kms_key: Option<PrimField<String>>,
}
impl PrivatecaCaPoolEncryptionSpecEl {
    #[doc = "Set the field `cloud_kms_key`.\nThe resource name for an existing Cloud KMS key in the format\n'projects/*/locations/*/keyRings/*/cryptoKeys/*'."]
    pub fn set_cloud_kms_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cloud_kms_key = Some(v.into());
        self
    }
}
impl ToListMappable for PrivatecaCaPoolEncryptionSpecEl {
    type O = BlockAssignable<PrivatecaCaPoolEncryptionSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCaPoolEncryptionSpecEl {}
impl BuildPrivatecaCaPoolEncryptionSpecEl {
    pub fn build(self) -> PrivatecaCaPoolEncryptionSpecEl {
        PrivatecaCaPoolEncryptionSpecEl {
            cloud_kms_key: core::default::Default::default(),
        }
    }
}
pub struct PrivatecaCaPoolEncryptionSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCaPoolEncryptionSpecElRef {
    fn new(shared: StackShared, base: String) -> PrivatecaCaPoolEncryptionSpecElRef {
        PrivatecaCaPoolEncryptionSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCaPoolEncryptionSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_kms_key` after provisioning.\nThe resource name for an existing Cloud KMS key in the format\n'projects/*/locations/*/keyRings/*/cryptoKeys/*'."]
    pub fn cloud_kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_kms_key", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesEl {
    allow_config_based_issuance: PrimField<bool>,
    allow_csr_based_issuance: PrimField<bool>,
}
impl PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesEl {}
impl ToListMappable for PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesEl {
    type O = BlockAssignable<PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesEl {
    #[doc = "When true, allows callers to create Certificates by specifying a CertificateConfig."]
    pub allow_config_based_issuance: PrimField<bool>,
    #[doc = "When true, allows callers to create Certificates by specifying a CSR."]
    pub allow_csr_based_issuance: PrimField<bool>,
}
impl BuildPrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesEl {
        PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesEl {
            allow_config_based_issuance: self.allow_config_based_issuance,
            allow_csr_based_issuance: self.allow_csr_based_issuance,
        }
    }
}
pub struct PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesElRef {
        PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_config_based_issuance` after provisioning.\nWhen true, allows callers to create Certificates by specifying a CertificateConfig."]
    pub fn allow_config_based_issuance(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_config_based_issuance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_csr_based_issuance` after provisioning.\nWhen true, allows callers to create Certificates by specifying a CSR."]
    pub fn allow_csr_based_issuance(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_csr_based_issuance", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveEl {
    signature_algorithm: PrimField<String>,
}
impl PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveEl {}
impl ToListMappable for PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveEl {
    type O = BlockAssignable<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveEl {
    #[doc = "The algorithm used. Possible values: [\"ECDSA_P256\", \"ECDSA_P384\", \"EDDSA_25519\"]"]
    pub signature_algorithm: PrimField<String>,
}
impl BuildPrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveEl {
        PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveEl {
            signature_algorithm: self.signature_algorithm,
        }
    }
}
pub struct PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveElRef {
        PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `signature_algorithm` after provisioning.\nThe algorithm used. Possible values: [\"ECDSA_P256\", \"ECDSA_P384\", \"EDDSA_25519\"]"]
    pub fn signature_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.signature_algorithm", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_modulus_size: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_modulus_size: Option<PrimField<String>>,
}
impl PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaEl {
    #[doc = "Set the field `max_modulus_size`.\nThe maximum allowed RSA modulus size, in bits. If this is not set, or if set to zero, the\nservice will not enforce an explicit upper bound on RSA modulus sizes."]
    pub fn set_max_modulus_size(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_modulus_size = Some(v.into());
        self
    }
    #[doc = "Set the field `min_modulus_size`.\nThe minimum allowed RSA modulus size, in bits. If this is not set, or if set to zero, the\nservice-level min RSA modulus size will continue to apply."]
    pub fn set_min_modulus_size(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_modulus_size = Some(v.into());
        self
    }
}
impl ToListMappable for PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaEl {
    type O = BlockAssignable<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaEl {}
impl BuildPrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaEl {
        PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaEl {
            max_modulus_size: core::default::Default::default(),
            min_modulus_size: core::default::Default::default(),
        }
    }
}
pub struct PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaElRef {
        PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_modulus_size` after provisioning.\nThe maximum allowed RSA modulus size, in bits. If this is not set, or if set to zero, the\nservice will not enforce an explicit upper bound on RSA modulus sizes."]
    pub fn max_modulus_size(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_modulus_size", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_modulus_size` after provisioning.\nThe minimum allowed RSA modulus size, in bits. If this is not set, or if set to zero, the\nservice-level min RSA modulus size will continue to apply."]
    pub fn min_modulus_size(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_modulus_size", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElDynamic {
    elliptic_curve:
        Option<DynamicBlock<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveEl>>,
    rsa: Option<DynamicBlock<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaEl>>,
}
#[derive(Serialize)]
pub struct PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    elliptic_curve: Option<Vec<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rsa: Option<Vec<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaEl>>,
    dynamic: PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElDynamic,
}
impl PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesEl {
    #[doc = "Set the field `elliptic_curve`.\n"]
    pub fn set_elliptic_curve(
        mut self,
        v: impl Into<BlockAssignable<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.elliptic_curve = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.elliptic_curve = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rsa`.\n"]
    pub fn set_rsa(
        mut self,
        v: impl Into<BlockAssignable<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rsa = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rsa = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesEl {
    type O = BlockAssignable<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCaPoolIssuancePolicyElAllowedKeyTypesEl {}
impl BuildPrivatecaCaPoolIssuancePolicyElAllowedKeyTypesEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesEl {
        PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesEl {
            elliptic_curve: core::default::Default::default(),
            rsa: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRef {
        PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `elliptic_curve` after provisioning.\n"]
    pub fn elliptic_curve(
        &self,
    ) -> ListRef<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.elliptic_curve", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rsa` after provisioning.\n"]
    pub fn rsa(&self) -> ListRef<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rsa", self.base))
    }
}
#[derive(Serialize)]
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdEl {
    object_id_path: ListField<PrimField<f64>>,
}
impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdEl {}
impl ToListMappable
    for PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdEl
{
    type O = BlockAssignable<
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdEl {
    #[doc = "An ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages."]
    pub object_id_path: ListField<PrimField<f64>>,
}
impl BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdEl {
    pub fn build(
        self,
    ) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdEl {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdEl {
            object_id_path: self.object_id_path,
        }
    }
}
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdElRef {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdElRef {
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
struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElDynamic {
    object_id: Option<
        DynamicBlock<
            PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsEl {
    critical: PrimField<bool>,
    value: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_id: Option<
        Vec<PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdEl>,
    >,
    dynamic: PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElDynamic,
}
impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsEl {
    #[doc = "Set the field `object_id`.\n"]
    pub fn set_object_id(
        mut self,
        v: impl Into<
            BlockAssignable<
                PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdEl,
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
impl ToListMappable for PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsEl {
    type O = BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsEl {
    #[doc = "Indicates whether or not this extension is critical (i.e., if the client does not know how to\nhandle this extension, the client should consider this to be an error)."]
    pub critical: PrimField<bool>,
    #[doc = "The value of this X.509 extension. A base64-encoded string."]
    pub value: PrimField<String>,
}
impl BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsEl {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsEl {
            critical: self.critical,
            value: self.value,
            object_id: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElRef {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElRef {
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
    ) -> ListRef<PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.object_id", self.base))
    }
}
#[derive(Serialize)]
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    is_ca: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_issuer_path_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    non_ca: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zero_max_issuer_path_length: Option<PrimField<bool>>,
}
impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsEl {
    #[doc = "Set the field `is_ca`.\nWhen true, the \"CA\" in Basic Constraints extension will be set to true."]
    pub fn set_is_ca(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_ca = Some(v.into());
        self
    }
    #[doc = "Set the field `max_issuer_path_length`.\nRefers to the \"path length constraint\" in Basic Constraints extension. For a CA certificate, this value describes the depth of\nsubordinate CA certificates that are allowed. If this value is less than 0, the request will fail."]
    pub fn set_max_issuer_path_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_issuer_path_length = Some(v.into());
        self
    }
    #[doc = "Set the field `non_ca`.\nWhen true, the \"CA\" in Basic Constraints extension will be set to false.\nIf both 'is_ca' and 'non_ca' are unset, the extension will be omitted from the CA certificate."]
    pub fn set_non_ca(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.non_ca = Some(v.into());
        self
    }
    #[doc = "Set the field `zero_max_issuer_path_length`.\nWhen true, the \"path length constraint\" in Basic Constraints extension will be set to 0.\nif both 'max_issuer_path_length' and 'zero_max_issuer_path_length' are unset,\nthe max path length will be omitted from the CA certificate."]
    pub fn set_zero_max_issuer_path_length(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.zero_max_issuer_path_length = Some(v.into());
        self
    }
}
impl ToListMappable for PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsEl {
    type O = BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsEl {}
impl BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsEl {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsEl {
            is_ca: core::default::Default::default(),
            max_issuer_path_length: core::default::Default::default(),
            non_ca: core::default::Default::default(),
            zero_max_issuer_path_length: core::default::Default::default(),
        }
    }
}
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsElRef {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `is_ca` after provisioning.\nWhen true, the \"CA\" in Basic Constraints extension will be set to true."]
    pub fn is_ca(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_ca", self.base))
    }
    #[doc = "Get a reference to the value of field `max_issuer_path_length` after provisioning.\nRefers to the \"path length constraint\" in Basic Constraints extension. For a CA certificate, this value describes the depth of\nsubordinate CA certificates that are allowed. If this value is less than 0, the request will fail."]
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
    #[doc = "Get a reference to the value of field `zero_max_issuer_path_length` after provisioning.\nWhen true, the \"path length constraint\" in Basic Constraints extension will be set to 0.\nif both 'max_issuer_path_length' and 'zero_max_issuer_path_length' are unset,\nthe max path length will be omitted from the CA certificate."]
    pub fn zero_max_issuer_path_length(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zero_max_issuer_path_length", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageEl {
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
impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageEl {
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
impl ToListMappable for PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageEl {
    type O =
        BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageEl {}
impl BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageEl {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageEl {
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
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageElRef {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageElRef {
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
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageEl {
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
impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageEl {
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
    for PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageEl
{
    type O = BlockAssignable<
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageEl {}
impl BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageEl {
    pub fn build(
        self,
    ) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageEl {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageEl {
            client_auth: core::default::Default::default(),
            code_signing: core::default::Default::default(),
            email_protection: core::default::Default::default(),
            ocsp_signing: core::default::Default::default(),
            server_auth: core::default::Default::default(),
            time_stamping: core::default::Default::default(),
        }
    }
}
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageElRef {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageElRef {
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
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesEl {
    object_id_path: ListField<PrimField<f64>>,
}
impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesEl {}
impl ToListMappable
    for PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesEl
{
    type O = BlockAssignable<
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesEl
{
    #[doc = "An ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages."]
    pub object_id_path: ListField<PrimField<f64>>,
}
impl BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesEl {
    pub fn build(
        self,
    ) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesEl {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesEl {
            object_id_path: self.object_id_path,
        }
    }
}
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesElRef
    {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesElRef {
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
struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElDynamic {
    base_key_usage: Option<
        DynamicBlock<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageEl>,
    >,
    extended_key_usage: Option<
        DynamicBlock<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageEl>,
    >,
    unknown_extended_key_usages: Option<
        DynamicBlock<
            PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    base_key_usage:
        Option<Vec<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extended_key_usage:
        Option<Vec<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unknown_extended_key_usages: Option<
        Vec<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesEl>,
    >,
    dynamic: PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElDynamic,
}
impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageEl {
    #[doc = "Set the field `base_key_usage`.\n"]
    pub fn set_base_key_usage(
        mut self,
        v: impl Into<
            BlockAssignable<
                PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageEl,
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
                PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageEl,
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
        v: impl Into<
            BlockAssignable<
                PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesEl,
            >,
        >,
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
impl ToListMappable for PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageEl {
    type O = BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageEl {}
impl BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageEl {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageEl {
            base_key_usage: core::default::Default::default(),
            extended_key_usage: core::default::Default::default(),
            unknown_extended_key_usages: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElRef {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `base_key_usage` after provisioning.\n"]
    pub fn base_key_usage(
        &self,
    ) -> ListRef<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.base_key_usage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `extended_key_usage` after provisioning.\n"]
    pub fn extended_key_usage(
        &self,
    ) -> ListRef<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageElRef>
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
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.unknown_extended_key_usages", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsEl {
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
impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsEl {
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
impl ToListMappable for PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsEl {
    type O = BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsEl {
    #[doc = "Indicates whether or not the name constraints are marked critical."]
    pub critical: PrimField<bool>,
}
impl BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsEl {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsEl {
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
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsElRef {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsElRef {
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
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsEl {
    object_id_path: ListField<PrimField<f64>>,
}
impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsEl {}
impl ToListMappable for PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsEl {
    type O = BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsEl {
    #[doc = "An ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages."]
    pub object_id_path: ListField<PrimField<f64>>,
}
impl BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsEl {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsEl {
            object_id_path: self.object_id_path,
        }
    }
}
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsElRef {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsElRef {
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
struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElDynamic {
    additional_extensions:
        Option<DynamicBlock<PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsEl>>,
    ca_options: Option<DynamicBlock<PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsEl>>,
    key_usage: Option<DynamicBlock<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageEl>>,
    name_constraints:
        Option<DynamicBlock<PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsEl>>,
    policy_ids: Option<DynamicBlock<PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsEl>>,
}
#[derive(Serialize)]
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aia_ocsp_servers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_extensions:
        Option<Vec<PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_options: Option<Vec<PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_usage: Option<Vec<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_constraints: Option<Vec<PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_ids: Option<Vec<PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsEl>>,
    dynamic: PrivatecaCaPoolIssuancePolicyElBaselineValuesElDynamic,
}
impl PrivatecaCaPoolIssuancePolicyElBaselineValuesEl {
    #[doc = "Set the field `aia_ocsp_servers`.\nDescribes Online Certificate Status Protocol (OCSP) endpoint addresses that appear in the\n\"Authority Information Access\" extension in the certificate."]
    pub fn set_aia_ocsp_servers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.aia_ocsp_servers = Some(v.into());
        self
    }
    #[doc = "Set the field `additional_extensions`.\n"]
    pub fn set_additional_extensions(
        mut self,
        v: impl Into<
            BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsEl>,
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
        v: impl Into<BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsEl>>,
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
        v: impl Into<BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageEl>>,
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
        v: impl Into<BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsEl>>,
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
        v: impl Into<BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsEl>>,
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
impl ToListMappable for PrivatecaCaPoolIssuancePolicyElBaselineValuesEl {
    type O = BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesEl {}
impl BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesEl {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesEl {
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
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCaPoolIssuancePolicyElBaselineValuesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElRef {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElRef {
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
    ) -> ListRef<PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_extensions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ca_options` after provisioning.\n"]
    pub fn ca_options(
        &self,
    ) -> ListRef<PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ca_options", self.base))
    }
    #[doc = "Get a reference to the value of field `key_usage` after provisioning.\n"]
    pub fn key_usage(
        &self,
    ) -> ListRef<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.key_usage", self.base))
    }
    #[doc = "Get a reference to the value of field `name_constraints` after provisioning.\n"]
    pub fn name_constraints(
        &self,
    ) -> ListRef<PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.name_constraints", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `policy_ids` after provisioning.\n"]
    pub fn policy_ids(
        &self,
    ) -> ListRef<PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.policy_ids", self.base))
    }
}
#[derive(Serialize)]
pub struct PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}
impl PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionEl {
    #[doc = "Set the field `description`.\nDescription of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nString indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nTitle for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
}
impl ToListMappable for PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionEl {
    type O = BlockAssignable<PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionEl {
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: PrimField<String>,
}
impl BuildPrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionEl {
        PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionEl {
            description: core::default::Default::default(),
            expression: self.expression,
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}
pub struct PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionElRef {
        PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nString indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nTitle for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}
#[derive(Serialize, Default)]
struct PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElDynamic {
    cel_expression:
        Option<DynamicBlock<PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionEl>>,
}
#[derive(Serialize)]
pub struct PrivatecaCaPoolIssuancePolicyElIdentityConstraintsEl {
    allow_subject_alt_names_passthrough: PrimField<bool>,
    allow_subject_passthrough: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cel_expression:
        Option<Vec<PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionEl>>,
    dynamic: PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElDynamic,
}
impl PrivatecaCaPoolIssuancePolicyElIdentityConstraintsEl {
    #[doc = "Set the field `cel_expression`.\n"]
    pub fn set_cel_expression(
        mut self,
        v: impl Into<
            BlockAssignable<PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cel_expression = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cel_expression = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PrivatecaCaPoolIssuancePolicyElIdentityConstraintsEl {
    type O = BlockAssignable<PrivatecaCaPoolIssuancePolicyElIdentityConstraintsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCaPoolIssuancePolicyElIdentityConstraintsEl {
    #[doc = "If this is set, the SubjectAltNames extension may be copied from a certificate request into the signed certificate.\nOtherwise, the requested SubjectAltNames will be discarded."]
    pub allow_subject_alt_names_passthrough: PrimField<bool>,
    #[doc = "If this is set, the Subject field may be copied from a certificate request into the signed certificate.\nOtherwise, the requested Subject will be discarded."]
    pub allow_subject_passthrough: PrimField<bool>,
}
impl BuildPrivatecaCaPoolIssuancePolicyElIdentityConstraintsEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElIdentityConstraintsEl {
        PrivatecaCaPoolIssuancePolicyElIdentityConstraintsEl {
            allow_subject_alt_names_passthrough: self.allow_subject_alt_names_passthrough,
            allow_subject_passthrough: self.allow_subject_passthrough,
            cel_expression: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElRef {
        PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_subject_alt_names_passthrough` after provisioning.\nIf this is set, the SubjectAltNames extension may be copied from a certificate request into the signed certificate.\nOtherwise, the requested SubjectAltNames will be discarded."]
    pub fn allow_subject_alt_names_passthrough(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_subject_alt_names_passthrough", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_subject_passthrough` after provisioning.\nIf this is set, the Subject field may be copied from a certificate request into the signed certificate.\nOtherwise, the requested Subject will be discarded."]
    pub fn allow_subject_passthrough(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_subject_passthrough", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cel_expression` after provisioning.\n"]
    pub fn cel_expression(
        &self,
    ) -> ListRef<PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cel_expression", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct PrivatecaCaPoolIssuancePolicyElDynamic {
    allowed_issuance_modes:
        Option<DynamicBlock<PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesEl>>,
    allowed_key_types: Option<DynamicBlock<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesEl>>,
    baseline_values: Option<DynamicBlock<PrivatecaCaPoolIssuancePolicyElBaselineValuesEl>>,
    identity_constraints:
        Option<DynamicBlock<PrivatecaCaPoolIssuancePolicyElIdentityConstraintsEl>>,
}
#[derive(Serialize)]
pub struct PrivatecaCaPoolIssuancePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backdate_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_lifetime: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_issuance_modes: Option<Vec<PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_key_types: Option<Vec<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    baseline_values: Option<Vec<PrivatecaCaPoolIssuancePolicyElBaselineValuesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_constraints: Option<Vec<PrivatecaCaPoolIssuancePolicyElIdentityConstraintsEl>>,
    dynamic: PrivatecaCaPoolIssuancePolicyElDynamic,
}
impl PrivatecaCaPoolIssuancePolicyEl {
    #[doc = "Set the field `backdate_duration`.\nThe duration to backdate all certificates issued from this CaPool. If not set, the\ncertificates will be issued with a not_before_time of the issuance time (i.e. the current\ntime). If set, the certificates will be issued with a not_before_time of the issuance\ntime minus the backdate_duration. The not_after_time will be adjusted to preserve the\nrequested lifetime. The backdate_duration must be less than or equal to 48 hours."]
    pub fn set_backdate_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backdate_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `maximum_lifetime`.\nThe maximum lifetime allowed for issued Certificates. Note that if the issuing CertificateAuthority\nexpires before a Certificate's requested maximumLifetime, the effective lifetime will be explicitly truncated to match it."]
    pub fn set_maximum_lifetime(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.maximum_lifetime = Some(v.into());
        self
    }
    #[doc = "Set the field `allowed_issuance_modes`.\n"]
    pub fn set_allowed_issuance_modes(
        mut self,
        v: impl Into<BlockAssignable<PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.allowed_issuance_modes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_issuance_modes = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `allowed_key_types`.\n"]
    pub fn set_allowed_key_types(
        mut self,
        v: impl Into<BlockAssignable<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.allowed_key_types = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_key_types = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `baseline_values`.\n"]
    pub fn set_baseline_values(
        mut self,
        v: impl Into<BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.baseline_values = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.baseline_values = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `identity_constraints`.\n"]
    pub fn set_identity_constraints(
        mut self,
        v: impl Into<BlockAssignable<PrivatecaCaPoolIssuancePolicyElIdentityConstraintsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.identity_constraints = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.identity_constraints = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PrivatecaCaPoolIssuancePolicyEl {
    type O = BlockAssignable<PrivatecaCaPoolIssuancePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCaPoolIssuancePolicyEl {}
impl BuildPrivatecaCaPoolIssuancePolicyEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyEl {
        PrivatecaCaPoolIssuancePolicyEl {
            backdate_duration: core::default::Default::default(),
            maximum_lifetime: core::default::Default::default(),
            allowed_issuance_modes: core::default::Default::default(),
            allowed_key_types: core::default::Default::default(),
            baseline_values: core::default::Default::default(),
            identity_constraints: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PrivatecaCaPoolIssuancePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCaPoolIssuancePolicyElRef {
    fn new(shared: StackShared, base: String) -> PrivatecaCaPoolIssuancePolicyElRef {
        PrivatecaCaPoolIssuancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCaPoolIssuancePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backdate_duration` after provisioning.\nThe duration to backdate all certificates issued from this CaPool. If not set, the\ncertificates will be issued with a not_before_time of the issuance time (i.e. the current\ntime). If set, the certificates will be issued with a not_before_time of the issuance\ntime minus the backdate_duration. The not_after_time will be adjusted to preserve the\nrequested lifetime. The backdate_duration must be less than or equal to 48 hours."]
    pub fn backdate_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backdate_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maximum_lifetime` after provisioning.\nThe maximum lifetime allowed for issued Certificates. Note that if the issuing CertificateAuthority\nexpires before a Certificate's requested maximumLifetime, the effective lifetime will be explicitly truncated to match it."]
    pub fn maximum_lifetime(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_lifetime", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allowed_issuance_modes` after provisioning.\n"]
    pub fn allowed_issuance_modes(
        &self,
    ) -> ListRef<PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_issuance_modes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allowed_key_types` after provisioning.\n"]
    pub fn allowed_key_types(
        &self,
    ) -> ListRef<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_key_types", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `baseline_values` after provisioning.\n"]
    pub fn baseline_values(&self) -> ListRef<PrivatecaCaPoolIssuancePolicyElBaselineValuesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.baseline_values", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `identity_constraints` after provisioning.\n"]
    pub fn identity_constraints(
        &self,
    ) -> ListRef<PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.identity_constraints", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrivatecaCaPoolPublishingOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding_format: Option<PrimField<String>>,
    publish_ca_cert: PrimField<bool>,
    publish_crl: PrimField<bool>,
}
impl PrivatecaCaPoolPublishingOptionsEl {
    #[doc = "Set the field `encoding_format`.\nSpecifies the encoding format of each CertificateAuthority's CA\ncertificate and CRLs. If this is omitted, CA certificates and CRLs\nwill be published in PEM. Possible values: [\"PEM\", \"DER\"]"]
    pub fn set_encoding_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encoding_format = Some(v.into());
        self
    }
}
impl ToListMappable for PrivatecaCaPoolPublishingOptionsEl {
    type O = BlockAssignable<PrivatecaCaPoolPublishingOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCaPoolPublishingOptionsEl {
    #[doc = "When true, publishes each CertificateAuthority's CA certificate and includes its URL in the \"Authority Information Access\"\nX.509 extension in all issued Certificates. If this is false, the CA certificate will not be published and the corresponding\nX.509 extension will not be written in issued certificates."]
    pub publish_ca_cert: PrimField<bool>,
    #[doc = "When true, publishes each CertificateAuthority's CRL and includes its URL in the \"CRL Distribution Points\" X.509 extension\nin all issued Certificates. If this is false, CRLs will not be published and the corresponding X.509 extension will not\nbe written in issued certificates. CRLs will expire 7 days from their creation. However, we will rebuild daily. CRLs are\nalso rebuilt shortly after a certificate is revoked."]
    pub publish_crl: PrimField<bool>,
}
impl BuildPrivatecaCaPoolPublishingOptionsEl {
    pub fn build(self) -> PrivatecaCaPoolPublishingOptionsEl {
        PrivatecaCaPoolPublishingOptionsEl {
            encoding_format: core::default::Default::default(),
            publish_ca_cert: self.publish_ca_cert,
            publish_crl: self.publish_crl,
        }
    }
}
pub struct PrivatecaCaPoolPublishingOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCaPoolPublishingOptionsElRef {
    fn new(shared: StackShared, base: String) -> PrivatecaCaPoolPublishingOptionsElRef {
        PrivatecaCaPoolPublishingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCaPoolPublishingOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `encoding_format` after provisioning.\nSpecifies the encoding format of each CertificateAuthority's CA\ncertificate and CRLs. If this is omitted, CA certificates and CRLs\nwill be published in PEM. Possible values: [\"PEM\", \"DER\"]"]
    pub fn encoding_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encoding_format", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `publish_ca_cert` after provisioning.\nWhen true, publishes each CertificateAuthority's CA certificate and includes its URL in the \"Authority Information Access\"\nX.509 extension in all issued Certificates. If this is false, the CA certificate will not be published and the corresponding\nX.509 extension will not be written in issued certificates."]
    pub fn publish_ca_cert(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.publish_ca_cert", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `publish_crl` after provisioning.\nWhen true, publishes each CertificateAuthority's CRL and includes its URL in the \"CRL Distribution Points\" X.509 extension\nin all issued Certificates. If this is false, CRLs will not be published and the corresponding X.509 extension will not\nbe written in issued certificates. CRLs will expire 7 days from their creation. However, we will rebuild daily. CRLs are\nalso rebuilt shortly after a certificate is revoked."]
    pub fn publish_crl(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.publish_crl", self.base))
    }
}
#[derive(Serialize)]
pub struct PrivatecaCaPoolTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl PrivatecaCaPoolTimeoutsEl {
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
impl ToListMappable for PrivatecaCaPoolTimeoutsEl {
    type O = BlockAssignable<PrivatecaCaPoolTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCaPoolTimeoutsEl {}
impl BuildPrivatecaCaPoolTimeoutsEl {
    pub fn build(self) -> PrivatecaCaPoolTimeoutsEl {
        PrivatecaCaPoolTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct PrivatecaCaPoolTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCaPoolTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> PrivatecaCaPoolTimeoutsElRef {
        PrivatecaCaPoolTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCaPoolTimeoutsElRef {
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
struct PrivatecaCaPoolDynamic {
    encryption_spec: Option<DynamicBlock<PrivatecaCaPoolEncryptionSpecEl>>,
    issuance_policy: Option<DynamicBlock<PrivatecaCaPoolIssuancePolicyEl>>,
    publishing_options: Option<DynamicBlock<PrivatecaCaPoolPublishingOptionsEl>>,
}
