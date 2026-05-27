use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataStorageControlFolderIntelligenceConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
}
struct DataStorageControlFolderIntelligenceConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataStorageControlFolderIntelligenceConfigData>,
}
#[derive(Clone)]
pub struct DataStorageControlFolderIntelligenceConfig(
    Rc<DataStorageControlFolderIntelligenceConfig_>,
);
impl DataStorageControlFolderIntelligenceConfig {
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
    #[doc = "Get a reference to the value of field `edition_config` after provisioning.\nEdition configuration of the Storage Intelligence resource. Valid values are INHERIT, TRIAL, DISABLED and STANDARD."]
    pub fn edition_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.edition_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_intelligence_config` after provisioning.\nThe Intelligence config that is effective for the resource."]
    pub fn effective_intelligence_config(
        &self,
    ) -> ListRef<DataStorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.effective_intelligence_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nFilter over location and bucket using include or exclude semantics. Resources that match the include or exclude filter are exclusively included or excluded from the Storage Intelligence plan."]
    pub fn filter(&self) -> ListRef<DataStorageControlFolderIntelligenceConfigFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier of the GCP Folder. For GCP Folder, this field can be folder number."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trial_config` after provisioning.\nThe trial configuration of the Storage Intelligence resource."]
    pub fn trial_config(
        &self,
    ) -> ListRef<DataStorageControlFolderIntelligenceConfigTrialConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trial_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time at which the Storage Intelligence Config resource is last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
}
impl Referable for DataStorageControlFolderIntelligenceConfig {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataStorageControlFolderIntelligenceConfig {}
impl ToListMappable for DataStorageControlFolderIntelligenceConfig {
    type O = ListRef<DataStorageControlFolderIntelligenceConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataStorageControlFolderIntelligenceConfig_ {
    fn extract_datasource_type(&self) -> String {
        "google_storage_control_folder_intelligence_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataStorageControlFolderIntelligenceConfig {
    pub tf_id: String,
    #[doc = "Identifier of the GCP Folder. For GCP Folder, this field can be folder number."]
    pub name: PrimField<String>,
}
impl BuildDataStorageControlFolderIntelligenceConfig {
    pub fn build(self, stack: &mut Stack) -> DataStorageControlFolderIntelligenceConfig {
        let out = DataStorageControlFolderIntelligenceConfig(Rc::new(
            DataStorageControlFolderIntelligenceConfig_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataStorageControlFolderIntelligenceConfigData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    id: core::default::Default::default(),
                    name: self.name,
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataStorageControlFolderIntelligenceConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageControlFolderIntelligenceConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataStorageControlFolderIntelligenceConfigRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `edition_config` after provisioning.\nEdition configuration of the Storage Intelligence resource. Valid values are INHERIT, TRIAL, DISABLED and STANDARD."]
    pub fn edition_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.edition_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_intelligence_config` after provisioning.\nThe Intelligence config that is effective for the resource."]
    pub fn effective_intelligence_config(
        &self,
    ) -> ListRef<DataStorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.effective_intelligence_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nFilter over location and bucket using include or exclude semantics. Resources that match the include or exclude filter are exclusively included or excluded from the Storage Intelligence plan."]
    pub fn filter(&self) -> ListRef<DataStorageControlFolderIntelligenceConfigFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier of the GCP Folder. For GCP Folder, this field can be folder number."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trial_config` after provisioning.\nThe trial configuration of the Storage Intelligence resource."]
    pub fn trial_config(
        &self,
    ) -> ListRef<DataStorageControlFolderIntelligenceConfigTrialConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trial_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time at which the Storage Intelligence Config resource is last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataStorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_edition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    intelligence_config: Option<PrimField<String>>,
}
impl DataStorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigEl {
    #[doc = "Set the field `effective_edition`.\n"]
    pub fn set_effective_edition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.effective_edition = Some(v.into());
        self
    }
    #[doc = "Set the field `intelligence_config`.\n"]
    pub fn set_intelligence_config(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.intelligence_config = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigEl {
    type O =
        BlockAssignable<DataStorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigEl {}
impl BuildDataStorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigEl {
    pub fn build(self) -> DataStorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigEl {
        DataStorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigEl {
            effective_edition: core::default::Default::default(),
            intelligence_config: core::default::Default::default(),
        }
    }
}
pub struct DataStorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigElRef {
        DataStorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `effective_edition` after provisioning.\n"]
    pub fn effective_edition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_edition", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `intelligence_config` after provisioning.\n"]
    pub fn intelligence_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.intelligence_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_id_regexes: Option<ListField<PrimField<String>>>,
}
impl DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
    #[doc = "Set the field `bucket_id_regexes`.\n"]
    pub fn set_bucket_id_regexes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.bucket_id_regexes = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsEl
{
    type O = BlockAssignable<
        DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {}
impl BuildDataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
    pub fn build(
        self,
    ) -> DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
        DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
            bucket_id_regexes: core::default::Default::default(),
        }
    }
}
pub struct DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef {
        DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_id_regexes` after provisioning.\n"]
    pub fn bucket_id_regexes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bucket_id_regexes", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    locations: Option<ListField<PrimField<String>>>,
}
impl DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
    #[doc = "Set the field `locations`.\n"]
    pub fn set_locations(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.locations = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsEl
{
    type O = BlockAssignable<
        DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
}
impl BuildDataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
    pub fn build(
        self,
    ) -> DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
        DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
            locations: core::default::Default::default(),
        }
    }
}
pub struct DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef {
        DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `locations` after provisioning.\n"]
    pub fn locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }
}
#[derive(Serialize)]
pub struct DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_id_regexes: Option<ListField<PrimField<String>>>,
}
impl DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
    #[doc = "Set the field `bucket_id_regexes`.\n"]
    pub fn set_bucket_id_regexes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.bucket_id_regexes = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsEl
{
    type O = BlockAssignable<
        DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {}
impl BuildDataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
    pub fn build(
        self,
    ) -> DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
        DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
            bucket_id_regexes: core::default::Default::default(),
        }
    }
}
pub struct DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef {
        DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_id_regexes` after provisioning.\n"]
    pub fn bucket_id_regexes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bucket_id_regexes", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    locations: Option<ListField<PrimField<String>>>,
}
impl DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
    #[doc = "Set the field `locations`.\n"]
    pub fn set_locations(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.locations = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsEl
{
    type O = BlockAssignable<
        DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
}
impl BuildDataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
    pub fn build(
        self,
    ) -> DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
        DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
            locations: core::default::Default::default(),
        }
    }
}
pub struct DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef {
        DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `locations` after provisioning.\n"]
    pub fn locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }
}
#[derive(Serialize)]
pub struct DataStorageControlFolderIntelligenceConfigFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_cloud_storage_buckets: Option<
        ListField<DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_cloud_storage_locations: Option<
        ListField<
            DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    included_cloud_storage_buckets: Option<
        ListField<DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    included_cloud_storage_locations: Option<
        ListField<
            DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsEl,
        >,
    >,
}
impl DataStorageControlFolderIntelligenceConfigFilterEl {
    #[doc = "Set the field `excluded_cloud_storage_buckets`.\n"]
    pub fn set_excluded_cloud_storage_buckets(
        mut self,
        v: impl Into<
            ListField<
                DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsEl,
            >,
        >,
    ) -> Self {
        self.excluded_cloud_storage_buckets = Some(v.into());
        self
    }
    #[doc = "Set the field `excluded_cloud_storage_locations`.\n"]
    pub fn set_excluded_cloud_storage_locations(
        mut self,
        v: impl Into<
            ListField<
                DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsEl,
            >,
        >,
    ) -> Self {
        self.excluded_cloud_storage_locations = Some(v.into());
        self
    }
    #[doc = "Set the field `included_cloud_storage_buckets`.\n"]
    pub fn set_included_cloud_storage_buckets(
        mut self,
        v: impl Into<
            ListField<
                DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsEl,
            >,
        >,
    ) -> Self {
        self.included_cloud_storage_buckets = Some(v.into());
        self
    }
    #[doc = "Set the field `included_cloud_storage_locations`.\n"]
    pub fn set_included_cloud_storage_locations(
        mut self,
        v: impl Into<
            ListField<
                DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsEl,
            >,
        >,
    ) -> Self {
        self.included_cloud_storage_locations = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageControlFolderIntelligenceConfigFilterEl {
    type O = BlockAssignable<DataStorageControlFolderIntelligenceConfigFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageControlFolderIntelligenceConfigFilterEl {}
impl BuildDataStorageControlFolderIntelligenceConfigFilterEl {
    pub fn build(self) -> DataStorageControlFolderIntelligenceConfigFilterEl {
        DataStorageControlFolderIntelligenceConfigFilterEl {
            excluded_cloud_storage_buckets: core::default::Default::default(),
            excluded_cloud_storage_locations: core::default::Default::default(),
            included_cloud_storage_buckets: core::default::Default::default(),
            included_cloud_storage_locations: core::default::Default::default(),
        }
    }
}
pub struct DataStorageControlFolderIntelligenceConfigFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageControlFolderIntelligenceConfigFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageControlFolderIntelligenceConfigFilterElRef {
        DataStorageControlFolderIntelligenceConfigFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageControlFolderIntelligenceConfigFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `excluded_cloud_storage_buckets` after provisioning.\n"]
    pub fn excluded_cloud_storage_buckets(
        &self,
    ) -> ListRef<DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.excluded_cloud_storage_buckets", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `excluded_cloud_storage_locations` after provisioning.\n"]
    pub fn excluded_cloud_storage_locations(
        &self,
    ) -> ListRef<DataStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.excluded_cloud_storage_locations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `included_cloud_storage_buckets` after provisioning.\n"]
    pub fn included_cloud_storage_buckets(
        &self,
    ) -> ListRef<DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.included_cloud_storage_buckets", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `included_cloud_storage_locations` after provisioning.\n"]
    pub fn included_cloud_storage_locations(
        &self,
    ) -> ListRef<DataStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.included_cloud_storage_locations", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataStorageControlFolderIntelligenceConfigTrialConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_time: Option<PrimField<String>>,
}
impl DataStorageControlFolderIntelligenceConfigTrialConfigEl {
    #[doc = "Set the field `expire_time`.\n"]
    pub fn set_expire_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expire_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageControlFolderIntelligenceConfigTrialConfigEl {
    type O = BlockAssignable<DataStorageControlFolderIntelligenceConfigTrialConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageControlFolderIntelligenceConfigTrialConfigEl {}
impl BuildDataStorageControlFolderIntelligenceConfigTrialConfigEl {
    pub fn build(self) -> DataStorageControlFolderIntelligenceConfigTrialConfigEl {
        DataStorageControlFolderIntelligenceConfigTrialConfigEl {
            expire_time: core::default::Default::default(),
        }
    }
}
pub struct DataStorageControlFolderIntelligenceConfigTrialConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageControlFolderIntelligenceConfigTrialConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageControlFolderIntelligenceConfigTrialConfigElRef {
        DataStorageControlFolderIntelligenceConfigTrialConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageControlFolderIntelligenceConfigTrialConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\n"]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.base))
    }
}
