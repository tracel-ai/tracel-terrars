use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataFsxWindowsFileSystemData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataFsxWindowsFileSystem_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataFsxWindowsFileSystemData>,
}

#[derive(Clone)]
pub struct DataFsxWindowsFileSystem(Rc<DataFsxWindowsFileSystem_>);

impl DataFsxWindowsFileSystem {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `active_directory_id` after provisioning.\n"]
    pub fn active_directory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.active_directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aliases` after provisioning.\n"]
    pub fn aliases(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.aliases", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `audit_log_configuration` after provisioning.\n"]
    pub fn audit_log_configuration(&self) -> ListRef<DataFsxWindowsFileSystemAuditLogConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.audit_log_configuration", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `deployment_type` after provisioning.\n"]
    pub fn deployment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployment_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disk_iops_configuration` after provisioning.\n"]
    pub fn disk_iops_configuration(&self) -> ListRef<DataFsxWindowsFileSystemDiskIopsConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.disk_iops_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interface_ids` after provisioning.\n"]
    pub fn network_interface_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.network_interface_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_file_server_ip` after provisioning.\n"]
    pub fn preferred_file_server_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preferred_file_server_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_subnet_id` after provisioning.\n"]
    pub fn preferred_subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preferred_subnet_id", self.extract_ref()))
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
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
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
}

impl Referable for DataFsxWindowsFileSystem {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataFsxWindowsFileSystem { }

impl ToListMappable for DataFsxWindowsFileSystem {
    type O = ListRef<DataFsxWindowsFileSystemRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataFsxWindowsFileSystem_ {
    fn extract_datasource_type(&self) -> String {
        "aws_fsx_windows_file_system".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataFsxWindowsFileSystem {
    pub tf_id: String,
    #[doc = ""]
    pub id: PrimField<String>,
}

impl BuildDataFsxWindowsFileSystem {
    pub fn build(self, stack: &mut Stack) -> DataFsxWindowsFileSystem {
        let out = DataFsxWindowsFileSystem(Rc::new(DataFsxWindowsFileSystem_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataFsxWindowsFileSystemData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: self.id,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataFsxWindowsFileSystemRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataFsxWindowsFileSystemRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataFsxWindowsFileSystemRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `active_directory_id` after provisioning.\n"]
    pub fn active_directory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.active_directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aliases` after provisioning.\n"]
    pub fn aliases(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.aliases", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `audit_log_configuration` after provisioning.\n"]
    pub fn audit_log_configuration(&self) -> ListRef<DataFsxWindowsFileSystemAuditLogConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.audit_log_configuration", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `deployment_type` after provisioning.\n"]
    pub fn deployment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployment_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disk_iops_configuration` after provisioning.\n"]
    pub fn disk_iops_configuration(&self) -> ListRef<DataFsxWindowsFileSystemDiskIopsConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.disk_iops_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interface_ids` after provisioning.\n"]
    pub fn network_interface_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.network_interface_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_file_server_ip` after provisioning.\n"]
    pub fn preferred_file_server_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preferred_file_server_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_subnet_id` after provisioning.\n"]
    pub fn preferred_subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preferred_subnet_id", self.extract_ref()))
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
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
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
}

#[derive(Serialize)]
pub struct DataFsxWindowsFileSystemAuditLogConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audit_log_destination: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_access_audit_log_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_share_access_audit_log_level: Option<PrimField<String>>,
}

impl DataFsxWindowsFileSystemAuditLogConfigurationEl {
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
    pub fn set_file_share_access_audit_log_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_share_access_audit_log_level = Some(v.into());
        self
    }
}

impl ToListMappable for DataFsxWindowsFileSystemAuditLogConfigurationEl {
    type O = BlockAssignable<DataFsxWindowsFileSystemAuditLogConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFsxWindowsFileSystemAuditLogConfigurationEl {}

impl BuildDataFsxWindowsFileSystemAuditLogConfigurationEl {
    pub fn build(self) -> DataFsxWindowsFileSystemAuditLogConfigurationEl {
        DataFsxWindowsFileSystemAuditLogConfigurationEl {
            audit_log_destination: core::default::Default::default(),
            file_access_audit_log_level: core::default::Default::default(),
            file_share_access_audit_log_level: core::default::Default::default(),
        }
    }
}

pub struct DataFsxWindowsFileSystemAuditLogConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataFsxWindowsFileSystemAuditLogConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataFsxWindowsFileSystemAuditLogConfigurationElRef {
        DataFsxWindowsFileSystemAuditLogConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFsxWindowsFileSystemAuditLogConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audit_log_destination` after provisioning.\n"]
    pub fn audit_log_destination(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.audit_log_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `file_access_audit_log_level` after provisioning.\n"]
    pub fn file_access_audit_log_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_access_audit_log_level", self.base))
    }

    #[doc = "Get a reference to the value of field `file_share_access_audit_log_level` after provisioning.\n"]
    pub fn file_share_access_audit_log_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_share_access_audit_log_level", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFsxWindowsFileSystemDiskIopsConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}

impl DataFsxWindowsFileSystemDiskIopsConfigurationEl {
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

impl ToListMappable for DataFsxWindowsFileSystemDiskIopsConfigurationEl {
    type O = BlockAssignable<DataFsxWindowsFileSystemDiskIopsConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFsxWindowsFileSystemDiskIopsConfigurationEl {}

impl BuildDataFsxWindowsFileSystemDiskIopsConfigurationEl {
    pub fn build(self) -> DataFsxWindowsFileSystemDiskIopsConfigurationEl {
        DataFsxWindowsFileSystemDiskIopsConfigurationEl {
            iops: core::default::Default::default(),
            mode: core::default::Default::default(),
        }
    }
}

pub struct DataFsxWindowsFileSystemDiskIopsConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataFsxWindowsFileSystemDiskIopsConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataFsxWindowsFileSystemDiskIopsConfigurationElRef {
        DataFsxWindowsFileSystemDiskIopsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFsxWindowsFileSystemDiskIopsConfigurationElRef {
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
