use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct GlueSecurityConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_configuration: Option<Vec<GlueSecurityConfigurationEncryptionConfigurationEl>>,
    dynamic: GlueSecurityConfigurationDynamic,
}
struct GlueSecurityConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GlueSecurityConfigurationData>,
}
#[derive(Clone)]
pub struct GlueSecurityConfiguration(Rc<GlueSecurityConfiguration_>);
impl GlueSecurityConfiguration {
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
    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        self,
        v: impl Into<BlockAssignable<GlueSecurityConfigurationEncryptionConfigurationEl>>,
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> ListRef<GlueSecurityConfigurationEncryptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.extract_ref()),
        )
    }
}
impl Referable for GlueSecurityConfiguration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for GlueSecurityConfiguration {}
impl ToListMappable for GlueSecurityConfiguration {
    type O = ListRef<GlueSecurityConfigurationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for GlueSecurityConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_glue_security_configuration".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildGlueSecurityConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildGlueSecurityConfiguration {
    pub fn build(self, stack: &mut Stack) -> GlueSecurityConfiguration {
        let out = GlueSecurityConfiguration(Rc::new(GlueSecurityConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GlueSecurityConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                encryption_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct GlueSecurityConfigurationRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueSecurityConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl GlueSecurityConfigurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> ListRef<GlueSecurityConfigurationEncryptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct GlueSecurityConfigurationEncryptionConfigurationElCloudwatchEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_encryption_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
}
impl GlueSecurityConfigurationEncryptionConfigurationElCloudwatchEncryptionEl {
    #[doc = "Set the field `cloudwatch_encryption_mode`.\n"]
    pub fn set_cloudwatch_encryption_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cloudwatch_encryption_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_arn = Some(v.into());
        self
    }
}
impl ToListMappable for GlueSecurityConfigurationEncryptionConfigurationElCloudwatchEncryptionEl {
    type O =
        BlockAssignable<GlueSecurityConfigurationEncryptionConfigurationElCloudwatchEncryptionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueSecurityConfigurationEncryptionConfigurationElCloudwatchEncryptionEl {}
impl BuildGlueSecurityConfigurationEncryptionConfigurationElCloudwatchEncryptionEl {
    pub fn build(self) -> GlueSecurityConfigurationEncryptionConfigurationElCloudwatchEncryptionEl {
        GlueSecurityConfigurationEncryptionConfigurationElCloudwatchEncryptionEl {
            cloudwatch_encryption_mode: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
        }
    }
}
pub struct GlueSecurityConfigurationEncryptionConfigurationElCloudwatchEncryptionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueSecurityConfigurationEncryptionConfigurationElCloudwatchEncryptionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GlueSecurityConfigurationEncryptionConfigurationElCloudwatchEncryptionElRef {
        GlueSecurityConfigurationEncryptionConfigurationElCloudwatchEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueSecurityConfigurationEncryptionConfigurationElCloudwatchEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloudwatch_encryption_mode` after provisioning.\n"]
    pub fn cloudwatch_encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloudwatch_encryption_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct GlueSecurityConfigurationEncryptionConfigurationElJobBookmarksEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    job_bookmarks_encryption_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
}
impl GlueSecurityConfigurationEncryptionConfigurationElJobBookmarksEncryptionEl {
    #[doc = "Set the field `job_bookmarks_encryption_mode`.\n"]
    pub fn set_job_bookmarks_encryption_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.job_bookmarks_encryption_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_arn = Some(v.into());
        self
    }
}
impl ToListMappable for GlueSecurityConfigurationEncryptionConfigurationElJobBookmarksEncryptionEl {
    type O =
        BlockAssignable<GlueSecurityConfigurationEncryptionConfigurationElJobBookmarksEncryptionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueSecurityConfigurationEncryptionConfigurationElJobBookmarksEncryptionEl {}
impl BuildGlueSecurityConfigurationEncryptionConfigurationElJobBookmarksEncryptionEl {
    pub fn build(
        self,
    ) -> GlueSecurityConfigurationEncryptionConfigurationElJobBookmarksEncryptionEl {
        GlueSecurityConfigurationEncryptionConfigurationElJobBookmarksEncryptionEl {
            job_bookmarks_encryption_mode: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
        }
    }
}
pub struct GlueSecurityConfigurationEncryptionConfigurationElJobBookmarksEncryptionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueSecurityConfigurationEncryptionConfigurationElJobBookmarksEncryptionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GlueSecurityConfigurationEncryptionConfigurationElJobBookmarksEncryptionElRef {
        GlueSecurityConfigurationEncryptionConfigurationElJobBookmarksEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueSecurityConfigurationEncryptionConfigurationElJobBookmarksEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `job_bookmarks_encryption_mode` after provisioning.\n"]
    pub fn job_bookmarks_encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_bookmarks_encryption_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct GlueSecurityConfigurationEncryptionConfigurationElS3EncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_encryption_mode: Option<PrimField<String>>,
}
impl GlueSecurityConfigurationEncryptionConfigurationElS3EncryptionEl {
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `s3_encryption_mode`.\n"]
    pub fn set_s3_encryption_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_encryption_mode = Some(v.into());
        self
    }
}
impl ToListMappable for GlueSecurityConfigurationEncryptionConfigurationElS3EncryptionEl {
    type O = BlockAssignable<GlueSecurityConfigurationEncryptionConfigurationElS3EncryptionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueSecurityConfigurationEncryptionConfigurationElS3EncryptionEl {}
impl BuildGlueSecurityConfigurationEncryptionConfigurationElS3EncryptionEl {
    pub fn build(self) -> GlueSecurityConfigurationEncryptionConfigurationElS3EncryptionEl {
        GlueSecurityConfigurationEncryptionConfigurationElS3EncryptionEl {
            kms_key_arn: core::default::Default::default(),
            s3_encryption_mode: core::default::Default::default(),
        }
    }
}
pub struct GlueSecurityConfigurationEncryptionConfigurationElS3EncryptionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueSecurityConfigurationEncryptionConfigurationElS3EncryptionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GlueSecurityConfigurationEncryptionConfigurationElS3EncryptionElRef {
        GlueSecurityConfigurationEncryptionConfigurationElS3EncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueSecurityConfigurationEncryptionConfigurationElS3EncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `s3_encryption_mode` after provisioning.\n"]
    pub fn s3_encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_encryption_mode", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct GlueSecurityConfigurationEncryptionConfigurationElDynamic {
    cloudwatch_encryption: Option<
        DynamicBlock<GlueSecurityConfigurationEncryptionConfigurationElCloudwatchEncryptionEl>,
    >,
    job_bookmarks_encryption: Option<
        DynamicBlock<GlueSecurityConfigurationEncryptionConfigurationElJobBookmarksEncryptionEl>,
    >,
    s3_encryption:
        Option<DynamicBlock<GlueSecurityConfigurationEncryptionConfigurationElS3EncryptionEl>>,
}
#[derive(Serialize)]
pub struct GlueSecurityConfigurationEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_encryption:
        Option<Vec<GlueSecurityConfigurationEncryptionConfigurationElCloudwatchEncryptionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    job_bookmarks_encryption:
        Option<Vec<GlueSecurityConfigurationEncryptionConfigurationElJobBookmarksEncryptionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_encryption: Option<Vec<GlueSecurityConfigurationEncryptionConfigurationElS3EncryptionEl>>,
    dynamic: GlueSecurityConfigurationEncryptionConfigurationElDynamic,
}
impl GlueSecurityConfigurationEncryptionConfigurationEl {
    #[doc = "Set the field `cloudwatch_encryption`.\n"]
    pub fn set_cloudwatch_encryption(
        mut self,
        v: impl Into<
            BlockAssignable<
                GlueSecurityConfigurationEncryptionConfigurationElCloudwatchEncryptionEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_encryption = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_encryption = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `job_bookmarks_encryption`.\n"]
    pub fn set_job_bookmarks_encryption(
        mut self,
        v: impl Into<
            BlockAssignable<
                GlueSecurityConfigurationEncryptionConfigurationElJobBookmarksEncryptionEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.job_bookmarks_encryption = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.job_bookmarks_encryption = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `s3_encryption`.\n"]
    pub fn set_s3_encryption(
        mut self,
        v: impl Into<BlockAssignable<GlueSecurityConfigurationEncryptionConfigurationElS3EncryptionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_encryption = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_encryption = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GlueSecurityConfigurationEncryptionConfigurationEl {
    type O = BlockAssignable<GlueSecurityConfigurationEncryptionConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueSecurityConfigurationEncryptionConfigurationEl {}
impl BuildGlueSecurityConfigurationEncryptionConfigurationEl {
    pub fn build(self) -> GlueSecurityConfigurationEncryptionConfigurationEl {
        GlueSecurityConfigurationEncryptionConfigurationEl {
            cloudwatch_encryption: core::default::Default::default(),
            job_bookmarks_encryption: core::default::Default::default(),
            s3_encryption: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GlueSecurityConfigurationEncryptionConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueSecurityConfigurationEncryptionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GlueSecurityConfigurationEncryptionConfigurationElRef {
        GlueSecurityConfigurationEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueSecurityConfigurationEncryptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloudwatch_encryption` after provisioning.\n"]
    pub fn cloudwatch_encryption(
        &self,
    ) -> ListRef<GlueSecurityConfigurationEncryptionConfigurationElCloudwatchEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloudwatch_encryption", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `job_bookmarks_encryption` after provisioning.\n"]
    pub fn job_bookmarks_encryption(
        &self,
    ) -> ListRef<GlueSecurityConfigurationEncryptionConfigurationElJobBookmarksEncryptionElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.job_bookmarks_encryption", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3_encryption` after provisioning.\n"]
    pub fn s3_encryption(
        &self,
    ) -> ListRef<GlueSecurityConfigurationEncryptionConfigurationElS3EncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_encryption", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct GlueSecurityConfigurationDynamic {
    encryption_configuration:
        Option<DynamicBlock<GlueSecurityConfigurationEncryptionConfigurationEl>>,
}
