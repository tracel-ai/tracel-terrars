use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataBackupDrDataSourcesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    backup_vault_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_by: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataBackupDrDataSources_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBackupDrDataSourcesData>,
}
#[derive(Clone)]
pub struct DataBackupDrDataSources(Rc<DataBackupDrDataSources_>);
impl DataBackupDrDataSources {
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
    #[doc = "Set the field `filter`.\nThe filter to apply to list results."]
    pub fn set_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `order_by`.\nThe order to sort results by."]
    pub fn set_order_by(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().order_by = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `backup_vault_id` after provisioning.\n"]
    pub fn backup_vault_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_vault_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_sources` after provisioning.\nThe list of DataSources found."]
    pub fn data_sources(&self) -> ListRef<DataBackupDrDataSourcesDataSourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_sources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nThe filter to apply to list results."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `order_by` after provisioning.\nThe order to sort results by."]
    pub fn order_by(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.order_by", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
}
impl Referable for DataBackupDrDataSources {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataBackupDrDataSources {}
impl ToListMappable for DataBackupDrDataSources {
    type O = ListRef<DataBackupDrDataSourcesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataBackupDrDataSources_ {
    fn extract_datasource_type(&self) -> String {
        "google_backup_dr_data_sources".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataBackupDrDataSources {
    pub tf_id: String,
    #[doc = ""]
    pub backup_vault_id: PrimField<String>,
    #[doc = ""]
    pub location: PrimField<String>,
}
impl BuildDataBackupDrDataSources {
    pub fn build(self, stack: &mut Stack) -> DataBackupDrDataSources {
        let out = DataBackupDrDataSources(Rc::new(DataBackupDrDataSources_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBackupDrDataSourcesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                backup_vault_id: self.backup_vault_id,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                order_by: core::default::Default::default(),
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataBackupDrDataSourcesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrDataSourcesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataBackupDrDataSourcesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `backup_vault_id` after provisioning.\n"]
    pub fn backup_vault_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_vault_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_sources` after provisioning.\nThe list of DataSources found."]
    pub fn data_sources(&self) -> ListRef<DataBackupDrDataSourcesDataSourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_sources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nThe filter to apply to list results."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `order_by` after provisioning.\nThe order to sort results by."]
    pub fn order_by(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.order_by", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElBackupApplianceBackupConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    application_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_appliance_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_appliance_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sla_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slp_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slt_name: Option<PrimField<String>>,
}
impl DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElBackupApplianceBackupConfigEl {
    #[doc = "Set the field `application_name`.\n"]
    pub fn set_application_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.application_name = Some(v.into());
        self
    }
    #[doc = "Set the field `backup_appliance_id`.\n"]
    pub fn set_backup_appliance_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backup_appliance_id = Some(v.into());
        self
    }
    #[doc = "Set the field `backup_appliance_name`.\n"]
    pub fn set_backup_appliance_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backup_appliance_name = Some(v.into());
        self
    }
    #[doc = "Set the field `host_name`.\n"]
    pub fn set_host_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_name = Some(v.into());
        self
    }
    #[doc = "Set the field `sla_id`.\n"]
    pub fn set_sla_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sla_id = Some(v.into());
        self
    }
    #[doc = "Set the field `slp_name`.\n"]
    pub fn set_slp_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.slp_name = Some(v.into());
        self
    }
    #[doc = "Set the field `slt_name`.\n"]
    pub fn set_slt_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.slt_name = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElBackupApplianceBackupConfigEl
{
    type O = BlockAssignable<
        DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElBackupApplianceBackupConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupDrDataSourcesDataSourcesElBackupConfigInfoElBackupApplianceBackupConfigEl
{}
impl BuildDataBackupDrDataSourcesDataSourcesElBackupConfigInfoElBackupApplianceBackupConfigEl {
    pub fn build(
        self,
    ) -> DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElBackupApplianceBackupConfigEl {
        DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElBackupApplianceBackupConfigEl {
            application_name: core::default::Default::default(),
            backup_appliance_id: core::default::Default::default(),
            backup_appliance_name: core::default::Default::default(),
            host_name: core::default::Default::default(),
            sla_id: core::default::Default::default(),
            slp_name: core::default::Default::default(),
            slt_name: core::default::Default::default(),
        }
    }
}
pub struct DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElBackupApplianceBackupConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElBackupApplianceBackupConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElBackupApplianceBackupConfigElRef
    {
        DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElBackupApplianceBackupConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElBackupApplianceBackupConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `application_name` after provisioning.\n"]
    pub fn application_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `backup_appliance_id` after provisioning.\n"]
    pub fn backup_appliance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_appliance_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `backup_appliance_name` after provisioning.\n"]
    pub fn backup_appliance_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_appliance_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `host_name` after provisioning.\n"]
    pub fn host_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_name", self.base))
    }
    #[doc = "Get a reference to the value of field `sla_id` after provisioning.\n"]
    pub fn sla_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sla_id", self.base))
    }
    #[doc = "Get a reference to the value of field `slp_name` after provisioning.\n"]
    pub fn slp_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slp_name", self.base))
    }
    #[doc = "Get a reference to the value of field `slt_name` after provisioning.\n"]
    pub fn slt_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slt_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElGcpBackupConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_plan: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_plan_association: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_plan_description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_plan_rules: Option<ListField<PrimField<String>>>,
}
impl DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElGcpBackupConfigEl {
    #[doc = "Set the field `backup_plan`.\n"]
    pub fn set_backup_plan(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backup_plan = Some(v.into());
        self
    }
    #[doc = "Set the field `backup_plan_association`.\n"]
    pub fn set_backup_plan_association(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backup_plan_association = Some(v.into());
        self
    }
    #[doc = "Set the field `backup_plan_description`.\n"]
    pub fn set_backup_plan_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backup_plan_description = Some(v.into());
        self
    }
    #[doc = "Set the field `backup_plan_rules`.\n"]
    pub fn set_backup_plan_rules(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.backup_plan_rules = Some(v.into());
        self
    }
}
impl ToListMappable for DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElGcpBackupConfigEl {
    type O =
        BlockAssignable<DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElGcpBackupConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupDrDataSourcesDataSourcesElBackupConfigInfoElGcpBackupConfigEl {}
impl BuildDataBackupDrDataSourcesDataSourcesElBackupConfigInfoElGcpBackupConfigEl {
    pub fn build(self) -> DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElGcpBackupConfigEl {
        DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElGcpBackupConfigEl {
            backup_plan: core::default::Default::default(),
            backup_plan_association: core::default::Default::default(),
            backup_plan_description: core::default::Default::default(),
            backup_plan_rules: core::default::Default::default(),
        }
    }
}
pub struct DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElGcpBackupConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElGcpBackupConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElGcpBackupConfigElRef {
        DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElGcpBackupConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElGcpBackupConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup_plan` after provisioning.\n"]
    pub fn backup_plan(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup_plan", self.base))
    }
    #[doc = "Get a reference to the value of field `backup_plan_association` after provisioning.\n"]
    pub fn backup_plan_association(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_plan_association", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `backup_plan_description` after provisioning.\n"]
    pub fn backup_plan_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_plan_description", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `backup_plan_rules` after provisioning.\n"]
    pub fn backup_plan_rules(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backup_plan_rules", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataBackupDrDataSourcesDataSourcesElBackupConfigInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_appliance_backup_config: Option<
        ListField<
            DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElBackupApplianceBackupConfigEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_backup_config:
        Option<ListField<DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElGcpBackupConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_backup_error: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_backup_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_successful_backup_consistency_time: Option<PrimField<String>>,
}
impl DataBackupDrDataSourcesDataSourcesElBackupConfigInfoEl {
    #[doc = "Set the field `backup_appliance_backup_config`.\n"]
    pub fn set_backup_appliance_backup_config(
        mut self,
        v: impl Into<
            ListField<
                DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElBackupApplianceBackupConfigEl,
            >,
        >,
    ) -> Self {
        self.backup_appliance_backup_config = Some(v.into());
        self
    }
    #[doc = "Set the field `gcp_backup_config`.\n"]
    pub fn set_gcp_backup_config(
        mut self,
        v: impl Into<ListField<DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElGcpBackupConfigEl>>,
    ) -> Self {
        self.gcp_backup_config = Some(v.into());
        self
    }
    #[doc = "Set the field `last_backup_error`.\n"]
    pub fn set_last_backup_error(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
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
}
impl ToListMappable for DataBackupDrDataSourcesDataSourcesElBackupConfigInfoEl {
    type O = BlockAssignable<DataBackupDrDataSourcesDataSourcesElBackupConfigInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupDrDataSourcesDataSourcesElBackupConfigInfoEl {}
impl BuildDataBackupDrDataSourcesDataSourcesElBackupConfigInfoEl {
    pub fn build(self) -> DataBackupDrDataSourcesDataSourcesElBackupConfigInfoEl {
        DataBackupDrDataSourcesDataSourcesElBackupConfigInfoEl {
            backup_appliance_backup_config: core::default::Default::default(),
            gcp_backup_config: core::default::Default::default(),
            last_backup_error: core::default::Default::default(),
            last_backup_state: core::default::Default::default(),
            last_successful_backup_consistency_time: core::default::Default::default(),
        }
    }
}
pub struct DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElRef {
        DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup_appliance_backup_config` after provisioning.\n"]
    pub fn backup_appliance_backup_config(
        &self,
    ) -> ListRef<
        DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElBackupApplianceBackupConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backup_appliance_backup_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gcp_backup_config` after provisioning.\n"]
    pub fn gcp_backup_config(
        &self,
    ) -> ListRef<DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElGcpBackupConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcp_backup_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `last_backup_error` after provisioning.\n"]
    pub fn last_backup_error(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
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
}
#[derive(Serialize)]
pub struct DataBackupDrDataSourcesDataSourcesElDataSourceBackupApplianceApplicationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    appliance_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_appliance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataBackupDrDataSourcesDataSourcesElDataSourceBackupApplianceApplicationEl {
    #[doc = "Set the field `appliance_id`.\n"]
    pub fn set_appliance_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.appliance_id = Some(v.into());
        self
    }
    #[doc = "Set the field `application_id`.\n"]
    pub fn set_application_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.application_id = Some(v.into());
        self
    }
    #[doc = "Set the field `application_name`.\n"]
    pub fn set_application_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.application_name = Some(v.into());
        self
    }
    #[doc = "Set the field `backup_appliance`.\n"]
    pub fn set_backup_appliance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backup_appliance = Some(v.into());
        self
    }
    #[doc = "Set the field `host_id`.\n"]
    pub fn set_host_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_id = Some(v.into());
        self
    }
    #[doc = "Set the field `hostname`.\n"]
    pub fn set_hostname(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hostname = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataBackupDrDataSourcesDataSourcesElDataSourceBackupApplianceApplicationEl {
    type O =
        BlockAssignable<DataBackupDrDataSourcesDataSourcesElDataSourceBackupApplianceApplicationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupDrDataSourcesDataSourcesElDataSourceBackupApplianceApplicationEl {}
impl BuildDataBackupDrDataSourcesDataSourcesElDataSourceBackupApplianceApplicationEl {
    pub fn build(
        self,
    ) -> DataBackupDrDataSourcesDataSourcesElDataSourceBackupApplianceApplicationEl {
        DataBackupDrDataSourcesDataSourcesElDataSourceBackupApplianceApplicationEl {
            appliance_id: core::default::Default::default(),
            application_id: core::default::Default::default(),
            application_name: core::default::Default::default(),
            backup_appliance: core::default::Default::default(),
            host_id: core::default::Default::default(),
            hostname: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataBackupDrDataSourcesDataSourcesElDataSourceBackupApplianceApplicationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrDataSourcesDataSourcesElDataSourceBackupApplianceApplicationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBackupDrDataSourcesDataSourcesElDataSourceBackupApplianceApplicationElRef {
        DataBackupDrDataSourcesDataSourcesElDataSourceBackupApplianceApplicationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupDrDataSourcesDataSourcesElDataSourceBackupApplianceApplicationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `appliance_id` after provisioning.\n"]
    pub fn appliance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.appliance_id", self.base))
    }
    #[doc = "Get a reference to the value of field `application_id` after provisioning.\n"]
    pub fn application_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `application_name` after provisioning.\n"]
    pub fn application_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `backup_appliance` after provisioning.\n"]
    pub fn backup_appliance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_appliance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `host_id` after provisioning.\n"]
    pub fn host_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_id", self.base))
    }
    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceElComputeInstanceDataSourcePropertiesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_disk_count: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_disk_size_gb: Option<PrimField<String>>,
}
impl
    DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceElComputeInstanceDataSourcePropertiesEl
{
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `total_disk_count`.\n"]
    pub fn set_total_disk_count(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.total_disk_count = Some(v.into());
        self
    }
    #[doc = "Set the field `total_disk_size_gb`.\n"]
    pub fn set_total_disk_size_gb(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.total_disk_size_gb = Some(v.into());
        self
    }
}
impl ToListMappable for DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceElComputeInstanceDataSourcePropertiesEl { type O = BlockAssignable < DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceElComputeInstanceDataSourcePropertiesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceElComputeInstanceDataSourcePropertiesEl
{}
impl BuildDataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceElComputeInstanceDataSourcePropertiesEl { pub fn build (self) -> DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceElComputeInstanceDataSourcePropertiesEl { DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceElComputeInstanceDataSourcePropertiesEl { description : core :: default :: Default :: default () , machine_type : core :: default :: Default :: default () , name : core :: default :: Default :: default () , total_disk_count : core :: default :: Default :: default () , total_disk_size_gb : core :: default :: Default :: default () , } } }
pub struct DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceElComputeInstanceDataSourcePropertiesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceElComputeInstanceDataSourcePropertiesElRef { fn new (shared : StackShared , base : String) -> DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceElComputeInstanceDataSourcePropertiesElRef { DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceElComputeInstanceDataSourcePropertiesElRef { shared : shared , base : base . to_string () , } } }
impl DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceElComputeInstanceDataSourcePropertiesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `description` after provisioning.\n"] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `machine_type` after provisioning.\n"] pub fn machine_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.machine_type" , self . base)) } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `total_disk_count` after provisioning.\n"] pub fn total_disk_count (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.total_disk_count" , self . base)) } # [doc = "Get a reference to the value of field `total_disk_size_gb` after provisioning.\n"] pub fn total_disk_size_gb (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.total_disk_size_gb" , self . base)) } }
#[derive(Serialize)]
pub struct DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceEl { # [serde (skip_serializing_if = "Option::is_none")] compute_instance_data_source_properties : Option < ListField < DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceElComputeInstanceDataSourcePropertiesEl > > , # [serde (skip_serializing_if = "Option::is_none")] gcp_resourcename : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] location : Option < PrimField < String > > , # [serde (rename = "type" , skip_serializing_if = "Option::is_none")] type_ : Option < PrimField < String > > , }
impl DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceEl {
    #[doc = "Set the field `compute_instance_data_source_properties`.\n"]
    pub fn set_compute_instance_data_source_properties(
        mut self,
        v : impl Into < ListField < DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceElComputeInstanceDataSourcePropertiesEl > >,
    ) -> Self {
        self.compute_instance_data_source_properties = Some(v.into());
        self
    }
    #[doc = "Set the field `gcp_resourcename`.\n"]
    pub fn set_gcp_resourcename(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_resourcename = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceEl {
    type O = BlockAssignable<DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceEl {}
impl BuildDataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceEl {
    pub fn build(self) -> DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceEl {
        DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceEl {
            compute_instance_data_source_properties: core::default::Default::default(),
            gcp_resourcename: core::default::Default::default(),
            location: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceElRef {
        DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `compute_instance_data_source_properties` after provisioning.\n"]    pub fn compute_instance_data_source_properties (& self) -> ListRef < DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceElComputeInstanceDataSourcePropertiesElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.compute_instance_data_source_properties", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gcp_resourcename` after provisioning.\n"]
    pub fn gcp_resourcename(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_resourcename", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBackupDrDataSourcesDataSourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_config_info: Option<ListField<DataBackupDrDataSourcesDataSourcesElBackupConfigInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_count: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_source_backup_appliance_application: Option<
        ListField<DataBackupDrDataSourcesDataSourcesElDataSourceBackupApplianceApplicationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_source_gcp_resource:
        Option<ListField<DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    etag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_stored_bytes: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
}
impl DataBackupDrDataSourcesDataSourcesEl {
    #[doc = "Set the field `backup_config_info`.\n"]
    pub fn set_backup_config_info(
        mut self,
        v: impl Into<ListField<DataBackupDrDataSourcesDataSourcesElBackupConfigInfoEl>>,
    ) -> Self {
        self.backup_config_info = Some(v.into());
        self
    }
    #[doc = "Set the field `backup_count`.\n"]
    pub fn set_backup_count(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backup_count = Some(v.into());
        self
    }
    #[doc = "Set the field `config_state`.\n"]
    pub fn set_config_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.config_state = Some(v.into());
        self
    }
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `data_source_backup_appliance_application`.\n"]
    pub fn set_data_source_backup_appliance_application(
        mut self,
        v: impl Into<
            ListField<DataBackupDrDataSourcesDataSourcesElDataSourceBackupApplianceApplicationEl>,
        >,
    ) -> Self {
        self.data_source_backup_appliance_application = Some(v.into());
        self
    }
    #[doc = "Set the field `data_source_gcp_resource`.\n"]
    pub fn set_data_source_gcp_resource(
        mut self,
        v: impl Into<ListField<DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceEl>>,
    ) -> Self {
        self.data_source_gcp_resource = Some(v.into());
        self
    }
    #[doc = "Set the field `etag`.\n"]
    pub fn set_etag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.etag = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `total_stored_bytes`.\n"]
    pub fn set_total_stored_bytes(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.total_stored_bytes = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataBackupDrDataSourcesDataSourcesEl {
    type O = BlockAssignable<DataBackupDrDataSourcesDataSourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupDrDataSourcesDataSourcesEl {}
impl BuildDataBackupDrDataSourcesDataSourcesEl {
    pub fn build(self) -> DataBackupDrDataSourcesDataSourcesEl {
        DataBackupDrDataSourcesDataSourcesEl {
            backup_config_info: core::default::Default::default(),
            backup_count: core::default::Default::default(),
            config_state: core::default::Default::default(),
            create_time: core::default::Default::default(),
            data_source_backup_appliance_application: core::default::Default::default(),
            data_source_gcp_resource: core::default::Default::default(),
            etag: core::default::Default::default(),
            labels: core::default::Default::default(),
            name: core::default::Default::default(),
            state: core::default::Default::default(),
            total_stored_bytes: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}
pub struct DataBackupDrDataSourcesDataSourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrDataSourcesDataSourcesElRef {
    fn new(shared: StackShared, base: String) -> DataBackupDrDataSourcesDataSourcesElRef {
        DataBackupDrDataSourcesDataSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupDrDataSourcesDataSourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup_config_info` after provisioning.\n"]
    pub fn backup_config_info(
        &self,
    ) -> ListRef<DataBackupDrDataSourcesDataSourcesElBackupConfigInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backup_config_info", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `backup_count` after provisioning.\n"]
    pub fn backup_count(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup_count", self.base))
    }
    #[doc = "Get a reference to the value of field `config_state` after provisioning.\n"]
    pub fn config_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.config_state", self.base))
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `data_source_backup_appliance_application` after provisioning.\n"]
    pub fn data_source_backup_appliance_application(
        &self,
    ) -> ListRef<DataBackupDrDataSourcesDataSourcesElDataSourceBackupApplianceApplicationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_source_backup_appliance_application", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_source_gcp_resource` after provisioning.\n"]
    pub fn data_source_gcp_resource(
        &self,
    ) -> ListRef<DataBackupDrDataSourcesDataSourcesElDataSourceGcpResourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_source_gcp_resource", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.base))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `total_stored_bytes` after provisioning.\n"]
    pub fn total_stored_bytes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_stored_bytes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
