use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct S3tablesTableBucketData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_configuration: Option<S3tablesTableBucketEncryptionConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_configuration: Option<S3tablesTableBucketMaintenanceConfiguration>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}
struct S3tablesTableBucket_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<S3tablesTableBucketData>,
}
#[derive(Clone)]
pub struct S3tablesTableBucket(Rc<S3tablesTableBucket_>);
impl S3tablesTableBucket {
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
    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        self,
        v: impl Into<S3tablesTableBucketEncryptionConfiguration>,
    ) -> Self {
        self.0.data.borrow_mut().encryption_configuration = Some(v.into());
        self
    }
    #[doc = "Set the field `force_destroy`.\n"]
    pub fn set_force_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_configuration`.\n"]
    pub fn set_maintenance_configuration(
        self,
        v: impl Into<S3tablesTableBucketMaintenanceConfiguration>,
    ) -> Self {
        self.0.data.borrow_mut().maintenance_configuration = Some(v.into());
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
    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> S3tablesTableBucketEncryptionConfigurationRef {
        S3tablesTableBucketEncryptionConfigurationRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_destroy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_configuration` after provisioning.\n"]
    pub fn maintenance_configuration(&self) -> S3tablesTableBucketMaintenanceConfigurationRef {
        S3tablesTableBucketMaintenanceConfigurationRef::new(
            self.shared().clone(),
            format!("{}.maintenance_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `owner_account_id` after provisioning.\n"]
    pub fn owner_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
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
}
impl Referable for S3tablesTableBucket {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for S3tablesTableBucket {}
impl ToListMappable for S3tablesTableBucket {
    type O = ListRef<S3tablesTableBucketRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for S3tablesTableBucket_ {
    fn extract_resource_type(&self) -> String {
        "aws_s3tables_table_bucket".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildS3tablesTableBucket {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildS3tablesTableBucket {
    pub fn build(self, stack: &mut Stack) -> S3tablesTableBucket {
        let out = S3tablesTableBucket(Rc::new(S3tablesTableBucket_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(S3tablesTableBucketData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                encryption_configuration: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                maintenance_configuration: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct S3tablesTableBucketRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3tablesTableBucketRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl S3tablesTableBucketRef {
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
    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> S3tablesTableBucketEncryptionConfigurationRef {
        S3tablesTableBucketEncryptionConfigurationRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_destroy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_configuration` after provisioning.\n"]
    pub fn maintenance_configuration(&self) -> S3tablesTableBucketMaintenanceConfigurationRef {
        S3tablesTableBucketMaintenanceConfigurationRef::new(
            self.shared().clone(),
            format!("{}.maintenance_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `owner_account_id` after provisioning.\n"]
    pub fn owner_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
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
}
#[derive(Serialize)]
pub struct S3tablesTableBucketEncryptionConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sse_algorithm: Option<PrimField<String>>,
}
impl S3tablesTableBucketEncryptionConfiguration {
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `sse_algorithm`.\n"]
    pub fn set_sse_algorithm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sse_algorithm = Some(v.into());
        self
    }
}
impl ToListMappable for S3tablesTableBucketEncryptionConfiguration {
    type O = BlockAssignable<S3tablesTableBucketEncryptionConfiguration>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3tablesTableBucketEncryptionConfiguration {}
impl BuildS3tablesTableBucketEncryptionConfiguration {
    pub fn build(self) -> S3tablesTableBucketEncryptionConfiguration {
        S3tablesTableBucketEncryptionConfiguration {
            kms_key_arn: core::default::Default::default(),
            sse_algorithm: core::default::Default::default(),
        }
    }
}
pub struct S3tablesTableBucketEncryptionConfigurationRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3tablesTableBucketEncryptionConfigurationRef {
    fn new(shared: StackShared, base: String) -> S3tablesTableBucketEncryptionConfigurationRef {
        S3tablesTableBucketEncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3tablesTableBucketEncryptionConfigurationRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `sse_algorithm` after provisioning.\n"]
    pub fn sse_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sse_algorithm", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    non_current_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unreferenced_days: Option<PrimField<f64>>,
}
impl S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalSettings {
    #[doc = "Set the field `non_current_days`.\n"]
    pub fn set_non_current_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.non_current_days = Some(v.into());
        self
    }
    #[doc = "Set the field `unreferenced_days`.\n"]
    pub fn set_unreferenced_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.unreferenced_days = Some(v.into());
        self
    }
}
impl ToListMappable
    for S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalSettings
{
    type O = BlockAssignable<
        S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalSettings,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalSettings {
}
impl BuildS3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalSettings {
    pub fn build(
        self,
    ) -> S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalSettings {
        S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalSettings {
            non_current_days: core::default::Default::default(),
            unreferenced_days: core::default::Default::default(),
        }
    }
}
pub struct S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalSettingsRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalSettingsRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalSettingsRef {
        S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalSettingsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `non_current_days` after provisioning.\n"]
    pub fn non_current_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.non_current_days", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `unreferenced_days` after provisioning.\n"]
    pub fn unreferenced_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unreferenced_days", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemoval {
    #[serde(skip_serializing_if = "Option::is_none")]
    settings:
        Option<S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}
impl S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemoval {
    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(
        mut self,
        v: impl Into<S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalSettings>,
    ) -> Self {
        self.settings = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}
impl ToListMappable for S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemoval {
    type O =
        BlockAssignable<S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemoval>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemoval {}
impl BuildS3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemoval {
    pub fn build(
        self,
    ) -> S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemoval {
        S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemoval {
            settings: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}
pub struct S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalRef {
        S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(
        &self,
    ) -> S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalSettingsRef {
        S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalSettingsRef::new(
            self.shared().clone(),
            format!("{}.settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
#[derive(Serialize)]
pub struct S3tablesTableBucketMaintenanceConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    iceberg_unreferenced_file_removal:
        Option<S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemoval>,
}
impl S3tablesTableBucketMaintenanceConfiguration {
    #[doc = "Set the field `iceberg_unreferenced_file_removal`.\n"]
    pub fn set_iceberg_unreferenced_file_removal(
        mut self,
        v: impl Into<S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemoval>,
    ) -> Self {
        self.iceberg_unreferenced_file_removal = Some(v.into());
        self
    }
}
impl ToListMappable for S3tablesTableBucketMaintenanceConfiguration {
    type O = BlockAssignable<S3tablesTableBucketMaintenanceConfiguration>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3tablesTableBucketMaintenanceConfiguration {}
impl BuildS3tablesTableBucketMaintenanceConfiguration {
    pub fn build(self) -> S3tablesTableBucketMaintenanceConfiguration {
        S3tablesTableBucketMaintenanceConfiguration {
            iceberg_unreferenced_file_removal: core::default::Default::default(),
        }
    }
}
pub struct S3tablesTableBucketMaintenanceConfigurationRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3tablesTableBucketMaintenanceConfigurationRef {
    fn new(shared: StackShared, base: String) -> S3tablesTableBucketMaintenanceConfigurationRef {
        S3tablesTableBucketMaintenanceConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3tablesTableBucketMaintenanceConfigurationRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `iceberg_unreferenced_file_removal` after provisioning.\n"]
    pub fn iceberg_unreferenced_file_removal(
        &self,
    ) -> S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalRef {
        S3tablesTableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalRef::new(
            self.shared().clone(),
            format!("{}.iceberg_unreferenced_file_removal", self.base),
        )
    }
}
