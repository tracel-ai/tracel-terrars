use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataAcmpcaCertificateAuthorityData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}
struct DataAcmpcaCertificateAuthority_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAcmpcaCertificateAuthorityData>,
}
#[derive(Clone)]
pub struct DataAcmpcaCertificateAuthority(Rc<DataAcmpcaCertificateAuthority_>);
impl DataAcmpcaCertificateAuthority {
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
    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_chain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `certificate_signing_request` after provisioning.\n"]
    pub fn certificate_signing_request(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_signing_request", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `key_storage_security_standard` after provisioning.\n"]
    pub fn key_storage_security_standard(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_storage_security_standard", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `not_after` after provisioning.\n"]
    pub fn not_after(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.not_after", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `not_before` after provisioning.\n"]
    pub fn not_before(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.not_before", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `revocation_configuration` after provisioning.\n"]
    pub fn revocation_configuration(
        &self,
    ) -> ListRef<DataAcmpcaCertificateAuthorityRevocationConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.revocation_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `serial` after provisioning.\n"]
    pub fn serial(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.serial", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `usage_mode` after provisioning.\n"]
    pub fn usage_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.usage_mode", self.extract_ref()),
        )
    }
}
impl Referable for DataAcmpcaCertificateAuthority {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataAcmpcaCertificateAuthority {}
impl ToListMappable for DataAcmpcaCertificateAuthority {
    type O = ListRef<DataAcmpcaCertificateAuthorityRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataAcmpcaCertificateAuthority_ {
    fn extract_datasource_type(&self) -> String {
        "aws_acmpca_certificate_authority".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataAcmpcaCertificateAuthority {
    pub tf_id: String,
    #[doc = ""]
    pub arn: PrimField<String>,
}
impl BuildDataAcmpcaCertificateAuthority {
    pub fn build(self, stack: &mut Stack) -> DataAcmpcaCertificateAuthority {
        let out = DataAcmpcaCertificateAuthority(Rc::new(DataAcmpcaCertificateAuthority_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAcmpcaCertificateAuthorityData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                arn: self.arn,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataAcmpcaCertificateAuthorityRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAcmpcaCertificateAuthorityRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataAcmpcaCertificateAuthorityRef {
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
    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_chain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `certificate_signing_request` after provisioning.\n"]
    pub fn certificate_signing_request(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_signing_request", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `key_storage_security_standard` after provisioning.\n"]
    pub fn key_storage_security_standard(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_storage_security_standard", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `not_after` after provisioning.\n"]
    pub fn not_after(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.not_after", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `not_before` after provisioning.\n"]
    pub fn not_before(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.not_before", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `revocation_configuration` after provisioning.\n"]
    pub fn revocation_configuration(
        &self,
    ) -> ListRef<DataAcmpcaCertificateAuthorityRevocationConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.revocation_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `serial` after provisioning.\n"]
    pub fn serial(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.serial", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `usage_mode` after provisioning.\n"]
    pub fn usage_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.usage_mode", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_cname: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration_in_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_object_acl: Option<PrimField<String>>,
}
impl DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl {
    #[doc = "Set the field `custom_cname`.\n"]
    pub fn set_custom_cname(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.custom_cname = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `expiration_in_days`.\n"]
    pub fn set_expiration_in_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.expiration_in_days = Some(v.into());
        self
    }
    #[doc = "Set the field `s3_bucket_name`.\n"]
    pub fn set_s3_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_bucket_name = Some(v.into());
        self
    }
    #[doc = "Set the field `s3_object_acl`.\n"]
    pub fn set_s3_object_acl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_object_acl = Some(v.into());
        self
    }
}
impl ToListMappable for DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl {
    type O =
        BlockAssignable<DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl {}
impl BuildDataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl {
    pub fn build(
        self,
    ) -> DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl {
        DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl {
            custom_cname: core::default::Default::default(),
            enabled: core::default::Default::default(),
            expiration_in_days: core::default::Default::default(),
            s3_bucket_name: core::default::Default::default(),
            s3_object_acl: core::default::Default::default(),
        }
    }
}
pub struct DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationElRef {
        DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_cname` after provisioning.\n"]
    pub fn custom_cname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_cname", self.base))
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `expiration_in_days` after provisioning.\n"]
    pub fn expiration_in_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expiration_in_days", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_bucket_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3_object_acl` after provisioning.\n"]
    pub fn s3_object_acl(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_object_acl", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ocsp_custom_cname: Option<PrimField<String>>,
}
impl DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `ocsp_custom_cname`.\n"]
    pub fn set_ocsp_custom_cname(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ocsp_custom_cname = Some(v.into());
        self
    }
}
impl ToListMappable for DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl {
    type O =
        BlockAssignable<DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl {}
impl BuildDataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl {
    pub fn build(
        self,
    ) -> DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl {
        DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl {
            enabled: core::default::Default::default(),
            ocsp_custom_cname: core::default::Default::default(),
        }
    }
}
pub struct DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationElRef {
        DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `ocsp_custom_cname` after provisioning.\n"]
    pub fn ocsp_custom_cname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ocsp_custom_cname", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAcmpcaCertificateAuthorityRevocationConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    crl_configuration: Option<
        ListField<DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    ocsp_configuration: Option<
        ListField<DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl>,
    >,
}
impl DataAcmpcaCertificateAuthorityRevocationConfigurationEl {
    #[doc = "Set the field `crl_configuration`.\n"]
    pub fn set_crl_configuration(
        mut self,
        v: impl Into<
            ListField<DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl>,
        >,
    ) -> Self {
        self.crl_configuration = Some(v.into());
        self
    }
    #[doc = "Set the field `ocsp_configuration`.\n"]
    pub fn set_ocsp_configuration(
        mut self,
        v: impl Into<
            ListField<DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl>,
        >,
    ) -> Self {
        self.ocsp_configuration = Some(v.into());
        self
    }
}
impl ToListMappable for DataAcmpcaCertificateAuthorityRevocationConfigurationEl {
    type O = BlockAssignable<DataAcmpcaCertificateAuthorityRevocationConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAcmpcaCertificateAuthorityRevocationConfigurationEl {}
impl BuildDataAcmpcaCertificateAuthorityRevocationConfigurationEl {
    pub fn build(self) -> DataAcmpcaCertificateAuthorityRevocationConfigurationEl {
        DataAcmpcaCertificateAuthorityRevocationConfigurationEl {
            crl_configuration: core::default::Default::default(),
            ocsp_configuration: core::default::Default::default(),
        }
    }
}
pub struct DataAcmpcaCertificateAuthorityRevocationConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAcmpcaCertificateAuthorityRevocationConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAcmpcaCertificateAuthorityRevocationConfigurationElRef {
        DataAcmpcaCertificateAuthorityRevocationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAcmpcaCertificateAuthorityRevocationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `crl_configuration` after provisioning.\n"]
    pub fn crl_configuration(
        &self,
    ) -> ListRef<DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.crl_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ocsp_configuration` after provisioning.\n"]
    pub fn ocsp_configuration(
        &self,
    ) -> ListRef<DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ocsp_configuration", self.base),
        )
    }
}
