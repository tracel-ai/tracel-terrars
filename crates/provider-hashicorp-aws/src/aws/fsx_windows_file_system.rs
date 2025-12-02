use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct FsxWindowsFileSystemData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active_directory_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aliases: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_backup_retention_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_tags_to_backups: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    daily_automatic_backup_start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_type: Option<PrimField<String>>,
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
    audit_log_configuration: Option<Vec<FsxWindowsFileSystemAuditLogConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_iops_configuration: Option<Vec<FsxWindowsFileSystemDiskIopsConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_managed_active_directory: Option<Vec<FsxWindowsFileSystemSelfManagedActiveDirectoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FsxWindowsFileSystemTimeoutsEl>,
    dynamic: FsxWindowsFileSystemDynamic,
}
struct FsxWindowsFileSystem_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FsxWindowsFileSystemData>,
}
#[derive(Clone)]
pub struct FsxWindowsFileSystem(Rc<FsxWindowsFileSystem_>);
impl FsxWindowsFileSystem {
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
    #[doc = "Set the field `active_directory_id`.\n"]
    pub fn set_active_directory_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().active_directory_id = Some(v.into());
        self
    }
    #[doc = "Set the field `aliases`.\n"]
    pub fn set_aliases(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().aliases = Some(v.into());
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
    #[doc = "Set the field `deployment_type`.\n"]
    pub fn set_deployment_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deployment_type = Some(v.into());
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
    #[doc = "Set the field `audit_log_configuration`.\n"]
    pub fn set_audit_log_configuration(
        self,
        v: impl Into<BlockAssignable<FsxWindowsFileSystemAuditLogConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().audit_log_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.audit_log_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `disk_iops_configuration`.\n"]
    pub fn set_disk_iops_configuration(
        self,
        v: impl Into<BlockAssignable<FsxWindowsFileSystemDiskIopsConfigurationEl>>,
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
    #[doc = "Set the field `self_managed_active_directory`.\n"]
    pub fn set_self_managed_active_directory(
        self,
        v: impl Into<BlockAssignable<FsxWindowsFileSystemSelfManagedActiveDirectoryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().self_managed_active_directory = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .self_managed_active_directory = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FsxWindowsFileSystemTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `active_directory_id` after provisioning.\n"]
    pub fn active_directory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.active_directory_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `aliases` after provisioning.\n"]
    pub fn aliases(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.aliases", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `daily_automatic_backup_start_time` after provisioning.\n"]
    pub fn daily_automatic_backup_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.daily_automatic_backup_start_time", self.extract_ref()),
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
    pub fn network_interface_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
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
    #[doc = "Get a reference to the value of field `preferred_file_server_ip` after provisioning.\n"]
    pub fn preferred_file_server_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preferred_file_server_ip", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `remote_administration_endpoint` after provisioning.\n"]
    pub fn remote_administration_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.remote_administration_endpoint", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `audit_log_configuration` after provisioning.\n"]
    pub fn audit_log_configuration(
        &self,
    ) -> ListRef<FsxWindowsFileSystemAuditLogConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.audit_log_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disk_iops_configuration` after provisioning.\n"]
    pub fn disk_iops_configuration(
        &self,
    ) -> ListRef<FsxWindowsFileSystemDiskIopsConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk_iops_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_managed_active_directory` after provisioning.\n"]
    pub fn self_managed_active_directory(
        &self,
    ) -> ListRef<FsxWindowsFileSystemSelfManagedActiveDirectoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.self_managed_active_directory", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxWindowsFileSystemTimeoutsElRef {
        FsxWindowsFileSystemTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for FsxWindowsFileSystem {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for FsxWindowsFileSystem {}
impl ToListMappable for FsxWindowsFileSystem {
    type O = ListRef<FsxWindowsFileSystemRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for FsxWindowsFileSystem_ {
    fn extract_resource_type(&self) -> String {
        "aws_fsx_windows_file_system".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildFsxWindowsFileSystem {
    pub tf_id: String,
    #[doc = ""]
    pub subnet_ids: ListField<PrimField<String>>,
    #[doc = ""]
    pub throughput_capacity: PrimField<f64>,
}
impl BuildFsxWindowsFileSystem {
    pub fn build(self, stack: &mut Stack) -> FsxWindowsFileSystem {
        let out = FsxWindowsFileSystem(Rc::new(FsxWindowsFileSystem_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FsxWindowsFileSystemData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                active_directory_id: core::default::Default::default(),
                aliases: core::default::Default::default(),
                automatic_backup_retention_days: core::default::Default::default(),
                backup_id: core::default::Default::default(),
                copy_tags_to_backups: core::default::Default::default(),
                daily_automatic_backup_start_time: core::default::Default::default(),
                deployment_type: core::default::Default::default(),
                final_backup_tags: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                preferred_subnet_id: core::default::Default::default(),
                region: core::default::Default::default(),
                security_group_ids: core::default::Default::default(),
                skip_final_backup: core::default::Default::default(),
                storage_capacity: core::default::Default::default(),
                storage_type: core::default::Default::default(),
                subnet_ids: self.subnet_ids,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                throughput_capacity: self.throughput_capacity,
                weekly_maintenance_start_time: core::default::Default::default(),
                audit_log_configuration: core::default::Default::default(),
                disk_iops_configuration: core::default::Default::default(),
                self_managed_active_directory: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct FsxWindowsFileSystemRef {
    shared: StackShared,
    base: String,
}
impl Ref for FsxWindowsFileSystemRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl FsxWindowsFileSystemRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `active_directory_id` after provisioning.\n"]
    pub fn active_directory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.active_directory_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `aliases` after provisioning.\n"]
    pub fn aliases(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.aliases", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `daily_automatic_backup_start_time` after provisioning.\n"]
    pub fn daily_automatic_backup_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.daily_automatic_backup_start_time", self.extract_ref()),
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
    pub fn network_interface_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
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
    #[doc = "Get a reference to the value of field `preferred_file_server_ip` after provisioning.\n"]
    pub fn preferred_file_server_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preferred_file_server_ip", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `remote_administration_endpoint` after provisioning.\n"]
    pub fn remote_administration_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.remote_administration_endpoint", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `audit_log_configuration` after provisioning.\n"]
    pub fn audit_log_configuration(
        &self,
    ) -> ListRef<FsxWindowsFileSystemAuditLogConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.audit_log_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disk_iops_configuration` after provisioning.\n"]
    pub fn disk_iops_configuration(
        &self,
    ) -> ListRef<FsxWindowsFileSystemDiskIopsConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk_iops_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_managed_active_directory` after provisioning.\n"]
    pub fn self_managed_active_directory(
        &self,
    ) -> ListRef<FsxWindowsFileSystemSelfManagedActiveDirectoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.self_managed_active_directory", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxWindowsFileSystemTimeoutsElRef {
        FsxWindowsFileSystemTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct FsxWindowsFileSystemAuditLogConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audit_log_destination: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_access_audit_log_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_share_access_audit_log_level: Option<PrimField<String>>,
}
impl FsxWindowsFileSystemAuditLogConfigurationEl {
    #[doc = "Set the field `audit_log_destination`.\n"]
    pub fn set_audit_log_destination(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.audit_log_destination = Some(v.into());
        self
    }
    #[doc = "Set the field `file_access_audit_log_level`.\n"]
    pub fn set_file_access_audit_log_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_access_audit_log_level = Some(v.into());
        self
    }
    #[doc = "Set the field `file_share_access_audit_log_level`.\n"]
    pub fn set_file_share_access_audit_log_level(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.file_share_access_audit_log_level = Some(v.into());
        self
    }
}
impl ToListMappable for FsxWindowsFileSystemAuditLogConfigurationEl {
    type O = BlockAssignable<FsxWindowsFileSystemAuditLogConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFsxWindowsFileSystemAuditLogConfigurationEl {}
impl BuildFsxWindowsFileSystemAuditLogConfigurationEl {
    pub fn build(self) -> FsxWindowsFileSystemAuditLogConfigurationEl {
        FsxWindowsFileSystemAuditLogConfigurationEl {
            audit_log_destination: core::default::Default::default(),
            file_access_audit_log_level: core::default::Default::default(),
            file_share_access_audit_log_level: core::default::Default::default(),
        }
    }
}
pub struct FsxWindowsFileSystemAuditLogConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FsxWindowsFileSystemAuditLogConfigurationElRef {
    fn new(shared: StackShared, base: String) -> FsxWindowsFileSystemAuditLogConfigurationElRef {
        FsxWindowsFileSystemAuditLogConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FsxWindowsFileSystemAuditLogConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `audit_log_destination` after provisioning.\n"]
    pub fn audit_log_destination(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.audit_log_destination", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `file_access_audit_log_level` after provisioning.\n"]
    pub fn file_access_audit_log_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_access_audit_log_level", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `file_share_access_audit_log_level` after provisioning.\n"]
    pub fn file_share_access_audit_log_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_share_access_audit_log_level", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct FsxWindowsFileSystemDiskIopsConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}
impl FsxWindowsFileSystemDiskIopsConfigurationEl {
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
impl ToListMappable for FsxWindowsFileSystemDiskIopsConfigurationEl {
    type O = BlockAssignable<FsxWindowsFileSystemDiskIopsConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFsxWindowsFileSystemDiskIopsConfigurationEl {}
impl BuildFsxWindowsFileSystemDiskIopsConfigurationEl {
    pub fn build(self) -> FsxWindowsFileSystemDiskIopsConfigurationEl {
        FsxWindowsFileSystemDiskIopsConfigurationEl {
            iops: core::default::Default::default(),
            mode: core::default::Default::default(),
        }
    }
}
pub struct FsxWindowsFileSystemDiskIopsConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FsxWindowsFileSystemDiskIopsConfigurationElRef {
    fn new(shared: StackShared, base: String) -> FsxWindowsFileSystemDiskIopsConfigurationElRef {
        FsxWindowsFileSystemDiskIopsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FsxWindowsFileSystemDiskIopsConfigurationElRef {
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
pub struct FsxWindowsFileSystemSelfManagedActiveDirectoryEl {
    dns_ips: SetField<PrimField<String>>,
    domain_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_system_administrators_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organizational_unit_distinguished_name: Option<PrimField<String>>,
    password: PrimField<String>,
    username: PrimField<String>,
}
impl FsxWindowsFileSystemSelfManagedActiveDirectoryEl {
    #[doc = "Set the field `file_system_administrators_group`.\n"]
    pub fn set_file_system_administrators_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_system_administrators_group = Some(v.into());
        self
    }
    #[doc = "Set the field `organizational_unit_distinguished_name`.\n"]
    pub fn set_organizational_unit_distinguished_name(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.organizational_unit_distinguished_name = Some(v.into());
        self
    }
}
impl ToListMappable for FsxWindowsFileSystemSelfManagedActiveDirectoryEl {
    type O = BlockAssignable<FsxWindowsFileSystemSelfManagedActiveDirectoryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFsxWindowsFileSystemSelfManagedActiveDirectoryEl {
    #[doc = ""]
    pub dns_ips: SetField<PrimField<String>>,
    #[doc = ""]
    pub domain_name: PrimField<String>,
    #[doc = ""]
    pub password: PrimField<String>,
    #[doc = ""]
    pub username: PrimField<String>,
}
impl BuildFsxWindowsFileSystemSelfManagedActiveDirectoryEl {
    pub fn build(self) -> FsxWindowsFileSystemSelfManagedActiveDirectoryEl {
        FsxWindowsFileSystemSelfManagedActiveDirectoryEl {
            dns_ips: self.dns_ips,
            domain_name: self.domain_name,
            file_system_administrators_group: core::default::Default::default(),
            organizational_unit_distinguished_name: core::default::Default::default(),
            password: self.password,
            username: self.username,
        }
    }
}
pub struct FsxWindowsFileSystemSelfManagedActiveDirectoryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FsxWindowsFileSystemSelfManagedActiveDirectoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FsxWindowsFileSystemSelfManagedActiveDirectoryElRef {
        FsxWindowsFileSystemSelfManagedActiveDirectoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FsxWindowsFileSystemSelfManagedActiveDirectoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dns_ips` after provisioning.\n"]
    pub fn dns_ips(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.dns_ips", self.base))
    }
    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }
    #[doc = "Get a reference to the value of field `file_system_administrators_group` after provisioning.\n"]
    pub fn file_system_administrators_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_system_administrators_group", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `organizational_unit_distinguished_name` after provisioning.\n"]
    pub fn organizational_unit_distinguished_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organizational_unit_distinguished_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize)]
pub struct FsxWindowsFileSystemTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl FsxWindowsFileSystemTimeoutsEl {
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
impl ToListMappable for FsxWindowsFileSystemTimeoutsEl {
    type O = BlockAssignable<FsxWindowsFileSystemTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFsxWindowsFileSystemTimeoutsEl {}
impl BuildFsxWindowsFileSystemTimeoutsEl {
    pub fn build(self) -> FsxWindowsFileSystemTimeoutsEl {
        FsxWindowsFileSystemTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct FsxWindowsFileSystemTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FsxWindowsFileSystemTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FsxWindowsFileSystemTimeoutsElRef {
        FsxWindowsFileSystemTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FsxWindowsFileSystemTimeoutsElRef {
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
struct FsxWindowsFileSystemDynamic {
    audit_log_configuration: Option<DynamicBlock<FsxWindowsFileSystemAuditLogConfigurationEl>>,
    disk_iops_configuration: Option<DynamicBlock<FsxWindowsFileSystemDiskIopsConfigurationEl>>,
    self_managed_active_directory:
        Option<DynamicBlock<FsxWindowsFileSystemSelfManagedActiveDirectoryEl>>,
}
