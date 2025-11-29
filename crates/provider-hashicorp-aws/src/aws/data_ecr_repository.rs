use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEcrRepositoryData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registry_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataEcrRepository_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEcrRepositoryData>,
}

#[derive(Clone)]
pub struct DataEcrRepository(Rc<DataEcrRepository_>);

impl DataEcrRepository {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `registry_id`.\n"]
    pub fn set_registry_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().registry_id = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> ListRef<DataEcrRepositoryEncryptionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_scanning_configuration` after provisioning.\n"]
    pub fn image_scanning_configuration(&self) -> ListRef<DataEcrRepositoryImageScanningConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image_scanning_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tag_mutability` after provisioning.\n"]
    pub fn image_tag_mutability(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_tag_mutability", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tag_mutability_exclusion_filter` after provisioning.\n"]
    pub fn image_tag_mutability_exclusion_filter(
        &self,
    ) -> ListRef<DataEcrRepositoryImageTagMutabilityExclusionFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image_tag_mutability_exclusion_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `most_recent_image_tags` after provisioning.\n"]
    pub fn most_recent_image_tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.most_recent_image_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository_url` after provisioning.\n"]
    pub fn repository_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl Referable for DataEcrRepository {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataEcrRepository { }

impl ToListMappable for DataEcrRepository {
    type O = ListRef<DataEcrRepositoryRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataEcrRepository_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ecr_repository".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEcrRepository {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildDataEcrRepository {
    pub fn build(self, stack: &mut Stack) -> DataEcrRepository {
        let out = DataEcrRepository(Rc::new(DataEcrRepository_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEcrRepositoryData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                registry_id: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEcrRepositoryRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcrRepositoryRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataEcrRepositoryRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> ListRef<DataEcrRepositoryEncryptionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_scanning_configuration` after provisioning.\n"]
    pub fn image_scanning_configuration(&self) -> ListRef<DataEcrRepositoryImageScanningConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image_scanning_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tag_mutability` after provisioning.\n"]
    pub fn image_tag_mutability(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_tag_mutability", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tag_mutability_exclusion_filter` after provisioning.\n"]
    pub fn image_tag_mutability_exclusion_filter(
        &self,
    ) -> ListRef<DataEcrRepositoryImageTagMutabilityExclusionFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image_tag_mutability_exclusion_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `most_recent_image_tags` after provisioning.\n"]
    pub fn most_recent_image_tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.most_recent_image_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository_url` after provisioning.\n"]
    pub fn repository_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEcrRepositoryEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key: Option<PrimField<String>>,
}

impl DataEcrRepositoryEncryptionConfigurationEl {
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

impl ToListMappable for DataEcrRepositoryEncryptionConfigurationEl {
    type O = BlockAssignable<DataEcrRepositoryEncryptionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcrRepositoryEncryptionConfigurationEl {}

impl BuildDataEcrRepositoryEncryptionConfigurationEl {
    pub fn build(self) -> DataEcrRepositoryEncryptionConfigurationEl {
        DataEcrRepositoryEncryptionConfigurationEl {
            encryption_type: core::default::Default::default(),
            kms_key: core::default::Default::default(),
        }
    }
}

pub struct DataEcrRepositoryEncryptionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcrRepositoryEncryptionConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataEcrRepositoryEncryptionConfigurationElRef {
        DataEcrRepositoryEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcrRepositoryEncryptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_type` after provisioning.\n"]
    pub fn encryption_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption_type", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\n"]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcrRepositoryImageScanningConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    scan_on_push: Option<PrimField<bool>>,
}

impl DataEcrRepositoryImageScanningConfigurationEl {
    #[doc = "Set the field `scan_on_push`.\n"]
    pub fn set_scan_on_push(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.scan_on_push = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcrRepositoryImageScanningConfigurationEl {
    type O = BlockAssignable<DataEcrRepositoryImageScanningConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcrRepositoryImageScanningConfigurationEl {}

impl BuildDataEcrRepositoryImageScanningConfigurationEl {
    pub fn build(self) -> DataEcrRepositoryImageScanningConfigurationEl {
        DataEcrRepositoryImageScanningConfigurationEl { scan_on_push: core::default::Default::default() }
    }
}

pub struct DataEcrRepositoryImageScanningConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcrRepositoryImageScanningConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataEcrRepositoryImageScanningConfigurationElRef {
        DataEcrRepositoryImageScanningConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcrRepositoryImageScanningConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scan_on_push` after provisioning.\n"]
    pub fn scan_on_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.scan_on_push", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcrRepositoryImageTagMutabilityExclusionFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_type: Option<PrimField<String>>,
}

impl DataEcrRepositoryImageTagMutabilityExclusionFilterEl {
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

impl ToListMappable for DataEcrRepositoryImageTagMutabilityExclusionFilterEl {
    type O = BlockAssignable<DataEcrRepositoryImageTagMutabilityExclusionFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcrRepositoryImageTagMutabilityExclusionFilterEl {}

impl BuildDataEcrRepositoryImageTagMutabilityExclusionFilterEl {
    pub fn build(self) -> DataEcrRepositoryImageTagMutabilityExclusionFilterEl {
        DataEcrRepositoryImageTagMutabilityExclusionFilterEl {
            filter: core::default::Default::default(),
            filter_type: core::default::Default::default(),
        }
    }
}

pub struct DataEcrRepositoryImageTagMutabilityExclusionFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcrRepositoryImageTagMutabilityExclusionFilterElRef {
    fn new(shared: StackShared, base: String) -> DataEcrRepositoryImageTagMutabilityExclusionFilterElRef {
        DataEcrRepositoryImageTagMutabilityExclusionFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcrRepositoryImageTagMutabilityExclusionFilterElRef {
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
