use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct EcrRepositoryData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_delete: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_tag_mutability: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_configuration: Option<Vec<EcrRepositoryEncryptionConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_scanning_configuration: Option<Vec<EcrRepositoryImageScanningConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_tag_mutability_exclusion_filter:
        Option<Vec<EcrRepositoryImageTagMutabilityExclusionFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<EcrRepositoryTimeoutsEl>,
    dynamic: EcrRepositoryDynamic,
}
struct EcrRepository_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EcrRepositoryData>,
}
#[derive(Clone)]
pub struct EcrRepository(Rc<EcrRepository_>);
impl EcrRepository {
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
    #[doc = "Set the field `force_delete`.\n"]
    pub fn set_force_delete(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_delete = Some(v.into());
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
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
    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        self,
        v: impl Into<BlockAssignable<EcrRepositoryEncryptionConfigurationEl>>,
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
    #[doc = "Set the field `image_scanning_configuration`.\n"]
    pub fn set_image_scanning_configuration(
        self,
        v: impl Into<BlockAssignable<EcrRepositoryImageScanningConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().image_scanning_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .image_scanning_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `image_tag_mutability_exclusion_filter`.\n"]
    pub fn set_image_tag_mutability_exclusion_filter(
        self,
        v: impl Into<BlockAssignable<EcrRepositoryImageTagMutabilityExclusionFilterEl>>,
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
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<EcrRepositoryTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `force_delete` after provisioning.\n"]
    pub fn force_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_delete", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.registry_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `repository_url` after provisioning.\n"]
    pub fn repository_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_url", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> ListRef<EcrRepositoryEncryptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `image_scanning_configuration` after provisioning.\n"]
    pub fn image_scanning_configuration(
        &self,
    ) -> ListRef<EcrRepositoryImageScanningConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.image_scanning_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `image_tag_mutability_exclusion_filter` after provisioning.\n"]
    pub fn image_tag_mutability_exclusion_filter(
        &self,
    ) -> ListRef<EcrRepositoryImageTagMutabilityExclusionFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.image_tag_mutability_exclusion_filter",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EcrRepositoryTimeoutsElRef {
        EcrRepositoryTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for EcrRepository {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for EcrRepository {}
impl ToListMappable for EcrRepository {
    type O = ListRef<EcrRepositoryRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for EcrRepository_ {
    fn extract_resource_type(&self) -> String {
        "aws_ecr_repository".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildEcrRepository {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildEcrRepository {
    pub fn build(self, stack: &mut Stack) -> EcrRepository {
        let out = EcrRepository(Rc::new(EcrRepository_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EcrRepositoryData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                force_delete: core::default::Default::default(),
                id: core::default::Default::default(),
                image_tag_mutability: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                encryption_configuration: core::default::Default::default(),
                image_scanning_configuration: core::default::Default::default(),
                image_tag_mutability_exclusion_filter: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct EcrRepositoryRef {
    shared: StackShared,
    base: String,
}
impl Ref for EcrRepositoryRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl EcrRepositoryRef {
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
    #[doc = "Get a reference to the value of field `force_delete` after provisioning.\n"]
    pub fn force_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_delete", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.registry_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `repository_url` after provisioning.\n"]
    pub fn repository_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_url", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> ListRef<EcrRepositoryEncryptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `image_scanning_configuration` after provisioning.\n"]
    pub fn image_scanning_configuration(
        &self,
    ) -> ListRef<EcrRepositoryImageScanningConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.image_scanning_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `image_tag_mutability_exclusion_filter` after provisioning.\n"]
    pub fn image_tag_mutability_exclusion_filter(
        &self,
    ) -> ListRef<EcrRepositoryImageTagMutabilityExclusionFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.image_tag_mutability_exclusion_filter",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EcrRepositoryTimeoutsElRef {
        EcrRepositoryTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct EcrRepositoryEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key: Option<PrimField<String>>,
}
impl EcrRepositoryEncryptionConfigurationEl {
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
impl ToListMappable for EcrRepositoryEncryptionConfigurationEl {
    type O = BlockAssignable<EcrRepositoryEncryptionConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEcrRepositoryEncryptionConfigurationEl {}
impl BuildEcrRepositoryEncryptionConfigurationEl {
    pub fn build(self) -> EcrRepositoryEncryptionConfigurationEl {
        EcrRepositoryEncryptionConfigurationEl {
            encryption_type: core::default::Default::default(),
            kms_key: core::default::Default::default(),
        }
    }
}
pub struct EcrRepositoryEncryptionConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EcrRepositoryEncryptionConfigurationElRef {
    fn new(shared: StackShared, base: String) -> EcrRepositoryEncryptionConfigurationElRef {
        EcrRepositoryEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EcrRepositoryEncryptionConfigurationElRef {
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
pub struct EcrRepositoryImageScanningConfigurationEl {
    scan_on_push: PrimField<bool>,
}
impl EcrRepositoryImageScanningConfigurationEl {}
impl ToListMappable for EcrRepositoryImageScanningConfigurationEl {
    type O = BlockAssignable<EcrRepositoryImageScanningConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEcrRepositoryImageScanningConfigurationEl {
    #[doc = ""]
    pub scan_on_push: PrimField<bool>,
}
impl BuildEcrRepositoryImageScanningConfigurationEl {
    pub fn build(self) -> EcrRepositoryImageScanningConfigurationEl {
        EcrRepositoryImageScanningConfigurationEl {
            scan_on_push: self.scan_on_push,
        }
    }
}
pub struct EcrRepositoryImageScanningConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EcrRepositoryImageScanningConfigurationElRef {
    fn new(shared: StackShared, base: String) -> EcrRepositoryImageScanningConfigurationElRef {
        EcrRepositoryImageScanningConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EcrRepositoryImageScanningConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `scan_on_push` after provisioning.\n"]
    pub fn scan_on_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.scan_on_push", self.base))
    }
}
#[derive(Serialize)]
pub struct EcrRepositoryImageTagMutabilityExclusionFilterEl {
    filter: PrimField<String>,
    filter_type: PrimField<String>,
}
impl EcrRepositoryImageTagMutabilityExclusionFilterEl {}
impl ToListMappable for EcrRepositoryImageTagMutabilityExclusionFilterEl {
    type O = BlockAssignable<EcrRepositoryImageTagMutabilityExclusionFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEcrRepositoryImageTagMutabilityExclusionFilterEl {
    #[doc = ""]
    pub filter: PrimField<String>,
    #[doc = ""]
    pub filter_type: PrimField<String>,
}
impl BuildEcrRepositoryImageTagMutabilityExclusionFilterEl {
    pub fn build(self) -> EcrRepositoryImageTagMutabilityExclusionFilterEl {
        EcrRepositoryImageTagMutabilityExclusionFilterEl {
            filter: self.filter,
            filter_type: self.filter_type,
        }
    }
}
pub struct EcrRepositoryImageTagMutabilityExclusionFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EcrRepositoryImageTagMutabilityExclusionFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EcrRepositoryImageTagMutabilityExclusionFilterElRef {
        EcrRepositoryImageTagMutabilityExclusionFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EcrRepositoryImageTagMutabilityExclusionFilterElRef {
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
#[derive(Serialize)]
pub struct EcrRepositoryTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl EcrRepositoryTimeoutsEl {
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
}
impl ToListMappable for EcrRepositoryTimeoutsEl {
    type O = BlockAssignable<EcrRepositoryTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEcrRepositoryTimeoutsEl {}
impl BuildEcrRepositoryTimeoutsEl {
    pub fn build(self) -> EcrRepositoryTimeoutsEl {
        EcrRepositoryTimeoutsEl {
            delete: core::default::Default::default(),
        }
    }
}
pub struct EcrRepositoryTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EcrRepositoryTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> EcrRepositoryTimeoutsElRef {
        EcrRepositoryTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EcrRepositoryTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}
#[derive(Serialize, Default)]
struct EcrRepositoryDynamic {
    encryption_configuration: Option<DynamicBlock<EcrRepositoryEncryptionConfigurationEl>>,
    image_scanning_configuration: Option<DynamicBlock<EcrRepositoryImageScanningConfigurationEl>>,
    image_tag_mutability_exclusion_filter:
        Option<DynamicBlock<EcrRepositoryImageTagMutabilityExclusionFilterEl>>,
}
