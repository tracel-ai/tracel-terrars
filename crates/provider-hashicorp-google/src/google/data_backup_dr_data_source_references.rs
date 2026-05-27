use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataBackupDrDataSourceReferencesData {
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
struct DataBackupDrDataSourceReferences_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBackupDrDataSourceReferencesData>,
}
#[derive(Clone)]
pub struct DataBackupDrDataSourceReferences(Rc<DataBackupDrDataSourceReferences_>);
impl DataBackupDrDataSourceReferences {
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
    #[doc = "Get a reference to the value of field `data_source_references` after provisioning.\nA list of the data source references found."]
    pub fn data_source_references(
        &self,
    ) -> ListRef<DataBackupDrDataSourceReferencesDataSourceReferencesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_source_references", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location to list the data source references from."]
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
impl Referable for DataBackupDrDataSourceReferences {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataBackupDrDataSourceReferences {}
impl ToListMappable for DataBackupDrDataSourceReferences {
    type O = ListRef<DataBackupDrDataSourceReferencesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataBackupDrDataSourceReferences_ {
    fn extract_datasource_type(&self) -> String {
        "google_backup_dr_data_source_references".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataBackupDrDataSourceReferences {
    pub tf_id: String,
    #[doc = "The location to list the data source references from."]
    pub location: PrimField<String>,
}
impl BuildDataBackupDrDataSourceReferences {
    pub fn build(self, stack: &mut Stack) -> DataBackupDrDataSourceReferences {
        let out = DataBackupDrDataSourceReferences(Rc::new(DataBackupDrDataSourceReferences_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBackupDrDataSourceReferencesData {
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
pub struct DataBackupDrDataSourceReferencesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrDataSourceReferencesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataBackupDrDataSourceReferencesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `data_source_references` after provisioning.\nA list of the data source references found."]
    pub fn data_source_references(
        &self,
    ) -> ListRef<DataBackupDrDataSourceReferencesDataSourceReferencesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_source_references", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location to list the data source references from."]
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
pub struct DataBackupDrDataSourceReferencesDataSourceReferencesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_config_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_source: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_resource_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_backup_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_successful_backup_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_type: Option<PrimField<String>>,
}
impl DataBackupDrDataSourceReferencesDataSourceReferencesEl {
    #[doc = "Set the field `backup_config_state`.\n"]
    pub fn set_backup_config_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backup_config_state = Some(v.into());
        self
    }
    #[doc = "Set the field `backup_count`.\n"]
    pub fn set_backup_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.backup_count = Some(v.into());
        self
    }
    #[doc = "Set the field `data_source`.\n"]
    pub fn set_data_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_source = Some(v.into());
        self
    }
    #[doc = "Set the field `gcp_resource_name`.\n"]
    pub fn set_gcp_resource_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_resource_name = Some(v.into());
        self
    }
    #[doc = "Set the field `last_backup_state`.\n"]
    pub fn set_last_backup_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_backup_state = Some(v.into());
        self
    }
    #[doc = "Set the field `last_successful_backup_time`.\n"]
    pub fn set_last_successful_backup_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_successful_backup_time = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_type = Some(v.into());
        self
    }
}
impl ToListMappable for DataBackupDrDataSourceReferencesDataSourceReferencesEl {
    type O = BlockAssignable<DataBackupDrDataSourceReferencesDataSourceReferencesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupDrDataSourceReferencesDataSourceReferencesEl {}
impl BuildDataBackupDrDataSourceReferencesDataSourceReferencesEl {
    pub fn build(self) -> DataBackupDrDataSourceReferencesDataSourceReferencesEl {
        DataBackupDrDataSourceReferencesDataSourceReferencesEl {
            backup_config_state: core::default::Default::default(),
            backup_count: core::default::Default::default(),
            data_source: core::default::Default::default(),
            gcp_resource_name: core::default::Default::default(),
            last_backup_state: core::default::Default::default(),
            last_successful_backup_time: core::default::Default::default(),
            name: core::default::Default::default(),
            resource_type: core::default::Default::default(),
        }
    }
}
pub struct DataBackupDrDataSourceReferencesDataSourceReferencesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrDataSourceReferencesDataSourceReferencesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBackupDrDataSourceReferencesDataSourceReferencesElRef {
        DataBackupDrDataSourceReferencesDataSourceReferencesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupDrDataSourceReferencesDataSourceReferencesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup_config_state` after provisioning.\n"]
    pub fn backup_config_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_config_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `backup_count` after provisioning.\n"]
    pub fn backup_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup_count", self.base))
    }
    #[doc = "Get a reference to the value of field `data_source` after provisioning.\n"]
    pub fn data_source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_source", self.base))
    }
    #[doc = "Get a reference to the value of field `gcp_resource_name` after provisioning.\n"]
    pub fn gcp_resource_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_resource_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `last_backup_state` after provisioning.\n"]
    pub fn last_backup_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_backup_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `last_successful_backup_time` after provisioning.\n"]
    pub fn last_successful_backup_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_successful_backup_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.base),
        )
    }
}
