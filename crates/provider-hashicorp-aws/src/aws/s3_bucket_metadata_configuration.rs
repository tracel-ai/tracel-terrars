use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct S3BucketMetadataConfigurationData {
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
    expected_bucket_owner: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_configuration: Option<Vec<S3BucketMetadataConfigurationMetadataConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<S3BucketMetadataConfigurationTimeoutsEl>,
    dynamic: S3BucketMetadataConfigurationDynamic,
}

struct S3BucketMetadataConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<S3BucketMetadataConfigurationData>,
}

#[derive(Clone)]
pub struct S3BucketMetadataConfiguration(Rc<S3BucketMetadataConfiguration_>);

impl S3BucketMetadataConfiguration {
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

    #[doc = "Set the field `expected_bucket_owner`.\n"]
    pub fn set_expected_bucket_owner(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().expected_bucket_owner = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `metadata_configuration`.\n"]
    pub fn set_metadata_configuration(
        self,
        v: impl Into<BlockAssignable<S3BucketMetadataConfigurationMetadataConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metadata_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metadata_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<S3BucketMetadataConfigurationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expected_bucket_owner", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `metadata_configuration` after provisioning.\n"]
    pub fn metadata_configuration(
        &self,
    ) -> ListRef<S3BucketMetadataConfigurationMetadataConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> S3BucketMetadataConfigurationTimeoutsElRef {
        S3BucketMetadataConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for S3BucketMetadataConfiguration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for S3BucketMetadataConfiguration {}

impl ToListMappable for S3BucketMetadataConfiguration {
    type O = ListRef<S3BucketMetadataConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for S3BucketMetadataConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_s3_bucket_metadata_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3BucketMetadataConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub bucket: PrimField<String>,
}

impl BuildS3BucketMetadataConfiguration {
    pub fn build(self, stack: &mut Stack) -> S3BucketMetadataConfiguration {
        let out = S3BucketMetadataConfiguration(Rc::new(S3BucketMetadataConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(S3BucketMetadataConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bucket: self.bucket,
                expected_bucket_owner: core::default::Default::default(),
                region: core::default::Default::default(),
                metadata_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3BucketMetadataConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketMetadataConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl S3BucketMetadataConfigurationRef {
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

    #[doc = "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expected_bucket_owner", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `metadata_configuration` after provisioning.\n"]
    pub fn metadata_configuration(
        &self,
    ) -> ListRef<S3BucketMetadataConfigurationMetadataConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> S3BucketMetadataConfigurationTimeoutsElRef {
        S3BucketMetadataConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct S3BucketMetadataConfigurationMetadataConfigurationElDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    table_bucket_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_bucket_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_namespace: Option<PrimField<String>>,
}

impl S3BucketMetadataConfigurationMetadataConfigurationElDestinationEl {
    #[doc = "Set the field `table_bucket_arn`.\n"]
    pub fn set_table_bucket_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.table_bucket_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `table_bucket_type`.\n"]
    pub fn set_table_bucket_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.table_bucket_type = Some(v.into());
        self
    }

    #[doc = "Set the field `table_namespace`.\n"]
    pub fn set_table_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.table_namespace = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketMetadataConfigurationMetadataConfigurationElDestinationEl {
    type O = BlockAssignable<S3BucketMetadataConfigurationMetadataConfigurationElDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketMetadataConfigurationMetadataConfigurationElDestinationEl {}

impl BuildS3BucketMetadataConfigurationMetadataConfigurationElDestinationEl {
    pub fn build(self) -> S3BucketMetadataConfigurationMetadataConfigurationElDestinationEl {
        S3BucketMetadataConfigurationMetadataConfigurationElDestinationEl {
            table_bucket_arn: core::default::Default::default(),
            table_bucket_type: core::default::Default::default(),
            table_namespace: core::default::Default::default(),
        }
    }
}

pub struct S3BucketMetadataConfigurationMetadataConfigurationElDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketMetadataConfigurationMetadataConfigurationElDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketMetadataConfigurationMetadataConfigurationElDestinationElRef {
        S3BucketMetadataConfigurationMetadataConfigurationElDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketMetadataConfigurationMetadataConfigurationElDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table_bucket_arn` after provisioning.\n"]
    pub fn table_bucket_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_bucket_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `table_bucket_type` after provisioning.\n"]
    pub fn table_bucket_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_bucket_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `table_namespace` after provisioning.\n"]
    pub fn table_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_namespace", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationElEncryptionConfigurationEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
    sse_algorithm: PrimField<String>,
}

impl S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationElEncryptionConfigurationEl {
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_arn = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationElEncryptionConfigurationEl {
    type O =
        BlockAssignable<
            S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationElEncryptionConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationElEncryptionConfigurationEl
{
    #[doc = ""]
    pub sse_algorithm: PrimField<String>,
}

impl BuildS3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationElEncryptionConfigurationEl {
    pub fn build(
        self,
    ) -> S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationElEncryptionConfigurationEl {
        S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationElEncryptionConfigurationEl {
            kms_key_arn: core::default::Default::default(),
            sse_algorithm: self.sse_algorithm,
        }
    }
}

pub struct S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationElEncryptionConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationElEncryptionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationElEncryptionConfigurationElRef {
        S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationElEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationElEncryptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sse_algorithm` after provisioning.\n"]
    pub fn sse_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sse_algorithm", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationElDynamic {
    encryption_configuration: Option<
        DynamicBlock<
            S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationElEncryptionConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationEl {
    configuration_state: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_configuration: Option<
        Vec<
            S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationElEncryptionConfigurationEl,
        >,
    >,
    dynamic: S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationElDynamic,
}

impl S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationEl {
    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationElEncryptionConfigurationEl,
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
}

impl ToListMappable
    for S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationEl
{
    type O = BlockAssignable<
        S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationEl {
    #[doc = ""]
    pub configuration_state: PrimField<String>,
}

impl BuildS3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationEl {
    pub fn build(
        self,
    ) -> S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationEl {
        S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationEl {
            configuration_state: self.configuration_state,
            encryption_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationElRef {
        S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configuration_state` after provisioning.\n"]
    pub fn configuration_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.configuration_state", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `table_arn` after provisioning.\n"]
    pub fn table_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> ListRef<
        S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationElEncryptionConfigurationElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElEncryptionConfigurationEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
    sse_algorithm: PrimField<String>,
}

impl S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElEncryptionConfigurationEl {
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_arn = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElEncryptionConfigurationEl {
    type O =
        BlockAssignable<
            S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElEncryptionConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElEncryptionConfigurationEl
{
    #[doc = ""]
    pub sse_algorithm: PrimField<String>,
}

impl BuildS3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElEncryptionConfigurationEl {
    pub fn build(
        self,
    ) -> S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElEncryptionConfigurationEl {
        S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElEncryptionConfigurationEl {
            kms_key_arn: core::default::Default::default(),
            sse_algorithm: self.sse_algorithm,
        }
    }
}

pub struct S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElEncryptionConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElEncryptionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElEncryptionConfigurationElRef {
        S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElEncryptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sse_algorithm` after provisioning.\n"]
    pub fn sse_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sse_algorithm", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElRecordExpirationEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    days: Option<PrimField<f64>>,
    expiration: PrimField<String>,
}

impl S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElRecordExpirationEl {
    #[doc = "Set the field `days`.\n"]
    pub fn set_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.days = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElRecordExpirationEl {
    type O =
        BlockAssignable<
            S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElRecordExpirationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElRecordExpirationEl
{
    #[doc = ""]
    pub expiration: PrimField<String>,
}

impl BuildS3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElRecordExpirationEl {
    pub fn build(
        self,
    ) -> S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElRecordExpirationEl {
        S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElRecordExpirationEl {
            days: core::default::Default::default(),
            expiration: self.expiration,
        }
    }
}

pub struct S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElRecordExpirationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElRecordExpirationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElRecordExpirationElRef {
        S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElRecordExpirationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElRecordExpirationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `days` after provisioning.\n"]
    pub fn days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.days", self.base))
    }

    #[doc = "Get a reference to the value of field `expiration` after provisioning.\n"]
    pub fn expiration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expiration", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElDynamic {
    encryption_configuration: Option<
        DynamicBlock<
            S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElEncryptionConfigurationEl,
        >,
    >,
    record_expiration: Option<
        DynamicBlock<
            S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElRecordExpirationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_configuration: Option<
        Vec<S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElEncryptionConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_expiration: Option<
        Vec<S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElRecordExpirationEl>,
    >,
    dynamic: S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElDynamic,
}

impl S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationEl {
    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElEncryptionConfigurationEl,
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

    #[doc = "Set the field `record_expiration`.\n"]
    pub fn set_record_expiration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElRecordExpirationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.record_expiration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.record_expiration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationEl
{
    type O = BlockAssignable<
        S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationEl {}

impl BuildS3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationEl {
    pub fn build(
        self,
    ) -> S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationEl {
        S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationEl {
            encryption_configuration: core::default::Default::default(),
            record_expiration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElRef {
        S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table_arn` after provisioning.\n"]
    pub fn table_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> ListRef<
        S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElEncryptionConfigurationElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `record_expiration` after provisioning.\n"]
    pub fn record_expiration(
        &self,
    ) -> ListRef<
        S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElRecordExpirationElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.record_expiration", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct S3BucketMetadataConfigurationMetadataConfigurationElDynamic {
    inventory_table_configuration: Option<
        DynamicBlock<
            S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationEl,
        >,
    >,
    journal_table_configuration: Option<
        DynamicBlock<
            S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct S3BucketMetadataConfigurationMetadataConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    inventory_table_configuration: Option<
        Vec<S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    journal_table_configuration: Option<
        Vec<S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationEl>,
    >,
    dynamic: S3BucketMetadataConfigurationMetadataConfigurationElDynamic,
}

impl S3BucketMetadataConfigurationMetadataConfigurationEl {
    #[doc = "Set the field `inventory_table_configuration`.\n"]
    pub fn set_inventory_table_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.inventory_table_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.inventory_table_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `journal_table_configuration`.\n"]
    pub fn set_journal_table_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.journal_table_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.journal_table_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for S3BucketMetadataConfigurationMetadataConfigurationEl {
    type O = BlockAssignable<S3BucketMetadataConfigurationMetadataConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketMetadataConfigurationMetadataConfigurationEl {}

impl BuildS3BucketMetadataConfigurationMetadataConfigurationEl {
    pub fn build(self) -> S3BucketMetadataConfigurationMetadataConfigurationEl {
        S3BucketMetadataConfigurationMetadataConfigurationEl {
            inventory_table_configuration: core::default::Default::default(),
            journal_table_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3BucketMetadataConfigurationMetadataConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketMetadataConfigurationMetadataConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketMetadataConfigurationMetadataConfigurationElRef {
        S3BucketMetadataConfigurationMetadataConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketMetadataConfigurationMetadataConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> ListRef<S3BucketMetadataConfigurationMetadataConfigurationElDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `inventory_table_configuration` after provisioning.\n"]
    pub fn inventory_table_configuration(
        &self,
    ) -> ListRef<S3BucketMetadataConfigurationMetadataConfigurationElInventoryTableConfigurationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inventory_table_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `journal_table_configuration` after provisioning.\n"]
    pub fn journal_table_configuration(
        &self,
    ) -> ListRef<S3BucketMetadataConfigurationMetadataConfigurationElJournalTableConfigurationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.journal_table_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct S3BucketMetadataConfigurationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
}

impl S3BucketMetadataConfigurationTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketMetadataConfigurationTimeoutsEl {
    type O = BlockAssignable<S3BucketMetadataConfigurationTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketMetadataConfigurationTimeoutsEl {}

impl BuildS3BucketMetadataConfigurationTimeoutsEl {
    pub fn build(self) -> S3BucketMetadataConfigurationTimeoutsEl {
        S3BucketMetadataConfigurationTimeoutsEl {
            create: core::default::Default::default(),
        }
    }
}

pub struct S3BucketMetadataConfigurationTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketMetadataConfigurationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> S3BucketMetadataConfigurationTimeoutsElRef {
        S3BucketMetadataConfigurationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketMetadataConfigurationTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketMetadataConfigurationDynamic {
    metadata_configuration:
        Option<DynamicBlock<S3BucketMetadataConfigurationMetadataConfigurationEl>>,
}
