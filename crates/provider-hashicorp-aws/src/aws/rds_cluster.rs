use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct RdsClusterData {
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
    availability_zones: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backtrack_window: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_retention_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_certificate_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_identifier_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_members: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_scalability_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_tags_to_snapshot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_insights_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_cluster_instance_class: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_cluster_parameter_group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_instance_parameter_group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_subnet_group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_system_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_automated_backups: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_iam_role_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_global_write_forwarding: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_http_endpoint: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_local_write_forwarding: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled_cloudwatch_logs_exports: Option<SetField<PrimField<String>>>,
    engine: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine_lifecycle_support: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    final_snapshot_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    global_cluster_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_database_authentication_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_roles: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manage_master_user_password: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    master_password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    master_password_wo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    master_password_wo_version: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    master_user_secret_kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    master_username: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitoring_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitoring_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performance_insights_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performance_insights_kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performance_insights_retention_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_backup_window: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_maintenance_window: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replication_source_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_final_snapshot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_encrypted: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restore_to_point_in_time: Option<Vec<RdsClusterRestoreToPointInTimeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_import: Option<Vec<RdsClusterS3ImportEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scaling_configuration: Option<Vec<RdsClusterScalingConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serverlessv2_scaling_configuration: Option<Vec<RdsClusterServerlessv2ScalingConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<RdsClusterTimeoutsEl>,
    dynamic: RdsClusterDynamic,
}

struct RdsCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RdsClusterData>,
}

#[derive(Clone)]
pub struct RdsCluster(Rc<RdsCluster_>);

impl RdsCluster {
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

    #[doc = "Set the field `availability_zones`.\n"]
    pub fn set_availability_zones(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().availability_zones = Some(v.into());
        self
    }

    #[doc = "Set the field `backtrack_window`.\n"]
    pub fn set_backtrack_window(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().backtrack_window = Some(v.into());
        self
    }

    #[doc = "Set the field `backup_retention_period`.\n"]
    pub fn set_backup_retention_period(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().backup_retention_period = Some(v.into());
        self
    }

