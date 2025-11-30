use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataSignerSigningProfileData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataSignerSigningProfile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSignerSigningProfileData>,
}

#[derive(Clone)]
pub struct DataSignerSigningProfile(Rc<DataSignerSigningProfile_>);

impl DataSignerSigningProfile {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
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
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `platform_display_name` after provisioning.\n"]
    pub fn platform_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.platform_display_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `platform_id` after provisioning.\n"]
    pub fn platform_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.platform_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `revocation_record` after provisioning.\n"]
    pub fn revocation_record(&self) -> ListRef<DataSignerSigningProfileRevocationRecordElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.revocation_record", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `signature_validity_period` after provisioning.\n"]
    pub fn signature_validity_period(
        &self,
    ) -> ListRef<DataSignerSigningProfileSignatureValidityPeriodElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.signature_validity_period", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `signing_material` after provisioning.\n"]
    pub fn signing_material(&self) -> ListRef<DataSignerSigningProfileSigningMaterialElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.signing_material", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `signing_parameters` after provisioning.\n"]
    pub fn signing_parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.signing_parameters", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `version_arn` after provisioning.\n"]
    pub fn version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_arn", self.extract_ref()),
        )
    }
}

impl Referable for DataSignerSigningProfile {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataSignerSigningProfile {}

impl ToListMappable for DataSignerSigningProfile {
    type O = ListRef<DataSignerSigningProfileRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSignerSigningProfile_ {
    fn extract_datasource_type(&self) -> String {
        "aws_signer_signing_profile".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSignerSigningProfile {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildDataSignerSigningProfile {
    pub fn build(self, stack: &mut Stack) -> DataSignerSigningProfile {
        let out = DataSignerSigningProfile(Rc::new(DataSignerSigningProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSignerSigningProfileData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSignerSigningProfileRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSignerSigningProfileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataSignerSigningProfileRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
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
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `platform_display_name` after provisioning.\n"]
    pub fn platform_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.platform_display_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `platform_id` after provisioning.\n"]
    pub fn platform_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.platform_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `revocation_record` after provisioning.\n"]
    pub fn revocation_record(&self) -> ListRef<DataSignerSigningProfileRevocationRecordElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.revocation_record", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `signature_validity_period` after provisioning.\n"]
    pub fn signature_validity_period(
        &self,
    ) -> ListRef<DataSignerSigningProfileSignatureValidityPeriodElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.signature_validity_period", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `signing_material` after provisioning.\n"]
    pub fn signing_material(&self) -> ListRef<DataSignerSigningProfileSigningMaterialElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.signing_material", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `signing_parameters` after provisioning.\n"]
    pub fn signing_parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.signing_parameters", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `version_arn` after provisioning.\n"]
    pub fn version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_arn", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataSignerSigningProfileRevocationRecordEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    revocation_effective_from: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revoked_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revoked_by: Option<PrimField<String>>,
}

impl DataSignerSigningProfileRevocationRecordEl {
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

impl ToListMappable for DataSignerSigningProfileRevocationRecordEl {
    type O = BlockAssignable<DataSignerSigningProfileRevocationRecordEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSignerSigningProfileRevocationRecordEl {}

impl BuildDataSignerSigningProfileRevocationRecordEl {
    pub fn build(self) -> DataSignerSigningProfileRevocationRecordEl {
        DataSignerSigningProfileRevocationRecordEl {
            revocation_effective_from: core::default::Default::default(),
            revoked_at: core::default::Default::default(),
            revoked_by: core::default::Default::default(),
        }
    }
}

pub struct DataSignerSigningProfileRevocationRecordElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSignerSigningProfileRevocationRecordElRef {
    fn new(shared: StackShared, base: String) -> DataSignerSigningProfileRevocationRecordElRef {
        DataSignerSigningProfileRevocationRecordElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSignerSigningProfileRevocationRecordElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `revocation_effective_from` after provisioning.\n"]
    pub fn revocation_effective_from(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revocation_effective_from", self.base),
        )
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
pub struct DataSignerSigningProfileSignatureValidityPeriodEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl DataSignerSigningProfileSignatureValidityPeriodEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataSignerSigningProfileSignatureValidityPeriodEl {
    type O = BlockAssignable<DataSignerSigningProfileSignatureValidityPeriodEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSignerSigningProfileSignatureValidityPeriodEl {}

impl BuildDataSignerSigningProfileSignatureValidityPeriodEl {
    pub fn build(self) -> DataSignerSigningProfileSignatureValidityPeriodEl {
        DataSignerSigningProfileSignatureValidityPeriodEl {
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataSignerSigningProfileSignatureValidityPeriodElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSignerSigningProfileSignatureValidityPeriodElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSignerSigningProfileSignatureValidityPeriodElRef {
        DataSignerSigningProfileSignatureValidityPeriodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSignerSigningProfileSignatureValidityPeriodElRef {
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
pub struct DataSignerSigningProfileSigningMaterialEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_arn: Option<PrimField<String>>,
}

impl DataSignerSigningProfileSigningMaterialEl {
    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_arn = Some(v.into());
        self
    }
}

impl ToListMappable for DataSignerSigningProfileSigningMaterialEl {
    type O = BlockAssignable<DataSignerSigningProfileSigningMaterialEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSignerSigningProfileSigningMaterialEl {}

impl BuildDataSignerSigningProfileSigningMaterialEl {
    pub fn build(self) -> DataSignerSigningProfileSigningMaterialEl {
        DataSignerSigningProfileSigningMaterialEl {
            certificate_arn: core::default::Default::default(),
        }
    }
}

pub struct DataSignerSigningProfileSigningMaterialElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSignerSigningProfileSigningMaterialElRef {
    fn new(shared: StackShared, base: String) -> DataSignerSigningProfileSigningMaterialElRef {
        DataSignerSigningProfileSigningMaterialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSignerSigningProfileSigningMaterialElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_arn", self.base),
        )
    }
}
