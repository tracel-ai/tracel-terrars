use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct EcrRepositoryCreationTemplateData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    applied_for: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_tag_mutability: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_policy: Option<PrimField<String>>,
    prefix: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_configuration: Option<Vec<EcrRepositoryCreationTemplateEncryptionConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_tag_mutability_exclusion_filter:
        Option<Vec<EcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterEl>>,
    dynamic: EcrRepositoryCreationTemplateDynamic,
}
struct EcrRepositoryCreationTemplate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EcrRepositoryCreationTemplateData>,
}
#[derive(Clone)]
pub struct EcrRepositoryCreationTemplate(Rc<EcrRepositoryCreationTemplate_>);
impl EcrRepositoryCreationTemplate {
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
    #[doc = "Set the field `custom_role_arn`.\n"]
    pub fn set_custom_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().custom_role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `image_tag_mutability`.\n"]
    pub fn set_image_tag_mutability(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().image_tag_mutability = Some(v.into());
        self
    }
    #[doc = "Set the field `lifecycle_policy`.\n"]
    pub fn set_lifecycle_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().lifecycle_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `repository_policy`.\n"]
    pub fn set_repository_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().repository_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_tags`.\n"]
    pub fn set_resource_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().resource_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        self,
        v: impl Into<BlockAssignable<EcrRepositoryCreationTemplateEncryptionConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `image_tag_mutability_exclusion_filter`.\n"]
    pub fn set_image_tag_mutability_exclusion_filter(
        self,
        v: impl Into<BlockAssignable<EcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0
                    .data
                    .borrow_mut()
                    .image_tag_mutability_exclusion_filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .image_tag_mutability_exclusion_filter = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `applied_for` after provisioning.\n"]
    pub fn applied_for(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.applied_for", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_role_arn` after provisioning.\n"]
    pub fn custom_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `image_tag_mutability` after provisioning.\n"]
    pub fn image_tag_mutability(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_tag_mutability", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lifecycle_policy` after provisioning.\n"]
    pub fn lifecycle_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifecycle_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.registry_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `repository_policy` after provisioning.\n"]
    pub fn repository_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_tags` after provisioning.\n"]
    pub fn resource_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> ListRef<EcrRepositoryCreationTemplateEncryptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `image_tag_mutability_exclusion_filter` after provisioning.\n"]
    pub fn image_tag_mutability_exclusion_filter(
        &self,
    ) -> ListRef<EcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.image_tag_mutability_exclusion_filter",
                self.extract_ref()
            ),
        )
    }
}
impl Referable for EcrRepositoryCreationTemplate {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for EcrRepositoryCreationTemplate {}
impl ToListMappable for EcrRepositoryCreationTemplate {
    type O = ListRef<EcrRepositoryCreationTemplateRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for EcrRepositoryCreationTemplate_ {
    fn extract_resource_type(&self) -> String {
        "aws_ecr_repository_creation_template".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildEcrRepositoryCreationTemplate {
    pub tf_id: String,
    #[doc = ""]
    pub applied_for: SetField<PrimField<String>>,
    #[doc = ""]
    pub prefix: PrimField<String>,
}
impl BuildEcrRepositoryCreationTemplate {
    pub fn build(self, stack: &mut Stack) -> EcrRepositoryCreationTemplate {
        let out = EcrRepositoryCreationTemplate(Rc::new(EcrRepositoryCreationTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EcrRepositoryCreationTemplateData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                applied_for: self.applied_for,
                custom_role_arn: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                image_tag_mutability: core::default::Default::default(),
                lifecycle_policy: core::default::Default::default(),
                prefix: self.prefix,
                region: core::default::Default::default(),
                repository_policy: core::default::Default::default(),
                resource_tags: core::default::Default::default(),
                encryption_configuration: core::default::Default::default(),
                image_tag_mutability_exclusion_filter: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct EcrRepositoryCreationTemplateRef {
    shared: StackShared,
    base: String,
}
impl Ref for EcrRepositoryCreationTemplateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl EcrRepositoryCreationTemplateRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `applied_for` after provisioning.\n"]
    pub fn applied_for(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.applied_for", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_role_arn` after provisioning.\n"]
    pub fn custom_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `image_tag_mutability` after provisioning.\n"]
    pub fn image_tag_mutability(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_tag_mutability", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lifecycle_policy` after provisioning.\n"]
    pub fn lifecycle_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifecycle_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.registry_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `repository_policy` after provisioning.\n"]
    pub fn repository_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_tags` after provisioning.\n"]
    pub fn resource_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> ListRef<EcrRepositoryCreationTemplateEncryptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `image_tag_mutability_exclusion_filter` after provisioning.\n"]
    pub fn image_tag_mutability_exclusion_filter(
        &self,
    ) -> ListRef<EcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.image_tag_mutability_exclusion_filter",
                self.extract_ref()
            ),
        )
    }
}
#[derive(Serialize)]
pub struct EcrRepositoryCreationTemplateEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key: Option<PrimField<String>>,
}
impl EcrRepositoryCreationTemplateEncryptionConfigurationEl {
    #[doc = "Set the field `encryption_type`.\n"]
    pub fn set_encryption_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_type = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key`.\n"]
    pub fn set_kms_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key = Some(v.into());
        self
    }
}
impl ToListMappable for EcrRepositoryCreationTemplateEncryptionConfigurationEl {
    type O = BlockAssignable<EcrRepositoryCreationTemplateEncryptionConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEcrRepositoryCreationTemplateEncryptionConfigurationEl {}
impl BuildEcrRepositoryCreationTemplateEncryptionConfigurationEl {
    pub fn build(self) -> EcrRepositoryCreationTemplateEncryptionConfigurationEl {
        EcrRepositoryCreationTemplateEncryptionConfigurationEl {
            encryption_type: core::default::Default::default(),
            kms_key: core::default::Default::default(),
        }
    }
}
pub struct EcrRepositoryCreationTemplateEncryptionConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EcrRepositoryCreationTemplateEncryptionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EcrRepositoryCreationTemplateEncryptionConfigurationElRef {
        EcrRepositoryCreationTemplateEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EcrRepositoryCreationTemplateEncryptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `encryption_type` after provisioning.\n"]
    pub fn encryption_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\n"]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
}
#[derive(Serialize)]
pub struct EcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterEl {
    filter: PrimField<String>,
    filter_type: PrimField<String>,
}
impl EcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterEl {}
impl ToListMappable for EcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterEl {
    type O = BlockAssignable<EcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterEl {
    #[doc = ""]
    pub filter: PrimField<String>,
    #[doc = ""]
    pub filter_type: PrimField<String>,
}
impl BuildEcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterEl {
    pub fn build(self) -> EcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterEl {
        EcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterEl {
            filter: self.filter,
            filter_type: self.filter_type,
        }
    }
}
pub struct EcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterElRef {
        EcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
    #[doc = "Get a reference to the value of field `filter_type` after provisioning.\n"]
    pub fn filter_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter_type", self.base))
    }
}
#[derive(Serialize, Default)]
struct EcrRepositoryCreationTemplateDynamic {
    encryption_configuration:
        Option<DynamicBlock<EcrRepositoryCreationTemplateEncryptionConfigurationEl>>,
    image_tag_mutability_exclusion_filter:
        Option<DynamicBlock<EcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterEl>>,
}
