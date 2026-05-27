use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataBackupDrBackupData {
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
    project: PrimField<String>,
}
struct DataBackupDrBackup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBackupDrBackupData>,
}
#[derive(Clone)]
pub struct DataBackupDrBackup(Rc<DataBackupDrBackup_>);
impl DataBackupDrBackup {
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
    #[doc = "Get a reference to the value of field `backup_vault_id` after provisioning.\n"]
    pub fn backup_vault_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_vault_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backups` after provisioning.\nList of all backups under data source."]
    pub fn backups(&self) -> ListRef<DataBackupDrBackupBackupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the backup was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\n"]
    pub fn data_source_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of resource"]
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
}
impl Referable for DataBackupDrBackup {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataBackupDrBackup {}
impl ToListMappable for DataBackupDrBackup {
    type O = ListRef<DataBackupDrBackupRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataBackupDrBackup_ {
    fn extract_datasource_type(&self) -> String {
        "google_backup_dr_backup".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataBackupDrBackup {
    pub tf_id: String,
    #[doc = ""]
    pub backup_vault_id: PrimField<String>,
    #[doc = ""]
    pub data_source_id: PrimField<String>,
    #[doc = ""]
    pub location: PrimField<String>,
    #[doc = ""]
    pub project: PrimField<String>,
}
impl BuildDataBackupDrBackup {
    pub fn build(self, stack: &mut Stack) -> DataBackupDrBackup {
        let out = DataBackupDrBackup(Rc::new(DataBackupDrBackup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBackupDrBackupData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                backup_vault_id: self.backup_vault_id,
                data_source_id: self.data_source_id,
                id: core::default::Default::default(),
                location: self.location,
                project: self.project,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataBackupDrBackupRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrBackupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataBackupDrBackupRef {
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
    #[doc = "Get a reference to the value of field `backups` after provisioning.\nList of all backups under data source."]
    pub fn backups(&self) -> ListRef<DataBackupDrBackupBackupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the backup was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\n"]
    pub fn data_source_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of resource"]
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
}
#[derive(Serialize)]
pub struct DataBackupDrBackupBackupsElComputeInstanceBackupPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_flush: Option<PrimField<bool>>,
}
impl DataBackupDrBackupBackupsElComputeInstanceBackupPropertiesEl {
    #[doc = "Set the field `guest_flush`.\n"]
    pub fn set_guest_flush(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.guest_flush = Some(v.into());
        self
    }
}
impl ToListMappable for DataBackupDrBackupBackupsElComputeInstanceBackupPropertiesEl {
    type O = BlockAssignable<DataBackupDrBackupBackupsElComputeInstanceBackupPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupDrBackupBackupsElComputeInstanceBackupPropertiesEl {}
impl BuildDataBackupDrBackupBackupsElComputeInstanceBackupPropertiesEl {
    pub fn build(self) -> DataBackupDrBackupBackupsElComputeInstanceBackupPropertiesEl {
        DataBackupDrBackupBackupsElComputeInstanceBackupPropertiesEl {
            guest_flush: core::default::Default::default(),
        }
    }
}
pub struct DataBackupDrBackupBackupsElComputeInstanceBackupPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrBackupBackupsElComputeInstanceBackupPropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBackupDrBackupBackupsElComputeInstanceBackupPropertiesElRef {
        DataBackupDrBackupBackupsElComputeInstanceBackupPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupDrBackupBackupsElComputeInstanceBackupPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `guest_flush` after provisioning.\n"]
    pub fn guest_flush(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.guest_flush", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBackupDrBackupBackupsElDiskBackupPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_flush: Option<PrimField<bool>>,
}
impl DataBackupDrBackupBackupsElDiskBackupPropertiesEl {
    #[doc = "Set the field `guest_flush`.\n"]
    pub fn set_guest_flush(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.guest_flush = Some(v.into());
        self
    }
}
impl ToListMappable for DataBackupDrBackupBackupsElDiskBackupPropertiesEl {
    type O = BlockAssignable<DataBackupDrBackupBackupsElDiskBackupPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupDrBackupBackupsElDiskBackupPropertiesEl {}
impl BuildDataBackupDrBackupBackupsElDiskBackupPropertiesEl {
    pub fn build(self) -> DataBackupDrBackupBackupsElDiskBackupPropertiesEl {
        DataBackupDrBackupBackupsElDiskBackupPropertiesEl {
            guest_flush: core::default::Default::default(),
        }
    }
}
pub struct DataBackupDrBackupBackupsElDiskBackupPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrBackupBackupsElDiskBackupPropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBackupDrBackupBackupsElDiskBackupPropertiesElRef {
        DataBackupDrBackupBackupsElDiskBackupPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupDrBackupBackupsElDiskBackupPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `guest_flush` after provisioning.\n"]
    pub fn guest_flush(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.guest_flush", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBackupDrBackupBackupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_vault_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_instance_backup_properties:
        Option<ListField<DataBackupDrBackupBackupsElComputeInstanceBackupPropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_source_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_backup_properties: Option<ListField<DataBackupDrBackupBackupsElDiskBackupPropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataBackupDrBackupBackupsEl {
    #[doc = "Set the field `backup_id`.\n"]
    pub fn set_backup_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backup_id = Some(v.into());
        self
    }
    #[doc = "Set the field `backup_vault_id`.\n"]
    pub fn set_backup_vault_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backup_vault_id = Some(v.into());
        self
    }
    #[doc = "Set the field `compute_instance_backup_properties`.\n"]
    pub fn set_compute_instance_backup_properties(
        mut self,
        v: impl Into<ListField<DataBackupDrBackupBackupsElComputeInstanceBackupPropertiesEl>>,
    ) -> Self {
        self.compute_instance_backup_properties = Some(v.into());
        self
    }
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `data_source_id`.\n"]
    pub fn set_data_source_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_source_id = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_backup_properties`.\n"]
    pub fn set_disk_backup_properties(
        mut self,
        v: impl Into<ListField<DataBackupDrBackupBackupsElDiskBackupPropertiesEl>>,
    ) -> Self {
        self.disk_backup_properties = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DataBackupDrBackupBackupsEl {
    type O = BlockAssignable<DataBackupDrBackupBackupsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupDrBackupBackupsEl {}
impl BuildDataBackupDrBackupBackupsEl {
    pub fn build(self) -> DataBackupDrBackupBackupsEl {
        DataBackupDrBackupBackupsEl {
            backup_id: core::default::Default::default(),
            backup_vault_id: core::default::Default::default(),
            compute_instance_backup_properties: core::default::Default::default(),
            create_time: core::default::Default::default(),
            data_source_id: core::default::Default::default(),
            disk_backup_properties: core::default::Default::default(),
            location: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DataBackupDrBackupBackupsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrBackupBackupsElRef {
    fn new(shared: StackShared, base: String) -> DataBackupDrBackupBackupsElRef {
        DataBackupDrBackupBackupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupDrBackupBackupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup_id` after provisioning.\n"]
    pub fn backup_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup_id", self.base))
    }
    #[doc = "Get a reference to the value of field `backup_vault_id` after provisioning.\n"]
    pub fn backup_vault_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_vault_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `compute_instance_backup_properties` after provisioning.\n"]
    pub fn compute_instance_backup_properties(
        &self,
    ) -> ListRef<DataBackupDrBackupBackupsElComputeInstanceBackupPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compute_instance_backup_properties", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\n"]
    pub fn data_source_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disk_backup_properties` after provisioning.\n"]
    pub fn disk_backup_properties(
        &self,
    ) -> ListRef<DataBackupDrBackupBackupsElDiskBackupPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk_backup_properties", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