    #[doc = "Set the field `ca_certificate_identifier`.\n"]
    pub fn set_ca_certificate_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ca_certificate_identifier = Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_identifier`.\n"]
    pub fn set_cluster_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cluster_identifier = Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_identifier_prefix`.\n"]
    pub fn set_cluster_identifier_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cluster_identifier_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_members`.\n"]
    pub fn set_cluster_members(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().cluster_members = Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_scalability_type`.\n"]
    pub fn set_cluster_scalability_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cluster_scalability_type = Some(v.into());
        self
    }

    #[doc = "Set the field `copy_tags_to_snapshot`.\n"]
    pub fn set_copy_tags_to_snapshot(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().copy_tags_to_snapshot = Some(v.into());
        self
    }

    #[doc = "Set the field `database_insights_mode`.\n"]
    pub fn set_database_insights_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().database_insights_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `database_name`.\n"]
    pub fn set_database_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().database_name = Some(v.into());
        self
    }

    #[doc = "Set the field `db_cluster_instance_class`.\n"]
    pub fn set_db_cluster_instance_class(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().db_cluster_instance_class = Some(v.into());
        self
    }

    #[doc = "Set the field `db_cluster_parameter_group_name`.\n"]
    pub fn set_db_cluster_parameter_group_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().db_cluster_parameter_group_name = Some(v.into());
        self
    }

    #[doc = "Set the field `db_instance_parameter_group_name`.\n"]
    pub fn set_db_instance_parameter_group_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().db_instance_parameter_group_name = Some(v.into());
        self
    }

    #[doc = "Set the field `db_subnet_group_name`.\n"]
    pub fn set_db_subnet_group_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().db_subnet_group_name = Some(v.into());
        self
    }

    #[doc = "Set the field `db_system_id`.\n"]
    pub fn set_db_system_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().db_system_id = Some(v.into());
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

    #[doc = "Set the field `domain_iam_role_name`.\n"]
    pub fn set_domain_iam_role_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().domain_iam_role_name = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_global_write_forwarding`.\n"]
    pub fn set_enable_global_write_forwarding(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_global_write_forwarding = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_http_endpoint`.\n"]
    pub fn set_enable_http_endpoint(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_http_endpoint = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_local_write_forwarding`.\n"]
    pub fn set_enable_local_write_forwarding(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_local_write_forwarding = Some(v.into());
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

    #[doc = "Set the field `engine_lifecycle_support`.\n"]
    pub fn set_engine_lifecycle_support(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().engine_lifecycle_support = Some(v.into());
        self
    }

    #[doc = "Set the field `engine_mode`.\n"]
    pub fn set_engine_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().engine_mode = Some(v.into());
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

    #[doc = "Set the field `global_cluster_identifier`.\n"]
    pub fn set_global_cluster_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().global_cluster_identifier = Some(v.into());
        self
    }

    #[doc = "Set the field `iam_database_authentication_enabled`.\n"]
    pub fn set_iam_database_authentication_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().iam_database_authentication_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `iam_roles`.\n"]
    pub fn set_iam_roles(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().iam_roles = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
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

    #[doc = "Set the field `manage_master_user_password`.\n"]
    pub fn set_manage_master_user_password(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().manage_master_user_password = Some(v.into());
        self
    }

    #[doc = "Set the field `master_password`.\n"]
    pub fn set_master_password(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().master_password = Some(v.into());
        self
    }

    #[doc = "Set the field `master_password_wo`.\n"]
    pub fn set_master_password_wo(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().master_password_wo = Some(v.into());
        self
    }

    #[doc = "Set the field `master_password_wo_version`.\n"]
    pub fn set_master_password_wo_version(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().master_password_wo_version = Some(v.into());
        self
    }

    #[doc = "Set the field `master_user_secret_kms_key_id`.\n"]
    pub fn set_master_user_secret_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().master_user_secret_kms_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `master_username`.\n"]
    pub fn set_master_username(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().master_username = Some(v.into());
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

    #[doc = "Set the field `network_type`.\n"]
    pub fn set_network_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network_type = Some(v.into());
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

    #[doc = "Set the field `preferred_backup_window`.\n"]
    pub fn set_preferred_backup_window(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().preferred_backup_window = Some(v.into());
        self
    }

    #[doc = "Set the field `preferred_maintenance_window`.\n"]
    pub fn set_preferred_maintenance_window(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().preferred_maintenance_window = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `replication_source_identifier`.\n"]
    pub fn set_replication_source_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().replication_source_identifier = Some(v.into());
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

    #[doc = "Set the field `source_region`.\n"]
    pub fn set_source_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_region = Some(v.into());
        self
    }

    #[doc = "Set the field `storage_encrypted`.\n"]
    pub fn set_storage_encrypted(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().storage_encrypted = Some(v.into());
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

    #[doc = "Set the field `vpc_security_group_ids`.\n"]
    pub fn set_vpc_security_group_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().vpc_security_group_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `restore_to_point_in_time`.\n"]
    pub fn set_restore_to_point_in_time(
        self,
        v: impl Into<BlockAssignable<RdsClusterRestoreToPointInTimeEl>>,
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
    pub fn set_s3_import(self, v: impl Into<BlockAssignable<RdsClusterS3ImportEl>>) -> Self {
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

    #[doc = "Set the field `scaling_configuration`.\n"]
    pub fn set_scaling_configuration(
        self,
        v: impl Into<BlockAssignable<RdsClusterScalingConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scaling_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scaling_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `serverlessv2_scaling_configuration`.\n"]
    pub fn set_serverlessv2_scaling_configuration(
        self,
        v: impl Into<BlockAssignable<RdsClusterServerlessv2ScalingConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().serverlessv2_scaling_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .serverlessv2_scaling_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<RdsClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
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

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.availability_zones", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `backtrack_window` after provisioning.\n"]
    pub fn backtrack_window(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backtrack_window", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `backup_retention_period` after provisioning.\n"]
    pub fn backup_retention_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_retention_period", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ca_certificate_identifier` after provisioning.\n"]
    pub fn ca_certificate_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ca_certificate_identifier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ca_certificate_valid_till` after provisioning.\n"]
    pub fn ca_certificate_valid_till(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ca_certificate_valid_till", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_identifier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_identifier_prefix` after provisioning.\n"]
    pub fn cluster_identifier_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_identifier_prefix", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_members` after provisioning.\n"]
    pub fn cluster_members(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.cluster_members", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_resource_id` after provisioning.\n"]
    pub fn cluster_resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_resource_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_scalability_type` after provisioning.\n"]
    pub fn cluster_scalability_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_scalability_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `copy_tags_to_snapshot` after provisioning.\n"]
    pub fn copy_tags_to_snapshot(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.copy_tags_to_snapshot", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `database_insights_mode` after provisioning.\n"]
    pub fn database_insights_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_insights_mode", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `db_cluster_instance_class` after provisioning.\n"]
    pub fn db_cluster_instance_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_cluster_instance_class", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `db_cluster_parameter_group_name` after provisioning.\n"]
    pub fn db_cluster_parameter_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_cluster_parameter_group_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `db_instance_parameter_group_name` after provisioning.\n"]
    pub fn db_instance_parameter_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_instance_parameter_group_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `db_subnet_group_name` after provisioning.\n"]
    pub fn db_subnet_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_subnet_group_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `db_system_id` after provisioning.\n"]
    pub fn db_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_system_id", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `domain_iam_role_name` after provisioning.\n"]
    pub fn domain_iam_role_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_iam_role_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_global_write_forwarding` after provisioning.\n"]
    pub fn enable_global_write_forwarding(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_global_write_forwarding", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_http_endpoint` after provisioning.\n"]
    pub fn enable_http_endpoint(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_http_endpoint", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_local_write_forwarding` after provisioning.\n"]
    pub fn enable_local_write_forwarding(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_local_write_forwarding", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `engine_mode` after provisioning.\n"]
    pub fn engine_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_mode", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `global_cluster_identifier` after provisioning.\n"]
    pub fn global_cluster_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.global_cluster_identifier", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `iam_roles` after provisioning.\n"]
    pub fn iam_roles(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.iam_roles", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `manage_master_user_password` after provisioning.\n"]
    pub fn manage_master_user_password(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.manage_master_user_password", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `master_password` after provisioning.\n"]
    pub fn master_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_password", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `master_password_wo` after provisioning.\n"]
    pub fn master_password_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_password_wo", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `master_password_wo_version` after provisioning.\n"]
    pub fn master_password_wo_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_password_wo_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `master_user_secret` after provisioning.\n"]
    pub fn master_user_secret(&self) -> ListRef<RdsClusterMasterUserSecretElRef> {
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

    #[doc = "Get a reference to the value of field `master_username` after provisioning.\n"]
    pub fn master_username(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_username", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `network_type` after provisioning.\n"]
    pub fn network_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_type", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `preferred_backup_window` after provisioning.\n"]
    pub fn preferred_backup_window(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preferred_backup_window", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `preferred_maintenance_window` after provisioning.\n"]
    pub fn preferred_maintenance_window(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preferred_maintenance_window", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `reader_endpoint` after provisioning.\n"]
    pub fn reader_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reader_endpoint", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `replication_source_identifier` after provisioning.\n"]
    pub fn replication_source_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_source_identifier", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `source_region` after provisioning.\n"]
    pub fn source_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `storage_encrypted` after provisioning.\n"]
    pub fn storage_encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_encrypted", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.vpc_security_group_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `restore_to_point_in_time` after provisioning.\n"]
    pub fn restore_to_point_in_time(&self) -> ListRef<RdsClusterRestoreToPointInTimeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restore_to_point_in_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `s3_import` after provisioning.\n"]
    pub fn s3_import(&self) -> ListRef<RdsClusterS3ImportElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_import", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scaling_configuration` after provisioning.\n"]
    pub fn scaling_configuration(&self) -> ListRef<RdsClusterScalingConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scaling_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `serverlessv2_scaling_configuration` after provisioning.\n"]
    pub fn serverlessv2_scaling_configuration(
        &self,
    ) -> ListRef<RdsClusterServerlessv2ScalingConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.serverlessv2_scaling_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RdsClusterTimeoutsElRef {
        RdsClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for RdsCluster {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for RdsCluster {}

impl ToListMappable for RdsCluster {
    type O = ListRef<RdsClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for RdsCluster_ {
    fn extract_resource_type(&self) -> String {
        "aws_rds_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRdsCluster {
    pub tf_id: String,
    #[doc = ""]
    pub engine: PrimField<String>,
}

impl BuildRdsCluster {
    pub fn build(self, stack: &mut Stack) -> RdsCluster {
        let out = RdsCluster(Rc::new(RdsCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RdsClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allocated_storage: core::default::Default::default(),
                allow_major_version_upgrade: core::default::Default::default(),
                apply_immediately: core::default::Default::default(),
                availability_zones: core::default::Default::default(),
                backtrack_window: core::default::Default::default(),
                backup_retention_period: core::default::Default::default(),
                ca_certificate_identifier: core::default::Default::default(),
                cluster_identifier: core::default::Default::default(),
                cluster_identifier_prefix: core::default::Default::default(),
                cluster_members: core::default::Default::default(),
                cluster_scalability_type: core::default::Default::default(),
                copy_tags_to_snapshot: core::default::Default::default(),
                database_insights_mode: core::default::Default::default(),
                database_name: core::default::Default::default(),
                db_cluster_instance_class: core::default::Default::default(),
                db_cluster_parameter_group_name: core::default::Default::default(),
                db_instance_parameter_group_name: core::default::Default::default(),
                db_subnet_group_name: core::default::Default::default(),
                db_system_id: core::default::Default::default(),
                delete_automated_backups: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                domain: core::default::Default::default(),
                domain_iam_role_name: core::default::Default::default(),
                enable_global_write_forwarding: core::default::Default::default(),
                enable_http_endpoint: core::default::Default::default(),
                enable_local_write_forwarding: core::default::Default::default(),
                enabled_cloudwatch_logs_exports: core::default::Default::default(),
                engine: self.engine,
                engine_lifecycle_support: core::default::Default::default(),
                engine_mode: core::default::Default::default(),
                engine_version: core::default::Default::default(),
                final_snapshot_identifier: core::default::Default::default(),
                global_cluster_identifier: core::default::Default::default(),
                iam_database_authentication_enabled: core::default::Default::default(),
                iam_roles: core::default::Default::default(),
                id: core::default::Default::default(),
                iops: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                manage_master_user_password: core::default::Default::default(),
                master_password: core::default::Default::default(),
                master_password_wo: core::default::Default::default(),
                master_password_wo_version: core::default::Default::default(),
                master_user_secret_kms_key_id: core::default::Default::default(),
                master_username: core::default::Default::default(),
                monitoring_interval: core::default::Default::default(),
                monitoring_role_arn: core::default::Default::default(),
                network_type: core::default::Default::default(),
                performance_insights_enabled: core::default::Default::default(),
                performance_insights_kms_key_id: core::default::Default::default(),
                performance_insights_retention_period: core::default::Default::default(),
                port: core::default::Default::default(),
                preferred_backup_window: core::default::Default::default(),
                preferred_maintenance_window: core::default::Default::default(),
                region: core::default::Default::default(),
                replication_source_identifier: core::default::Default::default(),
                skip_final_snapshot: core::default::Default::default(),
                snapshot_identifier: core::default::Default::default(),
                source_region: core::default::Default::default(),
                storage_encrypted: core::default::Default::default(),
                storage_type: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                vpc_security_group_ids: core::default::Default::default(),
                restore_to_point_in_time: core::default::Default::default(),
                s3_import: core::default::Default::default(),
                scaling_configuration: core::default::Default::default(),
                serverlessv2_scaling_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RdsClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for RdsClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl RdsClusterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.availability_zones", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `backtrack_window` after provisioning.\n"]
    pub fn backtrack_window(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backtrack_window", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `backup_retention_period` after provisioning.\n"]
    pub fn backup_retention_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_retention_period", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ca_certificate_identifier` after provisioning.\n"]
    pub fn ca_certificate_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ca_certificate_identifier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ca_certificate_valid_till` after provisioning.\n"]
    pub fn ca_certificate_valid_till(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ca_certificate_valid_till", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_identifier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_identifier_prefix` after provisioning.\n"]
    pub fn cluster_identifier_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_identifier_prefix", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_members` after provisioning.\n"]
    pub fn cluster_members(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.cluster_members", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_resource_id` after provisioning.\n"]
    pub fn cluster_resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_resource_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_scalability_type` after provisioning.\n"]
    pub fn cluster_scalability_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_scalability_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `copy_tags_to_snapshot` after provisioning.\n"]
    pub fn copy_tags_to_snapshot(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.copy_tags_to_snapshot", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `database_insights_mode` after provisioning.\n"]
    pub fn database_insights_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_insights_mode", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `db_cluster_instance_class` after provisioning.\n"]
    pub fn db_cluster_instance_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_cluster_instance_class", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `db_cluster_parameter_group_name` after provisioning.\n"]
    pub fn db_cluster_parameter_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_cluster_parameter_group_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `db_instance_parameter_group_name` after provisioning.\n"]
    pub fn db_instance_parameter_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_instance_parameter_group_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `db_subnet_group_name` after provisioning.\n"]
    pub fn db_subnet_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_subnet_group_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `db_system_id` after provisioning.\n"]
    pub fn db_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_system_id", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `domain_iam_role_name` after provisioning.\n"]
    pub fn domain_iam_role_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_iam_role_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_global_write_forwarding` after provisioning.\n"]
    pub fn enable_global_write_forwarding(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_global_write_forwarding", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_http_endpoint` after provisioning.\n"]
    pub fn enable_http_endpoint(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_http_endpoint", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_local_write_forwarding` after provisioning.\n"]
    pub fn enable_local_write_forwarding(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_local_write_forwarding", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `engine_mode` after provisioning.\n"]
    pub fn engine_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_mode", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `global_cluster_identifier` after provisioning.\n"]
    pub fn global_cluster_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.global_cluster_identifier", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `iam_roles` after provisioning.\n"]
    pub fn iam_roles(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.iam_roles", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `manage_master_user_password` after provisioning.\n"]
    pub fn manage_master_user_password(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.manage_master_user_password", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `master_password` after provisioning.\n"]
    pub fn master_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_password", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `master_password_wo` after provisioning.\n"]
    pub fn master_password_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_password_wo", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `master_password_wo_version` after provisioning.\n"]
    pub fn master_password_wo_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_password_wo_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `master_user_secret` after provisioning.\n"]
    pub fn master_user_secret(&self) -> ListRef<RdsClusterMasterUserSecretElRef> {
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

    #[doc = "Get a reference to the value of field `master_username` after provisioning.\n"]
    pub fn master_username(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_username", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `network_type` after provisioning.\n"]
    pub fn network_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_type", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `preferred_backup_window` after provisioning.\n"]
    pub fn preferred_backup_window(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preferred_backup_window", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `preferred_maintenance_window` after provisioning.\n"]
    pub fn preferred_maintenance_window(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preferred_maintenance_window", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `reader_endpoint` after provisioning.\n"]
    pub fn reader_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reader_endpoint", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `replication_source_identifier` after provisioning.\n"]
    pub fn replication_source_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_source_identifier", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `source_region` after provisioning.\n"]
    pub fn source_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `storage_encrypted` after provisioning.\n"]
    pub fn storage_encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_encrypted", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.vpc_security_group_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `restore_to_point_in_time` after provisioning.\n"]
    pub fn restore_to_point_in_time(&self) -> ListRef<RdsClusterRestoreToPointInTimeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restore_to_point_in_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `s3_import` after provisioning.\n"]
    pub fn s3_import(&self) -> ListRef<RdsClusterS3ImportElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_import", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scaling_configuration` after provisioning.\n"]
    pub fn scaling_configuration(&self) -> ListRef<RdsClusterScalingConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scaling_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `serverlessv2_scaling_configuration` after provisioning.\n"]
    pub fn serverlessv2_scaling_configuration(
        &self,
    ) -> ListRef<RdsClusterServerlessv2ScalingConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.serverlessv2_scaling_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RdsClusterTimeoutsElRef {
        RdsClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct RdsClusterMasterUserSecretEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_status: Option<PrimField<String>>,
}

