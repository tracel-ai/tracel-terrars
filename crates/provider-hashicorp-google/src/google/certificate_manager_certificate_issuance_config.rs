use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CertificateManagerCertificateIssuanceConfigData {
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
    id: Option<PrimField<String>>,
    key_algorithm: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    lifetime: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    rotation_window_percentage: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_authority_config:
        Option<Vec<CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CertificateManagerCertificateIssuanceConfigTimeoutsEl>,
    dynamic: CertificateManagerCertificateIssuanceConfigDynamic,
}
struct CertificateManagerCertificateIssuanceConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CertificateManagerCertificateIssuanceConfigData>,
}
#[derive(Clone)]
pub struct CertificateManagerCertificateIssuanceConfig(
    Rc<CertificateManagerCertificateIssuanceConfig_>,
);
impl CertificateManagerCertificateIssuanceConfig {
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
    #[doc = "Set the field `description`.\nOne or more paragraphs of text description of a CertificateIssuanceConfig."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\n'Set of label tags associated with the CertificateIssuanceConfig resource.\n An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe Certificate Manager location. If not specified, \"global\" is used."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `certificate_authority_config`.\n"]
    pub fn set_certificate_authority_config(
        self,
        v: impl Into<
            BlockAssignable<
                CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().certificate_authority_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .certificate_authority_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<CertificateManagerCertificateIssuanceConfigTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe creation timestamp of a CertificateIssuanceConfig. Timestamp is in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds with up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOne or more paragraphs of text description of a CertificateIssuanceConfig."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `key_algorithm` after provisioning.\nKey algorithm to use when generating the private key. Possible values: [\"RSA_2048\", \"ECDSA_P256\"]"]
    pub fn key_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_algorithm", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n'Set of label tags associated with the CertificateIssuanceConfig resource.\n An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lifetime` after provisioning.\nLifetime of issued certificates. A duration in seconds with up to nine fractional digits, ending with 's'.\nExample: \"1814400s\". Valid values are from 21 days (1814400s) to 30 days (2592000s)"]
    pub fn lifetime(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifetime", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe Certificate Manager location. If not specified, \"global\" is used."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA user-defined name of the certificate issuance config.\nCertificateIssuanceConfig names must be unique globally."]
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
    #[doc = "Get a reference to the value of field `rotation_window_percentage` after provisioning.\nIt specifies the percentage of elapsed time of the certificate lifetime to wait before renewing the certificate.\nMust be a number between 1-99, inclusive.\nYou must set the rotation window percentage in relation to the certificate lifetime so that certificate renewal occurs at least 7 days after\nthe certificate has been issued and at least 7 days before it expires."]
    pub fn rotation_window_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rotation_window_percentage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe last update timestamp of a CertificateIssuanceConfig. Timestamp is in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds with up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `certificate_authority_config` after provisioning.\n"]
    pub fn certificate_authority_config(
        &self,
    ) -> ListRef<CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.certificate_authority_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CertificateManagerCertificateIssuanceConfigTimeoutsElRef {
        CertificateManagerCertificateIssuanceConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for CertificateManagerCertificateIssuanceConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CertificateManagerCertificateIssuanceConfig {}
impl ToListMappable for CertificateManagerCertificateIssuanceConfig {
    type O = ListRef<CertificateManagerCertificateIssuanceConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CertificateManagerCertificateIssuanceConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_certificate_manager_certificate_issuance_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCertificateManagerCertificateIssuanceConfig {
    pub tf_id: String,
    #[doc = "Key algorithm to use when generating the private key. Possible values: [\"RSA_2048\", \"ECDSA_P256\"]"]
    pub key_algorithm: PrimField<String>,
    #[doc = "Lifetime of issued certificates. A duration in seconds with up to nine fractional digits, ending with 's'.\nExample: \"1814400s\". Valid values are from 21 days (1814400s) to 30 days (2592000s)"]
    pub lifetime: PrimField<String>,
    #[doc = "A user-defined name of the certificate issuance config.\nCertificateIssuanceConfig names must be unique globally."]
    pub name: PrimField<String>,
    #[doc = "It specifies the percentage of elapsed time of the certificate lifetime to wait before renewing the certificate.\nMust be a number between 1-99, inclusive.\nYou must set the rotation window percentage in relation to the certificate lifetime so that certificate renewal occurs at least 7 days after\nthe certificate has been issued and at least 7 days before it expires."]
    pub rotation_window_percentage: PrimField<f64>,
}
impl BuildCertificateManagerCertificateIssuanceConfig {
    pub fn build(self, stack: &mut Stack) -> CertificateManagerCertificateIssuanceConfig {
        let out = CertificateManagerCertificateIssuanceConfig(Rc::new(
            CertificateManagerCertificateIssuanceConfig_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(CertificateManagerCertificateIssuanceConfigData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    deletion_policy: core::default::Default::default(),
                    description: core::default::Default::default(),
                    id: core::default::Default::default(),
                    key_algorithm: self.key_algorithm,
                    labels: core::default::Default::default(),
                    lifetime: self.lifetime,
                    location: core::default::Default::default(),
                    name: self.name,
                    project: core::default::Default::default(),
                    rotation_window_percentage: self.rotation_window_percentage,
                    certificate_authority_config: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CertificateManagerCertificateIssuanceConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for CertificateManagerCertificateIssuanceConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CertificateManagerCertificateIssuanceConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe creation timestamp of a CertificateIssuanceConfig. Timestamp is in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds with up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOne or more paragraphs of text description of a CertificateIssuanceConfig."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `key_algorithm` after provisioning.\nKey algorithm to use when generating the private key. Possible values: [\"RSA_2048\", \"ECDSA_P256\"]"]
    pub fn key_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_algorithm", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n'Set of label tags associated with the CertificateIssuanceConfig resource.\n An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lifetime` after provisioning.\nLifetime of issued certificates. A duration in seconds with up to nine fractional digits, ending with 's'.\nExample: \"1814400s\". Valid values are from 21 days (1814400s) to 30 days (2592000s)"]
    pub fn lifetime(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifetime", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe Certificate Manager location. If not specified, \"global\" is used."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA user-defined name of the certificate issuance config.\nCertificateIssuanceConfig names must be unique globally."]
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
    #[doc = "Get a reference to the value of field `rotation_window_percentage` after provisioning.\nIt specifies the percentage of elapsed time of the certificate lifetime to wait before renewing the certificate.\nMust be a number between 1-99, inclusive.\nYou must set the rotation window percentage in relation to the certificate lifetime so that certificate renewal occurs at least 7 days after\nthe certificate has been issued and at least 7 days before it expires."]
    pub fn rotation_window_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rotation_window_percentage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe last update timestamp of a CertificateIssuanceConfig. Timestamp is in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds with up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `certificate_authority_config` after provisioning.\n"]
    pub fn certificate_authority_config(
        &self,
    ) -> ListRef<CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.certificate_authority_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CertificateManagerCertificateIssuanceConfigTimeoutsElRef {
        CertificateManagerCertificateIssuanceConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigEl
{
    ca_pool: PrimField<String>,
}
impl CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigEl { }
impl ToListMappable for CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigEl { type O = BlockAssignable < CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigEl
{
    #[doc = "A CA pool resource used to issue a certificate.\nThe CA pool string has a relative resource path following the form\n\"projects/{project}/locations/{location}/caPools/{caPool}\"."]
    pub ca_pool: PrimField<String>,
}
impl BuildCertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigEl { pub fn build (self) -> CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigEl { CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigEl { ca_pool : self . ca_pool , } } }
pub struct CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigElRef { fn new (shared : StackShared , base : String) -> CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigElRef { CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigElRef { shared : shared , base : base . to_string () , } } }
impl CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `ca_pool` after provisioning.\nA CA pool resource used to issue a certificate.\nThe CA pool string has a relative resource path following the form\n\"projects/{project}/locations/{location}/caPools/{caPool}\"."] pub fn ca_pool (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ca_pool" , self . base)) } }
#[derive(Serialize, Default)]
struct CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElDynamic { certificate_authority_service_config : Option < DynamicBlock < CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigEl >> , }
#[derive(Serialize)]
pub struct CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigEl { # [serde (skip_serializing_if = "Option::is_none")] certificate_authority_service_config : Option < Vec < CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigEl > > , dynamic : CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElDynamic , }
impl CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigEl {
    #[doc = "Set the field `certificate_authority_service_config`.\n"]
    pub fn set_certificate_authority_service_config(
        mut self,
        v : impl Into < BlockAssignable < CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.certificate_authority_service_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.certificate_authority_service_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigEl {
    type O =
        BlockAssignable<CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigEl {}
impl BuildCertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigEl {
    pub fn build(self) -> CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigEl {
        CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigEl {
            certificate_authority_service_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElRef {
        CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificate_authority_service_config` after provisioning.\n"]    pub fn certificate_authority_service_config (& self) -> ListRef < CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.certificate_authority_service_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CertificateManagerCertificateIssuanceConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CertificateManagerCertificateIssuanceConfigTimeoutsEl {
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
impl ToListMappable for CertificateManagerCertificateIssuanceConfigTimeoutsEl {
    type O = BlockAssignable<CertificateManagerCertificateIssuanceConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCertificateManagerCertificateIssuanceConfigTimeoutsEl {}
impl BuildCertificateManagerCertificateIssuanceConfigTimeoutsEl {
    pub fn build(self) -> CertificateManagerCertificateIssuanceConfigTimeoutsEl {
        CertificateManagerCertificateIssuanceConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct CertificateManagerCertificateIssuanceConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CertificateManagerCertificateIssuanceConfigTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CertificateManagerCertificateIssuanceConfigTimeoutsElRef {
        CertificateManagerCertificateIssuanceConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CertificateManagerCertificateIssuanceConfigTimeoutsElRef {
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
struct CertificateManagerCertificateIssuanceConfigDynamic {
    certificate_authority_config: Option<
        DynamicBlock<CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigEl>,
    >,
}
