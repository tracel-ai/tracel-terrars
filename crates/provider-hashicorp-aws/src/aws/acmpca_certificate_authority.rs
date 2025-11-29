use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AcmpcaCertificateAuthorityData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_storage_security_standard: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permanent_deletion_time_in_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_authority_configuration: Option<Vec<AcmpcaCertificateAuthorityCertificateAuthorityConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revocation_configuration: Option<Vec<AcmpcaCertificateAuthorityRevocationConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AcmpcaCertificateAuthorityTimeoutsEl>,
    dynamic: AcmpcaCertificateAuthorityDynamic,
}

struct AcmpcaCertificateAuthority_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AcmpcaCertificateAuthorityData>,
}

#[derive(Clone)]
pub struct AcmpcaCertificateAuthority(Rc<AcmpcaCertificateAuthority_>);

impl AcmpcaCertificateAuthority {
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

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `key_storage_security_standard`.\n"]
    pub fn set_key_storage_security_standard(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().key_storage_security_standard = Some(v.into());
        self
    }

    #[doc = "Set the field `permanent_deletion_time_in_days`.\n"]
    pub fn set_permanent_deletion_time_in_days(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().permanent_deletion_time_in_days = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
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

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `usage_mode`.\n"]
    pub fn set_usage_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().usage_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_authority_configuration`.\n"]
    pub fn set_certificate_authority_configuration(
        self,
        v: impl Into<BlockAssignable<AcmpcaCertificateAuthorityCertificateAuthorityConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().certificate_authority_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.certificate_authority_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `revocation_configuration`.\n"]
    pub fn set_revocation_configuration(
        self,
        v: impl Into<BlockAssignable<AcmpcaCertificateAuthorityRevocationConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().revocation_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.revocation_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<AcmpcaCertificateAuthorityTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_signing_request` after provisioning.\n"]
    pub fn certificate_signing_request(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_signing_request", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_storage_security_standard` after provisioning.\n"]
    pub fn key_storage_security_standard(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_storage_security_standard", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `not_after` after provisioning.\n"]
    pub fn not_after(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.not_after", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `not_before` after provisioning.\n"]
    pub fn not_before(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.not_before", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permanent_deletion_time_in_days` after provisioning.\n"]
    pub fn permanent_deletion_time_in_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.permanent_deletion_time_in_days", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `serial` after provisioning.\n"]
    pub fn serial(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.serial", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `usage_mode` after provisioning.\n"]
    pub fn usage_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.usage_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_authority_configuration` after provisioning.\n"]
    pub fn certificate_authority_configuration(
        &self,
    ) -> ListRef<AcmpcaCertificateAuthorityCertificateAuthorityConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.certificate_authority_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revocation_configuration` after provisioning.\n"]
    pub fn revocation_configuration(&self) -> ListRef<AcmpcaCertificateAuthorityRevocationConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.revocation_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AcmpcaCertificateAuthorityTimeoutsElRef {
        AcmpcaCertificateAuthorityTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for AcmpcaCertificateAuthority {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for AcmpcaCertificateAuthority { }

impl ToListMappable for AcmpcaCertificateAuthority {
    type O = ListRef<AcmpcaCertificateAuthorityRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AcmpcaCertificateAuthority_ {
    fn extract_resource_type(&self) -> String {
        "aws_acmpca_certificate_authority".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAcmpcaCertificateAuthority {
    pub tf_id: String,
}

impl BuildAcmpcaCertificateAuthority {
    pub fn build(self, stack: &mut Stack) -> AcmpcaCertificateAuthority {
        let out = AcmpcaCertificateAuthority(Rc::new(AcmpcaCertificateAuthority_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AcmpcaCertificateAuthorityData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                key_storage_security_standard: core::default::Default::default(),
                permanent_deletion_time_in_days: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: core::default::Default::default(),
                usage_mode: core::default::Default::default(),
                certificate_authority_configuration: core::default::Default::default(),
                revocation_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AcmpcaCertificateAuthorityRef {
    shared: StackShared,
    base: String,
}

impl Ref for AcmpcaCertificateAuthorityRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl AcmpcaCertificateAuthorityRef {
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

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_signing_request` after provisioning.\n"]
    pub fn certificate_signing_request(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_signing_request", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_storage_security_standard` after provisioning.\n"]
    pub fn key_storage_security_standard(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_storage_security_standard", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `not_after` after provisioning.\n"]
    pub fn not_after(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.not_after", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `not_before` after provisioning.\n"]
    pub fn not_before(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.not_before", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permanent_deletion_time_in_days` after provisioning.\n"]
    pub fn permanent_deletion_time_in_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.permanent_deletion_time_in_days", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `serial` after provisioning.\n"]
    pub fn serial(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.serial", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `usage_mode` after provisioning.\n"]
    pub fn usage_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.usage_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_authority_configuration` after provisioning.\n"]
    pub fn certificate_authority_configuration(
        &self,
    ) -> ListRef<AcmpcaCertificateAuthorityCertificateAuthorityConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.certificate_authority_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revocation_configuration` after provisioning.\n"]
    pub fn revocation_configuration(&self) -> ListRef<AcmpcaCertificateAuthorityRevocationConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.revocation_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AcmpcaCertificateAuthorityTimeoutsElRef {
        AcmpcaCertificateAuthorityTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct AcmpcaCertificateAuthorityCertificateAuthorityConfigurationElSubjectEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    common_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    distinguished_name_qualifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation_qualifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    given_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initials: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locality: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organization: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organizational_unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pseudonym: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    surname: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}

impl AcmpcaCertificateAuthorityCertificateAuthorityConfigurationElSubjectEl {
    #[doc = "Set the field `common_name`.\n"]
    pub fn set_common_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.common_name = Some(v.into());
        self
    }

    #[doc = "Set the field `country`.\n"]
    pub fn set_country(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.country = Some(v.into());
        self
    }

    #[doc = "Set the field `distinguished_name_qualifier`.\n"]
    pub fn set_distinguished_name_qualifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.distinguished_name_qualifier = Some(v.into());
        self
    }

    #[doc = "Set the field `generation_qualifier`.\n"]
    pub fn set_generation_qualifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.generation_qualifier = Some(v.into());
        self
    }

    #[doc = "Set the field `given_name`.\n"]
    pub fn set_given_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.given_name = Some(v.into());
        self
    }

    #[doc = "Set the field `initials`.\n"]
    pub fn set_initials(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.initials = Some(v.into());
        self
    }

    #[doc = "Set the field `locality`.\n"]
    pub fn set_locality(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.locality = Some(v.into());
        self
    }

    #[doc = "Set the field `organization`.\n"]
    pub fn set_organization(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.organization = Some(v.into());
        self
    }

    #[doc = "Set the field `organizational_unit`.\n"]
    pub fn set_organizational_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.organizational_unit = Some(v.into());
        self
    }

    #[doc = "Set the field `pseudonym`.\n"]
    pub fn set_pseudonym(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pseudonym = Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }

    #[doc = "Set the field `surname`.\n"]
    pub fn set_surname(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.surname = Some(v.into());
        self
    }

    #[doc = "Set the field `title`.\n"]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
}

impl ToListMappable for AcmpcaCertificateAuthorityCertificateAuthorityConfigurationElSubjectEl {
    type O = BlockAssignable<AcmpcaCertificateAuthorityCertificateAuthorityConfigurationElSubjectEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAcmpcaCertificateAuthorityCertificateAuthorityConfigurationElSubjectEl {}

impl BuildAcmpcaCertificateAuthorityCertificateAuthorityConfigurationElSubjectEl {
    pub fn build(self) -> AcmpcaCertificateAuthorityCertificateAuthorityConfigurationElSubjectEl {
        AcmpcaCertificateAuthorityCertificateAuthorityConfigurationElSubjectEl {
            common_name: core::default::Default::default(),
            country: core::default::Default::default(),
            distinguished_name_qualifier: core::default::Default::default(),
            generation_qualifier: core::default::Default::default(),
            given_name: core::default::Default::default(),
            initials: core::default::Default::default(),
            locality: core::default::Default::default(),
            organization: core::default::Default::default(),
            organizational_unit: core::default::Default::default(),
            pseudonym: core::default::Default::default(),
            state: core::default::Default::default(),
            surname: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}

pub struct AcmpcaCertificateAuthorityCertificateAuthorityConfigurationElSubjectElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AcmpcaCertificateAuthorityCertificateAuthorityConfigurationElSubjectElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AcmpcaCertificateAuthorityCertificateAuthorityConfigurationElSubjectElRef {
        AcmpcaCertificateAuthorityCertificateAuthorityConfigurationElSubjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AcmpcaCertificateAuthorityCertificateAuthorityConfigurationElSubjectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.base))
    }

    #[doc = "Get a reference to the value of field `country` after provisioning.\n"]
    pub fn country(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.country", self.base))
    }

    #[doc = "Get a reference to the value of field `distinguished_name_qualifier` after provisioning.\n"]
    pub fn distinguished_name_qualifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.distinguished_name_qualifier", self.base))
    }

    #[doc = "Get a reference to the value of field `generation_qualifier` after provisioning.\n"]
    pub fn generation_qualifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.generation_qualifier", self.base))
    }

    #[doc = "Get a reference to the value of field `given_name` after provisioning.\n"]
    pub fn given_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.given_name", self.base))
    }

    #[doc = "Get a reference to the value of field `initials` after provisioning.\n"]
    pub fn initials(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.initials", self.base))
    }

    #[doc = "Get a reference to the value of field `locality` after provisioning.\n"]
    pub fn locality(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.locality", self.base))
    }

    #[doc = "Get a reference to the value of field `organization` after provisioning.\n"]
    pub fn organization(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.organization", self.base))
    }

    #[doc = "Get a reference to the value of field `organizational_unit` after provisioning.\n"]
    pub fn organizational_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.organizational_unit", self.base))
    }

    #[doc = "Get a reference to the value of field `pseudonym` after provisioning.\n"]
    pub fn pseudonym(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pseudonym", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `surname` after provisioning.\n"]
    pub fn surname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.surname", self.base))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}

#[derive(Serialize, Default)]
struct AcmpcaCertificateAuthorityCertificateAuthorityConfigurationElDynamic {
    subject: Option<DynamicBlock<AcmpcaCertificateAuthorityCertificateAuthorityConfigurationElSubjectEl>>,
}

#[derive(Serialize)]
pub struct AcmpcaCertificateAuthorityCertificateAuthorityConfigurationEl {
    key_algorithm: PrimField<String>,
    signing_algorithm: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject: Option<Vec<AcmpcaCertificateAuthorityCertificateAuthorityConfigurationElSubjectEl>>,
    dynamic: AcmpcaCertificateAuthorityCertificateAuthorityConfigurationElDynamic,
}

impl AcmpcaCertificateAuthorityCertificateAuthorityConfigurationEl {
    #[doc = "Set the field `subject`.\n"]
    pub fn set_subject(
        mut self,
        v: impl Into<BlockAssignable<AcmpcaCertificateAuthorityCertificateAuthorityConfigurationElSubjectEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.subject = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.subject = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AcmpcaCertificateAuthorityCertificateAuthorityConfigurationEl {
    type O = BlockAssignable<AcmpcaCertificateAuthorityCertificateAuthorityConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAcmpcaCertificateAuthorityCertificateAuthorityConfigurationEl {
    #[doc = ""]
    pub key_algorithm: PrimField<String>,
    #[doc = ""]
    pub signing_algorithm: PrimField<String>,
}

impl BuildAcmpcaCertificateAuthorityCertificateAuthorityConfigurationEl {
    pub fn build(self) -> AcmpcaCertificateAuthorityCertificateAuthorityConfigurationEl {
        AcmpcaCertificateAuthorityCertificateAuthorityConfigurationEl {
            key_algorithm: self.key_algorithm,
            signing_algorithm: self.signing_algorithm,
            subject: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AcmpcaCertificateAuthorityCertificateAuthorityConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AcmpcaCertificateAuthorityCertificateAuthorityConfigurationElRef {
    fn new(shared: StackShared, base: String) -> AcmpcaCertificateAuthorityCertificateAuthorityConfigurationElRef {
        AcmpcaCertificateAuthorityCertificateAuthorityConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AcmpcaCertificateAuthorityCertificateAuthorityConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_algorithm` after provisioning.\n"]
    pub fn key_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_algorithm", self.base))
    }

    #[doc = "Get a reference to the value of field `signing_algorithm` after provisioning.\n"]
    pub fn signing_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.signing_algorithm", self.base))
    }

    #[doc = "Get a reference to the value of field `subject` after provisioning.\n"]
    pub fn subject(&self) -> ListRef<AcmpcaCertificateAuthorityCertificateAuthorityConfigurationElSubjectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.subject", self.base))
    }
}

#[derive(Serialize)]
pub struct AcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl {
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

impl AcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl {
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

impl ToListMappable for AcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl {
    type O = BlockAssignable<AcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl {}

impl BuildAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl {
    pub fn build(self) -> AcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl {
        AcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl {
            custom_cname: core::default::Default::default(),
            enabled: core::default::Default::default(),
            expiration_in_days: core::default::Default::default(),
            s3_bucket_name: core::default::Default::default(),
            s3_object_acl: core::default::Default::default(),
        }
    }
}

pub struct AcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationElRef {
        AcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationElRef {
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
        PrimExpr::new(self.shared().clone(), format!("{}.expiration_in_days", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_object_acl` after provisioning.\n"]
    pub fn s3_object_acl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_object_acl", self.base))
    }
}

#[derive(Serialize)]
pub struct AcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ocsp_custom_cname: Option<PrimField<String>>,
}

impl AcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl {
    #[doc = "Set the field `ocsp_custom_cname`.\n"]
    pub fn set_ocsp_custom_cname(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ocsp_custom_cname = Some(v.into());
        self
    }
}

impl ToListMappable for AcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl {
    type O = BlockAssignable<AcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}

impl BuildAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl {
    pub fn build(self) -> AcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl {
        AcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl {
            enabled: self.enabled,
            ocsp_custom_cname: core::default::Default::default(),
        }
    }
}

pub struct AcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationElRef {
        AcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `ocsp_custom_cname` after provisioning.\n"]
    pub fn ocsp_custom_cname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ocsp_custom_cname", self.base))
    }
}

#[derive(Serialize, Default)]
struct AcmpcaCertificateAuthorityRevocationConfigurationElDynamic {
    crl_configuration: Option<DynamicBlock<AcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl>>,
    ocsp_configuration: Option<
        DynamicBlock<AcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct AcmpcaCertificateAuthorityRevocationConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    crl_configuration: Option<Vec<AcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ocsp_configuration: Option<Vec<AcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl>>,
    dynamic: AcmpcaCertificateAuthorityRevocationConfigurationElDynamic,
}

impl AcmpcaCertificateAuthorityRevocationConfigurationEl {
    #[doc = "Set the field `crl_configuration`.\n"]
    pub fn set_crl_configuration(
        mut self,
        v: impl Into<BlockAssignable<AcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.crl_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.crl_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ocsp_configuration`.\n"]
    pub fn set_ocsp_configuration(
        mut self,
        v: impl Into<BlockAssignable<AcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ocsp_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ocsp_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AcmpcaCertificateAuthorityRevocationConfigurationEl {
    type O = BlockAssignable<AcmpcaCertificateAuthorityRevocationConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAcmpcaCertificateAuthorityRevocationConfigurationEl {}

impl BuildAcmpcaCertificateAuthorityRevocationConfigurationEl {
    pub fn build(self) -> AcmpcaCertificateAuthorityRevocationConfigurationEl {
        AcmpcaCertificateAuthorityRevocationConfigurationEl {
            crl_configuration: core::default::Default::default(),
            ocsp_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AcmpcaCertificateAuthorityRevocationConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AcmpcaCertificateAuthorityRevocationConfigurationElRef {
    fn new(shared: StackShared, base: String) -> AcmpcaCertificateAuthorityRevocationConfigurationElRef {
        AcmpcaCertificateAuthorityRevocationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AcmpcaCertificateAuthorityRevocationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `crl_configuration` after provisioning.\n"]
    pub fn crl_configuration(&self) -> ListRef<AcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.crl_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `ocsp_configuration` after provisioning.\n"]
    pub fn ocsp_configuration(
        &self,
    ) -> ListRef<AcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ocsp_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct AcmpcaCertificateAuthorityTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
}

impl AcmpcaCertificateAuthorityTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
}

impl ToListMappable for AcmpcaCertificateAuthorityTimeoutsEl {
    type O = BlockAssignable<AcmpcaCertificateAuthorityTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAcmpcaCertificateAuthorityTimeoutsEl {}

impl BuildAcmpcaCertificateAuthorityTimeoutsEl {
    pub fn build(self) -> AcmpcaCertificateAuthorityTimeoutsEl {
        AcmpcaCertificateAuthorityTimeoutsEl { create: core::default::Default::default() }
    }
}

pub struct AcmpcaCertificateAuthorityTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AcmpcaCertificateAuthorityTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> AcmpcaCertificateAuthorityTimeoutsElRef {
        AcmpcaCertificateAuthorityTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AcmpcaCertificateAuthorityTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}

#[derive(Serialize, Default)]
struct AcmpcaCertificateAuthorityDynamic {
    certificate_authority_configuration: Option<
        DynamicBlock<AcmpcaCertificateAuthorityCertificateAuthorityConfigurationEl>,
    >,
    revocation_configuration: Option<DynamicBlock<AcmpcaCertificateAuthorityRevocationConfigurationEl>>,
}
