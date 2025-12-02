use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct S3tablesTableData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_configuration: Option<S3tablesTableEncryptionConfiguration>,
    format: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_configuration: Option<S3tablesTableMaintenanceConfiguration>,
    name: PrimField<String>,
    namespace: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    table_bucket_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Vec<S3tablesTableMetadataEl>>,
    dynamic: S3tablesTableDynamic,
}
struct S3tablesTable_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<S3tablesTableData>,
}
#[derive(Clone)]
pub struct S3tablesTable(Rc<S3tablesTable_>);
impl S3tablesTable {
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
        v: impl Into<S3tablesTableEncryptionConfiguration>,
    ) -> Self {
        self.0.data.borrow_mut().encryption_configuration = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_configuration`.\n"]
    pub fn set_maintenance_configuration(
        self,
        v: impl Into<S3tablesTableMaintenanceConfiguration>,
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
    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(self, v: impl Into<BlockAssignable<S3tablesTableMetadataEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metadata = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metadata = Some(d);
            }
        }
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
    #[doc = "Get a reference to the value of field `created_by` after provisioning.\n"]
    pub fn created_by(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_by", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> S3tablesTableEncryptionConfigurationRef {
        S3tablesTableEncryptionConfigurationRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.format", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_configuration` after provisioning.\n"]
    pub fn maintenance_configuration(&self) -> S3tablesTableMaintenanceConfigurationRef {
        S3tablesTableMaintenanceConfigurationRef::new(
            self.shared().clone(),
            format!("{}.maintenance_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata_location` after provisioning.\n"]
    pub fn metadata_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metadata_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.modified_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `modified_by` after provisioning.\n"]
    pub fn modified_by(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.modified_by", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.namespace", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `table_bucket_arn` after provisioning.\n"]
    pub fn table_bucket_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_bucket_arn", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version_token` after provisioning.\n"]
    pub fn version_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_token", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `warehouse_location` after provisioning.\n"]
    pub fn warehouse_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.warehouse_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> ListRef<S3tablesTableMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }
}
impl Referable for S3tablesTable {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for S3tablesTable {}
impl ToListMappable for S3tablesTable {
    type O = ListRef<S3tablesTableRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for S3tablesTable_ {
    fn extract_resource_type(&self) -> String {
        "aws_s3tables_table".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildS3tablesTable {
    pub tf_id: String,
    #[doc = ""]
    pub format: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub namespace: PrimField<String>,
    #[doc = ""]
    pub table_bucket_arn: PrimField<String>,
}
impl BuildS3tablesTable {
    pub fn build(self, stack: &mut Stack) -> S3tablesTable {
        let out = S3tablesTable(Rc::new(S3tablesTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(S3tablesTableData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                encryption_configuration: core::default::Default::default(),
                format: self.format,
                maintenance_configuration: core::default::Default::default(),
                name: self.name,
                namespace: self.namespace,
                region: core::default::Default::default(),
                table_bucket_arn: self.table_bucket_arn,
                tags: core::default::Default::default(),
                metadata: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct S3tablesTableRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3tablesTableRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl S3tablesTableRef {
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
    #[doc = "Get a reference to the value of field `created_by` after provisioning.\n"]
    pub fn created_by(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_by", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> S3tablesTableEncryptionConfigurationRef {
        S3tablesTableEncryptionConfigurationRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.format", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_configuration` after provisioning.\n"]
    pub fn maintenance_configuration(&self) -> S3tablesTableMaintenanceConfigurationRef {
        S3tablesTableMaintenanceConfigurationRef::new(
            self.shared().clone(),
            format!("{}.maintenance_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata_location` after provisioning.\n"]
    pub fn metadata_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metadata_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.modified_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `modified_by` after provisioning.\n"]
    pub fn modified_by(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.modified_by", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.namespace", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `table_bucket_arn` after provisioning.\n"]
    pub fn table_bucket_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_bucket_arn", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version_token` after provisioning.\n"]
    pub fn version_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_token", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `warehouse_location` after provisioning.\n"]
    pub fn warehouse_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.warehouse_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> ListRef<S3tablesTableMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct S3tablesTableEncryptionConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sse_algorithm: Option<PrimField<String>>,
}
impl S3tablesTableEncryptionConfiguration {
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
impl ToListMappable for S3tablesTableEncryptionConfiguration {
    type O = BlockAssignable<S3tablesTableEncryptionConfiguration>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3tablesTableEncryptionConfiguration {}
impl BuildS3tablesTableEncryptionConfiguration {
    pub fn build(self) -> S3tablesTableEncryptionConfiguration {
        S3tablesTableEncryptionConfiguration {
            kms_key_arn: core::default::Default::default(),
            sse_algorithm: core::default::Default::default(),
        }
    }
}
pub struct S3tablesTableEncryptionConfigurationRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3tablesTableEncryptionConfigurationRef {
    fn new(shared: StackShared, base: String) -> S3tablesTableEncryptionConfigurationRef {
        S3tablesTableEncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3tablesTableEncryptionConfigurationRef {
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
pub struct S3tablesTableMaintenanceConfigurationIcebergCompactionSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    target_file_size_mb: Option<PrimField<f64>>,
}
impl S3tablesTableMaintenanceConfigurationIcebergCompactionSettings {
    #[doc = "Set the field `target_file_size_mb`.\n"]
    pub fn set_target_file_size_mb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.target_file_size_mb = Some(v.into());
        self
    }
}
impl ToListMappable for S3tablesTableMaintenanceConfigurationIcebergCompactionSettings {
    type O = BlockAssignable<S3tablesTableMaintenanceConfigurationIcebergCompactionSettings>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3tablesTableMaintenanceConfigurationIcebergCompactionSettings {}
impl BuildS3tablesTableMaintenanceConfigurationIcebergCompactionSettings {
    pub fn build(self) -> S3tablesTableMaintenanceConfigurationIcebergCompactionSettings {
        S3tablesTableMaintenanceConfigurationIcebergCompactionSettings {
            target_file_size_mb: core::default::Default::default(),
        }
    }
}
pub struct S3tablesTableMaintenanceConfigurationIcebergCompactionSettingsRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3tablesTableMaintenanceConfigurationIcebergCompactionSettingsRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3tablesTableMaintenanceConfigurationIcebergCompactionSettingsRef {
        S3tablesTableMaintenanceConfigurationIcebergCompactionSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3tablesTableMaintenanceConfigurationIcebergCompactionSettingsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target_file_size_mb` after provisioning.\n"]
    pub fn target_file_size_mb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_file_size_mb", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct S3tablesTableMaintenanceConfigurationIcebergCompaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<S3tablesTableMaintenanceConfigurationIcebergCompactionSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}
impl S3tablesTableMaintenanceConfigurationIcebergCompaction {
    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(
        mut self,
        v: impl Into<S3tablesTableMaintenanceConfigurationIcebergCompactionSettings>,
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
impl ToListMappable for S3tablesTableMaintenanceConfigurationIcebergCompaction {
    type O = BlockAssignable<S3tablesTableMaintenanceConfigurationIcebergCompaction>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3tablesTableMaintenanceConfigurationIcebergCompaction {}
impl BuildS3tablesTableMaintenanceConfigurationIcebergCompaction {
    pub fn build(self) -> S3tablesTableMaintenanceConfigurationIcebergCompaction {
        S3tablesTableMaintenanceConfigurationIcebergCompaction {
            settings: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}
pub struct S3tablesTableMaintenanceConfigurationIcebergCompactionRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3tablesTableMaintenanceConfigurationIcebergCompactionRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3tablesTableMaintenanceConfigurationIcebergCompactionRef {
        S3tablesTableMaintenanceConfigurationIcebergCompactionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3tablesTableMaintenanceConfigurationIcebergCompactionRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> S3tablesTableMaintenanceConfigurationIcebergCompactionSettingsRef {
        S3tablesTableMaintenanceConfigurationIcebergCompactionSettingsRef::new(
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
pub struct S3tablesTableMaintenanceConfigurationIcebergSnapshotManagementSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_snapshot_age_hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_snapshots_to_keep: Option<PrimField<f64>>,
}
impl S3tablesTableMaintenanceConfigurationIcebergSnapshotManagementSettings {
    #[doc = "Set the field `max_snapshot_age_hours`.\n"]
    pub fn set_max_snapshot_age_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_snapshot_age_hours = Some(v.into());
        self
    }
    #[doc = "Set the field `min_snapshots_to_keep`.\n"]
    pub fn set_min_snapshots_to_keep(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_snapshots_to_keep = Some(v.into());
        self
    }
}
impl ToListMappable for S3tablesTableMaintenanceConfigurationIcebergSnapshotManagementSettings {
    type O =
        BlockAssignable<S3tablesTableMaintenanceConfigurationIcebergSnapshotManagementSettings>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3tablesTableMaintenanceConfigurationIcebergSnapshotManagementSettings {}
impl BuildS3tablesTableMaintenanceConfigurationIcebergSnapshotManagementSettings {
    pub fn build(self) -> S3tablesTableMaintenanceConfigurationIcebergSnapshotManagementSettings {
        S3tablesTableMaintenanceConfigurationIcebergSnapshotManagementSettings {
            max_snapshot_age_hours: core::default::Default::default(),
            min_snapshots_to_keep: core::default::Default::default(),
        }
    }
}
pub struct S3tablesTableMaintenanceConfigurationIcebergSnapshotManagementSettingsRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3tablesTableMaintenanceConfigurationIcebergSnapshotManagementSettingsRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3tablesTableMaintenanceConfigurationIcebergSnapshotManagementSettingsRef {
        S3tablesTableMaintenanceConfigurationIcebergSnapshotManagementSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3tablesTableMaintenanceConfigurationIcebergSnapshotManagementSettingsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_snapshot_age_hours` after provisioning.\n"]
    pub fn max_snapshot_age_hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_snapshot_age_hours", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_snapshots_to_keep` after provisioning.\n"]
    pub fn min_snapshots_to_keep(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_snapshots_to_keep", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct S3tablesTableMaintenanceConfigurationIcebergSnapshotManagement {
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<S3tablesTableMaintenanceConfigurationIcebergSnapshotManagementSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}
impl S3tablesTableMaintenanceConfigurationIcebergSnapshotManagement {
    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(
        mut self,
        v: impl Into<S3tablesTableMaintenanceConfigurationIcebergSnapshotManagementSettings>,
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
impl ToListMappable for S3tablesTableMaintenanceConfigurationIcebergSnapshotManagement {
    type O = BlockAssignable<S3tablesTableMaintenanceConfigurationIcebergSnapshotManagement>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3tablesTableMaintenanceConfigurationIcebergSnapshotManagement {}
impl BuildS3tablesTableMaintenanceConfigurationIcebergSnapshotManagement {
    pub fn build(self) -> S3tablesTableMaintenanceConfigurationIcebergSnapshotManagement {
        S3tablesTableMaintenanceConfigurationIcebergSnapshotManagement {
            settings: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}
pub struct S3tablesTableMaintenanceConfigurationIcebergSnapshotManagementRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3tablesTableMaintenanceConfigurationIcebergSnapshotManagementRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3tablesTableMaintenanceConfigurationIcebergSnapshotManagementRef {
        S3tablesTableMaintenanceConfigurationIcebergSnapshotManagementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3tablesTableMaintenanceConfigurationIcebergSnapshotManagementRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(
        &self,
    ) -> S3tablesTableMaintenanceConfigurationIcebergSnapshotManagementSettingsRef {
        S3tablesTableMaintenanceConfigurationIcebergSnapshotManagementSettingsRef::new(
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
pub struct S3tablesTableMaintenanceConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    iceberg_compaction: Option<S3tablesTableMaintenanceConfigurationIcebergCompaction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iceberg_snapshot_management:
        Option<S3tablesTableMaintenanceConfigurationIcebergSnapshotManagement>,
}
impl S3tablesTableMaintenanceConfiguration {
    #[doc = "Set the field `iceberg_compaction`.\n"]
    pub fn set_iceberg_compaction(
        mut self,
        v: impl Into<S3tablesTableMaintenanceConfigurationIcebergCompaction>,
    ) -> Self {
        self.iceberg_compaction = Some(v.into());
        self
    }
    #[doc = "Set the field `iceberg_snapshot_management`.\n"]
    pub fn set_iceberg_snapshot_management(
        mut self,
        v: impl Into<S3tablesTableMaintenanceConfigurationIcebergSnapshotManagement>,
    ) -> Self {
        self.iceberg_snapshot_management = Some(v.into());
        self
    }
}
impl ToListMappable for S3tablesTableMaintenanceConfiguration {
    type O = BlockAssignable<S3tablesTableMaintenanceConfiguration>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3tablesTableMaintenanceConfiguration {}
impl BuildS3tablesTableMaintenanceConfiguration {
    pub fn build(self) -> S3tablesTableMaintenanceConfiguration {
        S3tablesTableMaintenanceConfiguration {
            iceberg_compaction: core::default::Default::default(),
            iceberg_snapshot_management: core::default::Default::default(),
        }
    }
}
pub struct S3tablesTableMaintenanceConfigurationRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3tablesTableMaintenanceConfigurationRef {
    fn new(shared: StackShared, base: String) -> S3tablesTableMaintenanceConfigurationRef {
        S3tablesTableMaintenanceConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3tablesTableMaintenanceConfigurationRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `iceberg_compaction` after provisioning.\n"]
    pub fn iceberg_compaction(&self) -> S3tablesTableMaintenanceConfigurationIcebergCompactionRef {
        S3tablesTableMaintenanceConfigurationIcebergCompactionRef::new(
            self.shared().clone(),
            format!("{}.iceberg_compaction", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `iceberg_snapshot_management` after provisioning.\n"]
    pub fn iceberg_snapshot_management(
        &self,
    ) -> S3tablesTableMaintenanceConfigurationIcebergSnapshotManagementRef {
        S3tablesTableMaintenanceConfigurationIcebergSnapshotManagementRef::new(
            self.shared().clone(),
            format!("{}.iceberg_snapshot_management", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct S3tablesTableMetadataElIcebergElSchemaElFieldEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<PrimField<bool>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl S3tablesTableMetadataElIcebergElSchemaElFieldEl {
    #[doc = "Set the field `required`.\nA Boolean value that specifies whether values are required for each row in this field. Default: false."]
    pub fn set_required(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.required = Some(v.into());
        self
    }
}
impl ToListMappable for S3tablesTableMetadataElIcebergElSchemaElFieldEl {
    type O = BlockAssignable<S3tablesTableMetadataElIcebergElSchemaElFieldEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3tablesTableMetadataElIcebergElSchemaElFieldEl {
    #[doc = "The name of the field."]
    pub name: PrimField<String>,
    #[doc = "The field type. S3 Tables supports all Apache Iceberg primitive types."]
    pub type_: PrimField<String>,
}
impl BuildS3tablesTableMetadataElIcebergElSchemaElFieldEl {
    pub fn build(self) -> S3tablesTableMetadataElIcebergElSchemaElFieldEl {
        S3tablesTableMetadataElIcebergElSchemaElFieldEl {
            name: self.name,
            required: core::default::Default::default(),
            type_: self.type_,
        }
    }
}
pub struct S3tablesTableMetadataElIcebergElSchemaElFieldElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3tablesTableMetadataElIcebergElSchemaElFieldElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3tablesTableMetadataElIcebergElSchemaElFieldElRef {
        S3tablesTableMetadataElIcebergElSchemaElFieldElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3tablesTableMetadataElIcebergElSchemaElFieldElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the field."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `required` after provisioning.\nA Boolean value that specifies whether values are required for each row in this field. Default: false."]
    pub fn required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe field type. S3 Tables supports all Apache Iceberg primitive types."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize, Default)]
struct S3tablesTableMetadataElIcebergElSchemaElDynamic {
    field: Option<DynamicBlock<S3tablesTableMetadataElIcebergElSchemaElFieldEl>>,
}
#[derive(Serialize)]
pub struct S3tablesTableMetadataElIcebergElSchemaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<Vec<S3tablesTableMetadataElIcebergElSchemaElFieldEl>>,
    dynamic: S3tablesTableMetadataElIcebergElSchemaElDynamic,
}
impl S3tablesTableMetadataElIcebergElSchemaEl {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(
        mut self,
        v: impl Into<BlockAssignable<S3tablesTableMetadataElIcebergElSchemaElFieldEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.field = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.field = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for S3tablesTableMetadataElIcebergElSchemaEl {
    type O = BlockAssignable<S3tablesTableMetadataElIcebergElSchemaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3tablesTableMetadataElIcebergElSchemaEl {}
impl BuildS3tablesTableMetadataElIcebergElSchemaEl {
    pub fn build(self) -> S3tablesTableMetadataElIcebergElSchemaEl {
        S3tablesTableMetadataElIcebergElSchemaEl {
            field: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct S3tablesTableMetadataElIcebergElSchemaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3tablesTableMetadataElIcebergElSchemaElRef {
    fn new(shared: StackShared, base: String) -> S3tablesTableMetadataElIcebergElSchemaElRef {
        S3tablesTableMetadataElIcebergElSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3tablesTableMetadataElIcebergElSchemaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> ListRef<S3tablesTableMetadataElIcebergElSchemaElFieldElRef> {
        ListRef::new(self.shared().clone(), format!("{}.field", self.base))
    }
}
#[derive(Serialize, Default)]
struct S3tablesTableMetadataElIcebergElDynamic {
    schema: Option<DynamicBlock<S3tablesTableMetadataElIcebergElSchemaEl>>,
}
#[derive(Serialize)]
pub struct S3tablesTableMetadataElIcebergEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<Vec<S3tablesTableMetadataElIcebergElSchemaEl>>,
    dynamic: S3tablesTableMetadataElIcebergElDynamic,
}
impl S3tablesTableMetadataElIcebergEl {
    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(
        mut self,
        v: impl Into<BlockAssignable<S3tablesTableMetadataElIcebergElSchemaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schema = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schema = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for S3tablesTableMetadataElIcebergEl {
    type O = BlockAssignable<S3tablesTableMetadataElIcebergEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3tablesTableMetadataElIcebergEl {}
impl BuildS3tablesTableMetadataElIcebergEl {
    pub fn build(self) -> S3tablesTableMetadataElIcebergEl {
        S3tablesTableMetadataElIcebergEl {
            schema: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct S3tablesTableMetadataElIcebergElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3tablesTableMetadataElIcebergElRef {
    fn new(shared: StackShared, base: String) -> S3tablesTableMetadataElIcebergElRef {
        S3tablesTableMetadataElIcebergElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3tablesTableMetadataElIcebergElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> ListRef<S3tablesTableMetadataElIcebergElSchemaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schema", self.base))
    }
}
#[derive(Serialize, Default)]
struct S3tablesTableMetadataElDynamic {
    iceberg: Option<DynamicBlock<S3tablesTableMetadataElIcebergEl>>,
}
#[derive(Serialize)]
pub struct S3tablesTableMetadataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    iceberg: Option<Vec<S3tablesTableMetadataElIcebergEl>>,
    dynamic: S3tablesTableMetadataElDynamic,
}
impl S3tablesTableMetadataEl {
    #[doc = "Set the field `iceberg`.\n"]
    pub fn set_iceberg(
        mut self,
        v: impl Into<BlockAssignable<S3tablesTableMetadataElIcebergEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.iceberg = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.iceberg = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for S3tablesTableMetadataEl {
    type O = BlockAssignable<S3tablesTableMetadataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3tablesTableMetadataEl {}
impl BuildS3tablesTableMetadataEl {
    pub fn build(self) -> S3tablesTableMetadataEl {
        S3tablesTableMetadataEl {
            iceberg: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct S3tablesTableMetadataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3tablesTableMetadataElRef {
    fn new(shared: StackShared, base: String) -> S3tablesTableMetadataElRef {
        S3tablesTableMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3tablesTableMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `iceberg` after provisioning.\n"]
    pub fn iceberg(&self) -> ListRef<S3tablesTableMetadataElIcebergElRef> {
        ListRef::new(self.shared().clone(), format!("{}.iceberg", self.base))
    }
}
#[derive(Serialize, Default)]
struct S3tablesTableDynamic {
    metadata: Option<DynamicBlock<S3tablesTableMetadataEl>>,
}
