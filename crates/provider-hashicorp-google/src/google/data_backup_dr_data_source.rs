use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataBackupDrDataSourceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    backup_vault_id: PrimField<String>,
    data_source_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataBackupDrDataSource_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBackupDrDataSourceData>,
}
#[derive(Clone)]
pub struct DataBackupDrDataSource(Rc<DataBackupDrDataSource_>);
impl DataBackupDrDataSource {
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
    #[doc = "Get a reference to the value of field `backup_config_info` after provisioning.\nDetails of how the resource is configured for backup."]
    pub fn backup_config_info(&self) -> ListRef<DataBackupDrDataSourceBackupConfigInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backup_config_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_count` after provisioning.\nNumber of backups in the data source."]
    pub fn backup_count(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_vault_id` after provisioning.\n"]
    pub fn backup_vault_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_vault_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config_state` after provisioning.\nThe backup configuration state."]
    pub fn config_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.config_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the instance was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_source_backup_appliance_application` after provisioning.\nThe backed up resource is a backup appliance application."]
    pub fn data_source_backup_appliance_application(
        &self,
    ) -> ListRef<DataBackupDrDataSourceDataSourceBackupApplianceApplicationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.data_source_backup_appliance_application",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `data_source_gcp_resource` after provisioning.\nThe backed up resource is a Google Cloud resource.\n\t\tThe word 'DataSource' was included in the names to indicate that this is\n\t\tthe representation of the Google Cloud resource used within the\n\t\tDataSource object."]
    pub fn data_source_gcp_resource(
        &self,
    ) -> ListRef<DataBackupDrDataSourceDataSourceGcpResourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_source_gcp_resource", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\n"]
    pub fn data_source_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nServer specified ETag for the ManagementServer resource to prevent simultaneous updates from overwiting each other."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user provided metadata."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the datasource to create.\n\t\tIt must have the format \"projects/{project}/locations/{location}/backupVaults/{backupvault}/dataSources/{datasource}\".\n\t\t'{datasource}' cannot be changed after creation. It must be between 3-63 characters long and must be unique within the backup vault."]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe DataSource resource instance state."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `total_stored_bytes` after provisioning.\nThe number of bytes (metadata and data) stored in this datasource."]
    pub fn total_stored_bytes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_stored_bytes", self.extract_ref()),
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
impl Referable for DataBackupDrDataSource {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataBackupDrDataSource {}
impl ToListMappable for DataBackupDrDataSource {
    type O = ListRef<DataBackupDrDataSourceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataBackupDrDataSource_ {
    fn extract_datasource_type(&self) -> String {
        "google_backup_dr_data_source".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataBackupDrDataSource {
    pub tf_id: String,
    #[doc = ""]
    pub backup_vault_id: PrimField<String>,
    #[doc = ""]
    pub data_source_id: PrimField<String>,
    #[doc = ""]
    pub location: PrimField<String>,
}
impl BuildDataBackupDrDataSource {
    pub fn build(self, stack: &mut Stack) -> DataBackupDrDataSource {
        let out = DataBackupDrDataSource(Rc::new(DataBackupDrDataSource_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBackupDrDataSourceData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                backup_vault_id: self.backup_vault_id,
                data_source_id: self.data_source_id,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataBackupDrDataSourceRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrDataSourceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataBackupDrDataSourceRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `backup_config_info` after provisioning.\nDetails of how the resource is configured for backup."]
    pub fn backup_config_info(&self) -> ListRef<DataBackupDrDataSourceBackupConfigInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backup_config_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_count` after provisioning.\nNumber of backups in the data source."]
    pub fn backup_count(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_vault_id` after provisioning.\n"]
    pub fn backup_vault_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_vault_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config_state` after provisioning.\nThe backup configuration state."]
    pub fn config_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.config_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the instance was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_source_backup_appliance_application` after provisioning.\nThe backed up resource is a backup appliance application."]
    pub fn data_source_backup_appliance_application(
        &self,
    ) -> ListRef<DataBackupDrDataSourceDataSourceBackupApplianceApplicationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.data_source_backup_appliance_application",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `data_source_gcp_resource` after provisioning.\nThe backed up resource is a Google Cloud resource.\n\t\tThe word 'DataSource' was included in the names to indicate that this is\n\t\tthe representation of the Google Cloud resource used within the\n\t\tDataSource object."]
    pub fn data_source_gcp_resource(
        &self,
    ) -> ListRef<DataBackupDrDataSourceDataSourceGcpResourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_source_gcp_resource", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\n"]
    pub fn data_source_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nServer specified ETag for the ManagementServer resource to prevent simultaneous updates from overwiting each other."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user provided metadata."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the datasource to create.\n\t\tIt must have the format \"projects/{project}/locations/{location}/backupVaults/{backupvault}/dataSources/{datasource}\".\n\t\t'{datasource}' cannot be changed after creation. It must be between 3-63 characters long and must be unique within the backup vault."]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe DataSource resource instance state."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `total_stored_bytes` after provisioning.\nThe number of bytes (metadata and data) stored in this datasource."]
    pub fn total_stored_bytes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_stored_bytes", self.extract_ref()),
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
pub struct DataBackupDrDataSourceBackupConfigInfoElBackupApplianceBackupConfigEl {
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
impl DataBackupDrDataSourceBackupConfigInfoElBackupApplianceBackupConfigEl {
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
impl ToListMappable for DataBackupDrDataSourceBackupConfigInfoElBackupApplianceBackupConfigEl {
    type O = BlockAssignable<DataBackupDrDataSourceBackupConfigInfoElBackupApplianceBackupConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupDrDataSourceBackupConfigInfoElBackupApplianceBackupConfigEl {}
impl BuildDataBackupDrDataSourceBackupConfigInfoElBackupApplianceBackupConfigEl {
    pub fn build(self) -> DataBackupDrDataSourceBackupConfigInfoElBackupApplianceBackupConfigEl {
        DataBackupDrDataSourceBackupConfigInfoElBackupApplianceBackupConfigEl {
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
pub struct DataBackupDrDataSourceBackupConfigInfoElBackupApplianceBackupConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrDataSourceBackupConfigInfoElBackupApplianceBackupConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBackupDrDataSourceBackupConfigInfoElBackupApplianceBackupConfigElRef {
        DataBackupDrDataSourceBackupConfigInfoElBackupApplianceBackupConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupDrDataSourceBackupConfigInfoElBackupApplianceBackupConfigElRef {
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
pub struct DataBackupDrDataSourceBackupConfigInfoElGcpBackupConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_plan: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_plan_association: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_plan_description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_plan_rules: Option<ListField<PrimField<String>>>,
}
impl DataBackupDrDataSourceBackupConfigInfoElGcpBackupConfigEl {
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
impl ToListMappable for DataBackupDrDataSourceBackupConfigInfoElGcpBackupConfigEl {
    type O = BlockAssignable<DataBackupDrDataSourceBackupConfigInfoElGcpBackupConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupDrDataSourceBackupConfigInfoElGcpBackupConfigEl {}
impl BuildDataBackupDrDataSourceBackupConfigInfoElGcpBackupConfigEl {
    pub fn build(self) -> DataBackupDrDataSourceBackupConfigInfoElGcpBackupConfigEl {
        DataBackupDrDataSourceBackupConfigInfoElGcpBackupConfigEl {
            backup_plan: core::default::Default::default(),
            backup_plan_association: core::default::Default::default(),
            backup_plan_description: core::default::Default::default(),
            backup_plan_rules: core::default::Default::default(),
        }
    }
}
pub struct DataBackupDrDataSourceBackupConfigInfoElGcpBackupConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrDataSourceBackupConfigInfoElGcpBackupConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBackupDrDataSourceBackupConfigInfoElGcpBackupConfigElRef {
        DataBackupDrDataSourceBackupConfigInfoElGcpBackupConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupDrDataSourceBackupConfigInfoElGcpBackupConfigElRef {
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
pub struct DataBackupDrDataSourceBackupConfigInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_appliance_backup_config:
        Option<ListField<DataBackupDrDataSourceBackupConfigInfoElBackupApplianceBackupConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_backup_config: Option<ListField<DataBackupDrDataSourceBackupConfigInfoElGcpBackupConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_backup_error: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_backup_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_successful_backup_consistency_time: Option<PrimField<String>>,
}
impl DataBackupDrDataSourceBackupConfigInfoEl {
    #[doc = "Set the field `backup_appliance_backup_config`.\n"]
    pub fn set_backup_appliance_backup_config(
        mut self,
        v: impl Into<ListField<DataBackupDrDataSourceBackupConfigInfoElBackupApplianceBackupConfigEl>>,
    ) -> Self {
        self.backup_appliance_backup_config = Some(v.into());
        self
    }
    #[doc = "Set the field `gcp_backup_config`.\n"]
    pub fn set_gcp_backup_config(
        mut self,
        v: impl Into<ListField<DataBackupDrDataSourceBackupConfigInfoElGcpBackupConfigEl>>,
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
impl ToListMappable for DataBackupDrDataSourceBackupConfigInfoEl {
    type O = BlockAssignable<DataBackupDrDataSourceBackupConfigInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupDrDataSourceBackupConfigInfoEl {}
impl BuildDataBackupDrDataSourceBackupConfigInfoEl {
    pub fn build(self) -> DataBackupDrDataSourceBackupConfigInfoEl {
        DataBackupDrDataSourceBackupConfigInfoEl {
            backup_appliance_backup_config: core::default::Default::default(),
            gcp_backup_config: core::default::Default::default(),
            last_backup_error: core::default::Default::default(),
            last_backup_state: core::default::Default::default(),
            last_successful_backup_consistency_time: core::default::Default::default(),
        }
    }
}
pub struct DataBackupDrDataSourceBackupConfigInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrDataSourceBackupConfigInfoElRef {
    fn new(shared: StackShared, base: String) -> DataBackupDrDataSourceBackupConfigInfoElRef {
        DataBackupDrDataSourceBackupConfigInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupDrDataSourceBackupConfigInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup_appliance_backup_config` after provisioning.\n"]
    pub fn backup_appliance_backup_config(
        &self,
    ) -> ListRef<DataBackupDrDataSourceBackupConfigInfoElBackupApplianceBackupConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backup_appliance_backup_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gcp_backup_config` after provisioning.\n"]
    pub fn gcp_backup_config(
        &self,
    ) -> ListRef<DataBackupDrDataSourceBackupConfigInfoElGcpBackupConfigElRef> {
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
pub struct DataBackupDrDataSourceDataSourceBackupApplianceApplicationEl {
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
impl DataBackupDrDataSourceDataSourceBackupApplianceApplicationEl {
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
impl ToListMappable for DataBackupDrDataSourceDataSourceBackupApplianceApplicationEl {
    type O = BlockAssignable<DataBackupDrDataSourceDataSourceBackupApplianceApplicationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupDrDataSourceDataSourceBackupApplianceApplicationEl {}
impl BuildDataBackupDrDataSourceDataSourceBackupApplianceApplicationEl {
    pub fn build(self) -> DataBackupDrDataSourceDataSourceBackupApplianceApplicationEl {
        DataBackupDrDataSourceDataSourceBackupApplianceApplicationEl {
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
pub struct DataBackupDrDataSourceDataSourceBackupApplianceApplicationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrDataSourceDataSourceBackupApplianceApplicationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBackupDrDataSourceDataSourceBackupApplianceApplicationElRef {
        DataBackupDrDataSourceDataSourceBackupApplianceApplicationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupDrDataSourceDataSourceBackupApplianceApplicationElRef {
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
pub struct DataBackupDrDataSourceDataSourceGcpResourceElComputeInstanceDataSourcePropertiesEl {
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
impl DataBackupDrDataSourceDataSourceGcpResourceElComputeInstanceDataSourcePropertiesEl {
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
impl ToListMappable
    for DataBackupDrDataSourceDataSourceGcpResourceElComputeInstanceDataSourcePropertiesEl
{
    type O = BlockAssignable<
        DataBackupDrDataSourceDataSourceGcpResourceElComputeInstanceDataSourcePropertiesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupDrDataSourceDataSourceGcpResourceElComputeInstanceDataSourcePropertiesEl
{}
impl BuildDataBackupDrDataSourceDataSourceGcpResourceElComputeInstanceDataSourcePropertiesEl {
    pub fn build(
        self,
    ) -> DataBackupDrDataSourceDataSourceGcpResourceElComputeInstanceDataSourcePropertiesEl {
        DataBackupDrDataSourceDataSourceGcpResourceElComputeInstanceDataSourcePropertiesEl {
            description: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            name: core::default::Default::default(),
            total_disk_count: core::default::Default::default(),
            total_disk_size_gb: core::default::Default::default(),
        }
    }
}
pub struct DataBackupDrDataSourceDataSourceGcpResourceElComputeInstanceDataSourcePropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrDataSourceDataSourceGcpResourceElComputeInstanceDataSourcePropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBackupDrDataSourceDataSourceGcpResourceElComputeInstanceDataSourcePropertiesElRef {
        DataBackupDrDataSourceDataSourceGcpResourceElComputeInstanceDataSourcePropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupDrDataSourceDataSourceGcpResourceElComputeInstanceDataSourcePropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `total_disk_count` after provisioning.\n"]
    pub fn total_disk_count(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_disk_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `total_disk_size_gb` after provisioning.\n"]
    pub fn total_disk_size_gb(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_disk_size_gb", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataBackupDrDataSourceDataSourceGcpResourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_instance_data_source_properties: Option<
        ListField<
            DataBackupDrDataSourceDataSourceGcpResourceElComputeInstanceDataSourcePropertiesEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_resourcename: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataBackupDrDataSourceDataSourceGcpResourceEl {
    #[doc = "Set the field `compute_instance_data_source_properties`.\n"]
    pub fn set_compute_instance_data_source_properties(
        mut self,
        v: impl Into<
            ListField<
                DataBackupDrDataSourceDataSourceGcpResourceElComputeInstanceDataSourcePropertiesEl,
            >,
        >,
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
impl ToListMappable for DataBackupDrDataSourceDataSourceGcpResourceEl {
    type O = BlockAssignable<DataBackupDrDataSourceDataSourceGcpResourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupDrDataSourceDataSourceGcpResourceEl {}
impl BuildDataBackupDrDataSourceDataSourceGcpResourceEl {
    pub fn build(self) -> DataBackupDrDataSourceDataSourceGcpResourceEl {
        DataBackupDrDataSourceDataSourceGcpResourceEl {
            compute_instance_data_source_properties: core::default::Default::default(),
            gcp_resourcename: core::default::Default::default(),
            location: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataBackupDrDataSourceDataSourceGcpResourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrDataSourceDataSourceGcpResourceElRef {
    fn new(shared: StackShared, base: String) -> DataBackupDrDataSourceDataSourceGcpResourceElRef {
        DataBackupDrDataSourceDataSourceGcpResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupDrDataSourceDataSourceGcpResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `compute_instance_data_source_properties` after provisioning.\n"]
    pub fn compute_instance_data_source_properties(
        &self,
    ) -> ListRef<
        DataBackupDrDataSourceDataSourceGcpResourceElComputeInstanceDataSourcePropertiesElRef,
    > {
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
