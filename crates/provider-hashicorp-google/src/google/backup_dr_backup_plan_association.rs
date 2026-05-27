use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BackupDrBackupPlanAssociationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    backup_plan: PrimField<String>,
    backup_plan_association_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    resource: PrimField<String>,
    resource_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BackupDrBackupPlanAssociationTimeoutsEl>,
}
struct BackupDrBackupPlanAssociation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BackupDrBackupPlanAssociationData>,
}
#[derive(Clone)]
pub struct BackupDrBackupPlanAssociation(Rc<BackupDrBackupPlanAssociation_>);
impl BackupDrBackupPlanAssociation {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BackupDrBackupPlanAssociationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `backup_plan` after provisioning.\nThe BP with which resource needs to be created\nNote:\n- A Backup Plan configured for 'compute.googleapis.com/Instance', can only protect instance type resources.\n- A Backup Plan configured for 'compute.googleapis.com/Disk' can be used to protect both standard Disks and Regional Disks resources.\n- A Backup Plan configured for 'file.googleapis.com/Instance' can only protect Filestore instances."]
    pub fn backup_plan(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_plan", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_plan_association_id` after provisioning.\nThe id of backupplan association"]
    pub fn backup_plan_association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_plan_association_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the instance was created"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_source` after provisioning.\nResource name of data source which will be used as storage location for backups taken"]
    pub fn data_source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the backupplan association"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of backup plan association resource created"]
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
    #[doc = "Get a reference to the value of field `resource` after provisioning.\nThe resource for which BPA needs to be created"]
    pub fn resource(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\nThe resource type of workload on which backupplan is applied.\nExamples include, \"compute.googleapis.com/Instance\", \"compute.googleapis.com/Disk\", \"compute.googleapis.com/RegionDisk\", and \"file.googleapis.com/Instance\""]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules_config_info` after provisioning.\nMessage for rules config info"]
    pub fn rules_config_info(&self) -> ListRef<BackupDrBackupPlanAssociationRulesConfigInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules_config_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the instance was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BackupDrBackupPlanAssociationTimeoutsElRef {
        BackupDrBackupPlanAssociationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BackupDrBackupPlanAssociation {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BackupDrBackupPlanAssociation {}
impl ToListMappable for BackupDrBackupPlanAssociation {
    type O = ListRef<BackupDrBackupPlanAssociationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BackupDrBackupPlanAssociation_ {
    fn extract_resource_type(&self) -> String {
        "google_backup_dr_backup_plan_association".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBackupDrBackupPlanAssociation {
    pub tf_id: String,
    #[doc = "The BP with which resource needs to be created\nNote:\n- A Backup Plan configured for 'compute.googleapis.com/Instance', can only protect instance type resources.\n- A Backup Plan configured for 'compute.googleapis.com/Disk' can be used to protect both standard Disks and Regional Disks resources.\n- A Backup Plan configured for 'file.googleapis.com/Instance' can only protect Filestore instances."]
    pub backup_plan: PrimField<String>,
    #[doc = "The id of backupplan association"]
    pub backup_plan_association_id: PrimField<String>,
    #[doc = "The location for the backupplan association"]
    pub location: PrimField<String>,
    #[doc = "The resource for which BPA needs to be created"]
    pub resource: PrimField<String>,
    #[doc = "The resource type of workload on which backupplan is applied.\nExamples include, \"compute.googleapis.com/Instance\", \"compute.googleapis.com/Disk\", \"compute.googleapis.com/RegionDisk\", and \"file.googleapis.com/Instance\""]
    pub resource_type: PrimField<String>,
}
impl BuildBackupDrBackupPlanAssociation {
    pub fn build(self, stack: &mut Stack) -> BackupDrBackupPlanAssociation {
        let out = BackupDrBackupPlanAssociation(Rc::new(BackupDrBackupPlanAssociation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BackupDrBackupPlanAssociationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                backup_plan: self.backup_plan,
                backup_plan_association_id: self.backup_plan_association_id,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                resource: self.resource,
                resource_type: self.resource_type,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BackupDrBackupPlanAssociationRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrBackupPlanAssociationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BackupDrBackupPlanAssociationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup_plan` after provisioning.\nThe BP with which resource needs to be created\nNote:\n- A Backup Plan configured for 'compute.googleapis.com/Instance', can only protect instance type resources.\n- A Backup Plan configured for 'compute.googleapis.com/Disk' can be used to protect both standard Disks and Regional Disks resources.\n- A Backup Plan configured for 'file.googleapis.com/Instance' can only protect Filestore instances."]
    pub fn backup_plan(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_plan", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_plan_association_id` after provisioning.\nThe id of backupplan association"]
    pub fn backup_plan_association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_plan_association_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the instance was created"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_source` after provisioning.\nResource name of data source which will be used as storage location for backups taken"]
    pub fn data_source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the backupplan association"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of backup plan association resource created"]
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
    #[doc = "Get a reference to the value of field `resource` after provisioning.\nThe resource for which BPA needs to be created"]
    pub fn resource(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\nThe resource type of workload on which backupplan is applied.\nExamples include, \"compute.googleapis.com/Instance\", \"compute.googleapis.com/Disk\", \"compute.googleapis.com/RegionDisk\", and \"file.googleapis.com/Instance\""]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules_config_info` after provisioning.\nMessage for rules config info"]
    pub fn rules_config_info(&self) -> ListRef<BackupDrBackupPlanAssociationRulesConfigInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules_config_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the instance was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BackupDrBackupPlanAssociationTimeoutsElRef {
        BackupDrBackupPlanAssociationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl BackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable for BackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorEl {
    type O = BlockAssignable<BackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorEl {}
impl BuildBackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorEl {
    pub fn build(self) -> BackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorEl {
        BackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorEl {
            code: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct BackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorElRef {
        BackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct BackupDrBackupPlanAssociationRulesConfigInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    last_backup_error:
        Option<ListField<BackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_backup_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_successful_backup_consistency_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_id: Option<PrimField<String>>,
}
impl BackupDrBackupPlanAssociationRulesConfigInfoEl {
    #[doc = "Set the field `last_backup_error`.\n"]
    pub fn set_last_backup_error(
        mut self,
        v: impl Into<ListField<BackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorEl>>,
    ) -> Self {
        self.last_backup_error = Some(v.into());
        self
    }
    #[doc = "Set the field `last_backup_state`.\n"]
    pub fn set_last_backup_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_backup_state = Some(v.into());
        self
    }
    #[doc = "Set the field `last_successful_backup_consistency_time`.\n"]
    pub fn set_last_successful_backup_consistency_time(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.last_successful_backup_consistency_time = Some(v.into());
        self
    }
    #[doc = "Set the field `rule_id`.\n"]
    pub fn set_rule_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rule_id = Some(v.into());
        self
    }
}
impl ToListMappable for BackupDrBackupPlanAssociationRulesConfigInfoEl {
    type O = BlockAssignable<BackupDrBackupPlanAssociationRulesConfigInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrBackupPlanAssociationRulesConfigInfoEl {}
impl BuildBackupDrBackupPlanAssociationRulesConfigInfoEl {
    pub fn build(self) -> BackupDrBackupPlanAssociationRulesConfigInfoEl {
        BackupDrBackupPlanAssociationRulesConfigInfoEl {
            last_backup_error: core::default::Default::default(),
            last_backup_state: core::default::Default::default(),
            last_successful_backup_consistency_time: core::default::Default::default(),
            rule_id: core::default::Default::default(),
        }
    }
}
pub struct BackupDrBackupPlanAssociationRulesConfigInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrBackupPlanAssociationRulesConfigInfoElRef {
    fn new(shared: StackShared, base: String) -> BackupDrBackupPlanAssociationRulesConfigInfoElRef {
        BackupDrBackupPlanAssociationRulesConfigInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrBackupPlanAssociationRulesConfigInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `last_backup_error` after provisioning.\n"]
    pub fn last_backup_error(
        &self,
    ) -> ListRef<BackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.last_backup_error", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `last_backup_state` after provisioning.\n"]
    pub fn last_backup_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_backup_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `last_successful_backup_consistency_time` after provisioning.\n"]
    pub fn last_successful_backup_consistency_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_successful_backup_consistency_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\n"]
    pub fn rule_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.base))
    }
}
#[derive(Serialize)]
pub struct BackupDrBackupPlanAssociationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BackupDrBackupPlanAssociationTimeoutsEl {
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
impl ToListMappable for BackupDrBackupPlanAssociationTimeoutsEl {
    type O = BlockAssignable<BackupDrBackupPlanAssociationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrBackupPlanAssociationTimeoutsEl {}
impl BuildBackupDrBackupPlanAssociationTimeoutsEl {
    pub fn build(self) -> BackupDrBackupPlanAssociationTimeoutsEl {
        BackupDrBackupPlanAssociationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BackupDrBackupPlanAssociationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrBackupPlanAssociationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BackupDrBackupPlanAssociationTimeoutsElRef {
        BackupDrBackupPlanAssociationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrBackupPlanAssociationTimeoutsElRef {
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
