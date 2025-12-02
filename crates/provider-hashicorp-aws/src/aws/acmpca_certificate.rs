use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AcmpcaCertificateData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_passthrough: Option<PrimField<String>>,
    certificate_authority_arn: PrimField<String>,
    certificate_signing_request: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    signing_algorithm: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validity: Option<Vec<AcmpcaCertificateValidityEl>>,
    dynamic: AcmpcaCertificateDynamic,
}
struct AcmpcaCertificate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AcmpcaCertificateData>,
}
#[derive(Clone)]
pub struct AcmpcaCertificate(Rc<AcmpcaCertificate_>);
impl AcmpcaCertificate {
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
    #[doc = "Set the field `api_passthrough`.\n"]
    pub fn set_api_passthrough(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().api_passthrough = Some(v.into());
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
    #[doc = "Set the field `template_arn`.\n"]
    pub fn set_template_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().template_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `validity`.\n"]
    pub fn set_validity(self, v: impl Into<BlockAssignable<AcmpcaCertificateValidityEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().validity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.validity = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `api_passthrough` after provisioning.\n"]
    pub fn api_passthrough(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_passthrough", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `certificate_authority_arn` after provisioning.\n"]
    pub fn certificate_authority_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_authority_arn", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `signing_algorithm` after provisioning.\n"]
    pub fn signing_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.signing_algorithm", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template_arn` after provisioning.\n"]
    pub fn template_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `validity` after provisioning.\n"]
    pub fn validity(&self) -> ListRef<AcmpcaCertificateValidityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.validity", self.extract_ref()),
        )
    }
}
impl Referable for AcmpcaCertificate {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AcmpcaCertificate {}
impl ToListMappable for AcmpcaCertificate {
    type O = ListRef<AcmpcaCertificateRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AcmpcaCertificate_ {
    fn extract_resource_type(&self) -> String {
        "aws_acmpca_certificate".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAcmpcaCertificate {
    pub tf_id: String,
    #[doc = ""]
    pub certificate_authority_arn: PrimField<String>,
    #[doc = ""]
    pub certificate_signing_request: PrimField<String>,
    #[doc = ""]
    pub signing_algorithm: PrimField<String>,
}
impl BuildAcmpcaCertificate {
    pub fn build(self, stack: &mut Stack) -> AcmpcaCertificate {
        let out = AcmpcaCertificate(Rc::new(AcmpcaCertificate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AcmpcaCertificateData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                api_passthrough: core::default::Default::default(),
                certificate_authority_arn: self.certificate_authority_arn,
                certificate_signing_request: self.certificate_signing_request,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                signing_algorithm: self.signing_algorithm,
                template_arn: core::default::Default::default(),
                validity: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AcmpcaCertificateRef {
    shared: StackShared,
    base: String,
}
impl Ref for AcmpcaCertificateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AcmpcaCertificateRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_passthrough` after provisioning.\n"]
    pub fn api_passthrough(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_passthrough", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `certificate_authority_arn` after provisioning.\n"]
    pub fn certificate_authority_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_authority_arn", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `signing_algorithm` after provisioning.\n"]
    pub fn signing_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.signing_algorithm", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template_arn` after provisioning.\n"]
    pub fn template_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `validity` after provisioning.\n"]
    pub fn validity(&self) -> ListRef<AcmpcaCertificateValidityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.validity", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AcmpcaCertificateValidityEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    value: PrimField<String>,
}
impl AcmpcaCertificateValidityEl {}
impl ToListMappable for AcmpcaCertificateValidityEl {
    type O = BlockAssignable<AcmpcaCertificateValidityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAcmpcaCertificateValidityEl {
    #[doc = ""]
    pub type_: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildAcmpcaCertificateValidityEl {
    pub fn build(self) -> AcmpcaCertificateValidityEl {
        AcmpcaCertificateValidityEl {
            type_: self.type_,
            value: self.value,
        }
    }
}
pub struct AcmpcaCertificateValidityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AcmpcaCertificateValidityElRef {
    fn new(shared: StackShared, base: String) -> AcmpcaCertificateValidityElRef {
        AcmpcaCertificateValidityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AcmpcaCertificateValidityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct AcmpcaCertificateDynamic {
    validity: Option<DynamicBlock<AcmpcaCertificateValidityEl>>,
}
