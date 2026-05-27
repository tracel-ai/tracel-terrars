use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataBackupDrBackupPlanAssociationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    backup_plan_association_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataBackupDrBackupPlanAssociation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBackupDrBackupPlanAssociationData>,
}
#[derive(Clone)]
pub struct DataBackupDrBackupPlanAssociation(Rc<DataBackupDrBackupPlanAssociation_>);
impl DataBackupDrBackupPlanAssociation {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
    pub fn rules_config_info(
        &self,
    ) -> ListRef<DataBackupDrBackupPlanAssociationRulesConfigInfoElRef> {
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
}
impl Referable for DataBackupDrBackupPlanAssociation {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataBackupDrBackupPlanAssociation {}
impl ToListMappable for DataBackupDrBackupPlanAssociation {
    type O = ListRef<DataBackupDrBackupPlanAssociationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataBackupDrBackupPlanAssociation_ {
    fn extract_datasource_type(&self) -> String {
        "google_backup_dr_backup_plan_association".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataBackupDrBackupPlanAssociation {
    pub tf_id: String,
    #[doc = "The id of backupplan association"]
    pub backup_plan_association_id: PrimField<String>,
    #[doc = "The location for the backupplan association"]
    pub location: PrimField<String>,
}
impl BuildDataBackupDrBackupPlanAssociation {
    pub fn build(self, stack: &mut Stack) -> DataBackupDrBackupPlanAssociation {
        let out = DataBackupDrBackupPlanAssociation(Rc::new(DataBackupDrBackupPlanAssociation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBackupDrBackupPlanAssociationData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                backup_plan_association_id: self.backup_plan_association_id,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataBackupDrBackupPlanAssociationRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrBackupPlanAssociationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataBackupDrBackupPlanAssociationRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
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
    pub fn rules_config_info(
        &self,
    ) -> ListRef<DataBackupDrBackupPlanAssociationRulesConfigInfoElRef> {
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
}
#[derive(Serialize)]
pub struct DataBackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl DataBackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorEl {
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
impl ToListMappable for DataBackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorEl {
    type O = BlockAssignable<DataBackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorEl {}
impl BuildDataBackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorEl {
    pub fn build(self) -> DataBackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorEl {
        DataBackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorEl {
            code: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct DataBackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorElRef {
        DataBackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorElRef {
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
pub struct DataBackupDrBackupPlanAssociationRulesConfigInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    last_backup_error:
        Option<ListField<DataBackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_backup_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_successful_backup_consistency_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_id: Option<PrimField<String>>,
}
impl DataBackupDrBackupPlanAssociationRulesConfigInfoEl {
    #[doc = "Set the field `last_backup_error`.\n"]
    pub fn set_last_backup_error(
        mut self,
        v: impl Into<ListField<DataBackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorEl>>,
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
impl ToListMappable for DataBackupDrBackupPlanAssociationRulesConfigInfoEl {
    type O = BlockAssignable<DataBackupDrBackupPlanAssociationRulesConfigInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupDrBackupPlanAssociationRulesConfigInfoEl {}
impl BuildDataBackupDrBackupPlanAssociationRulesConfigInfoEl {
    pub fn build(self) -> DataBackupDrBackupPlanAssociationRulesConfigInfoEl {
        DataBackupDrBackupPlanAssociationRulesConfigInfoEl {
            last_backup_error: core::default::Default::default(),
            last_backup_state: core::default::Default::default(),
            last_successful_backup_consistency_time: core::default::Default::default(),
            rule_id: core::default::Default::default(),
        }
    }
}
pub struct DataBackupDrBackupPlanAssociationRulesConfigInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrBackupPlanAssociationRulesConfigInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBackupDrBackupPlanAssociationRulesConfigInfoElRef {
        DataBackupDrBackupPlanAssociationRulesConfigInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupDrBackupPlanAssociationRulesConfigInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `last_backup_error` after provisioning.\n"]
    pub fn last_backup_error(
        &self,
    ) -> ListRef<DataBackupDrBackupPlanAssociationRulesConfigInfoElLastBackupErrorElRef> {
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