impl RdsClusterMasterUserSecretEl {
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

impl ToListMappable for RdsClusterMasterUserSecretEl {
    type O = BlockAssignable<RdsClusterMasterUserSecretEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRdsClusterMasterUserSecretEl {}

impl BuildRdsClusterMasterUserSecretEl {
    pub fn build(self) -> RdsClusterMasterUserSecretEl {
        RdsClusterMasterUserSecretEl {
            kms_key_id: core::default::Default::default(),
            secret_arn: core::default::Default::default(),
            secret_status: core::default::Default::default(),
        }
    }
}

pub struct RdsClusterMasterUserSecretElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RdsClusterMasterUserSecretElRef {
    fn new(shared: StackShared, base: String) -> RdsClusterMasterUserSecretElRef {
        RdsClusterMasterUserSecretElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RdsClusterMasterUserSecretElRef {
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
pub struct RdsClusterRestoreToPointInTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    restore_to_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restore_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_cluster_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_cluster_resource_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_latest_restorable_time: Option<PrimField<bool>>,
}

impl RdsClusterRestoreToPointInTimeEl {
    #[doc = "Set the field `restore_to_time`.\n"]
    pub fn set_restore_to_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.restore_to_time = Some(v.into());
        self
    }

    #[doc = "Set the field `restore_type`.\n"]
    pub fn set_restore_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.restore_type = Some(v.into());
        self
    }

