use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataStorageControlProjectIntelligenceConfigData {
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
struct DataStorageControlProjectIntelligenceConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataStorageControlProjectIntelligenceConfigData>,
}
#[derive(Clone)]
pub struct DataStorageControlProjectIntelligenceConfig(
    Rc<DataStorageControlProjectIntelligenceConfig_>,
);
impl DataStorageControlProjectIntelligenceConfig {
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
    ) -> ListRef<DataStorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.effective_intelligence_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nFilter over location and bucket using include or exclude semantics. Resources that match the include or exclude filter are exclusively included or excluded from the Storage Intelligence plan."]
    pub fn filter(&self) -> ListRef<DataStorageControlProjectIntelligenceConfigFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier of the GCP project. For GCP project, this field can be project name or project number."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trial_config` after provisioning.\nThe trial configuration of the Storage Intelligence resource."]
    pub fn trial_config(
        &self,
    ) -> ListRef<DataStorageControlProjectIntelligenceConfigTrialConfigElRef> {
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
impl Referable for DataStorageControlProjectIntelligenceConfig {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataStorageControlProjectIntelligenceConfig {}
impl ToListMappable for DataStorageControlProjectIntelligenceConfig {
    type O = ListRef<DataStorageControlProjectIntelligenceConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataStorageControlProjectIntelligenceConfig_ {
    fn extract_datasource_type(&self) -> String {
        "google_storage_control_project_intelligence_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataStorageControlProjectIntelligenceConfig {
    pub tf_id: String,
    #[doc = "Identifier of the GCP project. For GCP project, this field can be project name or project number."]
    pub name: PrimField<String>,
}
impl BuildDataStorageControlProjectIntelligenceConfig {
    pub fn build(self, stack: &mut Stack) -> DataStorageControlProjectIntelligenceConfig {
        let out = DataStorageControlProjectIntelligenceConfig(Rc::new(
            DataStorageControlProjectIntelligenceConfig_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataStorageControlProjectIntelligenceConfigData {
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
pub struct DataStorageControlProjectIntelligenceConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageControlProjectIntelligenceConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataStorageControlProjectIntelligenceConfigRef {
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
    ) -> ListRef<DataStorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.effective_intelligence_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nFilter over location and bucket using include or exclude semantics. Resources that match the include or exclude filter are exclusively included or excluded from the Storage Intelligence plan."]
    pub fn filter(&self) -> ListRef<DataStorageControlProjectIntelligenceConfigFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier of the GCP project. For GCP project, this field can be project name or project number."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trial_config` after provisioning.\nThe trial configuration of the Storage Intelligence resource."]
    pub fn trial_config(
        &self,
    ) -> ListRef<DataStorageControlProjectIntelligenceConfigTrialConfigElRef> {
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
pub struct DataStorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_edition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    intelligence_config: Option<PrimField<String>>,
}
impl DataStorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigEl {
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
impl ToListMappable for DataStorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigEl {
    type O =
        BlockAssignable<DataStorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigEl {}
impl BuildDataStorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigEl {
    pub fn build(self) -> DataStorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigEl {
        DataStorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigEl {
            effective_edition: core::default::Default::default(),
            intelligence_config: core::default::Default::default(),
        }
    }
}
pub struct DataStorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigElRef {
        DataStorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigElRef {
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
pub struct DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_id_regexes: Option<ListField<PrimField<String>>>,
}
impl DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
    #[doc = "Set the field `bucket_id_regexes`.\n"]
    pub fn set_bucket_id_regexes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.bucket_id_regexes = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsEl
{
    type O = BlockAssignable<
        DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
}
impl BuildDataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
    pub fn build(
        self,
    ) -> DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
        DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
            bucket_id_regexes: core::default::Default::default(),
        }
    }
}
pub struct DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef {
        DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef {
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
pub struct DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    locations: Option<ListField<PrimField<String>>>,
}
impl DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
    #[doc = "Set the field `locations`.\n"]
    pub fn set_locations(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.locations = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsEl
{
    type O = BlockAssignable<
        DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsEl
{}
impl BuildDataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
    pub fn build(
        self,
    ) -> DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
        DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
            locations: core::default::Default::default(),
        }
    }
}
pub struct DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef {
        DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `locations` after provisioning.\n"]
    pub fn locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }
}
#[derive(Serialize)]
pub struct DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_id_regexes: Option<ListField<PrimField<String>>>,
}
impl DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
    #[doc = "Set the field `bucket_id_regexes`.\n"]
    pub fn set_bucket_id_regexes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.bucket_id_regexes = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsEl
{
    type O = BlockAssignable<
        DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
}
impl BuildDataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
    pub fn build(
        self,
    ) -> DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
        DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
            bucket_id_regexes: core::default::Default::default(),
        }
    }
}
pub struct DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef {
        DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef {
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
pub struct DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    locations: Option<ListField<PrimField<String>>>,
}
impl DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
    #[doc = "Set the field `locations`.\n"]
    pub fn set_locations(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.locations = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsEl
{
    type O = BlockAssignable<
        DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsEl
{}
impl BuildDataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
    pub fn build(
        self,
    ) -> DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
        DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
            locations: core::default::Default::default(),
        }
    }
}
pub struct DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef {
        DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `locations` after provisioning.\n"]
    pub fn locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }
}
#[derive(Serialize)]
pub struct DataStorageControlProjectIntelligenceConfigFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_cloud_storage_buckets: Option<
        ListField<DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_cloud_storage_locations: Option<
        ListField<
            DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    included_cloud_storage_buckets: Option<
        ListField<DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    included_cloud_storage_locations: Option<
        ListField<
            DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsEl,
        >,
    >,
}
impl DataStorageControlProjectIntelligenceConfigFilterEl {
    #[doc = "Set the field `excluded_cloud_storage_buckets`.\n"]
    pub fn set_excluded_cloud_storage_buckets(
        mut self,
        v: impl Into<
            ListField<
                DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsEl,
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
                DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsEl,
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
                DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsEl,
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
                DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsEl,
            >,
        >,
    ) -> Self {
        self.included_cloud_storage_locations = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageControlProjectIntelligenceConfigFilterEl {
    type O = BlockAssignable<DataStorageControlProjectIntelligenceConfigFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageControlProjectIntelligenceConfigFilterEl {}
impl BuildDataStorageControlProjectIntelligenceConfigFilterEl {
    pub fn build(self) -> DataStorageControlProjectIntelligenceConfigFilterEl {
        DataStorageControlProjectIntelligenceConfigFilterEl {
            excluded_cloud_storage_buckets: core::default::Default::default(),
            excluded_cloud_storage_locations: core::default::Default::default(),
            included_cloud_storage_buckets: core::default::Default::default(),
            included_cloud_storage_locations: core::default::Default::default(),
        }
    }
}
pub struct DataStorageControlProjectIntelligenceConfigFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageControlProjectIntelligenceConfigFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageControlProjectIntelligenceConfigFilterElRef {
        DataStorageControlProjectIntelligenceConfigFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageControlProjectIntelligenceConfigFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `excluded_cloud_storage_buckets` after provisioning.\n"]
    pub fn excluded_cloud_storage_buckets(
        &self,
    ) -> ListRef<DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.excluded_cloud_storage_buckets", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `excluded_cloud_storage_locations` after provisioning.\n"]
    pub fn excluded_cloud_storage_locations(
        &self,
    ) -> ListRef<
        DataStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.excluded_cloud_storage_locations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `included_cloud_storage_buckets` after provisioning.\n"]
    pub fn included_cloud_storage_buckets(
        &self,
    ) -> ListRef<DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.included_cloud_storage_buckets", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `included_cloud_storage_locations` after provisioning.\n"]
    pub fn included_cloud_storage_locations(
        &self,
    ) -> ListRef<
        DataStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.included_cloud_storage_locations", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataStorageControlProjectIntelligenceConfigTrialConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_time: Option<PrimField<String>>,
}
impl DataStorageControlProjectIntelligenceConfigTrialConfigEl {
    #[doc = "Set the field `expire_time`.\n"]
    pub fn set_expire_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expire_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageControlProjectIntelligenceConfigTrialConfigEl {
    type O = BlockAssignable<DataStorageControlProjectIntelligenceConfigTrialConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageControlProjectIntelligenceConfigTrialConfigEl {}
impl BuildDataStorageControlProjectIntelligenceConfigTrialConfigEl {
    pub fn build(self) -> DataStorageControlProjectIntelligenceConfigTrialConfigEl {
        DataStorageControlProjectIntelligenceConfigTrialConfigEl {
            expire_time: core::default::Default::default(),
        }
    }
}
pub struct DataStorageControlProjectIntelligenceConfigTrialConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageControlProjectIntelligenceConfigTrialConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageControlProjectIntelligenceConfigTrialConfigElRef {
        DataStorageControlProjectIntelligenceConfigTrialConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageControlProjectIntelligenceConfigTrialConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\n"]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.base))
    }
}
