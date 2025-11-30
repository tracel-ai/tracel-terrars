use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct LightsailLbCertificateData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    lb_name: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_alternative_names: Option<SetField<PrimField<String>>>,
}

struct LightsailLbCertificate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LightsailLbCertificateData>,
}

#[derive(Clone)]
pub struct LightsailLbCertificate(Rc<LightsailLbCertificate_>);

impl LightsailLbCertificate {
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

    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().domain_name = Some(v.into());
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

    #[doc = "Set the field `subject_alternative_names`.\n"]
    pub fn set_subject_alternative_names(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().subject_alternative_names = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_validation_records` after provisioning.\n"]
    pub fn domain_validation_records(
        &self,
    ) -> SetRef<LightsailLbCertificateDomainValidationRecordsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.domain_validation_records", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lb_name` after provisioning.\n"]
    pub fn lb_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lb_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subject_alternative_names` after provisioning.\n"]
    pub fn subject_alternative_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subject_alternative_names", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `support_code` after provisioning.\n"]
    pub fn support_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.support_code", self.extract_ref()),
        )
    }
}

impl Referable for LightsailLbCertificate {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for LightsailLbCertificate {}

impl ToListMappable for LightsailLbCertificate {
    type O = ListRef<LightsailLbCertificateRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for LightsailLbCertificate_ {
    fn extract_resource_type(&self) -> String {
        "aws_lightsail_lb_certificate".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLightsailLbCertificate {
    pub tf_id: String,
    #[doc = ""]
    pub lb_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildLightsailLbCertificate {
    pub fn build(self, stack: &mut Stack) -> LightsailLbCertificate {
        let out = LightsailLbCertificate(Rc::new(LightsailLbCertificate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LightsailLbCertificateData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                domain_name: core::default::Default::default(),
                id: core::default::Default::default(),
                lb_name: self.lb_name,
                name: self.name,
                region: core::default::Default::default(),
                subject_alternative_names: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LightsailLbCertificateRef {
    shared: StackShared,
    base: String,
}

impl Ref for LightsailLbCertificateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl LightsailLbCertificateRef {
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

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_validation_records` after provisioning.\n"]
    pub fn domain_validation_records(
        &self,
    ) -> SetRef<LightsailLbCertificateDomainValidationRecordsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.domain_validation_records", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lb_name` after provisioning.\n"]
    pub fn lb_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lb_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subject_alternative_names` after provisioning.\n"]
    pub fn subject_alternative_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subject_alternative_names", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `support_code` after provisioning.\n"]
    pub fn support_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.support_code", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct LightsailLbCertificateDomainValidationRecordsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_record_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_record_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_record_value: Option<PrimField<String>>,
}

impl LightsailLbCertificateDomainValidationRecordsEl {
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

impl ToListMappable for LightsailLbCertificateDomainValidationRecordsEl {
    type O = BlockAssignable<LightsailLbCertificateDomainValidationRecordsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLightsailLbCertificateDomainValidationRecordsEl {}

impl BuildLightsailLbCertificateDomainValidationRecordsEl {
    pub fn build(self) -> LightsailLbCertificateDomainValidationRecordsEl {
        LightsailLbCertificateDomainValidationRecordsEl {
            domain_name: core::default::Default::default(),
            resource_record_name: core::default::Default::default(),
            resource_record_type: core::default::Default::default(),
            resource_record_value: core::default::Default::default(),
        }
    }
}

pub struct LightsailLbCertificateDomainValidationRecordsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LightsailLbCertificateDomainValidationRecordsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LightsailLbCertificateDomainValidationRecordsElRef {
        LightsailLbCertificateDomainValidationRecordsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LightsailLbCertificateDomainValidationRecordsElRef {
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
