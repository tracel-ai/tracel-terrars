use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct StorageInsightsDatasetConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    activity_data_retention_period_days: Option<PrimField<f64>>,
    dataset_config_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_newly_created_buckets: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link_dataset: Option<PrimField<bool>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organization_number: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organization_scope: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    retention_period_days: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_cloud_storage_buckets:
        Option<Vec<StorageInsightsDatasetConfigExcludeCloudStorageBucketsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_cloud_storage_locations:
        Option<Vec<StorageInsightsDatasetConfigExcludeCloudStorageLocationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity: Option<Vec<StorageInsightsDatasetConfigIdentityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_cloud_storage_buckets:
        Option<Vec<StorageInsightsDatasetConfigIncludeCloudStorageBucketsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_cloud_storage_locations:
        Option<Vec<StorageInsightsDatasetConfigIncludeCloudStorageLocationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_folders: Option<Vec<StorageInsightsDatasetConfigSourceFoldersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_projects: Option<Vec<StorageInsightsDatasetConfigSourceProjectsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<StorageInsightsDatasetConfigTimeoutsEl>,
    dynamic: StorageInsightsDatasetConfigDynamic,
}
struct StorageInsightsDatasetConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<StorageInsightsDatasetConfigData>,
}
#[derive(Clone)]
pub struct StorageInsightsDatasetConfig(Rc<StorageInsightsDatasetConfig_>);
impl StorageInsightsDatasetConfig {
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
    #[doc = "Set the field `activity_data_retention_period_days`.\nNumber of days of activity data that must be retained. If not specified, retentionPeriodDays will be used. Set to 0 to turn off the activity data."]
    pub fn set_activity_data_retention_period_days(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().activity_data_retention_period_days = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nAn optional user-provided description for the dataset configuration with a maximum length of 256 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `include_newly_created_buckets`.\nIf set to true, the request includes all the newly created buckets in the dataset that meet the inclusion and exclusion rules."]
    pub fn set_include_newly_created_buckets(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_newly_created_buckets = Some(v.into());
        self
    }
    #[doc = "Set the field `link_dataset`.\nA boolean terraform only flag to link/unlink dataset.\n\nSetting this field to true while creation will automatically link the created dataset as an additional functionality.\n-> **Note** A dataset config resource can only be destroyed once it is unlinked,\nso users must set this field to false to unlink the dataset and destroy the dataset config resource."]
    pub fn set_link_dataset(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().link_dataset = Some(v.into());
        self
    }
    #[doc = "Set the field `organization_number`.\nOrganization resource ID that the source projects should belong to.\nProjects that do not belong to the provided organization are not considered when creating the dataset."]
    pub fn set_organization_number(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().organization_number = Some(v.into());
        self
    }
    #[doc = "Set the field `organization_scope`.\nDefines the options for providing a source organization for the DatasetConfig."]
    pub fn set_organization_scope(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().organization_scope = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `exclude_cloud_storage_buckets`.\n"]
    pub fn set_exclude_cloud_storage_buckets(
        self,
        v: impl Into<BlockAssignable<StorageInsightsDatasetConfigExcludeCloudStorageBucketsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().exclude_cloud_storage_buckets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .exclude_cloud_storage_buckets = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `exclude_cloud_storage_locations`.\n"]
    pub fn set_exclude_cloud_storage_locations(
        self,
        v: impl Into<BlockAssignable<StorageInsightsDatasetConfigExcludeCloudStorageLocationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().exclude_cloud_storage_locations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .exclude_cloud_storage_locations = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `identity`.\n"]
    pub fn set_identity(
        self,
        v: impl Into<BlockAssignable<StorageInsightsDatasetConfigIdentityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().identity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.identity = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `include_cloud_storage_buckets`.\n"]
    pub fn set_include_cloud_storage_buckets(
        self,
        v: impl Into<BlockAssignable<StorageInsightsDatasetConfigIncludeCloudStorageBucketsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().include_cloud_storage_buckets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .include_cloud_storage_buckets = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `include_cloud_storage_locations`.\n"]
    pub fn set_include_cloud_storage_locations(
        self,
        v: impl Into<BlockAssignable<StorageInsightsDatasetConfigIncludeCloudStorageLocationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().include_cloud_storage_locations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .include_cloud_storage_locations = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source_folders`.\n"]
    pub fn set_source_folders(
        self,
        v: impl Into<BlockAssignable<StorageInsightsDatasetConfigSourceFoldersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_folders = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_folders = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source_projects`.\n"]
    pub fn set_source_projects(
        self,
        v: impl Into<BlockAssignable<StorageInsightsDatasetConfigSourceProjectsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_projects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_projects = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<StorageInsightsDatasetConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `activity_data_retention_period_days` after provisioning.\nNumber of days of activity data that must be retained. If not specified, retentionPeriodDays will be used. Set to 0 to turn off the activity data."]
    pub fn activity_data_retention_period_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.activity_data_retention_period_days", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe UTC time at which the DatasetConfig was created. This is auto-populated."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataset_config_id` after provisioning.\nThe user-defined ID of the DatasetConfig"]
    pub fn dataset_config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataset_config_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataset_config_state` after provisioning.\nState of the DatasetConfig."]
    pub fn dataset_config_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataset_config_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional user-provided description for the dataset configuration with a maximum length of 256 characters."]
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
    #[doc = "Get a reference to the value of field `include_newly_created_buckets` after provisioning.\nIf set to true, the request includes all the newly created buckets in the dataset that meet the inclusion and exclusion rules."]
    pub fn include_newly_created_buckets(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_newly_created_buckets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `link` after provisioning.\nDetails of the linked DatasetConfig."]
    pub fn link(&self) -> ListRef<StorageInsightsDatasetConfigLinkElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `link_dataset` after provisioning.\nA boolean terraform only flag to link/unlink dataset.\n\nSetting this field to true while creation will automatically link the created dataset as an additional functionality.\n-> **Note** A dataset config resource can only be destroyed once it is unlinked,\nso users must set this field to false to unlink the dataset and destroy the dataset config resource."]
    pub fn link_dataset(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.link_dataset", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the DatasetConfig."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe full canonical resource name of the DatasetConfig (e.g., projects/P/locations/L/datasetConfigs/ID)."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization_number` after provisioning.\nOrganization resource ID that the source projects should belong to.\nProjects that do not belong to the provided organization are not considered when creating the dataset."]
    pub fn organization_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization_number", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization_scope` after provisioning.\nDefines the options for providing a source organization for the DatasetConfig."]
    pub fn organization_scope(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization_scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retention_period_days` after provisioning.\nNumber of days of history that must be retained."]
    pub fn retention_period_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention_period_days", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem generated unique identifier for the resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe UTC time at which the DatasetConfig was updated. This is auto-populated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `exclude_cloud_storage_buckets` after provisioning.\n"]
    pub fn exclude_cloud_storage_buckets(
        &self,
    ) -> ListRef<StorageInsightsDatasetConfigExcludeCloudStorageBucketsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_cloud_storage_buckets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `exclude_cloud_storage_locations` after provisioning.\n"]
    pub fn exclude_cloud_storage_locations(
        &self,
    ) -> ListRef<StorageInsightsDatasetConfigExcludeCloudStorageLocationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_cloud_storage_locations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `identity` after provisioning.\n"]
    pub fn identity(&self) -> ListRef<StorageInsightsDatasetConfigIdentityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.identity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `include_cloud_storage_buckets` after provisioning.\n"]
    pub fn include_cloud_storage_buckets(
        &self,
    ) -> ListRef<StorageInsightsDatasetConfigIncludeCloudStorageBucketsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_cloud_storage_buckets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `include_cloud_storage_locations` after provisioning.\n"]
    pub fn include_cloud_storage_locations(
        &self,
    ) -> ListRef<StorageInsightsDatasetConfigIncludeCloudStorageLocationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_cloud_storage_locations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_folders` after provisioning.\n"]
    pub fn source_folders(&self) -> ListRef<StorageInsightsDatasetConfigSourceFoldersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_folders", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_projects` after provisioning.\n"]
    pub fn source_projects(&self) -> ListRef<StorageInsightsDatasetConfigSourceProjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_projects", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageInsightsDatasetConfigTimeoutsElRef {
        StorageInsightsDatasetConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for StorageInsightsDatasetConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for StorageInsightsDatasetConfig {}
impl ToListMappable for StorageInsightsDatasetConfig {
    type O = ListRef<StorageInsightsDatasetConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for StorageInsightsDatasetConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_storage_insights_dataset_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildStorageInsightsDatasetConfig {
    pub tf_id: String,
    #[doc = "The user-defined ID of the DatasetConfig"]
    pub dataset_config_id: PrimField<String>,
    #[doc = "The location of the DatasetConfig."]
    pub location: PrimField<String>,
    #[doc = "Number of days of history that must be retained."]
    pub retention_period_days: PrimField<f64>,
}
impl BuildStorageInsightsDatasetConfig {
    pub fn build(self, stack: &mut Stack) -> StorageInsightsDatasetConfig {
        let out = StorageInsightsDatasetConfig(Rc::new(StorageInsightsDatasetConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(StorageInsightsDatasetConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                activity_data_retention_period_days: core::default::Default::default(),
                dataset_config_id: self.dataset_config_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                include_newly_created_buckets: core::default::Default::default(),
                link_dataset: core::default::Default::default(),
                location: self.location,
                organization_number: core::default::Default::default(),
                organization_scope: core::default::Default::default(),
                project: core::default::Default::default(),
                retention_period_days: self.retention_period_days,
                exclude_cloud_storage_buckets: core::default::Default::default(),
                exclude_cloud_storage_locations: core::default::Default::default(),
                identity: core::default::Default::default(),
                include_cloud_storage_buckets: core::default::Default::default(),
                include_cloud_storage_locations: core::default::Default::default(),
                source_folders: core::default::Default::default(),
                source_projects: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct StorageInsightsDatasetConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageInsightsDatasetConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl StorageInsightsDatasetConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `activity_data_retention_period_days` after provisioning.\nNumber of days of activity data that must be retained. If not specified, retentionPeriodDays will be used. Set to 0 to turn off the activity data."]
    pub fn activity_data_retention_period_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.activity_data_retention_period_days", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe UTC time at which the DatasetConfig was created. This is auto-populated."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataset_config_id` after provisioning.\nThe user-defined ID of the DatasetConfig"]
    pub fn dataset_config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataset_config_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataset_config_state` after provisioning.\nState of the DatasetConfig."]
    pub fn dataset_config_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataset_config_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional user-provided description for the dataset configuration with a maximum length of 256 characters."]
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
    #[doc = "Get a reference to the value of field `include_newly_created_buckets` after provisioning.\nIf set to true, the request includes all the newly created buckets in the dataset that meet the inclusion and exclusion rules."]
    pub fn include_newly_created_buckets(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_newly_created_buckets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `link` after provisioning.\nDetails of the linked DatasetConfig."]
    pub fn link(&self) -> ListRef<StorageInsightsDatasetConfigLinkElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `link_dataset` after provisioning.\nA boolean terraform only flag to link/unlink dataset.\n\nSetting this field to true while creation will automatically link the created dataset as an additional functionality.\n-> **Note** A dataset config resource can only be destroyed once it is unlinked,\nso users must set this field to false to unlink the dataset and destroy the dataset config resource."]
    pub fn link_dataset(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.link_dataset", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the DatasetConfig."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe full canonical resource name of the DatasetConfig (e.g., projects/P/locations/L/datasetConfigs/ID)."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization_number` after provisioning.\nOrganization resource ID that the source projects should belong to.\nProjects that do not belong to the provided organization are not considered when creating the dataset."]
    pub fn organization_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization_number", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization_scope` after provisioning.\nDefines the options for providing a source organization for the DatasetConfig."]
    pub fn organization_scope(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization_scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retention_period_days` after provisioning.\nNumber of days of history that must be retained."]
    pub fn retention_period_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention_period_days", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem generated unique identifier for the resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe UTC time at which the DatasetConfig was updated. This is auto-populated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `exclude_cloud_storage_buckets` after provisioning.\n"]
    pub fn exclude_cloud_storage_buckets(
        &self,
    ) -> ListRef<StorageInsightsDatasetConfigExcludeCloudStorageBucketsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_cloud_storage_buckets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `exclude_cloud_storage_locations` after provisioning.\n"]
    pub fn exclude_cloud_storage_locations(
        &self,
    ) -> ListRef<StorageInsightsDatasetConfigExcludeCloudStorageLocationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_cloud_storage_locations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `identity` after provisioning.\n"]
    pub fn identity(&self) -> ListRef<StorageInsightsDatasetConfigIdentityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.identity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `include_cloud_storage_buckets` after provisioning.\n"]
    pub fn include_cloud_storage_buckets(
        &self,
    ) -> ListRef<StorageInsightsDatasetConfigIncludeCloudStorageBucketsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_cloud_storage_buckets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `include_cloud_storage_locations` after provisioning.\n"]
    pub fn include_cloud_storage_locations(
        &self,
    ) -> ListRef<StorageInsightsDatasetConfigIncludeCloudStorageLocationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_cloud_storage_locations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_folders` after provisioning.\n"]
    pub fn source_folders(&self) -> ListRef<StorageInsightsDatasetConfigSourceFoldersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_folders", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_projects` after provisioning.\n"]
    pub fn source_projects(&self) -> ListRef<StorageInsightsDatasetConfigSourceProjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_projects", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageInsightsDatasetConfigTimeoutsElRef {
        StorageInsightsDatasetConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct StorageInsightsDatasetConfigLinkEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    linked: Option<PrimField<bool>>,
}
impl StorageInsightsDatasetConfigLinkEl {
    #[doc = "Set the field `dataset`.\n"]
    pub fn set_dataset(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dataset = Some(v.into());
        self
    }
    #[doc = "Set the field `linked`.\n"]
    pub fn set_linked(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.linked = Some(v.into());
        self
    }
}
impl ToListMappable for StorageInsightsDatasetConfigLinkEl {
    type O = BlockAssignable<StorageInsightsDatasetConfigLinkEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageInsightsDatasetConfigLinkEl {}
impl BuildStorageInsightsDatasetConfigLinkEl {
    pub fn build(self) -> StorageInsightsDatasetConfigLinkEl {
        StorageInsightsDatasetConfigLinkEl {
            dataset: core::default::Default::default(),
            linked: core::default::Default::default(),
        }
    }
}
pub struct StorageInsightsDatasetConfigLinkElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageInsightsDatasetConfigLinkElRef {
    fn new(shared: StackShared, base: String) -> StorageInsightsDatasetConfigLinkElRef {
        StorageInsightsDatasetConfigLinkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageInsightsDatasetConfigLinkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset` after provisioning.\n"]
    pub fn dataset(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset", self.base))
    }
    #[doc = "Get a reference to the value of field `linked` after provisioning.\n"]
    pub fn linked(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.linked", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_prefix_regex: Option<PrimField<String>>,
}
impl StorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsEl {
    #[doc = "Set the field `bucket_name`.\nThe list of cloud storage bucket names to exclude in the DatasetConfig.\nExactly one of the bucket_name and bucket_prefix_regex should be specified."]
    pub fn set_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_name = Some(v.into());
        self
    }
    #[doc = "Set the field `bucket_prefix_regex`.\nThe list of regex patterns for bucket names matching the regex.\nRegex should follow the syntax specified in google/re2 on GitHub.\nExactly one of the bucket_name and bucket_prefix_regex should be specified."]
    pub fn set_bucket_prefix_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_prefix_regex = Some(v.into());
        self
    }
}
impl ToListMappable
    for StorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsEl
{
    type O = BlockAssignable<
        StorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsEl {}
impl BuildStorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsEl {
    pub fn build(
        self,
    ) -> StorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsEl {
        StorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsEl {
            bucket_name: core::default::Default::default(),
            bucket_prefix_regex: core::default::Default::default(),
        }
    }
}
pub struct StorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsElRef {
        StorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nThe list of cloud storage bucket names to exclude in the DatasetConfig.\nExactly one of the bucket_name and bucket_prefix_regex should be specified."]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
    #[doc = "Get a reference to the value of field `bucket_prefix_regex` after provisioning.\nThe list of regex patterns for bucket names matching the regex.\nRegex should follow the syntax specified in google/re2 on GitHub.\nExactly one of the bucket_name and bucket_prefix_regex should be specified."]
    pub fn bucket_prefix_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_prefix_regex", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct StorageInsightsDatasetConfigExcludeCloudStorageBucketsElDynamic {
    cloud_storage_buckets: Option<
        DynamicBlock<StorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsEl>,
    >,
}
#[derive(Serialize)]
pub struct StorageInsightsDatasetConfigExcludeCloudStorageBucketsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_storage_buckets:
        Option<Vec<StorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsEl>>,
    dynamic: StorageInsightsDatasetConfigExcludeCloudStorageBucketsElDynamic,
}
impl StorageInsightsDatasetConfigExcludeCloudStorageBucketsEl {
    #[doc = "Set the field `cloud_storage_buckets`.\n"]
    pub fn set_cloud_storage_buckets(
        mut self,
        v: impl Into<
            BlockAssignable<
                StorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloud_storage_buckets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_storage_buckets = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for StorageInsightsDatasetConfigExcludeCloudStorageBucketsEl {
    type O = BlockAssignable<StorageInsightsDatasetConfigExcludeCloudStorageBucketsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageInsightsDatasetConfigExcludeCloudStorageBucketsEl {}
impl BuildStorageInsightsDatasetConfigExcludeCloudStorageBucketsEl {
    pub fn build(self) -> StorageInsightsDatasetConfigExcludeCloudStorageBucketsEl {
        StorageInsightsDatasetConfigExcludeCloudStorageBucketsEl {
            cloud_storage_buckets: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct StorageInsightsDatasetConfigExcludeCloudStorageBucketsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageInsightsDatasetConfigExcludeCloudStorageBucketsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageInsightsDatasetConfigExcludeCloudStorageBucketsElRef {
        StorageInsightsDatasetConfigExcludeCloudStorageBucketsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageInsightsDatasetConfigExcludeCloudStorageBucketsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_storage_buckets` after provisioning.\n"]
    pub fn cloud_storage_buckets(
        &self,
    ) -> ListRef<StorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_storage_buckets", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageInsightsDatasetConfigExcludeCloudStorageLocationsEl {
    locations: ListField<PrimField<String>>,
}
impl StorageInsightsDatasetConfigExcludeCloudStorageLocationsEl {}
impl ToListMappable for StorageInsightsDatasetConfigExcludeCloudStorageLocationsEl {
    type O = BlockAssignable<StorageInsightsDatasetConfigExcludeCloudStorageLocationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageInsightsDatasetConfigExcludeCloudStorageLocationsEl {
    #[doc = "The list of cloud storage locations to exclude in the DatasetConfig."]
    pub locations: ListField<PrimField<String>>,
}
impl BuildStorageInsightsDatasetConfigExcludeCloudStorageLocationsEl {
    pub fn build(self) -> StorageInsightsDatasetConfigExcludeCloudStorageLocationsEl {
        StorageInsightsDatasetConfigExcludeCloudStorageLocationsEl {
            locations: self.locations,
        }
    }
}
pub struct StorageInsightsDatasetConfigExcludeCloudStorageLocationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageInsightsDatasetConfigExcludeCloudStorageLocationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageInsightsDatasetConfigExcludeCloudStorageLocationsElRef {
        StorageInsightsDatasetConfigExcludeCloudStorageLocationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageInsightsDatasetConfigExcludeCloudStorageLocationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `locations` after provisioning.\nThe list of cloud storage locations to exclude in the DatasetConfig."]
    pub fn locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageInsightsDatasetConfigIdentityEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl StorageInsightsDatasetConfigIdentityEl {}
impl ToListMappable for StorageInsightsDatasetConfigIdentityEl {
    type O = BlockAssignable<StorageInsightsDatasetConfigIdentityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageInsightsDatasetConfigIdentityEl {
    #[doc = "Type of identity to use for the DatasetConfig. Possible values: [\"IDENTITY_TYPE_PER_CONFIG\", \"IDENTITY_TYPE_PER_PROJECT\"]"]
    pub type_: PrimField<String>,
}
impl BuildStorageInsightsDatasetConfigIdentityEl {
    pub fn build(self) -> StorageInsightsDatasetConfigIdentityEl {
        StorageInsightsDatasetConfigIdentityEl { type_: self.type_ }
    }
}
pub struct StorageInsightsDatasetConfigIdentityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageInsightsDatasetConfigIdentityElRef {
    fn new(shared: StackShared, base: String) -> StorageInsightsDatasetConfigIdentityElRef {
        StorageInsightsDatasetConfigIdentityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageInsightsDatasetConfigIdentityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the identity."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of identity to use for the DatasetConfig. Possible values: [\"IDENTITY_TYPE_PER_CONFIG\", \"IDENTITY_TYPE_PER_PROJECT\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_prefix_regex: Option<PrimField<String>>,
}
impl StorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsEl {
    #[doc = "Set the field `bucket_name`.\nThe list of cloud storage bucket names to include in the DatasetConfig.\nExactly one of the bucket_name and bucket_prefix_regex should be specified."]
    pub fn set_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_name = Some(v.into());
        self
    }
    #[doc = "Set the field `bucket_prefix_regex`.\nThe list of regex patterns for bucket names matching the regex.\nRegex should follow the syntax specified in google/re2 on GitHub.\nExactly one of the bucket_name and bucket_prefix_regex should be specified."]
    pub fn set_bucket_prefix_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_prefix_regex = Some(v.into());
        self
    }
}
impl ToListMappable
    for StorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsEl
{
    type O = BlockAssignable<
        StorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsEl {}
impl BuildStorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsEl {
    pub fn build(
        self,
    ) -> StorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsEl {
        StorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsEl {
            bucket_name: core::default::Default::default(),
            bucket_prefix_regex: core::default::Default::default(),
        }
    }
}
pub struct StorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsElRef {
        StorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nThe list of cloud storage bucket names to include in the DatasetConfig.\nExactly one of the bucket_name and bucket_prefix_regex should be specified."]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
    #[doc = "Get a reference to the value of field `bucket_prefix_regex` after provisioning.\nThe list of regex patterns for bucket names matching the regex.\nRegex should follow the syntax specified in google/re2 on GitHub.\nExactly one of the bucket_name and bucket_prefix_regex should be specified."]
    pub fn bucket_prefix_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_prefix_regex", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct StorageInsightsDatasetConfigIncludeCloudStorageBucketsElDynamic {
    cloud_storage_buckets: Option<
        DynamicBlock<StorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsEl>,
    >,
}
#[derive(Serialize)]
pub struct StorageInsightsDatasetConfigIncludeCloudStorageBucketsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_storage_buckets:
        Option<Vec<StorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsEl>>,
    dynamic: StorageInsightsDatasetConfigIncludeCloudStorageBucketsElDynamic,
}
impl StorageInsightsDatasetConfigIncludeCloudStorageBucketsEl {
    #[doc = "Set the field `cloud_storage_buckets`.\n"]
    pub fn set_cloud_storage_buckets(
        mut self,
        v: impl Into<
            BlockAssignable<
                StorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloud_storage_buckets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_storage_buckets = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for StorageInsightsDatasetConfigIncludeCloudStorageBucketsEl {
    type O = BlockAssignable<StorageInsightsDatasetConfigIncludeCloudStorageBucketsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageInsightsDatasetConfigIncludeCloudStorageBucketsEl {}
impl BuildStorageInsightsDatasetConfigIncludeCloudStorageBucketsEl {
    pub fn build(self) -> StorageInsightsDatasetConfigIncludeCloudStorageBucketsEl {
        StorageInsightsDatasetConfigIncludeCloudStorageBucketsEl {
            cloud_storage_buckets: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct StorageInsightsDatasetConfigIncludeCloudStorageBucketsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageInsightsDatasetConfigIncludeCloudStorageBucketsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageInsightsDatasetConfigIncludeCloudStorageBucketsElRef {
        StorageInsightsDatasetConfigIncludeCloudStorageBucketsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageInsightsDatasetConfigIncludeCloudStorageBucketsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_storage_buckets` after provisioning.\n"]
    pub fn cloud_storage_buckets(
        &self,
    ) -> ListRef<StorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_storage_buckets", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageInsightsDatasetConfigIncludeCloudStorageLocationsEl {
    locations: ListField<PrimField<String>>,
}
impl StorageInsightsDatasetConfigIncludeCloudStorageLocationsEl {}
impl ToListMappable for StorageInsightsDatasetConfigIncludeCloudStorageLocationsEl {
    type O = BlockAssignable<StorageInsightsDatasetConfigIncludeCloudStorageLocationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageInsightsDatasetConfigIncludeCloudStorageLocationsEl {
    #[doc = "The list of cloud storage locations to include in the DatasetConfig."]
    pub locations: ListField<PrimField<String>>,
}
impl BuildStorageInsightsDatasetConfigIncludeCloudStorageLocationsEl {
    pub fn build(self) -> StorageInsightsDatasetConfigIncludeCloudStorageLocationsEl {
        StorageInsightsDatasetConfigIncludeCloudStorageLocationsEl {
            locations: self.locations,
        }
    }
}
pub struct StorageInsightsDatasetConfigIncludeCloudStorageLocationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageInsightsDatasetConfigIncludeCloudStorageLocationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageInsightsDatasetConfigIncludeCloudStorageLocationsElRef {
        StorageInsightsDatasetConfigIncludeCloudStorageLocationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageInsightsDatasetConfigIncludeCloudStorageLocationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `locations` after provisioning.\nThe list of cloud storage locations to include in the DatasetConfig."]
    pub fn locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageInsightsDatasetConfigSourceFoldersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    folder_numbers: Option<ListField<PrimField<String>>>,
}
impl StorageInsightsDatasetConfigSourceFoldersEl {
    #[doc = "Set the field `folder_numbers`.\nThe list of folder numbers to include in the DatasetConfig."]
    pub fn set_folder_numbers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.folder_numbers = Some(v.into());
        self
    }
}
impl ToListMappable for StorageInsightsDatasetConfigSourceFoldersEl {
    type O = BlockAssignable<StorageInsightsDatasetConfigSourceFoldersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageInsightsDatasetConfigSourceFoldersEl {}
impl BuildStorageInsightsDatasetConfigSourceFoldersEl {
    pub fn build(self) -> StorageInsightsDatasetConfigSourceFoldersEl {
        StorageInsightsDatasetConfigSourceFoldersEl {
            folder_numbers: core::default::Default::default(),
        }
    }
}
pub struct StorageInsightsDatasetConfigSourceFoldersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageInsightsDatasetConfigSourceFoldersElRef {
    fn new(shared: StackShared, base: String) -> StorageInsightsDatasetConfigSourceFoldersElRef {
        StorageInsightsDatasetConfigSourceFoldersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageInsightsDatasetConfigSourceFoldersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `folder_numbers` after provisioning.\nThe list of folder numbers to include in the DatasetConfig."]
    pub fn folder_numbers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.folder_numbers", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageInsightsDatasetConfigSourceProjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    project_numbers: Option<ListField<PrimField<String>>>,
}
impl StorageInsightsDatasetConfigSourceProjectsEl {
    #[doc = "Set the field `project_numbers`.\nThe list of project numbers to include in the DatasetConfig."]
    pub fn set_project_numbers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.project_numbers = Some(v.into());
        self
    }
}
impl ToListMappable for StorageInsightsDatasetConfigSourceProjectsEl {
    type O = BlockAssignable<StorageInsightsDatasetConfigSourceProjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageInsightsDatasetConfigSourceProjectsEl {}
impl BuildStorageInsightsDatasetConfigSourceProjectsEl {
    pub fn build(self) -> StorageInsightsDatasetConfigSourceProjectsEl {
        StorageInsightsDatasetConfigSourceProjectsEl {
            project_numbers: core::default::Default::default(),
        }
    }
}
pub struct StorageInsightsDatasetConfigSourceProjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageInsightsDatasetConfigSourceProjectsElRef {
    fn new(shared: StackShared, base: String) -> StorageInsightsDatasetConfigSourceProjectsElRef {
        StorageInsightsDatasetConfigSourceProjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageInsightsDatasetConfigSourceProjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `project_numbers` after provisioning.\nThe list of project numbers to include in the DatasetConfig."]
    pub fn project_numbers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.project_numbers", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageInsightsDatasetConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl StorageInsightsDatasetConfigTimeoutsEl {
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
impl ToListMappable for StorageInsightsDatasetConfigTimeoutsEl {
    type O = BlockAssignable<StorageInsightsDatasetConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageInsightsDatasetConfigTimeoutsEl {}
impl BuildStorageInsightsDatasetConfigTimeoutsEl {
    pub fn build(self) -> StorageInsightsDatasetConfigTimeoutsEl {
        StorageInsightsDatasetConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct StorageInsightsDatasetConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageInsightsDatasetConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> StorageInsightsDatasetConfigTimeoutsElRef {
        StorageInsightsDatasetConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageInsightsDatasetConfigTimeoutsElRef {
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
struct StorageInsightsDatasetConfigDynamic {
    exclude_cloud_storage_buckets:
        Option<DynamicBlock<StorageInsightsDatasetConfigExcludeCloudStorageBucketsEl>>,
    exclude_cloud_storage_locations:
        Option<DynamicBlock<StorageInsightsDatasetConfigExcludeCloudStorageLocationsEl>>,
    identity: Option<DynamicBlock<StorageInsightsDatasetConfigIdentityEl>>,
    include_cloud_storage_buckets:
        Option<DynamicBlock<StorageInsightsDatasetConfigIncludeCloudStorageBucketsEl>>,
    include_cloud_storage_locations:
        Option<DynamicBlock<StorageInsightsDatasetConfigIncludeCloudStorageLocationsEl>>,
    source_folders: Option<DynamicBlock<StorageInsightsDatasetConfigSourceFoldersEl>>,
    source_projects: Option<DynamicBlock<StorageInsightsDatasetConfigSourceProjectsEl>>,
}
