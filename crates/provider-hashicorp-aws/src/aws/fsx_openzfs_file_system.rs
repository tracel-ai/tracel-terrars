use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct FsxOpenzfsFileSystemData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_backup_retention_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_tags_to_backups: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_tags_to_volumes: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    daily_automatic_backup_start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_options: Option<SetField<PrimField<String>>>,
    deployment_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_ip_address_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    final_backup_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_subnet_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_table_ids: Option<SetField<PrimField<String>>>,
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
    throughput_capacity: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_maintenance_start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_iops_configuration: Option<Vec<FsxOpenzfsFileSystemDiskIopsConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_cache_configuration: Option<Vec<FsxOpenzfsFileSystemReadCacheConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_volume_configuration: Option<Vec<FsxOpenzfsFileSystemRootVolumeConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FsxOpenzfsFileSystemTimeoutsEl>,
    dynamic: FsxOpenzfsFileSystemDynamic,
}
struct FsxOpenzfsFileSystem_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FsxOpenzfsFileSystemData>,
}
#[derive(Clone)]
pub struct FsxOpenzfsFileSystem(Rc<FsxOpenzfsFileSystem_>);
impl FsxOpenzfsFileSystem {
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
    #[doc = "Set the field `copy_tags_to_volumes`.\n"]
    pub fn set_copy_tags_to_volumes(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().copy_tags_to_volumes = Some(v.into());
        self
    }
    #[doc = "Set the field `daily_automatic_backup_start_time`.\n"]
    pub fn set_daily_automatic_backup_start_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().daily_automatic_backup_start_time = Some(v.into());
        self
    }
    #[doc = "Set the field `delete_options`.\n"]
    pub fn set_delete_options(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().delete_options = Some(v.into());
        self
    }
    #[doc = "Set the field `endpoint_ip_address_range`.\n"]
    pub fn set_endpoint_ip_address_range(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().endpoint_ip_address_range = Some(v.into());
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
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = Some(v.into());
        self
    }
    #[doc = "Set the field `preferred_subnet_id`.\n"]
    pub fn set_preferred_subnet_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().preferred_subnet_id = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `route_table_ids`.\n"]
    pub fn set_route_table_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().route_table_ids = Some(v.into());
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
    #[doc = "Set the field `weekly_maintenance_start_time`.\n"]
    pub fn set_weekly_maintenance_start_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().weekly_maintenance_start_time = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_iops_configuration`.\n"]
    pub fn set_disk_iops_configuration(
        self,
        v: impl Into<BlockAssignable<FsxOpenzfsFileSystemDiskIopsConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().disk_iops_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.disk_iops_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `read_cache_configuration`.\n"]
    pub fn set_read_cache_configuration(
        self,
        v: impl Into<BlockAssignable<FsxOpenzfsFileSystemReadCacheConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().read_cache_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.read_cache_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `root_volume_configuration`.\n"]
    pub fn set_root_volume_configuration(
        self,
        v: impl Into<BlockAssignable<FsxOpenzfsFileSystemRootVolumeConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().root_volume_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.root_volume_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FsxOpenzfsFileSystemTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `automatic_backup_retention_days` after provisioning.\n"]
    pub fn automatic_backup_retention_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.automatic_backup_retention_days", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_id` after provisioning.\n"]
    pub fn backup_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `copy_tags_to_backups` after provisioning.\n"]
    pub fn copy_tags_to_backups(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.copy_tags_to_backups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `copy_tags_to_volumes` after provisioning.\n"]
    pub fn copy_tags_to_volumes(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.copy_tags_to_volumes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `daily_automatic_backup_start_time` after provisioning.\n"]
    pub fn daily_automatic_backup_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.daily_automatic_backup_start_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_options` after provisioning.\n"]
    pub fn delete_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.delete_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deployment_type` after provisioning.\n"]
    pub fn deployment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_ip_address` after provisioning.\n"]
    pub fn endpoint_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_ip_address_range` after provisioning.\n"]
    pub fn endpoint_ip_address_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_ip_address_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `final_backup_tags` after provisioning.\n"]
    pub fn final_backup_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.final_backup_tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_interface_ids` after provisioning.\n"]
    pub fn network_interface_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_interface_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `preferred_subnet_id` after provisioning.\n"]
    pub fn preferred_subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preferred_subnet_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `root_volume_id` after provisioning.\n"]
    pub fn root_volume_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_volume_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `route_table_ids` after provisioning.\n"]
    pub fn route_table_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.route_table_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `skip_final_backup` after provisioning.\n"]
    pub fn skip_final_backup(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_final_backup", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_capacity` after provisioning.\n"]
    pub fn storage_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_capacity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subnet_ids", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `throughput_capacity` after provisioning.\n"]
    pub fn throughput_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throughput_capacity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `weekly_maintenance_start_time` after provisioning.\n"]
    pub fn weekly_maintenance_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.weekly_maintenance_start_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disk_iops_configuration` after provisioning.\n"]
    pub fn disk_iops_configuration(
        &self,
    ) -> ListRef<FsxOpenzfsFileSystemDiskIopsConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk_iops_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `read_cache_configuration` after provisioning.\n"]
    pub fn read_cache_configuration(
        &self,
    ) -> ListRef<FsxOpenzfsFileSystemReadCacheConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.read_cache_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `root_volume_configuration` after provisioning.\n"]
    pub fn root_volume_configuration(
        &self,
    ) -> ListRef<FsxOpenzfsFileSystemRootVolumeConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.root_volume_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxOpenzfsFileSystemTimeoutsElRef {
        FsxOpenzfsFileSystemTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for FsxOpenzfsFileSystem {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for FsxOpenzfsFileSystem {}
impl ToListMappable for FsxOpenzfsFileSystem {
    type O = ListRef<FsxOpenzfsFileSystemRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for FsxOpenzfsFileSystem_ {
    fn extract_resource_type(&self) -> String {
        "aws_fsx_openzfs_file_system".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildFsxOpenzfsFileSystem {
    pub tf_id: String,
    #[doc = ""]
    pub deployment_type: PrimField<String>,
    #[doc = ""]
    pub subnet_ids: ListField<PrimField<String>>,
    #[doc = ""]
    pub throughput_capacity: PrimField<f64>,
}
impl BuildFsxOpenzfsFileSystem {
    pub fn build(self, stack: &mut Stack) -> FsxOpenzfsFileSystem {
        let out = FsxOpenzfsFileSystem(Rc::new(FsxOpenzfsFileSystem_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FsxOpenzfsFileSystemData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                automatic_backup_retention_days: core::default::Default::default(),
                backup_id: core::default::Default::default(),
                copy_tags_to_backups: core::default::Default::default(),
                copy_tags_to_volumes: core::default::Default::default(),
                daily_automatic_backup_start_time: core::default::Default::default(),
                delete_options: core::default::Default::default(),
                deployment_type: self.deployment_type,
                endpoint_ip_address_range: core::default::Default::default(),
                final_backup_tags: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                preferred_subnet_id: core::default::Default::default(),
                region: core::default::Default::default(),
                route_table_ids: core::default::Default::default(),
                security_group_ids: core::default::Default::default(),
                skip_final_backup: core::default::Default::default(),
                storage_capacity: core::default::Default::default(),
                storage_type: core::default::Default::default(),
                subnet_ids: self.subnet_ids,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                throughput_capacity: self.throughput_capacity,
                weekly_maintenance_start_time: core::default::Default::default(),
                disk_iops_configuration: core::default::Default::default(),
                read_cache_configuration: core::default::Default::default(),
                root_volume_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct FsxOpenzfsFileSystemRef {
    shared: StackShared,
    base: String,
}
impl Ref for FsxOpenzfsFileSystemRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl FsxOpenzfsFileSystemRef {
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
    #[doc = "Get a reference to the value of field `automatic_backup_retention_days` after provisioning.\n"]
    pub fn automatic_backup_retention_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.automatic_backup_retention_days", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_id` after provisioning.\n"]
    pub fn backup_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `copy_tags_to_backups` after provisioning.\n"]
    pub fn copy_tags_to_backups(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.copy_tags_to_backups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `copy_tags_to_volumes` after provisioning.\n"]
    pub fn copy_tags_to_volumes(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.copy_tags_to_volumes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `daily_automatic_backup_start_time` after provisioning.\n"]
    pub fn daily_automatic_backup_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.daily_automatic_backup_start_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_options` after provisioning.\n"]
    pub fn delete_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.delete_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deployment_type` after provisioning.\n"]
    pub fn deployment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_ip_address` after provisioning.\n"]
    pub fn endpoint_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_ip_address_range` after provisioning.\n"]
    pub fn endpoint_ip_address_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_ip_address_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `final_backup_tags` after provisioning.\n"]
    pub fn final_backup_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.final_backup_tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_interface_ids` after provisioning.\n"]
    pub fn network_interface_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_interface_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `preferred_subnet_id` after provisioning.\n"]
    pub fn preferred_subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preferred_subnet_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `root_volume_id` after provisioning.\n"]
    pub fn root_volume_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_volume_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `route_table_ids` after provisioning.\n"]
    pub fn route_table_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.route_table_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `skip_final_backup` after provisioning.\n"]
    pub fn skip_final_backup(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_final_backup", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_capacity` after provisioning.\n"]
    pub fn storage_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_capacity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subnet_ids", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `throughput_capacity` after provisioning.\n"]
    pub fn throughput_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throughput_capacity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `weekly_maintenance_start_time` after provisioning.\n"]
    pub fn weekly_maintenance_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.weekly_maintenance_start_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disk_iops_configuration` after provisioning.\n"]
    pub fn disk_iops_configuration(
        &self,
    ) -> ListRef<FsxOpenzfsFileSystemDiskIopsConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk_iops_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `read_cache_configuration` after provisioning.\n"]
    pub fn read_cache_configuration(
        &self,
    ) -> ListRef<FsxOpenzfsFileSystemReadCacheConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.read_cache_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `root_volume_configuration` after provisioning.\n"]
    pub fn root_volume_configuration(
        &self,
    ) -> ListRef<FsxOpenzfsFileSystemRootVolumeConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.root_volume_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxOpenzfsFileSystemTimeoutsElRef {
        FsxOpenzfsFileSystemTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct FsxOpenzfsFileSystemDiskIopsConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}
impl FsxOpenzfsFileSystemDiskIopsConfigurationEl {
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
impl ToListMappable for FsxOpenzfsFileSystemDiskIopsConfigurationEl {
    type O = BlockAssignable<FsxOpenzfsFileSystemDiskIopsConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFsxOpenzfsFileSystemDiskIopsConfigurationEl {}
impl BuildFsxOpenzfsFileSystemDiskIopsConfigurationEl {
    pub fn build(self) -> FsxOpenzfsFileSystemDiskIopsConfigurationEl {
        FsxOpenzfsFileSystemDiskIopsConfigurationEl {
            iops: core::default::Default::default(),
            mode: core::default::Default::default(),
        }
    }
}
pub struct FsxOpenzfsFileSystemDiskIopsConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FsxOpenzfsFileSystemDiskIopsConfigurationElRef {
    fn new(shared: StackShared, base: String) -> FsxOpenzfsFileSystemDiskIopsConfigurationElRef {
        FsxOpenzfsFileSystemDiskIopsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FsxOpenzfsFileSystemDiskIopsConfigurationElRef {
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
pub struct FsxOpenzfsFileSystemReadCacheConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sizing_mode: Option<PrimField<String>>,
}
impl FsxOpenzfsFileSystemReadCacheConfigurationEl {
    #[doc = "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size = Some(v.into());
        self
    }
    #[doc = "Set the field `sizing_mode`.\n"]
    pub fn set_sizing_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sizing_mode = Some(v.into());
        self
    }
}
impl ToListMappable for FsxOpenzfsFileSystemReadCacheConfigurationEl {
    type O = BlockAssignable<FsxOpenzfsFileSystemReadCacheConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFsxOpenzfsFileSystemReadCacheConfigurationEl {}
impl BuildFsxOpenzfsFileSystemReadCacheConfigurationEl {
    pub fn build(self) -> FsxOpenzfsFileSystemReadCacheConfigurationEl {
        FsxOpenzfsFileSystemReadCacheConfigurationEl {
            size: core::default::Default::default(),
            sizing_mode: core::default::Default::default(),
        }
    }
}
pub struct FsxOpenzfsFileSystemReadCacheConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FsxOpenzfsFileSystemReadCacheConfigurationElRef {
    fn new(shared: StackShared, base: String) -> FsxOpenzfsFileSystemReadCacheConfigurationElRef {
        FsxOpenzfsFileSystemReadCacheConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FsxOpenzfsFileSystemReadCacheConfigurationElRef {
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
pub struct FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsEl {
    clients: PrimField<String>,
    options: ListField<PrimField<String>>,
}
impl FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsEl {}
impl ToListMappable
    for FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsEl
{
    type O = BlockAssignable<
        FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsEl {
    #[doc = ""]
    pub clients: PrimField<String>,
    #[doc = ""]
    pub options: ListField<PrimField<String>>,
}
impl BuildFsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsEl {
    pub fn build(
        self,
    ) -> FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsEl {
        FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsEl {
            clients: self.clients,
            options: self.options,
        }
    }
}
pub struct FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsElRef {
        FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `clients` after provisioning.\n"]
    pub fn clients(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.clients", self.base))
    }
    #[doc = "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.options", self.base))
    }
}
#[derive(Serialize, Default)]
struct FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElDynamic {
    client_configurations: Option<
        DynamicBlock<
            FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_configurations: Option<
        Vec<FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsEl>,
    >,
    dynamic: FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElDynamic,
}
impl FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsEl {
    #[doc = "Set the field `client_configurations`.\n"]
    pub fn set_client_configurations(
        mut self,
        v: impl Into<
            BlockAssignable<
                FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.client_configurations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.client_configurations = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsEl {
    type O = BlockAssignable<FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsEl {}
impl BuildFsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsEl {
    pub fn build(self) -> FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsEl {
        FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsEl {
            client_configurations: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElRef {
        FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasEl {
    id: PrimField<f64>,
    storage_capacity_quota_gib: PrimField<f64>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasEl {}
impl ToListMappable for FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasEl {
    type O = BlockAssignable<FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasEl {
    #[doc = ""]
    pub id: PrimField<f64>,
    #[doc = ""]
    pub storage_capacity_quota_gib: PrimField<f64>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildFsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasEl {
    pub fn build(self) -> FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasEl {
        FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasEl {
            id: self.id,
            storage_capacity_quota_gib: self.storage_capacity_quota_gib,
            type_: self.type_,
        }
    }
}
pub struct FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasElRef {
        FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_capacity_quota_gib` after provisioning.\n"]
    pub fn storage_capacity_quota_gib(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_capacity_quota_gib", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize, Default)]
struct FsxOpenzfsFileSystemRootVolumeConfigurationElDynamic {
    nfs_exports: Option<DynamicBlock<FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsEl>>,
    user_and_group_quotas:
        Option<DynamicBlock<FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasEl>>,
}
#[derive(Serialize)]
pub struct FsxOpenzfsFileSystemRootVolumeConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_tags_to_snapshots: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_compression_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_size_kib: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nfs_exports: Option<Vec<FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_and_group_quotas:
        Option<Vec<FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasEl>>,
    dynamic: FsxOpenzfsFileSystemRootVolumeConfigurationElDynamic,
}
impl FsxOpenzfsFileSystemRootVolumeConfigurationEl {
    #[doc = "Set the field `copy_tags_to_snapshots`.\n"]
    pub fn set_copy_tags_to_snapshots(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.copy_tags_to_snapshots = Some(v.into());
        self
    }
    #[doc = "Set the field `data_compression_type`.\n"]
    pub fn set_data_compression_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_compression_type = Some(v.into());
        self
    }
    #[doc = "Set the field `read_only`.\n"]
    pub fn set_read_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.read_only = Some(v.into());
        self
    }
    #[doc = "Set the field `record_size_kib`.\n"]
    pub fn set_record_size_kib(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.record_size_kib = Some(v.into());
        self
    }
    #[doc = "Set the field `nfs_exports`.\n"]
    pub fn set_nfs_exports(
        mut self,
        v: impl Into<BlockAssignable<FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.nfs_exports = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.nfs_exports = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `user_and_group_quotas`.\n"]
    pub fn set_user_and_group_quotas(
        mut self,
        v: impl Into<BlockAssignable<FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.user_and_group_quotas = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.user_and_group_quotas = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for FsxOpenzfsFileSystemRootVolumeConfigurationEl {
    type O = BlockAssignable<FsxOpenzfsFileSystemRootVolumeConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFsxOpenzfsFileSystemRootVolumeConfigurationEl {}
impl BuildFsxOpenzfsFileSystemRootVolumeConfigurationEl {
    pub fn build(self) -> FsxOpenzfsFileSystemRootVolumeConfigurationEl {
        FsxOpenzfsFileSystemRootVolumeConfigurationEl {
            copy_tags_to_snapshots: core::default::Default::default(),
            data_compression_type: core::default::Default::default(),
            read_only: core::default::Default::default(),
            record_size_kib: core::default::Default::default(),
            nfs_exports: core::default::Default::default(),
            user_and_group_quotas: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FsxOpenzfsFileSystemRootVolumeConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FsxOpenzfsFileSystemRootVolumeConfigurationElRef {
    fn new(shared: StackShared, base: String) -> FsxOpenzfsFileSystemRootVolumeConfigurationElRef {
        FsxOpenzfsFileSystemRootVolumeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FsxOpenzfsFileSystemRootVolumeConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `copy_tags_to_snapshots` after provisioning.\n"]
    pub fn copy_tags_to_snapshots(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.copy_tags_to_snapshots", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_compression_type` after provisioning.\n"]
    pub fn data_compression_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_compression_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `read_only` after provisioning.\n"]
    pub fn read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }
    #[doc = "Get a reference to the value of field `record_size_kib` after provisioning.\n"]
    pub fn record_size_kib(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.record_size_kib", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nfs_exports` after provisioning.\n"]
    pub fn nfs_exports(
        &self,
    ) -> ListRef<FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.nfs_exports", self.base))
    }
}
#[derive(Serialize)]
pub struct FsxOpenzfsFileSystemTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl FsxOpenzfsFileSystemTimeoutsEl {
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
impl ToListMappable for FsxOpenzfsFileSystemTimeoutsEl {
    type O = BlockAssignable<FsxOpenzfsFileSystemTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFsxOpenzfsFileSystemTimeoutsEl {}
impl BuildFsxOpenzfsFileSystemTimeoutsEl {
    pub fn build(self) -> FsxOpenzfsFileSystemTimeoutsEl {
        FsxOpenzfsFileSystemTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct FsxOpenzfsFileSystemTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FsxOpenzfsFileSystemTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FsxOpenzfsFileSystemTimeoutsElRef {
        FsxOpenzfsFileSystemTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FsxOpenzfsFileSystemTimeoutsElRef {
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
struct FsxOpenzfsFileSystemDynamic {
    disk_iops_configuration: Option<DynamicBlock<FsxOpenzfsFileSystemDiskIopsConfigurationEl>>,
    read_cache_configuration: Option<DynamicBlock<FsxOpenzfsFileSystemReadCacheConfigurationEl>>,
    root_volume_configuration: Option<DynamicBlock<FsxOpenzfsFileSystemRootVolumeConfigurationEl>>,
}
