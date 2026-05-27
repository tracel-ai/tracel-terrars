use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SqlDatabaseInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backupdr_backup: Option<PrimField<String>>,
    database_version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    final_backup_description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    master_instance_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replica_names: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_password_wo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_password_wo_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    clone: Option<Vec<SqlDatabaseInstanceCloneEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    point_in_time_restore_context: Option<Vec<SqlDatabaseInstancePointInTimeRestoreContextEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replica_configuration: Option<Vec<SqlDatabaseInstanceReplicaConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replication_cluster: Option<Vec<SqlDatabaseInstanceReplicationClusterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restore_backup_context: Option<Vec<SqlDatabaseInstanceRestoreBackupContextEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<Vec<SqlDatabaseInstanceSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SqlDatabaseInstanceTimeoutsEl>,
    dynamic: SqlDatabaseInstanceDynamic,
}
struct SqlDatabaseInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SqlDatabaseInstanceData>,
}
#[derive(Clone)]
pub struct SqlDatabaseInstance(Rc<SqlDatabaseInstance_>);
impl SqlDatabaseInstance {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `backupdr_backup`.\nThe name of the BackupDR backup to restore from."]
    pub fn set_backupdr_backup(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().backupdr_backup = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_protection`.\nUsed to block Terraform from deleting a SQL Instance. Defaults to true."]
    pub fn set_deletion_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_key_name`.\n"]
    pub fn set_encryption_key_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().encryption_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `final_backup_description`.\nThe description of final backup if instance enable create final backup during instance deletion. "]
    pub fn set_final_backup_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().final_backup_description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_type`.\nThe type of the instance. See https://cloud.google.com/sql/docs/mysql/admin-api/rest/v1/instances#SqlInstanceType for supported values."]
    pub fn set_instance_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().instance_type = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_version`.\nMaintenance version."]
    pub fn set_maintenance_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().maintenance_version = Some(v.into());
        self
    }
    #[doc = "Set the field `master_instance_name`.\nThe name of the instance that will act as the master in the replication setup. Note, this requires the master to have binary_log_enabled set, as well as existing backups."]
    pub fn set_master_instance_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().master_instance_name = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nThe name of the instance. If the name is left blank, Terraform will randomly generate one when the instance is first created. This is done because after a name is used, it cannot be reused for up to one week."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `node_count`.\nFor a read pool instance, the number of nodes in the read pool. For read pools with auto scaling enabled, this field is read only."]
    pub fn set_node_count(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nThe region the instance will sit in. Note, Cloud SQL is not available in all regions. A valid region must be provided to use this resource. If a region is not provided in the resource definition, the provider region will be used instead, but this will be an apply-time error for instances if the provider region is not supported with Cloud SQL. If you choose not to provide the region argument for this resource, make sure you understand this."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `replica_names`.\nThe replicas of the instance."]
    pub fn set_replica_names(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().replica_names = Some(v.into());
        self
    }
    #[doc = "Set the field `root_password`.\nInitial root password. Required for MS SQL Server."]
    pub fn set_root_password(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().root_password = Some(v.into());
        self
    }
    #[doc = "Set the field `root_password_wo`.\nInitial root password. Required for MS SQL Server.\n\t\t\t\tNote: This property is write-only and will not be read from the API. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn set_root_password_wo(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().root_password_wo = Some(v.into());
        self
    }
    #[doc = "Set the field `root_password_wo_version`.\nTriggers update of root_password_wo write-only. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn set_root_password_wo_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().root_password_wo_version = Some(v.into());
        self
    }
    #[doc = "Set the field `clone`.\n"]
    pub fn set_clone(self, v: impl Into<BlockAssignable<SqlDatabaseInstanceCloneEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().clone = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.clone = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `point_in_time_restore_context`.\n"]
    pub fn set_point_in_time_restore_context(
        self,
        v: impl Into<BlockAssignable<SqlDatabaseInstancePointInTimeRestoreContextEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().point_in_time_restore_context = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .point_in_time_restore_context = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `replica_configuration`.\n"]
    pub fn set_replica_configuration(
        self,
        v: impl Into<BlockAssignable<SqlDatabaseInstanceReplicaConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().replica_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.replica_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `replication_cluster`.\n"]
    pub fn set_replication_cluster(
        self,
        v: impl Into<BlockAssignable<SqlDatabaseInstanceReplicationClusterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().replication_cluster = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.replication_cluster = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `restore_backup_context`.\n"]
    pub fn set_restore_backup_context(
        self,
        v: impl Into<BlockAssignable<SqlDatabaseInstanceRestoreBackupContextEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().restore_backup_context = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.restore_backup_context = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(
        self,
        v: impl Into<BlockAssignable<SqlDatabaseInstanceSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<SqlDatabaseInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `available_maintenance_versions` after provisioning.\nAvailable Maintenance versions."]
    pub fn available_maintenance_versions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.available_maintenance_versions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backupdr_backup` after provisioning.\nThe name of the BackupDR backup to restore from."]
    pub fn backupdr_backup(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backupdr_backup", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_name` after provisioning.\nThe connection name of the instance to be used in connection strings. For example, when connecting with Cloud SQL Proxy."]
    pub fn connection_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_version` after provisioning.\nThe MySQL, PostgreSQL or SQL Server version to use. Supported values include MYSQL_5_6, MYSQL_5_7, MYSQL_8_0, MYSQL_8_4, POSTGRES_9_6, POSTGRES_10, POSTGRES_11, POSTGRES_12, POSTGRES_13, POSTGRES_14, POSTGRES_15, POSTGRES_16, POSTGRES_17, POSTGRES_18, SQLSERVER_2022_STANDARD, SQLSERVER_2022_ENTERPRISE, SQLSERVER_2022_EXPRESS, SQLSERVER_2022_WEB, SQLSERVER_2025_STANDARD, SQLSERVER_2025_ENTERPRISE, SQLSERVER_2025_EXPRESS, SQLSERVER_2025_WEB. Database Version Policies includes an up-to-date reference of supported versions."]
    pub fn database_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nUsed to block Terraform from deleting a SQL Instance. Defaults to true."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\nThe instance-level dns name of the instance for PSC instances or public IP CAS instances."]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_names` after provisioning.\nThe list of DNS names used by this instance. Different connection types for an instance may have different DNS names. DNS names can apply to an individual instance or a cluster of instances."]
    pub fn dns_names(&self) -> ListRef<SqlDatabaseInstanceDnsNamesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dns_names", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_key_name` after provisioning.\n"]
    pub fn encryption_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `final_backup_description` after provisioning.\nThe description of final backup if instance enable create final backup during instance deletion. "]
    pub fn final_backup_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.final_backup_description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `first_ip_address` after provisioning.\nThe first IPv4 address of any type assigned. This is to support accessing the first address in the list in a terraform output when the resource is configured with a count."]
    pub fn first_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.first_ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\nThe type of the instance. See https://cloud.google.com/sql/docs/mysql/admin-api/rest/v1/instances#SqlInstanceType for supported values."]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> ListRef<SqlDatabaseInstanceIpAddressElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_version` after provisioning.\nMaintenance version."]
    pub fn maintenance_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `master_instance_name` after provisioning.\nThe name of the instance that will act as the master in the replication setup. Note, this requires the master to have binary_log_enabled set, as well as existing backups."]
    pub fn master_instance_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_instance_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the instance. If the name is left blank, Terraform will randomly generate one when the instance is first created. This is done because after a name is used, it cannot be reused for up to one week."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_count` after provisioning.\nFor a read pool instance, the number of nodes in the read pool. For read pools with auto scaling enabled, this field is read only."]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_ip_address` after provisioning.\nIPv4 address assigned. This is a workaround for an issue fixed in Terraform 0.12 but also provides a convenient way to access an IP of a specific type without performing filtering in a Terraform config."]
    pub fn private_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_service_attachment_link` after provisioning.\nThe link to service attachment of PSC instance."]
    pub fn psc_service_attachment_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.psc_service_attachment_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `public_ip_address` after provisioning.\nIPv4 address assigned. This is a workaround for an issue fixed in Terraform 0.12 but also provides a convenient way to access an IP of a specific type without performing filtering in a Terraform config."]
    pub fn public_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region the instance will sit in. Note, Cloud SQL is not available in all regions. A valid region must be provided to use this resource. If a region is not provided in the resource definition, the provider region will be used instead, but this will be an apply-time error for instances if the provider region is not supported with Cloud SQL. If you choose not to provide the region argument for this resource, make sure you understand this."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replica_names` after provisioning.\nThe replicas of the instance."]
    pub fn replica_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replica_names", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `root_password` after provisioning.\nInitial root password. Required for MS SQL Server."]
    pub fn root_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_password", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `root_password_wo` after provisioning.\nInitial root password. Required for MS SQL Server.\n\t\t\t\tNote: This property is write-only and will not be read from the API. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn root_password_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_password_wo", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `root_password_wo_version` after provisioning.\nTriggers update of root_password_wo write-only. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn root_password_wo_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_password_wo_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_ca_cert` after provisioning.\n"]
    pub fn server_ca_cert(&self) -> ListRef<SqlDatabaseInstanceServerCaCertElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.server_ca_cert", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account_email_address` after provisioning.\nThe service account email address assigned to the instance."]
    pub fn service_account_email_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_email_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `clone` after provisioning.\n"]
    pub fn clone(&self) -> ListRef<SqlDatabaseInstanceCloneElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.clone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `point_in_time_restore_context` after provisioning.\n"]
    pub fn point_in_time_restore_context(
        &self,
    ) -> ListRef<SqlDatabaseInstancePointInTimeRestoreContextElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.point_in_time_restore_context", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replica_configuration` after provisioning.\n"]
    pub fn replica_configuration(&self) -> ListRef<SqlDatabaseInstanceReplicaConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replica_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replication_cluster` after provisioning.\n"]
    pub fn replication_cluster(&self) -> ListRef<SqlDatabaseInstanceReplicationClusterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replication_cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restore_backup_context` after provisioning.\n"]
    pub fn restore_backup_context(&self) -> ListRef<SqlDatabaseInstanceRestoreBackupContextElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restore_backup_context", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> ListRef<SqlDatabaseInstanceSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SqlDatabaseInstanceTimeoutsElRef {
        SqlDatabaseInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for SqlDatabaseInstance {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SqlDatabaseInstance {}
impl ToListMappable for SqlDatabaseInstance {
    type O = ListRef<SqlDatabaseInstanceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SqlDatabaseInstance_ {
    fn extract_resource_type(&self) -> String {
        "google_sql_database_instance".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSqlDatabaseInstance {
    pub tf_id: String,
    #[doc = "The MySQL, PostgreSQL or SQL Server version to use. Supported values include MYSQL_5_6, MYSQL_5_7, MYSQL_8_0, MYSQL_8_4, POSTGRES_9_6, POSTGRES_10, POSTGRES_11, POSTGRES_12, POSTGRES_13, POSTGRES_14, POSTGRES_15, POSTGRES_16, POSTGRES_17, POSTGRES_18, SQLSERVER_2022_STANDARD, SQLSERVER_2022_ENTERPRISE, SQLSERVER_2022_EXPRESS, SQLSERVER_2022_WEB, SQLSERVER_2025_STANDARD, SQLSERVER_2025_ENTERPRISE, SQLSERVER_2025_EXPRESS, SQLSERVER_2025_WEB. Database Version Policies includes an up-to-date reference of supported versions."]
    pub database_version: PrimField<String>,
}
impl BuildSqlDatabaseInstance {
    pub fn build(self, stack: &mut Stack) -> SqlDatabaseInstance {
        let out = SqlDatabaseInstance(Rc::new(SqlDatabaseInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SqlDatabaseInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                backupdr_backup: core::default::Default::default(),
                database_version: self.database_version,
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                encryption_key_name: core::default::Default::default(),
                final_backup_description: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_type: core::default::Default::default(),
                maintenance_version: core::default::Default::default(),
                master_instance_name: core::default::Default::default(),
                name: core::default::Default::default(),
                node_count: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                replica_names: core::default::Default::default(),
                root_password: core::default::Default::default(),
                root_password_wo: core::default::Default::default(),
                root_password_wo_version: core::default::Default::default(),
                clone: core::default::Default::default(),
                point_in_time_restore_context: core::default::Default::default(),
                replica_configuration: core::default::Default::default(),
                replication_cluster: core::default::Default::default(),
                restore_backup_context: core::default::Default::default(),
                settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SqlDatabaseInstanceRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SqlDatabaseInstanceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `available_maintenance_versions` after provisioning.\nAvailable Maintenance versions."]
    pub fn available_maintenance_versions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.available_maintenance_versions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backupdr_backup` after provisioning.\nThe name of the BackupDR backup to restore from."]
    pub fn backupdr_backup(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backupdr_backup", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_name` after provisioning.\nThe connection name of the instance to be used in connection strings. For example, when connecting with Cloud SQL Proxy."]
    pub fn connection_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_version` after provisioning.\nThe MySQL, PostgreSQL or SQL Server version to use. Supported values include MYSQL_5_6, MYSQL_5_7, MYSQL_8_0, MYSQL_8_4, POSTGRES_9_6, POSTGRES_10, POSTGRES_11, POSTGRES_12, POSTGRES_13, POSTGRES_14, POSTGRES_15, POSTGRES_16, POSTGRES_17, POSTGRES_18, SQLSERVER_2022_STANDARD, SQLSERVER_2022_ENTERPRISE, SQLSERVER_2022_EXPRESS, SQLSERVER_2022_WEB, SQLSERVER_2025_STANDARD, SQLSERVER_2025_ENTERPRISE, SQLSERVER_2025_EXPRESS, SQLSERVER_2025_WEB. Database Version Policies includes an up-to-date reference of supported versions."]
    pub fn database_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nUsed to block Terraform from deleting a SQL Instance. Defaults to true."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\nThe instance-level dns name of the instance for PSC instances or public IP CAS instances."]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_names` after provisioning.\nThe list of DNS names used by this instance. Different connection types for an instance may have different DNS names. DNS names can apply to an individual instance or a cluster of instances."]
    pub fn dns_names(&self) -> ListRef<SqlDatabaseInstanceDnsNamesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dns_names", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_key_name` after provisioning.\n"]
    pub fn encryption_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `final_backup_description` after provisioning.\nThe description of final backup if instance enable create final backup during instance deletion. "]
    pub fn final_backup_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.final_backup_description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `first_ip_address` after provisioning.\nThe first IPv4 address of any type assigned. This is to support accessing the first address in the list in a terraform output when the resource is configured with a count."]
    pub fn first_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.first_ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\nThe type of the instance. See https://cloud.google.com/sql/docs/mysql/admin-api/rest/v1/instances#SqlInstanceType for supported values."]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> ListRef<SqlDatabaseInstanceIpAddressElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_version` after provisioning.\nMaintenance version."]
    pub fn maintenance_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `master_instance_name` after provisioning.\nThe name of the instance that will act as the master in the replication setup. Note, this requires the master to have binary_log_enabled set, as well as existing backups."]
    pub fn master_instance_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_instance_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the instance. If the name is left blank, Terraform will randomly generate one when the instance is first created. This is done because after a name is used, it cannot be reused for up to one week."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_count` after provisioning.\nFor a read pool instance, the number of nodes in the read pool. For read pools with auto scaling enabled, this field is read only."]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_ip_address` after provisioning.\nIPv4 address assigned. This is a workaround for an issue fixed in Terraform 0.12 but also provides a convenient way to access an IP of a specific type without performing filtering in a Terraform config."]
    pub fn private_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_service_attachment_link` after provisioning.\nThe link to service attachment of PSC instance."]
    pub fn psc_service_attachment_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.psc_service_attachment_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `public_ip_address` after provisioning.\nIPv4 address assigned. This is a workaround for an issue fixed in Terraform 0.12 but also provides a convenient way to access an IP of a specific type without performing filtering in a Terraform config."]
    pub fn public_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region the instance will sit in. Note, Cloud SQL is not available in all regions. A valid region must be provided to use this resource. If a region is not provided in the resource definition, the provider region will be used instead, but this will be an apply-time error for instances if the provider region is not supported with Cloud SQL. If you choose not to provide the region argument for this resource, make sure you understand this."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replica_names` after provisioning.\nThe replicas of the instance."]
    pub fn replica_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replica_names", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `root_password` after provisioning.\nInitial root password. Required for MS SQL Server."]
    pub fn root_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_password", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `root_password_wo` after provisioning.\nInitial root password. Required for MS SQL Server.\n\t\t\t\tNote: This property is write-only and will not be read from the API. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn root_password_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_password_wo", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `root_password_wo_version` after provisioning.\nTriggers update of root_password_wo write-only. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn root_password_wo_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_password_wo_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_ca_cert` after provisioning.\n"]
    pub fn server_ca_cert(&self) -> ListRef<SqlDatabaseInstanceServerCaCertElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.server_ca_cert", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account_email_address` after provisioning.\nThe service account email address assigned to the instance."]
    pub fn service_account_email_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_email_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `clone` after provisioning.\n"]
    pub fn clone(&self) -> ListRef<SqlDatabaseInstanceCloneElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.clone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `point_in_time_restore_context` after provisioning.\n"]
    pub fn point_in_time_restore_context(
        &self,
    ) -> ListRef<SqlDatabaseInstancePointInTimeRestoreContextElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.point_in_time_restore_context", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replica_configuration` after provisioning.\n"]
    pub fn replica_configuration(&self) -> ListRef<SqlDatabaseInstanceReplicaConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replica_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replication_cluster` after provisioning.\n"]
    pub fn replication_cluster(&self) -> ListRef<SqlDatabaseInstanceReplicationClusterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replication_cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restore_backup_context` after provisioning.\n"]
    pub fn restore_backup_context(&self) -> ListRef<SqlDatabaseInstanceRestoreBackupContextElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restore_backup_context", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> ListRef<SqlDatabaseInstanceSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SqlDatabaseInstanceTimeoutsElRef {
        SqlDatabaseInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceDnsNamesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl SqlDatabaseInstanceDnsNamesEl {
    #[doc = "Set the field `connection_type`.\n"]
    pub fn set_connection_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connection_type = Some(v.into());
        self
    }
    #[doc = "Set the field `dns_scope`.\n"]
    pub fn set_dns_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dns_scope = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for SqlDatabaseInstanceDnsNamesEl {
    type O = BlockAssignable<SqlDatabaseInstanceDnsNamesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceDnsNamesEl {}
impl BuildSqlDatabaseInstanceDnsNamesEl {
    pub fn build(self) -> SqlDatabaseInstanceDnsNamesEl {
        SqlDatabaseInstanceDnsNamesEl {
            connection_type: core::default::Default::default(),
            dns_scope: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct SqlDatabaseInstanceDnsNamesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceDnsNamesElRef {
    fn new(shared: StackShared, base: String) -> SqlDatabaseInstanceDnsNamesElRef {
        SqlDatabaseInstanceDnsNamesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceDnsNamesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_type` after provisioning.\n"]
    pub fn connection_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dns_scope` after provisioning.\n"]
    pub fn dns_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_scope", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceIpAddressEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_to_retire: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl SqlDatabaseInstanceIpAddressEl {
    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address = Some(v.into());
        self
    }
    #[doc = "Set the field `time_to_retire`.\n"]
    pub fn set_time_to_retire(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.time_to_retire = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for SqlDatabaseInstanceIpAddressEl {
    type O = BlockAssignable<SqlDatabaseInstanceIpAddressEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceIpAddressEl {}
impl BuildSqlDatabaseInstanceIpAddressEl {
    pub fn build(self) -> SqlDatabaseInstanceIpAddressEl {
        SqlDatabaseInstanceIpAddressEl {
            ip_address: core::default::Default::default(),
            time_to_retire: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct SqlDatabaseInstanceIpAddressElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceIpAddressElRef {
    fn new(shared: StackShared, base: String) -> SqlDatabaseInstanceIpAddressElRef {
        SqlDatabaseInstanceIpAddressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceIpAddressElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
    #[doc = "Get a reference to the value of field `time_to_retire` after provisioning.\n"]
    pub fn time_to_retire(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_to_retire", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceServerCaCertEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cert: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    common_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sha1_fingerprint: Option<PrimField<String>>,
}
impl SqlDatabaseInstanceServerCaCertEl {
    #[doc = "Set the field `cert`.\n"]
    pub fn set_cert(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cert = Some(v.into());
        self
    }
    #[doc = "Set the field `common_name`.\n"]
    pub fn set_common_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.common_name = Some(v.into());
        self
    }
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `expiration_time`.\n"]
    pub fn set_expiration_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expiration_time = Some(v.into());
        self
    }
    #[doc = "Set the field `sha1_fingerprint`.\n"]
    pub fn set_sha1_fingerprint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sha1_fingerprint = Some(v.into());
        self
    }
}
impl ToListMappable for SqlDatabaseInstanceServerCaCertEl {
    type O = BlockAssignable<SqlDatabaseInstanceServerCaCertEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceServerCaCertEl {}
impl BuildSqlDatabaseInstanceServerCaCertEl {
    pub fn build(self) -> SqlDatabaseInstanceServerCaCertEl {
        SqlDatabaseInstanceServerCaCertEl {
            cert: core::default::Default::default(),
            common_name: core::default::Default::default(),
            create_time: core::default::Default::default(),
            expiration_time: core::default::Default::default(),
            sha1_fingerprint: core::default::Default::default(),
        }
    }
}
pub struct SqlDatabaseInstanceServerCaCertElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceServerCaCertElRef {
    fn new(shared: StackShared, base: String) -> SqlDatabaseInstanceServerCaCertElRef {
        SqlDatabaseInstanceServerCaCertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceServerCaCertElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cert` after provisioning.\n"]
    pub fn cert(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cert", self.base))
    }
    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.base))
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `expiration_time` after provisioning.\n"]
    pub fn expiration_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expiration_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sha1_fingerprint` after provisioning.\n"]
    pub fn sha1_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sha1_fingerprint", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceCloneEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allocated_ip_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_names: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    point_in_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_instance_deletion_time: Option<PrimField<String>>,
    source_instance_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_project: Option<PrimField<String>>,
}
impl SqlDatabaseInstanceCloneEl {
    #[doc = "Set the field `allocated_ip_range`.\nThe name of the allocated ip range for the private ip CloudSQL instance. For example: \"google-managed-services-default\". If set, the cloned instance ip will be created in the allocated range. The range name must comply with [RFC 1035](https://tools.ietf.org/html/rfc1035). Specifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])?."]
    pub fn set_allocated_ip_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.allocated_ip_range = Some(v.into());
        self
    }
    #[doc = "Set the field `database_names`.\n(SQL Server only, use with point_in_time) clone only the specified databases from the source instance. Clone all databases if empty."]
    pub fn set_database_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.database_names = Some(v.into());
        self
    }
    #[doc = "Set the field `point_in_time`.\nThe timestamp of the point in time that should be restored."]
    pub fn set_point_in_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.point_in_time = Some(v.into());
        self
    }
    #[doc = "Set the field `preferred_zone`.\n(Point-in-time recovery for PostgreSQL only) Clone to an instance in the specified zone. If no zone is specified, clone to the same zone as the source instance."]
    pub fn set_preferred_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.preferred_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `source_instance_deletion_time`.\nThe timestamp of when the source instance was deleted for a clone from a deleted instance."]
    pub fn set_source_instance_deletion_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_instance_deletion_time = Some(v.into());
        self
    }
    #[doc = "Set the field `source_project`.\nThe project ID of the source project"]
    pub fn set_source_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_project = Some(v.into());
        self
    }
}
impl ToListMappable for SqlDatabaseInstanceCloneEl {
    type O = BlockAssignable<SqlDatabaseInstanceCloneEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceCloneEl {
    #[doc = "The name of the instance from which the point in time should be restored."]
    pub source_instance_name: PrimField<String>,
}
impl BuildSqlDatabaseInstanceCloneEl {
    pub fn build(self) -> SqlDatabaseInstanceCloneEl {
        SqlDatabaseInstanceCloneEl {
            allocated_ip_range: core::default::Default::default(),
            database_names: core::default::Default::default(),
            point_in_time: core::default::Default::default(),
            preferred_zone: core::default::Default::default(),
            source_instance_deletion_time: core::default::Default::default(),
            source_instance_name: self.source_instance_name,
            source_project: core::default::Default::default(),
        }
    }
}
pub struct SqlDatabaseInstanceCloneElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceCloneElRef {
    fn new(shared: StackShared, base: String) -> SqlDatabaseInstanceCloneElRef {
        SqlDatabaseInstanceCloneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceCloneElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allocated_ip_range` after provisioning.\nThe name of the allocated ip range for the private ip CloudSQL instance. For example: \"google-managed-services-default\". If set, the cloned instance ip will be created in the allocated range. The range name must comply with [RFC 1035](https://tools.ietf.org/html/rfc1035). Specifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])?."]
    pub fn allocated_ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocated_ip_range", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `database_names` after provisioning.\n(SQL Server only, use with point_in_time) clone only the specified databases from the source instance. Clone all databases if empty."]
    pub fn database_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.database_names", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `point_in_time` after provisioning.\nThe timestamp of the point in time that should be restored."]
    pub fn point_in_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.point_in_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `preferred_zone` after provisioning.\n(Point-in-time recovery for PostgreSQL only) Clone to an instance in the specified zone. If no zone is specified, clone to the same zone as the source instance."]
    pub fn preferred_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preferred_zone", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_instance_deletion_time` after provisioning.\nThe timestamp of when the source instance was deleted for a clone from a deleted instance."]
    pub fn source_instance_deletion_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_instance_deletion_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_instance_name` after provisioning.\nThe name of the instance from which the point in time should be restored."]
    pub fn source_instance_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_instance_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_project` after provisioning.\nThe project ID of the source project"]
    pub fn source_project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_project", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SqlDatabaseInstancePointInTimeRestoreContextEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allocated_ip_range: Option<PrimField<String>>,
    datasource: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    point_in_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_instance: Option<PrimField<String>>,
}
impl SqlDatabaseInstancePointInTimeRestoreContextEl {
    #[doc = "Set the field `allocated_ip_range`.\nThe name of the allocated IP range for the internal IP Cloud SQL instance. For example: \"google-managed-services-default\". If you set this, then Cloud SQL creates the IP address for the cloned instance in the allocated range. This range must comply with [RFC 1035](https://tools.ietf.org/html/rfc1035) standards. Specifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])?."]
    pub fn set_allocated_ip_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.allocated_ip_range = Some(v.into());
        self
    }
    #[doc = "Set the field `point_in_time`.\nThe date and time to which you want to restore the instance."]
    pub fn set_point_in_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.point_in_time = Some(v.into());
        self
    }
    #[doc = "Set the field `preferred_zone`.\nPoint-in-time recovery of an instance to the specified zone. If no zone is specified, then clone to the same primary zone as the source instance."]
    pub fn set_preferred_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.preferred_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nThe region of the target instance to restore to."]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
    #[doc = "Set the field `target_instance`.\nThe name of the target instance to restore to."]
    pub fn set_target_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_instance = Some(v.into());
        self
    }
}
impl ToListMappable for SqlDatabaseInstancePointInTimeRestoreContextEl {
    type O = BlockAssignable<SqlDatabaseInstancePointInTimeRestoreContextEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstancePointInTimeRestoreContextEl {
    #[doc = "The Google Cloud Backup and Disaster Recovery Datasource URI. For example: \"projects/my-project/locations/us-central1/datasources/my-datasource\"."]
    pub datasource: PrimField<String>,
}
impl BuildSqlDatabaseInstancePointInTimeRestoreContextEl {
    pub fn build(self) -> SqlDatabaseInstancePointInTimeRestoreContextEl {
        SqlDatabaseInstancePointInTimeRestoreContextEl {
            allocated_ip_range: core::default::Default::default(),
            datasource: self.datasource,
            point_in_time: core::default::Default::default(),
            preferred_zone: core::default::Default::default(),
            region: core::default::Default::default(),
            target_instance: core::default::Default::default(),
        }
    }
}
pub struct SqlDatabaseInstancePointInTimeRestoreContextElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstancePointInTimeRestoreContextElRef {
    fn new(shared: StackShared, base: String) -> SqlDatabaseInstancePointInTimeRestoreContextElRef {
        SqlDatabaseInstancePointInTimeRestoreContextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstancePointInTimeRestoreContextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allocated_ip_range` after provisioning.\nThe name of the allocated IP range for the internal IP Cloud SQL instance. For example: \"google-managed-services-default\". If you set this, then Cloud SQL creates the IP address for the cloned instance in the allocated range. This range must comply with [RFC 1035](https://tools.ietf.org/html/rfc1035) standards. Specifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])?."]
    pub fn allocated_ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocated_ip_range", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `datasource` after provisioning.\nThe Google Cloud Backup and Disaster Recovery Datasource URI. For example: \"projects/my-project/locations/us-central1/datasources/my-datasource\"."]
    pub fn datasource(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.datasource", self.base))
    }
    #[doc = "Get a reference to the value of field `point_in_time` after provisioning.\nThe date and time to which you want to restore the instance."]
    pub fn point_in_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.point_in_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `preferred_zone` after provisioning.\nPoint-in-time recovery of an instance to the specified zone. If no zone is specified, then clone to the same primary zone as the source instance."]
    pub fn preferred_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preferred_zone", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the target instance to restore to."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
    #[doc = "Get a reference to the value of field `target_instance` after provisioning.\nThe name of the target instance to restore to."]
    pub fn target_instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_instance", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceReplicaConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cascadable_replica: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connect_retry_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dump_file_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failover_target: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    master_heartbeat_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_cipher: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verify_server_certificate: Option<PrimField<bool>>,
}
impl SqlDatabaseInstanceReplicaConfigurationEl {
    #[doc = "Set the field `ca_certificate`.\nPEM representation of the trusted CA's x509 certificate."]
    pub fn set_ca_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ca_certificate = Some(v.into());
        self
    }
    #[doc = "Set the field `cascadable_replica`.\nSpecifies if a SQL Server replica is a cascadable replica. A cascadable replica is a SQL Server cross region replica that supports replica(s) under it."]
    pub fn set_cascadable_replica(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.cascadable_replica = Some(v.into());
        self
    }
    #[doc = "Set the field `client_certificate`.\nPEM representation of the replica's x509 certificate."]
    pub fn set_client_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_certificate = Some(v.into());
        self
    }
    #[doc = "Set the field `client_key`.\nPEM representation of the replica's private key. The corresponding public key in encoded in the client_certificate."]
    pub fn set_client_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_key = Some(v.into());
        self
    }
    #[doc = "Set the field `connect_retry_interval`.\nThe number of seconds between connect retries. MySQL's default is 60 seconds."]
    pub fn set_connect_retry_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.connect_retry_interval = Some(v.into());
        self
    }
    #[doc = "Set the field `dump_file_path`.\nPath to a SQL file in Google Cloud Storage from which replica instances are created. Format is gs://bucket/filename."]
    pub fn set_dump_file_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dump_file_path = Some(v.into());
        self
    }
    #[doc = "Set the field `failover_target`.\nSpecifies if the replica is the failover target. If the field is set to true the replica will be designated as a failover replica. If the master instance fails, the replica instance will be promoted as the new master instance. Not supported for Postgres"]
    pub fn set_failover_target(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.failover_target = Some(v.into());
        self
    }
    #[doc = "Set the field `master_heartbeat_period`.\nTime in ms between replication heartbeats."]
    pub fn set_master_heartbeat_period(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.master_heartbeat_period = Some(v.into());
        self
    }
    #[doc = "Set the field `password`.\nPassword for the replication connection."]
    pub fn set_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password = Some(v.into());
        self
    }
    #[doc = "Set the field `ssl_cipher`.\nPermissible ciphers for use in SSL encryption."]
    pub fn set_ssl_cipher(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_cipher = Some(v.into());
        self
    }
    #[doc = "Set the field `username`.\nUsername for replication connection."]
    pub fn set_username(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.username = Some(v.into());
        self
    }
    #[doc = "Set the field `verify_server_certificate`.\nTrue if the master's common name value is checked during the SSL handshake."]
    pub fn set_verify_server_certificate(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.verify_server_certificate = Some(v.into());
        self
    }
}
impl ToListMappable for SqlDatabaseInstanceReplicaConfigurationEl {
    type O = BlockAssignable<SqlDatabaseInstanceReplicaConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceReplicaConfigurationEl {}
impl BuildSqlDatabaseInstanceReplicaConfigurationEl {
    pub fn build(self) -> SqlDatabaseInstanceReplicaConfigurationEl {
        SqlDatabaseInstanceReplicaConfigurationEl {
            ca_certificate: core::default::Default::default(),
            cascadable_replica: core::default::Default::default(),
            client_certificate: core::default::Default::default(),
            client_key: core::default::Default::default(),
            connect_retry_interval: core::default::Default::default(),
            dump_file_path: core::default::Default::default(),
            failover_target: core::default::Default::default(),
            master_heartbeat_period: core::default::Default::default(),
            password: core::default::Default::default(),
            ssl_cipher: core::default::Default::default(),
            username: core::default::Default::default(),
            verify_server_certificate: core::default::Default::default(),
        }
    }
}
pub struct SqlDatabaseInstanceReplicaConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceReplicaConfigurationElRef {
    fn new(shared: StackShared, base: String) -> SqlDatabaseInstanceReplicaConfigurationElRef {
        SqlDatabaseInstanceReplicaConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceReplicaConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ca_certificate` after provisioning.\nPEM representation of the trusted CA's x509 certificate."]
    pub fn ca_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ca_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cascadable_replica` after provisioning.\nSpecifies if a SQL Server replica is a cascadable replica. A cascadable replica is a SQL Server cross region replica that supports replica(s) under it."]
    pub fn cascadable_replica(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cascadable_replica", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_certificate` after provisioning.\nPEM representation of the replica's x509 certificate."]
    pub fn client_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_key` after provisioning.\nPEM representation of the replica's private key. The corresponding public key in encoded in the client_certificate."]
    pub fn client_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_key", self.base))
    }
    #[doc = "Get a reference to the value of field `connect_retry_interval` after provisioning.\nThe number of seconds between connect retries. MySQL's default is 60 seconds."]
    pub fn connect_retry_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connect_retry_interval", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dump_file_path` after provisioning.\nPath to a SQL file in Google Cloud Storage from which replica instances are created. Format is gs://bucket/filename."]
    pub fn dump_file_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dump_file_path", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `failover_target` after provisioning.\nSpecifies if the replica is the failover target. If the field is set to true the replica will be designated as a failover replica. If the master instance fails, the replica instance will be promoted as the new master instance. Not supported for Postgres"]
    pub fn failover_target(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failover_target", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `master_heartbeat_period` after provisioning.\nTime in ms between replication heartbeats."]
    pub fn master_heartbeat_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_heartbeat_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nPassword for the replication connection."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `ssl_cipher` after provisioning.\nPermissible ciphers for use in SSL encryption."]
    pub fn ssl_cipher(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl_cipher", self.base))
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername for replication connection."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
    #[doc = "Get a reference to the value of field `verify_server_certificate` after provisioning.\nTrue if the master's common name value is checked during the SSL handshake."]
    pub fn verify_server_certificate(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.verify_server_certificate", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceReplicationClusterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    failover_dr_replica_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psa_write_endpoint: Option<PrimField<String>>,
}
impl SqlDatabaseInstanceReplicationClusterEl {
    #[doc = "Set the field `failover_dr_replica_name`.\nIf the instance is a primary instance, then this field identifies the disaster recovery (DR) replica. The standard format of this field is \"your-project:your-instance\". You can also set this field to \"your-instance\", but cloud SQL backend will convert it to the aforementioned standard format."]
    pub fn set_failover_dr_replica_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.failover_dr_replica_name = Some(v.into());
        self
    }
    #[doc = "Set the field `psa_write_endpoint`.\nIf set, this field indicates this instance has a private service access (PSA) DNS endpoint that is pointing to the primary instance of the cluster. If this instance is the primary, then the DNS endpoint points to this instance. After a switchover or replica failover operation, this DNS endpoint points to the promoted instance. This is a read-only field, returned to the user as information. This field can exist even if a standalone instance doesn't have a DR replica yet or the DR replica is deleted."]
    pub fn set_psa_write_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.psa_write_endpoint = Some(v.into());
        self
    }
}
impl ToListMappable for SqlDatabaseInstanceReplicationClusterEl {
    type O = BlockAssignable<SqlDatabaseInstanceReplicationClusterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceReplicationClusterEl {}
impl BuildSqlDatabaseInstanceReplicationClusterEl {
    pub fn build(self) -> SqlDatabaseInstanceReplicationClusterEl {
        SqlDatabaseInstanceReplicationClusterEl {
            failover_dr_replica_name: core::default::Default::default(),
            psa_write_endpoint: core::default::Default::default(),
        }
    }
}
pub struct SqlDatabaseInstanceReplicationClusterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceReplicationClusterElRef {
    fn new(shared: StackShared, base: String) -> SqlDatabaseInstanceReplicationClusterElRef {
        SqlDatabaseInstanceReplicationClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceReplicationClusterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dr_replica` after provisioning.\nRead-only field that indicates whether the replica is a DR replica."]
    pub fn dr_replica(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.dr_replica", self.base))
    }
    #[doc = "Get a reference to the value of field `failover_dr_replica_name` after provisioning.\nIf the instance is a primary instance, then this field identifies the disaster recovery (DR) replica. The standard format of this field is \"your-project:your-instance\". You can also set this field to \"your-instance\", but cloud SQL backend will convert it to the aforementioned standard format."]
    pub fn failover_dr_replica_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failover_dr_replica_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `psa_write_endpoint` after provisioning.\nIf set, this field indicates this instance has a private service access (PSA) DNS endpoint that is pointing to the primary instance of the cluster. If this instance is the primary, then the DNS endpoint points to this instance. After a switchover or replica failover operation, this DNS endpoint points to the promoted instance. This is a read-only field, returned to the user as information. This field can exist even if a standalone instance doesn't have a DR replica yet or the DR replica is deleted."]
    pub fn psa_write_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.psa_write_endpoint", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceRestoreBackupContextEl {
    backup_run_id: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
impl SqlDatabaseInstanceRestoreBackupContextEl {
    #[doc = "Set the field `instance_id`.\nThe ID of the instance that the backup was taken from."]
    pub fn set_instance_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe full project ID of the source instance."]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }
}
impl ToListMappable for SqlDatabaseInstanceRestoreBackupContextEl {
    type O = BlockAssignable<SqlDatabaseInstanceRestoreBackupContextEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceRestoreBackupContextEl {
    #[doc = "The ID of the backup run to restore from."]
    pub backup_run_id: PrimField<f64>,
}
impl BuildSqlDatabaseInstanceRestoreBackupContextEl {
    pub fn build(self) -> SqlDatabaseInstanceRestoreBackupContextEl {
        SqlDatabaseInstanceRestoreBackupContextEl {
            backup_run_id: self.backup_run_id,
            instance_id: core::default::Default::default(),
            project: core::default::Default::default(),
        }
    }
}
pub struct SqlDatabaseInstanceRestoreBackupContextElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceRestoreBackupContextElRef {
    fn new(shared: StackShared, base: String) -> SqlDatabaseInstanceRestoreBackupContextElRef {
        SqlDatabaseInstanceRestoreBackupContextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceRestoreBackupContextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup_run_id` after provisioning.\nThe ID of the backup run to restore from."]
    pub fn backup_run_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_run_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\nThe ID of the instance that the backup was taken from."]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.base))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe full project ID of the source instance."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElActiveDirectoryConfigEl {
    domain: PrimField<String>,
}
impl SqlDatabaseInstanceSettingsElActiveDirectoryConfigEl {}
impl ToListMappable for SqlDatabaseInstanceSettingsElActiveDirectoryConfigEl {
    type O = BlockAssignable<SqlDatabaseInstanceSettingsElActiveDirectoryConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceSettingsElActiveDirectoryConfigEl {
    #[doc = "Domain name of the Active Directory for SQL Server (e.g., mydomain.com)."]
    pub domain: PrimField<String>,
}
impl BuildSqlDatabaseInstanceSettingsElActiveDirectoryConfigEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElActiveDirectoryConfigEl {
        SqlDatabaseInstanceSettingsElActiveDirectoryConfigEl {
            domain: self.domain,
        }
    }
}
pub struct SqlDatabaseInstanceSettingsElActiveDirectoryConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceSettingsElActiveDirectoryConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SqlDatabaseInstanceSettingsElActiveDirectoryConfigElRef {
        SqlDatabaseInstanceSettingsElActiveDirectoryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceSettingsElActiveDirectoryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\nDomain name of the Active Directory for SQL Server (e.g., mydomain.com)."]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    threads_per_core: Option<PrimField<f64>>,
}
impl SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl {
    #[doc = "Set the field `threads_per_core`.\nThe number of threads per physical core. Can be 1 or 2."]
    pub fn set_threads_per_core(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.threads_per_core = Some(v.into());
        self
    }
}
impl ToListMappable for SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl {
    type O = BlockAssignable<SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl {}
impl BuildSqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl {
        SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl {
            threads_per_core: core::default::Default::default(),
        }
    }
}
pub struct SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesElRef {
        SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `threads_per_core` after provisioning.\nThe number of threads per physical core. Can be 1 or 2."]
    pub fn threads_per_core(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.threads_per_core", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl {
    retained_backups: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_unit: Option<PrimField<String>>,
}
impl SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl {
    #[doc = "Set the field `retention_unit`.\nThe unit that 'retainedBackups' represents. Defaults to COUNT"]
    pub fn set_retention_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.retention_unit = Some(v.into());
        self
    }
}
impl ToListMappable
    for SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl
{
    type O = BlockAssignable<
        SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl {
    #[doc = "Number of backups to retain."]
    pub retained_backups: PrimField<f64>,
}
impl BuildSqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl {
    pub fn build(
        self,
    ) -> SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl {
        SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl {
            retained_backups: self.retained_backups,
            retention_unit: core::default::Default::default(),
        }
    }
}
pub struct SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsElRef {
        SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `retained_backups` after provisioning.\nNumber of backups to retain."]
    pub fn retained_backups(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retained_backups", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `retention_unit` after provisioning.\nThe unit that 'retainedBackups' represents. Defaults to COUNT"]
    pub fn retention_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention_unit", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SqlDatabaseInstanceSettingsElBackupConfigurationElDynamic {
    backup_retention_settings: Option<
        DynamicBlock<SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl>,
    >,
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElBackupConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    binary_log_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    point_in_time_recovery_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transaction_log_retention_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_retention_settings:
        Option<Vec<SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl>>,
    dynamic: SqlDatabaseInstanceSettingsElBackupConfigurationElDynamic,
}
impl SqlDatabaseInstanceSettingsElBackupConfigurationEl {
    #[doc = "Set the field `binary_log_enabled`.\nTrue if binary logging is enabled. If settings.backup_configuration.enabled is false, this must be as well. Can only be used with MySQL."]
    pub fn set_binary_log_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.binary_log_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\nTrue if backup configuration is enabled."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nLocation of the backup configuration."]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `point_in_time_recovery_enabled`.\nTrue if Point-in-time recovery is enabled."]
    pub fn set_point_in_time_recovery_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.point_in_time_recovery_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\nHH:MM format time indicating when backup configuration starts."]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
    #[doc = "Set the field `transaction_log_retention_days`.\nThe number of days of transaction logs we retain for point in time restore, from 1-7. (For PostgreSQL Enterprise Plus instances, from 1 to 35.)"]
    pub fn set_transaction_log_retention_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.transaction_log_retention_days = Some(v.into());
        self
    }
    #[doc = "Set the field `backup_retention_settings`.\n"]
    pub fn set_backup_retention_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.backup_retention_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.backup_retention_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SqlDatabaseInstanceSettingsElBackupConfigurationEl {
    type O = BlockAssignable<SqlDatabaseInstanceSettingsElBackupConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceSettingsElBackupConfigurationEl {}
impl BuildSqlDatabaseInstanceSettingsElBackupConfigurationEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElBackupConfigurationEl {
        SqlDatabaseInstanceSettingsElBackupConfigurationEl {
            binary_log_enabled: core::default::Default::default(),
            enabled: core::default::Default::default(),
            location: core::default::Default::default(),
            point_in_time_recovery_enabled: core::default::Default::default(),
            start_time: core::default::Default::default(),
            transaction_log_retention_days: core::default::Default::default(),
            backup_retention_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SqlDatabaseInstanceSettingsElBackupConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceSettingsElBackupConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SqlDatabaseInstanceSettingsElBackupConfigurationElRef {
        SqlDatabaseInstanceSettingsElBackupConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceSettingsElBackupConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup_tier` after provisioning.\nBackup tier that manages the backups for the instance."]
    pub fn backup_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup_tier", self.base))
    }
    #[doc = "Get a reference to the value of field `binary_log_enabled` after provisioning.\nTrue if binary logging is enabled. If settings.backup_configuration.enabled is false, this must be as well. Can only be used with MySQL."]
    pub fn binary_log_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.binary_log_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nTrue if backup configuration is enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation of the backup configuration."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `point_in_time_recovery_enabled` after provisioning.\nTrue if Point-in-time recovery is enabled."]
    pub fn point_in_time_recovery_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.point_in_time_recovery_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\nHH:MM format time indicating when backup configuration starts."]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
    #[doc = "Get a reference to the value of field `transaction_log_retention_days` after provisioning.\nThe number of days of transaction logs we retain for point in time restore, from 1-7. (For PostgreSQL Enterprise Plus instances, from 1 to 35.)"]
    pub fn transaction_log_retention_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transaction_log_retention_days", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `backup_retention_settings` after provisioning.\n"]
    pub fn backup_retention_settings(
        &self,
    ) -> ListRef<SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backup_retention_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElConnectionPoolConfigElFlagsEl {
    name: PrimField<String>,
    value: PrimField<String>,
}
impl SqlDatabaseInstanceSettingsElConnectionPoolConfigElFlagsEl {}
impl ToListMappable for SqlDatabaseInstanceSettingsElConnectionPoolConfigElFlagsEl {
    type O = BlockAssignable<SqlDatabaseInstanceSettingsElConnectionPoolConfigElFlagsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceSettingsElConnectionPoolConfigElFlagsEl {
    #[doc = "Name of the flag."]
    pub name: PrimField<String>,
    #[doc = "Value of the flag."]
    pub value: PrimField<String>,
}
impl BuildSqlDatabaseInstanceSettingsElConnectionPoolConfigElFlagsEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElConnectionPoolConfigElFlagsEl {
        SqlDatabaseInstanceSettingsElConnectionPoolConfigElFlagsEl {
            name: self.name,
            value: self.value,
        }
    }
}
pub struct SqlDatabaseInstanceSettingsElConnectionPoolConfigElFlagsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceSettingsElConnectionPoolConfigElFlagsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SqlDatabaseInstanceSettingsElConnectionPoolConfigElFlagsElRef {
        SqlDatabaseInstanceSettingsElConnectionPoolConfigElFlagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceSettingsElConnectionPoolConfigElFlagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the flag."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue of the flag."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct SqlDatabaseInstanceSettingsElConnectionPoolConfigElDynamic {
    flags: Option<DynamicBlock<SqlDatabaseInstanceSettingsElConnectionPoolConfigElFlagsEl>>,
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElConnectionPoolConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_pooling_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<Vec<SqlDatabaseInstanceSettingsElConnectionPoolConfigElFlagsEl>>,
    dynamic: SqlDatabaseInstanceSettingsElConnectionPoolConfigElDynamic,
}
impl SqlDatabaseInstanceSettingsElConnectionPoolConfigEl {
    #[doc = "Set the field `connection_pooling_enabled`.\nWhether Managed Connection Pool is enabled for this instance."]
    pub fn set_connection_pooling_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.connection_pooling_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `flags`.\n"]
    pub fn set_flags(
        mut self,
        v: impl Into<BlockAssignable<SqlDatabaseInstanceSettingsElConnectionPoolConfigElFlagsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.flags = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.flags = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SqlDatabaseInstanceSettingsElConnectionPoolConfigEl {
    type O = BlockAssignable<SqlDatabaseInstanceSettingsElConnectionPoolConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceSettingsElConnectionPoolConfigEl {}
impl BuildSqlDatabaseInstanceSettingsElConnectionPoolConfigEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElConnectionPoolConfigEl {
        SqlDatabaseInstanceSettingsElConnectionPoolConfigEl {
            connection_pooling_enabled: core::default::Default::default(),
            flags: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SqlDatabaseInstanceSettingsElConnectionPoolConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceSettingsElConnectionPoolConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SqlDatabaseInstanceSettingsElConnectionPoolConfigElRef {
        SqlDatabaseInstanceSettingsElConnectionPoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceSettingsElConnectionPoolConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_pooling_enabled` after provisioning.\nWhether Managed Connection Pool is enabled for this instance."]
    pub fn connection_pooling_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_pooling_enabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElDataCacheConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_cache_enabled: Option<PrimField<bool>>,
}
impl SqlDatabaseInstanceSettingsElDataCacheConfigEl {
    #[doc = "Set the field `data_cache_enabled`.\nWhether data cache is enabled for the instance."]
    pub fn set_data_cache_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.data_cache_enabled = Some(v.into());
        self
    }
}
impl ToListMappable for SqlDatabaseInstanceSettingsElDataCacheConfigEl {
    type O = BlockAssignable<SqlDatabaseInstanceSettingsElDataCacheConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceSettingsElDataCacheConfigEl {}
impl BuildSqlDatabaseInstanceSettingsElDataCacheConfigEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElDataCacheConfigEl {
        SqlDatabaseInstanceSettingsElDataCacheConfigEl {
            data_cache_enabled: core::default::Default::default(),
        }
    }
}
pub struct SqlDatabaseInstanceSettingsElDataCacheConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceSettingsElDataCacheConfigElRef {
    fn new(shared: StackShared, base: String) -> SqlDatabaseInstanceSettingsElDataCacheConfigElRef {
        SqlDatabaseInstanceSettingsElDataCacheConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceSettingsElDataCacheConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_cache_enabled` after provisioning.\nWhether data cache is enabled for the instance."]
    pub fn data_cache_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_cache_enabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElDatabaseFlagsEl {
    name: PrimField<String>,
    value: PrimField<String>,
}
impl SqlDatabaseInstanceSettingsElDatabaseFlagsEl {}
impl ToListMappable for SqlDatabaseInstanceSettingsElDatabaseFlagsEl {
    type O = BlockAssignable<SqlDatabaseInstanceSettingsElDatabaseFlagsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceSettingsElDatabaseFlagsEl {
    #[doc = "Name of the flag."]
    pub name: PrimField<String>,
    #[doc = "Value of the flag."]
    pub value: PrimField<String>,
}
impl BuildSqlDatabaseInstanceSettingsElDatabaseFlagsEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElDatabaseFlagsEl {
        SqlDatabaseInstanceSettingsElDatabaseFlagsEl {
            name: self.name,
            value: self.value,
        }
    }
}
pub struct SqlDatabaseInstanceSettingsElDatabaseFlagsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceSettingsElDatabaseFlagsElRef {
    fn new(shared: StackShared, base: String) -> SqlDatabaseInstanceSettingsElDatabaseFlagsElRef {
        SqlDatabaseInstanceSettingsElDatabaseFlagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceSettingsElDatabaseFlagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the flag."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue of the flag."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl {
    end_date: PrimField<String>,
    start_date: PrimField<String>,
    time: PrimField<String>,
}
impl SqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl {}
impl ToListMappable for SqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl {
    type O = BlockAssignable<SqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl {
    #[doc = "End date before which maintenance will not take place. The date is in format yyyy-mm-dd i.e., 2020-11-01, or mm-dd, i.e., 11-01"]
    pub end_date: PrimField<String>,
    #[doc = "Start date after which maintenance will not take place. The date is in format yyyy-mm-dd i.e., 2020-11-01, or mm-dd, i.e., 11-01"]
    pub start_date: PrimField<String>,
    #[doc = "Time in UTC when the \"deny maintenance period\" starts on start_date and ends on end_date. The time is in format: HH:mm:SS, i.e., 00:00:00"]
    pub time: PrimField<String>,
}
impl BuildSqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl {
        SqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl {
            end_date: self.end_date,
            start_date: self.start_date,
            time: self.time,
        }
    }
}
pub struct SqlDatabaseInstanceSettingsElDenyMaintenancePeriodElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceSettingsElDenyMaintenancePeriodElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SqlDatabaseInstanceSettingsElDenyMaintenancePeriodElRef {
        SqlDatabaseInstanceSettingsElDenyMaintenancePeriodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceSettingsElDenyMaintenancePeriodElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_date` after provisioning.\nEnd date before which maintenance will not take place. The date is in format yyyy-mm-dd i.e., 2020-11-01, or mm-dd, i.e., 11-01"]
    pub fn end_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_date", self.base))
    }
    #[doc = "Get a reference to the value of field `start_date` after provisioning.\nStart date after which maintenance will not take place. The date is in format yyyy-mm-dd i.e., 2020-11-01, or mm-dd, i.e., 11-01"]
    pub fn start_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_date", self.base))
    }
    #[doc = "Get a reference to the value of field `time` after provisioning.\nTime in UTC when the \"deny maintenance period\" starts on start_date and ends on end_date. The time is in format: HH:mm:SS, i.e., 00:00:00"]
    pub fn time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time", self.base))
    }
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElEntraidConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    application_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tenant_id: Option<PrimField<String>>,
}
impl SqlDatabaseInstanceSettingsElEntraidConfigEl {
    #[doc = "Set the field `application_id`.\nThe application ID for the Entra ID configuration."]
    pub fn set_application_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.application_id = Some(v.into());
        self
    }
    #[doc = "Set the field `tenant_id`.\nThe tenant ID for the Entra ID configuration."]
    pub fn set_tenant_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tenant_id = Some(v.into());
        self
    }
}
impl ToListMappable for SqlDatabaseInstanceSettingsElEntraidConfigEl {
    type O = BlockAssignable<SqlDatabaseInstanceSettingsElEntraidConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceSettingsElEntraidConfigEl {}
impl BuildSqlDatabaseInstanceSettingsElEntraidConfigEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElEntraidConfigEl {
        SqlDatabaseInstanceSettingsElEntraidConfigEl {
            application_id: core::default::Default::default(),
            tenant_id: core::default::Default::default(),
        }
    }
}
pub struct SqlDatabaseInstanceSettingsElEntraidConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceSettingsElEntraidConfigElRef {
    fn new(shared: StackShared, base: String) -> SqlDatabaseInstanceSettingsElEntraidConfigElRef {
        SqlDatabaseInstanceSettingsElEntraidConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceSettingsElEntraidConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `application_id` after provisioning.\nThe application ID for the Entra ID configuration."]
    pub fn application_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tenant_id` after provisioning.\nThe tenant ID for the Entra ID configuration."]
    pub fn tenant_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElFinalBackupConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_days: Option<PrimField<f64>>,
}
impl SqlDatabaseInstanceSettingsElFinalBackupConfigEl {
    #[doc = "Set the field `enabled`.\nWhen this parameter is set to true, the final backup is enabled for the instance"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `retention_days`.\nThe number of days to retain the final backup after the instance deletion. The valid range is between 1 and 365. For instances managed by BackupDR, the valid range is between 1 day and 99 years. The final backup will be purged at (time_of_instance_deletion + retention_days)."]
    pub fn set_retention_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.retention_days = Some(v.into());
        self
    }
}
impl ToListMappable for SqlDatabaseInstanceSettingsElFinalBackupConfigEl {
    type O = BlockAssignable<SqlDatabaseInstanceSettingsElFinalBackupConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceSettingsElFinalBackupConfigEl {}
impl BuildSqlDatabaseInstanceSettingsElFinalBackupConfigEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElFinalBackupConfigEl {
        SqlDatabaseInstanceSettingsElFinalBackupConfigEl {
            enabled: core::default::Default::default(),
            retention_days: core::default::Default::default(),
        }
    }
}
pub struct SqlDatabaseInstanceSettingsElFinalBackupConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceSettingsElFinalBackupConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SqlDatabaseInstanceSettingsElFinalBackupConfigElRef {
        SqlDatabaseInstanceSettingsElFinalBackupConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceSettingsElFinalBackupConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhen this parameter is set to true, the final backup is enabled for the instance"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `retention_days` after provisioning.\nThe number of days to retain the final backup after the instance deletion. The valid range is between 1 and 365. For instances managed by BackupDR, the valid range is between 1 day and 99 years. The final backup will be purged at (time_of_instance_deletion + retention_days)."]
    pub fn retention_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention_days", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElInsightsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enhanced_query_insights_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_insights_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_plans_per_minute: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_string_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_application_tags: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_client_address: Option<PrimField<bool>>,
}
impl SqlDatabaseInstanceSettingsElInsightsConfigEl {
    #[doc = "Set the field `enhanced_query_insights_enabled`.\nTrue if Enhanced Query Insights feature is enabled."]
    pub fn set_enhanced_query_insights_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enhanced_query_insights_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `query_insights_enabled`.\nTrue if Query Insights feature is enabled."]
    pub fn set_query_insights_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.query_insights_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `query_plans_per_minute`.\nNumber of query execution plans captured by Insights per minute for all queries combined. Between 0 and 20. Default to 5. For Enterprise Plus instances, from 0 to 200."]
    pub fn set_query_plans_per_minute(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.query_plans_per_minute = Some(v.into());
        self
    }
    #[doc = "Set the field `query_string_length`.\nMaximum query length stored in bytes. Between 256 and 4500. Default to 1024. For Enterprise Plus instances, from 1024 to 100000."]
    pub fn set_query_string_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.query_string_length = Some(v.into());
        self
    }
    #[doc = "Set the field `record_application_tags`.\nTrue if Query Insights will record application tags from query when enabled."]
    pub fn set_record_application_tags(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.record_application_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `record_client_address`.\nTrue if Query Insights will record client address when enabled."]
    pub fn set_record_client_address(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.record_client_address = Some(v.into());
        self
    }
}
impl ToListMappable for SqlDatabaseInstanceSettingsElInsightsConfigEl {
    type O = BlockAssignable<SqlDatabaseInstanceSettingsElInsightsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceSettingsElInsightsConfigEl {}
impl BuildSqlDatabaseInstanceSettingsElInsightsConfigEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElInsightsConfigEl {
        SqlDatabaseInstanceSettingsElInsightsConfigEl {
            enhanced_query_insights_enabled: core::default::Default::default(),
            query_insights_enabled: core::default::Default::default(),
            query_plans_per_minute: core::default::Default::default(),
            query_string_length: core::default::Default::default(),
            record_application_tags: core::default::Default::default(),
            record_client_address: core::default::Default::default(),
        }
    }
}
pub struct SqlDatabaseInstanceSettingsElInsightsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceSettingsElInsightsConfigElRef {
    fn new(shared: StackShared, base: String) -> SqlDatabaseInstanceSettingsElInsightsConfigElRef {
        SqlDatabaseInstanceSettingsElInsightsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceSettingsElInsightsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enhanced_query_insights_enabled` after provisioning.\nTrue if Enhanced Query Insights feature is enabled."]
    pub fn enhanced_query_insights_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enhanced_query_insights_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `query_insights_enabled` after provisioning.\nTrue if Query Insights feature is enabled."]
    pub fn query_insights_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_insights_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `query_plans_per_minute` after provisioning.\nNumber of query execution plans captured by Insights per minute for all queries combined. Between 0 and 20. Default to 5. For Enterprise Plus instances, from 0 to 200."]
    pub fn query_plans_per_minute(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_plans_per_minute", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `query_string_length` after provisioning.\nMaximum query length stored in bytes. Between 256 and 4500. Default to 1024. For Enterprise Plus instances, from 1024 to 100000."]
    pub fn query_string_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_string_length", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `record_application_tags` after provisioning.\nTrue if Query Insights will record application tags from query when enabled."]
    pub fn record_application_tags(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.record_application_tags", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `record_client_address` after provisioning.\nTrue if Query Insights will record client address when enabled."]
    pub fn record_client_address(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.record_client_address", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    value: PrimField<String>,
}
impl SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl {
    #[doc = "Set the field `expiration_time`.\n"]
    pub fn set_expiration_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expiration_time = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl {
    type O = BlockAssignable<SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl {
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl {
        SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl {
            expiration_time: core::default::Default::default(),
            name: core::default::Default::default(),
            value: self.value,
        }
    }
}
pub struct SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksElRef {
        SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expiration_time` after provisioning.\n"]
    pub fn expiration_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expiration_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElPscAutoConnectionsEl {
    consumer_network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_service_project_id: Option<PrimField<String>>,
}
impl SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElPscAutoConnectionsEl {
    #[doc = "Set the field `consumer_service_project_id`.\nThe project ID of consumer service project of this consumer endpoint."]
    pub fn set_consumer_service_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.consumer_service_project_id = Some(v.into());
        self
    }
}
impl ToListMappable
    for SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElPscAutoConnectionsEl
{
    type O = BlockAssignable<
        SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElPscAutoConnectionsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElPscAutoConnectionsEl {
    #[doc = "The consumer network of this consumer endpoint. This must be a resource path that includes both the host project and the network name. The consumer host project of this network might be different from the consumer service project."]
    pub consumer_network: PrimField<String>,
}
impl BuildSqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElPscAutoConnectionsEl {
    pub fn build(
        self,
    ) -> SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElPscAutoConnectionsEl {
        SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElPscAutoConnectionsEl {
            consumer_network: self.consumer_network,
            consumer_service_project_id: core::default::Default::default(),
        }
    }
}
pub struct SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElPscAutoConnectionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElPscAutoConnectionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElPscAutoConnectionsElRef {
        SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElPscAutoConnectionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElPscAutoConnectionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `consumer_network` after provisioning.\nThe consumer network of this consumer endpoint. This must be a resource path that includes both the host project and the network name. The consumer host project of this network might be different from the consumer service project."]
    pub fn consumer_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consumer_network", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `consumer_network_status` after provisioning.\nThe connection policy status of the consumer network."]
    pub fn consumer_network_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consumer_network_status", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `consumer_service_project_id` after provisioning.\nThe project ID of consumer service project of this consumer endpoint."]
    pub fn consumer_service_project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consumer_service_project_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\nThe IP address of the consumer endpoint."]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe connection status of the consumer endpoint."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
#[derive(Serialize, Default)]
struct SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElDynamic {
    psc_auto_connections: Option<
        DynamicBlock<SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElPscAutoConnectionsEl>,
    >,
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_consumer_projects: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_attachment_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_auto_connections:
        Option<Vec<SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElPscAutoConnectionsEl>>,
    dynamic: SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElDynamic,
}
impl SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl {
    #[doc = "Set the field `allowed_consumer_projects`.\nList of consumer projects that are allow-listed for PSC connections to this instance. This instance can be connected to with PSC from any network in these projects. Each consumer project in this list may be represented by a project number (numeric) or by a project id (alphanumeric)."]
    pub fn set_allowed_consumer_projects(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.allowed_consumer_projects = Some(v.into());
        self
    }
    #[doc = "Set the field `network_attachment_uri`.\nName of network attachment resource used to authorize a producer service to connect a PSC interface to the consumer's VPC. For example: \"projects/myProject/regions/myRegion/networkAttachments/myNetworkAttachment\". This is required to enable outbound connection on a PSC instance."]
    pub fn set_network_attachment_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_attachment_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `psc_enabled`.\nWhether PSC connectivity is enabled for this instance."]
    pub fn set_psc_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.psc_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `psc_auto_connections`.\n"]
    pub fn set_psc_auto_connections(
        mut self,
        v: impl Into<
            BlockAssignable<
                SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElPscAutoConnectionsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.psc_auto_connections = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.psc_auto_connections = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl {
    type O = BlockAssignable<SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl {}
impl BuildSqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl {
        SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl {
            allowed_consumer_projects: core::default::Default::default(),
            network_attachment_uri: core::default::Default::default(),
            psc_enabled: core::default::Default::default(),
            psc_auto_connections: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElRef {
        SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_consumer_projects` after provisioning.\nList of consumer projects that are allow-listed for PSC connections to this instance. This instance can be connected to with PSC from any network in these projects. Each consumer project in this list may be represented by a project number (numeric) or by a project id (alphanumeric)."]
    pub fn allowed_consumer_projects(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.allowed_consumer_projects", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network_attachment_uri` after provisioning.\nName of network attachment resource used to authorize a producer service to connect a PSC interface to the consumer's VPC. For example: \"projects/myProject/regions/myRegion/networkAttachments/myNetworkAttachment\". This is required to enable outbound connection on a PSC instance."]
    pub fn network_attachment_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_attachment_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `psc_enabled` after provisioning.\nWhether PSC connectivity is enabled for this instance."]
    pub fn psc_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.psc_enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `psc_auto_connections` after provisioning.\n"]
    pub fn psc_auto_connections(
        &self,
    ) -> ListRef<SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElPscAutoConnectionsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_auto_connections", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SqlDatabaseInstanceSettingsElIpConfigurationElDynamic {
    authorized_networks:
        Option<DynamicBlock<SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl>>,
    psc_config: Option<DynamicBlock<SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl>>,
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElIpConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allocated_ip_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_subject_alternative_names: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_private_path_for_google_cloud_services: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv4_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_ca_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_ca_pool: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_certificate_rotation_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorized_networks:
        Option<Vec<SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_config: Option<Vec<SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl>>,
    dynamic: SqlDatabaseInstanceSettingsElIpConfigurationElDynamic,
}
impl SqlDatabaseInstanceSettingsElIpConfigurationEl {
    #[doc = "Set the field `allocated_ip_range`.\nThe name of the allocated ip range for the private ip CloudSQL instance. For example: \"google-managed-services-default\". If set, the instance ip will be created in the allocated range. The range name must comply with RFC 1035. Specifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])?."]
    pub fn set_allocated_ip_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.allocated_ip_range = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_subject_alternative_names`.\nThe custom subject alternative names for an instance with \"CUSTOMER_MANAGED_CAS_CA\" as the \"server_ca_mode\"."]
    pub fn set_custom_subject_alternative_names(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.custom_subject_alternative_names = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_private_path_for_google_cloud_services`.\nWhether Google Cloud services such as BigQuery are allowed to access data in this Cloud SQL instance over a private IP connection. SQLSERVER database type is not supported."]
    pub fn set_enable_private_path_for_google_cloud_services(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.enable_private_path_for_google_cloud_services = Some(v.into());
        self
    }
    #[doc = "Set the field `ipv4_enabled`.\nWhether this Cloud SQL instance should be assigned a public IPV4 address. At least ipv4_enabled must be enabled or a private_network must be configured."]
    pub fn set_ipv4_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ipv4_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `private_network`.\nThe VPC network from which the Cloud SQL instance is accessible for private IP. For example, projects/myProject/global/networks/default. Specifying a network enables private IP. At least ipv4_enabled must be enabled or a private_network must be configured. This setting can be updated, but it cannot be removed after it is set."]
    pub fn set_private_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_network = Some(v.into());
        self
    }
    #[doc = "Set the field `server_ca_mode`.\nSpecify how the server certificate's Certificate Authority is hosted."]
    pub fn set_server_ca_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.server_ca_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `server_ca_pool`.\nThe resource name of the server CA pool for an instance with \"CUSTOMER_MANAGED_CAS_CA\" as the \"server_ca_mode\"."]
    pub fn set_server_ca_pool(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.server_ca_pool = Some(v.into());
        self
    }
    #[doc = "Set the field `server_certificate_rotation_mode`.\nSettings for how the server certificate gets rotated."]
    pub fn set_server_certificate_rotation_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.server_certificate_rotation_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `ssl_mode`.\nSpecify how SSL connection should be enforced in DB connections."]
    pub fn set_ssl_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `authorized_networks`.\n"]
    pub fn set_authorized_networks(
        mut self,
        v: impl Into<
            BlockAssignable<SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.authorized_networks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.authorized_networks = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `psc_config`.\n"]
    pub fn set_psc_config(
        mut self,
        v: impl Into<BlockAssignable<SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.psc_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.psc_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SqlDatabaseInstanceSettingsElIpConfigurationEl {
    type O = BlockAssignable<SqlDatabaseInstanceSettingsElIpConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceSettingsElIpConfigurationEl {}
impl BuildSqlDatabaseInstanceSettingsElIpConfigurationEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElIpConfigurationEl {
        SqlDatabaseInstanceSettingsElIpConfigurationEl {
            allocated_ip_range: core::default::Default::default(),
            custom_subject_alternative_names: core::default::Default::default(),
            enable_private_path_for_google_cloud_services: core::default::Default::default(),
            ipv4_enabled: core::default::Default::default(),
            private_network: core::default::Default::default(),
            server_ca_mode: core::default::Default::default(),
            server_ca_pool: core::default::Default::default(),
            server_certificate_rotation_mode: core::default::Default::default(),
            ssl_mode: core::default::Default::default(),
            authorized_networks: core::default::Default::default(),
            psc_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SqlDatabaseInstanceSettingsElIpConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceSettingsElIpConfigurationElRef {
    fn new(shared: StackShared, base: String) -> SqlDatabaseInstanceSettingsElIpConfigurationElRef {
        SqlDatabaseInstanceSettingsElIpConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceSettingsElIpConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allocated_ip_range` after provisioning.\nThe name of the allocated ip range for the private ip CloudSQL instance. For example: \"google-managed-services-default\". If set, the instance ip will be created in the allocated range. The range name must comply with RFC 1035. Specifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])?."]
    pub fn allocated_ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocated_ip_range", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `custom_subject_alternative_names` after provisioning.\nThe custom subject alternative names for an instance with \"CUSTOMER_MANAGED_CAS_CA\" as the \"server_ca_mode\"."]
    pub fn custom_subject_alternative_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.custom_subject_alternative_names", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_private_path_for_google_cloud_services` after provisioning.\nWhether Google Cloud services such as BigQuery are allowed to access data in this Cloud SQL instance over a private IP connection. SQLSERVER database type is not supported."]
    pub fn enable_private_path_for_google_cloud_services(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.enable_private_path_for_google_cloud_services",
                self.base
            ),
        )
    }
    #[doc = "Get a reference to the value of field `ipv4_enabled` after provisioning.\nWhether this Cloud SQL instance should be assigned a public IPV4 address. At least ipv4_enabled must be enabled or a private_network must be configured."]
    pub fn ipv4_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv4_enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `private_network` after provisioning.\nThe VPC network from which the Cloud SQL instance is accessible for private IP. For example, projects/myProject/global/networks/default. Specifying a network enables private IP. At least ipv4_enabled must be enabled or a private_network must be configured. This setting can be updated, but it cannot be removed after it is set."]
    pub fn private_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_network", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `server_ca_mode` after provisioning.\nSpecify how the server certificate's Certificate Authority is hosted."]
    pub fn server_ca_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_ca_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `server_ca_pool` after provisioning.\nThe resource name of the server CA pool for an instance with \"CUSTOMER_MANAGED_CAS_CA\" as the \"server_ca_mode\"."]
    pub fn server_ca_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_ca_pool", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `server_certificate_rotation_mode` after provisioning.\nSettings for how the server certificate gets rotated."]
    pub fn server_certificate_rotation_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_certificate_rotation_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_mode` after provisioning.\nSpecify how SSL connection should be enforced in DB connections."]
    pub fn ssl_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.base))
    }
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElLocationPreferenceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    follow_gae_application: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}
impl SqlDatabaseInstanceSettingsElLocationPreferenceEl {
    #[doc = "Set the field `follow_gae_application`.\nA Google App Engine application whose zone to remain in. Must be in the same region as this instance."]
    pub fn set_follow_gae_application(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.follow_gae_application = Some(v.into());
        self
    }
    #[doc = "Set the field `secondary_zone`.\nThe preferred Compute Engine zone for the secondary/failover"]
    pub fn set_secondary_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secondary_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\nThe preferred compute engine zone."]
    pub fn set_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zone = Some(v.into());
        self
    }
}
impl ToListMappable for SqlDatabaseInstanceSettingsElLocationPreferenceEl {
    type O = BlockAssignable<SqlDatabaseInstanceSettingsElLocationPreferenceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceSettingsElLocationPreferenceEl {}
impl BuildSqlDatabaseInstanceSettingsElLocationPreferenceEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElLocationPreferenceEl {
        SqlDatabaseInstanceSettingsElLocationPreferenceEl {
            follow_gae_application: core::default::Default::default(),
            secondary_zone: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}
pub struct SqlDatabaseInstanceSettingsElLocationPreferenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceSettingsElLocationPreferenceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SqlDatabaseInstanceSettingsElLocationPreferenceElRef {
        SqlDatabaseInstanceSettingsElLocationPreferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceSettingsElLocationPreferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `follow_gae_application` after provisioning.\nA Google App Engine application whose zone to remain in. Must be in the same region as this instance."]
    pub fn follow_gae_application(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.follow_gae_application", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_zone` after provisioning.\nThe preferred Compute Engine zone for the secondary/failover"]
    pub fn secondary_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secondary_zone", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe preferred compute engine zone."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElMaintenanceWindowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hour: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_track: Option<PrimField<String>>,
}
impl SqlDatabaseInstanceSettingsElMaintenanceWindowEl {
    #[doc = "Set the field `day`.\nDay of week (1-7), starting on Monday"]
    pub fn set_day(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.day = Some(v.into());
        self
    }
    #[doc = "Set the field `hour`.\nHour of day (0-23), ignored if day not set"]
    pub fn set_hour(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hour = Some(v.into());
        self
    }
    #[doc = "Set the field `update_track`.\nReceive updates after one week (canary) or after two weeks (stable) or after five weeks (week5) of notification."]
    pub fn set_update_track(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_track = Some(v.into());
        self
    }
}
impl ToListMappable for SqlDatabaseInstanceSettingsElMaintenanceWindowEl {
    type O = BlockAssignable<SqlDatabaseInstanceSettingsElMaintenanceWindowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceSettingsElMaintenanceWindowEl {}
impl BuildSqlDatabaseInstanceSettingsElMaintenanceWindowEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElMaintenanceWindowEl {
        SqlDatabaseInstanceSettingsElMaintenanceWindowEl {
            day: core::default::Default::default(),
            hour: core::default::Default::default(),
            update_track: core::default::Default::default(),
        }
    }
}
pub struct SqlDatabaseInstanceSettingsElMaintenanceWindowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceSettingsElMaintenanceWindowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SqlDatabaseInstanceSettingsElMaintenanceWindowElRef {
        SqlDatabaseInstanceSettingsElMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceSettingsElMaintenanceWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day` after provisioning.\nDay of week (1-7), starting on Monday"]
    pub fn day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }
    #[doc = "Get a reference to the value of field `hour` after provisioning.\nHour of day (0-23), ignored if day not set"]
    pub fn hour(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hour", self.base))
    }
    #[doc = "Get a reference to the value of field `update_track` after provisioning.\nReceive updates after one week (canary) or after two weeks (stable) or after five weeks (week5) of notification."]
    pub fn update_track(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_track", self.base))
    }
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElPasswordValidationPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    complexity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disallow_username_substring: Option<PrimField<bool>>,
    enable_password_policy: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_change_interval: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reuse_interval: Option<PrimField<f64>>,
}
impl SqlDatabaseInstanceSettingsElPasswordValidationPolicyEl {
    #[doc = "Set the field `complexity`.\nPassword complexity."]
    pub fn set_complexity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.complexity = Some(v.into());
        self
    }
    #[doc = "Set the field `disallow_username_substring`.\nDisallow username as a part of the password."]
    pub fn set_disallow_username_substring(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disallow_username_substring = Some(v.into());
        self
    }
    #[doc = "Set the field `min_length`.\nMinimum number of characters allowed."]
    pub fn set_min_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_length = Some(v.into());
        self
    }
    #[doc = "Set the field `password_change_interval`.\nMinimum interval after which the password can be changed. This flag is only supported for PostgresSQL."]
    pub fn set_password_change_interval(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password_change_interval = Some(v.into());
        self
    }
    #[doc = "Set the field `reuse_interval`.\nNumber of previous passwords that cannot be reused."]
    pub fn set_reuse_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.reuse_interval = Some(v.into());
        self
    }
}
impl ToListMappable for SqlDatabaseInstanceSettingsElPasswordValidationPolicyEl {
    type O = BlockAssignable<SqlDatabaseInstanceSettingsElPasswordValidationPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceSettingsElPasswordValidationPolicyEl {
    #[doc = "Whether the password policy is enabled or not."]
    pub enable_password_policy: PrimField<bool>,
}
impl BuildSqlDatabaseInstanceSettingsElPasswordValidationPolicyEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElPasswordValidationPolicyEl {
        SqlDatabaseInstanceSettingsElPasswordValidationPolicyEl {
            complexity: core::default::Default::default(),
            disallow_username_substring: core::default::Default::default(),
            enable_password_policy: self.enable_password_policy,
            min_length: core::default::Default::default(),
            password_change_interval: core::default::Default::default(),
            reuse_interval: core::default::Default::default(),
        }
    }
}
pub struct SqlDatabaseInstanceSettingsElPasswordValidationPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceSettingsElPasswordValidationPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SqlDatabaseInstanceSettingsElPasswordValidationPolicyElRef {
        SqlDatabaseInstanceSettingsElPasswordValidationPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceSettingsElPasswordValidationPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `complexity` after provisioning.\nPassword complexity."]
    pub fn complexity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.complexity", self.base))
    }
    #[doc = "Get a reference to the value of field `disallow_username_substring` after provisioning.\nDisallow username as a part of the password."]
    pub fn disallow_username_substring(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disallow_username_substring", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_password_policy` after provisioning.\nWhether the password policy is enabled or not."]
    pub fn enable_password_policy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_password_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_length` after provisioning.\nMinimum number of characters allowed."]
    pub fn min_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_length", self.base))
    }
    #[doc = "Get a reference to the value of field `password_change_interval` after provisioning.\nMinimum interval after which the password can be changed. This flag is only supported for PostgresSQL."]
    pub fn password_change_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password_change_interval", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `reuse_interval` after provisioning.\nNumber of previous passwords that cannot be reused."]
    pub fn reuse_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reuse_interval", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigElTargetMetricsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metric: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_value: Option<PrimField<f64>>,
}
impl SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigElTargetMetricsEl {
    #[doc = "Set the field `metric`.\nMetric name for Read Pool Auto Scale."]
    pub fn set_metric(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metric = Some(v.into());
        self
    }
    #[doc = "Set the field `target_value`.\nTarget value for Read Pool Auto Scale."]
    pub fn set_target_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.target_value = Some(v.into());
        self
    }
}
impl ToListMappable for SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigElTargetMetricsEl {
    type O = BlockAssignable<SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigElTargetMetricsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigElTargetMetricsEl {}
impl BuildSqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigElTargetMetricsEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigElTargetMetricsEl {
        SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigElTargetMetricsEl {
            metric: core::default::Default::default(),
            target_value: core::default::Default::default(),
        }
    }
}
pub struct SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigElTargetMetricsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigElTargetMetricsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigElTargetMetricsElRef {
        SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigElTargetMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigElTargetMetricsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metric` after provisioning.\nMetric name for Read Pool Auto Scale."]
    pub fn metric(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric", self.base))
    }
    #[doc = "Get a reference to the value of field `target_value` after provisioning.\nTarget value for Read Pool Auto Scale."]
    pub fn target_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_value", self.base))
    }
}
#[derive(Serialize, Default)]
struct SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigElDynamic {
    target_metrics:
        Option<DynamicBlock<SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigElTargetMetricsEl>>,
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_scale_in: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_in_cooldown_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_out_cooldown_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_metrics:
        Option<Vec<SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigElTargetMetricsEl>>,
    dynamic: SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigElDynamic,
}
impl SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigEl {
    #[doc = "Set the field `disable_scale_in`.\nTrue if auto scale in is disabled."]
    pub fn set_disable_scale_in(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_scale_in = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\nTrue if Read Pool Auto Scale is enabled."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `max_node_count`.\nMaximum number of nodes in the read pool. If set to lower than current node count, node count will be updated."]
    pub fn set_max_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `min_node_count`.\nMinimum number of nodes in the read pool. If set to higher than current node count, node count will be updated."]
    pub fn set_min_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `scale_in_cooldown_seconds`.\nThe cooldown period for scale in operations."]
    pub fn set_scale_in_cooldown_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.scale_in_cooldown_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `scale_out_cooldown_seconds`.\nThe cooldown period for scale out operations."]
    pub fn set_scale_out_cooldown_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.scale_out_cooldown_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `target_metrics`.\n"]
    pub fn set_target_metrics(
        mut self,
        v: impl Into<
            BlockAssignable<SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigElTargetMetricsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target_metrics = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target_metrics = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigEl {
    type O = BlockAssignable<SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigEl {}
impl BuildSqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigEl {
        SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigEl {
            disable_scale_in: core::default::Default::default(),
            enabled: core::default::Default::default(),
            max_node_count: core::default::Default::default(),
            min_node_count: core::default::Default::default(),
            scale_in_cooldown_seconds: core::default::Default::default(),
            scale_out_cooldown_seconds: core::default::Default::default(),
            target_metrics: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigElRef {
        SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disable_scale_in` after provisioning.\nTrue if auto scale in is disabled."]
    pub fn disable_scale_in(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_scale_in", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nTrue if Read Pool Auto Scale is enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `max_node_count` after provisioning.\nMaximum number of nodes in the read pool. If set to lower than current node count, node count will be updated."]
    pub fn max_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_node_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_node_count` after provisioning.\nMinimum number of nodes in the read pool. If set to higher than current node count, node count will be updated."]
    pub fn min_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_node_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scale_in_cooldown_seconds` after provisioning.\nThe cooldown period for scale in operations."]
    pub fn scale_in_cooldown_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scale_in_cooldown_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scale_out_cooldown_seconds` after provisioning.\nThe cooldown period for scale out operations."]
    pub fn scale_out_cooldown_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scale_out_cooldown_seconds", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElSqlServerAuditConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_interval: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upload_interval: Option<PrimField<String>>,
}
impl SqlDatabaseInstanceSettingsElSqlServerAuditConfigEl {
    #[doc = "Set the field `bucket`.\nThe name of the destination bucket (e.g., gs://mybucket)."]
    pub fn set_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket = Some(v.into());
        self
    }
    #[doc = "Set the field `retention_interval`.\nHow long to keep generated audit files. A duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\".."]
    pub fn set_retention_interval(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.retention_interval = Some(v.into());
        self
    }
    #[doc = "Set the field `upload_interval`.\nHow often to upload generated audit files. A duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_upload_interval(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.upload_interval = Some(v.into());
        self
    }
}
impl ToListMappable for SqlDatabaseInstanceSettingsElSqlServerAuditConfigEl {
    type O = BlockAssignable<SqlDatabaseInstanceSettingsElSqlServerAuditConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceSettingsElSqlServerAuditConfigEl {}
impl BuildSqlDatabaseInstanceSettingsElSqlServerAuditConfigEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElSqlServerAuditConfigEl {
        SqlDatabaseInstanceSettingsElSqlServerAuditConfigEl {
            bucket: core::default::Default::default(),
            retention_interval: core::default::Default::default(),
            upload_interval: core::default::Default::default(),
        }
    }
}
pub struct SqlDatabaseInstanceSettingsElSqlServerAuditConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceSettingsElSqlServerAuditConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SqlDatabaseInstanceSettingsElSqlServerAuditConfigElRef {
        SqlDatabaseInstanceSettingsElSqlServerAuditConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceSettingsElSqlServerAuditConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nThe name of the destination bucket (e.g., gs://mybucket)."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `retention_interval` after provisioning.\nHow long to keep generated audit files. A duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\".."]
    pub fn retention_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention_interval", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `upload_interval` after provisioning.\nHow often to upload generated audit files. A duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn upload_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.upload_interval", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SqlDatabaseInstanceSettingsElDynamic {
    active_directory_config:
        Option<DynamicBlock<SqlDatabaseInstanceSettingsElActiveDirectoryConfigEl>>,
    advanced_machine_features:
        Option<DynamicBlock<SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl>>,
    backup_configuration: Option<DynamicBlock<SqlDatabaseInstanceSettingsElBackupConfigurationEl>>,
    connection_pool_config:
        Option<DynamicBlock<SqlDatabaseInstanceSettingsElConnectionPoolConfigEl>>,
    data_cache_config: Option<DynamicBlock<SqlDatabaseInstanceSettingsElDataCacheConfigEl>>,
    database_flags: Option<DynamicBlock<SqlDatabaseInstanceSettingsElDatabaseFlagsEl>>,
    deny_maintenance_period:
        Option<DynamicBlock<SqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl>>,
    entraid_config: Option<DynamicBlock<SqlDatabaseInstanceSettingsElEntraidConfigEl>>,
    final_backup_config: Option<DynamicBlock<SqlDatabaseInstanceSettingsElFinalBackupConfigEl>>,
    insights_config: Option<DynamicBlock<SqlDatabaseInstanceSettingsElInsightsConfigEl>>,
    ip_configuration: Option<DynamicBlock<SqlDatabaseInstanceSettingsElIpConfigurationEl>>,
    location_preference: Option<DynamicBlock<SqlDatabaseInstanceSettingsElLocationPreferenceEl>>,
    maintenance_window: Option<DynamicBlock<SqlDatabaseInstanceSettingsElMaintenanceWindowEl>>,
    password_validation_policy:
        Option<DynamicBlock<SqlDatabaseInstanceSettingsElPasswordValidationPolicyEl>>,
    read_pool_auto_scale_config:
        Option<DynamicBlock<SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigEl>>,
    sql_server_audit_config:
        Option<DynamicBlock<SqlDatabaseInstanceSettingsElSqlServerAuditConfigEl>>,
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    activation_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_upgrade_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connector_enforcement: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_api_access: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_autoresize: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_autoresize_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_dataplex_integration: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_google_ml_integration: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pricing_plan: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retain_backups_on_delete: Option<PrimField<bool>>,
    tier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active_directory_config: Option<Vec<SqlDatabaseInstanceSettingsElActiveDirectoryConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_machine_features: Option<Vec<SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_configuration: Option<Vec<SqlDatabaseInstanceSettingsElBackupConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_pool_config: Option<Vec<SqlDatabaseInstanceSettingsElConnectionPoolConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_cache_config: Option<Vec<SqlDatabaseInstanceSettingsElDataCacheConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_flags: Option<Vec<SqlDatabaseInstanceSettingsElDatabaseFlagsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deny_maintenance_period: Option<Vec<SqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entraid_config: Option<Vec<SqlDatabaseInstanceSettingsElEntraidConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    final_backup_config: Option<Vec<SqlDatabaseInstanceSettingsElFinalBackupConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insights_config: Option<Vec<SqlDatabaseInstanceSettingsElInsightsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_configuration: Option<Vec<SqlDatabaseInstanceSettingsElIpConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location_preference: Option<Vec<SqlDatabaseInstanceSettingsElLocationPreferenceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_window: Option<Vec<SqlDatabaseInstanceSettingsElMaintenanceWindowEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_validation_policy:
        Option<Vec<SqlDatabaseInstanceSettingsElPasswordValidationPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_pool_auto_scale_config:
        Option<Vec<SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sql_server_audit_config: Option<Vec<SqlDatabaseInstanceSettingsElSqlServerAuditConfigEl>>,
    dynamic: SqlDatabaseInstanceSettingsElDynamic,
}
impl SqlDatabaseInstanceSettingsEl {
    #[doc = "Set the field `activation_policy`.\nThis specifies when the instance should be active. Can be either ALWAYS, NEVER or ON_DEMAND."]
    pub fn set_activation_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.activation_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `auto_upgrade_enabled`.\nEnables Automatic Version Upgrade feature. Can be used with MySQL only."]
    pub fn set_auto_upgrade_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_upgrade_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `availability_type`.\nThe availability type of the Cloud SQL instance, high availability\n(REGIONAL) or single zone (ZONAL). For all instances, ensure that\nsettings.backup_configuration.enabled is set to true.\nFor MySQL instances, ensure that settings.backup_configuration.binary_log_enabled is set to true.\nFor Postgres instances, ensure that settings.backup_configuration.point_in_time_recovery_enabled\nis set to true. Defaults to ZONAL.\nFor read pool instances, this field is read-only. The availability type is changed by specifying\nthe number of nodes (node_count)."]
    pub fn set_availability_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.availability_type = Some(v.into());
        self
    }
    #[doc = "Set the field `collation`.\nThe name of server instance collation."]
    pub fn set_collation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.collation = Some(v.into());
        self
    }
    #[doc = "Set the field `connector_enforcement`.\nEnables the enforcement of Cloud SQL Auth Proxy or Cloud SQL connectors for all the connections. If enabled, all the direct connections are rejected."]
    pub fn set_connector_enforcement(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connector_enforcement = Some(v.into());
        self
    }
    #[doc = "Set the field `data_api_access`.\nAllows using ExecuteSql API to connect to the instance. Disallowed when unspecified."]
    pub fn set_data_api_access(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_api_access = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_protection_enabled`.\nConfiguration to protect against accidental instance deletion."]
    pub fn set_deletion_protection_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deletion_protection_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_autoresize`.\nEnables auto-resizing of the storage size. Defaults to true."]
    pub fn set_disk_autoresize(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disk_autoresize = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_autoresize_limit`.\nThe maximum size, in GB, to which storage capacity can be automatically increased. The default value is 0, which specifies that there is no limit."]
    pub fn set_disk_autoresize_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_autoresize_limit = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_size`.\nThe size of data disk, in GB. Size of a running instance cannot be reduced but can be increased. The minimum value is 10GB for PD_SSD, PD_HDD and 20GB for HYPERDISK_BALANCED."]
    pub fn set_disk_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_size = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_type`.\nThe type of supported data disk is tier dependent and can be PD_SSD or PD_HDD or HYPERDISK_BALANCED."]
    pub fn set_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_type = Some(v.into());
        self
    }
    #[doc = "Set the field `edition`.\nThe edition of the instance, can be ENTERPRISE or ENTERPRISE_PLUS."]
    pub fn set_edition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.edition = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_dataplex_integration`.\nEnables Dataplex Integration."]
    pub fn set_enable_dataplex_integration(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_dataplex_integration = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_google_ml_integration`.\nEnables Vertex AI Integration."]
    pub fn set_enable_google_ml_integration(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_google_ml_integration = Some(v.into());
        self
    }
    #[doc = "Set the field `pricing_plan`.\nPricing plan for this instance, can only be PER_USE."]
    pub fn set_pricing_plan(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pricing_plan = Some(v.into());
        self
    }
    #[doc = "Set the field `retain_backups_on_delete`.\nWhen this parameter is set to true, Cloud SQL retains backups of the instance even after the instance is deleted. The ON_DEMAND backup will be retained until customer deletes the backup or the project. The AUTOMATED backup will be retained based on the backups retention setting."]
    pub fn set_retain_backups_on_delete(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.retain_backups_on_delete = Some(v.into());
        self
    }
    #[doc = "Set the field `time_zone`.\nThe time_zone to be used by the database engine (supported only for SQL Server), in SQL Server timezone format."]
    pub fn set_time_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.time_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `user_labels`.\nA set of key/value user label pairs to assign to the instance."]
    pub fn set_user_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.user_labels = Some(v.into());
        self
    }
    #[doc = "Set the field `active_directory_config`.\n"]
    pub fn set_active_directory_config(
        mut self,
        v: impl Into<BlockAssignable<SqlDatabaseInstanceSettingsElActiveDirectoryConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.active_directory_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.active_directory_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `advanced_machine_features`.\n"]
    pub fn set_advanced_machine_features(
        mut self,
        v: impl Into<BlockAssignable<SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.advanced_machine_features = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_machine_features = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `backup_configuration`.\n"]
    pub fn set_backup_configuration(
        mut self,
        v: impl Into<BlockAssignable<SqlDatabaseInstanceSettingsElBackupConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.backup_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.backup_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `connection_pool_config`.\n"]
    pub fn set_connection_pool_config(
        mut self,
        v: impl Into<BlockAssignable<SqlDatabaseInstanceSettingsElConnectionPoolConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.connection_pool_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.connection_pool_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `data_cache_config`.\n"]
    pub fn set_data_cache_config(
        mut self,
        v: impl Into<BlockAssignable<SqlDatabaseInstanceSettingsElDataCacheConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data_cache_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data_cache_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `database_flags`.\n"]
    pub fn set_database_flags(
        mut self,
        v: impl Into<BlockAssignable<SqlDatabaseInstanceSettingsElDatabaseFlagsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.database_flags = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.database_flags = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `deny_maintenance_period`.\n"]
    pub fn set_deny_maintenance_period(
        mut self,
        v: impl Into<BlockAssignable<SqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.deny_maintenance_period = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.deny_maintenance_period = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `entraid_config`.\n"]
    pub fn set_entraid_config(
        mut self,
        v: impl Into<BlockAssignable<SqlDatabaseInstanceSettingsElEntraidConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.entraid_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.entraid_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `final_backup_config`.\n"]
    pub fn set_final_backup_config(
        mut self,
        v: impl Into<BlockAssignable<SqlDatabaseInstanceSettingsElFinalBackupConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.final_backup_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.final_backup_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `insights_config`.\n"]
    pub fn set_insights_config(
        mut self,
        v: impl Into<BlockAssignable<SqlDatabaseInstanceSettingsElInsightsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.insights_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.insights_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ip_configuration`.\n"]
    pub fn set_ip_configuration(
        mut self,
        v: impl Into<BlockAssignable<SqlDatabaseInstanceSettingsElIpConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ip_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `location_preference`.\n"]
    pub fn set_location_preference(
        mut self,
        v: impl Into<BlockAssignable<SqlDatabaseInstanceSettingsElLocationPreferenceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.location_preference = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.location_preference = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(
        mut self,
        v: impl Into<BlockAssignable<SqlDatabaseInstanceSettingsElMaintenanceWindowEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.maintenance_window = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.maintenance_window = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `password_validation_policy`.\n"]
    pub fn set_password_validation_policy(
        mut self,
        v: impl Into<BlockAssignable<SqlDatabaseInstanceSettingsElPasswordValidationPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.password_validation_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.password_validation_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `read_pool_auto_scale_config`.\n"]
    pub fn set_read_pool_auto_scale_config(
        mut self,
        v: impl Into<BlockAssignable<SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.read_pool_auto_scale_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.read_pool_auto_scale_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sql_server_audit_config`.\n"]
    pub fn set_sql_server_audit_config(
        mut self,
        v: impl Into<BlockAssignable<SqlDatabaseInstanceSettingsElSqlServerAuditConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sql_server_audit_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sql_server_audit_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SqlDatabaseInstanceSettingsEl {
    type O = BlockAssignable<SqlDatabaseInstanceSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceSettingsEl {
    #[doc = "The machine type to use. See tiers for more details and supported versions. Postgres supports only shared-core machine types, and custom machine types such as db-custom-2-13312. See the Custom Machine Type Documentation to learn about specifying custom machine types."]
    pub tier: PrimField<String>,
}
impl BuildSqlDatabaseInstanceSettingsEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsEl {
        SqlDatabaseInstanceSettingsEl {
            activation_policy: core::default::Default::default(),
            auto_upgrade_enabled: core::default::Default::default(),
            availability_type: core::default::Default::default(),
            collation: core::default::Default::default(),
            connector_enforcement: core::default::Default::default(),
            data_api_access: core::default::Default::default(),
            deletion_protection_enabled: core::default::Default::default(),
            disk_autoresize: core::default::Default::default(),
            disk_autoresize_limit: core::default::Default::default(),
            disk_size: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            edition: core::default::Default::default(),
            enable_dataplex_integration: core::default::Default::default(),
            enable_google_ml_integration: core::default::Default::default(),
            pricing_plan: core::default::Default::default(),
            retain_backups_on_delete: core::default::Default::default(),
            tier: self.tier,
            time_zone: core::default::Default::default(),
            user_labels: core::default::Default::default(),
            active_directory_config: core::default::Default::default(),
            advanced_machine_features: core::default::Default::default(),
            backup_configuration: core::default::Default::default(),
            connection_pool_config: core::default::Default::default(),
            data_cache_config: core::default::Default::default(),
            database_flags: core::default::Default::default(),
            deny_maintenance_period: core::default::Default::default(),
            entraid_config: core::default::Default::default(),
            final_backup_config: core::default::Default::default(),
            insights_config: core::default::Default::default(),
            ip_configuration: core::default::Default::default(),
            location_preference: core::default::Default::default(),
            maintenance_window: core::default::Default::default(),
            password_validation_policy: core::default::Default::default(),
            read_pool_auto_scale_config: core::default::Default::default(),
            sql_server_audit_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SqlDatabaseInstanceSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceSettingsElRef {
    fn new(shared: StackShared, base: String) -> SqlDatabaseInstanceSettingsElRef {
        SqlDatabaseInstanceSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `activation_policy` after provisioning.\nThis specifies when the instance should be active. Can be either ALWAYS, NEVER or ON_DEMAND."]
    pub fn activation_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.activation_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `auto_upgrade_enabled` after provisioning.\nEnables Automatic Version Upgrade feature. Can be used with MySQL only."]
    pub fn auto_upgrade_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_upgrade_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `availability_type` after provisioning.\nThe availability type of the Cloud SQL instance, high availability\n(REGIONAL) or single zone (ZONAL). For all instances, ensure that\nsettings.backup_configuration.enabled is set to true.\nFor MySQL instances, ensure that settings.backup_configuration.binary_log_enabled is set to true.\nFor Postgres instances, ensure that settings.backup_configuration.point_in_time_recovery_enabled\nis set to true. Defaults to ZONAL.\nFor read pool instances, this field is read-only. The availability type is changed by specifying\nthe number of nodes (node_count)."]
    pub fn availability_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `collation` after provisioning.\nThe name of server instance collation."]
    pub fn collation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.collation", self.base))
    }
    #[doc = "Get a reference to the value of field `connector_enforcement` after provisioning.\nEnables the enforcement of Cloud SQL Auth Proxy or Cloud SQL connectors for all the connections. If enabled, all the direct connections are rejected."]
    pub fn connector_enforcement(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connector_enforcement", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_api_access` after provisioning.\nAllows using ExecuteSql API to connect to the instance. Disallowed when unspecified."]
    pub fn data_api_access(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_api_access", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\nConfiguration to protect against accidental instance deletion."]
    pub fn deletion_protection_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disk_autoresize` after provisioning.\nEnables auto-resizing of the storage size. Defaults to true."]
    pub fn disk_autoresize(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disk_autoresize", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disk_autoresize_limit` after provisioning.\nThe maximum size, in GB, to which storage capacity can be automatically increased. The default value is 0, which specifies that there is no limit."]
    pub fn disk_autoresize_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disk_autoresize_limit", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disk_size` after provisioning.\nThe size of data disk, in GB. Size of a running instance cannot be reduced but can be increased. The minimum value is 10GB for PD_SSD, PD_HDD and 20GB for HYPERDISK_BALANCED."]
    pub fn disk_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size", self.base))
    }
    #[doc = "Get a reference to the value of field `disk_type` after provisioning.\nThe type of supported data disk is tier dependent and can be PD_SSD or PD_HDD or HYPERDISK_BALANCED."]
    pub fn disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }
    #[doc = "Get a reference to the value of field `edition` after provisioning.\nThe edition of the instance, can be ENTERPRISE or ENTERPRISE_PLUS."]
    pub fn edition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.edition", self.base))
    }
    #[doc = "Get a reference to the value of field `effective_availability_type` after provisioning.\nThe availability type of the Cloud SQL instance, high availability\n(REGIONAL) or single zone (ZONAL). This field always contains the value that is reported by the\nAPI (for read pools, effective_availability_type may differ from availability_type)."]
    pub fn effective_availability_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_availability_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_dataplex_integration` after provisioning.\nEnables Dataplex Integration."]
    pub fn enable_dataplex_integration(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_dataplex_integration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_google_ml_integration` after provisioning.\nEnables Vertex AI Integration."]
    pub fn enable_google_ml_integration(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_google_ml_integration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pricing_plan` after provisioning.\nPricing plan for this instance, can only be PER_USE."]
    pub fn pricing_plan(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pricing_plan", self.base))
    }
    #[doc = "Get a reference to the value of field `retain_backups_on_delete` after provisioning.\nWhen this parameter is set to true, Cloud SQL retains backups of the instance even after the instance is deleted. The ON_DEMAND backup will be retained until customer deletes the backup or the project. The AUTOMATED backup will be retained based on the backups retention setting."]
    pub fn retain_backups_on_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retain_backups_on_delete", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tier` after provisioning.\nThe machine type to use. See tiers for more details and supported versions. Postgres supports only shared-core machine types, and custom machine types such as db-custom-2-13312. See the Custom Machine Type Documentation to learn about specifying custom machine types."]
    pub fn tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tier", self.base))
    }
    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\nThe time_zone to be used by the database engine (supported only for SQL Server), in SQL Server timezone format."]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
    #[doc = "Get a reference to the value of field `user_labels` after provisioning.\nA set of key/value user label pairs to assign to the instance."]
    pub fn user_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.user_labels", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nUsed to make sure changes to the settings block are atomic."]
    pub fn version(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
    #[doc = "Get a reference to the value of field `active_directory_config` after provisioning.\n"]
    pub fn active_directory_config(
        &self,
    ) -> ListRef<SqlDatabaseInstanceSettingsElActiveDirectoryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.active_directory_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `advanced_machine_features` after provisioning.\n"]
    pub fn advanced_machine_features(
        &self,
    ) -> ListRef<SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_machine_features", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `backup_configuration` after provisioning.\n"]
    pub fn backup_configuration(
        &self,
    ) -> ListRef<SqlDatabaseInstanceSettingsElBackupConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backup_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_cache_config` after provisioning.\n"]
    pub fn data_cache_config(&self) -> ListRef<SqlDatabaseInstanceSettingsElDataCacheConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_cache_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `deny_maintenance_period` after provisioning.\n"]
    pub fn deny_maintenance_period(
        &self,
    ) -> ListRef<SqlDatabaseInstanceSettingsElDenyMaintenancePeriodElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.deny_maintenance_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `entraid_config` after provisioning.\n"]
    pub fn entraid_config(&self) -> ListRef<SqlDatabaseInstanceSettingsElEntraidConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.entraid_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `final_backup_config` after provisioning.\n"]
    pub fn final_backup_config(
        &self,
    ) -> ListRef<SqlDatabaseInstanceSettingsElFinalBackupConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.final_backup_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `insights_config` after provisioning.\n"]
    pub fn insights_config(&self) -> ListRef<SqlDatabaseInstanceSettingsElInsightsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.insights_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ip_configuration` after provisioning.\n"]
    pub fn ip_configuration(&self) -> ListRef<SqlDatabaseInstanceSettingsElIpConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ip_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `location_preference` after provisioning.\n"]
    pub fn location_preference(
        &self,
    ) -> ListRef<SqlDatabaseInstanceSettingsElLocationPreferenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.location_preference", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(
        &self,
    ) -> ListRef<SqlDatabaseInstanceSettingsElMaintenanceWindowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_window", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `password_validation_policy` after provisioning.\n"]
    pub fn password_validation_policy(
        &self,
    ) -> ListRef<SqlDatabaseInstanceSettingsElPasswordValidationPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.password_validation_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `read_pool_auto_scale_config` after provisioning.\n"]
    pub fn read_pool_auto_scale_config(
        &self,
    ) -> ListRef<SqlDatabaseInstanceSettingsElReadPoolAutoScaleConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.read_pool_auto_scale_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sql_server_audit_config` after provisioning.\n"]
    pub fn sql_server_audit_config(
        &self,
    ) -> ListRef<SqlDatabaseInstanceSettingsElSqlServerAuditConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sql_server_audit_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SqlDatabaseInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl SqlDatabaseInstanceTimeoutsEl {
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
impl ToListMappable for SqlDatabaseInstanceTimeoutsEl {
    type O = BlockAssignable<SqlDatabaseInstanceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlDatabaseInstanceTimeoutsEl {}
impl BuildSqlDatabaseInstanceTimeoutsEl {
    pub fn build(self) -> SqlDatabaseInstanceTimeoutsEl {
        SqlDatabaseInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct SqlDatabaseInstanceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlDatabaseInstanceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> SqlDatabaseInstanceTimeoutsElRef {
        SqlDatabaseInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlDatabaseInstanceTimeoutsElRef {
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
struct SqlDatabaseInstanceDynamic {
    clone: Option<DynamicBlock<SqlDatabaseInstanceCloneEl>>,
    point_in_time_restore_context:
        Option<DynamicBlock<SqlDatabaseInstancePointInTimeRestoreContextEl>>,
    replica_configuration: Option<DynamicBlock<SqlDatabaseInstanceReplicaConfigurationEl>>,
    replication_cluster: Option<DynamicBlock<SqlDatabaseInstanceReplicationClusterEl>>,
    restore_backup_context: Option<DynamicBlock<SqlDatabaseInstanceRestoreBackupContextEl>>,
    settings: Option<DynamicBlock<SqlDatabaseInstanceSettingsEl>>,
}
