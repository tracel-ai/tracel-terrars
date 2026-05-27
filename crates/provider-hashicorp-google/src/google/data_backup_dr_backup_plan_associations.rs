use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataBackupDrBackupPlanAssociationsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_type: Option<PrimField<String>>,
}
struct DataBackupDrBackupPlanAssociations_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBackupDrBackupPlanAssociationsData>,
}
#[derive(Clone)]
pub struct DataBackupDrBackupPlanAssociations(Rc<DataBackupDrBackupPlanAssociations_>);
impl DataBackupDrBackupPlanAssociations {
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
    #[doc = "Set the field `project`.\nThe ID of the project in which the resource belongs."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_type`.\nThe resource type of workload on which backup plan is applied. Examples include, \"compute.googleapis.com/Instance\", \"compute.googleapis.com/Disk\"."]
    pub fn set_resource_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().resource_type = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `associations` after provisioning.\nA list of the backup plan associations found."]
    pub fn associations(&self) -> ListRef<DataBackupDrBackupPlanAssociationsAssociationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.associations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location to list the backup plan associations from."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\nThe resource type of workload on which backup plan is applied. Examples include, \"compute.googleapis.com/Instance\", \"compute.googleapis.com/Disk\"."]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.extract_ref()),
        )
    }
}
impl Referable for DataBackupDrBackupPlanAssociations {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataBackupDrBackupPlanAssociations {}
impl ToListMappable for DataBackupDrBackupPlanAssociations {
    type O = ListRef<DataBackupDrBackupPlanAssociationsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataBackupDrBackupPlanAssociations_ {
    fn extract_datasource_type(&self) -> String {
        "google_backup_dr_backup_plan_associations".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataBackupDrBackupPlanAssociations {
    pub tf_id: String,
    #[doc = "The location to list the backup plan associations from."]
    pub location: PrimField<String>,
}
impl BuildDataBackupDrBackupPlanAssociations {
    pub fn build(self, stack: &mut Stack) -> DataBackupDrBackupPlanAssociations {
        let out =
            DataBackupDrBackupPlanAssociations(Rc::new(DataBackupDrBackupPlanAssociations_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataBackupDrBackupPlanAssociationsData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    id: core::default::Default::default(),
                    location: self.location,
                    project: core::default::Default::default(),
                    resource_type: core::default::Default::default(),
                }),
            }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataBackupDrBackupPlanAssociationsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrBackupPlanAssociationsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataBackupDrBackupPlanAssociationsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `associations` after provisioning.\nA list of the backup plan associations found."]
    pub fn associations(&self) -> ListRef<DataBackupDrBackupPlanAssociationsAssociationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.associations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location to list the backup plan associations from."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\nThe resource type of workload on which backup plan is applied. Examples include, \"compute.googleapis.com/Instance\", \"compute.googleapis.com/Disk\"."]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoElLastBackupErrorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoElLastBackupErrorEl {
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
impl ToListMappable
    for DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoElLastBackupErrorEl
{
    type O = BlockAssignable<
        DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoElLastBackupErrorEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoElLastBackupErrorEl
{}
impl BuildDataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoElLastBackupErrorEl {
    pub fn build(
        self,
    ) -> DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoElLastBackupErrorEl {
        DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoElLastBackupErrorEl {
            code: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoElLastBackupErrorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoElLastBackupErrorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoElLastBackupErrorElRef {
        DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoElLastBackupErrorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoElLastBackupErrorElRef {
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
pub struct DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    last_backup_error: Option<
        ListField<
            DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoElLastBackupErrorEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_backup_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_successful_backup_consistency_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_id: Option<PrimField<String>>,
}
impl DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoEl {
    #[doc = "Set the field `last_backup_error`.\n"]
    pub fn set_last_backup_error(
        mut self,
        v: impl Into<
            ListField<
                DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoElLastBackupErrorEl,
            >,
        >,
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
impl ToListMappable for DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoEl {
    type O = BlockAssignable<DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoEl {}
impl BuildDataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoEl {
    pub fn build(self) -> DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoEl {
        DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoEl {
            last_backup_error: core::default::Default::default(),
            last_backup_state: core::default::Default::default(),
            last_successful_backup_consistency_time: core::default::Default::default(),
            rule_id: core::default::Default::default(),
        }
    }
}
pub struct DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoElRef {
        DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `last_backup_error` after provisioning.\n"]
    pub fn last_backup_error(
        &self,
    ) -> ListRef<
        DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoElLastBackupErrorElRef,
    > {
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
pub struct DataBackupDrBackupPlanAssociationsAssociationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_plan: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_source: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules_config_info:
        Option<ListField<DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoEl>>,
}
impl DataBackupDrBackupPlanAssociationsAssociationsEl {
    #[doc = "Set the field `backup_plan`.\n"]
    pub fn set_backup_plan(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backup_plan = Some(v.into());
        self
    }
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `data_source`.\n"]
    pub fn set_data_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_source = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `resource`.\n"]
    pub fn set_resource(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource = Some(v.into());
        self
    }
    #[doc = "Set the field `rules_config_info`.\n"]
    pub fn set_rules_config_info(
        mut self,
        v: impl Into<ListField<DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoEl>>,
    ) -> Self {
        self.rules_config_info = Some(v.into());
        self
    }
}
impl ToListMappable for DataBackupDrBackupPlanAssociationsAssociationsEl {
    type O = BlockAssignable<DataBackupDrBackupPlanAssociationsAssociationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupDrBackupPlanAssociationsAssociationsEl {}
impl BuildDataBackupDrBackupPlanAssociationsAssociationsEl {
    pub fn build(self) -> DataBackupDrBackupPlanAssociationsAssociationsEl {
        DataBackupDrBackupPlanAssociationsAssociationsEl {
            backup_plan: core::default::Default::default(),
            create_time: core::default::Default::default(),
            data_source: core::default::Default::default(),
            name: core::default::Default::default(),
            resource: core::default::Default::default(),
            rules_config_info: core::default::Default::default(),
        }
    }
}
pub struct DataBackupDrBackupPlanAssociationsAssociationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrBackupPlanAssociationsAssociationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBackupDrBackupPlanAssociationsAssociationsElRef {
        DataBackupDrBackupPlanAssociationsAssociationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupDrBackupPlanAssociationsAssociationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup_plan` after provisioning.\n"]
    pub fn backup_plan(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup_plan", self.base))
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `data_source` after provisioning.\n"]
    pub fn data_source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_source", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `resource` after provisioning.\n"]
    pub fn resource(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }
    #[doc = "Get a reference to the value of field `rules_config_info` after provisioning.\n"]
    pub fn rules_config_info(
        &self,
    ) -> ListRef<DataBackupDrBackupPlanAssociationsAssociationsElRulesConfigInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules_config_info", self.base),
        )
    }
}
