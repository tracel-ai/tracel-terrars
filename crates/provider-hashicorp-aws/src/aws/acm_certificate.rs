use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct AcmCertificateData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_authority_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_body: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_chain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    early_renewal_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_alternative_names: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<Vec<AcmCertificateOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_option: Option<Vec<AcmCertificateValidationOptionEl>>,
    dynamic: AcmCertificateDynamic,
}

struct AcmCertificate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AcmCertificateData>,
}

#[derive(Clone)]
pub struct AcmCertificate(Rc<AcmCertificate_>);

impl AcmCertificate {
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

    #[doc = "Set the field `certificate_authority_arn`.\n"]
    pub fn set_certificate_authority_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().certificate_authority_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_body`.\n"]
    pub fn set_certificate_body(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().certificate_body = Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_chain`.\n"]
    pub fn set_certificate_chain(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().certificate_chain = Some(v.into());
        self
    }

    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().domain_name = Some(v.into());
        self
    }

    #[doc = "Set the field `early_renewal_duration`.\n"]
    pub fn set_early_renewal_duration(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().early_renewal_duration = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `key_algorithm`.\n"]
    pub fn set_key_algorithm(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().key_algorithm = Some(v.into());
        self
    }

    #[doc = "Set the field `private_key`.\n"]
    pub fn set_private_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().private_key = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `subject_alternative_names`.\n"]
    pub fn set_subject_alternative_names(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().subject_alternative_names = Some(v.into());
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

    #[doc = "Set the field `validation_method`.\n"]
    pub fn set_validation_method(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().validation_method = Some(v.into());
        self
    }

    #[doc = "Set the field `options`.\n"]
    pub fn set_options(self, v: impl Into<BlockAssignable<AcmCertificateOptionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.options = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `validation_option`.\n"]
    pub fn set_validation_option(
        self,
        v: impl Into<BlockAssignable<AcmCertificateValidationOptionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().validation_option = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.validation_option = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_authority_arn` after provisioning.\n"]
    pub fn certificate_authority_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_authority_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `certificate_body` after provisioning.\n"]
    pub fn certificate_body(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_body", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_chain", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_validation_options` after provisioning.\n"]
    pub fn domain_validation_options(&self) -> SetRef<AcmCertificateDomainValidationOptionsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.domain_validation_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `early_renewal_duration` after provisioning.\n"]
    pub fn early_renewal_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.early_renewal_duration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_algorithm` after provisioning.\n"]
    pub fn key_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_algorithm", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `pending_renewal` after provisioning.\n"]
    pub fn pending_renewal(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pending_renewal", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `private_key` after provisioning.\n"]
    pub fn private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_key", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `renewal_eligibility` after provisioning.\n"]
    pub fn renewal_eligibility(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.renewal_eligibility", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `renewal_summary` after provisioning.\n"]
    pub fn renewal_summary(&self) -> ListRef<AcmCertificateRenewalSummaryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.renewal_summary", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subject_alternative_names` after provisioning.\n"]
    pub fn subject_alternative_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subject_alternative_names", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `validation_emails` after provisioning.\n"]
    pub fn validation_emails(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.validation_emails", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `validation_method` after provisioning.\n"]
    pub fn validation_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.validation_method", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> ListRef<AcmCertificateOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.options", self.extract_ref()),
        )
    }
}

