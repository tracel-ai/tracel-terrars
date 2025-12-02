use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct S3BucketReplicationConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    role: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<Vec<S3BucketReplicationConfigurationRuleEl>>,
    dynamic: S3BucketReplicationConfigurationDynamic,
}
struct S3BucketReplicationConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<S3BucketReplicationConfigurationData>,
}
#[derive(Clone)]
pub struct S3BucketReplicationConfiguration(Rc<S3BucketReplicationConfiguration_>);
impl S3BucketReplicationConfiguration {
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
    #[doc = "Set the field `token`.\n"]
    pub fn set_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().token = Some(v.into());
        self
    }
    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        self,
        v: impl Into<BlockAssignable<S3BucketReplicationConfigurationRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.token", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<S3BucketReplicationConfigurationRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule", self.extract_ref()),
        )
    }
}
impl Referable for S3BucketReplicationConfiguration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for S3BucketReplicationConfiguration {}
impl ToListMappable for S3BucketReplicationConfiguration {
    type O = ListRef<S3BucketReplicationConfigurationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for S3BucketReplicationConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_s3_bucket_replication_configuration".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildS3BucketReplicationConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub bucket: PrimField<String>,
    #[doc = ""]
    pub role: PrimField<String>,
}
impl BuildS3BucketReplicationConfiguration {
    pub fn build(self, stack: &mut Stack) -> S3BucketReplicationConfiguration {
        let out = S3BucketReplicationConfiguration(Rc::new(S3BucketReplicationConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(S3BucketReplicationConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bucket: self.bucket,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                role: self.role,
                token: core::default::Default::default(),
                rule: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct S3BucketReplicationConfigurationRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketReplicationConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl S3BucketReplicationConfigurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.token", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<S3BucketReplicationConfigurationRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationEl {
    status: PrimField<String>,
}
impl S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationEl {}
impl ToListMappable for S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationEl {
    type O = BlockAssignable<S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketReplicationConfigurationRuleElDeleteMarkerReplicationEl {
    #[doc = ""]
    pub status: PrimField<String>,
}
impl BuildS3BucketReplicationConfigurationRuleElDeleteMarkerReplicationEl {
    pub fn build(self) -> S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationEl {
        S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationEl {
            status: self.status,
        }
    }
}
pub struct S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationElRef {
        S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationEl {
    owner: PrimField<String>,
}
impl S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationEl {}
impl ToListMappable
    for S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationEl
{
    type O = BlockAssignable<
        S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationEl {
    #[doc = ""]
    pub owner: PrimField<String>,
}
impl BuildS3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationEl {
    pub fn build(
        self,
    ) -> S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationEl {
        S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationEl {
            owner: self.owner,
        }
    }
}
pub struct S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationElRef {
        S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }
}
#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationEl {
    replica_kms_key_id: PrimField<String>,
}
impl S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationEl {}
impl ToListMappable
    for S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationEl
{
    type O = BlockAssignable<
        S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationEl {
    #[doc = ""]
    pub replica_kms_key_id: PrimField<String>,
}
impl BuildS3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationEl {
    pub fn build(
        self,
    ) -> S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationEl {
        S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationEl {
            replica_kms_key_id: self.replica_kms_key_id,
        }
    }
}
pub struct S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationElRef {
        S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `replica_kms_key_id` after provisioning.\n"]
    pub fn replica_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replica_kms_key_id", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdEl {
    minutes: PrimField<f64>,
}
impl S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdEl {}
impl ToListMappable
    for S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdEl
{
    type O = BlockAssignable<
        S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdEl {
    #[doc = ""]
    pub minutes: PrimField<f64>,
}
impl BuildS3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdEl {
    pub fn build(
        self,
    ) -> S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdEl {
        S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdEl {
            minutes: self.minutes,
        }
    }
}
pub struct S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdElRef {
        S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
}
#[derive(Serialize, Default)]
struct S3BucketReplicationConfigurationRuleElDestinationElMetricsElDynamic {
    event_threshold: Option<
        DynamicBlock<S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdEl>,
    >,
}
#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElDestinationElMetricsEl {
    status: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_threshold:
        Option<Vec<S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdEl>>,
    dynamic: S3BucketReplicationConfigurationRuleElDestinationElMetricsElDynamic,
}
impl S3BucketReplicationConfigurationRuleElDestinationElMetricsEl {
    #[doc = "Set the field `event_threshold`.\n"]
    pub fn set_event_threshold(
        mut self,
        v: impl Into<
            BlockAssignable<
                S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.event_threshold = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.event_threshold = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for S3BucketReplicationConfigurationRuleElDestinationElMetricsEl {
    type O = BlockAssignable<S3BucketReplicationConfigurationRuleElDestinationElMetricsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketReplicationConfigurationRuleElDestinationElMetricsEl {
    #[doc = ""]
    pub status: PrimField<String>,
}
impl BuildS3BucketReplicationConfigurationRuleElDestinationElMetricsEl {
    pub fn build(self) -> S3BucketReplicationConfigurationRuleElDestinationElMetricsEl {
        S3BucketReplicationConfigurationRuleElDestinationElMetricsEl {
            status: self.status,
            event_threshold: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct S3BucketReplicationConfigurationRuleElDestinationElMetricsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketReplicationConfigurationRuleElDestinationElMetricsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketReplicationConfigurationRuleElDestinationElMetricsElRef {
        S3BucketReplicationConfigurationRuleElDestinationElMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketReplicationConfigurationRuleElDestinationElMetricsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
    #[doc = "Get a reference to the value of field `event_threshold` after provisioning.\n"]
    pub fn event_threshold(
        &self,
    ) -> ListRef<S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.event_threshold", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeEl {
    minutes: PrimField<f64>,
}
impl S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeEl {}
impl ToListMappable for S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeEl {
    type O =
        BlockAssignable<S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeEl {
    #[doc = ""]
    pub minutes: PrimField<f64>,
}
impl BuildS3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeEl {
    pub fn build(
        self,
    ) -> S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeEl {
        S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeEl {
            minutes: self.minutes,
        }
    }
}
pub struct S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeElRef {
        S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
}
#[derive(Serialize, Default)]
struct S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElDynamic {
    time: Option<
        DynamicBlock<S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeEl>,
    >,
}
#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeEl {
    status: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<Vec<S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeEl>>,
    dynamic: S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElDynamic,
}
impl S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeEl {
    #[doc = "Set the field `time`.\n"]
    pub fn set_time(
        mut self,
        v: impl Into<
            BlockAssignable<
                S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.time = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.time = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeEl {
    type O = BlockAssignable<S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketReplicationConfigurationRuleElDestinationElReplicationTimeEl {
    #[doc = ""]
    pub status: PrimField<String>,
}
impl BuildS3BucketReplicationConfigurationRuleElDestinationElReplicationTimeEl {
    pub fn build(self) -> S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeEl {
        S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeEl {
            status: self.status,
            time: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElRef {
        S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
    #[doc = "Get a reference to the value of field `time` after provisioning.\n"]
    pub fn time(
        &self,
    ) -> ListRef<S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.time", self.base))
    }
}
#[derive(Serialize, Default)]
struct S3BucketReplicationConfigurationRuleElDestinationElDynamic {
    access_control_translation: Option<
        DynamicBlock<S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationEl>,
    >,
    encryption_configuration: Option<
        DynamicBlock<S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationEl>,
    >,
    metrics: Option<DynamicBlock<S3BucketReplicationConfigurationRuleElDestinationElMetricsEl>>,
    replication_time:
        Option<DynamicBlock<S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeEl>>,
}
#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    account: Option<PrimField<String>>,
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_class: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_control_translation:
        Option<Vec<S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_configuration:
        Option<Vec<S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metrics: Option<Vec<S3BucketReplicationConfigurationRuleElDestinationElMetricsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replication_time:
        Option<Vec<S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeEl>>,
    dynamic: S3BucketReplicationConfigurationRuleElDestinationElDynamic,
}
impl S3BucketReplicationConfigurationRuleElDestinationEl {
    #[doc = "Set the field `account`.\n"]
    pub fn set_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.account = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_class`.\n"]
    pub fn set_storage_class(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.storage_class = Some(v.into());
        self
    }
    #[doc = "Set the field `access_control_translation`.\n"]
    pub fn set_access_control_translation(
        mut self,
        v: impl Into<
            BlockAssignable<
                S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.access_control_translation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.access_control_translation = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.encryption_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `metrics`.\n"]
    pub fn set_metrics(
        mut self,
        v: impl Into<BlockAssignable<S3BucketReplicationConfigurationRuleElDestinationElMetricsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metrics = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metrics = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `replication_time`.\n"]
    pub fn set_replication_time(
        mut self,
        v: impl Into<
            BlockAssignable<S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.replication_time = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.replication_time = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for S3BucketReplicationConfigurationRuleElDestinationEl {
    type O = BlockAssignable<S3BucketReplicationConfigurationRuleElDestinationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketReplicationConfigurationRuleElDestinationEl {
    #[doc = ""]
    pub bucket: PrimField<String>,
}
impl BuildS3BucketReplicationConfigurationRuleElDestinationEl {
    pub fn build(self) -> S3BucketReplicationConfigurationRuleElDestinationEl {
        S3BucketReplicationConfigurationRuleElDestinationEl {
            account: core::default::Default::default(),
            bucket: self.bucket,
            storage_class: core::default::Default::default(),
            access_control_translation: core::default::Default::default(),
            encryption_configuration: core::default::Default::default(),
            metrics: core::default::Default::default(),
            replication_time: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct S3BucketReplicationConfigurationRuleElDestinationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketReplicationConfigurationRuleElDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketReplicationConfigurationRuleElDestinationElRef {
        S3BucketReplicationConfigurationRuleElDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketReplicationConfigurationRuleElDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `account` after provisioning.\n"]
    pub fn account(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account", self.base))
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\n"]
    pub fn storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_class", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `access_control_translation` after provisioning.\n"]
    pub fn access_control_translation(
        &self,
    ) -> ListRef<S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_control_translation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> ListRef<S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metrics` after provisioning.\n"]
    pub fn metrics(
        &self,
    ) -> ListRef<S3BucketReplicationConfigurationRuleElDestinationElMetricsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.metrics", self.base))
    }
    #[doc = "Get a reference to the value of field `replication_time` after provisioning.\n"]
    pub fn replication_time(
        &self,
    ) -> ListRef<S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replication_time", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElExistingObjectReplicationEl {
    status: PrimField<String>,
}
impl S3BucketReplicationConfigurationRuleElExistingObjectReplicationEl {}
impl ToListMappable for S3BucketReplicationConfigurationRuleElExistingObjectReplicationEl {
    type O = BlockAssignable<S3BucketReplicationConfigurationRuleElExistingObjectReplicationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketReplicationConfigurationRuleElExistingObjectReplicationEl {
    #[doc = ""]
    pub status: PrimField<String>,
}
impl BuildS3BucketReplicationConfigurationRuleElExistingObjectReplicationEl {
    pub fn build(self) -> S3BucketReplicationConfigurationRuleElExistingObjectReplicationEl {
        S3BucketReplicationConfigurationRuleElExistingObjectReplicationEl {
            status: self.status,
        }
    }
}
pub struct S3BucketReplicationConfigurationRuleElExistingObjectReplicationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketReplicationConfigurationRuleElExistingObjectReplicationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketReplicationConfigurationRuleElExistingObjectReplicationElRef {
        S3BucketReplicationConfigurationRuleElExistingObjectReplicationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketReplicationConfigurationRuleElExistingObjectReplicationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElFilterElAndEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}
impl S3BucketReplicationConfigurationRuleElFilterElAndEl {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
}
impl ToListMappable for S3BucketReplicationConfigurationRuleElFilterElAndEl {
    type O = BlockAssignable<S3BucketReplicationConfigurationRuleElFilterElAndEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketReplicationConfigurationRuleElFilterElAndEl {}
impl BuildS3BucketReplicationConfigurationRuleElFilterElAndEl {
    pub fn build(self) -> S3BucketReplicationConfigurationRuleElFilterElAndEl {
        S3BucketReplicationConfigurationRuleElFilterElAndEl {
            prefix: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}
pub struct S3BucketReplicationConfigurationRuleElFilterElAndElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketReplicationConfigurationRuleElFilterElAndElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketReplicationConfigurationRuleElFilterElAndElRef {
        S3BucketReplicationConfigurationRuleElFilterElAndElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketReplicationConfigurationRuleElFilterElAndElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}
#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElFilterElTagEl {
    key: PrimField<String>,
    value: PrimField<String>,
}
impl S3BucketReplicationConfigurationRuleElFilterElTagEl {}
impl ToListMappable for S3BucketReplicationConfigurationRuleElFilterElTagEl {
    type O = BlockAssignable<S3BucketReplicationConfigurationRuleElFilterElTagEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketReplicationConfigurationRuleElFilterElTagEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildS3BucketReplicationConfigurationRuleElFilterElTagEl {
    pub fn build(self) -> S3BucketReplicationConfigurationRuleElFilterElTagEl {
        S3BucketReplicationConfigurationRuleElFilterElTagEl {
            key: self.key,
            value: self.value,
        }
    }
}
pub struct S3BucketReplicationConfigurationRuleElFilterElTagElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketReplicationConfigurationRuleElFilterElTagElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketReplicationConfigurationRuleElFilterElTagElRef {
        S3BucketReplicationConfigurationRuleElFilterElTagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketReplicationConfigurationRuleElFilterElTagElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct S3BucketReplicationConfigurationRuleElFilterElDynamic {
    and: Option<DynamicBlock<S3BucketReplicationConfigurationRuleElFilterElAndEl>>,
    tag: Option<DynamicBlock<S3BucketReplicationConfigurationRuleElFilterElTagEl>>,
}
#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    and: Option<Vec<S3BucketReplicationConfigurationRuleElFilterElAndEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<Vec<S3BucketReplicationConfigurationRuleElFilterElTagEl>>,
    dynamic: S3BucketReplicationConfigurationRuleElFilterElDynamic,
}
impl S3BucketReplicationConfigurationRuleElFilterEl {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl Into<BlockAssignable<S3BucketReplicationConfigurationRuleElFilterElAndEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.and = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.and = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(
        mut self,
        v: impl Into<BlockAssignable<S3BucketReplicationConfigurationRuleElFilterElTagEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tag = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tag = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for S3BucketReplicationConfigurationRuleElFilterEl {
    type O = BlockAssignable<S3BucketReplicationConfigurationRuleElFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketReplicationConfigurationRuleElFilterEl {}
impl BuildS3BucketReplicationConfigurationRuleElFilterEl {
    pub fn build(self) -> S3BucketReplicationConfigurationRuleElFilterEl {
        S3BucketReplicationConfigurationRuleElFilterEl {
            prefix: core::default::Default::default(),
            and: core::default::Default::default(),
            tag: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct S3BucketReplicationConfigurationRuleElFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketReplicationConfigurationRuleElFilterElRef {
    fn new(shared: StackShared, base: String) -> S3BucketReplicationConfigurationRuleElFilterElRef {
        S3BucketReplicationConfigurationRuleElFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketReplicationConfigurationRuleElFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> ListRef<S3BucketReplicationConfigurationRuleElFilterElAndElRef> {
        ListRef::new(self.shared().clone(), format!("{}.and", self.base))
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> ListRef<S3BucketReplicationConfigurationRuleElFilterElTagElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}
#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsEl {
    status: PrimField<String>,
}
impl S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsEl {}
impl ToListMappable
    for S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsEl
{
    type O = BlockAssignable<
        S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsEl
{
    #[doc = ""]
    pub status: PrimField<String>,
}
impl BuildS3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsEl {
    pub fn build(
        self,
    ) -> S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsEl {
        S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsEl {
            status: self.status,
        }
    }
}
pub struct S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsElRef
    {
        S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl {
    status: PrimField<String>,
}
impl S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl {}
impl ToListMappable
    for S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl
{
    type O = BlockAssignable<
        S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl
{
    #[doc = ""]
    pub status: PrimField<String>,
}
impl BuildS3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl {
    pub fn build(
        self,
    ) -> S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl
    {
        S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl {
            status: self.status,
        }
    }
}
pub struct S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsElRef
    {
        S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
#[derive(Serialize, Default)]
struct S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElDynamic {
    replica_modifications: Option<
        DynamicBlock<
            S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsEl,
        >,
    >,
    sse_kms_encrypted_objects: Option<
        DynamicBlock<
            S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    replica_modifications: Option<
        Vec<S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    sse_kms_encrypted_objects: Option<
        Vec<
            S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl,
        >,
    >,
    dynamic: S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElDynamic,
}
impl S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaEl {
    #[doc = "Set the field `replica_modifications`.\n"]
    pub fn set_replica_modifications(
        mut self,
        v : impl Into < BlockAssignable < S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.replica_modifications = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.replica_modifications = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sse_kms_encrypted_objects`.\n"]
    pub fn set_sse_kms_encrypted_objects(
        mut self,
        v : impl Into < BlockAssignable < S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sse_kms_encrypted_objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sse_kms_encrypted_objects = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaEl {
    type O = BlockAssignable<S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketReplicationConfigurationRuleElSourceSelectionCriteriaEl {}
impl BuildS3BucketReplicationConfigurationRuleElSourceSelectionCriteriaEl {
    pub fn build(self) -> S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaEl {
        S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaEl {
            replica_modifications: core::default::Default::default(),
            sse_kms_encrypted_objects: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElRef {
        S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `replica_modifications` after provisioning.\n"]
    pub fn replica_modifications(
        &self,
    ) -> ListRef<
        S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replica_modifications", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sse_kms_encrypted_objects` after provisioning.\n"]
    pub fn sse_kms_encrypted_objects(
        &self,
    ) -> ListRef<
        S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sse_kms_encrypted_objects", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct S3BucketReplicationConfigurationRuleElDynamic {
    delete_marker_replication:
        Option<DynamicBlock<S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationEl>>,
    destination: Option<DynamicBlock<S3BucketReplicationConfigurationRuleElDestinationEl>>,
    existing_object_replication:
        Option<DynamicBlock<S3BucketReplicationConfigurationRuleElExistingObjectReplicationEl>>,
    filter: Option<DynamicBlock<S3BucketReplicationConfigurationRuleElFilterEl>>,
    source_selection_criteria:
        Option<DynamicBlock<S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaEl>>,
}
#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    status: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_marker_replication:
        Option<Vec<S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination: Option<Vec<S3BucketReplicationConfigurationRuleElDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    existing_object_replication:
        Option<Vec<S3BucketReplicationConfigurationRuleElExistingObjectReplicationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<S3BucketReplicationConfigurationRuleElFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_selection_criteria:
        Option<Vec<S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaEl>>,
    dynamic: S3BucketReplicationConfigurationRuleElDynamic,
}
impl S3BucketReplicationConfigurationRuleEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.priority = Some(v.into());
        self
    }
    #[doc = "Set the field `delete_marker_replication`.\n"]
    pub fn set_delete_marker_replication(
        mut self,
        v: impl Into<BlockAssignable<S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.delete_marker_replication = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.delete_marker_replication = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v: impl Into<BlockAssignable<S3BucketReplicationConfigurationRuleElDestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `existing_object_replication`.\n"]
    pub fn set_existing_object_replication(
        mut self,
        v: impl Into<BlockAssignable<S3BucketReplicationConfigurationRuleElExistingObjectReplicationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.existing_object_replication = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.existing_object_replication = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl Into<BlockAssignable<S3BucketReplicationConfigurationRuleElFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source_selection_criteria`.\n"]
    pub fn set_source_selection_criteria(
        mut self,
        v: impl Into<BlockAssignable<S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_selection_criteria = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_selection_criteria = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for S3BucketReplicationConfigurationRuleEl {
    type O = BlockAssignable<S3BucketReplicationConfigurationRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketReplicationConfigurationRuleEl {
    #[doc = ""]
    pub status: PrimField<String>,
}
impl BuildS3BucketReplicationConfigurationRuleEl {
    pub fn build(self) -> S3BucketReplicationConfigurationRuleEl {
        S3BucketReplicationConfigurationRuleEl {
            id: core::default::Default::default(),
            prefix: core::default::Default::default(),
            priority: core::default::Default::default(),
            status: self.status,
            delete_marker_replication: core::default::Default::default(),
            destination: core::default::Default::default(),
            existing_object_replication: core::default::Default::default(),
            filter: core::default::Default::default(),
            source_selection_criteria: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct S3BucketReplicationConfigurationRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketReplicationConfigurationRuleElRef {
    fn new(shared: StackShared, base: String) -> S3BucketReplicationConfigurationRuleElRef {
        S3BucketReplicationConfigurationRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketReplicationConfigurationRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
    #[doc = "Get a reference to the value of field `delete_marker_replication` after provisioning.\n"]
    pub fn delete_marker_replication(
        &self,
    ) -> ListRef<S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.delete_marker_replication", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> ListRef<S3BucketReplicationConfigurationRuleElDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
    #[doc = "Get a reference to the value of field `existing_object_replication` after provisioning.\n"]
    pub fn existing_object_replication(
        &self,
    ) -> ListRef<S3BucketReplicationConfigurationRuleElExistingObjectReplicationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.existing_object_replication", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<S3BucketReplicationConfigurationRuleElFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }
    #[doc = "Get a reference to the value of field `source_selection_criteria` after provisioning.\n"]
    pub fn source_selection_criteria(
        &self,
    ) -> ListRef<S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_selection_criteria", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct S3BucketReplicationConfigurationDynamic {
    rule: Option<DynamicBlock<S3BucketReplicationConfigurationRuleEl>>,
}