    #[doc = "Set the field `source_cluster_identifier`.\n"]
    pub fn set_source_cluster_identifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_cluster_identifier = Some(v.into());
        self
    }

    #[doc = "Set the field `source_cluster_resource_id`.\n"]
    pub fn set_source_cluster_resource_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_cluster_resource_id = Some(v.into());
        self
    }

    #[doc = "Set the field `use_latest_restorable_time`.\n"]
    pub fn set_use_latest_restorable_time(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_latest_restorable_time = Some(v.into());
        self
    }
}

impl ToListMappable for RdsClusterRestoreToPointInTimeEl {
    type O = BlockAssignable<RdsClusterRestoreToPointInTimeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRdsClusterRestoreToPointInTimeEl {}

impl BuildRdsClusterRestoreToPointInTimeEl {
    pub fn build(self) -> RdsClusterRestoreToPointInTimeEl {
        RdsClusterRestoreToPointInTimeEl {
            restore_to_time: core::default::Default::default(),
            restore_type: core::default::Default::default(),
            source_cluster_identifier: core::default::Default::default(),
            source_cluster_resource_id: core::default::Default::default(),
            use_latest_restorable_time: core::default::Default::default(),
        }
    }
}

pub struct RdsClusterRestoreToPointInTimeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RdsClusterRestoreToPointInTimeElRef {
    fn new(shared: StackShared, base: String) -> RdsClusterRestoreToPointInTimeElRef {
        RdsClusterRestoreToPointInTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RdsClusterRestoreToPointInTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `restore_to_time` after provisioning.\n"]
    pub fn restore_to_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.restore_to_time", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `restore_type` after provisioning.\n"]
    pub fn restore_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.restore_type", self.base))
    }

