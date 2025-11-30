use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct SagemakerImageVersionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aliases: Option<SetField<PrimField<String>>>,
    base_image: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    horovod: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    image_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    job_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ml_framework: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    processor: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    programming_lang: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    release_notes: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vendor_guidance: Option<PrimField<String>>,
}

struct SagemakerImageVersion_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SagemakerImageVersionData>,
}

#[derive(Clone)]
pub struct SagemakerImageVersion(Rc<SagemakerImageVersion_>);

impl SagemakerImageVersion {
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

    #[doc = "Set the field `aliases`.\n"]
    pub fn set_aliases(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().aliases = Some(v.into());
        self
    }

    #[doc = "Set the field `horovod`.\n"]
    pub fn set_horovod(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().horovod = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `job_type`.\n"]
    pub fn set_job_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().job_type = Some(v.into());
        self
    }

    #[doc = "Set the field `ml_framework`.\n"]
    pub fn set_ml_framework(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ml_framework = Some(v.into());
        self
    }

    #[doc = "Set the field `processor`.\n"]
    pub fn set_processor(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().processor = Some(v.into());
        self
    }

    #[doc = "Set the field `programming_lang`.\n"]
    pub fn set_programming_lang(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().programming_lang = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `release_notes`.\n"]
    pub fn set_release_notes(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().release_notes = Some(v.into());
        self
    }

    #[doc = "Set the field `vendor_guidance`.\n"]
    pub fn set_vendor_guidance(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().vendor_guidance = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `aliases` after provisioning.\n"]
    pub fn aliases(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.aliases", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `base_image` after provisioning.\n"]
    pub fn base_image(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_image", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `container_image` after provisioning.\n"]
    pub fn container_image(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_image", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `horovod` after provisioning.\n"]
    pub fn horovod(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.horovod", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_arn` after provisioning.\n"]
    pub fn image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `image_name` after provisioning.\n"]
    pub fn image_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `job_type` after provisioning.\n"]
    pub fn job_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ml_framework` after provisioning.\n"]
    pub fn ml_framework(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ml_framework", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `processor` after provisioning.\n"]
    pub fn processor(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.processor", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `programming_lang` after provisioning.\n"]
    pub fn programming_lang(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.programming_lang", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `release_notes` after provisioning.\n"]
    pub fn release_notes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.release_notes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vendor_guidance` after provisioning.\n"]
    pub fn vendor_guidance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vendor_guidance", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
}

impl Referable for SagemakerImageVersion {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for SagemakerImageVersion {}

impl ToListMappable for SagemakerImageVersion {
    type O = ListRef<SagemakerImageVersionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SagemakerImageVersion_ {
    fn extract_resource_type(&self) -> String {
        "aws_sagemaker_image_version".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerImageVersion {
    pub tf_id: String,
    #[doc = ""]
    pub base_image: PrimField<String>,
    #[doc = ""]
    pub image_name: PrimField<String>,
}

impl BuildSagemakerImageVersion {
    pub fn build(self, stack: &mut Stack) -> SagemakerImageVersion {
        let out = SagemakerImageVersion(Rc::new(SagemakerImageVersion_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SagemakerImageVersionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                aliases: core::default::Default::default(),
                base_image: self.base_image,
                horovod: core::default::Default::default(),
                id: core::default::Default::default(),
                image_name: self.image_name,
                job_type: core::default::Default::default(),
                ml_framework: core::default::Default::default(),
                processor: core::default::Default::default(),
                programming_lang: core::default::Default::default(),
                region: core::default::Default::default(),
                release_notes: core::default::Default::default(),
                vendor_guidance: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerImageVersionRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerImageVersionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl SagemakerImageVersionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aliases` after provisioning.\n"]
    pub fn aliases(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.aliases", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `base_image` after provisioning.\n"]
    pub fn base_image(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_image", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `container_image` after provisioning.\n"]
    pub fn container_image(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_image", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `horovod` after provisioning.\n"]
    pub fn horovod(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.horovod", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_arn` after provisioning.\n"]
    pub fn image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `image_name` after provisioning.\n"]
    pub fn image_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `job_type` after provisioning.\n"]
    pub fn job_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ml_framework` after provisioning.\n"]
    pub fn ml_framework(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ml_framework", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `processor` after provisioning.\n"]
    pub fn processor(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.processor", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `programming_lang` after provisioning.\n"]
    pub fn programming_lang(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.programming_lang", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `release_notes` after provisioning.\n"]
    pub fn release_notes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.release_notes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vendor_guidance` after provisioning.\n"]
    pub fn vendor_guidance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vendor_guidance", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
}
