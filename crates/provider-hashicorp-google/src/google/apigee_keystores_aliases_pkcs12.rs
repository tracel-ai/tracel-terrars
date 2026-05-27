use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ApigeeKeystoresAliasesPkcs12Data {
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
    deletion_policy: Option<PrimField<String>>,
    environment: PrimField<String>,
    file: PrimField<String>,
    filehash: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    keystore: PrimField<String>,
    org_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApigeeKeystoresAliasesPkcs12TimeoutsEl>,
}
struct ApigeeKeystoresAliasesPkcs12_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApigeeKeystoresAliasesPkcs12Data>,
}
#[derive(Clone)]
pub struct ApigeeKeystoresAliasesPkcs12(Rc<ApigeeKeystoresAliasesPkcs12_>);
impl ApigeeKeystoresAliasesPkcs12 {
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
    #[doc = "Set the field `password`.\nPassword for the Private Key if it's encrypted"]
    pub fn set_password(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().password = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ApigeeKeystoresAliasesPkcs12TimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `alias` after provisioning.\nAlias Name"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alias", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `certs_info` after provisioning.\nChain of certificates under this alias."]
    pub fn certs_info(&self) -> ListRef<ApigeeKeystoresAliasesPkcs12CertsInfoElRef> {
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
    #[doc = "Get a reference to the value of field `environment` after provisioning.\nEnvironment associated with the alias"]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.environment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `file` after provisioning.\nCert content"]
    pub fn file(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filehash` after provisioning.\nHash of the pkcs file"]
    pub fn filehash(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filehash", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `keystore` after provisioning.\nKeystore Name"]
    pub fn keystore(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.keystore", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nOrganization ID associated with the alias"]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nPassword for the Private Key if it's encrypted"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nOptional.Type of Alias"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeKeystoresAliasesPkcs12TimeoutsElRef {
        ApigeeKeystoresAliasesPkcs12TimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ApigeeKeystoresAliasesPkcs12 {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ApigeeKeystoresAliasesPkcs12 {}
impl ToListMappable for ApigeeKeystoresAliasesPkcs12 {
    type O = ListRef<ApigeeKeystoresAliasesPkcs12Ref>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ApigeeKeystoresAliasesPkcs12_ {
    fn extract_resource_type(&self) -> String {
        "google_apigee_keystores_aliases_pkcs12".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApigeeKeystoresAliasesPkcs12 {
    pub tf_id: String,
    #[doc = "Alias Name"]
    pub alias: PrimField<String>,
    #[doc = "Environment associated with the alias"]
    pub environment: PrimField<String>,
    #[doc = "Cert content"]
    pub file: PrimField<String>,
    #[doc = "Hash of the pkcs file"]
    pub filehash: PrimField<String>,
    #[doc = "Keystore Name"]
    pub keystore: PrimField<String>,
    #[doc = "Organization ID associated with the alias"]
    pub org_id: PrimField<String>,
}
impl BuildApigeeKeystoresAliasesPkcs12 {
    pub fn build(self, stack: &mut Stack) -> ApigeeKeystoresAliasesPkcs12 {
        let out = ApigeeKeystoresAliasesPkcs12(Rc::new(ApigeeKeystoresAliasesPkcs12_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApigeeKeystoresAliasesPkcs12Data {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                alias: self.alias,
                deletion_policy: core::default::Default::default(),
                environment: self.environment,
                file: self.file,
                filehash: self.filehash,
                id: core::default::Default::default(),
                keystore: self.keystore,
                org_id: self.org_id,
                password: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ApigeeKeystoresAliasesPkcs12Ref {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeKeystoresAliasesPkcs12Ref {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ApigeeKeystoresAliasesPkcs12Ref {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `alias` after provisioning.\nAlias Name"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alias", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `certs_info` after provisioning.\nChain of certificates under this alias."]
    pub fn certs_info(&self) -> ListRef<ApigeeKeystoresAliasesPkcs12CertsInfoElRef> {
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
    #[doc = "Get a reference to the value of field `environment` after provisioning.\nEnvironment associated with the alias"]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.environment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `file` after provisioning.\nCert content"]
    pub fn file(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filehash` after provisioning.\nHash of the pkcs file"]
    pub fn filehash(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filehash", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `keystore` after provisioning.\nKeystore Name"]
    pub fn keystore(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.keystore", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nOrganization ID associated with the alias"]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nPassword for the Private Key if it's encrypted"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nOptional.Type of Alias"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeKeystoresAliasesPkcs12TimeoutsElRef {
        ApigeeKeystoresAliasesPkcs12TimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ApigeeKeystoresAliasesPkcs12CertsInfoElCertInfoEl {
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
impl ApigeeKeystoresAliasesPkcs12CertsInfoElCertInfoEl {
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
impl ToListMappable for ApigeeKeystoresAliasesPkcs12CertsInfoElCertInfoEl {
    type O = BlockAssignable<ApigeeKeystoresAliasesPkcs12CertsInfoElCertInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeKeystoresAliasesPkcs12CertsInfoElCertInfoEl {}
impl BuildApigeeKeystoresAliasesPkcs12CertsInfoElCertInfoEl {
    pub fn build(self) -> ApigeeKeystoresAliasesPkcs12CertsInfoElCertInfoEl {
        ApigeeKeystoresAliasesPkcs12CertsInfoElCertInfoEl {
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
pub struct ApigeeKeystoresAliasesPkcs12CertsInfoElCertInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeKeystoresAliasesPkcs12CertsInfoElCertInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApigeeKeystoresAliasesPkcs12CertsInfoElCertInfoElRef {
        ApigeeKeystoresAliasesPkcs12CertsInfoElCertInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeKeystoresAliasesPkcs12CertsInfoElCertInfoElRef {
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
pub struct ApigeeKeystoresAliasesPkcs12CertsInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cert_info: Option<ListField<ApigeeKeystoresAliasesPkcs12CertsInfoElCertInfoEl>>,
}
impl ApigeeKeystoresAliasesPkcs12CertsInfoEl {
    #[doc = "Set the field `cert_info`.\n"]
    pub fn set_cert_info(
        mut self,
        v: impl Into<ListField<ApigeeKeystoresAliasesPkcs12CertsInfoElCertInfoEl>>,
    ) -> Self {
        self.cert_info = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeKeystoresAliasesPkcs12CertsInfoEl {
    type O = BlockAssignable<ApigeeKeystoresAliasesPkcs12CertsInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeKeystoresAliasesPkcs12CertsInfoEl {}
impl BuildApigeeKeystoresAliasesPkcs12CertsInfoEl {
    pub fn build(self) -> ApigeeKeystoresAliasesPkcs12CertsInfoEl {
        ApigeeKeystoresAliasesPkcs12CertsInfoEl {
            cert_info: core::default::Default::default(),
        }
    }
}
pub struct ApigeeKeystoresAliasesPkcs12CertsInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeKeystoresAliasesPkcs12CertsInfoElRef {
    fn new(shared: StackShared, base: String) -> ApigeeKeystoresAliasesPkcs12CertsInfoElRef {
        ApigeeKeystoresAliasesPkcs12CertsInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeKeystoresAliasesPkcs12CertsInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cert_info` after provisioning.\n"]
    pub fn cert_info(&self) -> ListRef<ApigeeKeystoresAliasesPkcs12CertsInfoElCertInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cert_info", self.base))
    }
}
#[derive(Serialize)]
pub struct ApigeeKeystoresAliasesPkcs12TimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl ApigeeKeystoresAliasesPkcs12TimeoutsEl {
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
impl ToListMappable for ApigeeKeystoresAliasesPkcs12TimeoutsEl {
    type O = BlockAssignable<ApigeeKeystoresAliasesPkcs12TimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeKeystoresAliasesPkcs12TimeoutsEl {}
impl BuildApigeeKeystoresAliasesPkcs12TimeoutsEl {
    pub fn build(self) -> ApigeeKeystoresAliasesPkcs12TimeoutsEl {
        ApigeeKeystoresAliasesPkcs12TimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct ApigeeKeystoresAliasesPkcs12TimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeKeystoresAliasesPkcs12TimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ApigeeKeystoresAliasesPkcs12TimeoutsElRef {
        ApigeeKeystoresAliasesPkcs12TimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeKeystoresAliasesPkcs12TimeoutsElRef {
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
