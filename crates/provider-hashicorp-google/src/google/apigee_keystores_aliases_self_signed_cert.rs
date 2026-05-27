use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ApigeeKeystoresAliasesSelfSignedCertData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    alias: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cert_validity_in_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    environment: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_size: Option<PrimField<String>>,
    keystore: PrimField<String>,
    org_id: PrimField<String>,
    sig_alg: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject: Option<Vec<ApigeeKeystoresAliasesSelfSignedCertSubjectEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_alternative_dns_names:
        Option<Vec<ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApigeeKeystoresAliasesSelfSignedCertTimeoutsEl>,
    dynamic: ApigeeKeystoresAliasesSelfSignedCertDynamic,
}
struct ApigeeKeystoresAliasesSelfSignedCert_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApigeeKeystoresAliasesSelfSignedCertData>,
}
#[derive(Clone)]
pub struct ApigeeKeystoresAliasesSelfSignedCert(Rc<ApigeeKeystoresAliasesSelfSignedCert_>);
impl ApigeeKeystoresAliasesSelfSignedCert {
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
    #[doc = "Set the field `cert_validity_in_days`.\nValidity duration of certificate, in days. Accepts positive non-zero value. Defaults to 365."]
    pub fn set_cert_validity_in_days(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().cert_validity_in_days = Some(v.into());
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
    #[doc = "Set the field `key_size`.\nKey size. Default and maximum value is 2048 bits."]
    pub fn set_key_size(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().key_size = Some(v.into());
        self
    }
    #[doc = "Set the field `subject`.\n"]
    pub fn set_subject(
        self,
        v: impl Into<BlockAssignable<ApigeeKeystoresAliasesSelfSignedCertSubjectEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().subject = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.subject = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `subject_alternative_dns_names`.\n"]
    pub fn set_subject_alternative_dns_names(
        self,
        v: impl Into<BlockAssignable<ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().subject_alternative_dns_names = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .subject_alternative_dns_names = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<ApigeeKeystoresAliasesSelfSignedCertTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `alias` after provisioning.\nAlias for the key/certificate pair. Values must match the regular expression [\\w\\s-.]{1,255}.\nThis must be provided for all formats except selfsignedcert; self-signed certs may specify the alias in either\nthis parameter or the JSON body."]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alias", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cert_validity_in_days` after provisioning.\nValidity duration of certificate, in days. Accepts positive non-zero value. Defaults to 365."]
    pub fn cert_validity_in_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cert_validity_in_days", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `certs_info` after provisioning.\nChain of certificates under this alias."]
    pub fn certs_info(&self) -> ListRef<ApigeeKeystoresAliasesSelfSignedCertCertsInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.certs_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `environment` after provisioning.\nThe Apigee environment name"]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.environment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `key_size` after provisioning.\nKey size. Default and maximum value is 2048 bits."]
    pub fn key_size(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `keystore` after provisioning.\nThe Apigee keystore name associated in an Apigee environment"]
    pub fn keystore(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.keystore", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization name associated with the Apigee environment"]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sig_alg` after provisioning.\nSignature algorithm to generate private key. Valid values are SHA512withRSA, SHA384withRSA, and SHA256withRSA"]
    pub fn sig_alg(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sig_alg", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nOptional.Type of Alias"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subject` after provisioning.\n"]
    pub fn subject(&self) -> ListRef<ApigeeKeystoresAliasesSelfSignedCertSubjectElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subject", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subject_alternative_dns_names` after provisioning.\n"]
    pub fn subject_alternative_dns_names(
        &self,
    ) -> ListRef<ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subject_alternative_dns_names", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeKeystoresAliasesSelfSignedCertTimeoutsElRef {
        ApigeeKeystoresAliasesSelfSignedCertTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ApigeeKeystoresAliasesSelfSignedCert {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ApigeeKeystoresAliasesSelfSignedCert {}
impl ToListMappable for ApigeeKeystoresAliasesSelfSignedCert {
    type O = ListRef<ApigeeKeystoresAliasesSelfSignedCertRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ApigeeKeystoresAliasesSelfSignedCert_ {
    fn extract_resource_type(&self) -> String {
        "google_apigee_keystores_aliases_self_signed_cert".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApigeeKeystoresAliasesSelfSignedCert {
    pub tf_id: String,
    #[doc = "Alias for the key/certificate pair. Values must match the regular expression [\\w\\s-.]{1,255}.\nThis must be provided for all formats except selfsignedcert; self-signed certs may specify the alias in either\nthis parameter or the JSON body."]
    pub alias: PrimField<String>,
    #[doc = "The Apigee environment name"]
    pub environment: PrimField<String>,
    #[doc = "The Apigee keystore name associated in an Apigee environment"]
    pub keystore: PrimField<String>,
    #[doc = "The Apigee Organization name associated with the Apigee environment"]
    pub org_id: PrimField<String>,
    #[doc = "Signature algorithm to generate private key. Valid values are SHA512withRSA, SHA384withRSA, and SHA256withRSA"]
    pub sig_alg: PrimField<String>,
}
impl BuildApigeeKeystoresAliasesSelfSignedCert {
    pub fn build(self, stack: &mut Stack) -> ApigeeKeystoresAliasesSelfSignedCert {
        let out =
            ApigeeKeystoresAliasesSelfSignedCert(Rc::new(ApigeeKeystoresAliasesSelfSignedCert_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(ApigeeKeystoresAliasesSelfSignedCertData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    alias: self.alias,
                    cert_validity_in_days: core::default::Default::default(),
                    deletion_policy: core::default::Default::default(),
                    environment: self.environment,
                    id: core::default::Default::default(),
                    key_size: core::default::Default::default(),
                    keystore: self.keystore,
                    org_id: self.org_id,
                    sig_alg: self.sig_alg,
                    subject: core::default::Default::default(),
                    subject_alternative_dns_names: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ApigeeKeystoresAliasesSelfSignedCertRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeKeystoresAliasesSelfSignedCertRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ApigeeKeystoresAliasesSelfSignedCertRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `alias` after provisioning.\nAlias for the key/certificate pair. Values must match the regular expression [\\w\\s-.]{1,255}.\nThis must be provided for all formats except selfsignedcert; self-signed certs may specify the alias in either\nthis parameter or the JSON body."]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alias", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cert_validity_in_days` after provisioning.\nValidity duration of certificate, in days. Accepts positive non-zero value. Defaults to 365."]
    pub fn cert_validity_in_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cert_validity_in_days", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `certs_info` after provisioning.\nChain of certificates under this alias."]
    pub fn certs_info(&self) -> ListRef<ApigeeKeystoresAliasesSelfSignedCertCertsInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.certs_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `environment` after provisioning.\nThe Apigee environment name"]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.environment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `key_size` after provisioning.\nKey size. Default and maximum value is 2048 bits."]
    pub fn key_size(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `keystore` after provisioning.\nThe Apigee keystore name associated in an Apigee environment"]
    pub fn keystore(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.keystore", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization name associated with the Apigee environment"]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sig_alg` after provisioning.\nSignature algorithm to generate private key. Valid values are SHA512withRSA, SHA384withRSA, and SHA256withRSA"]
    pub fn sig_alg(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sig_alg", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nOptional.Type of Alias"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subject` after provisioning.\n"]
    pub fn subject(&self) -> ListRef<ApigeeKeystoresAliasesSelfSignedCertSubjectElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subject", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subject_alternative_dns_names` after provisioning.\n"]
    pub fn subject_alternative_dns_names(
        &self,
    ) -> ListRef<ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subject_alternative_dns_names", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeKeystoresAliasesSelfSignedCertTimeoutsElRef {
        ApigeeKeystoresAliasesSelfSignedCertTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    basic_constraints: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expiry_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_valid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serial_number: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sig_alg_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_alternative_names: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_from: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<f64>>,
}
impl ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoEl {
    #[doc = "Set the field `basic_constraints`.\n"]
    pub fn set_basic_constraints(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.basic_constraints = Some(v.into());
        self
    }
    #[doc = "Set the field `expiry_date`.\n"]
    pub fn set_expiry_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expiry_date = Some(v.into());
        self
    }
    #[doc = "Set the field `is_valid`.\n"]
    pub fn set_is_valid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.is_valid = Some(v.into());
        self
    }
    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issuer = Some(v.into());
        self
    }
    #[doc = "Set the field `public_key`.\n"]
    pub fn set_public_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.public_key = Some(v.into());
        self
    }
    #[doc = "Set the field `serial_number`.\n"]
    pub fn set_serial_number(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.serial_number = Some(v.into());
        self
    }
    #[doc = "Set the field `sig_alg_name`.\n"]
    pub fn set_sig_alg_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sig_alg_name = Some(v.into());
        self
    }
    #[doc = "Set the field `subject`.\n"]
    pub fn set_subject(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subject = Some(v.into());
        self
    }
    #[doc = "Set the field `subject_alternative_names`.\n"]
    pub fn set_subject_alternative_names(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.subject_alternative_names = Some(v.into());
        self
    }
    #[doc = "Set the field `valid_from`.\n"]
    pub fn set_valid_from(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.valid_from = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoEl {
    type O = BlockAssignable<ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoEl {}
impl BuildApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoEl {
    pub fn build(self) -> ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoEl {
        ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoEl {
            basic_constraints: core::default::Default::default(),
            expiry_date: core::default::Default::default(),
            is_valid: core::default::Default::default(),
            issuer: core::default::Default::default(),
            public_key: core::default::Default::default(),
            serial_number: core::default::Default::default(),
            sig_alg_name: core::default::Default::default(),
            subject: core::default::Default::default(),
            subject_alternative_names: core::default::Default::default(),
            valid_from: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoElRef {
        ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `basic_constraints` after provisioning.\n"]
    pub fn basic_constraints(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.basic_constraints", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `expiry_date` after provisioning.\n"]
    pub fn expiry_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expiry_date", self.base))
    }
    #[doc = "Get a reference to the value of field `is_valid` after provisioning.\n"]
    pub fn is_valid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_valid", self.base))
    }
    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }
    #[doc = "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.base))
    }
    #[doc = "Get a reference to the value of field `serial_number` after provisioning.\n"]
    pub fn serial_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.serial_number", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sig_alg_name` after provisioning.\n"]
    pub fn sig_alg_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sig_alg_name", self.base))
    }
    #[doc = "Get a reference to the value of field `subject` after provisioning.\n"]
    pub fn subject(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subject", self.base))
    }
    #[doc = "Get a reference to the value of field `subject_alternative_names` after provisioning.\n"]
    pub fn subject_alternative_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subject_alternative_names", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `valid_from` after provisioning.\n"]
    pub fn valid_from(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.valid_from", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize)]
pub struct ApigeeKeystoresAliasesSelfSignedCertCertsInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cert_info: Option<ListField<ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoEl>>,
}
impl ApigeeKeystoresAliasesSelfSignedCertCertsInfoEl {
    #[doc = "Set the field `cert_info`.\n"]
    pub fn set_cert_info(
        mut self,
        v: impl Into<ListField<ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoEl>>,
    ) -> Self {
        self.cert_info = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeKeystoresAliasesSelfSignedCertCertsInfoEl {
    type O = BlockAssignable<ApigeeKeystoresAliasesSelfSignedCertCertsInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeKeystoresAliasesSelfSignedCertCertsInfoEl {}
impl BuildApigeeKeystoresAliasesSelfSignedCertCertsInfoEl {
    pub fn build(self) -> ApigeeKeystoresAliasesSelfSignedCertCertsInfoEl {
        ApigeeKeystoresAliasesSelfSignedCertCertsInfoEl {
            cert_info: core::default::Default::default(),
        }
    }
}
pub struct ApigeeKeystoresAliasesSelfSignedCertCertsInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeKeystoresAliasesSelfSignedCertCertsInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApigeeKeystoresAliasesSelfSignedCertCertsInfoElRef {
        ApigeeKeystoresAliasesSelfSignedCertCertsInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeKeystoresAliasesSelfSignedCertCertsInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cert_info` after provisioning.\n"]
    pub fn cert_info(
        &self,
    ) -> ListRef<ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cert_info", self.base))
    }
}
#[derive(Serialize)]
pub struct ApigeeKeystoresAliasesSelfSignedCertSubjectEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    common_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locality: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    org: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    org_unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl ApigeeKeystoresAliasesSelfSignedCertSubjectEl {
    #[doc = "Set the field `common_name`.\nCommon name of the organization. Maximum length is 64 characters."]
    pub fn set_common_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.common_name = Some(v.into());
        self
    }
    #[doc = "Set the field `country_code`.\nTwo-letter country code. Example, IN for India, US for United States of America."]
    pub fn set_country_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.country_code = Some(v.into());
        self
    }
    #[doc = "Set the field `email`.\nEmail address. Max 255 characters."]
    pub fn set_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email = Some(v.into());
        self
    }
    #[doc = "Set the field `locality`.\nCity or town name. Maximum length is 128 characters."]
    pub fn set_locality(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.locality = Some(v.into());
        self
    }
    #[doc = "Set the field `org`.\nOrganization name. Maximum length is 64 characters."]
    pub fn set_org(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.org = Some(v.into());
        self
    }
    #[doc = "Set the field `org_unit`.\nOrganization team name. Maximum length is 64 characters."]
    pub fn set_org_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.org_unit = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\nState or district name. Maximum length is 128 characters."]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeKeystoresAliasesSelfSignedCertSubjectEl {
    type O = BlockAssignable<ApigeeKeystoresAliasesSelfSignedCertSubjectEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeKeystoresAliasesSelfSignedCertSubjectEl {}
impl BuildApigeeKeystoresAliasesSelfSignedCertSubjectEl {
    pub fn build(self) -> ApigeeKeystoresAliasesSelfSignedCertSubjectEl {
        ApigeeKeystoresAliasesSelfSignedCertSubjectEl {
            common_name: core::default::Default::default(),
            country_code: core::default::Default::default(),
            email: core::default::Default::default(),
            locality: core::default::Default::default(),
            org: core::default::Default::default(),
            org_unit: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct ApigeeKeystoresAliasesSelfSignedCertSubjectElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeKeystoresAliasesSelfSignedCertSubjectElRef {
    fn new(shared: StackShared, base: String) -> ApigeeKeystoresAliasesSelfSignedCertSubjectElRef {
        ApigeeKeystoresAliasesSelfSignedCertSubjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeKeystoresAliasesSelfSignedCertSubjectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `common_name` after provisioning.\nCommon name of the organization. Maximum length is 64 characters."]
    pub fn common_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.base))
    }
    #[doc = "Get a reference to the value of field `country_code` after provisioning.\nTwo-letter country code. Example, IN for India, US for United States of America."]
    pub fn country_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.country_code", self.base))
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\nEmail address. Max 255 characters."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
    #[doc = "Get a reference to the value of field `locality` after provisioning.\nCity or town name. Maximum length is 128 characters."]
    pub fn locality(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.locality", self.base))
    }
    #[doc = "Get a reference to the value of field `org` after provisioning.\nOrganization name. Maximum length is 64 characters."]
    pub fn org(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.org", self.base))
    }
    #[doc = "Get a reference to the value of field `org_unit` after provisioning.\nOrganization team name. Maximum length is 64 characters."]
    pub fn org_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.org_unit", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState or district name. Maximum length is 128 characters."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_alternative_name: Option<PrimField<String>>,
}
impl ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesEl {
    #[doc = "Set the field `subject_alternative_name`.\nSubject Alternative Name"]
    pub fn set_subject_alternative_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subject_alternative_name = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesEl {
    type O = BlockAssignable<ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesEl {}
impl BuildApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesEl {
    pub fn build(self) -> ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesEl {
        ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesEl {
            subject_alternative_name: core::default::Default::default(),
        }
    }
}
pub struct ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesElRef {
        ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `subject_alternative_name` after provisioning.\nSubject Alternative Name"]
    pub fn subject_alternative_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subject_alternative_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApigeeKeystoresAliasesSelfSignedCertTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl ApigeeKeystoresAliasesSelfSignedCertTimeoutsEl {
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
}
impl ToListMappable for ApigeeKeystoresAliasesSelfSignedCertTimeoutsEl {
    type O = BlockAssignable<ApigeeKeystoresAliasesSelfSignedCertTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeKeystoresAliasesSelfSignedCertTimeoutsEl {}
impl BuildApigeeKeystoresAliasesSelfSignedCertTimeoutsEl {
    pub fn build(self) -> ApigeeKeystoresAliasesSelfSignedCertTimeoutsEl {
        ApigeeKeystoresAliasesSelfSignedCertTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct ApigeeKeystoresAliasesSelfSignedCertTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeKeystoresAliasesSelfSignedCertTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ApigeeKeystoresAliasesSelfSignedCertTimeoutsElRef {
        ApigeeKeystoresAliasesSelfSignedCertTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeKeystoresAliasesSelfSignedCertTimeoutsElRef {
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
}
#[derive(Serialize, Default)]
struct ApigeeKeystoresAliasesSelfSignedCertDynamic {
    subject: Option<DynamicBlock<ApigeeKeystoresAliasesSelfSignedCertSubjectEl>>,
    subject_alternative_dns_names:
        Option<DynamicBlock<ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesEl>>,
}
