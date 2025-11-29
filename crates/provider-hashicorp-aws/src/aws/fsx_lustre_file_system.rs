use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct FsxLustreFileSystemData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_import_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_backup_retention_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_tags_to_backups: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    daily_automatic_backup_start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_compression_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drive_cache_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    efa_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    export_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_system_type_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    final_backup_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    import_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    imported_file_chunk_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_unit_storage_throughput: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_final_backup: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_type: Option<PrimField<String>>,
    subnet_ids: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throughput_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_maintenance_start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_read_cache_configuration: Option<Vec<FsxLustreFileSystemDataReadCacheConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_configuration: Option<Vec<FsxLustreFileSystemLogConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_configuration: Option<Vec<FsxLustreFileSystemMetadataConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_squash_configuration: Option<Vec<FsxLustreFileSystemRootSquashConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FsxLustreFileSystemTimeoutsEl>,
    dynamic: FsxLustreFileSystemDynamic,
}

struct FsxLustreFileSystem_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FsxLustreFileSystemData>,
}

#[derive(Clone)]
pub struct FsxLustreFileSystem(Rc<FsxLustreFileSystem_>);

impl FsxLustreFileSystem {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `auto_import_policy`.\n"]
    pub fn set_auto_import_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().auto_import_policy = Some(v.into());
        self
    }

    #[doc = "Set the field `automatic_backup_retention_days`.\n"]
    pub fn set_automatic_backup_retention_days(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().automatic_backup_retention_days = Some(v.into());
        self
    }

    #[doc = "Set the field `backup_id`.\n"]
    pub fn set_backup_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().backup_id = Some(v.into());
        self
    }

    #[doc = "Set the field `copy_tags_to_backups`.\n"]
    pub fn set_copy_tags_to_backups(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().copy_tags_to_backups = Some(v.into());
        self
    }

    #[doc = "Set the field `daily_automatic_backup_start_time`.\n"]
    pub fn set_daily_automatic_backup_start_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().daily_automatic_backup_start_time = Some(v.into());
        self
    }

    #[doc = "Set the field `data_compression_type`.\n"]
    pub fn set_data_compression_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().data_compression_type = Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_type`.\n"]
    pub fn set_deployment_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deployment_type = Some(v.into());
        self
    }

    #[doc = "Set the field `drive_cache_type`.\n"]
    pub fn set_drive_cache_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().drive_cache_type = Some(v.into());
        self
    }

    #[doc = "Set the field `efa_enabled`.\n"]
    pub fn set_efa_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().efa_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `export_path`.\n"]
    pub fn set_export_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().export_path = Some(v.into());
        self
    }

    #[doc = "Set the field `file_system_type_version`.\n"]
    pub fn set_file_system_type_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().file_system_type_version = Some(v.into());
        self
    }

    #[doc = "Set the field `final_backup_tags`.\n"]
    pub fn set_final_backup_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().final_backup_tags = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `import_path`.\n"]
    pub fn set_import_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().import_path = Some(v.into());
        self
    }

    #[doc = "Set the field `imported_file_chunk_size`.\n"]
    pub fn set_imported_file_chunk_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().imported_file_chunk_size = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `per_unit_storage_throughput`.\n"]
    pub fn set_per_unit_storage_throughput(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().per_unit_storage_throughput = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().security_group_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `skip_final_backup`.\n"]
    pub fn set_skip_final_backup(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_final_backup = Some(v.into());
        self
    }

    #[doc = "Set the field `storage_capacity`.\n"]
    pub fn set_storage_capacity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().storage_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `storage_type`.\n"]
    pub fn set_storage_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().storage_type = Some(v.into());
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

    #[doc = "Set the field `throughput_capacity`.\n"]
    pub fn set_throughput_capacity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().throughput_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `weekly_maintenance_start_time`.\n"]
    pub fn set_weekly_maintenance_start_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().weekly_maintenance_start_time = Some(v.into());
        self
    }

    #[doc = "Set the field `data_read_cache_configuration`.\n"]
    pub fn set_data_read_cache_configuration(
        self,
        v: impl Into<BlockAssignable<FsxLustreFileSystemDataReadCacheConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_read_cache_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_read_cache_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `log_configuration`.\n"]
    pub fn set_log_configuration(self, v: impl Into<BlockAssignable<FsxLustreFileSystemLogConfigurationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metadata_configuration`.\n"]
    pub fn set_metadata_configuration(
        self,
        v: impl Into<BlockAssignable<FsxLustreFileSystemMetadataConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metadata_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metadata_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `root_squash_configuration`.\n"]
    pub fn set_root_squash_configuration(
        self,
        v: impl Into<BlockAssignable<FsxLustreFileSystemRootSquashConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().root_squash_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.root_squash_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FsxLustreFileSystemTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_import_policy` after provisioning.\n"]
    pub fn auto_import_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_import_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automatic_backup_retention_days` after provisioning.\n"]
    pub fn automatic_backup_retention_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.automatic_backup_retention_days", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_id` after provisioning.\n"]
    pub fn backup_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `copy_tags_to_backups` after provisioning.\n"]
    pub fn copy_tags_to_backups(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.copy_tags_to_backups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `daily_automatic_backup_start_time` after provisioning.\n"]
    pub fn daily_automatic_backup_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.daily_automatic_backup_start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_compression_type` after provisioning.\n"]
    pub fn data_compression_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_compression_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_type` after provisioning.\n"]
    pub fn deployment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployment_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `drive_cache_type` after provisioning.\n"]
    pub fn drive_cache_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.drive_cache_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `efa_enabled` after provisioning.\n"]
    pub fn efa_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.efa_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `export_path` after provisioning.\n"]
    pub fn export_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.export_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_type_version` after provisioning.\n"]
    pub fn file_system_type_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_system_type_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `final_backup_tags` after provisioning.\n"]
    pub fn final_backup_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.final_backup_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `import_path` after provisioning.\n"]
    pub fn import_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.import_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `imported_file_chunk_size` after provisioning.\n"]
    pub fn imported_file_chunk_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.imported_file_chunk_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mount_name` after provisioning.\n"]
    pub fn mount_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mount_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interface_ids` after provisioning.\n"]
    pub fn network_interface_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.network_interface_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `per_unit_storage_throughput` after provisioning.\n"]
    pub fn per_unit_storage_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.per_unit_storage_throughput", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skip_final_backup` after provisioning.\n"]
    pub fn skip_final_backup(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.skip_final_backup", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_capacity` after provisioning.\n"]
    pub fn storage_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `throughput_capacity` after provisioning.\n"]
    pub fn throughput_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.throughput_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `weekly_maintenance_start_time` after provisioning.\n"]
    pub fn weekly_maintenance_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.weekly_maintenance_start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_read_cache_configuration` after provisioning.\n"]
    pub fn data_read_cache_configuration(&self) -> ListRef<FsxLustreFileSystemDataReadCacheConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_read_cache_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_configuration` after provisioning.\n"]
    pub fn log_configuration(&self) -> ListRef<FsxLustreFileSystemLogConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.log_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata_configuration` after provisioning.\n"]
    pub fn metadata_configuration(&self) -> ListRef<FsxLustreFileSystemMetadataConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.metadata_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_squash_configuration` after provisioning.\n"]
    pub fn root_squash_configuration(&self) -> ListRef<FsxLustreFileSystemRootSquashConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.root_squash_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxLustreFileSystemTimeoutsElRef {
        FsxLustreFileSystemTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for FsxLustreFileSystem {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for FsxLustreFileSystem { }

impl ToListMappable for FsxLustreFileSystem {
    type O = ListRef<FsxLustreFileSystemRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for FsxLustreFileSystem_ {
    fn extract_resource_type(&self) -> String {
        "aws_fsx_lustre_file_system".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFsxLustreFileSystem {
    pub tf_id: String,
    #[doc = ""]
    pub subnet_ids: ListField<PrimField<String>>,
}

impl BuildFsxLustreFileSystem {
    pub fn build(self, stack: &mut Stack) -> FsxLustreFileSystem {
        let out = FsxLustreFileSystem(Rc::new(FsxLustreFileSystem_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FsxLustreFileSystemData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                auto_import_policy: core::default::Default::default(),
                automatic_backup_retention_days: core::default::Default::default(),
                backup_id: core::default::Default::default(),
                copy_tags_to_backups: core::default::Default::default(),
                daily_automatic_backup_start_time: core::default::Default::default(),
                data_compression_type: core::default::Default::default(),
                deployment_type: core::default::Default::default(),
                drive_cache_type: core::default::Default::default(),
                efa_enabled: core::default::Default::default(),
                export_path: core::default::Default::default(),
                file_system_type_version: core::default::Default::default(),
                final_backup_tags: core::default::Default::default(),
                id: core::default::Default::default(),
                import_path: core::default::Default::default(),
                imported_file_chunk_size: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                per_unit_storage_throughput: core::default::Default::default(),
                region: core::default::Default::default(),
                security_group_ids: core::default::Default::default(),
                skip_final_backup: core::default::Default::default(),
                storage_capacity: core::default::Default::default(),
                storage_type: core::default::Default::default(),
                subnet_ids: self.subnet_ids,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                throughput_capacity: core::default::Default::default(),
                weekly_maintenance_start_time: core::default::Default::default(),
                data_read_cache_configuration: core::default::Default::default(),
                log_configuration: core::default::Default::default(),
                metadata_configuration: core::default::Default::default(),
                root_squash_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FsxLustreFileSystemRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxLustreFileSystemRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl FsxLustreFileSystemRef {
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

    #[doc = "Get a reference to the value of field `auto_import_policy` after provisioning.\n"]
    pub fn auto_import_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_import_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automatic_backup_retention_days` after provisioning.\n"]
    pub fn automatic_backup_retention_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.automatic_backup_retention_days", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_id` after provisioning.\n"]
    pub fn backup_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `copy_tags_to_backups` after provisioning.\n"]
    pub fn copy_tags_to_backups(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.copy_tags_to_backups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `daily_automatic_backup_start_time` after provisioning.\n"]
    pub fn daily_automatic_backup_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.daily_automatic_backup_start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_compression_type` after provisioning.\n"]
    pub fn data_compression_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_compression_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_type` after provisioning.\n"]
    pub fn deployment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployment_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `drive_cache_type` after provisioning.\n"]
    pub fn drive_cache_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.drive_cache_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `efa_enabled` after provisioning.\n"]
    pub fn efa_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.efa_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `export_path` after provisioning.\n"]
    pub fn export_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.export_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_type_version` after provisioning.\n"]
    pub fn file_system_type_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_system_type_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `final_backup_tags` after provisioning.\n"]
    pub fn final_backup_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.final_backup_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `import_path` after provisioning.\n"]
    pub fn import_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.import_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `imported_file_chunk_size` after provisioning.\n"]
    pub fn imported_file_chunk_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.imported_file_chunk_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mount_name` after provisioning.\n"]
    pub fn mount_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mount_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interface_ids` after provisioning.\n"]
    pub fn network_interface_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.network_interface_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `per_unit_storage_throughput` after provisioning.\n"]
    pub fn per_unit_storage_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.per_unit_storage_throughput", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skip_final_backup` after provisioning.\n"]
    pub fn skip_final_backup(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.skip_final_backup", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_capacity` after provisioning.\n"]
    pub fn storage_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `throughput_capacity` after provisioning.\n"]
    pub fn throughput_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.throughput_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `weekly_maintenance_start_time` after provisioning.\n"]
    pub fn weekly_maintenance_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.weekly_maintenance_start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_read_cache_configuration` after provisioning.\n"]
    pub fn data_read_cache_configuration(&self) -> ListRef<FsxLustreFileSystemDataReadCacheConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_read_cache_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_configuration` after provisioning.\n"]
    pub fn log_configuration(&self) -> ListRef<FsxLustreFileSystemLogConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.log_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata_configuration` after provisioning.\n"]
    pub fn metadata_configuration(&self) -> ListRef<FsxLustreFileSystemMetadataConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.metadata_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_squash_configuration` after provisioning.\n"]
    pub fn root_squash_configuration(&self) -> ListRef<FsxLustreFileSystemRootSquashConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.root_squash_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxLustreFileSystemTimeoutsElRef {
        FsxLustreFileSystemTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct FsxLustreFileSystemDataReadCacheConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<PrimField<f64>>,
    sizing_mode: PrimField<String>,
}

impl FsxLustreFileSystemDataReadCacheConfigurationEl {
    #[doc = "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size = Some(v.into());
        self
    }
}

impl ToListMappable for FsxLustreFileSystemDataReadCacheConfigurationEl {
    type O = BlockAssignable<FsxLustreFileSystemDataReadCacheConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxLustreFileSystemDataReadCacheConfigurationEl {
    #[doc = ""]
    pub sizing_mode: PrimField<String>,
}

impl BuildFsxLustreFileSystemDataReadCacheConfigurationEl {
    pub fn build(self) -> FsxLustreFileSystemDataReadCacheConfigurationEl {
        FsxLustreFileSystemDataReadCacheConfigurationEl {
            size: core::default::Default::default(),
            sizing_mode: self.sizing_mode,
        }
    }
}

pub struct FsxLustreFileSystemDataReadCacheConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxLustreFileSystemDataReadCacheConfigurationElRef {
    fn new(shared: StackShared, base: String) -> FsxLustreFileSystemDataReadCacheConfigurationElRef {
        FsxLustreFileSystemDataReadCacheConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxLustreFileSystemDataReadCacheConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc = "Get a reference to the value of field `sizing_mode` after provisioning.\n"]
    pub fn sizing_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sizing_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxLustreFileSystemLogConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level: Option<PrimField<String>>,
}

impl FsxLustreFileSystemLogConfigurationEl {
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination = Some(v.into());
        self
    }

    #[doc = "Set the field `level`.\n"]
    pub fn set_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.level = Some(v.into());
        self
    }
}

impl ToListMappable for FsxLustreFileSystemLogConfigurationEl {
    type O = BlockAssignable<FsxLustreFileSystemLogConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxLustreFileSystemLogConfigurationEl {}

impl BuildFsxLustreFileSystemLogConfigurationEl {
    pub fn build(self) -> FsxLustreFileSystemLogConfigurationEl {
        FsxLustreFileSystemLogConfigurationEl {
            destination: core::default::Default::default(),
            level: core::default::Default::default(),
        }
    }
}

pub struct FsxLustreFileSystemLogConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxLustreFileSystemLogConfigurationElRef {
    fn new(shared: StackShared, base: String) -> FsxLustreFileSystemLogConfigurationElRef {
        FsxLustreFileSystemLogConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxLustreFileSystemLogConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `level` after provisioning.\n"]
    pub fn level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.level", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxLustreFileSystemMetadataConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}

impl FsxLustreFileSystemMetadataConfigurationEl {
    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.iops = Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}

impl ToListMappable for FsxLustreFileSystemMetadataConfigurationEl {
    type O = BlockAssignable<FsxLustreFileSystemMetadataConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxLustreFileSystemMetadataConfigurationEl {}

impl BuildFsxLustreFileSystemMetadataConfigurationEl {
    pub fn build(self) -> FsxLustreFileSystemMetadataConfigurationEl {
        FsxLustreFileSystemMetadataConfigurationEl {
            iops: core::default::Default::default(),
            mode: core::default::Default::default(),
        }
    }
}

pub struct FsxLustreFileSystemMetadataConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxLustreFileSystemMetadataConfigurationElRef {
    fn new(shared: StackShared, base: String) -> FsxLustreFileSystemMetadataConfigurationElRef {
        FsxLustreFileSystemMetadataConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxLustreFileSystemMetadataConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxLustreFileSystemRootSquashConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    no_squash_nids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_squash: Option<PrimField<String>>,
}

impl FsxLustreFileSystemRootSquashConfigurationEl {
    #[doc = "Set the field `no_squash_nids`.\n"]
    pub fn set_no_squash_nids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.no_squash_nids = Some(v.into());
        self
    }

    #[doc = "Set the field `root_squash`.\n"]
    pub fn set_root_squash(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.root_squash = Some(v.into());
        self
    }
}

impl ToListMappable for FsxLustreFileSystemRootSquashConfigurationEl {
    type O = BlockAssignable<FsxLustreFileSystemRootSquashConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxLustreFileSystemRootSquashConfigurationEl {}

impl BuildFsxLustreFileSystemRootSquashConfigurationEl {
    pub fn build(self) -> FsxLustreFileSystemRootSquashConfigurationEl {
        FsxLustreFileSystemRootSquashConfigurationEl {
            no_squash_nids: core::default::Default::default(),
            root_squash: core::default::Default::default(),
        }
    }
}

pub struct FsxLustreFileSystemRootSquashConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxLustreFileSystemRootSquashConfigurationElRef {
    fn new(shared: StackShared, base: String) -> FsxLustreFileSystemRootSquashConfigurationElRef {
        FsxLustreFileSystemRootSquashConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxLustreFileSystemRootSquashConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `no_squash_nids` after provisioning.\n"]
    pub fn no_squash_nids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.no_squash_nids", self.base))
    }

    #[doc = "Get a reference to the value of field `root_squash` after provisioning.\n"]
    pub fn root_squash(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_squash", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxLustreFileSystemTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl FsxLustreFileSystemTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for FsxLustreFileSystemTimeoutsEl {
    type O = BlockAssignable<FsxLustreFileSystemTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxLustreFileSystemTimeoutsEl {}

impl BuildFsxLustreFileSystemTimeoutsEl {
    pub fn build(self) -> FsxLustreFileSystemTimeoutsEl {
        FsxLustreFileSystemTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct FsxLustreFileSystemTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxLustreFileSystemTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FsxLustreFileSystemTimeoutsElRef {
        FsxLustreFileSystemTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxLustreFileSystemTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct FsxLustreFileSystemDynamic {
    data_read_cache_configuration: Option<DynamicBlock<FsxLustreFileSystemDataReadCacheConfigurationEl>>,
    log_configuration: Option<DynamicBlock<FsxLustreFileSystemLogConfigurationEl>>,
    metadata_configuration: Option<DynamicBlock<FsxLustreFileSystemMetadataConfigurationEl>>,
    root_squash_configuration: Option<DynamicBlock<FsxLustreFileSystemRootSquashConfigurationEl>>,
}
