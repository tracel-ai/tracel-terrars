use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DbInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allocated_storage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_major_version_upgrade: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apply_immediately: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_minor_version_upgrade: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_retention_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_target: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_window: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_cert_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    character_set_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_tags_to_snapshot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_iam_instance_profile: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_owned_ip_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_insights_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_subnet_group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dedicated_log_volume: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_automated_backups: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_auth_secret_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_dns_ips: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_fqdn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_iam_role_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_ou: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled_cloudwatch_logs_exports: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine_lifecycle_support: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    final_snapshot_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_database_authentication_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identifier_prefix: Option<PrimField<String>>,
    instance_class: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    license_model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_window: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manage_master_user_password: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    master_user_secret_kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_allocated_storage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitoring_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitoring_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_az: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nchar_character_set_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    option_group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter_group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_wo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_wo_version: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performance_insights_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performance_insights_kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performance_insights_retention_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publicly_accessible: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replica_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replicate_source_db: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_final_snapshot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_encrypted: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_throughput: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timezone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upgrade_storage_config: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blue_green_update: Option<Vec<DbInstanceBlueGreenUpdateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restore_to_point_in_time: Option<Vec<DbInstanceRestoreToPointInTimeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_import: Option<Vec<DbInstanceS3ImportEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DbInstanceTimeoutsEl>,
    dynamic: DbInstanceDynamic,
}
struct DbInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DbInstanceData>,
}
#[derive(Clone)]
pub struct DbInstance(Rc<DbInstance_>);
impl DbInstance {
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
    #[doc = "Set the field `allocated_storage`.\n"]
    pub fn set_allocated_storage(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().allocated_storage = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_major_version_upgrade`.\n"]
    pub fn set_allow_major_version_upgrade(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_major_version_upgrade = Some(v.into());
        self
    }
    #[doc = "Set the field `apply_immediately`.\n"]
    pub fn set_apply_immediately(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().apply_immediately = Some(v.into());
        self
    }
    #[doc = "Set the field `auto_minor_version_upgrade`.\n"]
    pub fn set_auto_minor_version_upgrade(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_minor_version_upgrade = Some(v.into());
        self
    }
    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().availability_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `backup_retention_period`.\n"]
    pub fn set_backup_retention_period(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().backup_retention_period = Some(v.into());
        self
    }
    #[doc = "Set the field `backup_target`.\n"]
    pub fn set_backup_target(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().backup_target = Some(v.into());
        self
    }
    #[doc = "Set the field `backup_window`.\n"]
    pub fn set_backup_window(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().backup_window = Some(v.into());
        self
    }
    #[doc = "Set the field `ca_cert_identifier`.\n"]
    pub fn set_ca_cert_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ca_cert_identifier = Some(v.into());
        self
    }
    #[doc = "Set the field `character_set_name`.\n"]
    pub fn set_character_set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().character_set_name = Some(v.into());
        self
    }
    #[doc = "Set the field `copy_tags_to_snapshot`.\n"]
    pub fn set_copy_tags_to_snapshot(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().copy_tags_to_snapshot = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_iam_instance_profile`.\n"]
    pub fn set_custom_iam_instance_profile(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().custom_iam_instance_profile = Some(v.into());
        self
    }
    #[doc = "Set the field `customer_owned_ip_enabled`.\n"]
    pub fn set_customer_owned_ip_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().customer_owned_ip_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `database_insights_mode`.\n"]
    pub fn set_database_insights_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().database_insights_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `db_name`.\n"]
    pub fn set_db_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().db_name = Some(v.into());
        self
    }
    #[doc = "Set the field `db_subnet_group_name`.\n"]
    pub fn set_db_subnet_group_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().db_subnet_group_name = Some(v.into());
        self
    }
    #[doc = "Set the field `dedicated_log_volume`.\n"]
    pub fn set_dedicated_log_volume(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().dedicated_log_volume = Some(v.into());
        self
    }
    #[doc = "Set the field `delete_automated_backups`.\n"]
    pub fn set_delete_automated_backups(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_automated_backups = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_protection`.\n"]
    pub fn set_deletion_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = Some(v.into());
        self
    }
    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().domain = Some(v.into());
        self
    }
    #[doc = "Set the field `domain_auth_secret_arn`.\n"]
    pub fn set_domain_auth_secret_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().domain_auth_secret_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `domain_dns_ips`.\n"]
    pub fn set_domain_dns_ips(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().domain_dns_ips = Some(v.into());
        self
    }
    #[doc = "Set the field `domain_fqdn`.\n"]
    pub fn set_domain_fqdn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().domain_fqdn = Some(v.into());
        self
    }
    #[doc = "Set the field `domain_iam_role_name`.\n"]
    pub fn set_domain_iam_role_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().domain_iam_role_name = Some(v.into());
        self
    }
    #[doc = "Set the field `domain_ou`.\n"]
    pub fn set_domain_ou(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().domain_ou = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled_cloudwatch_logs_exports`.\n"]
    pub fn set_enabled_cloudwatch_logs_exports(
        self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.0.data.borrow_mut().enabled_cloudwatch_logs_exports = Some(v.into());
        self
    }
    #[doc = "Set the field `engine`.\n"]
    pub fn set_engine(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().engine = Some(v.into());
        self
    }
    #[doc = "Set the field `engine_lifecycle_support`.\n"]
    pub fn set_engine_lifecycle_support(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().engine_lifecycle_support = Some(v.into());
        self
    }
    #[doc = "Set the field `engine_version`.\n"]
    pub fn set_engine_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().engine_version = Some(v.into());
        self
    }
    #[doc = "Set the field `final_snapshot_identifier`.\n"]
    pub fn set_final_snapshot_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().final_snapshot_identifier = Some(v.into());
        self
    }
    #[doc = "Set the field `iam_database_authentication_enabled`.\n"]
    pub fn set_iam_database_authentication_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().iam_database_authentication_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `identifier`.\n"]
    pub fn set_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().identifier = Some(v.into());
        self
    }
    #[doc = "Set the field `identifier_prefix`.\n"]
    pub fn set_identifier_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().identifier_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().iops = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = Some(v.into());
        self
    }
    #[doc = "Set the field `license_model`.\n"]
    pub fn set_license_model(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().license_model = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().maintenance_window = Some(v.into());
        self
    }
    #[doc = "Set the field `manage_master_user_password`.\n"]
    pub fn set_manage_master_user_password(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().manage_master_user_password = Some(v.into());
        self
    }
    #[doc = "Set the field `master_user_secret_kms_key_id`.\n"]
    pub fn set_master_user_secret_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().master_user_secret_kms_key_id = Some(v.into());
        self
    }
    #[doc = "Set the field `max_allocated_storage`.\n"]
    pub fn set_max_allocated_storage(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_allocated_storage = Some(v.into());
        self
    }
    #[doc = "Set the field `monitoring_interval`.\n"]
    pub fn set_monitoring_interval(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().monitoring_interval = Some(v.into());
        self
    }
    #[doc = "Set the field `monitoring_role_arn`.\n"]
    pub fn set_monitoring_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().monitoring_role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `multi_az`.\n"]
    pub fn set_multi_az(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().multi_az = Some(v.into());
        self
    }
    #[doc = "Set the field `nchar_character_set_name`.\n"]
    pub fn set_nchar_character_set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().nchar_character_set_name = Some(v.into());
        self
    }
    #[doc = "Set the field `network_type`.\n"]
    pub fn set_network_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network_type = Some(v.into());
        self
    }
    #[doc = "Set the field `option_group_name`.\n"]
    pub fn set_option_group_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().option_group_name = Some(v.into());
        self
    }
    #[doc = "Set the field `parameter_group_name`.\n"]
    pub fn set_parameter_group_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parameter_group_name = Some(v.into());
        self
    }
    #[doc = "Set the field `password`.\n"]
    pub fn set_password(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().password = Some(v.into());
        self
    }
    #[doc = "Set the field `password_wo`.\n"]
    pub fn set_password_wo(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().password_wo = Some(v.into());
        self
    }
    #[doc = "Set the field `password_wo_version`.\n"]
    pub fn set_password_wo_version(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().password_wo_version = Some(v.into());
        self
    }
    #[doc = "Set the field `performance_insights_enabled`.\n"]
    pub fn set_performance_insights_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().performance_insights_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `performance_insights_kms_key_id`.\n"]
    pub fn set_performance_insights_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().performance_insights_kms_key_id = Some(v.into());
        self
    }
    #[doc = "Set the field `performance_insights_retention_period`.\n"]
    pub fn set_performance_insights_retention_period(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0
            .data
            .borrow_mut()
            .performance_insights_retention_period = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = Some(v.into());
        self
    }
    #[doc = "Set the field `publicly_accessible`.\n"]
    pub fn set_publicly_accessible(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().publicly_accessible = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `replica_mode`.\n"]
    pub fn set_replica_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().replica_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `replicate_source_db`.\n"]
    pub fn set_replicate_source_db(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().replicate_source_db = Some(v.into());
        self
    }
    #[doc = "Set the field `skip_final_snapshot`.\n"]
    pub fn set_skip_final_snapshot(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_final_snapshot = Some(v.into());
        self
    }
    #[doc = "Set the field `snapshot_identifier`.\n"]
    pub fn set_snapshot_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().snapshot_identifier = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_encrypted`.\n"]
    pub fn set_storage_encrypted(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().storage_encrypted = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_throughput`.\n"]
    pub fn set_storage_throughput(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().storage_throughput = Some(v.into());
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
    #[doc = "Set the field `timezone`.\n"]
    pub fn set_timezone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().timezone = Some(v.into());
        self
    }
    #[doc = "Set the field `upgrade_storage_config`.\n"]
    pub fn set_upgrade_storage_config(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().upgrade_storage_config = Some(v.into());
        self
    }
    #[doc = "Set the field `username`.\n"]
    pub fn set_username(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().username = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_security_group_ids`.\n"]
    pub fn set_vpc_security_group_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().vpc_security_group_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `blue_green_update`.\n"]
    pub fn set_blue_green_update(
        self,
        v: impl Into<BlockAssignable<DbInstanceBlueGreenUpdateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().blue_green_update = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.blue_green_update = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `restore_to_point_in_time`.\n"]
    pub fn set_restore_to_point_in_time(
        self,
        v: impl Into<BlockAssignable<DbInstanceRestoreToPointInTimeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().restore_to_point_in_time = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.restore_to_point_in_time = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `s3_import`.\n"]
    pub fn set_s3_import(self, v: impl Into<BlockAssignable<DbInstanceS3ImportEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().s3_import = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.s3_import = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DbInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allocated_storage` after provisioning.\n"]
    pub fn allocated_storage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocated_storage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allow_major_version_upgrade` after provisioning.\n"]
    pub fn allow_major_version_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_major_version_upgrade", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `apply_immediately` after provisioning.\n"]
    pub fn apply_immediately(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.apply_immediately", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `auto_minor_version_upgrade` after provisioning.\n"]
    pub fn auto_minor_version_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_minor_version_upgrade", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_retention_period` after provisioning.\n"]
    pub fn backup_retention_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_retention_period", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_target` after provisioning.\n"]
    pub fn backup_target(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_window` after provisioning.\n"]
    pub fn backup_window(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_window", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ca_cert_identifier` after provisioning.\n"]
    pub fn ca_cert_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ca_cert_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `character_set_name` after provisioning.\n"]
    pub fn character_set_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.character_set_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `copy_tags_to_snapshot` after provisioning.\n"]
    pub fn copy_tags_to_snapshot(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.copy_tags_to_snapshot", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_iam_instance_profile` after provisioning.\n"]
    pub fn custom_iam_instance_profile(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_iam_instance_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_owned_ip_enabled` after provisioning.\n"]
    pub fn customer_owned_ip_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_owned_ip_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_insights_mode` after provisioning.\n"]
    pub fn database_insights_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_insights_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_name` after provisioning.\n"]
    pub fn db_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_subnet_group_name` after provisioning.\n"]
    pub fn db_subnet_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_subnet_group_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dedicated_log_volume` after provisioning.\n"]
    pub fn dedicated_log_volume(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dedicated_log_volume", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_automated_backups` after provisioning.\n"]
    pub fn delete_automated_backups(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_automated_backups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_auth_secret_arn` after provisioning.\n"]
    pub fn domain_auth_secret_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_auth_secret_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_dns_ips` after provisioning.\n"]
    pub fn domain_dns_ips(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.domain_dns_ips", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_fqdn` after provisioning.\n"]
    pub fn domain_fqdn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_fqdn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_iam_role_name` after provisioning.\n"]
    pub fn domain_iam_role_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_iam_role_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_ou` after provisioning.\n"]
    pub fn domain_ou(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_ou", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enabled_cloudwatch_logs_exports` after provisioning.\n"]
    pub fn enabled_cloudwatch_logs_exports(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.enabled_cloudwatch_logs_exports", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_lifecycle_support` after provisioning.\n"]
    pub fn engine_lifecycle_support(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_lifecycle_support", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_version_actual` after provisioning.\n"]
    pub fn engine_version_actual(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_version_actual", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `final_snapshot_identifier` after provisioning.\n"]
    pub fn final_snapshot_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.final_snapshot_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hosted_zone_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iam_database_authentication_enabled` after provisioning.\n"]
    pub fn iam_database_authentication_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_database_authentication_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `identifier_prefix` after provisioning.\n"]
    pub fn identifier_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identifier_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_class` after provisioning.\n"]
    pub fn instance_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_class", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iops", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `latest_restorable_time` after provisioning.\n"]
    pub fn latest_restorable_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest_restorable_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `license_model` after provisioning.\n"]
    pub fn license_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.license_model", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `listener_endpoint` after provisioning.\n"]
    pub fn listener_endpoint(&self) -> ListRef<DbInstanceListenerEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.listener_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_window", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `manage_master_user_password` after provisioning.\n"]
    pub fn manage_master_user_password(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.manage_master_user_password", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `master_user_secret` after provisioning.\n"]
    pub fn master_user_secret(&self) -> ListRef<DbInstanceMasterUserSecretElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.master_user_secret", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `master_user_secret_kms_key_id` after provisioning.\n"]
    pub fn master_user_secret_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_user_secret_kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_allocated_storage` after provisioning.\n"]
    pub fn max_allocated_storage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_allocated_storage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monitoring_interval` after provisioning.\n"]
    pub fn monitoring_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monitoring_interval", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monitoring_role_arn` after provisioning.\n"]
    pub fn monitoring_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monitoring_role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multi_az` after provisioning.\n"]
    pub fn multi_az(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multi_az", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `nchar_character_set_name` after provisioning.\n"]
    pub fn nchar_character_set_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nchar_character_set_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_type` after provisioning.\n"]
    pub fn network_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `option_group_name` after provisioning.\n"]
    pub fn option_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.option_group_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parameter_group_name` after provisioning.\n"]
    pub fn parameter_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parameter_group_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `password_wo` after provisioning.\n"]
    pub fn password_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password_wo", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `password_wo_version` after provisioning.\n"]
    pub fn password_wo_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password_wo_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `performance_insights_enabled` after provisioning.\n"]
    pub fn performance_insights_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.performance_insights_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `performance_insights_kms_key_id` after provisioning.\n"]
    pub fn performance_insights_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.performance_insights_kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `performance_insights_retention_period` after provisioning.\n"]
    pub fn performance_insights_retention_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.performance_insights_retention_period",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\n"]
    pub fn publicly_accessible(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.publicly_accessible", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replica_mode` after provisioning.\n"]
    pub fn replica_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replica_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replicas` after provisioning.\n"]
    pub fn replicas(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replicas", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replicate_source_db` after provisioning.\n"]
    pub fn replicate_source_db(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replicate_source_db", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `skip_final_snapshot` after provisioning.\n"]
    pub fn skip_final_snapshot(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_final_snapshot", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_identifier` after provisioning.\n"]
    pub fn snapshot_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshot_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_encrypted` after provisioning.\n"]
    pub fn storage_encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_encrypted", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_throughput` after provisioning.\n"]
    pub fn storage_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_throughput", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_type", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `timezone` after provisioning.\n"]
    pub fn timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timezone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `upgrade_storage_config` after provisioning.\n"]
    pub fn upgrade_storage_config(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.upgrade_storage_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.username", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.vpc_security_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `blue_green_update` after provisioning.\n"]
    pub fn blue_green_update(&self) -> ListRef<DbInstanceBlueGreenUpdateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.blue_green_update", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restore_to_point_in_time` after provisioning.\n"]
    pub fn restore_to_point_in_time(&self) -> ListRef<DbInstanceRestoreToPointInTimeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restore_to_point_in_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_import` after provisioning.\n"]
    pub fn s3_import(&self) -> ListRef<DbInstanceS3ImportElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_import", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DbInstanceTimeoutsElRef {
        DbInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DbInstance {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DbInstance {}
impl ToListMappable for DbInstance {
    type O = ListRef<DbInstanceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DbInstance_ {
    fn extract_resource_type(&self) -> String {
        "aws_db_instance".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDbInstance {
    pub tf_id: String,
    #[doc = ""]
    pub instance_class: PrimField<String>,
}
impl BuildDbInstance {
    pub fn build(self, stack: &mut Stack) -> DbInstance {
        let out = DbInstance(Rc::new(DbInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DbInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allocated_storage: core::default::Default::default(),
                allow_major_version_upgrade: core::default::Default::default(),
                apply_immediately: core::default::Default::default(),
                auto_minor_version_upgrade: core::default::Default::default(),
                availability_zone: core::default::Default::default(),
                backup_retention_period: core::default::Default::default(),
                backup_target: core::default::Default::default(),
                backup_window: core::default::Default::default(),
                ca_cert_identifier: core::default::Default::default(),
                character_set_name: core::default::Default::default(),
                copy_tags_to_snapshot: core::default::Default::default(),
                custom_iam_instance_profile: core::default::Default::default(),
                customer_owned_ip_enabled: core::default::Default::default(),
                database_insights_mode: core::default::Default::default(),
                db_name: core::default::Default::default(),
                db_subnet_group_name: core::default::Default::default(),
                dedicated_log_volume: core::default::Default::default(),
                delete_automated_backups: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                domain: core::default::Default::default(),
                domain_auth_secret_arn: core::default::Default::default(),
                domain_dns_ips: core::default::Default::default(),
                domain_fqdn: core::default::Default::default(),
                domain_iam_role_name: core::default::Default::default(),
                domain_ou: core::default::Default::default(),
                enabled_cloudwatch_logs_exports: core::default::Default::default(),
                engine: core::default::Default::default(),
                engine_lifecycle_support: core::default::Default::default(),
                engine_version: core::default::Default::default(),
                final_snapshot_identifier: core::default::Default::default(),
                iam_database_authentication_enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                identifier: core::default::Default::default(),
                identifier_prefix: core::default::Default::default(),
                instance_class: self.instance_class,
                iops: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                license_model: core::default::Default::default(),
                maintenance_window: core::default::Default::default(),
                manage_master_user_password: core::default::Default::default(),
                master_user_secret_kms_key_id: core::default::Default::default(),
                max_allocated_storage: core::default::Default::default(),
                monitoring_interval: core::default::Default::default(),
                monitoring_role_arn: core::default::Default::default(),
                multi_az: core::default::Default::default(),
                nchar_character_set_name: core::default::Default::default(),
                network_type: core::default::Default::default(),
                option_group_name: core::default::Default::default(),
                parameter_group_name: core::default::Default::default(),
                password: core::default::Default::default(),
                password_wo: core::default::Default::default(),
                password_wo_version: core::default::Default::default(),
                performance_insights_enabled: core::default::Default::default(),
                performance_insights_kms_key_id: core::default::Default::default(),
                performance_insights_retention_period: core::default::Default::default(),
                port: core::default::Default::default(),
                publicly_accessible: core::default::Default::default(),
                region: core::default::Default::default(),
                replica_mode: core::default::Default::default(),
                replicate_source_db: core::default::Default::default(),
                skip_final_snapshot: core::default::Default::default(),
                snapshot_identifier: core::default::Default::default(),
                storage_encrypted: core::default::Default::default(),
                storage_throughput: core::default::Default::default(),
                storage_type: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                timezone: core::default::Default::default(),
                upgrade_storage_config: core::default::Default::default(),
                username: core::default::Default::default(),
                vpc_security_group_ids: core::default::Default::default(),
                blue_green_update: core::default::Default::default(),
                restore_to_point_in_time: core::default::Default::default(),
                s3_import: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DbInstanceRef {
    shared: StackShared,
    base: String,
}
impl Ref for DbInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DbInstanceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allocated_storage` after provisioning.\n"]
    pub fn allocated_storage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocated_storage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allow_major_version_upgrade` after provisioning.\n"]
    pub fn allow_major_version_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_major_version_upgrade", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `apply_immediately` after provisioning.\n"]
    pub fn apply_immediately(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.apply_immediately", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `auto_minor_version_upgrade` after provisioning.\n"]
    pub fn auto_minor_version_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_minor_version_upgrade", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_retention_period` after provisioning.\n"]
    pub fn backup_retention_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_retention_period", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_target` after provisioning.\n"]
    pub fn backup_target(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_window` after provisioning.\n"]
    pub fn backup_window(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_window", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ca_cert_identifier` after provisioning.\n"]
    pub fn ca_cert_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ca_cert_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `character_set_name` after provisioning.\n"]
    pub fn character_set_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.character_set_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `copy_tags_to_snapshot` after provisioning.\n"]
    pub fn copy_tags_to_snapshot(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.copy_tags_to_snapshot", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_iam_instance_profile` after provisioning.\n"]
    pub fn custom_iam_instance_profile(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_iam_instance_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_owned_ip_enabled` after provisioning.\n"]
    pub fn customer_owned_ip_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_owned_ip_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_insights_mode` after provisioning.\n"]
    pub fn database_insights_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_insights_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_name` after provisioning.\n"]
    pub fn db_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_subnet_group_name` after provisioning.\n"]
    pub fn db_subnet_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_subnet_group_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dedicated_log_volume` after provisioning.\n"]
    pub fn dedicated_log_volume(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dedicated_log_volume", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_automated_backups` after provisioning.\n"]
    pub fn delete_automated_backups(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_automated_backups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_auth_secret_arn` after provisioning.\n"]
    pub fn domain_auth_secret_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_auth_secret_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_dns_ips` after provisioning.\n"]
    pub fn domain_dns_ips(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.domain_dns_ips", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_fqdn` after provisioning.\n"]
    pub fn domain_fqdn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_fqdn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_iam_role_name` after provisioning.\n"]
    pub fn domain_iam_role_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_iam_role_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_ou` after provisioning.\n"]
    pub fn domain_ou(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_ou", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enabled_cloudwatch_logs_exports` after provisioning.\n"]
    pub fn enabled_cloudwatch_logs_exports(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.enabled_cloudwatch_logs_exports", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_lifecycle_support` after provisioning.\n"]
    pub fn engine_lifecycle_support(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_lifecycle_support", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_version_actual` after provisioning.\n"]
    pub fn engine_version_actual(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_version_actual", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `final_snapshot_identifier` after provisioning.\n"]
    pub fn final_snapshot_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.final_snapshot_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hosted_zone_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iam_database_authentication_enabled` after provisioning.\n"]
    pub fn iam_database_authentication_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_database_authentication_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `identifier_prefix` after provisioning.\n"]
    pub fn identifier_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identifier_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_class` after provisioning.\n"]
    pub fn instance_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_class", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iops", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `latest_restorable_time` after provisioning.\n"]
    pub fn latest_restorable_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest_restorable_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `license_model` after provisioning.\n"]
    pub fn license_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.license_model", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `listener_endpoint` after provisioning.\n"]
    pub fn listener_endpoint(&self) -> ListRef<DbInstanceListenerEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.listener_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_window", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `manage_master_user_password` after provisioning.\n"]
    pub fn manage_master_user_password(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.manage_master_user_password", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `master_user_secret` after provisioning.\n"]
    pub fn master_user_secret(&self) -> ListRef<DbInstanceMasterUserSecretElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.master_user_secret", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `master_user_secret_kms_key_id` after provisioning.\n"]
    pub fn master_user_secret_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_user_secret_kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_allocated_storage` after provisioning.\n"]
    pub fn max_allocated_storage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_allocated_storage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monitoring_interval` after provisioning.\n"]
    pub fn monitoring_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monitoring_interval", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monitoring_role_arn` after provisioning.\n"]
    pub fn monitoring_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monitoring_role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multi_az` after provisioning.\n"]
    pub fn multi_az(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multi_az", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `nchar_character_set_name` after provisioning.\n"]
    pub fn nchar_character_set_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nchar_character_set_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_type` after provisioning.\n"]
    pub fn network_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `option_group_name` after provisioning.\n"]
    pub fn option_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.option_group_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parameter_group_name` after provisioning.\n"]
    pub fn parameter_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parameter_group_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `password_wo` after provisioning.\n"]
    pub fn password_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password_wo", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `password_wo_version` after provisioning.\n"]
    pub fn password_wo_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password_wo_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `performance_insights_enabled` after provisioning.\n"]
    pub fn performance_insights_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.performance_insights_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `performance_insights_kms_key_id` after provisioning.\n"]
    pub fn performance_insights_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.performance_insights_kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `performance_insights_retention_period` after provisioning.\n"]
    pub fn performance_insights_retention_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.performance_insights_retention_period",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\n"]
    pub fn publicly_accessible(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.publicly_accessible", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replica_mode` after provisioning.\n"]
    pub fn replica_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replica_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replicas` after provisioning.\n"]
    pub fn replicas(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replicas", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replicate_source_db` after provisioning.\n"]
    pub fn replicate_source_db(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replicate_source_db", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `skip_final_snapshot` after provisioning.\n"]
    pub fn skip_final_snapshot(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_final_snapshot", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_identifier` after provisioning.\n"]
    pub fn snapshot_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshot_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_encrypted` after provisioning.\n"]
    pub fn storage_encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_encrypted", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_throughput` after provisioning.\n"]
    pub fn storage_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_throughput", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_type", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `timezone` after provisioning.\n"]
    pub fn timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timezone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `upgrade_storage_config` after provisioning.\n"]
    pub fn upgrade_storage_config(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.upgrade_storage_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.username", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.vpc_security_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `blue_green_update` after provisioning.\n"]
    pub fn blue_green_update(&self) -> ListRef<DbInstanceBlueGreenUpdateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.blue_green_update", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restore_to_point_in_time` after provisioning.\n"]
    pub fn restore_to_point_in_time(&self) -> ListRef<DbInstanceRestoreToPointInTimeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restore_to_point_in_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_import` after provisioning.\n"]
    pub fn s3_import(&self) -> ListRef<DbInstanceS3ImportElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_import", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DbInstanceTimeoutsElRef {
        DbInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DbInstanceListenerEndpointEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hosted_zone_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl DbInstanceListenerEndpointEl {
    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address = Some(v.into());
        self
    }
    #[doc = "Set the field `hosted_zone_id`.\n"]
    pub fn set_hosted_zone_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hosted_zone_id = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}
impl ToListMappable for DbInstanceListenerEndpointEl {
    type O = BlockAssignable<DbInstanceListenerEndpointEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDbInstanceListenerEndpointEl {}
impl BuildDbInstanceListenerEndpointEl {
    pub fn build(self) -> DbInstanceListenerEndpointEl {
        DbInstanceListenerEndpointEl {
            address: core::default::Default::default(),
            hosted_zone_id: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}
pub struct DbInstanceListenerEndpointElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DbInstanceListenerEndpointElRef {
    fn new(shared: StackShared, base: String) -> DbInstanceListenerEndpointElRef {
        DbInstanceListenerEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DbInstanceListenerEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }
    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hosted_zone_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct DbInstanceMasterUserSecretEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_status: Option<PrimField<String>>,
}
impl DbInstanceMasterUserSecretEl {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_arn`.\n"]
    pub fn set_secret_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_status`.\n"]
    pub fn set_secret_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_status = Some(v.into());
        self
    }
}
impl ToListMappable for DbInstanceMasterUserSecretEl {
    type O = BlockAssignable<DbInstanceMasterUserSecretEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDbInstanceMasterUserSecretEl {}
impl BuildDbInstanceMasterUserSecretEl {
    pub fn build(self) -> DbInstanceMasterUserSecretEl {
        DbInstanceMasterUserSecretEl {
            kms_key_id: core::default::Default::default(),
            secret_arn: core::default::Default::default(),
            secret_status: core::default::Default::default(),
        }
    }
}
pub struct DbInstanceMasterUserSecretElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DbInstanceMasterUserSecretElRef {
    fn new(shared: StackShared, base: String) -> DbInstanceMasterUserSecretElRef {
        DbInstanceMasterUserSecretElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DbInstanceMasterUserSecretElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_status` after provisioning.\n"]
    pub fn secret_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_status", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DbInstanceBlueGreenUpdateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DbInstanceBlueGreenUpdateEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DbInstanceBlueGreenUpdateEl {
    type O = BlockAssignable<DbInstanceBlueGreenUpdateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDbInstanceBlueGreenUpdateEl {}
impl BuildDbInstanceBlueGreenUpdateEl {
    pub fn build(self) -> DbInstanceBlueGreenUpdateEl {
        DbInstanceBlueGreenUpdateEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DbInstanceBlueGreenUpdateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DbInstanceBlueGreenUpdateElRef {
    fn new(shared: StackShared, base: String) -> DbInstanceBlueGreenUpdateElRef {
        DbInstanceBlueGreenUpdateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DbInstanceBlueGreenUpdateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DbInstanceRestoreToPointInTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    restore_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_db_instance_automated_backups_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_db_instance_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_dbi_resource_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_latest_restorable_time: Option<PrimField<bool>>,
}
impl DbInstanceRestoreToPointInTimeEl {
    #[doc = "Set the field `restore_time`.\n"]
    pub fn set_restore_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.restore_time = Some(v.into());
        self
    }
    #[doc = "Set the field `source_db_instance_automated_backups_arn`.\n"]
    pub fn set_source_db_instance_automated_backups_arn(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.source_db_instance_automated_backups_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `source_db_instance_identifier`.\n"]
    pub fn set_source_db_instance_identifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_db_instance_identifier = Some(v.into());
        self
    }
    #[doc = "Set the field `source_dbi_resource_id`.\n"]
    pub fn set_source_dbi_resource_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_dbi_resource_id = Some(v.into());
        self
    }
    #[doc = "Set the field `use_latest_restorable_time`.\n"]
    pub fn set_use_latest_restorable_time(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_latest_restorable_time = Some(v.into());
        self
    }
}
impl ToListMappable for DbInstanceRestoreToPointInTimeEl {
    type O = BlockAssignable<DbInstanceRestoreToPointInTimeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDbInstanceRestoreToPointInTimeEl {}
impl BuildDbInstanceRestoreToPointInTimeEl {
    pub fn build(self) -> DbInstanceRestoreToPointInTimeEl {
        DbInstanceRestoreToPointInTimeEl {
            restore_time: core::default::Default::default(),
            source_db_instance_automated_backups_arn: core::default::Default::default(),
            source_db_instance_identifier: core::default::Default::default(),
            source_dbi_resource_id: core::default::Default::default(),
            use_latest_restorable_time: core::default::Default::default(),
        }
    }
}
pub struct DbInstanceRestoreToPointInTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DbInstanceRestoreToPointInTimeElRef {
    fn new(shared: StackShared, base: String) -> DbInstanceRestoreToPointInTimeElRef {
        DbInstanceRestoreToPointInTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DbInstanceRestoreToPointInTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `restore_time` after provisioning.\n"]
    pub fn restore_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.restore_time", self.base))
    }
    #[doc = "Get a reference to the value of field `source_db_instance_automated_backups_arn` after provisioning.\n"]
    pub fn source_db_instance_automated_backups_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_db_instance_automated_backups_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_db_instance_identifier` after provisioning.\n"]
    pub fn source_db_instance_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_db_instance_identifier", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_dbi_resource_id` after provisioning.\n"]
    pub fn source_dbi_resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_dbi_resource_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `use_latest_restorable_time` after provisioning.\n"]
    pub fn use_latest_restorable_time(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_latest_restorable_time", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DbInstanceS3ImportEl {
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_prefix: Option<PrimField<String>>,
    ingestion_role: PrimField<String>,
    source_engine: PrimField<String>,
    source_engine_version: PrimField<String>,
}
impl DbInstanceS3ImportEl {
    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_prefix = Some(v.into());
        self
    }
}
impl ToListMappable for DbInstanceS3ImportEl {
    type O = BlockAssignable<DbInstanceS3ImportEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDbInstanceS3ImportEl {
    #[doc = ""]
    pub bucket_name: PrimField<String>,
    #[doc = ""]
    pub ingestion_role: PrimField<String>,
    #[doc = ""]
    pub source_engine: PrimField<String>,
    #[doc = ""]
    pub source_engine_version: PrimField<String>,
}
impl BuildDbInstanceS3ImportEl {
    pub fn build(self) -> DbInstanceS3ImportEl {
        DbInstanceS3ImportEl {
            bucket_name: self.bucket_name,
            bucket_prefix: core::default::Default::default(),
            ingestion_role: self.ingestion_role,
            source_engine: self.source_engine,
            source_engine_version: self.source_engine_version,
        }
    }
}
pub struct DbInstanceS3ImportElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DbInstanceS3ImportElRef {
    fn new(shared: StackShared, base: String) -> DbInstanceS3ImportElRef {
        DbInstanceS3ImportElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DbInstanceS3ImportElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_prefix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ingestion_role` after provisioning.\n"]
    pub fn ingestion_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ingestion_role", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_engine` after provisioning.\n"]
    pub fn source_engine(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_engine", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_engine_version` after provisioning.\n"]
    pub fn source_engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_engine_version", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DbInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DbInstanceTimeoutsEl {
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
impl ToListMappable for DbInstanceTimeoutsEl {
    type O = BlockAssignable<DbInstanceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDbInstanceTimeoutsEl {}
impl BuildDbInstanceTimeoutsEl {
    pub fn build(self) -> DbInstanceTimeoutsEl {
        DbInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DbInstanceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DbInstanceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DbInstanceTimeoutsElRef {
        DbInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DbInstanceTimeoutsElRef {
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
struct DbInstanceDynamic {
    blue_green_update: Option<DynamicBlock<DbInstanceBlueGreenUpdateEl>>,
    restore_to_point_in_time: Option<DynamicBlock<DbInstanceRestoreToPointInTimeEl>>,
    s3_import: Option<DynamicBlock<DbInstanceS3ImportEl>>,
}