impl Referable for AcmCertificate {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for AcmCertificate {}

impl ToListMappable for AcmCertificate {
    type O = ListRef<AcmCertificateRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AcmCertificate_ {
    fn extract_resource_type(&self) -> String {
        "aws_acm_certificate".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAcmCertificate {
    pub tf_id: String,
}

impl BuildAcmCertificate {
    pub fn build(self, stack: &mut Stack) -> AcmCertificate {
        let out = AcmCertificate(Rc::new(AcmCertificate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AcmCertificateData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                certificate_authority_arn: core::default::Default::default(),
                certificate_body: core::default::Default::default(),
                certificate_chain: core::default::Default::default(),
                domain_name: core::default::Default::default(),
                early_renewal_duration: core::default::Default::default(),
                id: core::default::Default::default(),
                key_algorithm: core::default::Default::default(),
                private_key: core::default::Default::default(),
                region: core::default::Default::default(),
                subject_alternative_names: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                validation_method: core::default::Default::default(),
                options: core::default::Default::default(),
                validation_option: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AcmCertificateRef {
    shared: StackShared,
    base: String,
}

impl Ref for AcmCertificateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl AcmCertificateRef {
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

    #[doc = "Get a reference to the value of field `certificate_authority_arn` after provisioning.\n"]
    pub fn certificate_authority_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_authority_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `certificate_body` after provisioning.\n"]
    pub fn certificate_body(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_body", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_chain", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_validation_options` after provisioning.\n"]
    pub fn domain_validation_options(&self) -> SetRef<AcmCertificateDomainValidationOptionsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.domain_validation_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `early_renewal_duration` after provisioning.\n"]
    pub fn early_renewal_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.early_renewal_duration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_algorithm` after provisioning.\n"]
    pub fn key_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_algorithm", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `pending_renewal` after provisioning.\n"]
    pub fn pending_renewal(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pending_renewal", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `private_key` after provisioning.\n"]
    pub fn private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_key", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `renewal_eligibility` after provisioning.\n"]
    pub fn renewal_eligibility(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.renewal_eligibility", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `renewal_summary` after provisioning.\n"]
    pub fn renewal_summary(&self) -> ListRef<AcmCertificateRenewalSummaryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.renewal_summary", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subject_alternative_names` after provisioning.\n"]
    pub fn subject_alternative_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subject_alternative_names", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `validation_emails` after provisioning.\n"]
    pub fn validation_emails(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.validation_emails", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `validation_method` after provisioning.\n"]
    pub fn validation_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.validation_method", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> ListRef<AcmCertificateOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.options", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct AcmCertificateDomainValidationOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_record_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_record_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_record_value: Option<PrimField<String>>,
}

impl AcmCertificateDomainValidationOptionsEl {
    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain_name = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_record_name`.\n"]
    pub fn set_resource_record_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_record_name = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_record_type`.\n"]
    pub fn set_resource_record_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_record_type = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_record_value`.\n"]
    pub fn set_resource_record_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_record_value = Some(v.into());
        self
    }
}

impl ToListMappable for AcmCertificateDomainValidationOptionsEl {
    type O = BlockAssignable<AcmCertificateDomainValidationOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAcmCertificateDomainValidationOptionsEl {}

impl BuildAcmCertificateDomainValidationOptionsEl {
    pub fn build(self) -> AcmCertificateDomainValidationOptionsEl {
        AcmCertificateDomainValidationOptionsEl {
            domain_name: core::default::Default::default(),
            resource_record_name: core::default::Default::default(),
            resource_record_type: core::default::Default::default(),
            resource_record_value: core::default::Default::default(),
        }
    }
}

pub struct AcmCertificateDomainValidationOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AcmCertificateDomainValidationOptionsElRef {
    fn new(shared: StackShared, base: String) -> AcmCertificateDomainValidationOptionsElRef {
        AcmCertificateDomainValidationOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AcmCertificateDomainValidationOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_record_name` after provisioning.\n"]
    pub fn resource_record_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_record_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `resource_record_type` after provisioning.\n"]
    pub fn resource_record_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_record_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `resource_record_value` after provisioning.\n"]
    pub fn resource_record_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_record_value", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct AcmCertificateRenewalSummaryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    renewal_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    renewal_status_reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<PrimField<String>>,
}

impl AcmCertificateRenewalSummaryEl {
    #[doc = "Set the field `renewal_status`.\n"]
    pub fn set_renewal_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.renewal_status = Some(v.into());
        self
    }

    #[doc = "Set the field `renewal_status_reason`.\n"]
    pub fn set_renewal_status_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.renewal_status_reason = Some(v.into());
        self
    }

    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.updated_at = Some(v.into());
        self
    }
}

impl ToListMappable for AcmCertificateRenewalSummaryEl {
    type O = BlockAssignable<AcmCertificateRenewalSummaryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAcmCertificateRenewalSummaryEl {}

impl BuildAcmCertificateRenewalSummaryEl {
    pub fn build(self) -> AcmCertificateRenewalSummaryEl {
        AcmCertificateRenewalSummaryEl {
            renewal_status: core::default::Default::default(),
            renewal_status_reason: core::default::Default::default(),
            updated_at: core::default::Default::default(),
        }
    }
}

pub struct AcmCertificateRenewalSummaryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AcmCertificateRenewalSummaryElRef {
    fn new(shared: StackShared, base: String) -> AcmCertificateRenewalSummaryElRef {
        AcmCertificateRenewalSummaryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AcmCertificateRenewalSummaryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `renewal_status` after provisioning.\n"]
    pub fn renewal_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.renewal_status", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `renewal_status_reason` after provisioning.\n"]
    pub fn renewal_status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.renewal_status_reason", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }
}

#[derive(Serialize)]
pub struct AcmCertificateOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_transparency_logging_preference: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    export: Option<PrimField<String>>,
}

impl AcmCertificateOptionsEl {
    #[doc = "Set the field `certificate_transparency_logging_preference`.\n"]
    pub fn set_certificate_transparency_logging_preference(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.certificate_transparency_logging_preference = Some(v.into());
        self
    }

