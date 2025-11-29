use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SignerSigningProfileData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
    platform_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signing_parameters: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signature_validity_period: Option<Vec<SignerSigningProfileSignatureValidityPeriodEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signing_material: Option<Vec<SignerSigningProfileSigningMaterialEl>>,
    dynamic: SignerSigningProfileDynamic,
}

struct SignerSigningProfile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SignerSigningProfileData>,
}

#[derive(Clone)]
pub struct SignerSigningProfile(Rc<SignerSigningProfile_>);

impl SignerSigningProfile {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `signing_parameters`.\n"]
    pub fn set_signing_parameters(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().signing_parameters = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `signature_validity_period`.\n"]
    pub fn set_signature_validity_period(
        self,
        v: impl Into<BlockAssignable<SignerSigningProfileSignatureValidityPeriodEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().signature_validity_period = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.signature_validity_period = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `signing_material`.\n"]
    pub fn set_signing_material(self, v: impl Into<BlockAssignable<SignerSigningProfileSigningMaterialEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().signing_material = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.signing_material = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_display_name` after provisioning.\n"]
    pub fn platform_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_id` after provisioning.\n"]
    pub fn platform_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revocation_record` after provisioning.\n"]
    pub fn revocation_record(&self) -> ListRef<SignerSigningProfileRevocationRecordElRef> {
        ListRef::new(self.shared().clone(), format!("{}.revocation_record", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signing_parameters` after provisioning.\n"]
    pub fn signing_parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.signing_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_arn` after provisioning.\n"]
    pub fn version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signature_validity_period` after provisioning.\n"]
    pub fn signature_validity_period(&self) -> ListRef<SignerSigningProfileSignatureValidityPeriodElRef> {
        ListRef::new(self.shared().clone(), format!("{}.signature_validity_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signing_material` after provisioning.\n"]
    pub fn signing_material(&self) -> ListRef<SignerSigningProfileSigningMaterialElRef> {
        ListRef::new(self.shared().clone(), format!("{}.signing_material", self.extract_ref()))
    }
}

impl Referable for SignerSigningProfile {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for SignerSigningProfile { }

impl ToListMappable for SignerSigningProfile {
    type O = ListRef<SignerSigningProfileRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SignerSigningProfile_ {
    fn extract_resource_type(&self) -> String {
        "aws_signer_signing_profile".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSignerSigningProfile {
    pub tf_id: String,
    #[doc = ""]
    pub platform_id: PrimField<String>,
}

impl BuildSignerSigningProfile {
    pub fn build(self, stack: &mut Stack) -> SignerSigningProfile {
        let out = SignerSigningProfile(Rc::new(SignerSigningProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SignerSigningProfileData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                platform_id: self.platform_id,
                region: core::default::Default::default(),
                signing_parameters: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                signature_validity_period: core::default::Default::default(),
                signing_material: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SignerSigningProfileRef {
    shared: StackShared,
    base: String,
}

impl Ref for SignerSigningProfileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl SignerSigningProfileRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_display_name` after provisioning.\n"]
    pub fn platform_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_id` after provisioning.\n"]
    pub fn platform_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revocation_record` after provisioning.\n"]
    pub fn revocation_record(&self) -> ListRef<SignerSigningProfileRevocationRecordElRef> {
        ListRef::new(self.shared().clone(), format!("{}.revocation_record", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signing_parameters` after provisioning.\n"]
    pub fn signing_parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.signing_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_arn` after provisioning.\n"]
    pub fn version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signature_validity_period` after provisioning.\n"]
    pub fn signature_validity_period(&self) -> ListRef<SignerSigningProfileSignatureValidityPeriodElRef> {
        ListRef::new(self.shared().clone(), format!("{}.signature_validity_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signing_material` after provisioning.\n"]
    pub fn signing_material(&self) -> ListRef<SignerSigningProfileSigningMaterialElRef> {
        ListRef::new(self.shared().clone(), format!("{}.signing_material", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SignerSigningProfileRevocationRecordEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    revocation_effective_from: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revoked_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revoked_by: Option<PrimField<String>>,
}

impl SignerSigningProfileRevocationRecordEl {
    #[doc = "Set the field `revocation_effective_from`.\n"]
    pub fn set_revocation_effective_from(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.revocation_effective_from = Some(v.into());
        self
    }

    #[doc = "Set the field `revoked_at`.\n"]
    pub fn set_revoked_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.revoked_at = Some(v.into());
        self
    }

    #[doc = "Set the field `revoked_by`.\n"]
    pub fn set_revoked_by(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.revoked_by = Some(v.into());
        self
    }
}

impl ToListMappable for SignerSigningProfileRevocationRecordEl {
    type O = BlockAssignable<SignerSigningProfileRevocationRecordEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSignerSigningProfileRevocationRecordEl {}

impl BuildSignerSigningProfileRevocationRecordEl {
    pub fn build(self) -> SignerSigningProfileRevocationRecordEl {
        SignerSigningProfileRevocationRecordEl {
            revocation_effective_from: core::default::Default::default(),
            revoked_at: core::default::Default::default(),
            revoked_by: core::default::Default::default(),
        }
    }
}

pub struct SignerSigningProfileRevocationRecordElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SignerSigningProfileRevocationRecordElRef {
    fn new(shared: StackShared, base: String) -> SignerSigningProfileRevocationRecordElRef {
        SignerSigningProfileRevocationRecordElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SignerSigningProfileRevocationRecordElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `revocation_effective_from` after provisioning.\n"]
    pub fn revocation_effective_from(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revocation_effective_from", self.base))
    }

    #[doc = "Get a reference to the value of field `revoked_at` after provisioning.\n"]
    pub fn revoked_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revoked_at", self.base))
    }

    #[doc = "Get a reference to the value of field `revoked_by` after provisioning.\n"]
    pub fn revoked_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revoked_by", self.base))
    }
}

#[derive(Serialize)]
pub struct SignerSigningProfileSignatureValidityPeriodEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    value: PrimField<f64>,
}

impl SignerSigningProfileSignatureValidityPeriodEl { }

impl ToListMappable for SignerSigningProfileSignatureValidityPeriodEl {
    type O = BlockAssignable<SignerSigningProfileSignatureValidityPeriodEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSignerSigningProfileSignatureValidityPeriodEl {
    #[doc = ""]
    pub type_: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildSignerSigningProfileSignatureValidityPeriodEl {
    pub fn build(self) -> SignerSigningProfileSignatureValidityPeriodEl {
        SignerSigningProfileSignatureValidityPeriodEl {
            type_: self.type_,
            value: self.value,
        }
    }
}

pub struct SignerSigningProfileSignatureValidityPeriodElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SignerSigningProfileSignatureValidityPeriodElRef {
    fn new(shared: StackShared, base: String) -> SignerSigningProfileSignatureValidityPeriodElRef {
        SignerSigningProfileSignatureValidityPeriodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SignerSigningProfileSignatureValidityPeriodElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SignerSigningProfileSigningMaterialEl {
    certificate_arn: PrimField<String>,
}

impl SignerSigningProfileSigningMaterialEl { }

impl ToListMappable for SignerSigningProfileSigningMaterialEl {
    type O = BlockAssignable<SignerSigningProfileSigningMaterialEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSignerSigningProfileSigningMaterialEl {
    #[doc = ""]
    pub certificate_arn: PrimField<String>,
}

impl BuildSignerSigningProfileSigningMaterialEl {
    pub fn build(self) -> SignerSigningProfileSigningMaterialEl {
        SignerSigningProfileSigningMaterialEl { certificate_arn: self.certificate_arn }
    }
}

pub struct SignerSigningProfileSigningMaterialElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SignerSigningProfileSigningMaterialElRef {
    fn new(shared: StackShared, base: String) -> SignerSigningProfileSigningMaterialElRef {
        SignerSigningProfileSigningMaterialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SignerSigningProfileSigningMaterialElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SignerSigningProfileDynamic {
    signature_validity_period: Option<DynamicBlock<SignerSigningProfileSignatureValidityPeriodEl>>,
    signing_material: Option<DynamicBlock<SignerSigningProfileSigningMaterialEl>>,
}
