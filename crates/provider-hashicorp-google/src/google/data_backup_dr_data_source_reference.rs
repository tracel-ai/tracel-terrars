use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataBackupDrDataSourceReferenceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    data_source_reference_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataBackupDrDataSourceReference_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBackupDrDataSourceReferenceData>,
}
#[derive(Clone)]
pub struct DataBackupDrDataSourceReference(Rc<DataBackupDrDataSourceReference_>);
impl DataBackupDrDataSourceReference {
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
    #[doc = "Get a reference to the value of field `backup_config_state` after provisioning.\nThe state of the backup config for the data source."]
    pub fn backup_config_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_config_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_count` after provisioning.\nThe number of backups for the data source."]
    pub fn backup_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_source` after provisioning.\nThe underlying data source resource."]
    pub fn data_source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_source_reference_id` after provisioning.\nThe `id` of the data source reference."]
    pub fn data_source_reference_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source_reference_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcp_resource_name` after provisioning.\nThe GCP resource name for the data source."]
    pub fn gcp_resource_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_resource_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_backup_state` after provisioning.\nThe state of the last backup."]
    pub fn last_backup_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_backup_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_successful_backup_time` after provisioning.\nThe last time a successful backup was made."]
    pub fn last_successful_backup_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_successful_backup_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the data source reference."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.extract_ref()),
        )
    }
}
impl Referable for DataBackupDrDataSourceReference {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataBackupDrDataSourceReference {}
impl ToListMappable for DataBackupDrDataSourceReference {
    type O = ListRef<DataBackupDrDataSourceReferenceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataBackupDrDataSourceReference_ {
    fn extract_datasource_type(&self) -> String {
        "google_backup_dr_data_source_reference".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataBackupDrDataSourceReference {
    pub tf_id: String,
    #[doc = "The `id` of the data source reference."]
    pub data_source_reference_id: PrimField<String>,
    #[doc = "The location of the data source reference."]
    pub location: PrimField<String>,
}
impl BuildDataBackupDrDataSourceReference {
    pub fn build(self, stack: &mut Stack) -> DataBackupDrDataSourceReference {
        let out = DataBackupDrDataSourceReference(Rc::new(DataBackupDrDataSourceReference_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBackupDrDataSourceReferenceData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                data_source_reference_id: self.data_source_reference_id,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataBackupDrDataSourceReferenceRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrDataSourceReferenceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataBackupDrDataSourceReferenceRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `backup_config_state` after provisioning.\nThe state of the backup config for the data source."]
    pub fn backup_config_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_config_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_count` after provisioning.\nThe number of backups for the data source."]
    pub fn backup_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_source` after provisioning.\nThe underlying data source resource."]
    pub fn data_source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_source_reference_id` after provisioning.\nThe `id` of the data source reference."]
    pub fn data_source_reference_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source_reference_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcp_resource_name` after provisioning.\nThe GCP resource name for the data source."]
    pub fn gcp_resource_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_resource_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_backup_state` after provisioning.\nThe state of the last backup."]
    pub fn last_backup_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_backup_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_successful_backup_time` after provisioning.\nThe last time a successful backup was made."]
    pub fn last_successful_backup_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_successful_backup_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the data source reference."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.extract_ref()),
        )
    }
}