    #[doc = "Set the field `export`.\n"]
    pub fn set_export(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.export = Some(v.into());
        self
    }
}

impl ToListMappable for AcmCertificateOptionsEl {
    type O = BlockAssignable<AcmCertificateOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAcmCertificateOptionsEl {}

impl BuildAcmCertificateOptionsEl {
    pub fn build(self) -> AcmCertificateOptionsEl {
        AcmCertificateOptionsEl {
            certificate_transparency_logging_preference: core::default::Default::default(),
            export: core::default::Default::default(),
        }
    }
}

pub struct AcmCertificateOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AcmCertificateOptionsElRef {
    fn new(shared: StackShared, base: String) -> AcmCertificateOptionsElRef {
        AcmCertificateOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AcmCertificateOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_transparency_logging_preference` after provisioning.\n"]
    pub fn certificate_transparency_logging_preference(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_transparency_logging_preference", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `export` after provisioning.\n"]
    pub fn export(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.export", self.base))
    }
}

#[derive(Serialize)]
pub struct AcmCertificateValidationOptionEl {
    domain_name: PrimField<String>,
    validation_domain: PrimField<String>,
}

impl AcmCertificateValidationOptionEl {}

impl ToListMappable for AcmCertificateValidationOptionEl {
    type O = BlockAssignable<AcmCertificateValidationOptionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAcmCertificateValidationOptionEl {
    #[doc = ""]
    pub domain_name: PrimField<String>,
    #[doc = ""]
    pub validation_domain: PrimField<String>,
}

impl BuildAcmCertificateValidationOptionEl {
    pub fn build(self) -> AcmCertificateValidationOptionEl {
        AcmCertificateValidationOptionEl {
            domain_name: self.domain_name,
            validation_domain: self.validation_domain,
        }
    }
}

pub struct AcmCertificateValidationOptionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AcmCertificateValidationOptionElRef {
    fn new(shared: StackShared, base: String) -> AcmCertificateValidationOptionElRef {
        AcmCertificateValidationOptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AcmCertificateValidationOptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `validation_domain` after provisioning.\n"]
    pub fn validation_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.validation_domain", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct AcmCertificateDynamic {
    options: Option<DynamicBlock<AcmCertificateOptionsEl>>,
    validation_option: Option<DynamicBlock<AcmCertificateValidationOptionEl>>,
}
