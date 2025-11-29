use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DocdbClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_major_version_upgrade: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apply_immediately: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zones: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_retention_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_identifier_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_members: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_cluster_parameter_group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_subnet_group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled_cloudwatch_logs_exports: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    final_snapshot_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    global_cluster_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
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
    master_username: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_backup_window: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_maintenance_window: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_final_snapshot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_identifier: Option<PrimField<String>>,
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
    restore_to_point_in_time: Option<Vec<DocdbClusterRestoreToPointInTimeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serverless_v2_scaling_configuration: Option<Vec<DocdbClusterServerlessV2ScalingConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DocdbClusterTimeoutsEl>,
    dynamic: DocdbClusterDynamic,
}

struct DocdbCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DocdbClusterData>,
}

#[derive(Clone)]
pub struct DocdbCluster(Rc<DocdbCluster_>);

impl DocdbCluster {
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

    #[doc = "Set the field `backup_retention_period`.\n"]
    pub fn set_backup_retention_period(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().backup_retention_period = Some(v.into());
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

    #[doc = "Set the field `db_cluster_parameter_group_name`.\n"]
    pub fn set_db_cluster_parameter_group_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().db_cluster_parameter_group_name = Some(v.into());
        self
    }

    #[doc = "Set the field `db_subnet_group_name`.\n"]
    pub fn set_db_subnet_group_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().db_subnet_group_name = Some(v.into());
        self
    }

    #[doc = "Set the field `deletion_protection`.\n"]
    pub fn set_deletion_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = Some(v.into());
        self
    }

    #[doc = "Set the field `enabled_cloudwatch_logs_exports`.\n"]
    pub fn set_enabled_cloudwatch_logs_exports(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().enabled_cloudwatch_logs_exports = Some(v.into());
        self
    }

    #[doc = "Set the field `engine`.\n"]
    pub fn set_engine(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().engine = Some(v.into());
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

    #[doc = "Set the field `master_username`.\n"]
    pub fn set_master_username(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().master_username = Some(v.into());
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
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
        v: impl Into<BlockAssignable<DocdbClusterRestoreToPointInTimeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().restore_to_point_in_time = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.restore_to_point_in_time = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `serverless_v2_scaling_configuration`.\n"]
    pub fn set_serverless_v2_scaling_configuration(
        self,
        v: impl Into<BlockAssignable<DocdbClusterServerlessV2ScalingConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().serverless_v2_scaling_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.serverless_v2_scaling_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DocdbClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `allow_major_version_upgrade` after provisioning.\n"]
    pub fn allow_major_version_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_major_version_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `apply_immediately` after provisioning.\n"]
    pub fn apply_immediately(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.apply_immediately", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_retention_period` after provisioning.\n"]
    pub fn backup_retention_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup_retention_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_identifier_prefix` after provisioning.\n"]
    pub fn cluster_identifier_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_members` after provisioning.\n"]
    pub fn cluster_members(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.cluster_members", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_resource_id` after provisioning.\n"]
    pub fn cluster_resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_resource_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_cluster_parameter_group_name` after provisioning.\n"]
    pub fn db_cluster_parameter_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_cluster_parameter_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_subnet_group_name` after provisioning.\n"]
    pub fn db_subnet_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_subnet_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled_cloudwatch_logs_exports` after provisioning.\n"]
    pub fn enabled_cloudwatch_logs_exports(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.enabled_cloudwatch_logs_exports", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `final_snapshot_identifier` after provisioning.\n"]
    pub fn final_snapshot_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.final_snapshot_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `global_cluster_identifier` after provisioning.\n"]
    pub fn global_cluster_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.global_cluster_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `manage_master_user_password` after provisioning.\n"]
    pub fn manage_master_user_password(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.manage_master_user_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_password` after provisioning.\n"]
    pub fn master_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_password_wo` after provisioning.\n"]
    pub fn master_password_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_password_wo", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_password_wo_version` after provisioning.\n"]
    pub fn master_password_wo_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_password_wo_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_user_secret` after provisioning.\n"]
    pub fn master_user_secret(&self) -> ListRef<DocdbClusterMasterUserSecretElRef> {
        ListRef::new(self.shared().clone(), format!("{}.master_user_secret", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_username` after provisioning.\n"]
    pub fn master_username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_username", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_backup_window` after provisioning.\n"]
    pub fn preferred_backup_window(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preferred_backup_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_maintenance_window` after provisioning.\n"]
    pub fn preferred_maintenance_window(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preferred_maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reader_endpoint` after provisioning.\n"]
    pub fn reader_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reader_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skip_final_snapshot` after provisioning.\n"]
    pub fn skip_final_snapshot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.skip_final_snapshot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_identifier` after provisioning.\n"]
    pub fn snapshot_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_encrypted` after provisioning.\n"]
    pub fn storage_encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_encrypted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_to_point_in_time` after provisioning.\n"]
    pub fn restore_to_point_in_time(&self) -> ListRef<DocdbClusterRestoreToPointInTimeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.restore_to_point_in_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `serverless_v2_scaling_configuration` after provisioning.\n"]
    pub fn serverless_v2_scaling_configuration(&self) -> ListRef<DocdbClusterServerlessV2ScalingConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.serverless_v2_scaling_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DocdbClusterTimeoutsElRef {
        DocdbClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for DocdbCluster {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DocdbCluster { }

impl ToListMappable for DocdbCluster {
    type O = ListRef<DocdbClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DocdbCluster_ {
    fn extract_resource_type(&self) -> String {
        "aws_docdb_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDocdbCluster {
    pub tf_id: String,
}

impl BuildDocdbCluster {
    pub fn build(self, stack: &mut Stack) -> DocdbCluster {
        let out = DocdbCluster(Rc::new(DocdbCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DocdbClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allow_major_version_upgrade: core::default::Default::default(),
                apply_immediately: core::default::Default::default(),
                availability_zones: core::default::Default::default(),
                backup_retention_period: core::default::Default::default(),
                cluster_identifier: core::default::Default::default(),
                cluster_identifier_prefix: core::default::Default::default(),
                cluster_members: core::default::Default::default(),
                db_cluster_parameter_group_name: core::default::Default::default(),
                db_subnet_group_name: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                enabled_cloudwatch_logs_exports: core::default::Default::default(),
                engine: core::default::Default::default(),
                engine_version: core::default::Default::default(),
                final_snapshot_identifier: core::default::Default::default(),
                global_cluster_identifier: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                manage_master_user_password: core::default::Default::default(),
                master_password: core::default::Default::default(),
                master_password_wo: core::default::Default::default(),
                master_password_wo_version: core::default::Default::default(),
                master_username: core::default::Default::default(),
                port: core::default::Default::default(),
                preferred_backup_window: core::default::Default::default(),
                preferred_maintenance_window: core::default::Default::default(),
                region: core::default::Default::default(),
                skip_final_snapshot: core::default::Default::default(),
                snapshot_identifier: core::default::Default::default(),
                storage_encrypted: core::default::Default::default(),
                storage_type: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                vpc_security_group_ids: core::default::Default::default(),
                restore_to_point_in_time: core::default::Default::default(),
                serverless_v2_scaling_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DocdbClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for DocdbClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DocdbClusterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_major_version_upgrade` after provisioning.\n"]
    pub fn allow_major_version_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_major_version_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `apply_immediately` after provisioning.\n"]
    pub fn apply_immediately(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.apply_immediately", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_retention_period` after provisioning.\n"]
    pub fn backup_retention_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup_retention_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_identifier_prefix` after provisioning.\n"]
    pub fn cluster_identifier_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_members` after provisioning.\n"]
    pub fn cluster_members(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.cluster_members", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_resource_id` after provisioning.\n"]
    pub fn cluster_resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_resource_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_cluster_parameter_group_name` after provisioning.\n"]
    pub fn db_cluster_parameter_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_cluster_parameter_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_subnet_group_name` after provisioning.\n"]
    pub fn db_subnet_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_subnet_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled_cloudwatch_logs_exports` after provisioning.\n"]
    pub fn enabled_cloudwatch_logs_exports(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.enabled_cloudwatch_logs_exports", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `final_snapshot_identifier` after provisioning.\n"]
    pub fn final_snapshot_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.final_snapshot_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `global_cluster_identifier` after provisioning.\n"]
    pub fn global_cluster_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.global_cluster_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `manage_master_user_password` after provisioning.\n"]
    pub fn manage_master_user_password(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.manage_master_user_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_password` after provisioning.\n"]
    pub fn master_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_password_wo` after provisioning.\n"]
    pub fn master_password_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_password_wo", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_password_wo_version` after provisioning.\n"]
    pub fn master_password_wo_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_password_wo_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_user_secret` after provisioning.\n"]
    pub fn master_user_secret(&self) -> ListRef<DocdbClusterMasterUserSecretElRef> {
        ListRef::new(self.shared().clone(), format!("{}.master_user_secret", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_username` after provisioning.\n"]
    pub fn master_username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_username", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_backup_window` after provisioning.\n"]
    pub fn preferred_backup_window(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preferred_backup_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_maintenance_window` after provisioning.\n"]
    pub fn preferred_maintenance_window(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preferred_maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reader_endpoint` after provisioning.\n"]
    pub fn reader_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reader_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skip_final_snapshot` after provisioning.\n"]
    pub fn skip_final_snapshot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.skip_final_snapshot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_identifier` after provisioning.\n"]
    pub fn snapshot_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_encrypted` after provisioning.\n"]
    pub fn storage_encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_encrypted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_to_point_in_time` after provisioning.\n"]
    pub fn restore_to_point_in_time(&self) -> ListRef<DocdbClusterRestoreToPointInTimeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.restore_to_point_in_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `serverless_v2_scaling_configuration` after provisioning.\n"]
    pub fn serverless_v2_scaling_configuration(&self) -> ListRef<DocdbClusterServerlessV2ScalingConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.serverless_v2_scaling_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DocdbClusterTimeoutsElRef {
        DocdbClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DocdbClusterMasterUserSecretEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_status: Option<PrimField<String>>,
}

impl DocdbClusterMasterUserSecretEl {
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

impl ToListMappable for DocdbClusterMasterUserSecretEl {
    type O = BlockAssignable<DocdbClusterMasterUserSecretEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocdbClusterMasterUserSecretEl {}

impl BuildDocdbClusterMasterUserSecretEl {
    pub fn build(self) -> DocdbClusterMasterUserSecretEl {
        DocdbClusterMasterUserSecretEl {
            kms_key_id: core::default::Default::default(),
            secret_arn: core::default::Default::default(),
            secret_status: core::default::Default::default(),
        }
    }
}

pub struct DocdbClusterMasterUserSecretElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DocdbClusterMasterUserSecretElRef {
    fn new(shared: StackShared, base: String) -> DocdbClusterMasterUserSecretElRef {
        DocdbClusterMasterUserSecretElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocdbClusterMasterUserSecretElRef {
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
        PrimExpr::new(self.shared().clone(), format!("{}.secret_status", self.base))
    }
}

#[derive(Serialize)]
pub struct DocdbClusterRestoreToPointInTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    restore_to_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restore_type: Option<PrimField<String>>,
    source_cluster_identifier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_latest_restorable_time: Option<PrimField<bool>>,
}

impl DocdbClusterRestoreToPointInTimeEl {
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

    #[doc = "Set the field `use_latest_restorable_time`.\n"]
    pub fn set_use_latest_restorable_time(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_latest_restorable_time = Some(v.into());
        self
    }
}

impl ToListMappable for DocdbClusterRestoreToPointInTimeEl {
    type O = BlockAssignable<DocdbClusterRestoreToPointInTimeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocdbClusterRestoreToPointInTimeEl {
    #[doc = ""]
    pub source_cluster_identifier: PrimField<String>,
}

impl BuildDocdbClusterRestoreToPointInTimeEl {
    pub fn build(self) -> DocdbClusterRestoreToPointInTimeEl {
        DocdbClusterRestoreToPointInTimeEl {
            restore_to_time: core::default::Default::default(),
            restore_type: core::default::Default::default(),
            source_cluster_identifier: self.source_cluster_identifier,
            use_latest_restorable_time: core::default::Default::default(),
        }
    }
}

pub struct DocdbClusterRestoreToPointInTimeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DocdbClusterRestoreToPointInTimeElRef {
    fn new(shared: StackShared, base: String) -> DocdbClusterRestoreToPointInTimeElRef {
        DocdbClusterRestoreToPointInTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocdbClusterRestoreToPointInTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `restore_to_time` after provisioning.\n"]
    pub fn restore_to_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.restore_to_time", self.base))
    }

    #[doc = "Get a reference to the value of field `restore_type` after provisioning.\n"]
    pub fn restore_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.restore_type", self.base))
    }

    #[doc = "Get a reference to the value of field `source_cluster_identifier` after provisioning.\n"]
    pub fn source_cluster_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_cluster_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `use_latest_restorable_time` after provisioning.\n"]
    pub fn use_latest_restorable_time(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_latest_restorable_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DocdbClusterServerlessV2ScalingConfigurationEl {
    max_capacity: PrimField<f64>,
    min_capacity: PrimField<f64>,
}

impl DocdbClusterServerlessV2ScalingConfigurationEl { }

impl ToListMappable for DocdbClusterServerlessV2ScalingConfigurationEl {
    type O = BlockAssignable<DocdbClusterServerlessV2ScalingConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocdbClusterServerlessV2ScalingConfigurationEl {
    #[doc = ""]
    pub max_capacity: PrimField<f64>,
    #[doc = ""]
    pub min_capacity: PrimField<f64>,
}

impl BuildDocdbClusterServerlessV2ScalingConfigurationEl {
    pub fn build(self) -> DocdbClusterServerlessV2ScalingConfigurationEl {
        DocdbClusterServerlessV2ScalingConfigurationEl {
            max_capacity: self.max_capacity,
            min_capacity: self.min_capacity,
        }
    }
}

pub struct DocdbClusterServerlessV2ScalingConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DocdbClusterServerlessV2ScalingConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DocdbClusterServerlessV2ScalingConfigurationElRef {
        DocdbClusterServerlessV2ScalingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocdbClusterServerlessV2ScalingConfigurationElRef {
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
}

#[derive(Serialize)]
pub struct DocdbClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl DocdbClusterTimeoutsEl {
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

impl ToListMappable for DocdbClusterTimeoutsEl {
    type O = BlockAssignable<DocdbClusterTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocdbClusterTimeoutsEl {}

impl BuildDocdbClusterTimeoutsEl {
    pub fn build(self) -> DocdbClusterTimeoutsEl {
        DocdbClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DocdbClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DocdbClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DocdbClusterTimeoutsElRef {
        DocdbClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocdbClusterTimeoutsElRef {
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
struct DocdbClusterDynamic {
    restore_to_point_in_time: Option<DynamicBlock<DocdbClusterRestoreToPointInTimeEl>>,
    serverless_v2_scaling_configuration: Option<DynamicBlock<DocdbClusterServerlessV2ScalingConfigurationEl>>,
}