    #[doc = "Get a reference to the value of field `source_cluster_identifier` after provisioning.\n"]
    pub fn source_cluster_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_cluster_identifier", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `source_cluster_resource_id` after provisioning.\n"]
    pub fn source_cluster_resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_cluster_resource_id", self.base),
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
pub struct RdsClusterS3ImportEl {
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_prefix: Option<PrimField<String>>,
    ingestion_role: PrimField<String>,
    source_engine: PrimField<String>,
    source_engine_version: PrimField<String>,
}

impl RdsClusterS3ImportEl {
    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_prefix = Some(v.into());
        self
    }
}

impl ToListMappable for RdsClusterS3ImportEl {
    type O = BlockAssignable<RdsClusterS3ImportEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRdsClusterS3ImportEl {
    #[doc = ""]
    pub bucket_name: PrimField<String>,
    #[doc = ""]
    pub ingestion_role: PrimField<String>,
    #[doc = ""]
    pub source_engine: PrimField<String>,
    #[doc = ""]
    pub source_engine_version: PrimField<String>,
}

impl BuildRdsClusterS3ImportEl {
    pub fn build(self) -> RdsClusterS3ImportEl {
        RdsClusterS3ImportEl {
            bucket_name: self.bucket_name,
            bucket_prefix: core::default::Default::default(),
            ingestion_role: self.ingestion_role,
            source_engine: self.source_engine,
            source_engine_version: self.source_engine_version,
        }
    }
}

pub struct RdsClusterS3ImportElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RdsClusterS3ImportElRef {
    fn new(shared: StackShared, base: String) -> RdsClusterS3ImportElRef {
        RdsClusterS3ImportElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RdsClusterS3ImportElRef {
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
pub struct RdsClusterScalingConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_pause: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds_before_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds_until_auto_pause: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_action: Option<PrimField<String>>,
}

impl RdsClusterScalingConfigurationEl {
    #[doc = "Set the field `auto_pause`.\n"]
    pub fn set_auto_pause(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_pause = Some(v.into());
        self
    }

    #[doc = "Set the field `max_capacity`.\n"]
    pub fn set_max_capacity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `min_capacity`.\n"]
    pub fn set_min_capacity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `seconds_before_timeout`.\n"]
    pub fn set_seconds_before_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds_before_timeout = Some(v.into());
        self
    }

    #[doc = "Set the field `seconds_until_auto_pause`.\n"]
    pub fn set_seconds_until_auto_pause(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds_until_auto_pause = Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_action`.\n"]
    pub fn set_timeout_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timeout_action = Some(v.into());
        self
    }
}

impl ToListMappable for RdsClusterScalingConfigurationEl {
    type O = BlockAssignable<RdsClusterScalingConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRdsClusterScalingConfigurationEl {}

impl BuildRdsClusterScalingConfigurationEl {
    pub fn build(self) -> RdsClusterScalingConfigurationEl {
        RdsClusterScalingConfigurationEl {
            auto_pause: core::default::Default::default(),
            max_capacity: core::default::Default::default(),
            min_capacity: core::default::Default::default(),
            seconds_before_timeout: core::default::Default::default(),
            seconds_until_auto_pause: core::default::Default::default(),
            timeout_action: core::default::Default::default(),
        }
    }
}

pub struct RdsClusterScalingConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RdsClusterScalingConfigurationElRef {
    fn new(shared: StackShared, base: String) -> RdsClusterScalingConfigurationElRef {
        RdsClusterScalingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RdsClusterScalingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_pause` after provisioning.\n"]
    pub fn auto_pause(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_pause", self.base))
    }

    #[doc = "Get a reference to the value of field `max_capacity` after provisioning.\n"]
    pub fn max_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `min_capacity` after provisioning.\n"]
    pub fn min_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `seconds_before_timeout` after provisioning.\n"]
    pub fn seconds_before_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.seconds_before_timeout", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `seconds_until_auto_pause` after provisioning.\n"]
    pub fn seconds_until_auto_pause(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.seconds_until_auto_pause", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `timeout_action` after provisioning.\n"]
    pub fn timeout_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_action", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct RdsClusterServerlessv2ScalingConfigurationEl {
    max_capacity: PrimField<f64>,
    min_capacity: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds_until_auto_pause: Option<PrimField<f64>>,
}

impl RdsClusterServerlessv2ScalingConfigurationEl {
    #[doc = "Set the field `seconds_until_auto_pause`.\n"]
    pub fn set_seconds_until_auto_pause(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds_until_auto_pause = Some(v.into());
        self
    }
}

impl ToListMappable for RdsClusterServerlessv2ScalingConfigurationEl {
    type O = BlockAssignable<RdsClusterServerlessv2ScalingConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRdsClusterServerlessv2ScalingConfigurationEl {
    #[doc = ""]
    pub max_capacity: PrimField<f64>,
    #[doc = ""]
    pub min_capacity: PrimField<f64>,
}

impl BuildRdsClusterServerlessv2ScalingConfigurationEl {
    pub fn build(self) -> RdsClusterServerlessv2ScalingConfigurationEl {
        RdsClusterServerlessv2ScalingConfigurationEl {
            max_capacity: self.max_capacity,
            min_capacity: self.min_capacity,
            seconds_until_auto_pause: core::default::Default::default(),
        }
    }
}

pub struct RdsClusterServerlessv2ScalingConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RdsClusterServerlessv2ScalingConfigurationElRef {
    fn new(shared: StackShared, base: String) -> RdsClusterServerlessv2ScalingConfigurationElRef {
        RdsClusterServerlessv2ScalingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RdsClusterServerlessv2ScalingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_capacity` after provisioning.\n"]
    pub fn max_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `min_capacity` after provisioning.\n"]
    pub fn min_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `seconds_until_auto_pause` after provisioning.\n"]
    pub fn seconds_until_auto_pause(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.seconds_until_auto_pause", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct RdsClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl RdsClusterTimeoutsEl {
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

impl ToListMappable for RdsClusterTimeoutsEl {
    type O = BlockAssignable<RdsClusterTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRdsClusterTimeoutsEl {}

impl BuildRdsClusterTimeoutsEl {
    pub fn build(self) -> RdsClusterTimeoutsEl {
        RdsClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct RdsClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RdsClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> RdsClusterTimeoutsElRef {
        RdsClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RdsClusterTimeoutsElRef {
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
struct RdsClusterDynamic {
    restore_to_point_in_time: Option<DynamicBlock<RdsClusterRestoreToPointInTimeEl>>,
    s3_import: Option<DynamicBlock<RdsClusterS3ImportEl>>,
    scaling_configuration: Option<DynamicBlock<RdsClusterScalingConfigurationEl>>,
    serverlessv2_scaling_configuration:
        Option<DynamicBlock<RdsClusterServerlessv2ScalingConfigurationEl>>,
}
