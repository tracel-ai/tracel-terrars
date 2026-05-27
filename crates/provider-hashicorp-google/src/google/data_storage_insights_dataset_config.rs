use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataStorageInsightsDatasetConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    dataset_config_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataStorageInsightsDatasetConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataStorageInsightsDatasetConfigData>,
}
#[derive(Clone)]
pub struct DataStorageInsightsDatasetConfig(Rc<DataStorageInsightsDatasetConfig_>);
impl DataStorageInsightsDatasetConfig {
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
    #[doc = "Get a reference to the value of field `exclude_cloud_storage_buckets` after provisioning.\nDefined the options for excluding cloud storage buckets for the DatasetConfig."]
    pub fn exclude_cloud_storage_buckets(
        &self,
    ) -> ListRef<DataStorageInsightsDatasetConfigExcludeCloudStorageBucketsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_cloud_storage_buckets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `exclude_cloud_storage_locations` after provisioning.\nDefines the options for excluding cloud storage locations for the DatasetConfig."]
    pub fn exclude_cloud_storage_locations(
        &self,
    ) -> ListRef<DataStorageInsightsDatasetConfigExcludeCloudStorageLocationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_cloud_storage_locations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `identity` after provisioning.\nIdentity used by DatasetConfig."]
    pub fn identity(&self) -> ListRef<DataStorageInsightsDatasetConfigIdentityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.identity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `include_cloud_storage_buckets` after provisioning.\nDefines the options for including cloud storage buckets for the DatasetConfig."]
    pub fn include_cloud_storage_buckets(
        &self,
    ) -> ListRef<DataStorageInsightsDatasetConfigIncludeCloudStorageBucketsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_cloud_storage_buckets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `include_cloud_storage_locations` after provisioning.\nDefines the options for including cloud storage locations for the DatasetConfig."]
    pub fn include_cloud_storage_locations(
        &self,
    ) -> ListRef<DataStorageInsightsDatasetConfigIncludeCloudStorageLocationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_cloud_storage_locations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `include_newly_created_buckets` after provisioning.\nIf set to true, the request includes all the newly created buckets in the dataset that meet the inclusion and exclusion rules."]
    pub fn include_newly_created_buckets(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_newly_created_buckets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `link` after provisioning.\nDetails of the linked DatasetConfig."]
    pub fn link(&self) -> ListRef<DataStorageInsightsDatasetConfigLinkElRef> {
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
    #[doc = "Get a reference to the value of field `source_folders` after provisioning.\nDefines the options for providing source folders for the DatasetConfig."]
    pub fn source_folders(&self) -> ListRef<DataStorageInsightsDatasetConfigSourceFoldersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_folders", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_projects` after provisioning.\nDefines the options for providing source projects for the DatasetConfig."]
    pub fn source_projects(&self) -> ListRef<DataStorageInsightsDatasetConfigSourceProjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_projects", self.extract_ref()),
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
}
impl Referable for DataStorageInsightsDatasetConfig {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataStorageInsightsDatasetConfig {}
impl ToListMappable for DataStorageInsightsDatasetConfig {
    type O = ListRef<DataStorageInsightsDatasetConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataStorageInsightsDatasetConfig_ {
    fn extract_datasource_type(&self) -> String {
        "google_storage_insights_dataset_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataStorageInsightsDatasetConfig {
    pub tf_id: String,
    #[doc = "The user-defined ID of the DatasetConfig"]
    pub dataset_config_id: PrimField<String>,
    #[doc = "The location of the DatasetConfig."]
    pub location: PrimField<String>,
}
impl BuildDataStorageInsightsDatasetConfig {
    pub fn build(self, stack: &mut Stack) -> DataStorageInsightsDatasetConfig {
        let out = DataStorageInsightsDatasetConfig(Rc::new(DataStorageInsightsDatasetConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataStorageInsightsDatasetConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                dataset_config_id: self.dataset_config_id,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataStorageInsightsDatasetConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageInsightsDatasetConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataStorageInsightsDatasetConfigRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
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
    #[doc = "Get a reference to the value of field `exclude_cloud_storage_buckets` after provisioning.\nDefined the options for excluding cloud storage buckets for the DatasetConfig."]
    pub fn exclude_cloud_storage_buckets(
        &self,
    ) -> ListRef<DataStorageInsightsDatasetConfigExcludeCloudStorageBucketsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_cloud_storage_buckets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `exclude_cloud_storage_locations` after provisioning.\nDefines the options for excluding cloud storage locations for the DatasetConfig."]
    pub fn exclude_cloud_storage_locations(
        &self,
    ) -> ListRef<DataStorageInsightsDatasetConfigExcludeCloudStorageLocationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_cloud_storage_locations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `identity` after provisioning.\nIdentity used by DatasetConfig."]
    pub fn identity(&self) -> ListRef<DataStorageInsightsDatasetConfigIdentityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.identity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `include_cloud_storage_buckets` after provisioning.\nDefines the options for including cloud storage buckets for the DatasetConfig."]
    pub fn include_cloud_storage_buckets(
        &self,
    ) -> ListRef<DataStorageInsightsDatasetConfigIncludeCloudStorageBucketsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_cloud_storage_buckets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `include_cloud_storage_locations` after provisioning.\nDefines the options for including cloud storage locations for the DatasetConfig."]
    pub fn include_cloud_storage_locations(
        &self,
    ) -> ListRef<DataStorageInsightsDatasetConfigIncludeCloudStorageLocationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_cloud_storage_locations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `include_newly_created_buckets` after provisioning.\nIf set to true, the request includes all the newly created buckets in the dataset that meet the inclusion and exclusion rules."]
    pub fn include_newly_created_buckets(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_newly_created_buckets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `link` after provisioning.\nDetails of the linked DatasetConfig."]
    pub fn link(&self) -> ListRef<DataStorageInsightsDatasetConfigLinkElRef> {
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
    #[doc = "Get a reference to the value of field `source_folders` after provisioning.\nDefines the options for providing source folders for the DatasetConfig."]
    pub fn source_folders(&self) -> ListRef<DataStorageInsightsDatasetConfigSourceFoldersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_folders", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_projects` after provisioning.\nDefines the options for providing source projects for the DatasetConfig."]
    pub fn source_projects(&self) -> ListRef<DataStorageInsightsDatasetConfigSourceProjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_projects", self.extract_ref()),
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
}
#[derive(Serialize)]
pub struct DataStorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_prefix_regex: Option<PrimField<String>>,
}
impl DataStorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_name = Some(v.into());
        self
    }
    #[doc = "Set the field `bucket_prefix_regex`.\n"]
    pub fn set_bucket_prefix_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_prefix_regex = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataStorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsEl
{
    type O = BlockAssignable<
        DataStorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsEl {
}
impl BuildDataStorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsEl {
    pub fn build(
        self,
    ) -> DataStorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsEl {
        DataStorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsEl {
            bucket_name: core::default::Default::default(),
            bucket_prefix_regex: core::default::Default::default(),
        }
    }
}
pub struct DataStorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsElRef {
        DataStorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
    #[doc = "Get a reference to the value of field `bucket_prefix_regex` after provisioning.\n"]
    pub fn bucket_prefix_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_prefix_regex", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataStorageInsightsDatasetConfigExcludeCloudStorageBucketsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_storage_buckets: Option<
        ListField<
            DataStorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsEl,
        >,
    >,
}
impl DataStorageInsightsDatasetConfigExcludeCloudStorageBucketsEl {
    #[doc = "Set the field `cloud_storage_buckets`.\n"]
    pub fn set_cloud_storage_buckets(
        mut self,
        v: impl Into<
            ListField<
                DataStorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsEl,
            >,
        >,
    ) -> Self {
        self.cloud_storage_buckets = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageInsightsDatasetConfigExcludeCloudStorageBucketsEl {
    type O = BlockAssignable<DataStorageInsightsDatasetConfigExcludeCloudStorageBucketsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageInsightsDatasetConfigExcludeCloudStorageBucketsEl {}
impl BuildDataStorageInsightsDatasetConfigExcludeCloudStorageBucketsEl {
    pub fn build(self) -> DataStorageInsightsDatasetConfigExcludeCloudStorageBucketsEl {
        DataStorageInsightsDatasetConfigExcludeCloudStorageBucketsEl {
            cloud_storage_buckets: core::default::Default::default(),
        }
    }
}
pub struct DataStorageInsightsDatasetConfigExcludeCloudStorageBucketsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageInsightsDatasetConfigExcludeCloudStorageBucketsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageInsightsDatasetConfigExcludeCloudStorageBucketsElRef {
        DataStorageInsightsDatasetConfigExcludeCloudStorageBucketsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageInsightsDatasetConfigExcludeCloudStorageBucketsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_storage_buckets` after provisioning.\n"]
    pub fn cloud_storage_buckets(
        &self,
    ) -> ListRef<DataStorageInsightsDatasetConfigExcludeCloudStorageBucketsElCloudStorageBucketsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_storage_buckets", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataStorageInsightsDatasetConfigExcludeCloudStorageLocationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    locations: Option<ListField<PrimField<String>>>,
}
impl DataStorageInsightsDatasetConfigExcludeCloudStorageLocationsEl {
    #[doc = "Set the field `locations`.\n"]
    pub fn set_locations(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.locations = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageInsightsDatasetConfigExcludeCloudStorageLocationsEl {
    type O = BlockAssignable<DataStorageInsightsDatasetConfigExcludeCloudStorageLocationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageInsightsDatasetConfigExcludeCloudStorageLocationsEl {}
impl BuildDataStorageInsightsDatasetConfigExcludeCloudStorageLocationsEl {
    pub fn build(self) -> DataStorageInsightsDatasetConfigExcludeCloudStorageLocationsEl {
        DataStorageInsightsDatasetConfigExcludeCloudStorageLocationsEl {
            locations: core::default::Default::default(),
        }
    }
}
pub struct DataStorageInsightsDatasetConfigExcludeCloudStorageLocationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageInsightsDatasetConfigExcludeCloudStorageLocationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageInsightsDatasetConfigExcludeCloudStorageLocationsElRef {
        DataStorageInsightsDatasetConfigExcludeCloudStorageLocationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageInsightsDatasetConfigExcludeCloudStorageLocationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `locations` after provisioning.\n"]
    pub fn locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }
}
#[derive(Serialize)]
pub struct DataStorageInsightsDatasetConfigIdentityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataStorageInsightsDatasetConfigIdentityEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageInsightsDatasetConfigIdentityEl {
    type O = BlockAssignable<DataStorageInsightsDatasetConfigIdentityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageInsightsDatasetConfigIdentityEl {}
impl BuildDataStorageInsightsDatasetConfigIdentityEl {
    pub fn build(self) -> DataStorageInsightsDatasetConfigIdentityEl {
        DataStorageInsightsDatasetConfigIdentityEl {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataStorageInsightsDatasetConfigIdentityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageInsightsDatasetConfigIdentityElRef {
    fn new(shared: StackShared, base: String) -> DataStorageInsightsDatasetConfigIdentityElRef {
        DataStorageInsightsDatasetConfigIdentityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageInsightsDatasetConfigIdentityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataStorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_prefix_regex: Option<PrimField<String>>,
}
impl DataStorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_name = Some(v.into());
        self
    }
    #[doc = "Set the field `bucket_prefix_regex`.\n"]
    pub fn set_bucket_prefix_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_prefix_regex = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataStorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsEl
{
    type O = BlockAssignable<
        DataStorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsEl {
}
impl BuildDataStorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsEl {
    pub fn build(
        self,
    ) -> DataStorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsEl {
        DataStorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsEl {
            bucket_name: core::default::Default::default(),
            bucket_prefix_regex: core::default::Default::default(),
        }
    }
}
pub struct DataStorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsElRef {
        DataStorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
    #[doc = "Get a reference to the value of field `bucket_prefix_regex` after provisioning.\n"]
    pub fn bucket_prefix_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_prefix_regex", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataStorageInsightsDatasetConfigIncludeCloudStorageBucketsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_storage_buckets: Option<
        ListField<
            DataStorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsEl,
        >,
    >,
}
impl DataStorageInsightsDatasetConfigIncludeCloudStorageBucketsEl {
    #[doc = "Set the field `cloud_storage_buckets`.\n"]
    pub fn set_cloud_storage_buckets(
        mut self,
        v: impl Into<
            ListField<
                DataStorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsEl,
            >,
        >,
    ) -> Self {
        self.cloud_storage_buckets = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageInsightsDatasetConfigIncludeCloudStorageBucketsEl {
    type O = BlockAssignable<DataStorageInsightsDatasetConfigIncludeCloudStorageBucketsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageInsightsDatasetConfigIncludeCloudStorageBucketsEl {}
impl BuildDataStorageInsightsDatasetConfigIncludeCloudStorageBucketsEl {
    pub fn build(self) -> DataStorageInsightsDatasetConfigIncludeCloudStorageBucketsEl {
        DataStorageInsightsDatasetConfigIncludeCloudStorageBucketsEl {
            cloud_storage_buckets: core::default::Default::default(),
        }
    }
}
pub struct DataStorageInsightsDatasetConfigIncludeCloudStorageBucketsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageInsightsDatasetConfigIncludeCloudStorageBucketsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageInsightsDatasetConfigIncludeCloudStorageBucketsElRef {
        DataStorageInsightsDatasetConfigIncludeCloudStorageBucketsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageInsightsDatasetConfigIncludeCloudStorageBucketsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_storage_buckets` after provisioning.\n"]
    pub fn cloud_storage_buckets(
        &self,
    ) -> ListRef<DataStorageInsightsDatasetConfigIncludeCloudStorageBucketsElCloudStorageBucketsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_storage_buckets", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataStorageInsightsDatasetConfigIncludeCloudStorageLocationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    locations: Option<ListField<PrimField<String>>>,
}
impl DataStorageInsightsDatasetConfigIncludeCloudStorageLocationsEl {
    #[doc = "Set the field `locations`.\n"]
    pub fn set_locations(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.locations = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageInsightsDatasetConfigIncludeCloudStorageLocationsEl {
    type O = BlockAssignable<DataStorageInsightsDatasetConfigIncludeCloudStorageLocationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageInsightsDatasetConfigIncludeCloudStorageLocationsEl {}
impl BuildDataStorageInsightsDatasetConfigIncludeCloudStorageLocationsEl {
    pub fn build(self) -> DataStorageInsightsDatasetConfigIncludeCloudStorageLocationsEl {
        DataStorageInsightsDatasetConfigIncludeCloudStorageLocationsEl {
            locations: core::default::Default::default(),
        }
    }
}
pub struct DataStorageInsightsDatasetConfigIncludeCloudStorageLocationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageInsightsDatasetConfigIncludeCloudStorageLocationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageInsightsDatasetConfigIncludeCloudStorageLocationsElRef {
        DataStorageInsightsDatasetConfigIncludeCloudStorageLocationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageInsightsDatasetConfigIncludeCloudStorageLocationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `locations` after provisioning.\n"]
    pub fn locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }
}
#[derive(Serialize)]
pub struct DataStorageInsightsDatasetConfigLinkEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    linked: Option<PrimField<bool>>,
}
impl DataStorageInsightsDatasetConfigLinkEl {
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
impl ToListMappable for DataStorageInsightsDatasetConfigLinkEl {
    type O = BlockAssignable<DataStorageInsightsDatasetConfigLinkEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageInsightsDatasetConfigLinkEl {}
impl BuildDataStorageInsightsDatasetConfigLinkEl {
    pub fn build(self) -> DataStorageInsightsDatasetConfigLinkEl {
        DataStorageInsightsDatasetConfigLinkEl {
            dataset: core::default::Default::default(),
            linked: core::default::Default::default(),
        }
    }
}
pub struct DataStorageInsightsDatasetConfigLinkElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageInsightsDatasetConfigLinkElRef {
    fn new(shared: StackShared, base: String) -> DataStorageInsightsDatasetConfigLinkElRef {
        DataStorageInsightsDatasetConfigLinkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageInsightsDatasetConfigLinkElRef {
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
pub struct DataStorageInsightsDatasetConfigSourceFoldersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    folder_numbers: Option<ListField<PrimField<String>>>,
}
impl DataStorageInsightsDatasetConfigSourceFoldersEl {
    #[doc = "Set the field `folder_numbers`.\n"]
    pub fn set_folder_numbers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.folder_numbers = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageInsightsDatasetConfigSourceFoldersEl {
    type O = BlockAssignable<DataStorageInsightsDatasetConfigSourceFoldersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageInsightsDatasetConfigSourceFoldersEl {}
impl BuildDataStorageInsightsDatasetConfigSourceFoldersEl {
    pub fn build(self) -> DataStorageInsightsDatasetConfigSourceFoldersEl {
        DataStorageInsightsDatasetConfigSourceFoldersEl {
            folder_numbers: core::default::Default::default(),
        }
    }
}
pub struct DataStorageInsightsDatasetConfigSourceFoldersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageInsightsDatasetConfigSourceFoldersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageInsightsDatasetConfigSourceFoldersElRef {
        DataStorageInsightsDatasetConfigSourceFoldersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageInsightsDatasetConfigSourceFoldersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `folder_numbers` after provisioning.\n"]
    pub fn folder_numbers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.folder_numbers", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataStorageInsightsDatasetConfigSourceProjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    project_numbers: Option<ListField<PrimField<String>>>,
}
impl DataStorageInsightsDatasetConfigSourceProjectsEl {
    #[doc = "Set the field `project_numbers`.\n"]
    pub fn set_project_numbers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.project_numbers = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageInsightsDatasetConfigSourceProjectsEl {
    type O = BlockAssignable<DataStorageInsightsDatasetConfigSourceProjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageInsightsDatasetConfigSourceProjectsEl {}
impl BuildDataStorageInsightsDatasetConfigSourceProjectsEl {
    pub fn build(self) -> DataStorageInsightsDatasetConfigSourceProjectsEl {
        DataStorageInsightsDatasetConfigSourceProjectsEl {
            project_numbers: core::default::Default::default(),
        }
    }
}
pub struct DataStorageInsightsDatasetConfigSourceProjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageInsightsDatasetConfigSourceProjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageInsightsDatasetConfigSourceProjectsElRef {
        DataStorageInsightsDatasetConfigSourceProjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageInsightsDatasetConfigSourceProjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `project_numbers` after provisioning.\n"]
    pub fn project_numbers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.project_numbers", self.base),
        )
    }
}
