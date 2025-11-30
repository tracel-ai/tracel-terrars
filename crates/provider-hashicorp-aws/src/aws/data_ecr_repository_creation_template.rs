use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataEcrRepositoryCreationTemplateData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    prefix: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_tags: Option<RecField<PrimField<String>>>,
}

struct DataEcrRepositoryCreationTemplate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEcrRepositoryCreationTemplateData>,
}

#[derive(Clone)]
pub struct DataEcrRepositoryCreationTemplate(Rc<DataEcrRepositoryCreationTemplate_>);

impl DataEcrRepositoryCreationTemplate {
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

    #[doc = "Set the field `resource_tags`.\n"]
    pub fn set_resource_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().resource_tags = Some(v.into());
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

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> ListRef<DataEcrRepositoryCreationTemplateEncryptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `image_tag_mutability_exclusion_filter` after provisioning.\n"]
    pub fn image_tag_mutability_exclusion_filter(
        &self,
    ) -> ListRef<DataEcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.image_tag_mutability_exclusion_filter",
                self.extract_ref()
            ),
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
}

impl Referable for DataEcrRepositoryCreationTemplate {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataEcrRepositoryCreationTemplate {}

impl ToListMappable for DataEcrRepositoryCreationTemplate {
    type O = ListRef<DataEcrRepositoryCreationTemplateRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataEcrRepositoryCreationTemplate_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ecr_repository_creation_template".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEcrRepositoryCreationTemplate {
    pub tf_id: String,
    #[doc = ""]
    pub prefix: PrimField<String>,
}

impl BuildDataEcrRepositoryCreationTemplate {
    pub fn build(self, stack: &mut Stack) -> DataEcrRepositoryCreationTemplate {
        let out = DataEcrRepositoryCreationTemplate(Rc::new(DataEcrRepositoryCreationTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEcrRepositoryCreationTemplateData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                prefix: self.prefix,
                region: core::default::Default::default(),
                resource_tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEcrRepositoryCreationTemplateRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcrRepositoryCreationTemplateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataEcrRepositoryCreationTemplateRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
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

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> ListRef<DataEcrRepositoryCreationTemplateEncryptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `image_tag_mutability_exclusion_filter` after provisioning.\n"]
    pub fn image_tag_mutability_exclusion_filter(
        &self,
    ) -> ListRef<DataEcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.image_tag_mutability_exclusion_filter",
                self.extract_ref()
            ),
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
}

#[derive(Serialize)]
pub struct DataEcrRepositoryCreationTemplateEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key: Option<PrimField<String>>,
}

impl DataEcrRepositoryCreationTemplateEncryptionConfigurationEl {
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

impl ToListMappable for DataEcrRepositoryCreationTemplateEncryptionConfigurationEl {
    type O = BlockAssignable<DataEcrRepositoryCreationTemplateEncryptionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcrRepositoryCreationTemplateEncryptionConfigurationEl {}

impl BuildDataEcrRepositoryCreationTemplateEncryptionConfigurationEl {
    pub fn build(self) -> DataEcrRepositoryCreationTemplateEncryptionConfigurationEl {
        DataEcrRepositoryCreationTemplateEncryptionConfigurationEl {
            encryption_type: core::default::Default::default(),
            kms_key: core::default::Default::default(),
        }
    }
}

pub struct DataEcrRepositoryCreationTemplateEncryptionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcrRepositoryCreationTemplateEncryptionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEcrRepositoryCreationTemplateEncryptionConfigurationElRef {
        DataEcrRepositoryCreationTemplateEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcrRepositoryCreationTemplateEncryptionConfigurationElRef {
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
pub struct DataEcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_type: Option<PrimField<String>>,
}

impl DataEcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterEl {
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter = Some(v.into());
        self
    }

    #[doc = "Set the field `filter_type`.\n"]
    pub fn set_filter_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter_type = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterEl {
    type O = BlockAssignable<DataEcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterEl {}

impl BuildDataEcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterEl {
    pub fn build(self) -> DataEcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterEl {
        DataEcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterEl {
            filter: core::default::Default::default(),
            filter_type: core::default::Default::default(),
        }
    }
}

pub struct DataEcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterElRef {
        DataEcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcrRepositoryCreationTemplateImageTagMutabilityExclusionFilterElRef {
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
