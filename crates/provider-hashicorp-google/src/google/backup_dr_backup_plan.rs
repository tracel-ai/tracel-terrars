use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BackupDrBackupPlanData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    backup_plan_id: PrimField<String>,
    backup_vault: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_retention_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_custom_on_demand_retention_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    resource_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_rules: Option<Vec<BackupDrBackupPlanBackupRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_instance_backup_plan_properties:
        Option<Vec<BackupDrBackupPlanComputeInstanceBackupPlanPropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_backup_plan_properties: Option<Vec<BackupDrBackupPlanDiskBackupPlanPropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BackupDrBackupPlanTimeoutsEl>,
    dynamic: BackupDrBackupPlanDynamic,
}
struct BackupDrBackupPlan_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BackupDrBackupPlanData>,
}
#[derive(Clone)]
pub struct BackupDrBackupPlan(Rc<BackupDrBackupPlan_>);
impl BackupDrBackupPlan {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nThe description allows for additional details about 'BackupPlan' and its use cases to be provided."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `log_retention_days`.\nThis is only applicable for CloudSql resource. Days for which logs will be stored. This value should be greater than or equal to minimum enforced log retention duration of the backup vault."]
    pub fn set_log_retention_days(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().log_retention_days = Some(v.into());
        self
    }
    #[doc = "Set the field `max_custom_on_demand_retention_days`.\nThe maximum number of days for which an on-demand backup taken with custom retention can be retained."]
    pub fn set_max_custom_on_demand_retention_days(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_custom_on_demand_retention_days = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `backup_rules`.\n"]
    pub fn set_backup_rules(
        self,
        v: impl Into<BlockAssignable<BackupDrBackupPlanBackupRulesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().backup_rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.backup_rules = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `compute_instance_backup_plan_properties`.\n"]
    pub fn set_compute_instance_backup_plan_properties(
        self,
        v: impl Into<BlockAssignable<BackupDrBackupPlanComputeInstanceBackupPlanPropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0
                    .data
                    .borrow_mut()
                    .compute_instance_backup_plan_properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .compute_instance_backup_plan_properties = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `disk_backup_plan_properties`.\n"]
    pub fn set_disk_backup_plan_properties(
        self,
        v: impl Into<BlockAssignable<BackupDrBackupPlanDiskBackupPlanPropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().disk_backup_plan_properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.disk_backup_plan_properties = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BackupDrBackupPlanTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `backup_plan_id` after provisioning.\nThe ID of the backup plan"]
    pub fn backup_plan_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_plan_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_vault` after provisioning.\nBackup vault where the backups gets stored using this Backup plan."]
    pub fn backup_vault(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_vault", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_vault_service_account` after provisioning.\nThe Google Cloud Platform Service Account to be used by the BackupVault for taking backups."]
    pub fn backup_vault_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_vault_service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nWhen the 'BackupPlan' was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description allows for additional details about 'BackupPlan' and its use cases to be provided."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the backup plan"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_retention_days` after provisioning.\nThis is only applicable for CloudSql resource. Days for which logs will be stored. This value should be greater than or equal to minimum enforced log retention duration of the backup vault."]
    pub fn log_retention_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_retention_days", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_custom_on_demand_retention_days` after provisioning.\nThe maximum number of days for which an on-demand backup taken with custom retention can be retained."]
    pub fn max_custom_on_demand_retention_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_custom_on_demand_retention_days", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of backup plan resource created"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\nThe resource type to which the 'BackupPlan' will be applied.\nExamples include, \"compute.googleapis.com/Instance\", \"compute.googleapis.com/Disk\", \"sqladmin.googleapis.com/Instance\", \"alloydb.googleapis.com/Cluster\", \"file.googleapis.com/Instance\" and \"storage.googleapis.com/Bucket\"."]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `supported_resource_types` after provisioning.\nThe list of all resource types to which the 'BackupPlan' can be applied."]
    pub fn supported_resource_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_resource_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nWhen the 'BackupPlan' was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_rules` after provisioning.\n"]
    pub fn backup_rules(&self) -> ListRef<BackupDrBackupPlanBackupRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backup_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compute_instance_backup_plan_properties` after provisioning.\n"]
    pub fn compute_instance_backup_plan_properties(
        &self,
    ) -> ListRef<BackupDrBackupPlanComputeInstanceBackupPlanPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.compute_instance_backup_plan_properties",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `disk_backup_plan_properties` after provisioning.\n"]
    pub fn disk_backup_plan_properties(
        &self,
    ) -> ListRef<BackupDrBackupPlanDiskBackupPlanPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk_backup_plan_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BackupDrBackupPlanTimeoutsElRef {
        BackupDrBackupPlanTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BackupDrBackupPlan {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BackupDrBackupPlan {}
impl ToListMappable for BackupDrBackupPlan {
    type O = ListRef<BackupDrBackupPlanRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BackupDrBackupPlan_ {
    fn extract_resource_type(&self) -> String {
        "google_backup_dr_backup_plan".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBackupDrBackupPlan {
    pub tf_id: String,
    #[doc = "The ID of the backup plan"]
    pub backup_plan_id: PrimField<String>,
    #[doc = "Backup vault where the backups gets stored using this Backup plan."]
    pub backup_vault: PrimField<String>,
    #[doc = "The location for the backup plan"]
    pub location: PrimField<String>,
    #[doc = "The resource type to which the 'BackupPlan' will be applied.\nExamples include, \"compute.googleapis.com/Instance\", \"compute.googleapis.com/Disk\", \"sqladmin.googleapis.com/Instance\", \"alloydb.googleapis.com/Cluster\", \"file.googleapis.com/Instance\" and \"storage.googleapis.com/Bucket\"."]
    pub resource_type: PrimField<String>,
}
impl BuildBackupDrBackupPlan {
    pub fn build(self, stack: &mut Stack) -> BackupDrBackupPlan {
        let out = BackupDrBackupPlan(Rc::new(BackupDrBackupPlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BackupDrBackupPlanData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                backup_plan_id: self.backup_plan_id,
                backup_vault: self.backup_vault,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                log_retention_days: core::default::Default::default(),
                max_custom_on_demand_retention_days: core::default::Default::default(),
                project: core::default::Default::default(),
                resource_type: self.resource_type,
                backup_rules: core::default::Default::default(),
                compute_instance_backup_plan_properties: core::default::Default::default(),
                disk_backup_plan_properties: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BackupDrBackupPlanRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrBackupPlanRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BackupDrBackupPlanRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup_plan_id` after provisioning.\nThe ID of the backup plan"]
    pub fn backup_plan_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_plan_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_vault` after provisioning.\nBackup vault where the backups gets stored using this Backup plan."]
    pub fn backup_vault(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_vault", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_vault_service_account` after provisioning.\nThe Google Cloud Platform Service Account to be used by the BackupVault for taking backups."]
    pub fn backup_vault_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_vault_service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nWhen the 'BackupPlan' was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description allows for additional details about 'BackupPlan' and its use cases to be provided."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the backup plan"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_retention_days` after provisioning.\nThis is only applicable for CloudSql resource. Days for which logs will be stored. This value should be greater than or equal to minimum enforced log retention duration of the backup vault."]
    pub fn log_retention_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_retention_days", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_custom_on_demand_retention_days` after provisioning.\nThe maximum number of days for which an on-demand backup taken with custom retention can be retained."]
    pub fn max_custom_on_demand_retention_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_custom_on_demand_retention_days", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of backup plan resource created"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\nThe resource type to which the 'BackupPlan' will be applied.\nExamples include, \"compute.googleapis.com/Instance\", \"compute.googleapis.com/Disk\", \"sqladmin.googleapis.com/Instance\", \"alloydb.googleapis.com/Cluster\", \"file.googleapis.com/Instance\" and \"storage.googleapis.com/Bucket\"."]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `supported_resource_types` after provisioning.\nThe list of all resource types to which the 'BackupPlan' can be applied."]
    pub fn supported_resource_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_resource_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nWhen the 'BackupPlan' was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_rules` after provisioning.\n"]
    pub fn backup_rules(&self) -> ListRef<BackupDrBackupPlanBackupRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backup_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compute_instance_backup_plan_properties` after provisioning.\n"]
    pub fn compute_instance_backup_plan_properties(
        &self,
    ) -> ListRef<BackupDrBackupPlanComputeInstanceBackupPlanPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.compute_instance_backup_plan_properties",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `disk_backup_plan_properties` after provisioning.\n"]
    pub fn disk_backup_plan_properties(
        &self,
    ) -> ListRef<BackupDrBackupPlanDiskBackupPlanPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk_backup_plan_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BackupDrBackupPlanTimeoutsElRef {
        BackupDrBackupPlanTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BackupDrBackupPlanBackupRulesElStandardScheduleElBackupWindowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_hour_of_day: Option<PrimField<f64>>,
    start_hour_of_day: PrimField<f64>,
}
impl BackupDrBackupPlanBackupRulesElStandardScheduleElBackupWindowEl {
    #[doc = "Set the field `end_hour_of_day`.\nThe hour of the day (1-24) when the window ends, for example, if the value of end hour of the day is 10, that means the backup window end time is 10:00.\nThe end hour of the day should be greater than the start"]
    pub fn set_end_hour_of_day(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.end_hour_of_day = Some(v.into());
        self
    }
}
impl ToListMappable for BackupDrBackupPlanBackupRulesElStandardScheduleElBackupWindowEl {
    type O = BlockAssignable<BackupDrBackupPlanBackupRulesElStandardScheduleElBackupWindowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrBackupPlanBackupRulesElStandardScheduleElBackupWindowEl {
    #[doc = "The hour of the day (0-23) when the window starts, for example, if the value of the start hour of the day is 6, that means the backup window starts at 6:00."]
    pub start_hour_of_day: PrimField<f64>,
}
impl BuildBackupDrBackupPlanBackupRulesElStandardScheduleElBackupWindowEl {
    pub fn build(self) -> BackupDrBackupPlanBackupRulesElStandardScheduleElBackupWindowEl {
        BackupDrBackupPlanBackupRulesElStandardScheduleElBackupWindowEl {
            end_hour_of_day: core::default::Default::default(),
            start_hour_of_day: self.start_hour_of_day,
        }
    }
}
pub struct BackupDrBackupPlanBackupRulesElStandardScheduleElBackupWindowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrBackupPlanBackupRulesElStandardScheduleElBackupWindowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrBackupPlanBackupRulesElStandardScheduleElBackupWindowElRef {
        BackupDrBackupPlanBackupRulesElStandardScheduleElBackupWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrBackupPlanBackupRulesElStandardScheduleElBackupWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_hour_of_day` after provisioning.\nThe hour of the day (1-24) when the window ends, for example, if the value of end hour of the day is 10, that means the backup window end time is 10:00.\nThe end hour of the day should be greater than the start"]
    pub fn end_hour_of_day(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.end_hour_of_day", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `start_hour_of_day` after provisioning.\nThe hour of the day (0-23) when the window starts, for example, if the value of the start hour of the day is 6, that means the backup window starts at 6:00."]
    pub fn start_hour_of_day(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_hour_of_day", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BackupDrBackupPlanBackupRulesElStandardScheduleElWeekDayOfMonthEl {
    day_of_week: PrimField<String>,
    week_of_month: PrimField<String>,
}
impl BackupDrBackupPlanBackupRulesElStandardScheduleElWeekDayOfMonthEl {}
impl ToListMappable for BackupDrBackupPlanBackupRulesElStandardScheduleElWeekDayOfMonthEl {
    type O = BlockAssignable<BackupDrBackupPlanBackupRulesElStandardScheduleElWeekDayOfMonthEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrBackupPlanBackupRulesElStandardScheduleElWeekDayOfMonthEl {
    #[doc = "Specifies the day of the week. Possible values: [\"DAY_OF_WEEK_UNSPECIFIED\", \"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub day_of_week: PrimField<String>,
    #[doc = "WeekOfMonth enumerates possible weeks in the month, e.g. the first, third, or last week of the month. Possible values: [\"WEEK_OF_MONTH_UNSPECIFIED\", \"FIRST\", \"SECOND\", \"THIRD\", \"FOURTH\", \"LAST\"]"]
    pub week_of_month: PrimField<String>,
}
impl BuildBackupDrBackupPlanBackupRulesElStandardScheduleElWeekDayOfMonthEl {
    pub fn build(self) -> BackupDrBackupPlanBackupRulesElStandardScheduleElWeekDayOfMonthEl {
        BackupDrBackupPlanBackupRulesElStandardScheduleElWeekDayOfMonthEl {
            day_of_week: self.day_of_week,
            week_of_month: self.week_of_month,
        }
    }
}
pub struct BackupDrBackupPlanBackupRulesElStandardScheduleElWeekDayOfMonthElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrBackupPlanBackupRulesElStandardScheduleElWeekDayOfMonthElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrBackupPlanBackupRulesElStandardScheduleElWeekDayOfMonthElRef {
        BackupDrBackupPlanBackupRulesElStandardScheduleElWeekDayOfMonthElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrBackupPlanBackupRulesElStandardScheduleElWeekDayOfMonthElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day_of_week` after provisioning.\nSpecifies the day of the week. Possible values: [\"DAY_OF_WEEK_UNSPECIFIED\", \"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn day_of_week(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }
    #[doc = "Get a reference to the value of field `week_of_month` after provisioning.\nWeekOfMonth enumerates possible weeks in the month, e.g. the first, third, or last week of the month. Possible values: [\"WEEK_OF_MONTH_UNSPECIFIED\", \"FIRST\", \"SECOND\", \"THIRD\", \"FOURTH\", \"LAST\"]"]
    pub fn week_of_month(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.week_of_month", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct BackupDrBackupPlanBackupRulesElStandardScheduleElDynamic {
    backup_window:
        Option<DynamicBlock<BackupDrBackupPlanBackupRulesElStandardScheduleElBackupWindowEl>>,
    week_day_of_month:
        Option<DynamicBlock<BackupDrBackupPlanBackupRulesElStandardScheduleElWeekDayOfMonthEl>>,
}
#[derive(Serialize)]
pub struct BackupDrBackupPlanBackupRulesElStandardScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    days_of_month: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    days_of_week: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hourly_frequency: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    months: Option<ListField<PrimField<String>>>,
    recurrence_type: PrimField<String>,
    time_zone: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_window: Option<Vec<BackupDrBackupPlanBackupRulesElStandardScheduleElBackupWindowEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    week_day_of_month:
        Option<Vec<BackupDrBackupPlanBackupRulesElStandardScheduleElWeekDayOfMonthEl>>,
    dynamic: BackupDrBackupPlanBackupRulesElStandardScheduleElDynamic,
}
impl BackupDrBackupPlanBackupRulesElStandardScheduleEl {
    #[doc = "Set the field `days_of_month`.\nSpecifies days of months like 1, 5, or 14 on which jobs will run."]
    pub fn set_days_of_month(mut self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.days_of_month = Some(v.into());
        self
    }
    #[doc = "Set the field `days_of_week`.\nSpecifies days of week like MONDAY or TUESDAY, on which jobs will run. This is required for 'recurrence_type', 'WEEKLY' and is not applicable otherwise. Possible values: [\"DAY_OF_WEEK_UNSPECIFIED\", \"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn set_days_of_week(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.days_of_week = Some(v.into());
        self
    }
    #[doc = "Set the field `hourly_frequency`.\nSpecifies frequency for hourly backups. An hourly frequency of 2 means jobs will run every 2 hours from start time till end time defined.\nThis is required for 'recurrence_type', 'HOURLY' and is not applicable otherwise."]
    pub fn set_hourly_frequency(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hourly_frequency = Some(v.into());
        self
    }
    #[doc = "Set the field `months`.\nSpecifies values of months Possible values: [\"MONTH_UNSPECIFIED\", \"JANUARY\", \"FEBRUARY\", \"MARCH\", \"APRIL\", \"MAY\", \"JUNE\", \"JULY\", \"AUGUST\", \"SEPTEMBER\", \"OCTOBER\", \"NOVEMBER\", \"DECEMBER\"]"]
    pub fn set_months(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.months = Some(v.into());
        self
    }
    #[doc = "Set the field `backup_window`.\n"]
    pub fn set_backup_window(
        mut self,
        v: impl Into<BlockAssignable<BackupDrBackupPlanBackupRulesElStandardScheduleElBackupWindowEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.backup_window = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.backup_window = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `week_day_of_month`.\n"]
    pub fn set_week_day_of_month(
        mut self,
        v: impl Into<BlockAssignable<BackupDrBackupPlanBackupRulesElStandardScheduleElWeekDayOfMonthEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.week_day_of_month = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.week_day_of_month = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BackupDrBackupPlanBackupRulesElStandardScheduleEl {
    type O = BlockAssignable<BackupDrBackupPlanBackupRulesElStandardScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrBackupPlanBackupRulesElStandardScheduleEl {
    #[doc = "RecurrenceType enumerates the applicable periodicity for the schedule. Possible values: [\"HOURLY\", \"DAILY\", \"WEEKLY\", \"MONTHLY\", \"YEARLY\"]"]
    pub recurrence_type: PrimField<String>,
    #[doc = "The time zone to be used when interpreting the schedule."]
    pub time_zone: PrimField<String>,
}
impl BuildBackupDrBackupPlanBackupRulesElStandardScheduleEl {
    pub fn build(self) -> BackupDrBackupPlanBackupRulesElStandardScheduleEl {
        BackupDrBackupPlanBackupRulesElStandardScheduleEl {
            days_of_month: core::default::Default::default(),
            days_of_week: core::default::Default::default(),
            hourly_frequency: core::default::Default::default(),
            months: core::default::Default::default(),
            recurrence_type: self.recurrence_type,
            time_zone: self.time_zone,
            backup_window: core::default::Default::default(),
            week_day_of_month: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BackupDrBackupPlanBackupRulesElStandardScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrBackupPlanBackupRulesElStandardScheduleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrBackupPlanBackupRulesElStandardScheduleElRef {
        BackupDrBackupPlanBackupRulesElStandardScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrBackupPlanBackupRulesElStandardScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `days_of_month` after provisioning.\nSpecifies days of months like 1, 5, or 14 on which jobs will run."]
    pub fn days_of_month(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.days_of_month", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `days_of_week` after provisioning.\nSpecifies days of week like MONDAY or TUESDAY, on which jobs will run. This is required for 'recurrence_type', 'WEEKLY' and is not applicable otherwise. Possible values: [\"DAY_OF_WEEK_UNSPECIFIED\", \"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn days_of_week(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.days_of_week", self.base))
    }
    #[doc = "Get a reference to the value of field `hourly_frequency` after provisioning.\nSpecifies frequency for hourly backups. An hourly frequency of 2 means jobs will run every 2 hours from start time till end time defined.\nThis is required for 'recurrence_type', 'HOURLY' and is not applicable otherwise."]
    pub fn hourly_frequency(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hourly_frequency", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `months` after provisioning.\nSpecifies values of months Possible values: [\"MONTH_UNSPECIFIED\", \"JANUARY\", \"FEBRUARY\", \"MARCH\", \"APRIL\", \"MAY\", \"JUNE\", \"JULY\", \"AUGUST\", \"SEPTEMBER\", \"OCTOBER\", \"NOVEMBER\", \"DECEMBER\"]"]
    pub fn months(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.months", self.base))
    }
    #[doc = "Get a reference to the value of field `recurrence_type` after provisioning.\nRecurrenceType enumerates the applicable periodicity for the schedule. Possible values: [\"HOURLY\", \"DAILY\", \"WEEKLY\", \"MONTHLY\", \"YEARLY\"]"]
    pub fn recurrence_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.recurrence_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\nThe time zone to be used when interpreting the schedule."]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
    #[doc = "Get a reference to the value of field `backup_window` after provisioning.\n"]
    pub fn backup_window(
        &self,
    ) -> ListRef<BackupDrBackupPlanBackupRulesElStandardScheduleElBackupWindowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backup_window", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `week_day_of_month` after provisioning.\n"]
    pub fn week_day_of_month(
        &self,
    ) -> ListRef<BackupDrBackupPlanBackupRulesElStandardScheduleElWeekDayOfMonthElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.week_day_of_month", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct BackupDrBackupPlanBackupRulesElDynamic {
    standard_schedule: Option<DynamicBlock<BackupDrBackupPlanBackupRulesElStandardScheduleEl>>,
}
#[derive(Serialize)]
pub struct BackupDrBackupPlanBackupRulesEl {
    backup_retention_days: PrimField<f64>,
    rule_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    standard_schedule: Option<Vec<BackupDrBackupPlanBackupRulesElStandardScheduleEl>>,
    dynamic: BackupDrBackupPlanBackupRulesElDynamic,
}
impl BackupDrBackupPlanBackupRulesEl {
    #[doc = "Set the field `standard_schedule`.\n"]
    pub fn set_standard_schedule(
        mut self,
        v: impl Into<BlockAssignable<BackupDrBackupPlanBackupRulesElStandardScheduleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.standard_schedule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.standard_schedule = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BackupDrBackupPlanBackupRulesEl {
    type O = BlockAssignable<BackupDrBackupPlanBackupRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrBackupPlanBackupRulesEl {
    #[doc = "Configures the duration for which backup data will be kept. The value should be greater than or equal to minimum enforced retention of the backup vault."]
    pub backup_retention_days: PrimField<f64>,
    #[doc = "The unique ID of this 'BackupRule'. The 'rule_id' is unique per 'BackupPlan'."]
    pub rule_id: PrimField<String>,
}
impl BuildBackupDrBackupPlanBackupRulesEl {
    pub fn build(self) -> BackupDrBackupPlanBackupRulesEl {
        BackupDrBackupPlanBackupRulesEl {
            backup_retention_days: self.backup_retention_days,
            rule_id: self.rule_id,
            standard_schedule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BackupDrBackupPlanBackupRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrBackupPlanBackupRulesElRef {
    fn new(shared: StackShared, base: String) -> BackupDrBackupPlanBackupRulesElRef {
        BackupDrBackupPlanBackupRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrBackupPlanBackupRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup_retention_days` after provisioning.\nConfigures the duration for which backup data will be kept. The value should be greater than or equal to minimum enforced retention of the backup vault."]
    pub fn backup_retention_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_retention_days", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\nThe unique ID of this 'BackupRule'. The 'rule_id' is unique per 'BackupPlan'."]
    pub fn rule_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.base))
    }
    #[doc = "Get a reference to the value of field `standard_schedule` after provisioning.\n"]
    pub fn standard_schedule(
        &self,
    ) -> ListRef<BackupDrBackupPlanBackupRulesElStandardScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.standard_schedule", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BackupDrBackupPlanComputeInstanceBackupPlanPropertiesEl {
    guest_flush: PrimField<bool>,
}
impl BackupDrBackupPlanComputeInstanceBackupPlanPropertiesEl {}
impl ToListMappable for BackupDrBackupPlanComputeInstanceBackupPlanPropertiesEl {
    type O = BlockAssignable<BackupDrBackupPlanComputeInstanceBackupPlanPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrBackupPlanComputeInstanceBackupPlanPropertiesEl {
    #[doc = "Indicates whether to perform a guest flush operation before taking a\ncompute instance backup. When set to true, the system will attempt\nto ensure application-consistent backups."]
    pub guest_flush: PrimField<bool>,
}
impl BuildBackupDrBackupPlanComputeInstanceBackupPlanPropertiesEl {
    pub fn build(self) -> BackupDrBackupPlanComputeInstanceBackupPlanPropertiesEl {
        BackupDrBackupPlanComputeInstanceBackupPlanPropertiesEl {
            guest_flush: self.guest_flush,
        }
    }
}
pub struct BackupDrBackupPlanComputeInstanceBackupPlanPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrBackupPlanComputeInstanceBackupPlanPropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrBackupPlanComputeInstanceBackupPlanPropertiesElRef {
        BackupDrBackupPlanComputeInstanceBackupPlanPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrBackupPlanComputeInstanceBackupPlanPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `guest_flush` after provisioning.\nIndicates whether to perform a guest flush operation before taking a\ncompute instance backup. When set to true, the system will attempt\nto ensure application-consistent backups."]
    pub fn guest_flush(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.guest_flush", self.base))
    }
}
#[derive(Serialize)]
pub struct BackupDrBackupPlanDiskBackupPlanPropertiesEl {
    guest_flush: PrimField<bool>,
}
impl BackupDrBackupPlanDiskBackupPlanPropertiesEl {}
impl ToListMappable for BackupDrBackupPlanDiskBackupPlanPropertiesEl {
    type O = BlockAssignable<BackupDrBackupPlanDiskBackupPlanPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrBackupPlanDiskBackupPlanPropertiesEl {
    #[doc = "Indicates whether to perform a guest flush operation before taking a disk\nbackup. When set to true, the system will attempt to ensure\napplication-consistent backups. When set to false, the system will\ncreate crash-consistent backups."]
    pub guest_flush: PrimField<bool>,
}
impl BuildBackupDrBackupPlanDiskBackupPlanPropertiesEl {
    pub fn build(self) -> BackupDrBackupPlanDiskBackupPlanPropertiesEl {
        BackupDrBackupPlanDiskBackupPlanPropertiesEl {
            guest_flush: self.guest_flush,
        }
    }
}
pub struct BackupDrBackupPlanDiskBackupPlanPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrBackupPlanDiskBackupPlanPropertiesElRef {
    fn new(shared: StackShared, base: String) -> BackupDrBackupPlanDiskBackupPlanPropertiesElRef {
        BackupDrBackupPlanDiskBackupPlanPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrBackupPlanDiskBackupPlanPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `guest_flush` after provisioning.\nIndicates whether to perform a guest flush operation before taking a disk\nbackup. When set to true, the system will attempt to ensure\napplication-consistent backups. When set to false, the system will\ncreate crash-consistent backups."]
    pub fn guest_flush(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.guest_flush", self.base))
    }
}
#[derive(Serialize)]
pub struct BackupDrBackupPlanTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BackupDrBackupPlanTimeoutsEl {
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
impl ToListMappable for BackupDrBackupPlanTimeoutsEl {
    type O = BlockAssignable<BackupDrBackupPlanTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrBackupPlanTimeoutsEl {}
impl BuildBackupDrBackupPlanTimeoutsEl {
    pub fn build(self) -> BackupDrBackupPlanTimeoutsEl {
        BackupDrBackupPlanTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BackupDrBackupPlanTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrBackupPlanTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BackupDrBackupPlanTimeoutsElRef {
        BackupDrBackupPlanTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrBackupPlanTimeoutsElRef {
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
struct BackupDrBackupPlanDynamic {
    backup_rules: Option<DynamicBlock<BackupDrBackupPlanBackupRulesEl>>,
    compute_instance_backup_plan_properties:
        Option<DynamicBlock<BackupDrBackupPlanComputeInstanceBackupPlanPropertiesEl>>,
    disk_backup_plan_properties: Option<DynamicBlock<BackupDrBackupPlanDiskBackupPlanPropertiesEl>>,
}
